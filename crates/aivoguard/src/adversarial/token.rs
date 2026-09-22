//! IdentifierToken — exact UTF-8 identity token (§8.5.3 / R-20).

/// Normative IdentifierToken: UTF-8 string with length ≥ 1.
///
/// Equality is exact, case-sensitive, with no trim / Unicode normalization /
/// case-folding / fuzzy matching. Empty is invalid.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdentifierToken(String);

impl IdentifierToken {
    /// Validate and construct an IdentifierToken.
    ///
    /// Returns `None` when `s` is empty (length &lt; 1).
    #[must_use]
    pub fn try_new(s: impl Into<String>) -> Option<Self> {
        let s = s.into();
        if s.is_empty() {
            None
        } else {
            Some(Self(s))
        }
    }

    /// Construct without validation (caller guarantees length ≥ 1).
    ///
    /// Prefer [`Self::try_new`] at trust boundaries.
    #[must_use]
    pub fn new_unchecked(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Whether `s` is a valid IdentifierToken.
    #[must_use]
    pub fn is_valid(s: &str) -> bool {
        !s.is_empty()
    }

    /// Exact equality (case-sensitive; no normalization).
    #[must_use]
    pub fn equals(&self, other: &str) -> bool {
        self.0 == other
    }

    /// Borrow the underlying string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Owned string clone.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for IdentifierToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
