//! Case-level structural validation (frozen §7.3 / §8.1–§8.2 / R-G3 / R-F2).

use crate::payment_settlement::error::{PaymentSettlementError, PaymentSettlementErrorId};
use crate::payment_settlement::types::{
    FeeMode, PaymentScenarioBinding, PaymentSettlementCase, SettlementExpectation,
    SettlementExpectationClass, SettlementObservation, SettlementStatus,
};
use crate::simulator::SimulationResult;

/// Validate case structure before observation evaluation (§14 step 1 subset).
///
/// Order (deterministic):
/// 1. non-empty case identity
/// 2. declaration amount domain
/// 3. fee declaration structure / InvalidAmount rules
/// 4. terminal_partial static contradiction (§7.3)
/// 5. expectation ids / closed payloads / R-G3
/// 6. R-F2 NetSettlementExact vs derived net (FeeExclusive/Inclusive)
///
/// # Errors
///
/// Returns the first violated frozen case-validation rule.
pub fn validate_payment_settlement_case(
    case: &PaymentSettlementCase,
) -> Result<(), PaymentSettlementError> {
    if case.case_id.is_empty() || case.case_version.is_empty() {
        return Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidPaymentCase,
            "case_id and case_version must be non-empty",
        ));
    }

    validate_declaration_amount_domain(case)?;
    validate_fee_declaration(case)?;
    validate_terminal_partial_contradiction(case)?;
    validate_expectation_set(&case.expectations)?;
    validate_net_settlement_derived_expectation(case)?;
    Ok(())
}

/// Validate expectation identities and payload domains (R-G3).
///
/// # Errors
///
/// Returns [`PaymentSettlementErrorId::InvalidPaymentCase`] on duplicate/empty ids
/// or negative expected amounts on `>= 0` payloads.
pub fn validate_expectation_set(
    expectations: &[SettlementExpectation],
) -> Result<(), PaymentSettlementError> {
    let mut seen: Vec<&str> = Vec::new();
    for expectation in expectations {
        if expectation.expectation_id.is_empty() {
            return Err(PaymentSettlementError::new(
                PaymentSettlementErrorId::InvalidPaymentCase,
                "expectation_id must be non-empty",
            ));
        }
        if seen.contains(&expectation.expectation_id.as_str()) {
            return Err(PaymentSettlementError::new(
                PaymentSettlementErrorId::InvalidPaymentCase,
                format!("duplicate expectation_id `{}`", expectation.expectation_id),
            ));
        }
        seen.push(expectation.expectation_id.as_str());
        validate_expectation_payload_domain(&expectation.class)?;
    }
    Ok(())
}

fn validate_expectation_payload_domain(
    class: &SettlementExpectationClass,
) -> Result<(), PaymentSettlementError> {
    match class {
        SettlementExpectationClass::SettledAmountExact {
            expected_settled_amount,
        }
        | SettlementExpectationClass::SettledAmountAbsoluteTolerance {
            expected_settled_amount,
            ..
        } if *expected_settled_amount < 0 => Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidPaymentCase,
            "expected_settled_amount must be >= 0",
        )),
        SettlementExpectationClass::FeeExact {
            expected_fee_amount,
        } if *expected_fee_amount < 0 => Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidPaymentCase,
            "expected_fee_amount must be >= 0",
        )),
        SettlementExpectationClass::RefundAmountExact {
            expected_refund_amount,
        } if *expected_refund_amount < 0 => Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidPaymentCase,
            "expected_refund_amount must be >= 0",
        )),
        SettlementExpectationClass::SettledAmountExact { .. }
        | SettlementExpectationClass::SettledAmountAbsoluteTolerance { .. }
        | SettlementExpectationClass::FeeExact { .. }
        | SettlementExpectationClass::RefundAmountExact { .. }
        | SettlementExpectationClass::SettlementStatusExact { .. }
        | SettlementExpectationClass::NetSettlementExact { .. }
        | SettlementExpectationClass::RefundStatusExact { .. }
        | SettlementExpectationClass::ReversalStatusExact
        | SettlementExpectationClass::EscrowStatusExact { .. }
        | SettlementExpectationClass::SettlementExecutionExact { .. } => Ok(()),
    }
}

fn validate_declaration_amount_domain(
    case: &PaymentSettlementCase,
) -> Result<(), PaymentSettlementError> {
    if case.payment_declaration.gross_amount < 0 {
        return Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidAmount,
            "payment_declaration.gross_amount must be >= 0",
        ));
    }
    if case.settlement_declaration.requested_amount < 0 {
        return Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidAmount,
            "settlement_declaration.requested_amount must be >= 0",
        ));
    }
    Ok(())
}

