//! TASK-42 adversarial regression matrix for Gate-6 M07 core.
//!
//! Exercises `evaluate_payment_settlement` end-to-end against the frozen
//! specification at `b7d1874`. No new semantics — verification only.

use aivoguard::{
    adapt_settlement_observation, evaluate_payment_settlement, is_valid_status_phase,
    validate_settlement_observation, AdapterProvenance, EconomicState, EnginePins as M06EnginePins,
    EscrowStatus, ExecutionStatus, ExternalPaymentSettlementObservation, FeeDeclaration, FeeMode,
    FeeTiming, M06Binding, M07EnginePins, PaymentDeclaration, PaymentScenarioBinding,
    PaymentSettlementBinding, PaymentSettlementCase, PaymentSettlementErrorId,
    PaymentSettlementVerdict, RefundOutcome, RegressionProvenance, RegressionResult,
    RegressionVerdict, ScenarioId, SemanticSettlementAdapter, SettlementDeclaration,
    SettlementExecutionState, SettlementExpectation, SettlementExpectationClass,
    SettlementObservation, SettlementObservationAdapter, SettlementPhase, SettlementStatus,
    SimulationResult, ENGINE_VERSION as M01, M02_ENGINE_VERSION as M02, M03_ENGINE_VERSION as M03,
    M06_ENGINE_VERSION,
};

