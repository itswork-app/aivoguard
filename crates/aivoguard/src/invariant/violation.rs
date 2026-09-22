//! First-class deterministic Violation records.

use crate::invariant::definition::{InvariantId, InvariantScope};

/// Structured fact (authoritative; prose is non-authoritative).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredFact {
    /// Fact key.
    pub key: String,
    /// Fact value.
    pub value: String,
}

/// Deterministic evaluation location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluationLocation {
    /// State-scoped location.
    State {
        /// Optional account.
        account: Option<String>,
        /// Optional asset.
        asset: Option<String>,
        /// Optional facet.
        facet: Option<String>,
    },
    /// Transition-scoped location.
    Transition {
        /// Disposition label if known.
        disposition: Option<String>,
    },
    /// History logical position.
    HistoryLogical {
        /// Authoritative logical position.
        logical_position: i64,
    },
    /// History ordinal within directed traversal.
    HistoryOrdinal {
        /// 0-based traversal ordinal.
        ordinal: usize,
        /// Direction label.
        direction: String,
    },
    /// Generic invariant-level location.
    Invariant,
}

/// Deterministic Violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    /// Invariant identity.
    pub invariant_id: InvariantId,
    /// Definition version.
    pub definition_version: String,
    /// Evaluation scope.
    pub scope: InvariantScope,
    /// Location.
    pub location: EvaluationLocation,
    /// Condition required by the invariant.
    pub required_condition: Vec<StructuredFact>,
    /// Authoritative observation.
    pub observed_condition: Vec<StructuredFact>,
    /// Authoritative reference facts.
    pub authoritative_reference: Vec<StructuredFact>,
    /// Additional deterministic context.
    pub context: Vec<StructuredFact>,
}
