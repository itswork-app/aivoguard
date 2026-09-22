# AivoGuard Adversarial Scenario Engine Specification

| Field | Value |
| --- | --- |
| Document | `docs/design/adversarial-scenario-engine-specification.md` |
| Task | **TASK-10** / **TASK-10R** (remediation) |
| Module | **M04 — Adversarial Scenario Engine** |
| Gate | **GATE 4 — OPEN** |
| **STATUS** | **READY_FOR_REVIEW** |
| Normative freeze | **NOT FROZEN** |
| Implementation | **BLOCKED** — this document does **not** authorize implementation |
| Depends on | Product Scope (**FROZEN**); Domain Contract (**FROZEN**); Economic Kernel Specification (**FROZEN**, Gate 1 **CLOSED**); Economic Invariant Engine Specification (**FROZEN**, Gate 2 **CLOSED**); Deterministic Simulator Specification (**FROZEN**, Gate 3 **CLOSED**); M01/M02/M03 implementation (**PASS**) |

```text
STATUS: READY_FOR_REVIEW
NORMATIVE FREEZE: NOT FROZEN
IMPLEMENTATION: BLOCKED
GATE: GATE 4 — OPEN
```

This document is the **reviewable normative candidate** for Gate-4 M04 after
TASK-10R semantic remediation. It is **not frozen**. Material freeze requires
an explicit freeze audit task. Implementation requires a later explicit
authorization task after freeze.

Related:

* [`product-scope.md`](product-scope.md) (**FROZEN**)
* [`domain-contract.md`](domain-contract.md) (**FROZEN**)
* [`economic-kernel-specification.md`](economic-kernel-specification.md) (**FROZEN**)
* [`economic-invariant-engine-specification.md`](economic-invariant-engine-specification.md) (**FROZEN**)
* [`deterministic-simulator-specification.md`](deterministic-simulator-specification.md) (**FROZEN**)
* [`../architecture/architecture-baseline.md`](../architecture/architecture-baseline.md) (architecture only)
* [`SPECIFICATION-POLICY.md`](SPECIFICATION-POLICY.md)

---

## 1. Status and Authority

### 1.1 Authority hierarchy

```text
Product Scope FROZEN
        ↓
Domain Contract FROZEN
        ↓
Economic Kernel Specification FROZEN (M01)
        ↓
Economic Invariant Engine Specification FROZEN (M02)
        ↓
Deterministic Simulator Specification FROZEN (M03)
        ↓
M04 Adversarial Scenario Engine Specification (this document; READY_FOR_REVIEW)
        ↓
Accepted ADRs
        ↓
Implementation (blocked)
```

M04 **MUST NOT** silently modify any frozen specification above.

If a required M04 behavior contradicts a frozen specification:

```text
STOP → REPORT → do not patch around it in code or by silent amendment
```

### 1.2 Canonical authority assignment

```text
M01 Economic Kernel        = authoritative economic truth
M02 Invariant Engine       = authoritative invariant evaluation
M03 Deterministic Simulator = deterministic sequential orchestration
M04 Adversarial Engine     = adversarial scenario generation / transformation
```

### 1.3 Product role

> A deterministic adversarial scenario-generation and scenario-transformation
> layer that derives explicit, inspectable, reproducible economic test scenarios
> from declared base scenarios and feeds them to M03 for execution against the
> M01/M02 foundation.

M04 answers:

```text
What economically adversarial situations can be systematically tested?
```

M04 does **not** answer:

```text
What economically happened?                         → M01
Does an invariant hold?                             → M02
How is a scenario executed deterministically?       → M03
Does observed output match a regression expectation? → M06
```

---

## 2. Purpose

M04 defines how AivoGuard **generates, transforms, composes, validates, and
records provenance for** adversarial economic scenarios without becoming an
economic authority.

Core separation:

```text
ADVERSARIAL INPUT GENERATION   (M04)
        ≠
ECONOMIC EXECUTION             (M03)
        ≠
ECONOMIC TRUTH                 (M01)
        ≠
INVARIANT EVALUATION           (M02)
```

---

## 3. Scope

### 3.1 In scope (Gate 4 semantics)

* AdversarialScenario model
* Base scenario requirements
* AdversarialTransformation model
* Initial adversarial category taxonomy
* Applicability / validity / non-applicability
* Transformation composition (with explosion controls)
* Generation / mutation / composition (and deferred shrinking)
* Determinism, provenance, reproducibility
* Enumeration limits and deterministic ordering
* M04 error taxonomy and result layers
* LLM proposal boundary (non-authority)
* Open Decision register

### 3.2 Out of scope (explicit)

* M04 Rust modules / crate layout
* Public Rust API, CLI, HTTP, GraphQL, SDK, bindings
* Concrete serialization formats
* Persistence / database
* RNG library selection / seeded randomness implementation
* Shrinking/minimization algorithm freeze (deferred; §22)
* M05 Chaos Engine semantics (distinct module)
* M06 regression expectation language
* M07/M08 adapters, M09 multi-agent worlds
* Live network, wallets, blockchains, real payments
* Amending frozen M01/M02/M03 semantics

### 3.3 Normative reuse (do not redefine)

| Concept | Owner |
| --- | --- |
| `EconomicWorld`, `EconomicState`, `Action`, `Money`, dispositions | Domain / M01 |
| `KernelOutcome`, `Transaction`, `ExecutionContext` | M01 |
| `Invariant`, `PASS`/`FAIL`/`ERROR`, history targets | M02 |
| `Scenario`, `StopPolicy`, `InvariantEvaluationPlan`, execution algorithm | M03 |
| Numeric: no `f32`/`f64` for authoritative economic truth | Product / M01 |
| Deterministic collections | ADR 0002 |

M04 **consumes** M03 Scenario semantics. Derived adversarial artifacts that are
intended for execution **MUST** be representable as valid M03 Scenario inputs
(or an explicit M04→M03 projection defined without redefining M03).

---

## 4. Definitions

### Adversarial Scenario Engine (M04)

The Gate-4 module that produces adversarial scenario artifacts from declared
base scenarios and transformations.

### Base Scenario

An explicit M03-compatible Scenario (World, initial state, actions,
configuration, optional invariant plan, stop policy, deterministic metadata)
used as the source of an adversarial derivation.

### AdversarialScenario

