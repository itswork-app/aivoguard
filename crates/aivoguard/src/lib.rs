//! `AivoGuard` — deterministic economic testing infrastructure.
//!
//! * Gate-1 **M01 Economic Kernel** — [`kernel`]
//! * Gate-2 **M02 Economic Invariant Engine** — [`invariant`] (read-only)
//! * Gate-3 **M03 Deterministic Simulator** — [`simulator`] (orchestration)
//! * Gate-4 **M04 Adversarial Scenario Engine** — [`adversarial`] (scenario generation)
//! * Gate-5 **M06 Economic Regression Engine** — [`regression`] (expectation comparison)
//! * Gate-6 **M07 Payment & Settlement Testing** — [`payment_settlement`] (types foundation)

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod adversarial;
pub mod invariant;
pub mod kernel;
pub mod payment_settlement;
pub mod regression;
pub mod simulator;

pub use adversarial::{
    action_id_equals, apply_mutation, apply_transformation, check_plan_incompatibility,
    definition_with_kind_projections, enumerate_parameter_candidates, find_first_incompatibility,
    inherit_all_projections, plan_occurrences, projections_for_kind, resolve_action_id,
    resolve_action_index, run_generation, take_parameter_candidates, validate_checklist,
    validate_definition_projections, validate_projection_table, validate_scenario, ActionTarget,
    AddressingMode, AdversarialError, AdversarialErrorClass, ApplicabilityPolicy,
    ApplicationOutcomeKind, CandidateOperator, ChecklistView, CompositionPlan, DuplicateIdMode,
    FieldProjection, GeneratedAdversarialScenario, GenerationCounters, GenerationRequest,
    GenerationResult, GenerationStatus, GeneratorConfig, GeneratorLimits, IdentifierToken,
    IncompatibilityEvidence, IncompatibilityRule, LimitBreachEvidence, LimitCounterId,
    NonApplicableRecord, ParameterCandidateIter, ParameterDimension, ParameterDomain,
    ParameterTuple, ParameterType, ParameterValue, PlanOccurrence, PlanParamProductIter,
    PositionConstraint, ProjectionMode, ResolutionClass, ScenarioField, ScenarioProvenance,
    StructuralInvalidReason, StructuralValidationResult, TargetResolution, TransformApplication,
    TransformationDefinition, TransformationKind, TruncationPolicy, ValidationClassification,
    M04_ENGINE_VERSION,
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
pub use payment_settlement::{
    adapt_settlement_observation, compose_m06, evaluate_payment_settlement, is_valid_status_phase,
    m06_verdict_label, validate_case_observation_binding, validate_case_scenario_binding,
    validate_engine_pins, validate_expectation_set, validate_payment_settlement_case,
    validate_requested_amount_case_binding, validate_scenario_binding,
    validate_settlement_observation, validate_status_phase, AdapterProvenance,
    ComparisonOperator as PaymentComparisonOperator, EscrowStatus,
    ExternalPaymentSettlementObservation, FeeDeclaration, FeeMode, FeeTiming, M06Binding,
    M06ComposeOutcome, M07EnginePins, PaymentDeclaration, PaymentMismatchEvidence,
    PaymentScenarioBinding, PaymentSettlementBinding, PaymentSettlementCase,
    PaymentSettlementError, PaymentSettlementErrorId, PaymentSettlementProvenance,
    PaymentSettlementResult, PaymentSettlementVerdict, RefundOutcome, SemanticSettlementAdapter,
    SettlementDeclaration, SettlementExecutionState, SettlementExpectation,
    SettlementExpectationClass, SettlementObservation, SettlementObservationAdapter,
    SettlementPhase, SettlementStatus,
};
pub use regression::{
    compare_absolute_tolerance, evaluate_regression, ComparisonOperator, EnginePins, Expectation,
    ExpectationClass, ExpectationPhase, ExpectedDisposition, ExpectedExecutionStatus,
    ExpectedFatalCauseClass, MismatchClass, MismatchEvidence, RegressionCase, RegressionError,
    RegressionErrorId, RegressionObservation, RegressionProvenance, RegressionResult,
    RegressionVerdict, ScenarioBinding, StructuredValue, ToleranceCompare, M06_ENGINE_VERSION,
};
pub use simulator::{
    economic_history_view, run_simulation, AttemptClassification, EvaluationPoint, ExecutionStatus,
    FatalCause, InvariantEvaluationPlan, InvariantEvaluationRecord, Scenario, ScenarioId,
    SimulationError, SimulationErrorClass, SimulationResult, StepResult, StopCondition, StopPolicy,
    M03_ENGINE_VERSION,
};
