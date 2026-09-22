# AivoGuard Economic Invariant Engine Specification

| Field | Value |
| --- | --- |
| Document | `docs/design/economic-invariant-engine-specification.md` |
| Task | TASK-05 / TASK-05R / TASK-05R2 / TASK-05R3 |
| Module | **M02 — Economic Invariant Engine** |
| Gate | Gate 2 — Invariant Engine |
| **STATUS** | **READY_FOR_REVIEW** |
| Normative freeze | **NOT FROZEN** |
| Implementation | **BLOCKED** |
| Remediation | TASK-05R / TASK-05R2 / TASK-05R3 semantic closure |
| Depends on | Product Scope (**FROZEN**); Domain Contract (**FROZEN**); Economic Kernel Specification (**FROZEN**); M01 implementation (TASK-04 **PASS**) |

This document is the candidate normative specification for M02 after TASK-05R3
final semantic closure. It does **not** authorize M02 implementation.
Freeze requires explicit TASK-05F approval.

Related:

* [`product-scope.md`](product-scope.md)
* [`domain-contract.md`](domain-contract.md)
* [`economic-kernel-specification.md`](economic-kernel-specification.md)

---

## 1. Purpose

M02 evaluates whether **declared economic invariants** hold over authoritative
M01 economic data (states, transitions, transactions, events, and declared
history).

Conceptually:

```text
EconomicWorld
+
EconomicState / StateTransition / History
+
KernelOutcome (when relevant)
+
Declared Invariants
        ↓
Invariant Evaluation
        ↓
Invariant Results
        ↓
Violations
```

M02 is an **invariant evaluation layer**. It is **not** the Economic Kernel.

---

## 2. Authority hierarchy

```text
Product Scope FROZEN
        ↓
Domain Contract FROZEN
        ↓
Economic Kernel Specification FROZEN
        ↓
M02 Invariant Engine Specification (this document; READY_FOR_REVIEW)
        ↓
M02 Implementation (blocked)
```

M02 must **not** redefine M01 economic truth.

M01 remains authoritative for:

```text
state transitions
transaction disposition
economic effects
balances
money
fees
prices
settlement effects
economic execution result
```

M02 evaluates declared properties **over** that authoritative result.

---

## 3. Core distinctions (mandatory)

```text
Economic Truth          (M01)
        ≠
Invariant Evaluation    (M02)
        ≠
Test Expectation        (Scenario / regression / test harness)
```

### Examples

```text
Kernel:
  Transfer rejected because balance is insufficient
M02 invariant:
  "available balance must never become negative"
Result:
  PASS   (property holds on authoritative StateAfter / non-mutation)
```

```text
Kernel:
  Transfer accepted; StateAfter has negative available balance
M02 invariant:
  "available balance must never be negative"
Result:
  FAIL
```

M02 must **never** rewrite the Kernel’s economic outcome, disposition, effects,
or balances.

---

## 4. Responsibilities

### M02 is responsible for

* evaluating declared invariants
* determining whether an invariant holds
* producing deterministic invariant results
* producing violations
* associating violations with evaluation/state/transition/history location
* evaluating invariants over state, transition, and history (where supported)
* preserving deterministic evidence
* distinguishing invariant failure from invariant engine error
* supporting reproducible invariant evaluation

### M02 is NOT responsible for

* executing Actions
* mutating EconomicState
* determining transaction disposition
* calculating fees / prices / applying balances / settlement
* simulations, adversarial generation, chaos, regression orchestration
* LLM authority, external I/O, payment providers, blockchain RPC
* generic cybersecurity assertions
* deciding whether a violation was “expected” by a test

---

## 5. Read-only guarantee

Invariant evaluation **MUST NOT** mutate authoritative economic state.

```text
Authoritative State
        ↓
M02 Evaluation
        ↓
Invariant Results / Violations
```

An invariant may report a violation. It must **not** repair it.

Prohibited automatic behavior:

```text
balance correction
rollback
fee correction
state normalization
```

---

## 6. Invariant concept

An **Invariant** is:

> A deterministic, declared economic property that must hold for a specified
> evaluation scope under a declared EconomicWorld and authoritative economic
> data.

Each invariant must establish:

```text
what property is being evaluated
what data it applies to
when it applies
what constitutes satisfaction
what constitutes violation
```

Concrete DSL / macro / JSON / YAML / LLM prompt languages are **not** frozen
here (OD-04 deferred). Semantics of evaluation **are** frozen by this contract.

---

## 7. Invariant identity and versioning

Every declared invariant requires stable semantic identity distinguishing at
least:

```text
invariant_id
definition_version
applicable World binding (or explicit “World-agnostic with parameters”)
scope kind
```

Display names are non-authoritative.

A change that alters whether identical authoritative inputs PASS/FAIL/ERROR is a
**semantic version change** and requires an explicit definition_version bump
and specification/amendment discipline for normative shared invariants.

---

## 8. Invariant scopes

M02 MUST distinguish at least:

### 8.1 State invariant

Evaluates one authoritative `EconomicState`.

Example (conceptual): available balance ≥ 0 **when World prohibits negatives**.

### 8.2 Transition invariant

Evaluates:

```text
StateBefore
+
Transaction / Effects
+
StateAfter
(+ disposition when required by the invariant)
```

Example (conceptual): material StateAfter differences are explained by declared
effects (user-level assertion layer; foundational consistency remains M01).

### 8.3 History invariant

Evaluates a declared **ordered** sequence of authoritative economic
records/transitions.

Example (conceptual): settlement-related event cannot precede authorization
event in logical order.

History invariants **MUST NOT** assume wall-clock order. They use declared
logical ordering only.

---

## 9. Evaluation target resolution

```text
Invariant
+
EvaluationTarget
        ↓
InvariantResult
```

Conceptual targets:

```text
State
Transition
Transaction
Effect
Event
History
```

### Compatibility matrix (normative)

