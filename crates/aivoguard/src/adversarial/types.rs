//! Shared M04 domain types (provisional API; AD-15 remains OPEN).

use crate::adversarial::token::IdentifierToken;

/// Engine version string (identity-stability input; not AD-03 algorithm).
pub const ENGINE_VERSION: &str = "aivoguard-m04-0.0.0";

/// Applicability policy (§8.6). Default: `AllowNonApplicable`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ApplicabilityPolicy {
    /// Preconditions fail → normal `NON_APPLICABLE`.
    #[default]
    AllowNonApplicable,
    /// Preconditions fail → `ERROR` / `NON_APPLICABLE_TRANSFORMATION`.
    RequireApplicable,
}

/// Generation status (§18.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationStatus {
    /// All declared candidates evaluated within limits.
    Complete,
    /// Stopped at truncation boundary (`TRUNCATE_AT_N`).
    Truncated,
    /// Limit breach under `FAIL_ON_EXCEED`, or generation error.
    Failed,
}

/// Truncation policy (§18.2). Default: `FailOnExceed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TruncationPolicy {
    /// Next unit beyond N → `GENERATION_ERROR` / `FAILED`.
    #[default]
    FailOnExceed,
    /// Stop before N+1; status `TRUNCATED`; no generation error solely for boundary.
    TruncateAtN,
}

/// Top-level transformation application outcome (§8.3 / §8.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationOutcomeKind {
    /// Candidate produced (then VALID or INVALID).
    Derived,
    /// Preconditions failed under `ALLOW_NON_APPLICABLE`.
    NonApplicable,
    /// Definition/config/parameter/composition/engine failure.
    Error,
}

/// M04 closed structural validation classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationClassification {
    /// Closed checklist passed.
    Valid,
    /// Candidate produced; first failing checklist reason recorded.
    Invalid,
}

/// Structural invalid reason codes (§8.5.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructuralInvalidReason {
    /// Check 1 / 4a.
    InvalidScenarioStructure,
    /// Check 2.
    InvalidRequiredField,
    /// Check 3.
    InvalidProjection,
    /// Check 4b.
    InvalidInitialStateStructure,
    /// Check 5.
    InvalidActionStructure,
    /// Check 6.
    InvalidActionReference,
    /// Check 7.
    InvalidDomainReference,
    /// Check 8.
    InvalidExecutionConfiguration,
    /// Check 9.
    InvalidInvariantPlanStructure,
    /// Check 10.
    InvalidStopPolicyStructure,
    /// Check 11.
    ForbiddenHiddenInput,
    /// Check 12.
    InvalidProvenance,
}

/// Action addressing mode (§10.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    /// Resolve by exact ActionId.
    ActionId,
    /// Resolve by zero-based index into current intermediate sequence.
    Index,
}

/// Position constraint grammar (§10.4) — sole Gate-4 form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionConstraint {
    /// Ordered pair `(left_index, right_index)`; no implicit reverse.
    ExactPair {
        /// Left plan index (zero-based).
        left_index: usize,
        /// Right plan index (zero-based).
        right_index: usize,
    },
}

/// One declared incompatibility rule (§10.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncompatibilityRule {
    /// Rule identity.
    pub rule_id: IdentifierToken,
    /// Left transformation identity.
    pub left_transformation_identity: IdentifierToken,
    /// Right transformation identity.
    pub right_transformation_identity: IdentifierToken,
    /// Optional left version (omit = wildcard).
    pub left_version: Option<IdentifierToken>,
    /// Optional right version (omit = wildcard).
    pub right_version: Option<IdentifierToken>,
    /// Optional exact pair constraint.
    pub position_constraint: Option<PositionConstraint>,
}

