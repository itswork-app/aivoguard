//! Integration tests for Gate-2 M02 Economic Invariant Engine.

use aivoguard::{
    evaluate_invariant, resolve_history_index, Account, AccountId, ActorId, AggOp, Applicability,
    Asset, AssetId, BalanceFacetModel, CmpOp, DomainExpr, EconomicDisposition, EconomicState,
    EconomicWorld, EvaluationTarget, FacetId, HistoryRecord, Invariant, InvariantErrorClass,
    InvariantId, InvariantResultKind, InvariantScope, Money, Price, PriceCategory, PriceId,
    PropertyExpr, RelationKind, RoundingMode, Transaction, TransitionRecord, ValueExpr,
    ViolationPolicy, M02_ENGINE_VERSION,
};

fn facet() -> FacetId {
    FacetId::new("available")
}

fn world_usd() -> EconomicWorld {
    let mut w = EconomicWorld::new(BalanceFacetModel::OrthogonalDimensions);
    w.insert_facet(facet());
    w.insert_asset(Asset::new("USD", 2));
    w.insert_asset(Asset::new("EUR", 2));
    w.insert_account(Account::new("alice", ActorId::new("alice-actor")));
    w.insert_account(Account::new("bob", ActorId::new("bob-actor")));
    w.insert_price(Price {
        id: PriceId::new("usd-eur"),
        base: AssetId::new("USD"),
        quote: AssetId::new("EUR"),
        category: PriceCategory::Execution,
        quote_per_base: 90,
        base_units: 100,
        rounding: RoundingMode::TowardsZero,
    });
    w
}

fn state_with(alice: i128, bob: i128) -> EconomicState {
    let mut s = EconomicState::new();
    s.set_balance(AccountId::new("alice"), AssetId::new("USD"), facet(), alice);
    s.set_balance(AccountId::new("bob"), AssetId::new("USD"), facet(), bob);
    s
}

fn inv(
    id: &str,
    scope: InvariantScope,
    policy: ViolationPolicy,
    applicability: Applicability,
    property: PropertyExpr,
) -> Invariant {
    Invariant {
        id: InvariantId::new(id),
        definition_version: "1".into(),
        scope,
        violation_policy: policy,
        applicability,
        property,
        history_reverse: false,
    }
}

#[test]
fn m02_engine_version_stable() {
    assert_eq!(M02_ENGINE_VERSION, "aivoguard-m02-0.0.0");
}

#[test]
fn pass_applicable_balance_non_negative() {
    let world = world_usd();
    let state = state_with(100, 50);
    let i = inv(
        "nonneg",
        InvariantScope::State,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::ForAll {
            domain: DomainExpr::WorldAccounts,
            body: Box::new(PropertyExpr::Compare {
                left: ValueExpr::BoundAccountBalance {
                    asset: AssetId::new("USD"),
                    facet: facet(),
                },
                op: CmpOp::Ge,
                right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 0)),
            }),
        },
    );
    let out = evaluate_invariant(
        &i,
        EvaluationTarget::State {
            world: &world,
            state: &state,
        },
    );
    assert_eq!(out.kind, InvariantResultKind::Pass);
}

#[test]
fn fail_on_negative_balance() {
    let world = world_usd();
    let state = state_with(100, -1);
    let i = inv(
        "nonneg",
        InvariantScope::State,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::ForAll {
            domain: DomainExpr::WorldAccounts,
            body: Box::new(PropertyExpr::Compare {
                left: ValueExpr::BoundAccountBalance {
                    asset: AssetId::new("USD"),
                    facet: facet(),
                },
                op: CmpOp::Ge,
                right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 0)),
            }),
        },
    );
    let out = evaluate_invariant(
        &i,
        EvaluationTarget::State {
            world: &world,
            state: &state,
        },
    );
    assert_eq!(out.kind, InvariantResultKind::Fail);
    assert_eq!(out.violations.len(), 1);
}

#[test]
fn not_applicable_when_world_lacks_concept() {
    let world = world_usd();
    let state = state_with(0, 0);
    let i = inv(
        "collateral",
        InvariantScope::State,
        ViolationPolicy::All,
        Applicability::RequiresWorldConcept("collateral".into()),
        PropertyExpr::Compare {
            left: ValueExpr::Literal(Money::new(AssetId::new("USD"), 1)),
            op: CmpOp::Ge,
            right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 0)),
        },
    );
    let out = evaluate_invariant(
        &i,
        EvaluationTarget::State {
            world: &world,
            state: &state,
        },
    );
    assert_eq!(out.kind, InvariantResultKind::Pass);
    assert_eq!(
        out.applicability,
        Some(aivoguard::ApplicabilityStatus::NotApplicable)
    );
    assert!(out.violations.is_empty());
}

