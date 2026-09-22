//! Dimensionless cardinality for M02 COUNT (not economic Money).

/// Exact dimensionless count value (`i128`, checked arithmetic only).
///
/// This is **not** [`crate::Money`] and must never be registered as an Asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Count(i128);

impl Count {
    /// Construct a count.
    #[must_use]
    pub const fn new(value: i128) -> Self {
        Self(value)
    }

    /// Borrow the integer cardinality.
    #[must_use]
    pub const fn value(self) -> i128 {
        self.0
    }

    /// Checked addition.
    ///
    /// # Errors
    ///
    /// Returns `None` on overflow.
    #[must_use]
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match self.0.checked_add(other.0) {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }
}
