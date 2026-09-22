//! Economically relevant occurrences (not an event bus).

use crate::kernel::error::EconomicDisposition;

/// Deterministic economically relevant event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EconomicEvent {
    /// Action evaluation completed with an economic disposition.
    ActionEvaluated {
        /// Disposition.
        disposition: EconomicDisposition,
        /// Logical order from `ExecutionContext`.
        logical_order: u64,
    },
}
