# AivoGuard Payment & Settlement Testing Specification

| Field | Value |
| --- | --- |
| Document | `docs/design/payment-settlement-testing-specification.md` |
| Task | **TASK-26**…**TASK-34**; **TASK-35** re-audit **PASS**; **TASK-36** normative freeze; Gate 6 scope **TASK-25** |
| Module | **M07 — Payment & Settlement Testing** |
| Gate | **GATE 6** |
| **STATUS** | **FROZEN** |
| Normative freeze | **FROZEN** |
| Implementation | **BLOCKED** — freeze does **not** authorize implementation |
| Depends on | Product Scope (**FROZEN**); Domain Contract (**FROZEN**); M01–M04 (**FROZEN** / **IMPLEMENTED**); M06 (**FROZEN** / **IMPLEMENTED**, Gate 5 **CLOSED**, `e815c87`); Architecture baseline (boundary M07) |
| Process | [`SPECIFICATION-POLICY.md`](SPECIFICATION-POLICY.md) |

```text
STATUS: FROZEN
NORMATIVE FREEZE: FROZEN
TASK-35: PASS
TASK-36: FREEZE
IMPLEMENTATION: BLOCKED
GATE: GATE 6 — SPECIFICATION FROZEN
NEXT: separate implementation-authorization task (not this freeze)
```

This document is the **frozen** normative specification for Gate-6 M07.
TASK-35 independently returned PASS. TASK-36 records the normative freeze.
Implementation remains **BLOCKED** until an explicit later authorization task.

Open decisions **AD-03**, **AD-12**, **AD-14**, **AD-15**, **M06-OD-01/02/03**,
**DC-13**, **OD-07**, **DC-09**, and **DC-06** remain **OPEN**. This freeze does
**not** close or amend any of them. M07-OD-01 / M07-OD-02 / M07-OD-03 remain OPEN.

Related:

* [`product-scope.md`](product-scope.md) (**FROZEN**) — Gate 6 = Payment / Settlement Testing
* [`domain-contract.md`](domain-contract.md) (**FROZEN**) — DC-09 / DC-06 **OPEN**
* [`economic-kernel-specification.md`](economic-kernel-specification.md) (**FROZEN**)
* [`economic-invariant-engine-specification.md`](economic-invariant-engine-specification.md) (**FROZEN**)
* [`deterministic-simulator-specification.md`](deterministic-simulator-specification.md) (**FROZEN**)
* [`adversarial-scenario-engine-specification.md`](adversarial-scenario-engine-specification.md) (**FROZEN**)
* [`economic-regression-engine-specification.md`](economic-regression-engine-specification.md) (**FROZEN**)
* ADR 0001 — `i128` minor-unit amounts (**Accepted**)

---

## 1. Status and Authority

### 1.1 Authority hierarchy

```text
Product Scope FROZEN
        ↓
Domain Contract FROZEN
        ↓
M01 / M02 / M03 / M04 / M06 FROZEN
        ↓
This M07 specification (FROZEN)
        ↓
Accepted ADRs
        ↓
Implementation (NOT AUTHORIZED)
        ↓
Tests
```

If this document conflicts with a frozen predecessor, the predecessor wins until
an explicit amendment resolves the conflict.

### 1.2 Module thesis

> M07 deterministically evaluates whether declared payment and settlement test
> expectations are satisfied by authoritative observations, without becoming
> economic, invariant, simulation, adversarial, chaos, or regression authority.

```text
M01 = economic truth
M02 = invariant evaluation
M03 = simulation / termination
M04 = adversarial generation
M05 = chaos (future; distinct)
M06 = regression expectation comparison
M07 = payment / settlement testing
M08 = x402 adapter (future)
M09 = multi-agent world (future)
```

### 1.3 Domain Contract alignment (DC-09 / DC-06)

Domain Contract §30 records conceptual settlement statuses and defers
payment-provider settlement semantics to **DC-09** (**OPEN**).

Domain Contract records reversal/refund accounting under **DC-06** (**OPEN**).

This frozen M07 specification:

* defines a **testing-layer** settlement status taxonomy and observation contract;
* does **not** close DC-09 or DC-06;
* identifies exactly which semantics are independent vs constrained (§24–§25).

### 1.4 Product Scope alignment

Product Scope §19 Gate 6 = Payment / Settlement Testing (M07).
Payment-provider integrations remain out of scope unless a future amendment
explicitly authorizes them. This freeze keeps provider integration **OUT OF
SCOPE**.

---

## 2. Non-goals

M07 does **not** define or authorize:

* new M01 economic primitives (escrow accounts, settlement ledgers, etc.)
* M02 invariant language changes
* M03 sequencing / termination changes
* M04 scenario generation
* M05 chaos injection
* M06 regression comparison redesign
* M08 x402 protocol semantics
* M09 multi-agent settlement
* wire serialization (**AD-14 OPEN**)
* public Rust API / CLI / SDK / HTTP freeze (**AD-15** / **OD-07 OPEN**)
* scenario shrinking (**AD-12 OPEN**)
* content-derived identity algorithms (**AD-03 OPEN**)
* floating-point economic authority
* LLM judgment as authority

---

## 3. Core authority boundary

### 3.1 M07 OWNS

* payment / settlement **testing** semantics
* `PaymentSettlementCase` declarations
* settlement-phase expectation classes (closed taxonomy §12)
* settlement observation contracts (testing-layer)
* deterministic binding between case and observation
* adapter-boundary translation **contracts** (semantics only; no wire format)
* settlement-specific mismatch / error evidence
* M07 verdict `MATCH | MISMATCH | ERROR`

### 3.2 M07 CONSUMES

* M01 `EconomicState` / economic dispositions / effects (read-only)
* M02 invariant kinds when present in observation
* M03 `SimulationResult` when bound
* M04-generated `Scenario` when bound (does not generate)
* M06 `RegressionResult` when a case optionally binds generic regression
* externally supplied payment/settlement observations **only** after explicit
  adapter translation into the M07 observation model

### 3.3 M07 MUST NEVER OWN

* balance / fee / conversion arithmetic as economic truth
* invariant evaluation
* simulation sequencing / termination
* adversarial generation / transformation
* chaos injection
* generic regression comparison (M06)
* external provider truth
* x402 protocol product authority
* multi-agent orchestration

### 3.4 No-duplicate-authority matrix

| Operation | M07 | M01 | M02 | M03 | M04 | M06 |
| --- | --- | --- | --- | --- | --- | --- |
| Payment declaration | **Own** | Consume | — | — | — | — |
| Economic effect | Consume | **Own** | — | — | — | — |
| Invariant evaluation | Consume | — | **Own** | — | — | — |
| Simulation | Consume | — | — | **Own** | — | — |
| Adversarial generation | Consume | — | — | — | **Own** | — |
| Generic regression comparison | Consume | — | — | — | — | **Own** |
| Settlement test semantics / settlement expectations | **Own** | Consume | Consume | Consume | Optional | Optional |

Any conflict with this matrix is a **specification blocker**.

---

## 4. Conceptual flow

```text
PaymentSettlementCase (declared)
        ↓
(optional) M04 Scenario binding
        ↓
(external orchestration) M03 run_simulation / M01 evaluate
        ↓
Authoritative SimulationResult / EconomicState
        ↓
(optional) External adapter → SettlementObservation fields
        ↓
M07 evaluate_payment_settlement
        ↓
MATCH | MISMATCH | ERROR
        ↓
(optional) M06 regression binding for generic economic expectations
```

M07 **MUST NOT** execute M01/M03/M04/M05/M06 as hidden authority inside
comparison. Orchestration that produces observations is **external** to M07
evaluation (same pattern as M06).

---

## 5. Domain vocabulary

### 5.1 Payment

A **Payment** is a declared testing-layer intent describing a payment operation
to be observed. It is **not** automatically an M01 `Action`.

A Payment **MAY** be bound to zero or more M01 Actions / M03 Scenario steps by
explicit binding identifiers. Binding is declarative; M07 does not invent Actions.

### 5.2 Settlement

**Settlement** is the testing-layer concept of completion/application status of
a declared payment flow as observed after authoritative economic execution.

Settlement is **not** a license to invent M01 settlement ledgers. Where Domain
**DC-09** remains OPEN, M07 settlement statuses are **testing observations**,
not Domain Contract closures.

### 5.3 Distinctions (normative)

```text
declared intent     ≠ attempted operation
attempted operation ≠ successful operation
successful operation ≠ economic effect
economic effect     ≠ observed settlement status
```

M07 **MUST NOT** infer successful economic settlement solely because an
operation was attempted.

---

## 6. SettlementStatus (closed Gate-6 taxonomy)