A derived, explicit, inspectable, reproducible Scenario artifact plus M04
provenance and adversarial metadata (§6).

### AdversarialTransformation

A declared, inspectable mapping from a base (or intermediate) scenario to a
derived scenario under explicit parameters and preconditions (§8).

### Adversarial Intent / Category

The declared test intent taxonomy class of a transformation or scenario
(§9, §12). Intent is **not** economic truth.

### Generation

Creating one or more AdversarialScenario artifacts from a base scenario and a
generation plan.

### Mutation

Applying a single declared AdversarialTransformation to an existing scenario.

### Composition

Applying an ordered sequence of declared transformations (§10).

### Shrinking

Reducing an adversarial scenario while preserving a declared property.
**DEFERRED** for Gate-4 freeze unless remediated (§22).

### Applicable Transformation

A transformation whose declared preconditions hold for the current intermediate
scenario snapshot.

### Non-Applicable Transformation (`NON_APPLICABLE`)

A transformation that was considered and whose declared preconditions do not
hold. This is a **normal application outcome**, not an engine failure (§8.6).

### Required Applicability Failure

A generation plan with `ApplicabilityPolicy::REQUIRE_APPLICABLE` encounters a
transformation whose preconditions fail. This is an **M04 error**:
`NON_APPLICABLE_TRANSFORMATION` (§8.6, §15).

### Valid Adversarial Scenario

A DERIVED candidate accepted by Domain/M03 contracts and executable via M03.

### Invalid Adversarial Scenario

A DERIVED candidate that violates declared scenario/domain requirements; not
silently “fixed” into validity. Remains `DERIVED + INVALID` (§8.5).

### M04 Engine Failure

A failure of M04 itself (distinct from M01/M02/M03 errors).

### Provenance

The reconstructible derivation chain from base → transformations → derived
scenario (§20).

### Scenario Snapshot

The explicit intermediate Scenario artifact on which a transformation is
applied (the base, or the result of prior composition steps).

---

## 5. Architectural Role

```text
Base Scenario / Economic World
            |
            v
   M04 Adversarial Generator / Transformer
            |
            v
   Adversarial Scenario (+ provenance)
            |
            v
          M03 Simulator
            |
            v
          M01 Kernel
            |
            v
          M02 Invariant Engine (optional)
            |
            v
    Evidence / Result (layered)
```

### M04 may

* accept an explicit Base Scenario
* apply declared AdversarialTransformations
* compose transformations under explicit rules and limits
* validate derived candidates against Domain/M03 structural requirements
* produce provenance and M04 evidence
* declare which M03 invariant plan / stop policy a derived scenario carries
  (as Scenario fields — not by evaluating them)
* enumerate finite candidate sets under declared limits and deterministic order

### M04 MUST NOT

* directly mutate authoritative `EconomicState` during economic execution
* bypass M01 or M03
* calculate authoritative balances, fees, prices, conversions, or settlement
* determine transaction success/failure
* determine invariant `PASS`/`FAIL`/`ERROR`
* repair or normalize economic outcomes
* reinterpret M01 outcomes
* replace M03 orchestration / termination / execution-limit semantics
* use wall clock, host env, network, filesystem, DB, or unseeded RNG as
  authoritative inputs
* use an LLM as economic or invariant authority

---

## 6. AdversarialScenario Model

### 6.1 Conceptual structure

```text
AdversarialScenario
├── scenario identity / version
├── base scenario reference (identity + version)
├── adversarial category / intent
├── applied transformation(s) + parameters
├── deterministic generation configuration
├── derived M03-compatible Scenario payload
│     ├── EconomicWorld
│     ├── initial EconomicState
│     ├── ordered Actions
│     ├── execution configuration
│     ├── optional invariant evaluation plan
│     └── stop policy
├── provenance / derivation chain
└── reproducibility metadata (engine version, config id, optional seed field)
```

### 6.2 Required properties

Every generated adversarial scenario **MUST** be:

```text
explicit
inspectable
reproducible
deterministic
attributable to its source scenario
versionable
independently executable (when Valid)
```

No hidden mutation. No undeclared fields that affect economic semantics.

### 6.3 Identity (normative requirements; concrete algorithm OPEN)

Identity **MUST NOT** rely solely on memory address or ephemeral runtime IDs.

Gate-4 requires that identity be:

* stable under identical declared inputs
* sufficient for provenance linking
* independent of host process state

Whether identity is **declared**, **content-derived**, or hybrid is **AD-03**
(Open Decision). Cryptographic hash algorithms are **not** frozen here.

---

## 7. Base Scenario

M04 **MUST** operate on an explicit Base Scenario.

Minimum contents (aligned with M03):

```text
EconomicWorld
initial EconomicState
Action sequence
execution configuration (incl. maximum_action_steps)
optional InvariantEvaluationPlan
optional StopPolicy
deterministic metadata (identity/version/configuration_id)
```

### 7.1 Missing inputs

If required source information is absent:

```text
→ M04 error (INVALID_BASE_SCENARIO / INVALID_ADVERSARIAL_CONFIGURATION)
```

**MUST NOT** derive missing inputs from:

```text
host clock
environment
filesystem
network
process state
random global state
external service
LLM output
```

### 7.2 No inventing economic state

M04 **MUST NOT** invent balances, World rules, or Action parameters that are
not either:

* present in the Base Scenario, or
* explicitly produced by a declared transformation with inspectable parameters.

---

## 8. AdversarialTransformation

### 8.1 Transformation input / result contract

```text
TransformationInput
    =
    ScenarioSnapshot          (current intermediate Scenario)
    +
    TransformationDefinition
    +
    TransformationParameters
    +
    TransformationContext
```

```text
TransformationInput
    ->
TransformationApplicationResult
```

`TransformationContext` includes at least:

```text
composition position (0-based index in the declared plan)
applicability policy (§8.6)
generator configuration reference (limits, ordering)
M04 engine version
parent / source scenario identity + version
```

A transformation **operates on an explicit Scenario snapshot** and produces an
**explicit candidate Scenario artifact** (or NonApplicable / Error). No hidden
fields may change.

### 8.2 Required declaration fields

Each transformation **MUST** declare:

```text
transformation identity / version
target (§13)
parameters / ParameterDomain (§17A)
preconditions
transformation semantics
applicability rules
field projection rules (§8.4)
provenance contribution
```

### 8.3 Application outcomes (top level)