| Invariant scope | Compatible targets | Incompatible → |
| --- | --- | --- |
| STATE | `State` | any other → `ERROR` (`INCOMPATIBLE_TARGET`) |
| TRANSITION | `Transition` (StateBefore + Transaction/Effects + StateAfter) | State-only or History-only → `ERROR` (`INCOMPATIBLE_TARGET`) |
| HISTORY | `History` | State/Transition-only → `ERROR` (`INCOMPATIBLE_TARGET`) |

### Target type vs missing data (TASK-05R2)

```text
TARGET TYPE / SCOPE MISMATCH
→ ERROR(INCOMPATIBLE_TARGET)

TARGET IS COMPATIBLE
but required authoritative field/data is absent
→ ERROR(MISSING_REQUIRED_DATA)
```

Examples:

```text
HISTORY invariant + State target
→ ERROR(INCOMPATIBLE_TARGET)

TRANSITION invariant + History target
→ ERROR(INCOMPATIBLE_TARGET)

HISTORY invariant + History target
  + required logical-order field absent
→ ERROR(MISSING_REQUIRED_DATA)

STATE invariant + State target
  + required account missing
→ ERROR(MISSING_REQUIRED_DATA)

STATE invariant + State target
  + required balance facet absent
→ ERROR(MISSING_REQUIRED_DATA)
```

Additional rules:

* `Transaction` / `Effect` / `Event` targets are valid only when the invariant’s
  structured definition explicitly binds that target kind; otherwise
  `ERROR(INCOMPATIBLE_TARGET)`.
* Compatible History target lacking required logical-order keys →
  `ERROR(MISSING_REQUIRED_DATA)` (never PASS; never infer order from
  wall-clock / arrival / DB insertion / filesystem / thread scheduling).

Rust types and serialization are deferred.

---

## 10. Invariant result model

Deterministic result categories (unchanged; **no fourth category**):

| Result | Meaning |
| --- | --- |
| `PASS` | Evaluation completed under this contract; no property violation |
| `FAIL` | Evaluation completed; property does not hold (one or more Violations) |
| `ERROR` | Property could not be evaluated correctly |

```text
FAIL ≠ ERROR
```

`PASS` includes both:

1. property checked and holds; and
2. valid invariant that is **not applicable** to the World (see §10A), which
   produces PASS with zero Violations and mandatory evidence
   `applicability_status = NotApplicable`.

Rationale for not adding `NOT_APPLICABLE` as a fourth result: changing the
result model would be a breaking semantic amendment; vacuous non-applicability
is representable inside PASS with explicit evidence so harnesses can distinguish
it from “property verified true”.

---

## 10A. Applicability semantics (TASK-05R / TASK-05R3)

Normative distinctions:

| Situation | Result |
| --- | --- |
| Applicable invariant; property holds | `PASS` (`applicability_status = Applicable`) |
| Applicable invariant; property false | `FAIL` + Violation(s) |
| Valid invariant; World applicability constraints false | `PASS` (`applicability_status = NotApplicable`), zero Violations |
| Malformed / invalid invariant definition | `ERROR` (`INVALID_INVARIANT_DEFINITION`) |
| Target incompatible with declared scope | `ERROR` (`INCOMPATIBLE_TARGET`) |
| Applicable invariant; required authoritative data missing | `ERROR` (`MISSING_REQUIRED_DATA`) |
| Applicability itself cannot be decided because required applicability inputs are missing | `ERROR` (`MISSING_REQUIRED_DATA`) |

### Applicability vs missing data (TASK-05R3)

Applicability may be resolved **only** from data explicitly required by the
applicability declaration.

```text
applicability established false from available authoritative World/configuration facts
→ PASS + applicability_status = NotApplicable

determining applicability requires authoritative data that is missing
→ ERROR(MISSING_REQUIRED_DATA)

missing required evaluation data MUST NOT be interpreted as
“World does not support this concept”
```

Examples:

```text
World explicitly declares no collateral concept
→ PASS + NotApplicable

World supports collateral, but required collateral state field is absent
→ ERROR(MISSING_REQUIRED_DATA)

Do not use absence of observed evaluation data as evidence that the World
lacks the economic concept.
```

These MUST NOT collapse:

```text
valid but inapplicable World
≠
invalid invariant definition
≠
incompatible evaluation target
≠
missing required data for an applicable invariant
≠
missing data needed to decide applicability
```

No silent PASS/FAIL without recording applicability status in evidence when
applicability constraints exist.

---

## 10B. Missing authoritative data (TASK-05R)

```text
required authoritative data exists → evaluation proceeds
required authoritative data missing → ERROR (MISSING_REQUIRED_DATA)
observed authoritative data violates property → FAIL
```

Prohibited inferences unless the invariant **explicitly** declares absence
semantics:

```text
missing value → zero
missing value → false
missing value → empty collection
missing value → PASS
```

Distinguish where relevant:

```text
absent data
explicit zero
empty collection
unknown value
```

Implementations must not invent missing economic values.

---

## 11. Violation model

A **Violation** is a first-class deterministic record that an invariant FAILed.

It must answer:

```text
Which invariant failed?
Where did it fail?
What condition did the invariant require?
What was observed?
What authoritative state/transition/history was involved?
Why does the invariant not hold? (structured facts)
```

Minimum semantic fields:

```text
invariant identity (id + definition_version)
evaluation scope
evaluation location
required_condition (structured)   — condition required by the invariant
observed_condition (structured)   — authoritative observation
authoritative reference (state / transition / history span)
deterministic context facts
```

**Terminology (TASK-05R):** `required_condition` is **not** the harness “expected
test outcome”. Test expectation matching remains outside M02.

Human-readable prose may be derived; **structured facts are authoritative**.

### Multiple violations and partial evaluation (TASK-05R / TASK-05R2)

Policies declared on the invariant:

```text
all         — retain all Violations discovered in the required domain
first-only  — retain only the first Violation in deterministic domain order
```

**Normative rule (TASK-05R2):** `first-only` controls **violation collection**,
not required-domain **error discovery**.

```text
required evaluation domain
        ↓
must be evaluated completely enough to determine
whether any required subdomain produces ERROR
        ↓
violation collection policy controls retained Violations only
```

