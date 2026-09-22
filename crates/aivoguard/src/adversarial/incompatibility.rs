//! Incompatibility matching — occurrence + EXACT_PAIR semantics (§10.4 / R-19).

use crate::adversarial::token::IdentifierToken;
use crate::adversarial::types::{
    IncompatibilityEvidence, IncompatibilityRule, PlanOccurrence, PositionConstraint,
};

/// Build plan occurrences from parallel identity/version vectors.
///
/// Repeated identities remain distinct occurrences (position-based).
#[must_use]
pub fn plan_occurrences(
    identities: &[IdentifierToken],
    versions: &[IdentifierToken],
) -> Result<Vec<PlanOccurrence>, &'static str> {
    if identities.len() != versions.len() {
        return Err("composition plan identity/version length mismatch");
    }
    Ok(identities
        .iter()
        .zip(versions.iter())
        .enumerate()
        .map(|(index, (identity, version))| PlanOccurrence {
            index,
            identity: identity.clone(),
            version: version.clone(),
        })
        .collect())
}

fn side_matches(
    occurrence: &PlanOccurrence,
    identity: &IdentifierToken,
    version: Option<&IdentifierToken>,
) -> bool {
    if !occurrence.identity.eq(identity) {
        return false;
    }
    match version {
        None => true,
        Some(v) => occurrence.version.eq(v),
    }
}

fn pair_satisfies(
    occurrences: &[PlanOccurrence],
    rule: &IncompatibilityRule,
    left_index: usize,
    right_index: usize,
) -> bool {
    if left_index == right_index {
        return false;
    }
    let Some(left) = occurrences.get(left_index) else {
        return false;
    };
    let Some(right) = occurrences.get(right_index) else {
        return false;
    };
    side_matches(
        left,
        &rule.left_transformation_identity,
        rule.left_version.as_ref(),
    ) && side_matches(
        right,
        &rule.right_transformation_identity,
        rule.right_version.as_ref(),
    )
}

/// Find the first matching incompatibility rule under §10.4.
///
/// Rules scanned in declared vector order. First match wins.
/// Unconstrained pairs enumerated `(left ASC, right ASC)` with `left != right`.
/// `EXACT_PAIR(a,b)` evaluates only `(a,b)` — never `(b,a)`.
#[must_use]
pub fn find_first_incompatibility(
    occurrences: &[PlanOccurrence],
    rules: &[IncompatibilityRule],
) -> Option<IncompatibilityEvidence> {
    for rule in rules {
        if let Some(evidence) = match_rule(occurrences, rule) {
            return Some(evidence);
        }
    }
    None
}

fn match_rule(
    occurrences: &[PlanOccurrence],
    rule: &IncompatibilityRule,
) -> Option<IncompatibilityEvidence> {
    match rule.position_constraint {
        Some(PositionConstraint::ExactPair {
            left_index,
            right_index,
        }) => {
            if pair_satisfies(occurrences, rule, left_index, right_index) {
                Some(evidence_from(rule, left_index, right_index))
            } else {
                None
            }
        }
        None => {
            let n = occurrences.len();
            for left_index in 0..n {
                for right_index in 0..n {
                    if left_index == right_index {
                        continue;
                    }
                    if pair_satisfies(occurrences, rule, left_index, right_index) {
                        return Some(evidence_from(rule, left_index, right_index));
                    }
                }
            }
            None
        }
    }
}

fn evidence_from(
    rule: &IncompatibilityRule,
    left_plan_index: usize,
    right_plan_index: usize,
) -> IncompatibilityEvidence {
    IncompatibilityEvidence {
        rule_id: rule.rule_id.clone(),
        left_transformation_identity: rule.left_transformation_identity.clone(),
        right_transformation_identity: rule.right_transformation_identity.clone(),
        left_version: rule.left_version.clone(),
        right_version: rule.right_version.clone(),
        left_plan_index,
        right_plan_index,
        position_constraint: rule.position_constraint,
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::adversarial::token::IdentifierToken;

    fn tok(s: &str) -> IdentifierToken {
        IdentifierToken::try_new(s).expect("token")
    }

    #[test]
    fn exact_pair_does_not_match_reverse() {
        let plan = plan_occurrences(&[tok("A"), tok("B")], &[tok("1"), tok("1")]).unwrap();
        let rule = IncompatibilityRule {
            rule_id: tok("r1"),
            left_transformation_identity: tok("A"),
            right_transformation_identity: tok("B"),
            left_version: None,
            right_version: None,
            position_constraint: Some(PositionConstraint::ExactPair {
                left_index: 1,
                right_index: 0,
            }),
        };
        // EXACT_PAIR(1,0) requires left@1=A and right@0=B — fails.
        assert!(find_first_incompatibility(&plan, &[rule]).is_none());
    }
}
