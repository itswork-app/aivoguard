# AivoGuard Adversarial Scenario Engine Specification

| Field | Value |
| --- | --- |
| Document | `docs/design/adversarial-scenario-engine-specification.md` |
| Task | **TASK-10** … **TASK-10RRRR** (targeted freeze remediation R2-B-01/R2-B-02) |
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
TASK-10R through TASK-10RRRR (including R2-B-01 / R2-B-02 closure). It is
**not frozen**. Material freeze requires an explicit freeze audit task.
Implementation requires a later explicit authorization task after freeze.

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

A DERIVED candidate that passed the closed M04 structural checklist
(`DERIVED + VALID`, §8.5.1). Does not guarantee M03 execution success or M01
economic success.

### Invalid Adversarial Scenario

A DERIVED candidate that failed the closed M04 structural checklist
(`DERIVED + INVALID` with `StructuralInvalidReason`); not silently “fixed”.

### M04 Engine Failure

A failure of M04 itself (distinct from M01/M02/M03 errors).

### Provenance

The reconstructible derivation chain from base → transformations → derived
scenario (§20).

### Scenario Snapshot

The explicit intermediate Scenario artifact on which a transformation is
applied (the base, or the result of prior composition steps).

### TransformationApplication

One evaluation of a declared transformation against a ScenarioSnapshot with
concrete parameters. Counted by transformation-evaluation counters (§18).
Produces `DERIVED`, `NON_APPLICABLE`, or `ERROR`.

### CompositionIntermediate

A ScenarioSnapshot produced solely as an internal step of one declared
composition chain. **Not** automatically an externally emitted adversarial
scenario (§18).

### GeneratedAdversarialScenario

A candidate artifact that is **emitted** as a generated adversarial scenario
result (subject to M04 structural validation classification). Counted by
`maximum_generated_scenarios` (§18).

### ParameterValue / ParameterDomain / ParameterDimension

Conceptual parameter typing for transformations (§17A). Not a Rust freeze.

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
* validate derived candidates against the **closed** M04 structural checklist
  (§8.5.1)
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

### 6.3 Identity stability vs identity algorithm (R-13)

Two distinct concepts:

```text
Identity stability     = NORMATIVE Gate-4 semantic property
Identity derivation algorithm = AD-03 (OPEN)
```

#### Identity stability (normative)

Identity **MUST NOT** rely solely on memory address or ephemeral runtime IDs.

For identical declared:

```text
Base Scenario
Transformation definition / version
Transformation parameters
Generator configuration
ApplicabilityPolicy
M04 engine version
declared seed, if any
```

the derived scenario identity **MUST** be semantically stable under the same
identity/versioning contract used by that implementation/engine version.

Identity **MUST** be:

* deterministic
* provenance-linkable
* independent of memory address
* independent of host/process state
* independent of wall clock
* independent of random global state

Changing any declared input that is identity-relevant **MUST NOT** silently
retain the prior identity (see also metadata projection §8.4.1).

#### Identity algorithm (OPEN — AD-03)

Whether identity is **declared**, **content-derived**, or hybrid — and which
hash/string format is used — remains **AD-03**. Cryptographic hash algorithms,
UUID schemes, and exact string formats are **not** frozen.

Two conforming implementations **MAY** produce different identity *strings*
if they use different AD-03 algorithms. §21 requires **semantic** ordered
equivalence of scenario payloads, classifications, and provenance fields —
**not** byte-identical identity strings across distinct identity algorithms.

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
| Duplicate Action | Inserts a copy at a declared insertion index. **Default identity rule (R-16):** the duplicate’s `action_id` is set to `None` (`REMOVE_EXPLICITLY`) unless TransformationParameters **explicitly** declare either (a) `REPLACE_EXPLICITLY` with a new ActionId that is unique in the resulting current sequence, or (b) `INHERIT` of the source ActionId. Silent implicit id invention from host state is forbidden. `INHERIT` that produces a non-unique ActionId is permitted as a Scenario payload, but subsequent ActionId target resolution against that id is ambiguous and fails deterministically (§10.3) |
| Execution ordering | Resulting sequence order is the declared post-transformation order; M03 executes that order |
| Unrelated Actions | Remain byte-for-byte / field-for-field unchanged |
| Replacement parameters | Represented as explicit TransformationParameters naming the Action field(s) replaced |
| Invalid resulting Action | If the transformation successfully emits a candidate that fails the closed M04 structural checklist (§8.5) → `DERIVED + INVALID` with a checklist reason code. If parameters are outside the transformation’s declared ParameterDomain / type → `ERROR` / `INVALID_TRANSFORMATION_PARAMETER` |

A transformation **MUST NOT** silently alter unrelated Scenario fields
(including World, initial state, plans, stop policy, or execution config)
unless those fields have a declared non-`INHERIT_UNCHANGED` projection.

### 8.5 Transformation result lifecycle and closed structural validation (R-12 / R-15)

```text
TransformationApplicationResult
    = DERIVED(candidate, ValidationClassification)
    | NON_APPLICABLE(reason)
    | ERROR(M04Error)
```

```text
ValidationClassification = VALID | INVALID
```

Conceptual pipeline (does **not** redefine M03):

