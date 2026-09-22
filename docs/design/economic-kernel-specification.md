# AivoGuard Economic Kernel Specification

| Field | Value |
| --- | --- |
| Document | `docs/design/economic-kernel-specification.md` |
| Task | TASK-03 / TASK-03F |
| **STATUS** | **FROZEN** |
| **NORMATIVE FREEZE** | **FROZEN** |
| **GATE** | **GATE 1 — CLOSED** (Economic Kernel M01) |
| Implementation | **BLOCKED** until explicit TASK-04 authorization |
| Depends on | Product Scope (**FROZEN**); Domain Contract (**FROZEN**, Gate 0 **CLOSED**) |

### Freeze statement

```text
TASK-03 semantic review completed.
Economic Kernel Specification is now the normative Gate-1 contract.
Implementation remains blocked pending explicit TASK-04 authorization.
```

This document is the **frozen** implementation-authoritative specification for
the AivoGuard Economic Kernel (M01). It refines Gate-1-critical semantics left
open by the Domain Contract. It must **not** contradict the frozen Domain
Contract.

Gate 1 closure does **NOT** authorize Rust implementation. An explicit
implementation task (**TASK-04**) is required.

No Rust APIs, crate layouts, serialization formats, CLIs, SDKs, or database
schemas are frozen by this document.

Related:

* [`product-scope.md`](product-scope.md) (FROZEN)
* [`domain-contract.md`](domain-contract.md) (FROZEN)
* [`SPECIFICATION-POLICY.md`](SPECIFICATION-POLICY.md)

---

## 1. Purpose

The Economic Kernel is the deterministic core that evaluates **one declared
Action** against explicit:

```text
EconomicWorld
+
EconomicState
+
Action
+
ExecutionContext
```

and produces authoritative:

```text
Economic Result
+
State Effects
+
Transaction Semantics
+
Evidence
```

The kernel is **M01**. It is **not** M02 (Invariant Engine), M03 (Simulator),
or any adapter/integration layer.

M01 must **not** silently introduce scenario orchestration, multi-step
scheduling, timeline simulation, agent loops, Monte Carlo execution, or chaos
execution — those belong to later modules.

---

## 2. Authority hierarchy

```text
Frozen Product Scope
        ↓
Frozen Domain Contract
        ↓
Economic Kernel Specification (this document; **FROZEN**; Gate 1 **CLOSED**)
        ↓
Implementation (blocked until explicit TASK-04 authorization)
        ↓
Tests
```

If any statement here would contradict the Domain Contract, the Domain Contract
wins and this specification must be amended — never the reverse by silent code.

---

## 3. Core evaluation operation

Conceptually (not a public API):

```text
EconomicWorld
+
EconomicState
+
Action
+
ExecutionContext
        ↓
Kernel Evaluation
        ↓
KernelOutcome
```

`KernelOutcome` is either:

* an **economic evaluation outcome** (including accepted/rejected/failed
  dispositions), or
* a **non-economic failure** (invalid input / configuration error / engine
  error).

Non-economic failures must never be rewritten as successful economic PASS.

---

## 4. Responsibilities

### 4.1 Validate declared inputs

The kernel determines whether supplied World, State, Action, and
ExecutionContext are structurally and semantically valid for evaluation.

Invalid input must **not** silently become a valid economic outcome.

### 4.2 Evaluate Action

An Action is intent. The kernel determines its economic disposition under World
rules and State.

```text
Action submitted ≠ Action accepted
```

### 4.3 Produce authoritative state effects

When an Action is accepted and economically effective, the kernel determines
explicit, deterministic, traceable, reproducible changes to EconomicState.

No hidden balance mutation is permitted.

### 4.4 Produce Transaction semantics

The kernel produces the authoritative Transaction record for the evaluation
(when recording is in scope for the evaluation; see §13).

Cardinality (Domain Contract DC-R03):

```text
Transaction may have:
0 state effects
1 state effect
many state effects
```

### 4.5 Produce Events where required

When the World declares economically relevant Events, the kernel defines when
they are generated and how they relate to Action, Transaction, and State
Transition. Storage format is deferred.

### 4.6 Produce Evidence

The kernel produces the minimum evidence required by §23.

