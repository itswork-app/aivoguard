//! M07 domain types (provisional; AD-15 remains OPEN).
//!
//! Phase 1–2 type foundation only — no validation or evaluation semantics.

use crate::payment_settlement::error::PaymentSettlementError;
use crate::regression::RegressionResult;
use crate::simulator::SimulationResult;

/// Opaque adapter provenance (non-authoritative; no provider clock/economic truth).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AdapterProvenance {
    /// Adapter identity token when present.
    pub adapter_id: Option<String>,
    /// Adapter version token when present.
    pub adapter_version: Option<String>,
    /// Non-authoritative free-form notes.
    pub notes: Option<String>,
}

/// Exact binding identity for case ↔ observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentSettlementBinding {
    /// Payment id.
    pub payment_id: String,
    /// Payment version.
    pub payment_version: String,
    /// Configuration id.
    pub configuration_id: String,
}

/// Optional M07 scenario binding (declarative; M07 does not execute scenarios).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentScenarioBinding {
    /// Scenario id.
    pub scenario_id: String,
    /// Scenario version.
    pub scenario_version: String,
    /// Configuration id.
    pub configuration_id: String,
}

/// Optional engine version / semantic pins (exact compare later; no discovery).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M07EnginePins {
    /// Expected M01 engine version when set.
    pub m01_engine_version: Option<String>,
    /// Expected M02 engine version when set.
    pub m02_engine_version: Option<String>,
    /// Expected M03 engine version when set.
    pub m03_engine_version: Option<String>,
    /// Opaque M07 semantic label when set (AD-15 OPEN).
    pub m07_label: Option<String>,
}

/// Consumer-side M06 composition binding (already-evaluated result only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M06Binding {
    /// Declared M06 case id.
    pub m06_case_id: String,
    /// Declared M06 case version.
    pub m06_case_version: String,
    /// Already-evaluated M06 result (M07 does not call `evaluate_regression`).
    pub regression_result: RegressionResult,
}

/// Closed Gate-6 settlement status taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlementStatus {
    /// Declared; not yet terminal.
    Pending,
    /// Authorization currently claimed.
    Authorized,
    /// Capture currently claimed.
    Captured,
    /// Full requested amount currently claimed settled.
    Settled,
    /// Strict partial settlement claim (`0 < settled < requested`).
    PartiallySettled,
    /// Settlement-lane attempt currently claimed failed.
    Failed,
    /// Application currently claimed reversed.
    Reversed,
    /// Full refund currently claimed.
    Refunded,
    /// Partial refund currently claimed.
    PartiallyRefunded,
}

/// Closed Gate-6 settlement phase taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlementPhase {
    /// Declare phase.
    Declare,
    /// Authorize phase.
    Authorize,
    /// Capture phase.
    Capture,
    /// Settle phase.
    Settle,
    /// Refund phase.
    Refund,
    /// Reverse phase.
    Reverse,
    /// Complete phase.
    Complete,
}

/// Closed settlement execution-state taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlementExecutionState {
    /// No payment/settlement operation attempt is claimed.
    NotAttempted,
    /// An attempt is claimed; successful execution is not claimed.
    Attempted,
    /// Execution of the declared operation at `phase` is claimed.
    Executed,
}

/// Closed refund-outcome taxonomy (distinct from `SettlementStatus::Failed`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefundOutcome {
    /// Refund attempt currently claimed fully successful.
    SucceededFull,
    /// Refund attempt currently claimed partially successful.
    SucceededPartial,
    /// Refund attempt currently claimed failed.
    Failed,
}

/// Closed escrow-status taxonomy (observation-only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscrowStatus {
    /// No escrow claim declared.
    NoneDeclared,
    /// Escrow currently claimed locked.
    Locked,
    /// Escrow currently claimed released.
    Released,
    /// Escrow release currently claimed failed.
    ReleaseFailed,
    /// Escrow currently claimed cancelled.
    Cancelled,
}

/// Declared fee timing category (orthogonal to [`FeeMode`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeeTiming {
    /// Fee timing with capture.
    WithCapture,
    /// Fee timing with settlement.
    WithSettlement,
    /// Declared-only timing category.
    DeclaredOnly,
}

/// Declared fee arithmetic / net-settlement interpretation (orthogonal to [`FeeTiming`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeeMode {
    /// Fee exclusive of declared gross (net derivation authorized later).
    FeeExclusive,
    /// Fee inclusive in declared gross (net derivation authorized later).
    FeeInclusive,
    /// Fee declared only; MUST NOT derive net.
    FeeDeclaredOnly,
}

