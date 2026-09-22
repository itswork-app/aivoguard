# AivoGuard Architecture Baseline

| Field | Value |
| --- | --- |
| Document | `docs/architecture/architecture-baseline.md` |
| Task | TASK-00B |
| Status | **BASELINE** (architecture documentation; not a normative economic spec) |
| Authority | Subordinate to frozen design specifications and accepted ADRs |
| Implementation | Documents current + target structure; M01 and M02 are implemented |

This document maps frozen product/domain/economic specifications into repository
architecture and module boundaries. It does **not** invent economic semantics,
freeze open DSL/API/serialization decisions, or authorize new modules beyond
what frozen gates already implement.

Related:

* [`../design/product-scope.md`](../design/product-scope.md) (**FROZEN**)
* [`../design/domain-contract.md`](../design/domain-contract.md) (**FROZEN**)
* [`../design/economic-kernel-specification.md`](../design/economic-kernel-specification.md) (**FROZEN**)
* [`../design/economic-invariant-engine-specification.md`](../design/economic-invariant-engine-specification.md) (**FROZEN**; Gate 2 **CLOSED**)
* [`../design/deterministic-simulator-specification.md`](../design/deterministic-simulator-specification.md) (**FROZEN**; Gate 3 **CLOSED**; implemented TASK-08)
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
ADAPTERS            M07 payment/settlement testing + semantic adapter boundary
                    (IMPLEMENTED / FROZEN_FOR_CURRENT_SCOPE; Gate 6);
                    M08 x402 external translation (future)
ORCHESTRATION       M03 simulator (IMPLEMENTED / PASS; Gate 3 CLOSED); M09 multi-agent (future)
EVALUATION          M02 invariants (IMPLEMENTED / PASS; Gate 2 CLOSED);
                    M06 regression (IMPLEMENTED; Gate 5 CLOSED)
ECONOMIC AUTHORITY  M01 kernel (IMPLEMENTED / PASS; Gate 1 CLOSED)
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
    kernel/              ← ECONOMIC AUTHORITY + DOMAIN (M01)
    invariant/           ← EVALUATION (M02; read-only)
    simulator/           ← ORCHESTRATION (M03; sequential; read-only over M01/M02)
    adversarial/         ← M04 adversarial scenario generation (IMPLEMENTED)
    regression/          ← M06 expectation comparison (IMPLEMENTED)
    payment_settlement/  ← M07 payment/settlement testing + semantic adapter
                           (IMPLEMENTED / FROZEN_FOR_CURRENT_SCOPE)
    # future seams (NOT created as product modules):
    # chaos/ x402 provider adapters / multi-agent …
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

### M01 — Economic Kernel (IMPLEMENTED / PASS; Gate 1 CLOSED)

Owns:

* EconomicWorld, EconomicState, Action, ExecutionContext
* economic validation, authorization/eligibility, evaluation
* effects, transaction disposition, state transition
* foundational checks, kernel evidence

Does **not** own: invariant language, scenario orchestration, adversarial/
chaos generation, regression harness, payment/x402 adapters, multi-agent
orchestration.

### M02 — Economic Invariant Engine (IMPLEMENTED / PASS; Gate 2 CLOSED)

```text
READ-ONLY · DETERMINISTIC · NON-MUTATING
```

Owns: invariant definition, target resolution, applicability, evaluation,
comparison, aggregation, quantification, relationship checking, violations,
`PASS | FAIL | ERROR`.

Must **not**: execute Actions, mutate EconomicState, recalculate M01 economics,
repair/rollback state, change fees/prices/settlement, replace M01 evidence.

M02 must not become a second economic engine.

Specification: **FROZEN**. Implementation: present under `crates/aivoguard/src/invariant/`
(TASK-06 / TASK-06R).

### M03 — Deterministic Simulator (IMPLEMENTED / PASS; Gate 3 CLOSED)

May construct scenarios, provide initial state, invoke M01, capture transitions,
optionally invoke M02, advance logical simulation, produce results.

M02 evaluation is **optional**: a simulation may produce authoritative M01
transitions without running invariants. When invariants are requested, M03
invokes M02 as a read-only consumer of authoritative outputs.

Must **not** redefine M01 economic semantics. Orchestrates only.

Specification: **FROZEN** (`docs/design/deterministic-simulator-specification.md`;
TASK-07 / TASK-07R / TASK-07F). Implementation: present under
`crates/aivoguard/src/simulator/` (TASK-08 / TASK-08R). Independent
implementation audit: TASK-08F **PASS**. Implementation baseline commit:
`c4ce9bb`.

### M04 — Adversarial Scenario Engine (IMPLEMENTED; Gate 4 CLOSED)

Gate 4 **CLOSED**. Normative specification:
[`docs/design/adversarial-scenario-engine-specification.md`](../design/adversarial-scenario-engine-specification.md)
(**FROZEN**). Authorized by **TASK-10H**; implemented under
`crates/aivoguard/src/adversarial/` (**TASK-11**).

Normative baseline: `58f0e01`. Freeze: **TASK-10G** (audit **TASK-10F-R3**).

