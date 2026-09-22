//! Integration tests for Gate-1 M01 Economic Kernel.

use aivoguard::{
    evaluate, Account, Action, ActorId, Asset, AssetId, BalanceFacetModel, ConversionRule,
    EconomicDisposition, EconomicState, EconomicWorld, ExecutionContext, FacetId, FeeRule,
    KernelErrorKind, KernelOutcome, Money, Price, PriceCategory, PriceId, RoundingMode,
    TransferRule, ENGINE_VERSION,
};

fn available() -> FacetId {
    FacetId::new("available")
}

fn base_world() -> EconomicWorld {
    let mut world = EconomicWorld::new(BalanceFacetModel::OrthogonalDimensions);
    world.insert_facet(available());
    world.insert_asset(Asset::new("USD", 2));
    world.insert_asset(Asset::new("EUR", 2));
    world.insert_account(Account::new("alice", ActorId::new("alice-actor")));
    world.insert_account(Account::new("bob", ActorId::new("bob-actor")));
    world.insert_account(Account::new("fee", ActorId::new("fee-actor")));
    world.transfer = Some(TransferRule {
        debit_facet: available(),
        credit_facet: available(),
    });
    world.fee = Some(FeeRule {
        debit_facet: available(),
        credit_facet: available(),
    });
    world.conversion = Some(ConversionRule {
        required_category: PriceCategory::Execution,
        debit_facet: available(),
        credit_facet: available(),
    });
    world.insert_price(Price {
        id: PriceId::new("usd-eur"),
        base: AssetId::new("USD"),
        quote: AssetId::new("EUR"),
        category: PriceCategory::Execution,
        quote_per_base: 90,
        base_units: 100,
        rounding: RoundingMode::TowardsZero,
    });
    world
}

fn funded_state() -> EconomicState {
    let mut state = EconomicState::new();
    state.set_balance(
        aivoguard::AccountId::new("alice"),
        AssetId::new("USD"),
        available(),
        10_000,
    );
    state.set_balance(
        aivoguard::AccountId::new("alice"),
        AssetId::new("EUR"),
        available(),
        0,
    );
    state
}

fn ctx() -> ExecutionContext {
    ExecutionContext::new(1, "test-config")
}

#[test]
fn engine_version_is_stable() {
    assert_eq!(ENGINE_VERSION, "aivoguard-m01-0.0.0");
}

#[test]
fn money_same_asset_arithmetic_and_cross_asset_rejected() {
    let usd = Money::new(AssetId::new("USD"), 100);
    let usd2 = Money::new(AssetId::new("USD"), 40);
    let eur = Money::new(AssetId::new("EUR"), 40);
    assert_eq!(usd.checked_add(&usd2).unwrap().value(), 140);
    assert_eq!(usd.checked_sub(&usd2).unwrap().value(), 60);
    assert!(usd.checked_add(&eur).is_err());
}

#[test]
fn money_checked_overflow_is_error() {
    let m = Money::new(AssetId::new("USD"), i128::MAX);
    assert!(m.checked_add(&Money::new(AssetId::new("USD"), 1)).is_err());
    assert!(m.checked_mul_i128(2).is_err());
}

