# AivoGuard Deterministic Simulator Specification

| Field | Value |
| --- | --- |
| Document | `docs/design/deterministic-simulator-specification.md` |
| Task | **TASK-07** / **TASK-07R** |
| Module | **M03 — Deterministic Economic Simulator** |
| Gate | **GATE 3 — OPEN** |
| **STATUS** | **READY_FOR_REVIEW** |
| Normative freeze | **NOT FROZEN** |
| Implementation | **BLOCKED** — this document does **not** authorize implementation |
| Depends on | Product Scope (**FROZEN**); Domain Contract (**FROZEN**, Gate 0 **CLOSED**); Economic Kernel Specification (**FROZEN**, Gate 1 **CLOSED**); Economic Invariant Engine Specification (**FROZEN**, Gate 2 **CLOSED**); M01 implementation (TASK-04 **PASS**); M02 implementation (TASK-06 / TASK-06R **PASS**) |
| Remediation | TASK-07R — AFTER_ACTION order, History mapping, M01 Error order, completion after Fatal, execution-limit boundary |

```text
STATUS: READY_FOR_REVIEW
NORMATIVE FREEZE: NOT FROZEN
IMPLEMENTATION: BLOCKED
GATE: GATE 3 — OPEN
```

This document is the **reviewable normative candidate** for Gate-3 M03. It is
**not frozen**. Material freeze requires an explicit freeze/remediation task.
Implementation requires a later explicit authorization task after freeze.

Related:

* [`product-scope.md`](product-scope.md) (**FROZEN**)
* [`domain-contract.md`](domain-contract.md) (**FROZEN**)
* [`economic-kernel-specification.md`](economic-kernel-specification.md) (**FROZEN**)
* [`economic-invariant-engine-specification.md`](economic-invariant-engine-specification.md) (**FROZEN**)
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
M03 Deterministic Simulator Specification (this document; READY_FOR_REVIEW)
        ↓
Accepted ADRs
        ↓
Implementation (blocked)
```

M03 **MUST NOT** silently modify any frozen specification above.

If an implementability contradiction with a frozen specification is discovered:

```text
STOP → REPORT → do not patch around it in code or by silent amendment
```

### 1.2 Canonical authority assignment

```text
M01 Economic Kernel        = authoritative economic truth
M02 Invariant Engine       = authoritative invariant evaluation
M03 Simulator              = deterministic orchestration / simulation control
```

M03 is an orchestration layer. M03 **MUST NOT** become a second economic engine.

### 1.3 Product role

> A deterministic economic simulation orchestrator that executes an explicit
> scenario by repeatedly invoking the authoritative M01 Economic Kernel,
> optionally evaluating declared M02 invariants, and producing reproducible
> simulation results and evidence.

---

## 2. Purpose

M03 defines how AivoGuard executes a **finite, ordered** sequence of economic
Actions against an explicit initial `EconomicWorld` and `EconomicState` by
invoking M01 for every economically authoritative transition, optionally
invoking M02 at declared evaluation points, and emitting a reproducible
`SimulationResult` with first-class evidence lineage.

M03 answers:

```text
Given these explicit inputs, what sequence of authoritative M01 outcomes
and optional M02 evaluations occurs, and how did the simulation terminate?
```

M03 does **not** answer (those belong elsewhere):

```text
Did the observed result match a declared regression expectation?  → M06
How should adversarial actions be generated?                      → M04
How should chaos faults be injected?                              → M05
How do external payment/x402 systems map in?                      → M07/M08
How do multi-agent worlds compose?                                → M09
```

---

## 3. Scope

### 3.1 In scope (Gate 3 semantics)

* Scenario as explicit reproducible simulation input
* Deterministic sequential action execution via M01
* State progression derived solely from M01 outcomes
* ExecutionContext construction for M01 (deterministic)
* Termination (normal / early / fatal)
* Explicit execution limits
* Optional M02 evaluation scheduling and recording
* Simulation evidence lineage and replay identity
* Simulation-level error taxonomy (distinct from M01/M02)
* Conceptual SimulationResult contents

### 3.2 Out of scope (explicit)

* M03 Rust modules / crate layout
* Public Rust API, CLI, HTTP, GraphQL, SDK, bindings
* Scenario / result serialization formats
* Persistence / database
* RNG library selection
* Parallel / concurrent execution model
* Adversarial generation (M04)
* Chaos injection (M05)
* Regression expectation language (M06)
* Payment / x402 / blockchain / exchange adapters (M07/M08)
* Multi-agent world semantics (M09)
* DSL / parser for scenarios (related Domain Contract OD-04 / product OD-07)
* Amending frozen M01 or M02 semantics

### 3.3 Normative reuse (do not redefine)

M03 **reuses** without reinterpretation:

| Concept | Owner |
| --- | --- |
| `EconomicWorld`, `EconomicState`, `Action`, `Money`, `Asset`, `Account`, `Actor`, `Price`, facets | Domain Contract / M01 |
| `ExecutionContext`, `KernelOutcome`, `Transaction`, `StateEffect`, `Evidence` (kernel) | M01 |
| Dispositions `AcceptedEffective`, `Rejected`, `FailedEconomic` | M01 |
| Non-economic classes `InvalidInput`, `ConfigurationError`, `EngineError` | M01 |
| Zero-effect `AcceptedEffective` | M01 |
| Atomicity / partiality | M01 / World |
| `Invariant`, `EvaluationTarget`, `InvariantOutcome` (`PASS`/`FAIL`/`ERROR`) | M02 |
| M02 eight ERROR classes and precedence | M02 |
| Dimensionless `Count` vs `Money` | M02 (TASK-06R) |
| Numeric: no `f32`/`f64` for authoritative economic truth | Product Scope / M01 / ADR 0001 |
| Deterministic keyed collections | ADR 0002 |

---

## 4. Definitions

### Simulation

A single deterministic run of a Scenario under this specification: orchestration
that invokes M01 for each executed Action and optionally M02 at declared points.

### Scenario

An explicit, versionable, inspectable, reproducible declaration of everything
required to run a Simulation (World, initial state, actions, configuration,
optional invariant plan, deterministic metadata). See §8.

### Simulation Step

One ordinal position in the simulation’s authoritative action sequence,
identified by a simulation-owned **step index** (`0 .. n-1`). Distinct from any
optional Action identifier owned by M01/domain.

### Initial State

The explicit `EconomicState` declared by the Scenario as `State_0`. Must be
declared, inspectable, versionable, and reproducible (Domain Contract DC-R05).

### Current State

The simulation’s authoritative `EconomicState` reference after the last
completed M01 step (or `State_0` before any Action). Current State is
simulation control state pointing at M01-produced authoritative state — not an
independent economic authority.

### Action Sequence

A finite, ordered list `Action[0], Action[1], …, Action[n-1]`. Order is
authoritative. Host/filesystem/database/network enumeration order **MUST NOT**
influence sequence order.

### Step Result

The authoritative record of one processed simulation step, including step
index, Action, M01 `KernelOutcome`, derived state before/after (when present),
and any M02 evaluations scheduled for that step.

### Simulation Result

The complete outcome artifact of a Simulation: termination reason, identity /
configuration, executed steps, M01 outcomes, state progression facts, optional
M02 results, simulation-level errors, and replay identity. See §19.
**Not** a regression expectation verdict (M06).

### Termination

The deterministic end of a Simulation run under one of: Normal Completion,
Early Termination, or Fatal Termination (§12).

### Normal Completion

All declared Actions in the Scenario’s Action Sequence were processed according
to declared stop policy, and no fatal simulation/engine condition terminated
the run early. Zero-action Scenarios complete normally with `State_0` only.

`ON_SIMULATION_COMPLETION` evaluations **are eligible** after Normal
Completion (§12, §14).

### Early Termination

The Scenario’s **declared** stop condition became true after a processed step
(or before remaining actions), and the Simulation stops without processing
further Actions. Prior completed steps remain valid evidence.

`ON_SIMULATION_COMPLETION` evaluations **are eligible** after Early
Termination (§12, §14).

### Fatal Termination

A non-recoverable simulation-level failure or declared fatal M01 non-economic
error prevents further execution. Prior completed steps remain valid evidence.
No silent skip, retry, or repair.

`ON_SIMULATION_COMPLETION` **MUST NOT** run after Fatal Termination (§12, §14).

### Replay

Re-execution of a Simulation using the same explicit inputs and engine
versions, producing semantically equivalent authoritative outputs (§17).

### Determinism

For identical explicit simulation inputs and engine versions, M03 produces
semantically equivalent authoritative Simulation Results (§16).

### Execution Limit

An explicit, declared upper bound on how many Actions may be executed in a run
(at minimum `maximum_action_steps`). Exceeding it is a deterministic
simulation-level failure — not wall-clock/CPU/memory timeout semantics (§13).

### Optional Invariant Evaluation

Declared M02 evaluations at explicit simulation points. M03 records M02
results; M03 does not reinterpret or mutate economics based on them (§14).

### Simulation Evidence

The deterministic lineage from Scenario → Step → Action → M01 outcome →
optional M02 evaluations → SimulationResult (§15). Distinct from LLM
explanation.

### Simulation Error

A failure of M03 orchestration / Scenario validity / execution limits, in the
M03 error taxonomy (§18). Distinct from M01 economic dispositions, M01
non-economic errors, and M02 `PASS`/`FAIL`/`ERROR`.

---

## 5. Architectural Role

```text
Interfaces / CLI / adapters (future)
          ↓