fn binding() -> PaymentSettlementBinding {
    PaymentSettlementBinding {
        payment_id: "pay-1".into(),
        payment_version: "v1".into(),
        configuration_id: "cfg-1".into(),
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
        case_id: "case-1".into(),
        case_version: "1".into(),
        binding: binding(),
        payment_declaration: PaymentDeclaration {
            payment_id: "pay-1".into(),
            payer: "alice".into(),
            payee: "bob".into(),
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

fn m06_result(verdict: RegressionVerdict, id: &str, version: &str) -> RegressionResult {
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
            observation_engine_pins: M06EnginePins::default(),
            m06_spec_version_label: M06_ENGINE_VERSION.to_owned(),
            evaluation_policy: "ALL_MISMATCHES".into(),
        },
    }
}

fn minimal_sim() -> SimulationResult {
    let state = EconomicState::new();
    SimulationResult {
        execution_status: ExecutionStatus::NormalCompletion,
        scenario_id: ScenarioId::new("sc-1"),
        scenario_version: "1".into(),
        configuration_id: "cfg-1".into(),
        initial_state: state.clone(),
        final_state: state,
        step_records: vec![],
        invariant_evaluation_records: vec![],
        completion_evaluation_records: vec![],
        simulation_errors: vec![],
        executed_action_count: 0,
        m01_engine_version: M01.to_owned(),
        m02_engine_version: M02.to_owned(),
        m03_engine_version: M03.to_owned(),
    }
}

fn assert_error(case: &PaymentSettlementCase, id: PaymentSettlementErrorId) {
    let result = evaluate_payment_settlement(case);
    assert_eq!(result.verdict, PaymentSettlementVerdict::Error);
    assert_eq!(result.error.as_ref().unwrap().error_id, id);
    assert!(result.mismatches.is_empty(), "ERROR must clear mismatches");
}

// A. valid lifecycle observations
#[test]
fn a_valid_settled_match() {
    let case = base_case(vec![exp(
        "s",
        SettlementExpectationClass::SettlementStatusExact {
            expected_status: SettlementStatus::Settled,
        },
        true,
    )]);
    let result = evaluate_payment_settlement(&case);
    assert_eq!(result.verdict, PaymentSettlementVerdict::Match);
    assert!(result.mismatches.is_empty());
    assert!(result.error.is_none());
}

// B. invalid status/phase
#[test]
fn b_invalid_status_phase_pairs() {
    let statuses = [
        SettlementStatus::Pending,
        SettlementStatus::Authorized,
        SettlementStatus::Captured,
        SettlementStatus::Settled,
        SettlementStatus::PartiallySettled,
        SettlementStatus::Failed,
        SettlementStatus::Reversed,
        SettlementStatus::Refunded,
        SettlementStatus::PartiallyRefunded,
    ];
    let phases = [
        SettlementPhase::Declare,
        SettlementPhase::Authorize,
        SettlementPhase::Capture,
        SettlementPhase::Settle,
        SettlementPhase::Refund,
        SettlementPhase::Reverse,
        SettlementPhase::Complete,
    ];
    for status in statuses {
        for phase in phases {
            if is_valid_status_phase(status, phase) {
                continue;
            }
            let mut case = base_case(vec![]);
            case.observation.settlement_status = status;
            case.observation.phase = phase;
            // Make observation structurally closer to status requirements where possible.
            match status {
                SettlementStatus::Pending => {
                    case.observation.requested_amount = None;
                    case.observation.settled_amount = None;
                    case.observation.fee_amount = None;
                    case.observation.net_settlement_amount = None;
                    case.observation.escrow_status = None;
                    case.observation.execution_state = SettlementExecutionState::NotAttempted;
                }
                SettlementStatus::Failed => {
                    case.observation.requested_amount = None;
                    case.observation.settled_amount = None;
                    case.observation.fee_amount = None;
                    case.observation.net_settlement_amount = None;
                    case.observation.escrow_status = None;
                    case.observation.execution_state = SettlementExecutionState::Attempted;
                }
                SettlementStatus::Refunded => {
                    case.observation.refund_outcome = Some(RefundOutcome::SucceededFull);
                    case.observation.refund_amount = Some(1);
                    case.observation.requested_amount = None;
                    case.observation.settled_amount = None;
                    case.observation.fee_amount = None;
                    case.observation.net_settlement_amount = None;
                    case.observation.escrow_status = None;
                }
                _ => {}
            }
            assert_error(
                &case,
                PaymentSettlementErrorId::InvalidSettlementObservation,
            );
        }
    }
}

// C. R/O/A — A fields rejected
#[test]
fn c_pending_rejects_settled_amount_present() {
    let mut case = base_case(vec![]);
    case.observation.settlement_status = SettlementStatus::Pending;
    case.observation.phase = SettlementPhase::Declare;
    case.observation.execution_state = SettlementExecutionState::NotAttempted;
    case.observation.requested_amount = None;
    case.observation.settled_amount = Some(1); // A
    case.observation.fee_amount = None;
    case.observation.net_settlement_amount = None;
    case.observation.escrow_status = None;
    assert_error(
        &case,
        PaymentSettlementErrorId::InvalidSettlementObservation,
    );
}

// D. amount-domain
#[test]
fn d_negative_settled_rejected_negative_net_allowed() {
    let mut case = base_case(vec![]);
    case.observation.settled_amount = Some(-1);
    assert_error(
        &case,
        PaymentSettlementErrorId::InvalidSettlementObservation,
    );

    let mut case = base_case(vec![exp(
        "n",
        SettlementExpectationClass::NetSettlementExact {
            expected_net_settlement_amount: -7,
        },
        true,
    )]);
    case.observation.net_settlement_amount = Some(-7);
    assert_eq!(
        evaluate_payment_settlement(&case).verdict,
        PaymentSettlementVerdict::Match
    );
}

// E. Settled invariant
#[test]
fn e_settled_unequal_and_nonzero_unsettled() {
    let mut case = base_case(vec![]);
    case.observation.settled_amount = Some(90);
    assert_error(
        &case,
        PaymentSettlementErrorId::InvalidSettlementObservation,
    );

    let mut case = base_case(vec![]);
    case.observation.unsettled_amount = Some(1);
    assert_error(
        &case,
        PaymentSettlementErrorId::InvalidSettlementObservation,
    );

    let mut case = base_case(vec![]);
    case.observation.unsettled_amount = Some(0);
    assert_eq!(
        evaluate_payment_settlement(&case).verdict,
        PaymentSettlementVerdict::Match
    );
}

// F. PartiallySettled matrix
#[test]
fn f_partially_settled_adversarial_vectors() {
    let vectors: &[(i128, i128, i128, bool)] = &[
        (100, 0, 100, false),
        (100, 50, 50, true),
        (100, 100, 0, false),
        (100, 150, 0, false),
        (0, 1, 1, false),
        (100, 50, 49, false),
        (100, 50, 51, false),
    ];
    for &(requested, settled, unsettled, ok) in vectors {
        let mut case = base_case(vec![]);
        case.settlement_declaration.requested_amount = requested;
        case.observation.settlement_status = SettlementStatus::PartiallySettled;
        case.observation.phase = SettlementPhase::Settle;
        case.observation.requested_amount = Some(requested);
        case.observation.settled_amount = Some(settled);
        case.observation.unsettled_amount = Some(unsettled);
        case.observation.fee_amount = None;
        case.observation.net_settlement_amount = None;
        case.observation.escrow_status = None;
        let result = evaluate_payment_settlement(&case);
        if ok {
            assert_eq!(
                result.verdict,
                PaymentSettlementVerdict::Match,
                "expected valid {requested}/{settled}/{unsettled}"
            );
        } else {
            assert_eq!(
                result.verdict,
                PaymentSettlementVerdict::Error,
                "expected invalid {requested}/{settled}/{unsettled}"
            );
            assert_eq!(
                result.error.as_ref().unwrap().error_id,
                PaymentSettlementErrorId::InvalidSettlementObservation
            );
        }
    }
}

// G. expectation mismatch
#[test]
fn g_expectation_mismatch() {
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
    assert!(result.error.is_none());
}

// H. missing observation
#[test]
fn h_missing_fee_is_error_not_mismatch() {
    let mut case = base_case(vec![exp(
        "f",
        SettlementExpectationClass::FeeExact {
            expected_fee_amount: 5,
        },
        true,
    )]);
    case.observation.fee_amount = None;
    assert_error(&case, PaymentSettlementErrorId::MissingObservationField);
}

// I. negative expectations
#[test]
fn i_negative_nonnegative_domain_expectations() {
    for class in [
        SettlementExpectationClass::SettledAmountExact {
            expected_settled_amount: -1,
        },
        SettlementExpectationClass::SettledAmountAbsoluteTolerance {
            expected_settled_amount: -1,
            tolerance: 0,
        },
        SettlementExpectationClass::FeeExact {
            expected_fee_amount: -1,
        },
        SettlementExpectationClass::RefundAmountExact {
            expected_refund_amount: -1,
        },
    ] {
        let case = base_case(vec![exp("x", class, true)]);
        assert_error(&case, PaymentSettlementErrorId::InvalidPaymentCase);
    }

    // Signed net remains valid as expectation payload.
    let mut case = base_case(vec![exp(
        "n",
        SettlementExpectationClass::NetSettlementExact {
            expected_net_settlement_amount: -1,
        },
        true,
    )]);
    case.observation.net_settlement_amount = Some(-1);
    assert_eq!(
        evaluate_payment_settlement(&case).verdict,
        PaymentSettlementVerdict::Match
    );
}

// J. binding mismatch (no normalization)
#[test]
fn j_binding_mismatch_no_casefold_or_trim() {
    let mut case = base_case(vec![]);
    case.observation.binding.payment_id = "PAY-1".into();
    assert_error(&case, PaymentSettlementErrorId::ObservationBindingMismatch);

    let mut case = base_case(vec![]);
    case.observation.binding.payment_id = "pay-1 ".into();
    assert_error(&case, PaymentSettlementErrorId::ObservationBindingMismatch);
}

// K. declaration cross-check
#[test]
fn k_declaration_crosscheck_settled_and_partial() {
    let mut case = base_case(vec![]);
    case.observation.requested_amount = Some(99);
    case.observation.settled_amount = Some(99);
    assert_error(
        &case,
        PaymentSettlementErrorId::InvalidSettlementObservation,
    );

    let mut case = base_case(vec![]);
    case.observation.settlement_status = SettlementStatus::Failed;
    case.observation.execution_state = SettlementExecutionState::Attempted;
    case.observation.requested_amount = Some(99);
    case.observation.settled_amount = None;
    case.observation.fee_amount = None;
    case.observation.net_settlement_amount = None;
    case.observation.escrow_status = None;
    assert_eq!(
        evaluate_payment_settlement(&case).verdict,
        PaymentSettlementVerdict::Match
    );
}

// L. engine-pin mismatch
#[test]
fn l_engine_pin_mismatch() {
    let mut case = base_case(vec![]);
    case.observation.simulation_result = Some(minimal_sim());
    case.engine_pins = Some(M07EnginePins {
        m01_engine_version: Some("wrong".into()),
        ..M07EnginePins::default()
    });
    assert_error(&case, PaymentSettlementErrorId::EnginePinMismatch);
}

// M. terminal_partial contradiction
#[test]
fn m_terminal_partial_and_refund_metadata() {
    let mut case = base_case(vec![exp(
        "s",
        SettlementExpectationClass::SettlementStatusExact {
            expected_status: SettlementStatus::Settled,
        },
        true,
    )]);
    case.settlement_declaration.terminal_partial = Some(true);
    assert_error(&case, PaymentSettlementErrorId::InvalidSettlementCase);

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

// N/O/P. M06 composition
#[test]
fn n_o_p_m06_match_mismatch_error() {
    let mut match_case = base_case(vec![]);
    match_case.m06_binding = Some(M06Binding {
        m06_case_id: "m".into(),
        m06_case_version: "1".into(),
        regression_result: m06_result(RegressionVerdict::Match, "m", "1"),
    });
    assert_eq!(
        evaluate_payment_settlement(&match_case).verdict,
        PaymentSettlementVerdict::Match
    );

    let mut mismatch_case = base_case(vec![exp(
        "s",
        SettlementExpectationClass::SettlementStatusExact {
            expected_status: SettlementStatus::Pending,
        },
        true,
    )]);
    mismatch_case.m06_binding = Some(M06Binding {
        m06_case_id: "m".into(),
        m06_case_version: "1".into(),
        regression_result: m06_result(RegressionVerdict::Mismatch, "m", "1"),
    });
    let result = evaluate_payment_settlement(&mismatch_case);
    assert_eq!(result.verdict, PaymentSettlementVerdict::Mismatch);
    assert_eq!(result.mismatches.len(), 2);
    assert_eq!(result.mismatches[0].expectation_id, "s");
    assert_eq!(result.mismatches[1].class, "M06Composition");

    let mut error_case = base_case(vec![exp(
        "s",
        SettlementExpectationClass::SettlementStatusExact {
            expected_status: SettlementStatus::Pending,
        },
        true,
    )]);
    error_case.m06_binding = Some(M06Binding {
        m06_case_id: "m".into(),
        m06_case_version: "1".into(),
        regression_result: m06_result(RegressionVerdict::Error, "m", "1"),
    });
    assert_error(&error_case, PaymentSettlementErrorId::M06CompositionError);
}

// Q. mixed mismatch + ERROR dominance + pipeline earliest stage
#[test]
fn q_error_dominance_and_pipeline_order() {
    // E1 mismatch then missing field ERROR → mismatches cleared.
    let mut case = base_case(vec![
        exp(
            "e1",
            SettlementExpectationClass::SettlementStatusExact {
                expected_status: SettlementStatus::Pending,
            },
            true,
        ),
        exp(
            "e2",
            SettlementExpectationClass::FeeExact {
                expected_fee_amount: 5,
            },
            true,
        ),
        exp(
            "e3",
            SettlementExpectationClass::SettledAmountExact {
                expected_settled_amount: 1,
            },
            true,
        ),
    ]);
    case.observation.fee_amount = None;
    assert_error(&case, PaymentSettlementErrorId::MissingObservationField);

    // Case-stage R-G3 wins before invalid observation phase.
    let mut case = base_case(vec![exp(
        "bad",
        SettlementExpectationClass::SettledAmountExact {
            expected_settled_amount: -1,
        },
        true,
    )]);
    case.observation.phase = SettlementPhase::Authorize; // invalid with Settled
    assert_error(&case, PaymentSettlementErrorId::InvalidPaymentCase);
}

// R. deterministic repeatability
#[test]
fn r_deterministic_repeatability() {
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
    ]);
    let a = evaluate_payment_settlement(&case);
    let b = evaluate_payment_settlement(&case);
    assert_eq!(a, b);
}

