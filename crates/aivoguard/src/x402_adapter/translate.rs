//! Deterministic inbound translation: observation + config → semantic record.

use crate::x402_adapter::config::M08MappingConfiguration;
use crate::x402_adapter::error::M08AdapterError;
use crate::x402_adapter::provenance::build_provenance;
use crate::x402_adapter::types::{
    AdapterStatus, TranslatedAmount, TrustClass, X402ExternalObservation, X402SemanticRecord,
};
use crate::x402_adapter::validate::{
    classify_observation_kind, classify_version, validate_amount_and_asset, validate_configuration,
    validate_required_fields, validate_structure,
};

/// Adapt an explicit external / fixture observation into a non-authoritative
/// [`X402SemanticRecord`].
///
/// Pure deterministic transformation over `observation` + `config`.
/// Mapping / input / configuration failures return [`Err`]; success always
/// carries [`TrustClass::UntrustedClaim`]. Does **not** mutate M01 state,
/// emit M07 observations, perform network I/O, or verify cryptography.
///
/// # Errors
///
/// Returns [`M08AdapterError`] for configuration, structural, mapping, amount,
/// version, or kind classification failures.
pub fn adapt_x402_observation(
    observation: &X402ExternalObservation,
    config: &M08MappingConfiguration,
) -> Result<X402SemanticRecord, M08AdapterError> {
    // 2. configuration validation
    validate_configuration(config)?;
    // 3. structural validation
    validate_structure(observation)?;
    // 4. required fields
    validate_required_fields(observation, config)?;
    // 5. amount / asset
    let amount = validate_amount_and_asset(observation)?;
    // 6. version policy
    classify_version(observation, config)?;
    // 7. observation kind allow-list
    classify_observation_kind(observation, config)?;
    // 8–10. trust = untrusted; translate; provenance (no trust upgrade)
    let provenance = build_provenance(
        config,
        &observation.binding,
        observation.adapter_provenance.as_ref(),
    );

    Ok(X402SemanticRecord {
        binding: observation.binding.clone(),
        protocol_version: observation.protocol_version.clone(),
        observation_kind: observation.observation_kind.clone(),
        source_asset_id: observation.source_asset_id.clone(),
        amount: amount.map(|value| TranslatedAmount { value }),
        network: observation.network.clone(),
        scheme: observation.scheme.clone(),
        payer: observation.payer.clone(),
        payee: observation.payee.clone(),
        payment_reference: observation.payment_reference.clone(),
        field_carriers: observation.field_carriers.clone(),
        notes: observation.notes.clone(),
        mapping_version: config.mapping_version.clone(),
        provenance,
        trust_class: TrustClass::UntrustedClaim,
        adapter_status: AdapterStatus::Translated,
    })
}