### 4.7 Out of responsibility (not M01)

* Arbitrary user-defined invariant language evaluation (M02)
* Multi-step scenario orchestration / scheduling (M03)
* Adversarial/chaos engines (M04/M05)
* Regression case store (M06)
* External adapters (M07/M08/…)
* Test-expectation matching as a substitute for economic truth (DC-R04 /
  scenario layer)

The kernel **may** enforce **foundational state-consistency rules** required for
safe evaluation (§25). That is not the full M02 engine.

---

## 5. Input contract

### 5.1 EconomicWorld

Supplies the rules under which evaluation occurs.

The kernel must **not** invent rules absent from the World.

For Gate 1, a World used with the kernel **must** declare at least:

* Asset identities and unit/scale metadata required by §11
* Balance facet model declaration required by §12
* Action-type rules referenced by the Action being evaluated
* Authorization/eligibility rules with economic effect for that Action
* Fee / price / conversion rules **if** the Action requires them
* Whether the Action type is atomic or explicitly partial (§8)

### 5.2 EconomicState

Authoritative current economic state. Explicit input. No hidden global state.

Gate 1 State must include all holdings/facets material to the Action under the
World’s declared balance model.

### 5.3 Action

Explicit economic intent. The Domain Contract does not require a universal
action enum; Gate 1 likewise does **not** freeze a universal enum.

Minimum semantic requirements for evaluation:

* Action type identity interpretable by the World
* Actor identity (and counterparties as required by the Action type)
* Economically material parameters (amounts, assets, accounts, references)
* Optional Action identifier if provided (§18)

### 5.4 ExecutionContext

Declared inputs that may materially affect deterministic evaluation.

Allowed categories for Gate 1:

```text
logical ordering information (required when order affects truth)
declared timestamp/time input (only if World requires it; never host clock)
randomness seed/source (only if a future amended World requires randomness; Gate 1 default: none)
engine configuration (declared)
World-specific execution parameters (declared)
engine version identity (for determinism/regression identity)
```

**Constraints (TASK-03F):**

* ExecutionContext must **not** become a catch-all for arbitrary hidden inputs.
* Every field must be **declared**, **semantically relevant**, **reproducible**,
  and included in the determinism identity if economically material.
* Undeclared host/process/environment values are prohibited.

**Prohibited as undeclared dependencies:**

```text
host / process / environment state
wall-clock reads
hidden globals
undeclared external I/O
HTTP / database / blockchain RPC / payment provider / LLM / wallet calls
```

Data structure is not finalized here.

---

## 6. Evaluation pipeline

Normative conceptual pipeline (World may refine ordering where explicitly
required, but must remain deterministic):

```text
1. Input validation
2. Authorization / economic eligibility evaluation
3. Action semantic evaluation
4. Economic effect calculation
5. Transaction disposition
6. State transition determination
7. Foundational state-consistency checks (kernel-required)
8. Event / evidence generation
9. Result construction
```

**Normative constraint:** the implementation must not mutate authoritative
EconomicState before validation has established that the operation can
legitimately proceed under the World’s atomicity rules.

For atomic operations, effects are applied all-or-nothing (§8).

---

## 7. Atomicity

For a World-declared **atomic** operation:

```text
Either:
    all authoritative economic effects apply
or:
    none apply
```

A rejected / failed-economic / invalid evaluation must not partially mutate
authoritative economic state.

For a World that **explicitly** permits partial execution:

```text
partial execution = explicit World semantics
```

not an implementation accident. The kernel must **not** infer partial execution
merely because implementation encounters an intermediate failure.

Partiality must be observable in authoritative results (disposition + effect set
+ evidence).

---

## 8. State transition model

```text
StateBefore
    ↓
evaluated Action
    ↓
declared effects
    ↓
StateAfter
```

Requirements:

* `StateBefore` is the EconomicState input (or an explicit snapshot thereof).
* Declared effects enumerate all authoritative economic mutations.
* `StateAfter` is either produced explicitly or uniquely determined by applying
  declared effects to `StateBefore` under World rules.
* Every economically material difference between `StateBefore` and `StateAfter`
  must be explainable by the evaluated Transaction’s declared effects.

