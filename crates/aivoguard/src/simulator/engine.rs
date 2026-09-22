//! Authoritative Gate-3 sequential simulation engine.

use crate::invariant::{
    evaluate_invariant, EvaluationTarget, HistoryRecord, Invariant, InvariantScope,
    TransitionRecord, M02_ENGINE_VERSION,
};
use crate::kernel::{
    evaluate, EconomicDisposition, EconomicState, ExecutionContext, KernelOutcome,
    ENGINE_VERSION as M01_ENGINE_VERSION,
};

use crate::simulator::error::{SimulationError, SimulationErrorClass};
use crate::simulator::result::{ExecutionStatus, FatalCause, SimulationResult};
use crate::simulator::scenario::{Scenario, StopCondition};
use crate::simulator::step::{
    AttemptClassification, EvaluationPoint, InvariantEvaluationRecord, StepResult,
};

/// M03 engine version (deterministic evidence / replay identity).
pub const ENGINE_VERSION: &str = "aivoguard-m03-0.0.0";

/// Run a Simulation for an explicit Scenario.
///
/// Implements the frozen Gate-3 normative algorithm (§10). M01 remains the sole
/// economic authority; M02 is optional and read-only.
#[must_use]
pub fn run_simulation(scenario: &Scenario) -> SimulationResult {
    let mut result = empty_result(scenario);

    if let Err(err) = validate_scenario(scenario) {
        result.simulation_errors.push(err.clone());
        result.execution_status = ExecutionStatus::FatalTermination {
            cause: FatalCause::Simulation(err),
        };
        return result;
    }

    let mut current_state = scenario.initial_state.clone();
    let mut executed_action_count: u64 = 0;
    let mut economic_history: Vec<HistoryRecord> = Vec::new();
    let mut early_reason: Option<String> = None;

    for (step_index, action) in scenario.actions.iter().enumerate() {
        // 1–2. Limit check before BEFORE_ACTION / M01.
        if executed_action_count >= scenario.maximum_action_steps {
            let err = SimulationError::new(
                SimulationErrorClass::ExecutionLimitExceeded,
                format!(
                    "executed_action_count ({executed_action_count}) >= maximum_action_steps ({})",
                    scenario.maximum_action_steps
                ),
            );
            result.step_records.push(StepResult {
                step_index,
                action: action.clone(),
                classification: AttemptClassification::NotAttempted,
                state_before: None,
                state_after: None,
                kernel_outcome: None,
                execution_context: None,
                before_action: Vec::new(),
                after_action: Vec::new(),
            });
            result.simulation_errors.push(err.clone());
            result.executed_action_count = executed_action_count;
            result.final_state = current_state;
            result.execution_status = ExecutionStatus::FatalTermination {
                cause: FatalCause::Simulation(err),
            };
            // Fatal → no ON_SIMULATION_COMPLETION
            flatten_invariant_records(&mut result);
            return result;
        }

        let state_i = current_state.clone();

        // 3. BEFORE_ACTION
        let before_records = schedule_invariants(
            EvaluationPoint::BeforeAction,
            scenario,
            &state_i,
            None,
            None,
            &scenario.invariant_plan.before_action,
        );
        if let Some(reason) = evaluate_stop_policy(
            &scenario.stop_policy.conditions,
            &StopFacts {
                completed_step_index: None,
                disposition: None,
                m02_point: Some(EvaluationPoint::BeforeAction),
                m02_records: &before_records,
            },
        ) {
            result.step_records.push(StepResult {
                step_index,
                action: action.clone(),
                classification: AttemptClassification::NotAttempted,
                state_before: Some(state_i),
                state_after: None,
                kernel_outcome: None,
                execution_context: None,
                before_action: before_records,
                after_action: Vec::new(),
            });
            early_reason = Some(reason);
            break;
        }

        // 4–6. ExecutionContext + M01
        let ctx = ExecutionContext {
            logical_order: step_index as u64,
            declared_unix_secs: scenario.declared_unix_secs,
            configuration_id: scenario.configuration_id.clone(),
        };
        let outcome = evaluate(&scenario.world, &state_i, action, &ctx);
        executed_action_count = executed_action_count.saturating_add(1);

        match outcome {
            KernelOutcome::Economic {
                disposition,
                state_before,
                state_after,
                transaction,
                events,
                evidence,
            } => {
                // 7a–7b. Adopt authoritative State_(i+1) before AFTER_ACTION.
                current_state = state_after.clone();
                let kernel_outcome = KernelOutcome::Economic {
                    disposition,
                    state_before: state_before.clone(),
                    state_after: state_after.clone(),
                    transaction: transaction.clone(),
                    events,
                    evidence,
                };

                economic_history.push(HistoryRecord {
                    logical_position: i64::try_from(step_index).unwrap_or(i64::MAX),
                    transition: Some(TransitionRecord {
                        state_before: state_before.clone(),
                        state_after: state_after.clone(),
                        transaction: transaction.clone(),
                    }),
                });

                // 7c. AFTER_ACTION
                let after_records = schedule_invariants(
                    EvaluationPoint::AfterAction,
                    scenario,
                    &state_after,
                    Some((&state_before, &state_after, &transaction)),
                    Some(&economic_history),
                    &scenario.invariant_plan.after_action,
                );

                let step = StepResult {
                    step_index,
                    action: action.clone(),
                    classification: AttemptClassification::AttemptedM01EconomicOutcome,
                    state_before: Some(state_before),
                    state_after: Some(state_after),
                    kernel_outcome: Some(kernel_outcome),
                    execution_context: Some(ctx),
                    before_action: before_records,
                    after_action: after_records,
                };

                // 7e. Stop policy — declared condition order is authoritative.
                if let Some(reason) = evaluate_stop_policy(
                    &scenario.stop_policy.conditions,
                    &StopFacts {
                        completed_step_index: Some(step_index),
                        disposition: Some(disposition),
                        m02_point: Some(EvaluationPoint::AfterAction),
                        m02_records: &step.after_action,
                    },
                ) {
                    result.step_records.push(step);
                    early_reason = Some(reason);
                    break;
                }

                result.step_records.push(step);
            }
            KernelOutcome::Error { error, evidence } => {
                // M01 Error path: preserve State_i; no AFTER_ACTION invoke; Fatal.
                let after_records = schedule_invariants(
                    EvaluationPoint::AfterAction,
                    scenario,
                    &state_i,
                    None, // no authoritative post-action Transition / State_(i+1)
                    None,
                    &scenario.invariant_plan.after_action,
                );
                result.step_records.push(StepResult {
                    step_index,
                    action: action.clone(),
                    classification: AttemptClassification::AttemptedM01Error,
                    state_before: Some(state_i),
                    state_after: None,
                    kernel_outcome: Some(KernelOutcome::Error {
                        error: error.clone(),
                        evidence,
                    }),
                    execution_context: Some(ctx),
                    before_action: before_records,
                    after_action: after_records,
                });
                result.executed_action_count = executed_action_count;
                result.final_state = current_state;
                result.execution_status = ExecutionStatus::FatalTermination {
                    cause: FatalCause::M01(error),
                };
                flatten_invariant_records(&mut result);
                return result;
            }
        }
    }

    result.executed_action_count = executed_action_count;
    result.final_state = current_state.clone();

    if let Some(reason) = early_reason {
        result.execution_status = ExecutionStatus::EarlyTermination { reason };
    } else {
        result.execution_status = ExecutionStatus::NormalCompletion;
    }
    result.completion_evaluation_records = schedule_invariants(
        EvaluationPoint::OnSimulationCompletion,
        scenario,
        &current_state,
        None,
        Some(&economic_history),
        &scenario.invariant_plan.on_completion,
    );

    flatten_invariant_records(&mut result);
    result
}