M03 Simulator (orchestration)
          ↓
M02 Invariant Engine (optional, read-only)
          ↓
M01 Economic Kernel (required, authoritative)
          ↓
Domain primitives
```

### M03 may

* accept an explicit Scenario
* hold simulation control state (step index, current state reference,
  termination status, limits, evaluation schedule)
* construct deterministic M01 `ExecutionContext`
* invoke M01 for each executed Action
* capture and order M01 outcomes / evidence
* optionally invoke M02 with compatible `EvaluationTarget`s
* produce SimulationResult + evidence lineage
* enforce declared execution limits and stop conditions

### M03 MUST NOT

* calculate balances, fees, prices, settlement, or conservation independently
* reimplement transfer/conversion/fee semantics
* mutate `EconomicState` except by adopting M01’s authoritative
  `state_after` / non-mutation outcome
* determine invariant `PASS`/`FAIL`/`ERROR` (M02 does)
* repair failed economic outcomes
* invent missing World/State/Action data
* use wall-clock time, host env, network, database, or filesystem as
  authoritative inputs
* use unseeded randomness or LLM output as economic / simulation truth
* convert M02 `FAIL`/`ERROR` into M01 economic failure
* convert M01 economic outcomes into M02 results
* define regression expectation matching (M06)
* generate adversarial/chaos inputs (M04/M05)
* own external adapters (M07/M08)

---

## 6. Canonical Simulation Model

```text
VALIDATE SCENARIO
        ↓
State_0
        ↓
for each Action[i] (while not terminated):
        ↓
CHECK EXECUTION LIMIT  (executed_action_count vs maximum_action_steps)
        ↓
BEFORE_ACTION M02 (if declared; State_i only)
        ↓
M01(Action_i, State_i)
        ↓
┌──────────────────────────────────────┐
│ Economic Outcome                     │
│ adopt authoritative State_(i+1)      │
│ AFTER_ACTION M02 (post-M01 view)     │
│ record StepResult                    │
│ evaluate stop policy                 │
└──────────────────────────────────────┘
        OR
┌──────────────────────────────────────┐
│ M01 Non-Economic Error               │
│ preserve State_i                     │
│ no transition / no fabricated tx/ev  │
│ record error evidence                │
│ Fatal Termination by default         │
└──────────────────────────────────────┘
        ↓
if Normal Completion OR Early Termination:
    ON_SIMULATION_COMPLETION M02 (if declared)
        ↓
