//! Adversarial transformations and composition application (§8 / §10).

use crate::adversarial::error::{AdversarialError, AdversarialErrorClass};
use crate::adversarial::parameters::ParameterTuple;
use crate::adversarial::projection::{
    inherit_all_projections, validate_projection_table, FieldProjection, ProjectionMode,
    ScenarioField,
};
use crate::adversarial::resolve::{ActionTarget, ResolutionClass};
use crate::adversarial::token::IdentifierToken;
use crate::adversarial::types::{ApplicabilityPolicy, ParameterValue};
use crate::kernel::account::AccountId;
use crate::kernel::action::{Action, ActionId};
use crate::kernel::asset::AssetId;
use crate::kernel::balance::FacetId;
use crate::simulator::{Scenario, ScenarioId};

/// How a duplicated Action receives its `action_id` (R-16).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DuplicateIdMode {
    /// `REMOVE_EXPLICITLY` → `None` (default).
    RemoveExplicitly,
    /// Inherit source ActionId (may create ambiguity later).
    Inherit,
    /// Replace with an explicit unique id.
    ReplaceExplicitly(IdentifierToken),
}

/// Gate-4 transformation kinds (closed production set for this implementation).
///
/// Taxonomy coverage is via these explicit kinds; extension requires amendment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransformationKind {
    /// Metadata-only derivation (payload inherited unchanged).
    MetadataSuffix {
        /// Suffix appended to scenario id (must be non-empty IdentifierToken fragment).
        id_suffix: IdentifierToken,
        /// Replacement version label.
        new_version: IdentifierToken,
    },
    /// Duplicate an action into the current sequence.
    DuplicateAction {
        /// Source action.
        source: ActionTarget,
        /// Insertion index in the resulting sequence.
        insert_at: usize,
        /// Identity mode for the duplicate.
        id_mode: DuplicateIdMode,
    },
    /// Delete an action from the current sequence.
    DeleteAction {
        /// Target action.
        target: ActionTarget,
    },
    /// Insert an explicit action at an index.
    InsertAction {
        /// Insertion index.
        index: usize,
        /// Action to insert.
        action: Action,
    },
    /// Replace an action at a target.
    ReplaceAction {
        /// Target action.
        target: ActionTarget,
        /// Replacement action.
        replacement: Action,
    },
    /// Reorder actions by declared permutation of current indices.
    ReorderActions {
        /// `new_order[i]` = old index placed at position `i`.
        new_order: Vec<usize>,
    },
    /// Set/replace one initial-state balance cell (explicit parameters only).
    SetInitialBalance {
        /// Account id.
        account: IdentifierToken,
        /// Asset id.
        asset: IdentifierToken,
        /// Facet id.
        facet: IdentifierToken,
        /// Minor-unit amount (`i128`).
        amount: i128,
    },
    /// Replace `maximum_action_steps`.
    ReplaceMaximumActionSteps {
        /// New limit.
        value: u64,
    },
    /// Parameterized amount replacement on a Transfer / TransferWithFee target.
    ///
    /// Bound from parameter tuple key `amount` when present; otherwise uses `fallback_amount`.
    ReplaceTransferAmount {
        /// Target action.
        target: ActionTarget,
        /// Fallback amount when no parameter binding.
        fallback_amount: i128,
    },
}

/// Declared transformation entry in a composition plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformationDefinition {
    /// Transformation identity.
    pub identity: IdentifierToken,
    /// Transformation version.
    pub version: IdentifierToken,
    /// Kind / semantics.
    pub kind: TransformationKind,
    /// Explicit per-field projection declarations (§8.4; F-02).
    pub projections: Vec<crate::adversarial::projection::FieldProjection>,
    /// Optional parameter domain (empty = single application).
    pub parameter_domain: crate::adversarial::types::ParameterDomain,
}

/// Outcome of applying one transformation to a snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransformApplication {
    /// Candidate produced.
    Derived {
        /// Derived scenario.
        scenario: Scenario,
        /// Action mutations performed in this application.
        action_mutations: u64,
        /// Target resolution provenance lines.
        resolutions: Vec<crate::adversarial::resolve::TargetResolution>,
    },
    /// Preconditions unmet (normal under AllowNonApplicable).
    NonApplicable {
        /// Reason.
        reason: String,
        /// Resolution provenance if any.
        resolutions: Vec<crate::adversarial::resolve::TargetResolution>,
    },
    /// Hard failure.
    Error(AdversarialError),
}