#[test]
fn refund_coupling_and_tolerance_boundary() {
    let mut case = base_case(vec![]);
    case.observation.settlement_status = SettlementStatus::Refunded;
    case.observation.phase = SettlementPhase::Refund;
    case.observation.requested_amount = None;
    case.observation.settled_amount = None;
    case.observation.fee_amount = None;
    case.observation.net_settlement_amount = None;
    case.observation.escrow_status = None;
    case.observation.refund_amount = Some(10);
    case.observation.refund_outcome = Some(RefundOutcome::SucceededPartial);
    assert_error(
        &case,
        PaymentSettlementErrorId::InvalidSettlementObservation,
    );

    let case = base_case(vec![exp(
        "t",
        SettlementExpectationClass::SettledAmountAbsoluteTolerance {
            expected_settled_amount: 100,
            tolerance: 5,
        },
        true,
    )]);
    assert_eq!(
        evaluate_payment_settlement(&case).verdict,
        PaymentSettlementVerdict::Match
    );
}

#[test]
fn fee_declaration_and_rf2_case_validation() {
    let mut case = base_case(vec![exp(
        "n",
        SettlementExpectationClass::NetSettlementExact {
            expected_net_settlement_amount: 90,
        },
        true,
    )]);
    case.payment_declaration.fee = Some(FeeDeclaration {
        fee_amount: 10,
        fee_payer: None,
        fee_recipient: None,
        fee_asset: "USD".into(),
        fee_timing: FeeTiming::WithSettlement,
        fee_mode: FeeMode::FeeExclusive,
    });
    case.observation.net_settlement_amount = Some(90);
    assert_eq!(
        evaluate_payment_settlement(&case).verdict,
        PaymentSettlementVerdict::Match
    );

    case.expectations[0].class = SettlementExpectationClass::NetSettlementExact {
        expected_net_settlement_amount: 91,
    };
    assert_error(&case, PaymentSettlementErrorId::InvalidPaymentCase);

    let mut case = base_case(vec![]);
    case.payment_declaration.fee = Some(FeeDeclaration {
        fee_amount: 1,
        fee_payer: None,
        fee_recipient: None,
        fee_asset: "EUR".into(),
        fee_timing: FeeTiming::DeclaredOnly,
        fee_mode: FeeMode::FeeDeclaredOnly,
    });
    assert_error(&case, PaymentSettlementErrorId::InvalidAmount);
}

