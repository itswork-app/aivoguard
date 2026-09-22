//! Explicit reproducible Simulation inputs (no DSL / serialization freeze).

use crate::invariant::{Invariant, InvariantResultKind};
use crate::kernel::{Action, EconomicDisposition, EconomicState, EconomicWorld};

use crate::simulator::step::EvaluationPoint;

/// Scenario identity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScenarioId(String);

impl ScenarioId {
    /// Create a scenario identity.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Borrow the identity string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Declared M02 evaluation schedule (optional; empty = no M02).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InvariantEvaluationPlan {
    /// Invariants evaluated at `BEFORE_ACTION` (State target).
    pub before_action: Vec<Invariant>,
    /// Invariants evaluated at `AFTER_ACTION` (post-M01 Economic only).
    pub after_action: Vec<Invariant>,
    /// Invariants evaluated at `ON_SIMULATION_COMPLETION` (Normal/Early only).
    pub on_completion: Vec<Invariant>,
}

/// Declared early-stop condition (orchestration policy; does not rewrite M01/M02).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StopCondition {
    /// Early-stop after the named step index has been fully processed (Economic path).
    AfterCompletedStep(usize),
    /// Early-stop when an Economic disposition matches.
    OnEconomicDisposition(EconomicDisposition),
    /// Early-stop when an actually-executed M02 evaluation yields the given kind at a point.
    OnM02Kind {
        /// Evaluation point.
        point: EvaluationPoint,
        /// M02 result kind that triggers stop.
        kind: InvariantResultKind,
    },
}

/// Ordered stop-policy checks (first matching condition wins).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StopPolicy {
    /// Declared conditions.
    pub conditions: Vec<StopCondition>,
}

/// Explicit Gate-3 Scenario input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scenario {
    /// Scenario identity.
    pub id: ScenarioId,
    /// Scenario version (reproducibility identity).
    pub version: String,
    /// Authoritative World.
    pub world: EconomicWorld,
    /// Explicit initial `EconomicState` (`State_0`).
    pub initial_state: EconomicState,
    /// Authoritative ordered Action sequence.
    pub actions: Vec<Action>,
    /// Maximum Actions that may be invoked through M01 (`u64`; 0 is legal).
    pub maximum_action_steps: u64,
    /// Declared execution configuration label (M01 `ExecutionContext` + replay identity).
    pub configuration_id: String,
    /// Optional declared logical timestamp for every step (never host clock).
    pub declared_unix_secs: Option<i64>,
    /// Optional M02 evaluation plan.
    pub invariant_plan: InvariantEvaluationPlan,
    /// Optional early-stop policy.
    pub stop_policy: StopPolicy,
}

impl Scenario {
    /// Construct a minimal Scenario with no M02 plan and no stop conditions.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        world: EconomicWorld,
        initial_state: EconomicState,
        actions: Vec<Action>,
        maximum_action_steps: u64,
        configuration_id: impl Into<String>,
    ) -> Self {
        Self {
            id: ScenarioId::new(id),
            version: "1".into(),
            world,
            initial_state,
            actions,
            maximum_action_steps,
            configuration_id: configuration_id.into(),
            declared_unix_secs: None,
            invariant_plan: InvariantEvaluationPlan::default(),
            stop_policy: StopPolicy::default(),
        }
    }
}