/// Apply a transformation to a scenario snapshot.
#[must_use]
pub fn apply_transformation(
    snapshot: &Scenario,
    def: &TransformationDefinition,
    params: &ParameterTuple,
    policy: ApplicabilityPolicy,
    composition_index: usize,
) -> TransformApplication {
    let amount_override = params
        .bindings
        .iter()
        .find(|(id, _)| id.as_str() == "amount")
        .and_then(|(_, v)| match v {
            ParameterValue::EconomicAmount(n) | ParameterValue::Integer(n) => Some(*n),
            _ => None,
        });

    match &def.kind {
        TransformationKind::MetadataSuffix {
            id_suffix,
            new_version,
        } => {
            let mut s = snapshot.clone();
            s.id = ScenarioId::new(format!("{}#{}", s.id.as_str(), id_suffix.as_str()));
            s.version = new_version.as_str().to_owned();
            let _ = composition_index;
            TransformApplication::Derived {
                scenario: s,
                action_mutations: 0,
                resolutions: Vec::new(),
            }
        }
        TransformationKind::DuplicateAction {
            source,
            insert_at,
            id_mode,
        } => match source.resolve(&snapshot.actions) {
            ResolutionClass::Resolved { position, record } => {
                let mut actions = snapshot.actions.clone();
                let mut dup = actions[position].clone();
                let new_id = match id_mode {
                    DuplicateIdMode::RemoveExplicitly => None,
                    DuplicateIdMode::Inherit => dup.action_id().cloned(),
                    DuplicateIdMode::ReplaceExplicitly(tok) => Some(ActionId::new(tok.as_str())),
                };
                set_action_id(&mut dup, new_id);
                if *insert_at > actions.len() {
                    return non_or_err(policy, "duplicate insert_at out of range", vec![record]);
                }
                actions.insert(*insert_at, dup);
                let mut s = snapshot.clone();
                s.actions = actions;
                bump_metadata(&mut s, &def.identity, composition_index);
                TransformApplication::Derived {
                    scenario: s,
                    action_mutations: 1,
                    resolutions: vec![record],
                }
            }
            ResolutionClass::Missing { record } => {
                non_or_err(policy, "duplicate source missing", vec![record])
            }
            ResolutionClass::Ambiguous { record } => {
                TransformApplication::Error(AdversarialError::new(
                    AdversarialErrorClass::InvalidTransformParameter,
                    format!("AMBIGUOUS_ACTION_ID match_count={}", record.match_count),
                ))
            }
            ResolutionClass::MalformedParameter { reason } => TransformApplication::Error(
                AdversarialError::new(AdversarialErrorClass::InvalidTransformParameter, reason),
            ),
        },
        TransformationKind::DeleteAction { target } => match target.resolve(&snapshot.actions) {
            ResolutionClass::Resolved { position, record } => {
                let mut actions = snapshot.actions.clone();
                actions.remove(position);
                let mut s = snapshot.clone();
                s.actions = actions;
                bump_metadata(&mut s, &def.identity, composition_index);
                TransformApplication::Derived {
                    scenario: s,
                    action_mutations: 1,
                    resolutions: vec![record],
                }
            }
            ResolutionClass::Missing { record } => {
                non_or_err(policy, "delete target missing", vec![record])
            }
            ResolutionClass::Ambiguous { record } => {
                TransformApplication::Error(AdversarialError::new(
                    AdversarialErrorClass::InvalidTransformParameter,
                    format!("AMBIGUOUS_ACTION_ID match_count={}", record.match_count),
                ))
            }
            ResolutionClass::MalformedParameter { reason } => TransformApplication::Error(
                AdversarialError::new(AdversarialErrorClass::InvalidTransformParameter, reason),
            ),
        },
        TransformationKind::InsertAction { index, action } => {
            if *index > snapshot.actions.len() {
                return non_or_err(policy, "insert index out of range", Vec::new());
            }
            let mut s = snapshot.clone();
            s.actions.insert(*index, action.clone());
            bump_metadata(&mut s, &def.identity, composition_index);
            TransformApplication::Derived {
                scenario: s,
                action_mutations: 1,
                resolutions: Vec::new(),
            }
        }
        TransformationKind::ReplaceAction {
            target,
            replacement,
        } => match target.resolve(&snapshot.actions) {
            ResolutionClass::Resolved { position, record } => {
                let mut s = snapshot.clone();
                s.actions[position] = replacement.clone();
                bump_metadata(&mut s, &def.identity, composition_index);
                TransformApplication::Derived {
                    scenario: s,
                    action_mutations: 1,
                    resolutions: vec![record],
                }
            }
            ResolutionClass::Missing { record } => {
                non_or_err(policy, "replace target missing", vec![record])
            }
            ResolutionClass::Ambiguous { record } => {
                TransformApplication::Error(AdversarialError::new(
                    AdversarialErrorClass::InvalidTransformParameter,
                    format!("AMBIGUOUS_ACTION_ID match_count={}", record.match_count),
                ))
            }
            ResolutionClass::MalformedParameter { reason } => TransformApplication::Error(
                AdversarialError::new(AdversarialErrorClass::InvalidTransformParameter, reason),
            ),
        },
        TransformationKind::ReorderActions { new_order } => {
            let n = snapshot.actions.len();
            if new_order.len() != n {
                return TransformApplication::Error(AdversarialError::new(
                    AdversarialErrorClass::InvalidTransformParameter,
                    "reorder permutation length mismatch",
                ));
            }
            let mut seen = vec![false; n];
            for &idx in new_order {
                if idx >= n || seen[idx] {
                    return TransformApplication::Error(AdversarialError::new(
                        AdversarialErrorClass::InvalidTransformParameter,
                        "reorder permutation invalid",
                    ));
                }
                seen[idx] = true;
            }
            let mut s = snapshot.clone();
            s.actions = new_order
                .iter()
                .map(|&i| snapshot.actions[i].clone())
                .collect();
            bump_metadata(&mut s, &def.identity, composition_index);
            TransformApplication::Derived {
                scenario: s,
                action_mutations: 1,
                resolutions: Vec::new(),
            }
        }
        TransformationKind::SetInitialBalance {
            account,
            asset,
            facet,
            amount,
        } => {
            let mut s = snapshot.clone();
            s.initial_state.set_balance(
                AccountId::new(account.as_str()),
                AssetId::new(asset.as_str()),
                FacetId::new(facet.as_str()),
                *amount,
            );
            bump_metadata(&mut s, &def.identity, composition_index);
            TransformApplication::Derived {
                scenario: s,
                action_mutations: 0,
                resolutions: Vec::new(),
            }
        }
        TransformationKind::ReplaceMaximumActionSteps { value } => {
            let mut s = snapshot.clone();
            s.maximum_action_steps = *value;
            bump_metadata(&mut s, &def.identity, composition_index);
            TransformApplication::Derived {
                scenario: s,
                action_mutations: 0,
                resolutions: Vec::new(),
            }
        }
        TransformationKind::ReplaceTransferAmount {
            target,
            fallback_amount,
        } => {
            let amount = amount_override.unwrap_or(*fallback_amount);
            match target.resolve(&snapshot.actions) {
                ResolutionClass::Resolved { position, record } => {
                    let mut s = snapshot.clone();
                    match replace_transfer_amount(&mut s.actions[position], amount) {
                        Ok(()) => {
                            bump_metadata(&mut s, &def.identity, composition_index);
                            TransformApplication::Derived {
                                scenario: s,
                                action_mutations: 1,
                                resolutions: vec![record],
                            }
                        }
                        Err(reason) => non_or_err(policy, reason, vec![record]),
                    }
                }
                ResolutionClass::Missing { record } => {
                    non_or_err(policy, "amount target missing", vec![record])
                }
                ResolutionClass::Ambiguous { record } => {
                    TransformApplication::Error(AdversarialError::new(
                        AdversarialErrorClass::InvalidTransformParameter,
                        format!("AMBIGUOUS_ACTION_ID match_count={}", record.match_count),
                    ))
                }
                ResolutionClass::MalformedParameter { reason } => TransformApplication::Error(
                    AdversarialError::new(AdversarialErrorClass::InvalidTransformParameter, reason),
                ),
            }
        }
    }
}

