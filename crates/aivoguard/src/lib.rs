//! `AivoGuard` — deterministic economic testing infrastructure.
//!
//! * Gate-1 **M01 Economic Kernel** — [`kernel`]
//! * Gate-2 **M02 Economic Invariant Engine** — [`invariant`] (read-only)
//! * Gate-3 **M03 Deterministic Simulator** — [`simulator`] (orchestration)
//! * Gate-4 **M04 Adversarial Scenario Engine** — [`adversarial`] (scenario generation)

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod adversarial;
pub mod invariant;
pub mod kernel;
pub mod simulator;

pub use adversarial::{
    action_id_equals, apply_mutation, apply_transformation, check_plan_incompatibility,
    enumerate_parameter_candidates, find_first_incompatibility, plan_occurrences,
    resolve_action_id, resolve_action_index, run_generation, validate_checklist, validate_scenario,
    ActionTarget, AddressingMode, AdversarialError, AdversarialErrorClass, ApplicabilityPolicy,
    ApplicationOutcomeKind, CandidateOperator, ChecklistView, CompositionPlan, DuplicateIdMode,
    GeneratedAdversarialScenario, GenerationCounters, GenerationRequest, GenerationResult,
    GenerationStatus, GeneratorConfig, GeneratorLimits, IdentifierToken, IncompatibilityEvidence,
    IncompatibilityRule, NonApplicableRecord, ParameterDimension, ParameterDomain, ParameterTuple,
    ParameterType, ParameterValue, PlanOccurrence, PositionConstraint, ResolutionClass,
    ScenarioProvenance, StructuralInvalidReason, StructuralValidationResult, TargetResolution,
    TransformApplication, TransformationDefinition, TransformationKind, TruncationPolicy,
    ValidationClassification, M04_ENGINE_VERSION,
};
pub use invariant::{
    evaluate_invariant, resolve_history_index, AggOp, Applicability, ApplicabilityStatus, CmpOp,
    Count, DomainExpr, EvaluationLocation, EvaluationTarget, HistoryRecord, Invariant,
    InvariantError, InvariantErrorClass, InvariantEvidence, InvariantId, InvariantOutcome,
    InvariantResultKind, InvariantScope, PropertyExpr, RelationKind, StructuredFact,
    TransitionRecord, ValueExpr, Violation, ViolationPolicy, M02_ENGINE_VERSION,
};
pub use kernel::{
    divide_with_rounding, evaluate, Account, AccountId, Action, ActionId, ActorId, ArithError,
    Asset, AssetId, BalanceFacetModel, ConversionRule, EconomicDisposition, EconomicEvent,
    EconomicState, EconomicWorld, Evidence, ExecutionContext, FacetId, FeeRule, KernelError,
    KernelErrorKind, KernelOutcome, Money, Price, PriceCategory, PriceId, RoundingMode,
    StateEffect, Transaction, TransferRule, ENGINE_VERSION,
};
pub use simulator::{
    economic_history_view, run_simulation, AttemptClassification, EvaluationPoint, ExecutionStatus,
    FatalCause, InvariantEvaluationPlan, InvariantEvaluationRecord, Scenario, ScenarioId,
    SimulationError, SimulationErrorClass, SimulationResult, StepResult, StopCondition, StopPolicy,
    M03_ENGINE_VERSION,
};