```text
StateAfter changed with no declared economic effect explaining the change
        =
prohibited
```

This is a **semantic** requirement. It does **not** require the kernel to
implement a generic structural diff algorithm.

Not chosen here: struct layout, DB schema, serialization, persistence.

---

## 9. Economic consistency

The kernel preserves Domain Contract rules for balances, assets, money, prices,
fees, settlement, liabilities, positions, and authorization-related economic
constraints **as declared by the World**.

```text
No invariant category is universally imposed on every World.
```

Conservation and similar categories are World/Scenario concerns (often M02),
except foundational consistency required by §25 for safe kernel evaluation.

---

## 10. Numeric authority (resolves OD-02 for Gate 1)

### 10.1 Decision (EK-NUM-01)

**Authoritative monetary amounts are signed integers in Asset-declared minor
units.**

```text
Money amount  = integer minor-unit quantity
Asset         = identity + declared scale / unit semantics
```

Binary floating-point (`f32`/`f64` and equivalents) is **never** authoritative.

This does **NOT** require every Asset to be a conventional decimal currency.
Any Asset whose unit/scale semantics are **explicitly defined** by the
World/Asset contract is permitted (including non-decimal instruments), provided
amounts remain exact integer quantities in that declared unit system.

### 10.2 Scale / precision

* Each Asset **must** declare unit semantics: either a non-negative integer
  `scale` meaning amounts are in units of `10^(-scale)` of a declared major
  unit, **or** an equivalent explicitly declared minor-unit / indivisible-unit
  definition recorded on the Asset.
* Amounts for different Assets are **not** directly interchangeable.
* Mixing scales/units without an explicit World conversion/valuation rule is
  **invalid**.

### 10.3 Arithmetic

```text
same Asset → arithmetic permitted
different Assets → arithmetic forbidden without explicit conversion
```

* Addition/subtraction of amounts is defined only for the **same Asset**.
* Multiplication/division by dimensionless integer factors is allowed where
  World rules require (e.g. quantity × unit price components) only under
  explicitly defined intermediate exactness rules in the World/Action type.
* Gate 1 default for fee/price computations that yield a quantity not
  representable in the target Asset’s minor unit: Worlds **must** declare
  rounding mode for that calculation context.
* If a World Action type can produce a non-representable minor-unit result
  without a declared rounding mode → **Configuration Error** (or Invalid
  World), not a silent round.

### 10.4 Rounding (Gate 1 baseline)

Rounding occurs **only** where a World/Action calculation produces a quantity
that cannot be represented directly in the target Asset’s minor unit.

No implementation may silently select a rounding mode.

When a World declares rounding, it must choose an explicit mode from:

```text
towards_zero
away_from_zero
floor
ceil
half_away_from_zero
```

Reproducibility requires that:

```text
rounding policy
+
target Asset (unit/scale)
+
calculation context
```

are declared inputs sufficient to reproduce the result.

No IEEE float rounding is authoritative. No Rust rounding API is defined here.

### 10.5 Overflow / underflow

* Authoritative arithmetic uses **checked** integer operations within the
  representation range chosen by implementation of this integer model.
* Overflow or underflow of authoritative arithmetic → **Engine Error**
  (evaluation cannot complete correctly), never a wrapped economic PASS.
* Underflow of a balance facet below World-permitted bounds during effect
  application is an **economic** failure/rejection under World rules when
  detected as rule violation; it is not silent wraparound.

### 10.6 Comparison / zero / negative

* Comparison is integer comparison within the same Asset.
* Zero is the integer `0` minor units.

**Signed representation ≠ economically valid negative balance:**

```text
representable negative integer
        ≠
economically valid negative balance
```

A signed integer representation is an **implementation representation**.

It does **NOT** mean every World permits negative balances.

A World may reject a negative balance even though the underlying representation
can represent it. Negative amounts are economically valid **only** when the
World explicitly defines liability/credit semantics for that use (Domain
Contract). Otherwise negative effects that would create prohibited negatives →
economic rejection/failure per World rules.

This specification does **not** switch to an unsigned representation.

### 10.7 Conversion

Cross-asset conversion requires explicit World rule + authoritative price
identity (§15). No silent FX.

