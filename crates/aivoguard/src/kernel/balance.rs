//! Balance facet identifiers and World-declared facet models.

use std::fmt;

/// Balance facet identity (e.g. available, reserved, pending, settled).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FacetId(String);

impl FacetId {
    /// Create a facet identity.
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

impl fmt::Display for FacetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// World-declared balance facet architecture (EK-BAL-01).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BalanceFacetModel {
    /// Facets are mutually exclusive buckets of one quantity.
    MutuallyExclusiveBuckets,
    /// Facets are orthogonal dimensions.
    OrthogonalDimensions,
}