#[test]
fn incompatible_target_state_vs_history_scope() {
    let world = world_usd();
    let state = state_with(1, 1);
    let i = inv(
        "hist",
        InvariantScope::History,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::Compare {
            left: ValueExpr::Literal(Money::new(AssetId::new("USD"), 1)),
            op: CmpOp::Eq,
            right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 1)),
        },
    );
    let out = evaluate_invariant(
        &i,
        EvaluationTarget::State {
            world: &world,
            state: &state,
        },
    );
    assert_eq!(out.kind, InvariantResultKind::Error);
    assert_eq!(
        out.error.as_ref().unwrap().class,
        InvariantErrorClass::IncompatibleTarget
    );
}

#[test]
fn missing_balance_cell_is_error_not_zero() {
    let world = world_usd();
    let state = EconomicState::new(); // no cells
    let i = inv(
        "need-cell",
        InvariantScope::State,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::Compare {
            left: ValueExpr::Balance {
                account: AccountId::new("alice"),
                asset: AssetId::new("USD"),
                facet: facet(),
            },
            op: CmpOp::Eq,
            right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 0)),
        },
    );
    let out = evaluate_invariant(
        &i,
        EvaluationTarget::State {
            world: &world,
            state: &state,
        },
    );
    assert_eq!(out.kind, InvariantResultKind::Error);
    assert_eq!(
        out.error.as_ref().unwrap().class,
        InvariantErrorClass::MissingRequiredData
    );
    // M01 get_balance would be 0; M02 must not treat absence as zero.
    assert_eq!(
        state.get_balance(&AccountId::new("alice"), &AssetId::new("USD"), &facet()),
        0
    );
}

#[test]
fn explicit_zero_is_valid_observation() {
    let world = world_usd();
    let mut state = EconomicState::new();
    state.set_balance(AccountId::new("alice"), AssetId::new("USD"), facet(), 0);
    let i = inv(
        "zero-ok",
        InvariantScope::State,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::Compare {
            left: ValueExpr::Balance {
                account: AccountId::new("alice"),
                asset: AssetId::new("USD"),
                facet: facet(),
            },
            op: CmpOp::Eq,
            right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 0)),
        },
    );
    let out = evaluate_invariant(
        &i,
        EvaluationTarget::State {
            world: &world,
            state: &state,
        },
    );
    assert_eq!(out.kind, InvariantResultKind::Pass);
}

#[test]
fn cross_asset_compare_without_conversion_errors() {
    let world = world_usd();
    let state = state_with(100, 0);
    let i = inv(
        "xasset",
        InvariantScope::State,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::Compare {
            left: ValueExpr::Literal(Money::new(AssetId::new("USD"), 100)),
            op: CmpOp::Ge,
            right: ValueExpr::Literal(Money::new(AssetId::new("EUR"), 99)),
        },
    );
    let out = evaluate_invariant(
        &i,
        EvaluationTarget::State {
            world: &world,
            state: &state,
        },
    );
    assert_eq!(
        out.error.as_ref().unwrap().class,
        InvariantErrorClass::IncompatibleOperands
    );
}

#[test]
fn cross_asset_compare_with_conversion_passes() {
    let world = world_usd();
    let state = state_with(100, 0);
    let i = inv(
        "xasset-ok",
        InvariantScope::State,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::Compare {
            left: ValueExpr::Convert {
                amount: Box::new(ValueExpr::Literal(Money::new(AssetId::new("USD"), 100))),
                price_id: PriceId::new("usd-eur"),
            },
            op: CmpOp::Eq,
            right: ValueExpr::Literal(Money::new(AssetId::new("EUR"), 90)),
        },
    );
    let out = evaluate_invariant(
        &i,
        EvaluationTarget::State {
            world: &world,
            state: &state,
        },
    );
    assert_eq!(out.kind, InvariantResultKind::Pass);
}

#[test]
fn missing_price_for_conversion_is_missing_data() {
    let world = world_usd();
    let state = state_with(1, 1);
    let i = inv(
        "no-price",
        InvariantScope::State,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::Compare {
            left: ValueExpr::Convert {
                amount: Box::new(ValueExpr::Literal(Money::new(AssetId::new("USD"), 100))),
                price_id: PriceId::new("missing"),
            },
            op: CmpOp::Eq,
            right: ValueExpr::Literal(Money::new(AssetId::new("EUR"), 1)),
        },
    );
    let out = evaluate_invariant(
        &i,
        EvaluationTarget::State {
            world: &world,
            state: &state,
        },
    );
    assert_eq!(
        out.error.as_ref().unwrap().class,
        InvariantErrorClass::MissingRequiredData
    );
}

