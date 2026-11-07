# AivoGuard Economic Regression Engine Specification

| Field | Value |
| --- | --- |
| Document | `docs/design/economic-regression-engine-specification.md` |
| Task | **TASK-16** … **TASK-17A**; freeze **TASK-19** |
| Module | **M06 — Economic Regression Engine** |
| Gate | **GATE 5 — CLOSED** (specification freeze; implementation **BLOCKED**) |
| **STATUS** | **FROZEN** |
| Normative freeze | **FROZEN** |
| Implementation | **BLOCKED** — freeze does **not** authorize implementation |
| Depends on | Product Scope (**FROZEN**); Domain Contract (**FROZEN**); Economic Kernel Specification (**FROZEN**, Gate 1 **CLOSED**); Economic Invariant Engine Specification (**FROZEN**, Gate 2 **CLOSED**); Deterministic Simulator Specification (**FROZEN**, Gate 3 **CLOSED**); Adversarial Scenario Engine Specification (**FROZEN**, Gate 4 **CLOSED**); M01–M04 implementations (**PASS** / **ACCEPTED**) |
| Freeze authority | **TASK-19** formal freeze (audit: **TASK-18** **READY_FOR_FREEZE**) |
| Related architecture | [`../architecture/architecture-baseline.md`](../architecture/architecture-baseline.md) (boundary: M06) |
| Process | [`SPECIFICATION-POLICY.md`](SPECIFICATION-POLICY.md) |

```text
STATUS: FROZEN
NORMATIVE FREEZE: FROZEN
IMPLEMENTATION: BLOCKED
GATE: GATE 5 — CLOSED
```

This document is the **frozen** normative specification for Gate-5 M06.
Material semantic changes require an explicit specification amendment. Freeze
does **not** authorize M06 implementation, API/CLI/DSL design, or crate
creation. A separate implementation-authorization task is required.

Open decisions **AD-03**, **AD-12**, **AD-14**, and **AD-15** remain **OPEN**
and are not resolved by this freeze. **DC-13** remains formally **OPEN** in the
Domain Contract; this freeze does **not** close or amend DC-13.

Related:

* [`product-scope.md`](product-scope.md) (**FROZEN**) — Gate 5 = Regression Engine
* [`domain-contract.md`](domain-contract.md) (**FROZEN**) — DC-R04; **DC-13 OPEN**
* [`economic-kernel-specification.md`](economic-kernel-specification.md) (**FROZEN**)
* [`economic-invariant-engine-specification.md`](economic-invariant-engine-specification.md) (**FROZEN**)
* [`deterministic-simulator-specification.md`](deterministic-simulator-specification.md) (**FROZEN**)
* [`adversarial-scenario-engine-specification.md`](adversarial-scenario-engine-specification.md) (**FROZEN**)
* ADR 0001 — `i128` minor-unit amounts (**Accepted**)

---

## 1. Status and Authority

### 1.1 Authority hierarchy

```text
Product Scope FROZEN
        ↓
Domain Contract FROZEN
        ↓
M01 / M02 / M03 / M04 FROZEN
        ↓
This M06 specification (**FROZEN**)
        ↓
Accepted ADRs
        ↓
Implementation (NOT AUTHORIZED by this freeze)
        ↓
Tests
```

If this document conflicts with a frozen predecessor, the predecessor wins until
an explicit amendment resolves the conflict.

### 1.2 Module thesis

> M06 deterministically evaluates whether authoritative observed execution
> results satisfy explicitly declared regression expectations.

```text
M01 = economic truth
M02 = invariant evaluation
M03 = simulation / termination
M04 = adversarial generation
M06 = regression expectation comparison
```

M06 is a **consumer/comparator**. It is **not** an economic engine, invariant
engine, simulator, or scenario generator.

### 1.3 Domain Contract alignment (DC-R04 / DC-13)

Domain Contract **DC-R04** requires:

```text
Economic Truth ≠ Invariant Evaluation ≠ Test Expectation
```

Domain Contract **DC-13** (result composition: economic × execution ×
Scenario/Test expectation) remains listed **OPEN** in `domain-contract.md`.

**Gate-5 rule (this document):**

```text
M06 defines the Gate-5 expectation-matching semantics required by the
regression engine.

This specification provides the M06 realization of regression expectations,
but DC-13 remains formally OPEN in the Domain Contract until an explicit
Domain Contract amendment is authorized and applied.

This document does not close or amend DC-13.
```

A separate Domain Contract amendment **SHOULD** cross-reference this document
as the Gate-5 M06 realization of regression expectation matching (see §30).
That amendment is **not** performed by this document.

M03 continues to expose layered `SimulationResult` fields without collapsing
them into a harness PASS/FAIL. M06 consumes those layers via declared
expectations.

---

## 2. Scope

### 2.1 In scope

* Regression case model
* Closed Gate-5 expectation taxonomy (M06 regression expectation semantics;
  DC-13 remains formally OPEN in the Domain Contract)
