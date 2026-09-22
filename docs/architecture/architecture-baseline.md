# AivoGuard Architecture Baseline

| Field | Value |
| --- | --- |
| Document | `docs/architecture/architecture-baseline.md` |
| Task | TASK-00B |
| Status | **BASELINE** (architecture documentation; not a normative economic spec) |
| Authority | Subordinate to frozen design specifications and accepted ADRs |
| Implementation | Documents current + target structure only — does **not** authorize M02+ |

This document maps frozen product/domain/economic specifications into repository
architecture and module boundaries. It does **not** invent economic semantics,
freeze open DSL/API/serialization decisions, or authorize implementation.

Related:

* [`../design/product-scope.md`](../design/product-scope.md) (**FROZEN**)
* [`../design/domain-contract.md`](../design/domain-contract.md) (**FROZEN**)
* [`../design/economic-kernel-specification.md`](../design/economic-kernel-specification.md) (**FROZEN**)
* [`../design/economic-invariant-engine-specification.md`](../design/economic-invariant-engine-specification.md) (**READY_FOR_REVIEW**)
* [`../design/SPECIFICATION-POLICY.md`](../design/SPECIFICATION-POLICY.md)
* [`../adr/`](../adr/)

---

## 1. Repository audit (CURRENT)

### Workspace

| Item | Current state |
| --- | --- |
| Cargo workspace | Single member: `crates/aivoguard` |
| External crates | **None** (std only) |
| Network / DB / LLM deps | Absent |
| Examples | Empty placeholder |
| `docs/architecture/` | Was empty prior to this baseline |

### Implemented code

| Path | Role |
| --- | --- |
| `crates/aivoguard/src/lib.rs` | Crate root; re-exports M01 public surface |
| `crates/aivoguard/src/kernel/` | **M01 Economic Kernel** implementation |
| `crates/aivoguard/tests/m01_kernel.rs` | Integration tests for M01 |

M01 modules (internal): `account`, `action`, `asset`, `balance`, `effect`,
`engine`, `error`, `event`, `evidence`, `execution`, `money`, `outcome`,
`rounding`, `state`, `transaction`, `world`.

Public entry: `kernel::evaluate` → `KernelOutcome`.

### Public surface (CURRENT)

Re-exported types include domain/kernel concepts such as `EconomicWorld`,
`EconomicState`, `Action`, `ExecutionContext`, `Money`, `Asset`, `Transaction`,
`StateEffect`, `Evidence`, `KernelOutcome`, `KernelError`, etc.

This surface is the **current M01 implementation API**. It is not a frozen
product CLI/SDK/DSL contract.

### Architecture docs (CURRENT → after TASK-00B)

| Document | Role |
| --- | --- |
| This file | Canonical architecture baseline |
| ADR 0001 | `i128` amount representation (Accepted) |
| ADR 0002 | `BTreeMap`/`BTreeSet` ordering (Accepted) |

No competing architecture baselines exist.

---

## 2. Architecture principle

Canonical execution (product-aligned):

```text
EconomicWorld
      +
EconomicState
      +
Action
      +
ExecutionContext
      │
      ▼
┌──────────────────────────┐
│ M01 Economic Kernel       │
│ authoritative economics  │
└────────────┬─────────────┘
             │
             ▼
     Authoritative Result
     State / Transition /
     Transaction / Effects /
     Evidence
             │
             ▼
┌──────────────────────────┐
│ M02 Invariant Engine     │
│ read-only evaluation     │
└────────────┬─────────────┘
             │
             ▼
       PASS / FAIL / ERROR
```

Later modules (M03–M09) **compose around** these authorities; they must not
replace M01 economic truth or M02 invariant evaluation semantics.

---

## 3. Architectural layers

Conceptual layers (dependency flows **downward**):

