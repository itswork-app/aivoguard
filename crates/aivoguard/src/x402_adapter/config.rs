//! Declared M08 mapping configuration (explicit; no hidden version catalogue).

/// Explicit version classification policy (M08-OD-04 remains OPEN).
///
/// `AllowList([])` is **invalid** configuration — never coerced to
/// [`VersionPolicy::RejectAll`] or [`VersionPolicy::ClaimOnly`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionPolicy {
    /// Version is an opaque claim; never reject on version alone.
    ClaimOnly,
    /// Non-empty allow-list; absent or unlisted claim → unsupported.
    AllowList(Vec<String>),
    /// Explicit reject-all; any observation fails version classification.
    RejectAll,
}

/// Explicit mapping configuration required for every adapt call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M08MappingConfiguration {
    /// Deterministic mapping pin (empty → configuration failure).
    pub mapping_version: String,
    /// Configuration identity for provenance.
    pub configuration_id: String,
    /// Configuration version for provenance.
    pub configuration_version: String,
    /// Explicit version policy.
    pub version_policy: VersionPolicy,
    /// When true, absent amount → invalid mapping.
    pub require_amount: bool,
    /// When true, absent asset → invalid mapping.
    pub require_asset: bool,
    /// Optional closed set of observation kinds; `None` = no kind filter.
    /// Empty `Some(vec![])` is invalid configuration.
    pub observation_kind_allow_list: Option<Vec<String>>,
}

impl M08MappingConfiguration {
    /// Convenience constructor for tests / fixtures.
    #[must_use]
    pub fn claim_only(
        mapping_version: impl Into<String>,
        configuration_id: impl Into<String>,
        configuration_version: impl Into<String>,
    ) -> Self {
        Self {
            mapping_version: mapping_version.into(),
            configuration_id: configuration_id.into(),
            configuration_version: configuration_version.into(),
            version_policy: VersionPolicy::ClaimOnly,
            require_amount: false,
            require_asset: false,
            observation_kind_allow_list: None,
        }
    }
}
