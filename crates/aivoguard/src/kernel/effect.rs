//! Explicit state effects applied atomically under World rules.

use crate::kernel::account::AccountId;
use crate::kernel::asset::AssetId;
use crate::kernel::balance::FacetId;

/// One authoritative balance mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateEffect {
    /// Target account.
    pub account: AccountId,
    /// Target asset.
    pub asset: AssetId,
    /// Target facet.
    pub facet: FacetId,
    /// Signed minor-unit delta (checked when applied).
    pub delta: i128,
    /// Deterministic cause label for evidence.
    pub cause: String,
}