/// Build the M02 authoritative economic history view from M03 step records.
///
/// Only `ATTEMPTED_M01_ECONOMIC_OUTCOME` steps contribute. M01 errors are excluded.
#[must_use]
pub fn economic_history_view(steps: &[StepResult]) -> Vec<HistoryRecord> {
    let mut out = Vec::new();
    for step in steps {
        if step.classification != AttemptClassification::AttemptedM01EconomicOutcome {
            continue;
        }
        let Some(KernelOutcome::Economic {
            state_before,
            state_after,
            transaction,
            ..
        }) = &step.kernel_outcome
        else {
            continue;
        };
        out.push(HistoryRecord {
            logical_position: i64::try_from(step.step_index).unwrap_or(i64::MAX),
            transition: Some(TransitionRecord {
                state_before: state_before.clone(),
                state_after: state_after.clone(),
                transaction: transaction.clone(),
            }),
        });
    }
    out
}

fn empty_result(scenario: &Scenario) -> SimulationResult {
    SimulationResult {
        execution_status: ExecutionStatus::NormalCompletion,
        scenario_id: scenario.id.clone(),
        scenario_version: scenario.version.clone(),
        configuration_id: scenario.configuration_id.clone(),
        initial_state: scenario.initial_state.clone(),
        final_state: scenario.initial_state.clone(),
        step_records: Vec::new(),
        invariant_evaluation_records: Vec::new(),
        completion_evaluation_records: Vec::new(),
        simulation_errors: Vec::new(),
        executed_action_count: 0,
        m01_engine_version: M01_ENGINE_VERSION.to_owned(),
        m02_engine_version: M02_ENGINE_VERSION.to_owned(),
        m03_engine_version: ENGINE_VERSION.to_owned(),
    }
}

