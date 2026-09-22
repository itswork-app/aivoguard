//! M06 Economic Regression Engine — declared expectation comparison.
//!
//! Implements the frozen Gate-5 Economic Regression Engine Specification
//! (`ca4c88a`). This module does **not** calculate economic truth (M01),
//! evaluate invariants (M02), execute simulations (M03), or generate
//! adversarial scenarios (M04).
//!
//! Open decisions AD-03 / AD-12 / AD-14 / AD-15 and M06-OD-01/02/03 remain OPEN.
//! Concrete Rust types here are provisional implementation surfaces (AD-15).

#![allow(clippy::module_name_repetitions)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::needless_pass_by_value)]

mod engine;
mod error;
mod numeric;
mod types;

pub use engine::evaluate_regression;
pub use error::{RegressionError, RegressionErrorId};
pub use numeric::{compare_absolute_tolerance, ToleranceCompare};
pub use types::{
    ComparisonOperator, ENGINE_VERSION as M06_ENGINE_VERSION, EnginePins, Expectation,
    ExpectationClass, ExpectationPhase, ExpectedDisposition, ExpectedExecutionStatus,
    ExpectedFatalCauseClass, MismatchClass, MismatchEvidence, RegressionCase,
    RegressionObservation, RegressionProvenance, RegressionResult, RegressionVerdict,
    ScenarioBinding, StructuredValue,
};
