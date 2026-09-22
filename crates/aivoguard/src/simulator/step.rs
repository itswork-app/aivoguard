//! Per-step simulation records and M02 scheduling evidence.

use crate::invariant::InvariantOutcome;
use crate::kernel::{Action, EconomicState, ExecutionContext, KernelOutcome};

/// Attempt classification for an accounted Action position (frozen).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttemptClassification {
    /// Action was not passed to M01.
    NotAttempted,
    /// M01 returned `KernelOutcome::Economic`.
    AttemptedM01EconomicOutcome,
    /// M01 returned `KernelOutcome::Error`.
    AttemptedM01Error,
}

/// M02 evaluation point names (frozen).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvaluationPoint {
    /// Before M01, observing `State_i`.
    BeforeAction,
    /// After M01 Economic outcome and state adoption.
    AfterAction,
    /// After Normal/Early completion only.
    OnSimulationCompletion,
}

/// Record of an M02 scheduling decision / outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvariantEvaluationRecord {
    /// M02 was invoked and produced an outcome.
    Executed {
        /// Point.
        point: EvaluationPoint,
        /// Invariant identity string (from definition).
        invariant_id: String,
        /// Authoritative M02 outcome.
        outcome: InvariantOutcome,
    },
    /// Declared evaluation was not invoked (M03 scheduling layer).
    NotExecuted {
        /// Point.
        point: EvaluationPoint,
        /// Invariant identity string when known.
        invariant_id: String,
        /// Explicit reason.
        reason: String,
    },
}

/// Authoritative record for one simulation step position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepResult {
    /// Simulation-owned step index.
    pub step_index: usize,
    /// Action at this position.
    pub action: Action,
    /// Attempt classification.
    pub classification: AttemptClassification,
    /// State before M01 when the step reached M01 (or `BEFORE_ACTION`).
    pub state_before: Option<EconomicState>,
    /// Authoritative state after when M01 Economic outcome exists.
    pub state_after: Option<EconomicState>,
    /// Complete M01 outcome when M01 was invoked.
    pub kernel_outcome: Option<KernelOutcome>,
    /// Deterministic `ExecutionContext` when M01 was invoked.
    pub execution_context: Option<ExecutionContext>,
    /// `BEFORE_ACTION` records for this step.
    pub before_action: Vec<InvariantEvaluationRecord>,
    /// `AFTER_ACTION` records for this step.
    pub after_action: Vec<InvariantEvaluationRecord>,
}
