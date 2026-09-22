//! Deterministic Economic Kernel evaluation entrypoint.

use crate::kernel::account::{AccountId, ActorId};
use crate::kernel::action::Action;
use crate::kernel::asset::AssetId;
use crate::kernel::effect::StateEffect;
use crate::kernel::error::{EconomicDisposition, KernelError};
use crate::kernel::event::EconomicEvent;
use crate::kernel::evidence::Evidence;
use crate::kernel::execution::ExecutionContext;
use crate::kernel::outcome::KernelOutcome;
use crate::kernel::rounding::divide_with_rounding;
use crate::kernel::state::EconomicState;
use crate::kernel::transaction::Transaction;
use crate::kernel::world::{EconomicWorld, PriceId};

/// Engine version included in determinism identity.
pub const ENGINE_VERSION: &str = "aivoguard-m01-0.0.0";

/// Evaluate one Action against declared World, State, and `ExecutionContext`.
///
/// Pure evaluation: no host clock, RNG, network, filesystem, or global mutable
/// economic state.
#[must_use]
pub fn evaluate(
    world: &EconomicWorld,
    state: &EconomicState,
    action: &Action,
    execution_context: &ExecutionContext,
) -> KernelOutcome {
    match evaluate_inner(world, state, action, execution_context) {
        Ok(outcome) => outcome,
        Err(error) => KernelOutcome::Error {
            evidence: Evidence {
                action_summary: action_summary(action),
                world_summary: world_summary(world),
                execution_context: execution_context.clone(),
                disposition: None,
                effects: Vec::new(),
                reason: error.reason.clone(),
            },
            error,
        },
    }
}

fn evaluate_inner(
    world: &EconomicWorld,
    state: &EconomicState,
    action: &Action,
    execution_context: &ExecutionContext,
) -> Result<KernelOutcome, KernelError> {
    validate_world(world)?;

    let decision = match action {
        Action::NoOp { actor, .. } => {
            if world.actors.contains(actor) {
                evaluate_noop(world)
            } else {
                Decision::rejected("actor not declared in World")
            }
        }
        Action::Transfer {
            actor,
            from,
            to,
            asset,
            amount,
            ..
        } => evaluate_transfer(world, state, actor, from, to, asset, *amount)?,
        Action::TransferWithFee {
            actor,
            from,
            to,
            asset,
            amount,
            fee_payer,
            fee_recipient,
            fee_asset,
            fee_amount,
            ..
        } => evaluate_transfer_with_fee(
            world,
            state,
            actor,
            from,
            to,
            asset,
            *amount,
            fee_payer,
            fee_recipient,
            fee_asset,
            *fee_amount,
        )?,
        Action::Convert {
            actor,
            account,
            from_asset,
            to_asset,
            from_amount,
            price_id,
            ..
        } => evaluate_convert(
            world,
            state,
            actor,
            account,
            from_asset,
            to_asset,
            *from_amount,
            price_id,
        )?,
    };

    Ok(finish(world, state, action, execution_context, decision))
}

struct Decision {
    disposition: EconomicDisposition,
    effects: Vec<StateEffect>,
    reason: String,
}

impl Decision {
    fn accepted(effects: Vec<StateEffect>, reason: impl Into<String>) -> Self {
        Self {
            disposition: EconomicDisposition::AcceptedEffective,
            effects,
            reason: reason.into(),
        }
    }

    fn rejected(reason: impl Into<String>) -> Self {
        Self {
            disposition: EconomicDisposition::Rejected,
            effects: Vec::new(),
            reason: reason.into(),
        }
    }

    fn failed(reason: impl Into<String>) -> Self {
        Self {
            disposition: EconomicDisposition::FailedEconomic,
            effects: Vec::new(),
            reason: reason.into(),
        }
    }
}

fn finish(
    world: &EconomicWorld,
    state: &EconomicState,
    action: &Action,
    execution_context: &ExecutionContext,
    decision: Decision,
) -> KernelOutcome {
    let Decision {
        disposition,
        effects,
        reason,
    } = decision;

    let state_after = if disposition == EconomicDisposition::AcceptedEffective {
        match state.apply_effects(world, &effects) {
            Ok(next) => next,
            Err(error) => {
                return KernelOutcome::Error {
                    evidence: Evidence {
                        action_summary: action_summary(action),
                        world_summary: world_summary(world),
                        execution_context: execution_context.clone(),
                        disposition: None,
                        effects: Vec::new(),
                        reason: error.reason.clone(),
                    },
                    error,
                };
            }
        }
    } else {
        state.clone()
    };

    let transaction = Transaction {
        actor: action.actor().clone(),
        disposition,
        effects: effects.clone(),
    };
    let events = vec![EconomicEvent::ActionEvaluated {
        disposition,
        logical_order: execution_context.logical_order,
    }];
    let evidence = Evidence {
        action_summary: action_summary(action),
        world_summary: world_summary(world),
        execution_context: execution_context.clone(),
        disposition: Some(disposition),
        effects,
        reason,
    };

    KernelOutcome::Economic {
        disposition,
        state_before: state.clone(),
        state_after,
        transaction,
        events,
        evidence,
    }
}