fn validate_scenario(scenario: &Scenario) -> Result<(), SimulationError> {
    if scenario.id.as_str().is_empty() {
        return Err(SimulationError::new(
            SimulationErrorClass::MissingSimulationInput,
            "scenario id is required",
        ));
    }
    if scenario.configuration_id.is_empty() {
        return Err(SimulationError::new(
            SimulationErrorClass::InvalidSimulationConfiguration,
            "configuration_id is required",
        ));
    }
    Ok(())
}

/// Facts available when evaluating `StopPolicy` (only what the call site has).
struct StopFacts<'a> {
    /// Set only after an Economic M01 step has been fully processed.
    completed_step_index: Option<usize>,
    /// Set only after an Economic M01 outcome.
    disposition: Option<EconomicDisposition>,
    /// Point whose M02 records may be inspected.
    m02_point: Option<EvaluationPoint>,
    m02_records: &'a [InvariantEvaluationRecord],
}

/// Declared `StopPolicy.conditions` order is authoritative; first match wins.
fn evaluate_stop_policy(conditions: &[StopCondition], facts: &StopFacts<'_>) -> Option<String> {
    for (condition_index, condition) in conditions.iter().enumerate() {
        if let Some(detail) = condition_match_detail(condition, facts) {
            return Some(format!(
                "stop condition[{condition_index}] matched: {detail}"
            ));
        }
    }
    None
}

fn condition_match_detail(condition: &StopCondition, facts: &StopFacts<'_>) -> Option<String> {
    match condition {
        StopCondition::AfterCompletedStep(i) => {
            if facts.completed_step_index == Some(*i) {
                Some(format!("AfterCompletedStep({i})"))
            } else {
                None
            }
        }
        StopCondition::OnEconomicDisposition(d) => {
            if facts.disposition == Some(*d) {
                Some(format!("OnEconomicDisposition({d:?})"))
            } else {
                None
            }
        }
        StopCondition::OnM02Kind { point, kind } => {
            if facts.m02_point != Some(*point) {
                return None;
            }
            for rec in facts.m02_records {
                if let InvariantEvaluationRecord::Executed {
                    point: p,
                    outcome,
                    invariant_id,
                    ..
                } = rec
                {
                    if *p == *point && outcome.kind == *kind {
                        return Some(format!(
                            "OnM02Kind {{ point: {point:?}, kind: {kind:?}, invariant: {invariant_id} }}"
                        ));
                    }
                }
            }
            None
        }
    }
}

