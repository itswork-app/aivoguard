//! M07 observation validation primitives (Phase 3).
//!
//! Implements frozen Gate-6 status↔phase matrix, R/O/A field matrix,
//! Settled / PartiallySettled amount invariants, and observation amount-domain
//! checks. Does **not** evaluate expectations, fees, M06 composition, or pins.

use crate::payment_settlement::error::{PaymentSettlementError, PaymentSettlementErrorId};
use crate::payment_settlement::types::{
    PaymentSettlementBinding, RefundOutcome, SettlementDeclaration, SettlementObservation,
    SettlementPhase, SettlementStatus,
};

/// Field presence class from the frozen §11.5 matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Presence {
    /// Must be `Some`.
    Required,
    /// `Some` or `None` permitted (subject to other rules).
    Optional,
    /// Must be `None`.
    Absent,
}

/// Whether `(status, phase)` is in the frozen §11.4 closed matrix.
#[must_use]
#[allow(clippy::unnested_or_patterns)]
pub fn is_valid_status_phase(status: SettlementStatus, phase: SettlementPhase) -> bool {
    matches!(
        (status, phase),
        (
            SettlementStatus::Pending,
            SettlementPhase::Declare
                | SettlementPhase::Authorize
                | SettlementPhase::Capture
                | SettlementPhase::Settle
        ) | (SettlementStatus::Authorized, SettlementPhase::Authorize)
            | (SettlementStatus::Captured, SettlementPhase::Capture)
            | (
                SettlementStatus::Settled | SettlementStatus::PartiallySettled,
                SettlementPhase::Settle | SettlementPhase::Complete
            )
            | (SettlementStatus::Failed, SettlementPhase::Settle)
            | (SettlementStatus::Reversed, SettlementPhase::Reverse)
            | (
                SettlementStatus::Refunded | SettlementStatus::PartiallyRefunded,
                SettlementPhase::Refund | SettlementPhase::Complete
            )
    )
}

/// Validate status↔phase compatibility (closed matrix).
///
/// Closed Rust enums cannot encode an "unknown" status/phase; invalid pairs
/// classify as [`PaymentSettlementErrorId::InvalidSettlementObservation`].
/// [`PaymentSettlementErrorId::UnsupportedSettlementState`] remains reserved for
/// values outside the taxonomy (e.g. adapter raw mapping in a later phase).
///
/// # Errors
///
/// Returns [`PaymentSettlementError`] when the pair is not in the frozen matrix.
pub fn validate_status_phase(
    status: SettlementStatus,
    phase: SettlementPhase,
) -> Result<(), PaymentSettlementError> {
    if is_valid_status_phase(status, phase) {
        Ok(())
    } else {
        Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidSettlementObservation,
            "status/phase pair is not in the frozen Gate-6 matrix",
        ))
    }
}