fn validate_world(world: &EconomicWorld) -> Result<(), KernelError> {
    if !world.atomic {
        return Err(KernelError::configuration(
            "Gate-1 kernel requires atomic Worlds; partial Worlds are not enabled",
        ));
    }
    for account in world.accounts.values() {
        if !world.actors.contains(&account.owner) {
            return Err(KernelError::configuration(format!(
                "account {} owner {} missing from World actors",
                account.id, account.owner
            )));
        }
    }
    Ok(())
}

fn evaluate_noop(world: &EconomicWorld) -> Decision {
    if world.allow_noop {
        Decision::accepted(Vec::new(), "NoOp accepted with zero effects")
    } else {
        Decision::rejected("NoOp not permitted by World")
    }
}

fn evaluate_transfer(
    world: &EconomicWorld,
    state: &EconomicState,
    actor: &ActorId,
    from: &AccountId,
    to: &AccountId,
    asset: &AssetId,
    amount: i128,
) -> Result<Decision, KernelError> {
    let Some(rule) = world.transfer.as_ref() else {
        return Err(KernelError::configuration(
            "transfer Action requires World transfer rule",
        ));
    };
    if amount <= 0 {
        return Err(KernelError::invalid_input(
            "transfer amount must be positive",
        ));
    }
    require_account(world, from)?;
    require_account(world, to)?;
    require_asset(world, asset)?;
    if let Some(rejected) = authorize_spender(world, actor, from) {
        return Ok(rejected);
    }

    let available = state.get_balance(from, asset, &rule.debit_facet);
    if available < amount && !world.allow_negative_balances {
        return Ok(Decision::failed("insufficient funds for transfer"));
    }

    Ok(Decision::accepted(
        vec![
            StateEffect {
                account: from.clone(),
                asset: asset.clone(),
                facet: rule.debit_facet.clone(),
                delta: -amount,
                cause: "transfer_debit".into(),
            },
            StateEffect {
                account: to.clone(),
                asset: asset.clone(),
                facet: rule.credit_facet.clone(),
                delta: amount,
                cause: "transfer_credit".into(),
            },
        ],
        "transfer accepted",
    ))
}

#[allow(clippy::too_many_arguments)]
fn evaluate_transfer_with_fee(
    world: &EconomicWorld,
    state: &EconomicState,
    actor: &ActorId,
    from: &AccountId,
    to: &AccountId,
    asset: &AssetId,
    amount: i128,
    fee_payer: &AccountId,
    fee_recipient: &AccountId,
    fee_asset: &AssetId,
    fee_amount: i128,
) -> Result<Decision, KernelError> {
    let Some(transfer) = world.transfer.as_ref() else {
        return Err(KernelError::configuration(
            "TransferWithFee requires World transfer rule",
        ));
    };
    let Some(fee) = world.fee.as_ref() else {
        return Err(KernelError::configuration(
            "TransferWithFee requires World fee rule",
        ));
    };
    if amount <= 0 {
        return Err(KernelError::invalid_input(
            "transfer amount must be positive",
        ));
    }
    if fee_amount < 0 {
        return Err(KernelError::invalid_input(
            "fee amount must be non-negative",
        ));
    }
    require_account(world, from)?;
    require_account(world, to)?;
    require_account(world, fee_payer)?;
    require_account(world, fee_recipient)?;
    require_asset(world, asset)?;
    require_asset(world, fee_asset)?;
    if let Some(rejected) = authorize_spender(world, actor, from) {
        return Ok(rejected);
    }
    if let Some(rejected) = authorize_spender(world, actor, fee_payer) {
        return Ok(rejected);
    }

    let available = state.get_balance(from, asset, &transfer.debit_facet);
    if available < amount && !world.allow_negative_balances {
        return Ok(Decision::failed(
            "insufficient funds for transfer principal",
        ));
    }
    let fee_available = state.get_balance(fee_payer, fee_asset, &fee.debit_facet);
    if fee_amount > 0 && fee_available < fee_amount && !world.allow_negative_balances {
        return Ok(Decision::failed("insufficient funds for fee"));
    }

    let mut effects = vec![
        StateEffect {
            account: from.clone(),
            asset: asset.clone(),
            facet: transfer.debit_facet.clone(),
            delta: -amount,
            cause: "transfer_debit".into(),
        },
        StateEffect {
            account: to.clone(),
            asset: asset.clone(),
            facet: transfer.credit_facet.clone(),
            delta: amount,
            cause: "transfer_credit".into(),
        },
    ];
    if fee_amount > 0 {
        effects.push(StateEffect {
            account: fee_payer.clone(),
            asset: fee_asset.clone(),
            facet: fee.debit_facet.clone(),
            delta: -fee_amount,
            cause: "fee_debit".into(),
        });
        effects.push(StateEffect {
            account: fee_recipient.clone(),
            asset: fee_asset.clone(),
            facet: fee.credit_facet.clone(),
            delta: fee_amount,
            cause: "fee_credit".into(),
        });
    }

    Ok(Decision::accepted(effects, "transfer with fee accepted"))
}

