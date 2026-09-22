//! Authoritative evaluation targets (read-only views).

use crate::kernel::state::EconomicState;
use crate::kernel::transaction::Transaction;
use crate::kernel::world::EconomicWorld;

use crate::invariant::history::HistoryRecord;

/// Read-only evaluation target.
#[derive(Debug, Clone, Copy)]
pub enum EvaluationTarget<'a> {
    /// State target.
    State {
        /// World.
        world: &'a EconomicWorld,
        /// Authoritative state.
        state: &'a EconomicState,
    },
    /// Transition target.
    Transition {
        /// World.
        world: &'a EconomicWorld,
        /// State before.
        state_before: &'a EconomicState,
        /// State after.
        state_after: &'a EconomicState,
        /// Transaction / disposition / effects.
        transaction: &'a Transaction,
    },
    /// History target (authoritative ordered records).
    History {
        /// World.
        world: &'a EconomicWorld,
        /// Ordered history records.
        records: &'a [HistoryRecord],
    },
}

impl<'a> EvaluationTarget<'a> {
    /// Borrow World.
    #[must_use]
    pub const fn world(self) -> &'a EconomicWorld {
        match self {
            Self::State { world, .. }
            | Self::Transition { world, .. }
            | Self::History { world, .. } => world,
        }
    }

    /// Target kind label for evidence.
    #[must_use]
    pub const fn kind_label(self) -> &'static str {
        match self {
            Self::State { .. } => "State",
            Self::Transition { .. } => "Transition",
            Self::History { .. } => "History",
        }
    }
}
