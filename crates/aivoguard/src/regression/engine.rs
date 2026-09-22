//! M06 regression evaluation engine.

use crate::invariant::InvariantResultKind;
use crate::kernel::{AccountId, AssetId, FacetId, KernelOutcome};
use crate::regression::error::{RegressionError, RegressionErrorId};
use crate::regression::numeric::{compare_absolute_tolerance, ToleranceCompare};
use crate::regression::types::{
    ComparisonOperator, ENGINE_VERSION, EnginePins, Expectation, ExpectationClass,
    ExpectationPhase, MismatchClass, MismatchEvidence, RegressionCase, RegressionProvenance,
    RegressionResult, RegressionVerdict, StructuredValue,
};
use crate::simulator::{EvaluationPoint, InvariantEvaluationRecord, SimulationResult};

/// Evaluate a regression case against its authoritative observation.
#[must_use]
pub fn evaluate_regression(case: &RegressionCase) -> RegressionResult {
    let provenance = build_provenance(case);
    let base = |verdict: RegressionVerdict,
                mismatches: Vec<MismatchEvidence>,
                error: Option<RegressionError>,
                evaluated: u64,
                skipped: u64| RegressionResult {
        verdict,
        case_id: case.case_id.clone(),
        case_version: case.case_version.clone(),
        mismatches,
        error,
        expectations_evaluated: evaluated,
        expectations_skipped: skipped,
        provenance: provenance.clone(),
    };

    if let Err(err) = validate_case_structure(case) {
        return base(RegressionVerdict::Error, Vec::new(), Some(err), 0, 0);
    }

    if let Err(err) = validate_binding(case) {
        return base(RegressionVerdict::Error, Vec::new(), Some(err), 0, 0);
    }

    if let Err(err) = validate_expectation_set(&case.expectations) {
        return base(RegressionVerdict::Error, Vec::new(), Some(err), 0, 0);
    }

    if let Err(err) = validate_engine_pins(case) {
        return base(RegressionVerdict::Error, Vec::new(), Some(err), 0, 0);
    }

    let sim = &case.observation.simulation_result;
    let mut mismatches = Vec::new();
    let mut evaluated = 0_u64;
    let mut skipped = 0_u64;

    for expectation in &case.expectations {
        if !expectation.applicable {
            skipped = skipped.saturating_add(1);
            continue;
        }
        evaluated = evaluated.saturating_add(1);
        match evaluate_one(expectation, sim) {
            EvalOne::Error(err) => {
                // §14.1: mismatches[] empty unless MISMATCH (P3: prior evidence non-authoritative).
                return base(
                    RegressionVerdict::Error,
                    Vec::new(),
                    Some(err),
                    evaluated,
                    skipped,
                );
            }
            EvalOne::Mismatch(mut evidence) => {
                evidence.ordinal = mismatches.len() as u64;
                mismatches.push(evidence);
            }
            EvalOne::Satisfied => {}
        }
    }

    if mismatches.is_empty() {
        base(RegressionVerdict::Match, mismatches, None, evaluated, skipped)
    } else {
        base(
            RegressionVerdict::Mismatch,
            mismatches,
            None,
            evaluated,
            skipped,
        )
    }
}

enum EvalOne {
    Satisfied,
    Mismatch(MismatchEvidence),
    Error(RegressionError),
}

fn build_provenance(case: &RegressionCase) -> RegressionProvenance {
    let sim = &case.observation.simulation_result;
    RegressionProvenance {
        scenario_id: case.scenario_binding.scenario_id.clone(),
        scenario_version: case.scenario_binding.scenario_version.clone(),
        configuration_id: case.scenario_binding.configuration_id.clone(),
        observation_engine_pins: EnginePins {
            m01_engine_version: Some(sim.m01_engine_version.clone()),
            m02_engine_version: Some(sim.m02_engine_version.clone()),
            m03_engine_version: Some(sim.m03_engine_version.clone()),
        },
        m06_spec_version_label: ENGINE_VERSION.to_owned(),
        evaluation_policy: "ALL_MISMATCHES".to_owned(),
    }
}

fn validate_case_structure(case: &RegressionCase) -> Result<(), RegressionError> {
    if case.case_id.is_empty() {
        return Err(RegressionError::new(
            RegressionErrorId::InvalidRegressionCase,
            "case_id is empty",
        ));
    }
    if case.case_version.is_empty() {
        return Err(RegressionError::new(
            RegressionErrorId::InvalidRegressionCase,
            "case_version is empty",
        ));
    }
    if case.scenario_binding.scenario_id.is_empty()
        || case.scenario_binding.scenario_version.is_empty()
        || case.scenario_binding.configuration_id.is_empty()
    {
        return Err(RegressionError::new(
            RegressionErrorId::InvalidRegressionCase,
            "scenario binding fields must be non-empty",
        ));
    }
    Ok(())
}