### 10.8 Machine width / bigint

```text
integer width / bigint choice:
IMPLEMENTATION DECISION
NOT DOMAIN SEMANTICS
```

Implementations must preserve EK-NUM-01 exactness and map range exhaustion to
`EngineError`. This freeze does **not** select `i64`, `i128`, bigint, etc.

### 10.9 Still deferred

* Canonical serialization of amounts (OD-03/OD-06).
* Concrete Rust numeric type selection (implementation).
---

## 11. Balance model (Gate-1 portion of DC-01)

### 11.1 Decision (EK-BAL-01)

The kernel does **not** impose a universal facet algebra.

A World **must** declare:

1. Which balance facets it uses (may include `available`, `reserved`, `pending`,
   `settled`, and/or others).
2. Whether facets are **mutually exclusive buckets**, **orthogonal dimensions**,
   or **another explicitly specified model**.
3. Permitted facet transitions and their semantic causes.
4. Whether `settled` implies spendability (default: **no** — settled ≠
   available).
5. Relationships among pending/reserved/available (default: **not** identical).

### 11.2 Minimum consistency before accepting a balance mutation

Before applying a balance mutation, the kernel must verify:

* Target Account and Asset exist under World/State.
* Facet referenced by the effect exists in the World model.
* Mutation respects declared facet model (no implicit pending→available, etc.).
* Post-condition quantities obey World negativity/liability rules.
* For bucket models: no double-counting contradiction introduced by the effect
  set under the World’s declared total/component rules (if any).
* For orthogonal models: each dimension updated only as declared by effects.

Failure of these checks yields economic rejection/failure or configuration
error as appropriate (§14) — never silent repair.

---

## 12. Transaction dispositions (Gate-1 portion of DC-02)

### 12.1 Decision (EK-TX-01)

Canonical **economic disposition** categories for recorded evaluations:

| Disposition | Meaning |
| --- | --- |
| `AcceptedEffective` | Action succeeds under World rules; declared authoritative effects are accepted (0..n effects) |
| `Rejected` | Action cannot proceed under applicable eligibility / permission / economic rule |
| `FailedEconomic` | Action was evaluable/eligible, but economic conditions prevented successful execution |

Examples:

* unauthorized spend → `Rejected`
* insufficient available funds (eligible Action, failed condition) → `FailedEconomic`
* successful no-op World Action with no mutations → `AcceptedEffective` with 0 effects

Ordinary economic failure must **not** become `EngineError`.

### 12.2 Zero-effect `AcceptedEffective` (intentional)

```text
AcceptedEffective + 0 state effects
```

is **intentionally permitted** and semantically distinct from rejection/failure.

It means: a legitimate World-defined Action whose successful execution causes
**no** authoritative state mutation. It must **not** be reinterpreted as
`Rejected`, `FailedEconomic`, or `EngineError`.

### 12.3 Rejection-side effects

Unsuccessful attempts (`Rejected` / `FailedEconomic`), when recorded, have
**zero** economic state effects **unless** the World **explicitly** declares
rejection-side effects.

Any rejection-side effect must be:

```text
explicitly declared by the World
deterministic
observable in authoritative results / evidence
compatible with atomicity (all declared rejection-side effects or none)
```

There is **no** universal rejection-side-effect model.

### 12.4 Non-economic outcomes (not dispositions)

| Class | Meaning |
| --- | --- |
| `InvalidInput` | Malformed / incomplete Action or inputs relative to kernel/World contract |
| `ConfigurationError` | World/State configuration insufficient or inconsistent for evaluation |
| `EngineError` | Kernel cannot correctly complete evaluation (bug, overflow, internal violation) |

These are **not** economic dispositions and must not be labeled
`AcceptedEffective`.

### 12.5 Recording policy

Whether every attempt is recorded is Scenario/orchestration policy. When a
record is produced by the kernel evaluation, it uses the taxonomy above.

---

## 13. Error boundary

| Situation | Classification |
| --- | --- |
| Insufficient funds under World rules | `FailedEconomic` |
| Unauthorized spend | `Rejected` |
| Malformed Action parameters | `InvalidInput` |
| Checked integer overflow in authoritative math | `EngineError` |
| Internal kernel consistency violation | `EngineError` |