```text
INTERFACES          CLI / SDK / HTTP / language bindings (future; not frozen)
ADAPTERS            M07 / M08 external translation (future)
ORCHESTRATION       M03 simulator, M09 multi-agent composition (future)
EVALUATION          M02 invariants; M06 regression comparison (future / not impl)
ECONOMIC AUTHORITY  M01 kernel (IMPLEMENTED)
DOMAIN              World / State / Action / Asset / Money primitives (in M01)
```

Orchestration depends on **M01** for economic truth. M02 is an **optional**
downstream consumer of authoritative M01 outputs (and of simulation results),
not a mandatory layer for every simulation run.
### CURRENT Rust layout

Prefer **one crate** with clear modules over premature multi-crate splits:

```text
crates/aivoguard/
  src/
    lib.rs
    kernel/     ← ECONOMIC AUTHORITY + DOMAIN (M01)
    # future seams (NOT created):
    # invariants/   ← M02
    # simulation/   ← M03
    # ...
```

Additional crates are authorized only when a frozen specification and explicit
implementation task require them — not for architectural aesthetics.

---

## 4. Authoritative economic core

```text
AUTHORITATIVE ECONOMIC CORE = M01 (kernel) + domain primitives it owns
```

External layers may: translate, configure, invoke, observe, serialize, display,
generate **candidate** inputs.

External layers must **not** override authoritative economic outcomes.

Forbidden authority inversions:

```text
CLI → directly mutates balance
LLM → determines invariant result
adapter → changes transaction result
database → becomes economic source of truth
simulator → independently reimplements M01 economics
```

---

## 5. Module boundaries

### M01 — Economic Kernel (IMPLEMENTED)

Owns:

* EconomicWorld, EconomicState, Action, ExecutionContext
* economic validation, authorization/eligibility, evaluation
* effects, transaction disposition, state transition
* foundational checks, kernel evidence

Does **not** own: invariant language, scenario orchestration, adversarial/
chaos generation, regression harness, payment/x402 adapters, multi-agent
orchestration.

### M02 — Economic Invariant Engine (SPEC READY_FOR_REVIEW; NOT IMPLEMENTED)

```text
READ-ONLY · DETERMINISTIC · NON-MUTATING
```

Owns: invariant definition, target resolution, applicability, evaluation,
comparison, aggregation, quantification, relationship checking, violations,
`PASS | FAIL | ERROR`.

Must **not**: execute Actions, mutate EconomicState, recalculate M01 economics,
repair/rollback state, change fees/prices/settlement, replace M01 evidence.

M02 must not become a second economic engine.

### M03 — Deterministic Simulator (BOUNDARY ONLY)

May construct scenarios, provide initial state, invoke M01, capture transitions,
optionally invoke M02, advance logical simulation, produce results.

M02 evaluation is **optional**: a simulation may produce authoritative M01
transitions without running invariants. When invariants are requested, M03
invokes M02 as a read-only consumer of authoritative outputs.

Must **not** redefine M01 economic semantics. Orchestrates only.

### M04 / M05 — Adversarial / Chaos (BOUNDARY ONLY)

Generate or transform scenarios/actions/perturbations per future specs.
Neither silently alters authoritative economic semantics. No detailed APIs here.

### M06 — Regression (BOUNDARY ONLY)

Compares declared expectations against authoritative outputs.

```text
Economic Truth ≠ Invariant Evaluation ≠ Test Expectation
```

A regression mismatch is not automatically an invariant violation.
M06 is not an economic authority.

### M07 / M08 — Payment / x402 adapters (BOUNDARY ONLY)

```text
External System → Adapter / Translator → AivoGuard authoritative model
```

Adapters translate; they must not inject external behavior into the
deterministic kernel. No implementation in this task.

### M09 — Multi-Agent World (BOUNDARY ONLY)

Composes multiple actors in an EconomicWorld. Economic consequences still pass
through M01. Does not replace M01. No detailed multi-agent APIs here.

---

## 6. Dependency direction

```text
Interfaces / CLI / adapters
          ↓
Orchestration (M03 / M09)
          ↓
Evaluation (M02) / regression (M06)
          ↓
Economic authority (M01)
          ↓
Domain primitives
```