M04 generates/transforms adversarial Scenario inputs for M03. It does **not**
own economic truth (M01), invariant evaluation (M02), or simulation sequencing
(M03). Open decisions AD-03 / AD-14 / AD-15 remain OPEN.

### M05 — Chaos (BOUNDARY ONLY)

Economic fault-injection perturbations per future specs. Distinct from M04.
Neither silently alters authoritative economic semantics. No detailed APIs here.

### M06 — Regression (**FROZEN** specification; IMPLEMENTED)

Compares declared expectations against authoritative outputs per
[`docs/design/economic-regression-engine-specification.md`](../design/economic-regression-engine-specification.md)
(**FROZEN**, Gate 5 **CLOSED**, TASK-19; audit TASK-18). Implementation under
`crates/aivoguard/src/regression/`.

```text
Economic Truth ≠ Invariant Evaluation ≠ Test Expectation
```

A regression mismatch is not automatically an invariant violation.
M06 is not an economic authority. DC-13 remains formally OPEN in the Domain
Contract; M06 freeze does not close DC-13.

### M07 — Payment & Settlement Testing (**IMPLEMENTED** / **FROZEN_FOR_CURRENT_SCOPE**)

Specification:
[`docs/design/payment-settlement-testing-specification.md`](../design/payment-settlement-testing-specification.md)
(**FROZEN** @ `b7d1874`; Gate 6). Implementation under
`crates/aivoguard/src/payment_settlement/` (TASK-38…41 core; TASK-43 semantic
adapter; audits TASK-42 / TASK-44 **PASS**; freeze evidence TASK-45).

Owns (testing layer only):

* PaymentSettlementCase / declarations / SettlementObservation / result
* status↔phase matrix, R/O/A field matrix, Settled / PartiallySettled invariants
* amount-domain and fee semantics, closed settlement expectations
* deterministic evaluation, mismatch/ERROR dominance, binding, engine pins
* M06 composition (**consume-only**)
* semantic external-observation adapter boundary (no provider I/O)

```text
External semantic observation
        ↓
M07 semantic adapter
        ↓
SettlementObservation
        ↓
M07 validation / evaluation
```

Does **not** own: M01 economic truth, M02 invariants, M03 simulation, M04
generation, M06 expectation engine internals, provider payment execution,
HTTP/RPC/webhook/DB, or a frozen external wire format (**AD-14 OPEN**).

Provider-specific integrations remain **NOT AUTHORIZED**. Open decisions
AD-03 / AD-12 / AD-14 / AD-15, DC-06 / DC-09 / DC-13, OD-07, M06-OD-*, and
M07-OD-* remain **OPEN**.

### M08 — x402 adapters (BOUNDARY ONLY)

```text
External System → Adapter / Translator → AivoGuard authoritative model
```

Adapters translate; they must not inject external behavior into the
deterministic kernel. No M08 implementation in this baseline.

### M09 — Multi-Agent World (BOUNDARY ONLY)

Composes multiple actors in an EconomicWorld. Economic consequences still pass
through M01. Does not replace M01. No detailed multi-agent APIs here.

---

## 6. Dependency direction

