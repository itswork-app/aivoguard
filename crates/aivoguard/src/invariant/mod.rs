//! M02 Economic Invariant Engine — read-only deterministic evaluation.
//!
//! Implements the frozen Gate-2 Economic Invariant Engine Specification.
//! This module does **not** execute Actions, mutate economic state, or
//! recalculate M01 economics.

#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::use_self)]
#![allow(clippy::branches_sharing_code)]

mod definition;
mod error;
mod evaluate;
mod evidence;
mod history;
mod result;
mod target;
mod violation;

pub use definition::{
    AggOp, Applicability, CmpOp, DomainExpr, Invariant, InvariantId, InvariantScope, PropertyExpr,
    RelationKind, ValueExpr, ViolationPolicy,
};
pub use error::{InvariantError, InvariantErrorClass};
pub use evaluate::{
    evaluate_invariant, resolve_history_index, ENGINE_VERSION as M02_ENGINE_VERSION,
};
pub use evidence::InvariantEvidence;
pub use history::{HistoryRecord, TransitionRecord};
pub use result::{ApplicabilityStatus, InvariantOutcome, InvariantResultKind};
pub use target::EvaluationTarget;
pub use violation::{EvaluationLocation, StructuredFact, Violation};
