//! Authoritative evaluation record for a dispositioned Action.

use crate::kernel::account::ActorId;
use crate::kernel::effect::StateEffect;
use crate::kernel::error::EconomicDisposition;

/// Authoritative evaluation record.
///
/// Includes the acting participant so downstream read-only evaluators (M02)
/// can observe `transaction.actor` without reconstructing Action intent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    /// Acting participant that submitted the evaluated Action.
    pub actor: ActorId,
    /// Economic disposition.
    pub disposition: EconomicDisposition,
    /// Declared state effects (empty for non-success or zero-effect success).
    pub effects: Vec<StateEffect>,
}