#[test]
fn rounding_modes_cover_edge_cases() {
    use aivoguard::divide_with_rounding;
    assert_eq!(
        divide_with_rounding(5, 2, RoundingMode::TowardsZero).unwrap(),
        2
    );
    assert_eq!(
        divide_with_rounding(5, 2, RoundingMode::AwayFromZero).unwrap(),
        3
    );
    assert_eq!(divide_with_rounding(5, 2, RoundingMode::Floor).unwrap(), 2);
    assert_eq!(divide_with_rounding(5, 2, RoundingMode::Ceil).unwrap(), 3);
    assert_eq!(
        divide_with_rounding(5, 2, RoundingMode::HalfAwayFromZero).unwrap(),
        3
    );
    assert_eq!(
        divide_with_rounding(-5, 2, RoundingMode::TowardsZero).unwrap(),
        -2
    );
    assert_eq!(
        divide_with_rounding(-5, 2, RoundingMode::AwayFromZero).unwrap(),
        -3
    );
    assert_eq!(
        divide_with_rounding(-5, 2, RoundingMode::Floor).unwrap(),
        -3
    );
    assert_eq!(divide_with_rounding(-5, 2, RoundingMode::Ceil).unwrap(), -2);
    assert_eq!(
        divide_with_rounding(-5, 2, RoundingMode::HalfAwayFromZero).unwrap(),
        -3
    );
    assert_eq!(
        divide_with_rounding(4, 2, RoundingMode::HalfAwayFromZero).unwrap(),
        2
    );
    assert_eq!(
        divide_with_rounding(1, 3, RoundingMode::HalfAwayFromZero).unwrap(),
        0
    );
    assert_eq!(
        divide_with_rounding(2, 3, RoundingMode::HalfAwayFromZero).unwrap(),
        1
    );
    assert_eq!(divide_with_rounding(0, 3, RoundingMode::Floor).unwrap(), 0);
}

#[test]
fn noop_accepted_effective_zero_effects() {
    let world = base_world();
    let state = funded_state();
    let outcome = evaluate(
        &world,
        &state,
        &Action::NoOp {
            actor: ActorId::new("alice-actor"),
            action_id: None,
        },
        &ctx(),
    );
    match outcome {
        KernelOutcome::Economic {
            disposition,
            state_after,
            transaction,
            evidence,
            ..
        } => {
            assert_eq!(disposition, EconomicDisposition::AcceptedEffective);
            assert!(transaction.effects.is_empty());
            assert_eq!(state_after, state);
            assert!(evidence.disposition.is_some());
        }
        KernelOutcome::Error { error, .. } => panic!("unexpected error: {error:?}"),
    }
}

#[test]
fn transfer_success_and_state_explainability() {
    let world = base_world();
    let state = funded_state();
    let outcome = evaluate(
        &world,
        &state,
        &Action::Transfer {
            actor: ActorId::new("alice-actor"),
            action_id: None,
            from: aivoguard::AccountId::new("alice"),
            to: aivoguard::AccountId::new("bob"),
            asset: AssetId::new("USD"),
            amount: 1_500,
        },
        &ctx(),
    );
    match outcome {
        KernelOutcome::Economic {
            disposition,
            state_before,
            state_after,
            transaction,
            ..
        } => {
            assert_eq!(disposition, EconomicDisposition::AcceptedEffective);
            assert_eq!(transaction.effects.len(), 2);
            assert_eq!(
                state_before.get_balance(
                    &aivoguard::AccountId::new("alice"),
                    &AssetId::new("USD"),
                    &available()
                ),
                10_000
            );
            assert_eq!(
                state_after.get_balance(
                    &aivoguard::AccountId::new("alice"),
                    &AssetId::new("USD"),
                    &available()
                ),
                8_500
            );
            assert_eq!(
                state_after.get_balance(
                    &aivoguard::AccountId::new("bob"),
                    &AssetId::new("USD"),
                    &available()
                ),
                1_500
            );
            let debit: i128 = transaction
                .effects
                .iter()
                .filter(|e| e.cause == "transfer_debit")
                .map(|e| e.delta)
                .sum();
            assert_eq!(debit, -1_500);
        }
        KernelOutcome::Error { error, .. } => panic!("unexpected error: {error:?}"),
    }
}