M07 recognizes **exactly** the following settlement statuses as the **current
snapshot** status of one payment flow (see §11.0):

| Status | Meaning (current observed claim) |
| --- | --- |
| `Pending` | Declared; not yet terminal; no completion claim |
| `Authorized` | Authorization currently claimed; capture/settlement not claimed complete |
| `Captured` | Capture currently claimed; settlement completion not claimed |
| `Settled` | Full requested amount currently claimed settled under declared mode |
| `PartiallySettled` | Strict partial settlement claim: `0 < settled < requested` with `unsettled > 0` and subtraction equality (§8.4 / R-G1); not a silent failure |
| `Failed` | Current claim that the **settlement-lane** attempt failed (terminal for settlement lane) |
| `Reversed` | Current claim that the application is reversed |
| `Refunded` | Current claim of full refund (basis accounting is Domain/DC-06; M07 does not derive it — §9.1) |
| `PartiallyRefunded` | Current claim of partial refund (basis accounting is Domain/DC-06; M07 does not derive it — §9.1) |

**Normative rules:**

* Closed set — unknown status → `ERROR` / `UnsupportedSettlementState`.
* Exactly one `settlement_status` per `SettlementObservation` (single snapshot).
* `PartiallySettled` is **not** automatically `Failed`.
* `Pending` is **not** `Failed`.
* `Failed` (settlement-lane) does **not** imply an M01 economic effect unless M01
  reports one.
* M07 **MUST NOT** infer a prior `Settled` (or any other prior status) merely
  because the current status is `Reversed`, `Refunded`, or `PartiallyRefunded`
  (§11.0). Those statuses are explicit current claims, not reconstructed history.
* Mapping these statuses onto Domain §30 conceptual words does **not** close
  **DC-09**.

**Refund-lane failure is not `SettlementStatus::Failed`.** See §9.1 and
`RefundOutcome` (§11.1).

---

## 7. Lifecycle model

### 7.1 Primary lane (informative shape; not wall-clock)

```text
Declare → Authorize → Capture → Settle → Complete
```

Optional branches (testing-layer **current claims**, not inferred history):

```text
Settle → PartiallySettled
Settle → Failed          // settlement-lane failure
Settle → Delayed         // Pending + explicit delay marker; §10
current claim → Reversed
current claim → Refunded / PartiallyRefunded
refund attempt → RefundOutcome::Failed   // distinct from SettlementStatus::Failed
```

### 7.2 Normative lifecycle rules

1. Progression is **not** inferred from wall-clock or scheduler time.
2. A case **MAY** expect a current status without requiring intermediate statuses
   to appear as historical records (Gate 6 has **no** history collection; §11.0).
3. Delay is represented by explicit observation fields / scenario inputs
   (§10), never ambient time.
4. Terminality for partial statuses is **case-declared only** (§7.3) — not inferred
   from amounts.

### 7.3 Terminal partial declaration (M07-A4 / R-02 / F-04)

On `settlement_declaration`:

```text
terminal_partial         : Option<bool>
terminal_partial_refund  : Option<bool>
```

**Normative classification:** these fields are **case-level metadata / declaration
constraints only**. They are **not** observation fields, **not** expectation
classes, and **not** independently evaluated against `SettlementObservation`.

```text
Gate-6 terminal_partial and terminal_partial_refund are NOT evaluated
against SettlementObservation.

They do NOT contribute directly to MATCH, MISMATCH, or ERROR except for
the explicitly defined static declaration contradiction rules.

They MUST NOT be interpreted as observation assertions.

They MUST NOT cause M07 to infer terminality.

They MUST NOT cause M07 to infer non-terminal status.

They MUST NOT be used to classify a PartiallySettled or PartiallyRefunded
observation as terminal at runtime.
```

M07 **MUST NOT**:

* infer terminality from amounts;
* infer terminality from phase;
* infer terminality from status alone;
* fabricate a terminality observation field;
* convert `Some(false)` into a positive non-terminal assertion;
* produce MATCH or MISMATCH solely from these fields.

| Value | Meaning |
| --- | --- |
| `None` | No terminality constraint is declared |
| `Some(true)` | The case declares that a matching `PartiallySettled` / `PartiallyRefunded` current claim is **terminal** for this case’s declared interpretation |
| `Some(false)` | The case does **not** declare terminality through this metadata field. **MUST NOT** be read as “observation is non-terminal” |

**Static contradiction only (case validation → `InvalidSettlementCase`):**

```text
terminal_partial = Some(true)
AND an applicable SettlementStatusExact expects Settled
→ InvalidSettlementCase
```

```text
terminal_partial_refund has no verdict authority and creates no
case-validation contradiction.

M07 MUST NOT infer compatibility or incompatibility between
terminal_partial_refund and SettlementStatusExact.

Some(true) / Some(false) / None on terminal_partial_refund remain
declaration metadata only (§7.3 normative classification above).
```

No other terminality rule is introduced. No declaration↔observation terminality
MISMATCH exists (no observation field). Harnesses outside M07 **MAY** read these
flags as documentation only.

---

## 8. Amount, fee, and unit semantics

### 8.1 Amounts (R-F1 / R-F4)

All M07 economic amounts **MUST** be:

* signed integer minor units consistent with M01 / ADR 0001 (`i128`);
* compared without float;
* compared without silent cross-asset conversion.

#### Asset identity authority (R-F1)

```text
Gate-6 observation amounts are interpreted exclusively in
case.payment_declaration.asset_id.

SettlementObservation does NOT carry an independent economic asset
authority field.

M07 MUST NOT perform asset conversion, FX, or cross-asset reinterpretation.

External adapters MUST reject any source amount whose source asset
does not equal case.payment_declaration.asset_id before constructing
SettlementObservation.

source asset mismatch → ExternalAdapterError
```

Because asset rejection occurs at the adapter boundary, a conforming Gate-6
`SettlementObservation` that reaches M07 evaluation is already denominated in
`payment_declaration.asset_id`. M07 does not re-check a missing observation
asset field.

#### Amount sign domain (R-F4)

Underlying representation remains `i128` (ADR 0001). Semantic domain:

```text
MUST be >= 0 (declaration → InvalidAmount; observation → InvalidSettlementObservation):

    gross_amount
    requested_amount          // declaration and observation
    settled_amount
    unsettled_amount
    refund_amount
    fee_amount
```

```text
net_settlement_amount MAY be signed.

It represents a derived or observed net quantity and MUST remain compatible
with ADR 0001 checked arithmetic. A negative net_settlement_amount is NOT
automatically InvalidAmount / InvalidSettlementObservation solely due to sign.
```

`escrow_amount` remains optional, non-evaluated, non-authoritative (§9.3); if
present it **MUST** be `>= 0` or → `InvalidSettlementObservation`.

Checked arithmetic only. Overflow / undefined checked ops →
`ERROR` / `NumericComparisonError`.

Tolerance, when used, **MUST** follow the same normative procedure as M06 §11.3
(checked_sub → checked_abs → checked i128→u128 → `<= tolerance`).

### 8.2 Fee model (M07-A1 / R-A3)

Fee authority lives **only** inside an optional `FeeDeclaration`. There is
**no** top-level `payment_declaration.fee_mode`.

```text
payment_declaration
├── gross_amount : i128
└── fee : Option<FeeDeclaration>

FeeDeclaration
├── fee_amount    : i128          // REQUIRED when fee = Some
├── fee_payer     : account token // OPTIONAL
├── fee_recipient : account token // OPTIONAL
├── fee_asset     : asset token   // REQUIRED when fee = Some
├── fee_timing    : WithCapture | WithSettlement | DeclaredOnly  // REQUIRED
└── fee_mode      : FeeExclusive | FeeInclusive | FeeDeclaredOnly // REQUIRED
```

When `fee = None`, no fee declaration fields exist and M07 **MUST NOT** invent
fee amounts, modes, or assets.

When `fee = Some(...)`:

```text
fee_amount, fee_asset, fee_timing, and fee_mode MUST all be present.

Absence of any of those four fields → InvalidPaymentCase.

fee_payer / fee_recipient MAY be omitted (tokens are non-authoritative labels).

Negative fee_amount / cross-asset fee_asset / fee_amount > gross where
prohibited → InvalidAmount (after presence is satisfied).
```

#### Fee timing vs fee mode (R-F3)

```text
fee_timing controls only the declared timing category
    (WithCapture | WithSettlement | DeclaredOnly).

fee_mode controls only fee arithmetic / net-settlement interpretation
    (FeeExclusive | FeeInclusive | FeeDeclaredOnly).

fee_timing MUST NOT alter fee_mode semantics.
fee_mode MUST NOT alter fee_timing semantics.

These fields are orthogonal. Any pair is structurally valid.
M07 MUST NOT invent a compatibility matrix that rejects combinations.
```