Rule:

```text
Future modules depend on authoritative primitives;
authoritative primitives do not depend on future modules.
```

Conceptual graph (no circular authority):

```text
M02 → M01 / domain
M03 → M01 (required); M02 optional when invariant evaluation is requested
M04 → scenario / economic interfaces
M05 → simulation / test interfaces
M06 → M01 outputs; optionally M02 / M03 outputs
M07 → external integration boundary
M08 → x402 integration boundary
M09 → M01 + agent / world composition
```

Lower layers must not depend on CLI, UI, LLM, network, database, or payment
providers.

---

## 7. Pure vs impure

Prefer pure / environment-free logic for:

* economic arithmetic
* economic state transition
* kernel evaluation
* invariant evaluation
* deterministic simulation semantics (when specified)

External I/O belongs outside the authoritative core. Do not add async, network,
or database dependencies to the kernel to “prepare” for future features.

**CURRENT:** M01 is std-only and free of I/O/LLM/network.

---

## 8. Canonical data flow

```text
Scenario / Caller
      │
      ▼
EconomicWorld + EconomicState + Action + ExecutionContext
      │
      ▼
M01
      ├── StateBefore
      ├── Effects
      ├── Transaction disposition
      ├── StateAfter
      └── Evidence
      │
      ▼
M02 (when implemented)
      ├── Invariant evaluation
      ├── Violations
      └── PASS / FAIL / ERROR
      │
      ▼
Simulation / Regression / Reporting
```

No hidden state between stages. Reporting is not a second economic authority.

---

## 9. Data ownership matrix

| Concept | Authoritative owner |
| --- | --- |
| EconomicWorld | M01 / domain |
| EconomicState | M01 / domain |
| Action | M01 / domain |
| ExecutionContext | M01 |
| Economic effects | M01 |
| Transaction disposition | M01 |
| State transition | M01 |
| Kernel evidence | M01 |
| Invariant definition | M02 |
| Invariant violation | M02 |
| PASS / FAIL / ERROR (invariants) | M02 |
| Scenario | M03 / scenario layer |
| Regression expectation | M06 |
| External translation | M07 / M08 adapters |
| Multi-agent composition | M09 |

Alignment check: matches Product Scope module boundaries and Domain Contract
ownership of economic primitives; M02 ownership matches the M02 specification
candidate (not yet frozen).

---

## 10. State mutability

| Layer | Mutation rule |
| --- | --- |
| M01 | Authoritative state transition permitted |
| M02 | Read-only; no API that mutates authoritative EconomicState |
| M03 | Holds simulation state only by invoking authoritative transitions |
| M04 / M05 | Inputs / perturbations only — not economic truth |
| M06 | Read / compare |
| Adapters | Translate |

Prefer immutable / read-only views for M02 inputs when implemented. Exact Rust
types remain unfrozen until the M02 specification freezes and an
implementation task authorizes them.

---

## 11. Numeric boundary

Authoritative monetary arithmetic uses exact representation + checked ops.

**CURRENT implementation decision (ADR 0001):** `i128` + Asset-declared minor
units + checked arithmetic.

Architecture references this as the current M01 decision; it does not broaden
it into a new domain requirement beyond the frozen kernel specification.

No layer may convert authoritative amounts to floating point for economic
decisions.

---

## 12. Determinism boundary

Prohibited as hidden inputs to authoritative behavior:

```text
wall clock · environment · filesystem ordering · network state
database ordering · thread scheduling · unseeded randomness
LLM output · global mutable economic state
```

**CURRENT (ADR 0002):** economically material keyed collections use
`BTreeMap` / `BTreeSet`.

Future nondeterminism requires: explicit input + declared seed + defined
replay semantics — specified before implementation.

---

## 13. Evidence flow

```text
M01 → Evidence → M02 / M03 / M06 → diagnostics / regression / reports
```

Evidence is first-class. Downstream consumers observe; they do not become
competing sources of economic truth.

