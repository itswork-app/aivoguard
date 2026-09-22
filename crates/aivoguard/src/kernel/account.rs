//! Actors and accounts.

use std::fmt;

/// Economically relevant participant identity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActorId(String);

impl ActorId {
    /// Create an actor identity.
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

impl fmt::Display for ActorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Account identity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccountId(String);

impl AccountId {
    /// Create an account identity.
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

impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Account owned/controlled by an actor under World rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    /// Account identity.
    pub id: AccountId,
    /// Controlling actor.
    pub owner: ActorId,
}

impl Account {
    /// Construct an account.
    #[must_use]
    pub fn new(id: impl Into<String>, owner: ActorId) -> Self {
        Self {
            id: AccountId::new(id),
            owner,
        }
    }
}
