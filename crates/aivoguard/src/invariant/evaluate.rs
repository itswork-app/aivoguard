//! M02 invariant evaluation engine.

use crate::invariant::count::Count;
use crate::invariant::definition::{
    AggOp, Applicability, CmpOp, DomainExpr, Invariant, InvariantScope, PropertyExpr, RelationKind,
    ValueExpr, ViolationPolicy,
};
use crate::invariant::error::{InvariantError, InvariantErrorClass};
use crate::invariant::evidence::InvariantEvidence;
use crate::invariant::history::HistoryRecord;
use crate::invariant::result::{ApplicabilityStatus, InvariantOutcome, InvariantResultKind};
use crate::invariant::target::EvaluationTarget;
use crate::invariant::violation::{EvaluationLocation, StructuredFact, Violation};
use crate::kernel::account::AccountId;
use crate::kernel::asset::AssetId;
use crate::kernel::balance::FacetId;
use crate::kernel::error::EconomicDisposition;
use crate::kernel::money::Money;
use crate::kernel::rounding::divide_with_rounding;
use crate::kernel::state::EconomicState;
use crate::kernel::world::{EconomicWorld, PriceId};

/// M02 engine version (deterministic evidence).
pub const ENGINE_VERSION: &str = "aivoguard-m02-0.0.0";

#[derive(Debug, Clone)]
struct Bindings {
    account: Option<AccountId>,
    history_ordinal: Option<usize>,
}

impl Bindings {
    fn empty() -> Self {
        Self {
            account: None,
            history_ordinal: None,
        }
    }
}

/// Evaluate a structured invariant against an authoritative target.
#[must_use]
pub fn evaluate_invariant(invariant: &Invariant, target: EvaluationTarget<'_>) -> InvariantOutcome {
    let mut evidence = InvariantEvidence {
        invariant_id: invariant.id.clone(),
        definition_version: invariant.definition_version.clone(),
        target_kind: target.kind_label().to_owned(),
        engine_version: ENGINE_VERSION.to_owned(),
        result_kind: InvariantResultKind::Pass,
        applicability: None,
        error_class: None,
        error_reason: None,
        notes: Vec::new(),
    };

    if let Err(err) = check_target_compat(invariant.scope, target) {
        return error_outcome(evidence, err, Vec::new());
    }

    match resolve_applicability(&invariant.applicability, target.world()) {
        Ok(ApplicabilityStatus::NotApplicable) => {
            evidence.result_kind = InvariantResultKind::Pass;
            evidence.applicability = Some(ApplicabilityStatus::NotApplicable);
            evidence
                .notes
                .push("applicability_status=NotApplicable".into());
            return InvariantOutcome {
                kind: InvariantResultKind::Pass,
                applicability: Some(ApplicabilityStatus::NotApplicable),
                violations: Vec::new(),
                error: None,
                evidence,
            };
        }
        Ok(ApplicabilityStatus::Applicable) => {}
        Err(err) => return error_outcome(evidence, err, Vec::new()),
    }

    let mut violations = Vec::new();
    let mut discovered_error: Option<InvariantError> = None;
    let mut bindings = Bindings::empty();

    match eval_property(
        invariant,
        target,
        &invariant.property,
        &mut bindings,
        &mut violations,
        &mut discovered_error,
    ) {
        Ok(()) => {}
        Err(err) => {
            discovered_error = Some(match discovered_error {
                Some(prev) => prev.prefer(err),
                None => err,
            });
        }
    }

    if let Some(err) = discovered_error {
        return error_outcome(evidence, err, violations);
    }

    if violations.is_empty() {
        evidence.result_kind = InvariantResultKind::Pass;
        evidence.applicability = Some(ApplicabilityStatus::Applicable);
        InvariantOutcome {
            kind: InvariantResultKind::Pass,
            applicability: Some(ApplicabilityStatus::Applicable),
            violations: Vec::new(),
            error: None,
            evidence,
        }
    } else {
        let retained = retain_violations(invariant.violation_policy, violations);
        evidence.result_kind = InvariantResultKind::Fail;
        evidence.applicability = Some(ApplicabilityStatus::Applicable);
        InvariantOutcome {
            kind: InvariantResultKind::Fail,
            applicability: Some(ApplicabilityStatus::Applicable),
            violations: retained,
            error: None,
            evidence,
        }
    }
}