SimulationResult
```

**Normative rule:** every economically authoritative state transition in a
Simulation is produced by M01. M03 orchestrates; M03 does not reinterpret what
a transition means.

The authoritative per-step algorithm is §10. If any other section appears to
conflict with §10, **§10 wins** for execution ordering.

---

## 7. Simulation Inputs

All authoritative simulation inputs are **explicit**. No hidden host/process
configuration participates in simulation semantics.

### 7.1 EconomicWorld

An explicit `EconomicWorld`.

M03 **MUST NOT** infer or construct hidden economic rules from the host
environment. World identity/versionability follows Domain Contract.

During a single Simulation run, the effective World is the Scenario’s declared
World (Domain Contract: one effective World per run unless a future amended
World-mutation semantic is specified). This Gate-3 specification does **not**
authorize mid-run World mutation.

### 7.2 Initial EconomicState

An explicit `EconomicState` = `State_0`.

Required properties (DC-R05):

```text
explicit
inspectable
versionable
reproducible
part of simulation evidence / reproducibility identity
```

**Prohibited:** silently deriving initial state from wall clock, host
environment, process state, global mutable state, undeclared external APIs,
hidden randomness, or machine-local configuration.

**No implicit empty/default economic state** unless the Scenario **explicitly**
declares that the initial state is the empty/default `EconomicState` value as
a chosen input (declaration is still required; silence is not).

### 7.3 Action Sequence

Explicitly ordered finite sequence:

```text
Action[0], Action[1], ..., Action[n-1]
```

* Order is authoritative.
* Length may be zero (valid Scenario).
* Filesystem / database / network / map-iteration order **MUST NOT** determine
  Action order.
* Optional Action identifiers (M01) are **not** required for sequencing.
  Simulation step index is authoritative for order (§21).

### 7.4 Execution Configuration

Declared configuration required by M03 semantics, at minimum:

```text
maximum_action_steps          (Execution Limit; §13)
stop_policy / stop_conditions (optional; §12)
invariant_evaluation_plan     (optional; §14)
execution_configuration_id    (declared label included in replay identity)
M01 engine version identity   (for determinism / evidence)
M02 engine version identity   (when M02 is used)
optional declared logical-time inputs for ExecutionContext (§11)
optional declared seed field  (see §20; Gate 3 default: unused / absent)
```

No hidden host/process configuration.

### 7.5 Optional Invariants

M03 may receive declared M02 `Invariant`s plus an evaluation plan (§14).

M03 **MUST NOT** alter M02 invariant semantics, ERROR taxonomy, precedence,
applicability, or target compatibility.

---

## 8. Scenario

### 8.1 Conceptual structure

```text
Scenario
├── scenario identity / version
├── EconomicWorld
├── initial EconomicState
├── ordered Actions
├── execution configuration (§7.4)
├── optional invariant evaluation plan (§14)
└── deterministic execution metadata
    (engine version pins, configuration id, optional seed field, etc.)
```

### 8.2 Required properties

```text
reproducible
explicit
versionable
inspectable
serializable in principle
```

### 8.3 Explicitly not frozen

* Serialization format (JSON/YAML/TOML/binary/…) — **OPEN** (`M03-OD-02`)
* Public CLI / API for authoring Scenarios — **OPEN** (`M03-OD-04`, `M03-OD-05`)
* Scenario composition / macros / includes — **OPEN** (`M03-OD-10`)
* Expectation / assertion language — **out of M03**; belongs to M06 / DC-13

### 8.4 Invalid Scenario

A Scenario that omits required inputs, contradicts declared configuration
(e.g. negative limits), or is otherwise malformed for M03 yields
`InvalidScenario` / `MissingSimulationInput` / `InvalidSimulationConfiguration`
**before** any Action is executed (§18). No partial “best effort” run.

---

## 9. State Progression

### 9.1 Normative progression

```text
State_0 = explicit initial EconomicState

for each Action[i] that actually reaches M01 (see §10, §13):
    Kernel(Action_i, State_i, World, ExecutionContext_i)
            ↓
    KernelOutcome_i
            ↓
    State_(i+1)   determined strictly per §9.2
```

An Action prevented by the execution limit (§13) **never** reaches M01 and
therefore does **not** produce `State_(i+1)`.

### 9.2 Next-state derivation (authoritative)

| M01 outcome | Next Current State |
| --- | --- |
| `KernelOutcome::Economic` with any disposition | Adopt M01’s authoritative `state_after` as `State_(i+1)` **before** `AFTER_ACTION` M02 (§10) |
| `KernelOutcome::Error` (`InvalidInput` / `ConfigurationError` / `EngineError`) | **No** authoritative `State_(i+1)`; Current State remains `State_i`; no fabricated transition (§10.4) |

M03 **MUST NOT**:

* manually mutate balances
* reconstruct or invent economic effects
* apply a subset of M01 effects
* merge independent M01 transitions
* rollback an authoritative M01 transition
* fabricate `state_after` for an M01 Error

### 9.3 Disposition preservation

M03 **MUST** preserve M01 dispositions exactly:

```text
AcceptedEffective
Rejected
FailedEconomic
```

Forbidden conversions include (non-exhaustive):

```text
Rejected            → AcceptedEffective / Success
FailedEconomic      → AcceptedEffective / Success
EngineError         → FailedEconomic / Rejected
AcceptedEffective   → FailedEconomic merely because effects.length == 0
```

### 9.4 Zero-effect AcceptedEffective

```text
AcceptedEffective + 0 state effects
```

is a **valid successful** M01 economic outcome (M01 §12.2).

M03 **MUST NOT** treat zero effects as failure, rejection, or error merely
because numeric balances did not change. `State_(i+1)` equals `State_i` when
M01 reports that non-mutation. That equal state is still the authoritative
post-M01 state for `AFTER_ACTION`.

### 9.5 Atomicity / partiality inheritance

M03 inherits M01/World atomicity and explicit partiality semantics.

M03 **MUST NOT** split an atomic kernel transition, partially apply effects, or
invent partial execution. If M01 records rejection-side effects under an
explicit World rule, M03 preserves that exact effect set.

---

## 10. Normative Execution Algorithm

This section is the **authoritative Gate-3 step algorithm**. Implementers
**MUST** follow this ordering.

### 10.0 Simulation prologue

```text
1. VALIDATE SCENARIO
   - missing/invalid inputs → simulation error; no Actions executed
   - invalid maximum_action_steps (e.g. negative) →
     InvalidSimulationConfiguration; no Actions executed
2. State_0 ← explicit initial EconomicState
3. executed_action_count ← 0
4. Current State ← State_0
5. termination ← none
```

### 10.1 Per-Action loop (authoritative order)

For each Action index `i` in Scenario order, while not terminated:

```text
1. Resolve State_i = Current State.

2. CHECK EXECUTION LIMIT (§13):
   if executed_action_count >= maximum_action_steps:
       Fatal Termination
       cause = ExecutionLimitExceeded
       Action[i] is NOT_ATTEMPTED (not passed to M01)
       BEFORE_ACTION MUST NOT run for Action[i]
       AFTER_ACTION MUST NOT run for Action[i]
       stop the loop
   else continue.

3. BEFORE_ACTION M02 (if declared for this step) (§14.3):
   - observes State_i only
   - MUST NOT mutate State_i / World / Action
   - MUST NOT alter M01 inputs
   - record PASS/FAIL/ERROR exactly as M02 produced
   - default: continue unless Scenario stop condition says otherwise
   - if a declared stop condition terminates here: Early Termination;
     do NOT invoke M01 for Action[i]; mark Action[i] NOT_ATTEMPTED

4. Construct deterministic ExecutionContext_i (§11).

5. Invoke M01(World, State_i, Action_i, ExecutionContext_i).
   executed_action_count ← executed_action_count + 1

6. Capture complete KernelOutcome_i.

7. Branch on KernelOutcome (§10.2 Economic / §10.4 Error).
```

### 10.2 M01 Economic Outcome path

When `KernelOutcome::Economic`:

```text
7a. Derive/adopt State_(i+1) strictly from M01 `state_after` (§9.2).
    Current State ← State_(i+1).

