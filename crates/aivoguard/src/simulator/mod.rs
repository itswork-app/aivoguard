//! M03 Deterministic Economic Simulator — sequential orchestration over M01/M02.
//!
//! Implements the frozen Gate-3 Deterministic Simulator Specification.
//! This module does **not** calculate economic truth (M01) or invariant truth (M02).

#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::too_many_lines)]

mod engine;
mod error;
mod result;
mod scenario;
mod step;

pub use engine::{economic_history_view, run_simulation, ENGINE_VERSION as M03_ENGINE_VERSION};
pub use error::{SimulationError, SimulationErrorClass};
pub use result::{ExecutionStatus, FatalCause, SimulationResult};
pub use scenario::{InvariantEvaluationPlan, Scenario, ScenarioId, StopCondition, StopPolicy};
pub use step::{AttemptClassification, EvaluationPoint, InvariantEvaluationRecord, StepResult};
