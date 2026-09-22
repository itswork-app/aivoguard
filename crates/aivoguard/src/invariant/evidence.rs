//! Deterministic M02 evidence (in-memory; no serialization freeze).

use crate::invariant::definition::InvariantId;
use crate::invariant::error::InvariantErrorClass;
use crate::invariant::result::{ApplicabilityStatus, InvariantResultKind};

/// Evidence sufficient to reproduce/understand an evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantEvidence {
    /// Invariant identity.
    pub invariant_id: InvariantId,
    /// Definition version.
    pub definition_version: String,
    /// Target kind label.
    pub target_kind: String,
    /// M02 engine version.
    pub engine_version: String,
    /// Result kind.
    pub result_kind: InvariantResultKind,
    /// Applicability status when recorded.
    pub applicability: Option<ApplicabilityStatus>,
    /// ERROR class when ERROR.
    pub error_class: Option<InvariantErrorClass>,
    /// ERROR reason when ERROR.
    pub error_reason: Option<String>,
    /// Deterministic notes.
    pub notes: Vec<String>,
}
