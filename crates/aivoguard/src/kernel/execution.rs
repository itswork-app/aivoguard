//! Explicit deterministic execution inputs (no host clock / RNG / env).

/// Declared execution inputs for a single kernel evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionContext {
    /// Logical order key when ordering affects economic truth.
    pub logical_order: u64,
    /// Optional declared timestamp (never read from the host clock).
    pub declared_unix_secs: Option<i64>,
    /// Engine configuration label included in determinism identity.
    pub configuration_id: String,
}

impl ExecutionContext {
    /// Construct a minimal deterministic context.
    #[must_use]
    pub fn new(logical_order: u64, configuration_id: impl Into<String>) -> Self {
        Self {
            logical_order,
            declared_unix_secs: None,
            configuration_id: configuration_id.into(),
        }
    }
}
