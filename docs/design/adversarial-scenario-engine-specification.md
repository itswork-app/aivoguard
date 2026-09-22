# AivoGuard Adversarial Scenario Engine Specification

| Field | Value |
| --- | --- |
| Document | `docs/design/adversarial-scenario-engine-specification.md` |
| Task | **TASK-10** |
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

This document is the **reviewable normative candidate** for Gate-4 M04. It is
**not frozen**. Material freeze requires an explicit freeze/remediation task.
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

A transformation whose declared preconditions hold for the given base.

### Non-Applicable Transformation

A transformation whose preconditions do not hold; recorded explicitly, never
silently skipped as success.

### Valid Adversarial Scenario

A derived scenario accepted by Domain/M03 contracts and executable via M03.

### Invalid Adversarial Scenario

A candidate that violates declared scenario/domain requirements; not silently
“fixed” into validity.

### M04 Engine Failure

A failure of M04 itself (distinct from M01/M02/M03 errors).

### Provenance

The reconstructible derivation chain from base → transformations → derived
scenario (§20).

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

### 8.1 Conceptual equation

```text
BaseScenario
    +
AdversarialTransformation
    =
DerivedAdversarialScenario
```

### 8.2 Required declaration fields

Each transformation **MUST** declare:

```text
transformation identity / version
target (§13)
parameters
preconditions
transformation semantics
applicability rules
resulting scenario projection
provenance contribution
```

### 8.3 Application outcomes

Applying a transformation yields exactly one of:

| Outcome | Meaning |
| --- | --- |
| `Derived` | Preconditions hold; Valid or Invalid candidate produced (validation separate) |
| `NonApplicable` | Preconditions fail; no silent skip-as-success |
| `M04 Error` | Definition/configuration/parameter/composition/engine failure |

Do **not** silently skip invalid transformations.

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

Examples:

* low / zero / near-limit balances
* competing balances
* precondition boundary states

M04 may construct **declared Scenario initial states** only within Domain /
World contracts. It **MUST NOT** bypass M01 to mutate state during execution.

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

applied left-to-right (or an explicitly declared direction that must be
deterministic and documented in the transformation plan).

**Gate-4 default:** apply in declared vector order index `0 .. k-1`.

### 10.2 Required composition rules

| Concern | Rule |
| --- | --- |
| Ordering | Declared order is authoritative |
| Associativity | **Not** assumed; only explicit ordered plans are normative |
| Duplicates | Allowed only if the plan explicitly repeats; otherwise **AD-05** |
| Conflicts | Incompatible transformations → `INCOMPATIBLE_TRANSFORMATION` |
| Depth | Bounded by `maximum_composition_depth` (§18) |
| Identity | Derived scenario identity/provenance includes full ordered chain |
| Explosion | Bounded by generation limits (§18) |

Example:

```text
Base
  -> duplicate action
  -> boundary amount
  -> reordered action
```

must produce a deterministic derived scenario identity and provenance.

No implicit composition outside a declared plan.

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
| Initial EconomicState | Yes | Declared Scenario input only; Domain/World valid |
| ExecutionContext fields carried by Scenario | Yes | Never host clock; only declared logical inputs |
| Execution configuration (limits, config id) | Yes | Invalid configs → Invalid / M04 error |
| Invariant evaluation plan | Yes | Declares plan; does not evaluate |
| Stop policy | Yes | Declares policy; does not redefine M03 stop semantics |

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
| Valid adversarial scenario | Accepted by Domain/M03 contract; executable via M03 |
| Invalid adversarial scenario | Violates declared requirements; not silently repaired |
| Non-applicable transformation | Preconditions unmet |
| M04 engine failure | M04 cannot correctly complete generation |

These **MUST NOT** be conflated.

---

## 15. Error Model

### 15.1 M04 error classes (Gate-4 minimum)

| Class | Meaning |
| --- | --- |
| `INVALID_ADVERSARIAL_DEFINITION` | Transformation/plan definition malformed |
| `INVALID_ADVERSARIAL_CONFIGURATION` | Generator/limits/config illegal |
| `INVALID_BASE_SCENARIO` | Base Scenario missing/invalid for M04 |
| `NON_APPLICABLE_TRANSFORMATION` | Preconditions fail (may also be a non-error NonApplicable outcome; see §8.3) |
| `INCOMPATIBLE_TRANSFORMATION` | Composition/conflict between transformations |
| `INVALID_TRANSFORMATION_PARAMETER` | Parameter out of declared domain |
| `COMPOSITION_ERROR` | Composition plan cannot be applied correctly |
| `GENERATION_ERROR` | Enumeration/generation cannot complete under declared rules |
| `ENGINE_ERROR` | Internal M04 fault |

