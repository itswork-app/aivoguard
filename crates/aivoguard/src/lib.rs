//! `AivoGuard` — deterministic economic testing infrastructure.
//!
//! * Gate-1 **M01 Economic Kernel** — [`kernel`]
//! * Gate-2 **M02 Economic Invariant Engine** — [`invariant`] (read-only)
//! * Gate-3 **M03 Deterministic Simulator** — [`simulator`] (orchestration)

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod invariant;
pub mod kernel;
pub mod simulator;

pub use invariant::{
    evaluate_invariant, resolve_history_index, AggOp, Applicability, ApplicabilityStatus, CmpOp,
    Count, DomainExpr, EvaluationLocation, EvaluationTarget, HistoryRecord, Invariant,
    InvariantError, InvariantErrorClass, InvariantEvidence, InvariantId, InvariantOutcome,
    InvariantResultKind, InvariantScope, PropertyExpr, RelationKind, StructuredFact,
    TransitionRecord, ValueExpr, Violation, ViolationPolicy, M02_ENGINE_VERSION,
};
pub use kernel::{
    divide_with_rounding, evaluate, Account, AccountId, Action, ActorId, ArithError, Asset,
    AssetId, BalanceFacetModel, ConversionRule, EconomicDisposition, EconomicEvent, EconomicState,
    EconomicWorld, Evidence, ExecutionContext, FacetId, FeeRule, KernelError, KernelErrorKind,
    KernelOutcome, Money, Price, PriceCategory, PriceId, RoundingMode, StateEffect, Transaction,
    TransferRule, ENGINE_VERSION,
};
pub use simulator::{
    economic_history_view, run_simulation, AttemptClassification, EvaluationPoint, ExecutionStatus,
    FatalCause, InvariantEvaluationPlan, InvariantEvaluationRecord, Scenario, ScenarioId,
    SimulationError, SimulationErrorClass, SimulationResult, StepResult, StopCondition, StopPolicy,
    M03_ENGINE_VERSION,
};
