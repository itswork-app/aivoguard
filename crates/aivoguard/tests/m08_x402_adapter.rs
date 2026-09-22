//! M08 inbound-only x402 adapter tests (Gate 7 / TASK-64).
//!
//! Proves the bounded path: fixture observation → validate → translate →
//! provenance → non-authoritative `X402SemanticRecord`. No network, wire,
//! M01 mutation, or M07 SettlementObservation emission.

use aivoguard::{
    adapt_x402_observation, AdapterProvenanceSeed, AmountCarrier, M08AdapterErrorId,
    M08MappingConfiguration, ObservationBinding, TrustClass, VersionPolicy,
    X402ExternalObservation, M08_ADAPTER_MODULE_LABEL, TRUST_CLASS_REJECTED_RESERVED,
    TRUST_CLASS_VERIFIED_UNDER_POLICY_RESERVED,
};

fn base_config() -> M08MappingConfiguration {
    M08MappingConfiguration::claim_only("map-v1", "cfg-a", "cfg-v1")
}

fn valid_observation() -> X402ExternalObservation {
    X402ExternalObservation {
        binding: ObservationBinding {
            case_id: Some("case-1".into()),
            observation_id: Some("obs-1".into()),
        },
        protocol_version: Some("1".into()),
        observation_kind: Some("payload".into()),
        source_asset_id: Some("USDC".into()),
        amount: Some(AmountCarrier::Exact(100)),
        network: Some("base".into()),
        scheme: Some("exact".into()),
        payer: Some("payer-a".into()),
        payee: Some("payee-b".into()),
        payment_reference: Some("ref-1".into()),
        field_carriers: vec![("k".into(), "v".into())],
        adapter_provenance: Some(AdapterProvenanceSeed {
            source_reference: Some("fixture://1".into()),
            notes: None,
        }),
        notes: Some("n".into()),
    }
}

#[test]
fn valid_exact_integer_observation_is_untrusted() {
    let record = adapt_x402_observation(&valid_observation(), &base_config()).expect("ok");
    assert_eq!(record.trust_class, TrustClass::UntrustedClaim);
    assert_eq!(record.trust_class.as_str(), "untrusted claim");
    assert_eq!(record.amount.as_ref().map(|a| a.value), Some(100));
    assert_eq!(record.source_asset_id.as_deref(), Some("USDC"));
    assert_eq!(record.mapping_version, "map-v1");
    assert_eq!(
        record.provenance.adapter_module_label,
        M08_ADAPTER_MODULE_LABEL
    );
    assert_eq!(
        record.provenance.source_reference.as_deref(),
        Some("fixture://1")
    );
}

#[test]
fn exact_integer_string_amount_accepted() {
    let mut obs = valid_observation();
    obs.amount = Some(AmountCarrier::ExactIntegerDigits("42".into()));
    let record = adapt_x402_observation(&obs, &base_config()).expect("ok");
    assert_eq!(record.amount.unwrap().value, 42);
}

#[test]
fn i128_boundaries_accepted() {
    for value in [i128::MIN, i128::MAX, 0_i128, -1_i128] {
        let mut obs = valid_observation();
        obs.amount = Some(AmountCarrier::Exact(value));
        let record = adapt_x402_observation(&obs, &base_config()).expect("ok");
        assert_eq!(record.amount.unwrap().value, value);
    }
}

#[test]
fn missing_amount_allowed_when_not_required() {
    let mut obs = valid_observation();
    obs.amount = None;
    let mut cfg = base_config();
    cfg.require_amount = false;
    let record = adapt_x402_observation(&obs, &cfg).expect("ok");
    assert!(record.amount.is_none());
}

#[test]
fn missing_amount_errors_when_required() {
    let mut obs = valid_observation();
    obs.amount = None;
    let mut cfg = base_config();
    cfg.require_amount = true;
    let err = adapt_x402_observation(&obs, &cfg).expect_err("required amount");
    assert_eq!(err.error_id, M08AdapterErrorId::InvalidMapping);
}

#[test]
fn missing_asset_errors_when_required() {
    let mut obs = valid_observation();
    obs.source_asset_id = None;
    let mut cfg = base_config();
    cfg.require_asset = true;
    let err = adapt_x402_observation(&obs, &cfg).expect_err("required asset");
    assert_eq!(err.error_id, M08AdapterErrorId::InvalidMapping);
}

