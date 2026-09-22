//! Integration tests for Gate-5 M06 Economic Regression Engine.
//!
//! Covers frozen acceptance criteria (§22) against `ca4c88a`.

use aivoguard::{
    compare_absolute_tolerance, evaluate_regression, AccountId, Action, ActorId,
    ApplicabilityStatus, AssetId, AttemptClassification, ComparisonOperator, EconomicDisposition,
    EconomicState, EnginePins, EvaluationPoint, Evidence, ExecutionContext, ExecutionStatus,
    Expectation, ExpectationClass, ExpectationPhase, ExpectedDisposition, ExpectedExecutionStatus,
    ExpectedFatalCauseClass, FacetId, InvariantEvaluationRecord, InvariantEvidence, InvariantId,
    InvariantOutcome, InvariantResultKind, KernelOutcome, MismatchClass, RegressionCase,
    RegressionErrorId, RegressionObservation, RegressionVerdict, ScenarioBinding, ScenarioId,
    SimulationResult, StepResult, StructuredValue, ToleranceCompare, Transaction,
    ENGINE_VERSION as M01_ENGINE_VERSION, M02_ENGINE_VERSION, M03_ENGINE_VERSION,
    M06_ENGINE_VERSION,
};

fn facet() -> FacetId {
    FacetId::new("available")
}

fn state_with(alice: i128, bob: Option<i128>) -> EconomicState {
    let mut s = EconomicState::new();
    s.set_balance(AccountId::new("alice"), AssetId::new("USD"), facet(), alice);
    if let Some(b) = bob {
        s.set_balance(AccountId::new("bob"), AssetId::new("USD"), facet(), b);
    }
    s
}

fn noop_action() -> Action {
    Action::NoOp {
        actor: ActorId::new("alice-actor"),
        action_id: None,
    }
}

fn empty_evidence() -> Evidence {
    Evidence {
        action_summary: "noop".into(),
        world_summary: "test".into(),
        execution_context: ExecutionContext::new(0, "cfg"),
        disposition: Some(EconomicDisposition::AcceptedEffective),
        effects: vec![],
        reason: "ok".into(),
    }
}

fn economic_outcome(disposition: EconomicDisposition, state: &EconomicState) -> KernelOutcome {
    KernelOutcome::Economic {
        disposition,
        state_before: state.clone(),
        state_after: state.clone(),
        transaction: Transaction {
            actor: ActorId::new("alice-actor"),
            disposition,
            effects: vec![],
        },
        events: vec![],
        evidence: empty_evidence(),
    }
}

fn pass_outcome(id: &str) -> InvariantOutcome {
    InvariantOutcome {
        kind: InvariantResultKind::Pass,
        applicability: Some(ApplicabilityStatus::Applicable),
        violations: vec![],
        error: None,
        evidence: InvariantEvidence {
            invariant_id: InvariantId::new(id),
            definition_version: "1".into(),
            target_kind: "state".into(),
            engine_version: M02_ENGINE_VERSION.to_owned(),
            result_kind: InvariantResultKind::Pass,
            applicability: Some(ApplicabilityStatus::Applicable),
            error_class: None,
            error_reason: None,
            notes: vec![],
        },
    }
}

fn fail_outcome(id: &str) -> InvariantOutcome {
    let mut o = pass_outcome(id);
    o.kind = InvariantResultKind::Fail;
    o.evidence.result_kind = InvariantResultKind::Fail;
    o
}

fn base_sim(final_state: EconomicState) -> SimulationResult {
    SimulationResult {
        execution_status: ExecutionStatus::NormalCompletion,
        scenario_id: ScenarioId::new("sc-1"),
        scenario_version: "1".into(),
        configuration_id: "cfg".into(),
        initial_state: final_state.clone(),
        final_state,
        step_records: vec![],
        invariant_evaluation_records: vec![],
        completion_evaluation_records: vec![],
        simulation_errors: vec![],
        executed_action_count: 0,
        m01_engine_version: M01_ENGINE_VERSION.to_owned(),
        m02_engine_version: M02_ENGINE_VERSION.to_owned(),
        m03_engine_version: M03_ENGINE_VERSION.to_owned(),
    }
}