fn non_or_err(
    policy: ApplicabilityPolicy,
    reason: impl Into<String>,
    resolutions: Vec<crate::adversarial::resolve::TargetResolution>,
) -> TransformApplication {
    let reason = reason.into();
    match policy {
        ApplicabilityPolicy::AllowNonApplicable => TransformApplication::NonApplicable {
            reason,
            resolutions,
        },
        ApplicabilityPolicy::RequireApplicable => TransformApplication::Error(
            AdversarialError::new(AdversarialErrorClass::NonApplicableTransform, reason),
        ),
    }
}

/// Provisional metadata derivation (AD-03 remains OPEN).
///
/// Explicit rule: append `#<identity>@<composition_index>` to id and set version
/// to `{old}+m04`. This satisfies §8.4.1 explicit metadata change without closing AD-03.
fn bump_metadata(scenario: &mut Scenario, identity: &IdentifierToken, composition_index: usize) {
    scenario.id = ScenarioId::new(format!(
        "{}#{}@{}",
        scenario.id.as_str(),
        identity.as_str(),
        composition_index
    ));
    scenario.version = format!("{}+m04", scenario.version);
}

fn set_action_id(action: &mut Action, id: Option<ActionId>) {
    match action {
        Action::NoOp { action_id, .. }
        | Action::Transfer { action_id, .. }
        | Action::TransferWithFee { action_id, .. }
        | Action::Convert { action_id, .. } => *action_id = id,
    }
}

