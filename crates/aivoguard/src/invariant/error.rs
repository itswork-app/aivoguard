//! M02 ERROR classification (exactly eight conceptual classes).

/// Conceptual M02 ERROR class (Gate-2 frozen taxonomy).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InvariantErrorClass {
    /// Malformed or structurally invalid invariant / reference / binding.
    InvalidInvariantDefinition,
    /// Well-formed configuration inconsistent with declared World.
    InvalidInvariantConfiguration,
    /// Evaluation target type/scope incompatible with the invariant.
    IncompatibleTarget,
    /// Compatible target, but required authoritative data is absent.
    MissingRequiredData,
    /// Operands exist but cannot legally be operated on under declared rules.
    IncompatibleOperands,
    /// Well-formed operation not supported by Gate-2 semantics.
    UnsupportedOperation,
    /// Checked arithmetic failure or invalid numeric domain.
    ArithmeticError,
    /// Unexpected internal engine failure.
    EngineError,
}

impl InvariantErrorClass {
    /// Precedence rank (lower = higher precedence).
    #[must_use]
    pub const fn precedence(self) -> u8 {
        match self {
            Self::InvalidInvariantDefinition => 0,
            Self::InvalidInvariantConfiguration => 1,
            Self::IncompatibleTarget => 2,
            Self::MissingRequiredData => 3,
            Self::IncompatibleOperands => 4,
            Self::UnsupportedOperation => 5,
            Self::ArithmeticError => 6,
            Self::EngineError => 7,
        }
    }

    /// Select the higher-precedence class among two discovered causes.
    #[must_use]
    pub fn prefer(self, other: Self) -> Self {
        if self.precedence() <= other.precedence() {
            self
        } else {
            other
        }
    }
}

/// Typed M02 evaluation error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantError {
    /// Error classification.
    pub class: InvariantErrorClass,
    /// Deterministic reason string.
    pub reason: String,
}

impl InvariantError {
    /// Construct an error of the given class.
    #[must_use]
    pub fn new(class: InvariantErrorClass, reason: impl Into<String>) -> Self {
        Self {
            class,
            reason: reason.into(),
        }
    }

    /// Prefer this error or `other` by Gate-2 precedence.
    #[must_use]
    pub fn prefer(self, other: Self) -> Self {
        if self.class.precedence() <= other.class.precedence() {
            self
        } else {
            other
        }
    }
}
