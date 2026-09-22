//! Action target resolution — ActionId / INDEX (§10.3 / R-16).

use crate::adversarial::token::IdentifierToken;
use crate::adversarial::types::AddressingMode;
use crate::kernel::action::{Action, ActionId};

/// Result of resolving an Action target against the current intermediate sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetResolution {
    /// Addressing mode used.
    pub mode: AddressingMode,
    /// Requested ActionId string (when mode is ActionId).
    pub requested_id: Option<IdentifierToken>,
    /// Requested index (when mode is Index).
    pub requested_index: Option<usize>,
    /// Match count `N`.
    pub match_count: usize,
    /// Resolved position when `N == 1`.
    pub resolved_position: Option<usize>,
}

/// Classification of a resolution attempt for applicability / error mapping.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolutionClass {
    /// Unique match (`N = 1`).
    Resolved {
        /// Zero-based position in the current sequence.
        position: usize,
        /// Provenance record.
        record: TargetResolution,
    },
    /// Missing reference (`N = 0` for ActionId, or index out of range).
    Missing {
        /// Provenance record.
        record: TargetResolution,
    },
    /// Ambiguous ActionId (`N > 1`) → parameter error.
    Ambiguous {
        /// Provenance record.
        record: TargetResolution,
    },
    /// Malformed parameter (empty / non-token id).
    MalformedParameter {
        /// Deterministic reason.
        reason: String,
    },
}

/// Resolve by ActionId against the current intermediate sequence.
#[must_use]
pub fn resolve_action_id(actions: &[Action], id: &str) -> ResolutionClass {
    if !IdentifierToken::is_valid(id) {
        return ResolutionClass::MalformedParameter {
            reason: "ActionId parameter is empty or not an IdentifierToken".into(),
        };
    }
    let token = IdentifierToken::new_unchecked(id);
    let mut positions = Vec::new();
    for (i, action) in actions.iter().enumerate() {
        if let Some(aid) = action.action_id() {
            if aid.as_str() == id {
                positions.push(i);
            }
        }
    }
    let match_count = positions.len();
    let record = TargetResolution {
        mode: AddressingMode::ActionId,
        requested_id: Some(token),
        requested_index: None,
        match_count,
        resolved_position: positions.first().copied().filter(|_| match_count == 1),
    };
    match match_count {
        0 => ResolutionClass::Missing { record },
        1 => ResolutionClass::Resolved {
            position: positions[0],
            record,
        },
        _ => ResolutionClass::Ambiguous { record },
    }
}

/// Resolve by zero-based positional index against the current intermediate sequence.
#[must_use]
pub fn resolve_action_index(actions: &[Action], index: usize) -> ResolutionClass {
    let record = TargetResolution {
        mode: AddressingMode::Index,
        requested_id: None,
        requested_index: Some(index),
        match_count: usize::from(index < actions.len()),
        resolved_position: (index < actions.len()).then_some(index),
    };
    if index < actions.len() {
        ResolutionClass::Resolved {
            position: index,
            record,
        }
    } else {
        ResolutionClass::Missing { record }
    }
}

/// Action target declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionTarget {
    /// Address by ActionId.
    ById(IdentifierToken),
    /// Address by zero-based index.
    ByIndex(usize),
}

impl ActionTarget {
    /// Resolve against the current sequence.
    #[must_use]
    pub fn resolve(&self, actions: &[Action]) -> ResolutionClass {
        match self {
            Self::ById(id) => resolve_action_id(actions, id.as_str()),
            Self::ByIndex(i) => resolve_action_index(actions, *i),
        }
    }
}

/// Compare an ActionId to a token string by exact IdentifierToken rules.
#[must_use]
pub fn action_id_equals(action_id: &ActionId, token: &str) -> bool {
    action_id.as_str() == token
}
