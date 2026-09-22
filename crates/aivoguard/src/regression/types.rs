//! M06 domain types (provisional; AD-15 remains OPEN).

use crate::invariant::InvariantResultKind;
use crate::kernel::EconomicDisposition;
use crate::simulator::{ExecutionStatus, FatalCause, SimulationResult};

/// Declared M06 engine version label (identity-stability input; not AD-03).
pub const ENGINE_VERSION: &str = "m06-0.1.0";

/// Scenario binding — declarative only; M06 does not execute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioBinding {
    /// Scenario id.
    pub scenario_id: String,
    /// Scenario version.
    pub scenario_version: String,
    /// Configuration id.
    pub configuration_id: String,
}

/// Optional engine version pins.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EnginePins {
    /// Expected M01 engine version when set.
    pub m01_engine_version: Option<String>,
    /// Expected M02 engine version when set.
    pub m02_engine_version: Option<String>,
    /// Expected M03 engine version when set.
    pub m03_engine_version: Option<String>,
}

/// Authoritative observation package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegressionObservation {
    /// M03 simulation result (required).
    pub simulation_result: SimulationResult,
}

/// Closed Gate-5 expectation class payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpectationClass {
    /// Exact final balance cell.
    FinalBalanceExact {
        /// Account id token.
        account_id: String,
        /// Asset id token.
        asset_id: String,
        /// Facet id token.
        facet_id: String,
        /// Expected minor-unit amount.
        amount: i128,
    },
    /// Absolute-tolerance final balance cell.
    FinalBalanceAbsoluteTolerance {
        /// Account id token.
        account_id: String,
        /// Asset id token.
        asset_id: String,
        /// Facet id token.
        facet_id: String,
        /// Nominal expected amount.
        amount: i128,
        /// Non-negative absolute tolerance in minor units.
        tolerance: u128,
    },
    /// Exact execution status.
    ExecutionStatusExact {
        /// Expected status shape.
        expected: ExpectedExecutionStatus,
    },
    /// Exact executed action count.
    ExecutedActionCountExact {
        /// Expected count.
        count: u64,
    },
    /// Exact M01 disposition at a positional step.
    StepDispositionExact {
        /// 0-based positional index into `step_records`.
        step_index: u64,
        /// Expected disposition (Accepted maps to AcceptedEffective).
        disposition: ExpectedDisposition,
    },
    /// Exact M02 kind at step/phase/id.
    InvariantKindExact {
        /// Invariant id.
        invariant_id: String,
        /// Evaluation phase.
        phase: ExpectationPhase,
        /// Step index (nested step_records attribution).
        step_index: u64,
        /// Expected M02 kind.
        expected_kind: InvariantResultKind,
    },
    /// Exact completion-phase M02 kind.
    CompletionInvariantKindExact {
        /// Invariant id.
        invariant_id: String,
        /// Expected M02 kind.
        expected_kind: InvariantResultKind,
    },
}

/// Phase selector for per-step invariant expectations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpectationPhase {
    /// BEFORE_ACTION.
    BeforeAction,
    /// AFTER_ACTION.
    AfterAction,
}

/// Expected economic disposition names from the frozen M06 contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpectedDisposition {
    /// Maps to [`EconomicDisposition::AcceptedEffective`].
    Accepted,
    /// Rejected.
    Rejected,
    /// FailedEconomic.
    FailedEconomic,
}

impl ExpectedDisposition {
    /// Compare against an observed M01 disposition.
    #[must_use]
    pub const fn matches(self, observed: EconomicDisposition) -> bool {
        matches!(
            (self, observed),
            (Self::Accepted, EconomicDisposition::AcceptedEffective)
                | (Self::Rejected, EconomicDisposition::Rejected)
                | (Self::FailedEconomic, EconomicDisposition::FailedEconomic)
        )
    }
}

/// Expected execution status for Gate-5.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpectedExecutionStatus {
    /// Normal completion.
    NormalCompletion,
    /// Early termination with exact reason string.
    EarlyTermination {
        /// Exact reason.
        reason_exact: String,
    },
    /// Fatal with cause class only.
    FatalTermination {
        /// Simulation vs M01 cause class.
        cause_class: ExpectedFatalCauseClass,
    },
}

/// Fatal cause class for Gate-5 expectations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpectedFatalCauseClass {
    /// M03 simulation error.
    Simulation,
    /// M01 kernel error.
    M01,
}

