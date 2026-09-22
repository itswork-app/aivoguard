//! World rules under which Actions are evaluated.

use std::collections::{BTreeMap, BTreeSet};

use crate::kernel::account::{Account, AccountId, ActorId};
use crate::kernel::asset::{Asset, AssetId};
use crate::kernel::balance::{BalanceFacetModel, FacetId};
use crate::kernel::rounding::RoundingMode;

/// Price identity within a World.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PriceId(String);

impl PriceId {
    /// Create a price identity.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// Price category (EK-PRICE-01).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriceCategory {
    /// Quoted price.
    Quoted,
    /// Execution price.
    Execution,
    /// Reference price.
    Reference,
    /// Observed external price captured as declared input.
    ObservedExternal,
}

/// Authoritative price: quote minor units per `quote_per_base` base minor units.
///
/// Direction is explicit: this is **base→quote** (`base/quote` naming in docs is
/// the pair; numeric meaning is `quote_amount = from_amount * quote_per_base /
/// base_units` with declared rounding).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Price {
    /// Price identity.
    pub id: PriceId,
    /// Base asset.
    pub base: AssetId,
    /// Quote asset.
    pub quote: AssetId,
    /// Category.
    pub category: PriceCategory,
    /// Quote minor units credited per `base_units` base minor units debited.
    pub quote_per_base: i128,
    /// Base minor-unit divisor for the ratio (must be > 0).
    pub base_units: i128,
    /// Rounding when conversion is not exact.
    pub rounding: RoundingMode,
}

/// Transfer facet pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferRule {
    /// Facet debited on the source account.
    pub debit_facet: FacetId,
    /// Facet credited on the destination account.
    pub credit_facet: FacetId,
}

/// Fee rule applied when an Action requires fees.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeeRule {
    /// Facet debited on the fee payer.
    pub debit_facet: FacetId,
    /// Facet credited on the fee recipient.
    pub credit_facet: FacetId,
}

/// Conversion uses a named authoritative price.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversionRule {
    /// Required price category.
    pub required_category: PriceCategory,
    /// Facet debited for the source asset.
    pub debit_facet: FacetId,
    /// Facet credited for the destination asset.
    pub credit_facet: FacetId,
}

/// Complete rule environment for kernel evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EconomicWorld {
    /// Declared assets.
    pub assets: BTreeMap<AssetId, Asset>,
    /// Declared accounts.
    pub accounts: BTreeMap<AccountId, Account>,
    /// Actors authorized to spend from their owned accounts.
    pub actors: BTreeSet<ActorId>,
    /// Declared facets.
    pub facets: BTreeSet<FacetId>,
    /// Facet architecture.
    pub facet_model: BalanceFacetModel,
    /// Whether negative balances are economically permitted.
    pub allow_negative_balances: bool,
    /// Whether operations are atomic (Gate-1 Worlds use `true`).
    pub atomic: bool,
    /// Whether [`crate::kernel::action::Action::NoOp`] is permitted.
    pub allow_noop: bool,
    /// Transfer rule when transfers are enabled.
    pub transfer: Option<TransferRule>,
    /// Fee rule when fee-bearing actions are enabled.
    pub fee: Option<FeeRule>,
    /// Conversion rule when conversions are enabled.
    pub conversion: Option<ConversionRule>,
    /// Declared prices.
    pub prices: BTreeMap<PriceId, Price>,
}

impl EconomicWorld {
    /// Empty world builder starting point.
    #[must_use]
    pub fn new(facet_model: BalanceFacetModel) -> Self {
        Self {
            assets: BTreeMap::new(),
            accounts: BTreeMap::new(),
            actors: BTreeSet::new(),
            facets: BTreeSet::new(),
            facet_model,
            allow_negative_balances: false,
            atomic: true,
            allow_noop: true,
            transfer: None,
            fee: None,
            conversion: None,
            prices: BTreeMap::new(),
        }
    }

    /// Insert an asset definition.
    pub fn insert_asset(&mut self, asset: Asset) {
        self.assets.insert(asset.id.clone(), asset);
    }

    /// Insert an account definition.
    pub fn insert_account(&mut self, account: Account) {
        self.actors.insert(account.owner.clone());
        self.accounts.insert(account.id.clone(), account);
    }

    /// Declare a facet.
    pub fn insert_facet(&mut self, facet: FacetId) {
        self.facets.insert(facet);
    }

    /// Insert a price.
    pub fn insert_price(&mut self, price: Price) {
        self.prices.insert(price.id.clone(), price);
    }
}