fn presence_matrix(status: SettlementStatus) -> FieldPresenceRow {
    match status {
        SettlementStatus::Pending => FieldPresenceRow {
            requested: Presence::Optional,
            settled: Presence::Absent,
            unsettled: Presence::Absent,
            fee: Presence::Optional,
            net: Presence::Optional,
            refund_amount: Presence::Absent,
            refund_outcome: Presence::Absent,
            escrow: Presence::Optional,
            delay: Presence::Optional,
            m07_label: Presence::Optional,
        },
        SettlementStatus::Authorized | SettlementStatus::Captured => FieldPresenceRow {
            requested: Presence::Optional,
            settled: Presence::Absent,
            unsettled: Presence::Absent,
            fee: Presence::Optional,
            net: Presence::Optional,
            refund_amount: Presence::Absent,
            refund_outcome: Presence::Absent,
            escrow: Presence::Optional,
            delay: Presence::Absent,
            m07_label: Presence::Optional,
        },
        SettlementStatus::Settled => FieldPresenceRow {
            requested: Presence::Required,
            settled: Presence::Required,
            unsettled: Presence::Optional,
            fee: Presence::Optional,
            net: Presence::Optional,
            refund_amount: Presence::Absent,
            refund_outcome: Presence::Absent,
            escrow: Presence::Optional,
            delay: Presence::Absent,
            m07_label: Presence::Optional,
        },
        SettlementStatus::PartiallySettled => FieldPresenceRow {
            requested: Presence::Required,
            settled: Presence::Required,
            unsettled: Presence::Required,
            fee: Presence::Optional,
            net: Presence::Optional,
            refund_amount: Presence::Absent,
            refund_outcome: Presence::Absent,
            escrow: Presence::Optional,
            delay: Presence::Absent,
            m07_label: Presence::Optional,
        },
        SettlementStatus::Failed | SettlementStatus::Reversed => FieldPresenceRow {
            requested: Presence::Optional,
            settled: Presence::Optional,
            unsettled: Presence::Optional,
            fee: Presence::Optional,
            net: Presence::Optional,
            refund_amount: Presence::Absent,
            refund_outcome: Presence::Absent,
            escrow: Presence::Optional,
            delay: Presence::Absent,
            m07_label: Presence::Optional,
        },
        SettlementStatus::Refunded | SettlementStatus::PartiallyRefunded => FieldPresenceRow {
            requested: Presence::Optional,
            settled: Presence::Optional,
            unsettled: Presence::Optional,
            fee: Presence::Optional,
            net: Presence::Optional,
            refund_amount: Presence::Required,
            refund_outcome: Presence::Required,
            escrow: Presence::Optional,
            delay: Presence::Absent,
            m07_label: Presence::Optional,
        },
    }
}

struct FieldPresenceRow {
    requested: Presence,
    settled: Presence,
    unsettled: Presence,
    fee: Presence,
    net: Presence,
    refund_amount: Presence,
    refund_outcome: Presence,
    escrow: Presence,
    delay: Presence,
    m07_label: Presence,
}

fn reject_if_absent_populated<T>(
    presence: Presence,
    value: Option<&T>,
    field: &str,
) -> Result<(), PaymentSettlementError> {
    if matches!(presence, Presence::Absent) && value.is_some() {
        return Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidSettlementObservation,
            format!("field `{field}` is A (MUST be absent) but is Some(_)"),
        ));
    }
    Ok(())
}

fn reject_if_required_missing<T>(
    presence: Presence,
    value: Option<&T>,
    field: &str,
) -> Result<(), PaymentSettlementError> {
    if matches!(presence, Presence::Required) && value.is_none() {
        return Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidSettlementObservation,
            format!("field `{field}` is R (required) but is None"),
        ));
    }
    Ok(())
}

fn validate_field_matrix(obs: &SettlementObservation) -> Result<(), PaymentSettlementError> {
    let row = presence_matrix(obs.settlement_status);

    // A: Some(_) forbidden (deterministic field order).
    reject_if_absent_populated(row.settled, obs.settled_amount.as_ref(), "settled_amount")?;
    reject_if_absent_populated(
        row.unsettled,
        obs.unsettled_amount.as_ref(),
        "unsettled_amount",
    )?;
    reject_if_absent_populated(
        row.refund_amount,
        obs.refund_amount.as_ref(),
        "refund_amount",
    )?;
    reject_if_absent_populated(
        row.refund_outcome,
        obs.refund_outcome.as_ref(),
        "refund_outcome",
    )?;
    if matches!(row.delay, Presence::Absent) {
        reject_if_absent_populated(Presence::Absent, obs.delay_marker.as_ref(), "delay_marker")?;
        reject_if_absent_populated(
            Presence::Absent,
            obs.settlement_delay_steps.as_ref(),
            "settlement_delay_steps",
        )?;
    }
    if matches!(row.escrow, Presence::Absent) {
        reject_if_absent_populated(
            Presence::Absent,
            obs.escrow_status.as_ref(),
            "escrow_status",
        )?;
        reject_if_absent_populated(
            Presence::Absent,
            obs.escrow_amount.as_ref(),
            "escrow_amount",
        )?;
    }

    // R: None forbidden.
    reject_if_required_missing(
        row.requested,
        obs.requested_amount.as_ref(),
        "requested_amount",
    )?;
    reject_if_required_missing(row.settled, obs.settled_amount.as_ref(), "settled_amount")?;
    reject_if_required_missing(
        row.unsettled,
        obs.unsettled_amount.as_ref(),
        "unsettled_amount",
    )?;
    reject_if_required_missing(row.fee, obs.fee_amount.as_ref(), "fee_amount")?;
    reject_if_required_missing(
        row.net,
        obs.net_settlement_amount.as_ref(),
        "net_settlement_amount",
    )?;
    reject_if_required_missing(
        row.refund_amount,
        obs.refund_amount.as_ref(),
        "refund_amount",
    )?;
    reject_if_required_missing(
        row.refund_outcome,
        obs.refund_outcome.as_ref(),
        "refund_outcome",
    )?;
    reject_if_required_missing(row.m07_label, obs.m07_label.as_ref(), "m07_label")?;

    Ok(())
}