Therefore:

```text
all:
  evaluate all required subdomains
  retain all Violations
  any required ERROR → overall ERROR

first-only:
  evaluate all required subdomains
  retain only the first deterministic Violation
  any required ERROR → overall ERROR
```

The implementation MUST NOT short-circuit required evaluation merely because
the first Violation has been found. If later members of the required domain
are not to be evaluated, that MUST be an explicit invariant semantic — not an
accidental consequence of `first-only`.

**Deterministic “first” Violation:** “First” is determined solely by the
authoritative deterministic domain order already established by the inputs
(e.g. ordered account keys). Example: ordered accounts `A,B,C,D` with
violations at `C` and `D` → `first-only` retains the Violation at `C`.
Prohibited: thread completion order, HashMap iteration, filesystem order,
network order. If the domain has no deterministic authoritative ordering →
`ERROR` (do not invent a sorting rule that changes domain semantics).

**ERROR precedence over incomplete FAIL:**

If any **required** subdomain cannot be evaluated (`ERROR` cause), the overall
invariant result is `ERROR`. A retained Violation MUST NOT hide an ERROR.
Do **not** report overall `FAIL` while any required subdomain ERRORed.

Normative scenarios (`first-only`, ordered accounts `1..10`):

| Situation | Overall | Retained Violations |
| --- | --- | --- |
| Account 3 violates; account 7 missing required data | `ERROR(MISSING_REQUIRED_DATA)` | may record diagnostic for account 3; **not** a complete FAIL |
| Accounts 3 and 7 violate; no ERROR | `FAIL` | only account 3 Violation |
| Accounts 3 and 7 both missing required data | `ERROR` | none (errors are not Violations); class via §21 precedence among discovered errors |
| Account 2 missing required data; account 5 violates | `ERROR` | Violation may be observed diagnostically; overall remains ERROR |
| Account 5 violates; later account 8 missing required data | `ERROR` | first Violation (account 5) may be diagnostic; overall ERROR |

Evaluation errors are **not** converted into Violations.

When multiple ERROR causes are discovered in the required domain, select the
reported class by §21 precedence among causes **actually discovered** — do not
manufacture higher-precedence causes.

---

## 12. Expected violation vs actual violation

M02 reports invariant truth only:

```text
Invariant PASS | FAIL | ERROR
```

M02 must **NOT** decide whether a violation was expected/unexpected by a test.

Harness / Scenario / Regression interpret expected-vs-actual **after** M02.

---

## 13. Invariant categories (extensible; not universal)

Documented categories (illustrative bindings, not mandatory for every World):

```text
Conservation
BalanceConsistency
AssetConservation
LiabilityConsistency
FeeCorrectness
SettlementCorrectness
AuthorizationEconomicValidity
SupplyConsistency
CollateralConsistency
PositionConsistency
StateTransitionValidity
```

M02 must **not** assume every World has every category.

---

## 14. World-specific semantics

Invariants operate under the declared `EconomicWorld`.

```text
World A: negative balances prohibited
World B: negative balances permitted for explicit credit accounts
```

Therefore `balance >= 0` is **not** a universal invariant. Applicability must be
declared by the invariant and/or its World binding.

M02 must never invent economic assumptions absent from World + invariant
declaration.

---

## 15. Conservation

Conservation is **explicitly scoped**, never universal.

```text
World / invariant declares what quantity is conserved
```

A conservation invariant must identify:

```text
conservation property
measurement boundary
asset identity (or declared aggregate definition)
scope (state / transition / history)
```

No silent cross-asset conservation.

---

## 16. Balance consistency

M02 may evaluate World-declared balance relationships (e.g. facet sums) **only**
where the World explicitly declares that relationship.

M01 permits bucket, orthogonal, and other declared facet models. M02 must
respect the World model and must **not** impose universal facet arithmetic.

---

## 17. Fee correctness

M02 may evaluate declared fee invariants over **authoritative M01 fee effects**.

M02 does **not** recalculate fees as an alternative economic authority.

---

## 18. State transition validity (user-declared)

M02 may verify higher-level transition assertions.

Foundational state/effect consistency required for safe M01 evaluation remains
**M01** responsibility. M02 must not become a second economic engine.

---

## 19. Composition of multiple invariants

Multiple invariants over the same target are evaluated as **independent
deterministic evaluations** by default.

```text
Invariant A = PASS
Invariant B = ERROR
Invariant C = FAIL
```

must be representable simultaneously.

Failure/error of one must not suppress unrelated invariants unless a composite
invariant explicitly defines dependent evaluation.

Do not collapse the batch into a single generic FAIL at the M02 layer; batch
aggregation for CI/tests belongs to a later composition/harness layer unless a
frozen composite invariant defines otherwise.

---

## 20. Short-circuiting

Default:

```text
Independent invariants: evaluate all independently (no accidental order dependence).
Composite invariant: may short-circuit only if its semantics explicitly define it.
```

Evaluation order among independent invariants must not change their individual
PASS/FAIL/ERROR results. Where emission order of results matters for evidence,
use deterministic ordering by `invariant_id` (lexicographic) then
`definition_version`.

**Constraint (TASK-05R2):** `first-only` violation policy is **not** a license
to short-circuit required-domain ERROR discovery (§11). Any short-circuit of
required members must be an explicit invariant semantic, not implied by
violation-collection policy.

---

## 21. Invariant ERROR classification (TASK-05R / TASK-05R2 / TASK-05R3)

Exactly eight conceptual ERROR cause classes (Rust enum names **not** frozen).
Do **not** introduce a ninth class.