```text
Base Scenario
    ↓
TransformationApplication
    ↓
Candidate Artifact
    ↓
M04 Closed Structural Validation (§8.5.1)
    ├── INVALID  → DERIVED + INVALID (StructuralInvalidReason; not silently repaired)
    └── VALID    → DERIVED + VALID
          ↓
        M03 Scenario acceptance / execution
          ↓
        M01 economic evaluation
          ↓
        M02 invariant evaluation (if planned)
```

```text
M04 structural validation
        ≠
M03 scenario acceptance / execution
        ≠
M01 economic evaluation
        ≠
M02 invariant evaluation
```

#### What `DERIVED + VALID` means

The candidate passed the **closed** M04 structural checklist (§8.5.1) in full.

It does **NOT** mean M03 execution is guaranteed to succeed, nor that any
economic Action will succeed, nor any M02 result.

#### What `DERIVED + INVALID` means

M04 successfully produced a candidate artifact, but the closed M04 structural
checklist failed. It **MUST** include exactly one primary
`StructuralInvalidReason` (first failing check). It **MUST NOT** be silently
repaired.

`DERIVED + INVALID` does **NOT** mean:

```text
M03 rejected execution
M01 Rejected / FailedEconomic / InvalidInput / ConfigurationError
M02 PASS / FAIL / ERROR
```

Do **not** automatically promote `DERIVED + INVALID` to a transformation
`ERROR`.

```text
generation success ≠ M04 structural validity ≠ M03 acceptance ≠ economic failure
```

#### 8.5.1 Closed M04 structural validation checklist (MUST)

For every **emitted** candidate (and for any candidate classified `DERIVED`
before emission classification), M04 **MUST** run exactly these checks, in
this order. First failure wins; later checks are not evaluated for the
primary reason.

| Order | Check | MUST check | MUST NOT check | Failure reason |
| --- | --- | --- | --- | --- |
| 1 | Scenario envelope | Candidate has Scenario-shaped envelope (id, version, world, initial_state, actions, maximum_action_steps, configuration_id, declared_unix_secs, invariant_plan, stop_policy fields present as declared Scenario structure) | Economic truth of any Action | `INVALID_SCENARIO_STRUCTURE` |
| 2 | Required fields | `id` non-empty; `version` non-empty; `configuration_id` non-empty; `maximum_action_steps` is a declared non-negative integer representable as M03 `u64` semantics; `actions` is an ordered sequence (length may be 0) | Whether the scenario “should” have actions | `INVALID_REQUIRED_FIELD` |
| 3 | Projection consistency | Every Scenario field touched by the producing transformation(s) has a declared projection mode; no field changed without declared mode; metadata `id`/`version` updated when payload fields changed (§8.4.1) | Semantic desirability of the projection | `INVALID_PROJECTION` |
| 4 | World / initial-state structure | **Sub-order (R-20):** **4a** World envelope structure first — World is present as the declared Scenario `world` structure. On failure → `INVALID_SCENARIO_STRUCTURE` (stop; do not evaluate 4b for primary reason). **4b** only after 4a passes — `initial_state` exists as declared structure; balance cells are explicit `(account, asset, facet) → signed integer minor-unit amount`; no hidden/internal M01 state; no fabricated Transaction/Event/Effect embedded as state; no host/env-derived amounts (§8.7). On failure → `INVALID_INITIAL_STATE_STRUCTURE` | Membership truth, authorization, economic validity, fees, prices, settlement, Action success | `INVALID_SCENARIO_STRUCTURE` (4a) / `INVALID_INITIAL_STATE_STRUCTURE` (4b) |
| 5 | Action sequence structure | Each Action is one of the Domain/M01 Action variants with required variant fields present and correctly typed (including `ECONOMIC_AMOUNT` vs `INTEGER` separation for numeric fields) | Whether M01 would Accept/Reject/Fail the Action | `INVALID_ACTION_STRUCTURE` |
| 6 | Action identity / reference validity | Positional indexes used by the producing transform were in-range at application time (recorded in provenance); ActionId references used by the producing transform resolved with match-count = 1 (§10.3); no unresolved required reference left in the transform application record | Future M03 sequencing preferences | `INVALID_ACTION_REFERENCE` |
| 7 | Domain reference representability | Every required Actor/Account/Asset/Facet/Price/ActionId (when `Some`) appearing in Actions or initial_state cells is an `IdentifierToken` (§8.5.3): UTF-8 string length ≥ 1; exact equality; case-sensitive; no trim/normalize/case-fold. Empty → fail. **World membership is NOT checked** | World membership, account/asset existence, actor authorization, ownership, balance sufficiency | `INVALID_DOMAIN_REFERENCE` |
| 8 | Execution configuration | `maximum_action_steps`, `configuration_id`, and `declared_unix_secs` (if `Some`) have structurally legal representations; `configuration_id` is an `IdentifierToken` / non-empty string per §8.5.3; `declared_unix_secs` is not sourced from host clock | Whether the limit is “wise” | `INVALID_EXECUTION_CONFIGURATION` |
| 9 | Invariant-plan structure | `before_action` / `after_action` / `on_completion` vectors are present (may be empty); each declared invariant entry has the **minimum** envelope `{ id, definition_version }` where both are non-empty UTF-8 strings under §8.5.3 exact-token rules. **Does not evaluate invariants** | PASS/FAIL/ERROR, applicability, target compatibility, quantification, aggregation | `INVALID_INVARIANT_PLAN_STRUCTURE` |
| 10 | Stop-policy structure | `stop_policy.conditions` is an ordered vector (may be empty); each condition is one of the M03 `StopCondition` shapes with required fields present | Whether a condition will fire | `INVALID_STOP_POLICY_STRUCTURE` |
| 11 | Hidden-input prohibition | No field of the candidate is marked or evidenced as derived from wall clock, env, filesystem, network, process state, global RNG, or unvalidated LLM output | — | `FORBIDDEN_HIDDEN_INPUT` |
| 12 | Provenance completeness | Emitted candidate carries the provenance fields required by §20 for reconstruction | Beauty of provenance formatting | `INVALID_PROVENANCE` |

