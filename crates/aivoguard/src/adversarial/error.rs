//! M04 error taxonomy (Gate-4 minimum; §15.1).

use crate::adversarial::types::IncompatibilityEvidence;

/// M04 error class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdversarialErrorClass {
    /// Transformation/plan definition malformed.
    InvalidAdversarialDefinition,
    /// Generator/limits/config illegal.
    InvalidAdversarialConfiguration,
    /// Base Scenario missing/invalid for M04.
    InvalidBaseScenario,
    /// Required applicability failed (`REQUIRE_APPLICABLE`).
    NonApplicableTransform,
    /// Declared semantic incompatibility rule matched (§10.4).
    IncompatibleTransform,
    /// Parameter out of domain, or ActionId ambiguous (`AMBIGUOUS_ACTION_ID`).
    InvalidTransformParameter,
    /// Structural composition-plan failure (not declared incompatibility).
    CompositionError,
    /// Enumeration/generation cannot complete under declared rules.
    GenerationError,
    /// Internal M04 fault.
    EngineError,
}

/// Typed M04 error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdversarialError {
    /// Error classification.
    pub class: AdversarialErrorClass,
    /// Deterministic reason (not host tracing).
    pub reason: String,
    /// Optional incompatibility evidence when class is `IncompatibleTransform`.
    pub incompatibility: Option<IncompatibilityEvidence>,
}

impl AdversarialError {
    /// Construct an error without incompatibility evidence.
    #[must_use]
    pub fn new(class: AdversarialErrorClass, reason: impl Into<String>) -> Self {
        Self {
            class,
            reason: reason.into(),
            incompatibility: None,
        }
    }

    /// Construct an incompatibility error with required evidence.
    #[must_use]
    pub fn incompatible(evidence: IncompatibilityEvidence) -> Self {
        let reason = format!(
            "INCOMPATIBLE_TRANSFORMATION rule_id={} left@{} right@{}",
            evidence.rule_id.as_str(),
            evidence.left_plan_index,
            evidence.right_plan_index
        );
        Self {
            class: AdversarialErrorClass::IncompatibleTransform,
            reason,
            incompatibility: Some(evidence),
        }
    }
}