#### Fee amount sign

```text
fee_amount MUST be >= 0.

A negative fee_amount is InvalidAmount.

M07 MUST NOT interpret a negative fee as a rebate, credit, subsidy,
or other economic effect.
```

#### Fee asset

```text
fee_asset MUST equal payment_declaration.asset_id for Gate 6.

Cross-asset fee settlement, conversion, FX, exchange-rate application,
or multi-asset fee accounting is OUT OF SCOPE for Gate 6.

A cross-asset fee requirement MUST NOT be silently converted into the
payment asset.

Such a requirement is InvalidAmount / unsupported declaration semantics
and requires a future explicit amendment.
```

#### FeeExclusive

```text
gross_amount = declared payment gross amount
fee_amount   = separately declared fee
derived_net_settlement_amount = checked_sub(gross_amount, fee_amount)

The checked subtraction MUST succeed.

If fee_amount > gross_amount:
    InvalidAmount

If checked arithmetic fails:
    NumericComparisonError
```

M07 does not independently determine whether an external economic system
actually charged the fee. It only evaluates declared fee expectations against
the supplied observation.

#### FeeInclusive

```text
gross_amount includes the fee portion.
fee_amount identifies the declared fee portion.
derived_net_settlement_amount = checked_sub(gross_amount, fee_amount)

fee_amount MUST be >= 0.
fee_amount MUST be <= gross_amount.

If fee_amount > gross_amount:
    InvalidAmount

If checked arithmetic fails:
    NumericComparisonError
```

#### FeeDeclaredOnly

```text
fee fields MAY be declared and compared as observations.

M07 MUST NOT derive a net settlement amount from gross_amount and fee_amount
under FeeDeclaredOnly.

M07 MUST NOT reinterpret FeeDeclaredOnly as FeeExclusive or FeeInclusive.
```

#### Net settlement derivation consumer (R-F2)

```text
For FeeExclusive / FeeInclusive:

derived_net_settlement_amount
    = checked_sub(gross_amount, fee_amount)

The derived value is an internal case-side derived value.

It MUST NOT mutate observation.net_settlement_amount.
It MUST NOT invent an observation field.

M07 MUST NOT silently derive for FeeDeclaredOnly, unspecified,
ambiguous, or unsupported fee modes.
```

```text
NetSettlementExact payload:
    expected_net_settlement_amount : i128

If NetSettlementExact is applicable:
    observation.net_settlement_amount (Option)
        None  → MissingObservationField
        Some  → exact compare against expected_net_settlement_amount

When fee = Some and fee_mode is FeeExclusive or FeeInclusive
AND NetSettlementExact is applicable:
    expected_net_settlement_amount
    MUST equal derived_net_settlement_amount.

Mismatch of expected vs derived at case validation → InvalidPaymentCase.

When fee = None or fee_mode = FeeDeclaredOnly:
    NetSettlementExact compares observation to expected only;
    no fee-derived equality check is performed.
```

M07 **MUST NOT** silently reinterpret inclusive vs exclusive fees.
Mismatch of fee fields under `FeeExact` → MISMATCH; arithmetic failure →
`NumericComparisonError`; structural fee invalidity → `InvalidAmount`.

### 8.3 Amount field authority (M07-A2)

```text
payment_declaration.gross_amount
    = declared payment amount

settlement_declaration.requested_amount
    = declared settlement request amount

observation.requested_amount
    = observed requested settlement amount
```

These fields have **distinct** semantic roles.

```text
payment_declaration.gross_amount
MUST NOT be implicitly required to equal
settlement_declaration.requested_amount.

A difference between these declarations is not itself an M07 error.

M07 MUST NOT infer that one field replaces the other.
```

For Gate 6:

```text
settlement_declaration.requested_amount
is the authoritative case-level declaration of the amount requested
for settlement.

payment_declaration.gross_amount remains the payment declaration amount.
```

`observation.requested_amount` is an observed field.

Gate 6 does **not** introduce a dedicated expectation class for
`requested_amount`. Instead, for statuses that structurally require it,
observation-to-case binding is a **structural** rule (Option A / R-A1):

```text
For every Settled or PartiallySettled observation:

observation.requested_amount
MUST equal
case.settlement_declaration.requested_amount.

A difference → InvalidSettlementObservation.

M07 MUST NOT derive observation.requested_amount from
payment_declaration.gross_amount.
M07 MUST NOT silently accept a divergent observed requested amount.
```

If a future amendment adds an expectation class comparing
`observation.requested_amount`, that class **MUST** explicitly identify
whether it compares against `payment_declaration.gross_amount` or
`settlement_declaration.requested_amount`. Gate 6 does not introduce such a
class; the structural rule above is authoritative for Settled /
PartiallySettled.

### 8.4 Partial settlement amounts

Observation fields (when present):

```text
requested_amount : i128
settled_amount   : i128
unsettled_amount : i128
```

For `PartiallySettled` (§11.5 / R-G1): `requested_amount`, `settled_amount`,
and `unsettled_amount` are **all required** on the observation.

#### PartiallySettled strict amount invariant (R-G1; normative)

The status definition “some but not all” is enforced **only** by the following
structural rules — not by informal reading of the status label alone:

```text
requested_amount >= 0          // §8.1 amount-domain
settled_amount > 0
settled_amount < requested_amount
unsettled_amount > 0
unsettled_amount == checked_sub(requested_amount, settled_amount)
```

Structural violations (any of):

```text
settled_amount <= 0
OR settled_amount >= requested_amount
OR unsettled_amount <= 0
OR unsettled_amount ≠ checked_sub(requested_amount, settled_amount)
OR checked_sub fails
```

→ `InvalidSettlementObservation`.

Normative validation order for `PartiallySettled` amount fields (deterministic;
implementations MUST NOT choose a different error class for the same input):

```text
1. required-field presence (requested, settled, unsettled)
2. non-negative amount-domain validation (§8.1)
3. PartiallySettled strict inequalities
   (settled > 0; settled < requested; unsettled > 0)
4. checked_sub(requested_amount, settled_amount)
5. equality of unsettled_amount to that result
```

Conformance examples (normative):

```text
requested=100, settled=0,   unsettled=100  → InvalidSettlementObservation
requested=100, settled=50,  unsettled=50   → valid (subject to other rules)
requested=100, settled=100, unsettled=0    → InvalidSettlementObservation
requested=100, settled=150                 → InvalidSettlementObservation
```

`settled_amount < requested_amount` alone does **not** imply `Failed`.

#### Settled full-amount invariant (R-A2)

```text
For SettlementStatus::Settled:

requested_amount MUST be present.
settled_amount MUST be present.

settled_amount MUST equal requested_amount
    (checked equality; no float; same Asset).

If unsettled_amount is present:
    unsettled_amount MUST equal 0.

Failure of any of these structural relationships:
    InvalidSettlementObservation.
```

This validates internal consistency of the M07 testing-layer snapshot only.
It does **not** close DC-06 or DC-09.

---

## 9. Refund, reversal, and escrow (constrained)

### 9.1 Refund (**DC-06 CONSTRAINED**; M07-A3)

M07 **MAY** declare and observe:

* full refund via `RefundOutcome::SucceededFull` and/or `SettlementStatus::Refunded`
* partial refund via `RefundOutcome::SucceededPartial` and/or
  `SettlementStatus::PartiallyRefunded`
* refund-attempt failure via **`RefundOutcome::Failed`** (not via
  `SettlementStatus::Failed`)

#### Refund-basis authority

```text
M07 does NOT define or derive the authoritative refundable basis.

M07 may compare:
    expected refund_amount
        against
    observed refund_amount
when RefundAmountExact is applicable.

M07 does not independently determine whether an observed refund amount
is economically refundable, refundable net of fees, refundable against
gross amount, refundable against settled amount, or refundable under any
other Domain/DC-06 accounting rule.

RefundAmountExact = exact observation comparison only.
```

```text
Gate 6 MUST NOT independently classify a refund as exceeding its
authoritative refundable basis.

Any refundable-basis validation requiring DC-06 accounting semantics
remains outside M07 until explicitly frozen by the Domain Contract.

An adapter MUST NOT silently clamp a refund amount to a presumed basis.
```

#### Refund amount sign

```text
refund_amount MUST be >= 0.

Negative refund_amount in a declaration → InvalidAmount.
Negative refund_amount in an observation → InvalidSettlementObservation.
```

#### Refund failure ≠ settlement failure (F-03; normative)

```text
SettlementStatus::Failed
  = settlement-lane attempt currently claimed failed

RefundOutcome::Failed
  = refund operation/attempt currently claimed failed
```

