//! Explicit Scenario-field projection contract (§8.4 / R-01).

/// Scenario fields that must each declare exactly one projection mode (§8.4.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScenarioField {
    /// `EconomicWorld` (`world`).
    World,
    /// `initial EconomicState` (`initial_state`).
    InitialState,
    /// Ordered `Actions` (`actions`).
    Actions,
    /// `maximum_action_steps`.
    MaximumActionSteps,
    /// `configuration_id`.
    ConfigurationId,
    /// `declared_unix_secs`.
    DeclaredUnixSecs,
    /// `invariant_plan`.
    InvariantPlan,
    /// `stop_policy`.
    StopPolicy,
    /// Scenario metadata `id`.
    Id,
    /// Scenario metadata `version`.
    Version,
}

impl ScenarioField {
    /// Canonical ordered list of all required Scenario fields (§8.4.1).
    #[must_use]
    pub const fn all() -> [Self; 10] {
        [
            Self::World,
            Self::InitialState,
            Self::Actions,
            Self::MaximumActionSteps,
            Self::ConfigurationId,
            Self::DeclaredUnixSecs,
            Self::InvariantPlan,
            Self::StopPolicy,
            Self::Id,
            Self::Version,
        ]
    }

    /// Stable identity string for diagnostics (not a serialization freeze).
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::World => "world",
            Self::InitialState => "initial_state",
            Self::Actions => "actions",
            Self::MaximumActionSteps => "maximum_action_steps",
            Self::ConfigurationId => "configuration_id",
            Self::DeclaredUnixSecs => "declared_unix_secs",
            Self::InvariantPlan => "invariant_plan",
            Self::StopPolicy => "stop_policy",
            Self::Id => "id",
            Self::Version => "version",
        }
    }
}

/// Projection mode for one Scenario field (§8.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionMode {
    /// Field copied from the current ScenarioSnapshot.
    InheritUnchanged,
    /// Field replaced by an explicit parameter value.
    ReplaceExplicitly,
    /// Field computed by a declared inspectable derivation rule.
    DeriveExplicitly,
    /// Field cleared / emptied by explicit declaration.
    RemoveExplicitly,
    /// Transformation must not touch this field.
    NotAllowed,
}

/// One declared field → mode binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldProjection {
    /// Scenario field.
    pub field: ScenarioField,
    /// Declared projection mode.
    pub mode: ProjectionMode,
}

/// Validate a projection declaration table independently of execution.
///
/// Requires exactly one declaration per required Scenario field; rejects
/// missing, duplicate, or incomplete tables.
pub fn validate_projection_table(decls: &[FieldProjection]) -> Result<(), String> {
    let required = ScenarioField::all();
    let mut seen = [false; 10];
    for d in decls {
        let Some(idx) = required.iter().position(|f| *f == d.field) else {
            return Err("unknown scenario field in projection table".into());
        };
        if seen[idx] {
            return Err(format!(
                "duplicate projection declaration for field {}",
                d.field.as_str()
            ));
        }
        seen[idx] = true;
    }
    for (i, present) in seen.iter().enumerate() {
        if !*present {
            return Err(format!(
                "missing projection declaration for field {}",
                required[i].as_str()
            ));
        }
    }
    if decls.len() != required.len() {
        return Err(format!(
            "projection table length {} != required {}",
            decls.len(),
            required.len()
        ));
    }
    Ok(())
}

/// Build a complete inherit-all table (all fields `INHERIT_UNCHANGED`).
#[must_use]
pub fn inherit_all_projections() -> Vec<FieldProjection> {
    ScenarioField::all()
        .into_iter()
        .map(|field| FieldProjection {
            field,
            mode: ProjectionMode::InheritUnchanged,
        })
        .collect()
}
