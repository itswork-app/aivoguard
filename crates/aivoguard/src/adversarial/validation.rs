//! Closed M04 structural validation checklist (§8.5.1 / R-12 / R-15 / R-20).

use crate::adversarial::token::IdentifierToken;
use crate::adversarial::types::{StructuralInvalidReason, ValidationClassification};
use crate::kernel::action::Action;
use crate::simulator::{InvariantEvaluationPlan, Scenario, StopCondition, StopPolicy};

/// Result of closed structural validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralValidationResult {
    /// VALID or INVALID.
    pub classification: ValidationClassification,
    /// First failing reason (only when INVALID).
    pub primary_reason: Option<StructuralInvalidReason>,
    /// Deterministic detail string.
    pub detail: String,
}

impl StructuralValidationResult {
    fn valid() -> Self {
        Self {
            classification: ValidationClassification::Valid,
            primary_reason: None,
            detail: String::new(),
        }
    }

    fn invalid(reason: StructuralInvalidReason, detail: impl Into<String>) -> Self {
        Self {
            classification: ValidationClassification::Invalid,
            primary_reason: Some(reason),
            detail: detail.into(),
        }
    }

    /// Whether validation passed.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.classification == ValidationClassification::Valid
    }
}

/// Structural view used by the ordered checklist.
///
/// Production candidates from M03 `Scenario` always have `world_present = true`.
/// The view exists so fail-fast **4a → 4b** ordering is testable without inventing
/// M01/M03 types that omit World.
#[derive(Debug, Clone)]
pub struct ChecklistView {
    /// Scenario id.
    pub id: String,
    /// Scenario version.
    pub version: String,
    /// Configuration id.
    pub configuration_id: String,
    /// Maximum action steps.
    pub maximum_action_steps: u64,
    /// Whether World envelope is present (4a).
    pub world_present: bool,
    /// Whether initial_state envelope is present (4b).
    pub initial_state_present: bool,
    /// Balance cells for IdentifierToken checks: (account, asset, facet, amount).
    pub balance_cells: Vec<(String, String, String, i128)>,
    /// Actions.
    pub actions: Vec<Action>,
    /// Declared unix secs.
    pub declared_unix_secs: Option<i64>,
    /// Invariant plan.
    pub invariant_plan: InvariantEvaluationPlan,
    /// Stop policy.
    pub stop_policy: StopPolicy,
    /// Declared projection table for check 3 (F-02). Must be complete when present.
    pub projection_declarations: Vec<crate::adversarial::projection::FieldProjection>,
    /// Whether provenance is complete (check 12).
    pub provenance_complete: bool,
    /// Whether any field is marked as host/env-derived (check 11).
    pub has_forbidden_hidden_input: bool,
    /// Whether prior ActionId/INDEX refs resolved uniquely when required (check 6).
    pub action_references_valid: bool,
}

impl ChecklistView {
    /// Build a checklist view from an M03 Scenario (world always present).
    #[must_use]
    pub fn from_scenario(
        scenario: &Scenario,
        provenance_complete: bool,
        projection_declarations: Vec<crate::adversarial::projection::FieldProjection>,
    ) -> Self {
        let balance_cells = scenario
            .initial_state
            .balances()
            .iter()
            .map(|(k, v)| {
                (
                    k.account.as_str().to_owned(),
                    k.asset.as_str().to_owned(),
                    k.facet.as_str().to_owned(),
                    *v,
                )
            })
            .collect();
        Self {
            id: scenario.id.as_str().to_owned(),
            version: scenario.version.clone(),
            configuration_id: scenario.configuration_id.clone(),
            maximum_action_steps: scenario.maximum_action_steps,
            world_present: true,
            initial_state_present: true,
            balance_cells,
            actions: scenario.actions.clone(),
            declared_unix_secs: scenario.declared_unix_secs,
            invariant_plan: scenario.invariant_plan.clone(),
            stop_policy: scenario.stop_policy.clone(),
            projection_declarations,
            provenance_complete,
            has_forbidden_hidden_input: false,
            action_references_valid: true,
        }
    }
}

