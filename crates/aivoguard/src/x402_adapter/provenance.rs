//! Deterministic provenance attachment (never upgrades trust).

use crate::x402_adapter::config::M08MappingConfiguration;
use crate::x402_adapter::types::{
    AdapterProvenanceSeed, ObservationBinding, X402Provenance, M08_ADAPTER_MODULE_LABEL,
};

/// Build provenance for a successful translation.
///
/// Provenance is explanatory only — callers must keep `trust_class` as
/// untrusted claim after attachment.
#[must_use]
pub fn build_provenance(
    config: &M08MappingConfiguration,
    binding: &ObservationBinding,
    seed: Option<&AdapterProvenanceSeed>,
) -> X402Provenance {
    X402Provenance {
        mapping_version: config.mapping_version.clone(),
        configuration_id: config.configuration_id.clone(),
        configuration_version: config.configuration_version.clone(),
        observation_binding: binding.clone(),
        adapter_module_label: M08_ADAPTER_MODULE_LABEL.to_owned(),
        source_reference: seed.and_then(|s| s.source_reference.clone()),
    }
}