7b. Construct the authoritative post-action evaluation view from M01 only:
    State Before = State_i
    State After  = State_(i+1)   (= M01 state_after)
    Transaction / Effects / Events = exactly as M01 produced
    (including AcceptedEffective with zero effects)

7c. Run declared AFTER_ACTION M02 evaluations (§14.4):
    AFTER_ACTION M02 evaluation sees the authoritative post-M01 state
    when such a state exists.
    Eligible targets include State(State_(i+1)) and/or Transition(...)
    and/or History projections (§14.6) as declared.

7d. Record the complete Step Result (§15):
    classification = ATTEMPTED_M01_ECONOMIC_OUTCOME

7e. Evaluate termination / stop policy (§12):
    - if declared Early stop → Early Termination; break
    - else continue to Action_(i+1) if any remain
```

### 10.3 Normative AFTER_ACTION timing rule

```text
AFTER_ACTION runs only after M01 has been invoked AND, for Economic outcomes,
after State_(i+1) has been adopted from M01.

AFTER_ACTION MUST NOT run before State_(i+1) adoption on an Economic outcome.
```

Therefore the previously ambiguous order “record → M02 → derive state” is
**rejected**. The normative order is:

```text
M01 → adopt State_(i+1) → AFTER_ACTION → record Step Result → stop policy
```

### 10.4 M01 Non-Economic Error path

When `KernelOutcome::Error` (`InvalidInput` | `ConfigurationError` |
`EngineError`):

```text
M01 Error
    ↓
capture complete M01 error outcome
    ↓
State remains State_i
    ↓
no authoritative State_(i+1)
    ↓
no fabricated Transition
    ↓
no fabricated Transaction / Event
    ↓
AFTER_ACTION evaluations: NOT executed (§14.4)
    ↓
record Step/Error evidence
    classification = ATTEMPTED_M01_ERROR
    ↓
apply M03 fatal policy (§18.3)
    ↓
Fatal Termination by default
```

M03 **MUST NOT**:

* retry
* skip silently
* repair
* convert the error to an economic disposition
* fabricate a state transition
* fabricate a transaction
* fabricate an event

### 10.5 Epilogue (completion phase)

```text
if Normal Completion OR Early Termination:
    run ON_SIMULATION_COMPLETION M02 if declared (§14.5)
    finalize SimulationResult
else if Fatal Termination:
    ON_SIMULATION_COMPLETION MUST NOT run
    completion_evaluation_records = empty
    finalize SimulationResult with fatal cause
```

### 10.6 Zero-action Scenario

If the Action Sequence length is 0 and Scenario validation succeeds:

```text
Normal Completion
→ ON_SIMULATION_COMPLETION eligible (if declared)
→ no M01 invocations
→ executed_action_count = 0
```

### 10.7 Authority reminder

```text
no Action that advances economic truth may bypass M01
silent skip / silent retry / automatic repair = prohibited
```

---

## 11. Execution Context

M03 constructs M01 `ExecutionContext` for each Action that reaches M01
(§10.1 step 4–5).

### 11.1 Deterministic construction

At minimum, for step index `i`:

```text
logical_order        ← derived from simulation step position / declared plan
                       (e.g. step i → logical_order = i, or an explicit
                        Scenario-declared mapping — must be deterministic)
configuration_id     ← Scenario execution_configuration_id
declared_unix_secs   ← only if Scenario explicitly provides a declared
                       logical timestamp for the step; NEVER host clock
```

Exact field packing remains an implementation decision (`M03-OD-01`) but must
satisfy M01’s ExecutionContext constraints.

### 11.2 Prohibited authoritative sources

```text
wall clock / local timezone
OS environment variables
process ID / thread scheduling
filesystem state / enumeration order
network state
database state
unseeded randomness
LLM output
undeclared global mutable state
```

### 11.3 Real-time simulation

This Gate-3 specification does **not** introduce real-time / wall-clock
simulation. Logical step ordering is sufficient. Any future real-time model
requires explicit amendment and must remain deterministic.

---

## 12. Termination

### 12.1 Normal Completion

Occurs when:

* the Action Sequence is fully processed under the declared stop policy, and
* no fatal condition terminated the run.

Includes the zero-action case (§10.6).

**Completion phase:**

```text
Normal Completion
→ ON_SIMULATION_COMPLETION evaluations are eligible (§14.5)
```

### 12.2 Early Termination

Occurs only when a **Scenario-declared** stop condition is satisfied.

Examples of **declarable** stop conditions (conceptual; not a frozen DSL):

```text
stop after step k
stop after first Rejected disposition
stop after first FailedEconomic disposition
stop after first M02 FAIL (record only; does not rewrite M01)
stop after BEFORE_ACTION M02 FAIL/ERROR (Action not executed)
```

Early Termination:

* **MUST NOT** invent undeclared stop reasons
* **MUST** leave prior Step Results intact as evidence
* **MUST NOT** execute remaining Actions after the stop decision

Default when no stop condition is declared:

```text
continue through all Actions unless a fatal condition occurs
```

In particular, `Rejected` / `FailedEconomic` / M02 `FAIL` do **not** by
themselves terminate the Simulation unless the Scenario explicitly declares
that policy.

**Completion phase:**

```text
Early Termination
→ ON_SIMULATION_COMPLETION evaluations are eligible (§14.5)
```

The evaluation sees the authoritative current/final state and available
history **at the moment termination occurs**.

### 12.3 Fatal Termination

Occurs when further execution is impossible or prohibited under this
specification, including:

* M03 simulation errors classified as fatal (§18)
* M01 `KernelOutcome::Error` under the fatal policy in §18.3
* Execution Limit exceeded (§13)

Fatal Termination:

* **MUST NOT** silently recover, skip, or retry
* **MUST** preserve prior completed Step Results as valid evidence
* **MUST** record the fatal cause in SimulationResult

**Completion phase (normative):**

```text
Fatal Termination
→ NO ON_SIMULATION_COMPLETION evaluation
```

```text
ON_SIMULATION_COMPLETION is a completion-phase evaluation and is not
automatically executed after Fatal Termination.
```

Reason: a fatal M01/M03 error means the simulation did not reach a valid
completion checkpoint. Partial state **MUST NOT** be described as a completed
simulation state for completion-phase evaluation.

Diagnostic evaluation of partial history after Fatal Termination is a **future
explicit capability** and is **NOT** part of Gate 3.

### 12.4 No silent skip / retry

```text
silent skip of Actions          = prohibited
silent retry of M01             = prohibited
automatic economic repair       = prohibited
```

---

## 13. Execution Limits

### 13.1 Required concepts

```text
maximum_action_steps     — declared Scenario/configuration bound
executed_action_count    — number of Actions actually invoked through M01
```

`maximum_action_steps` is part of reproducibility identity.

### 13.2 Normative boundary check

**Before** invoking M01 for Action[i] (and **before** `BEFORE_ACTION` for that
Action — see §10.1):

```text
if executed_action_count >= maximum_action_steps:
    Fatal Termination
    cause = ExecutionLimitExceeded
    Action[i] is NOT passed to M01
    Action[i] classification = NOT_ATTEMPTED
    BEFORE_ACTION MUST NOT be invoked for that Action
    AFTER_ACTION MUST NOT be invoked for that Action
