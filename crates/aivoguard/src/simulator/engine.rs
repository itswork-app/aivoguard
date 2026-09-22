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
        let before_records =
            run_before_action(scenario, &state_i, &scenario.invariant_plan.before_action);
        if let Some(reason) = stop_after_m02(
            &scenario.stop_policy.conditions,
            EvaluationPoint::BeforeAction,
            &before_records,
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
                let after_records = run_after_action_economic(
                    scenario,
                    &state_before,
                    &state_after,
                    &transaction,
                    &economic_history,
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

                // 7e. Stop policy
                if let Some(reason) =
                    stop_after_economic(&scenario.stop_policy.conditions, step_index, disposition)
                {
                    result.step_records.push(step);
                    early_reason = Some(reason);
                    break;
                }
                if let Some(reason) = stop_after_m02(
                    &scenario.stop_policy.conditions,
                    EvaluationPoint::AfterAction,
                    &step.after_action,
                ) {
                    result.step_records.push(step);
                    early_reason = Some(reason);
                    break;
                }

                result.step_records.push(step);
            }
            KernelOutcome::Error { error, evidence } => {
                // M01 Error path: preserve State_i; no AFTER_ACTION; Fatal.
                let after_records =
                    not_executed_after_on_error(&scenario.invariant_plan.after_action);
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
    result.completion_evaluation_records = run_completion(
        scenario,
        &current_state,
        &economic_history,
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

fn run_before_action(
    scenario: &Scenario,
    state_i: &EconomicState,
    invariants: &[Invariant],
) -> Vec<InvariantEvaluationRecord> {
    let mut records = Vec::new();
    for inv in invariants {
        if inv.scope != InvariantScope::State {
            records.push(InvariantEvaluationRecord::NotExecuted {
                point: EvaluationPoint::BeforeAction,
                invariant_id: inv.id.as_str().to_owned(),
                reason: "BEFORE_ACTION requires State scope".into(),
            });
            continue;
        }
        let outcome = evaluate_invariant(
            inv,
            EvaluationTarget::State {
                world: &scenario.world,
                state: state_i,
            },
        );
        records.push(InvariantEvaluationRecord::Executed {
            point: EvaluationPoint::BeforeAction,
            invariant_id: inv.id.as_str().to_owned(),
            outcome,
        });
    }
    records
}

fn run_after_action_economic(
    scenario: &Scenario,
    state_before: &EconomicState,
    state_after: &EconomicState,
    transaction: &crate::kernel::Transaction,
    history: &[HistoryRecord],
    invariants: &[Invariant],
) -> Vec<InvariantEvaluationRecord> {
    let mut records = Vec::new();
    for inv in invariants {
        let outcome = match inv.scope {
            InvariantScope::State => evaluate_invariant(
                inv,
                EvaluationTarget::State {
                    world: &scenario.world,
                    state: state_after,
                },
            ),
            InvariantScope::Transition => evaluate_invariant(
                inv,
                EvaluationTarget::Transition {
                    world: &scenario.world,
                    state_before,
                    state_after,
                    transaction,
                },
            ),
            InvariantScope::History => evaluate_invariant(
                inv,
                EvaluationTarget::History {
                    world: &scenario.world,
                    records: history,
                },
            ),
        };
        records.push(InvariantEvaluationRecord::Executed {
            point: EvaluationPoint::AfterAction,
            invariant_id: inv.id.as_str().to_owned(),
            outcome,
        });
    }
    records
}

fn not_executed_after_on_error(invariants: &[Invariant]) -> Vec<InvariantEvaluationRecord> {
    invariants
        .iter()
        .map(|inv| InvariantEvaluationRecord::NotExecuted {
            point: EvaluationPoint::AfterAction,
            invariant_id: inv.id.as_str().to_owned(),
            reason: "AFTER_ACTION not executed: M01 non-economic error (no authoritative post-action result)"
                .into(),
        })
        .collect()
}

fn run_completion(
    scenario: &Scenario,
    final_state: &EconomicState,
    history: &[HistoryRecord],
    invariants: &[Invariant],
) -> Vec<InvariantEvaluationRecord> {
    let mut records = Vec::new();
    for inv in invariants {
        let outcome = match inv.scope {
            InvariantScope::State => evaluate_invariant(
                inv,
                EvaluationTarget::State {
                    world: &scenario.world,
                    state: final_state,
                },
            ),
            InvariantScope::History => evaluate_invariant(
                inv,
                EvaluationTarget::History {
                    world: &scenario.world,
                    records: history,
                },
            ),
            InvariantScope::Transition => {
                records.push(InvariantEvaluationRecord::NotExecuted {
                    point: EvaluationPoint::OnSimulationCompletion,
                    invariant_id: inv.id.as_str().to_owned(),
                    reason: "ON_SIMULATION_COMPLETION does not provide a single Transition target"
                        .into(),
                });
                continue;
            }
        };
        records.push(InvariantEvaluationRecord::Executed {
            point: EvaluationPoint::OnSimulationCompletion,
            invariant_id: inv.id.as_str().to_owned(),
            outcome,
        });
    }
    records
}

fn stop_after_economic(
    conditions: &[StopCondition],
    step_index: usize,
    disposition: EconomicDisposition,
) -> Option<String> {
    for c in conditions {
        match c {
            StopCondition::AfterCompletedStep(i) if *i == step_index => {
                return Some(format!("stop after completed step {step_index}"));
            }
            StopCondition::OnEconomicDisposition(d) if *d == disposition => {
                return Some(format!("stop on disposition {disposition:?}"));
            }
            _ => {}
        }
    }
    None
}

fn stop_after_m02(
    conditions: &[StopCondition],
    point: EvaluationPoint,
    records: &[InvariantEvaluationRecord],
) -> Option<String> {
    for c in conditions {
        let StopCondition::OnM02Kind {
            point: want_point,
            kind: want_kind,
        } = c
        else {
            continue;
        };
        if *want_point != point {
            continue;
        }
        for rec in records {
            if let InvariantEvaluationRecord::Executed {
                point: p,
                outcome,
                invariant_id,
                ..
            } = rec
            {
                if *p == point && outcome.kind == *want_kind {
                    return Some(format!(
                        "stop on M02 {want_kind:?} at {point:?} ({invariant_id})"
                    ));
                }
            }
        }
    }
    None
}

fn flatten_invariant_records(result: &mut SimulationResult) {
    let mut flat = Vec::new();
    for step in &result.step_records {
        flat.extend(step.before_action.iter().cloned());
        flat.extend(step.after_action.iter().cloned());
    }
    result.invariant_evaluation_records = flat;
}