fn error_outcome(
    mut evidence: InvariantEvidence,
    err: InvariantError,
    violations: Vec<Violation>,
) -> InvariantOutcome {
    evidence.result_kind = InvariantResultKind::Error;
    evidence.error_class = Some(err.class);
    evidence.error_reason = Some(err.reason.clone());
    InvariantOutcome {
        kind: InvariantResultKind::Error,
        applicability: None,
        violations,
        error: Some(err),
        evidence,
    }
}

fn check_target_compat(
    scope: InvariantScope,
    target: EvaluationTarget<'_>,
) -> Result<(), InvariantError> {
    let ok = matches!(
        (scope, target),
        (InvariantScope::State, EvaluationTarget::State { .. })
            | (
                InvariantScope::Transition,
                EvaluationTarget::Transition { .. }
            )
            | (InvariantScope::History, EvaluationTarget::History { .. })
    );
    if ok {
        Ok(())
    } else {
        Err(InvariantError::new(
            InvariantErrorClass::IncompatibleTarget,
            format!(
                "scope {:?} incompatible with target {}",
                scope,
                target.kind_label()
            ),
        ))
    }
}

fn resolve_applicability(
    spec: &Applicability,
    world: &EconomicWorld,
) -> Result<ApplicabilityStatus, InvariantError> {
    match spec {
        Applicability::Always => Ok(ApplicabilityStatus::Applicable),
        Applicability::RequiresWorldConcept(concept) => {
            // Presence of the concept set itself is authoritative World config.
            // Missing concept declaration → NotApplicable (World lacks concept).
            // We cannot treat absent evaluation data as NotApplicable.
            if world.has_concept(concept) {
                Ok(ApplicabilityStatus::Applicable)
            } else {
                Ok(ApplicabilityStatus::NotApplicable)
            }
        }
    }
}

fn retain_violations(policy: ViolationPolicy, violations: Vec<Violation>) -> Vec<Violation> {
    match policy {
        ViolationPolicy::All => violations,
        ViolationPolicy::FirstOnly => violations.into_iter().take(1).collect(),
    }
}