fn binding() -> ScenarioBinding {
    ScenarioBinding {
        scenario_id: "sc-1".into(),
        scenario_version: "1".into(),
        configuration_id: "cfg".into(),
    }
}

fn case_with(
    expectations: Vec<Expectation>,
    observation: SimulationResult,
    pins: Option<EnginePins>,
) -> RegressionCase {
    RegressionCase {
        case_id: "case-1".into(),
        case_version: "1".into(),
        scenario_binding: binding(),
        expectations,
        observation: RegressionObservation {
            simulation_result: observation,
        },
        engine_pins: pins,
        notes: None,
    }
}

fn bal_exact(id: &str, amount: i128, applicable: bool) -> Expectation {
    Expectation {
        expectation_id: id.into(),
        applicable,
        class: ExpectationClass::FinalBalanceExact {
            account_id: "alice".into(),
            asset_id: "USD".into(),
            facet_id: "available".into(),
            amount,
        },
    }
}

fn bal_tol(id: &str, amount: i128, tolerance: u128) -> Expectation {
    Expectation {
        expectation_id: id.into(),
        applicable: true,
        class: ExpectationClass::FinalBalanceAbsoluteTolerance {
            account_id: "alice".into(),
            asset_id: "USD".into(),
            facet_id: "available".into(),
            amount,
            tolerance,
        },
    }
}

#[test]
fn m06_engine_version_exported() {
    assert_eq!(M06_ENGINE_VERSION, "m06-0.1.0");
}

#[test]
fn exact_balance_match() {
    let sim = base_sim(state_with(100, Some(0)));
    let r = evaluate_regression(&case_with(vec![bal_exact("e1", 100, true)], sim, None));
    assert_eq!(r.verdict, RegressionVerdict::Match);
    assert!(r.mismatches.is_empty());
    assert!(r.error.is_none());
    assert_eq!(r.expectations_evaluated, 1);
}

#[test]
fn exact_balance_mismatch() {
    let sim = base_sim(state_with(100, Some(0)));
    let r = evaluate_regression(&case_with(vec![bal_exact("e1", 99, true)], sim, None));
    assert_eq!(r.verdict, RegressionVerdict::Mismatch);
    assert_eq!(r.mismatches.len(), 1);
    assert_eq!(r.mismatches[0].expectation_id, "e1");
    assert_eq!(
        r.mismatches[0].mismatch_class,
        MismatchClass::ValueInequality
    );
    assert_eq!(r.mismatches[0].operator, ComparisonOperator::Exact);
    assert_eq!(r.mismatches[0].ordinal, 0);
}

#[test]
fn absolute_tolerance_match_inclusive_boundary() {
    let sim = base_sim(state_with(10, None));
    let r = evaluate_regression(&case_with(vec![bal_tol("t1", 7, 3)], sim, None));
    assert_eq!(r.verdict, RegressionVerdict::Match);
}

#[test]
fn absolute_tolerance_mismatch() {
    // observed=10, expected=7, tolerance=2 → |10-7|=3 > 2 → MISMATCH
    // §13: evidence observed must be the actual balance, not the difference.
    let sim = base_sim(state_with(10, None));
    let r = evaluate_regression(&case_with(vec![bal_tol("t1", 7, 2)], sim, None));
    assert_eq!(r.verdict, RegressionVerdict::Mismatch);
    assert_eq!(r.mismatches[0].class, "FinalBalanceAbsoluteTolerance");
    assert_eq!(r.mismatches[0].expected, StructuredValue::I128(7));
    assert_eq!(r.mismatches[0].observed, StructuredValue::I128(10));
    assert_ne!(r.mismatches[0].observed, StructuredValue::U128(3));
}

