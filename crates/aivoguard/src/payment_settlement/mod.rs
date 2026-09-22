//! M07 Payment & Settlement Testing — evaluation types (Gate 6).
//!
//! Implements Phase 1–5 core for the frozen M07 specification
//! (`b7d1874`). This module does **not** calculate economic truth (M01),
//! evaluate invariants (M02), run simulations (M03), generate adversarial
//! scenarios (M04), or reimplement regression comparison (M06).
//!
//! Open decisions AD-03 / AD-12 / AD-14 / AD-15, DC-06 / DC-09, and
//! M07-OD-01/02/03 remain OPEN. Concrete Rust types are provisional (AD-15).

#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::doc_markdown)]

mod adapter;
mod case;
mod compose;
mod engine;
mod error;
mod pins;
mod types;
mod validate;

pub use adapter::{
    adapt_settlement_observation, ExternalPaymentSettlementObservation, SemanticSettlementAdapter,
    SettlementObservationAdapter,
};
pub use case::{
    validate_case_scenario_binding, validate_expectation_set, validate_payment_settlement_case,
    validate_scenario_binding,
};
pub use compose::{compose_m06, m06_verdict_label, M06ComposeOutcome};
pub use engine::evaluate_payment_settlement;
pub use error::{PaymentSettlementError, PaymentSettlementErrorId};
pub use pins::validate_engine_pins;
pub use types::{
    AdapterProvenance, ComparisonOperator, EscrowStatus, FeeDeclaration, FeeMode, FeeTiming,
    M06Binding, M07EnginePins, PaymentDeclaration, PaymentMismatchEvidence, PaymentScenarioBinding,
    PaymentSettlementBinding, PaymentSettlementCase, PaymentSettlementProvenance,
    PaymentSettlementResult, PaymentSettlementVerdict, RefundOutcome, SettlementDeclaration,
    SettlementExecutionState, SettlementExpectation, SettlementExpectationClass,
    SettlementObservation, SettlementPhase, SettlementStatus,
};
pub use validate::{
    is_valid_status_phase, validate_case_observation_binding,
    validate_requested_amount_case_binding, validate_settlement_observation, validate_status_phase,
};
