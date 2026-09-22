//! M07 semantic adapter boundary (§17 / R-F1).
//!
//! Maps an already-obtained external semantic snapshot into
//! [`SettlementObservation`]. Does **not** contact providers, freeze wire
//! formats (AD-14 OPEN), invent amounts, or evaluate M07 expectations.

use crate::payment_settlement::error::{PaymentSettlementError, PaymentSettlementErrorId};
use crate::payment_settlement::types::{
    AdapterProvenance, EscrowStatus, PaymentDeclaration, PaymentSettlementBinding, RefundOutcome,
    SettlementExecutionState, SettlementObservation, SettlementPhase, SettlementStatus,
};

/// External semantic observation input (provisional; not a wire schema).
///
/// Token fields are closed-mapped by explicit tables. Unknown tokens →
/// [`PaymentSettlementErrorId::ExternalAdapterError`]. This type is **not** a
/// JSON/REST/RPC schema and does **not** close AD-14.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalPaymentSettlementObservation {
    /// Binding identity to carry into the observation.
    pub binding: PaymentSettlementBinding,
    /// Source asset for amounts on this snapshot (exact compare vs declaration).
    pub source_asset_id: String,
    /// External settlement-status token (closed map).
    pub settlement_status: String,
    /// External execution-state token (closed map).
    pub execution_state: String,
    /// External phase token (closed map).
    pub phase: String,
    /// Optional external refund-outcome token.
    pub refund_outcome: Option<String>,
    /// Observed requested amount (exact i128; no fabrication).
    pub requested_amount: Option<i128>,
    /// Observed settled amount.
    pub settled_amount: Option<i128>,
    /// Observed unsettled amount.
    pub unsettled_amount: Option<i128>,
    /// Observed fee amount.
    pub fee_amount: Option<i128>,
    /// Observed net settlement amount (MAY be signed).
    pub net_settlement_amount: Option<i128>,
    /// Observed refund amount.
    pub refund_amount: Option<i128>,
    /// Optional escrow-status token.
    pub escrow_status: Option<String>,
    /// Optional escrow amount.
    pub escrow_amount: Option<i128>,
    /// Optional delay marker.
    pub delay_marker: Option<String>,
    /// Optional settlement delay steps.
    pub settlement_delay_steps: Option<u64>,
    /// Opaque M07 semantic label carrier.
    pub m07_label: Option<String>,
    /// Adapter provenance (non-authoritative).
    pub adapter_provenance: Option<AdapterProvenance>,
    /// Non-authoritative notes.
    pub notes: Option<String>,
}

/// Semantic adapter surface (master §21). Implementations MUST NOT perform
/// provider I/O or wire decoding.
pub trait SettlementObservationAdapter {
    /// Adapt an external semantic snapshot using declaration asset authority.
    ///
    /// # Errors
    ///
    /// Returns [`PaymentSettlementErrorId::ExternalAdapterError`] on asset
    /// mismatch or unmapped external tokens.
    fn adapt(
        &self,
        external: &ExternalPaymentSettlementObservation,
        declaration: &PaymentDeclaration,
    ) -> Result<SettlementObservation, PaymentSettlementError>;
}

/// Default unit adapter — pure deterministic field map.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SemanticSettlementAdapter;

impl SettlementObservationAdapter for SemanticSettlementAdapter {
    fn adapt(
        &self,
        external: &ExternalPaymentSettlementObservation,
        declaration: &PaymentDeclaration,
    ) -> Result<SettlementObservation, PaymentSettlementError> {
        adapt_settlement_observation(external, declaration)
    }
}

