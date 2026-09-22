//! M07 expectation evaluation engine (Phase 4–5).
//!
//! Full Gate-6 core pipeline: case/observation validation, binding, pins,
//! expectation evaluation, and M06 composition. Does **not** implement
//! provider adapters or wire format.

use crate::payment_settlement::case::{
    validate_case_scenario_binding, validate_payment_settlement_case,
};
use crate::payment_settlement::compose::{compose_m06, m06_verdict_label, M06ComposeOutcome};
use crate::payment_settlement::error::{PaymentSettlementError, PaymentSettlementErrorId};
use crate::payment_settlement::pins::validate_engine_pins;
use crate::payment_settlement::types::{
    ComparisonOperator, PaymentMismatchEvidence, PaymentSettlementCase,
    PaymentSettlementProvenance, PaymentSettlementResult, PaymentSettlementVerdict,
    SettlementExpectation, SettlementExpectationClass, SettlementObservation, SettlementPhase,
    SettlementStatus,
};
use crate::payment_settlement::validate::{
    validate_case_observation_binding, validate_requested_amount_case_binding,
    validate_settlement_observation,
};
use crate::regression::{compare_absolute_tolerance, ToleranceCompare};

/// Evaluate a payment/settlement case against its single observation.
///
/// Frozen §14 pipeline:
///
/// 1. case structure (declaration amounts, fee, terminal_partial, R-G3, R-F2)
/// 2. observation structural validation (status/phase + matrix)
/// 3. R-A1 requested-amount case binding
/// 4. case ↔ observation binding
/// 5. optional scenario binding vs SimulationResult (§16)
/// 6. optional engine pins
/// 7. expectations in declaration order
/// 8. M06 composition when bound
/// 9. MATCH | MISMATCH | ERROR
#[must_use]
pub fn evaluate_payment_settlement(case: &PaymentSettlementCase) -> PaymentSettlementResult {
    let mut provenance = build_provenance(case);

    if let Err(err) = validate_payment_settlement_case(case) {
        return error_result(provenance, err);
    }

    if let Err(err) = validate_settlement_observation(&case.observation) {
        return error_result(provenance, err);
    }

    if let Err(err) =
        validate_requested_amount_case_binding(&case.observation, &case.settlement_declaration)
    {
        return error_result(provenance, err);
    }

    if let Err(err) = validate_case_observation_binding(&case.binding, &case.observation.binding) {
        return error_result(provenance, err);
    }

    if let Err(err) = validate_case_scenario_binding(case, &case.observation) {
        return error_result(provenance, err);
    }

    if let Some(pins) = &case.engine_pins {
        if let Err(err) = validate_engine_pins(pins, &case.observation) {
            return error_result(provenance, err);
        }
    }

    let mut mismatches = Vec::new();

    for expectation in &case.expectations {
        if !expectation.applicable {
            continue;
        }
        match evaluate_one(expectation, &case.observation, case) {
            EvalOne::Error(err) => {
                return error_result(provenance, err);
            }
            EvalOne::Mismatch(mut evidence) => {
                evidence.ordinal = mismatches.len() as u64;
                mismatches.push(evidence);
            }
            EvalOne::Satisfied => {}
        }
    }

    if let Some(binding) = &case.m06_binding {
        provenance.m06_case_id = Some(binding.m06_case_id.clone());
        provenance.m06_case_version = Some(binding.m06_case_version.clone());
        provenance.m06_verdict =
            Some(m06_verdict_label(binding.regression_result.verdict).to_owned());

        match compose_m06(binding, case) {
            Ok(M06ComposeOutcome::Match) => {}
            Ok(M06ComposeOutcome::Mismatch(evidence)) => {
                let mut evidence = *evidence;
                evidence.ordinal = mismatches.len() as u64;
                mismatches.push(evidence);
            }
            Err(err) => {
                return error_result(provenance, err);
            }
        }
    }

    if mismatches.is_empty() {
        PaymentSettlementResult {
            verdict: PaymentSettlementVerdict::Match,
            mismatches,
            error: None,
            provenance,
        }
    } else {
        PaymentSettlementResult {
            verdict: PaymentSettlementVerdict::Mismatch,
            mismatches,
            error: None,
            provenance,
        }
    }
}

fn error_result(
    provenance: PaymentSettlementProvenance,
    err: PaymentSettlementError,
) -> PaymentSettlementResult {
    PaymentSettlementResult {
        verdict: PaymentSettlementVerdict::Error,
        mismatches: Vec::new(),
        error: Some(err),
        provenance,
    }
}

enum EvalOne {
    Satisfied,
    Mismatch(PaymentMismatchEvidence),
    Error(PaymentSettlementError),
}