If all twelve checks pass → `DERIVED + VALID`.

#### 8.5.2 M04 MUST NOT check (closed exclusion)

```text
economic correctness / balances after execution
authorization truth
fee / price / conversion / settlement correctness
balance sufficiency
transaction success / M01 dispositions
invariant PASS / FAIL / ERROR
M03 termination / stop firing
M03 Scenario acceptance beyond the closed checklist above
```

The exact M03 acceptance/execution contract remains owned by M03 for any
semantic not listed in §8.5.1.

#### 8.5.3 IdentifierToken minimum (R-20)

For all identifier tokens checked by M04 structural validation (and for
ActionId / transformation-identity equality elsewhere in M04):

```text
IdentifierToken:
    UTF-8 string
    length >= 1
```

Normative comparison rules:

```text
exact string equality
case-sensitive
NO trimming
NO Unicode normalization
NO case-folding
NO whitespace collapsing
NO fuzzy matching
```

Examples (token structure only — not World membership):

```text
"abc"       VALID token
" A "       VALID token
"abc def"   VALID token
"Ä"         VALID token
""          INVALID token
```

Empty required identifier → `INVALID_DOMAIN_REFERENCE` under Check #7 (or
`INVALID_INVARIANT_PLAN_STRUCTURE` / `INVALID_REQUIRED_FIELD` when the empty
token is the Scenario `id` / invariant `id` / `definition_version` under the
check that owns that field).

M04 MUST NOT impose a richer identifier grammar unless already required by a
frozen upstream specification. A valid token does **not** establish World
membership.

No automatic repair, canonicalization, or schema normalization.

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
| Structural Domain/M03 representability | Closed M04 checklist (§8.5.1); then M03 |
| M03 Scenario acceptance / execution | M03 |
| Authoritative economic consequences of Actions | M01 after M03 execution |
| Intentionally invalid candidate | `DERIVED + INVALID` with `StructuralInvalidReason` |

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
| Conflicts | Only via the closed incompatibility contract (§10.4). Default = compatible |
| Depth | Bounded by `maximum_composition_depth` (§18) |
| Identity | Derived scenario identity/provenance includes full ordered chain |
| Explosion | Bounded by generation limits (§18) |

No implicit composition outside a declared plan.

### 10.3 Composition target identity (R-06 / R-16)

Transformation references **MUST** resolve against the ScenarioSnapshot on
which the transformation is applied (the current intermediate scenario after
every previous transformation in the declared composition sequence).

#### Addressing modes

| Mode | Semantics |
| --- | --- |
| `ActionId` addressing | Resolve by exact `action_id` equality in the **current** intermediate `actions` sequence |
| Positional `INDEX` addressing | Zero-based index `i` means `actions[i]` of the **current** intermediate sequence only. Never original-base positions after prior mutations |

No other addressing mode is normative for Gate-4 Action targets.

#### ActionId resolution (closed; R-16)

`ActionId` is a usable identity reference **only when unique** in the current
intermediate action sequence.

Let `N` = number of Actions in the current intermediate sequence whose
`action_id` equals the requested id (`Some(id)` only; `None` never matches).

| `N` | Classification |
| --- | --- |
| `0` | Missing reference: well-typed non-empty `IdentifierToken` id with no match → `NON_APPLICABLE` under `ALLOW_NON_APPLICABLE`; `ERROR` / `NON_APPLICABLE_TRANSFORMATION` under `REQUIRE_APPLICABLE`. Empty or non-`IdentifierToken` id parameter → `ERROR` / `INVALID_TRANSFORMATION_PARAMETER` (malformed) |
| `1` | Resolve to that unique Action |
| `> 1` | **Ambiguous** → `ERROR` / `INVALID_TRANSFORMATION_PARAMETER` with reason `AMBIGUOUS_ACTION_ID`. **MUST NOT** silently select first/last/any match |

ActionId equality uses `IdentifierToken` exact rules (§8.5.3).


Implementations **MUST NOT** choose among matches using HashMap/HashSet
iteration, thread scheduling, memory address, or any incidental order.

#### Positional INDEX resolution

| Case | Classification |
| --- | --- |
| Index not a non-negative integer / outside ParameterDomain | `ERROR` / `INVALID_TRANSFORMATION_PARAMETER` |
| Index ≥ current sequence length | Missing reference → `NON_APPLICABLE` / required `NON_APPLICABLE_TRANSFORMATION` per §8.6 (same as absent ActionId) |
| Index in range | Resolve to `actions[i]` |