/// Optional fee declaration (sole fee_mode authority when present).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeeDeclaration {
    /// Declared fee amount (minor units).
    pub fee_amount: i128,
    /// Optional fee payer token.
    pub fee_payer: Option<String>,
    /// Optional fee recipient token.
    pub fee_recipient: Option<String>,
    /// Declared fee asset token (must equal payment asset at validation time).
    pub fee_asset: String,
    /// Declared timing category.
    pub fee_timing: FeeTiming,
    /// Declared arithmetic mode.
    pub fee_mode: FeeMode,
}

/// Payment declaration (case-level).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentDeclaration {
    /// Payment id.
    pub payment_id: String,
    /// Payer account token.
    pub payer: String,
    /// Payee account token.
    pub payee: String,
    /// Sole Gate-6 economic asset authority.
    pub asset_id: String,
    /// Declared gross amount (minor units).
    pub gross_amount: i128,
    /// Optional fee declaration.
    pub fee: Option<FeeDeclaration>,
}

/// Settlement declaration (case-level).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettlementDeclaration {
    /// Declared settlement request amount (minor units).
    pub requested_amount: i128,
    /// Case-level terminality metadata for partial settlement (not observation).
    pub terminal_partial: Option<bool>,
    /// Case-level terminality metadata for partial refund (metadata only).
    pub terminal_partial_refund: Option<bool>,
}

/// Single-snapshot settlement observation (no independent asset authority).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettlementObservation {
    /// Binding identity.
    pub binding: PaymentSettlementBinding,
    /// Current settlement status claim.
    pub settlement_status: SettlementStatus,
    /// Current execution-state claim.
    pub execution_state: SettlementExecutionState,
    /// Current phase claim.
    pub phase: SettlementPhase,
    /// Optional refund-outcome claim.
    pub refund_outcome: Option<RefundOutcome>,
    /// Observed requested amount.
    pub requested_amount: Option<i128>,
    /// Observed settled amount.
    pub settled_amount: Option<i128>,
    /// Observed unsettled amount.
    pub unsettled_amount: Option<i128>,
    /// Observed fee amount.
    pub fee_amount: Option<i128>,
    /// Observed net settlement amount (MAY be signed).
    pub net_settlement_amount: Option<i128>,
    /// Observed refund amount.
    pub refund_amount: Option<i128>,
    /// Optional escrow status.
    pub escrow_status: Option<EscrowStatus>,
    /// Optional escrow amount (non-authoritative; not evaluated by closed expectations).
    pub escrow_amount: Option<i128>,
    /// Optional delay marker.
    pub delay_marker: Option<String>,
    /// Optional settlement delay in simulation steps.
    pub settlement_delay_steps: Option<u64>,
    /// Opaque M07 semantic label carrier.
    pub m07_label: Option<String>,
    /// Optional M03 simulation result when M03-bound.
    pub simulation_result: Option<SimulationResult>,
    /// Optional adapter provenance.
    pub adapter_provenance: Option<AdapterProvenance>,
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

/// Closed Gate-6 expectation class payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettlementExpectationClass {
    /// Exact settlement status.
    SettlementStatusExact {
        /// Expected status.
        expected_status: SettlementStatus,
    },
    /// Exact settled amount.
    SettledAmountExact {
        /// Expected settled amount (domain `>= 0` enforced at case validation later).
        expected_settled_amount: i128,
    },
    /// Settled amount with M06-compatible absolute tolerance (`tolerance: u128`).
    SettledAmountAbsoluteTolerance {
        /// Expected settled amount (domain `>= 0` enforced later).
        expected_settled_amount: i128,
        /// Non-negative absolute tolerance in minor units.
        tolerance: u128,
    },
    /// Exact fee amount.
    FeeExact {
        /// Expected fee amount (domain `>= 0` enforced later).
        expected_fee_amount: i128,
    },
    /// Exact net settlement amount (MAY be signed).
    NetSettlementExact {
        /// Expected net settlement amount.
        expected_net_settlement_amount: i128,
    },
    /// Exact refund outcome.
    RefundStatusExact {
        /// Expected refund outcome.
        expected_refund_outcome: RefundOutcome,
    },
    /// Exact refund amount.
    RefundAmountExact {
        /// Expected refund amount (domain `>= 0` enforced later).
        expected_refund_amount: i128,
    },
    /// Fixed reversal check: `phase == Reverse` ∧ `status == Reversed` (no variable payload).
    ReversalStatusExact,
    /// Exact escrow status.
    EscrowStatusExact {
        /// Expected escrow status.
        expected_escrow_status: EscrowStatus,
    },
    /// Exact execution state; optional exact phase.
    SettlementExecutionExact {
        /// Expected execution state.
        expected_execution: SettlementExecutionState,
        /// When `Some`, phase must match exactly.
        expected_phase: Option<SettlementPhase>,
    },
}