* Observation binding rules
* Comparison operators and equality
* Numeric comparison under M01/`i128` contracts
* Evaluation ordering and mismatch evidence
* Verdict taxonomy: `MATCH` | `MISMATCH` | `ERROR`
* Error classes distinct from mismatches
* Determinism, provenance, authority boundaries
* Acceptance-test requirements for a future implementation

### 2.2 Out of scope

* M01/M02/M03/M04 semantic changes
* M05 Chaos
* Scenario shrinking (**AD-12**)
* Serialization / wire formats (**AD-14**)
* Public Rust API / CLI / SDK / HTTP freeze (**AD-15**)
* Content-derived identity algorithms (**AD-03**)
* Payment / x402 / multi-agent
* LLM authority
* Floating-point economic comparison
* Fuzzy / heuristic / similarity matching
* Silent unit conversion or coercion

---

## 3. Core pipeline

```text
RegressionCase
      │
      ├── case identity (declared)
      ├── scenario binding (declarative reference)
      ├── expectation set (ordered)
      └── observation binding (authoritative observed result)
                │
                ▼
        M06 comparison engine
                │
                ▼
        RegressionResult
                │
        ┌───────┼────────┐
        ▼       ▼        ▼
      MATCH  MISMATCH   ERROR
```

Normative properties:

```text
MATCH     ⇒ every applicable expectation is satisfied
MISMATCH  ⇒ at least one applicable expectation is unsatisfied
ERROR     ⇒ comparison could not produce a valid MATCH/MISMATCH verdict
```

---

## 4. Normative identifiers

### 4.1 Declared tokens

Unless otherwise stated, identity strings in M06 (`case_id`, `expectation_id`,
account/asset/facet string ids when declared as tokens) use:

```text
non-empty UTF-8
exact equality
case-sensitive
no trim
no Unicode normalization beyond the stored code-unit sequence
```

**AD-03 remains OPEN.** M06 **MUST NOT** require a content-derived hash/identity
algorithm. Case identity is **declared**.

### 4.2 Provisional types

Concrete Rust layouts are **provisional** (**AD-15 OPEN**). Semantic fields in
this document are normative; field names in implementations may differ if a
bijection to these semantics is documented and tested.

---

## 5. Regression case model

### 5.1 RegressionCase

| Field | Required | Purpose | Equality / ordering |
| --- | --- | --- | --- |
| `case_id` | YES | Declared case identity | Exact token equality |
| `case_version` | YES | Declared case version label | Exact string equality |
| `scenario_binding` | YES | Declarative reference to the Scenario under test | See §5.2 |
| `expectations` | YES | Ordered expectation set (may be empty) | Declared order is evaluation order |
| `observation` | YES | Authoritative observed outputs | See §7 |
| `engine_pins` | NO | Optional declared engine-version pins | Exact string equality when present |
| `notes` | NO | Non-authoritative human annotation | Ignored by comparison |

Absence:

* Missing required field → `ERROR` / `InvalidRegressionCase`
* Empty `expectations` → valid; evaluates to **MATCH** (vacuous satisfaction)
* Absent optional `engine_pins` → no pin check
* Absent `notes` → ignored

Determinism: identical `RegressionCase` + identical `observation` ⇒ identical
`RegressionResult` (§16).

### 5.2 ScenarioBinding

Declarative only. M06 **MUST NOT** execute the Scenario.

| Field | Required | Meaning |
| --- | --- | --- |
| `scenario_id` | YES | Declared Scenario id |
| `scenario_version` | YES | Declared Scenario version |
| `configuration_id` | YES | Declared configuration id |

M06 **MUST** validate that `observation` scenario identity fields
(`scenario_id`, `scenario_version`, `configuration_id`) equal this binding by
exact string equality. Binding validation failure → `ERROR` /
`ObservationBindingMismatch`.

M06 still **MUST NOT** execute the Scenario; it only consumes an already-produced
authoritative observation.

### 5.3 Empty expectation set

```text
expectations = []  ⇒  MATCH
```

unless ERROR conditions trigger first (invalid case / observation).

---

## 6. Expectation model (M06 regression expectations)

### 6.1 Closed Gate-5 expectation classes

Gate-5 M06 supports **only** the following expectation classes. Unsupported
classes → `ERROR` / `UnsupportedExpectation`.

| Class | Authoritative observation source | Why included |
| --- | --- | --- |
| `FinalBalanceExact` | M03 `SimulationResult.final_state` (M01 state) | Economic state regression |
| `FinalBalanceAbsoluteTolerance` | same | Explicit bounded numeric slack |
| `ExecutionStatusExact` | M03 `execution_status` | Termination / completion regression |
| `ExecutedActionCountExact` | M03 `executed_action_count` | Sequencing extent |
| `StepDispositionExact` | M03 step records → M01 disposition | Economic disposition at a step |
| `InvariantKindExact` | M03 invariant evaluation records (M02 kinds) | Invariant outcome regression |
| `CompletionInvariantKindExact` | M03 completion-phase M02 records | Completion-phase invariant regression |