### Unknown Asset classification

Do **not** force every unknown-Asset condition into one class:

| Context | Classification |
| --- | --- |
| Action references an Asset identity that is simply invalid/malformed as an Action parameter relative to the kernel/World contract | `InvalidInput` |
| World/State is missing required Asset configuration that the Action type legitimately expects | `ConfigurationError` |

```text
ECONOMIC OUTCOME ≠ ENGINE ERROR
ENGINE ERROR ≠ economic PASS
economic rejection/failure ≠ configuration error ≠ engine error
```

---

## 14. Fees

Fees are explicit economic effects when present.

Minimum representable fields:

```text
fee payer
fee recipient
fee asset
fee amount   (integer minor units of fee asset)
fee timing   (as declared by World/Action type)
fee economic effect (facet/account mutations)
```

No universal fee schedule. World-specific fee rules remain possible. Missing fee
rule when an Action type requires fees → `ConfigurationError` or `Rejected`
per World declaration (must be explicit).

---

## 15. Price (Gate-1 portion of DC-08)

### 15.1 Decision (EK-PRICE-01)

Preserve Domain Contract distinctions:

```text
quoted price
execution price
reference price
observed external price
```

An authoritative price must identify at least:

```text
base Asset
quote Asset
price semantics / category
```

`BTC/USD` and `USD/BTC` are **not** interchangeable. The World/Action must
define which direction is authoritative for the evaluation.

If an Action requires conversion/valuation, the **World/Action type must declare
which price category and direction are authoritative** for that evaluation.

The kernel must not silently prefer “market” or “observed” prices.

Price category placement (config vs state vs external observation vs derived)
must be declared; derived/external prices are not independent truth without
declared capture in inputs (Domain Contract).

**Price numeric representation:** quantities appearing in prices use EK-NUM-01
integer minor units of the relevant Assets (or an explicitly declared rational
relation of integer quantities). Floating-point price representation is
prohibited. Further microstructure remains LATER-GATE / implementation detail
under these constraints.

Full market microstructure remains deferred (LATER-GATE portions of DC-08).
---

## 16. Settlement (Gate-1 portion of DC-09)

### 16.1 Decision (EK-SETTLE-01)

**Full settlement state machine is DEFERRED** beyond what Balance facets already
require.

Gate 1 requirements:

* If a World uses a `settled` (or analogous) facet, transitions into/out of it
  require explicit semantic causes (Domain Contract).
* The kernel does **not** invent payment-provider settlement statuses
  (`authorized`/`executed`/… full model) unless the World declares them.
* Worlds that need richer settlement must declare it; Gate 1 kernel applies
  declared effects only.

---

## 17. Idempotency (DC-07)

### 17.1 Decision (EK-IDEM-01)

**Full idempotency protocol is DEFERRED** for Gate 1.

Gate 1 baseline:

* Not every Action is idempotent.
* An Action **may** carry an optional identifier.
* The kernel does **not** assume `same identifier ⇒ skip re-execution` unless
  the World explicitly declares an idempotency rule and the ExecutionContext /
  State contains the declared idempotency store/inputs.

Distinctions remain conceptual:

```text
same Action
same Action identifier
same economic intent
repeated execution
```

---

## 18. Ordering and time (DC-04 / DC-05)

### 18.1 Decision (EK-ORDER-01)

* **Logical ordering** is authoritative when order affects economic truth.
* ExecutionContext must carry the declared logical order key for the evaluation
  when the World/Scenario requires ordered evaluation.
* Wall-clock time must not be read from the host. If time is needed, it is a
  **declared** ExecutionContext input and only used if the World requires it.
* Exact global vs partitioned logical-time lattice details beyond a declared
  totally ordered evaluation sequence for single-Action kernel calls remain
  refinable by M03; Gate 1 single evaluation uses the supplied context order
  metadata without inventing host time.

---

## 19. Randomness

### 19.1 Decision (EK-RAND-01)

**Gate 1 Economic Kernel evaluation is deterministic without random input.**

The kernel must not introduce RNG for extensibility.