fn eval_property(
    invariant: &Invariant,
    target: EvaluationTarget<'_>,
    property: &PropertyExpr,
    bindings: &mut Bindings,
    violations: &mut Vec<Violation>,
    discovered_error: &mut Option<InvariantError>,
) -> Result<(), InvariantError> {
    match property {
        PropertyExpr::Compare { left, op, right } => {
            let l = eval_value(invariant, target, left, bindings)?;
            let r = eval_value(invariant, target, right, bindings)?;
            match compare_scalars(&l, *op, &r) {
                Ok(true) => Ok(()),
                Ok(false) => {
                    let v = make_violation(
                        invariant,
                        location_from_bindings(bindings),
                        vec![fact("required", format!("{l:?} {op:?} {r:?}"))],
                        vec![fact("observed", "comparison false")],
                    );
                    record_violation(invariant.violation_policy, violations, v);
                    Ok(())
                }
                Err(err) => Err(err),
            }
        }
        PropertyExpr::ForAll { domain, body } => {
            let members = resolve_domain(invariant, target, domain, bindings)?;
            if members.is_empty() {
                return Ok(());
            }
            for member in members {
                apply_binding(bindings, &member);
                match eval_property(
                    invariant,
                    target,
                    body,
                    bindings,
                    violations,
                    discovered_error,
                ) {
                    Ok(()) => {}
                    Err(err) => {
                        *discovered_error = Some(match discovered_error.take() {
                            Some(prev) => prev.prefer(err),
                            None => err,
                        });
                    }
                }
                clear_binding(bindings, &member);
            }
            if let Some(err) = discovered_error.clone() {
                return Err(err);
            }
            Ok(())
        }
        PropertyExpr::Exists { domain, body } => {
            let members = resolve_domain(invariant, target, domain, bindings)?;
            if members.is_empty() {
                let v = make_violation(
                    invariant,
                    EvaluationLocation::Invariant,
                    vec![fact("required", "EXISTS witness")],
                    vec![fact("observed", "empty domain")],
                );
                record_violation(invariant.violation_policy, violations, v);
                return Ok(());
            }
            let mut found = false;
            let mut local_violations = Vec::new();
            for member in members {
                apply_binding(bindings, &member);
                let before = local_violations.len();
                match eval_property(
                    invariant,
                    target,
                    body,
                    bindings,
                    &mut local_violations,
                    discovered_error,
                ) {
                    Ok(()) => {
                        if local_violations.len() == before {
                            found = true;
                        }
                    }
                    Err(err) => {
                        *discovered_error = Some(match discovered_error.take() {
                            Some(prev) => prev.prefer(err),
                            None => err,
                        });
                    }
                }
                clear_binding(bindings, &member);
            }
            if let Some(err) = discovered_error.clone() {
                return Err(err);
            }
            if found {
                Ok(())
            } else {
                let v = make_violation(
                    invariant,
                    EvaluationLocation::Invariant,
                    vec![fact("required", "EXISTS witness")],
                    vec![fact("observed", "no witness")],
                );
                record_violation(invariant.violation_policy, violations, v);
                Ok(())
            }
        }
        PropertyExpr::Relation {
            kind,
            account,
            asset,
            facet,
        } => eval_relation(
            invariant,
            target,
            *kind,
            account,
            asset.as_ref(),
            facet.as_ref(),
            violations,
        ),
        PropertyExpr::DispositionAcceptedEffective => match target {
            EvaluationTarget::Transition { transaction, .. } => {
                if transaction.disposition == EconomicDisposition::AcceptedEffective {
                    Ok(())
                } else {
                    let v = make_violation(
                        invariant,
                        EvaluationLocation::Transition {
                            disposition: Some(format!("{:?}", transaction.disposition)),
                        },
                        vec![fact("required", "AcceptedEffective")],
                        vec![fact("observed", format!("{:?}", transaction.disposition))],
                    );
                    record_violation(invariant.violation_policy, violations, v);
                    Ok(())
                }
            }
            _ => Err(InvariantError::new(
                InvariantErrorClass::IncompatibleTarget,
                "DispositionAcceptedEffective requires Transition target",
            )),
        },
        PropertyExpr::Unsupported { reason } => Err(InvariantError::new(
            InvariantErrorClass::UnsupportedOperation,
            reason.clone(),
        )),
    }
}

fn record_violation(policy: ViolationPolicy, violations: &mut Vec<Violation>, v: Violation) {
    match policy {
        ViolationPolicy::All => violations.push(v),
        ViolationPolicy::FirstOnly => {
            if violations.is_empty() {
                violations.push(v);
            }
        }
    }
}

#[derive(Debug, Clone)]
enum DomainMember {
    Account(AccountId),
    HistoryOrdinal(usize),
    Facet(FacetId),
}

fn apply_binding(bindings: &mut Bindings, member: &DomainMember) {
    match member {
        DomainMember::Account(id) => bindings.account = Some(id.clone()),
        DomainMember::HistoryOrdinal(i) => bindings.history_ordinal = Some(*i),
        DomainMember::Facet(_) => {}
    }
}

fn clear_binding(bindings: &mut Bindings, member: &DomainMember) {
    match member {
        DomainMember::Account(_) => bindings.account = None,
        DomainMember::HistoryOrdinal(_) => bindings.history_ordinal = None,
        DomainMember::Facet(_) => {}
    }
}