```

Therefore:

```text
maximum_action_steps = 0
→ no Action may be passed to M01
→ first Action attempt produces ExecutionLimitExceeded
```

Example:

```text
maximum_action_steps = 2

Action[0] → M01 executes   (executed_action_count becomes 1)
Action[1] → M01 executes   (executed_action_count becomes 2)
Action[2] → NOT executed
           → ExecutionLimitExceeded
```

### 13.3 What the limit counts

The limit counts **Actions actually invoked through M01**.

It does **not** count:

```text
M02 evaluations
simulation metadata
NOT_ATTEMPTED Actions (limit / early-stop before M01)
hypothetical future Actions
```

### 13.4 Preflight vs runtime

Do **NOT** silently truncate a Scenario whose Action Sequence length `n` is
greater than `maximum_action_steps`.

Normative behavior:

```text
n > maximum_action_steps
→ execute at most maximum_action_steps Actions through M01
→ when the next Action would exceed the limit:
     Fatal Termination
     ExecutionLimitExceeded
     do not invoke M01 for that Action
```

This preserves the explicit fact that the Scenario requested more execution
than permitted.

Configuration validity:

```text
maximum_action_steps < 0  (or otherwise illegal)
→ InvalidSimulationConfiguration before any Action
```

`maximum_action_steps = 0` is **legal** and means “no M01 invocations
permitted”; the first Action attempt yields `ExecutionLimitExceeded`.

### 13.5 Non-semantic (prohibited as M03 semantics)

```text
CPU time budgets
wall-clock timeouts
memory pressure
host process timeouts
```

may exist as operational harness concerns outside M03 normative semantics.
They **MUST NOT** be used as substitutes for `maximum_action_steps` in the
economic simulation contract.

---

## 14. M02 Integration

### 14.1 Optionality

M02 evaluation is **optional**.

A Simulation may produce a complete authoritative M01 sequence with **zero**
M02 evaluations.

When requested, M03 invokes M02 as a **read-only** consumer of authoritative
M01 outputs / World / State / History views.

### 14.2 Evaluation points (summary)

| Point | Eligibility | Observes |
| --- | --- | --- |
| `BEFORE_ACTION` | Action eligible under limit; after validation; before M01 | `State_i` only |
| `AFTER_ACTION` | Only after M01 invoked | Post-M01 authoritative view when Economic; see §14.4 for Error |
| `ON_SIMULATION_COMPLETION` | Only after Normal Completion **or** Early Termination | Final Current State and/or economic History view at termination |

### 14.3 BEFORE_ACTION semantics

```text
BEFORE_ACTION
```

is evaluated **only** for an Action that is actually eligible to execute under
the execution limit (§13) and after Scenario validation (§10.0).

It observes:

```text
State_i
```

and **MUST NOT** mutate it.

If BEFORE_ACTION M02 returns `PASS` / `FAIL` / `ERROR`, default behavior:

```text
record result
continue
```

unless an explicit Scenario stop condition says otherwise.

A BEFORE_ACTION M02 result **MUST NOT** alter the Action or M01 input.

If the limit prevents the Action from executing:

```text
Action is not executed
BEFORE_ACTION must NOT be invoked for that unexecuted Action
```

This avoids creating M02 evidence for a step that never executes.

### 14.4 AFTER_ACTION semantics

```text
AFTER_ACTION
```

runs **only** after M01 has been invoked (§10).

#### Economic outcome

```text
M01 KernelOutcome::Economic
→ State_(i+1) exists (adopted from M01 state_after)
→ AFTER_ACTION may inspect authoritative post-action State / Transition /
  History projections
```

**Normative statement:**

```text
AFTER_ACTION M02 evaluation sees the authoritative post-M01 state
when such a state exists.
```

#### M01 Error

```text
M01 KernelOutcome::Error
→ no State_(i+1)
→ no authoritative transition
→ no fabricated AFTER_ACTION Transition
→ no fabricated Transaction / Event
```

Gate-3 default for **all** declared `AFTER_ACTION` evaluations on an M01 Error
step:

```text
NOT_EXECUTED / NOT_APPLICABLE at the M03 scheduling layer
(explicit reason: no authoritative post-action economic result)
```

Rationale: `AFTER_ACTION` is defined as a post-M01 **economic** evaluation.
`State_i` was already available to `BEFORE_ACTION`. Gate-3 does **not** require
re-invoking State evaluation over unchanged `State_i` after an M01 Error
before Fatal Termination.

If a declared AFTER_ACTION evaluation requires a target that is unavailable
(e.g. Transition when no Economic outcome exists):

**Default Gate-3 policy (resolved; not open):**

```text
record evaluation as NOT_EXECUTED / NOT_APPLICABLE at the M03 scheduling layer
with an explicit reason
```

It **MUST NOT** be represented as:

```text
M02 PASS
M02 FAIL
```

unless M02 was actually invoked and produced that result.

Do **not** create a fake M02 `ERROR` merely because the M01 step failed.
(If an implementation *does* invoke M02 with an incompatible target contrary to
this default, M02’s own `ERROR(INCOMPATIBLE_TARGET)` remains valid — but
Gate-3 **default** is to **not invoke** such evaluations.)

### 14.5 ON_SIMULATION_COMPLETION semantics

```text
ON_SIMULATION_COMPLETION
```

runs **only** after:

```text
Normal Completion
OR
Early Termination
```

It does **NOT** run after:

```text
Fatal Termination
```

```text
ON_SIMULATION_COMPLETION is a completion-phase evaluation and is not
automatically executed after Fatal Termination.
```

Completion-phase evaluation:

* **MUST NOT** mutate state
* observes authoritative current/final state and available **economic**
  History view at termination
* results are recorded in `completion_evaluation_records`, separate from
  execution status

### 14.6 History: M03 Step History vs M02 Economic History View

These are **not** synonyms.

#### Simulation Step Record (conceptual)

For every attempted or limit-blocked Action position that M03 accounts for:

```text
Simulation Step
├── step index
├── Action
├── attempt classification (§15.2)
├── M01 KernelOutcome (when M01 invoked)
├── State Before (when defined for the attempt)
├── State After, when authoritative (Economic only)
├── Transaction, when produced by M01
├── Effects, when produced by M01
├── Events, when produced by M01
└── M02 evaluations actually executed (or NOT_EXECUTED records)
```

#### M03 Step History (orchestration history)

Contains every accounted simulation step, including:

```text
ATTEMPTED_M01_ECONOMIC_OUTCOME records
ATTEMPTED_M01_ERROR records
NOT_ATTEMPTED records (e.g. ExecutionLimitExceeded / early-stop before M01)
simulation metadata (termination, limits, scheduling notes)
```

#### M02 Authoritative Economic History View

Contains **only** authoritative economic records produced by M01, appropriate
to the requested M02 History target:

```text
M01 EconomicOutcome
    → eligible for corresponding authoritative economic history records

