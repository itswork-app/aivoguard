//! M08 adapter-local error taxonomy (inbound-only).
//!
//! Adapter errors are **not** economic / M01 / M07 failures.

/// Machine-readable M08 adapter error identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M08AdapterErrorId {
    /// Structural / parse failure of the presented observation.
    MalformedExternalInput,
    /// Kind/version outside the declared configuration subset.
    UnsupportedProtocolFeature,
    /// Field / amount / asset rule failure, including overflow.
    InvalidMapping,
    /// Invalid or incomplete mapping configuration.
    ConfigurationFailure,
    /// PATH-C only — not emitted by the inbound-only adapt path while M08-OD-03 is OPEN.
    ExternalVerificationFailure,
    /// PATH-E only — out of scope for inbound-only; not emitted by adapt.
    TransportFailure,
}

/// Structured M08 adapter error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M08AdapterError {
    /// Error class.
    pub error_id: M08AdapterErrorId,
    /// Human diagnostic (non-authority).
    pub reason: String,
}

impl M08AdapterError {
    /// Construct an error with diagnostic reason.
    #[must_use]
    pub fn new(error_id: M08AdapterErrorId, reason: impl Into<String>) -> Self {
        Self {
            error_id,
            reason: reason.into(),
        }
    }
}
