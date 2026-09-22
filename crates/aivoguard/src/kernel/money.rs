//! Money amounts in Asset-declared minor units (`i128`, checked arithmetic).

use crate::kernel::asset::AssetId;
use crate::kernel::error::ArithError;

/// Authoritative monetary quantity for a single [`AssetId`].
///
/// Representation: signed `i128` minor units (ADR 0001). Floating point is never
/// used. Cross-asset arithmetic is forbidden without explicit conversion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Money {
    asset: AssetId,
    /// Minor-unit integer quantity.
    value: i128,
}

impl Money {
    /// Create money for an asset.
    #[must_use]
    pub fn new(asset: AssetId, value: i128) -> Self {
        Self { asset, value }
    }

    /// Asset identity.
    #[must_use]
    pub fn asset(&self) -> &AssetId {
        &self.asset
    }

    /// Minor-unit quantity.
    #[must_use]
    pub fn value(&self) -> i128 {
        self.value
    }

    /// Checked addition (same asset only).
    ///
    /// # Errors
    ///
    /// Returns [`ArithError`] on asset mismatch or overflow.
    pub fn checked_add(&self, other: &Self) -> Result<Self, ArithError> {
        self.ensure_same_asset(other)?;
        let value = self
            .value
            .checked_add(other.value)
            .ok_or(ArithError::Overflow)?;
        Ok(Self {
            asset: self.asset.clone(),
            value,
        })
    }

    /// Checked subtraction (same asset only).
    ///
    /// # Errors
    ///
    /// Returns [`ArithError`] on asset mismatch or overflow.
    pub fn checked_sub(&self, other: &Self) -> Result<Self, ArithError> {
        self.ensure_same_asset(other)?;
        let value = self
            .value
            .checked_sub(other.value)
            .ok_or(ArithError::Overflow)?;
        Ok(Self {
            asset: self.asset.clone(),
            value,
        })
    }

    /// Checked multiplication by a dimensionless integer factor.
    ///
    /// # Errors
    ///
    /// Returns [`ArithError::Overflow`] on overflow.
    pub fn checked_mul_i128(&self, factor: i128) -> Result<Self, ArithError> {
        let value = self.value.checked_mul(factor).ok_or(ArithError::Overflow)?;
        Ok(Self {
            asset: self.asset.clone(),
            value,
        })
    }

    fn ensure_same_asset(&self, other: &Self) -> Result<(), ArithError> {
        if self.asset == other.asset {
            Ok(())
        } else {
            Err(ArithError::AssetMismatch {
                left: self.asset.clone(),
                right: other.asset.clone(),
            })
        }
    }
}
