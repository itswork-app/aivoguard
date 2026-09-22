//! M02 result model: PASS / FAIL / ERROR only.

use crate::invariant::error::InvariantError;
use crate::invariant::evidence::InvariantEvidence;
use crate::invariant::violation::Violation;

/// Applicability status recorded with PASS results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicabilityStatus {
    /// Property was evaluated under an applicable World.
    Applicable,
    /// Valid invariant that does not apply to the declared World.
    NotApplicable,
}

/// Top-level result kind (no fourth category).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvariantResultKind {
    /// Evaluation completed; no property violation (includes `NotApplicable`).
    Pass,
    /// Evaluation completed; property does not hold.
    Fail,
    /// Property could not be evaluated correctly.
    Error,
}

/// Complete M02 evaluation outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantOutcome {
    /// PASS / FAIL / ERROR.
    pub kind: InvariantResultKind,
    /// Applicability when kind is PASS (always set for PASS).
    pub applicability: Option<ApplicabilityStatus>,
    /// Retained violations (FAIL, or diagnostics under ERROR).
    pub violations: Vec<Violation>,
    /// Error details when kind is ERROR.
    pub error: Option<InvariantError>,
    /// Deterministic evidence.
    pub evidence: InvariantEvidence,
}