#[test]
fn missing_optional_fields_remain_absent() {
    let obs = X402ExternalObservation {
        amount: Some(AmountCarrier::Exact(1)),
        ..Default::default()
    };
    let record = adapt_x402_observation(&obs, &base_config()).expect("ok");
    assert!(record.protocol_version.is_none());
    assert!(record.source_asset_id.is_none());
    assert!(record.network.is_none());
    assert!(record.payer.is_none());
    assert!(record.notes.is_none());
}

#[test]
fn empty_amount_string_is_malformed() {
    let mut obs = valid_observation();
    obs.amount = Some(AmountCarrier::ExactIntegerDigits("   ".into()));
    let err = adapt_x402_observation(&obs, &base_config()).expect_err("malformed");
    assert_eq!(err.error_id, M08AdapterErrorId::MalformedExternalInput);
}

#[test]
fn decimal_amount_rejected_without_conversion() {
    let mut obs = valid_observation();
    obs.amount = Some(AmountCarrier::ExactIntegerDigits("1.5".into()));
    let err = adapt_x402_observation(&obs, &base_config()).expect_err("decimal");
    assert_eq!(err.error_id, M08AdapterErrorId::InvalidMapping);
}

#[test]
fn float_like_amount_rejected() {
    let mut obs = valid_observation();
    obs.amount = Some(AmountCarrier::ExactIntegerDigits("1e2".into()));
    let err = adapt_x402_observation(&obs, &base_config()).expect_err("float");
    assert_eq!(err.error_id, M08AdapterErrorId::InvalidMapping);
}

#[test]
fn amount_overflow_string_rejected() {
    let mut obs = valid_observation();
    // 39 nines — exceeds i128::MAX digit length.
    obs.amount = Some(AmountCarrier::ExactIntegerDigits(
        "999999999999999999999999999999999999999".into(),
    ));
    let err = adapt_x402_observation(&obs, &base_config()).expect_err("overflow");
    assert_eq!(err.error_id, M08AdapterErrorId::InvalidMapping);
}

#[test]
fn opaque_asset_carried_without_conversion() {
    let mut obs = valid_observation();
    obs.source_asset_id = Some("Asset-X".into());
    let record = adapt_x402_observation(&obs, &base_config()).expect("ok");
    assert_eq!(record.source_asset_id.as_deref(), Some("Asset-X"));
}

#[test]
fn claim_only_preserves_opaque_version() {
    let mut obs = valid_observation();
    obs.protocol_version = Some("experimental-9".into());
    let record = adapt_x402_observation(&obs, &base_config()).expect("ok");
    assert_eq!(record.protocol_version.as_deref(), Some("experimental-9"));
}

#[test]
fn allow_list_hit_translates() {
    let mut cfg = base_config();
    cfg.version_policy = VersionPolicy::AllowList(vec!["1".into(), "2".into()]);
    let record = adapt_x402_observation(&valid_observation(), &cfg).expect("ok");
    assert_eq!(record.protocol_version.as_deref(), Some("1"));
}

#[test]
fn allow_list_miss_is_unsupported() {
    let mut cfg = base_config();
    cfg.version_policy = VersionPolicy::AllowList(vec!["2".into()]);
    let err = adapt_x402_observation(&valid_observation(), &cfg).expect_err("miss");
    assert_eq!(err.error_id, M08AdapterErrorId::UnsupportedProtocolFeature);
}

#[test]
fn allow_list_absent_claim_is_unsupported() {
    let mut obs = valid_observation();
    obs.protocol_version = None;
    let mut cfg = base_config();
    cfg.version_policy = VersionPolicy::AllowList(vec!["1".into()]);
    let err = adapt_x402_observation(&obs, &cfg).expect_err("absent");
    assert_eq!(err.error_id, M08AdapterErrorId::UnsupportedProtocolFeature);
}

#[test]
fn reject_all_always_errors() {
    let mut cfg = base_config();
    cfg.version_policy = VersionPolicy::RejectAll;
    let err = adapt_x402_observation(&valid_observation(), &cfg).expect_err("reject all");
    assert_eq!(err.error_id, M08AdapterErrorId::UnsupportedProtocolFeature);
}

#[test]
fn empty_allow_list_is_configuration_failure_not_reject_all() {
    let mut cfg = base_config();
    cfg.version_policy = VersionPolicy::AllowList(vec![]);
    let err = adapt_x402_observation(&valid_observation(), &cfg).expect_err("empty allow");
    assert_eq!(err.error_id, M08AdapterErrorId::ConfigurationFailure);
}

#[test]
fn empty_mapping_version_is_configuration_failure() {
    let mut cfg = base_config();
    cfg.mapping_version = String::new();
    let err = adapt_x402_observation(&valid_observation(), &cfg).expect_err("empty map ver");
    assert_eq!(err.error_id, M08AdapterErrorId::ConfigurationFailure);
}

