//! Checked absolute-tolerance comparison (§11.3).

use crate::regression::error::{RegressionError, RegressionErrorId};
use crate::regression::types::StructuredValue;

/// Result of absolute-tolerance comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToleranceCompare {
    /// `|observed - expected| <= tolerance`.
    Satisfied,
    /// Outside tolerance.
    Unsatisfied {
        /// Absolute difference as u128.
        absolute_difference_u128: u128,
    },
}

/// Normative absolute-tolerance procedure.
pub fn compare_absolute_tolerance(
    observed: i128,
    expected: i128,
    tolerance: u128,
) -> Result<ToleranceCompare, RegressionError> {
    let difference = observed.checked_sub(expected).ok_or_else(|| {
        RegressionError::new(
            RegressionErrorId::NumericComparisonError,
            "checked_sub overflow",
        )
        .with_field("observed", StructuredValue::I128(observed))
        .with_field("expected", StructuredValue::I128(expected))
    })?;

    let absolute_difference = checked_abs_i128(difference).ok_or_else(|| {
        RegressionError::new(
            RegressionErrorId::NumericComparisonError,
            "checked_abs overflow",
        )
        .with_field("difference", StructuredValue::I128(difference))
    })?;

    let absolute_difference_u128 = i128_to_u128_checked(absolute_difference).ok_or_else(|| {
        RegressionError::new(
            RegressionErrorId::NumericComparisonError,
            "checked i128→u128 conversion failed",
        )
        .with_field(
            "absolute_difference",
            StructuredValue::I128(absolute_difference),
        )
    })?;

    if absolute_difference_u128 <= tolerance {
        Ok(ToleranceCompare::Satisfied)
    } else {
        Ok(ToleranceCompare::Unsatisfied {
            absolute_difference_u128,
        })
    }
}

fn checked_abs_i128(value: i128) -> Option<i128> {
    if value == i128::MIN {
        None
    } else if value < 0 {
        Some(-value)
    } else {
        Some(value)
    }
}

fn i128_to_u128_checked(value: i128) -> Option<u128> {
    if value < 0 {
        None
    } else {
        Some(value as u128)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tolerance_boundary_inclusive() {
        let r = compare_absolute_tolerance(10, 7, 3).unwrap();
        assert_eq!(r, ToleranceCompare::Satisfied);
    }

    #[test]
    fn opposite_extremes_error() {
        let err = compare_absolute_tolerance(i128::MAX, i128::MIN, 0).unwrap_err();
        assert_eq!(err.error_id, RegressionErrorId::NumericComparisonError);
    }

    #[test]
    fn abs_min_difference_errors() {
        // difference = i128::MIN cannot be abs'd
        let err = compare_absolute_tolerance(i128::MIN, 0, 0).unwrap_err();
        assert_eq!(err.error_id, RegressionErrorId::NumericComparisonError);
    }
}