| Class | Deterministic rule (TASK-05R3) | Kind |
| --- | --- | --- |
| `INVALID_INVARIANT_DEFINITION` | Malformed or structurally invalid invariant / reference / binding | definition problem |
| `INVALID_INVARIANT_CONFIGURATION` | Well-formed invariant configuration inconsistent with the declared World / invariant configuration | configuration problem |
| `INCOMPATIBLE_TARGET` | Evaluation target type/scope incompatible with the invariant | target problem |
| `MISSING_REQUIRED_DATA` | Target is compatible, but required authoritative data is absent | authoritative-data problem |
| `INCOMPATIBLE_OPERANDS` | Operands exist and are well-formed but cannot legally be compared, aggregated, related, or otherwise operated on under declared rules | operation problem |
| `UNSUPPORTED_OPERATION` | Operation is well-formed and requested but is not supported by Gate-2 semantics | operation problem |
| `ARITHMETIC_ERROR` | Checked arithmetic cannot represent the required result, or numeric domain is invalid | arithmetic problem |
| `ENGINE_ERROR` | Unexpected internal engine failure | engine problem |

**Classification discipline (TASK-05R3):** implementations MUST NOT choose among
classes using informal “A or B” judgment. For identical valid authoritative
inputs, the ERROR class MUST be the same. When a condition matches more than
one class description, apply the **lowest** numbered matching rule in the
decision order below (not precedence of severity):

```text
1. Is the invariant/reference/binding malformed or structurally invalid?
   → INVALID_INVARIANT_DEFINITION
2. Else is the (well-formed) configuration inconsistent with declared World?
   → INVALID_INVARIANT_CONFIGURATION
3. Else is the target type/scope incompatible?
   → INCOMPATIBLE_TARGET
4. Else is required authoritative data absent?
   → MISSING_REQUIRED_DATA
5. Else are operands well-formed but illegal to operate on under declared rules?
   → INCOMPATIBLE_OPERANDS
6. Else is the requested operation outside Gate-2 supported operations?
   → UNSUPPORTED_OPERATION
7. Else is arithmetic/numeric domain failure?
   → ARITHMETIC_ERROR
8. Else unexpected internal failure?
   → ENGINE_ERROR
```

### `INCOMPATIBLE_TARGET` vs `MISSING_REQUIRED_DATA` (TASK-05R2)

```text
TARGET TYPE / SCOPE MISMATCH
→ INCOMPATIBLE_TARGET

TARGET IS COMPATIBLE
but required authoritative field/data is absent
→ MISSING_REQUIRED_DATA
```

Do **not** manufacture `INCOMPATIBLE_TARGET` when the target type is compatible
and only a required field is absent. Example:

```text
valid State target + missing balance
→ MISSING_REQUIRED_DATA
(not INCOMPATIBLE_TARGET)
```

### Conversion / valuation ERROR class (TASK-05R3)

```text
cross-Asset operation with no declared authoritative conversion/valuation
→ INCOMPATIBLE_OPERANDS

declared conversion/valuation required but its authoritative price/rate/data absent
→ MISSING_REQUIRED_DATA
```

### Precedence (when multiple independent ERROR causes are actually discovered)

```text
INVALID_INVARIANT_DEFINITION
  > INVALID_INVARIANT_CONFIGURATION
  > INCOMPATIBLE_TARGET
  > MISSING_REQUIRED_DATA
  > INCOMPATIBLE_OPERANDS
  > UNSUPPORTED_OPERATION
  > ARITHMETIC_ERROR
  > ENGINE_ERROR
```

**Discovery rule (TASK-05R2):** Error precedence applies only when multiple
independent error causes are **actually discovered** within the required
evaluation. Do not manufacture an error merely to satisfy precedence.

The same condition must not arbitrarily become FAIL in one implementation and
ERROR in another.

```text
FAIL  = property evaluated; property false
ERROR = property could not be evaluated correctly
```

---

## 22. Determinism

```text
same:
  EconomicWorld
  EvaluationTarget
  Invariant definition (+ version)
  Invariant configuration
  ordered history (when in scope)
  M02 EngineVersion

⇒ same semantic InvariantResult (+ violations) + same ERROR class when ERROR
```

Prohibited hidden influences: clock, randomness, network, database, filesystem,
environment, LLM, process/thread state, unordered map iteration for
authoritative emission.

Result ordering for multi-invariant batches:

```text
invariant_id (lexicographic)
  then definition_version
  then evaluation location / stable target order
```

Collection traversal for quantification/aggregation uses deterministic domain
ordering already established by authoritative inputs (e.g. `BTreeMap` key order
from M01 state), without inventing a sorting rule that changes economic meaning.

---

## 23. Ordering and history

History/transition invariants use **authoritative logical ordering** only.

* Ordered authoritative history → evaluate.
* History target type incompatible (e.g. State supplied) →
  `ERROR(INCOMPATIBLE_TARGET)`.
* Compatible History target missing required logical-order field/record →
  `ERROR(MISSING_REQUIRED_DATA)`.
* Never infer order from wall-clock, arrival, DB insertion, filesystem order,
  or thread scheduling.

---

## 24. Numeric and arithmetic failure semantics (TASK-05R)

M02 consumes M01 authoritative numerics:

```text
i128 minor units
checked arithmetic
Asset identity
no f32/f64 authoritative economics
World-declared rounding when a calculation requires it
```

```text
representable arithmetic → continue
overflow / underflow → ERROR (ARITHMETIC_ERROR)
invalid numeric domain → ERROR (ARITHMETIC_ERROR)
cross-Asset with no declared conversion → ERROR (INCOMPATIBLE_OPERANDS)
declared conversion data absent → ERROR (MISSING_REQUIRED_DATA)
```

Prohibited: wrap, saturate, clamp, approximate, silent round, invent rounding.

If rounding is required, use the World/invariant-declared rule only.

---

## 25. Comparison semantics (TASK-05R)

Supported conceptual operators:

```text
=
!=
<
<=
>
>=
```

Rules:

* Operands must share the same Asset identity **or** an explicit authoritative
  conversion/valuation to a common Asset must be declared (M01 price
  identity/direction/category).
* `100 USD >= 99 USD` → valid comparison domain.
* `100 USD >= 99 EUR` without declared conversion → `ERROR`
  (`INCOMPATIBLE_OPERANDS`), **not** FAIL, **never** PASS by raw integer compare.
* Missing operand → `ERROR` (`MISSING_REQUIRED_DATA`) unless absence is
  explicitly defined.