/// Adapt external semantic input → [`SettlementObservation`].
///
/// Asset authority check runs **before** observation construction.
///
/// # Errors
///
/// * Source asset ≠ `declaration.asset_id` → `ExternalAdapterError`
/// * Unknown/ambiguous status, phase, execution, refund, or escrow token →
///   `ExternalAdapterError`
pub fn adapt_settlement_observation(
    external: &ExternalPaymentSettlementObservation,
    declaration: &PaymentDeclaration,
) -> Result<SettlementObservation, PaymentSettlementError> {
    // R-F1: reject before constructing SettlementObservation.
    if external.source_asset_id != declaration.asset_id {
        return Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::ExternalAdapterError,
            format!(
                "source asset `{}` ≠ payment_declaration.asset_id `{}`",
                external.source_asset_id, declaration.asset_id
            ),
        ));
    }

    let settlement_status = map_status(&external.settlement_status)?;
    let execution_state = map_execution_state(&external.execution_state)?;
    let phase = map_phase(&external.phase)?;
    let refund_outcome = match &external.refund_outcome {
        None => None,
        Some(token) => Some(map_refund_outcome(token)?),
    };
    let escrow_status = match &external.escrow_status {
        None => None,
        Some(token) => Some(map_escrow_status(token)?),
    };

    Ok(SettlementObservation {
        binding: external.binding.clone(),
        settlement_status,
        execution_state,
        phase,
        refund_outcome,
        requested_amount: external.requested_amount,
        settled_amount: external.settled_amount,
        unsettled_amount: external.unsettled_amount,
        fee_amount: external.fee_amount,
        net_settlement_amount: external.net_settlement_amount,
        refund_amount: external.refund_amount,
        escrow_status,
        escrow_amount: external.escrow_amount,
        delay_marker: external.delay_marker.clone(),
        settlement_delay_steps: external.settlement_delay_steps,
        m07_label: external.m07_label.clone(),
        simulation_result: None,
        adapter_provenance: external.adapter_provenance.clone(),
        notes: external.notes.clone(),
    })
}

fn map_status(token: &str) -> Result<SettlementStatus, PaymentSettlementError> {
    match token {
        "Pending" => Ok(SettlementStatus::Pending),
        "Authorized" => Ok(SettlementStatus::Authorized),
        "Captured" => Ok(SettlementStatus::Captured),
        "Settled" => Ok(SettlementStatus::Settled),
        "PartiallySettled" => Ok(SettlementStatus::PartiallySettled),
        "Failed" => Ok(SettlementStatus::Failed),
        "Reversed" => Ok(SettlementStatus::Reversed),
        "Refunded" => Ok(SettlementStatus::Refunded),
        "PartiallyRefunded" => Ok(SettlementStatus::PartiallyRefunded),
        other => Err(adapter_reject(format!(
            "unknown or unsupported settlement_status token `{other}`"
        ))),
    }
}

fn map_phase(token: &str) -> Result<SettlementPhase, PaymentSettlementError> {
    match token {
        "Declare" => Ok(SettlementPhase::Declare),
        "Authorize" => Ok(SettlementPhase::Authorize),
        "Capture" => Ok(SettlementPhase::Capture),
        "Settle" => Ok(SettlementPhase::Settle),
        "Refund" => Ok(SettlementPhase::Refund),
        "Reverse" => Ok(SettlementPhase::Reverse),
        "Complete" => Ok(SettlementPhase::Complete),
        other => Err(adapter_reject(format!(
            "unknown or unsupported phase token `{other}`"
        ))),
    }
}

fn map_execution_state(token: &str) -> Result<SettlementExecutionState, PaymentSettlementError> {
    match token {
        "NotAttempted" => Ok(SettlementExecutionState::NotAttempted),
        "Attempted" => Ok(SettlementExecutionState::Attempted),
        "Executed" => Ok(SettlementExecutionState::Executed),
        other => Err(adapter_reject(format!(
            "unknown or unsupported execution_state token `{other}`"
        ))),
    }
}

fn map_refund_outcome(token: &str) -> Result<RefundOutcome, PaymentSettlementError> {
    match token {
        "SucceededFull" => Ok(RefundOutcome::SucceededFull),
        "SucceededPartial" => Ok(RefundOutcome::SucceededPartial),
        "Failed" => Ok(RefundOutcome::Failed),
        other => Err(adapter_reject(format!(
            "unknown or unsupported refund_outcome token `{other}`"
        ))),
    }
}

fn map_escrow_status(token: &str) -> Result<EscrowStatus, PaymentSettlementError> {
    match token {
        "NoneDeclared" => Ok(EscrowStatus::NoneDeclared),
        "Locked" => Ok(EscrowStatus::Locked),
        "Released" => Ok(EscrowStatus::Released),
        "ReleaseFailed" => Ok(EscrowStatus::ReleaseFailed),
        "Cancelled" => Ok(EscrowStatus::Cancelled),
        other => Err(adapter_reject(format!(
            "unknown or unsupported escrow_status token `{other}`"
        ))),
    }
}