If a future World requires randomness, it must supply seed/source/scope via
ExecutionContext under Domain Contract rules. That is out of Gate 1 default
scope; enabling it requires an explicit amendment.

---

## 20. Determinism contract (Gate-1 portion of DC-12)

```text
Same:
    EconomicWorld
    EconomicState
    Action
    ExecutionContext
    EngineVersion
    declared configuration
    declared seed where applicable (N/A for Gate 1 default)

⇒ semantically equivalent authoritative output
```

### Semantic equivalence includes

```text
disposition or non-economic error class
state effects (or explicit none)
StateBefore / StateAfter economically material fields
Transaction economically material fields
economically material Events declared by World
evidence facts required by §23
```

### Semantic equivalence does not require

```text
byte-for-byte equality
identical wall-clock annotations
identical non-material host metadata
```

Canonical serialization is not defined by TASK-03.

---

## 21. LLM boundary

The kernel never delegates authoritative economic decisions to an LLM.

LLM may propose Actions/scenarios/etc. externally. The kernel accepts only
deterministic, validated domain inputs.

---

## 22. Evidence (minimum)

Evidence must allow a developer to determine economically authoritative facts:

```text
what Action was evaluated
what StateBefore existed
what World rules / configuration applied
what ExecutionContext inputs applied
what economic effects were determined
what Transaction disposition occurred
what StateAfter resulted (or that no mutation occurred)
why accepted / rejected / failed / errored
```

Evidence focuses on economic authority, not complete host-level tracing.
Serialization format deferred (OD-06).

---

## 23. Pure core requirement

```text
Declared Inputs
        ↓
Deterministic Evaluation
        ↓
Authoritative Result
```

External I/O must not silently modify economic truth. Adapters are outside the
kernel.

---

## 24. External system boundary

```text
External System
      ↓
Adapter
      ↓
Declared Kernel Inputs
      ↓
M01 Economic Kernel
      ↓
Authoritative Result
```

External systems are not automatically authoritative inside the kernel.

M01 must **never** directly call HTTP, databases, blockchain RPC, payment
providers, LLMs, wallets, or external clocks as hidden economic dependencies.
Adapters translate observations into declared inputs only.

---

## 25. Kernel invariants (foundational only) — M01 ≠ M02

```text
M01 Economic Kernel
        ≠
M02 Economic Invariant Engine
```

Gate 1 kernel-enforced foundational checks are **only** those required to
safely produce a coherent EconomicState / evaluation result:

```text
state transition consistency (effects explain StateAfter)
balance consistency according to World-declared model
asset identity consistency (effects reference known Assets)
transaction / state-effect consistency (disposition vs effects cardinality)
deterministic evaluation consistency (no undeclared inputs)
numeric model compliance (EK-NUM-01)
```

The following belong to **M02** unless later frozen as explicit kernel safety
properties (they are **not** Gate 1 M01 scope):

```text
custom conservation rules
business-specific invariants
scenario-defined invariant policies
arbitrary user assertions
```

M01 must **not** become a hidden general-purpose invariant engine.
---

## 26. Output contract

Conceptual `KernelOutcome` categories:

### Economic evaluation result

```text
disposition ∈ {AcceptedEffective, Rejected, FailedEconomic}
StateBefore
state effects (possibly empty)
StateAfter (equals StateBefore if no effects)
Transaction semantics
economically material Events (if any)
evidence
deterministic execution metadata (engine version, logical order key, …)
```

### Non-economic failure result

```text
error class ∈ {InvalidInput, ConfigurationError, EngineError}
error information
evidence of inputs/context as available
deterministic execution metadata
```

No Rust structs, SDK, or wire format here.

### Relation to DC-13 / test expectation

The kernel outputs **economic truth + execution status**.

Scenario/Test expectation matching (whether an economic failure was “expected”)
is **outside** M01 authoritative evaluation — belonging to Scenario/test harness
or later result-composition specs. Gate 1 kernel must not collapse those layers.

---

## 27. Versioning

Semantic-breaking changes requiring specification amendment include changes to:

```text
numeric representation
balance semantics required by the kernel
transaction dispositions
state transition semantics
fee semantics
price authority rules
settlement facet rules used by the kernel
determinism semantics
error / disposition taxonomy
```