* No floating-point comparison.

Equality/ordering are integer minor-unit comparisons after any declared
exact conversion into a common Asset domain.

---

## 25A. Quantification semantics (TASK-05R)

Conceptual quantifiers:

```text
FOR_ALL (∀)
EXISTS  (∃)
```

Domain resolution uses deterministic ordered collections from authoritative
data.

| Quantifier | Empty domain | Missing domain collection |
| --- | --- | --- |
| `FOR_ALL` | `PASS` (vacuous truth) | `ERROR` (`MISSING_REQUIRED_DATA`) |
| `EXISTS` | `FAIL` (no witness) | `ERROR` (`MISSING_REQUIRED_DATA`) |

### Domain binding ERROR class (TASK-05R3)

```text
Malformed / non-structural domain binding
→ ERROR(INVALID_INVARIANT_DEFINITION)

Well-formed binding resolving to a value/domain whose type cannot satisfy
the requested quantifier/operation
→ ERROR(INCOMPATIBLE_OPERANDS)

Well-formed request for an operation/quantifier form not supported by Gate-2
→ ERROR(UNSUPPORTED_OPERATION)
```

Do **not** classify a malformed definition as `INCOMPATIBLE_OPERANDS`.

### Nested evaluation

If evaluating a member of a quantified domain yields `ERROR`, the whole
invariant is `ERROR` (no silent skip). If a member yields a Violation under
`all`/`first-only`, apply §11 multi-violation rules, subject to ERROR
precedence.

---

## 25B. Aggregation semantics (TASK-05R)

Gate-2 conceptual aggregations:

```text
SUM
COUNT
MIN
MAX
```

| Op | Empty input | Missing input | Asset rules | Overflow |
| --- | --- | --- | --- | --- |
| `COUNT` | `0` | `ERROR` | N/A (cardinality) | N/A |
| `SUM` | `0` in the **declared** Asset domain of the aggregation | `ERROR` | all terms same Asset unless declared conversion first | `ERROR` (`ARITHMETIC_ERROR`) |
| `MIN` | `ERROR` (no mathematically valid value) | `ERROR` | same Asset domain | N/A |
| `MAX` | `ERROR` | `ERROR` | same Asset domain | N/A |

No silent sentinel monetary values for MIN/MAX empty sets.

`SUM` of mixed Assets without conversion → `ERROR` (`INCOMPATIBLE_OPERANDS`).

---

## 25C. Cross-asset semantics

```text
same Asset → direct arithmetic/comparison allowed
different Assets → explicit authoritative conversion/valuation required
no declared conversion → ERROR(INCOMPATIBLE_OPERANDS)
declared conversion data absent → ERROR(MISSING_REQUIRED_DATA)
```

M02 cannot invent exchange rate, price direction, rounding, or valuation time.
Those remain governed by M01 / World declaration.
---

## 26. History semantics (TASK-05R2 / TASK-05R3)

History invariants operate over **declared authoritative history** composed of
M01-authoritative records (transactions/effects/events/transitions), not
arbitrary logs.

Distinguish:

```text
State
Transaction
Event
Transition
History
```

### History traversal (conceptual; no DSL freeze)

Conceptual traversal operations:

```text
ordered iteration (forward)
ordered iteration (reverse)
position / indexed access (by authoritative logical position)
range/span selection
relationship/order inspection
```

**Required authoritative ordering:** history must carry World/M01-authoritative
logical order keys. M02 MUST consume that order; it MUST NOT manufacture
economic history or silently repair/sort using implementation-specific order.

### Completeness for required domain (TASK-05R3)

History completeness is evaluated **only** relative to the domain explicitly
required by the invariant definition.

M02 MUST NOT infer that logical positions must be contiguous unless the World
or invariant **explicitly** declares contiguous logical positions as a
requirement.

Examples:

```text
A:
Invariant explicitly requires records/positions 10..20.
Record 13 is absent.
→ ERROR(MISSING_REQUIRED_DATA)

B:
Invariant requests all records actually present in an explicitly supplied
history domain.
The domain contains 10,11,12,14,15.
No contiguity requirement exists.
→ not automatically missing data; traverse the present ordered domain

C:
World/invariant explicitly declares a contiguous logical sequence.
Required position is absent.
→ ERROR(MISSING_REQUIRED_DATA)

D:
Required history record exists but its required ordering key is absent.
→ ERROR(MISSING_REQUIRED_DATA)

E:
Duplicate logical ordering keys where a total order is required.
→ ERROR(MISSING_REQUIRED_DATA)
```

M02 MUST NOT invent missing records or silently shrink a declared range.

Traversal membership is whatever the invariant’s structured definition
**explicitly selects** (e.g. all transitions, only transactions, only selected
target kinds). Filtering is declared by the invariant — not inferred by M02.
If the invariant does not declare a filter, traverse the full authoritative
history domain provided as the target.

Rules:

```text
authoritative + ordered + complete for the invariant’s required domain
→ traversal proceeds

compatible History target; required record/order absent relative to
the invariant’s declared required domain
→ ERROR(MISSING_REQUIRED_DATA)

duplicate logical order keys within required domain (total order required)
→ ERROR(MISSING_REQUIRED_DATA)

unordered / non-deterministic history presented as History
→ ERROR(MISSING_REQUIRED_DATA)
  (never silently sort)

supplied target is not a History target
→ ERROR(INCOMPATIBLE_TARGET)

empty authoritative history domain (compatible, ordered, present, empty)
→ empty domain for quantification/aggregation (§25A / §25B / §31F)
```

Never infer history ordering from wall clock, arrival time, database insertion
order, filesystem order, thread scheduling, or HashMap iteration.

Traversal is **read-only**.

### Range / span semantics

If an invariant requests a history span:

* `start <= end` must be valid under authoritative logical ordering.
* Invalid range binding → `ERROR(INVALID_INVARIANT_DEFINITION)`.
* Missing records required by the declared range →
  `ERROR(MISSING_REQUIRED_DATA)`.