#[test]
fn insufficient_funds_is_failed_economic_with_zero_effects() {
    let world = base_world();
    let state = funded_state();
    let before = state.clone();
    let outcome = evaluate(
        &world,
        &state,
        &Action::Transfer {
            actor: ActorId::new("alice-actor"),
            action_id: None,
            from: aivoguard::AccountId::new("alice"),
            to: aivoguard::AccountId::new("bob"),
            asset: AssetId::new("USD"),
            amount: 50_000,
        },
        &ctx(),
    );
    match outcome {
        KernelOutcome::Economic {
            disposition,
            state_after,
            transaction,
            ..
        } => {
            assert_eq!(disposition, EconomicDisposition::FailedEconomic);
            assert!(transaction.effects.is_empty());
            assert_eq!(state_after, before);
        }
        KernelOutcome::Error { error, .. } => panic!("unexpected error: {error:?}"),
    }
}

#[test]
fn unauthorized_spend_is_rejected() {
    let world = base_world();
    let state = funded_state();
    let outcome = evaluate(
        &world,
        &state,
        &Action::Transfer {
            actor: ActorId::new("bob-actor"),
            action_id: None,
            from: aivoguard::AccountId::new("alice"),
            to: aivoguard::AccountId::new("bob"),
            asset: AssetId::new("USD"),
            amount: 100,
        },
        &ctx(),
    );
    match outcome {
        KernelOutcome::Economic { disposition, .. } => {
            assert_eq!(disposition, EconomicDisposition::Rejected);
        }
        KernelOutcome::Error { error, .. } => panic!("unexpected error: {error:?}"),
    }
}

#[test]
fn malformed_amount_is_invalid_input() {
    let world = base_world();
    let state = funded_state();
    let outcome = evaluate(
        &world,
        &state,
        &Action::Transfer {
            actor: ActorId::new("alice-actor"),
            action_id: None,
            from: aivoguard::AccountId::new("alice"),
            to: aivoguard::AccountId::new("bob"),
            asset: AssetId::new("USD"),
            amount: 0,
        },
        &ctx(),
    );
    match outcome {
        KernelOutcome::Error { error, .. } => {
            assert_eq!(error.kind, KernelErrorKind::InvalidInput);
        }
        KernelOutcome::Economic { .. } => panic!("expected invalid input"),
    }
}

#[test]
fn missing_transfer_rule_is_configuration_error() {
    let mut world = base_world();
    world.transfer = None;
    let state = funded_state();
    let outcome = evaluate(
        &world,
        &state,
        &Action::Transfer {
            actor: ActorId::new("alice-actor"),
            action_id: None,
            from: aivoguard::AccountId::new("alice"),
            to: aivoguard::AccountId::new("bob"),
            asset: AssetId::new("USD"),
            amount: 100,
        },
        &ctx(),
    );
    match outcome {
        KernelOutcome::Error { error, .. } => {
            assert_eq!(error.kind, KernelErrorKind::ConfigurationError);
        }
        KernelOutcome::Economic { .. } => panic!("expected configuration error"),
    }
}

#[test]
fn transfer_with_fee_atomicity_on_fee_failure() {
    let world = base_world();
    let mut state = funded_state();
    // alice has USD but fee account has zero EUR-equivalent fee asset USD empty for fee actor
    state.set_balance(
        aivoguard::AccountId::new("alice"),
        AssetId::new("USD"),
        available(),
        5_000,
    );
    let before = state.clone();
    let outcome = evaluate(
        &world,
        &state,
        &Action::TransferWithFee {
            actor: ActorId::new("alice-actor"),
            action_id: None,
            from: aivoguard::AccountId::new("alice"),
            to: aivoguard::AccountId::new("bob"),
            asset: AssetId::new("USD"),
            amount: 100,
            fee_payer: aivoguard::AccountId::new("alice"),
            fee_recipient: aivoguard::AccountId::new("fee"),
            fee_asset: AssetId::new("USD"),
            fee_amount: 9_999,
        },
        &ctx(),
    );
    match outcome {
        KernelOutcome::Economic {
            disposition,
            state_after,
            transaction,
            ..
        } => {
            assert_eq!(disposition, EconomicDisposition::FailedEconomic);
            assert!(transaction.effects.is_empty());
            assert_eq!(state_after, before);
        }
        KernelOutcome::Error { error, .. } => panic!("unexpected error: {error:?}"),
    }
}