Applying a transformation yields exactly one of (§8.5):

| Outcome | Meaning |
| --- | --- |
| `DERIVED` | Preconditions hold; candidate produced; then VALID or INVALID |
| `NON_APPLICABLE` | Preconditions fail under `ALLOW_NON_APPLICABLE` |
| `ERROR` | Definition/configuration/parameter/composition/engine failure, or required-applicability failure |

Do **not** silently skip transformations. Every considered transformation
produces one explicit classification.

### 8.4 Transformation projection contract (R-01)

For every Scenario field, the transformation **MUST** declare exactly one
projection mode:

| Mode | Meaning |
| --- | --- |
| `INHERIT_UNCHANGED` | Field copied from the current ScenarioSnapshot |
| `REPLACE_EXPLICITLY` | Field replaced by an explicit parameter value |
| `DERIVE_EXPLICITLY` | Field computed by a declared, inspectable derivation rule over the snapshot + parameters (no M01 economic formulas) |
| `REMOVE_EXPLICITLY` | Field cleared / emptied by explicit declaration (only where the Domain/M03 contract permits empty) |
| `NOT_ALLOWED` | Transformation must not touch this field; attempting to declare otherwise → `INVALID_ADVERSARIAL_DEFINITION` |

#### 8.4.1 Normative default projection table

Unless a transformation definition **explicitly** declares otherwise for a
named field, Gate-4 defaults are:

| Scenario field | Default projection |
| --- | --- |
| `EconomicWorld` (`world`) | `INHERIT_UNCHANGED` |
| `initial EconomicState` (`initial_state`) | `INHERIT_UNCHANGED` |
| ordered `Actions` (`actions`) | `INHERIT_UNCHANGED` |
| execution configuration (`maximum_action_steps`, `configuration_id`, `declared_unix_secs`) | `INHERIT_UNCHANGED` |
| `invariant_plan` | `INHERIT_UNCHANGED` |
| `stop_policy` | `INHERIT_UNCHANGED` |
| scenario metadata (`id`, `version`) | `DERIVE_EXPLICITLY` (see below) |

**Metadata derivation (Gate-4):** when any other field is changed,
`id`/`version` **MUST** be updated by an explicit declared rule in the
transformation or generation plan (for example: append a derivation suffix /
increment a declared version label). Silent retention of the base `id`/`version`
while payload fields change is **forbidden**. Exact string algorithm remains
related to **AD-03**; the requirement that metadata change be explicit is
normative.

No unspecified field may silently change. A transformation that mutates a field
without a declared projection mode for that field is
`INVALID_ADVERSARIAL_DEFINITION`.

#### 8.4.2 Action transformation rules

Action transformations operate on the **current intermediate** `actions`
sequence (the ScenarioSnapshot), not on a stale copy of the original base
unless that snapshot is still identical.

| Concern | Normative rule |
| --- | --- |
| Insert / delete / replace / duplicate | Applied to the current intermediate sequence |
| Action reference | Prefer stable `ActionId` when the referenced Action declares one; positional index is permitted (§10.3) |
| Positional index | Refers to the **current** intermediate sequence after prior composition steps |
| Duplicate Action | Inserts a copy at a declared insertion index; the duplicate’s `action_id` **MUST** be set by an explicit rule: either `INHERIT` the source id (allowed only if World/idempotency semantics permit duplicates) or `REPLACE_EXPLICITLY` / `REMOVE_EXPLICITLY` (`None`) per transformation parameters. Silent implicit id invention from host state is forbidden |
| Execution ordering | Resulting sequence order is the declared post-transformation order; M03 executes that order |
| Unrelated Actions | Remain byte-for-byte / field-for-field unchanged |
| Replacement parameters | Represented as explicit TransformationParameters naming the Action field(s) replaced |
| Invalid resulting Action | If the transformation successfully emits a candidate whose Action violates Domain structural rules → `DERIVED + INVALID`. If parameters are outside the transformation’s declared ParameterDomain → `ERROR` / `INVALID_TRANSFORMATION_PARAMETER` |

A transformation **MUST NOT** silently alter unrelated Scenario fields
(including World, initial state, plans, stop policy, or execution config)
unless those fields have a declared non-`INHERIT_UNCHANGED` projection.

### 8.5 Transformation result lifecycle

```text
TransformationApplicationResult
    = DERIVED(candidate, ValidationClassification)
    | NON_APPLICABLE(reason)
    | ERROR(M04Error)
```

```text
ValidationClassification = VALID | INVALID
```

Rules:

* `DERIVED + INVALID` means the transformation **succeeded** at producing a
  candidate that fails Domain/M03 structural requirements.
* Do **not** automatically promote `DERIVED + INVALID` to a transformation
  `ERROR`.
* `generation success ≠ scenario validity ≠ economic failure`.

### 8.6 ApplicabilityPolicy (R-05; closes AD-16)

Every generation / composition plan **MUST** declare:

```text
ApplicabilityPolicy:
    ALLOW_NON_APPLICABLE
    REQUIRE_APPLICABLE
```

**Normative default:** `ALLOW_NON_APPLICABLE`.

| Policy | Preconditions fail | Result |
| --- | --- | --- |
| `ALLOW_NON_APPLICABLE` | Yes | `NON_APPLICABLE` (normal; not engine failure) |
| `REQUIRE_APPLICABLE` | Yes | `ERROR` with class `NON_APPLICABLE_TRANSFORMATION` |

A transformation **MUST NEVER** silently disappear from evidence. Every
considered transformation produces one of `DERIVED` / `NON_APPLICABLE` /
`ERROR`.

### 8.7 Initial EconomicState transformation contract (R-02)

Two distinct concepts:

```text
SCENARIO INITIAL-STATE DERIVATION
    = constructing a declared Scenario.initial_state candidate

RUNTIME ECONOMIC STATE MUTATION
    = mutating M01 authoritative state during/after evaluation
```

M04 **MAY** produce a different declared initial Scenario state.

M04 **MUST NEVER** mutate M01 runtime state.

#### Allowed declared initial-state fields

M04 may construct candidate `EconomicState` content consisting solely of
**explicit balance cells** representable in Domain/M01 state:

```text
(account, asset, facet) → i128 minor-unit amount
```

#### Forbidden