These are **distinct**.

Therefore:

```text
phase = Refund
settlement_status = Settled          // example: prior settlement claim remains
refund_outcome = Failed              // refund attempt failed
```

is a valid observation shape.

M07 **MUST NOT** interpret `RefundOutcome::Failed` as rewriting
`settlement_status` to `Failed`.

`RefundStatusExact` compares **`refund_outcome` only** (§12), not
`settlement_status`.

Gate 6 uses a **single current snapshot** (§11.0). There is **no** ordered
collection of refund-attempt records. Repeated refund attempts are **out of
Gate-6 observation scope** (would require an ordered history model amendment).

**Independent of DC-06 (testing-layer):**

* declaring expected refund outcome / amounts;
* comparing those expectations to observation fields;
* distinguishing refund declaration from M01 economic effect.

**Depends on DC-06 / Domain / M01 (not closed here):**

* whether refunds rewrite history vs append compensating records;
* exact refundable-basis accounting across Worlds;
* mandatory recording of refund as Transaction disposition taxonomy.

Gate-6 M07 **MUST NOT** invent Domain refund accounting. Where economic effect
of refund must be authoritative, M07 consumes M01 state after orchestration.

### 9.2 Reversal

**Reversal** ≠ **Refund** ≠ **Failed** ≠ compensating narrative without status.

| Concept | Meaning in M07 |
| --- | --- |
| `SettlementStatus::Failed` | Settlement-lane attempt currently claimed failed |
| `RefundOutcome::Failed` | Refund attempt currently claimed failed |
| `SettlementStatus::Reversed` | Current claim that the application is reversed |
| `SettlementStatus::Refunded` / `PartiallyRefunded` | Current refund claim |
| Compensating transaction | An M01 Action sequence producing offsetting effects; not a settlement status by itself |

`ReversalStatusExact` compares: `phase == Reverse` **and**
`settlement_status == Reversed` (exact). It does **not** reconstruct prior
`Settled` history (§11.0).

If Domain amendment is required for reversal history semantics, that remains
**DC-06 / Domain** — M07 only observes declared current status + M01 effects.

### 9.3 Escrow (**REQUIRES_FUTURE_DECISION / M07-OD-01**; M07-A5)

Frozen M01 does **not** define escrow lock primitives.

Gate 6 escrow remains **observation-only**.

```text
EscrowStatusExact
    compares observation.escrow_status exactly.

If EscrowStatusExact is applicable and escrow_status is None:
    MissingObservationField
```

`escrow_amount` remains:

```text
OPTIONAL
NON-AUTHORITATIVE
NOT EVALUATED BY ANY CLOSED GATE-6 EXPECTATION
```

There is **no** `EscrowAmountExact` in Gate 6.

```text
M07 MUST NOT derive escrow balances.
M07 MUST NOT derive escrow lock amounts.
M07 MUST NOT infer escrow release amounts.
M07 MUST NOT create M01 escrow primitives.
M07 MUST NOT require new M01 escrow accounts.
```

Any requirement for authoritative escrow economics remains:

```text
M07-OD-01
Domain / M01 amendment required
```

Gate-6 M07:

* **MAY** expose escrow as declared testing vocabulary **only when**
  observation fields are supplied by an adapter or Scenario metadata that
  does not invent M01 balances;
* **MUST NOT** fabricate lock balances.

---

## 10. Delayed settlement (no wall-clock authority)

Delay **MUST** be represented by at least one of:

* `SettlementStatus = Pending` plus `delay_marker: DeclaredDelayToken` (exact string);
* Scenario / ExecutionContext fields already owned by M03/M01 (logical time);
* explicit observation field `settlement_delay_steps: u64` (simulation steps), not host ms.

Forbidden as authority:

* `SystemTime`, timezone, sleep, network RTT, provider clocks.

---

## 11. SettlementObservation

### 11.0 Observation model choice (F-02 / F-05; normative)

Gate-6 M07 evaluates **exactly one current snapshot**:

```text
SettlementObservation = single authoritative current observation object
```

Gate 6 **does not** evaluate an ordered collection of historical status records.

Therefore:

* there is **no** `status_record_id`;
* there is **no** 0/1/>1 record cardinality over a status-history collection;
* M07 **MUST NOT** reconstruct lifecycle history from multiple records;
* `AmbiguousObservation` is **not** emitted for SettlementObservation lookup in
  Gate 6 (reserved; unused on the Gate-6 path);
* structural status/phase/field-matrix violations →
  `InvalidSettlementObservation` or `UnsupportedSettlementState` per §11.4 /
  §11.5 / §19;
* absence of an `Option` field required by an **applicable** expectation →
  `MissingObservationField` (§12); absence **MUST NOT** become zero.

`SettlementObservation` represents the **declared current** observed status at
the declared `phase`. M07 does not infer historical transitions that are not
explicitly represented by fields on this snapshot.

### 11.1 Package

```text
SettlementObservation
├── binding                 // PaymentSettlementBinding (required)
├── settlement_status       // SettlementStatus (required)
├── execution_state         // SettlementExecutionState (required)  // F-01
├── phase                   // SettlementPhase (required)
├── refund_outcome          // Option<RefundOutcome>                // F-03
├── requested_amount        // Option<i128>
├── settled_amount          // Option<i128>
├── unsettled_amount        // Option<i128>
├── fee_amount              // Option<i128>
├── net_settlement_amount   // Option<i128>
├── refund_amount           // Option<i128>
├── escrow_status           // Option<EscrowStatus>
├── escrow_amount           // Option<i128>
├── delay_marker            // Option<string>
├── settlement_delay_steps  // Option<u64>
├── m07_label               // Option<string>  // semantic pin carrier; R-03
├── simulation_result       // Option<SimulationResult>  // when M03-bound
├── adapter_provenance      // Option<AdapterProvenance>
└── notes                   // Option<string>  // non-authoritative
```

#### SettlementExecutionState (closed; F-01)

| State | Meaning |
| --- | --- |
| `NotAttempted` | No payment/settlement operation attempt is claimed |
| `Attempted` | An attempt is claimed; successful execution is **not** claimed |
| `Executed` | Execution of the declared operation at `phase` is claimed |

Rules:

* Mutually exclusive; exactly one value required on every observation.
* M07 **MUST NOT** infer `Executed` from M01 economic effects.
* M07 **MUST NOT** infer `Executed` from `settlement_status`.
* M07 **MUST NOT** infer `Attempted` from non-`Pending` status alone.
* `SettlementExecutionExact` compares `execution_state` (and optionally
  declared phase equality — §12). Absence of `execution_state` is impossible
  on a well-formed observation; malformed observation without it →
  `InvalidSettlementObservation`.

#### RefundOutcome (closed; F-03)

| Value | Meaning |
| --- | --- |
| `SucceededFull` | Refund attempt currently claimed fully successful |
| `SucceededPartial` | Refund attempt currently claimed partially successful |
| `Failed` | Refund attempt currently claimed failed |

`refund_outcome = None` means no refund-outcome claim is present (not zero,
not success, not failure).

#### Other closed sets

`EscrowStatus` when present: `NoneDeclared | Locked | Released | ReleaseFailed |
Cancelled`.

`SettlementPhase`: `Declare | Authorize | Capture | Settle | Refund | Reverse |
Complete`.

### 11.2 Cardinality (Gate-6 single snapshot)

For Gate-6 M07 comparison against `case.observation`:

```text
exactly one SettlementObservation is supplied on the case
→ evaluate that snapshot
```

There is no multi-record selection. External adapters **MUST** collapse any
provider multi-event history into one deterministic snapshot **before** M07
evaluation; how they do so is adapter responsibility and MUST be explicit in
adapter provenance notes (non-authoritative) without inventing amounts.

### 11.3 Presence rules

* Absence **MUST NOT** become zero.
* Use presence-distinguishing lookup for balance cells when reading M01 via
  `SimulationResult.final_state` (same rule as M06).
* Required/optional fields by status: §11.5; status↔phase validity: §11.4.

### 11.4 Status ↔ phase compatibility (R-01 / R-04; closed; exhaustive)

For every `SettlementObservation`, the pair

```text
(observation.settlement_status, observation.phase)
```

**MUST** appear in the following closed matrix. If it does not →
`ERROR` / `InvalidSettlementObservation`.

| SettlementStatus | Valid SettlementPhase values (exactly these) |
| --- | --- |
| `Pending` | `Declare`, `Authorize`, `Capture`, `Settle` |
| `Authorized` | `Authorize` |
| `Captured` | `Capture` |
| `Settled` | `Settle`, `Complete` |
| `PartiallySettled` | `Settle`, `Complete` |
| `Failed` | `Settle` |
| `Reversed` | `Reverse` |
| `Refunded` | `Refund`, `Complete` |
| `PartiallyRefunded` | `Refund`, `Complete` |