fn resolve_domain(
    invariant: &Invariant,
    target: EvaluationTarget<'_>,
    domain: &DomainExpr,
    _bindings: &Bindings,
) -> Result<Vec<DomainMember>, InvariantError> {
    match domain {
        DomainExpr::WorldAccounts => Ok(target
            .world()
            .accounts
            .keys()
            .cloned()
            .map(DomainMember::Account)
            .collect()),
        DomainExpr::Accounts(ids) => Ok(ids.iter().cloned().map(DomainMember::Account).collect()),
        DomainExpr::HistoryPresent | DomainExpr::HistoryRange { .. } => {
            let ordered = select_history_domain(invariant, target, domain)?;
            Ok(ordered
                .into_iter()
                .map(DomainMember::HistoryOrdinal)
                .collect())
        }
        DomainExpr::PresentFacetsOf { account, asset } => {
            let state = require_state(target)?;
            let mut facets = Vec::new();
            for facet in &target.world().facets {
                if state.has_balance(account, asset, facet) {
                    facets.push(DomainMember::Facet(facet.clone()));
                }
            }
            Ok(facets)
        }
    }
}

fn select_history_domain(
    invariant: &Invariant,
    target: EvaluationTarget<'_>,
    domain: &DomainExpr,
) -> Result<Vec<usize>, InvariantError> {
    let EvaluationTarget::History { records, .. } = target else {
        return Err(InvariantError::new(
            InvariantErrorClass::IncompatibleTarget,
            "history domain requires History target",
        ));
    };
    ensure_total_order(records)?;
    let selected = match domain {
        DomainExpr::HistoryPresent => {
            let mut idxs: Vec<usize> = (0..records.len()).collect();
            idxs.sort_by_key(|i| records[*i].logical_position);
            idxs
        }
        DomainExpr::HistoryRange {
            start,
            end,
            require_contiguous,
        } => {
            if start > end {
                return Err(InvariantError::new(
                    InvariantErrorClass::InvalidInvariantDefinition,
                    "history range start > end",
                ));
            }
            let mut idxs: Vec<usize> = records
                .iter()
                .enumerate()
                .filter(|(_, r)| r.logical_position >= *start && r.logical_position <= *end)
                .map(|(i, _)| i)
                .collect();
            idxs.sort_by_key(|i| records[*i].logical_position);
            if *require_contiguous {
                let mut pos = *start;
                while pos <= *end {
                    if !idxs.iter().any(|i| records[*i].logical_position == pos) {
                        return Err(InvariantError::new(
                            InvariantErrorClass::MissingRequiredData,
                            format!("missing required history logical position {pos}"),
                        ));
                    }
                    pos += 1;
                }
            }
            idxs
        }
        _ => {
            return Err(InvariantError::new(
                InvariantErrorClass::InvalidInvariantDefinition,
                "not a history domain",
            ));
        }
    };

    // Apply direction: reverse uses reverse ordinals for indexing via domain member order.
    let mut ordered = selected;
    if invariant.history_reverse {
        ordered.reverse();
    }
    Ok(ordered)
}

fn ensure_total_order(records: &[HistoryRecord]) -> Result<(), InvariantError> {
    let mut seen = std::collections::BTreeSet::new();
    for r in records {
        if !seen.insert(r.logical_position) {
            return Err(InvariantError::new(
                InvariantErrorClass::MissingRequiredData,
                format!(
                    "duplicate logical ordering key {} without tie-break",
                    r.logical_position
                ),
            ));
        }
    }
    Ok(())
}

fn require_state(target: EvaluationTarget<'_>) -> Result<&EconomicState, InvariantError> {
    match target {
        EvaluationTarget::State { state, .. } => Ok(state),
        EvaluationTarget::Transition { state_after, .. } => Ok(state_after),
        EvaluationTarget::History { .. } => Err(InvariantError::new(
            InvariantErrorClass::IncompatibleTarget,
            "state lookup requires State or Transition target",
        )),
    }
}