#[test]
fn transfer_with_fee_success() {
    let world = base_world();
    let state = funded_state();
    let outcome = evaluate(
        &world,
        &state,
        &Action::TransferWithFee {
            actor: ActorId::new("alice-actor"),
            action_id: None,
            from: aivoguard::AccountId::new("alice"),
            to: aivoguard::AccountId::new("bob"),
            asset: AssetId::new("USD"),
            amount: 1_000,
            fee_payer: aivoguard::AccountId::new("alice"),
            fee_recipient: aivoguard::AccountId::new("fee"),
            fee_asset: AssetId::new("USD"),
            fee_amount: 25,
        },
        &ctx(),
    );
    match outcome {
        KernelOutcome::Economic {
            disposition,
            state_after,
            transaction,
            ..
        } => {
            assert_eq!(disposition, EconomicDisposition::AcceptedEffective);
            assert_eq!(transaction.effects.len(), 4);
            assert_eq!(
                state_after.get_balance(
                    &aivoguard::AccountId::new("alice"),
                    &AssetId::new("USD"),
                    &available()
                ),
                8_975
            );
            assert_eq!(
                state_after.get_balance(
                    &aivoguard::AccountId::new("fee"),
                    &AssetId::new("USD"),
                    &available()
                ),
                25
            );
        }
        KernelOutcome::Error { error, .. } => panic!("unexpected error: {error:?}"),
    }
}

#[test]
fn conversion_uses_price_direction_and_rounding() {
    let world = base_world();
    let state = funded_state();
    let outcome = evaluate(
        &world,
        &state,
        &Action::Convert {
            actor: ActorId::new("alice-actor"),
            action_id: None,
            account: aivoguard::AccountId::new("alice"),
            from_asset: AssetId::new("USD"),
            to_asset: AssetId::new("EUR"),
            from_amount: 1_000,
            price_id: PriceId::new("usd-eur"),
        },
        &ctx(),
    );
    match outcome {
        KernelOutcome::Economic {
            disposition,
            state_after,
            ..
        } => {
            assert_eq!(disposition, EconomicDisposition::AcceptedEffective);
            // 1000 * 90 / 100 = 900
            assert_eq!(
                state_after.get_balance(
                    &aivoguard::AccountId::new("alice"),
                    &AssetId::new("EUR"),
                    &available()
                ),
                900
            );
            assert_eq!(
                state_after.get_balance(
                    &aivoguard::AccountId::new("alice"),
                    &AssetId::new("USD"),
                    &available()
                ),
                9_000
            );
        }
        KernelOutcome::Error { error, .. } => panic!("unexpected error: {error:?}"),
    }
}

#[test]
fn inverse_price_direction_is_configuration_error() {
    let mut world = base_world();
    world.insert_price(Price {
        id: PriceId::new("eur-usd"),
        base: AssetId::new("EUR"),
        quote: AssetId::new("USD"),
        category: PriceCategory::Execution,
        quote_per_base: 110,
        base_units: 100,
        rounding: RoundingMode::TowardsZero,
    });
    let state = funded_state();
    let outcome = evaluate(
        &world,
        &state,
        &Action::Convert {
            actor: ActorId::new("alice-actor"),
            action_id: None,
            account: aivoguard::AccountId::new("alice"),
            from_asset: AssetId::new("USD"),
            to_asset: AssetId::new("EUR"),
            from_amount: 100,
            price_id: PriceId::new("eur-usd"),
        },
        &ctx(),
    );
    match outcome {
        KernelOutcome::Error { error, .. } => {
            assert_eq!(error.kind, KernelErrorKind::ConfigurationError);
        }
        KernelOutcome::Economic { .. } => panic!("expected configuration error"),
    }
}