M01 Error
    → retained as M03 step/error evidence
    → NOT fabricated into Transaction / Event / Transition
    → NOT included as an economic History record
```

Do **NOT** automatically define an M01 Error as an economic transition.

#### History ordering

```text
History ordering = simulation step order / authoritative logical order
```

No hidden sorting. No wall-clock ordering. No database ordering. No filesystem
ordering.

When M02 requires a History target, M03 supplies this **economic history
projection**. M03 orchestration metadata remains separate from M02 economic
truth.

### 14.7 Non-reinterpretation

M03 **MUST** record actually-executed M02 outcomes exactly:

```text
PASS
FAIL
ERROR
```

including applicability status and Violations as produced by M02.

Forbidden:

```text
M02 FAIL  → rewrite as M01 FailedEconomic / Rejected
M02 ERROR → rewrite as M01 EngineError / economic failure
M02 PASS  → invent economic state mutation
M02 result → mutate EconomicWorld / EconomicState / Transaction / Effects
NOT_EXECUTED scheduling record → fake M02 PASS/FAIL
```

### 14.8 Termination interaction with M02

By default:

```text
M02 FAIL  ≠ simulation fatal termination
M02 ERROR ≠ automatic economic failure
```

A Scenario **may** declare an early-stop condition keyed to M02 `FAIL` or
`ERROR`. That is orchestration policy, not reinterpretation of M02 truth.

---

## 15. Evidence

### 15.1 Lineage

```text
Scenario
  ↓
Step (step index)
  ↓
Action
  ↓
attempt classification
  ↓
M01 KernelOutcome when invoked (+ kernel Evidence)
  ↓
Authoritative transition / non-mutation facts when Economic
  ↓
M02 evaluations actually executed (or NOT_EXECUTED records)
  ↓
termination decision
  ↓
SimulationResult
```

### 15.2 Attempt classification (mandatory)

Every Action position that participates in the run’s accounting has an
explicit classification:

```text
NOT_ATTEMPTED
ATTEMPTED_M01_ECONOMIC_OUTCOME
ATTEMPTED_M01_ERROR
```

| Classification | Meaning |
| --- | --- |
| `NOT_ATTEMPTED` | Action not passed to M01 (execution limit; or Early stop before M01, e.g. AFTER BEFORE_ACTION stop) |
| `ATTEMPTED_M01_ECONOMIC_OUTCOME` | M01 returned `KernelOutcome::Economic` |
| `ATTEMPTED_M01_ERROR` | M01 returned `KernelOutcome::Error` |

Do **not** invent a Transition for `ATTEMPTED_M01_ERROR`.

### 15.3 Per-step reconstructability

Evidence **MUST** show, as applicable:

```text
State Before
Action
M01 result (when invoked)
State After if authoritative
M02 evaluations actually executed
termination decision affecting the step / run
```

For an Action prevented by execution limit:

```text
Action remains unexecuted
No M01 outcome
No BEFORE_ACTION M02
No AFTER_ACTION M02
ExecutionLimitExceeded becomes simulation-level termination cause
classification = NOT_ATTEMPTED
```

### 15.4 Snapshots (conceptual)

```text
Initial State
State Before Action
KernelOutcome (when M01 invoked)
State After Action   (when M01 Economic outcome exists)
```

Serialization of snapshots is **not** frozen (`M03-OD-03`).

### 15.5 No alternative economic truth

Simulation metadata (step index, termination reason, limits, NOT_EXECUTED
scheduling notes) is **not** economic authority. M03 **MUST NOT** invent
economically authoritative Transactions or Events for convenience when M01
produced none.

---

## 16. Determinism

### 16.1 Equivalence inputs

Identical:

```text
Scenario (identity/version + all declared fields)
EconomicWorld
Initial EconomicState
Action Sequence (order + contents)
Execution configuration (including maximum_action_steps and stop policy)
M01 engine version
M02 engine version / invariant definitions / evaluation plan (when used)
explicit seed field (if present; §20)
```

⇒ M03 **MUST** produce **semantically equivalent** authoritative simulation
output.

### 16.2 Authoritative equivalence (minimum)

Semantic equivalence includes at least:

```text
termination class and cause classification
executed step indices and Actions
M01 dispositions / error classes and economically material outcome fields
authoritative state progression (State_0 … final state)
M02 PASS/FAIL/ERROR (+ applicability / violations) when evaluated
simulation-level error classes when present
```

Non-authoritative differences (host paths, log colors, wall-clock capture
timestamps outside declared logical time) **MUST NOT** break determinism
claims unless a Scenario explicitly includes them in comparison scope
(comparison/canonicalization details remain open; Domain Contract DC-12).

### 16.3 Prohibited dependencies

```text
wall clock
host environment
network
database
filesystem ordering
thread scheduling
unseeded RNG
LLM output
global mutable economic / simulation-control state across runs
```

### 16.4 Concurrency

Gate-3 execution is **deterministic sequential**. Parallel execution is
**OPEN** (`M03-OD-09`) and not authorized here.

---

## 17. Replay

### 17.1 Definition

Replay = re-run using the same explicit simulation inputs and engine versions.

Replay **MUST** yield semantically equivalent results under §16.

### 17.2 Independence

Replay **MUST NOT** depend on:

```text
current time
current environment
external live state
changing network data
mutable database state
```

### 17.3 Persistence

Persistence / storage format for Scenario and SimulationResult is **not**
frozen (`M03-OD-07`). Replay is defined over explicit inputs, not over a
particular store.

---

## 18. Error Semantics

### 18.1 Category separation (mandatory)

Keep distinct:

```text
(1) M01 economic dispositions
      AcceptedEffective | Rejected | FailedEconomic

(2) M01 non-economic failures
      InvalidInput | ConfigurationError | EngineError