#[allow(clippy::too_many_lines)]
fn evaluate_one(
    expectation: &SettlementExpectation,
    obs: &SettlementObservation,
    case: &PaymentSettlementCase,
) -> EvalOne {
    match &expectation.class {
        SettlementExpectationClass::SettlementStatusExact { expected_status } => {
            if obs.settlement_status == *expected_status {
                EvalOne::Satisfied
            } else {
                EvalOne::Mismatch(mismatch(
                    expectation,
                    case,
                    obs,
                    "SettlementStatusExact",
                    "settlement_status",
                    format!("{expected_status:?}"),
                    format!("{:?}", obs.settlement_status),
                    ComparisonOperator::Exact,
                    "StatusInequality",
                ))
            }
        }
        SettlementExpectationClass::SettledAmountExact {
            expected_settled_amount,
        } => match obs.settled_amount {
            None => missing(expectation, "settled_amount"),
            Some(observed) if observed == *expected_settled_amount => EvalOne::Satisfied,
            Some(observed) => EvalOne::Mismatch(mismatch(
                expectation,
                case,
                obs,
                "SettledAmountExact",
                "settled_amount",
                expected_settled_amount.to_string(),
                observed.to_string(),
                ComparisonOperator::Exact,
                "ValueInequality",
            )),
        },
        SettlementExpectationClass::SettledAmountAbsoluteTolerance {
            expected_settled_amount,
            tolerance,
        } => obs.settled_amount.map_or_else(
            || missing(expectation, "settled_amount"),
            |observed| match compare_absolute_tolerance(
                observed,
                *expected_settled_amount,
                *tolerance,
            ) {
                Ok(ToleranceCompare::Satisfied) => EvalOne::Satisfied,
                Ok(ToleranceCompare::Unsatisfied { .. }) => EvalOne::Mismatch(mismatch(
                    expectation,
                    case,
                    obs,
                    "SettledAmountAbsoluteTolerance",
                    "settled_amount",
                    expected_settled_amount.to_string(),
                    observed.to_string(),
                    ComparisonOperator::AbsoluteTolerance,
                    "ValueInequality",
                )),
                Err(reg_err) => EvalOne::Error(PaymentSettlementError::new(
                    PaymentSettlementErrorId::NumericComparisonError,
                    reg_err.reason,
                )),
            },
        ),
        SettlementExpectationClass::FeeExact {
            expected_fee_amount,
        } => match obs.fee_amount {
            None => missing(expectation, "fee_amount"),
            Some(observed) if observed == *expected_fee_amount => EvalOne::Satisfied,
            Some(observed) => EvalOne::Mismatch(mismatch(
                expectation,
                case,
                obs,
                "FeeExact",
                "fee_amount",
                expected_fee_amount.to_string(),
                observed.to_string(),
                ComparisonOperator::Exact,
                "ValueInequality",
            )),
        },
        SettlementExpectationClass::NetSettlementExact {
            expected_net_settlement_amount,
        } => match obs.net_settlement_amount {
            None => missing(expectation, "net_settlement_amount"),
            Some(observed) if observed == *expected_net_settlement_amount => EvalOne::Satisfied,
            Some(observed) => EvalOne::Mismatch(mismatch(
                expectation,
                case,
                obs,
                "NetSettlementExact",
                "net_settlement_amount",
                expected_net_settlement_amount.to_string(),
                observed.to_string(),
                ComparisonOperator::Exact,
                "ValueInequality",
            )),
        },
        SettlementExpectationClass::RefundStatusExact {
            expected_refund_outcome,
        } => match obs.refund_outcome {
            None => missing(expectation, "refund_outcome"),
            Some(observed) if observed == *expected_refund_outcome => EvalOne::Satisfied,
            Some(observed) => EvalOne::Mismatch(mismatch(
                expectation,
                case,
                obs,
                "RefundStatusExact",
                "refund_outcome",
                format!("{expected_refund_outcome:?}"),
                format!("{observed:?}"),
                ComparisonOperator::Exact,
                "StatusInequality",
            )),
        },
        SettlementExpectationClass::RefundAmountExact {
            expected_refund_amount,
        } => match obs.refund_amount {
            None => missing(expectation, "refund_amount"),
            Some(observed) if observed == *expected_refund_amount => EvalOne::Satisfied,
            Some(observed) => EvalOne::Mismatch(mismatch(
                expectation,
                case,
                obs,
                "RefundAmountExact",
                "refund_amount",
                expected_refund_amount.to_string(),
                observed.to_string(),
                ComparisonOperator::Exact,
                "ValueInequality",
            )),
        },
        SettlementExpectationClass::ReversalStatusExact => {
            let ok = obs.phase == SettlementPhase::Reverse
                && obs.settlement_status == SettlementStatus::Reversed;
            if ok {
                EvalOne::Satisfied
            } else {
                EvalOne::Mismatch(mismatch(
                    expectation,
                    case,
                    obs,
                    "ReversalStatusExact",
                    "phase+settlement_status",
                    "Reverse+Reversed".to_owned(),
                    format!("{:?}+{:?}", obs.phase, obs.settlement_status),
                    ComparisonOperator::Exact,
                    "StatusInequality",
                ))
            }
        }
        SettlementExpectationClass::EscrowStatusExact {
            expected_escrow_status,
        } => match obs.escrow_status {
            None => missing(expectation, "escrow_status"),
            Some(observed) if observed == *expected_escrow_status => EvalOne::Satisfied,
            Some(observed) => EvalOne::Mismatch(mismatch(
                expectation,
                case,
                obs,
                "EscrowStatusExact",
                "escrow_status",
                format!("{expected_escrow_status:?}"),
                format!("{observed:?}"),
                ComparisonOperator::Exact,
                "StatusInequality",
            )),
        },
        SettlementExpectationClass::SettlementExecutionExact {
            expected_execution,
            expected_phase,
        } => {
            if obs.execution_state != *expected_execution {
                return EvalOne::Mismatch(mismatch(
                    expectation,
                    case,
                    obs,
                    "SettlementExecutionExact",
                    "execution_state",
                    format!("{expected_execution:?}"),
                    format!("{:?}", obs.execution_state),
                    ComparisonOperator::Exact,
                    "StatusInequality",
                ));
            }
            if let Some(p) = expected_phase {
                if obs.phase != *p {
                    return EvalOne::Mismatch(mismatch(
                        expectation,
                        case,
                        obs,
                        "SettlementExecutionExact",
                        "phase",
                        format!("{p:?}"),
                        format!("{:?}", obs.phase),
                        ComparisonOperator::Exact,
                        "StatusInequality",
                    ));
                }
            }
            EvalOne::Satisfied
        }
    }
}

