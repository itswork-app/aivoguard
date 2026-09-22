//! Kernel error and economic disposition taxonomy (frozen EK-TX-01).

use crate::kernel::asset::AssetId;

/// Non-economic kernel failure classes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelErrorKind {
    /// Malformed or incomplete inputs relative to the kernel/World contract.
    InvalidInput,
    /// World/State configuration insufficient or inconsistent for evaluation.
    ConfigurationError,
    /// Kernel cannot correctly complete evaluation (overflow, internal fault).
    EngineError,
}

/// Typed non-economic kernel error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelError {
    /// Error classification.
    pub kind: KernelErrorKind,
    /// Deterministic human-readable reason (not host tracing).
    pub reason: String,
}

impl KernelError {
    /// Construct an [`InvalidInput`](KernelErrorKind::InvalidInput) error.
    #[must_use]
    pub fn invalid_input(reason: impl Into<String>) -> Self {
        Self {
            kind: KernelErrorKind::InvalidInput,
            reason: reason.into(),
        }
    }

    /// Construct a [`ConfigurationError`](KernelErrorKind::ConfigurationError).
    #[must_use]
    pub fn configuration(reason: impl Into<String>) -> Self {
        Self {
            kind: KernelErrorKind::ConfigurationError,
            reason: reason.into(),
        }
    }

    /// Construct an [`EngineError`](KernelErrorKind::EngineError).
    #[must_use]
    pub fn engine(reason: impl Into<String>) -> Self {
        Self {
            kind: KernelErrorKind::EngineError,
            reason: reason.into(),
        }
    }
}

/// Economic disposition for a successfully classified evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EconomicDisposition {
    /// Action succeeds; declared effects are accepted (may be empty).
    AcceptedEffective,
    /// Action cannot proceed under eligibility/permission rules.
    Rejected,
    /// Action was eligible but economic conditions prevented success.
    FailedEconomic,
}

/// Internal arithmetic failure mapped to [`KernelErrorKind::EngineError`] or
/// [`KernelErrorKind::InvalidInput`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithError {
    /// Checked integer overflow/underflow.
    Overflow,
    /// Attempted same-operation across distinct assets.
    AssetMismatch {
        /// Left-hand asset.
        left: AssetId,
        /// Right-hand asset.
        right: AssetId,
    },
}

impl From<ArithError> for KernelError {
    fn from(value: ArithError) -> Self {
        match value {
            ArithError::Overflow => Self::engine("checked integer arithmetic overflow"),
            ArithError::AssetMismatch { left, right } => Self::invalid_input(format!(
                "cross-asset arithmetic forbidden without conversion: {left} vs {right}"
            )),
        }
    }
}
