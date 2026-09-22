//! Provisional M08 carrier types (AD-15 OPEN — not a frozen public API).

use crate::x402_adapter::error::{M08AdapterError, M08AdapterErrorId};

/// Adapter module provenance label (constant; not a wall clock).
pub const M08_ADAPTER_MODULE_LABEL: &str = "m08_inbound_v0";

/// Reserved trust-class name — **not emitted** by inbound adapt (future path).
pub const TRUST_CLASS_REJECTED_RESERVED: &str = "rejected";

/// Reserved trust-class name — **NOT_EMITTABLE** while M08-OD-03 is OPEN.
pub const TRUST_CLASS_VERIFIED_UNDER_POLICY_RESERVED: &str = "verified-under-policy";

/// Case / observation binding identity (opaque tokens; exact compare; no trim).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ObservationBinding {
    /// Optional case identity token.
    pub case_id: Option<String>,
    /// Optional observation identity token.
    pub observation_id: Option<String>,
}

/// Exact-integer amount carrier (no `f32`/`f64`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmountCarrier {
    /// Already an `i128` value.
    Exact(i128),
    /// Exact integer decimal-digit string (optional sign); no fractional part.
    ExactIntegerDigits(String),
}

/// Emittable trust class for the inbound-only path.
///
/// Only [`TrustClass::UntrustedClaim`] is produced on `Ok`.
/// Names `rejected` and `verified-under-policy` remain recognized by the frozen
/// M08 specification but are **not emitted** by this path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustClass {
    /// Default for successful translation of external observations.
    UntrustedClaim,
}

impl TrustClass {
    /// Stable string label for diagnostics / equality checks.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UntrustedClaim => "untrusted claim",
        }
    }
}

/// Adapter-local success marker (not an economic verdict).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterStatus {
    /// Observation successfully translated.
    Translated,
}

/// Optional non-authoritative provenance seed from the caller.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AdapterProvenanceSeed {
    /// Opaque source reference (does not upgrade trust).
    pub source_reference: Option<String>,
    /// Opaque notes.
    pub notes: Option<String>,
}

/// Explicit external / fixture observation (provisional).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct X402ExternalObservation {
    /// Binding / case identity.
    pub binding: ObservationBinding,
    /// External protocol version **claim** (not product endorsement).
    pub protocol_version: Option<String>,
    /// Observation kind token (requirement / payload / verification / …).
    pub observation_kind: Option<String>,
    /// Opaque source asset claim.
    pub source_asset_id: Option<String>,
    /// Exact-integer amount carrier (absent remains absent).
    pub amount: Option<AmountCarrier>,
    /// Opaque network identifier.
    pub network: Option<String>,
    /// Opaque scheme identifier.
    pub scheme: Option<String>,
    /// Opaque payer identifier.
    pub payer: Option<String>,
    /// Opaque payee identifier.
    pub payee: Option<String>,
    /// Opaque payment reference.
    pub payment_reference: Option<String>,
    /// Additional opaque field carriers (`(key, value)` in declared order).
    pub field_carriers: Vec<(String, String)>,
    /// Optional non-authoritative provenance seed.
    pub adapter_provenance: Option<AdapterProvenanceSeed>,
    /// Non-authoritative notes.
    pub notes: Option<String>,
}

/// Deterministic provenance attached on successful translation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct X402Provenance {
    /// Mapping version pin from configuration.
    pub mapping_version: String,
    /// Configuration identity.
    pub configuration_id: String,
    /// Configuration version.
    pub configuration_version: String,
    /// Observation binding / identity from input.
    pub observation_binding: ObservationBinding,
    /// Constant adapter module label.
    pub adapter_module_label: String,
    /// Optional source reference from input seed (inert; does not upgrade trust).
    pub source_reference: Option<String>,
}

/// Translated amount on the semantic record (exact `i128` only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslatedAmount {
    /// Exact integer amount in minor-unit style carriers.
    pub value: i128,
}

