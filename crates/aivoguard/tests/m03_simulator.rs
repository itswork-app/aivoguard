//! Integration tests for Gate-3 M03 Deterministic Simulator.

use aivoguard::{
    economic_history_view, run_simulation, Account, AccountId, Action, ActorId, Applicability,
    Asset, AssetId, AttemptClassification, BalanceFacetModel, CmpOp, EconomicDisposition,
    EconomicState, EconomicWorld, EvaluationPoint, ExecutionStatus, FacetId, FatalCause, Invariant,
    InvariantEvaluationPlan, InvariantEvaluationRecord, InvariantId, InvariantResultKind,
    InvariantScope, KernelErrorKind, KernelOutcome, Money, PropertyExpr, Scenario,
    SimulationErrorClass, StopCondition, StopPolicy, TransferRule, ValueExpr, ViolationPolicy,
    M03_ENGINE_VERSION,
};

fn facet() -> FacetId {
    FacetId::new("available")
}

fn world() -> EconomicWorld {
    let mut w = EconomicWorld::new(BalanceFacetModel::OrthogonalDimensions);
    w.insert_facet(facet());
    w.insert_asset(Asset::new("USD", 2));
    w.insert_account(Account::new("alice", ActorId::new("alice-actor")));
    w.insert_account(Account::new("bob", ActorId::new("bob-actor")));
    w.transfer = Some(TransferRule {
        debit_facet: facet(),
        credit_facet: facet(),
    });
    w
}

fn funded() -> EconomicState {
    let mut s = EconomicState::new();
    s.set_balance(
        AccountId::new("alice"),
        AssetId::new("USD"),
        facet(),
        10_000,
    );
    s.set_balance(AccountId::new("bob"), AssetId::new("USD"), facet(), 0);
    s
}

fn noop() -> Action {
    Action::NoOp {
        actor: ActorId::new("alice-actor"),
        action_id: None,
    }
}

fn transfer(amount: i128) -> Action {
    Action::Transfer {
        actor: ActorId::new("alice-actor"),
        action_id: None,
        from: AccountId::new("alice"),
        to: AccountId::new("bob"),
        asset: AssetId::new("USD"),
        amount,
    }
}

fn unauthorized_transfer() -> Action {
    Action::Transfer {
        actor: ActorId::new("bob-actor"),
        action_id: None,
        from: AccountId::new("alice"),
        to: AccountId::new("bob"),
        asset: AssetId::new("USD"),
        amount: 1,
    }
}

fn state_nonneg_inv(id: &str) -> Invariant {
    Invariant {
        id: InvariantId::new(id),
        definition_version: "1".into(),
        scope: InvariantScope::State,
        violation_policy: ViolationPolicy::All,
        applicability: Applicability::Always,
        property: PropertyExpr::Compare {
            left: ValueExpr::Balance {
                account: AccountId::new("alice"),
                asset: AssetId::new("USD"),
                facet: facet(),
            },
            op: CmpOp::Ge,
            right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 0)),
        },
        history_reverse: false,
    }
}

fn state_fail_inv(id: &str) -> Invariant {
    Invariant {
        id: InvariantId::new(id),
        definition_version: "1".into(),
        scope: InvariantScope::State,
        violation_policy: ViolationPolicy::All,
        applicability: Applicability::Always,
        property: PropertyExpr::Compare {
            left: ValueExpr::Balance {
                account: AccountId::new("alice"),
                asset: AssetId::new("USD"),
                facet: facet(),
            },
            op: CmpOp::Lt,
            right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 0)),
        },
        history_reverse: false,
    }
}

#[test]
fn m03_engine_version_stable() {
    assert_eq!(M03_ENGINE_VERSION, "aivoguard-m03-0.0.0");
}

#[test]
fn zero_action_scenario_normal_completion() {
    let scenario = Scenario::new("empty", world(), funded(), vec![], 10, "cfg");
    let out = run_simulation(&scenario);
    assert!(out.is_normal());
    assert_eq!(out.executed_action_count, 0);
    assert_eq!(out.final_state, out.initial_state);
    assert!(out.step_records.is_empty());
    assert!(out.completion_evaluation_records.is_empty());
}

