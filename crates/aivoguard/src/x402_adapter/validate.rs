//! Configuration and structural validation for M08 inbound adapt.

use crate::x402_adapter::config::{M08MappingConfiguration, VersionPolicy};
use crate::x402_adapter::error::{M08AdapterError, M08AdapterErrorId};
use crate::x402_adapter::types::{resolve_amount_carrier, AmountCarrier, X402ExternalObservation};

/// Validate mapping configuration (pipeline step 2).
///
/// # Errors
///
/// Returns [`M08AdapterErrorId::ConfigurationFailure`] for empty mapping_version,
/// empty `AllowList`, or empty observation-kind allow-list.
pub fn validate_configuration(config: &M08MappingConfiguration) -> Result<(), M08AdapterError> {
    if config.mapping_version.is_empty() {
        return Err(M08AdapterError::new(
            M08AdapterErrorId::ConfigurationFailure,
            "mapping_version must be non-empty",
        ));
    }
    if config.configuration_id.is_empty() {
        return Err(M08AdapterError::new(
            M08AdapterErrorId::ConfigurationFailure,
            "configuration_id must be non-empty",
        ));
    }
    if config.configuration_version.is_empty() {
        return Err(M08AdapterError::new(
            M08AdapterErrorId::ConfigurationFailure,
            "configuration_version must be non-empty",
        ));
    }
    match &config.version_policy {
        VersionPolicy::AllowList(versions) if versions.is_empty() => {
            return Err(M08AdapterError::new(
                M08AdapterErrorId::ConfigurationFailure,
                "AllowList must be non-empty; empty list is not RejectAll or ClaimOnly",
            ));
        }
        VersionPolicy::ClaimOnly | VersionPolicy::AllowList(_) | VersionPolicy::RejectAll => {}
    }
    if let Some(kinds) = &config.observation_kind_allow_list {
        if kinds.is_empty() {
            return Err(M08AdapterError::new(
                M08AdapterErrorId::ConfigurationFailure,
                "observation_kind_allow_list must be non-empty when present",
            ));
        }
    }
    Ok(())
}

/// Structural checks on observation carriers (pipeline step 3).
///
/// # Errors
///
/// Currently reserved for future structural malformations; amount string
/// emptiness is validated during amount resolution.
pub fn validate_structure(_observation: &X402ExternalObservation) -> Result<(), M08AdapterError> {
    Ok(())
}

/// Required-field checks per configuration (pipeline step 4).
///
/// # Errors
///
/// [`M08AdapterErrorId::InvalidMapping`] when a required amount/asset is absent.
pub fn validate_required_fields(
    observation: &X402ExternalObservation,
    config: &M08MappingConfiguration,
) -> Result<(), M08AdapterError> {
    if config.require_amount && observation.amount.is_none() {
        return Err(M08AdapterError::new(
            M08AdapterErrorId::InvalidMapping,
            "amount is required by configuration but absent",
        ));
    }
    if config.require_asset && observation.source_asset_id.is_none() {
        return Err(M08AdapterError::new(
            M08AdapterErrorId::InvalidMapping,
            "source_asset_id is required by configuration but absent",
        ));
    }
    Ok(())
}

/// Validate and resolve amount; asset remains opaque (pipeline step 5).
///
/// # Errors
///
/// Propagates amount parse / mapping errors. Does not fabricate defaults.
pub fn validate_amount_and_asset(
    observation: &X402ExternalObservation,
) -> Result<Option<i128>, M08AdapterError> {
    match &observation.amount {
        None => Ok(None),
        Some(carrier) => {
            // Empty ExactIntegerDigits caught in parse.
            if let AmountCarrier::ExactIntegerDigits(s) = carrier {
                if s.is_empty() || s.chars().all(char::is_whitespace) {
                    return Err(M08AdapterError::new(
                        M08AdapterErrorId::MalformedExternalInput,
                        "amount string is empty or whitespace-only",
                    ));
                }
            }
            Ok(Some(resolve_amount_carrier(carrier)?))
        }
    }
}

/// Version classification per [`VersionPolicy`] (pipeline step 6).
///
/// # Errors
///
/// [`M08AdapterErrorId::UnsupportedProtocolFeature`] under AllowList miss or RejectAll.
pub fn classify_version(
    observation: &X402ExternalObservation,
    config: &M08MappingConfiguration,
) -> Result<(), M08AdapterError> {
    match &config.version_policy {
        VersionPolicy::ClaimOnly => Ok(()),
        VersionPolicy::RejectAll => Err(M08AdapterError::new(
            M08AdapterErrorId::UnsupportedProtocolFeature,
            "version_policy is RejectAll",
        )),
        VersionPolicy::AllowList(allowed) => match &observation.protocol_version {
            Some(claim) if allowed.iter().any(|v| v == claim) => Ok(()),
            Some(_) => Err(M08AdapterError::new(
                M08AdapterErrorId::UnsupportedProtocolFeature,
                "protocol_version claim not in AllowList",
            )),
            None => Err(M08AdapterError::new(
                M08AdapterErrorId::UnsupportedProtocolFeature,
                "protocol_version claim absent under AllowList policy",
            )),
        },
    }
}

/// Observation-kind allow-list check (pipeline step 7).
///
/// # Errors
///
/// [`M08AdapterErrorId::UnsupportedProtocolFeature`] when kind absent or not listed.
pub fn classify_observation_kind(
    observation: &X402ExternalObservation,
    config: &M08MappingConfiguration,
) -> Result<(), M08AdapterError> {
    let Some(allowed) = &config.observation_kind_allow_list else {
        return Ok(());
    };
    match &observation.observation_kind {
        Some(kind) if allowed.iter().any(|k| k == kind) => Ok(()),
        Some(_) => Err(M08AdapterError::new(
            M08AdapterErrorId::UnsupportedProtocolFeature,
            "observation_kind not in allow-list",
        )),
        None => Err(M08AdapterError::new(
            M08AdapterErrorId::UnsupportedProtocolFeature,
            "observation_kind absent under kind allow-list",
        )),
    }
}