#[allow(clippy::too_many_arguments)]
fn evaluate_convert(
    world: &EconomicWorld,
    state: &EconomicState,
    actor: &ActorId,
    account: &AccountId,
    from_asset: &AssetId,
    to_asset: &AssetId,
    from_amount: i128,
    price_id: &PriceId,
) -> Result<Decision, KernelError> {
    let Some(rule) = world.conversion.as_ref() else {
        return Err(KernelError::configuration(
            "Convert Action requires World conversion rule",
        ));
    };
    if from_amount <= 0 {
        return Err(KernelError::invalid_input(
            "conversion from_amount must be positive",
        ));
    }
    if from_asset == to_asset {
        return Err(KernelError::invalid_input(
            "conversion requires distinct assets",
        ));
    }
    require_account(world, account)?;
    require_asset(world, from_asset)?;
    require_asset(world, to_asset)?;
    if let Some(rejected) = authorize_spender(world, actor, account) {
        return Ok(rejected);
    }

    let Some(price) = world.prices.get(price_id) else {
        return Err(KernelError::configuration(
            "missing authoritative price for conversion",
        ));
    };
    if price.category != rule.required_category {
        return Err(KernelError::configuration(
            "price category does not match World conversion rule",
        ));
    }
    if price.base != *from_asset || price.quote != *to_asset {
        return Err(KernelError::configuration(
            "price base/quote direction does not match conversion assets",
        ));
    }
    if price.base_units <= 0 || price.quote_per_base < 0 {
        return Err(KernelError::configuration(
            "price ratio components must be valid (base_units > 0, quote_per_base >= 0)",
        ));
    }

    let available = state.get_balance(account, from_asset, &rule.debit_facet);
    if available < from_amount && !world.allow_negative_balances {
        return Ok(Decision::failed("insufficient funds for conversion source"));
    }

    let numer = from_amount
        .checked_mul(price.quote_per_base)
        .ok_or_else(|| KernelError::engine("overflow computing conversion numerator"))?;
    let to_amount = divide_with_rounding(numer, price.base_units, price.rounding)?;

    Ok(Decision::accepted(
        vec![
            StateEffect {
                account: account.clone(),
                asset: from_asset.clone(),
                facet: rule.debit_facet.clone(),
                delta: -from_amount,
                cause: "convert_debit".into(),
            },
            StateEffect {
                account: account.clone(),
                asset: to_asset.clone(),
                facet: rule.credit_facet.clone(),
                delta: to_amount,
                cause: "convert_credit".into(),
            },
        ],
        "conversion accepted",
    ))
}

fn authorize_spender(
    world: &EconomicWorld,
    actor: &ActorId,
    account_id: &AccountId,
) -> Option<Decision> {
    let Some(account) = world.accounts.get(account_id) else {
        return None; // caller must treat missing account via require
    };
    if !world.actors.contains(actor) || &account.owner != actor {
        return Some(Decision::rejected(
            "actor not authorized to spend from account",
        ));
    }
    None
}

fn require_account(world: &EconomicWorld, account_id: &AccountId) -> Result<(), KernelError> {
    if world.accounts.contains_key(account_id) {
        Ok(())
    } else {
        Err(KernelError::invalid_input(format!(
            "unknown account {account_id}"
        )))
    }
}

fn require_asset(world: &EconomicWorld, asset_id: &AssetId) -> Result<(), KernelError> {
    if world.assets.contains_key(asset_id) {
        Ok(())
    } else {
        // Action references unknown asset identity → InvalidInput (TASK-03F).
        Err(KernelError::invalid_input(format!(
            "unknown asset {asset_id}"
        )))
    }
}

fn action_summary(action: &Action) -> String {
    match action {
        Action::NoOp { actor, .. } => format!("NoOp actor={actor}"),
        Action::Transfer {
            actor,
            from,
            to,
            asset,
            amount,
            ..
        } => format!("Transfer actor={actor} {from}->{to} {asset}={amount}"),
        Action::TransferWithFee {
            actor,
            from,
            to,
            asset,
            amount,
            fee_amount,
            ..
        } => {
            format!("TransferWithFee actor={actor} {from}->{to} {asset}={amount} fee={fee_amount}")
        }
        Action::Convert {
            actor,
            account,
            from_asset,
            to_asset,
            from_amount,
            ..
        } => {
            format!(
                "Convert actor={actor} account={account} {from_asset}->{to_asset} amount={from_amount}"
            )
        }
    }
}

fn world_summary(world: &EconomicWorld) -> String {
    format!(
        "assets={} accounts={} facets={} atomic={} allow_negative={}",
        world.assets.len(),
        world.accounts.len(),
        world.facets.len(),
        world.atomic,
        world.allow_negative_balances
    )
}