fn reject_negative(value: i128, field: &str) -> Result<(), PaymentSettlementError> {
    if value < 0 {
        return Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidSettlementObservation,
            format!("observation `{field}` must be >= 0"),
        ));
    }
    Ok(())
}

fn validate_amount_domain(obs: &SettlementObservation) -> Result<(), PaymentSettlementError> {
    if let Some(v) = obs.requested_amount {
        reject_negative(v, "requested_amount")?;
    }
    if let Some(v) = obs.settled_amount {
        reject_negative(v, "settled_amount")?;
    }
    if let Some(v) = obs.unsettled_amount {
        reject_negative(v, "unsettled_amount")?;
    }
    if let Some(v) = obs.fee_amount {
        reject_negative(v, "fee_amount")?;
    }
    if let Some(v) = obs.refund_amount {
        reject_negative(v, "refund_amount")?;
    }
    if let Some(v) = obs.escrow_amount {
        reject_negative(v, "escrow_amount")?;
    }
    // net_settlement_amount MAY be signed — no sign rejection.
    Ok(())
}

fn validate_refund_outcome_coupling(
    obs: &SettlementObservation,
) -> Result<(), PaymentSettlementError> {
    match obs.settlement_status {
        SettlementStatus::Refunded => match obs.refund_outcome {
            Some(RefundOutcome::SucceededFull) => Ok(()),
            Some(_) => Err(PaymentSettlementError::new(
                PaymentSettlementErrorId::InvalidSettlementObservation,
                "Refunded requires refund_outcome = SucceededFull",
            )),
            None => Err(PaymentSettlementError::new(
                PaymentSettlementErrorId::InvalidSettlementObservation,
                "Refunded requires refund_outcome present",
            )),
        },
        SettlementStatus::PartiallyRefunded => match obs.refund_outcome {
            Some(RefundOutcome::SucceededPartial) => Ok(()),
            Some(_) => Err(PaymentSettlementError::new(
                PaymentSettlementErrorId::InvalidSettlementObservation,
                "PartiallyRefunded requires refund_outcome = SucceededPartial",
            )),
            None => Err(PaymentSettlementError::new(
                PaymentSettlementErrorId::InvalidSettlementObservation,
                "PartiallyRefunded requires refund_outcome present",
            )),
        },
        _ => Ok(()),
    }
}

fn validate_settled_invariant(obs: &SettlementObservation) -> Result<(), PaymentSettlementError> {
    let requested = obs.requested_amount.ok_or_else(|| {
        PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidSettlementObservation,
            "Settled requires requested_amount",
        )
    })?;
    let settled = obs.settled_amount.ok_or_else(|| {
        PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidSettlementObservation,
            "Settled requires settled_amount",
        )
    })?;
    if settled != requested {
        return Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidSettlementObservation,
            "Settled requires settled_amount == requested_amount",
        ));
    }
    if let Some(unsettled) = obs.unsettled_amount {
        if unsettled != 0 {
            return Err(PaymentSettlementError::new(
                PaymentSettlementErrorId::InvalidSettlementObservation,
                "Settled unsettled_amount when present MUST equal 0",
            ));
        }
    }
    Ok(())
}