fn missing(expectation: &SettlementExpectation, field: &str) -> EvalOne {
    EvalOne::Error(PaymentSettlementError::new(
        PaymentSettlementErrorId::MissingObservationField,
        format!(
            "expectation `{}` requires observation field `{field}`",
            expectation.expectation_id
        ),
    ))
}

#[allow(clippy::too_many_arguments)]
fn mismatch(
    expectation: &SettlementExpectation,
    case: &PaymentSettlementCase,
    obs: &SettlementObservation,
    class: &str,
    target: &str,
    expected: String,
    observed: String,
    operator: ComparisonOperator,
    mismatch_class: &str,
) -> PaymentMismatchEvidence {
    PaymentMismatchEvidence {
        expectation_id: expectation.expectation_id.clone(),
        class: class.to_owned(),
        target: target.to_owned(),
        expected,
        observed,
        operator,
        mismatch_class: mismatch_class.to_owned(),
        ordinal: 0,
        settlement_phase: obs.phase,
        payment_id: case.payment_declaration.payment_id.clone(),
        asset_id: case.payment_declaration.asset_id.clone(),
        provenance_ref: None,
    }
}

fn build_provenance(case: &PaymentSettlementCase) -> PaymentSettlementProvenance {
    let obs = &case.observation;
    let (m01, m02, m03) = obs
        .simulation_result
        .as_ref()
        .map_or((None, None, None), |sim| {
            (
                Some(sim.m01_engine_version.clone()),
                Some(sim.m02_engine_version.clone()),
                Some(sim.m03_engine_version.clone()),
            )
        });
    let (adapter_id, adapter_version) = obs.adapter_provenance.as_ref().map_or((None, None), |p| {
        (p.adapter_id.clone(), p.adapter_version.clone())
    });
    let (scenario_id, scenario_version, scenario_configuration_id) = case
        .scenario_binding
        .as_ref()
        .map_or((None, None, None), |s| {
            (
                Some(s.scenario_id.clone()),
                Some(s.scenario_version.clone()),
                Some(s.configuration_id.clone()),
            )
        });

    PaymentSettlementProvenance {
        case_id: case.case_id.clone(),
        case_version: case.case_version.clone(),
        payment_id: case.binding.payment_id.clone(),
        payment_version: case.binding.payment_version.clone(),
        configuration_id: case.binding.configuration_id.clone(),
        scenario_id,
        scenario_version,
        scenario_configuration_id,
        observation_m01_engine_version: m01,
        observation_m02_engine_version: m02,
        observation_m03_engine_version: m03,
        observation_m07_label: obs.m07_label.clone(),
        adapter_id,
        adapter_version,
        m06_case_id: None,
        m06_case_version: None,
        m06_verdict: None,
        m07_semantic_label: obs.m07_label.clone(),
        evaluation_policy: "ALL_MISMATCHES".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::payment_settlement::types::{
        EscrowStatus, PaymentDeclaration, PaymentSettlementBinding, RefundOutcome,
        SettlementDeclaration, SettlementExecutionState, SettlementExpectation,
        SettlementExpectationClass, SettlementObservation, SettlementPhase, SettlementStatus,
    };

    fn binding() -> PaymentSettlementBinding {
        PaymentSettlementBinding {
            payment_id: "pay".into(),
            payment_version: "1".into(),
            configuration_id: "cfg".into(),
        }
    }

    fn settled_obs() -> SettlementObservation {
        SettlementObservation {
            binding: binding(),
            settlement_status: SettlementStatus::Settled,
            execution_state: SettlementExecutionState::Executed,
            phase: SettlementPhase::Settle,
            refund_outcome: None,
            requested_amount: Some(100),
            settled_amount: Some(100),
            unsettled_amount: None,
            fee_amount: Some(5),
            net_settlement_amount: Some(95),
            refund_amount: None,
            escrow_status: Some(EscrowStatus::Locked),
            escrow_amount: None,
            delay_marker: None,
            settlement_delay_steps: None,
            m07_label: None,
            simulation_result: None,
            adapter_provenance: None,
            notes: None,
        }
    }

    fn base_case(expectations: Vec<SettlementExpectation>) -> PaymentSettlementCase {
        PaymentSettlementCase {
            case_id: "c1".into(),
            case_version: "v1".into(),
            binding: binding(),
            payment_declaration: PaymentDeclaration {
                payment_id: "pay".into(),
                payer: "a".into(),
                payee: "b".into(),
                asset_id: "USD".into(),
                gross_amount: 100,
                fee: None,
            },
            settlement_declaration: SettlementDeclaration {
                requested_amount: 100,
                terminal_partial: None,
                terminal_partial_refund: None,
            },
            expectations,
            observation: settled_obs(),
            scenario_binding: None,
            m06_binding: None,
            engine_pins: None,
            notes: None,
        }
    }

    fn exp(id: &str, class: SettlementExpectationClass, applicable: bool) -> SettlementExpectation {
        SettlementExpectation {
            expectation_id: id.into(),
            class,
            applicable,
        }
    }

    #[test]
    fn match_all_applicable() {
        let case = base_case(vec![
            exp(
                "s",
                SettlementExpectationClass::SettlementStatusExact {
                    expected_status: SettlementStatus::Settled,
                },
                true,
            ),
            exp(
                "a",
                SettlementExpectationClass::SettledAmountExact {
                    expected_settled_amount: 100,
                },
                true,
            ),
            exp(
                "f",
                SettlementExpectationClass::FeeExact {
                    expected_fee_amount: 5,
                },
                true,
            ),
            exp(
                "n",
                SettlementExpectationClass::NetSettlementExact {
                    expected_net_settlement_amount: 95,
                },
                true,
            ),
            exp(
                "e",
                SettlementExpectationClass::EscrowStatusExact {
                    expected_escrow_status: EscrowStatus::Locked,
                },
                true,
            ),
            exp(
                "x",
                SettlementExpectationClass::SettlementExecutionExact {
                    expected_execution: SettlementExecutionState::Executed,
                    expected_phase: Some(SettlementPhase::Settle),
                },
                true,
            ),
        ]);
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Match);
        assert!(result.mismatches.is_empty());
        assert!(result.error.is_none());
    }

    #[test]
    fn empty_expectations_match() {
        let result = evaluate_payment_settlement(&base_case(vec![]));
        assert_eq!(result.verdict, PaymentSettlementVerdict::Match);
    }

    #[test]
    fn status_mismatch() {
        let case = base_case(vec![exp(
            "s",
            SettlementExpectationClass::SettlementStatusExact {
                expected_status: SettlementStatus::Pending,
            },
            true,
        )]);
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Mismatch);
        assert_eq!(result.mismatches.len(), 1);
        assert_eq!(result.mismatches[0].ordinal, 0);
    }

    #[test]
    fn missing_settled_amount_is_error() {
        let mut case = base_case(vec![exp(
            "a",
            SettlementExpectationClass::SettledAmountExact {
                expected_settled_amount: 100,
            },
            true,
        )]);
        // Pending observation: settled must be A → observation validation fails first.
        // Use Failed with settled optional None.
        case.observation.settlement_status = SettlementStatus::Failed;
        case.observation.phase = SettlementPhase::Settle;
        case.observation.requested_amount = None;
        case.observation.settled_amount = None;
        case.observation.fee_amount = None;
        case.observation.net_settlement_amount = None;
        case.observation.escrow_status = None;
        case.observation.execution_state = SettlementExecutionState::Attempted;
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Error);
        assert_eq!(
            result.error.as_ref().unwrap().error_id,
            PaymentSettlementErrorId::MissingObservationField
        );
        assert!(result.mismatches.is_empty());
    }

    #[test]
    fn negative_expected_settled_is_invalid_payment_case() {
        let case = base_case(vec![exp(
            "a",
            SettlementExpectationClass::SettledAmountExact {
                expected_settled_amount: -1,
            },
            true,
        )]);
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Error);
        assert_eq!(
            result.error.as_ref().unwrap().error_id,
            PaymentSettlementErrorId::InvalidPaymentCase
        );
    }

    #[test]
    fn negative_expected_net_is_allowed() {
        let mut case = base_case(vec![exp(
            "n",
            SettlementExpectationClass::NetSettlementExact {
                expected_net_settlement_amount: -1,
            },
            true,
        )]);
        case.observation.net_settlement_amount = Some(-1);
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Match);
    }

    #[test]
    fn absolute_tolerance_boundary_and_outside() {
        // Exact match at expected.
        let exact = base_case(vec![exp(
            "t",
            SettlementExpectationClass::SettledAmountAbsoluteTolerance {
                expected_settled_amount: 100,
                tolerance: 5,
            },
            true,
        )]);
        assert_eq!(
            evaluate_payment_settlement(&exact).verdict,
            PaymentSettlementVerdict::Match
        );

        // Inclusive boundary: |105-100| == 5.
        let mut boundary = exact.clone();
        boundary.observation.settled_amount = Some(105);
        boundary.observation.requested_amount = Some(105);
        boundary.settlement_declaration.requested_amount = 105;
        assert_eq!(
            evaluate_payment_settlement(&boundary).verdict,
            PaymentSettlementVerdict::Match
        );

        // Inside: |104-100| < 5.
        let mut inside = exact.clone();
        inside.observation.settled_amount = Some(104);
        inside.observation.requested_amount = Some(104);
        inside.settlement_declaration.requested_amount = 104;
        assert_eq!(
            evaluate_payment_settlement(&inside).verdict,
            PaymentSettlementVerdict::Match
        );

        // Outside: |106-100| > 5.
        let mut outside = exact;
        outside.observation.settled_amount = Some(106);
        outside.observation.requested_amount = Some(106);
        outside.settlement_declaration.requested_amount = 106;
        let result = evaluate_payment_settlement(&outside);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Mismatch);
        assert_eq!(result.mismatches[0].observed, "106");
        assert_eq!(
            result.mismatches[0].operator,
            ComparisonOperator::AbsoluteTolerance
        );
    }

    #[test]
    fn declaration_order_preserved_in_mismatches() {
        let case = base_case(vec![
            exp(
                "first",
                SettlementExpectationClass::SettlementStatusExact {
                    expected_status: SettlementStatus::Pending,
                },
                true,
            ),
            exp(
                "skip",
                SettlementExpectationClass::FeeExact {
                    expected_fee_amount: 999,
                },
                false,
            ),
            exp(
                "second",
                SettlementExpectationClass::SettledAmountExact {
                    expected_settled_amount: 1,
                },
                true,
            ),
        ]);
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Mismatch);
        assert_eq!(result.mismatches.len(), 2);
        assert_eq!(result.mismatches[0].expectation_id, "first");
        assert_eq!(result.mismatches[0].ordinal, 0);
        assert_eq!(result.mismatches[1].expectation_id, "second");
        assert_eq!(result.mismatches[1].ordinal, 1);
    }

    #[test]
    fn error_dominance_clears_mismatches_and_aborts() {
        // A → MISMATCH, B → ERROR (missing fee), C → never evaluated.
        let mut case = base_case(vec![
            exp(
                "a",
                SettlementExpectationClass::SettlementStatusExact {
                    expected_status: SettlementStatus::Pending,
                },
                true,
            ),
            exp(
                "b",
                SettlementExpectationClass::FeeExact {
                    expected_fee_amount: 5,
                },
                true,
            ),
            exp(
                "c",
                SettlementExpectationClass::SettledAmountExact {
                    expected_settled_amount: 1,
                },
                true,
            ),
        ]);
        case.observation.fee_amount = None;
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Error);
        assert!(result.mismatches.is_empty());
        assert_eq!(
            result.error.as_ref().unwrap().error_id,
            PaymentSettlementErrorId::MissingObservationField
        );
    }

    #[test]
    fn duplicate_expectation_id_errors() {
        let case = base_case(vec![
            exp(
                "dup",
                SettlementExpectationClass::SettlementStatusExact {
                    expected_status: SettlementStatus::Settled,
                },
                true,
            ),
            exp(
                "dup",
                SettlementExpectationClass::FeeExact {
                    expected_fee_amount: 5,
                },
                true,
            ),
        ]);
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Error);
        assert_eq!(
            result.error.as_ref().unwrap().error_id,
            PaymentSettlementErrorId::InvalidPaymentCase
        );
    }

    #[test]
    fn reversal_status_exact_match_and_mismatch() {
        let mut case = base_case(vec![exp(
            "r",
            SettlementExpectationClass::ReversalStatusExact,
            true,
        )]);
        case.observation.settlement_status = SettlementStatus::Reversed;
        case.observation.phase = SettlementPhase::Reverse;
        case.observation.requested_amount = None;
        case.observation.settled_amount = None;
        case.observation.fee_amount = None;
        case.observation.net_settlement_amount = None;
        case.observation.escrow_status = None;
        case.observation.execution_state = SettlementExecutionState::Executed;
        assert_eq!(
            evaluate_payment_settlement(&case).verdict,
            PaymentSettlementVerdict::Match
        );

        // Settled observation is valid but fails ReversalStatusExact → MISMATCH.
        let case = base_case(vec![exp(
            "r",
            SettlementExpectationClass::ReversalStatusExact,
            true,
        )]);
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Mismatch);
        assert_eq!(result.mismatches[0].class, "ReversalStatusExact");
    }

    #[test]
    fn refund_status_and_amount_match_and_mismatch() {
        let mut case = base_case(vec![
            exp(
                "rs",
                SettlementExpectationClass::RefundStatusExact {
                    expected_refund_outcome: RefundOutcome::SucceededFull,
                },
                true,
            ),
            exp(
                "ra",
                SettlementExpectationClass::RefundAmountExact {
                    expected_refund_amount: 10,
                },
                true,
            ),
        ]);
        case.observation.settlement_status = SettlementStatus::Refunded;
        case.observation.phase = SettlementPhase::Refund;
        case.observation.requested_amount = None;
        case.observation.settled_amount = None;
        case.observation.fee_amount = None;
        case.observation.net_settlement_amount = None;
        case.observation.escrow_status = None;
        case.observation.refund_amount = Some(10);
        case.observation.refund_outcome = Some(RefundOutcome::SucceededFull);
        assert_eq!(
            evaluate_payment_settlement(&case).verdict,
            PaymentSettlementVerdict::Match
        );

        case.observation.refund_amount = Some(11);
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Mismatch);
        assert_eq!(result.mismatches[0].expectation_id, "ra");
    }

    #[test]
    fn missing_optional_observation_fields_are_errors() {
        // Failed allows optional amounts absent while remaining structurally valid.
        let mut case = base_case(vec![exp(
            "f",
            SettlementExpectationClass::FeeExact {
                expected_fee_amount: 1,
            },
            true,
        )]);
        case.observation.settlement_status = SettlementStatus::Failed;
        case.observation.phase = SettlementPhase::Settle;
        case.observation.requested_amount = None;
        case.observation.settled_amount = None;
        case.observation.fee_amount = None;
        case.observation.net_settlement_amount = None;
        case.observation.escrow_status = None;
        case.observation.refund_amount = None;
        case.observation.refund_outcome = None;
        case.observation.execution_state = SettlementExecutionState::Attempted;

        assert_missing(&case, PaymentSettlementErrorId::MissingObservationField);

        case.expectations = vec![exp(
            "n",
            SettlementExpectationClass::NetSettlementExact {
                expected_net_settlement_amount: 0,
            },
            true,
        )];
        assert_missing(&case, PaymentSettlementErrorId::MissingObservationField);

        case.expectations = vec![exp(
            "rs",
            SettlementExpectationClass::RefundStatusExact {
                expected_refund_outcome: RefundOutcome::Failed,
            },
            true,
        )];
        assert_missing(&case, PaymentSettlementErrorId::MissingObservationField);

        case.expectations = vec![exp(
            "ra",
            SettlementExpectationClass::RefundAmountExact {
                expected_refund_amount: 1,
            },
            true,
        )];
        assert_missing(&case, PaymentSettlementErrorId::MissingObservationField);

        case.expectations = vec![exp(
            "e",
            SettlementExpectationClass::EscrowStatusExact {
                expected_escrow_status: EscrowStatus::Locked,
            },
            true,
        )];
        assert_missing(&case, PaymentSettlementErrorId::MissingObservationField);

        case.expectations = vec![exp(
            "a",
            SettlementExpectationClass::SettledAmountExact {
                expected_settled_amount: 1,
            },
            true,
        )];
        assert_missing(&case, PaymentSettlementErrorId::MissingObservationField);

        case.expectations = vec![exp(
            "t",
            SettlementExpectationClass::SettledAmountAbsoluteTolerance {
                expected_settled_amount: 1,
                tolerance: 0,
            },
            true,
        )];
        assert_missing(&case, PaymentSettlementErrorId::MissingObservationField);
    }

    fn assert_missing(case: &PaymentSettlementCase, id: PaymentSettlementErrorId) {
        let result = evaluate_payment_settlement(case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Error);
        assert_eq!(result.error.as_ref().unwrap().error_id, id);
        assert!(result.mismatches.is_empty());
    }

    #[test]
    fn fee_net_escrow_execution_mismatches() {
        let case = base_case(vec![
            exp(
                "f",
                SettlementExpectationClass::FeeExact {
                    expected_fee_amount: 9,
                },
                true,
            ),
            exp(
                "n",
                SettlementExpectationClass::NetSettlementExact {
                    expected_net_settlement_amount: 0,
                },
                true,
            ),
            exp(
                "e",
                SettlementExpectationClass::EscrowStatusExact {
                    expected_escrow_status: EscrowStatus::Released,
                },
                true,
            ),
            exp(
                "x",
                SettlementExpectationClass::SettlementExecutionExact {
                    expected_execution: SettlementExecutionState::Attempted,
                    expected_phase: None,
                },
                true,
            ),
        ]);
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Mismatch);
        assert_eq!(result.mismatches.len(), 4);
        assert_eq!(result.mismatches[0].expectation_id, "f");
        assert_eq!(result.mismatches[1].expectation_id, "n");
        assert_eq!(result.mismatches[2].expectation_id, "e");
        assert_eq!(result.mismatches[3].expectation_id, "x");
    }

    #[test]
    fn negative_fee_and_refund_expected_rejected() {
        for class in [
            SettlementExpectationClass::FeeExact {
                expected_fee_amount: -1,
            },
            SettlementExpectationClass::RefundAmountExact {
                expected_refund_amount: -1,
            },
            SettlementExpectationClass::SettledAmountAbsoluteTolerance {
                expected_settled_amount: -1,
                tolerance: 0,
            },
        ] {
            let case = base_case(vec![exp("x", class, true)]);
            let result = evaluate_payment_settlement(&case);
            assert_eq!(result.verdict, PaymentSettlementVerdict::Error);
            assert_eq!(
                result.error.as_ref().unwrap().error_id,
                PaymentSettlementErrorId::InvalidPaymentCase
            );
        }
    }

    #[test]
    fn non_applicable_skipped() {
        let case = base_case(vec![exp(
            "skip",
            SettlementExpectationClass::SettlementStatusExact {
                expected_status: SettlementStatus::Pending,
            },
            false,
        )]);
        assert_eq!(
            evaluate_payment_settlement(&case).verdict,
            PaymentSettlementVerdict::Match
        );
    }

    // --- Phase 5: binding / pins / M06 / terminal_partial ---

    fn m06_result(
        verdict: crate::regression::RegressionVerdict,
        id: &str,
        version: &str,
    ) -> crate::regression::RegressionResult {
        use crate::regression::{EnginePins, RegressionProvenance, M06_ENGINE_VERSION};
        crate::regression::RegressionResult {
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

    #[test]
    fn binding_match_succeeds() {
        let case = base_case(vec![]);
        assert_eq!(
            evaluate_payment_settlement(&case).verdict,
            PaymentSettlementVerdict::Match
        );
    }

    #[test]
    fn binding_mismatch_no_normalization() {
        let mut case = base_case(vec![]);
        // Differ by trailing space — must NOT trim/fold.
        case.observation.binding.payment_id = "pay ".into();
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Error);
        assert_eq!(
            result.error.as_ref().unwrap().error_id,
            PaymentSettlementErrorId::ObservationBindingMismatch
        );
        assert!(result.mismatches.is_empty());
    }

    #[test]
    fn binding_empty_vs_present_is_mismatch() {
        // Binding is structurally required; empty identity ≠ declared identity.
        let mut case = base_case(vec![]);
        case.observation.binding.payment_id.clear();
        let result = evaluate_payment_settlement(&case);
        assert_eq!(
            result.error.as_ref().unwrap().error_id,
            PaymentSettlementErrorId::ObservationBindingMismatch
        );
    }

    #[test]
    fn settled_requested_amount_case_binding() {
        let mut case = base_case(vec![]);
        assert_eq!(
            evaluate_payment_settlement(&case).verdict,
            PaymentSettlementVerdict::Match
        );

        case.observation.requested_amount = Some(99);
        case.observation.settled_amount = Some(99);
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Error);
        assert_eq!(
            result.error.as_ref().unwrap().error_id,
            PaymentSettlementErrorId::InvalidSettlementObservation
        );
    }

    #[test]
    fn partially_settled_requested_amount_case_binding() {
        let mut case = base_case(vec![]);
        case.observation.settlement_status = SettlementStatus::PartiallySettled;
        case.observation.settled_amount = Some(40);
        case.observation.unsettled_amount = Some(60);
        case.observation.requested_amount = Some(100);
        assert_eq!(
            evaluate_payment_settlement(&case).verdict,
            PaymentSettlementVerdict::Match
        );

        case.observation.requested_amount = Some(99);
        case.observation.settled_amount = Some(40);
        case.observation.unsettled_amount = Some(59);
        let result = evaluate_payment_settlement(&case);
        assert_eq!(
            result.error.as_ref().unwrap().error_id,
            PaymentSettlementErrorId::InvalidSettlementObservation
        );
    }

    #[test]
    fn requested_binding_not_applied_to_failed() {
        let mut case = base_case(vec![]);
        case.observation.settlement_status = SettlementStatus::Failed;
        case.observation.phase = SettlementPhase::Settle;
        case.observation.execution_state = SettlementExecutionState::Attempted;
        case.observation.requested_amount = Some(99); // diverges from declaration 100
        case.observation.settled_amount = None;
        case.observation.fee_amount = None;
        case.observation.net_settlement_amount = None;
        case.observation.escrow_status = None;
        assert_eq!(
            evaluate_payment_settlement(&case).verdict,
            PaymentSettlementVerdict::Match
        );
    }

    #[test]
    fn engine_pins_match_and_mismatch() {
        use crate::kernel::EconomicState;
        use crate::payment_settlement::types::M07EnginePins;
        use crate::simulator::{ExecutionStatus, ScenarioId, SimulationResult, M03_ENGINE_VERSION};
        use crate::{ENGINE_VERSION as M01, M02_ENGINE_VERSION as M02};

        let state = EconomicState::new();
        let mut case = base_case(vec![]);
        case.observation.simulation_result = Some(SimulationResult {
            execution_status: ExecutionStatus::NormalCompletion,
            scenario_id: ScenarioId::new("sc"),
            scenario_version: "1".into(),
            configuration_id: "cfg".into(),
            initial_state: state.clone(),
            final_state: state,
            step_records: vec![],
            invariant_evaluation_records: vec![],
            completion_evaluation_records: vec![],
            simulation_errors: vec![],
            executed_action_count: 0,
            m01_engine_version: M01.to_owned(),
            m02_engine_version: M02.to_owned(),
            m03_engine_version: M03_ENGINE_VERSION.to_owned(),
        });
        case.observation.m07_label = Some("lab".into());
        case.engine_pins = Some(M07EnginePins {
            m01_engine_version: Some(M01.to_owned()),
            m02_engine_version: None,
            m03_engine_version: None,
            m07_label: Some("lab".into()),
        });
        assert_eq!(
            evaluate_payment_settlement(&case).verdict,
            PaymentSettlementVerdict::Match
        );

        case.engine_pins.as_mut().unwrap().m07_label = Some("other".into());
        let result = evaluate_payment_settlement(&case);
        assert_eq!(
            result.error.as_ref().unwrap().error_id,
            PaymentSettlementErrorId::EnginePinMismatch
        );

        // Absent engine_pins → no invented requirement.
        case.engine_pins = None;
        assert_eq!(
            evaluate_payment_settlement(&case).verdict,
            PaymentSettlementVerdict::Match
        );
    }

    #[test]
    fn m06_match_does_not_force_m07_match_alone() {
        use crate::payment_settlement::types::M06Binding;
        use crate::regression::RegressionVerdict;

        let mut case = base_case(vec![exp(
            "s",
            SettlementExpectationClass::SettlementStatusExact {
                expected_status: SettlementStatus::Pending,
            },
            true,
        )]);
        case.m06_binding = Some(M06Binding {
            m06_case_id: "m".into(),
            m06_case_version: "1".into(),
            regression_result: m06_result(RegressionVerdict::Match, "m", "1"),
        });
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Mismatch);
        assert_eq!(result.mismatches.len(), 1);
        assert_eq!(result.provenance.m06_verdict.as_deref(), Some("Match"));
    }

    #[test]
    fn m06_mismatch_appends_one_composition_after_expectations() {
        use crate::payment_settlement::types::M06Binding;
        use crate::regression::RegressionVerdict;

        let mut case = base_case(vec![exp(
            "s",
            SettlementExpectationClass::SettlementStatusExact {
                expected_status: SettlementStatus::Pending,
            },
            true,
        )]);
        case.m06_binding = Some(M06Binding {
            m06_case_id: "m".into(),
            m06_case_version: "1".into(),
            regression_result: m06_result(RegressionVerdict::Mismatch, "m", "1"),
        });
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Mismatch);
        assert_eq!(result.mismatches.len(), 2);
        assert_eq!(result.mismatches[0].expectation_id, "s");
        assert_eq!(result.mismatches[0].ordinal, 0);
        assert_eq!(result.mismatches[1].class, "M06Composition");
        assert_eq!(result.mismatches[1].ordinal, 1);
        assert!(result.error.is_none());
    }

    #[test]
    fn m06_error_clears_prior_mismatches() {
        use crate::payment_settlement::types::M06Binding;
        use crate::regression::RegressionVerdict;

        let mut case = base_case(vec![exp(
            "s",
            SettlementExpectationClass::SettlementStatusExact {
                expected_status: SettlementStatus::Pending,
            },
            true,
        )]);
        case.m06_binding = Some(M06Binding {
            m06_case_id: "m".into(),
            m06_case_version: "1".into(),
            regression_result: m06_result(RegressionVerdict::Error, "m", "1"),
        });
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Error);
        assert!(result.mismatches.is_empty());
        assert_eq!(
            result.error.as_ref().unwrap().error_id,
            PaymentSettlementErrorId::M06CompositionError
        );
        assert_eq!(result.provenance.m06_verdict.as_deref(), Some("Error"));
    }

    #[test]
    fn m06_absent_is_not_invented_error() {
        let case = base_case(vec![]);
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Match);
        assert!(result.provenance.m06_case_id.is_none());
    }

    #[test]
    fn m06_match_with_matching_expectations_is_match() {
        use crate::payment_settlement::types::M06Binding;
        use crate::regression::RegressionVerdict;

        let mut case = base_case(vec![exp(
            "s",
            SettlementExpectationClass::SettlementStatusExact {
                expected_status: SettlementStatus::Settled,
            },
            true,
        )]);
        case.m06_binding = Some(M06Binding {
            m06_case_id: "m".into(),
            m06_case_version: "1".into(),
            regression_result: m06_result(RegressionVerdict::Match, "m", "1"),
        });
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Match);
        assert!(result.mismatches.is_empty());
        assert!(result.error.is_none());
        assert_eq!(result.provenance.m06_verdict.as_deref(), Some("Match"));
    }

    #[test]
    fn terminal_partial_contradicts_settled_expectation() {
        let mut case = base_case(vec![exp(
            "s",
            SettlementExpectationClass::SettlementStatusExact {
                expected_status: SettlementStatus::Settled,
            },
            true,
        )]);
        case.settlement_declaration.terminal_partial = Some(true);
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Error);
        assert_eq!(
            result.error.as_ref().unwrap().error_id,
            PaymentSettlementErrorId::InvalidSettlementCase
        );
    }

    #[test]
    fn terminal_partial_does_not_contradict_partially_settled_expectation() {
        let mut case = base_case(vec![exp(
            "s",
            SettlementExpectationClass::SettlementStatusExact {
                expected_status: SettlementStatus::PartiallySettled,
            },
            true,
        )]);
        case.settlement_declaration.terminal_partial = Some(true);
        case.observation.settlement_status = SettlementStatus::PartiallySettled;
        case.observation.settled_amount = Some(40);
        case.observation.unsettled_amount = Some(60);
        let result = evaluate_payment_settlement(&case);
        assert_eq!(result.verdict, PaymentSettlementVerdict::Match);
    }

    #[test]
    fn terminal_partial_refund_has_no_contradiction_authority() {
        let mut case = base_case(vec![exp(
            "s",
            SettlementExpectationClass::SettlementStatusExact {
                expected_status: SettlementStatus::Settled,
            },
            true,
        )]);
        case.settlement_declaration.terminal_partial_refund = Some(true);
        assert_eq!(
            evaluate_payment_settlement(&case).verdict,
            PaymentSettlementVerdict::Match
        );
    }
}
