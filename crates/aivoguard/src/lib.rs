//! `AivoGuard` — deterministic economic testing infrastructure.
//!
//! Gate-1 **M01 Economic Kernel** is implemented under [`kernel`].
//! Later modules (M02+) are intentionally absent.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod kernel;

pub use kernel::{
    divide_with_rounding, evaluate, Account, AccountId, Action, ActorId, ArithError, Asset,
    AssetId, BalanceFacetModel, ConversionRule, EconomicDisposition, EconomicEvent, EconomicState,
    EconomicWorld, Evidence, ExecutionContext, FacetId, FeeRule, KernelError, KernelErrorKind,
    KernelOutcome, Money, Price, PriceCategory, PriceId, RoundingMode, StateEffect, Transaction,
    TransferRule, ENGINE_VERSION,
};
