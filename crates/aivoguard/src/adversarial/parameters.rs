//! Parameter candidate enumeration (§17A).

use crate::adversarial::error::{AdversarialError, AdversarialErrorClass};
use crate::adversarial::token::IdentifierToken;
use crate::adversarial::types::{
    CandidateOperator, ParameterDimension, ParameterDomain, ParameterType, ParameterValue,
};

/// One evaluated parameter tuple (ordered by dimension declaration order).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterTuple {
    /// Bindings in dimension order: (dimension_id, value).
    pub bindings: Vec<(IdentifierToken, ParameterValue)>,
}

/// Enumerate parameter candidate tuples in lexicographic dimension order.
///
/// Returns an error when a numeric operator leaves the domain / overflows
/// (classified as domain construction failure for that candidate — skipped as
/// non-applicable candidate slots are not invented; invalid domain → error).
pub fn enumerate_parameter_candidates(
    domain: &ParameterDomain,
) -> Result<Vec<ParameterTuple>, AdversarialError> {
    if domain.dimensions.is_empty() {
        return Ok(vec![ParameterTuple {
            bindings: Vec::new(),
        }]);
    }

    let mut per_dim: Vec<Vec<(IdentifierToken, ParameterValue)>> = Vec::new();
    for dim in &domain.dimensions {
        let values = dimension_values(dim)?;
        if values.is_empty() {
            return Err(AdversarialError::new(
                AdversarialErrorClass::InvalidAdversarialDefinition,
                format!(
                    "parameter dimension {} produced zero candidates",
                    dim.id.as_str()
                ),
            ));
        }
        per_dim.push(values.into_iter().map(|v| (dim.id.clone(), v)).collect());
    }

    Ok(cartesian_lex(&per_dim))
}

fn dimension_values(dim: &ParameterDimension) -> Result<Vec<ParameterValue>, AdversarialError> {
    match dim.value_type {
        ParameterType::Integer | ParameterType::EconomicAmount => {
            if dim.operators.is_empty() {
                return Ok(sort_numeric_values(&dim.explicit_values, dim.value_type));
            }
            let mut out = Vec::new();
            for op in &dim.operators {
                match apply_numeric_op(dim, *op)? {
                    Some(v) => out.push(v),
                    None => {
                        // Operator left domain / overflow → skip that candidate (NON_APPLICABLE
                        // at evaluation time). Do not invent substitutes.
                    }
                }
            }
            Ok(sort_numeric_owned(out, dim.value_type))
        }
        ParameterType::Index => {
            if dim.operators.is_empty() {
                let mut vals = dim.explicit_values.clone();
                vals.sort_by_key(|v| match v {
                    ParameterValue::Index(i) => *i,
                    _ => usize::MAX,
                });
                return Ok(vals);
            }
            let mut out = Vec::new();
            for op in &dim.operators {
                if let Some(ParameterValue::Integer(n)) = apply_numeric_op(dim, *op)? {
                    if n < 0 {
                        continue;
                    }
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    let idx = n as usize;
                    out.push(ParameterValue::Index(idx));
                }
            }
            out.sort_by_key(|v| match v {
                ParameterValue::Index(i) => *i,
                _ => usize::MAX,
            });
            Ok(out)
        }
        ParameterType::Boolean => {
            if dim.explicit_values.is_empty() {
                Ok(vec![
                    ParameterValue::Boolean(false),
                    ParameterValue::Boolean(true),
                ])
            } else {
                Ok(dim.explicit_values.clone())
            }
        }
        ParameterType::Identifier | ParameterType::Enum | ParameterType::OrderingSelection => {
            Ok(dim.explicit_values.clone())
        }
    }
}

fn apply_numeric_op(
    dim: &ParameterDimension,
    op: CandidateOperator,
) -> Result<Option<ParameterValue>, AdversarialError> {
    let lo = dim.lo;
    let hi = dim.hi;
    let mk = |n: i128| -> ParameterValue {
        match dim.value_type {
            ParameterType::EconomicAmount => ParameterValue::EconomicAmount(n),
            _ => ParameterValue::Integer(n),
        }
    };
    let in_domain = |n: i128| -> bool {
        if let Some(lo) = lo {
            if n < lo {
                return false;
            }
        }
        if let Some(hi) = hi {
            if n > hi {
                return false;
            }
        }
        true
    };

    match op {
        CandidateOperator::Min => {
            let Some(lo) = lo else {
                return Err(AdversarialError::new(
                    AdversarialErrorClass::InvalidAdversarialDefinition,
                    format!("MIN requires explicit lo on dimension {}", dim.id.as_str()),
                ));
            };
            Ok(Some(mk(lo)))
        }
        CandidateOperator::Max => {
            let Some(hi) = hi else {
                return Err(AdversarialError::new(
                    AdversarialErrorClass::InvalidAdversarialDefinition,
                    format!("MAX requires explicit hi on dimension {}", dim.id.as_str()),
                ));
            };
            Ok(Some(mk(hi)))
        }
        CandidateOperator::Exact(x) => {
            if !in_domain(x) {
                return Err(AdversarialError::new(
                    AdversarialErrorClass::InvalidTransformParameter,
                    format!("EXACT({x}) outside domain on dimension {}", dim.id.as_str()),
                ));
            }
            Ok(Some(mk(x)))
        }
        CandidateOperator::JustBelow(x) => match x.checked_sub(1) {
            None => Ok(None),
            Some(n) if in_domain(n) => Ok(Some(mk(n))),
            Some(_) => Ok(None),
        },
        CandidateOperator::JustAbove(x) => match x.checked_add(1) {
            None => Ok(None),
            Some(n) if in_domain(n) => Ok(Some(mk(n))),
            Some(_) => Ok(None),
        },
    }
}

fn sort_numeric_values(values: &[ParameterValue], ty: ParameterType) -> Vec<ParameterValue> {
    let mut v = values.to_vec();
    sort_numeric_owned_inplace(&mut v, ty);
    v
}

fn sort_numeric_owned(mut values: Vec<ParameterValue>, ty: ParameterType) -> Vec<ParameterValue> {
    sort_numeric_owned_inplace(&mut values, ty);
    values
}

fn sort_numeric_owned_inplace(values: &mut [ParameterValue], _ty: ParameterType) {
    values.sort_by_key(|v| match v {
        ParameterValue::Integer(n) | ParameterValue::EconomicAmount(n) => *n,
        _ => i128::MAX,
    });
}

fn cartesian_lex(per_dim: &[Vec<(IdentifierToken, ParameterValue)>]) -> Vec<ParameterTuple> {
    let mut result = vec![ParameterTuple {
        bindings: Vec::new(),
    }];
    for dim_vals in per_dim {
        let mut next = Vec::new();
        for prefix in &result {
            for binding in dim_vals {
                let mut bindings = prefix.bindings.clone();
                bindings.push(binding.clone());
                next.push(ParameterTuple { bindings });
            }
        }
        result = next;
    }
    result
}
