//! Rounding modes matching the frozen Gate-1 set.

use crate::kernel::error::KernelError;

/// Explicit rounding modes (no silent default).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundingMode {
    /// Truncate toward zero.
    TowardsZero,
    /// Round away from zero.
    AwayFromZero,
    /// Mathematical floor.
    Floor,
    /// Mathematical ceil.
    Ceil,
    /// Ties round away from zero.
    HalfAwayFromZero,
}

/// Round `numerator / denominator` into an integer quotient using `mode`.
///
/// Used when a calculation cannot land exactly on a target minor unit.
///
/// # Errors
///
/// Returns [`KernelError`] when `denominator == 0` or intermediate overflow occurs.
pub fn divide_with_rounding(
    numerator: i128,
    denominator: i128,
    mode: RoundingMode,
) -> Result<i128, KernelError> {
    if denominator == 0 {
        return Err(KernelError::invalid_input(
            "division by zero in rounding context",
        ));
    }

    let quot = numerator / denominator;
    let rem = numerator % denominator;
    if rem == 0 {
        return Ok(quot);
    }

    let abs_den = denominator
        .checked_abs()
        .ok_or_else(|| KernelError::engine("absolute value overflow for i128::MIN denominator"))?;
    let abs_rem = rem
        .checked_abs()
        .ok_or_else(|| KernelError::engine("absolute value overflow for remainder"))?;

    let round_away = match mode {
        RoundingMode::TowardsZero => false,
        RoundingMode::AwayFromZero => true,
        RoundingMode::Floor => numerator.signum() * denominator.signum() < 0,
        RoundingMode::Ceil => numerator.signum() * denominator.signum() > 0,
        RoundingMode::HalfAwayFromZero => {
            // Compare 2*|rem| with |den|; tie when equal → away from zero.
            let twice = abs_rem
                .checked_mul(2)
                .ok_or_else(|| KernelError::engine("overflow comparing half-away remainder"))?;
            twice >= abs_den
        }
    };

    if !round_away {
        return Ok(quot);
    }

    let step = if (numerator < 0) ^ (denominator < 0) {
        -1
    } else {
        1
    };
    quot.checked_add(step)
        .ok_or_else(|| KernelError::engine("overflow applying rounding step"))
}
