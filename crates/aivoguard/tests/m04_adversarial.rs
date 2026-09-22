//! Integration tests for Gate-4 M04 Adversarial Scenario Engine.

#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::too_many_lines)]

use aivoguard::{
    apply_mutation, check_plan_incompatibility, definition_with_kind_projections,
    find_first_incompatibility, inherit_all_projections, plan_occurrences, projections_for_kind,
    resolve_action_id, run_generation, take_parameter_candidates, validate_checklist,
    validate_projection_table, Account, AccountId, Action, ActionId, ActionTarget, ActorId,
    AdversarialErrorClass, Applicability, ApplicabilityPolicy, Asset, AssetId, BalanceFacetModel,
    CandidateOperator, ChecklistView, CmpOp, CompositionPlan, DuplicateIdMode, EconomicState,
    EconomicWorld, FacetId, FieldProjection, GenerationRequest, GenerationStatus, GeneratorConfig,
    GeneratorLimits, IdentifierToken, IncompatibilityRule, Invariant, InvariantEvaluationPlan,
    InvariantId, InvariantScope, LimitCounterId, ParameterDimension, ParameterDomain,
    ParameterTuple, ParameterType, ParameterValue, PositionConstraint, ProjectionMode,
    PropertyExpr, ResolutionClass, Scenario, ScenarioField, StructuralInvalidReason,
    TransformationDefinition, TransformationKind, TruncationPolicy, ValidationClassification,
    ValueExpr, ViolationPolicy, M04_ENGINE_VERSION,
};

fn tok(s: &str) -> IdentifierToken {
    IdentifierToken::try_new(s).expect("token")
}

fn facet() -> FacetId {
    FacetId::new("available")
}

fn world() -> EconomicWorld {
    let mut w = EconomicWorld::new(BalanceFacetModel::OrthogonalDimensions);
    w.insert_facet(facet());
    w.insert_asset(Asset::new("USD", 2));
    w.insert_account(Account::new("alice", ActorId::new("alice-actor")));
    w.insert_account(Account::new("bob", ActorId::new("bob-actor")));
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
    s
}

