//! M03 simulation-level error taxonomy (distinct from M01/M02).

/// M03 simulation-level error class (frozen Gate-3 set).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationErrorClass {
    /// Scenario structure/content is not a valid Simulation input.
    InvalidScenario,
    /// Required Scenario input absent.
    MissingSimulationInput,
    /// Declared configuration is inconsistent/illegal.
    InvalidSimulationConfiguration,
    /// Next Action would exceed `maximum_action_steps`.
    ExecutionLimitExceeded,
    /// M03 cannot correctly continue (orchestration fault).
    SimulationEngineError,
}

/// Typed M03 simulation error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationError {
    /// Error classification.
    pub class: SimulationErrorClass,
    /// Deterministic reason (not host tracing).
    pub reason: String,
}

impl SimulationError {
    /// Construct an error.
    #[must_use]
    pub fn new(class: SimulationErrorClass, reason: impl Into<String>) -> Self {
        Self {
            class,
            reason: reason.into(),
        }
    }
}
