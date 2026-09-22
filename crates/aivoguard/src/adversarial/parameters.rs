//! Parameter candidate enumeration (§17A) with bounded Cartesian generation (F-03).

use crate::adversarial::error::{AdversarialError, AdversarialErrorClass};
use crate::adversarial::token::IdentifierToken;
use crate::adversarial::types::{
    CandidateOperator, LimitBreachEvidence, LimitCounterId, ParameterDimension, ParameterDomain,
    ParameterType, ParameterValue,
};

/// One evaluated parameter tuple (ordered by dimension declaration order).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterTuple {
    /// Bindings in dimension order: (dimension_id, value).
    pub bindings: Vec<(IdentifierToken, ParameterValue)>,
}

/// Lexicographic Cartesian iterator over per-dimension candidate lists.
///
/// Does **not** materialize the full product. Advances like an odometer
/// (rightmost dimension fastest).
#[derive(Debug, Clone)]
pub struct ParameterCandidateIter {
    dims: Vec<Vec<(IdentifierToken, ParameterValue)>>,
    /// Current multi-index; empty dims → single empty tuple once.
    indices: Vec<usize>,
    finished: bool,
    empty_domain: bool,
}

impl ParameterCandidateIter {
    /// Build an iterator from a validated domain (per-dimension lists only).
    pub fn try_from_domain(domain: &ParameterDomain) -> Result<Self, AdversarialError> {
        if domain.dimensions.is_empty() {
            return Ok(Self {
                dims: Vec::new(),
                indices: Vec::new(),
                finished: false,
                empty_domain: true,
            });
        }
        let mut dims = Vec::with_capacity(domain.dimensions.len());
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
            dims.push(
                values
                    .into_iter()
                    .map(|v| (dim.id.clone(), v))
                    .collect::<Vec<_>>(),
            );
        }
        let indices = vec![0; dims.len()];
        Ok(Self {
            dims,
            indices,
            finished: false,
            empty_domain: false,
        })
    }

    fn current_tuple(&self) -> ParameterTuple {
        let bindings = self
            .indices
            .iter()
            .enumerate()
            .map(|(di, &vi)| self.dims[di][vi].clone())
            .collect();
        ParameterTuple { bindings }
    }

    fn advance(&mut self) {
        if self.dims.is_empty() {
            self.finished = true;
            return;
        }
        let mut carry = true;
        for di in (0..self.dims.len()).rev() {
            if !carry {
                break;
            }
            self.indices[di] += 1;
            if self.indices[di] < self.dims[di].len() {
                carry = false;
            } else {
                self.indices[di] = 0;
            }
        }
        if carry {
            self.finished = true;
        }
    }
}

impl Iterator for ParameterCandidateIter {
    type Item = ParameterTuple;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        if self.empty_domain {
            self.finished = true;
            return Some(ParameterTuple {
                bindings: Vec::new(),
            });
        }
        let item = self.current_tuple();
        self.advance();
        Some(item)
    }
}

/// Collect at most `max_inclusive` candidates in lex order.
///
/// If a further candidate would exist beyond `max_inclusive`, returns
/// `Err` with structured `LimitBreachEvidence` (`observed = max_inclusive + 1`).
/// Never materializes the full Cartesian product solely to discover the breach.
pub fn take_parameter_candidates(
    domain: &ParameterDomain,
    max_inclusive: u64,
) -> Result<Vec<ParameterTuple>, AdversarialError> {
    let iter = ParameterCandidateIter::try_from_domain(domain)?;
    let mut out = Vec::new();
    let mut produced: u64 = 0;
    for tuple in iter {
        produced = produced.saturating_add(1);
        if produced > max_inclusive {
            return Err(AdversarialError::limit_breach(LimitBreachEvidence {
                counter: LimitCounterId::MaximumParameterCandidates,
                observed: produced,
                limit: max_inclusive,
            }));
        }
        out.push(tuple);
    }
    Ok(out)
}

/// Enumerate all parameter candidate tuples in lexicographic dimension order.
///
/// Prefer [`take_parameter_candidates`] when a generation limit applies.
pub fn enumerate_parameter_candidates(
    domain: &ParameterDomain,
) -> Result<Vec<ParameterTuple>, AdversarialError> {
    take_parameter_candidates(domain, u64::MAX)
}

/// Multi-transform lexicographic product iterator (plan-major order).
///
/// Each position is a transform's parameter-candidate list. Yields one plan
/// binding at a time without materializing the full cross-product.
#[derive(Debug, Clone)]
pub struct PlanParamProductIter {
    per_transform: Vec<Vec<ParameterTuple>>,
    indices: Vec<usize>,
    finished: bool,
    empty: bool,
}

impl PlanParamProductIter {
    /// Build from per-transform candidate lists (already bounded/collected).
    #[must_use]
    pub fn new(per_transform: Vec<Vec<ParameterTuple>>) -> Self {
        if per_transform.is_empty() {
            return Self {
                per_transform,
                indices: Vec::new(),
                finished: false,
                empty: true,
            };
        }
        if per_transform.iter().any(Vec::is_empty) {
            return Self {
                per_transform,
                indices: Vec::new(),
                finished: true,
                empty: false,
            };
        }
        let indices = vec![0; per_transform.len()];
        Self {
            per_transform,
            indices,
            finished: false,
            empty: false,
        }
    }

    fn current(&self) -> Vec<ParameterTuple> {
        self.indices
            .iter()
            .enumerate()
            .map(|(ti, &vi)| self.per_transform[ti][vi].clone())
            .collect()
    }

    fn advance(&mut self) {
        let mut carry = true;
        for ti in (0..self.per_transform.len()).rev() {
            if !carry {
                break;
            }
            self.indices[ti] += 1;
            if self.indices[ti] < self.per_transform[ti].len() {
                carry = false;
            } else {
                self.indices[ti] = 0;
            }
        }
        if carry {
            self.finished = true;
        }
    }
}

impl Iterator for PlanParamProductIter {
    type Item = Vec<ParameterTuple>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        if self.empty {
            self.finished = true;
            return Some(Vec::new());
        }
        let item = self.current();
        self.advance();
        Some(item)
    }
}

fn dimension_values(dim: &ParameterDimension) -> Result<Vec<ParameterValue>, AdversarialError> {
    match dim.value_type {
        ParameterType::Integer | ParameterType::EconomicAmount => {
            if dim.operators.is_empty() {
                return Ok(sort_numeric_values(&dim.explicit_values, dim.value_type));
            }
            let mut out = Vec::new();
            for op in &dim.operators {
                if let Some(v) = apply_numeric_op(dim, *op)? {
                    out.push(v);
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
