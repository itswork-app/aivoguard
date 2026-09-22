//! M04 error taxonomy (Gate-4 minimum; §15.1).

use crate::adversarial::types::{IncompatibilityEvidence, LimitBreachEvidence};

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
    /// Deterministic reason (human diagnostic; not machine authority for limits).
    pub reason: String,
    /// Optional incompatibility evidence when class is `IncompatibleTransform`.
    pub incompatibility: Option<IncompatibilityEvidence>,
    /// Optional structured limit-breach evidence (F-04).
    pub limit_breach: Option<LimitBreachEvidence>,
}

impl AdversarialError {
    /// Construct an error without extra evidence.
    #[must_use]
    pub fn new(class: AdversarialErrorClass, reason: impl Into<String>) -> Self {
        Self {
            class,
            reason: reason.into(),
            incompatibility: None,
            limit_breach: None,
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
            limit_breach: None,
        }
    }

    /// Construct a generation limit-breach error with structured evidence.
    #[must_use]
    pub fn limit_breach(evidence: LimitBreachEvidence) -> Self {
        let reason = format!(
            "{:?} exceeded (observed={}, limit={})",
            evidence.counter, evidence.observed, evidence.limit
        );
        Self {
            class: AdversarialErrorClass::GenerationError,
            reason,
            incompatibility: None,
            limit_breach: Some(evidence),
        }
    }
}