#[test]
fn declaration_negative_gross_and_scenario_binding() {
    let mut case = base_case(vec![]);
    case.payment_declaration.gross_amount = -1;
    assert_error(&case, PaymentSettlementErrorId::InvalidAmount);

    let mut case = base_case(vec![]);
    case.observation.simulation_result = Some(minimal_sim());
    case.scenario_binding = Some(PaymentScenarioBinding {
        scenario_id: "other".into(),
        scenario_version: "1".into(),
        configuration_id: "cfg-1".into(),
    });
    assert_error(&case, PaymentSettlementErrorId::ObservationBindingMismatch);
}

#[test]
fn observation_has_no_asset_authority_field() {
    // Structural: SettlementObservation has no asset_id; payment_declaration owns asset.
    let obs = settled_obs();
    assert!(obs.adapter_provenance.is_none());
    let _ = AdapterProvenance::default();
    assert!(validate_settlement_observation(&obs).is_ok());
}

#[test]
fn declaration_order_preserved() {
    let case = base_case(vec![
        exp(
            "first",
            SettlementExpectationClass::SettlementStatusExact {
                expected_status: SettlementStatus::Pending,
            },
            true,
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
    assert_eq!(result.mismatches[0].expectation_id, "first");
    assert_eq!(result.mismatches[0].ordinal, 0);
    assert_eq!(result.mismatches[1].expectation_id, "second");
    assert_eq!(result.mismatches[1].ordinal, 1);
}

// ---------------------------------------------------------------------------
// TASK-43 — semantic adapter boundary
// ---------------------------------------------------------------------------

fn payment_declaration() -> PaymentDeclaration {
    PaymentDeclaration {
        payment_id: "pay-1".into(),
        payer: "alice".into(),
        payee: "bob".into(),
        asset_id: "USD".into(),
        gross_amount: 100,
        fee: None,
    }
}

fn external_base() -> ExternalPaymentSettlementObservation {
    ExternalPaymentSettlementObservation {
        binding: binding(),
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
        // Settled marks delay_* as A — keep absent for valid observation path.
        delay_marker: None,
        settlement_delay_steps: None,
        m07_label: Some("lab".into()),
        adapter_provenance: Some(AdapterProvenance {
            adapter_id: Some("semantic".into()),
            adapter_version: Some("1".into()),
            notes: Some("unit".into()),
        }),
        notes: Some("note".into()),
    }
}

/// A — Valid mapping
#[test]
fn adapter_a_valid_mapping() {
    let obs = adapt_settlement_observation(&external_base(), &payment_declaration()).unwrap();
    assert_eq!(obs.settlement_status, SettlementStatus::Settled);
    assert_eq!(obs.phase, SettlementPhase::Settle);
    assert_eq!(obs.execution_state, SettlementExecutionState::Executed);
    assert_eq!(obs.requested_amount, Some(100));
    assert_eq!(obs.settled_amount, Some(100));
    assert_eq!(obs.fee_amount, Some(5));
    assert_eq!(obs.net_settlement_amount, Some(95));
    assert_eq!(obs.escrow_status, Some(EscrowStatus::Locked));
    assert_eq!(obs.escrow_amount, Some(10));
    assert!(obs.delay_marker.is_none());
    assert!(obs.settlement_delay_steps.is_none());
    assert_eq!(
        obs.adapter_provenance
            .as_ref()
            .unwrap()
            .adapter_id
            .as_deref(),
        Some("semantic")
    );
    assert!(validate_settlement_observation(&obs).is_ok());
}

/// B — Asset mismatch
#[test]
fn adapter_b_asset_mismatch() {
    let mut ext = external_base();
    ext.source_asset_id = "EUR".into();
    let err = adapt_settlement_observation(&ext, &payment_declaration()).unwrap_err();
    assert_eq!(err.error_id, PaymentSettlementErrorId::ExternalAdapterError);
}

/// C/D/E — Unknown tokens
#[test]
fn adapter_cde_unknown_tokens() {
    let mut ext = external_base();
    ext.settlement_status = "Processing".into();
    assert_eq!(
        adapt_settlement_observation(&ext, &payment_declaration())
            .unwrap_err()
            .error_id,
        PaymentSettlementErrorId::ExternalAdapterError
    );

    let mut ext = external_base();
    ext.phase = "Finalize".into();
    assert_eq!(
        adapt_settlement_observation(&ext, &payment_declaration())
            .unwrap_err()
            .error_id,
        PaymentSettlementErrorId::ExternalAdapterError
    );

    let mut ext = external_base();
    ext.execution_state = "Succeeded".into();
    assert_eq!(
        adapt_settlement_observation(&ext, &payment_declaration())
            .unwrap_err()
            .error_id,
        PaymentSettlementErrorId::ExternalAdapterError
    );
}

/// F — Missing optionals remain None (no fabricated zeros)
#[test]
fn adapter_f_missing_optionals_remain_none() {
    let mut ext = external_base();
    ext.settlement_status = "Failed".into();
    ext.execution_state = "Attempted".into();
    ext.requested_amount = None;
    ext.settled_amount = None;
    ext.unsettled_amount = None;
    ext.fee_amount = None;
    ext.net_settlement_amount = None;
    ext.refund_amount = None;
    ext.escrow_status = None;
    ext.escrow_amount = None;
    ext.delay_marker = None;
    ext.settlement_delay_steps = None;
    let obs = adapt_settlement_observation(&ext, &payment_declaration()).unwrap();
    assert!(obs.fee_amount.is_none());
    assert!(obs.settled_amount.is_none());
    assert!(obs.delay_marker.is_none());
    assert!(obs.settlement_delay_steps.is_none());
    assert!(obs.escrow_status.is_none());
}

/// G — Negative amounts preserved; validator rejects
#[test]
fn adapter_g_negative_amount_preserved_then_validator() {
    let mut ext = external_base();
    ext.settled_amount = Some(-1);
    let obs = adapt_settlement_observation(&ext, &payment_declaration()).unwrap();
    assert_eq!(obs.settled_amount, Some(-1));
    let err = validate_settlement_observation(&obs).unwrap_err();
    assert_eq!(
        err.error_id,
        PaymentSettlementErrorId::InvalidSettlementObservation
    );
}

/// H — Refund mapping
#[test]
fn adapter_h_refund_mapping() {
    let mut ext = external_base();
    ext.settlement_status = "Refunded".into();
    ext.phase = "Refund".into();
    ext.requested_amount = None;
    ext.settled_amount = None;
    ext.fee_amount = None;
    ext.net_settlement_amount = None;
    ext.escrow_status = None;
    ext.escrow_amount = None;
    ext.delay_marker = None;
    ext.settlement_delay_steps = None;
    ext.refund_amount = Some(10);
    ext.refund_outcome = Some("SucceededFull".into());
    let obs = adapt_settlement_observation(&ext, &payment_declaration()).unwrap();
    assert_eq!(obs.settlement_status, SettlementStatus::Refunded);
    assert_eq!(obs.refund_outcome, Some(RefundOutcome::SucceededFull));
    assert!(validate_settlement_observation(&obs).is_ok());

    ext.settlement_status = "PartiallyRefunded".into();
    ext.refund_outcome = Some("SucceededPartial".into());
    let obs = adapt_settlement_observation(&ext, &payment_declaration()).unwrap();
    assert_eq!(obs.settlement_status, SettlementStatus::PartiallyRefunded);
    assert_eq!(obs.refund_outcome, Some(RefundOutcome::SucceededPartial));
}

/// I/J — Escrow + delay preservation / absence
#[test]
fn adapter_ij_escrow_and_delay() {
    let obs = adapt_settlement_observation(&external_base(), &payment_declaration()).unwrap();
    assert_eq!(obs.escrow_status, Some(EscrowStatus::Locked));
    assert!(obs.delay_marker.is_none());

    // Pending permits delay_* as O — preserve explicit delay without fabricating.
    let mut ext = external_base();
    ext.settlement_status = "Pending".into();
    ext.phase = "Declare".into();
    ext.execution_state = "NotAttempted".into();
    ext.requested_amount = None;
    ext.settled_amount = None;
    ext.fee_amount = None;
    ext.net_settlement_amount = None;
    ext.escrow_status = None;
    ext.escrow_amount = None;
    ext.delay_marker = Some("d1".into());
    ext.settlement_delay_steps = Some(3);
    let obs = adapt_settlement_observation(&ext, &payment_declaration()).unwrap();
    assert_eq!(obs.delay_marker.as_deref(), Some("d1"));
    assert_eq!(obs.settlement_delay_steps, Some(3));
    assert!(obs.escrow_status.is_none());
    assert!(validate_settlement_observation(&obs).is_ok());

    ext.delay_marker = None;
    ext.settlement_delay_steps = None;
    let obs = adapt_settlement_observation(&ext, &payment_declaration()).unwrap();
    assert!(obs.delay_marker.is_none());
    assert!(obs.settlement_delay_steps.is_none());
}

/// K — Provenance survives
#[test]
fn adapter_k_provenance() {
    let obs = adapt_settlement_observation(&external_base(), &payment_declaration()).unwrap();
    let p = obs.adapter_provenance.unwrap();
    assert_eq!(p.adapter_id.as_deref(), Some("semantic"));
    assert_eq!(p.adapter_version.as_deref(), Some("1"));
    assert_eq!(p.notes.as_deref(), Some("unit"));
}

/// L — Deterministic repeatability
#[test]
fn adapter_l_deterministic() {
    let ext = external_base();
    let decl = payment_declaration();
    let a = adapt_settlement_observation(&ext, &decl).unwrap();
    let b = SemanticSettlementAdapter.adapt(&ext, &decl).unwrap();
    assert_eq!(a, b);
}

/// M — No inference / repair
#[test]
fn adapter_m_no_inference() {
    // Status Settled with settled != requested — adapter does not repair.
    let mut ext = external_base();
    ext.settled_amount = Some(40);
    let obs = adapt_settlement_observation(&ext, &payment_declaration()).unwrap();
    assert_eq!(obs.settled_amount, Some(40));
    assert_eq!(obs.requested_amount, Some(100));
    assert!(validate_settlement_observation(&obs).is_err());

    // Fee absent stays absent (not zero).
    ext.fee_amount = None;
    let obs = adapt_settlement_observation(&ext, &payment_declaration()).unwrap();
    assert!(obs.fee_amount.is_none());

    // Phase Authorize with Settled status — adapter maps both; validator rejects pair.
    ext.phase = "Authorize".into();
    ext.settled_amount = Some(100);
    let obs = adapt_settlement_observation(&ext, &payment_declaration()).unwrap();
    assert_eq!(obs.phase, SettlementPhase::Authorize);
    assert_eq!(obs.settlement_status, SettlementStatus::Settled);
    assert!(validate_settlement_observation(&obs).is_err());
}

#[test]
fn adapter_amount_preservation_invariant() {
    let amounts = [0_i128, 1, 100, i128::MAX / 2, -7];
    for amount in amounts {
        let mut ext = external_base();
        ext.settlement_status = "Failed".into();
        ext.execution_state = "Attempted".into();
        ext.requested_amount = None;
        ext.settled_amount = None;
        ext.fee_amount = None;
        ext.net_settlement_amount = Some(amount);
        ext.escrow_status = None;
        ext.escrow_amount = None;
        ext.delay_marker = None;
        ext.settlement_delay_steps = None;
        let obs = adapt_settlement_observation(&ext, &payment_declaration()).unwrap();
        assert_eq!(obs.net_settlement_amount, Some(amount));
    }
}