fn adapter_reject(reason: impl Into<String>) -> PaymentSettlementError {
    PaymentSettlementError::new(PaymentSettlementErrorId::ExternalAdapterError, reason)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn declaration() -> PaymentDeclaration {
        PaymentDeclaration {
            payment_id: "pay".into(),
            payer: "a".into(),
            payee: "b".into(),
            asset_id: "USD".into(),
            gross_amount: 100,
            fee: None,
        }
    }

    fn external_settled() -> ExternalPaymentSettlementObservation {
        ExternalPaymentSettlementObservation {
            binding: PaymentSettlementBinding {
                payment_id: "pay".into(),
                payment_version: "1".into(),
                configuration_id: "cfg".into(),
            },
            source_asset_id: "USD".into(),
            settlement_status: "Settled".into(),
            execution_state: "Executed".into(),
            phase: "Settle".into(),
            refund_outcome: None,
            requested_amount: Some(100),
            settled_amount: Some(100),
            unsettled_amount: None,
            fee_amount: Some(5),
            net_settlement_amount: Some(95),
            refund_amount: None,
            escrow_status: Some("Locked".into()),
            escrow_amount: Some(10),
            delay_marker: None,
            settlement_delay_steps: None,
            m07_label: Some("lab".into()),
            adapter_provenance: Some(AdapterProvenance {
                adapter_id: Some("unit".into()),
                adapter_version: Some("0".into()),
                notes: Some("test".into()),
            }),
            notes: Some("n".into()),
        }
    }

    #[test]
    fn valid_mapping_preserves_amounts_and_provenance() {
        let obs = adapt_settlement_observation(&external_settled(), &declaration()).unwrap();
        assert_eq!(obs.settlement_status, SettlementStatus::Settled);
        assert_eq!(obs.settled_amount, Some(100));
        assert_eq!(obs.fee_amount, Some(5));
        assert_eq!(obs.net_settlement_amount, Some(95));
        assert_eq!(obs.m07_label.as_deref(), Some("lab"));
        assert_eq!(
            obs.adapter_provenance
                .as_ref()
                .unwrap()
                .adapter_id
                .as_deref(),
            Some("unit")
        );
    }

    #[test]
    fn asset_mismatch_rejects_before_construction() {
        let mut ext = external_settled();
        ext.source_asset_id = "EUR".into();
        let err = adapt_settlement_observation(&ext, &declaration()).unwrap_err();
        assert_eq!(err.error_id, PaymentSettlementErrorId::ExternalAdapterError);
    }

    #[test]
    fn unknown_status_rejected() {
        let mut ext = external_settled();
        ext.settlement_status = "Processing".into();
        let err = adapt_settlement_observation(&ext, &declaration()).unwrap_err();
        assert_eq!(err.error_id, PaymentSettlementErrorId::ExternalAdapterError);
    }

    #[test]
    fn missing_optionals_remain_none() {
        let mut ext = external_settled();
        ext.fee_amount = None;
        ext.settled_amount = None;
        ext.requested_amount = None;
        ext.unsettled_amount = None;
        ext.net_settlement_amount = None;
        ext.escrow_status = None;
        ext.escrow_amount = None;
        ext.delay_marker = None;
        ext.settlement_delay_steps = None;
        ext.settlement_status = "Failed".into();
        ext.execution_state = "Attempted".into();
        let obs = adapt_settlement_observation(&ext, &declaration()).unwrap();
        assert!(obs.fee_amount.is_none());
        assert!(obs.settled_amount.is_none());
        assert!(obs.delay_marker.is_none());
        assert!(obs.settlement_delay_steps.is_none());
    }

    #[test]
    fn deterministic_repeat() {
        let ext = external_settled();
        let decl = declaration();
        let a = adapt_settlement_observation(&ext, &decl).unwrap();
        let b = adapt_settlement_observation(&ext, &decl).unwrap();
        assert_eq!(a, b);
    }
}