Implementation refactors that **preserve** these semantics do **not**
automatically require a domain-spec amendment.
---

## 28. Testability (acceptance properties for future implementation)

Future implementation must eventually demonstrate:

1. **Determinism** — same declared inputs → same authoritative result
2. **No hidden state** — no undeclared mutable dependencies
3. **Atomicity** — atomic World ops never partially apply
4. **Explicit failure** — economic rejection/failure ≠ engine error
5. **State explainability** — material StateAfter diffs attributable to effects
6. **Numeric correctness** — EK-NUM-01 arithmetic/rounding/overflow rules
7. **Reproducibility** — recorded execution reproducible from declared inputs

Tests are not implemented in TASK-03.

---

## 29. Decisions resolved by TASK-03 / TASK-03F

| ID | Resolution | Classification |
| --- | --- | --- |
| OD-02 | EK-NUM-01 integer minor units + Asset unit/scale; no float | RESOLVED |
| DC-01 (Gate-1) | EK-BAL-01 World-declared facet model | RESOLVED |
| DC-02 (Gate-1) | EK-TX-01 dispositions + zero-effect AcceptedEffective clarified | RESOLVED |
| DC-04 / DC-05 (Gate-1) | EK-ORDER-01 | RESOLVED |
| DC-08 (Gate-1) | EK-PRICE-01 base/quote/direction + category | RESOLVED |
| DC-09 (Gate-1) | EK-SETTLE-01 full SM deferred | RESOLVED |
| DC-07 (Gate-1) | EK-IDEM-01 deferred protocol | RESOLVED |
| DC-12 (Gate-1) | semantic equivalence | RESOLVED |
| DC-13 (Gate-1) | expectation matching outside M01 | RESOLVED |
| Randomness | EK-RAND-01 no RNG | RESOLVED |
| Integer width / bigint | Implementation decision, not domain semantics | IMPLEMENTATION-ONLY |

---

## 30. Decisions deferred

| ID | Classification | Blocks TASK-04? |
| --- | --- | --- |
| OD-01 type/schema finalization | IMPLEMENTATION-ONLY / LATER with TASK-04 API design | No (semantics sufficient; API chosen in TASK-04) |
| OD-03 / OD-06 serialization | LATER-GATE | No |
| OD-04 invariant language | LATER-GATE (M02) | No |
| OD-05 simulation scheduling | LATER-GATE (M03) | No |
| OD-07 / OD-08 / OD-09 surfaces | LATER-GATE | No |
| OD-10 license | LATER-GATE | No |
| DC-03 correlation storage details | LATER-GATE / IMPLEMENTATION-ONLY | No |
| DC-06 reversal/refund details | LATER-GATE | No |
| DC-08 microstructure | LATER-GATE | No |
| DC-10 caching | IMPLEMENTATION-ONLY | No |
| DC-11 external mapping | LATER-GATE | No |
| Integer width / bigint | IMPLEMENTATION-ONLY | No (must obey EK-NUM-01 + EngineError) |
| Public Rust API / crate layout | IMPLEMENTATION-ONLY (TASK-04) | No for semantic freeze; required to *start coding* but not a semantic blocker |

---

## 31. Implementation blockers (process)

Before TASK-04 may implement the kernel:

1. This specification is **FROZEN** (satisfied by TASK-03F).
2. Explicit **TASK-04** authorization must be issued.
3. TASK-04 chooses Rust API/crate layout and integer width/bigint strategy under
   EK-NUM-01 without redefining economics.

No remaining **semantic** blockers for Gate 1 freeze.

---

## 32. Contradictions with Domain Contract

```text
NONE
```

---

## 33. Next step

```text
TASK-03F PASS
        ↓
Gate 1 CLOSED
        ↓
TASK-04 — Economic Kernel Implementation
```

Do **not** begin TASK-04 automatically.

---

## Document control

| Item | Value |
| --- | --- |
| Created by | TASK-03 |
| Frozen by | TASK-03F |
| Status | **FROZEN** |
| Normative freeze | **FROZEN** |
| Gate | **GATE 1 — CLOSED** |
| Implementation | **BLOCKED** until TASK-04 |
| Module | M01 Economic Kernel |
