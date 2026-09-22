//! Economic Kernel (M01) — deterministic evaluation of economically consequential actions.
//!
//! Implements the frozen Gate-1 Economic Kernel Specification. This module is
//! **not** an invariant engine (M02), simulator (M03), or adapter layer.

#![allow(clippy::missing_const_for_fn)]

pub(crate) mod account;
pub(crate) mod action;
pub(crate) mod asset;
pub(crate) mod balance;
pub(crate) mod effect;
pub(crate) mod engine;
pub(crate) mod error;
pub(crate) mod event;
pub(crate) mod evidence;
pub(crate) mod execution;
pub(crate) mod money;
pub(crate) mod outcome;
pub(crate) mod rounding;
pub(crate) mod state;
pub(crate) mod transaction;
pub(crate) mod world;

pub use account::{Account, AccountId, ActorId};
pub use action::{Action, ActionId};
pub use asset::{Asset, AssetId};
pub use balance::{BalanceFacetModel, FacetId};
pub use effect::StateEffect;
pub use engine::{evaluate, ENGINE_VERSION};
pub use error::{ArithError, EconomicDisposition, KernelError, KernelErrorKind};
pub use event::EconomicEvent;
pub use evidence::Evidence;
pub use execution::ExecutionContext;
pub use money::Money;
pub use outcome::KernelOutcome;
pub use rounding::{divide_with_rounding, RoundingMode};
pub use state::EconomicState;
pub use transaction::Transaction;
pub use world::{
    ConversionRule, EconomicWorld, FeeRule, Price, PriceCategory, PriceId, TransferRule,
};