/// Provenance evidence for a matched incompatibility (§10.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncompatibilityEvidence {
    /// Matched rule id.
    pub rule_id: IdentifierToken,
    /// Left transformation identity from the rule.
    pub left_transformation_identity: IdentifierToken,
    /// Right transformation identity from the rule.
    pub right_transformation_identity: IdentifierToken,
    /// Left version if declared on the rule.
    pub left_version: Option<IdentifierToken>,
    /// Right version if declared on the rule.
    pub right_version: Option<IdentifierToken>,
    /// Winning left plan index.
    pub left_plan_index: usize,
    /// Winning right plan index.
    pub right_plan_index: usize,
    /// Position constraint if present on the rule.
    pub position_constraint: Option<PositionConstraint>,
}

/// One occurrence in a composition plan (concrete zero-based position).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanOccurrence {
    /// Zero-based plan index.
    pub index: usize,
    /// Transformation identity at this position.
    pub identity: IdentifierToken,
    /// Transformation version at this position.
    pub version: IdentifierToken,
}

/// Parameter value types (§17A.1.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterValue {
    /// Exact identity token.
    Identifier(IdentifierToken),
    /// Dimensionless signed integer (not Money).
    Integer(i128),
    /// Economic minor-unit amount (`i128`; no f32/f64).
    EconomicAmount(i128),
    /// Exact boolean.
    Boolean(bool),
    /// Declared enum member.
    Enum(IdentifierToken),
    /// Non-negative positional index.
    Index(usize),
    /// Declared ordering selection label.
    OrderingSelection(IdentifierToken),
}

/// Boundary operators for discrete numeric dimensions (§17A.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateOperator {
    /// Declared domain lower bound.
    Min,
    /// Declared domain upper bound.
    Max,
    /// Exact value `x`.
    Exact(i128),
    /// `x - 1` (one integer / minor-unit step).
    JustBelow(i128),
    /// `x + 1`.
    JustAbove(i128),
}

/// Parameter value type tag for a dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterType {
    /// Identifier.
    Identifier,
    /// Integer.
    Integer,
    /// Economic amount.
    EconomicAmount,
    /// Boolean.
    Boolean,
    /// Enum.
    Enum,
    /// Index.
    Index,
    /// Ordering selection.
    OrderingSelection,
}

/// One parameter dimension (§17A.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterDimension {
    /// Dimension identity (ordering key within the domain).
    pub id: IdentifierToken,
    /// Value type.
    pub value_type: ParameterType,
    /// Inclusive lower bound (INTEGER / ECONOMIC_AMOUNT / INDEX).
    pub lo: Option<i128>,
    /// Inclusive upper bound.
    pub hi: Option<i128>,
    /// Declared operators (numeric types). Empty → use explicit `explicit_values`.
    pub operators: Vec<CandidateOperator>,
    /// Explicit candidate values (IDENTIFIER / ENUM / BOOLEAN / ORDERING / fallback).
    pub explicit_values: Vec<ParameterValue>,
}

/// Ordered parameter domain.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParameterDomain {
    /// Dimensions in declaration order (lexicographic major → minor).
    pub dimensions: Vec<ParameterDimension>,
}

/// Generator limits (§18). All maxima required; missing → config error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratorLimits {
    /// Max emitted generated scenarios.
    pub maximum_generated_scenarios: u64,
    /// Max transformation entries in a plan (static).
    pub maximum_transformations_per_plan: u64,
    /// Max composition depth `k`.
    pub maximum_composition_depth: u64,
    /// Max successful action mutations.
    pub maximum_action_mutations: u64,
    /// Max parameter tuples evaluated.
    pub maximum_parameter_candidates: u64,
}

/// Generator configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratorConfig {
    /// Limits.
    pub limits: GeneratorLimits,
    /// Applicability policy.
    pub applicability_policy: ApplicabilityPolicy,
    /// Truncation policy.
    pub truncation_policy: TruncationPolicy,
    /// Ordered incompatibility rules (may be empty).
    pub incompatibility_rules: Vec<IncompatibilityRule>,
}
