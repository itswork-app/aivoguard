//! Minimal reproducible M01 economic transfer example.
//!
//! Amounts are integer minor units: USD has 2 decimal places, so 10_000 =
//! USD 100.00 and 1_500 = USD 15.00. This example exercises the library only;
//! it does not connect to a payment provider or move real funds.

use aivoguard::{
    evaluate, Account, AccountId, Action, ActorId, Asset, AssetId, BalanceFacetModel,
    EconomicDisposition, EconomicState, EconomicWorld, ExecutionContext, FacetId, KernelOutcome,
    TransferRule,
};

fn main() {
    let available = FacetId::new("available");

    let mut world = EconomicWorld::new(BalanceFacetModel::OrthogonalDimensions);
    world.insert_facet(available.clone());
    world.insert_asset(Asset::new("USD", 2));
    world.insert_account(Account::new("alice", ActorId::new("alice-actor")));
    world.insert_account(Account::new("bob", ActorId::new("bob-actor")));
    world.transfer = Some(TransferRule {
        debit_facet: available.clone(),
        credit_facet: available.clone(),
    });

    let mut state = EconomicState::new();
    state.set_balance(
        AccountId::new("alice"),
        AssetId::new("USD"),
        available.clone(),
        10_000,
    );

    let outcome = evaluate(
        &world,
        &state,
        &Action::Transfer {
            actor: ActorId::new("alice-actor"),
            action_id: Some("demo-transfer-001".into()),
            from: AccountId::new("alice"),
            to: AccountId::new("bob"),
            asset: AssetId::new("USD"),
            amount: 1_500,
        },
        &ExecutionContext::new(1, "reproducible-demo-v1"),
    );

    match outcome {
        KernelOutcome::Economic {
            disposition,
            state_after,
            transaction,
            ..
        } => {
            println!("disposition: {disposition:?}");
            println!("effects: {}", transaction.effects.len());
            println!(
                "alice USD minor units: {}",
                state_after.get_balance(
                    &AccountId::new("alice"),
                    &AssetId::new("USD"),
                    &available,
                )
            );
            println!(
                "bob USD minor units: {}",
                state_after.get_balance(
                    &AccountId::new("bob"),
                    &AssetId::new("USD"),
                    &available,
                )
            );

            if disposition != EconomicDisposition::AcceptedEffective {
                std::process::exit(1);
            }
        }
        KernelOutcome::Error { error, .. } => {
            eprintln!("kernel error: {error:?}");
            std::process::exit(1);
        }
    }
}
