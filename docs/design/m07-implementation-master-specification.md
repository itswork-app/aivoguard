# AivoGuard M07 Implementation Master Specification

| Field | Value |
| --- | --- |
| Document | `docs/design/m07-implementation-master-specification.md` |
| Task | **TASK-37** (design) → **TASK-45** (implementation freeze evidence) |
| Module | **M07 — Payment & Settlement Testing** |
| Gate | **GATE 6** |
| Status | **FROZEN_FOR_CURRENT_SCOPE** |
| Code | **IMPLEMENTED** (core + semantic adapter boundary) |
| Normative freeze | See frozen M07 specification (immutable) |

```text
Normative authority:
  docs/design/payment-settlement-testing-specification.md
  @ b7d1874094c4b9666390cf90746838c0183ee0d6

SPECIFICATION:          FROZEN
IMPLEMENTATION DESIGN:  AUTHORIZED
IMPLEMENTATION:         FROZEN_FOR_CURRENT_SCOPE

Core (TASK-38…41):      IMPLEMENTED + audited (TASK-42 PASS)
Adapter boundary (43):  IMPLEMENTED + audited (TASK-44 PASS)
Provider / wire:        NOT AUTHORIZED / AD-14 OPEN

This document is subordinate. If it conflicts with the frozen M07
specification, the frozen specification wins. Do not amend the freeze
to make implementation easier.

FROZEN_FOR_CURRENT_SCOPE does NOT mean production-provider complete.
It records that the authorized Gate-6 in-crate scope was implemented and
independently audited. Provider integrations and wire formats remain out
of scope until separately authorized.
```

---

## 1. Authority & Frozen Baseline

```text
Product Scope FROZEN
        ↓
Domain Contract FROZEN
        ↓
M01 / M02 / M03 / M04 / M06 FROZEN + IMPLEMENTED
        ↓
M07 FROZEN SPECIFICATION @ b7d1874
        ↓
This M07 Implementation Master Spec (TASK-37)
        ↓
Cursor implementation rules (§34)
        ↓
Code (future authorized tasks only)
```

**Frozen baseline verification (TASK-37):**

* Commit `b7d1874094c4b9666390cf90746838c0183ee0d6` contains M07 `STATUS: FROZEN`.
* TASK-35 = PASS; TASK-36 = FREEZE COMPLETE.
* Open decisions remain OPEN (see §30).

**Anti-drift:** Implementation MUST NOT modify
`docs/design/payment-settlement-testing-specification.md` without amendment →
re-audit → new freeze.

---

## 2. Implementation Scope

Authorized to **design and later implement** (when a start task is issued):

* In-crate M07 evaluation engine for Gate-6 payment/settlement testing.
* Domain types, validation, expectation evaluation, verdict/error/ordering.
* M06 composition **consumer** boundary.
* Adapter **semantic** boundary (no wire format).
* Deterministic unit/integration tests proving the frozen contract.

Not authorized by this master spec alone to ship production provider adapters,
HTTP, CLI, DB, or serde wire freezes.

---

## 3. Non-Goals

Explicitly forbidden:

```text
M01 / M02 / M03 / M04 / M05 / M06 redesign
provider economic truth
new payment / settlement / refund / escrow economics
closing DC-06 / DC-09 / AD-14 / AD-15 / M07-OD-*
wire-format decision (AD-14)
engine-version policy decision (AD-15)
blockchain emulator
external payment provider integration (unless separately authorized)
UI / dashboard / CLI
LLM decision authority
floating-point monetary arithmetic
silent None→0 / None→false defaults
```

---

## 4. Repository Integration Map

### Existing reusable primitives