fn noop_named(id: &str) -> Action {
    Action::NoOp {
        actor: ActorId::new("alice-actor"),
        action_id: Some(ActionId::new(id)),
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

fn base_scenario(actions: Vec<Action>) -> Scenario {
    Scenario::new("base", world(), funded(), actions, 10, "cfg")
}

fn generous_limits() -> GeneratorLimits {
    GeneratorLimits {
        maximum_generated_scenarios: 100,
        maximum_transformations_per_plan: 100,
        maximum_composition_depth: 100,
        maximum_action_mutations: 100,
        maximum_parameter_candidates: 100,
    }
}

fn config_with_rules(rules: Vec<IncompatibilityRule>) -> GeneratorConfig {
    GeneratorConfig {
        limits: generous_limits(),
        applicability_policy: ApplicabilityPolicy::AllowNonApplicable,
        truncation_policy: TruncationPolicy::FailOnExceed,
        incompatibility_rules: rules,
    }
}

fn meta(id: &str, ver: &str) -> TransformationDefinition {
    definition_with_kind_projections(
        tok(id),
        tok(ver),
        TransformationKind::MetadataSuffix {
            id_suffix: tok(id),
            new_version: tok(ver),
        },
        ParameterDomain::default(),
    )
}

fn proj() -> Vec<FieldProjection> {
    inherit_all_projections()
}

#[test]
fn m04_engine_version_stable() {
    assert_eq!(M04_ENGINE_VERSION, "aivoguard-m04-0.0.0");
}

// --- IdentifierToken ---

#[test]
fn identifier_token_rejects_empty() {
    assert!(IdentifierToken::try_new("").is_none());
    assert!(!IdentifierToken::is_valid(""));
}

#[test]
fn identifier_token_case_sensitive_no_trim() {
    let a = tok("Abc");
    let b = tok("abc");
    assert_ne!(a, b);
    let spaced = tok(" A ");
    assert_eq!(spaced.as_str(), " A ");
    assert_ne!(spaced.as_str(), "A");
}

#[test]
fn identifier_token_utf8_accepted() {
    assert!(IdentifierToken::try_new("Ä").is_some());
    assert!(IdentifierToken::try_new("abc def").is_some());
}

// --- Occurrence + EXACT_PAIR ---

#[test]
fn occurrences_repeated_identities_remain_distinct() {
    let occ = plan_occurrences(
        &[tok("A"), tok("A"), tok("B")],
        &[tok("1"), tok("1"), tok("1")],
    )
    .unwrap();
    assert_eq!(occ.len(), 3);
    assert_eq!(occ[0].index, 0);
    assert_eq!(occ[1].index, 1);
    assert_eq!(occ[0].identity, occ[1].identity);
    assert_ne!(occ[0].index, occ[1].index);
}

#[test]
fn exact_pair_ordered_match() {
    let occ = plan_occurrences(&[tok("A"), tok("B")], &[tok("1"), tok("1")]).unwrap();
    let rule = IncompatibilityRule {
        rule_id: tok("r1"),
        left_transformation_identity: tok("A"),
        right_transformation_identity: tok("B"),
        left_version: None,
        right_version: None,
        position_constraint: Some(PositionConstraint::ExactPair {
            left_index: 0,
            right_index: 1,
        }),
    };
    let ev = find_first_incompatibility(&occ, &[rule]).unwrap();
    assert_eq!(ev.left_plan_index, 0);
    assert_eq!(ev.right_plan_index, 1);
}

#[test]
fn exact_pair_reversed_does_not_match() {
    let occ = plan_occurrences(&[tok("A"), tok("B")], &[tok("1"), tok("1")]).unwrap();
    let rule = IncompatibilityRule {
        rule_id: tok("r1"),
        left_transformation_identity: tok("A"),
        right_transformation_identity: tok("B"),
        left_version: None,
        right_version: None,
        position_constraint: Some(PositionConstraint::ExactPair {
            left_index: 1,
            right_index: 0,
        }),
    };
    assert!(find_first_incompatibility(&occ, &[rule]).is_none());
}

#[test]
fn exact_pair_same_index_never_matches() {
    let occ = plan_occurrences(&[tok("A"), tok("A")], &[tok("1"), tok("1")]).unwrap();
    let rule = IncompatibilityRule {
        rule_id: tok("r1"),
        left_transformation_identity: tok("A"),
        right_transformation_identity: tok("A"),
        left_version: None,
        right_version: None,
        position_constraint: Some(PositionConstraint::ExactPair {
            left_index: 0,
            right_index: 0,
        }),
    };
    assert!(find_first_incompatibility(&occ, &[rule]).is_none());
}

#[test]
fn unconstrained_lexicographic_first_pair_wins() {
    // plan A@0, B@1, A@2, B@3 — rule A incompatible with B
    let occ = plan_occurrences(
        &[tok("A"), tok("B"), tok("A"), tok("B")],
        &[tok("1"), tok("1"), tok("1"), tok("1")],
    )
    .unwrap();
    let rule = IncompatibilityRule {
        rule_id: tok("r1"),
        left_transformation_identity: tok("A"),
        right_transformation_identity: tok("B"),
        left_version: None,
        right_version: None,
        position_constraint: None,
    };
    let ev = find_first_incompatibility(&occ, &[rule]).unwrap();
    assert_eq!((ev.left_plan_index, ev.right_plan_index), (0, 1));
}

#[test]
fn self_rule_requires_two_distinct_positions() {
    let single = plan_occurrences(&[tok("A")], &[tok("1")]).unwrap();
    let rule = IncompatibilityRule {
        rule_id: tok("self"),
        left_transformation_identity: tok("A"),
        right_transformation_identity: tok("A"),
        left_version: None,
        right_version: None,
        position_constraint: None,
    };
    assert!(find_first_incompatibility(&single, std::slice::from_ref(&rule)).is_none());

    let two = plan_occurrences(&[tok("A"), tok("A")], &[tok("1"), tok("1")]).unwrap();
    let ev = find_first_incompatibility(&two, &[rule]).unwrap();
    assert_eq!((ev.left_plan_index, ev.right_plan_index), (0, 1));
}

#[test]
fn first_matching_rule_wins_later_ignored() {
    let occ = plan_occurrences(&[tok("A"), tok("B")], &[tok("1"), tok("1")]).unwrap();
    let rules = vec![
        IncompatibilityRule {
            rule_id: tok("first"),
            left_transformation_identity: tok("A"),
            right_transformation_identity: tok("B"),
            left_version: None,
            right_version: None,
            position_constraint: None,
        },
        IncompatibilityRule {
            rule_id: tok("second"),
            left_transformation_identity: tok("A"),
            right_transformation_identity: tok("B"),
            left_version: None,
            right_version: None,
            position_constraint: None,
        },
    ];
    let ev = find_first_incompatibility(&occ, &rules).unwrap();
    assert_eq!(ev.rule_id.as_str(), "first");
}

#[test]
fn version_wildcard_and_exact_mismatch() {
    let occ = plan_occurrences(&[tok("A"), tok("B")], &[tok("1"), tok("2")]).unwrap();
    let wildcard = IncompatibilityRule {
        rule_id: tok("w"),
        left_transformation_identity: tok("A"),
        right_transformation_identity: tok("B"),
        left_version: None,
        right_version: None,
        position_constraint: None,
    };
    assert!(find_first_incompatibility(&occ, &[wildcard]).is_some());

    let mismatch = IncompatibilityRule {
        rule_id: tok("m"),
        left_transformation_identity: tok("A"),
        right_transformation_identity: tok("B"),
        left_version: Some(tok("9")),
        right_version: None,
        position_constraint: None,
    };
    assert!(find_first_incompatibility(&occ, &[mismatch]).is_none());

    let exact = IncompatibilityRule {
        rule_id: tok("e"),
        left_transformation_identity: tok("A"),
        right_transformation_identity: tok("B"),
        left_version: Some(tok("1")),
        right_version: Some(tok("2")),
        position_constraint: None,
    };
    assert!(find_first_incompatibility(&occ, &[exact]).is_some());
}

// --- ActionId resolution ---

#[test]
fn action_id_n0_n1_n_gt1() {
    let actions = vec![noop_named("x"), noop_named("y")];
    match resolve_action_id(&actions, "x") {
        ResolutionClass::Resolved { position, record } => {
            assert_eq!(position, 0);
            assert_eq!(record.match_count, 1);
        }
        other => panic!("expected resolved, got {other:?}"),
    }
    match resolve_action_id(&actions, "missing") {
        ResolutionClass::Missing { record } => assert_eq!(record.match_count, 0),
        other => panic!("expected missing, got {other:?}"),
    }
    let amb = vec![noop_named("dup"), noop_named("dup")];
    match resolve_action_id(&amb, "dup") {
        ResolutionClass::Ambiguous { record } => assert_eq!(record.match_count, 2),
        other => panic!("expected ambiguous, got {other:?}"),
    }
    match resolve_action_id(&actions, "") {
        ResolutionClass::MalformedParameter { .. } => {}
        other => panic!("expected malformed, got {other:?}"),
    }
}

// --- Structural validation 4a → 4b ---

#[test]
fn structural_4a_failure_precedes_4b() {
    let mut view = ChecklistView::from_scenario(&base_scenario(vec![]), true, proj());
    view.world_present = false;
    view.initial_state_present = false;
    let r = validate_checklist(&view);
    assert_eq!(
        r.primary_reason,
        Some(StructuralInvalidReason::InvalidScenarioStructure)
    );
    assert!(r.detail.contains("4a"));
}

#[test]
fn structural_4b_only_after_4a_passes() {
    let mut view = ChecklistView::from_scenario(&base_scenario(vec![]), true, proj());
    view.world_present = true;
    view.initial_state_present = false;
    let r = validate_checklist(&view);
    assert_eq!(
        r.primary_reason,
        Some(StructuralInvalidReason::InvalidInitialStateStructure)
    );
}

#[test]
fn invariant_envelope_requires_id_and_definition_version() {
    let mut scenario = base_scenario(vec![]);
    scenario.invariant_plan = InvariantEvaluationPlan {
        before_action: vec![Invariant {
            id: InvariantId::new(""),
            definition_version: "1".into(),
            scope: InvariantScope::State,
            violation_policy: ViolationPolicy::All,
            applicability: Applicability::Always,
            property: PropertyExpr::Compare {
                left: ValueExpr::Literal(aivoguard::Money::new(AssetId::new("USD"), 0)),
                op: CmpOp::Eq,
                right: ValueExpr::Literal(aivoguard::Money::new(AssetId::new("USD"), 0)),
            },
            history_reverse: false,
        }],
        after_action: vec![],
        on_completion: vec![],
    };
    // Empty id fails at check 9 — but empty invariant id: InvariantId::new("")
    // Actually check 2 runs first on scenario id which is fine; check 9 catches empty inv id.
    // Wait - empty InvariantId - IdentifierToken::is_valid("") is false.
    let r = validate_checklist(&ChecklistView::from_scenario(&scenario, true, proj()));
    // Empty inv id → InvalidRequiredField? No - scenario id is "base". Check 9.
    // But InvariantId::new("") - as_str is "" → InvalidInvariantPlanStructure
    // However check 2 doesn't look at invariants.
    assert_eq!(
        r.primary_reason,
        Some(StructuralInvalidReason::InvalidInvariantPlanStructure)
    );

    let mut scenario2 = base_scenario(vec![]);
    scenario2.invariant_plan = InvariantEvaluationPlan {
        before_action: vec![Invariant {
            id: InvariantId::new("inv"),
            definition_version: String::new(),
            scope: InvariantScope::State,
            violation_policy: ViolationPolicy::All,
            applicability: Applicability::Always,
            property: PropertyExpr::Compare {
                left: ValueExpr::Literal(aivoguard::Money::new(AssetId::new("USD"), 0)),
                op: CmpOp::Eq,
                right: ValueExpr::Literal(aivoguard::Money::new(AssetId::new("USD"), 0)),
            },
            history_reverse: false,
        }],
        after_action: vec![],
        on_completion: vec![],
    };
    let r2 = validate_checklist(&ChecklistView::from_scenario(&scenario2, true, proj()));
    assert_eq!(
        r2.primary_reason,
        Some(StructuralInvalidReason::InvalidInvariantPlanStructure)
    );
}

#[test]
fn empty_domain_reference_in_balance_rejected() {
    let mut view = ChecklistView::from_scenario(&base_scenario(vec![]), true, proj());
    view.balance_cells = vec![(String::new(), "USD".into(), "available".into(), 1)];
    let r = validate_checklist(&view);
    assert_eq!(
        r.primary_reason,
        Some(StructuralInvalidReason::InvalidDomainReference)
    );
}

// --- Generation / composition / provenance ---

#[test]
fn generation_detects_incompatibility_with_provenance_indices() {
    let plan = CompositionPlan {
        transformations: vec![meta("A", "1"), meta("B", "1")],
    };
    let rules = vec![IncompatibilityRule {
        rule_id: tok("r1"),
        left_transformation_identity: tok("A"),
        right_transformation_identity: tok("B"),
        left_version: None,
        right_version: None,
        position_constraint: None,
    }];
    let err = check_plan_incompatibility(&plan, &rules).unwrap_err();
    assert_eq!(err.class, AdversarialErrorClass::IncompatibleTransform);
    let ev = err.incompatibility.unwrap();
    assert_eq!(ev.left_plan_index, 0);
    assert_eq!(ev.right_plan_index, 1);

    let req = GenerationRequest {
        base: base_scenario(vec![noop_named("a")]),
        plan,
        config: config_with_rules(rules),
        generator_config_id: "gen1".into(),
    };
    let out = run_generation(&req);
    assert_eq!(out.status, GenerationStatus::Failed);
    assert_eq!(
        out.error.as_ref().unwrap().class,
        AdversarialErrorClass::IncompatibleTransform
    );
}

#[test]
fn generation_emits_derived_with_provenance_chain() {
    let plan = CompositionPlan {
        transformations: vec![meta("T0", "1")],
    };
    let req = GenerationRequest {
        base: base_scenario(vec![noop_named("a")]),
        plan,
        config: config_with_rules(vec![]),
        generator_config_id: "gen1".into(),
    };
    let out = run_generation(&req);
    assert_eq!(out.status, GenerationStatus::Complete);
    assert_eq!(out.emitted.len(), 1);
    assert_eq!(out.emitted[0].validation, ValidationClassification::Valid);
    assert_eq!(out.emitted[0].provenance.composition_positions, vec![0]);
    assert_eq!(
        out.emitted[0].provenance.transformation_chain[0].0.as_str(),
        "T0"
    );
    assert_eq!(out.emitted[0].provenance.engine_version, M04_ENGINE_VERSION);
}

#[test]
fn duplicate_action_default_removes_id() {
    let base = base_scenario(vec![noop_named("keep")]);
    let def = definition_with_kind_projections(
        tok("dup"),
        tok("1"),
        TransformationKind::DuplicateAction {
            source: ActionTarget::ById(tok("keep")),
            insert_at: 1,
            id_mode: DuplicateIdMode::RemoveExplicitly,
        },
        ParameterDomain::default(),
    );
    match apply_mutation(
        &base,
        &def,
        &ParameterTuple { bindings: vec![] },
        ApplicabilityPolicy::AllowNonApplicable,
    ) {
        aivoguard::TransformApplication::Derived { scenario, .. } => {
            assert_eq!(scenario.actions.len(), 2);
            assert!(scenario.actions[1].action_id().is_none());
        }
        other => panic!("expected derived, got {other:?}"),
    }
}

#[test]
fn ambiguous_action_id_is_parameter_error() {
    let base = base_scenario(vec![noop_named("x"), noop_named("x")]);
    let def = definition_with_kind_projections(
        tok("del"),
        tok("1"),
        TransformationKind::DeleteAction {
            target: ActionTarget::ById(tok("x")),
        },
        ParameterDomain::default(),
    );
    match apply_mutation(
        &base,
        &def,
        &ParameterTuple { bindings: vec![] },
        ApplicabilityPolicy::AllowNonApplicable,
    ) {
        aivoguard::TransformApplication::Error(err) => {
            assert_eq!(err.class, AdversarialErrorClass::InvalidTransformParameter);
            assert!(err.reason.contains("AMBIGUOUS_ACTION_ID"));
        }
        other => panic!("expected error, got {other:?}"),
    }
}

#[test]
fn missing_action_non_applicable_under_allow() {
    let base = base_scenario(vec![noop_named("a")]);
    let def = definition_with_kind_projections(
        tok("del"),
        tok("1"),
        TransformationKind::DeleteAction {
            target: ActionTarget::ById(tok("missing")),
        },
        ParameterDomain::default(),
    );
    match apply_mutation(
        &base,
        &def,
        &ParameterTuple { bindings: vec![] },
        ApplicabilityPolicy::AllowNonApplicable,
    ) {
        aivoguard::TransformApplication::NonApplicable { .. } => {}
        other => panic!("expected non-applicable, got {other:?}"),
    }
    match apply_mutation(
        &base,
        &def,
        &ParameterTuple { bindings: vec![] },
        ApplicabilityPolicy::RequireApplicable,
    ) {
        aivoguard::TransformApplication::Error(err) => {
            assert_eq!(err.class, AdversarialErrorClass::NonApplicableTransform);
        }
        other => panic!("expected error, got {other:?}"),
    }
}

#[test]
fn generation_limit_fail_on_exceed() {
    let plan = CompositionPlan {
        transformations: vec![definition_with_kind_projections(
            tok("amt"),
            tok("1"),
            TransformationKind::ReplaceTransferAmount {
                target: ActionTarget::ByIndex(0),
                fallback_amount: 1,
            },
            ParameterDomain {
                dimensions: vec![ParameterDimension {
                    id: tok("amount"),
                    value_type: ParameterType::EconomicAmount,
                    lo: Some(1),
                    hi: Some(5),
                    operators: vec![
                        CandidateOperator::Exact(1),
                        CandidateOperator::Exact(2),
                        CandidateOperator::Exact(3),
                    ],
                    explicit_values: vec![],
                }],
            },
        )],
    };
    let mut limits = generous_limits();
    limits.maximum_generated_scenarios = 2;
    limits.maximum_parameter_candidates = 10;
    let req = GenerationRequest {
        base: base_scenario(vec![transfer(100)]),
        plan,
        config: GeneratorConfig {
            limits,
            applicability_policy: ApplicabilityPolicy::AllowNonApplicable,
            truncation_policy: TruncationPolicy::FailOnExceed,
            incompatibility_rules: vec![],
        },
        generator_config_id: "gen1".into(),
    };
    let out = run_generation(&req);
    assert_eq!(out.status, GenerationStatus::Failed);
    assert_eq!(out.emitted.len(), 2);
    assert_eq!(
        out.error.as_ref().unwrap().class,
        AdversarialErrorClass::GenerationError
    );
}

#[test]
fn determinism_identical_inputs_identical_outputs() {
    let plan = CompositionPlan {
        transformations: vec![
            meta("A", "1"),
            definition_with_kind_projections(
                tok("dup"),
                tok("1"),
                TransformationKind::DuplicateAction {
                    source: ActionTarget::ByIndex(0),
                    insert_at: 1,
                    id_mode: DuplicateIdMode::RemoveExplicitly,
                },
                ParameterDomain::default(),
            ),
        ],
    };
    let req = GenerationRequest {
        base: base_scenario(vec![noop_named("a"), transfer(5)]),
        plan,
        config: config_with_rules(vec![]),
        generator_config_id: "gen1".into(),
    };
    let a = run_generation(&req);
    let b = run_generation(&req);
    assert_eq!(a, b);
    assert_eq!(a.status, GenerationStatus::Complete);
}

#[test]
fn multiple_structural_failures_first_wins() {
    let mut view = ChecklistView::from_scenario(&base_scenario(vec![]), false, proj());
    view.id = String::new();
    view.world_present = false;
    view.provenance_complete = false;
    let r = validate_checklist(&view);
    // Check 2 (required field) precedes 4a and 12.
    assert_eq!(
        r.primary_reason,
        Some(StructuralInvalidReason::InvalidRequiredField)
    );
}

#[test]
fn whitespace_and_case_are_distinct_tokens_in_rules() {
    let occ = plan_occurrences(&[tok("A"), tok("B")], &[tok("1"), tok("1")]).unwrap();
    let rule = IncompatibilityRule {
        rule_id: tok("r"),
        left_transformation_identity: tok("a"),
        right_transformation_identity: tok("B"),
        left_version: None,
        right_version: None,
        position_constraint: None,
    };
    assert!(find_first_incompatibility(&occ, &[rule]).is_none());
}

// --- F-02 projection contract ---

#[test]
fn projection_valid_complete_declaration() {
    let table = projections_for_kind(&TransformationKind::MetadataSuffix {
        id_suffix: tok("x"),
        new_version: tok("1"),
    });
    assert!(validate_projection_table(&table).is_ok());
    let view = ChecklistView::from_scenario(&base_scenario(vec![]), true, table);
    assert!(validate_checklist(&view).is_valid());
}

#[test]
fn projection_missing_declaration_fails_check3() {
    let mut table = inherit_all_projections();
    table.pop();
    assert!(validate_projection_table(&table).is_err());
    let view = ChecklistView::from_scenario(&base_scenario(vec![]), true, table);
    let r = validate_checklist(&view);
    assert_eq!(
        r.primary_reason,
        Some(StructuralInvalidReason::InvalidProjection)
    );
}

#[test]
fn projection_duplicate_field_rejected() {
    let mut table = inherit_all_projections();
    table.push(FieldProjection {
        field: ScenarioField::World,
        mode: ProjectionMode::InheritUnchanged,
    });
    let err = validate_projection_table(&table).unwrap_err();
    assert!(err.contains("duplicate"));
}

#[test]
fn projection_inconsistent_with_kind_rejected_at_phase1() {
    let mut def = definition_with_kind_projections(
        tok("bad"),
        tok("1"),
        TransformationKind::MetadataSuffix {
            id_suffix: tok("x"),
            new_version: tok("1"),
        },
        ParameterDomain::default(),
    );
    // Force inconsistency: claim Actions REPLACE while kind only derives metadata.
    if let Some(p) = def
        .projections
        .iter_mut()
        .find(|p| p.field == ScenarioField::Actions)
    {
        p.mode = ProjectionMode::ReplaceExplicitly;
    }
    let req = GenerationRequest {
        base: base_scenario(vec![]),
        plan: CompositionPlan {
            transformations: vec![def],
        },
        config: config_with_rules(vec![]),
        generator_config_id: "gen1".into(),
    };
    let out = run_generation(&req);
    assert_eq!(out.status, GenerationStatus::Failed);
    assert_eq!(
        out.error.as_ref().unwrap().class,
        AdversarialErrorClass::InvalidAdversarialDefinition
    );
    assert!(out.error.as_ref().unwrap().reason.contains("projection"));
}

#[test]
fn projection_cannot_pass_via_boolean_flag() {
    // Empty declarations fail even if provenance/world flags look healthy.
    let view = ChecklistView::from_scenario(&base_scenario(vec![]), true, vec![]);
    let r = validate_checklist(&view);
    assert_eq!(
        r.primary_reason,
        Some(StructuralInvalidReason::InvalidProjection)
    );
}

// --- F-03 bounded parameter generation ---

#[test]
fn parameter_limit_one_on_huge_cartesian_is_bounded() {
    // 200 x 200 = 40_000 product; limit 1 must not require full materialization.
    let mut values_a = Vec::new();
    let mut values_b = Vec::new();
    for i in 0..200 {
        values_a.push(ParameterValue::Integer(i));
        values_b.push(ParameterValue::Integer(i));
    }
    let domain = ParameterDomain {
        dimensions: vec![
            ParameterDimension {
                id: tok("a"),
                value_type: ParameterType::Integer,
                lo: Some(0),
                hi: Some(199),
                operators: vec![],
                explicit_values: values_a,
            },
            ParameterDimension {
                id: tok("b"),
                value_type: ParameterType::Integer,
                lo: Some(0),
                hi: Some(199),
                operators: vec![],
                explicit_values: values_b,
            },
        ],
    };
    let err = take_parameter_candidates(&domain, 1).unwrap_err();
    let ev = err.limit_breach.expect("structured evidence");
    assert_eq!(ev.counter, LimitCounterId::MaximumParameterCandidates);
    assert_eq!(ev.limit, 1);
    assert_eq!(ev.observed, 2);
}

#[test]
fn parameter_ordering_first_candidate_stable() {
    let domain = ParameterDomain {
        dimensions: vec![
            ParameterDimension {
                id: tok("a"),
                value_type: ParameterType::Integer,
                lo: None,
                hi: None,
                operators: vec![],
                explicit_values: vec![
                    ParameterValue::Integer(1),
                    ParameterValue::Integer(2),
                ],
            },
            ParameterDimension {
                id: tok("b"),
                value_type: ParameterType::Integer,
                lo: None,
                hi: None,
                operators: vec![],
                explicit_values: vec![
                    ParameterValue::Integer(10),
                    ParameterValue::Integer(20),
                ],
            },
        ],
    };
    let a = take_parameter_candidates(&domain, 10).unwrap();
    let b = take_parameter_candidates(&domain, 10).unwrap();
    assert_eq!(a, b);
    assert_eq!(a[0].bindings[0].1, ParameterValue::Integer(1));
    assert_eq!(a[0].bindings[1].1, ParameterValue::Integer(10));
}

#[test]
fn generation_parameter_limit_structured_and_deterministic() {
    let plan = CompositionPlan {
        transformations: vec![definition_with_kind_projections(
            tok("amt"),
            tok("1"),
            TransformationKind::ReplaceTransferAmount {
                target: ActionTarget::ByIndex(0),
                fallback_amount: 1,
            },
            ParameterDomain {
                dimensions: vec![ParameterDimension {
                    id: tok("amount"),
                    value_type: ParameterType::EconomicAmount,
                    lo: Some(1),
                    hi: Some(5),
                    operators: vec![
                        CandidateOperator::Exact(1),
                        CandidateOperator::Exact(2),
                        CandidateOperator::Exact(3),
                    ],
                    explicit_values: vec![],
                }],
            },
        )],
    };
    let mut limits = generous_limits();
    limits.maximum_parameter_candidates = 1;
    limits.maximum_generated_scenarios = 10;
    let req = GenerationRequest {
        base: base_scenario(vec![transfer(100)]),
        plan,
        config: GeneratorConfig {
            limits,
            applicability_policy: ApplicabilityPolicy::AllowNonApplicable,
            truncation_policy: TruncationPolicy::FailOnExceed,
            incompatibility_rules: vec![],
        },
        generator_config_id: "gen1".into(),
    };
    let out1 = run_generation(&req);
    let out2 = run_generation(&req);
    assert_eq!(out1, out2);
    assert_eq!(out1.status, GenerationStatus::Failed);
    let ev = out1.error.as_ref().unwrap().limit_breach.as_ref().unwrap();
    assert_eq!(ev.counter, LimitCounterId::MaximumParameterCandidates);
    assert_eq!(ev.limit, 1);
    assert_eq!(ev.observed, 2);
    assert_eq!(out1.emitted.len(), 1);
}

// --- F-04 structured limit-breach evidence ---

#[test]
fn action_mutation_limit_breach_structured() {
    let plan = CompositionPlan {
        transformations: vec![definition_with_kind_projections(
            tok("dup"),
            tok("1"),
            TransformationKind::DuplicateAction {
                source: ActionTarget::ByIndex(0),
                insert_at: 1,
                id_mode: DuplicateIdMode::RemoveExplicitly,
            },
            ParameterDomain::default(),
        )],
    };
    let mut limits = generous_limits();
    limits.maximum_action_mutations = 0;
    let req = GenerationRequest {
        base: base_scenario(vec![noop_named("a")]),
        plan,
        config: GeneratorConfig {
            limits,
            applicability_policy: ApplicabilityPolicy::AllowNonApplicable,
            truncation_policy: TruncationPolicy::FailOnExceed,
            incompatibility_rules: vec![],
        },
        generator_config_id: "gen1".into(),
    };
    let out = run_generation(&req);
    assert_eq!(out.status, GenerationStatus::Failed);
    let ev = out.error.as_ref().unwrap().limit_breach.as_ref().unwrap();
    assert_eq!(ev.counter, LimitCounterId::MaximumActionMutations);
    assert_eq!(ev.limit, 0);
    assert_eq!(ev.observed, 1);
}