impl ExpectedExecutionStatus {
    /// Whether observed status satisfies this expectation.
    #[must_use]
    pub fn matches(&self, observed: &ExecutionStatus) -> bool {
        match (self, observed) {
            (Self::NormalCompletion, ExecutionStatus::NormalCompletion) => true,
            (
                Self::EarlyTermination { reason_exact },
                ExecutionStatus::EarlyTermination { reason },
            ) => reason_exact == reason,
            (
                Self::FatalTermination { cause_class },
                ExecutionStatus::FatalTermination { cause },
            ) => matches!(
                (cause_class, cause),
                (
                    ExpectedFatalCauseClass::Simulation,
                    FatalCause::Simulation(_)
                ) | (ExpectedFatalCauseClass::M01, FatalCause::M01(_))
            ),
            _ => false,
        }
    }
}

/// One declared expectation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expectation {
    /// Unique id within the case.
    pub expectation_id: String,
    /// Closed class payload.
    pub class: ExpectationClass,
    /// When false, skipped (no lookup / mismatch).
    pub applicable: bool,
}

/// Full regression case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegressionCase {
    /// Declared case id.
    pub case_id: String,
    /// Declared case version.
    pub case_version: String,
    /// Scenario binding.
    pub scenario_binding: ScenarioBinding,
    /// Ordered expectations (may be empty).
    pub expectations: Vec<Expectation>,
    /// Authoritative observation.
    pub observation: RegressionObservation,
    /// Optional engine pins.
    pub engine_pins: Option<EnginePins>,
    /// Non-authoritative notes.
    pub notes: Option<String>,
}

/// Comparison operator recorded in mismatch evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOperator {
    /// Exact equality.
    Exact,
    /// Absolute tolerance.
    AbsoluteTolerance,
}

/// Mismatch class label.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MismatchClass {
    /// Numeric / value inequality.
    ValueInequality,
    /// Kind inequality.
    KindInequality,
    /// Status inequality.
    StatusInequality,
    /// Count inequality.
    CountInequality,
    /// Disposition inequality.
    DispositionInequality,
}

/// Structured value for evidence payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructuredValue {
    /// Empty / none.
    None,
    /// UTF-8 string.
    String(String),
    /// Signed integer.
    I128(i128),
    /// Unsigned 128.
    U128(u128),
    /// Unsigned 64.
    U64(u64),
    /// Boolean.
    Bool(bool),
    /// Invariant result kind name.
    InvariantKind(InvariantResultKind),
    /// Expected disposition.
    Disposition(ExpectedDisposition),
    /// Expected execution status (debug-style label).
    ExecutionStatusLabel(String),
}

/// Structured mismatch evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MismatchEvidence {
    /// Expectation id.
    pub expectation_id: String,
    /// Class name label.
    pub class: String,
    /// Target descriptor.
    pub target: StructuredValue,
    /// Expected value.
    pub expected: StructuredValue,
    /// Observed value.
    pub observed: StructuredValue,
    /// Operator.
    pub operator: ComparisonOperator,
    /// Mismatch class.
    pub mismatch_class: MismatchClass,
    /// 0-based ordinal among mismatches in declaration order.
    pub ordinal: u64,
}

/// Authoritative regression verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegressionVerdict {
    /// All applicable expectations satisfied.
    Match,
    /// At least one applicable expectation unsatisfied; completed.
    Mismatch,
    /// Comparison could not produce MATCH/MISMATCH.
    Error,
}

/// Semantic provenance (AD-14 deferred).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegressionProvenance {
    /// Scenario id from binding.
    pub scenario_id: String,
    /// Scenario version from binding.
    pub scenario_version: String,
    /// Configuration id from binding.
    pub configuration_id: String,
    /// Observation engine pins copied from SimulationResult.
    pub observation_engine_pins: EnginePins,
    /// M06 engine version label.
    pub m06_spec_version_label: String,
    /// Evaluation policy label.
    pub evaluation_policy: String,
}

/// Complete regression evaluation result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegressionResult {
    /// Verdict.
    pub verdict: RegressionVerdict,
    /// Case id.
    pub case_id: String,
    /// Case version.
    pub case_version: String,
    /// Mismatches (authoritative only when verdict is Mismatch).
    pub mismatches: Vec<MismatchEvidence>,
    /// Error (present iff verdict is Error).
    pub error: Option<crate::regression::error::RegressionError>,
    /// Applicable expectations evaluated (before abort).
    pub expectations_evaluated: u64,
    /// Expectations with applicable=false.
    pub expectations_skipped: u64,
    /// Provenance.
    pub provenance: RegressionProvenance,
}