fn validate_partially_settled_invariant(
    obs: &SettlementObservation,
) -> Result<(), PaymentSettlementError> {
    // 1. required fields (also enforced by matrix; re-check for local order)
    let requested = obs.requested_amount.ok_or_else(|| {
        PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidSettlementObservation,
            "PartiallySettled requires requested_amount",
        )
    })?;
    let settled = obs.settled_amount.ok_or_else(|| {
        PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidSettlementObservation,
            "PartiallySettled requires settled_amount",
        )
    })?;
    let unsettled = obs.unsettled_amount.ok_or_else(|| {
        PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidSettlementObservation,
            "PartiallySettled requires unsettled_amount",
        )
    })?;

    // 2. non-negative amount-domain (already checked globally; keep order local)
    reject_negative(requested, "requested_amount")?;
    reject_negative(settled, "settled_amount")?;
    reject_negative(unsettled, "unsettled_amount")?;

    // 3. strict inequalities
    if settled <= 0 {
        return Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidSettlementObservation,
            "PartiallySettled requires settled_amount > 0",
        ));
    }
    if settled >= requested {
        return Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidSettlementObservation,
            "PartiallySettled requires settled_amount < requested_amount",
        ));
    }
    if unsettled <= 0 {
        return Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidSettlementObservation,
            "PartiallySettled requires unsettled_amount > 0",
        ));
    }

    // 4. checked_sub
    let derived = requested.checked_sub(settled).ok_or_else(|| {
        PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidSettlementObservation,
            "PartiallySettled checked_sub(requested, settled) failed",
        )
    })?;

    // 5. equality
    if unsettled != derived {
        return Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidSettlementObservation,
            "PartiallySettled requires unsettled_amount == checked_sub(requested, settled)",
        ));
    }

    Ok(())
}

fn validate_amount_consistency_when_all_present(
    obs: &SettlementObservation,
) -> Result<(), PaymentSettlementError> {
    // Frozen §11.5: if unsettled and both requested and settled are present →
    // unsettled MUST equal checked_sub(requested, settled).
    // PartiallySettled already enforced this; Settled with unsettled=0 also holds
    // when settled==requested. Apply for any status when all three are Some.
    let (Some(requested), Some(settled), Some(unsettled)) = (
        obs.requested_amount,
        obs.settled_amount,
        obs.unsettled_amount,
    ) else {
        return Ok(());
    };

    if matches!(obs.settlement_status, SettlementStatus::PartiallySettled) {
        // Already validated in dedicated path with mandatory order.
        return Ok(());
    }

    let derived = requested.checked_sub(settled).ok_or_else(|| {
        PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidSettlementObservation,
            "checked_sub(requested, settled) failed for amount consistency",
        )
    })?;
    if unsettled != derived {
        return Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidSettlementObservation,
            "unsettled_amount must equal checked_sub(requested_amount, settled_amount)",
        ));
    }
    Ok(())
}

/// Validate a single [`SettlementObservation`] against frozen Gate-6 structural rules.
///
/// Deterministic order:
/// 1. status↔phase
/// 2. R/O/A field matrix
/// 3. observation amount domain
/// 4. refund outcome coupling
/// 5. status-specific amount invariants
/// 6. cross-field amount consistency when all three amounts present
///
/// # Errors
///
/// Returns [`PaymentSettlementError`] on the first violated rule.
pub fn validate_settlement_observation(
    obs: &SettlementObservation,
) -> Result<(), PaymentSettlementError> {
    // Structurally required non-Option fields (binding, status, execution_state, phase)
    // are always present in the type.
    validate_status_phase(obs.settlement_status, obs.phase)?;
    validate_field_matrix(obs)?;
    validate_amount_domain(obs)?;
    validate_refund_outcome_coupling(obs)?;

    match obs.settlement_status {
        SettlementStatus::Settled => validate_settled_invariant(obs)?,
        SettlementStatus::PartiallySettled => validate_partially_settled_invariant(obs)?,
        _ => {}
    }

    validate_amount_consistency_when_all_present(obs)?;
    Ok(())
}