* Hidden/internal M01 engine fields
* Fabricating Transactions, Events, Effects, or dispositions as “state”
* Inferring balances via M01 fee/price/conversion/transfer formulas
* Host/environment-derived amounts
* Floating-point approximation

#### Membership / World / ownership

| Change | Allowed as declared Scenario input? | Notes |
| --- | --- | --- |
| Balance facet cell values | Yes | Explicit parameters only |
| Adding/removing balance cells | Yes | Explicit cell set/clear |
| Actor/account/asset **World membership** | Only via `REPLACE_EXPLICITLY` / `DERIVE_EXPLICITLY` on `world` | Separate from initial_state |
| Ownership relationships | Only via explicit World field projection | Not via inferred state |
| World change bundled into a “state” transform | No | Must declare World projection separately |

#### Validation ownership

| Check | Owner |
| --- | --- |
| Structural Domain/M03 Scenario acceptance | M04 candidate validation → then M03 |
| Authoritative economic consequences of Actions | M01 after M03 execution |
| Intentionally invalid candidate | `DERIVED + INVALID` with explicit reason |

Critical rule:

> M04 may construct a declared candidate initial state, but it does not
> establish economic truth about that state.

If a transformation requires an economic calculation not already represented
as explicit input (for example: “set balance to fee(amount)”), classify the
transformation as `INVALID_ADVERSARIAL_DEFINITION` /
`INVALID_TRANSFORMATION_PARAMETER` / unsupported — **do not** invent M01
semantics.

---

## 9. Adversarial Categories (initial taxonomy)

Categories are **test intents**. Applicability is always explicit. Not every
example is valid for every base.

### 9.1 Action sequence adversaries

Examples (applicability-gated):

* repeated action
* reordered action
* duplicate action
* omitted action
* unexpected / inserted action
* boundary action
* zero-value / max / min parameter action (where Action type permits)

### 9.2 Economic parameter adversaries

Examples:

* min / max / exact threshold amounts
* just-below / just-above thresholds
* fee / price / conversion / balance boundaries

M04 generates parameters. **M01** determines economic consequences.

### 9.3 Actor / account adversaries

Examples:

* different / unauthorized actor
* ownership boundary
* counterparties swapped
* conflicting actor/account relationships (where structurally representable)

M04 does **not** define authorization truth. **M01** remains authoritative.

### 9.4 State adversaries

Examples (intents only; realization governed by §8.7):

* low / zero / near-limit balances
* competing balances
* precondition boundary states

M04 may construct **declared Scenario initial states** only as Scenario
initial-state derivation (§8.7). It **MUST NOT** bypass M01 to mutate state
during execution.

### 9.5 Temporal / order adversaries

Within M03’s existing deterministic logical ordering:

* adjacent reordering
* permutations of action order
* execution-context boundary values (declared logical order / declared time)

M04 **MUST NOT** redefine M03 logical-order or ExecutionContext semantics.

### 9.6 History adversaries

Examples:

* repeated patterns
* long / sparse sequences
* history-window boundary lengths
* history-dependent precondition patterns

M04 respects M02 history semantics; it does not evaluate History invariants.

### 9.7 Failure-path adversaries

Targeted exercise of:

```text
Rejected
FailedEconomic
InvalidInput
ConfigurationError
```

M04 **MUST** distinguish economic outcomes from engine/configuration errors.
It **MUST NEVER** label an engine error as an adversarial economic success.

### 9.8 Taxonomy closure

The initial taxonomy above is **normative for Gate-4 review** but not claimed
exhaustive forever. Extension requires specification amendment (**AD-01**).

---

## 10. Transformation Composition

### 10.1 Composition supported

Yes. Composition is an ordered list:

```text
T_0 ∘ T_1 ∘ … ∘ T_(k-1)
```

**Gate-4 normative direction:** apply in declared vector order index
`0 .. k-1` (left-to-right). Associativity is **not** assumed.

### 10.2 Required composition rules

| Concern | Rule |
| --- | --- |
| Ordering | Declared plan vector order is authoritative |
| Associativity | **Not** assumed; only explicit ordered plans are normative |
| Duplicates | Explicit repeated entries in the plan are **allowed** and applied in order; silent auto-dedupe of plan entries is **forbidden** (**AD-05** closed for Gate-4) |
| Conflicts | Incompatible transformations → `INCOMPATIBLE_TRANSFORMATION` / `COMPOSITION_ERROR` |
| Depth | Bounded by `maximum_composition_depth` (§18) |
| Identity | Derived scenario identity/provenance includes full ordered chain |
| Explosion | Bounded by generation limits (§18) |

No implicit composition outside a declared plan.

### 10.3 Composition target identity (R-06)

Transformation references **MUST** resolve against the ScenarioSnapshot on
which the transformation is applied (the current intermediate scenario after
every previous transformation in the declared composition sequence).

#### Action addressing

| Method | Rule |
| --- | --- |
| Stable `ActionId` | Preferred when the target Action declares `Some(ActionId)`. Resolution searches the **current** intermediate sequence for matching `action_id` |
| Positional index | Permitted. Index `i` means `actions[i]` of the **current** intermediate scenario. Indices from the original base are invalid after earlier steps change length/order unless a stable `ActionId` is used |

No transformation may assume original-base indexes after an earlier
transformation changes the action sequence unless it explicitly uses stable
`ActionId`.

#### Missing target after prior steps

When a referenced Action (by id or index) does not exist in the current
snapshot:

| Case | Classification |
| --- | --- |
| Reference value outside the transformation’s declared ParameterDomain (illegal index type/range declaration, malformed id parameter) | `ERROR` / `INVALID_TRANSFORMATION_PARAMETER` |
| Reference is well-typed but the Action is absent (precondition of “target exists” fails) | `NON_APPLICABLE` under `ALLOW_NON_APPLICABLE`; `ERROR` / `NON_APPLICABLE_TRANSFORMATION` under `REQUIRE_APPLICABLE` |

**Gate-4 rule:** absence of a previously expected Action is treated as an
**applicability precondition failure** (`NON_APPLICABLE` / required error),
not as automatic `INVALID_TRANSFORMATION_PARAMETER`, when the parameter itself
is well-formed.

Duplicate / delete / replace that cannot locate their target follow the same
table.

---

## 11. Generation vs Mutation vs Composition vs Shrinking