* Contiguity inside the span is required only when World/invariant explicitly
  declares contiguous logical positions (§26 completeness examples A/C).
* Do not silently shrink the range.
* Do not silently skip missing records that the invariant requires.

---

## 27. Violation location

Conceptual location may associate a violation with:

```text
state
transaction
effect
event
logical sequence position
history range
```

Location must be deterministic. Concrete Rust shape is implementation-only.

---

## 28. Evidence and reproducibility

Evidence must enable reproduction/understanding of:

```text
which invariant
what target
what World
what State/Transition/History
what configuration
what logical ordering
what required_condition
what observed_condition
what result
why FAIL/ERROR
```

Evidence must also record `applicability_status` when applicability constraints exist.
Reproducibility inputs:

```text
World
+
Target
+
Invariant (+ version)
+
Configuration
+
Ordered history (if any)
+
M02 EngineVersion
```

No external live state may be required. Serialization format deferred.

---

## 29. LLM boundary

```text
LLM proposal
        ↓
deterministic validation / declaration
        ↓
declared invariant
        ↓
M02 evaluation
```

LLM must never determine PASS/FAIL/ERROR, observed economic state, or expected
economic value.

---

## 30. Boundary tables

### M01 vs M02

| Responsibility | M01 | M02 |
| --- | --- | --- |
| Execute Action | YES | NO |
| Determine transaction disposition | YES | NO |
| Calculate state effects | YES | NO |
| Apply authoritative state transition | YES | NO |
| Foundational state consistency | YES | NO |
| User-declared invariant evaluation | NO | YES |
| Conservation rules | World binding; evaluated in M02 | YES |
| Business-specific assertions | NO | YES |
| Produce invariant Violations | NO (foundational errors only) | YES |
| Test expectation matching | NO | NO |

### M02 vs M03

```text
M03 Simulator
  ↓
many kernel evaluations / orchestration
  ↓
M02 invariant evaluation
  ↓
results
```

M02 must not implement scenario scheduling, multi-step execution, Monte Carlo,
agent loops, or timeline orchestration.

### M02 vs M04/M05

Adversarial/chaos modules generate conditions. M02 only evaluates declared
invariants over authoritative results. M02 does not generate attacks.

### M02 vs adapters / I/O

M02 must not perform HTTP, database, blockchain RPC, payment, wallet, or LLM
calls as hidden dependencies.

---

## 31. Conceptual evaluation operations (not a DSL freeze)

OD-04 (concrete language/API) remains **OPEN**. This specification freezes
**semantics**, not syntax.

Conceptual operations:

```text
comparison (§25)
aggregation (§25B)
quantification (§25A)
relationship checking (§31C)
state lookup (§31A)
transition lookup (§31B)
history traversal (logical order) (§26)
```

### 31A. State lookup semantics (TASK-05R2)

Conceptual lookup of authoritative state components (examples):

```text
Actor
Account
Asset
Balance
Balance facet
other explicitly authoritative state component
```

A **valid state reference** is a structured binding that names a component
allowed by the invariant definition and resolvable against the authoritative
evaluation target (§9 / lookup scope below). Lookup is **read-only**: it MUST
NEVER create, normalize, repair, or mutate authoritative state.

#### Zero vs absent vs missing facet

| Situation | Classification | Result |
| --- | --- | --- |
| Referenced entity/component exists; observed value is explicit zero | ZERO VALUE | continue (zero is a valid observation) |
| Account/asset exists; balance quantity is explicit `0` in declared facet | ZERO VALUE | continue |
| Required entity (account/actor/asset/…) not present in authoritative target | ABSENT ENTITY | `ERROR(MISSING_REQUIRED_DATA)` |
| Entity exists; required balance facet not present | ABSENT FACET | `ERROR(MISSING_REQUIRED_DATA)` |
| Required collection present but empty | EMPTY DOMAIN | apply §25A / §25B / §31F (not “missing”) |
| Malformed / unbound lookup identifier in the invariant definition | INVALID REFERENCE | `ERROR(INVALID_INVARIANT_DEFINITION)` |
| Well-formed reference requiring a World capability/configuration the declared World does not provide | WORLD CAPABILITY ABSENT | `ERROR(INVALID_INVARIANT_CONFIGURATION)` |
| Well-formed reference to an operation/kind not supported by Gate-2 | GATE-2 UNSUPPORTED | `ERROR(UNSUPPORTED_OPERATION)` |
| Unrelated Asset without declared relationship/conversion | INCOMPATIBLE OPERANDS | `ERROR(INCOMPATIBLE_OPERANDS)` |

**World-reference classification (TASK-05R3):** implementations MUST NOT choose
between `INVALID_INVARIANT_CONFIGURATION` and `UNSUPPORTED_OPERATION` for the
same input. Use:

```text
malformed reference syntax/binding
→ INVALID_INVARIANT_DEFINITION

valid reference requiring a World capability the declared World does not provide
→ INVALID_INVARIANT_CONFIGURATION

valid reference to an operation/kind not supported by Gate-2
→ UNSUPPORTED_OPERATION
```

**World-declared absence:** if a World/invariant **explicitly** declares that
absence of a named entity is a legitimate observed value for that invariant
(e.g. absence means “no such account” as a boolean observation), then treat
that declared absence semantics as the observation — do **not** invent a
universal default. If absence is required data and no such declaration exists →
`ERROR(MISSING_REQUIRED_DATA)`.

Prohibited silent conversions:

```text
missing account → zero balance
missing asset → zero
missing facet → zero
absent entity → FAIL (unless invariant explicitly defines absence as false)
```

**Lookup scope** resolves only against the authoritative target:

```text
State invariant
→ lookup against authoritative State

Transition invariant
→ lookup against declared StateBefore / StateAfter context
  as explicitly bound by the invariant

History invariant
→ lookup only against state records explicitly present
  in the declared authoritative history/context
```

Do not invent hidden state snapshots.

### 31B. Transition lookup semantics (TASK-05R2)

A **valid transition reference** binds to an authoritative M01 transition
record identified by the invariant’s declared identity (e.g. logical
transition id / transaction correlation as provided by the authoritative
target). M02 does not invent transition ids.