fn eval_value(
    invariant: &Invariant,
    target: EvaluationTarget<'_>,
    expr: &ValueExpr,
    bindings: &Bindings,
) -> Result<ScalarValue, InvariantError> {
    match expr {
        ValueExpr::Literal(m) => Ok(ScalarValue::Money(m.clone())),
        ValueExpr::CountLiteral(n) => Ok(ScalarValue::Count(Count::new(*n))),
        ValueExpr::Balance {
            account,
            asset,
            facet,
        } => Ok(ScalarValue::Money(lookup_balance_required(
            target, account, asset, facet,
        )?)),
        ValueExpr::BoundAccountBalance { asset, facet } => {
            let account = bindings.account.as_ref().ok_or_else(|| {
                InvariantError::new(
                    InvariantErrorClass::InvalidInvariantDefinition,
                    "bound account balance without account binding",
                )
            })?;
            Ok(ScalarValue::Money(lookup_balance_required(
                target, account, asset, facet,
            )?))
        }
        ValueExpr::Aggregate {
            op,
            domain,
            of,
            declared_asset,
        } => eval_aggregate(
            invariant,
            target,
            *op,
            domain,
            of.as_deref(),
            declared_asset.as_ref(),
            bindings,
        ),
        ValueExpr::Convert { amount, price_id } => {
            let base = eval_value(invariant, target, amount, bindings)?;
            let base_amt = match base {
                ScalarValue::Money(m) => m,
                ScalarValue::Count(_) => {
                    return Err(InvariantError::new(
                        InvariantErrorClass::IncompatibleOperands,
                        "cannot convert COUNT through a price",
                    ));
                }
            };
            Ok(ScalarValue::Money(convert_amount(
                target.world(),
                &base_amt,
                price_id,
            )?))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ScalarValue {
    Money(Money),
    Count(Count),
}

fn lookup_balance_required(
    target: EvaluationTarget<'_>,
    account: &AccountId,
    asset: &AssetId,
    facet: &FacetId,
) -> Result<Money, InvariantError> {
    let world = target.world();
    if !world.accounts.contains_key(account) {
        return Err(InvariantError::new(
            InvariantErrorClass::MissingRequiredData,
            format!("missing account {}", account.as_str()),
        ));
    }
    if !world.assets.contains_key(asset) {
        return Err(InvariantError::new(
            InvariantErrorClass::InvalidInvariantConfiguration,
            format!("asset {} not declared in World", asset.as_str()),
        ));
    }
    if !world.facets.contains(facet) {
        return Err(InvariantError::new(
            InvariantErrorClass::InvalidInvariantConfiguration,
            format!("facet {} not declared in World", facet.as_str()),
        ));
    }
    let state = require_state(target)?;
    match state.balance_if_present(account, asset, facet) {
        Some(v) => Ok(Money::new(asset.clone(), v)),
        None => Err(InvariantError::new(
            InvariantErrorClass::MissingRequiredData,
            format!(
                "missing balance cell {}/{}/{}",
                account.as_str(),
                asset.as_str(),
                facet.as_str()
            ),
        )),
    }
}

fn eval_aggregate(
    invariant: &Invariant,
    target: EvaluationTarget<'_>,
    op: AggOp,
    domain: &DomainExpr,
    of: Option<&ValueExpr>,
    declared_asset: Option<&AssetId>,
    bindings: &Bindings,
) -> Result<ScalarValue, InvariantError> {
    let members = resolve_domain(invariant, target, domain, bindings)?;
    if matches!(op, AggOp::Count) {
        let n = i128::try_from(members.len()).map_err(|_| {
            InvariantError::new(InvariantErrorClass::ArithmeticError, "COUNT overflow i128")
        })?;
        return Ok(ScalarValue::Count(Count::new(n)));
    }

    if members.is_empty() {
        return match op {
            AggOp::Sum => {
                let asset = declared_asset.ok_or_else(|| {
                    InvariantError::new(
                        InvariantErrorClass::InvalidInvariantDefinition,
                        "empty SUM requires declared_asset",
                    )
                })?;
                Ok(ScalarValue::Money(Money::new(asset.clone(), 0)))
            }
            AggOp::Min | AggOp::Max => Err(InvariantError::new(
                InvariantErrorClass::ArithmeticError,
                "MIN/MAX over empty domain",
            )),
            AggOp::Count => unreachable!(),
        };
    }

    let of = of.ok_or_else(|| {
        InvariantError::new(
            InvariantErrorClass::InvalidInvariantDefinition,
            "aggregate Sum/Min/Max requires of expression",
        )
    })?;

    let mut values = Vec::new();
    let mut local_bindings = bindings.clone();
    for member in members {
        apply_binding(&mut local_bindings, &member);
        let v = match (&member, of) {
            (DomainMember::Facet(facet), ValueExpr::Balance { account, asset, .. }) => {
                lookup_balance_required(target, account, asset, facet)?
            }
            _ => match eval_value(invariant, target, of, &local_bindings)? {
                ScalarValue::Money(m) => m,
                ScalarValue::Count(_) => {
                    return Err(InvariantError::new(
                        InvariantErrorClass::IncompatibleOperands,
                        "SUM/MIN/MAX cannot aggregate COUNT values as Money",
                    ));
                }
            },
        };
        values.push(v);
        clear_binding(&mut local_bindings, &member);
    }

    match op {
        AggOp::Sum => {
            let mut acc = values[0].clone();
            for v in values.iter().skip(1) {
                acc = acc.checked_add(v).map_err(|e| match e {
                    crate::kernel::error::ArithError::Overflow => {
                        InvariantError::new(InvariantErrorClass::ArithmeticError, "SUM overflow")
                    }
                    crate::kernel::error::ArithError::AssetMismatch { .. } => InvariantError::new(
                        InvariantErrorClass::IncompatibleOperands,
                        "SUM mixed assets without conversion",
                    ),
                })?;
            }
            Ok(ScalarValue::Money(acc))
        }
        AggOp::Min => {
            let mut best = values[0].clone();
            for v in values.iter().skip(1) {
                if compare_money(v, CmpOp::Lt, &best)? {
                    best = v.clone();
                }
            }
            Ok(ScalarValue::Money(best))
        }
        AggOp::Max => {
            let mut best = values[0].clone();
            for v in values.iter().skip(1) {
                if compare_money(v, CmpOp::Gt, &best)? {
                    best = v.clone();
                }
            }
            Ok(ScalarValue::Money(best))
        }
        AggOp::Count => unreachable!(),
    }
}

fn compare_scalars(
    left: &ScalarValue,
    op: CmpOp,
    right: &ScalarValue,
) -> Result<bool, InvariantError> {
    match (left, right) {
        (ScalarValue::Money(l), ScalarValue::Money(r)) => compare_money(l, op, r),
        (ScalarValue::Count(l), ScalarValue::Count(r)) => {
            let lv = l.value();
            let rv = r.value();
            Ok(match op {
                CmpOp::Eq => lv == rv,
                CmpOp::Ne => lv != rv,
                CmpOp::Lt => lv < rv,
                CmpOp::Le => lv <= rv,
                CmpOp::Gt => lv > rv,
                CmpOp::Ge => lv >= rv,
            })
        }
        (ScalarValue::Money(_), ScalarValue::Count(_))
        | (ScalarValue::Count(_), ScalarValue::Money(_)) => Err(InvariantError::new(
            InvariantErrorClass::IncompatibleOperands,
            "cannot compare Money with dimensionless COUNT",
        )),
    }
}

fn convert_amount(
    world: &EconomicWorld,
    amount: &Money,
    price_id: &PriceId,
) -> Result<Money, InvariantError> {
    let price = world.prices.get(price_id).ok_or_else(|| {
        InvariantError::new(
            InvariantErrorClass::MissingRequiredData,
            format!("missing price {}", price_id.as_str()),
        )
    })?;
    if amount.asset() != &price.base {
        return Err(InvariantError::new(
            InvariantErrorClass::IncompatibleOperands,
            "convert amount asset is not price base",
        ));
    }
    // quote = amount * quote_per_base / base_units with declared rounding
    let numer = amount
        .value()
        .checked_mul(price.quote_per_base)
        .ok_or_else(|| {
            InvariantError::new(InvariantErrorClass::ArithmeticError, "conversion overflow")
        })?;
    let quote = divide_with_rounding(numer, price.base_units, price.rounding).map_err(|_| {
        InvariantError::new(
            InvariantErrorClass::ArithmeticError,
            "conversion division error",
        )
    })?;
    Ok(Money::new(price.quote.clone(), quote))
}

fn compare_money(left: &Money, op: CmpOp, right: &Money) -> Result<bool, InvariantError> {
    if left.asset() != right.asset() {
        return Err(InvariantError::new(
            InvariantErrorClass::IncompatibleOperands,
            format!(
                "cross-asset compare {} vs {} without conversion",
                left.asset().as_str(),
                right.asset().as_str()
            ),
        ));
    }
    let l = left.value();
    let r = right.value();
    Ok(match op {
        CmpOp::Eq => l == r,
        CmpOp::Ne => l != r,
        CmpOp::Lt => l < r,
        CmpOp::Le => l <= r,
        CmpOp::Gt => l > r,
        CmpOp::Ge => l >= r,
    })
}

fn eval_relation(
    invariant: &Invariant,
    target: EvaluationTarget<'_>,
    kind: RelationKind,
    account: &AccountId,
    asset: Option<&AssetId>,
    facet: Option<&FacetId>,
    violations: &mut Vec<Violation>,
) -> Result<(), InvariantError> {
    match kind {
        RelationKind::TransactionActorOwnsAccount => {
            let EvaluationTarget::Transition {
                world, transaction, ..
            } = target
            else {
                return Err(InvariantError::new(
                    InvariantErrorClass::IncompatibleTarget,
                    "TransactionActorOwnsAccount requires Transition",
                ));
            };
            let acct = world.accounts.get(account).ok_or_else(|| {
                InvariantError::new(
                    InvariantErrorClass::MissingRequiredData,
                    format!("missing account {}", account.as_str()),
                )
            })?;
            // Authoritative relation: transaction.actor == account.owner
            if transaction.actor.as_str().is_empty() {
                return Err(InvariantError::new(
                    InvariantErrorClass::MissingRequiredData,
                    "TransactionActorOwnsAccount requires transaction.actor",
                ));
            }
            if transaction.actor == acct.owner {
                Ok(())
            } else {
                let v = make_violation(
                    invariant,
                    EvaluationLocation::Transition {
                        disposition: Some(format!("{:?}", transaction.disposition)),
                    },
                    vec![fact(
                        "required",
                        format!(
                            "transaction.actor ({}) == account.owner ({})",
                            transaction.actor.as_str(),
                            acct.owner.as_str()
                        ),
                    )],
                    vec![fact("observed", "actor/owner mismatch")],
                );
                record_violation(invariant.violation_policy, violations, v);
                Ok(())
            }
        }
        RelationKind::BeforeAfterBalanceViaEffects => {
            let EvaluationTarget::Transition {
                state_before,
                state_after,
                transaction,
                ..
            } = target
            else {
                return Err(InvariantError::new(
                    InvariantErrorClass::IncompatibleTarget,
                    "BeforeAfterBalanceViaEffects requires Transition",
                ));
            };
            let asset = asset.ok_or_else(|| {
                InvariantError::new(
                    InvariantErrorClass::InvalidInvariantDefinition,
                    "BeforeAfterBalanceViaEffects requires asset",
                )
            })?;
            let facet = facet.ok_or_else(|| {
                InvariantError::new(
                    InvariantErrorClass::InvalidInvariantDefinition,
                    "BeforeAfterBalanceViaEffects requires facet",
                )
            })?;
            let before = state_before
                .balance_if_present(account, asset, facet)
                .ok_or_else(|| {
                    InvariantError::new(
                        InvariantErrorClass::MissingRequiredData,
                        "missing StateBefore balance",
                    )
                })?;
            let after = state_after
                .balance_if_present(account, asset, facet)
                .ok_or_else(|| {
                    InvariantError::new(
                        InvariantErrorClass::MissingRequiredData,
                        "missing StateAfter balance",
                    )
                })?;
            let mut delta = 0_i128;
            for e in &transaction.effects {
                if &e.account == account && &e.asset == asset && &e.facet == facet {
                    delta = delta.checked_add(e.delta).ok_or_else(|| {
                        InvariantError::new(
                            InvariantErrorClass::ArithmeticError,
                            "effect delta overflow",
                        )
                    })?;
                }
            }
            let expected = before.checked_add(delta).ok_or_else(|| {
                InvariantError::new(
                    InvariantErrorClass::ArithmeticError,
                    "before+delta overflow",
                )
            })?;
            if expected == after {
                Ok(())
            } else {
                let v = make_violation(
                    invariant,
                    EvaluationLocation::Transition {
                        disposition: Some(format!("{:?}", transaction.disposition)),
                    },
                    vec![fact(
                        "required",
                        format!("StateAfter == StateBefore + effects ({expected})"),
                    )],
                    vec![fact("observed", format!("StateAfter={after}"))],
                );
                record_violation(invariant.violation_policy, violations, v);
                Ok(())
            }
        }
    }
}

fn make_violation(
    invariant: &Invariant,
    location: EvaluationLocation,
    required: Vec<StructuredFact>,
    observed: Vec<StructuredFact>,
) -> Violation {
    Violation {
        invariant_id: invariant.id.clone(),
        definition_version: invariant.definition_version.clone(),
        scope: invariant.scope,
        location,
        required_condition: required,
        observed_condition: observed,
        authoritative_reference: Vec::new(),
        context: Vec::new(),
    }
}

fn fact(key: impl Into<String>, value: impl Into<String>) -> StructuredFact {
    StructuredFact {
        key: key.into(),
        value: value.into(),
    }
}

fn location_from_bindings(bindings: &Bindings) -> EvaluationLocation {
    if let Some(ref a) = bindings.account {
        EvaluationLocation::State {
            account: Some(a.as_str().to_owned()),
            asset: None,
            facet: None,
        }
    } else if let Some(ord) = bindings.history_ordinal {
        EvaluationLocation::HistoryOrdinal {
            ordinal: ord,
            direction: "bound".into(),
        }
    } else {
        EvaluationLocation::Invariant
    }
}

/// Resolve ordinal index into a history record reference.
///
/// Evaluation order: resolve range → select → apply direction → apply index.
///
/// # Errors
///
/// Negative index → `InvalidInvariantDefinition`; out of range → `MissingRequiredData`.
pub fn resolve_history_index<'a>(
    invariant: &Invariant,
    target: EvaluationTarget<'a>,
    domain: &DomainExpr,
    index: i64,
) -> Result<&'a crate::invariant::history::HistoryRecord, InvariantError> {
    if index < 0 {
        return Err(InvariantError::new(
            InvariantErrorClass::InvalidInvariantDefinition,
            "negative history ordinal index",
        ));
    }
    let ordered = select_history_domain(invariant, target, domain)?;
    let idx = usize::try_from(index).map_err(|_| {
        InvariantError::new(
            InvariantErrorClass::InvalidInvariantDefinition,
            "index conversion failed",
        )
    })?;
    if idx >= ordered.len() {
        return Err(InvariantError::new(
            InvariantErrorClass::MissingRequiredData,
            format!("history ordinal index {index} out of range"),
        ));
    }
    let EvaluationTarget::History { records, .. } = target else {
        return Err(InvariantError::new(
            InvariantErrorClass::IncompatibleTarget,
            "history index requires History target",
        ));
    };
    Ok(&records[ordered[idx]])
}