| Mode | Meaning | Gate-4 |
| --- | --- | --- |
| Generation | Produce derived scenarios from base + plan | In scope |
| Mutation | Apply one declared transformation | In scope |
| Composition | Apply ordered transformations | In scope |
| Shrinking | Minimize while preserving a property | **DEFERRED** (§22) |

---

## 12. Adversarial Objective / Intent

M04 **MUST NOT** assume every adversarial scenario is intended to cause failure.

Possible intents (non-exhaustive):

```text
violate an invariant
reach an economic boundary
expose unexpected economic behavior
exercise failure paths
test idempotency
test conservation
test authorization
test settlement / fee behavior
test state/history consistency
```

These are **test intents**, not economic truth.

```text
adversarial intent  ≠  actual economic outcome
```

Actual outcomes come from M01/M02 after M03 execution.

---

## 13. Target Model

### 13.1 Allowed transformation targets

| Target | Allowed | Notes |
| --- | --- | --- |
| Scenario metadata (id/version labels carried into derived artifact) | Yes | Must remain inspectable |
| Action (insert/delete/replace/duplicate) | Yes | Resulting sequence must stay M03-ordered |
| Action parameters | Yes | Within Action/World structural rules |
| Action ordering | Yes | Deterministic permutation of declared indices |
| Initial EconomicState | Yes | Declared Scenario input only; governed by §8.7 |
| ExecutionContext fields carried by Scenario | Yes | Never host clock; only declared logical inputs |
| Execution configuration (limits, config id) | Yes | Invalid configs → `DERIVED + INVALID` or config `ERROR` |
| Invariant evaluation plan | Yes | Declares plan; does not evaluate |
| Stop policy | Yes | Declares policy; does not redefine M03 stop semantics |

Action insert/delete/replace/duplicate and parameter edits follow §8.4.2 and
§10.3.

### 13.2 Forbidden targets

| Target | Forbidden |
| --- | --- |
| Internal mutable M01 engine state mid-evaluation | Yes |
| Fabricating Transactions/Events/Effects as economic truth | Yes |
| Overwriting M01 dispositions after the fact | Yes |
| Mutating M02 PASS/FAIL/ERROR results | Yes |
| Hidden host/environment inputs | Yes |

Validation of structural legality is owned jointly by:

* M04 candidate validation (pre-execution)
* M03 Scenario validation / execution
* M01 evaluation (economic/config outcomes)

M04 **MUST NOT** claim economic validity solely because generation succeeded.

---

## 14. Validity Model

| Class | Meaning |
| --- | --- |
| `DERIVED + VALID` | Candidate accepted by Domain/M03 contract; executable via M03 |
| `DERIVED + INVALID` | Candidate produced but violates declared requirements; not silently repaired |
| `NON_APPLICABLE` | Preconditions unmet (normal outcome under `ALLOW_NON_APPLICABLE`) |
| M04 `ERROR` | M04 cannot correctly complete the application / generation unit |

These **MUST NOT** be conflated. `DERIVED + INVALID` is not automatically an
`ERROR` (§8.5).

---

## 15. Error Model

### 15.1 M04 error classes (Gate-4 minimum)

| Class | Meaning |
| --- | --- |
| `INVALID_ADVERSARIAL_DEFINITION` | Transformation/plan definition malformed |
| `INVALID_ADVERSARIAL_CONFIGURATION` | Generator/limits/config illegal |
| `INVALID_BASE_SCENARIO` | Base Scenario missing/invalid for M04 |
| `NON_APPLICABLE_TRANSFORMATION` | Required applicability failed (`REQUIRE_APPLICABLE`) |
| `INCOMPATIBLE_TRANSFORMATION` | Composition/conflict between transformations |
| `INVALID_TRANSFORMATION_PARAMETER` | Parameter out of declared ParameterDomain |
| `COMPOSITION_ERROR` | Composition plan cannot be applied correctly |
| `GENERATION_ERROR` | Enumeration/generation cannot complete under declared rules (including limit breach without truncation policy) |
| `ENGINE_ERROR` | Internal M04 fault |

`NON_APPLICABLE` as a **normal outcome** is not this error class (§8.6).

### 15.2 Downstream ownership preserved

| Failure | Owner |
| --- | --- |
| M01 dispositions / KernelError | M01 |
| M02 PASS/FAIL/ERROR | M02 |
| M03 simulation errors / fatal / limits | M03 |
| Adversarial generation/validation | M04 |

Downstream M01/M02/M03 errors **MUST NOT** be converted into M04 error classes.

### 15.3 Validation phases and error precedence (R-04)

Errors are reported by **first failing phase**, not by incidental discovery
order inside an unspecified scan.

```text
PHASE 1 — transformation definition validation
PHASE 2 — generator configuration validation
PHASE 3 — base scenario validation
PHASE 4 — transformation parameter validation
PHASE 5 — applicability evaluation
PHASE 6 — composition compatibility
PHASE 7 — candidate generation (incl. limits)
PHASE 8 — engine execution (internal M04 faults)
```

#### Phase → error class mapping

| Phase | Primary error class(es) |
| --- | --- |
| 1 | `INVALID_ADVERSARIAL_DEFINITION` |
| 2 | `INVALID_ADVERSARIAL_CONFIGURATION` |
| 3 | `INVALID_BASE_SCENARIO` |
| 4 | `INVALID_TRANSFORMATION_PARAMETER` |
| 5 | `NON_APPLICABLE_TRANSFORMATION` (only under `REQUIRE_APPLICABLE`; else normal `NON_APPLICABLE`) |
| 6 | `INCOMPATIBLE_TRANSFORMATION` / `COMPOSITION_ERROR` |
| 7 | `GENERATION_ERROR` |
| 8 | `ENGINE_ERROR` |

#### Within-phase ordering

Within a phase, the error reported is the first failure under the phase’s
**declared deterministic order**:

* Phase 1: transformation definitions in plan vector order
* Phase 2: configuration fields in the declared configuration schema order
* Phase 3: base Scenario required-field checklist order (id → version → world →
  initial_state → actions → maximum_action_steps → configuration_id →
  declared_unix_secs → invariant_plan → stop_policy)
* Phase 4: parameter dimensions in declared ParameterDomain dimension order
* Phase 5: transformations in plan vector order
* Phase 6: adjacent pairs `(T_i, T_(i+1))` in plan order; then declared
  incompatibility rule order