| Primitive | Location | M07 use |
| --- | --- | --- |
| `i128` minor units / ADR 0001 | `kernel::Money`, convention | Amount fields as `i128`; asset via declaration |
| `AssetId` | `kernel::asset` | Optional for declaration `asset_id` token identity |
| Checked arithmetic patterns | `kernel::Money`, `regression::numeric` | Reuse `compare_absolute_tolerance`; local `checked_sub` for amounts |
| `SimulationResult` | `simulator::result` | Optional on `SettlementObservation` |
| `RegressionResult` | `regression::types` | Consumed via `M06Binding` (already evaluated) |
| `RegressionVerdict` | `regression::types` | Composition MATCH/MISMATCH/ERROR |
| Absolute tolerance | `regression::numeric::compare_absolute_tolerance` | `SettledAmountAbsoluteTolerance` |
| Expectation envelope pattern | `regression::types::Expectation` | Mirror: id + class + applicable |
| MismatchEvidence / ordinal pattern | `regression::types` | M07-local evidence types (do not reuse M06 classes) |
| Integration tests | `crates/aivoguard/tests/m0N_*.rs` | Add `tests/m07_payment_settlement.rs` |
| Std-only crate | `crates/aivoguard/Cargo.toml` | Keep zero external deps |

### Do not duplicate

* Do not reimplement M06 `evaluate_regression`.
* Do not fork `compare_absolute_tolerance` with different semantics.
* Do not introduce a second `Money`-based asset-carrying observation amount type that invents observation-side asset authority.

### Known architecture lag (non-blocker)

`docs/architecture/architecture-baseline.md` still lists M07 as “boundary only /
future”. Frozen Gate-6 M07 testing specification supersedes that status for
**implementation authorization of the testing engine**. Updating the baseline is
a separate documentation task; do not invent architecture in code beyond this
master map.

### Integration conflict check (TASK-37)

```text
REPOSITORY AUDIT: PASS
No frozen-predecessor type conflict that blocks M07 testing-layer types.
M07 amount fields are i128 under payment_declaration.asset_id (R-F1),
compatible with ADR 0001 / M01 Money representation without requiring
observation-side Money.
```

---

## 5. Module / File Boundary

Proposed module (provisional; AD-15 OPEN — types may be renamed later without
semantic change):

```text
crates/aivoguard/src/payment_settlement/
├── mod.rs
├── types.rs          # case, observation, result, enums, expectations
├── error.rs          # PaymentSettlementError / ErrorId
├── validate.rs       # case, status↔phase, field matrix, amounts, fees
├── evaluate.rs       # expectation evaluation
├── engine.rs         # evaluate_payment_settlement pipeline
├── compose.rs        # M06 composition only
├── pins.rs           # engine pin exact compare
├── evidence.rs       # mismatch evidence builders
└── adapter.rs        # semantic adapter trait / helpers (no wire)
```

Wire into:

```text
crates/aivoguard/src/lib.rs   // pub mod payment_settlement;
crates/aivoguard/tests/m07_payment_settlement.rs
```

**Forbidden paths unless separately authorized:** provider SDKs, HTTP, DB, CLI
crates, serde feature freezes.

Do **not** create `src/payment/` or `src/settlement/` as economic authorities —
the testing module name above is evaluation-only.

---

## 6. Domain Type Mapping

All types below are **provisional Rust shapes** (AD-15 OPEN). Names may adjust;
semantics must match the frozen spec.

| Frozen concept | Rust (provisional) | Module | Ownership | Serde | Validation |
| --- | --- | --- | --- | --- | --- |
| `PaymentSettlementCase` | `PaymentSettlementCase` | `types` | M07 | No | Case validation |
| `PaymentDeclaration` | `PaymentDeclaration` | `types` | M07 | No | Fee / amount |
| `FeeDeclaration` | `FeeDeclaration` | `types` | M07 | No | Presence + fee rules |
| `SettlementDeclaration` | `SettlementDeclaration` | `types` | M07 | No | Terminal metadata |
| `SettlementObservation` | `SettlementObservation` | `types` | M07 | No | Matrix + amounts |
| `PaymentSettlementResult` | `PaymentSettlementResult` | `types` | M07 | No | Engine output |
| `PaymentSettlementBinding` | `PaymentSettlementBinding` | `types` | M07 | No | Exact compare |
| `SettlementStatus` | `SettlementStatus` enum | `types` | M07 | No | Closed |
| `SettlementPhase` | `SettlementPhase` enum | `types` | M07 | No | Closed |
| `SettlementExecutionState` | `SettlementExecutionState` | `types` | M07 | No | Closed |
| `RefundOutcome` | `RefundOutcome` | `types` | M07 | No | Closed |
| `EscrowStatus` | `EscrowStatus` | `types` | M07 | No | Closed |
| `FeeTiming` | `FeeTiming` | `types` | M07 | No | Orthogonal |
| `FeeMode` | `FeeMode` | `types` | M07 | No | Arithmetic |
| Expectation envelope | `SettlementExpectation` | `types` | M07 | No | Id uniqueness |
| Expectation payloads | `SettlementExpectationClass` enum | `types` | M07 | No | R-G3 / domains |
| `ScenarioBinding` (M07 optional) | `PaymentScenarioBinding` | `types` | M07 | No | Exact ids |
| `M06Binding` | `M06Binding` | `compose` | M07 | No | Identity + consume |
| `EnginePins` (M07) | `M07EnginePins` | `pins` | M07 | No | Exact string |
| `MismatchEvidence` | `PaymentMismatchEvidence` | `evidence` | M07 | No | Ordinals |
| Errors | `PaymentSettlementErrorId` | `error` | M07 | No | Frozen taxonomy |