**Not** Gate-5 expectation classes (explicitly deferred / out of scope):

* Arbitrary structural Scenario field equality (beyond binding checks)
* M04 GenerationStatus / provenance as regression truth
* Chaos outcomes (M05)
* Serialized artifact digests (AD-14)
* Shrinking properties (AD-12)

### 6.2 Common expectation envelope

Every expectation **MUST** include:

| Field | Required | Meaning |
| --- | --- | --- |
| `expectation_id` | YES | Declared unique id within the case |
| `class` | YES | One closed class from §6.1 |
| `applicable` | YES | Boolean; if `false`, expectation is skipped (not a mismatch) |

Duplicate `expectation_id` within a case → `ERROR` / `InvalidExpectation`.

Multiple expectations **MAY** target the same observation cell/record. Each is
evaluated independently in declaration order under `ALL_MISMATCHES`.
Contradictory applicable expectations (e.g. two `FinalBalanceExact` on the same
cell with unequal amounts) yield **MISMATCH** for each unsatisfied expectation
and overall verdict **MISMATCH** when evaluation completes without ERROR — not
a separate contradiction ERROR class.

### 6.3 Class payloads

#### FinalBalanceExact

```text
account_id : token
asset_id   : token
facet_id   : token
amount     : i128   // minor units; M01 ADR 0001
```

Operator: **exact equality** of the observed balance cell amount.

**Partial-state rule (normative):**

```text
A FinalBalance* expectation constrains only the explicitly declared
(account, asset, facet) balance cell.

It does not assert equality of the complete EconomicState.

Unspecified balance cells are unconstrained.

Unspecified cells are not implicitly required to be zero.

Full-state equality is outside the Gate-5 M06 expectation taxonomy.
```

**Presence-distinguishing lookup (normative; M06 observation-consumption only):**

```text
M06 uses a presence-distinguishing authoritative observation lookup on
SimulationResult.final_state.

If the declared balance cell is present:
    compare its authoritative value.

If the declared balance cell is absent:
    return ERROR / MissingObservationField.

M06 MUST NOT silently interpret absence as zero.
M06 MUST NOT invent an economic value from absence.
```

This rule does **not** redefine M01 `EconomicState::get_balance` (which may treat
absence as zero for kernel reads). M06 **MUST NOT** use silent-zero kernel-read
semantics when evaluating FinalBalance* expectations.

**Different Asset ids are never silently converted.**

#### FinalBalanceAbsoluteTolerance

Same targeting fields and partial-state / presence-distinguishing lookup rules
as `FinalBalanceExact`, plus:

```text
amount     : i128   // center / expected nominal
tolerance  : u128   // absolute max deviation in minor units; non-negative
```

Satisfied iff the normative absolute-tolerance procedure in §11.3 holds.
Any checked arithmetic / conversion failure → `ERROR` / `NumericComparisonError`.

**No floating-point tolerance. No percentage tolerance in Gate 5.**

#### ExecutionStatusExact

```text
expected : NormalCompletion
         | EarlyTermination { reason_exact: string }
         | FatalTermination { cause_class: Simulation | M01 }
```

* `NormalCompletion` — exact match of status variant.
* `EarlyTermination` — variant match **and** exact `reason` string equality.
* `FatalTermination` — variant match **and** cause class equality
  (`Simulation` vs `M01`). Detailed inner error payloads are **not** compared
  unless a future amendment adds an exact-error expectation class.

#### ExecutedActionCountExact

```text
count : u64
```

Exact equality with `SimulationResult.executed_action_count`.

#### StepDispositionExact

```text
step_index  : u64   // 0-based positional index into SimulationResult.step_records
disposition : Accepted | Rejected | FailedEconomic
              // mirrors M01 EconomicDisposition names as exposed by observation
```

**Lookup cardinality (positional):**

```text
step_index out of range for step_records.length  → 0 matches → ERROR / MissingObservationField
step_index in range                               → exactly 1 positional record → evaluate it
```

Positional indexing never selects “first/last among duplicates”; the index
identifies exactly one vector slot when in range.

Requires the selected step record to expose an M01 economic disposition.
Step present without economic disposition (e.g. only M01 Error path) when this
expectation applies → **MISMATCH** if disposition cannot equal expected;
do **not** invent a disposition. If the step’s M01 outcome is `Error` and the
expectation required an economic disposition → **MISMATCH**.

#### InvariantKindExact

```text
invariant_id : token
phase        : BeforeAction | AfterAction
step_index   : u64
expected_kind: Pass | Fail | Error
```

**Matching key:** records in `invariant_evaluation_records` whose
`(step_index, phase, invariant_id)` equal the expectation target
(see §7.5 for cardinality and Executed / NotExecuted rules).