* Phase 7: candidate enumeration order (§29 / §17A)
* Phase 8: single engine fault (implementation-defined detail, but class is
  `ENGINE_ERROR`)

#### Normative precedence summary

```text
definition error (Phase 1)
    >
configuration error (Phase 2)
    >
base scenario error (Phase 3)
    >
parameter error (Phase 4)
    >
applicability-required error (Phase 5)
    >
composition error (Phase 6)
    >
generation error (Phase 7)
    >
engine error (Phase 8)
```

---

## 16. Determinism

Given identical:

```text
Base Scenario
World / configuration embedded therein
Transformation definition(s)
Transformation parameters / ParameterDomain
Generator configuration (limits, ordering, ApplicabilityPolicy)
M04 engine version
optional declared seed field (if future randomness authorized)
```

M04 **MUST** produce semantically equivalent adversarial output, including:

```text
candidate sequence / order
transformation application order
parameter candidate order
classification of NON_APPLICABLE
error precedence (phase + within-phase order)
limit boundary behavior
composition target resolution
derived Scenario semantic fields
provenance semantic fields
```

Two conforming implementations **MUST NOT** merely generate the same unordered
set. They **MUST** reproduce the same declared ordered result sequence where
ordering is normative (§29).

Prohibited authoritative dependencies:

```text
wall clock
random global state
environment
filesystem
network
database
process ID
thread scheduling
HashMap/HashSet iteration order
LLM response
```

---

## 17. Randomness

TASK-10 / Gate-4 default:

```text
No randomness.
```

Future controlled randomness (**AD-13**), if ever authorized, **MUST** be:

* explicitly declared
* seed-controlled
* reproducible
* versioned
* isolated from host/global RNG
* incapable of changing M01 economic authority

Do **not** introduce RNG crates or implementations in this task.

---

## 17A. Parameter Candidate Generation Semantics (R-03; closes AD-08)

### 17A.1 Model

```text
ParameterDomain
CandidateOperator
CandidateOrdering
CandidateLimit
```

A `ParameterDomain` declares one or more **dimensions**. Each dimension has:

```text
dimension identity (declared order in the domain)
value type (Gate-4 amounts: signed integer minor units / i128 semantics)
optional inclusive bounds [lo, hi]
declared CandidateOperator set
```

### 17A.2 Boundary operators (integer minor units)

Because M01 authoritative amounts use integer minor-unit semantics (ADR 0001 /
EK-NUM), boundary operators **MUST** operate on the declared exact
representation. **No floating-point approximation.**

For a discrete integer domain and threshold `x`:

```text
MIN          = declared domain lower bound (must be explicit)
MAX          = declared domain upper bound (must be explicit)
EXACT(x)     = x
JUST_BELOW(x) = x - 1   (one minor unit)
JUST_ABOVE(x) = x + 1   (one minor unit)
```

Checked bounds:

* If `JUST_BELOW(x)` overflows `i128` min, or leaves the declared domain
  `[lo, hi]`, the operator yields an explicit deterministic
  `NON_APPLICABLE` for that candidate (or `INVALID_TRANSFORMATION_PARAMETER`
  if `x` itself was outside the parameter domain).
* Same for `JUST_ABOVE(x)` at the upper extreme.
* Do **not** saturate, wrap, or invent a nearest in-domain substitute.

### 17A.3 Multi-dimension candidates

* Parameter lists are ordered by the declared dimension order of the
  `ParameterDomain`.
* Cartesian combinations are formed in **lexicographic order** over declared
  parameter dimensions (dimension 0 is the major key).
* Each dimension’s candidate values are ordered per §17A.4 before the
  Cartesian product is taken.

### 17A.4 Candidate value ordering

For integer candidates:

```text
ascending numeric order
```

unless the transformation explicitly declares another valid deterministic
ordering in its definition.

For multiple dimensions:

```text
lexicographic order using declared dimension order
```

If generation uses a finite source collection, that collection’s ordering
**MUST** itself be declared (for example: Scenario action vector order, or an
explicit sorted id list). No HashMap/HashSet/filesystem order.

### 17A.5 CandidateLimit

`maximum_parameter_candidates` (§18) bounds the number of **parameter tuples
evaluated**. Behavior on breach follows §18 (default: `GENERATION_ERROR`, no
silent truncation unless the plan declares truncation).

---

## 18. Enumeration and Explosion Control (R-08; closes AD-09 boundary)

Required explicit configuration concepts:

```text
maximum_generated_scenarios
maximum_transformations_per_plan
maximum_composition_depth
maximum_action_mutations
maximum_parameter_candidates
```

Default **numeric values** for these maxima are not frozen here (plan/config
must supply them). Missing or illegal maxima →
`INVALID_ADVERSARIAL_CONFIGURATION`.

### 18.1 What is counted

| Limit | Counter | Increments when |
| --- | --- | --- |
| `maximum_generated_scenarios` | generated scenario count | A transformation application yields `DERIVED` (VALID or INVALID). `NON_APPLICABLE` and `ERROR` do **not** increment this counter |
| `maximum_transformations_per_plan` | plan transformation entries | Count of transformation entries declared in the plan (static). Exceeding at plan validation → Phase 2 `INVALID_ADVERSARIAL_CONFIGURATION` |
| `maximum_composition_depth` | composition depth | Number of transformations applied in one composition chain (`k` in `T_0…T_(k-1)`). Checked before application; depth `k > N` → `GENERATION_ERROR` or config error if declared illegally |
| `maximum_action_mutations` | action mutation count | Each successful action insert/delete/replace/duplicate/reorder operation that modifies the action sequence in a `DERIVED` result. `NON_APPLICABLE` does not increment |
| `maximum_parameter_candidates` | parameter candidate count | Each parameter tuple **evaluated** (including those that yield `NON_APPLICABLE` or `DERIVED`). Evaluation attempt increments before classification |

### 18.2 Inclusivity and breach

```text
A maximum N means at most N counted units (inclusive).
Attempting to produce / evaluate unit N+1 is a GENERATION_ERROR
(Phase 7), unless the plan explicitly declares TruncationPolicy.
```

**Normative default TruncationPolicy:** `FAIL_ON_EXCEED` (no silent
truncation).

If a plan explicitly declares `TRUNCATE_AT_N`, generation stops after N
counted units for that counter, records truncation in provenance, and does
**not** raise `GENERATION_ERROR` for the omitted remainder.

### 18.3 Non-applicable vs counters