This matrix is **exhaustive** over the closed `SettlementStatus` ×
`SettlementPhase` product: every pair not listed is **invalid**.

Normative prohibitions:

* No “typical”, “usual”, “normally”, or implementation-defined phase choice.
* Validity of the pair does **not** imply that any prior lifecycle phase
  occurred. The matrix validates the **current snapshot only** (§11.0).
* Example: `Settled` + `Complete` does **not** reconstruct
  `Declare → … → Settle`.
* Example: `Refunded` + `Complete` does **not** require a prior `Settled`
  snapshot to have been observed by M07.

### 11.5 Field presence by status (normative)

Legend:

* **R** = required present (`Some`). Absence → structural
  `InvalidSettlementObservation` for fields required by status (distinct from
  expectation-time `MissingObservationField`).
* **O** = optional (`None` or `Some` permitted under other rules).
* **A** = MUST be absent (`None`).

```text
A = MUST be absent (None).

If a field marked A is present (Some(_)), the observation is invalid.

Classification:
InvalidSettlementObservation.
```

There is **no** “ignore” path. Presence of an A-marked field is never silently
accepted. “Conflicting meaning” is not a separate concept — presence alone is
the violation.

* Amount fields that are **R** for an applicable expectation but missing →
  `MissingObservationField` when that expectation is evaluated; matrix below
  is structural validity of the snapshot independent of expectations.

Always required on every snapshot:

```text
binding, settlement_status, execution_state, phase  = R
```

Field requirements depend on `settlement_status` only (phase already constrained
by §11.4):

| settlement_status | requested | settled | unsettled | fee | net | refund_amount | refund_outcome | escrow_* | delay_* | m07_label |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Pending | O | A | A | O | O | A | A | O | O | O |
| Authorized | O | A | A | O | O | A | A | O | A | O |
| Captured | O | A | A | O | O | A | A | O | A | O |
| Settled | R | R | O* | O | O | A | A | O | A | O |
| PartiallySettled | R | R | R | O | O | A | A | O | A | O |
| Failed | O | O | O | O | O | A | A | O | A | O |
| Reversed | O | O | O | O | O | A | A | O | A | O |
| Refunded | O | O | O | O | O | R | R=`SucceededFull` | O | A | O |
| PartiallyRefunded | O | O | O | O | O | R | R=`SucceededPartial` | O | A | O |

\* For `Settled` (R-A2):

```text
requested_amount and settled_amount are required.
settled_amount MUST equal requested_amount.
If unsettled_amount is present it MUST equal 0.
Also: observation.requested_amount MUST equal
      case.settlement_declaration.requested_amount (R-A1).
Any failure → InvalidSettlementObservation.
```

Additional normative checks:

1. For every field marked **A** in the matrix for the observation’s
   `settlement_status`: presence (`Some(_)`) →
   `InvalidSettlementObservation` (R-G2).
2. If `unsettled_amount` and both `requested_amount` and `settled_amount` are
   present → `unsettled` MUST equal `checked_sub(requested, settled)` else
   `InvalidSettlementObservation`.
3. For `Settled` or `PartiallySettled`: `observation.requested_amount` MUST
   equal `settlement_declaration.requested_amount` else
   `InvalidSettlementObservation` (R-A1). M07 **MUST NOT** derive the
   observed value from `gross_amount`.
4. For `Settled`: `settled_amount` MUST equal `requested_amount` else
   `InvalidSettlementObservation` (R-A2).
5. For `PartiallySettled`: apply the R-G1 strict amount invariant and
   validation order in §8.4 (`0 < settled < requested`, `unsettled > 0`,
   subtraction equality) → `InvalidSettlementObservation` on failure.
6. If `refund_outcome = Failed` → `refund_amount` is O; **MUST NOT** force
   `settlement_status = Failed`.
7. If `refund_outcome = None` and `RefundStatusExact` / `RefundAmountExact` is
   applicable → `MissingObservationField`.
8. Escrow fields: if `EscrowStatusExact` applicable and `escrow_status` absent →
   `MissingObservationField` (unchanged §9.3).
9. Delay: `delay_marker` / `settlement_delay_steps` remain optional context on
   `Pending`. If an applicable future delay expectation class is added by
   amendment and a required delay field is absent → `MissingObservationField`.
   Gate 6 closed taxonomy has no DelayExact class.
10. Status/phase validity is exclusively §11.4 (not examples).

#### Cross-field lifecycle authority (M07-A10)

```text
M07 validates cross-field relationships only where an explicit normative
rule defines such a relationship.

M07 MUST NOT infer additional lifecycle transitions from combinations of:
    execution_state
    settlement_status
    phase
    amount fields
    M01 economic effects
    refund_outcome
    escrow fields
```

The explicit Gate-6 cross-field rules are limited to:

```text
settlement_status ↔ phase matrix (§11.4)
settled/unsettled amount consistency (§8.4 / §11.5)
Settled: settled_amount == requested_amount; unsettled_amount = 0 when present
PartiallySettled: 0 < settled_amount < requested_amount;
    unsettled_amount == checked_sub(requested, settled); unsettled_amount > 0
Settled / PartiallySettled: observation.requested_amount ==
    settlement_declaration.requested_amount
A-marked fields MUST be None (§11.5 / R-G2)
Refunded ↔ SucceededFull
PartiallyRefunded ↔ SucceededPartial
Reversed ↔ Reverse phase (via ReversalStatusExact / §11.4)
```

No implicit lifecycle inference is authorized.

---

## 12. Expectation taxonomy (closed)

Gate-6 M07 supports **only** the classes below. Every class has a **closed
payload schema** (R-F5). Envelope fields are common to all classes:

```text
expectation_id : non-empty unique within case
class          : one closed class
applicable     : bool   // false → skip; no lookup; no mismatch; no ordinal
```

Duplicate `expectation_id` → `ERROR` / `InvalidPaymentCase`.

| Class | Compares |
| --- | --- |
| `SettlementStatusExact` | `settlement_status` exact |
| `SettledAmountExact` | `settled_amount` exact i128 |
| `SettledAmountAbsoluteTolerance` | settled amount with M06-compatible tolerance procedure |
| `FeeExact` | `fee_amount` exact |
| `NetSettlementExact` | `net_settlement_amount` exact |
| `RefundStatusExact` | `refund_outcome` exact |
| `RefundAmountExact` | `refund_amount` exact |
| `ReversalStatusExact` | fixed: `phase == Reverse` ∧ `settlement_status == Reversed` |
| `EscrowStatusExact` | `escrow_status` exact |
| `SettlementExecutionExact` | `execution_state` exact (+ optional phase) |

#### Closed payloads (R-F5)

```text
SettlementStatusExact
└── expected_status : SettlementStatus

SettledAmountExact
└── expected_settled_amount : i128   // MUST be >= 0 (§8.1 / R-G3)

SettledAmountAbsoluteTolerance
├── expected_settled_amount : i128   // MUST be >= 0 (R-G3)
└── tolerance : u128                 // M06 §11.3 procedure

FeeExact
└── expected_fee_amount : i128       // MUST be >= 0 (R-G3)

NetSettlementExact
└── expected_net_settlement_amount : i128   // MAY be signed; §8.2 R-F2
    // NOT subject to R-G3 non-negative expected-amount validation

RefundStatusExact
└── expected_refund_outcome : RefundOutcome
    // SucceededFull | SucceededPartial | Failed

RefundAmountExact
└── expected_refund_amount : i128    // MUST be >= 0 (R-G3)

ReversalStatusExact
└── (no variable payload fields)
    Fixed evaluation:
        observation.phase == Reverse
        AND observation.settlement_status == Reversed
    Both conditions required for MATCH when applicable.

EscrowStatusExact
└── expected_escrow_status : EscrowStatus

SettlementExecutionExact
├── expected_execution : SettlementExecutionState   // required
└── expected_phase     : Option<SettlementPhase>    // if Some, phase must match
```

#### Negative expected amount classification (R-G3)

```text
For every expectation payload whose amount domain is declared as >= 0,
a negative expected amount is invalid during case validation and MUST
produce InvalidPaymentCase before expectation evaluation begins.
```

Affected payloads (exhaustive for Gate 6):

```text
SettledAmountExact.expected_settled_amount
SettledAmountAbsoluteTolerance.expected_settled_amount
FeeExact.expected_fee_amount
RefundAmountExact.expected_refund_amount
```

```text
Negative expected amount in a Gate-6 expectation payload
→ InvalidPaymentCase
```