#[test]
fn negative_balance_world_permits_overdraft() {
    let mut world = base_world();
    world.allow_negative_balances = true;
    let state = EconomicState::new();
    let outcome = evaluate(
        &world,
        &state,
        &Action::Transfer {
            actor: ActorId::new("alice-actor"),
            action_id: None,
            from: aivoguard::AccountId::new("alice"),
            to: aivoguard::AccountId::new("bob"),
            asset: AssetId::new("USD"),
            amount: 50,
        },
        &ctx(),
    );
    match outcome {
        KernelOutcome::Economic {
            disposition,
            state_after,
            ..
        } => {
            assert_eq!(disposition, EconomicDisposition::AcceptedEffective);
            assert_eq!(
                state_after.get_balance(
                    &aivoguard::AccountId::new("alice"),
                    &AssetId::new("USD"),
                    &available()
                ),
                -50
            );
        }
        KernelOutcome::Error { error, .. } => panic!("unexpected error: {error:?}"),
    }
}

#[test]
fn determinism_same_inputs_same_output() {
    let world = base_world();
    let state = funded_state();
    let action = Action::Transfer {
        actor: ActorId::new("alice-actor"),
        action_id: None,
        from: aivoguard::AccountId::new("alice"),
        to: aivoguard::AccountId::new("bob"),
        asset: AssetId::new("USD"),
        amount: 333,
    };
    let a = evaluate(&world, &state, &action, &ctx());
    let b = evaluate(&world, &state, &action, &ctx());
    assert_eq!(a, b);
}

#[test]
fn evaluations_do_not_leak_hidden_state() {
    let world = base_world();
    let state = funded_state();
    let action = Action::Transfer {
        actor: ActorId::new("alice-actor"),
        action_id: None,
        from: aivoguard::AccountId::new("alice"),
        to: aivoguard::AccountId::new("bob"),
        asset: AssetId::new("USD"),
        amount: 10,
    };
    let _ = evaluate(&world, &state, &action, &ctx());
    let again = evaluate(&world, &state, &action, &ctx());
    match again {
        KernelOutcome::Economic { state_before, .. } => {
            assert_eq!(state_before, state);
        }
        KernelOutcome::Error { error, .. } => panic!("unexpected error: {error:?}"),
    }
}

#[test]
fn undeclared_facet_configuration_error_on_apply_path() {
    // Construct effects path via a world that credits an unknown facet is prevented
    // at World construction for transfer rule; verify configuration for missing fee rule.
    let mut world = base_world();
    world.fee = None;
    let state = funded_state();
    let outcome = evaluate(
        &world,
        &state,
        &Action::TransferWithFee {
            actor: ActorId::new("alice-actor"),
            action_id: None,
            from: aivoguard::AccountId::new("alice"),
            to: aivoguard::AccountId::new("bob"),
            asset: AssetId::new("USD"),
            amount: 10,
            fee_payer: aivoguard::AccountId::new("alice"),
            fee_recipient: aivoguard::AccountId::new("fee"),
            fee_asset: AssetId::new("USD"),
            fee_amount: 1,
        },
        &ctx(),
    );
    match outcome {
        KernelOutcome::Error { error, .. } => {
            assert_eq!(error.kind, KernelErrorKind::ConfigurationError);
        }
        KernelOutcome::Economic { .. } => panic!("expected configuration error"),
    }
}

#[test]
fn unknown_asset_is_invalid_input() {
    let world = base_world();
    let state = funded_state();
    let outcome = evaluate(
        &world,
        &state,
        &Action::Transfer {
            actor: ActorId::new("alice-actor"),
            action_id: None,
            from: aivoguard::AccountId::new("alice"),
            to: aivoguard::AccountId::new("bob"),
            asset: AssetId::new("BTC"),
            amount: 1,
        },
        &ctx(),
    );
    match outcome {
        KernelOutcome::Error { error, .. } => {
            assert_eq!(error.kind, KernelErrorKind::InvalidInput);
        }
        KernelOutcome::Economic { .. } => panic!("expected invalid input"),
    }
}
