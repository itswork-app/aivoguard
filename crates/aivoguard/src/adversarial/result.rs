//! M04 generation / application result model (§28).

use crate::adversarial::error::AdversarialError;
use crate::adversarial::provenance::ScenarioProvenance;
use crate::adversarial::types::{GenerationStatus, ValidationClassification};
use crate::simulator::Scenario;

/// One emitted generated adversarial scenario.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedAdversarialScenario {
    /// Derived M03-compatible scenario payload.
    pub scenario: Scenario,
    /// Structural validation classification.
    pub validation: ValidationClassification,
    /// Provenance.
    pub provenance: ScenarioProvenance,
}

/// Non-applicable record (normal under AllowNonApplicable).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonApplicableRecord {
    /// Plan / composition position.
    pub composition_index: usize,
    /// Transformation identity.
    pub transformation_identity: String,
    /// Reason.
    pub reason: String,
}

/// Full generation operation result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerationResult {
    /// Generation status.
    pub status: GenerationStatus,
    /// Emitted candidates in deterministic order.
    pub emitted: Vec<GeneratedAdversarialScenario>,
    /// Non-applicable records (ordered).
    pub non_applicable: Vec<NonApplicableRecord>,
    /// Primary error when status is Failed (or definition/config errors).
    pub error: Option<AdversarialError>,
    /// Counters at completion.
    pub counters: GenerationCounters,
}

/// Observable generation counters (§18).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GenerationCounters {
    /// Emitted generated scenario count.
    pub emitted_scenarios: u64,
    /// Transformation applications attempted.
    pub transformation_applications: u64,
    /// Parameter tuples evaluated.
    pub parameter_candidates_evaluated: u64,
    /// Successful action mutations.
    pub action_mutations: u64,
}