```text
Interfaces / CLI / provider adapters (future)
          ↓
Orchestration (M03 / M09)
          ↓
Evaluation (M02) / regression (M06) /
payment-settlement testing (M07; semantic adapter boundary)
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
M07 → payment/settlement testing; consumes M06; semantic adapter boundary
      (not provider truth; not wire protocol)
M08 → x402 integration boundary (future)
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
M02 (IMPLEMENTED; Gate 2 CLOSED)
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
| Transaction actor | M01 |
| State transition | M01 |
| Kernel evidence | M01 |
| Invariant definition | M02 |
| Invariant violation | M02 |
| PASS / FAIL / ERROR (invariants) | M02 |
| Scenario | M03 / scenario layer |
| Regression expectation | M06 |
| Payment/settlement testing observation & expectations | M07 |
| External semantic adapter mapping (non-authority) | M07 |
| External provider wire / production payment execution | NOT AUTHORIZED (AD-14 / DC-09 OPEN) |
| External translation (x402) | M08 |
| Multi-agent composition | M09 |

Alignment check: matches Product Scope module boundaries and Domain Contract
ownership of economic primitives; M02 ownership matches the **FROZEN** M02
specification (Gate 2 **CLOSED**).

---

## 10. State mutability

| Layer | Mutation rule |
| --- | --- |
| M01 | Authoritative state transition permitted |
| M02 | Read-only; no API that mutates authoritative EconomicState |
| M03 | Holds simulation state only by invoking authoritative transitions |
| M04 / M05 | Inputs / perturbations only — not economic truth |
| M06 | Read / compare |
| M07 | Read / validate / evaluate settlement observations; no payment execution |
| Provider / wire adapters | NOT AUTHORIZED (AD-14 OPEN); translate only when separately authorized |

Prefer immutable / read-only views for M02 inputs. M02 consumes M01
authoritative records (`EconomicWorld`, `EconomicState`, `Transaction`,
history) without mutation. COUNT uses a dimensionless `Count` type; it is
not Money and not an Asset.

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
| M03–M09 detailed APIs / serialization | Deferred where not frozen; M03/M06/M07 testing surfaces implemented for authorized scope; M07 provider wire (AD-14) and public product APIs remain OPEN |
| AD-14 external payment/settlement wire format | OPEN |
| M07-OD-01 / M07-OD-02 / M07-OD-03 | OPEN |

---

## 18. CURRENT vs TARGET

### CURRENT

* Single crate `aivoguard` with `kernel` (M01), `invariant` (M02),
  `simulator` (M03), `adversarial` (M04), `regression` (M06), and
  `payment_settlement` (M07)
* Frozen Product Scope, Domain Contract, Economic Kernel Specification
* M02 specification: **FROZEN** (Gate 2 **CLOSED**); implemented / **PASS**
  (TASK-06 / TASK-06R)
* M03 specification: **FROZEN** (Gate 3 **CLOSED**); implemented / **PASS**
  (TASK-08 / TASK-08R; TASK-08F audit **PASS**; baseline `c4ce9bb`)
* M04 specification: **FROZEN** (Gate 4 **CLOSED**); implemented (TASK-11;
  authorized TASK-10H)
* M06 specification: **FROZEN** (Gate 5 **CLOSED**; TASK-19; audit TASK-18);
  implemented under `src/regression/`
* M07 specification: **FROZEN** (Gate 6; @ `b7d1874`); **IMPLEMENTED** /
  **FROZEN_FOR_CURRENT_SCOPE** (core TASK-38…41; semantic adapter TASK-43;
  audits TASK-42 / TASK-44 **PASS**; freeze evidence TASK-45)
* M07 provider integrations and external wire formats: **NOT AUTHORIZED**
  (**AD-14 OPEN**)
* M05 / M08 / M09: product boundaries only; no code
* No network/DB/LLM in the authoritative core

### TARGET (conceptual)

* Authority stack: M01 remains sole economic authority; M02 is optional
  read-only evaluation over authoritative outputs; orchestration / adapters /
  interfaces compose around them
* M02 as a read-only downstream module under `crates/aivoguard/src/invariant/`
  (implemented; further packaging remains an implementation decision)
* M03 as a read-only orchestration module under `crates/aivoguard/src/simulator/`
  (implemented; packaging remains an implementation decision)
* M04 as specified by frozen Gate-4 contract (**FROZEN**; implemented TASK-11)
* M06 as specified by frozen Gate-5 contract (**FROZEN**; implemented)
* M07 as specified by frozen Gate-6 contract (**FROZEN**; implemented for
  current authorized scope; provider/wire require separate authorization)
* M05 / M08 / M09 as specified by future frozen gates
* Interfaces and provider adapters outside the economic core

---

## 19. Module status table

| Module | Specification | Architecture | Implementation | Gate |
| --- | --- | --- | --- | --- |
| M01 | Frozen | Current | Implemented / **PASS** | Gate 1 **CLOSED** / TASK-04 |
| M02 | Frozen | Defined | Implemented / **PASS** | Gate 2 **CLOSED** / TASK-06 / TASK-06R |
| M03 | Frozen | Defined | Implemented / **PASS** | Gate 3 **CLOSED** / TASK-08 / TASK-08R / TASK-08F / TASK-09 |
| M04 | **FROZEN** | Defined | **IMPLEMENTED** (TASK-11) | Gate 4 **CLOSED** / TASK-10G / TASK-10H |
| M05 | Future | Boundary only | Not implemented | Future |
| M06 | **FROZEN** | Defined | **IMPLEMENTED** | Gate 5 **CLOSED** / TASK-19 (audit TASK-18) |
| M07 | **FROZEN** | Defined | **IMPLEMENTED** / **FROZEN_FOR_CURRENT_SCOPE** (semantic adapter; no provider/wire) | Gate 6 / TASK-45 (audits TASK-42 / TASK-44) |
| M08 | Future | Boundary only | Not implemented | Future |
| M09 | Future | Boundary only | Not implemented | Future |

---

## 20. Architecture quality gate checklist

| # | Criterion | Status |
| --- | --- | --- |
| 1 | Every current implemented responsibility has an owner | PASS (M01/kernel; M02/invariant; M03/simulator; M04/adversarial; M06/regression; M07/payment_settlement) |
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
| Remediation | TASK-06R — transaction actor + COUNT dimensionality |
| Freeze | TASK-07F — Gate 3 M03 specification frozen |
| Implementation | TASK-08 / TASK-08R — M03 deterministic simulator |
| Independent audit | TASK-08F — M03 implementation audit **PASS** |
| Gate-3 closure | **TASK-09** — M03 baseline freeze / Gate 3 closure |
| M03 implementation baseline | `c4ce9bb` |
| Normative economic semantics | **None** (architecture only) |
| ADR created by this task | **None** (no new implementation decision) |
| M07 architecture sync | TASK-46 — baseline status synchronized to IMPLEMENTED / FROZEN_FOR_CURRENT_SCOPE |
| Next | Provider/wire/open-decision work requires separate authorization; M05/M08/M09 remain future |
