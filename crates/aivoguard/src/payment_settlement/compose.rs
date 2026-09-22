//! M06 composition consumption (§15).
//!
//! Consumes an already-evaluated [`RegressionResult`]. Does **not** call
//! `evaluate_regression` or reimplement M06.

use crate::payment_settlement::error::{PaymentSettlementError, PaymentSettlementErrorId};
use crate::payment_settlement::types::{
    ComparisonOperator, M06Binding, PaymentMismatchEvidence, PaymentSettlementCase,
};
use crate::regression::RegressionVerdict;

/// Outcome of Gate-6 `ComposeRequireM06Match` when binding identity is valid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M06ComposeOutcome {
    /// Bound M06 returned MATCH — no M07 mismatch evidence.
    Match,
    /// Bound M06 returned MISMATCH — exactly one `M06Composition` evidence item.
    Mismatch(Box<PaymentMismatchEvidence>),
}

/// Consume bound M06 result under ComposeRequireM06Match.
///
/// # Errors
///
/// * Identity mismatch between `M06Binding` and `RegressionResult` →
///   [`PaymentSettlementErrorId::M06CompositionError`]
/// * Bound M06 `ERROR` → [`PaymentSettlementErrorId::M06CompositionError`]
pub fn compose_m06(
    binding: &M06Binding,
    case: &PaymentSettlementCase,
) -> Result<M06ComposeOutcome, PaymentSettlementError> {
    let result = &binding.regression_result;
    if result.case_id != binding.m06_case_id || result.case_version != binding.m06_case_version {
        return Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::M06CompositionError,
            format!(
                "M06 binding identity mismatch: binding=`{}`/`{}` result=`{}`/`{}`",
                binding.m06_case_id, binding.m06_case_version, result.case_id, result.case_version
            ),
        ));
    }

    match result.verdict {
        RegressionVerdict::Match => Ok(M06ComposeOutcome::Match),
        RegressionVerdict::Mismatch => Ok(M06ComposeOutcome::Mismatch(Box::new(
            m06_mismatch_evidence(binding, case),
        ))),
        RegressionVerdict::Error => Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::M06CompositionError,
            format!(
                "bound M06 evaluation returned ERROR for `{}`/`{}`",
                binding.m06_case_id, binding.m06_case_version
            ),
        )),
    }
}

/// Label for provenance `m06_verdict` (§15.3).
#[must_use]
pub fn m06_verdict_label(verdict: RegressionVerdict) -> &'static str {
    match verdict {
        RegressionVerdict::Match => "Match",
        RegressionVerdict::Mismatch => "Mismatch",
        RegressionVerdict::Error => "Error",
    }
}

fn m06_mismatch_evidence(
    binding: &M06Binding,
    case: &PaymentSettlementCase,
) -> PaymentMismatchEvidence {
    PaymentMismatchEvidence {
        expectation_id: format!(
            "M06Composition:{}:{}",
            binding.m06_case_id, binding.m06_case_version
        ),
        class: "M06Composition".to_owned(),
        target: "m06_binding.regression_result.verdict".to_owned(),
        expected: "Match".to_owned(),
        observed: format!(
            "Mismatch;{};{}",
            binding.m06_case_id, binding.m06_case_version
        ),
        operator: ComparisonOperator::Exact,
        mismatch_class: "M06Composition".to_owned(),
        ordinal: 0,
        settlement_phase: case.observation.phase,
        payment_id: case.payment_declaration.payment_id.clone(),
        asset_id: case.payment_declaration.asset_id.clone(),
        provenance_ref: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::payment_settlement::types::{
        PaymentDeclaration, PaymentSettlementBinding, SettlementDeclaration,
        SettlementExecutionState, SettlementObservation, SettlementPhase, SettlementStatus,
    };
    use crate::regression::{
        EnginePins, RegressionProvenance, RegressionResult, RegressionVerdict, M06_ENGINE_VERSION,
    };

    fn empty_m06_result(verdict: RegressionVerdict, id: &str, version: &str) -> RegressionResult {
        RegressionResult {
            verdict,
            case_id: id.into(),
            case_version: version.into(),
            mismatches: Vec::new(),
            error: None,
            expectations_evaluated: 0,
            expectations_skipped: 0,
            provenance: RegressionProvenance {
                scenario_id: "sc".into(),
                scenario_version: "1".into(),
                configuration_id: "cfg".into(),
                observation_engine_pins: EnginePins::default(),
                m06_spec_version_label: M06_ENGINE_VERSION.to_owned(),
                evaluation_policy: "ALL_MISMATCHES".into(),
            },
        }
    }

    fn minimal_case() -> PaymentSettlementCase {
        PaymentSettlementCase {
            case_id: "c".into(),
            case_version: "v".into(),
            binding: PaymentSettlementBinding {
                payment_id: "pay".into(),
                payment_version: "1".into(),
                configuration_id: "cfg".into(),
            },
            payment_declaration: PaymentDeclaration {
                payment_id: "pay".into(),
                payer: "a".into(),
                payee: "b".into(),
                asset_id: "USD".into(),
                gross_amount: 1,
                fee: None,
            },
            settlement_declaration: SettlementDeclaration {
                requested_amount: 1,
                terminal_partial: None,
                terminal_partial_refund: None,
            },
            expectations: vec![],
            observation: SettlementObservation {
                binding: PaymentSettlementBinding {
                    payment_id: "pay".into(),
                    payment_version: "1".into(),
                    configuration_id: "cfg".into(),
                },
                settlement_status: SettlementStatus::Failed,
                execution_state: SettlementExecutionState::Attempted,
                phase: SettlementPhase::Settle,
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
            },
            scenario_binding: None,
            m06_binding: None,
            engine_pins: None,
            notes: None,
        }
    }

    #[test]
    fn m06_match_yields_no_evidence() {
        let binding = M06Binding {
            m06_case_id: "m".into(),
            m06_case_version: "1".into(),
            regression_result: empty_m06_result(RegressionVerdict::Match, "m", "1"),
        };
        assert_eq!(
            compose_m06(&binding, &minimal_case()).unwrap(),
            M06ComposeOutcome::Match
        );
    }

    #[test]
    fn m06_mismatch_yields_one_composition_evidence() {
        let binding = M06Binding {
            m06_case_id: "m".into(),
            m06_case_version: "1".into(),
            regression_result: empty_m06_result(RegressionVerdict::Mismatch, "m", "1"),
        };
        match compose_m06(&binding, &minimal_case()).unwrap() {
            M06ComposeOutcome::Mismatch(ev) => {
                assert_eq!(ev.class, "M06Composition");
                assert_eq!(ev.expected, "Match");
            }
            M06ComposeOutcome::Match => panic!("expected mismatch evidence"),
        }
    }

    #[test]
    fn m06_error_is_composition_error() {
        let binding = M06Binding {
            m06_case_id: "m".into(),
            m06_case_version: "1".into(),
            regression_result: empty_m06_result(RegressionVerdict::Error, "m", "1"),
        };
        let err = compose_m06(&binding, &minimal_case()).unwrap_err();
        assert_eq!(err.error_id, PaymentSettlementErrorId::M06CompositionError);
    }

    #[test]
    fn identity_mismatch_is_composition_error() {
        let binding = M06Binding {
            m06_case_id: "declared".into(),
            m06_case_version: "1".into(),
            regression_result: empty_m06_result(RegressionVerdict::Match, "other", "1"),
        };
        let err = compose_m06(&binding, &minimal_case()).unwrap_err();
        assert_eq!(err.error_id, PaymentSettlementErrorId::M06CompositionError);
    }
}