A transition consists conceptually of the authoritative:

```text
StateBefore
Effects / Transaction
StateAfter
Disposition (where explicitly relevant)
```

Lookup is **read-only**.

| Situation | Classification | Result |
| --- | --- | --- |
| Transition exists; all required components present | found | continue |
| Transition exists; effects collection empty (zero effects) | ZERO / EMPTY EFFECTS | continue — **not** missing transition |
| Transition exists; a required named component/field absent | ABSENT COMPONENT | `ERROR(MISSING_REQUIRED_DATA)` |
| Required transition not present in target | ABSENT ENTITY | `ERROR(MISSING_REQUIRED_DATA)` |
| Transition invariant bound to non-transition target | target kind mismatch | `ERROR(INCOMPATIBLE_TARGET)` |
| Malformed transition component binding | INVALID REFERENCE | `ERROR(INVALID_INVARIANT_DEFINITION)` |

**Disposition observation:** M02 may observe `AcceptedEffective`, `Rejected`,
and `FailedEconomic` transition records when they are present in the
authoritative target. Respect M01:

* zero-effect `AcceptedEffective` is a valid transition
* `Rejected` / `FailedEconomic` may have zero effects unless World semantics
  declare otherwise
* absence of an effect MUST NOT imply absence of the transition

M02 must **NOT**:

* reconstruct `StateAfter` from effects
* recalculate balances, fees, prices, settlement, or transaction disposition

M02 consumes authoritative M01 transition data only.

### 31C. Relationship checking semantics (TASK-05R2 / TASK-05R3)

A **relationship** is a declared predicate relating authoritative
values/entities/records. Relationships are **not** a generic graph engine.

Origins:

```text
intrinsic identity/equality over authoritative identifiers
    (e.g. same transaction_id)
World-declared / invariant-declared relations
    (e.g. actor owns account; effect links StateBefore→StateAfter)
```

**Relationship ERROR classification (TASK-05R3) — deterministic, no “A or B”:**

```text
malformed relationship expression/reference
→ INVALID_INVARIANT_DEFINITION

syntactically valid relationship kind not supported by Gate-2
→ UNSUPPORTED_OPERATION

supported relationship whose authoritative endpoints/data are absent
→ MISSING_REQUIRED_DATA

supported relationship whose operands are incompatible under declared
World/invariant semantics
→ INCOMPATIBLE_OPERANDS

relationship exists and holds
→ contributes to PASS

relationship exists and does not hold
→ FAIL + Violation

explicitly declared non-existence
→ evaluate according to invariant semantics (PASS or FAIL as declared);
  not MISSING_REQUIRED_DATA
```

Operands must be resolvable authoritative lookups (§31A / §31B) or literal
constants allowed by the invariant definition. Cross actor/account/asset
relationships are allowed **only** when the invariant/World explicitly
declares that relation:

```text
requested relation kind not in Gate-2 supported set
→ UNSUPPORTED_OPERATION

relation kind supported, but World/invariant does not declare the concrete
relation between these operand kinds
→ INCOMPATIBLE_OPERANDS
```

| Situation | Classification | Result |
| --- | --- | --- |
| Relationship exists and satisfies relation | exists + holds | contribute toward `PASS` |
| Relationship exists but violates relation | exists + fails | `FAIL` + Violation |
| Relationship declared; required endpoint/reference absent | unknown / missing | `ERROR(MISSING_REQUIRED_DATA)` |
| Relationship declared absent as an explicit observation | declared non-existence | evaluate per invariant; **not** missing-data ERROR |
| Relationship operands incompatible under declared rules | incompatible | `ERROR(INCOMPATIBLE_OPERANDS)` |
| Relationship kind not supported by Gate-2 | unsupported | `ERROR(UNSUPPORTED_OPERATION)` |
| Relationship expression malformed | invalid | `ERROR(INVALID_INVARIANT_DEFINITION)` |

Do not silently interpret missing relationship data as false unless the
invariant explicitly defines absence as the observed value.

```text
missing data ≠ observed violation
```

**vs M01 foundational consistency:** M01 remains authoritative for state
transition consistency, effect consistency, transaction/effect consistency,
and numeric validity. M02 may assert a higher-level declared property over
those authoritative records. M02 must not independently calculate whether M01
should have produced different effects.

### 31D. Nested operation error propagation (TASK-05R2)

General rule: if any required nested evaluation yields `ERROR`, the overall
invariant is `ERROR` (no silent skip; no partial FAIL claiming completeness).

Example:

```text
FOR_ALL accounts:
    SUM(balance facets) >= 0

account A → valid
account B → valid
account C → missing facet → ERROR
account D → negative balance → Violation

→ overall ERROR
```

`first-only` does not change this; it changes only retained Violations.

### 31E. Deterministic evaluation model (TASK-05R2 / TASK-05R3)

Normative evaluation sequence:

```text
1. Resolve invariant validity/configuration.
2. Resolve target compatibility.
3. Resolve applicability (§10A):
     - if applicability inputs missing → ERROR(MISSING_REQUIRED_DATA)
     - if NotApplicable → PASS + stop property evaluation
4. Resolve required authoritative data for the (applicable) property.
5. Traverse required evaluation domain deterministically.
6. Evaluate conceptual operations.
7. Collect violations according to policy (all | first-only).
8. If any required evaluation ERRORed:
      overall = ERROR
   else if one or more Violations:
      overall = FAIL
   else:
      overall = PASS
```

Where applicability is `NotApplicable`, the invariant does **not** evaluate its
property and returns `PASS` + `applicability_status = NotApplicable` (§10A).

Missing required evaluation data MUST NOT be treated as NotApplicable.

No hidden partial evaluation.

### 31F. Empty / missing / zero consistency (TASK-05R2 / TASK-05R3)

These concepts MUST NOT be collapsed:

| Concept | Meaning | Typical result |
| --- | --- | --- |
| EMPTY DOMAIN | Collection present; cardinality 0 | FOR_ALL→PASS; EXISTS→FAIL; COUNT→0; SUM→0 (declared Asset); MIN/MAX→ERROR |
| MISSING REQUIRED DATA | Needed authoritative input absent | ERROR(`MISSING_REQUIRED_DATA`) |
| ZERO VALUE | Explicit authoritative 0 | valid observation |
| ABSENT ENTITY | Required entity not in target | ERROR(`MISSING_REQUIRED_DATA`) unless invariant declares absence semantics |
| ABSENT FACET | Entity present; required facet absent | ERROR(`MISSING_REQUIRED_DATA`) |
| WORLD CAPABILITY ABSENT | Well-formed ref needs World capability not provided | ERROR(`INVALID_INVARIANT_CONFIGURATION`) |
| GATE-2 UNSUPPORTED | Well-formed ref/op outside Gate-2 | ERROR(`UNSUPPORTED_OPERATION`) |
| INVALID REFERENCE | Malformed binding in definition | ERROR(`INVALID_INVARIANT_DEFINITION`) |

Preserve closed aggregation/quantification/comparison semantics (§25 / §25A /
§25B). Zero is never a silent substitute for missing.

### 31G. Deterministic ordering scope (TASK-05R2)

Deterministic authoritative order governs:

```text
domain enumeration
target / history traversal
relationship operand evaluation order (when multiple independent checks)
lookup resolution over ordered collections
violation discovery order
error discovery order
retained first-only Violation
```

Ordering MUST come from authoritative inputs (e.g. World/M01 deterministic
key order, declared logical history order). Do not prescribe a Rust collection
type here beyond decisions already recorded in accepted ADRs for M01.

Prohibited influences: HashMap iteration, filesystem order, thread scheduling,
wall clock, environment, network, process state, LLM output.

### Gate-2 implementation-readiness criterion (TASK-05R / TASK-05R2)

> Two independent competent engineers implementing M02 from the frozen Product
> Scope, Domain Contract, Economic Kernel Specification, and this document must
> not be able to choose materially different PASS/FAIL/ERROR outcomes for
> identical valid authoritative inputs solely because this specification leaves
> the relevant semantic question unspecified — including state lookup,
> transition lookup, relationship checking, history traversal, first-only
> behavior, or error classification/precedence.

Given a structured invariant definition specifying identity, version, scope,
target binding, World applicability, derivation rules using the operations
above, and multi-violation policy (`all` | `first-only`), implementations MUST
agree on PASS / FAIL / ERROR (and ERROR class under §21).

Prose-only invariants are not Gate-2 evaluable → `ERROR`
(`INVALID_INVARIANT_DEFINITION`).

### Composition example

```text
FOR_ALL accounts:
    SUM(balance facets of declared Asset) >= 0
```

If `SUM` errors for any account → whole invariant `ERROR` (no skip).

---

## 32. Security / economic safety

M02 must defend against:

```text
silent invariant failure
FAIL → PASS conversion
ERROR → FAIL conversion
cross-asset confusion
floating-point approximation
hidden time dependency
unordered history evaluation
implicit state mutation
incomplete violation evidence
non-deterministic evaluation order
LLM authority over results
missing data treated as zero/PASS
raw cross-asset integer comparison
```

---

## 33. Versioning of this specification

Semantic changes to invariant meaning, evaluation semantics, result categories,
violation semantics, numeric treatment, history/ordering, applicability,
comparison/quantification/aggregation semantics require explicit amendment.
Implementation refactors preserving semantics do not.

---

## 34. Testability (future M02 implementation; not TASK-05)

Future implementation must eventually cover state/transition/history PASS/FAIL,
applicability NotApplicable PASS, missing-data ERROR, incompatible target ERROR,
empty FOR_ALL/EXISTS, empty SUM/MIN/MAX, cross-asset ERROR, arithmetic ERROR,
multi-violation + first-only with full required-domain ERROR discovery,
state/transition lookup, relationship checking, history range missing-record
ERROR, determinism, no mutation, harness-side expected-vs-actual separation.

---

## 35. Open decisions

| ID | Topic | Status | Blocks PASS/FAIL/ERROR agreement? |
| --- | --- | --- | --- |
| OD-04 | Concrete invariant language / API / DSL | OPEN | No (semantics closed; syntax deferred) |
| IE-01 | Concrete Rust types | OPEN | No |
| IE-02 | Persistence/serialization | OPEN | No |
| IE-03 | Shared normative invariant catalog | OPEN | No |
| IE-04 | CI/dashboard aggregation | OPEN | No (harness) |

---

## 36. Deferred

* M02 Rust modules / public API / DSL parser / CLI / DB / adapters / LLM
* Standard invariant catalog
* M03 orchestration bindings beyond conceptual flow
* Fourth result category (`NOT_APPLICABLE`) — explicitly **not** introduced

---

## 37. Contradictions with frozen specs

```text
NONE identified
```

---

## 38. Semantic closure status (TASK-05R3)

```text
SEMANTIC_CLOSURE: COMPLETE for Gate-2 PASS/FAIL/ERROR + ERROR-class determinism
  (deterministic 8-class taxonomy; domain binding; relationship classes;
   World vs Gate-2 unsupported refs; history completeness vs contiguity;
   applicability vs missing data)
TASK-05F: AUTHORIZED for independent freeze review only (not auto-started;
  not claimed complete/frozen)
IMPLEMENTATION: BLOCKED until freeze + explicit implementation task
STATUS: READY_FOR_REVIEW
NORMATIVE FREEZE: NOT FROZEN
GATE 2: NOT CLOSED
```

---

## 39. Next step

```text
TASK-05F — Economic Invariant Engine Specification Freeze
```

Do not implement M02; do not start TASK-05F automatically.

---

## Document control

| Item | Value |
| --- | --- |
| Created by | TASK-05 |
| Remediated by | TASK-05R, TASK-05R2, TASK-05R3 |
| Status | READY_FOR_REVIEW |
| Normative freeze | NOT FROZEN |
| Implementation | BLOCKED |
| Module | M02 |
