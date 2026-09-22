//! Authoritative economic state (explicit; no hidden globals).

use std::collections::BTreeMap;

use crate::kernel::account::AccountId;
use crate::kernel::asset::AssetId;
use crate::kernel::balance::FacetId;
use crate::kernel::effect::StateEffect;
use crate::kernel::error::KernelError;
use crate::kernel::world::EconomicWorld;

/// Key for a balance cell.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BalanceKey {
    /// Account.
    pub account: AccountId,
    /// Asset.
    pub asset: AssetId,
    /// Facet.
    pub facet: FacetId,
}

/// Explicit economic state.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EconomicState {
    /// Balance cells keyed deterministically.
    balances: BTreeMap<BalanceKey, i128>,
}

impl EconomicState {
    /// Empty state.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a balance cell (test/setup helper).
    pub fn set_balance(&mut self, account: AccountId, asset: AssetId, facet: FacetId, value: i128) {
        self.balances.insert(
            BalanceKey {
                account,
                asset,
                facet,
            },
            value,
        );
    }

    /// Read a balance cell (missing = 0).
    ///
    /// M01 economic reads treat absent cells as zero. M02 must use
    /// [`Self::balance_if_present`] when absence must be distinguished from
    /// explicit zero.
    #[must_use]
    pub fn get_balance(&self, account: &AccountId, asset: &AssetId, facet: &FacetId) -> i128 {
        self.balances
            .get(&BalanceKey {
                account: account.clone(),
                asset: asset.clone(),
                facet: facet.clone(),
            })
            .copied()
            .unwrap_or(0)
    }

    /// Read a balance cell only if the cell is present in authoritative state.
    ///
    /// Returns `None` when the cell is absent (distinct from explicit zero).
    #[must_use]
    pub fn balance_if_present(
        &self,
        account: &AccountId,
        asset: &AssetId,
        facet: &FacetId,
    ) -> Option<i128> {
        self.balances
            .get(&BalanceKey {
                account: account.clone(),
                asset: asset.clone(),
                facet: facet.clone(),
            })
            .copied()
    }

    /// Whether a balance cell exists in authoritative state.
    #[must_use]
    pub fn has_balance(&self, account: &AccountId, asset: &AssetId, facet: &FacetId) -> bool {
        self.balances.contains_key(&BalanceKey {
            account: account.clone(),
            asset: asset.clone(),
            facet: facet.clone(),
        })
    }

    /// Borrow all balance entries in deterministic order.
    #[must_use]
    pub fn balances(&self) -> &BTreeMap<BalanceKey, i128> {
        &self.balances
    }

    /// Apply effects atomically to a cloned state after validation.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError`] when validation fails or arithmetic overflows.
    pub fn apply_effects(
        &self,
        world: &EconomicWorld,
        effects: &[StateEffect],
    ) -> Result<Self, KernelError> {
        let mut next = self.clone();
        for effect in effects {
            next.apply_one(world, effect)?;
        }
        next.validate_bucket_model(world)?;
        Ok(next)
    }

    fn apply_one(
        &mut self,
        world: &EconomicWorld,
        effect: &StateEffect,
    ) -> Result<(), KernelError> {
        if !world.accounts.contains_key(&effect.account) {
            return Err(KernelError::invalid_input(format!(
                "unknown account {}",
                effect.account
            )));
        }
        if !world.assets.contains_key(&effect.asset) {
            return Err(KernelError::invalid_input(format!(
                "unknown asset {}",
                effect.asset
            )));
        }
        if !world.facets.contains(&effect.facet) {
            return Err(KernelError::configuration(format!(
                "undeclared facet {}",
                effect.facet
            )));
        }

        let key = BalanceKey {
            account: effect.account.clone(),
            asset: effect.asset.clone(),
            facet: effect.facet.clone(),
        };
        let current = self.balances.get(&key).copied().unwrap_or(0);
        let updated = current
            .checked_add(effect.delta)
            .ok_or_else(|| KernelError::engine("balance update overflow"))?;
        if updated < 0 && !world.allow_negative_balances {
            return Err(KernelError::invalid_input(
                "negative balance prohibited by World rules",
            ));
        }
        // Representable negative under allow_negative still uses signed ints.
        if updated == 0 {
            self.balances.remove(&key);
        } else {
            self.balances.insert(key, updated);
        }
        Ok(())
    }

    fn validate_bucket_model(&self, world: &EconomicWorld) -> Result<(), KernelError> {
        use crate::kernel::balance::BalanceFacetModel::MutuallyExclusiveBuckets;
        if world.facet_model != MutuallyExclusiveBuckets {
            return Ok(());
        }
        // Bucket model: per (account, asset), sum of facets is the conserved
        // quantity only when the World uses buckets as components of one total.
        // Gate-1 foundational check: no facet outside the declared set (already
        // enforced) and non-negative totals when negatives are forbidden.
        for (key, value) in &self.balances {
            if *value < 0 && !world.allow_negative_balances {
                return Err(KernelError::engine(format!(
                    "bucket model invariant: negative cell {}/{}/{}",
                    key.account, key.asset, key.facet
                )));
            }
        }
        Ok(())
    }
}