**Reuse without ownership transfer:**

| External type | Use |
| --- | --- |
| `SimulationResult` | Optional observation field |
| `RegressionResult` | Inside `M06Binding` |
| `compare_absolute_tolerance` | Settled absolute tolerance |

**Do not reuse M06 `EnginePins` as-is** — M07 requires `m07_label`. Define
`M07EnginePins { m01, m02, m03, m07_label }`.

---

## 7. PaymentSettlementCase Mapping

```text
PaymentSettlementCase
├── case_id / case_version: String (non-empty)
├── binding: PaymentSettlementBinding
├── payment_declaration: PaymentDeclaration
├── settlement_declaration: SettlementDeclaration
├── expectations: Vec<SettlementExpectation>  // declaration order
├── observation: SettlementObservation
├── scenario_binding: Option<PaymentScenarioBinding>
├── m06_binding: Option<M06Binding>
├── engine_pins: Option<M07EnginePins>
└── notes: Option<String>  // non-authoritative
```

`PaymentDeclaration`:

```text
payment_id: String
payer / payee: String tokens
asset_id: String                    // sole Gate-6 asset authority
gross_amount: i128                  // >= 0
fee: Option<FeeDeclaration>         // sole fee_mode authority
```

`FeeDeclaration` when `Some`: require `fee_amount`, `fee_asset`, `fee_timing`,
`fee_mode`; optional payer/recipient tokens.

`SettlementDeclaration`:

```text
requested_amount: i128              // >= 0
terminal_partial: Option<bool>
terminal_partial_refund: Option<bool>  // metadata only
```

---

## 8. SettlementObservation Mapping

```text
SettlementObservation
├── binding: PaymentSettlementBinding
├── settlement_status: SettlementStatus
├── execution_state: SettlementExecutionState
├── phase: SettlementPhase
├── refund_outcome: Option<RefundOutcome>
├── requested_amount / settled_amount / unsettled_amount: Option<i128>
├── fee_amount / net_settlement_amount / refund_amount: Option<i128>
├── escrow_status: Option<EscrowStatus>
├── escrow_amount: Option<i128>     // optional; non-authoritative; not expected
├── delay_marker: Option<String>
├── settlement_delay_steps: Option<u64>
├── m07_label: Option<String>
├── simulation_result: Option<SimulationResult>
├── adapter_provenance: Option<AdapterProvenance>
└── notes: Option<String>
```

Amounts are bare `i128` denominated in `payment_declaration.asset_id`.
**No** observation-side asset field.

---

## 9. PaymentSettlementResult Mapping

```text
PaymentSettlementResult
├── verdict: PaymentSettlementVerdict  // MATCH | MISMATCH | ERROR
├── mismatches: Vec<PaymentMismatchEvidence>  // empty on ERROR
├── error: Option<PaymentSettlementError>
└── provenance: PaymentSettlementProvenance
```

On ERROR: `mismatches` MUST be empty (frozen §14.0).

---

## 10. SettlementStatus / Phase Mapping

Closed enums exactly matching frozen taxonomies:

```text
SettlementStatus:
  Pending | Authorized | Captured | Settled | PartiallySettled
  | Failed | Reversed | Refunded | PartiallyRefunded

SettlementPhase:
  Declare | Authorize | Capture | Settle | Refund | Reverse | Complete
```

Unknown status → `UnsupportedSettlementState`.
Unknown phase / invalid pair → `InvalidSettlementObservation`.

Implement the frozen §11.4 matrix as a pure function
`is_valid_status_phase(status, phase) -> bool` with exhaustive match — no
defaults.

---

## 11. Expectation Implementation Mapping

```text
SettlementExpectation {
  expectation_id: String,
  class: SettlementExpectationClass,
  applicable: bool,
}

SettlementExpectationClass:
  SettlementStatusExact { expected_status }
  SettledAmountExact { expected_settled_amount }           // >= 0
  SettledAmountAbsoluteTolerance { expected_settled_amount, tolerance } // >= 0
  FeeExact { expected_fee_amount }                         // >= 0
  NetSettlementExact { expected_net_settlement_amount }    // signed OK
  RefundStatusExact { expected_refund_outcome }
  RefundAmountExact { expected_refund_amount }             // >= 0
  ReversalStatusExact                                  // fixed semantics
  EscrowStatusExact { expected_escrow_status }
  SettlementExecutionExact { expected_execution, expected_phase: Option }
```

`applicable == false` → skip (no lookup, no mismatch, no ordinal).

---

## 12. Field Presence Validation

Implement frozen §11.5 matrix:

```text
R → Some required; None → InvalidSettlementObservation
O → Some or None (other rules apply)
A → MUST be None; Some(_) → InvalidSettlementObservation
```

Encode matrix as data + interpreter — **one** authority (frozen table), no
second ad-hoc ruleset.

Structural R ≠ expectation-time `MissingObservationField` (Option targets).

---

## 13. Amount Validation & Checked Arithmetic

```text
Representation: i128
Asset: payment_declaration.asset_id only
No f32/f64; no silent conversion
```

Non-negative domain (≥ 0):

```text
gross, requested, settled, unsettled, refund, fee
(+ escrow_amount when present)
```

Declaration negatives → `InvalidAmount`.
Observation negatives → `InvalidSettlementObservation`.
`checked_*` failure → `NumericComparisonError`.

Reuse `regression::numeric::compare_absolute_tolerance` for absolute tolerance.

### PartiallySettled (R-G1) — exact

```text
1. required fields present
2. non-negative domain
3. settled > 0 && settled < requested && unsettled > 0
4. checked_sub(requested, settled)
5. unsettled == result
```

Failure → `InvalidSettlementObservation` only (no alternate class).

### Settled (R-A2)

```text
settled == requested
unsettled present ⇒ unsettled == 0
observation.requested == settlement_declaration.requested
```

---

## 14. Fee Semantics

```text
fee_timing ⊥ fee_mode   // no compatibility matrix
```

| Mode | Derive net? |
| --- | --- |
| FeeExclusive | `derived = checked_sub(gross, fee)`; require `fee <= gross` |
| FeeInclusive | same derivation constraints |
| FeeDeclaredOnly | MUST NOT derive |

`fee_asset` MUST equal `payment_declaration.asset_id` else `InvalidAmount`.

`NetSettlementExact`: compare observation to expected; when Exclusive/Inclusive
and applicable, expected MUST equal derived at case validation else
`InvalidPaymentCase`. Never mutate observation.

---

## 15. Refund / Reversal Semantics

```text
SettlementStatus::Failed ≠ RefundOutcome::Failed
RefundStatusExact → refund_outcome only
RefundAmountExact → exact amount compare (no basis derivation)
ReversalStatusExact → phase==Reverse AND status==Reversed
```

No lifecycle history reconstruction. DC-06 remains OPEN.

---

## 16. Escrow Semantics

```text
EscrowStatusExact → escrow_status exact; None → MissingObservationField
escrow_amount → optional, non-authoritative, no Gate-6 expectation
```

No M01 escrow primitives. M07-OD-01 remains OPEN.

---

## 17. Execution-State Semantics