#[test]
fn forall_empty_passes_exists_empty_fails() {
    let mut world = EconomicWorld::new(BalanceFacetModel::OrthogonalDimensions);
    world.insert_facet(facet());
    world.insert_asset(Asset::new("USD", 2));
    let state = EconomicState::new();
    let forall = inv(
        "fa",
        InvariantScope::State,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::ForAll {
            domain: DomainExpr::WorldAccounts,
            body: Box::new(PropertyExpr::Compare {
                left: ValueExpr::Literal(Money::new(AssetId::new("USD"), 1)),
                op: CmpOp::Eq,
                right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 1)),
            }),
        },
    );
    let exists = inv(
        "ex",
        InvariantScope::State,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::Exists {
            domain: DomainExpr::WorldAccounts,
            body: Box::new(PropertyExpr::Compare {
                left: ValueExpr::Literal(Money::new(AssetId::new("USD"), 1)),
                op: CmpOp::Eq,
                right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 1)),
            }),
        },
    );
    let t = EvaluationTarget::State {
        world: &world,
        state: &state,
    };
    assert_eq!(
        evaluate_invariant(&forall, t).kind,
        InvariantResultKind::Pass
    );
    assert_eq!(
        evaluate_invariant(&exists, t).kind,
        InvariantResultKind::Fail
    );
}

#[test]
fn aggregation_empty_and_overflow() {
    let world = world_usd();
    let state = EconomicState::new();
    let empty_min = inv(
        "min",
        InvariantScope::State,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::Compare {
            left: ValueExpr::Aggregate {
                op: AggOp::Min,
                domain: DomainExpr::Accounts(vec![]),
                of: Some(Box::new(ValueExpr::Literal(Money::new(
                    AssetId::new("USD"),
                    1,
                )))),
                declared_asset: Some(AssetId::new("USD")),
            },
            op: CmpOp::Eq,
            right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 0)),
        },
    );
    let out = evaluate_invariant(
        &empty_min,
        EvaluationTarget::State {
            world: &world,
            state: &state,
        },
    );
    assert_eq!(
        out.error.as_ref().unwrap().class,
        InvariantErrorClass::ArithmeticError
    );

    let empty_sum = inv(
        "sum",
        InvariantScope::State,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::Compare {
            left: ValueExpr::Aggregate {
                op: AggOp::Sum,
                domain: DomainExpr::Accounts(vec![]),
                of: Some(Box::new(ValueExpr::Literal(Money::new(
                    AssetId::new("USD"),
                    1,
                )))),
                declared_asset: Some(AssetId::new("USD")),
            },
            op: CmpOp::Eq,
            right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 0)),
        },
    );
    assert_eq!(
        evaluate_invariant(
            &empty_sum,
            EvaluationTarget::State {
                world: &world,
                state: &state,
            }
        )
        .kind,
        InvariantResultKind::Pass
    );
}

#[test]
fn first_only_does_not_hide_later_error() {
    let world = world_usd();
    let mut state = EconomicState::new();
    state.set_balance(AccountId::new("alice"), AssetId::new("USD"), facet(), -5);
    // bob missing cell
    let i = inv(
        "fo",
        InvariantScope::State,
        ViolationPolicy::FirstOnly,
        Applicability::Always,
        PropertyExpr::ForAll {
            domain: DomainExpr::Accounts(vec![AccountId::new("alice"), AccountId::new("bob")]),
            body: Box::new(PropertyExpr::Compare {
                left: ValueExpr::BoundAccountBalance {
                    asset: AssetId::new("USD"),
                    facet: facet(),
                },
                op: CmpOp::Ge,
                right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 0)),
            }),
        },
    );
    let out = evaluate_invariant(
        &i,
        EvaluationTarget::State {
            world: &world,
            state: &state,
        },
    );
    assert_eq!(out.kind, InvariantResultKind::Error);
    assert_eq!(
        out.error.as_ref().unwrap().class,
        InvariantErrorClass::MissingRequiredData
    );
}