fn replace_transfer_amount(action: &mut Action, amount: i128) -> Result<(), String> {
    match action {
        Action::Transfer { amount: a, .. } => {
            *a = amount;
            Ok(())
        }
        Action::TransferWithFee { amount: a, .. } => {
            *a = amount;
            Ok(())
        }
        _ => Err("ReplaceTransferAmount requires Transfer or TransferWithFee".into()),
    }
}

/// Canonical projection table for a closed kind (§8.4.1 defaults).
#[must_use]
pub fn projections_for_kind(kind: &TransformationKind) -> Vec<FieldProjection> {
    let mut table = inherit_all_projections();
    let set = |table: &mut [FieldProjection], field: ScenarioField, mode: ProjectionMode| {
        if let Some(slot) = table.iter_mut().find(|p| p.field == field) {
            slot.mode = mode;
        }
    };
    match kind {
        TransformationKind::MetadataSuffix { .. } => {
            set(&mut table, ScenarioField::Id, ProjectionMode::DeriveExplicitly);
            set(
                &mut table,
                ScenarioField::Version,
                ProjectionMode::DeriveExplicitly,
            );
        }
        TransformationKind::DuplicateAction { .. }
        | TransformationKind::DeleteAction { .. }
        | TransformationKind::InsertAction { .. }
        | TransformationKind::ReplaceAction { .. }
        | TransformationKind::ReorderActions { .. }
        | TransformationKind::ReplaceTransferAmount { .. } => {
            set(
                &mut table,
                ScenarioField::Actions,
                ProjectionMode::ReplaceExplicitly,
            );
            set(&mut table, ScenarioField::Id, ProjectionMode::DeriveExplicitly);
            set(
                &mut table,
                ScenarioField::Version,
                ProjectionMode::DeriveExplicitly,
            );
        }
        TransformationKind::SetInitialBalance { .. } => {
            set(
                &mut table,
                ScenarioField::InitialState,
                ProjectionMode::ReplaceExplicitly,
            );
            set(&mut table, ScenarioField::Id, ProjectionMode::DeriveExplicitly);
            set(
                &mut table,
                ScenarioField::Version,
                ProjectionMode::DeriveExplicitly,
            );
        }
        TransformationKind::ReplaceMaximumActionSteps { .. } => {
            set(
                &mut table,
                ScenarioField::MaximumActionSteps,
                ProjectionMode::ReplaceExplicitly,
            );
            set(&mut table, ScenarioField::Id, ProjectionMode::DeriveExplicitly);
            set(
                &mut table,
                ScenarioField::Version,
                ProjectionMode::DeriveExplicitly,
            );
        }
    }
    table
}

/// Validate projection table completeness and consistency with `kind`.
pub fn validate_definition_projections(def: &TransformationDefinition) -> Result<(), String> {
    validate_projection_table(&def.projections)?;
    validate_projection_matches_kind(def, &def.projections)
}

fn validate_projection_matches_kind(
    def: &TransformationDefinition,
    decls: &[FieldProjection],
) -> Result<(), String> {
    let expected = projections_for_kind(&def.kind);
    for e in &expected {
        let Some(actual) = decls.iter().find(|p| p.field == e.field) else {
            return Err(format!("missing field {}", e.field.as_str()));
        };
        if actual.mode != e.mode {
            return Err(format!(
                "projection for {} declared {:?}, inconsistent with kind (expected {:?})",
                e.field.as_str(),
                actual.mode,
                e.mode
            ));
        }
    }
    Ok(())
}

/// Construct a definition with projections matching `kind`.
#[must_use]
pub fn definition_with_kind_projections(
    identity: IdentifierToken,
    version: IdentifierToken,
    kind: TransformationKind,
    parameter_domain: crate::adversarial::types::ParameterDomain,
) -> TransformationDefinition {
    let projections = projections_for_kind(&kind);
    TransformationDefinition {
        identity,
        version,
        kind,
        projections,
        parameter_domain,
    }
}