```text
NotAttempted | Attempted | Executed
```

Do not infer from status, phase, or M01 effects.
`SettlementExecutionExact` compares `execution_state` and optional phase.

---

## 18. Binding Validation

```text
case.binding == observation.binding  // exact field equality
else → ObservationBindingMismatch
```

Do not derive `payment_id` / `payment_version` / `configuration_id`.

---

## 19. M06 Composition Boundary

```text
M06Binding {
  m06_case_id: String,
  m06_case_version: String,
  regression_result: RegressionResult,  // already evaluated
}
```

* Identity: `regression_result.case_id/version` exact match else
  `M06CompositionError`.
* Do **not** call `evaluate_regression` inside M07.
* MATCH → continue; MISMATCH → one `M06Composition` evidence after M07 mismatches;
  ERROR → M07 ERROR, `mismatches=[]`.

---

## 20. Engine-Pin Boundary

```text
M07EnginePins {
  m01_engine_version: Option<String>,
  m02_engine_version: Option<String>,
  m03_engine_version: Option<String>,
  m07_label: Option<String>,  // opaque; AD-15 OPEN
}
```

Exact string compare only. No discovery/generation/defaults.
Missing required comparable observation pin → `EnginePinMismatch`.

---

## 21. Adapter Boundary

```text
trait SettlementObservationAdapter {
  // maps external source → SettlementObservation OR ExternalAdapterError
}
```

Must:

* reject source asset ≠ `payment_declaration.asset_id` → `ExternalAdapterError`
* preserve i128; no FX
* explicit status map; reject ambiguity
* never fabricate M01 effects; never use provider clocks as delay authority

AD-14 remains OPEN — no wire schema in this crate phase.

---

## 22. Evaluation Pipeline

Public entrypoint (provisional name):

```text
evaluate_payment_settlement(case: &PaymentSettlementCase)
  -> PaymentSettlementResult
```

Order (frozen §14):

```text
1. case validation
2. status ↔ phase
3. observation field matrix (+ Settled / PartiallySettled / A / signs)
4. binding
5. expectation ids / classes / payloads (incl. R-G3 negatives)
6. engine pins
7. expectations in declaration order
8. M06 composition
9. MATCH iff mismatches empty else MISMATCH
```

On any ERROR stage: abort; `verdict=ERROR`; `mismatches=[]`.

---

## 23. Error Taxonomy Mapping

Implement frozen IDs only:

| ErrorId | Notes |
| --- | --- |
| `InvalidPaymentCase` | Incl. R-G3 negative expected amounts |
| `InvalidSettlementCase` | Incl. terminal_partial vs Settled |
| `InvalidSettlementObservation` | Matrix / amounts / pairs |
| `ObservationBindingMismatch` | Binding |
| `MissingObservationField` | Applicable Option absence |
| `AmbiguousObservation` | Reserved; unused Gate-6 path |
| `IncompatibleObservationType` | Wrong semantic type at boundary |
| `InvalidAmount` | Declaration amount semantics |
| `NumericComparisonError` | Checked arith failure |
| `UnsupportedSettlementState` | Unknown status |
| `ExternalAdapterError` | Adapter reject |
| `EconomicStateMismatch` | Reserved unused Gate-6 |
| `M06CompositionError` | M06 ERROR / identity |
| `EnginePinMismatch` | Pins |
| `EngineError` | Internal fault |

Integrate as M07-local enum; do not collapse into M06 `RegressionErrorId`.

---

## 24. Determinism Requirements

```text
same PaymentSettlementCase + same SettlementObservation
= same PaymentSettlementResult
```

Forbidden hidden authority: wall clock, RNG, network, filesystem order,
HashMap/HashSet iteration affecting verdict, threads, env, float, LLM,
provider timing.

Use `Vec` for expectations and mismatches; never sort by id/class/hash.

---

## 25. Test Architecture

```text
crates/aivoguard/tests/m07_payment_settlement.rs   // primary integration
// optional: unit tests colocated behind #[cfg(test)] in modules
```

Organize by frozen concern: status/phase, matrix, amounts, fees, expectations,
ordering, ERROR precedence, M06 compose, adapter, determinism.