| Outcome | generated scenario count | parameter candidate count | action mutation count |
| --- | --- | --- | --- |
| `DERIVED` | +1 | +1 (if driven by a parameter tuple) | +1 per mutating action op |
| `NON_APPLICABLE` | no | +1 (if a parameter tuple was evaluated) | no |
| `ERROR` | no | +1 if evaluation began; then generation aborts per precedence | no |

### 18.4 Ordering

Ordering of enumerated candidates **MUST** follow §29 / §17A.

No hidden heuristics.

---

## 19. Deduplication

Whether semantically equivalent adversarial scenarios are deduplicated is
**AD-06** (Open Decision).

If enabled in a future freeze:

* define equivalence
* define canonical identity
* preserve provenance of discarded duplicates
* use deterministic collision handling

Do **not** assume byte equality equals semantic equality unless specified.

Gate-4 default for READY_FOR_REVIEW: **no silent deduplication**; generators
emit the finite declared candidate set under limits. Equivalence policy remains
OPEN.

---

## 20. Provenance

Every derived adversarial scenario **MUST** preserve provenance sufficient to
reconstruct:

```text
base scenario identity / version
intermediate scenario version / identity (each composition step)
transformation identity / version
transformation parameters
target resolution method (ActionId vs positional index)
ApplicabilityPolicy
candidate generation ordering (plan + dimension + value order)
composition position
generation counters / limits (and TruncationPolicy if any)
resulting ValidationClassification (VALID | INVALID)
generator configuration
M04 engine version
optional seed field (if present)
parent scenario identity
full derivation chain
```

No hidden derivation state. Provenance is first-class evidence, not optional
logging.

---

## 21. Reproducibility

### 21.1 Contract

Given identical declared inputs and M04 engine version, another conforming
implementation **MUST** reproduce the same **semantic ordered** adversarial
result sequence (and provenance classifications).

### 21.2 Semantic equivalence (minimum)

```text
same ordered candidate sequence
same transformation application order
same parameter candidate order
same NON_APPLICABLE classifications and positions
same error class and phase when generation fails
same limit boundary behavior
same composition target resolution outcomes
same derived Scenario fields that affect M03 execution
same adversarial category/intent labels
same ValidationClassification
same provenance semantic fields listed in §20
```

Non-requirements:

* identical memory layout
* identical internal intermediate objects
* identical host paths / wall-clock stamps

Formal equality operators remain related to Domain Contract DC-12 /
**AD-11**.

---

## 22. Shrinking / Minimization

**DEFERRED.**

Shrinking is **not** part of the Gate-4 freeze candidate unless a later
remediation fully specifies:

* target property
* equivalence under shrink
* termination
* determinism
* non-interference with M01/M02 authority

A shrinker **MUST NOT** determine invariant truth or modify M01 semantics.

Recorded as **AD-12**.

---

## 23. LLM Boundary

An LLM may optionally **propose**:

* candidate transformations
* candidate parameters
* candidate adversarial intents
* candidate scenario descriptions

LLM output is **ALWAYS untrusted**.

It **MUST** pass deterministic validation before becoming an executable M04
artifact.

LLM **MUST NOT**:

* mutate authoritative state
* determine economic outcome
* determine invariant PASS/FAIL/ERROR
* bypass validation
* inject hidden randomness
* be required for deterministic execution

M04 **MUST** remain fully usable without an LLM.

---

## 24. M04 / M03 Boundary

| Concern | Owner |
| --- | --- |
| Scenario execution sequencing | M03 |
| Execution limits / termination | M03 |
| ExecutionContext construction for M01 | M03 |
| State adoption from KernelOutcome | M03 |
| Adversarial derivation of Scenario inputs | M04 |

M04 **MUST NOT** duplicate M03 simulation semantics.

---

## 25. M04 / M02 Boundary

M04 may **declare** an invariant evaluation plan on a derived Scenario.

M04 **MUST NOT** evaluate invariants.

M02 remains authoritative for PASS/FAIL/ERROR/Violations.

M04 may record downstream M02 results as evidence **after** M03 execution, but
must not reinterpret them as economic truth.

---

## 26. M04 / M01 Boundary

M04 constructs inputs. M01 evaluates economics.

M01 remains authoritative for balances, transactions, events, fees, prices,
conversions, dispositions, and foundational consistency.

---

## 27. Evidence

M04 evidence **MUST** answer:

```text
What base scenario was used?
What transformation was applied?
Why was it applicable / non-applicable?
What parameters were used?
What derived scenario resulted?
Was the scenario Valid / Invalid?
Was it executed via M03?
Which M03 run / SimulationResult reference applies?
Which M01 outcomes resulted (if executed)?
Which M02 evaluations resulted (if executed)?
```

Lifecycle labels (distinct):

```text
GENERATED
REJECTED
NON_APPLICABLE
INVALID
EXECUTED
```

Do **not** fabricate downstream M01/M02/M03 evidence when execution did not
occur.

---

## 28. Adversarial Result Model

Layers **MUST** remain distinct:

```text
1. Generation result
2. Validation result
3. Derived scenario (if any)
4. Execution reference / SimulationResult (if executed)
5. Downstream M02 evaluations (if any)
6. M04 errors (if any)
```

**Never** collapse into a single PASS/FAIL.

In particular:

```text
adversarial generation succeeded  ≠  economic system failed
```

---

## 29. Deterministic Ordering (R-07; closes AD-07)

Normative generation ordering hierarchy:

```text
1. transformation plan vector order
2. transformation candidate order (per transformation’s declared candidate set,
   or single application)
3. parameter-dimension order (ParameterDomain declaration order)
4. candidate value order (§17A.4; integers ascending unless declared otherwise)
5. composition order (left-to-right plan indices 0 .. k-1)
```

The generated scenario sequence **MUST** be reproducible under this hierarchy.

Never depend on:

```text
HashMap / HashSet iteration
filesystem order
thread completion order
network order
```

If ordering cannot be defined for a generation plan:

```text
→ INVALID_ADVERSARIAL_CONFIGURATION (Phase 2) or GENERATION_ERROR (Phase 7)
```

---

## 30. Generation Idempotency

**AD-19** (Open Decision), with Gate-4 interim rule for review:

```text
Applying the same transformation with the same parameters to the same base
under the same generator configuration MUST produce semantically equivalent
derived Scenario payloads and provenance classification.
```