This is a **case-validation** error (malformed declared test case), not an
observation error and not a MISMATCH.

`NetSettlementExact.expected_net_settlement_amount` remains signed and does
**NOT** receive this non-negative validation.

Observation-side negatives remain (§8.1 / R-F4):

```text
negative observed gross/requested/settled/unsettled/refund/fee
→ InvalidSettlementObservation
```

#### SettlementExecutionExact rules (F-01)

* Compares `execution_state` to `expected_execution`.
* If `expected_phase` is `Some(p)`, also requires `observation.phase == p`.
* Does **not** infer execution from status or M01 effects.
* Missing `execution_state` on observation → `InvalidSettlementObservation`
  (structurally required); expectation evaluation does not invent a default.

#### RefundStatusExact rules (F-03)

Compares only `observation.refund_outcome`. If `refund_outcome` is `None` →
`MissingObservationField`. Does **not** compare `settlement_status`.

#### NetSettlementExact rules (R-F2)

See §8.2 net settlement derivation consumer. Observation compare only; never
mutates observation. Fee-derived equality check on expected when
FeeExclusive/FeeInclusive applies.

**Sole status authority (F-09):** expected settlement status is expressed
**only** via `SettlementStatusExact` (and refund/reversal classes above).
There is **no** parallel `expected_status` field on the case declaration.

**M06 non-duplication rule:**

* Generic final-balance / execution-status / invariant-kind / step-disposition
  expectations **MUST NOT** be redefined by M07.
* Cases needing those comparisons **MUST** use `m06_binding` (§15).

#### Contradiction resolution priority (M07-A10)

```text
1. Frozen predecessor/domain constraints
2. M07 structural validity rules
3. M07 closed expectation semantics
4. M06 composition rule
5. Diagnostic notes / human-readable reason
```

Non-authoritative notes **MUST NOT** alter verdict.

If two applicable M07 expectations contradict each other:

```text
contradictory expectations → MISMATCH
```

under `ALL_MISMATCHES`, unless an explicit structural rule classifies the case
itself as invalid (`InvalidPaymentCase` / `InvalidSettlementCase`).

Empty expectation list → `MATCH` unless case-level ERROR.

#### Optional observation field rule (generic; M07-A9)

```text
For every applicable expectation whose target is an Option<T> observation
field:

    Some(value)
        → evaluate value normally

    None
        → MissingObservationField

unless the expectation class explicitly defines None as a valid comparable
value.

No Gate-6 closed expectation currently treats absent amount/status data as
zero.
```

This rule applies to (where the class targets that field):

```text
settled_amount
fee_amount
net_settlement_amount
refund_outcome
refund_amount
escrow_status
```

---

## 13. PaymentSettlementCase

```text
PaymentSettlementCase
├── case_id / case_version          // required non-empty
├── binding : PaymentSettlementBinding
├── payment_declaration
│     ├── payment_id
│     ├── payer / payee accounts (tokens)
│     ├── asset_id
│     ├── gross_amount
│     └── fee : Option<FeeDeclaration>   // sole fee authority; §8.2
│           ├── fee_amount               // required when Some
│           ├── fee_payer                // optional
│           ├── fee_recipient            // optional
│           ├── fee_asset                // required when Some
│           ├── fee_timing               // required when Some
│           └── fee_mode                 // required when Some; no top-level fee_mode
├── settlement_declaration
│     ├── requested_amount
│     ├── terminal_partial : Option<bool>         // §7.3; NOT expected_status
│     └── terminal_partial_refund : Option<bool>  // metadata only; no contradiction rule
├── expectations : ordered list     // may be empty; sole expectation authority
├── observation : SettlementObservation           // single snapshot §11.0
├── scenario_binding : optional { scenario_id, scenario_version, configuration_id }
├── m06_binding : optional M06Binding             // §15
├── engine_pins : optional EnginePins             // §14.1 / F-08
└── notes : optional non-authoritative string
```

`PaymentSettlementBinding`:

```text
payment_id
payment_version
configuration_id
```

Binding fields **MUST** exactly equal `observation.binding` or →
`ERROR` / `ObservationBindingMismatch`.

**F-09:** `expected_status` is **removed**. Authors MUST use
`SettlementStatusExact` in `expectations`.

---

## 14. Evaluation algorithm

Evidence policy: **`ALL_MISMATCHES`**.

```text
1. Validate case structure → InvalidPaymentCase / InvalidSettlementCase
   (includes §7.3 terminal_partial static conflict only; forbids expected_status;
   fee = Some requires fee_amount/fee_asset/fee_timing/fee_mode — §8.2;
   declaration amount sign domain §8.1; NetSettlementExact expected vs
   derived_net under FeeExclusive/Inclusive — §8.2 R-F2;
   negative expected amounts on >=0 payloads → InvalidPaymentCase — R-G3)
2. Validate status↔phase compatibility §11.4 → InvalidSettlementObservation
3. Validate observation field matrix §11.5 → InvalidSettlementObservation
   (A-marked Some(_) forbidden — R-G2; Settled full-amount invariant;
    PartiallySettled R-G1 order; Settled/PartiallySettled requested binding;
    observation amount sign domain §8.1)
4. Validate binding vs observation → ObservationBindingMismatch
5. Validate expectation ids unique / classes supported / payloads closed → InvalidPaymentCase
6. Optional engine_pins exact compare → EnginePinMismatch (§14.1)
7. For each expectation in declaration order:
     if applicable == false: skip (no ordinal consumed)
     else evaluate against the single snapshot
     on ERROR: abort; discard accumulated mismatches; return ERROR
     on unsatisfied: append MismatchEvidence in declaration order (§14.2)
8. If m06_binding present: apply §15 composition (§14.2 ordering)
9. mismatches empty → MATCH else MISMATCH
```

### 14.0 ERROR precedence (M07-A8; global invariant)

```text
ERROR dominates all previously accumulated mismatches.

If any evaluation stage produces ERROR:
    final verdict = ERROR
    final mismatches = []

Previously accumulated mismatch evidence MUST be discarded from the
returned result.
```

This applies to: case validation, observation validation, binding validation,
expectation-set validation, engine pin validation, expectation evaluation,
M06 composition, numeric validation, and adapter validation.

Therefore:

```text
MISMATCH then later ERROR  →  ERROR with mismatches=[]
```

never:

```text
ERROR + previous mismatches
```

### 14.1 Engine pins (F-08 / R-03 / M07-A6)

```text
EnginePins
├── m01_engine_version : Option<string>
├── m02_engine_version : Option<string>
├── m03_engine_version : Option<string>
└── m07_label          : Option<string>
```

**Comparison flow (normative):**

```text
Case-declared pin
        ↓
Observed/composed semantic pin
        ↓
Exact string comparison
        ↓
ok  OR  ERROR / EnginePinMismatch
```

**Ownership:**

* Pin **comparison** is owned by M07 evaluation.
* Upstream engine **identity authority** remains external (M01/M02/M03 version
  strings on `SimulationResult`; M07 does not redefine them).
* M07 **MUST NOT** manufacture missing pins.

**m01 / m02 / m03 pins:**

* When a pin is `Some(v)`, `observation.simulation_result` **MUST** be `Some`
  and the corresponding `m0X_engine_version` string **MUST** equal `v` exactly.
* If the pin is `Some` and `simulation_result` is `None` or the version string
  differs → `ERROR` / `EnginePinMismatch`.
* If the pin is `None` → no comparison for that pin.

**m07_label (opaque semantic pin; AD-15 remains OPEN):**

```text
m07_label is an opaque semantic pin string.

M07 assigns no versioning semantics to this string.

M07 does not define whether the label represents:
    specification version,
    implementation version,
    build version,
    runtime version,
    or another version concept.

M07 only performs exact string comparison when a case declares m07_label.
```

```text
case.engine_pins.m07_label = Some(L)
AND
observation.m07_label = Some(L')
requires L == L' (exact).
Otherwise → ERROR / EnginePinMismatch.

case declares Some(L) AND observation.m07_label is None
→ ERROR / EnginePinMismatch.

case omits m07_label → no m07_label comparison is performed.
```

```text
M07 MUST NOT discover m07_label.
M07 MUST NOT generate m07_label.
M07 MUST NOT infer m07_label.

The runtime/API mechanism supplying the label remains outside this
specification and constrained by AD-15.
```

Absent `engine_pins` → no pin checks; no invented pins.

### 14.2 Mismatch ordering and ordinals (M07-A7)

```text
Mismatch evidence ordering is normative.

For M07 expectations:
    mismatches MUST be appended in declaration order.

The first evaluated applicable unsatisfied expectation receives ordinal 0.
Subsequent M07 mismatch evidence receives monotonically increasing ordinals.

Skipped expectations (applicable=false) receive no mismatch evidence and
consume no ordinal.
```