/// Run the closed checklist in order; first failure wins.
#[must_use]
pub fn validate_checklist(view: &ChecklistView) -> StructuralValidationResult {
    // 1 — Scenario envelope
    // Presence of required Scenario-shaped fields is represented by ChecklistView.
    // Empty id/version handled in check 2; missing world handled in 4a.

    // 2 — Required fields
    if !IdentifierToken::is_valid(&view.id) {
        return StructuralValidationResult::invalid(
            StructuralInvalidReason::InvalidRequiredField,
            "scenario id is empty",
        );
    }
    if !IdentifierToken::is_valid(&view.version) {
        return StructuralValidationResult::invalid(
            StructuralInvalidReason::InvalidRequiredField,
            "scenario version is empty",
        );
    }
    if !IdentifierToken::is_valid(&view.configuration_id) {
        return StructuralValidationResult::invalid(
            StructuralInvalidReason::InvalidRequiredField,
            "configuration_id is empty",
        );
    }

    // 3 — Projection consistency (derived from declared table; not a boolean flag)
    if let Err(detail) =
        crate::adversarial::projection::validate_projection_table(&view.projection_declarations)
    {
        return StructuralValidationResult::invalid(
            StructuralInvalidReason::InvalidProjection,
            detail,
        );
    }

    // 4a — World envelope (MUST precede 4b)
    if !view.world_present {
        return StructuralValidationResult::invalid(
            StructuralInvalidReason::InvalidScenarioStructure,
            "4a: world envelope absent",
        );
    }

    // 4b — initial_state structure
    if !view.initial_state_present {
        return StructuralValidationResult::invalid(
            StructuralInvalidReason::InvalidInitialStateStructure,
            "4b: initial_state envelope absent",
        );
    }
    // Balance cells are explicit (account, asset, facet) → i128 by construction.

    // 5 — Action sequence structure (Rust Action enum is closed; always typed)
    // No additional structural failure possible for well-typed Action.

    // 6 — Action identity / reference validity
    if !view.action_references_valid {
        return StructuralValidationResult::invalid(
            StructuralInvalidReason::InvalidActionReference,
            "action reference invalid or ambiguous at application time",
        );
    }

    // 7 — Domain reference IdentifierToken
    for (account, asset, facet, _) in &view.balance_cells {
        if !IdentifierToken::is_valid(account)
            || !IdentifierToken::is_valid(asset)
            || !IdentifierToken::is_valid(facet)
        {
            return StructuralValidationResult::invalid(
                StructuralInvalidReason::InvalidDomainReference,
                "balance cell contains empty IdentifierToken",
            );
        }
    }
    for action in &view.actions {
        if let Some(fail) = action_domain_token_failure(action) {
            return StructuralValidationResult::invalid(
                StructuralInvalidReason::InvalidDomainReference,
                fail,
            );
        }
    }

    // 8 — Execution configuration
    if !IdentifierToken::is_valid(&view.configuration_id) {
        return StructuralValidationResult::invalid(
            StructuralInvalidReason::InvalidExecutionConfiguration,
            "configuration_id is not an IdentifierToken",
        );
    }
    // declared_unix_secs is Option<i64> — structurally legal when present.
    let _ = view.maximum_action_steps;
    let _ = view.declared_unix_secs;

    // 9 — Invariant-plan structure (minimum envelope {id, definition_version})
    if let Some(detail) = invariant_plan_failure(&view.invariant_plan) {
        return StructuralValidationResult::invalid(
            StructuralInvalidReason::InvalidInvariantPlanStructure,
            detail,
        );
    }

    // 10 — Stop-policy structure
    if let Some(detail) = stop_policy_failure(&view.stop_policy) {
        return StructuralValidationResult::invalid(
            StructuralInvalidReason::InvalidStopPolicyStructure,
            detail,
        );
    }

    // 11 — Hidden-input prohibition
    if view.has_forbidden_hidden_input {
        return StructuralValidationResult::invalid(
            StructuralInvalidReason::ForbiddenHiddenInput,
            "candidate marked as host/env/RNG/LLM derived",
        );
    }

    // 12 — Provenance completeness
    if !view.provenance_complete {
        return StructuralValidationResult::invalid(
            StructuralInvalidReason::InvalidProvenance,
            "emitted candidate missing required provenance",
        );
    }

    StructuralValidationResult::valid()
}

