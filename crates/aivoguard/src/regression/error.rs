//! M06 error taxonomy (frozen Gate-5).

use crate::regression::types::StructuredValue;

/// Machine-readable M06 error identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegressionErrorId {
    /// Case malformed.
    InvalidRegressionCase,
    /// Expectation malformed (including duplicate ids).
    InvalidExpectation,
    /// Expectation class not in Gate-5 closed taxonomy.
    UnsupportedExpectation,
    /// ScenarioBinding ≠ observation identity.
    ObservationBindingMismatch,
    /// Required observation absent / zero matches / out of range.
    MissingObservationField,
    /// Multiple records match one lookup key.
    AmbiguousObservation,
    /// Sole match is NotExecuted.
    NotExecutedObservation,
    /// Observation shape unusable.
    IncompatibleObservationType,
    /// Optional engine pins failed exact compare.
    EnginePinMismatch,
    /// Checked numeric ops failed.
    NumericComparisonError,
    /// Internal M06 fault.
    EngineError,
}

/// Structured M06 evaluation error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegressionError {
    /// Error class.
    pub error_id: RegressionErrorId,
    /// Human diagnostic (non-authority).
    pub reason: String,
    /// Optional structured evidence fields.
    pub evidence: Vec<(String, StructuredValue)>,
}

impl RegressionError {
    /// Construct an error with diagnostic reason.
    #[must_use]
    pub fn new(error_id: RegressionErrorId, reason: impl Into<String>) -> Self {
        Self {
            error_id,
            reason: reason.into(),
            evidence: Vec::new(),
        }
    }

    /// Attach one structured evidence field.
    #[must_use]
    pub fn with_field(mut self, key: impl Into<String>, value: StructuredValue) -> Self {
        self.evidence.push((key.into(), value));
        self
    }
}
