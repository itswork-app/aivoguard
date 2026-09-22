//! Asset identity and unit semantics.

use std::fmt;

/// Stable asset identity within a World.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssetId(String);

impl AssetId {
    /// Create an asset identity.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Borrow the identity string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AssetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Explicit economic instrument with declared minor-unit scale.
///
/// `scale` means amounts are integer quantities of `10^(-scale)` major units,
/// or an equivalent declared minor-unit system (EK-NUM-01). This does **not**
/// require conventional decimal currencies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asset {
    /// Asset identity.
    pub id: AssetId,
    /// Non-negative scale / minor-unit exponent.
    pub scale: u32,
}

impl Asset {
    /// Construct an asset definition.
    #[must_use]
    pub fn new(id: impl Into<String>, scale: u32) -> Self {
        Self {
            id: AssetId::new(id),
            scale,
        }
    }
}
