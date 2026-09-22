//! M08 x402 Adapter — inbound-only semantic translation (Gate 7).
//!
//! Implements the TASK-63-authorized bounded path:
//! `observation → validate → translate/classify → provenance → X402SemanticRecord`.
//!
//! This module does **not** contact networks, freeze wire codecs, verify
//! cryptography, mutate M01 [`crate::kernel::EconomicState`], or emit M07
//! [`crate::payment_settlement::SettlementObservation`].
//!
//! Open decisions M08-OD-01…06 and AD-15 remain OPEN. Concrete types are provisional.

#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::doc_markdown)]

mod config;
mod error;
mod provenance;
mod translate;
mod types;
mod validate;

pub use config::{M08MappingConfiguration, VersionPolicy};
pub use error::{M08AdapterError, M08AdapterErrorId};
pub use provenance::build_provenance;
pub use translate::adapt_x402_observation;
pub use types::{
    parse_exact_integer_digits, resolve_amount_carrier, AdapterProvenanceSeed, AdapterStatus,
    AmountCarrier, ObservationBinding, TranslatedAmount, TrustClass, X402ExternalObservation,
    X402Provenance, X402SemanticRecord, M08_ADAPTER_MODULE_LABEL, TRUST_CLASS_REJECTED_RESERVED,
    TRUST_CLASS_VERIFIED_UNDER_POLICY_RESERVED,
};
pub use validate::{
    classify_observation_kind, classify_version, validate_amount_and_asset, validate_configuration,
    validate_required_fields, validate_structure,
};