---

## 26. Test Matrix

Cover every:

* `SettlementStatus` × valid/invalid `SettlementPhase`
* Every R/O/A cell behavior (spot-check all A cells with `Some(_)`)
* Every expectation class (MATCH + MISMATCH + missing Option)
* Every emitted error id (reserved classes: assert non-emission on Gate-6 path)

---

## 27. Negative / Malformed Input Tests

Required cases include:

```text
PartiallySettled:
  100/0/100 → InvalidSettlementObservation
  100/50/50 → valid
  100/100/0 → InvalidSettlementObservation
  100/150 → InvalidSettlementObservation
  requested=0 + settled>0 → InvalidSettlementObservation

A-field: Pending + settled=Some(10) → InvalidSettlementObservation

Negative expected:
  SettledAmountExact / AbsoluteTolerance / FeeExact / RefundAmountExact
  → InvalidPaymentCase

Negative observed amounts → InvalidSettlementObservation
NetSettlementExact expected negative → allowed (signed)
```

---

## 28. Property / Invariant Tests

Where practical (no new semantics):

```text
PartiallySettled ⇒ 0 < settled < requested ∧ unsettled > 0
                 ∧ settled + unsettled == requested
Settled ⇒ settled == requested
A ⇒ Some(_) always InvalidSettlementObservation
ERROR ⇒ mismatches.is_empty()
Declaration-order mismatches ⇒ ordinals 0..n-1 contiguous for emitted items
```

---

## 29. Integration Tests

* End-to-end MATCH happy path (Settled + SettlementStatusExact).
* MISMATCH accumulates multiple applicable expectations in order.
* ERROR mid-pipeline clears prior mismatches.
* M06 MATCH / MISMATCH / ERROR composition.
* Engine pin mismatch.
* Adapter asset mismatch → ExternalAdapterError (unit adapter stub).

---

## 30. Open Decisions & Explicit Non-Implementation

Remain **OPEN** — do not encode normative defaults:

```text
AD-03, AD-12, AD-14, AD-15
M06-OD-01, M06-OD-02, M06-OD-03
DC-13, OD-07, DC-09, DC-06
M07-OD-01, M07-OD-02, M07-OD-03
```

If a future feature needs a closed decision: **STOP → REPORT → do not guess**.

Core Gate-6 testing path is implementable without closing these (observation-only
escrow; opaque `m07_label`; no wire serde).

---

## 31. Implementation Sequence

```text
Phase 0   Repository integration audit (this TASK-37)
Phase 1   Shared/domain type mapping in payment_settlement::types
Phase 2   Case / declaration / observation / result types
Phase 3   Validation primitives (signs, checked_sub helpers)
Phase 4   Status/phase + field matrix
Phase 5   Amount + fee + PartiallySettled / Settled invariants
Phase 6   Expectation engine
Phase 7   Verdict / error / ordering / ERROR precedence
Phase 8   Binding + engine pins
Phase 9   M06 composition
Phase 10  Adapter boundary (semantic stubs)
Phase 11  Complete test matrix
Phase 12  Determinism / invariant verification
Phase 13  Implementation audit (separate task)
```

First code task after authorization: **Phase 1–2 types + empty engine stub +
compile** — no provider work.

---

## 32. Acceptance Gates

| Gate | Meaning |
| --- | --- |
| G1 | Frozen spec traceability |
| G2 | Domain types |
| G3 | Validation matrix |
| G4 | Expectation engine |
| G5 | Error taxonomy |
| G6 | Ordering / precedence |
| G7 | M06 composition |
| G8 | Adapter boundary |
| G9 | Determinism |
| G10 | Complete test matrix |
| G11 | Open decisions preserved |
| G12 | Implementation audit |

Any fail blocks promotion.

---

## 33. Rollback / Failure Handling

* Spec conflict discovered → stop; do not patch frozen M07; open amendment.
* Test fail for frozen rule → fix code, not specification.
* Accidental frozen-doc edit → revert; report.
* Partial implementation mid-phase → keep `IMPLEMENTATION` status accurate
  (`PARTIAL`); do not claim PASS.

---

## 34. Cursor Implementation Rules