#### Target-resolution provenance (MUST)

Every Action target resolution record **MUST** identify:

```text
addressing mode (ActionId | INDEX)
requested identity or index
match count N
resolved target position (if N = 1)
deterministic ambiguity / missing reason (if N ≠ 1)
```

### 10.4 Incompatible transformation contract (R-14 / R-19)

#### Declaration

Incompatibility **MUST** be **explicitly declared** by the generation /
composition plan as an ordered vector:

```text
IncompatibilityRules: ordered list of IncompatibilityRule
```

Each `IncompatibilityRule` has the conceptual shape:

```text
IncompatibilityRule:
    rule_id
    left_transformation_identity
    right_transformation_identity
    optional left_version
    optional right_version
    optional position_constraint
```

`position_constraint`, when present, **MUST** be exactly:

```text
EXACT_PAIR(left_index, right_index)
```

Indices are **zero-based plan positions**. No other position-constraint
grammar is normative for Gate 4.

Inference of incompatibility by implementation heuristics is **forbidden**.

#### Transformation occurrence

A transformation **occurrence** is one concrete position in the declared
composition plan. Identity alone does **not** collapse repeats.

```text
plan[0] = TRANSFORM_A
plan[1] = TRANSFORM_B
plan[2] = TRANSFORM_A
```

```text
occurrences: A@0, B@1, A@2
```

#### Identity / version matching

An occurrence matches a side of a rule when:

1. its transformation identity equals the rule’s side identity by
   `IdentifierToken` exact equality (§8.5.3); and
2. if the rule specifies a version for that side, the occurrence version equals
   that version by exact equality.

If a side omits version → version is a **wildcard** for that side.

No fuzzy / case-folded / normalized matching.

#### Default

If no `IncompatibilityRule` produces a match:

```text
ordered transformations are COMPATIBLE
```

#### Self-rules

If `left_transformation_identity == right_transformation_identity`, the rule
is a **self-rule** and requires **two distinct** plan positions
(`left_index != right_index`).

A single occurrence of `A` does **not** satisfy `A` incompatible with `A`.

`EXACT_PAIR(a,a)` **MUST NOT** match (not an engine error — the rule is
deterministically non-matching).

#### Matching without position constraint

Enumerate candidate ordered pairs `(left_index, right_index)` where:

```text
left_index != right_index
```

and both occurrences satisfy their side’s identity/version constraints
(self-rules included).

Order candidates lexicographically:

```text
(left_index ASC, right_index ASC)
```

Example — plan `A@0, B@1, A@2, B@3`, rule `A` incompatible with `B`:

```text
(0,1), (0,3), (2,1), (2,3)
```

Self-rule with `A` at 0 and 2:

```text
(0,2), (2,0)   → first match (0,2) wins
```

First candidate pair that satisfies the constraints is the match for that rule.

#### Matching with `EXACT_PAIR(a,b)`

Evaluate **only** `(a,b)`:

1. both positions exist;
2. left occurrence matches left identity/version;
3. right occurrence matches right identity/version;
4. `a != b` (required; `a == b` never matches).

If all pass → rule matches. Otherwise → rule does not match.
**No** alternative pair may be searched.

#### Rule ordering and result

```text
Scan IncompatibilityRules in declared vector order index 0 .. n-1.
First matching rule wins → ERROR class = INCOMPATIBLE_TRANSFORMATION
```

Later incompatibility rules are not evaluated for the primary error.

| Class | Meaning |
| --- | --- |
| `INCOMPATIBLE_TRANSFORMATION` | Declared rule matched under §10.4 |
| `COMPOSITION_ERROR` | Structural composition-plan failure that is **not** declared semantic incompatibility |

#### Evidence (MUST)

```text
rule_id
left_transformation_identity
right_transformation_identity
left_version (if declared)
right_version (if declared)
left_plan_index
right_plan_index
position_constraint (if present)
```

The recorded pair **MUST** be exactly the pair that caused the first matching
rule (lexicographically first matching pair when unconstrained). Recording all
possible pairs is **not** normative for the primary match.

Two conforming implementations **MUST** produce the same matching rule,
`left_plan_index`, `right_plan_index`, error class, and evidence for the same
plan and rule vector.

#### No heuristics

```text
FORBIDDEN: implementation-specific conflict detection,
           overwrite heuristics, silent skip of conflicting steps,
           converting incompatibility into NonApplicable without a rule,
           unordered / HashSet occurrence traversal.
```

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

Validation of structural legality uses the three-layer boundary (§8.5):

* M04 **closed** structural checklist (§8.5.1)
* M03 Scenario acceptance / execution
* M01 economic evaluation (and M02 if planned)

M04 **MUST NOT** claim economic validity solely because generation or M04
structural validation succeeded.

---

## 14. Validity Model

| Class | Meaning |
| --- | --- |
| `DERIVED + VALID` | Closed M04 structural checklist (§8.5.1) passed. Does **not** guarantee M03 success or economic success |
| `DERIVED + INVALID` | Candidate produced, but first failing §8.5.1 check yielded a `StructuralInvalidReason`; not silently repaired |
| `NON_APPLICABLE` | Preconditions unmet (normal outcome under `ALLOW_NON_APPLICABLE`) |
| M04 `ERROR` | M04 cannot correctly complete the application / generation unit |