fn validate_binding(case: &RegressionCase) -> Result<(), RegressionError> {
    let sim = &case.observation.simulation_result;
    let bind = &case.scenario_binding;
    if bind.scenario_id != sim.scenario_id.as_str()
        || bind.scenario_version != sim.scenario_version
        || bind.configuration_id != sim.configuration_id
    {
        return Err(RegressionError::new(
            RegressionErrorId::ObservationBindingMismatch,
            "scenario binding does not match observation identity",
        )
        .with_field(
            "expected_scenario_id",
            StructuredValue::String(bind.scenario_id.clone()),
        )
        .with_field(
            "observed_scenario_id",
            StructuredValue::String(sim.scenario_id.as_str().to_owned()),
        ));
    }
    Ok(())
}

fn validate_expectation_set(expectations: &[Expectation]) -> Result<(), RegressionError> {
    let mut seen = Vec::new();
    for e in expectations {
        if e.expectation_id.is_empty() {
            return Err(RegressionError::new(
                RegressionErrorId::InvalidExpectation,
                "expectation_id is empty",
            ));
        }
        if seen.iter().any(|id: &String| id == &e.expectation_id) {
            return Err(RegressionError::new(
                RegressionErrorId::InvalidExpectation,
                format!("duplicate expectation_id {}", e.expectation_id),
            )
            .with_field(
                "expectation_id",
                StructuredValue::String(e.expectation_id.clone()),
            ));
        }
        seen.push(e.expectation_id.clone());
    }
    Ok(())
}

fn validate_engine_pins(case: &RegressionCase) -> Result<(), RegressionError> {
    let Some(pins) = &case.engine_pins else {
        return Ok(());
    };
    let sim = &case.observation.simulation_result;
    if let Some(v) = &pins.m01_engine_version {
        if v != &sim.m01_engine_version {
            return Err(pin_mismatch("m01_engine_version", v, &sim.m01_engine_version));
        }
    }
    if let Some(v) = &pins.m02_engine_version {
        if v != &sim.m02_engine_version {
            return Err(pin_mismatch("m02_engine_version", v, &sim.m02_engine_version));
        }
    }
    if let Some(v) = &pins.m03_engine_version {
        if v != &sim.m03_engine_version {
            return Err(pin_mismatch("m03_engine_version", v, &sim.m03_engine_version));
        }
    }
    Ok(())
}

fn pin_mismatch(name: &str, expected: &str, observed: &str) -> RegressionError {
    RegressionError::new(
        RegressionErrorId::EnginePinMismatch,
        format!("{name} pin mismatch"),
    )
    .with_field("pin", StructuredValue::String(name.to_owned()))
    .with_field(
        "expected",
        StructuredValue::String(expected.to_owned()),
    )
    .with_field(
        "observed",
        StructuredValue::String(observed.to_owned()),
    )
}

fn evaluate_one(expectation: &Expectation, sim: &SimulationResult) -> EvalOne {
    match &expectation.class {
        ExpectationClass::FinalBalanceExact {
            account_id,
            asset_id,
            facet_id,
            amount,
        } => eval_balance_exact(expectation, sim, account_id, asset_id, facet_id, *amount),
        ExpectationClass::FinalBalanceAbsoluteTolerance {
            account_id,
            asset_id,
            facet_id,
            amount,
            tolerance,
        } => eval_balance_tolerance(
            expectation,
            sim,
            account_id,
            asset_id,
            facet_id,
            *amount,
            *tolerance,
        ),
        ExpectationClass::ExecutionStatusExact { expected } => {
            if expected.matches(&sim.execution_status) {
                EvalOne::Satisfied
            } else {
                EvalOne::Mismatch(MismatchEvidence {
                    expectation_id: expectation.expectation_id.clone(),
                    class: "ExecutionStatusExact".into(),
                    target: StructuredValue::String("execution_status".into()),
                    expected: StructuredValue::ExecutionStatusLabel(format!("{expected:?}")),
                    observed: StructuredValue::ExecutionStatusLabel(format!(
                        "{:?}",
                        sim.execution_status
                    )),
                    operator: ComparisonOperator::Exact,
                    mismatch_class: MismatchClass::StatusInequality,
                    ordinal: 0,
                })
            }
        }
        ExpectationClass::ExecutedActionCountExact { count } => {
            if sim.executed_action_count == *count {
                EvalOne::Satisfied
            } else {
                EvalOne::Mismatch(MismatchEvidence {
                    expectation_id: expectation.expectation_id.clone(),
                    class: "ExecutedActionCountExact".into(),
                    target: StructuredValue::String("executed_action_count".into()),
                    expected: StructuredValue::U64(*count),
                    observed: StructuredValue::U64(sim.executed_action_count),
                    operator: ComparisonOperator::Exact,
                    mismatch_class: MismatchClass::CountInequality,
                    ordinal: 0,
                })
            }
        }
        ExpectationClass::StepDispositionExact {
            step_index,
            disposition,
        } => eval_step_disposition(expectation, sim, *step_index, *disposition),
        ExpectationClass::InvariantKindExact {
            invariant_id,
            phase,
            step_index,
            expected_kind,
        } => eval_invariant_kind(
            expectation,
            sim,
            invariant_id,
            *phase,
            *step_index,
            *expected_kind,
        ),
        ExpectationClass::CompletionInvariantKindExact {
            invariant_id,
            expected_kind,
        } => eval_completion_invariant(expectation, sim, invariant_id, *expected_kind),
    }
}

