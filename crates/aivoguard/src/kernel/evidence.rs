//! Kernel evaluation evidence (economically authoritative facts).

use crate::kernel::effect::StateEffect;
use crate::kernel::error::EconomicDisposition;
use crate::kernel::execution::ExecutionContext;

/// Evidence produced by a kernel evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    /// Short action summary.
    pub action_summary: String,
    /// World configuration identity (facet model + flags).
    pub world_summary: String,
    /// Execution context snapshot.
    pub execution_context: ExecutionContext,
    /// Disposition when evaluation produced an economic outcome.
    pub disposition: Option<EconomicDisposition>,
    /// Declared effects.
    pub effects: Vec<StateEffect>,
    /// Outcome reason.
    pub reason: String,
}