---

## 14. Serialization boundary

**Not frozen.** Distinguish in-memory authoritative model from future
serialization. Serialization must not become an alternative economic
authority. Semantics first; serialization second.

Open: OD-04 (M02 DSL/API), product OD items for formats/CLI/SDK as listed in
Product Scope.

---

## 15. CLI / API boundary

Final CLI, REST, GraphQL, SDK, DSL, and language bindings are **not** frozen.
They remain interface-layer seams. Do not implement or freeze them here.

---

## 16. Error boundary

| Concern | Owner |
| --- | --- |
| Kernel / economic disposition / engine errors | M01 |
| Invariant PASS / FAIL / ERROR | M02 |
| Adapter / external failures | M07 / M08 (translated only when a domain error is defined) |
| Scenario / config invalidity | Orchestration / harness (future) |

Do not collapse these into a universal catch-all that destroys semantic
distinctions.

---

## 17. Open decisions (preserved — not frozen by architecture)

Architecture remains compatible with, and does **not** decide:

| Topic | Status |
| --- | --- |
| M02 DSL / concrete API (OD-04) | OPEN |
| Serialization formats | OPEN |
| CLI / SDK / HTTP / GraphQL | OPEN |
| Plugin / adapter architecture (product OD-08) | OPEN |
| Language bindings | OPEN |
| Multi-crate split beyond `aivoguard` | OPEN until a frozen spec requires it |
| M03–M09 detailed semantics | Deferred to future gates |

---

## 18. CURRENT vs TARGET

### CURRENT

* Single crate `aivoguard` with `kernel` (M01) only
* Frozen Product Scope, Domain Contract, Economic Kernel Specification
* M02 specification candidate: READY_FOR_REVIEW / NOT FROZEN
* M03–M09: product boundaries only; no code
* No network/DB/LLM in the authoritative core

### TARGET (conceptual)

* Authority stack: M01 remains sole economic authority; M02 is optional
  read-only evaluation over authoritative outputs; orchestration / adapters /
  interfaces compose around them
* M02 as a read-only downstream module (crate or submodule TBD when authorized)
* M03–M09 as specified by future frozen gates
* Interfaces and adapters outside the economic core

---

## 19. Module status table

| Module | Specification | Architecture | Implementation | Gate |
| --- | --- | --- | --- | --- |
| M01 | Frozen | Current | Implemented | PASS |
| M02 | Ready for review | Defined | Not implemented | Gate 2 open |
| M03 | Future | Boundary only | Not implemented | Future |
| M04 | Future | Boundary only | Not implemented | Future |
| M05 | Future | Boundary only | Not implemented | Future |
| M06 | Future | Boundary only | Not implemented | Future |
| M07 | Future | Boundary only | Not implemented | Future |
| M08 | Future | Boundary only | Not implemented | Future |
| M09 | Future | Boundary only | Not implemented | Future |

---

## 20. Architecture quality gate checklist

| # | Criterion | Status |
| --- | --- | --- |
| 1 | Every current implemented responsibility has an owner | PASS (M01/kernel) |
| 2 | Every frozen domain responsibility has an architectural home | PASS |
| 3 | M01 authority unambiguous | PASS |
| 4 | M02 authority unambiguous | PASS |
| 5 | Dependency direction clear | PASS |
| 6 | Mutation boundaries clear | PASS |
| 7 | External I/O boundaries clear | PASS |
| 8 | Future modules cannot silently redefine M01 | PASS |
| 9 | No unresolved semantic decision accidentally frozen | PASS |
| 10 | CURRENT and TARGET clearly separated | PASS |

---

## 21. Document control

| Item | Value |
| --- | --- |
| Created by | TASK-00B |
| Consistency audit | TASK-00C — M03/M02 optional dependency clarified |
| Normative economic semantics | **None** (architecture only) |
| ADR created by this task | **None** (no new implementation decision) |
| Next | Continue Gate-2 process only when explicitly authorized |