**Important:** M02 `FAIL` does **not** by itself create M06 `MISMATCH` unless
an expectation of this (or completion) class declares the relationship.

#### CompletionInvariantKindExact

```text
invariant_id : token
expected_kind: Pass | Fail | Error
```

**Matching key:** records in `completion_evaluation_records` whose
`invariant_id` equals the expectation target (and whose evaluation point is
completion), subject to §7.5 cardinality and Executed / NotExecuted rules.
---

## 7. Observation model

### 7.1 Authoritative observation package

Gate-5 M06 observations are bound as:

```text
RegressionObservation
├── simulation_result : SimulationResult   // REQUIRED
└── (no fabricated fields)
```

`SimulationResult` is the M03 authoritative outcome summary (layered). M06
reads only fields defined by frozen M03 / carried M01/M02 results inside it.

### 7.2 Observation field catalog

| Field path | Source | Type (semantic) | Authority | Comparable by |
| --- | --- | --- | --- | --- |
| `execution_status` | M03 | status enum + payloads | M03 | `ExecutionStatusExact` |
| `final_state` balances | M01 via M03 | `(account,asset,facet)→i128` | M01 | balance expectations |
| `executed_action_count` | M03 | `u64` | M03 | `ExecutedActionCountExact` |
| `step_records[*].m01_disposition` | M01 via M03 | disposition enum | M01 | `StepDispositionExact` |
| `invariant_evaluation_records[*]` | M02 via M03 | kind + id + phase + step | M02 | `InvariantKindExact` |
| `completion_evaluation_records[*]` | M02 via M03 | kind + id | M02 | `CompletionInvariantKindExact` |
| `scenario_id` / `version` / `configuration_id` | M03 | strings | M03 | binding check |
| `simulation_errors` | M03 | error list | M03 | not Gate-5 expectation target (reserved) |
| `m01/m02/m03_engine_version` | M03 pins | strings | M03 | optional `engine_pins` |

### 7.3 Forbidden observation behavior

M06 **MUST NOT**:

* invent balances, dispositions, invariant kinds, or execution statuses
* re-run M01 `evaluate`
* re-run M02 evaluation
* re-run M03 simulation
* call M04 generation as part of comparison
* fill missing observation fields with defaults (including silent zero) unless
  the authoritative observation already exposes that value

Missing required observation content for an applicable expectation → `ERROR`
(not MISMATCH), class `MissingObservationField`.

### 7.4 Observation lookup cardinality (normative)

For expectation classes that select observation records by logical key
(`InvariantKindExact`, `CompletionInvariantKindExact`), M06 **MUST** collect
**all** records matching the declared key, then apply:

```text
0 matching records:
    ERROR / MissingObservationField

1 matching record:
    evaluate that record under §7.5

>1 matching records:
    ERROR / AmbiguousObservation
```

M06 **MUST NOT** silently select first, last, lowest index, or highest index
among duplicates. Collection iteration order **MUST NOT** affect which record
is chosen when cardinality ≠ 1 (the only outcomes are ERROR or evaluate-the-single).

`StepDispositionExact` uses positional indexing into `step_records` (§6.3):
in-range ⇒ exactly one slot; out-of-range ⇒ zero matches /
`MissingObservationField`. Positional indexing does not perform multi-match
selection.

### 7.5 Executed vs NotExecuted invariant records

When cardinality = 1 for `InvariantKindExact` or `CompletionInvariantKindExact`:

| Record form | Normative result |
| --- | --- |
| `Executed` with an M02 kind | Compare `expected_kind` by exact enum equality |
| `NotExecuted` | `ERROR` / `NotExecutedObservation` |

Rationale: a `NotExecuted` record does not provide an authoritative M02 kind
for comparison; the expectation cannot be evaluated as MATCH/MISMATCH.

M06 **MUST NOT** treat `NotExecuted` as an implicit Pass/Fail/Error kind.

### 7.6 M04 interaction

```text
M04 generates Scenario
        ↓
(external orchestration)
        ↓
M03 executes Scenario
        ↓
M01/M02 produce results inside SimulationResult
        ↓
M06 compares expectations against SimulationResult
```

M06 **MUST NOT** invoke M04 during comparison.

---

## 8. Comparison semantics

### 8.1 General rules

* Comparisons are **explicit** and **typed**.
* No implicit coercion between incompatible types/units/assets.
* No fuzzy matching.
* No LLM judgment.
* No heuristic similarity.

### 8.2 Exact equality

Used for tokens, strings, enums, `u64` counts, and `i128` amounts under
`FinalBalanceExact`.

```text
a == b  (domain equality of the semantic type)
```

### 8.3 Optional / presence

Gate-5 expectations do not use nullable amount fields. Applicability uses the
boolean `applicable` flag:

| `applicable` | Behavior |
| --- | --- |
| `true` | Expectation is evaluated |
| `false` | Skipped; neither MATCH contribution nor MISMATCH |