These **MUST NOT** be conflated. `DERIVED + INVALID` is not automatically an
`ERROR` (§8.5).

Even after `DERIVED + VALID`, M03 may still reject or fatally fail the Scenario
per its frozen contract. M03 acceptance still does not imply economic success
(M01 remains authoritative).

---

## 15. Error Model

### 15.1 M04 error classes (Gate-4 minimum)

| Class | Meaning |
| --- | --- |
| `INVALID_ADVERSARIAL_DEFINITION` | Transformation/plan definition malformed |
| `INVALID_ADVERSARIAL_CONFIGURATION` | Generator/limits/config illegal |
| `INVALID_BASE_SCENARIO` | Base Scenario missing/invalid for M04 |
| `NON_APPLICABLE_TRANSFORMATION` | Required applicability failed (`REQUIRE_APPLICABLE`) |
| `INCOMPATIBLE_TRANSFORMATION` | Declared semantic incompatibility rule matched (§10.4) |
| `INVALID_TRANSFORMATION_PARAMETER` | Parameter out of declared ParameterDomain, or ActionId resolution ambiguous (`AMBIGUOUS_ACTION_ID`, §10.3) |
| `COMPOSITION_ERROR` | Structural composition-plan failure that is **not** a declared semantic incompatibility (§10.4) |
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
* Phase 2: configuration fields in **plan-declared** configuration field order
* Phase 3: base Scenario required-field checklist order (id → version → world →
  initial_state → actions → maximum_action_steps → configuration_id →
  declared_unix_secs → invariant_plan → stop_policy)
* Phase 4: parameter dimensions in declared ParameterDomain dimension order
* Phase 5: transformations in plan vector order
* Phase 6: (a) structural composition-plan checks in plan order →
  `COMPOSITION_ERROR`; then (b) `IncompatibilityRules` in declared vector
  order with occurrence-pair semantics (§10.4 / R-19) → first match
  `INCOMPATIBLE_TRANSFORMATION`
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
emitted candidate sequence / order
transformation application order
parameter candidate order
classification of NON_APPLICABLE
GenerationStatus / TruncationPolicy behavior
error precedence (phase + within-phase order)
limit boundary behavior (emitted vs intermediate)
composition target resolution
derived Scenario semantic fields
provenance semantic fields
identity stability under §6.3 (not cross-algorithm string identity)
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

## 17A. Parameter Candidate Generation Semantics (R-03 / R-09)

### 17A.1 Model

```text
ParameterValue
ParameterDomain
ParameterDimension
CandidateOperator
CandidateOrdering
CandidateLimit
```

A `ParameterDomain` declares one or more `ParameterDimension`s. Each dimension
has:

```text
dimension identity (declared order in the domain)
ParameterValue type (§17A.1.1)
optional inclusive bounds [lo, hi]   (where the type supports bounds)
declared CandidateOperator set       (where operators apply)
```

Concrete Rust enums, parsers, and serialization are **not** frozen.

#### 17A.1.1 ParameterValue type semantics (R-09)

Gate-4 minimum type distinction:

| Type | Meaning |
| --- | --- |
| `IDENTIFIER` | Exact deterministic identity token (e.g. ActionId, AccountId, ActorId, AssetId, FacetId, PriceId, ScenarioId). Equality is exact and deterministic |
| `INTEGER` | Dimensionless signed integer. **MUST NOT** silently become Money / economic amount |
| `ECONOMIC_AMOUNT` | Authoritative economic minor-unit quantity. Follows M01 exact numeric semantics: signed integer minor units; no `f32`/`f64`; checked arithmetic; explicit asset/unit context required |
| `BOOLEAN` | Exact true/false |
| `ENUM` | Value from a declared finite named set (exact match) |
| `INDEX` | Non-negative positional reference into the **current** intermediate sequence (actions or other ordered collections as declared). Out-of-range after composition → applicability / parameter rules in §10.3 |
| `ORDERING_SELECTION` | Declared selection among deterministic permutations / reorderings of a declared ordered collection |

Additional types may remain open implementation detail if not required for
Gate-4 taxonomy coverage. Do **not** collapse all parameters into
`ECONOMIC_AMOUNT`.

### 17A.2 Boundary operators (discrete numeric dimensions)

Boundary operators apply only to dimensions whose type is `ECONOMIC_AMOUNT` or
`INTEGER` (and only when the dimension declares them).

Because M01 authoritative amounts use integer minor-unit semantics (ADR 0001 /
EK-NUM), `ECONOMIC_AMOUNT` operators **MUST** operate on the declared exact
representation. **No floating-point approximation.**

For a discrete integer domain and threshold `x`:

```text
MIN          = declared domain lower bound (must be explicit)
MAX          = declared domain upper bound (must be explicit)
EXACT(x)     = x
JUST_BELOW(x) = x - 1   (one integer step / one minor unit for ECONOMIC_AMOUNT)
JUST_ABOVE(x) = x + 1   (one integer step / one minor unit for ECONOMIC_AMOUNT)
```

Checked bounds:

* If `JUST_BELOW(x)` overflows the representable integer minimum, or leaves the
  declared domain `[lo, hi]`, the operator yields an explicit deterministic
  `NON_APPLICABLE` for that candidate (or `INVALID_TRANSFORMATION_PARAMETER`
  if `x` itself was outside the parameter domain).
* Same for `JUST_ABOVE(x)` at the upper extreme.
* Do **not** saturate, wrap, or invent a nearest in-domain substitute.

Operators such as `JUST_BELOW` / `JUST_ABOVE` are **not** defined for
`IDENTIFIER`, `BOOLEAN`, `ENUM`, `INDEX`, or `ORDERING_SELECTION` unless a
transformation explicitly declares a separate, inspectable candidate set for
those types.

### 17A.3 Multi-dimension candidates

* Parameter lists are ordered by the declared dimension order of the
  `ParameterDomain`.
* Cartesian combinations are formed in **lexicographic order** over declared
  parameter dimensions (dimension 0 is the major key).
* Each dimension’s candidate values are ordered per §17A.4 before the
  Cartesian product is taken.

### 17A.4 Candidate value ordering

| Type | Default ordering |
| --- | --- |
| `INTEGER` / `ECONOMIC_AMOUNT` | Ascending numeric order |
| `INDEX` | Ascending non-negative order |
| `IDENTIFIER` / `ENUM` | Declared source-collection order, or lexicographic exact-string order if the domain declares a string set |
| `BOOLEAN` | `false` then `true` unless declared otherwise |
| `ORDERING_SELECTION` | Declared permutation enumeration order |

A transformation may explicitly declare another valid deterministic ordering.

For multiple dimensions:

```text
lexicographic order using declared dimension order
```

If generation uses a finite source collection, that collection’s ordering
**MUST** itself be declared (for example: Scenario action vector order, or an
explicit sorted id list). No HashMap/HashSet/filesystem order.

### 17A.5 CandidateLimit

`maximum_parameter_candidates` (§18) bounds the number of **parameter tuples
evaluated**. Behavior on breach follows §18.

---

## 18. Enumeration and Explosion Control (R-08 / R-10 / R-11)

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

### 18.0 Counting concepts (R-10)

```text
TransformationApplication
    = one evaluation of a transformation against a snapshot

CompositionIntermediate
    = internal ScenarioSnapshot in a composition chain (not auto-emitted)

GeneratedAdversarialScenario
    = candidate actually emitted as a generation artifact
```

Invariant:

```text
application count ≠ intermediate count ≠ emitted scenario count
```

No implementation may infer one counter from another.

Example:

```text
Base
  ↓ T0   → CompositionIntermediate
  ↓ T1   → CompositionIntermediate
  ↓ T2   → GeneratedAdversarialScenario (emitted)
```

```text
transformation applications = 3
generated scenarios         = 1
```

unless the generation plan **explicitly** declares intermediate artifacts as
emitted candidates.

Independent branches that each emit:

```text
T0 candidate A → emitted
T0 candidate B → emitted
```

```text
generated scenarios = 2
```

Composition branches are counted by **final emitted** candidates, not by
intermediate snapshots.

### 18.1 What is counted

| Limit | Counter | Increments when |
| --- | --- | --- |
| `maximum_generated_scenarios` | emitted generated scenario count | A `GeneratedAdversarialScenario` is **emitted** (`DERIVED` VALID or INVALID). Composition intermediates do **not** increment. `NON_APPLICABLE` and `ERROR` do **not** increment |
| `maximum_transformations_per_plan` | plan transformation entries | Count of transformation entries declared in the plan (static). Exceeding at plan validation → Phase 2 `INVALID_ADVERSARIAL_CONFIGURATION` |
| `maximum_composition_depth` | composition depth | Number of transformations applied in one composition chain (`k` in `T_0…T_(k-1)`). Checked before application; depth `k > N` → limit breach per §18.2 |
| `maximum_action_mutations` | action mutation count | Each successful action insert/delete/replace/duplicate/reorder that modifies the action sequence in a `DERIVED` TransformationApplication (including on intermediates). `NON_APPLICABLE` does not increment |
| `maximum_parameter_candidates` | parameter candidate count | Each parameter tuple **evaluated** (including those that yield `NON_APPLICABLE` or `DERIVED`). Evaluation attempt increments before classification |

**Transformation evaluation count** (informational / for provenance): number of
`TransformationApplication`s attempted. Distinct from emitted scenario count.

### 18.2 GenerationStatus and limit breach (R-11)

```text
GenerationStatus =
    COMPLETE
    TRUNCATED
    FAILED
```

```text
A maximum N means at most N counted units (inclusive).
```

**Normative default TruncationPolicy:** `FAIL_ON_EXCEED`.

#### FAIL_ON_EXCEED

When the next counted unit would be `N+1`:

1. candidates successfully **emitted** before the breach remain observable
2. the `N+1` candidate is **NOT** emitted as a generated scenario
3. the generation operation becomes `FAILED`
4. the limit breach is recorded as `GENERATION_ERROR` (Phase 7)
5. no partially constructed `N+1` scenario is emitted
6. provenance/evidence records the exact breached counter and limit
7. already completed/emitted candidates are **not** retroactively invalidated

Example (`maximum_generated_scenarios = 3`):