fn eval_balance_exact(
    expectation: &Expectation,
    sim: &SimulationResult,
    account_id: &str,
    asset_id: &str,
    facet_id: &str,
    amount: i128,
) -> EvalOne {
    match lookup_balance(sim, account_id, asset_id, facet_id) {
        Err(e) => EvalOne::Error(e),
        Ok(observed) => {
            if observed == amount {
                EvalOne::Satisfied
            } else {
                EvalOne::Mismatch(MismatchEvidence {
                    expectation_id: expectation.expectation_id.clone(),
                    class: "FinalBalanceExact".into(),
                    target: balance_target(account_id, asset_id, facet_id),
                    expected: StructuredValue::I128(amount),
                    observed: StructuredValue::I128(observed),
                    operator: ComparisonOperator::Exact,
                    mismatch_class: MismatchClass::ValueInequality,
                    ordinal: 0,
                })
            }
        }
    }
}

fn eval_balance_tolerance(
    expectation: &Expectation,
    sim: &SimulationResult,
    account_id: &str,
    asset_id: &str,
    facet_id: &str,
    amount: i128,
    tolerance: u128,
) -> EvalOne {
    match lookup_balance(sim, account_id, asset_id, facet_id) {
        Err(e) => EvalOne::Error(e),
        Ok(observed) => match compare_absolute_tolerance(observed, amount, tolerance) {
            Err(e) => EvalOne::Error(e),
            Ok(ToleranceCompare::Satisfied) => EvalOne::Satisfied,
            Ok(ToleranceCompare::Unsatisfied { .. }) => EvalOne::Mismatch(MismatchEvidence {
                expectation_id: expectation.expectation_id.clone(),
                class: "FinalBalanceAbsoluteTolerance".into(),
                target: balance_target(account_id, asset_id, facet_id),
                expected: StructuredValue::I128(amount),
                observed: StructuredValue::I128(observed),
                operator: ComparisonOperator::AbsoluteTolerance,
                mismatch_class: MismatchClass::ValueInequality,
                ordinal: 0,
            }),
        },
    }
}

fn lookup_balance(
    sim: &SimulationResult,
    account_id: &str,
    asset_id: &str,
    facet_id: &str,
) -> Result<i128, RegressionError> {
    let account = AccountId::new(account_id);
    let asset = AssetId::new(asset_id);
    let facet = FacetId::new(facet_id);
    sim.final_state
        .balance_if_present(&account, &asset, &facet)
        .ok_or_else(|| {
            RegressionError::new(
                RegressionErrorId::MissingObservationField,
                "declared balance cell is absent",
            )
            .with_field(
                "account",
                StructuredValue::String(account_id.to_owned()),
            )
            .with_field("asset", StructuredValue::String(asset_id.to_owned()))
            .with_field("facet", StructuredValue::String(facet_id.to_owned()))
        })
}

fn balance_target(account: &str, asset: &str, facet: &str) -> StructuredValue {
    StructuredValue::String(format!("{account}/{asset}/{facet}"))
}

fn eval_step_disposition(
    expectation: &Expectation,
    sim: &SimulationResult,
    step_index: u64,
    disposition: crate::regression::types::ExpectedDisposition,
) -> EvalOne {
    let idx = step_index as usize;
    if idx >= sim.step_records.len() {
        return EvalOne::Error(RegressionError::new(
            RegressionErrorId::MissingObservationField,
            "step_index out of range for step_records",
        )
        .with_field("step_index", StructuredValue::U64(step_index)));
    }
    let step = &sim.step_records[idx];
    let observed_disp = match &step.kernel_outcome {
        Some(KernelOutcome::Economic { disposition: d, .. }) => Some(*d),
        Some(KernelOutcome::Error { .. }) | None => None,
    };
    match observed_disp {
        Some(d) if disposition.matches(d) => EvalOne::Satisfied,
        Some(d) => EvalOne::Mismatch(MismatchEvidence {
            expectation_id: expectation.expectation_id.clone(),
            class: "StepDispositionExact".into(),
            target: StructuredValue::U64(step_index),
            expected: StructuredValue::Disposition(disposition),
            observed: StructuredValue::String(format!("{d:?}")),
            operator: ComparisonOperator::Exact,
            mismatch_class: MismatchClass::DispositionInequality,
            ordinal: 0,
        }),
        None => {
            // No inventable disposition; cannot equal expected economic disposition.
            EvalOne::Mismatch(MismatchEvidence {
                expectation_id: expectation.expectation_id.clone(),
                class: "StepDispositionExact".into(),
                target: StructuredValue::U64(step_index),
                expected: StructuredValue::Disposition(disposition),
                observed: StructuredValue::None,
                operator: ComparisonOperator::Exact,
                mismatch_class: MismatchClass::DispositionInequality,
                ordinal: 0,
            })
        }
    }
}

