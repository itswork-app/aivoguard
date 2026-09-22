//! M07 error taxonomy (frozen Gate-6).

/// Machine-readable M07 error identity (closed; reserved classes unused on Gate-6 path).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentSettlementErrorId {
    /// Case malformed (incl. negative expected amounts on `>= 0` payloads).
    InvalidPaymentCase,
    /// Settlement declaration inconsistent (incl. terminal_partial static conflicts).
    InvalidSettlementCase,
    /// Observation structure / matrix / amount / status↔phase invalid.
    InvalidSettlementObservation,
    /// Case binding ≠ observation binding.
    ObservationBindingMismatch,
    /// Applicable expectation requires an absent Option field.
    MissingObservationField,
    /// Reserved — MUST NOT be emitted on Gate-6 single-snapshot path.
    AmbiguousObservation,
    /// Observation type at M07 boundary is not a SettlementObservation.
    IncompatibleObservationType,
    /// Declared amount semantics structurally invalid.
    InvalidAmount,
    /// Checked arithmetic could not complete.
    NumericComparisonError,
    /// Settlement status outside closed taxonomy.
    UnsupportedSettlementState,
    /// Adapter rejected / ambiguous map / source-asset mismatch.
    ExternalAdapterError,
    /// Reserved — unused on Gate-6 closed taxonomy.
    EconomicStateMismatch,
    /// M06 composition ERROR or binding identity failure.
    M06CompositionError,
    /// Optional engine pins failed exact compare.
    EnginePinMismatch,
    /// Internal M07 fault.
    EngineError,
}

/// Structured M07 evaluation error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentSettlementError {
    /// Error class.
    pub error_id: PaymentSettlementErrorId,
    /// Human diagnostic (non-authority).
    pub reason: String,
}

impl PaymentSettlementError {
    /// Construct an error with diagnostic reason.
    #[must_use]
    pub fn new(error_id: PaymentSettlementErrorId, reason: impl Into<String>) -> Self {
        Self {
            error_id,
            reason: reason.into(),
        }
    }
}