```text
candidate 1 → emitted
candidate 2 → emitted
candidate 3 → emitted
candidate 4 → attempted → GENERATION_ERROR

GenerationStatus = FAILED
emitted candidates = 1, 2, 3
candidate 4 = not emitted
```

#### TRUNCATE_AT_N

When explicitly declared:

```text
GenerationStatus = TRUNCATED
```

The generator stops before producing/evaluating additional counted units for
that counter. No `GENERATION_ERROR` is raised solely because the declared
truncation boundary was reached. Truncation **MUST** be explicit in
provenance/evidence.

#### Composition atomicity

A composition chain **MUST NOT** expose a partially constructed final
candidate as valid emitted output.

If a limit is breached during `T0 → T1 → T2` before final emission:

```text
the incomplete composition branch is NOT emitted
```

Previously completed **independent** emitted candidates remain observable
under the GenerationStatus contract above.

#### Status ≠ candidate validity

Do **not** collapse:

```text
FAILED
TRUNCATED
COMPLETE
```

into candidate `VALID`/`INVALID`.

A generation operation may be:

```text
FAILED + previously emitted DERIVED+VALID candidates
```

without making those candidates invalid.

### 18.3 Non-applicable vs counters

| Outcome | emitted scenario count | parameter candidate count | action mutation count | transformation applications |
| --- | --- | --- | --- | --- |
| `DERIVED` (emitted) | +1 | +1 (if driven by a parameter tuple) | +1 per mutating action op in that application | +1 |
| `DERIVED` (composition intermediate only) | no | +1 (if parameter-driven) | +1 per mutating action op | +1 |
| `NON_APPLICABLE` | no | +1 (if a parameter tuple was evaluated) | no | +1 |
| `ERROR` | no | +1 if evaluation began; then status per §18.2 | no | +1 if begun |

### 18.4 Ordering

Ordering of enumerated **emitted** candidates **MUST** follow §29 / §17A.

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

Every emitted GeneratedAdversarialScenario **MUST** preserve provenance
sufficient to reconstruct:

```text
base scenario identity / version
composition intermediates (identity/version markers per step; not auto-emitted)
transformation identity / version
transformation parameters (typed ParameterValues)
target resolution method (ActionId vs INDEX) + match count N
IncompatibilityRule identity when INCOMPATIBLE_TRANSFORMATION
left_plan_index / right_plan_index of the winning occurrence pair
StructuralInvalidReason when DERIVED+INVALID (incl. 4a/4b distinction)
IdentifierToken equality inputs where relevant
ApplicabilityPolicy
candidate generation ordering (plan + dimension + value order)
composition position
generation counters / limits
TruncationPolicy / GenerationStatus (COMPLETE | TRUNCATED | FAILED)
breached counter identity when FAILED/TRUNCATED
resulting ValidationClassification (VALID | INVALID) + invalid reason if any
identity-stability inputs (§6.3) — algorithm remains AD-03
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
result sequence (and provenance classifications), including
`GenerationStatus` and limit-boundary behavior.

Identity **string** byte-equality across implementations that choose different
AD-03 algorithms is **not** required (§6.3). Identity **stability** within a
declared identity/versioning contract **is** required.

### 21.2 Semantic equivalence (minimum)

```text
same ordered emitted candidate sequence
same transformation application order
same parameter candidate order
same NON_APPLICABLE classifications and positions
same GenerationStatus (COMPLETE | TRUNCATED | FAILED)
same error class and phase when generation fails
same limit boundary behavior (emitted vs non-emitted)
same composition target resolution outcomes
same distinction of intermediates vs emitted scenarios
same derived Scenario fields that affect M03 execution
same adversarial category/intent labels
same IncompatibilityRule hits with same left_plan_index / right_plan_index
same ValidationClassification (+ StructuralInvalidReason when INVALID)
same ActionId resolution match counts / ambiguity classifications
same provenance semantic fields listed in §20
```

Non-requirements:

* identical memory layout
* identical internal intermediate objects
* identical host paths / wall-clock stamps
* byte-identical identity strings across distinct AD-03 algorithms

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
| Scenario acceptance / fatal Scenario errors | M03 |
| Adversarial derivation of Scenario inputs | M04 |
| Pre-execution structural representability checks | M04 closed checklist (§8.5.1) |

M04 **MUST NOT** duplicate M03 simulation semantics.

`DERIVED + VALID` means the closed M04 checklist passed only. M03 may still
reject or fail the Scenario per its frozen contract.

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
What transformation applications were attempted?
Which steps were CompositionIntermediates vs emitted?
Why was each application applicable / non-applicable?
What typed parameters were used?
What GeneratedAdversarialScenario (if any) was emitted?
Was M04 structural validation VALID or INVALID (and why)?
What was GenerationStatus (COMPLETE | TRUNCATED | FAILED)?
Was any limit breached (which counter / N)?
Was the scenario executed via M03?
Which M03 run / SimulationResult reference applies?
Which M01 outcomes resulted (if executed)?
Which M02 evaluations resulted (if executed)?
```

Lifecycle labels (distinct):

```text
GENERATED          (emitted)
REJECTED
NON_APPLICABLE
INVALID            (DERIVED + INVALID)
EXECUTED
TRUNCATED          (GenerationStatus)
FAILED             (GenerationStatus with GENERATION_ERROR or other M04 error)
COMPLETE           (GenerationStatus)
```

