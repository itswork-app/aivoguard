//! Provenance records for M04 generation artifacts (§20).

use crate::adversarial::resolve::TargetResolution;
use crate::adversarial::token::IdentifierToken;
use crate::adversarial::types::{
    ApplicabilityPolicy, GenerationStatus, StructuralInvalidReason, TruncationPolicy,
    ValidationClassification,
};

/// Provenance attached to an emitted generated adversarial scenario.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioProvenance {
    /// Base scenario id.
    pub base_scenario_id: String,
    /// Base scenario version.
    pub base_scenario_version: String,
    /// Ordered transformation identities applied (composition chain).
    pub transformation_chain: Vec<(IdentifierToken, IdentifierToken)>,
    /// Composition positions (0-based) for each applied transform.
    pub composition_positions: Vec<usize>,
    /// Target resolutions recorded during application.
    pub target_resolutions: Vec<TargetResolution>,
    /// Applicability policy in force.
    pub applicability_policy: ApplicabilityPolicy,
    /// Truncation policy in force.
    pub truncation_policy: TruncationPolicy,
    /// Generation status of the enclosing generation operation (at emission time).
    pub generation_status_at_emit: GenerationStatus,
    /// Validation classification.
    pub validation: ValidationClassification,
    /// Structural invalid reason when INVALID.
    pub structural_invalid_reason: Option<StructuralInvalidReason>,
    /// M04 engine version.
    pub engine_version: String,
    /// Generator configuration id / label (declared).
    pub generator_config_id: String,
    /// Emitted scenario counter index (0-based among emitted).
    pub emitted_index: u64,
    /// Parameter candidate index evaluated for this emission (if any).
    pub parameter_candidate_index: Option<u64>,
    /// Structured limit-breach evidence when emission was under a failed generation (optional).
    pub limit_breach: Option<crate::adversarial::types::LimitBreachEvidence>,
}