(3) M02 invariant results
      PASS | FAIL | ERROR   (+ M02 ERROR classes)

(4) M03 simulation-level failures
      (this section)
```

**No silent conversion across categories.**

### 18.2 M03 simulation-level error classes

Minimum Gate-3 classes:

| Class | Meaning |
| --- | --- |
| `InvalidScenario` | Scenario structure/content is not a valid Simulation input |
| `MissingSimulationInput` | Required Scenario input absent (World, initial state, configuration field, etc.) |
| `InvalidSimulationConfiguration` | Declared configuration is inconsistent/illegal (e.g. invalid limit) |
| `ExecutionLimitExceeded` | Executing another Action would exceed `maximum_action_steps` |
| `SimulationEngineError` | M03 itself cannot correctly continue (orchestration bug / internal violation) |

These are **not** M01 dispositions and **not** M02 results.

### 18.3 Default stop policy for M01 outcomes

| Condition | Default M03 behavior |
| --- | --- |
| `AcceptedEffective` (incl. zero effects) | Adopt `state_after`; AFTER_ACTION eligible; record; continue (unless stop) |
| `Rejected` | Adopt `state_after`; AFTER_ACTION eligible; record; continue (unless stop) |
| `FailedEconomic` | Adopt `state_after`; AFTER_ACTION eligible; record; continue (unless stop) |
| M01 `InvalidInput` | Record as `ATTEMPTED_M01_ERROR`; no State_(i+1); **Fatal Termination** (default); no ON_SIMULATION_COMPLETION |
| M01 `ConfigurationError` | Record as `ATTEMPTED_M01_ERROR`; no State_(i+1); **Fatal Termination** (default); no ON_SIMULATION_COMPLETION |
| M01 `EngineError` | Record as `ATTEMPTED_M01_ERROR`; no State_(i+1); **Fatal Termination** (default); no ON_SIMULATION_COMPLETION |
| M02 `PASS` / `FAIL` / `ERROR` (actually invoked) | Record; continue (unless Scenario stop condition) |
| M03 `ExecutionLimitExceeded` | Action `NOT_ATTEMPTED`; **Fatal Termination**; no ON_SIMULATION_COMPLETION |
| M03 simulation error (other fatal classes) | **Fatal Termination**; no ON_SIMULATION_COMPLETION |

Rationale: M01 non-economic errors mean the kernel could not correctly complete
evaluation; continuing would invent economics. Scenario authors may declare
**additional** early-stop conditions for economic dispositions / M02 results;
they may **not** declare a policy that converts M01 `EngineError` into
`AcceptedEffective`.

### 18.4 Prior evidence remains valid

On Early or Fatal Termination, all previously completed Step Results remain
valid evidence. M03 **MUST NOT** erase or rewrite prior authoritative M01
outcomes.

---

## 19. Simulation Result

### 19.1 Purpose

`SimulationResult` is the authoritative **simulation outcome summary** for a
run under this specification.

It is **not** the M06 regression verdict and **not** a replacement for M01 or
M02 truth.

### 19.2 Domain Contract alignment (DC-R04 / DC-13)

Domain Contract requires conceptual `PASS` / `FAIL` / `ERROR` status classes
for SimulationResult and forbids collapsing layers. Exact overall composition
remains open (`DC-13`).

**Gate-3 rule:** M03 **MUST** always expose distinct layers:

```text
execution_status
  NormalCompletion | EarlyTermination | FatalTermination

step_records[]
  (attempt classification + M01 outcomes + state facts)

invariant_evaluation_records[]
  (per-step BEFORE_ACTION / AFTER_ACTION results actually produced by M02,
   plus M03 NOT_EXECUTED scheduling records where applicable)

completion_evaluation_records[]
  (ON_SIMULATION_COMPLETION results; empty after Fatal Termination)

simulation_errors[]
  (M03 classes when any)
