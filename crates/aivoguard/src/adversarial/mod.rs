//! M04 Adversarial Scenario Engine — deterministic scenario generation/transformation.
//!
//! Implements the frozen Gate-4 Adversarial Scenario Engine Specification.
//! This module does **not** calculate economic truth (M01), evaluate invariants
//! (M02), or own simulation sequencing (M03).
//!
//! Open decisions AD-03 / AD-14 / AD-15 remain OPEN. Concrete Rust types here are
//! provisional implementation surfaces, not a freeze of AD-15.

#![allow(clippy::module_name_repetitions)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::result_large_err)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::single_match_else)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::assigning_clones)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::similar_names)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::double_must_use)]

mod engine;
mod error;
mod incompatibility;
mod parameters;
mod projection;
mod provenance;
mod resolve;
mod result;
mod token;
mod transform;
mod types;
mod validation;

pub use engine::{
    apply_mutation, check_plan_incompatibility, run_generation, CompositionPlan, GenerationRequest,
};
pub use error::{AdversarialError, AdversarialErrorClass};
pub use incompatibility::{find_first_incompatibility, plan_occurrences};
pub use parameters::{
    enumerate_parameter_candidates, take_parameter_candidates, ParameterCandidateIter,
    ParameterTuple, PlanParamProductIter,
};
pub use projection::{
    inherit_all_projections, validate_projection_table, FieldProjection, ProjectionMode,
    ScenarioField,
};
pub use provenance::ScenarioProvenance;
pub use resolve::{
    action_id_equals, resolve_action_id, resolve_action_index, ActionTarget, ResolutionClass,
    TargetResolution,
};
pub use result::{
    GeneratedAdversarialScenario, GenerationCounters, GenerationResult, NonApplicableRecord,
};
pub use token::IdentifierToken;
pub use transform::{
    apply_transformation, definition_with_kind_projections, projections_for_kind,
    validate_definition_projections, DuplicateIdMode, TransformApplication,
    TransformationDefinition, TransformationKind,
};
pub use types::{
    AddressingMode, ApplicabilityPolicy, ApplicationOutcomeKind, CandidateOperator,
    GenerationStatus, GeneratorConfig, GeneratorLimits, IncompatibilityEvidence,
    IncompatibilityRule, LimitBreachEvidence, LimitCounterId, ParameterDimension, ParameterDomain,
    ParameterType, ParameterValue, PlanOccurrence, PositionConstraint, StructuralInvalidReason,
    TruncationPolicy, ValidationClassification, ENGINE_VERSION as M04_ENGINE_VERSION,
};
pub use validation::{
    validate_checklist, validate_scenario, ChecklistView, StructuralValidationResult,
};