Skipped expectations produce no mismatch evidence.

### 8.4 Collections

* Expectation set: **ordered** by declaration; duplicates of `expectation_id`
  forbidden.
* Balance map in `final_state`: lookup by exact `(account, asset, facet)` key;
  iteration order of the map **MUST NOT** affect verdict (key lookup only).
* Invariant / step records: indexed by declared keys (`step_index`, phase, id);
  scan order for lookup is deterministic but verdict depends only on the matched
  record’s fields.

### 8.5 Versions

* `case_version` and Scenario binding versions: exact string equality when
  checked.
* Expectation entries do **not** carry independent version fields in Gate 5.
* M04 version-wildcard semantics are **not** reused.

---

## 9. Evaluation ordering

### 9.1 Policy

```text
Evidence policy: ALL_MISMATCHES
Verdict rule:    MISMATCH if any applicable expectation fails; else MATCH
                 (unless ERROR occurs)
```

### 9.2 Algorithm (normative)

1. Validate `RegressionCase` structure → on failure: `ERROR`.
2. **MUST** validate observation binding vs ScenarioBinding → on failure:
   `ERROR` / `ObservationBindingMismatch`.
3. Validate expectation set (ids unique; classes supported; payloads well-typed)
   → on failure: `ERROR`.
4. Optional `engine_pins`: if present, exact-compare to observation pins → on
   failure: `ERROR` / `EnginePinMismatch`.
5. Initialize empty mismatch list.
6. For each expectation in **declared order**:
   * if `applicable == false`: continue
   * evaluate comparison
   * if evaluation cannot proceed: `ERROR` (abort; no MATCH/MISMATCH)
   * if unsatisfied: append `MismatchEvidence` (declared order)
7. If any mismatch recorded → `MISMATCH` with full ordered evidence list.
8. Else → `MATCH` with empty mismatch list.

**First mismatch does not stop further evaluation** under Gate 5
(`ALL_MISMATCHES`), unless an `ERROR` aborts the run.

---

## 10. Verdict model

```text
RegressionVerdict =
    MATCH
  | MISMATCH
  | ERROR
```

| Verdict | Meaning |
| --- | --- |
| `MATCH` | All applicable expectations satisfied; no comparison ERROR |
| `MISMATCH` | ≥1 applicable expectation unsatisfied; comparison completed |
| `ERROR` | Case/observation/expectation/comparison could not be evaluated |

### 10.1 Non-collapse rules

```text
M02 invariant FAIL          ≠  M06 MISMATCH
M01 Rejected/FailedEconomic ≠  M06 MISMATCH
M03 Early/FatalTermination  ≠  M06 MISMATCH
M03 Fatal / SimulationError ≠  M06 ERROR
```

unless an explicit expectation compares against that observation.

An M03 fatal run may still `MATCH` if expectations only constrain fields that
are present and satisfied (e.g. `ExecutionStatusExact = FatalTermination`).

---

## 11. Numeric semantics (mandatory)

### 11.1 Representation

Authoritative economic amounts compared by M06 **MUST** be **signed integer
minor units** consistent with M01 / ADR 0001:

```text
i128 minor units
checked arithmetic only
no f32/f64 authority
no rust_decimal requirement
```

M06 does **not** redefine Asset scale. Scale/unit meaning remains M01/World.

### 11.2 Exact amount comparison

`FinalBalanceExact`: `observed_i128 == expected_i128`.

### 11.3 Absolute tolerance

`FinalBalanceAbsoluteTolerance` uses this **normative** procedure only
(unchecked `observed - expected` and informal `abs(...)` are **forbidden** as
authoritative semantics):

```text
difference = checked_sub(observed, expected)
if difference is undefined (overflow):
    ERROR / NumericComparisonError

absolute_difference = checked_abs(difference)
if absolute_difference is undefined (overflow; includes difference = i128::MIN):
    ERROR / NumericComparisonError

absolute_difference_u128 = checked conversion of absolute_difference to u128
if conversion fails:
    ERROR / NumericComparisonError

if absolute_difference_u128 <= tolerance:   // tolerance is non-negative u128
    expectation satisfied
else:
    expectation mismatch
```

Normative requirements:

* conversion direction: `i128` absolute difference → `u128`
* conversion is **checked** and **lossless** (no float; no truncating cast)
* conversion failure → `NumericComparisonError`
* `tolerance` remains non-negative `u128`

Extreme combinations that **MUST** produce `NumericComparisonError` when
checked arithmetic cannot represent the difference or absolute value include
(non-exhaustive but required acceptance coverage):

```text
observed = i128::MAX , expected = i128::MIN
observed = i128::MIN , expected = i128::MAX
difference = i128::MIN (checked_abs fails)
```

### 11.4 Cross-asset / unit