#[test]
fn sequential_noop_and_state_adoption() {
    let scenario = Scenario::new("noop2", world(), funded(), vec![noop(), noop()], 10, "cfg");
    let out = run_simulation(&scenario);
    assert!(out.is_normal());
    assert_eq!(out.executed_action_count, 2);
    assert_eq!(out.step_records.len(), 2);
    assert_eq!(
        out.step_records[0].classification,
        AttemptClassification::AttemptedM01EconomicOutcome
    );
    match out.step_records[0].kernel_outcome.as_ref().unwrap() {
        KernelOutcome::Economic { disposition, .. } => {
            assert_eq!(*disposition, EconomicDisposition::AcceptedEffective);
        }
        KernelOutcome::Error { .. } => panic!("expected economic"),
    }
    assert_eq!(
        out.step_records[0].state_after.as_ref().unwrap(),
        &out.initial_state
    );
}

#[test]
fn accepted_effective_transfer_updates_state() {
    let before = funded();
    let scenario = Scenario::new(
        "xfer",
        world(),
        before.clone(),
        vec![transfer(100)],
        10,
        "cfg",
    );
    let out = run_simulation(&scenario);
    assert!(out.is_normal());
    let after = out
        .final_state
        .balance_if_present(&AccountId::new("alice"), &AssetId::new("USD"), &facet())
        .unwrap();
    assert_eq!(after, 9_900);
    let bob = out
        .final_state
        .balance_if_present(&AccountId::new("bob"), &AssetId::new("USD"), &facet())
        .unwrap();
    assert_eq!(bob, 100);
    assert_ne!(out.final_state, before);
}

#[test]
fn rejected_preserves_m01_semantics() {
    let scenario = Scenario::new(
        "rej",
        world(),
        funded(),
        vec![unauthorized_transfer()],
        10,
        "cfg",
    );
    let out = run_simulation(&scenario);
    assert!(out.is_normal());
    match out.step_records[0].kernel_outcome.as_ref().unwrap() {
        KernelOutcome::Economic { disposition, .. } => {
            assert_eq!(*disposition, EconomicDisposition::Rejected);
        }
        KernelOutcome::Error { .. } => panic!("expected economic rejected"),
    }
    assert_eq!(out.final_state, out.initial_state);
}

#[test]
fn failed_economic_insufficient_funds() {
    let scenario = Scenario::new(
        "fail",
        world(),
        funded(),
        vec![transfer(1_000_000)],
        10,
        "cfg",
    );
    let out = run_simulation(&scenario);
    assert!(out.is_normal());
    match out.step_records[0].kernel_outcome.as_ref().unwrap() {
        KernelOutcome::Economic { disposition, .. } => {
            assert_eq!(*disposition, EconomicDisposition::FailedEconomic);
        }
        KernelOutcome::Error { .. } => panic!("expected failed economic"),
    }
    assert_eq!(out.final_state, out.initial_state);
}

#[test]
fn execution_limit_zero_blocks_first_action() {
    let scenario = Scenario::new("lim0", world(), funded(), vec![noop()], 0, "cfg");
    let out = run_simulation(&scenario);
    assert!(out.is_fatal());
    assert_eq!(out.executed_action_count, 0);
    assert_eq!(out.step_records.len(), 1);
    assert_eq!(
        out.step_records[0].classification,
        AttemptClassification::NotAttempted
    );
    assert!(out.step_records[0].before_action.is_empty());
    assert!(out.step_records[0].after_action.is_empty());
    assert!(out.completion_evaluation_records.is_empty());
    match &out.execution_status {
        ExecutionStatus::FatalTermination {
            cause: FatalCause::Simulation(e),
        } => assert_eq!(e.class, SimulationErrorClass::ExecutionLimitExceeded),
        _ => panic!("expected limit fatal"),
    }
}