`NON_APPLICABLE_TRANSFORMATION` as an **error class** applies when a plan
**requires** applicability and it fails; otherwise NonApplicable is a normal
application outcome. Implementations must not collapse these without an
explicit plan flag (**AD-16** refinement at freeze).

### 15.2 Downstream ownership preserved

| Failure | Owner |
| --- | --- |
| M01 dispositions / KernelError | M01 |
| M02 PASS/FAIL/ERROR | M02 |
| M03 simulation errors / fatal / limits | M03 |
| Adversarial generation/validation | M04 |

### 15.3 Precedence (M04-local)

When multiple M04 errors arise in one generation unit, report the
**first** discovered under deterministic scan order:

```text
INVALID_ADVERSARIAL_DEFINITION
→ INVALID_ADVERSARIAL_CONFIGURATION
→ INVALID_BASE_SCENARIO
→ INVALID_TRANSFORMATION_PARAMETER
→ NON_APPLICABLE_TRANSFORMATION (when required)
→ INCOMPATIBLE_TRANSFORMATION
→ COMPOSITION_ERROR
→ GENERATION_ERROR
→ ENGINE_ERROR
```

---

## 16. Determinism

Given identical:

```text
Base Scenario
World / configuration embedded therein
Transformation definition(s)
Transformation parameters
Generator configuration (limits, ordering keys)
M04 engine version
optional declared seed field (if future randomness authorized)
```

M04 **MUST** produce semantically equivalent adversarial output.

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

## 18. Enumeration and Explosion Control

Required explicit configuration concepts:

```text
maximum_generated_scenarios
maximum_transformations_per_plan
maximum_composition_depth
maximum_action_mutations
maximum_parameter_candidates
```

Exceeding a limit yields deterministic `GENERATION_ERROR` (or plan rejection as
`INVALID_ADVERSARIAL_CONFIGURATION` if limits themselves are illegal).

Ordering of enumerated candidates **MUST** be deterministic (§29).

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
transformation identity / version
transformation parameters
composition order
generator configuration
M04 engine version
optional seed field (if present)
parent scenario identity
full derivation chain
```

Provenance is first-class evidence, not optional logging.

---

## 21. Reproducibility

### 21.1 Contract

Given identical declared inputs and M04 engine version, another conforming
implementation **MUST** reproduce the same **semantic** adversarial scenario
set (and provenance classifications).

### 21.2 Semantic equivalence (minimum)

```text
same derived Scenario fields that affect M03 execution
same adversarial category/intent labels
same transformation chain + parameters
same validity class
same M04 error class when generation fails
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

## 29. Deterministic Ordering

Where multiple transformations/scenarios are generated, order **MUST** be
explicit (declared plan order, then deterministic secondary keys such as
transformation id, parameter tuple order).

Never depend on:

```text
HashMap / HashSet iteration
filesystem order
thread completion order
network order
```

If ordering cannot be defined for a generation plan:

```text
→ INVALID_ADVERSARIAL_CONFIGURATION / GENERATION_ERROR
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
| AD-05 | Duplicate transformation handling in composition | OPEN |
| AD-06 | Deduplication / semantic equivalence | OPEN (default: no silent dedupe) |
| AD-07 | Secondary ordering keys beyond plan order | OPEN |
| AD-08 | Parameter generation domains / operators | OPEN |
| AD-09 | Default numeric values for generation limits | OPEN |
| AD-10 | Concrete provenance schema | OPEN |
| AD-11 | Formal reproducibility equality operators | OPEN |
| AD-12 | Shrinking / minimization | **DEFERRED** |
| AD-13 | Randomness policy / RNG source | OPEN (default: none) |
| AD-14 | Serialization format | OPEN |
| AD-15 | Concrete M04 result Rust/API types | OPEN |
| AD-16 | NonApplicable vs error-class refinement | OPEN |
| AD-17 | LLM proposal API surface | OPEN |
| AD-18 | Versioning scheme details | OPEN |
| AD-19 | Generation idempotency / duplicate emission policy | OPEN |
| AD-20 | CLI/API boundary | OPEN |

Do not resolve these merely for convenience.

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
| Status | **READY_FOR_REVIEW** |
| Normative freeze | **NOT FROZEN** |
| Implementation authorization | **NONE** (blocked) |
| Gate | **GATE 4 — OPEN** |
| Code / crates modified by this task | **None** |
| Frozen specifications modified | **None** |
| Next | Independent review → TASK-10R (if remediation) or TASK-10F (freeze) |

### Amendment rule

Material semantic change after freeze requires explicit specification
amendment. Until freeze, review comments may produce TASK-10R remediation
without treating this draft as implementation authority.