* Expectations target an explicit `(account, asset, facet)`.
* M06 **MUST NOT** convert between assets or facets.
* Comparing amounts across different asset identities is not expressible as a
  single Gate-5 balance expectation.

### 11.5 Arithmetic authority

M06 may perform **only** comparison arithmetic required by declared operators
(equality; absolute difference for tolerance). M06 **MUST NOT** compute new
economic fees, prices, settlements, or balances as authoritative truth.

---

## 12. Error model

Errors produce verdict `ERROR` and **MUST NOT** be reported as `MISMATCH`.

| error_id | Meaning | Trigger | Evidence |
| --- | --- | --- | --- |
| `InvalidRegressionCase` | Case malformed | missing required fields; empty ids | field path |
| `InvalidExpectation` | Expectation malformed | duplicate ids; bad payload | expectation_id |
| `UnsupportedExpectation` | Class not in §6.1 | unknown class | expectation_id / class |
| `ObservationBindingMismatch` | Binding ≠ observation identity | id/version/config mismatch | expected vs observed ids |
| `MissingObservationField` | Required observation absent | lookup miss / out of range / 0 matches | target descriptor |
| `AmbiguousObservation` | Multiple records match one key | >1 matching observation records | target descriptor + match count |
| `NotExecutedObservation` | Sole match is NotExecuted | single NotExecuted invariant record | expectation_id / target |
| `IncompatibleObservationType` | Observation shape unusable | missing SimulationResult | reason |
| `EnginePinMismatch` | Optional pins fail | version inequality | pin name |
| `NumericComparisonError` | Checked numeric ops fail | checked_sub / checked_abs / i128→u128 failure | operands |
| `EngineError` | Internal M06 fault | invariant of M06 broken | detail |

Each error **MUST** be machine-readable (`error_id` + structured fields). Human
`reason` strings are diagnostic only.

Determinism: same invalid inputs → same `error_id` and structured fields.

---

## 13. Mismatch evidence

```text
MismatchEvidence
├── expectation_id
├── class
├── target           // structured target descriptor
├── expected         // structured expected value
├── observed         // structured observed value
├── operator         // Exact | AbsoluteTolerance
├── mismatch_class   // ValueInequality | KindInequality | StatusInequality | …
└── ordinal          // 0-based index in declared evaluation order among mismatches
```

Machine authority = structured fields. Human messages optional and non-authoritative.

Mismatch list ordering = increasing `ordinal` = declared expectation order among
failing applicable expectations.

---

## 14. Result and provenance

### 14.1 RegressionResult

```text
RegressionResult
├── verdict                 // MATCH | MISMATCH | ERROR
├── case_id
├── case_version
├── mismatches[]            // empty unless MISMATCH
├── error                   // present iff ERROR
├── expectations_evaluated  // count of applicable expectations evaluated
├── expectations_skipped    // count with applicable=false
└── provenance
```

### 14.2 Provenance (semantic; AD-14 deferred)

```text
RegressionProvenance
├── scenario_id / scenario_version / configuration_id
├── observation_engine_pins (m01/m02/m03 versions as present)
├── m06_spec_version_label  // declared document/engine label when implemented
└── evaluation_policy       // ALL_MISMATCHES
```

Provenance **MUST** allow a consumer to answer:

1. Which case was evaluated?
2. Which expectations applied?
3. Which authoritative observation was used?
4. What matched / mismatched / errored and why?

Wire serialization of provenance is **OUT OF SCOPE** (**AD-14 OPEN**).

---

## 15. Authority boundaries

### 15.1 M06 MAY

* bind regression cases
* read authoritative `SimulationResult` observations
* evaluate declared expectations
* compare expected vs observed under this specification
* emit `MATCH` / `MISMATCH` / `ERROR`
* emit structured mismatch and error evidence

### 15.2 M06 MUST NOT

* recalculate M01 economic truth
* execute Actions as an economic engine
* mutate `EconomicState`
* replace or re-run M02 invariant evaluation
* invent invariant results
* replace M03 simulation or alter termination semantics
* advance simulation steps
* generate M04 adversarial scenarios during comparison
* perform M05 chaos behavior
* treat LLM output as authoritative
* silently modify frozen M01–M04 contracts
* introduce RNG, wall-clock, or unordered authoritative iteration

### 15.3 M06 is not a simulator

If execution is required, it occurs in M03 **before** M06. M06 consumes the
resulting observation only.

---

## 16. Determinism

```text
same RegressionCase
+ same RegressionObservation
= same RegressionResult
```

(semantic equality of verdict, ordered mismatches, and structured errors)

Forbidden as authority for comparison:

```text
randomness
wall clock
filesystem ordering
unordered map/set iteration affecting verdict
thread scheduling
environment variables
network state
LLM output
hidden mutable global state
```

Balance lookups are by key; map iteration must not affect outcomes.

---

## 17. Failure classification (non-collapse)

