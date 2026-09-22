//! Structured invariant definition (no DSL/parser).

use crate::kernel::account::AccountId;
use crate::kernel::asset::AssetId;
use crate::kernel::balance::FacetId;
use crate::kernel::money::Money;
use crate::kernel::world::PriceId;

/// Stable invariant identity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvariantId(String);

impl InvariantId {
    /// Create an invariant identity.
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

/// Invariant evaluation scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvariantScope {
    /// State target.
    State,
    /// Transition target.
    Transition,
    /// History target.
    History,
}

/// Violation retention policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationPolicy {
    /// Retain all violations.
    All,
    /// Retain only the first deterministic violation.
    FirstOnly,
}

/// World applicability declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Applicability {
    /// Always applicable.
    Always,
    /// Applicable only when World declares the named concept.
    RequiresWorldConcept(String),
}

/// Comparison operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
    /// `=`.
    Eq,
    /// `!=`.
    Ne,
    /// `<`.
    Lt,
    /// `<=`.
    Le,
    /// `>`.
    Gt,
    /// `>=`.
    Ge,
}

/// Aggregation operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggOp {
    /// SUM.
    Sum,
    /// COUNT.
    Count,
    /// MIN.
    Min,
    /// MAX.
    Max,
}

/// Supported Gate-2 relationship kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationKind {
    /// `transaction actor` equals `account owner` for a named account.
    TransactionActorOwnsAccount,
    /// `StateBefore` and `StateAfter` balances linked by declared effects for a cell.
    BeforeAfterBalanceViaEffects,
}

/// Value expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueExpr {
    /// Literal money.
    Literal(Money),
    /// Required balance cell (absence → `MISSING_REQUIRED_DATA`).
    Balance {
        /// Account.
        account: AccountId,
        /// Asset.
        asset: AssetId,
        /// Facet.
        facet: FacetId,
    },
    /// Balance of the account bound by the current quantifier domain.
    BoundAccountBalance {
        /// Asset.
        asset: AssetId,
        /// Facet.
        facet: FacetId,
    },
    /// Aggregation over a domain of money values.
    Aggregate {
        /// Operator.
        op: AggOp,
        /// Domain.
        domain: DomainExpr,
        /// Per-member value (for Sum/Min/Max); ignored for Count.
        of: Option<Box<ValueExpr>>,
        /// Declared Asset domain for empty SUM.
        declared_asset: Option<AssetId>,
    },
    /// Convert money via named World price (base→quote).
    Convert {
        /// Amount expression (must resolve to base asset of the price).
        amount: Box<ValueExpr>,
        /// Price identity.
        price_id: PriceId,
    },
}

/// Domain expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainExpr {
    /// All World accounts in deterministic key order.
    WorldAccounts,
    /// Explicit ordered account list (must be non-empty binding; empty list is empty domain).
    Accounts(Vec<AccountId>),
    /// All history records present (sparse OK).
    HistoryPresent,
    /// Inclusive logical range; records required for every position when `require_contiguous`.
    HistoryRange {
        /// Inclusive start logical position.
        start: i64,
        /// Inclusive end logical position.
        end: i64,
        /// When true, every logical position in `[start,end]` must exist.
        require_contiguous: bool,
    },
    /// Facet balances for one account+asset across declared World facets that are present.
    PresentFacetsOf {
        /// Account.
        account: AccountId,
        /// Asset.
        asset: AssetId,
    },
}

/// Property expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyExpr {
    /// Comparison.
    Compare {
        /// Left operand.
        left: ValueExpr,
        /// Operator.
        op: CmpOp,
        /// Right operand.
        right: ValueExpr,
    },
    /// Universal quantification.
    ForAll {
        /// Domain.
        domain: DomainExpr,
        /// Body.
        body: Box<PropertyExpr>,
    },
    /// Existential quantification.
    Exists {
        /// Domain.
        domain: DomainExpr,
        /// Body.
        body: Box<PropertyExpr>,
    },
    /// Relationship check.
    Relation {
        /// Kind.
        kind: RelationKind,
        /// Account involved (for ownership / balance relations).
        account: AccountId,
        /// Asset for balance relation.
        asset: Option<AssetId>,
        /// Facet for balance relation.
        facet: Option<FacetId>,
    },
    /// Transition disposition equals `AcceptedEffective` (zero effects still valid).
    DispositionAcceptedEffective,
    /// Well-formed request for an operation outside Gate-2 supported set.
    Unsupported {
        /// Reason.
        reason: String,
    },
}

/// Structured invariant definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invariant {
    /// Identity.
    pub id: InvariantId,
    /// Definition version.
    pub definition_version: String,
    /// Scope.
    pub scope: InvariantScope,
    /// Violation policy.
    pub violation_policy: ViolationPolicy,
    /// Applicability.
    pub applicability: Applicability,
    /// Property.
    pub property: PropertyExpr,
    /// History traversal direction (History scope).
    pub history_reverse: bool,
}