Whether emitting two identical candidates is an error, a duplicate marker, or
allowed multiplicity is **OPEN** pending freeze (**AD-06** / **AD-19**).

---

## 31. Scenario Identity

See §6.3 and **AD-02** / **AD-03**.

---

## 32. Versioning

Versioning **MUST** exist conceptually for:

```text
adversarial transformation definitions
generator configuration
derived scenarios
provenance records
reproducibility identity
M04 engine version
```

Changing semantics requires explicit version bumps; prior evidence must remain
interpretable against the version that produced it (**AD-18**).

---

## 33. Serialization

Concrete JSON/YAML/TOML/binary formats are **NOT** frozen (**AD-14**).

Semantic requirements for any future serialization:

```text
complete enough for reproducibility
explicit
deterministic
versioned
no hidden fields
no implementation-specific economic truth
```

---

## 34. Security / Safety Boundary

M04 is **economic testing infrastructure**.

It is **NOT**:

* a production attack engine
* a real-money exploit framework
* a blockchain attack executor
* a credential system
* a live transaction executor

**No** real external execution, network side effects, production wallets, live
blockchains, or real payments in Gate-4 M04 semantics.

---

## 35. Distinction from M05 Chaos

| Module | Role |
| --- | --- |
| M04 | Adversarial **scenario** generation/transformation (hostile/abnormal declared scenarios) |
| M05 | Economic **chaos / fault injection** during or around execution (deferred) |

M04 **MUST NOT** absorb M05 responsibilities in this specification.

---

## 36. Open Decisions (AD register)

| ID | Topic | Status |
| --- | --- | --- |
| AD-01 | Adversarial taxonomy closure / extension process | OPEN |
| AD-02 | Transformation identity algorithm | OPEN |
| AD-03 | Scenario identity algorithm | OPEN |
| AD-04 | Formal precondition language for applicability | OPEN |
| AD-05 | Duplicate transformation behavior in composition | **CLOSED_FOR_GATE-4_SEMANTICS** — explicit plan repeats allowed; no silent dedupe (§10.2) |
| AD-06 | Deduplication / semantic equivalence | OPEN (default: no silent dedupe) |
| AD-07 | Candidate / generation ordering | **CLOSED_FOR_GATE-4_SEMANTICS** — hierarchy in §29 / §17A |
| AD-08 | Parameter candidate generation | **CLOSED_FOR_GATE-4_SEMANTICS** — §17A |
| AD-09 | Generation limit boundary | **CLOSED_FOR_GATE-4_SEMANTICS** — §18; default numeric values remain **IMPLEMENTATION_DETAIL_OPEN** (must be supplied by config) |
| AD-10 | Concrete provenance schema | OPEN |
| AD-11 | Formal reproducibility equality operators | OPEN |
| AD-12 | Shrinking / minimization | **DEFERRED** |
| AD-13 | Randomness policy / RNG source | OPEN (default: none) |
| AD-14 | Serialization format | OPEN |
| AD-15 | Concrete M04 result Rust/API types | OPEN |
| AD-16 | NonApplicable vs error distinction | **CLOSED_FOR_GATE-4_SEMANTICS** — §8.5 / §8.6 |
| AD-17 | LLM proposal API surface | OPEN |
| AD-18 | Versioning scheme details | OPEN |
| AD-19 | Generation idempotency / duplicate emission policy | OPEN |
| AD-20 | CLI/API boundary | OPEN |

Do not resolve remaining OPEN items merely for convenience.

---

## 37. Implementation Status

```text
STATUS: READY_FOR_REVIEW
NORMATIVE FREEZE: NOT FROZEN
IMPLEMENTATION: BLOCKED
GATE 4: OPEN
```

TASK-10 does **NOT** authorize implementation.

A separate implementation task is required after independent specification
audit and freeze.

---

## 38. Gate 4 Readiness Criterion

> Two competent engineers reading only the frozen M04 specification and the
> existing frozen M01/M02/M03 specifications should be able to implement M04
> without materially disagreeing about scenario/transformation models,
> applicability, composition, determinism, provenance, error boundaries, or
> M01/M02/M03 authority splits.

If materially different answers are possible to the checklist in TASK-10 §37,
the document is **NOT** ready to freeze.

This TASK-10 document is **READY_FOR_REVIEW**, not frozen.

---

## 39. Testing Requirements (for future implementation)

Future M04 implementation tests **MUST** cover at least:

```text
base scenario required / missing base error
single mutation derive
non-applicable transformation explicit outcome
invalid parameter error
composition order determinism
incompatible composition error
generation limits
deterministic candidate ordering
provenance chain completeness
valid derived scenario executes via M03
M04 does not mutate M01 state independently
M02 not evaluated by M04
LLM proposals rejected without validation (when integrated later)
no host/env dependency
replay of identical generation inputs
```

---

## 40. Foundational M04 Invariants (specification-level)

### M04-INV-01

M04 never bypasses M01 for authoritative economic transitions.

### M04-INV-02

M04 never evaluates invariants (M02 authority).

### M04-INV-03

M04 never replaces M03 execution sequencing.

### M04-INV-04

Every derived executable scenario is explicit and inspectable.

### M04-INV-05

Every derivation has reconstructible provenance.

### M04-INV-06

Identical declared generation inputs yield semantically equivalent outputs.

### M04-INV-07

Invalid / non-applicable / engine-error classes are not silently converted into
Valid success.

### M04-INV-08

Adversarial intent is never treated as authoritative economic outcome.

### M04-INV-09

Generation limits are explicit and deterministic.

### M04-INV-10

No hidden environment state participates in authoritative generation semantics.

---

## 41. Document Control

| Item | Value |
| --- | --- |
| Created by | TASK-10 |
| Remediation | **TASK-10R** — Gate-4 semantic blockers R-01…R-08 |
| Status | **READY_FOR_REVIEW** |
| Normative freeze | **NOT FROZEN** |
| Implementation authorization | **NONE** (blocked) |
| Gate | **GATE 4 — OPEN** |
| Code / crates modified by this task | **None** |
| Frozen specifications modified | **None** |
| Next | Independent freeze audit (**TASK-10F**) after authorization |

### Amendment rule

Material semantic change after freeze requires explicit specification
amendment. Until freeze, review comments may produce further remediation
without treating this draft as implementation authority.