| Class | Owner | Meaning |
| --- | --- | --- |
| Economic disposition / effects | M01 | Economic truth |
| Invariant PASS/FAIL/ERROR | M02 | Invariant evaluation |
| Execution status / sim errors | M03 | Simulation / termination |
| Adversarial generation status | M04 | Generation (not consumed as Gate-5 expectation) |
| `REGRESSION_MATCH` | M06 | All applicable expectations satisfied |
| `REGRESSION_MISMATCH` | M06 | Expectation unsatisfied |
| `REGRESSION_ERROR` | M06 | Comparison could not complete |

These classes **MUST** remain distinguishable in harnesses that compose them.

---

## 18. Serialization

**AD-14 OPEN.**

```text
semantic data model = normative (this document)
wire format = DEFERRED
canonical bytes = DEFERRED
```

No JSON/YAML/TOML/CBOR/bincode requirement.

---

## 19. Public API

**AD-15 OPEN.**

This document defines semantic contracts only. It does **not** freeze public
Rust API, CLI, SDK, or HTTP surfaces.

---

## 20. Shrinking

**AD-12 OPEN / DEFERRED.**

M06 **MUST NOT** define scenario shrinking. Future shrinkers may consume M06
mismatch evidence externally.

---

## 21. Case identity

**AD-03 OPEN.**

Use declared `case_id` + `case_version`. Stability requirement: within one
engine configuration, identical declared ids denote the same case identity.
No hash algorithm is mandated.

---

## 22. Acceptance tests (normative for future implementation)

A conforming implementation **MUST** provide deterministic tests covering:

### 22.1 Basic

* satisfied expectations → `MATCH`
* violated expectation → `MISMATCH` with structured evidence
* invalid case → `ERROR` (`InvalidRegressionCase`)

### 22.2 Multiple expectations

* two failures → `MISMATCH` with evidence ordered by declaration
* `applicable=false` skipped; does not create mismatch
* contradictory applicable expectations on the same balance cell with unequal
  amounts → overall `MISMATCH` (each unsatisfied expectation contributes
  evidence); not a special contradiction ERROR

### 22.3 Observation authority and cardinality

* missing balance/step/invariant record needed by an applicable expectation →
  `ERROR` / `MissingObservationField`
* M06 never fabricates M01/M02/M03 values
* **0** matching invariant/completion records → `MissingObservationField`
* **1** matching `Executed` record → kind compared
* **>1** matching records → `ERROR` / `AmbiguousObservation`
* single matching `NotExecuted` record → `ERROR` / `NotExecutedObservation`
* absent declared balance cell → `MissingObservationField`
* absent balance cell **MUST NOT** become zero implicitly under M06 evaluation

### 22.4 Numeric

* `FinalBalanceExact` equality
* `FinalBalanceAbsoluteTolerance` boundary inclusive
  (`absolute_difference_u128 == tolerance` → satisfied)
* `checked_sub` overflow → `NumericComparisonError`
* `checked_abs` overflow (including `difference = i128::MIN`) →
  `NumericComparisonError`
* opposite-sign extremes `i128::MAX` vs `i128::MIN` (and inverse) under
  tolerance → `NumericComparisonError` when checked ops fail
* checked `i128` → `u128` conversion failure → `NumericComparisonError`
* no float comparison APIs in authoritative path

### 22.5 Partial balance

* declared cell matches **and** unrelated balance cells differ → `MATCH`
  (provided all applicable expectations are otherwise satisfied)

### 22.6 Binding / pins

* scenario binding validation is **mandatory**
* scenario binding mismatch → `ERROR` / `ObservationBindingMismatch`
* engine pin mismatch → `ERROR`

### 22.7 Error precedence

* expectation #1 unsatisfied (**would be** MISMATCH) **and** expectation #2
  produces evaluation ERROR (e.g. MissingObservationField) in declared order →
  final verdict **`ERROR`**; any mismatch evidence collected before the abort
  is **not** the authoritative final verdict (P3)

### 22.8 Non-collapse

* M02 FAIL without invariant expectation → may still `MATCH`
* M03 Fatal with matching `ExecutionStatusExact` → may `MATCH`

### 22.9 Determinism

* identical case+observation twice → identical `RegressionResult`

### 22.10 Authority boundaries

Tests / structural audit proving M06 comparison entrypoints do not:

* mutate `EconomicState`
* execute Actions
* invoke M03 run / M02 evaluate / M04 generate as part of compare

---

## 23. Semantic properties

```text
P1  MATCH ⇒ mismatches = [] ∧ error absent ∧ all applicable expectations satisfied
P2  MISMATCH ⇒ mismatches ≠ [] ∧ error absent ∧ every mismatches[i] corresponds
    to an unsatisfied applicable expectation in declaration order
P3  ERROR ⇒ verdict ERROR ∧ MATCH/MISMATCH evidence not authoritative
P4  applicable=false ⇒ never contributes a mismatch
P5  M06 does not change M01/M02/M03 authoritative outputs
```