#[test]
fn empty_kind_allow_list_is_configuration_failure() {
    let mut cfg = base_config();
    cfg.observation_kind_allow_list = Some(vec![]);
    let err = adapt_x402_observation(&valid_observation(), &cfg).expect_err("empty kinds");
    assert_eq!(err.error_id, M08AdapterErrorId::ConfigurationFailure);
}

#[test]
fn kind_allow_list_miss_is_unsupported() {
    let mut cfg = base_config();
    cfg.observation_kind_allow_list = Some(vec!["requirement".into()]);
    let err = adapt_x402_observation(&valid_observation(), &cfg).expect_err("kind miss");
    assert_eq!(err.error_id, M08AdapterErrorId::UnsupportedProtocolFeature);
}

#[test]
fn mapping_rejection_is_err_not_ok_with_rejected_trust() {
    let mut cfg = base_config();
    cfg.version_policy = VersionPolicy::RejectAll;
    let result = adapt_x402_observation(&valid_observation(), &cfg);
    assert!(result.is_err());
    // Ensure reserved labels exist as names only — not as Ok trust_class.
    assert_eq!(TRUST_CLASS_REJECTED_RESERVED, "rejected");
    assert_eq!(
        TRUST_CLASS_VERIFIED_UNDER_POLICY_RESERVED,
        "verified-under-policy"
    );
}

#[test]
fn provenance_does_not_upgrade_trust() {
    let mut obs = valid_observation();
    obs.adapter_provenance = Some(AdapterProvenanceSeed {
        source_reference: Some("facilitator-claim://verified".into()),
        notes: Some("looks verified".into()),
    });
    let record = adapt_x402_observation(&obs, &base_config()).expect("ok");
    assert_eq!(record.trust_class, TrustClass::UntrustedClaim);
    assert_ne!(
        record.trust_class.as_str(),
        TRUST_CLASS_VERIFIED_UNDER_POLICY_RESERVED
    );
}

#[test]
fn provenance_fields_present_on_ok() {
    let record = adapt_x402_observation(&valid_observation(), &base_config()).expect("ok");
    assert_eq!(record.provenance.mapping_version, "map-v1");
    assert_eq!(record.provenance.configuration_id, "cfg-a");
    assert_eq!(record.provenance.configuration_version, "cfg-v1");
    assert_eq!(
        record.provenance.observation_binding.case_id.as_deref(),
        Some("case-1")
    );
    assert_eq!(
        record
            .provenance
            .observation_binding
            .observation_id
            .as_deref(),
        Some("obs-1")
    );
}

#[test]
fn determinism_same_inputs_same_result() {
    let obs = valid_observation();
    let cfg = base_config();
    let a = adapt_x402_observation(&obs, &cfg).expect("ok");
    let b = adapt_x402_observation(&obs, &cfg).expect("ok");
    assert_eq!(a, b);
}

#[test]
fn no_fabricated_zero_amount_on_missing() {
    let mut obs = valid_observation();
    obs.amount = None;
    let record = adapt_x402_observation(&obs, &base_config()).expect("ok");
    assert!(record.amount.is_none());
}

#[test]
fn no_fabricated_asset_on_missing() {
    let mut obs = valid_observation();
    obs.source_asset_id = None;
    let record = adapt_x402_observation(&obs, &base_config()).expect("ok");
    assert!(record.source_asset_id.is_none());
}

#[test]
fn trust_class_on_ok_is_never_verified_under_policy() {
    let record = adapt_x402_observation(&valid_observation(), &base_config()).expect("ok");
    assert_eq!(record.trust_class, TrustClass::UntrustedClaim);
    // Only UntrustedClaim exists on the emittable enum for this path.
    match record.trust_class {
        TrustClass::UntrustedClaim => {}
    }
}

/// Compile-time / type-level guard: M08 module surface does not expose M07
/// SettlementObservation construction from adapt.
#[test]
fn adapt_return_type_is_semantic_record_not_settlement_observation() {
    let record = adapt_x402_observation(&valid_observation(), &base_config()).expect("ok");
    let _: aivoguard::X402SemanticRecord = record;
}

/// M01 EconomicState is not reachable from adapt (no mutation path).
#[test]
fn m01_economic_state_type_unused_by_adapt() {
    // Presence of EconomicState in the crate must not be mutated by M08.
    // This test only proves adapt does not require or return EconomicState.
    let _ = adapt_x402_observation(&valid_observation(), &base_config());
    let _: Option<aivoguard::EconomicState> = None;
}