#[test]
fn execution_limit_two_allows_exactly_two() {
    let scenario = Scenario::new(
        "lim2",
        world(),
        funded(),
        vec![noop(), noop(), noop()],
        2,
        "cfg",
    );
    let out = run_simulation(&scenario);
    assert!(out.is_fatal());
    assert_eq!(out.executed_action_count, 2);
    assert_eq!(out.step_records.len(), 3);
    assert_eq!(
        out.step_records[0].classification,
        AttemptClassification::AttemptedM01EconomicOutcome
    );
    assert_eq!(
        out.step_records[1].classification,
        AttemptClassification::AttemptedM01EconomicOutcome
    );
    assert_eq!(
        out.step_records[2].classification,
        AttemptClassification::NotAttempted
    );
    assert!(out.completion_evaluation_records.is_empty());
}

#[test]
fn before_action_runs_and_early_stop_skips_m01() {
    let mut scenario = Scenario::new("before", world(), funded(), vec![noop()], 10, "cfg");
    scenario.invariant_plan = InvariantEvaluationPlan {
        before_action: vec![state_fail_inv("fail-before")],
        after_action: vec![],
        on_completion: vec![],
    };
    scenario.stop_policy = StopPolicy {
        conditions: vec![StopCondition::OnM02Kind {
            point: EvaluationPoint::BeforeAction,
            kind: InvariantResultKind::Fail,
        }],
    };
    let out = run_simulation(&scenario);
    assert!(out.is_early());
    assert_eq!(out.executed_action_count, 0);
    assert_eq!(
        out.step_records[0].classification,
        AttemptClassification::NotAttempted
    );
    assert!(matches!(
        &out.step_records[0].before_action[0],
        InvariantEvaluationRecord::Executed {
            outcome,
            ..
        } if outcome.kind == InvariantResultKind::Fail
    ));
    assert!(out.completion_evaluation_records.is_empty());
}

#[test]
fn after_action_sees_adopted_post_state() {
    let mut scenario = Scenario::new("after", world(), funded(), vec![transfer(50)], 10, "cfg");
    scenario.invariant_plan = InvariantEvaluationPlan {
        before_action: vec![],
        after_action: vec![state_nonneg_inv("after-ok")],
        on_completion: vec![],
    };
    let out = run_simulation(&scenario);
    assert!(out.is_normal());
    let after = out.step_records[0].state_after.as_ref().unwrap();
    assert_eq!(
        after
            .balance_if_present(&AccountId::new("alice"), &AssetId::new("USD"), &facet())
            .unwrap(),
        9_950
    );
    assert!(matches!(
        &out.step_records[0].after_action[0],
        InvariantEvaluationRecord::Executed {
            outcome,
            ..
        } if outcome.kind == InvariantResultKind::Pass
    ));
}

#[test]
fn m01_invalid_input_is_fatal_no_after_action() {
    let bad = Action::Transfer {
        actor: ActorId::new("alice-actor"),
        action_id: None,
        from: AccountId::new("alice"),
        to: AccountId::new("bob"),
        asset: AssetId::new("USD"),
        amount: 0, // invalid
    };
    let mut scenario = Scenario::new("bad", world(), funded(), vec![bad], 10, "cfg");
    scenario.invariant_plan.after_action = vec![state_nonneg_inv("should-skip")];
    scenario.invariant_plan.on_completion = vec![state_nonneg_inv("no-completion")];
    let out = run_simulation(&scenario);
    assert!(out.is_fatal());
    assert_eq!(
        out.step_records[0].classification,
        AttemptClassification::AttemptedM01Error
    );
    assert!(out.step_records[0].state_after.is_none());
    assert!(matches!(
        &out.step_records[0].after_action[0],
        InvariantEvaluationRecord::NotExecuted { .. }
    ));
    assert!(out.completion_evaluation_records.is_empty());
    match out.step_records[0].kernel_outcome.as_ref().unwrap() {
        KernelOutcome::Error { error, .. } => {
            assert_eq!(error.kind, KernelErrorKind::InvalidInput);
        }
        KernelOutcome::Economic { .. } => panic!("expected m01 error"),
    }
    match &out.execution_status {
        ExecutionStatus::FatalTermination {
            cause: FatalCause::M01(e),
        } => assert_eq!(e.kind, KernelErrorKind::InvalidInput),
        _ => panic!("expected m01 fatal"),
    }
}