---

## 24. No heuristics

Forbidden:

```text
fuzzy matching without declared AbsoluteTolerance
heuristic similarity
LLM judgment
semantic guessing
implicit coercion
silent unit/asset conversion
implicit reordering of expectations
```

---

## 25. Open decisions

### 25.1 Preserved open decisions

```text
AD-03 OPEN  — identity algorithm
AD-12 OPEN  — shrinking / minimization
AD-14 OPEN  — serialization format
AD-15 OPEN  — concrete Rust/API types
```

### 25.2 New M06-specific open decisions

| ID | Question | Why open | Blocks implementation? |
| --- | --- | --- | --- |
| M06-OD-01 | Exact Rust type layout / public function signatures for M06 | AD-15 | **NO** (provisional internal types allowed) |
| M06-OD-02 | Whether Gate-5 should later add exact M03 `simulation_errors` expectations | Minimal closed taxonomy chosen for Gate 5 | **NO** for Gate-5 closed set |
| M06-OD-03 | Whether FatalTermination expectations should deepen to exact M01/M03 error payloads | Deferred detail | **NO** (cause class sufficient for Gate 5) |

### 25.3 Domain Contract

| ID | Status | Note |
| --- | --- | --- |
| DC-13 | Still **OPEN** in `domain-contract.md` | M06 defines Gate-5 expectation-matching semantics; this document does **not** close DC-13 (§1.3 / §30) |

---

## 26. Self-audit checklist

| Area | Status in this draft |
| --- | --- |
| Authority | Explicit §1 / §15 |
| Determinism | Explicit §16 |
| Comparison | Explicit §8–§11 |
| Expectations | Closed taxonomy §6 |
| Numeric | `i128` exact + absolute tolerance §11 |
| Ordering | Declared order + ALL_MISMATCHES §9 |
| Errors vs mismatches | §10 / §12 |
| Evidence / provenance | §13 / §14 |
| M01–M04 boundaries | §7.6 / §15 / §17 |
| AD-03/12/14/15 | Preserved OPEN §25 |
| DC-13 | Formally OPEN in Domain Contract; M06 semantics defined here without closing DC-13 (§1.3 / §30) |
| Numeric ops | checked_sub → checked_abs → checked i128→u128 (§11.3) |
| Observation cardinality | 0 / 1 / >1 + NotExecuted (§7.4–§7.5) |
| Balance absence | presence-distinguishing; no silent zero (§6.3) |
| Binding | MUST validate (§5.2 / §9.2) |

---

## 27. Non-goals reminder

M06 does not become:

```text
economic kernel
invariant engine
simulator
adversarial generator
chaos engine
serializer
CLI/API product surface
shrinker
```

---

## 28. Document control

| Item | Value |
| --- | --- |
| Created by | TASK-16; remediated TASK-17A; freeze TASK-19 |
| Status | **FROZEN** |
| Freeze | **FROZEN** |
| Freeze authority | TASK-19 (audit TASK-18 READY_FOR_FREEZE) |
| Implementation | **BLOCKED** (not authorized by freeze) |
| Supersedes | None (first M06 specification) |

Lifecycle:

```text
DRAFT → READY_FOR_REVIEW → FROZEN → IMPLEMENTATION_AUTHORIZED → …
```

This document is **FROZEN** by TASK-19.

---

## 29. Gate state (as of TASK-19)

```text
GATE 4: CLOSED / FROZEN
GATE 5: CLOSED (specification FROZEN; implementation BLOCKED)
M06:    FROZEN
IMPLEMENTATION: NOT AUTHORIZED
```

---

## 30. DOMAIN CONTRACT AMENDMENT REQUIRED

**Yes — administrative/cross-reference amendment recommended; not performed here.**

### Why

`domain-contract.md` still lists **DC-13** as **OPEN**.

M06 defines the Gate-5 expectation-matching semantics required by the regression
engine. This specification provides the M06 realization of regression
expectations, but **DC-13 remains formally OPEN** in the Domain Contract until
an explicit Domain Contract amendment is authorized and applied.

**This document does not close or amend DC-13.**

### Required amendment content (future explicit task)

1. Record that Gate-5 M06 provides the regression expectation-matching semantics
   referenced by DC-13’s Scenario/Test expectation component.
2. Preserve DC-R04 non-collapse rules.
3. Do **not** redefine M01/M02/M03 authorities.
4. Leave AD-14/AD-15 unresolved.
5. Keep DC-13 status transitions explicit (OPEN → resolved only by Domain
   Contract amendment).

### Not done in TASK-16 / TASK-17A

No edit to `docs/design/domain-contract.md`.

---

## 31. Hard stop

This specification does **not** authorize:

* M06 implementation
* M06 tests in-tree as product code
* changes to frozen M01–M04 specifications
* resolution of AD-03 / AD-12 / AD-14 / AD-15
* serialization, CLI/API, or shrinking