fn validate_fee_declaration(case: &PaymentSettlementCase) -> Result<(), PaymentSettlementError> {
    let Some(fee) = &case.payment_declaration.fee else {
        return Ok(());
    };

    // Rust struct fields are always present when FeeDeclaration exists; empty
    // fee_asset is treated as absence of the required token.
    if fee.fee_asset.is_empty() {
        return Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidPaymentCase,
            "fee = Some requires non-empty fee_asset",
        ));
    }

    if fee.fee_amount < 0 {
        return Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidAmount,
            "fee_amount must be >= 0",
        ));
    }

    if fee.fee_asset != case.payment_declaration.asset_id {
        return Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::InvalidAmount,
            "fee_asset must equal payment_declaration.asset_id",
        ));
    }

    match fee.fee_mode {
        FeeMode::FeeExclusive | FeeMode::FeeInclusive => {
            if fee.fee_amount > case.payment_declaration.gross_amount {
                return Err(PaymentSettlementError::new(
                    PaymentSettlementErrorId::InvalidAmount,
                    "fee_amount must be <= gross_amount for FeeExclusive/FeeInclusive",
                ));
            }
            // Ensure derived net is representable.
            if case
                .payment_declaration
                .gross_amount
                .checked_sub(fee.fee_amount)
                .is_none()
            {
                return Err(PaymentSettlementError::new(
                    PaymentSettlementErrorId::NumericComparisonError,
                    "checked_sub(gross_amount, fee_amount) failed",
                ));
            }
        }
        FeeMode::FeeDeclaredOnly => {}
    }

    Ok(())
}

fn validate_terminal_partial_contradiction(
    case: &PaymentSettlementCase,
) -> Result<(), PaymentSettlementError> {
    if case.settlement_declaration.terminal_partial != Some(true) {
        return Ok(());
    }
    for expectation in &case.expectations {
        if !expectation.applicable {
            continue;
        }
        if matches!(
            &expectation.class,
            SettlementExpectationClass::SettlementStatusExact {
                expected_status: SettlementStatus::Settled,
            }
        ) {
            return Err(PaymentSettlementError::new(
                PaymentSettlementErrorId::InvalidSettlementCase,
                format!(
                    "terminal_partial=Some(true) contradicts applicable SettlementStatusExact(Settled) (`{}`)",
                    expectation.expectation_id
                ),
            ));
        }
    }
    Ok(())
}

fn validate_net_settlement_derived_expectation(
    case: &PaymentSettlementCase,
) -> Result<(), PaymentSettlementError> {
    let Some(fee) = &case.payment_declaration.fee else {
        return Ok(());
    };
    if !matches!(fee.fee_mode, FeeMode::FeeExclusive | FeeMode::FeeInclusive) {
        return Ok(());
    }

    let derived = case
        .payment_declaration
        .gross_amount
        .checked_sub(fee.fee_amount)
        .ok_or_else(|| {
            PaymentSettlementError::new(
                PaymentSettlementErrorId::NumericComparisonError,
                "checked_sub(gross_amount, fee_amount) failed for R-F2",
            )
        })?;

    for expectation in &case.expectations {
        if !expectation.applicable {
            continue;
        }
        if let SettlementExpectationClass::NetSettlementExact {
            expected_net_settlement_amount,
        } = &expectation.class
        {
            if *expected_net_settlement_amount != derived {
                return Err(PaymentSettlementError::new(
                    PaymentSettlementErrorId::InvalidPaymentCase,
                    format!(
                        "NetSettlementExact expected ({expected_net_settlement_amount}) ≠ derived_net ({derived})"
                    ),
                ));
            }
        }
    }
    Ok(())
}

/// Optional scenario binding vs SimulationResult identity (§16).
///
/// Compared only when both `scenario_binding` and `simulation_result` are present.
///
/// # Errors
///
/// Returns [`PaymentSettlementErrorId::ObservationBindingMismatch`] on identity mismatch.
pub fn validate_scenario_binding(
    binding: &PaymentScenarioBinding,
    simulation: &SimulationResult,
) -> Result<(), PaymentSettlementError> {
    if binding.scenario_id == simulation.scenario_id.as_str()
        && binding.scenario_version == simulation.scenario_version
        && binding.configuration_id == simulation.configuration_id
    {
        Ok(())
    } else {
        Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::ObservationBindingMismatch,
            format!(
                "scenario_binding ≠ simulation_result identity: binding=`{}`/`{}`/`{}` sim=`{}`/`{}`/`{}`",
                binding.scenario_id,
                binding.scenario_version,
                binding.configuration_id,
                simulation.scenario_id.as_str(),
                simulation.scenario_version,
                simulation.configuration_id
            ),
        ))
    }
}

/// Convenience: validate scenario binding when both sides are present on a case.
pub fn validate_case_scenario_binding(
    case: &PaymentSettlementCase,
    obs: &SettlementObservation,
) -> Result<(), PaymentSettlementError> {
    match (&case.scenario_binding, &obs.simulation_result) {
        (Some(binding), Some(sim)) => validate_scenario_binding(binding, sim),
        _ => Ok(()),
    }
}