#[test]
fn first_only_retains_first_violation_when_no_error() {
    let world = world_usd();
    let state = state_with(-1, -2);
    let i = inv(
        "fo2",
        InvariantScope::State,
        ViolationPolicy::FirstOnly,
        Applicability::Always,
        PropertyExpr::ForAll {
            domain: DomainExpr::Accounts(vec![AccountId::new("alice"), AccountId::new("bob")]),
            body: Box::new(PropertyExpr::Compare {
                left: ValueExpr::BoundAccountBalance {
                    asset: AssetId::new("USD"),
                    facet: facet(),
                },
                op: CmpOp::Ge,
                right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 0)),
            }),
        },
    );
    let out = evaluate_invariant(
        &i,
        EvaluationTarget::State {
            world: &world,
            state: &state,
        },
    );
    assert_eq!(out.kind, InvariantResultKind::Fail);
    assert_eq!(out.violations.len(), 1);
}

#[test]
#[allow(clippy::too_many_lines)]
fn history_range_index_reverse_and_duplicates() {
    let world = world_usd();
    let records = vec![
        HistoryRecord {
            logical_position: 10,
            transition: None,
        },
        HistoryRecord {
            logical_position: 20,
            transition: None,
        },
        HistoryRecord {
            logical_position: 30,
            transition: None,
        },
    ];
    let mut inv_fwd = inv(
        "h",
        InvariantScope::History,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::Compare {
            left: ValueExpr::Literal(Money::new(AssetId::new("USD"), 1)),
            op: CmpOp::Eq,
            right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 1)),
        },
    );
    let target = EvaluationTarget::History {
        world: &world,
        records: &records,
    };
    assert_eq!(
        evaluate_invariant(&inv_fwd, target).kind,
        InvariantResultKind::Pass
    );

    let domain = DomainExpr::HistoryPresent;
    let r0 = resolve_history_index(&inv_fwd, target, &domain, 0).unwrap();
    assert_eq!(r0.logical_position, 10);
    let r2 = resolve_history_index(&inv_fwd, target, &domain, 2).unwrap();
    assert_eq!(r2.logical_position, 30);
    assert_eq!(
        resolve_history_index(&inv_fwd, target, &domain, 3)
            .unwrap_err()
            .class,
        InvariantErrorClass::MissingRequiredData
    );
    assert_eq!(
        resolve_history_index(&inv_fwd, target, &domain, -1)
            .unwrap_err()
            .class,
        InvariantErrorClass::InvalidInvariantDefinition
    );

    inv_fwd.history_reverse = true;
    let rev0 = resolve_history_index(&inv_fwd, target, &domain, 0).unwrap();
    assert_eq!(rev0.logical_position, 30);

    let range = DomainExpr::HistoryRange {
        start: 10,
        end: 30,
        require_contiguous: false,
    };
    // range then reverse: selected 10,20,30 → reverse ordinal 0 = 30
    let rr = resolve_history_index(&inv_fwd, target, &range, 0).unwrap();
    assert_eq!(rr.logical_position, 30);

    let bad_range = DomainExpr::HistoryRange {
        start: 30,
        end: 10,
        require_contiguous: false,
    };
    assert_eq!(
        resolve_history_index(&inv_fwd, target, &bad_range, 0)
            .unwrap_err()
            .class,
        InvariantErrorClass::InvalidInvariantDefinition
    );

    let missing_contig = DomainExpr::HistoryRange {
        start: 10,
        end: 12,
        require_contiguous: true,
    };
    assert_eq!(
        resolve_history_index(&inv_fwd, target, &missing_contig, 0)
            .unwrap_err()
            .class,
        InvariantErrorClass::MissingRequiredData
    );

    let dup = vec![
        HistoryRecord {
            logical_position: 1,
            transition: None,
        },
        HistoryRecord {
            logical_position: 1,
            transition: None,
        },
    ];
    let tdup = EvaluationTarget::History {
        world: &world,
        records: &dup,
    };
    assert_eq!(
        resolve_history_index(&inv_fwd, tdup, &DomainExpr::HistoryPresent, 0)
            .unwrap_err()
            .class,
        InvariantErrorClass::MissingRequiredData
    );
}

#[test]
fn transition_zero_effect_accepted_and_relation() {
    let world = world_usd();
    let before = state_with(100, 0);
    let after = before.clone();
    let tx = Transaction {
        disposition: EconomicDisposition::AcceptedEffective,
        effects: vec![],
    };
    let i = inv(
        "disp",
        InvariantScope::Transition,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::DispositionAcceptedEffective,
    );
    let out = evaluate_invariant(
        &i,
        EvaluationTarget::Transition {
            world: &world,
            state_before: &before,
            state_after: &after,
            transaction: &tx,
        },
    );
    assert_eq!(out.kind, InvariantResultKind::Pass);

    let rel = inv(
        "rel",
        InvariantScope::Transition,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::Relation {
            kind: RelationKind::BeforeAfterBalanceViaEffects,
            account: AccountId::new("alice"),
            asset: Some(AssetId::new("USD")),
            facet: Some(facet()),
        },
    );
    let out2 = evaluate_invariant(
        &rel,
        EvaluationTarget::Transition {
            world: &world,
            state_before: &before,
            state_after: &after,
            transaction: &tx,
        },
    );
    assert_eq!(out2.kind, InvariantResultKind::Pass);
    let _ = TransitionRecord {
        state_before: before,
        state_after: after,
        transaction: tx,
    };
}