#[test]
fn completion_runs_on_normal_not_on_fatal_limit() {
    let mut ok = Scenario::new("ok", world(), funded(), vec![noop()], 10, "cfg");
    ok.invariant_plan.on_completion = vec![state_nonneg_inv("done")];
    let out_ok = run_simulation(&ok);
    assert!(out_ok.is_normal());
    assert_eq!(out_ok.completion_evaluation_records.len(), 1);

    let mut lim = Scenario::new("lim", world(), funded(), vec![noop()], 0, "cfg");
    lim.invariant_plan.on_completion = vec![state_nonneg_inv("done")];
    let out_lim = run_simulation(&lim);
    assert!(out_lim.is_fatal());
    assert!(out_lim.completion_evaluation_records.is_empty());
}

#[test]
fn early_stop_after_step_runs_completion() {
    let mut scenario = Scenario::new("early", world(), funded(), vec![noop(), noop()], 10, "cfg");
    scenario.stop_policy = StopPolicy {
        conditions: vec![StopCondition::AfterCompletedStep(0)],
    };
    scenario.invariant_plan.on_completion = vec![state_nonneg_inv("early-done")];
    let out = run_simulation(&scenario);
    assert!(out.is_early());
    assert_eq!(out.executed_action_count, 1);
    assert_eq!(out.step_records.len(), 1);
    assert_eq!(out.completion_evaluation_records.len(), 1);
}

#[test]
fn history_view_excludes_m01_errors() {
    let bad = Action::Transfer {
        actor: ActorId::new("alice-actor"),
        action_id: None,
        from: AccountId::new("alice"),
        to: AccountId::new("bob"),
        asset: AssetId::new("USD"),
        amount: 0,
    };
    let scenario = Scenario::new("hist", world(), funded(), vec![noop(), bad], 10, "cfg");
    let out = run_simulation(&scenario);
    assert!(out.is_fatal());
    assert_eq!(out.step_records.len(), 2);
    let hist = economic_history_view(&out.step_records);
    assert_eq!(hist.len(), 1);
    assert_eq!(hist[0].logical_position, 0);
    assert!(hist[0].transition.is_some());
}

#[test]
fn determinism_and_replay_equivalence() {
    let scenario = Scenario::new(
        "det",
        world(),
        funded(),
        vec![transfer(10), noop(), transfer(5)],
        10,
        "cfg",
    );
    let a = run_simulation(&scenario);
    let b = run_simulation(&scenario);
    assert_eq!(a, b);
}

#[test]
fn m02_does_not_mutate_world_or_state() {
    let mut scenario = Scenario::new("ro", world(), funded(), vec![transfer(1)], 10, "cfg");
    scenario.invariant_plan = InvariantEvaluationPlan {
        before_action: vec![state_nonneg_inv("b")],
        after_action: vec![state_nonneg_inv("a")],
        on_completion: vec![state_nonneg_inv("c")],
    };
    let before_world = scenario.world.clone();
    let before_initial = scenario.initial_state.clone();
    let _ = run_simulation(&scenario);
    assert_eq!(scenario.world, before_world);
    assert_eq!(scenario.initial_state, before_initial);
}

#[test]
fn invalid_scenario_id_is_fatal() {
    let mut scenario = Scenario::new("", world(), funded(), vec![], 10, "cfg");
    scenario.id = aivoguard::ScenarioId::new("");
    let out = run_simulation(&scenario);
    assert!(out.is_fatal());
    assert_eq!(
        out.simulation_errors[0].class,
        SimulationErrorClass::MissingSimulationInput
    );
}
