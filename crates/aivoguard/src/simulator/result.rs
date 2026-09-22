//! `SimulationResult` layers (not a collapsed PASS/FAIL harness verdict).

use crate::kernel::{EconomicState, KernelError};

use crate::simulator::error::SimulationError;
use crate::simulator::scenario::ScenarioId;
use crate::simulator::step::{InvariantEvaluationRecord, StepResult};

/// Why a Fatal Termination occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FatalCause {
    /// M03 simulation-level error.
    Simulation(SimulationError),
    /// M01 non-economic error (default fatal policy).
    M01(KernelError),
}

/// Simulation execution status (distinct from M01/M02 truth).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionStatus {
    /// Action sequence completed under stop policy without fatal/early stop.
    NormalCompletion,
    /// Declared stop condition triggered.
    EarlyTermination {
        /// Deterministic reason.
        reason: String,
    },
    /// Non-recoverable termination.
    FatalTermination {
        /// Fatal cause.
        cause: FatalCause,
    },
}

/// Complete Gate-3 `SimulationResult`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationResult {
    /// Execution status layer.
    pub execution_status: ExecutionStatus,
    /// Scenario identity.
    pub scenario_id: ScenarioId,
    /// Scenario version.
    pub scenario_version: String,
    /// Configuration id.
    pub configuration_id: String,
    /// Explicit initial state.
    pub initial_state: EconomicState,
    /// Final current state (last adopted M01 `state_after`, or `State_0`).
    pub final_state: EconomicState,
    /// Ordered step records (M03 Step History).
    pub step_records: Vec<StepResult>,
    /// Flattened per-step invariant evaluation records (BEFORE/AFTER).
    pub invariant_evaluation_records: Vec<InvariantEvaluationRecord>,
    /// Completion-phase M02 records (empty after Fatal).
    pub completion_evaluation_records: Vec<InvariantEvaluationRecord>,
    /// Simulation-level errors accumulated for the run.
    pub simulation_errors: Vec<SimulationError>,
    /// Count of Actions actually invoked through M01.
    pub executed_action_count: u64,
    /// M01 engine version pin.
    pub m01_engine_version: String,
    /// M02 engine version pin.
    pub m02_engine_version: String,
    /// M03 engine version pin.
    pub m03_engine_version: String,
}

impl SimulationResult {
    /// Whether execution completed normally.
    #[must_use]
    pub const fn is_normal(&self) -> bool {
        matches!(self.execution_status, ExecutionStatus::NormalCompletion)
    }

    /// Whether execution terminated early by declared stop policy.
    #[must_use]
    pub const fn is_early(&self) -> bool {
        matches!(
            self.execution_status,
            ExecutionStatus::EarlyTermination { .. }
        )
    }

    /// Whether execution terminated fatally.
    #[must_use]
    pub const fn is_fatal(&self) -> bool {
        matches!(
            self.execution_status,
            ExecutionStatus::FatalTermination { .. }
        )
    }
}
