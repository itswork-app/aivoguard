//! Transaction record for a dispositioned Action evaluation.

use crate::kernel::effect::StateEffect;
use crate::kernel::error::EconomicDisposition;

/// Authoritative evaluation record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    /// Economic disposition.
    pub disposition: EconomicDisposition,
    /// Declared state effects (empty for non-success or zero-effect success).
    pub effects: Vec<StateEffect>,
}