/// One declared M07 expectation (declaration order is authoritative).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettlementExpectation {
    /// Unique id within the case.
    pub expectation_id: String,
    /// Closed class payload.
    pub class: SettlementExpectationClass,
    /// When false, skipped (no lookup / mismatch / ordinal) — enforced later.
    pub applicable: bool,
}

/// Full payment/settlement test case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentSettlementCase {
    /// Declared case id.
    pub case_id: String,
    /// Declared case version.
    pub case_version: String,
    /// Binding identity.
    pub binding: PaymentSettlementBinding,
    /// Payment declaration.
    pub payment_declaration: PaymentDeclaration,
    /// Settlement declaration.
    pub settlement_declaration: SettlementDeclaration,
    /// Ordered expectations (may be empty).
    pub expectations: Vec<SettlementExpectation>,
    /// Single authoritative observation snapshot.
    pub observation: SettlementObservation,
    /// Optional scenario binding.
    pub scenario_binding: Option<PaymentScenarioBinding>,
    /// Optional M06 composition binding.
    pub m06_binding: Option<M06Binding>,
    /// Optional engine pins.
    pub engine_pins: Option<M07EnginePins>,
    /// Non-authoritative notes.
    pub notes: Option<String>,
}

/// Authoritative M07 verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentSettlementVerdict {
    /// All applicable expectations satisfied; composed M06 (if any) MATCH; no ERROR.
    Match,
    /// ≥1 applicable unsatisfied expectation; evaluation completed.
    Mismatch,
    /// Case/observation/adapter/numeric/composition could not be evaluated.
    Error,
}

/// Structured mismatch evidence (ordering logic implemented later).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentMismatchEvidence {
    /// Expectation id.
    pub expectation_id: String,
    /// Class name label.
    pub class: String,
    /// Target descriptor.
    pub target: String,
    /// Expected value descriptor.
    pub expected: String,
    /// Observed value (actual observed amount for absolute-tolerance; not difference).
    pub observed: String,
    /// Operator.
    pub operator: ComparisonOperator,
    /// Mismatch class label.
    pub mismatch_class: String,
    /// 0-based ordinal among mismatches in declaration order.
    pub ordinal: u64,
    /// Settlement phase on the observation when recorded.
    pub settlement_phase: SettlementPhase,
    /// Payment id.
    pub payment_id: String,
    /// Asset id (from payment declaration authority).
    pub asset_id: String,
    /// Optional provenance reference token.
    pub provenance_ref: Option<String>,
}

/// Semantic provenance (AD-14 deferred; no wire serialization).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentSettlementProvenance {
    /// Case id.
    pub case_id: String,
    /// Case version.
    pub case_version: String,
    /// Payment id.
    pub payment_id: String,
    /// Payment version.
    pub payment_version: String,
    /// Configuration id.
    pub configuration_id: String,
    /// Optional scenario id.
    pub scenario_id: Option<String>,
    /// Optional scenario version.
    pub scenario_version: Option<String>,
    /// Optional scenario configuration id.
    pub scenario_configuration_id: Option<String>,
    /// Observation M01 pin when present on SimulationResult.
    pub observation_m01_engine_version: Option<String>,
    /// Observation M02 pin when present on SimulationResult.
    pub observation_m02_engine_version: Option<String>,
    /// Observation M03 pin when present on SimulationResult.
    pub observation_m03_engine_version: Option<String>,
    /// Observation m07_label when present.
    pub observation_m07_label: Option<String>,
    /// Adapter id when present.
    pub adapter_id: Option<String>,
    /// Adapter version when present.
    pub adapter_version: Option<String>,
    /// Composed M06 case id when present.
    pub m06_case_id: Option<String>,
    /// Composed M06 case version when present.
    pub m06_case_version: Option<String>,
    /// Composed M06 verdict kind when `m06_binding` is present (`Match`/`Mismatch`/`Error`).
    pub m06_verdict: Option<String>,
    /// Opaque semantic label copy when observation label is Some.
    pub m07_semantic_label: Option<String>,
    /// Evaluation policy label (`ALL_MISMATCHES`).
    pub evaluation_policy: String,
}