fn eval_invariant_kind(
    expectation: &Expectation,
    sim: &SimulationResult,
    invariant_id: &str,
    phase: ExpectationPhase,
    step_index: u64,
    expected_kind: InvariantResultKind,
) -> EvalOne {
    let idx = step_index as usize;
    if idx >= sim.step_records.len() {
        return EvalOne::Error(RegressionError::new(
            RegressionErrorId::MissingObservationField,
            "step_index out of range for invariant lookup",
        )
        .with_field("step_index", StructuredValue::U64(step_index)));
    }
    let step = &sim.step_records[idx];
    let records: &[InvariantEvaluationRecord] = match phase {
        ExpectationPhase::BeforeAction => &step.before_action,
        ExpectationPhase::AfterAction => &step.after_action,
    };
    let matches: Vec<&InvariantEvaluationRecord> = records
        .iter()
        .filter(|r| record_invariant_id(r) == invariant_id)
        .collect();
    resolve_invariant_matches(expectation, "InvariantKindExact", &matches, expected_kind)
}

fn eval_completion_invariant(
    expectation: &Expectation,
    sim: &SimulationResult,
    invariant_id: &str,
    expected_kind: InvariantResultKind,
) -> EvalOne {
    // completion_evaluation_records is already completion-scoped by M03.
    let matches: Vec<&InvariantEvaluationRecord> = sim
        .completion_evaluation_records
        .iter()
        .filter(|r| {
            record_invariant_id(r) == invariant_id
                && record_point(r) == EvaluationPoint::OnSimulationCompletion
        })
        .collect();
    resolve_invariant_matches(
        expectation,
        "CompletionInvariantKindExact",
        &matches,
        expected_kind,
    )
}

fn resolve_invariant_matches(
    expectation: &Expectation,
    class_name: &str,
    matches: &[&InvariantEvaluationRecord],
    expected_kind: InvariantResultKind,
) -> EvalOne {
    match matches.len() {
        0 => EvalOne::Error(RegressionError::new(
            RegressionErrorId::MissingObservationField,
            format!("no matching {class_name} observation records"),
        )),
        1 => match matches[0] {
            InvariantEvaluationRecord::NotExecuted { .. } => EvalOne::Error(RegressionError::new(
                RegressionErrorId::NotExecutedObservation,
                "sole matching record is NotExecuted",
            )
            .with_field(
                "expectation_id",
                StructuredValue::String(expectation.expectation_id.clone()),
            )),
            InvariantEvaluationRecord::Executed { outcome, .. } => {
                if outcome.kind == expected_kind {
                    EvalOne::Satisfied
                } else {
                    EvalOne::Mismatch(MismatchEvidence {
                        expectation_id: expectation.expectation_id.clone(),
                        class: class_name.into(),
                        target: StructuredValue::String(expectation.expectation_id.clone()),
                        expected: StructuredValue::InvariantKind(expected_kind),
                        observed: StructuredValue::InvariantKind(outcome.kind),
                        operator: ComparisonOperator::Exact,
                        mismatch_class: MismatchClass::KindInequality,
                        ordinal: 0,
                    })
                }
            }
        },
        n => EvalOne::Error(
            RegressionError::new(
                RegressionErrorId::AmbiguousObservation,
                format!("{n} matching observation records"),
            )
            .with_field("match_count", StructuredValue::U64(n as u64)),
        ),
    }
}

fn record_invariant_id(record: &InvariantEvaluationRecord) -> &str {
    match record {
        InvariantEvaluationRecord::Executed { invariant_id, .. }
        | InvariantEvaluationRecord::NotExecuted { invariant_id, .. } => invariant_id.as_str(),
    }
}

fn record_point(record: &InvariantEvaluationRecord) -> EvaluationPoint {
    match record {
        InvariantEvaluationRecord::Executed { point, .. }
        | InvariantEvaluationRecord::NotExecuted { point, .. } => *point,
    }
}
