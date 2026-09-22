//! M04 generation engine — composition, limits, structural validation.

use crate::adversarial::error::{AdversarialError, AdversarialErrorClass};
use crate::adversarial::incompatibility::{find_first_incompatibility, plan_occurrences};
use crate::adversarial::parameters::{enumerate_parameter_candidates, ParameterTuple};
use crate::adversarial::provenance::ScenarioProvenance;
use crate::adversarial::result::{
    GeneratedAdversarialScenario, GenerationCounters, GenerationResult, NonApplicableRecord,
};
use crate::adversarial::token::IdentifierToken;
use crate::adversarial::transform::{
    apply_transformation, TransformApplication, TransformationDefinition,
};
use crate::adversarial::types::{
    ApplicabilityPolicy, GenerationStatus, GeneratorConfig, TruncationPolicy, ENGINE_VERSION,
};
use crate::adversarial::validation::validate_scenario;
use crate::simulator::Scenario;

/// Composition plan: ordered transformation definitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositionPlan {
    /// Ordered transformations (`T_0 … T_(k-1)`).
    pub transformations: Vec<TransformationDefinition>,
}

/// Generation request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerationRequest {
    /// Base M03 scenario.
    pub base: Scenario,
    /// Composition / mutation plan.
    pub plan: CompositionPlan,
    /// Generator configuration.
    pub config: GeneratorConfig,
    /// Declared generator configuration label (provenance).
    pub generator_config_id: String,
}

/// Run adversarial generation under the frozen M04 contract.
#[must_use]
pub fn run_generation(request: &GenerationRequest) -> GenerationResult {
    match run_generation_inner(request) {
        Ok(result) => result,
        Err(error) => GenerationResult {
            status: GenerationStatus::Failed,
            emitted: Vec::new(),
            non_applicable: Vec::new(),
            error: Some(error),
            counters: GenerationCounters::default(),
        },
    }
}

fn run_generation_inner(request: &GenerationRequest) -> Result<GenerationResult, AdversarialError> {
    // Phase 1 — transformation definition validation (plan order)
    for (i, t) in request.plan.transformations.iter().enumerate() {
        if !IdentifierToken::is_valid(t.identity.as_str())
            || !IdentifierToken::is_valid(t.version.as_str())
        {
            return Err(AdversarialError::new(
                AdversarialErrorClass::InvalidAdversarialDefinition,
                format!("plan[{i}]: transformation identity/version invalid"),
            ));
        }
    }

    // Phase 2 — generator configuration validation
    validate_config(request)?;

    let plan_len = request.plan.transformations.len() as u64;
    if plan_len > request.config.limits.maximum_transformations_per_plan {
        return Err(AdversarialError::new(
            AdversarialErrorClass::InvalidAdversarialConfiguration,
            format!(
                "plan length {plan_len} exceeds maximum_transformations_per_plan {}",
                request.config.limits.maximum_transformations_per_plan
            ),
        ));
    }
    if plan_len > request.config.limits.maximum_composition_depth {
        return Err(AdversarialError::new(
            AdversarialErrorClass::InvalidAdversarialConfiguration,
            format!(
                "composition depth {plan_len} exceeds maximum_composition_depth {}",
                request.config.limits.maximum_composition_depth
            ),
        ));
    }

    // Phase 3 — base scenario validation
    validate_base(&request.base)?;

    // Phase 6 — incompatibility
    check_incompatibility_for_plan(&request.plan, &request.config)?;

    // Phase 7 — candidate generation
    Ok(generate_candidates(request)?)
}

fn validate_config(request: &GenerationRequest) -> Result<(), AdversarialError> {
    if !IdentifierToken::is_valid(&request.generator_config_id) {
        return Err(AdversarialError::new(
            AdversarialErrorClass::InvalidAdversarialConfiguration,
            "generator_config_id is empty",
        ));
    }
    Ok(())
}

fn validate_base(base: &Scenario) -> Result<(), AdversarialError> {
    if !IdentifierToken::is_valid(base.id.as_str()) {
        return Err(AdversarialError::new(
            AdversarialErrorClass::InvalidBaseScenario,
            "base scenario id is empty",
        ));
    }
    if !IdentifierToken::is_valid(&base.version) {
        return Err(AdversarialError::new(
            AdversarialErrorClass::InvalidBaseScenario,
            "base scenario version is empty",
        ));
    }
    if !IdentifierToken::is_valid(&base.configuration_id) {
        return Err(AdversarialError::new(
            AdversarialErrorClass::InvalidBaseScenario,
            "base configuration_id is empty",
        ));
    }
    Ok(())
}

fn check_incompatibility_for_plan(
    plan: &CompositionPlan,
    config: &GeneratorConfig,
) -> Result<(), AdversarialError> {
    if config.incompatibility_rules.is_empty() {
        return Ok(());
    }
    let identities: Vec<_> = plan
        .transformations
        .iter()
        .map(|t| t.identity.clone())
        .collect();
    let versions: Vec<_> = plan
        .transformations
        .iter()
        .map(|t| t.version.clone())
        .collect();
    let occurrences = plan_occurrences(&identities, &versions)
        .map_err(|r| AdversarialError::new(AdversarialErrorClass::CompositionError, r))?;
    if let Some(evidence) = find_first_incompatibility(&occurrences, &config.incompatibility_rules)
    {
        return Err(AdversarialError::incompatible(evidence));
    }
    Ok(())
}