/// Non-authoritative M08 semantic record (`Ok` only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct X402SemanticRecord {
    /// Binding identity (carried through).
    pub binding: ObservationBinding,
    /// Protocol version claim (may be absent under `ClaimOnly`).
    pub protocol_version: Option<String>,
    /// Observation kind (may be absent if not required).
    pub observation_kind: Option<String>,
    /// Opaque asset claim (absent remains absent).
    pub source_asset_id: Option<String>,
    /// Parsed exact amount (absent remains absent).
    pub amount: Option<TranslatedAmount>,
    /// Opaque network.
    pub network: Option<String>,
    /// Opaque scheme.
    pub scheme: Option<String>,
    /// Opaque payer.
    pub payer: Option<String>,
    /// Opaque payee.
    pub payee: Option<String>,
    /// Opaque payment reference.
    pub payment_reference: Option<String>,
    /// Opaque field carriers (order preserved).
    pub field_carriers: Vec<(String, String)>,
    /// Non-authoritative notes.
    pub notes: Option<String>,
    /// Mapping version from config.
    pub mapping_version: String,
    /// Required provenance.
    pub provenance: X402Provenance,
    /// Always [`TrustClass::UntrustedClaim`] on this path.
    pub trust_class: TrustClass,
    /// Adapter-local status.
    pub adapter_status: AdapterStatus,
}

/// Parse an exact-integer digit string into `i128` (no float / decimal conversion).
///
/// # Errors
///
/// * Empty / whitespace-only / any whitespace → [`M08AdapterErrorId::MalformedExternalInput`]
/// * Fractional / float-like / non-digit → [`M08AdapterErrorId::InvalidMapping`]
/// * Overflow → [`M08AdapterErrorId::InvalidMapping`]
pub fn parse_exact_integer_digits(raw: &str) -> Result<i128, M08AdapterError> {
    if raw.is_empty() || raw.chars().all(char::is_whitespace) {
        return Err(M08AdapterError::new(
            M08AdapterErrorId::MalformedExternalInput,
            "amount string is empty or whitespace-only",
        ));
    }
    if raw.chars().any(char::is_whitespace) {
        return Err(M08AdapterError::new(
            M08AdapterErrorId::MalformedExternalInput,
            "amount string contains whitespace",
        ));
    }
    // Reject float / scientific / fractional forms before parse.
    if raw.contains('.') || raw.contains('e') || raw.contains('E') {
        return Err(M08AdapterError::new(
            M08AdapterErrorId::InvalidMapping,
            "amount string is non-integer / float-like; no conversion authorized",
        ));
    }

    let (sign, digits) = match raw.as_bytes() {
        [b'+', rest @ ..] | [b'-', rest @ ..] => {
            if rest.is_empty() {
                return Err(M08AdapterError::new(
                    M08AdapterErrorId::InvalidMapping,
                    "amount string has sign without digits",
                ));
            }
            (&raw[..1], &raw[1..])
        }
        _ => ("", raw),
    };

    if !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err(M08AdapterError::new(
            M08AdapterErrorId::InvalidMapping,
            "amount string contains non-digit characters",
        ));
    }

    let parsed: i128 = format!("{sign}{digits}").parse().map_err(|_| {
        M08AdapterError::new(
            M08AdapterErrorId::InvalidMapping,
            "amount string overflows i128",
        )
    })?;
    Ok(parsed)
}

/// Resolve an [`AmountCarrier`] to `i128`.
///
/// # Errors
///
/// Propagates parse / mapping errors from [`parse_exact_integer_digits`].
pub fn resolve_amount_carrier(carrier: &AmountCarrier) -> Result<i128, M08AdapterError> {
    match carrier {
        AmountCarrier::Exact(v) => Ok(*v),
        AmountCarrier::ExactIntegerDigits(s) => parse_exact_integer_digits(s),
    }
}