#[test]
fn i128_extreme_exact_match() {
    let sim = base_sim(state_with(i128::MAX, None));
    let r = evaluate_regression(&case_with(
        vec![bal_exact("max", i128::MAX, true)],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Match);

    let sim = base_sim(state_with(i128::MIN + 1, None));
    let r = evaluate_regression(&case_with(
        vec![bal_exact("near_min", i128::MIN + 1, true)],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Match);
}

#[test]
fn checked_numeric_overflow_errors() {
    let err = compare_absolute_tolerance(i128::MAX, i128::MIN, 0).unwrap_err();
    assert_eq!(err.error_id, RegressionErrorId::NumericComparisonError);

    let err = compare_absolute_tolerance(i128::MIN, 0, 0).unwrap_err();
    assert_eq!(err.error_id, RegressionErrorId::NumericComparisonError);

    let sim = base_sim(state_with(i128::MAX, None));
    let r = evaluate_regression(&case_with(
        vec![bal_tol("overflow", i128::MIN, 0)],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Error);
    assert_eq!(
        r.error.as_ref().unwrap().error_id,
        RegressionErrorId::NumericComparisonError
    );
}

#[test]
fn absent_balance_is_error_not_zero() {
    let sim = base_sim(EconomicState::new());
    let r = evaluate_regression(&case_with(vec![bal_exact("e1", 0, true)], sim, None));
    assert_eq!(r.verdict, RegressionVerdict::Error);
    assert_eq!(
        r.error.as_ref().unwrap().error_id,
        RegressionErrorId::MissingObservationField
    );
}

#[test]
fn partial_balance_only_declared_cell() {
    // alice matches; bob differs from any unspoken value — still MATCH.
    let sim = base_sim(state_with(100, Some(999)));
    let r = evaluate_regression(&case_with(vec![bal_exact("e1", 100, true)], sim, None));
    assert_eq!(r.verdict, RegressionVerdict::Match);
}

#[test]
fn contradictory_expectations_yield_mismatch() {
    let sim = base_sim(state_with(100, None));
    let r = evaluate_regression(&case_with(
        vec![bal_exact("a", 100, true), bal_exact("b", 50, true)],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Mismatch);
    assert_eq!(r.mismatches.len(), 1);
    assert_eq!(r.mismatches[0].expectation_id, "b");
}

#[test]
fn empty_expectations_match() {
    let sim = base_sim(state_with(1, None));
    let r = evaluate_regression(&case_with(vec![], sim, None));
    assert_eq!(r.verdict, RegressionVerdict::Match);
    assert_eq!(r.expectations_evaluated, 0);
}

#[test]
fn all_non_applicable_expectations_match() {
    let sim = base_sim(state_with(1, None));
    let r = evaluate_regression(&case_with(
        vec![bal_exact("a", 999, false), bal_exact("b", 0, false)],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Match);
    assert_eq!(r.expectations_skipped, 2);
    assert_eq!(r.expectations_evaluated, 0);
    assert!(r.mismatches.is_empty());
}

#[test]
fn non_applicable_absent_balance_skips_without_error() {
    // applicable=false must skip before observation lookup: absent cell ≠ ERROR.
    let sim = base_sim(EconomicState::new());
    let r = evaluate_regression(&case_with(
        vec![
            bal_exact("absent_exact", 0, false),
            Expectation {
                expectation_id: "absent_tol".into(),
                applicable: false,
                class: ExpectationClass::FinalBalanceAbsoluteTolerance {
                    account_id: "ghost".into(),
                    asset_id: "USD".into(),
                    facet_id: "available".into(),
                    amount: 0,
                    tolerance: 0,
                },
            },
        ],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Match);
    assert!(r.mismatches.is_empty());
    assert!(r.error.is_none());
    assert_eq!(r.expectations_skipped, 2);
    assert_eq!(r.expectations_evaluated, 0);
}

#[test]
fn execution_status_exact() {
    let mut sim = base_sim(state_with(0, None));
    sim.execution_status = ExecutionStatus::EarlyTermination {
        reason: "budget".into(),
    };
    let r = evaluate_regression(&case_with(
        vec![Expectation {
            expectation_id: "st".into(),
            applicable: true,
            class: ExpectationClass::ExecutionStatusExact {
                expected: ExpectedExecutionStatus::EarlyTermination {
                    reason_exact: "budget".into(),
                },
            },
        }],
        sim.clone(),
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Match);

    let r = evaluate_regression(&case_with(
        vec![Expectation {
            expectation_id: "st".into(),
            applicable: true,
            class: ExpectationClass::ExecutionStatusExact {
                expected: ExpectedExecutionStatus::NormalCompletion,
            },
        }],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Mismatch);
    assert_eq!(
        r.mismatches[0].mismatch_class,
        MismatchClass::StatusInequality
    );
}

#[test]
fn executed_action_count_exact() {
    let mut sim = base_sim(state_with(0, None));
    sim.executed_action_count = 3;
    let r = evaluate_regression(&case_with(
        vec![Expectation {
            expectation_id: "c".into(),
            applicable: true,
            class: ExpectationClass::ExecutedActionCountExact { count: 3 },
        }],
        sim.clone(),
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Match);

    let r = evaluate_regression(&case_with(
        vec![Expectation {
            expectation_id: "c".into(),
            applicable: true,
            class: ExpectationClass::ExecutedActionCountExact { count: 2 },
        }],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Mismatch);
    assert_eq!(
        r.mismatches[0].mismatch_class,
        MismatchClass::CountInequality
    );
}

#[test]
fn step_disposition_exact() {
    let state = state_with(0, None);
    let mut sim = base_sim(state.clone());
    sim.step_records = vec![StepResult {
        step_index: 0,
        action: noop_action(),
        classification: AttemptClassification::AttemptedM01EconomicOutcome,
        state_before: Some(state.clone()),
        state_after: Some(state.clone()),
        kernel_outcome: Some(economic_outcome(
            EconomicDisposition::AcceptedEffective,
            &state,
        )),
        execution_context: Some(ExecutionContext::new(0, "cfg")),
        before_action: vec![],
        after_action: vec![],
    }];
    sim.executed_action_count = 1;

    let r = evaluate_regression(&case_with(
        vec![Expectation {
            expectation_id: "d".into(),
            applicable: true,
            class: ExpectationClass::StepDispositionExact {
                step_index: 0,
                disposition: ExpectedDisposition::Accepted,
            },
        }],
        sim.clone(),
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Match);

    let r = evaluate_regression(&case_with(
        vec![Expectation {
            expectation_id: "d".into(),
            applicable: true,
            class: ExpectationClass::StepDispositionExact {
                step_index: 0,
                disposition: ExpectedDisposition::Rejected,
            },
        }],
        sim.clone(),
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Mismatch);

    let r = evaluate_regression(&case_with(
        vec![Expectation {
            expectation_id: "d".into(),
            applicable: true,
            class: ExpectationClass::StepDispositionExact {
                step_index: 5,
                disposition: ExpectedDisposition::Accepted,
            },
        }],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Error);
    assert_eq!(
        r.error.as_ref().unwrap().error_id,
        RegressionErrorId::MissingObservationField
    );
}

#[test]
fn step_disposition_without_economic_disposition_is_mismatch() {
    // In-range step with no inventable disposition → MISMATCH (§6.3), not ERROR.
    let state = state_with(0, None);
    let mut sim = base_sim(state.clone());
    sim.step_records = vec![StepResult {
        step_index: 0,
        action: noop_action(),
        classification: AttemptClassification::AttemptedM01Error,
        state_before: Some(state),
        state_after: None,
        kernel_outcome: None,
        execution_context: None,
        before_action: vec![],
        after_action: vec![],
    }];
    let r = evaluate_regression(&case_with(
        vec![Expectation {
            expectation_id: "d".into(),
            applicable: true,
            class: ExpectationClass::StepDispositionExact {
                step_index: 0,
                disposition: ExpectedDisposition::Accepted,
            },
        }],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Mismatch);
    assert!(r.error.is_none());
    assert_eq!(r.mismatches.len(), 1);
    assert_eq!(r.mismatches[0].class, "StepDispositionExact");
    assert_eq!(
        r.mismatches[0].expected,
        StructuredValue::Disposition(ExpectedDisposition::Accepted)
    );
    assert_eq!(r.mismatches[0].observed, StructuredValue::None);
    assert_eq!(
        r.mismatches[0].mismatch_class,
        MismatchClass::DispositionInequality
    );
}

#[test]
fn invariant_cardinality_zero() {
    let mut sim = base_sim(state_with(0, None));
    sim.step_records = vec![StepResult {
        step_index: 0,
        action: noop_action(),
        classification: AttemptClassification::AttemptedM01EconomicOutcome,
        state_before: None,
        state_after: None,
        kernel_outcome: None,
        execution_context: None,
        before_action: vec![],
        after_action: vec![],
    }];
    let r = evaluate_regression(&case_with(
        vec![Expectation {
            expectation_id: "inv".into(),
            applicable: true,
            class: ExpectationClass::InvariantKindExact {
                invariant_id: "missing".into(),
                phase: ExpectationPhase::BeforeAction,
                step_index: 0,
                expected_kind: InvariantResultKind::Pass,
            },
        }],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Error);
    assert_eq!(
        r.error.as_ref().unwrap().error_id,
        RegressionErrorId::MissingObservationField
    );
}

#[test]
fn invariant_cardinality_one_match() {
    let mut sim = base_sim(state_with(0, None));
    sim.step_records = vec![StepResult {
        step_index: 0,
        action: noop_action(),
        classification: AttemptClassification::AttemptedM01EconomicOutcome,
        state_before: None,
        state_after: None,
        kernel_outcome: None,
        execution_context: None,
        before_action: vec![InvariantEvaluationRecord::Executed {
            point: EvaluationPoint::BeforeAction,
            invariant_id: "inv-a".into(),
            outcome: pass_outcome("inv-a"),
        }],
        after_action: vec![],
    }];
    let r = evaluate_regression(&case_with(
        vec![Expectation {
            expectation_id: "inv".into(),
            applicable: true,
            class: ExpectationClass::InvariantKindExact {
                invariant_id: "inv-a".into(),
                phase: ExpectationPhase::BeforeAction,
                step_index: 0,
                expected_kind: InvariantResultKind::Pass,
            },
        }],
        sim.clone(),
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Match);

    let r = evaluate_regression(&case_with(
        vec![Expectation {
            expectation_id: "inv".into(),
            applicable: true,
            class: ExpectationClass::InvariantKindExact {
                invariant_id: "inv-a".into(),
                phase: ExpectationPhase::BeforeAction,
                step_index: 0,
                expected_kind: InvariantResultKind::Fail,
            },
        }],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Mismatch);
    assert_eq!(
        r.mismatches[0].mismatch_class,
        MismatchClass::KindInequality
    );
}

#[test]
fn invariant_cardinality_gt_one_ambiguous() {
    let mut sim = base_sim(state_with(0, None));
    sim.step_records = vec![StepResult {
        step_index: 0,
        action: noop_action(),
        classification: AttemptClassification::AttemptedM01EconomicOutcome,
        state_before: None,
        state_after: None,
        kernel_outcome: None,
        execution_context: None,
        before_action: vec![
            InvariantEvaluationRecord::Executed {
                point: EvaluationPoint::BeforeAction,
                invariant_id: "dup".into(),
                outcome: pass_outcome("dup"),
            },
            InvariantEvaluationRecord::Executed {
                point: EvaluationPoint::BeforeAction,
                invariant_id: "dup".into(),
                outcome: fail_outcome("dup"),
            },
        ],
        after_action: vec![],
    }];
    let r = evaluate_regression(&case_with(
        vec![Expectation {
            expectation_id: "inv".into(),
            applicable: true,
            class: ExpectationClass::InvariantKindExact {
                invariant_id: "dup".into(),
                phase: ExpectationPhase::BeforeAction,
                step_index: 0,
                expected_kind: InvariantResultKind::Pass,
            },
        }],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Error);
    assert_eq!(
        r.error.as_ref().unwrap().error_id,
        RegressionErrorId::AmbiguousObservation
    );
}

#[test]
fn completion_invariant_cardinality() {
    let mut sim = base_sim(state_with(0, None));
    sim.completion_evaluation_records = vec![InvariantEvaluationRecord::Executed {
        point: EvaluationPoint::OnSimulationCompletion,
        invariant_id: "c-inv".into(),
        outcome: pass_outcome("c-inv"),
    }];
    let r = evaluate_regression(&case_with(
        vec![Expectation {
            expectation_id: "c".into(),
            applicable: true,
            class: ExpectationClass::CompletionInvariantKindExact {
                invariant_id: "c-inv".into(),
                expected_kind: InvariantResultKind::Pass,
            },
        }],
        sim.clone(),
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Match);

    // cardinality 0
    let r = evaluate_regression(&case_with(
        vec![Expectation {
            expectation_id: "c".into(),
            applicable: true,
            class: ExpectationClass::CompletionInvariantKindExact {
                invariant_id: "absent".into(),
                expected_kind: InvariantResultKind::Pass,
            },
        }],
        sim.clone(),
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Error);
    assert_eq!(
        r.error.as_ref().unwrap().error_id,
        RegressionErrorId::MissingObservationField
    );

    // cardinality >1
    sim.completion_evaluation_records
        .push(InvariantEvaluationRecord::Executed {
            point: EvaluationPoint::OnSimulationCompletion,
            invariant_id: "c-inv".into(),
            outcome: fail_outcome("c-inv"),
        });
    let r = evaluate_regression(&case_with(
        vec![Expectation {
            expectation_id: "c".into(),
            applicable: true,
            class: ExpectationClass::CompletionInvariantKindExact {
                invariant_id: "c-inv".into(),
                expected_kind: InvariantResultKind::Pass,
            },
        }],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Error);
    assert_eq!(
        r.error.as_ref().unwrap().error_id,
        RegressionErrorId::AmbiguousObservation
    );
}

#[test]
fn not_executed_observation_error() {
    let mut sim = base_sim(state_with(0, None));
    sim.step_records = vec![StepResult {
        step_index: 0,
        action: noop_action(),
        classification: AttemptClassification::NotAttempted,
        state_before: None,
        state_after: None,
        kernel_outcome: None,
        execution_context: None,
        before_action: vec![InvariantEvaluationRecord::NotExecuted {
            point: EvaluationPoint::BeforeAction,
            invariant_id: "inv-a".into(),
            reason: "skipped".into(),
        }],
        after_action: vec![],
    }];
    let r = evaluate_regression(&case_with(
        vec![Expectation {
            expectation_id: "inv".into(),
            applicable: true,
            class: ExpectationClass::InvariantKindExact {
                invariant_id: "inv-a".into(),
                phase: ExpectationPhase::BeforeAction,
                step_index: 0,
                expected_kind: InvariantResultKind::Pass,
            },
        }],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Error);
    assert_eq!(
        r.error.as_ref().unwrap().error_id,
        RegressionErrorId::NotExecutedObservation
    );
}

#[test]
fn mandatory_scenario_binding() {
    let mut sim = base_sim(state_with(0, None));
    sim.scenario_id = ScenarioId::new("other");
    let r = evaluate_regression(&case_with(vec![], sim, None));
    assert_eq!(r.verdict, RegressionVerdict::Error);
    assert_eq!(
        r.error.as_ref().unwrap().error_id,
        RegressionErrorId::ObservationBindingMismatch
    );
}

#[test]
fn engine_pin_mismatch() {
    let sim = base_sim(state_with(0, None));
    let pins = EnginePins {
        m01_engine_version: Some("wrong-m01".into()),
        m02_engine_version: None,
        m03_engine_version: None,
    };
    let r = evaluate_regression(&case_with(vec![], sim, Some(pins)));
    assert_eq!(r.verdict, RegressionVerdict::Error);
    assert_eq!(
        r.error.as_ref().unwrap().error_id,
        RegressionErrorId::EnginePinMismatch
    );
}

#[test]
fn error_after_prior_mismatch_aborts_with_error() {
    let sim = base_sim(state_with(100, None));
    // e1 would mismatch; e2 triggers MissingObservationField → final ERROR
    let r = evaluate_regression(&case_with(
        vec![
            bal_exact("e1", 50, true),
            Expectation {
                expectation_id: "e2".into(),
                applicable: true,
                class: ExpectationClass::FinalBalanceExact {
                    account_id: "ghost".into(),
                    asset_id: "USD".into(),
                    facet_id: "available".into(),
                    amount: 0,
                },
            },
        ],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Error);
    assert_eq!(
        r.error.as_ref().unwrap().error_id,
        RegressionErrorId::MissingObservationField
    );
    // §14.1: mismatches[] empty unless MISMATCH
    assert!(r.mismatches.is_empty());
    assert_eq!(r.expectations_evaluated, 2);
}

#[test]
fn deterministic_repeated_evaluation() {
    let sim = base_sim(state_with(42, Some(7)));
    let case = case_with(
        vec![
            bal_exact("a", 42, true),
            bal_exact("b", 0, true), // mismatch on alice? wait — bal_exact always alice
            Expectation {
                expectation_id: "c".into(),
                applicable: true,
                class: ExpectationClass::FinalBalanceExact {
                    account_id: "bob".into(),
                    asset_id: "USD".into(),
                    facet_id: "available".into(),
                    amount: 1, // mismatch
                },
            },
        ],
        sim,
        None,
    );
    let r1 = evaluate_regression(&case);
    let r2 = evaluate_regression(&case);
    assert_eq!(r1, r2);
    assert_eq!(r1.verdict, RegressionVerdict::Mismatch);
}

#[test]
fn declaration_order_mismatch_evidence() {
    let sim = base_sim(state_with(100, None));
    let r = evaluate_regression(&case_with(
        vec![
            bal_exact("first", 1, true),
            bal_exact("second", 2, true),
            bal_exact("third", 100, true),
        ],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Mismatch);
    assert_eq!(r.mismatches.len(), 2);
    assert_eq!(r.mismatches[0].expectation_id, "first");
    assert_eq!(r.mismatches[0].ordinal, 0);
    assert_eq!(r.mismatches[1].expectation_id, "second");
    assert_eq!(r.mismatches[1].ordinal, 1);
}

#[test]
fn authority_boundary_no_mutation_no_execution() {
    let sim = base_sim(state_with(10, Some(20)));
    let before = sim.clone();
    let case = case_with(vec![bal_exact("e1", 10, true)], sim.clone(), None);
    let r = evaluate_regression(&case);
    assert_eq!(r.verdict, RegressionVerdict::Match);
    // Observation state unchanged by comparison.
    assert_eq!(case.observation.simulation_result, before);
    assert_eq!(
        case.observation
            .simulation_result
            .final_state
            .balance_if_present(&AccountId::new("alice"), &AssetId::new("USD"), &facet()),
        Some(10)
    );
    // Structural: evaluate_regression takes &RegressionCase — no run_simulation /
    // evaluate_invariant / adversarial generation in this path.
    let _ = &sim.execution_status;
}

#[test]
fn duplicate_expectation_id_error() {
    let sim = base_sim(state_with(0, None));
    let r = evaluate_regression(&case_with(
        vec![bal_exact("dup", 0, true), bal_exact("dup", 1, true)],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Error);
    assert_eq!(
        r.error.as_ref().unwrap().error_id,
        RegressionErrorId::InvalidExpectation
    );
}

#[test]
fn m02_fail_without_expectation_may_still_match() {
    let mut sim = base_sim(state_with(5, None));
    sim.completion_evaluation_records = vec![InvariantEvaluationRecord::Executed {
        point: EvaluationPoint::OnSimulationCompletion,
        invariant_id: "failing".into(),
        outcome: fail_outcome("failing"),
    }];
    let r = evaluate_regression(&case_with(vec![bal_exact("e1", 5, true)], sim, None));
    assert_eq!(r.verdict, RegressionVerdict::Match);
}

#[test]
fn fatal_with_matching_status_may_match() {
    use aivoguard::{FatalCause, SimulationError, SimulationErrorClass};
    let mut sim = base_sim(state_with(0, None));
    sim.execution_status = ExecutionStatus::FatalTermination {
        cause: FatalCause::Simulation(SimulationError::new(
            SimulationErrorClass::SimulationEngineError,
            "boom",
        )),
    };
    let r = evaluate_regression(&case_with(
        vec![Expectation {
            expectation_id: "st".into(),
            applicable: true,
            class: ExpectationClass::ExecutionStatusExact {
                expected: ExpectedExecutionStatus::FatalTermination {
                    cause_class: ExpectedFatalCauseClass::Simulation,
                },
            },
        }],
        sim,
        None,
    ));
    assert_eq!(r.verdict, RegressionVerdict::Match);
}

#[test]
fn numeric_unit_boundary_helpers() {
    assert_eq!(
        compare_absolute_tolerance(10, 7, 3).unwrap(),
        ToleranceCompare::Satisfied
    );
    assert!(matches!(
        compare_absolute_tolerance(10, 7, 2).unwrap(),
        ToleranceCompare::Unsatisfied {
            absolute_difference_u128: 3
        }
    ));
}