enum LimitDecision {
    Ok,
    Fail(AdversarialError),
    Truncate,
}

fn try_increment(
    counter: &mut u64,
    limit: u64,
    policy: TruncationPolicy,
    name: &str,
) -> LimitDecision {
    let next = counter.saturating_add(1);
    if next > limit {
        return match policy {
            TruncationPolicy::FailOnExceed => LimitDecision::Fail(AdversarialError::new(
                AdversarialErrorClass::GenerationError,
                format!("{name} exceeded (limit={limit})"),
            )),
            TruncationPolicy::TruncateAtN => LimitDecision::Truncate,
        };
    }
    *counter = next;
    LimitDecision::Ok
}

fn generate_candidates(request: &GenerationRequest) -> Result<GenerationResult, AdversarialError> {
    let mut emitted = Vec::new();
    let mut non_applicable = Vec::new();
    let mut counters = GenerationCounters::default();
    let mut status = GenerationStatus::Complete;

    if request.plan.transformations.is_empty() {
        match try_emit(
            request,
            request.base.clone(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
            &mut emitted,
            &mut counters,
        ) {
            EmitOutcome::Emitted => {}
            EmitOutcome::Fail(err) => {
                return Ok(GenerationResult {
                    status: GenerationStatus::Failed,
                    emitted,
                    non_applicable,
                    error: Some(err),
                    counters,
                });
            }
            EmitOutcome::Truncate => {
                status = GenerationStatus::Truncated;
            }
        }
        return Ok(GenerationResult {
            status,
            emitted,
            non_applicable,
            error: None,
            counters,
        });
    }

    let per_transform_params: Vec<Vec<ParameterTuple>> = request
        .plan
        .transformations
        .iter()
        .map(|t| enumerate_parameter_candidates(&t.parameter_domain))
        .collect::<Result<Vec<_>, _>>()?;

    let param_products = cartesian_param_plans(&per_transform_params);

    for product in param_products {
        match try_increment(
            &mut counters.parameter_candidates_evaluated,
            request.config.limits.maximum_parameter_candidates,
            request.config.truncation_policy,
            "maximum_parameter_candidates",
        ) {
            LimitDecision::Ok => {}
            LimitDecision::Fail(err) => {
                return Ok(GenerationResult {
                    status: GenerationStatus::Failed,
                    emitted,
                    non_applicable,
                    error: Some(err),
                    counters,
                });
            }
            LimitDecision::Truncate => {
                status = GenerationStatus::Truncated;
                break;
            }
        }
        let param_eval_index = counters.parameter_candidates_evaluated.saturating_sub(1);

        let mut snapshot = request.base.clone();
        let mut chain = Vec::new();
        let mut positions = Vec::new();
        let mut resolutions = Vec::new();
        let mut skip_emit = false;

        for (ci, def) in request.plan.transformations.iter().enumerate() {
            counters.transformation_applications =
                counters.transformation_applications.saturating_add(1);
            let params = product.get(ci).cloned().unwrap_or_else(|| ParameterTuple {
                bindings: Vec::new(),
            });
            match apply_transformation(
                &snapshot,
                def,
                &params,
                request.config.applicability_policy,
                ci,
            ) {
                TransformApplication::Derived {
                    scenario,
                    action_mutations,
                    resolutions: res,
                } => {
                    if action_mutations > 0 {
                        match try_increment(
                            &mut counters.action_mutations,
                            request.config.limits.maximum_action_mutations,
                            request.config.truncation_policy,
                            "maximum_action_mutations",
                        ) {
                            LimitDecision::Ok => {}
                            LimitDecision::Fail(err) => {
                                return Ok(GenerationResult {
                                    status: GenerationStatus::Failed,
                                    emitted,
                                    non_applicable,
                                    error: Some(err),
                                    counters,
                                });
                            }
                            LimitDecision::Truncate => {
                                status = GenerationStatus::Truncated;
                                skip_emit = true;
                                break;
                            }
                        }
                        // try_increment already added 1; if action_mutations > 1, add remainder
                        if action_mutations > 1 {
                            counters.action_mutations = counters
                                .action_mutations
                                .saturating_add(action_mutations - 1);
                            if counters.action_mutations
                                > request.config.limits.maximum_action_mutations
                            {
                                match request.config.truncation_policy {
                                    TruncationPolicy::FailOnExceed => {
                                        return Ok(GenerationResult {
                                            status: GenerationStatus::Failed,
                                            emitted,
                                            non_applicable,
                                            error: Some(AdversarialError::new(
                                                AdversarialErrorClass::GenerationError,
                                                "maximum_action_mutations exceeded",
                                            )),
                                            counters,
                                        });
                                    }
                                    TruncationPolicy::TruncateAtN => {
                                        status = GenerationStatus::Truncated;
                                        skip_emit = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    chain.push((def.identity.clone(), def.version.clone()));
                    positions.push(ci);
                    resolutions.extend(res);
                    snapshot = scenario;
                }
                TransformApplication::NonApplicable { reason, .. } => {
                    non_applicable.push(NonApplicableRecord {
                        composition_index: ci,
                        transformation_identity: def.identity.as_str().to_owned(),
                        reason,
                    });
                    skip_emit = true;
                    break;
                }
                TransformApplication::Error(err) => {
                    return Ok(GenerationResult {
                        status: GenerationStatus::Failed,
                        emitted,
                        non_applicable,
                        error: Some(err),
                        counters,
                    });
                }
            }
        }

        if skip_emit {
            if status == GenerationStatus::Truncated {
                break;
            }
            continue;
        }

        match try_emit(
            request,
            snapshot,
            chain,
            positions,
            resolutions,
            Some(param_eval_index),
            &mut emitted,
            &mut counters,
        ) {
            EmitOutcome::Emitted => {}
            EmitOutcome::Fail(err) => {
                return Ok(GenerationResult {
                    status: GenerationStatus::Failed,
                    emitted,
                    non_applicable,
                    error: Some(err),
                    counters,
                });
            }
            EmitOutcome::Truncate => {
                status = GenerationStatus::Truncated;
                break;
            }
        }
    }

    Ok(GenerationResult {
        status,
        emitted,
        non_applicable,
        error: None,
        counters,
    })
}

enum EmitOutcome {
    Emitted,
    Fail(AdversarialError),
    Truncate,
}

fn try_emit(
    request: &GenerationRequest,
    scenario: Scenario,
    chain: Vec<(IdentifierToken, IdentifierToken)>,
    positions: Vec<usize>,
    resolutions: Vec<crate::adversarial::resolve::TargetResolution>,
    parameter_candidate_index: Option<u64>,
    emitted: &mut Vec<GeneratedAdversarialScenario>,
    counters: &mut GenerationCounters,
) -> EmitOutcome {
    match try_increment(
        &mut counters.emitted_scenarios,
        request.config.limits.maximum_generated_scenarios,
        request.config.truncation_policy,
        "maximum_generated_scenarios",
    ) {
        LimitDecision::Ok => {}
        LimitDecision::Fail(err) => return EmitOutcome::Fail(err),
        LimitDecision::Truncate => return EmitOutcome::Truncate,
    }

    let emitted_index = counters.emitted_scenarios.saturating_sub(1);
    let structural = validate_scenario(&scenario, true);
    let validation = structural.classification;
    let provenance = ScenarioProvenance {
        base_scenario_id: request.base.id.as_str().to_owned(),
        base_scenario_version: request.base.version.clone(),
        transformation_chain: chain,
        composition_positions: positions,
        target_resolutions: resolutions,
        applicability_policy: request.config.applicability_policy,
        truncation_policy: request.config.truncation_policy,
        generation_status_at_emit: GenerationStatus::Complete,
        validation,
        structural_invalid_reason: structural.primary_reason,
        engine_version: ENGINE_VERSION.to_owned(),
        generator_config_id: request.generator_config_id.clone(),
        emitted_index,
        parameter_candidate_index,
    };

    emitted.push(GeneratedAdversarialScenario {
        scenario,
        validation,
        provenance,
    });
    EmitOutcome::Emitted
}

fn cartesian_param_plans(per_transform: &[Vec<ParameterTuple>]) -> Vec<Vec<ParameterTuple>> {
    if per_transform.is_empty() {
        return vec![Vec::new()];
    }
    let mut result: Vec<Vec<ParameterTuple>> = vec![Vec::new()];
    for options in per_transform {
        let mut next = Vec::new();
        for prefix in &result {
            for opt in options {
                let mut row = prefix.clone();
                row.push(opt.clone());
                next.push(row);
            }
        }
        result = next;
    }
    result
}

/// Check incompatibility rules against a plan without running generation.
#[must_use]
pub fn check_plan_incompatibility(
    plan: &CompositionPlan,
    rules: &[crate::adversarial::types::IncompatibilityRule],
) -> Result<(), AdversarialError> {
    let identities: Vec<_> = plan
        .transformations
        .iter()
        .map(|t| t.identity.clone())
        .collect();
    let versions: Vec<_> = plan
        .transformations
        .iter()
        .map(|t| t.version.clone())
        .collect();
    let occurrences = plan_occurrences(&identities, &versions)
        .map_err(|r| AdversarialError::new(AdversarialErrorClass::CompositionError, r))?;
    if let Some(evidence) = find_first_incompatibility(&occurrences, rules) {
        return Err(AdversarialError::incompatible(evidence));
    }
    Ok(())
}

/// Apply a single mutation (one transformation) to a base scenario.
#[must_use]
pub fn apply_mutation(
    base: &Scenario,
    def: &TransformationDefinition,
    params: &ParameterTuple,
    policy: ApplicabilityPolicy,
) -> TransformApplication {
    apply_transformation(base, def, params, policy, 0)
}