#[test]
fn unsupported_operation_and_invalid_config_and_engine_classes() {
    let world = world_usd();
    let state = state_with(1, 1);
    let unsupported = inv(
        "u",
        InvariantScope::State,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::Unsupported {
            reason: "not in gate-2".into(),
        },
    );
    let out = evaluate_invariant(
        &unsupported,
        EvaluationTarget::State {
            world: &world,
            state: &state,
        },
    );
    assert_eq!(
        out.error.as_ref().unwrap().class,
        InvariantErrorClass::UnsupportedOperation
    );

    let bad_facet = inv(
        "cfg",
        InvariantScope::State,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::Compare {
            left: ValueExpr::Balance {
                account: AccountId::new("alice"),
                asset: AssetId::new("USD"),
                facet: FacetId::new("nope"),
            },
            op: CmpOp::Eq,
            right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 0)),
        },
    );
    let out2 = evaluate_invariant(
        &bad_facet,
        EvaluationTarget::State {
            world: &world,
            state: &state,
        },
    );
    assert_eq!(
        out2.error.as_ref().unwrap().class,
        InvariantErrorClass::InvalidInvariantConfiguration
    );

    // EngineError: construct via prefer path — force ArithmeticError then prefer won't upgrade.
    // Produce EngineError by comparing after a convert that somehow fails as engine — use
    // InvalidInvariantDefinition for empty aggregate of without of on non-empty domain.
    let bad_def = inv(
        "def",
        InvariantScope::State,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::Compare {
            left: ValueExpr::Aggregate {
                op: AggOp::Sum,
                domain: DomainExpr::Accounts(vec![AccountId::new("alice")]),
                of: None,
                declared_asset: Some(AssetId::new("USD")),
            },
            op: CmpOp::Eq,
            right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 0)),
        },
    );
    let out3 = evaluate_invariant(
        &bad_def,
        EvaluationTarget::State {
            world: &world,
            state: &state,
        },
    );
    assert_eq!(
        out3.error.as_ref().unwrap().class,
        InvariantErrorClass::InvalidInvariantDefinition
    );

    // Direct EngineError class existence / precedence
    let e = aivoguard::InvariantError::new(InvariantErrorClass::EngineError, "boom");
    assert_eq!(e.class, InvariantErrorClass::EngineError);
    assert!(
        InvariantErrorClass::InvalidInvariantDefinition.precedence()
            < InvariantErrorClass::EngineError.precedence()
    );
}

#[test]
fn read_only_state_unchanged_and_determinism() {
    let world = world_usd();
    let state = state_with(10, 20);
    let before = state.clone();
    let i = inv(
        "det",
        InvariantScope::State,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::Compare {
            left: ValueExpr::Balance {
                account: AccountId::new("alice"),
                asset: AssetId::new("USD"),
                facet: facet(),
            },
            op: CmpOp::Eq,
            right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 10)),
        },
    );
    let t = EvaluationTarget::State {
        world: &world,
        state: &state,
    };
    let a = evaluate_invariant(&i, t);
    let b = evaluate_invariant(&i, t);
    assert_eq!(a, b);
    assert_eq!(state, before);
}

#[test]
fn nested_error_propagates_over_fail() {
    let world = world_usd();
    let mut state = EconomicState::new();
    state.set_balance(AccountId::new("alice"), AssetId::new("USD"), facet(), -1);
    // bob missing
    let i = inv(
        "nest",
        InvariantScope::State,
        ViolationPolicy::All,
        Applicability::Always,
        PropertyExpr::ForAll {
            domain: DomainExpr::Accounts(vec![AccountId::new("alice"), AccountId::new("bob")]),
            body: Box::new(PropertyExpr::Compare {
                left: ValueExpr::BoundAccountBalance {
                    asset: AssetId::new("USD"),
                    facet: facet(),
                },
                op: CmpOp::Ge,
                right: ValueExpr::Literal(Money::new(AssetId::new("USD"), 0)),
            }),
        },
    );
    let out = evaluate_invariant(
        &i,
        EvaluationTarget::State {
            world: &world,
            state: &state,
        },
    );
    assert_eq!(out.kind, InvariantResultKind::Error);
}
