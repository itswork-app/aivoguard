//! Kernel evaluation outcomes.

use crate::kernel::error::{EconomicDisposition, KernelError};
use crate::kernel::event::EconomicEvent;
use crate::kernel::evidence::Evidence;
use crate::kernel::state::EconomicState;
use crate::kernel::transaction::Transaction;

/// Result of a single kernel evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelOutcome {
    /// Economic evaluation completed (including `Rejected` / `FailedEconomic`).
    Economic {
        /// Disposition.
        disposition: EconomicDisposition,
        /// State before evaluation.
        state_before: EconomicState,
        /// State after evaluation (`state_before` if no effects).
        state_after: EconomicState,
        /// Transaction record.
        transaction: Transaction,
        /// Economically relevant events.
        events: Vec<EconomicEvent>,
        /// Evidence.
        evidence: Evidence,
    },
    /// Non-economic failure.
    Error {
        /// Typed error.
        error: KernelError,
        /// Evidence of inputs as available.
        evidence: Evidence,
    },
}