When M06 composition is enabled:

```text
All M07 expectation mismatch evidence is appended first.

If M06 returns MISMATCH:
    exactly one M06Composition mismatch evidence item is appended after
    all M07 expectation mismatch evidence.

The M06Composition evidence item receives the next mismatch ordinal.
```

```text
M07 MUST NOT sort mismatch evidence by:
    expectation class,
    expectation_id,
    payment_id,
    phase,
    severity,
    lexical ordering,
    hash ordering,
    map ordering,
    or any other derived ordering.
```

---

## 15. M06 composition boundary (F-07)

M07 **MUST NOT** re-implement M06 expectation classes or call M06 internals as
a hidden second engine beyond the composition contract below.

### 15.1 Binding identity

```text
M06Binding
├── m06_case_id      : string   // required non-empty
├── m06_case_version : string   // required non-empty
└── regression_result : RegressionResult   // required when binding present
```

Gate-6 composition supplies the **already-evaluated** M06 `RegressionResult`
together with the identity of the M06 case it claims to represent.

Validation:

1. `regression_result.case_id` **MUST** equal `m06_case_id` (exact).
2. `regression_result.case_version` **MUST** equal `m06_case_version` (exact).
3. On mismatch → `ERROR` / `M06CompositionError` (binding identity failure).

M07 does **not** invoke `evaluate_regression` inside its compare entrypoint.
Orchestration that produces `RegressionResult` is **external** (same pattern as
observation production for M06 itself).

### 15.2 Composition policy

```text
ComposeRequireM06Match (Gate-6 default when m06_binding present):
  M07 MATCH requires M06 MATCH
  M06 MISMATCH ⇒ M07 MISMATCH
       with one structured mismatch evidence item:
         class = "M06Composition"
         expected = Match
         observed = M06 verdict + case identity
  M06 ERROR ⇒ M07 ERROR / M06CompositionError
       mismatches[] empty
```

### 15.3 Provenance

When composed, `PaymentSettlementProvenance` **MUST** include
`m06_case_id` / `m06_case_version` and the M06 verdict kind.

---

## 16. M04 integration

Optional `scenario_binding` **MUST** match `SimulationResult` identity fields
when `simulation_result` is present (exact string equality).

M07 **MUST NOT** call M04 generation.

---

## 17. External provider & adapter

### 17.1 Provider neutrality (P5)

Provider API payloads, webhooks, timestamps, transaction ids, and provider
balances are **not** economic truth.

### 17.2 Adapter contract (semantic)

```text
ExternalPaymentSettlementObservation
        ↓  deterministic explicit field map
SettlementObservation
```

Adapter **MUST**:

* map fields explicitly;
* reject incompatible / ambiguous values (`ExternalAdapterError`);
* reject any source amount whose source asset ≠
  `case.payment_declaration.asset_id` **before** constructing
  `SettlementObservation` (`ExternalAdapterError`) — §8.1 / R-F1;
* preserve exact `i128` amounts without conversion;
* preserve status via an explicit status map table;
* preserve provenance (`adapter_id`, `adapter_version`);
* never fabricate M01 effects or balances.

Adapter **MUST NOT**:

* silently repair invalid data;
* invent missing amounts as zero;
* convert or reprice across assets;
* use provider clocks as M07 delay authority.

Wire formats remain **AD-14 OPEN** — this section freezes **semantic mapping
requirements only**.

---

## 18. Verdict model

```text
PaymentSettlementVerdict = MATCH | MISMATCH | ERROR
```

| Verdict | Meaning |
| --- | --- |
| `MATCH` | All applicable M07 expectations satisfied; composed M06 (if any) MATCH; no ERROR |
| `MISMATCH` | ≥1 applicable unsatisfied expectation; evaluation completed |
| `ERROR` | Case/observation/adapter/numeric/composition could not be evaluated |

Non-collapse:

```text
M01 Rejected/FailedEconomic ≠ M07 MISMATCH
M02 FAIL ≠ M07 MISMATCH
M03 Fatal ≠ M07 ERROR
M06 MISMATCH ≠ automatic unless m06_binding composition applies
```

unless an explicit expectation / composition rule declares the relationship.

---

## 19. Error taxonomy (closed; M07-A9)

| error_id | Meaning |
| --- | --- |
| `InvalidPaymentCase` | Case malformed / empty ids / duplicate expectation ids / negative expected amount on a `>= 0` expectation payload (R-G3) |
| `InvalidSettlementCase` | Settlement declaration inconsistent (incl. §7.3 static conflicts) |
| `InvalidSettlementObservation` | Status/phase pair invalid, structural field rules violated, required observation amount consistency fails, observation amount sign domain violated (§8.1), or another explicit observation-structure rule fails |
| `ObservationBindingMismatch` | Binding ≠ observation identity |
| `MissingObservationField` | An applicable expectation requires an optional observation field and that field is absent; absence **MUST NOT** become zero |
| `AmbiguousObservation` | Reserved — **MUST NOT** be emitted on Gate-6 single-snapshot path |
| `IncompatibleObservationType` | Observation supplied to the M07 evaluation boundary whose semantic type cannot be interpreted as `SettlementObservation`. External provider payload mapping failures remain `ExternalAdapterError`. **MUST NOT** be used as a generic substitute for invalid field values |
| `InvalidAmount` | Declared amount semantics structurally invalid (negative gross/requested/settled/unsettled/refund/fee; fee_amount > gross where prohibited; unsupported cross-asset fee declaration) |
| `NumericComparisonError` | A required checked arithmetic operation cannot be represented or safely completed (e.g. `checked_sub` overflow/underflow) |
| `UnsupportedSettlementState` | `settlement_status` value is outside the closed `SettlementStatus` taxonomy |
| `ExternalAdapterError` | Adapter rejected / ambiguous map |
| `EconomicStateMismatch` | Reserved for future amendment — Gate 6 evaluators **MUST NOT** emit unless an amendment adds a class |
| `M06CompositionError` | Bound M06 evaluation returned ERROR or binding identity failed |
| `EnginePinMismatch` | Optional M07 engine_pins failed exact compare (§14.1) |
| `EngineError` | Internal M07 fault |

**Status / phase / observation classification order:**

```text
unknown status
    → UnsupportedSettlementState

known status + unknown phase
    → InvalidSettlementObservation

known status + known phase + invalid pair (§11.4)
    → InvalidSettlementObservation
```

Human `reason` strings are diagnostic only.

---

## 20. Mismatch evidence

```text
MismatchEvidence
├── expectation_id
├── class
├── target
├── expected
├── observed          // actual observed value — NOT absolute difference
├── operator          // Exact | AbsoluteTolerance
├── mismatch_class
├── ordinal
├── settlement_phase
├── payment_id
├── asset_id
└── provenance_ref
```

For absolute-tolerance mismatches, `observed` **MUST** be the observed amount;
difference is not stored as `observed`.

`ordinal` and append order are normative per §14.2 (declaration order for M07
expectations; one `M06Composition` evidence item after all M07 mismatches when
applicable). No reordering by class, id, severity, or derived keys.

---

## 21. Provenance (semantic; AD-14 deferred)

```text
PaymentSettlementProvenance
├── case_id / case_version
├── payment_id / payment_version / configuration_id
├── scenario binding fields if present
├── observation engine pins (m01/m02/m03 as present on SimulationResult)
├── observation.m07_label when present   // opaque semantic pin; not runtime discovery
├── adapter_id / adapter_version if present
├── m06 case identity if composed
├── m07_semantic_label                   // copy of observation.m07_label when Some; else omitted
└── evaluation_policy   // ALL_MISMATCHES
```

`m07_semantic_label` / `observation.m07_label` are **opaque semantic pin
strings** supplied with the observation package. M07 assigns **no versioning
semantics** to the string and does not discover them at runtime
(**AD-15 OPEN**).

No wire serialization in Gate 6.

---

## 22. Determinism

```text
same PaymentSettlementCase + same SettlementObservation
= same PaymentSettlementResult
```

Forbidden as hidden authority: RNG, wall clock, filesystem order, unordered
map/set iteration affecting verdict, thread scheduling, network, provider
timing, unbound environment variables, float economic compare, LLM judgment.

---

## 23. Semantic properties

```text
P1  Economic Truth Preservation — M07 never replaces M01
P2  Settlement Determinism — equivalent inputs ⇒ equivalent results
P3  Observation Integrity — missing/ambiguous ≠ silent success
P4  Authority Separation — M07 ≠ M01/M02/M03/M04/M05/M06
P5  Provider Neutrality — adapters translate; providers are not truth
P6  Exactness — i128 checked arithmetic; no silent conversion
P7  Lifecycle Separation — Failed/Pending/Partial/Reversed/Refunded distinct
```