/// Complete M07 evaluation result shape (engine not yet implemented).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentSettlementResult {
    /// Verdict.
    pub verdict: PaymentSettlementVerdict,
    /// Mismatches (MUST be empty when verdict is Error — enforced later).
    pub mismatches: Vec<PaymentMismatchEvidence>,
    /// Error (present iff verdict is Error — enforced later).
    pub error: Option<PaymentSettlementError>,
    /// Provenance.
    pub provenance: PaymentSettlementProvenance,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::payment_settlement::error::{PaymentSettlementError, PaymentSettlementErrorId};

    #[test]
    fn settlement_status_variants_are_distinct() {
        assert_ne!(
            SettlementStatus::Settled,
            SettlementStatus::PartiallySettled
        );
        assert_ne!(SettlementStatus::Failed, SettlementStatus::Refunded);
    }

    #[test]
    fn refund_outcome_failed_is_not_settlement_failed() {
        // Type-level distinctness: RefundOutcome::Failed vs SettlementStatus::Failed.
        let refund = RefundOutcome::Failed;
        let settlement = SettlementStatus::Failed;
        assert_eq!(refund, RefundOutcome::Failed);
        assert_eq!(settlement, SettlementStatus::Failed);
    }

    #[test]
    fn expectations_preserve_vec_declaration_order() {
        let expectations = [
            SettlementExpectation {
                expectation_id: "a".into(),
                class: SettlementExpectationClass::SettlementStatusExact {
                    expected_status: SettlementStatus::Pending,
                },
                applicable: true,
            },
            SettlementExpectation {
                expectation_id: "b".into(),
                class: SettlementExpectationClass::ReversalStatusExact,
                applicable: false,
            },
        ];
        assert_eq!(expectations[0].expectation_id, "a");
        assert_eq!(expectations[1].expectation_id, "b");
        assert!(matches!(
            expectations[1].class,
            SettlementExpectationClass::ReversalStatusExact
        ));
    }

    #[test]
    fn absolute_tolerance_uses_u128() {
        let class = SettlementExpectationClass::SettledAmountAbsoluteTolerance {
            expected_settled_amount: 100,
            tolerance: 5_u128,
        };
        match class {
            SettlementExpectationClass::SettledAmountAbsoluteTolerance { tolerance, .. } => {
                assert_eq!(tolerance, 5);
            }
            _ => panic!("unexpected class"),
        }
    }

    #[test]
    fn observation_has_no_asset_field() {
        let obs = SettlementObservation {
            binding: PaymentSettlementBinding {
                payment_id: "p".into(),
                payment_version: "1".into(),
                configuration_id: "c".into(),
            },
            settlement_status: SettlementStatus::Pending,
            execution_state: SettlementExecutionState::NotAttempted,
            phase: SettlementPhase::Declare,
            refund_outcome: None,
            requested_amount: None,
            settled_amount: None,
            unsettled_amount: None,
            fee_amount: None,
            net_settlement_amount: None,
            refund_amount: None,
            escrow_status: None,
            escrow_amount: None,
            delay_marker: None,
            settlement_delay_steps: None,
            m07_label: None,
            simulation_result: None,
            adapter_provenance: None,
            notes: None,
        };
        // Asset authority lives on PaymentDeclaration only — compile-time shape check.
        let _ = obs.settled_amount;
        let decl = PaymentDeclaration {
            payment_id: "p".into(),
            payer: "a".into(),
            payee: "b".into(),
            asset_id: "USD".into(),
            gross_amount: 0,
            fee: None,
        };
        assert_eq!(decl.asset_id, "USD");
    }

    #[test]
    fn fee_timing_and_mode_are_orthogonal_enums() {
        assert_ne!(
            format!("{:?}", FeeTiming::DeclaredOnly),
            format!("{:?}", FeeMode::FeeExclusive)
        );
    }

    #[test]
    fn error_taxonomy_includes_reserved_classes() {
        let _ =
            PaymentSettlementError::new(PaymentSettlementErrorId::AmbiguousObservation, "reserved");
        let _ = PaymentSettlementErrorId::EconomicStateMismatch;
    }
}