/// Validate an M03 Scenario under the closed checklist.
#[must_use]
pub fn validate_scenario(
    scenario: &Scenario,
    provenance_complete: bool,
    projection_declarations: &[crate::adversarial::projection::FieldProjection],
) -> StructuralValidationResult {
    validate_checklist(&ChecklistView::from_scenario(
        scenario,
        provenance_complete,
        projection_declarations.to_vec(),
    ))
}

fn invariant_plan_failure(plan: &InvariantEvaluationPlan) -> Option<String> {
    for (label, invs) in [
        ("before_action", &plan.before_action),
        ("after_action", &plan.after_action),
        ("on_completion", &plan.on_completion),
    ] {
        for inv in invs {
            if !IdentifierToken::is_valid(inv.id.as_str()) {
                return Some(format!("{label}: invariant id is empty"));
            }
            if !IdentifierToken::is_valid(&inv.definition_version) {
                return Some(format!(
                    "{label}: invariant definition_version is empty (id={})",
                    inv.id.as_str()
                ));
            }
        }
    }
    None
}

fn stop_policy_failure(policy: &StopPolicy) -> Option<String> {
    // StopCondition is a closed enum; each variant has required fields by construction.
    for (i, c) in policy.conditions.iter().enumerate() {
        match c {
            StopCondition::AfterCompletedStep(_)
            | StopCondition::OnEconomicDisposition(_)
            | StopCondition::OnM02Kind { .. } => {}
        }
        let _ = i;
    }
    None
}

fn action_domain_token_failure(action: &Action) -> Option<String> {
    match action {
        Action::NoOp { actor, action_id } => {
            if !IdentifierToken::is_valid(actor.as_str()) {
                return Some("NoOp actor is empty".into());
            }
            if let Some(id) = action_id {
                if !IdentifierToken::is_valid(id.as_str()) {
                    return Some("NoOp action_id is empty".into());
                }
            }
        }
        Action::Transfer {
            actor,
            action_id,
            from,
            to,
            asset,
            ..
        } => {
            for (label, s) in [
                ("actor", actor.as_str()),
                ("from", from.as_str()),
                ("to", to.as_str()),
                ("asset", asset.as_str()),
            ] {
                if !IdentifierToken::is_valid(s) {
                    return Some(format!("Transfer {label} is empty"));
                }
            }
            if let Some(id) = action_id {
                if !IdentifierToken::is_valid(id.as_str()) {
                    return Some("Transfer action_id is empty".into());
                }
            }
        }
        Action::TransferWithFee {
            actor,
            action_id,
            from,
            to,
            asset,
            fee_payer,
            fee_recipient,
            fee_asset,
            ..
        } => {
            for (label, s) in [
                ("actor", actor.as_str()),
                ("from", from.as_str()),
                ("to", to.as_str()),
                ("asset", asset.as_str()),
                ("fee_payer", fee_payer.as_str()),
                ("fee_recipient", fee_recipient.as_str()),
                ("fee_asset", fee_asset.as_str()),
            ] {
                if !IdentifierToken::is_valid(s) {
                    return Some(format!("TransferWithFee {label} is empty"));
                }
            }
            if let Some(id) = action_id {
                if !IdentifierToken::is_valid(id.as_str()) {
                    return Some("TransferWithFee action_id is empty".into());
                }
            }
        }
        Action::Convert {
            actor,
            action_id,
            account,
            from_asset,
            to_asset,
            price_id,
            ..
        } => {
            for (label, s) in [
                ("actor", actor.as_str()),
                ("account", account.as_str()),
                ("from_asset", from_asset.as_str()),
                ("to_asset", to_asset.as_str()),
                ("price_id", price_id.as_str()),
            ] {
                if !IdentifierToken::is_valid(s) {
                    return Some(format!("Convert {label} is empty"));
                }
            }
            if let Some(id) = action_id {
                if !IdentifierToken::is_valid(id.as_str()) {
                    return Some("Convert action_id is empty".into());
                }
            }
        }
    }
    None
}