---

## 24. Open decisions impact

| ID | Impact on Gate-6 M07 |
| --- | --- |
| AD-03 | CONSTRAINED (identity algorithms); declared ids sufficient |
| AD-12 | NO_DEPENDENCY |
| AD-14 | CONSTRAINED (no wire schema); semantic model only |
| AD-15 | CONSTRAINED (provisional Rust types allowed later; not frozen here) |
| M06-OD-01/02/03 | NO_DEPENDENCY (remain OPEN; composition consumes frozen M06) |
| DC-13 | NO_DEPENDENCY for closing; composition remains explicit |
| OD-07 | NO_DEPENDENCY (no CLI/API) |
| DC-09 | **CONSTRAINED** — Domain settlement model details; M07 statuses are testing-layer |
| DC-06 | **CONSTRAINED** — refund/reversal accounting depth |

### M07-specific open decisions

| ID | Question | Blocks Gate-6 freeze? |
| --- | --- | --- |
| M07-OD-01 | Escrow as first-class M01 primitive vs observation-only | **NO** for observation-only path; **YES** if M01 lock required |
| M07-OD-02 | Whether `EconomicStateMismatch` gains a Gate-6 expectation class | **NO** (reserved; unused) |
| M07-OD-03 | Depth of refundable-basis rules before DC-06 closes | **NO** for status/amount observation compare |

---

## 25. DC-09 / DC-06 explicit split

### Independent (Gate 6 may freeze later without closing DC-09/DC-06)

* testing-layer `SettlementStatus` taxonomy
* case / observation / expectation envelopes
* fee inclusive/exclusive declaration modes
* partial amount fields + consistency checks
* adapter semantic mapping requirements
* verdict / error / evidence / determinism
* M06 composition boundary

### Constrained (must not pretend Domain is closed)

* Domain settlement state model details (**DC-09**)
* payment-provider settlement semantics (**DC-09**)
* refund/reversal accounting & history rewrite rules (**DC-06**)
* escrow economic lock primitives (needs M01/Domain if required)

---

## 26. Acceptance criteria (normative for future implementation)

A conforming implementation **MUST** provide deterministic tests covering:

* lifecycle status exact match / mismatch (single snapshot)
* `SettlementExecutionExact` for `NotAttempted` / `Attempted` / `Executed`
* execution not inferred from status or M01 effects
* every status↔phase pair validated by closed §11.4 matrix
* invalid status/phase → InvalidSettlementObservation (no “typical” phase)
* status/phase validity does not imply lifecycle history
* partial settlement non-failure; amounts do not imply Failed
* `terminal_partial` / `_refund` are declaration metadata only; never
  independently MATCH/MISMATCH; never inferred from amounts/status/phase;
  `Some(false)` is not a non-terminal observation assertion (§7.3)
* `terminal_partial` static contradiction only vs `SettlementStatusExact(Settled)`;
  `terminal_partial_refund` creates **no** case-validation contradiction (R-A4)
* settlement-lane `Failed` vs `RefundOutcome::Failed` distinction
* refund failure leaves settlement_status unchanged unless separately declared
* `RefundAmountExact` is observation compare only; no refundable-basis derivation
* delay via explicit marker / steps (not wall clock)
* fee sole authority = `FeeDeclaration` when `fee = Some`; required
  `fee_amount` / `fee_asset` / `fee_timing` / `fee_mode`; no top-level
  `fee_mode` (R-A3); fee_timing ⊥ fee_mode (R-F3); fee sign ≥ 0;
  fee_asset = payment asset; FeeExclusive / FeeInclusive / FeeDeclaredOnly;
  no silent net under FeeDeclaredOnly
* `derived_net_settlement_amount` is case-side only; never mutates observation;
  `NetSettlementExact.expected_net_settlement_amount` compares observation;
  under FeeExclusive/Inclusive expected MUST equal derived (R-F2)
* amounts: gross/requested/settled/unsettled/refund/fee ≥ 0; net MAY be signed;
  asset identity = `payment_declaration.asset_id` only; adapter source-asset
  mismatch → ExternalAdapterError (R-F1 / R-F4)
* every expectation class has a closed payload schema (R-F5)
* `gross_amount` not silently substituted for `requested_amount` (§8.3)
* Settled/PartiallySettled: `observation.requested_amount` ==
  `settlement_declaration.requested_amount` else InvalidSettlementObservation
  (R-A1)
* Settled: `settled_amount` == `requested_amount`; unsettled present ⇒ 0
  (R-A2)
* PartiallySettled: `0 < settled < requested`, `unsettled > 0`, subtraction
  equality; settled=0 / settled=requested / settled>requested invalid (R-G1)
* matrix **A** fields MUST be `None`; `Some(_)` → InvalidSettlementObservation
  (R-G2)
* negative expected settled/fee/refund amounts → InvalidPaymentCase (R-G3);
  NetSettlementExact expected remains signed
* refund / partial refund / reversal as **current claims** (no history inference)
* escrow observation-only; `EscrowStatusExact` only; `escrow_amount` unevaluated;
  missing `escrow_status` when applicable → MissingObservationField
* binding mismatch
* required-field matrix violations → InvalidSettlementObservation;
  applicable Option absence → MissingObservationField (never zero)
* absolute tolerance evidence uses observed amount
* ERROR clears mismatches (§14.0)
* mismatch ordinals follow declaration order; M06Composition after M07 (§14.2)
* applicable=false skips lookup and consumes no ordinal
* empty expectations → MATCH
* contradictory expectations → MISMATCH (§12 contradiction priority)
* no duplicate `expected_status` case field (status only via expectations)
* M06 composition: identity match, MATCH/MISMATCH/ERROR per §15
* engine pins: opaque `m07_label` exact compare; no version semantics; no
  discovery/generation; AD-15 remains OPEN; missing comparable label →
  EnginePinMismatch
* adapter reject ambiguous map
* determinism repeated evaluation
* authority: no M01 evaluate / M03 run / M04 generate / M02 evaluate / M06
  reimplementation inside M07 compare entrypoint

---

## 27. Implementation boundary

This frozen specification **MUST NOT** be read as authorization to create:

```text
crates/aivoguard/src/payment/
crates/aivoguard/src/settlement/
```

or any M07 crate, tests, serde, DB, HTTP, CLI, SDK, or provider adapters.

Implementation requires a separate explicit authorization task after this freeze.

---

## 28. Self-audit checklist

| Area | Status after TASK-34 |
| --- | --- |
| Authority matrix | Explicit §3 |
| Lifecycle + statuses | §6–§7 |
| Terminal partial metadata | §7.3 |
| Amounts / asset / sign domain | §8.1 (R-F1 / R-F4) |
| Fees / timing⊥mode / net consumer | §8.2 (R-F2 / R-F3) |
| PartiallySettled strict invariant | §8.4 (R-G1) |
| requested_amount / Settled invariants | §8.3–§8.4 |
| Refund / reverse / escrow | §9 |
| Delay without wall clock | §10 |
| Single-snapshot observation | §11.0 |
| Status↔phase + field matrix A=absent | §11.4–§11.5 (R-G2) |
| Closed expectation payloads + R-G3 | §12 |
| Case model | §13 |
| ERROR / pins / ordinals | §14 |
| M06 composition | §15 |
| Adapter asset reject path | §17 (R-F1) |
| Verdict / errors / evidence | §18–§20 |
| Provenance / determinism | §21–§22 |
| Open decisions preserved | §24–§25 |

---

## 29. Document control

```text
TASK-26 draft
+ TASK-27 (F-01…F-09)
+ TASK-27A (R-01…R-04)
+ TASK-28 (M07-A1…A10)
+ TASK-29 / TASK-30 (R-A1…R-A5)
+ TASK-31 / TASK-32 (R-F1…R-F5)
+ TASK-33 / TASK-34 (R-G1…R-G3)
+ TASK-35 independent final re-audit: PASS
+ TASK-36 normative freeze

STATUS: FROZEN
NORMATIVE FREEZE: FROZEN
TASK-35: PASS
TASK-36: FREEZE
IMPLEMENTATION_BLOCKED

OPEN DECISIONS PRESERVED:
  AD-03, AD-12, AD-14, AD-15,
  M06-OD-01/02/03, DC-13, OD-07,
  DC-09, DC-06, M07-OD-01/02/03

MUST NOT:
  implement without separate authorization
  silently amend this frozen contract
  close DC-09 / DC-06 / AD-14 / AD-15 / M07-OD-01
  introduce M01 escrow primitives
  introduce provider-specific semantics
```