Cursor MUST:

1. Treat frozen M07 `@ b7d1874` as immutable authority.
2. Read repository architecture before creating files.
3. Search for reusable primitives before defining new ones.
4. Never invent missing semantics.
5. Never close an open decision.
6. Never change frozen predecessor contracts (M01–M06 / Domain / Product).
7. Never use floating-point economic arithmetic.
8. Preserve `i128` minor-unit amounts.
9. Use checked arithmetic.
10. Preserve deterministic evaluation ordering.
11. Preserve ERROR dominance (`mismatches=[]`).
12. Preserve mismatch declaration order.
13. Preserve R/O/A field semantics.
14. Preserve exact status/phase matrix.
15. Preserve exact expectation payload domains (incl. R-G3).
16. Keep M07 separate from M06 authority (consume only).
17. Keep provider adapters outside normative authority.
18. Add tests with every semantic implementation change.
19. Stop and report when a frozen-contract conflict is discovered.
20. Never silently patch the specification to make code compile.

### Anti-drift

Do not modify `docs/design/payment-settlement-testing-specification.md` without
explicit amendment authorization.

---

## Traceability Skeleton

| Implementation rule | Frozen section | Module | Test ID (planned) |
| --- | --- | --- | --- |
| PartiallySettled inequalities | §8.4 R-G1 | `validate` | `m07_ps_rg1_*` |
| A = MUST be None | §11.5 R-G2 | `validate` | `m07_matrix_a_*` |
| Negative expected amounts | §12 R-G3 | `validate` / `engine` | `m07_rg3_*` |
| Asset = declaration only | §8.1 R-F1 | `adapter` / `validate` | `m07_asset_*` |
| Net derive consumer | §8.2 R-F2 | `validate` / `evaluate` | `m07_fee_net_*` |
| fee_timing ⊥ fee_mode | §8.2 R-F3 | `types` | `m07_fee_orth_*` |
| ERROR clears mismatches | §14.0 | `engine` | `m07_err_prec_*` |
| Mismatch ordinals | §14.2 | `engine` | `m07_ord_*` |
| M06 composition | §15 | `compose` | `m07_m06_*` |
| Engine pins / m07_label | §14.1 | `pins` | `m07_pins_*` |

(Expand to full matrix during Phase 11.)

---

## Implementation Reporting Template

Every subsequent M07 implementation task must report:

```text
TASK:
BASELINE: b7d1874094c4b9666390cf90746838c0183ee0d6
FILES ADDED:
FILES MODIFIED:
FILES NOT TOUCHED:
FROZEN SPEC COMPLIANCE:
OPEN DECISIONS:
TESTS ADDED:
TESTS PASSED:
TESTS FAILED:
NEW SEMANTIC FINDINGS:
IMPLEMENTATION STATUS:
NEXT GATE:
```

---

## Document Control

```text
TASK-37 → TASK-45
STATUS: FROZEN_FOR_CURRENT_SCOPE
CODE: IMPLEMENTED (payment_settlement/ + m07_payment_settlement tests)
FROZEN SPEC: IMMUTABLE @ b7d1874094c4b9666390cf90746838c0183ee0d6

AUDITS:
  TASK-42 — core implementation audit — PASS
  TASK-44 — post-adapter implementation audit — PASS

IMPLEMENTATION SURFACE (authorized):
  TASK-38 types/skeleton
  TASK-39 validation / status-phase / R-O-A / invariants
  TASK-40 expectation evaluation / ERROR dominance
  TASK-41 binding / pins / M06 composition / terminal_partial
  TASK-43 semantic adapter boundary

NOT AUTHORIZED / NOT FROZEN:
  provider integrations
  HTTP / RPC / webhook / DB / CLI / UI
  wire serialization (AD-14 OPEN)
  open-decision closures (AD-03/12/14/15, M06-OD-*, DC-06/09/13, OD-07, M07-OD-*)

ARCHITECTURE NOTE:
  docs/architecture/architecture-baseline.md still describes M07 as future /
  boundary-only — documentation lag; do not treat baseline as M07 authority.

NEXT: separate authorization required for any provider/wire/open-decision work
```