/// Available authoritative target kinds at an evaluation point.
///
/// M03 schedules: if the invariant's required target kind is available, invoke
/// M02 (which owns PASS/FAIL/ERROR including `INCOMPATIBLE_TARGET`). If the
/// required kind is unavailable, record `NotExecuted` — do not invent an M02
/// result.
fn schedule_invariants(
    point: EvaluationPoint,
    scenario: &Scenario,
    state: &EconomicState,
    transition: Option<(&EconomicState, &EconomicState, &crate::kernel::Transaction)>,
    history: Option<&[HistoryRecord]>,
    invariants: &[Invariant],
) -> Vec<InvariantEvaluationRecord> {
    let mut records = Vec::new();
    for inv in invariants {
        match (point, inv.scope) {
            (
                EvaluationPoint::BeforeAction
                | EvaluationPoint::AfterAction
                | EvaluationPoint::OnSimulationCompletion,
                InvariantScope::State,
            ) => {
                // AFTER_ACTION requires authoritative State_(i+1); only when
                // `transition` is Some (Economic path). On M01 Error, transition
                // is None → NotExecuted below via AfterAction+State with no transition.
                if point == EvaluationPoint::AfterAction && transition.is_none() {
                    records.push(not_executed(
                        point,
                        inv,
                        "AFTER_ACTION State not available: no authoritative State_(i+1) (M01 Error)",
                    ));
                    continue;
                }
                let outcome = evaluate_invariant(
                    inv,
                    EvaluationTarget::State {
                        world: &scenario.world,
                        state,
                    },
                );
                records.push(executed(point, inv, outcome));
            }
            (EvaluationPoint::AfterAction, InvariantScope::Transition) => {
                let Some((before, after, tx)) = transition else {
                    records.push(not_executed(
                        point,
                        inv,
                        "AFTER_ACTION Transition not available: no authoritative post-action Transition (M01 Error)",
                    ));
                    continue;
                };
                let outcome = evaluate_invariant(
                    inv,
                    EvaluationTarget::Transition {
                        world: &scenario.world,
                        state_before: before,
                        state_after: after,
                        transaction: tx,
                    },
                );
                records.push(executed(point, inv, outcome));
            }
            (
                EvaluationPoint::AfterAction | EvaluationPoint::OnSimulationCompletion,
                InvariantScope::History,
            ) => {
                let Some(hist) = history else {
                    records.push(not_executed(
                        point,
                        inv,
                        "History target not available at this evaluation point",
                    ));
                    continue;
                };
                // AFTER_ACTION on M01 Error: no economic history update for this
                // step, but prior history may exist. Spec: all AFTER_ACTION on
                // M01 Error → NOT_EXECUTED (no authoritative post-action result).
                if point == EvaluationPoint::AfterAction && transition.is_none() {
                    records.push(not_executed(
                        point,
                        inv,
                        "AFTER_ACTION History not executed: M01 non-economic error (no authoritative post-action result)",
                    ));
                    continue;
                }
                let outcome = evaluate_invariant(
                    inv,
                    EvaluationTarget::History {
                        world: &scenario.world,
                        records: hist,
                    },
                );
                records.push(executed(point, inv, outcome));
            }
            (
                EvaluationPoint::BeforeAction,
                InvariantScope::Transition | InvariantScope::History,
            ) => {
                records.push(not_executed(
                    point,
                    inv,
                    "BEFORE_ACTION provides State_i only; required target kind unavailable",
                ));
            }
            (EvaluationPoint::OnSimulationCompletion, InvariantScope::Transition) => {
                records.push(not_executed(
                    point,
                    inv,
                    "ON_SIMULATION_COMPLETION does not provide a single Transition target",
                ));
            }
        }
    }
    records
}

fn executed(
    point: EvaluationPoint,
    inv: &Invariant,
    outcome: crate::invariant::InvariantOutcome,
) -> InvariantEvaluationRecord {
    InvariantEvaluationRecord::Executed {
        point,
        invariant_id: inv.id.as_str().to_owned(),
        outcome,
    }
}

fn not_executed(
    point: EvaluationPoint,
    inv: &Invariant,
    reason: &str,
) -> InvariantEvaluationRecord {
    InvariantEvaluationRecord::NotExecuted {
        point,
        invariant_id: inv.id.as_str().to_owned(),
        reason: reason.to_owned(),
    }
}

fn flatten_invariant_records(result: &mut SimulationResult) {
    let mut flat = Vec::new();
    for step in &result.step_records {
        flat.extend(step.before_action.iter().cloned());
        flat.extend(step.after_action.iter().cloned());
    }
    result.invariant_evaluation_records = flat;
}
