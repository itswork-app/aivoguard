//! Authoritative history / transition records for M02 (consumed, not reconstructed).

use crate::kernel::error::EconomicDisposition;
use crate::kernel::state::EconomicState;
use crate::kernel::transaction::Transaction;

/// Authoritative transition snapshot for M02.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionRecord {
    /// State before.
    pub state_before: EconomicState,
    /// State after.
    pub state_after: EconomicState,
    /// Transaction (disposition + effects).
    pub transaction: Transaction,
}

impl TransitionRecord {
    /// Disposition.
    #[must_use]
    pub fn disposition(&self) -> EconomicDisposition {
        self.transaction.disposition
    }
}

/// One authoritative history record with logical order key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryRecord {
    /// Authoritative logical position (total order key).
    pub logical_position: i64,
    /// Optional transition payload.
    pub transition: Option<TransitionRecord>,
}