Do **not** fabricate downstream M01/M02/M03 evidence when execution did not
occur. Do **not** treat composition intermediates as emitted unless the plan
explicitly emits them.

---

## 28. Adversarial Result Model

Layers **MUST** remain distinct:

```text
1. GenerationStatus (COMPLETE | TRUNCATED | FAILED)
2. Generation result (emitted candidates + NonApplicable/Error records)
3. M04 structural ValidationClassification per emitted candidate
4. Derived / emitted scenario (if any)
5. Execution reference / SimulationResult (if executed via M03)
6. Downstream M02 evaluations (if any)
7. M04 errors (if any)
```

**Never** collapse into a single PASS/FAIL.

In particular:

```text
adversarial generation COMPLETE
    ≠ M04 structural VALID
    ≠ M03 execution success
    ≠ economic system failed / succeeded

FAILED generation + prior emitted VALID candidates
    ≠ those candidates become INVALID
```

---

## 29. Deterministic Ordering (R-07; closes AD-07)

Normative generation ordering hierarchy:

```text
1. transformation plan vector order
2. transformation candidate order (per transformation’s declared candidate set,
   or single application)
3. parameter-dimension order (ParameterDomain declaration order)
4. candidate value order (§17A.4; typed defaults)
5. composition order (left-to-right plan indices 0 .. k-1)
```

The **emitted** GeneratedAdversarialScenario sequence **MUST** be reproducible
under this hierarchy. CompositionIntermediates are ordered by composition
position but are not automatically part of the emitted sequence (§18).

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

See §6.3 (R-13):

```text
identity stability = NORMATIVE
identity derivation algorithm = AD-03 OPEN
```

Also related: **AD-02** (transformation identity algorithm).

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
| AD-03 | Scenario identity **algorithm** (hash/format/declared vs content-derived) | OPEN — identity **stability** is normative (§6.3 R-13) |
| AD-04 | Formal precondition language for applicability | OPEN |
| AD-05 | Duplicate transformation behavior in composition | **CLOSED_FOR_GATE-4_SEMANTICS** — explicit plan repeats allowed; no silent dedupe (§10.2) |
| AD-06 | Deduplication / semantic equivalence | OPEN (default: no silent dedupe) |
| AD-07 | Candidate / generation ordering | **CLOSED_FOR_GATE-4_SEMANTICS** — hierarchy in §29 / §17A |
| AD-08 | Parameter candidate generation + ParameterValue types | **CLOSED_FOR_GATE-4_SEMANTICS** — §17A (R-03/R-09) |
| AD-09 | Generation limit boundary + emission vs intermediate + GenerationStatus | **CLOSED_FOR_GATE-4_SEMANTICS** — §18 (R-08/R-10/R-11); default numeric values remain **IMPLEMENTATION_DETAIL_OPEN** |
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
invalid parameter type / domain error
ECONOMIC_AMOUNT vs INTEGER separation (INTEGER is not Money)
composition order determinism
composition intermediates not counted as emitted scenarios
independent branches count as separate emitted scenarios
FAIL_ON_EXCEED preserves prior emitted candidates + FAILED status
TRUNCATE_AT_N yields TRUNCATED without GENERATION_ERROR
incomplete composition branch not emitted on mid-chain breach
DERIVED+VALID means closed checklist passed; does not imply M03/M01 success
DERIVED+INVALID has StructuralInvalidReason and is not silently repaired
ambiguous ActionId resolution is ERROR (never silent first-match)
declared IncompatibilityRules with occurrence pairs / EXACT_PAIR / self-rule ≥2 occurrences
check 4a world before 4b initial-state; IdentifierToken length≥1 exact equality
invariant entries require id + definition_version (non-evaluating)
generation limits
deterministic candidate ordering by ParameterValue type
provenance chain completeness (incl. GenerationStatus, resolution match counts)
valid derived scenario executes via M03
M04 does not mutate M01 state independently
M02 not evaluated by M04
identity stability under identical inputs (algorithm may differ)
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

Generation limits are explicit and deterministic; emitted count ≠ application
count ≠ intermediate count; GenerationStatus is distinct from candidate
validity.

### M04-INV-10

No hidden environment state participates in authoritative generation semantics.

### M04-INV-11

M04 structural VALID ≠ M03 acceptance ≠ M01 economic success.

### M04-INV-12

Identity stability is normative; identity derivation algorithm remains AD-03.

---

## 41. Document Control

| Item | Value |
| --- | --- |
| Created by | TASK-10 |
| Remediation | **TASK-10R**…**TASK-10RRR**; **TASK-10RRRR** (R-19 occurrence; R-20 structural minima — closes R2-B-01 / R2-B-02) |
| Status | **READY_FOR_REVIEW** |
| Normative freeze | **NOT FROZEN** |
| Implementation authorization | **NONE** (blocked) |
| Gate | **GATE 4 — OPEN** |
| Code / crates modified by this task | **None** |
| Frozen specifications modified | **None** |
| Next | Independent freeze re-audit (**TASK-10F-R3**) after authorization |

### Amendment rule

Material semantic change after freeze requires explicit specification
amendment. Until freeze, review comments may produce further remediation
without treating this draft as implementation authority.