```

For Fatal Termination:

```text
completion_evaluation_records = empty
```

unless a future explicit semantic extension says otherwise.

For Normal/Early Completion, completion-phase M02 records may exist if
declared and applicable.

M03 **MUST NOT** invent a single collapsed “test PASS/FAIL” that hides whether
failure was economic, invariant, expectation, or execution. Overall
PASS/FAIL/ERROR composition for harnesses remains **OPEN** pending DC-13 /
M06. Implementers may surface `execution_status` without claiming a final
Domain Contract SimulationResult enum freeze.

### 19.3 Required answerability

A SimulationResult **MUST** contain enough information to answer:

1. Did the simulation complete (Normal / Early / Fatal)?
2. Why did it stop?
3. What Scenario / configuration was executed?
4. What initial state was used?
5. Which Actions were executed?
6. Which steps completed?
7. What M01 outcomes occurred?
8. What state transitions / non-mutations occurred?
9. Which M02 evaluations occurred (if any)?
10. Which simulation-level failures occurred (if any)?
11. Can the execution be deterministically reproduced (identity inputs / versions)?

### 19.4 Not frozen

Concrete Rust structs and serialization — **OPEN** (`M03-OD-01`, `M03-OD-03`).

---

## 20. Randomness

### 20.1 Gate-3 default

M03 is deterministic **without** randomness.

Do **not** introduce an RNG merely for architectural preparation.

Product Scope / Domain Contract mention seed as part of reproducibility
identity. Gate-3 interpretation:

```text
seed field may appear in Scenario identity as absent / unused
```

Absence of RNG consumption is itself deterministic.

### 20.2 Future randomness (not authorized now)

If future simulation randomness is required, a specification amendment **MUST**
define before implementation:

```text
explicit seed
+
declared random source
+
defined consumption semantics
+
replay guarantee
```

RNG library selection remains **OPEN** (`M03-OD-08`). Do not freeze a crate here.

---

## 21. Action Identity and Step Identity

```text
simulation step identity  = M03-owned ordinal index (0-based)
Action identity           = optional M01/domain Action identifier
```

* Sequencing **MUST** work when Action IDs are absent.
* Duplicate Action IDs (if present) **MUST NOT** reorder steps; step index wins.
* Evidence references both when Action ID is present.

---

## 22. Transaction / Event Handling

* M03 **consumes** M01 authoritative `Transaction` / `EconomicEvent` /
  kernel `Evidence`.
* M03 **MUST NOT** manufacture economically authoritative Transactions or
  Events.
* Effect cardinality `0`, `1`, or many is preserved exactly.
* Simulation metadata (step index, termination) is distinct from economic
  events.

---

## 23. Authority Boundaries

### 23.1 Ownership

| Concern | Owner |
| --- | --- |
| Economic truth / transitions / dispositions / effects | M01 |
| Invariant PASS/FAIL/ERROR | M02 |
| Simulation orchestration / step order / limits / termination | M03 |
| Regression expectations | M06 (future) |
| Adversarial generation | M04 (future) |
| Chaos injection | M05 (future) |
| External adapters | M07/M08 (future) |
| Multi-agent composition | M09 (future) |

### 23.2 Forbidden M03 authorities

```text
M03 balance arithmetic
M03 fee calculation
M03 price calculation
M03 settlement implementation
M03 transaction reconstruction
M03 invariant reimplementation
M03 economic repair
M03 hidden state authority
```

### 23.3 Numeric authority

M03 inherits M01/M02 numeric semantics. M03 **MUST NOT** introduce `f32`/`f64`
or floating-point economic calculations. M03 does not reinterpret `Money`,
`Asset`, `Count`, `Price`, or rounding.

### 23.4 LLM boundary

An LLM may later generate candidate Scenarios/Actions or explain evidence.
LLM output **MUST NEVER** directly determine:

```text
authoritative EconomicState
M01 economic result
M02 PASS/FAIL/ERROR
simulation truth / termination truth
```

---

## 24. Foundational M03 Invariants

These are specification-level properties (not code yet):

### M03-INV-01

Every executed Action is passed through M01.

### M03-INV-02

M03 cannot independently modify authoritative economic state.

### M03-INV-03

The next authoritative state is derived from the M01 result (§9.2).

### M03-INV-04

Simulation Action ordering is deterministic and Scenario-declared.

### M03-INV-05

Replay of identical explicit inputs is semantically equivalent (§16–§17).

### M03-INV-06

M02 results cannot mutate economic state.

### M03-INV-07

Simulation errors cannot be silently converted into economic outcomes.

### M03-INV-08

Economic outcomes cannot be silently converted into invariant results.

### M03-INV-09

Execution limits are explicit and deterministic (§13).

### M03-INV-10

No hidden environment state participates in authoritative simulation semantics.

### M03-INV-11

An `AFTER_ACTION` M02 evaluation over State/Transition observes only the
authoritative result produced by M01 for that step. M03 **MUST NOT** fabricate
`state_after`, Transition, Transaction, or Event for an M01 Error.

### M03-INV-12

`ON_SIMULATION_COMPLETION` runs only after Normal Completion or Early
Termination — never after Fatal Termination.

### M03-INV-13

An Action prevented by `executed_action_count >= maximum_action_steps` is not
passed to M01 and does not receive `BEFORE_ACTION` or `AFTER_ACTION`
evaluation.

---

## 25. No Hidden Mutable State

M03 **MUST NOT** contain hidden economically meaningful mutable state.

Allowed explicit simulation control state examples:

```text
current step index
current state reference / value (M01-authored)
termination status
declared execution limits
evaluation schedule / plan cursor
accumulated step records
```

These are orchestration state, not a competing economic authority.

---

## 26. Testing Requirements (for future TASK-08+)

This task does **not** implement tests. Future implementation **MUST** cover at
least:

```text
single action
multiple sequential actions
zero-action scenario
explicit initial state
state progression across steps
Rejected action preserved
FailedEconomic action preserved
AcceptedEffective with zero effects preserved
M01 EngineError → fatal (default); no ON_SIMULATION_COMPLETION
M01 ConfigurationError → fatal (default); no ON_SIMULATION_COMPLETION
M01 InvalidInput → fatal (default); no ON_SIMULATION_COMPLETION
simulation-level InvalidScenario / MissingSimulationInput
Early Termination via declared stop condition → ON_SIMULATION_COMPLETION eligible
ExecutionLimitExceeded before M01 (Action NOT_ATTEMPTED; no BEFORE/AFTER M02)
AFTER_ACTION observes State_(i+1) after Economic outcomes
AFTER_ACTION Transition NOT_EXECUTED on M01 Error (not fake M02 PASS/FAIL)
M02 economic History excludes M01 Error steps
M02 PASS recorded without mutating state
M02 FAIL recorded without converting to M01 failure
M02 ERROR recorded without converting to economic failure
replay determinism (identical inputs → equivalent results)
action ordering independence from Action IDs
missing/invalid scenario input rejection
no hidden environment dependency
read-only M02 integration
evidence lineage reconstructability + attempt classification
World not mutated by M03 independently of M01
```

---

## 27. Open Decisions

| ID | Topic | Status |
| --- | --- | --- |
| M03-OD-01 | Concrete Rust Simulation / Scenario / Step Result types | OPEN |
| M03-OD-02 | Scenario serialization format | OPEN |
| M03-OD-03 | SimulationResult serialization format | OPEN |
| M03-OD-04 | Public API surface | OPEN |
| M03-OD-05 | CLI | OPEN |
| M03-OD-06 | Invariant evaluation scheduling **API** only (Error-step Transition / NOT_EXECUTED policy is **RESOLVED** in §14.4) | OPEN (API); semantic policy RESOLVED |
| M03-OD-07 | Persistence | OPEN |
| M03-OD-08 | Future randomness model / RNG source | OPEN |
| M03-OD-09 | Future parallel execution | OPEN |
| M03-OD-10 | Scenario composition | OPEN |
| DC-13 (upstream) | Exact SimulationResult PASS/FAIL/ERROR composition across layers | OPEN (Domain Contract) |
| OD-05 (upstream) | Domain Contract “simulation execution model” — this document is the Gate-3 candidate closure | READY_FOR_REVIEW via this spec |

Do not resolve these merely for convenience.

---

## 28. Gate 3 Readiness Criterion

> Two competent engineers reading only the frozen M03 specification and the
> existing frozen M01/M02 specifications should be able to implement M03 without
> materially disagreeing about state progression, action ordering, M01
> authority, termination semantics, error boundaries, deterministic replay, or
> M02 integration.

If important semantics remain ambiguous after review/remediation, the document
is **NOT** ready to freeze.

This TASK-07 document is **READY_FOR_REVIEW**, not frozen.

---

## 29. Document Control

| Item | Value |
| --- | --- |
| Created by | TASK-07 |
| Remediation | **TASK-07R** (AFTER_ACTION order; History mapping; M01 Error order; completion after Fatal; execution-limit boundary) |
| Status | **READY_FOR_REVIEW** |
| Normative freeze | **NOT FROZEN** |
| Implementation authorization | **NONE** (blocked) |
| Gate | **GATE 3 — OPEN** |
| Code / crates modified by this task | **None** |
| Frozen specifications modified | **None** |
| Next | Independent freeze audit → TASK-07F (or further remediation if needed) |

### Amendment rule

Material semantic change after freeze requires explicit specification
amendment. Until freeze, review comments may produce TASK-07R remediation
without treating this draft as implementation authority.