/// Exact case ↔ observation binding compare (§13 / §14 step 4).
///
/// # Errors
///
/// Returns [`PaymentSettlementErrorId::ObservationBindingMismatch`] when any
/// binding field differs (no normalization).
pub fn validate_case_observation_binding(
    case_binding: &PaymentSettlementBinding,
    observation_binding: &PaymentSettlementBinding,
) -> Result<(), PaymentSettlementError> {
    if case_binding == observation_binding {
        Ok(())
    } else {
        Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::ObservationBindingMismatch,
            format!(
                "case binding ≠ observation binding: case=`{}`/`{}`/`{}` obs=`{}`/`{}`/`{}`",
                case_binding.payment_id,
                case_binding.payment_version,
                case_binding.configuration_id,
                observation_binding.payment_id,
                observation_binding.payment_version,
                observation_binding.configuration_id
            ),
        ))
    }
}

/// Settled / PartiallySettled requested-amount case binding (R-A1).
///
/// # Errors
///
/// Returns [`PaymentSettlementErrorId::InvalidSettlementObservation`] when the
/// observed requested amount differs from
/// `settlement_declaration.requested_amount`.
pub fn validate_requested_amount_case_binding(
    obs: &SettlementObservation,
    declaration: &SettlementDeclaration,
) -> Result<(), PaymentSettlementError> {
    match obs.settlement_status {
        SettlementStatus::Settled | SettlementStatus::PartiallySettled => {
            let Some(observed) = obs.requested_amount else {
                return Err(PaymentSettlementError::new(
                    PaymentSettlementErrorId::InvalidSettlementObservation,
                    "Settled/PartiallySettled requires observation.requested_amount for R-A1",
                ));
            };
            if observed != declaration.requested_amount {
                return Err(PaymentSettlementError::new(
                    PaymentSettlementErrorId::InvalidSettlementObservation,
                    format!(
                        "observation.requested_amount ({observed}) ≠ settlement_declaration.requested_amount ({})",
                        declaration.requested_amount
                    ),
                ));
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::payment_settlement::types::{
        PaymentSettlementBinding, SettlementExecutionState, SettlementObservation,
    };

    fn binding() -> PaymentSettlementBinding {
        PaymentSettlementBinding {
            payment_id: "p1".into(),
            payment_version: "v1".into(),
            configuration_id: "c1".into(),
        }
    }

    fn base_obs(status: SettlementStatus, phase: SettlementPhase) -> SettlementObservation {
        SettlementObservation {
            binding: binding(),
            settlement_status: status,
            execution_state: SettlementExecutionState::NotAttempted,
            phase,
            refund_outcome: None,
            requested_amount: None,
            settled_amount: None,
            unsettled_amount: None,
            fee_amount: None,
            net_settlement_amount: None,
            refund_amount: None,
            escrow_status: None,
            escrow_amount: None,
            delay_marker: None,
            settlement_delay_steps: None,
            m07_label: None,
            simulation_result: None,
            adapter_provenance: None,
            notes: None,
        }
    }

    fn all_statuses() -> [SettlementStatus; 9] {
        [
            SettlementStatus::Pending,
            SettlementStatus::Authorized,
            SettlementStatus::Captured,
            SettlementStatus::Settled,
            SettlementStatus::PartiallySettled,
            SettlementStatus::Failed,
            SettlementStatus::Reversed,
            SettlementStatus::Refunded,
            SettlementStatus::PartiallyRefunded,
        ]
    }

    fn all_phases() -> [SettlementPhase; 7] {
        [
            SettlementPhase::Declare,
            SettlementPhase::Authorize,
            SettlementPhase::Capture,
            SettlementPhase::Settle,
            SettlementPhase::Refund,
            SettlementPhase::Reverse,
            SettlementPhase::Complete,
        ]
    }

    #[test]
    fn status_phase_matrix_exhaustive() {
        for status in all_statuses() {
            for phase in all_phases() {
                let ok = is_valid_status_phase(status, phase);
                let result = validate_status_phase(status, phase);
                if ok {
                    assert!(result.is_ok(), "{status:?}/{phase:?} should be valid");
                } else {
                    let err = result.expect_err("invalid pair");
                    assert_eq!(
                        err.error_id,
                        PaymentSettlementErrorId::InvalidSettlementObservation
                    );
                }
            }
        }
    }

    #[test]
    fn pending_rejects_settled_amount_present() {
        let mut obs = base_obs(SettlementStatus::Pending, SettlementPhase::Declare);
        obs.settled_amount = Some(1);
        let err = validate_settlement_observation(&obs).expect_err("A field");
        assert_eq!(
            err.error_id,
            PaymentSettlementErrorId::InvalidSettlementObservation
        );
    }

    #[test]
    fn pending_allows_optional_requested_absent() {
        let obs = base_obs(SettlementStatus::Pending, SettlementPhase::Declare);
        assert!(validate_settlement_observation(&obs).is_ok());
    }

    #[test]
    fn authorized_rejects_delay_present() {
        let mut obs = base_obs(SettlementStatus::Authorized, SettlementPhase::Authorize);
        obs.delay_marker = Some("d".into());
        assert!(validate_settlement_observation(&obs).is_err());
    }

    #[test]
    fn settled_valid_without_unsettled() {
        let mut obs = base_obs(SettlementStatus::Settled, SettlementPhase::Settle);
        obs.requested_amount = Some(100);
        obs.settled_amount = Some(100);
        obs.execution_state = SettlementExecutionState::Executed;
        assert!(validate_settlement_observation(&obs).is_ok());
    }

    #[test]
    fn settled_valid_with_unsettled_zero() {
        let mut obs = base_obs(SettlementStatus::Settled, SettlementPhase::Complete);
        obs.requested_amount = Some(100);
        obs.settled_amount = Some(100);
        obs.unsettled_amount = Some(0);
        obs.execution_state = SettlementExecutionState::Executed;
        assert!(validate_settlement_observation(&obs).is_ok());
    }

    #[test]
    fn settled_rejects_unequal_amounts() {
        let mut obs = base_obs(SettlementStatus::Settled, SettlementPhase::Settle);
        obs.requested_amount = Some(100);
        obs.settled_amount = Some(99);
        assert!(validate_settlement_observation(&obs).is_err());
    }

    #[test]
    fn settled_rejects_nonzero_unsettled() {
        let mut obs = base_obs(SettlementStatus::Settled, SettlementPhase::Settle);
        obs.requested_amount = Some(100);
        obs.settled_amount = Some(100);
        obs.unsettled_amount = Some(1);
        assert!(validate_settlement_observation(&obs).is_err());
    }

    #[test]
    fn settled_rejects_missing_requested() {
        let mut obs = base_obs(SettlementStatus::Settled, SettlementPhase::Settle);
        obs.settled_amount = Some(100);
        assert!(validate_settlement_observation(&obs).is_err());
    }

    #[test]
    fn partially_settled_valid_50_50() {
        let mut obs = base_obs(SettlementStatus::PartiallySettled, SettlementPhase::Settle);
        obs.requested_amount = Some(100);
        obs.settled_amount = Some(50);
        obs.unsettled_amount = Some(50);
        assert!(validate_settlement_observation(&obs).is_ok());
    }

    #[test]
    fn partially_settled_rejects_zero_settled() {
        let mut obs = base_obs(SettlementStatus::PartiallySettled, SettlementPhase::Settle);
        obs.requested_amount = Some(100);
        obs.settled_amount = Some(0);
        obs.unsettled_amount = Some(100);
        assert!(validate_settlement_observation(&obs).is_err());
    }

    #[test]
    fn partially_settled_rejects_full_settlement() {
        let mut obs = base_obs(SettlementStatus::PartiallySettled, SettlementPhase::Settle);
        obs.requested_amount = Some(100);
        obs.settled_amount = Some(100);
        obs.unsettled_amount = Some(0);
        assert!(validate_settlement_observation(&obs).is_err());
    }

    #[test]
    fn partially_settled_rejects_over_settlement() {
        let mut obs = base_obs(SettlementStatus::PartiallySettled, SettlementPhase::Settle);
        obs.requested_amount = Some(100);
        obs.settled_amount = Some(150);
        obs.unsettled_amount = Some(0);
        assert!(validate_settlement_observation(&obs).is_err());
    }

    #[test]
    fn partially_settled_rejects_zero_requested_with_positive_settled() {
        let mut obs = base_obs(SettlementStatus::PartiallySettled, SettlementPhase::Settle);
        obs.requested_amount = Some(0);
        obs.settled_amount = Some(1);
        obs.unsettled_amount = Some(0);
        assert!(validate_settlement_observation(&obs).is_err());
    }

    #[test]
    fn partially_settled_rejects_wrong_unsettled_low() {
        let mut obs = base_obs(SettlementStatus::PartiallySettled, SettlementPhase::Settle);
        obs.requested_amount = Some(100);
        obs.settled_amount = Some(50);
        obs.unsettled_amount = Some(49);
        assert!(validate_settlement_observation(&obs).is_err());
    }

    #[test]
    fn partially_settled_rejects_wrong_unsettled_high() {
        let mut obs = base_obs(SettlementStatus::PartiallySettled, SettlementPhase::Settle);
        obs.requested_amount = Some(100);
        obs.settled_amount = Some(50);
        obs.unsettled_amount = Some(51);
        assert!(validate_settlement_observation(&obs).is_err());
    }

    #[test]
    fn negative_settled_rejected() {
        let mut obs = base_obs(SettlementStatus::Failed, SettlementPhase::Settle);
        obs.settled_amount = Some(-1);
        assert!(validate_settlement_observation(&obs).is_err());
    }

    #[test]
    fn negative_net_settlement_allowed() {
        let mut obs = base_obs(SettlementStatus::Failed, SettlementPhase::Settle);
        obs.net_settlement_amount = Some(-5);
        assert!(validate_settlement_observation(&obs).is_ok());
    }

    #[test]
    fn refunded_requires_succeeded_full() {
        let mut obs = base_obs(SettlementStatus::Refunded, SettlementPhase::Refund);
        obs.refund_amount = Some(10);
        obs.refund_outcome = Some(RefundOutcome::SucceededFull);
        assert!(validate_settlement_observation(&obs).is_ok());

        obs.refund_outcome = Some(RefundOutcome::SucceededPartial);
        assert!(validate_settlement_observation(&obs).is_err());
    }

    #[test]
    fn partially_refunded_requires_succeeded_partial() {
        let mut obs = base_obs(
            SettlementStatus::PartiallyRefunded,
            SettlementPhase::Complete,
        );
        obs.refund_amount = Some(10);
        obs.refund_outcome = Some(RefundOutcome::SucceededPartial);
        assert!(validate_settlement_observation(&obs).is_ok());

        obs.refund_outcome = Some(RefundOutcome::SucceededFull);
        assert!(validate_settlement_observation(&obs).is_err());
    }

    #[test]
    fn matrix_a_fields_rejected_for_every_status() {
        // Exhaustive: for each status, populate each A-marked amount/outcome/delay field.
        for status in all_statuses() {
            let valid_phase = all_phases()
                .into_iter()
                .find(|p| is_valid_status_phase(status, *p))
                .expect("each status has ≥1 valid phase");
            let row = presence_matrix(status);

            if matches!(row.settled, Presence::Absent) {
                let mut obs = minimal_valid_for(status, valid_phase);
                obs.settled_amount = Some(1);
                assert!(
                    validate_settlement_observation(&obs).is_err(),
                    "{status:?} settled A"
                );
            }
            if matches!(row.unsettled, Presence::Absent) {
                let mut obs = minimal_valid_for(status, valid_phase);
                obs.unsettled_amount = Some(1);
                assert!(
                    validate_settlement_observation(&obs).is_err(),
                    "{status:?} unsettled A"
                );
            }
            if matches!(row.refund_amount, Presence::Absent) {
                let mut obs = minimal_valid_for(status, valid_phase);
                obs.refund_amount = Some(1);
                assert!(
                    validate_settlement_observation(&obs).is_err(),
                    "{status:?} refund_amount A"
                );
            }
            if matches!(row.refund_outcome, Presence::Absent) {
                let mut obs = minimal_valid_for(status, valid_phase);
                obs.refund_outcome = Some(RefundOutcome::Failed);
                assert!(
                    validate_settlement_observation(&obs).is_err(),
                    "{status:?} refund_outcome A"
                );
            }
            if matches!(row.delay, Presence::Absent) {
                let mut obs = minimal_valid_for(status, valid_phase);
                obs.delay_marker = Some("x".into());
                assert!(
                    validate_settlement_observation(&obs).is_err(),
                    "{status:?} delay A"
                );
            }
        }
    }

    #[test]
    fn matrix_required_fields_absent_rejected() {
        for status in all_statuses() {
            let valid_phase = all_phases()
                .into_iter()
                .find(|p| is_valid_status_phase(status, *p))
                .expect("valid phase");
            let row = presence_matrix(status);
            if matches!(row.requested, Presence::Required) {
                let mut obs = minimal_valid_for(status, valid_phase);
                obs.requested_amount = None;
                if matches!(status, SettlementStatus::Settled) {
                    obs.settled_amount = Some(0);
                }
                if matches!(status, SettlementStatus::PartiallySettled) {
                    obs.settled_amount = Some(1);
                    obs.unsettled_amount = Some(1);
                }
                assert!(
                    validate_settlement_observation(&obs).is_err(),
                    "{status:?} missing requested"
                );
            }
            if matches!(row.refund_amount, Presence::Required) {
                let mut obs = minimal_valid_for(status, valid_phase);
                obs.refund_amount = None;
                assert!(
                    validate_settlement_observation(&obs).is_err(),
                    "{status:?} missing refund_amount"
                );
            }
        }
    }

    fn minimal_valid_for(
        status: SettlementStatus,
        phase: SettlementPhase,
    ) -> SettlementObservation {
        let mut obs = base_obs(status, phase);
        match status {
            SettlementStatus::Settled => {
                obs.requested_amount = Some(100);
                obs.settled_amount = Some(100);
                obs.execution_state = SettlementExecutionState::Executed;
            }
            SettlementStatus::PartiallySettled => {
                obs.requested_amount = Some(100);
                obs.settled_amount = Some(40);
                obs.unsettled_amount = Some(60);
            }
            SettlementStatus::Refunded => {
                obs.refund_amount = Some(10);
                obs.refund_outcome = Some(RefundOutcome::SucceededFull);
            }
            SettlementStatus::PartiallyRefunded => {
                obs.refund_amount = Some(10);
                obs.refund_outcome = Some(RefundOutcome::SucceededPartial);
            }
            _ => {}
        }
        obs
    }

    #[test]
    fn every_status_has_minimal_valid_observation() {
        for status in all_statuses() {
            let phase = all_phases()
                .into_iter()
                .find(|p| is_valid_status_phase(status, *p))
                .expect("phase");
            let obs = minimal_valid_for(status, phase);
            assert!(
                validate_settlement_observation(&obs).is_ok(),
                "{status:?}/{phase:?} minimal valid failed"
            );
        }
    }
}
