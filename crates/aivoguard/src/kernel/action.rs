//! Economically consequential intent (Action ≠ success).

use crate::kernel::account::{AccountId, ActorId};
use crate::kernel::asset::AssetId;
use crate::kernel::world::PriceId;

/// Optional Action identifier (idempotency is World-defined, not universal).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActionId(String);

impl ActionId {
    /// Create an action identifier.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Borrow the identity string (`IdentifierToken` inspection; non-economic).
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Explicit Action intent submitted for evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// World-defined successful no-op (may yield `AcceptedEffective` + 0 effects).
    NoOp {
        /// Acting participant.
        actor: ActorId,
        /// Optional identifier.
        action_id: Option<ActionId>,
    },
    /// Debit one account facet and credit another for the same asset.
    Transfer {
        /// Acting participant.
        actor: ActorId,
        /// Optional identifier.
        action_id: Option<ActionId>,
        /// Source account.
        from: AccountId,
        /// Destination account.
        to: AccountId,
        /// Asset to move.
        asset: AssetId,
        /// Minor-unit amount (must be > 0).
        amount: i128,
    },
    /// Transfer plus an explicit fee effect.
    TransferWithFee {
        /// Acting participant.
        actor: ActorId,
        /// Optional identifier.
        action_id: Option<ActionId>,
        /// Source account.
        from: AccountId,
        /// Destination account.
        to: AccountId,
        /// Transfer asset.
        asset: AssetId,
        /// Transfer minor-unit amount (must be > 0).
        amount: i128,
        /// Fee payer account.
        fee_payer: AccountId,
        /// Fee recipient account.
        fee_recipient: AccountId,
        /// Fee asset.
        fee_asset: AssetId,
        /// Fee minor-unit amount (must be >= 0).
        fee_amount: i128,
    },
    /// Convert quantity from one asset to another using an authoritative price.
    Convert {
        /// Acting participant.
        actor: ActorId,
        /// Optional identifier.
        action_id: Option<ActionId>,
        /// Account holding both assets' facets.
        account: AccountId,
        /// Source asset.
        from_asset: AssetId,
        /// Destination asset.
        to_asset: AssetId,
        /// Source minor-unit amount (must be > 0).
        from_amount: i128,
        /// Authoritative price identity declared by the World.
        price_id: PriceId,
    },
}

impl Action {
    /// Acting participant.
    #[must_use]
    pub fn actor(&self) -> &ActorId {
        match self {
            Self::NoOp { actor, .. }
            | Self::Transfer { actor, .. }
            | Self::TransferWithFee { actor, .. }
            | Self::Convert { actor, .. } => actor,
        }
    }

    /// Optional action identifier.
    #[must_use]
    pub fn action_id(&self) -> Option<&ActionId> {
        match self {
            Self::NoOp { action_id, .. }
            | Self::Transfer { action_id, .. }
            | Self::TransferWithFee { action_id, .. }
            | Self::Convert { action_id, .. } => action_id.as_ref(),
        }
    }
}
