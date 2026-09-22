//! M07 engine-pin exact comparison (§14.1).
//!
//! No discovery, defaults, semver ranges, or external queries.

use crate::payment_settlement::error::{PaymentSettlementError, PaymentSettlementErrorId};
use crate::payment_settlement::types::{M07EnginePins, SettlementObservation};

/// Validate optional case-declared engine pins against the observation.
///
/// # Errors
///
/// Returns [`PaymentSettlementErrorId::EnginePinMismatch`] when a declared pin
/// is absent on the observation or differs by exact string compare.
pub fn validate_engine_pins(
    pins: &M07EnginePins,
    obs: &SettlementObservation,
) -> Result<(), PaymentSettlementError> {
    if let Some(expected) = &pins.m01_engine_version {
        let observed = obs
            .simulation_result
            .as_ref()
            .map(|s| s.m01_engine_version.as_str());
        exact_pin("m01_engine_version", expected, observed)?;
    }
    if let Some(expected) = &pins.m02_engine_version {
        let observed = obs
            .simulation_result
            .as_ref()
            .map(|s| s.m02_engine_version.as_str());
        exact_pin("m02_engine_version", expected, observed)?;
    }
    if let Some(expected) = &pins.m03_engine_version {
        let observed = obs
            .simulation_result
            .as_ref()
            .map(|s| s.m03_engine_version.as_str());
        exact_pin("m03_engine_version", expected, observed)?;
    }
    if let Some(expected) = &pins.m07_label {
        exact_pin("m07_label", expected, obs.m07_label.as_deref())?;
    }
    Ok(())
}

fn exact_pin(
    name: &str,
    expected: &str,
    observed: Option<&str>,
) -> Result<(), PaymentSettlementError> {
    match observed {
        Some(v) if v == expected => Ok(()),
        Some(v) => Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::EnginePinMismatch,
            format!("{name} pin mismatch: expected `{expected}`, observed `{v}`"),
        )),
        None => Err(PaymentSettlementError::new(
            PaymentSettlementErrorId::EnginePinMismatch,
            format!("{name} pin declared but observation value is absent"),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::EconomicState;
    use crate::payment_settlement::types::{
        PaymentSettlementBinding, SettlementExecutionState, SettlementObservation, SettlementPhase,
        SettlementStatus,
    };
    use crate::simulator::{ExecutionStatus, ScenarioId, SimulationResult, M03_ENGINE_VERSION};
    use crate::{ENGINE_VERSION as M01_ENGINE_VERSION, M02_ENGINE_VERSION};

    fn obs_with_sim(label: Option<&str>) -> SettlementObservation {
        let state = EconomicState::new();
        SettlementObservation {
            binding: PaymentSettlementBinding {
                payment_id: "pay".into(),
                payment_version: "1".into(),
                configuration_id: "cfg".into(),
            },
            settlement_status: SettlementStatus::Failed,
            execution_state: SettlementExecutionState::Attempted,
            phase: SettlementPhase::Settle,
            refund_outcome: None,
            requested_amount: None,
            settled_amount: None,
            unsettled_amount: None,
            fee_amount: None,
            net_settlement_amount: None,
            refund_amount: None,
            escrow_status: None,
            escrow_amount: None,
            delay_marker: None,
            settlement_delay_steps: None,
            m07_label: label.map(str::to_owned),
            simulation_result: Some(SimulationResult {
                execution_status: ExecutionStatus::NormalCompletion,
                scenario_id: ScenarioId::new("sc"),
                scenario_version: "1".into(),
                configuration_id: "cfg".into(),
                initial_state: state.clone(),
                final_state: state,
                step_records: vec![],
                invariant_evaluation_records: vec![],
                completion_evaluation_records: vec![],
                simulation_errors: vec![],
                executed_action_count: 0,
                m01_engine_version: M01_ENGINE_VERSION.to_owned(),
                m02_engine_version: M02_ENGINE_VERSION.to_owned(),
                m03_engine_version: M03_ENGINE_VERSION.to_owned(),
            }),
            adapter_provenance: None,
            notes: None,
        }
    }

    #[test]
    fn matching_pins_ok() {
        let pins = M07EnginePins {
            m01_engine_version: Some(M01_ENGINE_VERSION.to_owned()),
            m02_engine_version: Some(M02_ENGINE_VERSION.to_owned()),
            m03_engine_version: Some(M03_ENGINE_VERSION.to_owned()),
            m07_label: Some("lab".into()),
        };
        assert!(validate_engine_pins(&pins, &obs_with_sim(Some("lab"))).is_ok());
    }

    #[test]
    fn mismatched_pin_errors() {
        let pins = M07EnginePins {
            m01_engine_version: Some("wrong".into()),
            ..M07EnginePins::default()
        };
        let err = validate_engine_pins(&pins, &obs_with_sim(None)).unwrap_err();
        assert_eq!(err.error_id, PaymentSettlementErrorId::EnginePinMismatch);
    }

    #[test]
    fn absent_optional_pins_are_not_required() {
        let pins = M07EnginePins::default();
        assert!(validate_engine_pins(&pins, &obs_with_sim(None)).is_ok());
    }

    #[test]
    fn declared_m07_label_absent_on_observation_errors() {
        let pins = M07EnginePins {
            m07_label: Some("lab".into()),
            ..M07EnginePins::default()
        };
        let err = validate_engine_pins(&pins, &obs_with_sim(None)).unwrap_err();
        assert_eq!(err.error_id, PaymentSettlementErrorId::EnginePinMismatch);
    }
}
