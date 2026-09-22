# TASK-11A — INDEPENDENT M04 IMPLEMENTATION AUDIT

```text
TASK-11A — INDEPENDENT M04 IMPLEMENTATION AUDIT

IMPLEMENTATION:
  COMMIT: 92316ad

AUTHORIZATION:
  TASK-10H = GRANTED (commit d0fd041; working-tree artifact currently ABSENT)

FROZEN BASELINE:
  58f0e01

FREEZE_COMMIT:
  0ff5684

HEAD_AT_AUDIT:
  92316ad (matches implementation commit)

SPECIFICATION:
  UNCHANGED by TASK-11 / 92316ad
  (58f0e01..92316ad docs/design/ diff is ONLY TASK-10G freeze metadata;
   0ff5684..92316ad and d0fd041..92316ad docs/design/ are EMPTY)

TESTS:
  100 passed (M01 19 + M02 26 + M03 28 + M04 26 + unit 1); 0 failed

CLIPPY:
  PASS (-D warnings, --all-targets --all-features)

M01:
  INTACT (ActionId::as_str + re-export only; no economic semantic change)

M02:
  INTACT

M03:
  INTACT

AUDIT_MODE:
  NO CODE CHANGES
  NO TEST CHANGES
  NO SPEC CHANGES
  NO COMMITS
```

---

## 1. Repository state

| Item | Value |
| --- | --- |
| Branch | `main` |
| HEAD | `92316ad` |
| Working tree | clean at audit start |
| Ancestry | `58f0e01` ⊂ `0ff5684` ⊂ `d0fd041` ⊂ `92316ad` |
| TASK-10H md | **ABSENT** (deleted in 92316ad; recoverable at `d0fd041`) |
| TASK-11 report md | **ABSENT** (operator-deleted; not in commit) |

---

## 2. Change-scope classification (`d0fd041..92316ad`)

| Path | Class |
| --- | --- |
| `crates/aivoguard/src/adversarial/*` | DIRECT M04 |
| `crates/aivoguard/tests/m04_adversarial.rs` | DIRECT M04 |
| `crates/aivoguard/src/lib.rs` | NECESSARY M04 INTEGRATION |
| `crates/aivoguard/src/kernel/action.rs` (`ActionId::as_str`) | NECESSARY M04 INTEGRATION |
| `crates/aivoguard/src/kernel/mod.rs` (export `ActionId`) | NECESSARY M04 INTEGRATION |
| `CHANGELOG.md` / `docs/README.md` / `architecture-baseline.md` | DOCUMENTATION |
| Delete `TASK-10G` / `TASK-10H` reports | HYGIENE (operator preference; weakens local evidence chain) |
| Unrelated refactor | **NONE observed** |

---

## 3. Authority boundary

| Search | Result |
| --- | --- |
| `evaluate(` / M01 economic entry | **ABSENT** in `adversarial/` |
| `evaluate_invariant` | **ABSENT** |
| `run_simulation` | **ABSENT** |
| `f32` / `f64` | **ABSENT** (comment only) |
| `HashMap` / `HashSet` / RNG / time / threads / serde | **ABSENT** |

M04 consumes `Scenario` / `Action` / state-world types and checks M02 `{id, definition_version}` structurally only. **Authority boundaries PASS.**

---

## 4. Requirement audit (summary)

| Requirement | Implementation | Test | Evidence | Status |
| --- | --- | --- | --- | --- |
| Occurrence distinction | `PlanOccurrence` + index | yes | positions distinct for repeated ids | **PASS** |
| EXACT_PAIR ordered | evaluates only `(a,b)` | yes | reverse / `(a,a)` covered | **PASS** |
| Lexicographic pair selection | nested `left ASC, right ASC` | yes | `[A,B,A,B] → (0,1)` | **PASS** |
| Rule ordering | vector scan, first wins | yes | first vs second rule | **PASS** |
| Version wildcard / exact | `Option` side match | yes | wildcard + mismatch | **PASS** |
| IdentifierToken | exact `String`, len≥1 | partial | empty/case/space/UTF-8; **no NFC/NFD pair** | **PASS** (impl) / gap (test) |
| Structural 4a → 4b | ordered checks | yes | world absent wins over state absent | **PASS** (ordering) |
| Checklist completeness §8.5.1 | several checks hollow | partial | see FINDING-H02 | **FAIL** |
| Projection §8.4 | **no ProjectionMode**; flag always true | weak | see FINDING-H01 | **FAIL** |
| Invariant envelope | check 9 only | yes | empty id/version | **PASS** |
| Provenance §20 | partial `ScenarioProvenance` | partial | missing params etc. | **FAIL** (completeness) |
| Parameter ordering | dim lex + numeric asc | **no dedicated test** | code present | **PASS** (impl) / gap |
| Boundary ops | Min/Max/Exact/Just± | **no dedicated test** | Just± omit vs NON_APPLICABLE | **FAIL** (semantic) |
| Generation limits | counters + policies | FailOnExceed only | TruncateAtN / mutation budget untested | **PARTIAL** |
| Applicability policy | Allow vs Require | yes | missing target cases | **PASS** |
| Determinism | no forbidden sources; Eq replay test | yes | | **PASS** |
| ActionId N=0/1/>1 | resolve + errors | yes | | **PASS** |
| DuplicateIdMode | enum implemented | Remove only tested | Inherit/Replace untested | **PARTIAL** |
| ReorderActions | permutation validated | **no test** | | **PARTIAL** |
| AD-03 | provisional `bump_metadata`; docs say OPEN | n/a | | **PRESERVED OPEN** |
| AD-14 | no serde/serialization | n/a | | **PRESERVED OPEN** |
| AD-15 | types public; marked provisional in mod docs | n/a | broad `lib.rs` re-export | **PRESERVED OPEN** (concern) |

---

## 5. Structural checklist itemization (§8.5.1)

| # | Spec | Location | Actual enforcement | Tests | Verdict |
| --- | --- | --- | --- | --- | --- |
| 1 | Scenario envelope | `validate_checklist` | Comment only; no failure path | none | **INCOMPLETE** |
| 2 | Required fields | checks id/version/configuration_id | Enforced | first-fail test | **PASS** |
| 3 | Projection consistency | `projection_consistent` flag | Production `from_scenario` **always true** | flag-only | **INCOMPLETE** |
| 4a | World envelope | `world_present` | Enforced on `ChecklistView` | yes | **PASS** |
| 4b | Initial-state structure | `initial_state_present` | Envelope flag only; cells assumed typed | yes | **PARTIAL** |
| 5 | Action sequence structure | comment | Relies solely on Rust `Action` enum | none | **INCOMPLETE** vs non-Rust inputs |
| 6 | Action references | `action_references_valid` | Production always true; real ambiguity handled at apply-time as ERROR | apply tests | **PARTIAL** (check-6 vs phase-4 split) |
| 7 | Domain IdentifierToken | balances + action fields | Enforced | empty balance cell | **PASS** |
| 8 | Execution configuration | configuration_id token | Largely overlaps check 2 | weak | **PARTIAL** |
| 9 | Invariant envelope | id + definition_version | Enforced | yes | **PASS** |
| 10 | Stop-policy structure | `stop_policy_failure` | **Always `None`** | none | **INCOMPLETE** |
| 11 | Hidden input | flag | Only if caller sets flag; production false | none | **PARTIAL** |
| 12 | Provenance completeness | `provenance_complete` | Production generation passes `true` unconditionally | none negative | **PARTIAL** |

---

## 6. Projection matrix (closed `TransformationKind` set)

| Kind | Spec projection need | Implementation | Verdict |
| --- | --- | --- | --- |
| `MetadataSuffix` | metadata DERIVE; payload INHERIT | Sets id/version; inherits rest | Behavior OK; **no declared modes** |
| `DuplicateAction` | actions REPLACE/DERIVE; metadata DERIVE; `action_id` REMOVE/INHERIT/REPLACE | Matches R-16 modes | Behavior OK; **no declared modes** |
| `DeleteAction` / `InsertAction` / `ReplaceAction` / `ReorderActions` | actions mutated; metadata bump | `bump_metadata` | Behavior OK; **no declared modes** |
| `SetInitialBalance` | initial_state REPLACE cell; metadata bump | Explicit set_balance | Behavior OK; **no declared modes** |
| `ReplaceMaximumActionSteps` | exec config REPLACE; metadata bump | Field assign | Behavior OK; **no declared modes** |
| `ReplaceTransferAmount` | action param REPLACE | Transfer/TransferWithFee only | Behavior OK; **no declared modes** |

Normative §8.4 requires **declared** projection modes per field and validation failure `INVALID_PROJECTION` when undeclared mutation occurs. Implementation encodes projection only implicitly inside kinds and never fails check 3 on the generation path → **FINDING-H01**.

Taxonomy: kinds map to Gate-4 action/state/config adversaries; not an open-ended expansion. Acceptable as a closed production subset **if** projection declaration were enforceable.

---

## 7. Limits / status notes

* `FAIL_ON_EXCEED` for `maximum_generated_scenarios` tested (N=2, third fails).
* `TRUNCATE_AT_N` **not tested**.
* `maximum_action_mutations` path exists; multi-mutation remainder accounting present but current kinds only report `0|1` mutations — multi-mutation remainder largely dead.
* `generation_status_at_emit` **hardcoded** to `Complete` in `try_emit` even when later overall status becomes `Failed`/`Truncated` (emitted-before-breach may still be Complete under FAIL_ON_EXCEED; still imprecise vs full §20 recording).

---

## 8. Determinism / panic / numeric

* No nondeterministic authorities in `adversarial/`.
* Production panic surfaces: indexed access only after resolution/permutation validation; no production `unwrap`/`expect`/`panic!`.
* `i128 as usize` for Index candidates after `n < 0` filter — platform-width truncation theoretically possible for huge positive `i128` (LOW/MEDIUM edge).
* Counter math uses `saturating_add` (cannot wrap; may mask overflow vs hard error — LOW).

---

## FINDINGS

### FINDING-H01

| Field | Value |
| --- | --- |
| ID | H01 |
| SEVERITY | **HIGH** |
| LOCATION | `transform.rs`, `validation.rs` check 3, `ChecklistView::from_scenario` |
| REQUIREMENT | §8.4 / R-01 projection modes; check 3 `INVALID_PROJECTION` |
| OBSERVATION | No `ProjectionMode` / per-field declaration. Production path sets `projection_consistent = true` always. Check 3 cannot fail for generated candidates. |
| IMPACT | Frozen projection contract is not enforceable; undeclared field mutation cannot be rejected as `INVALID_PROJECTION`. |
| RECOMMENDATION | Introduce explicit projection declarations per transformation (or equivalent enforceable representation) and validate check 3 from those declarations. Do not invent modes beyond the frozen set. |

### FINDING-H02

| Field | Value |
| --- | --- |
| ID | H02 |
| SEVERITY | **HIGH** |
| LOCATION | `validation.rs` checks 1, 5, 10 (and production wiring for 3/6/11/12) |
| REQUIREMENT | §8.5.1 closed checklist — all twelve checks |
| OBSERVATION | Check 1 is a no-op comment. Check 5 assumes Rust typing. Check 10 always returns `None`. Several other checks are flag-gated and always “pass” on the generation path. |
| IMPACT | Claimed closed structural validation is incomplete relative to the frozen checklist; some `StructuralInvalidReason` codes are unreachable in production generation. |
| RECOMMENDATION | Either implement real enforcement for each check against the normative candidate model, or open an amendment clarifying which checks are vacuously satisfied by the chosen Rust representation — do not silently treat vacuity as PASS. |

### FINDING-H03

| Field | Value |
| --- | --- |
| ID | H03 |
| SEVERITY | **HIGH** |
| LOCATION | `provenance.rs`, `engine.rs` `try_emit` |
| REQUIREMENT | §20 provenance completeness |
| OBSERVATION | Emitted provenance omits typed transformation parameters, full candidate-ordering inputs, breached-counter identity on failure artifacts, and other §20 fields. Only a partial reconstructible subset is stored. |
| IMPACT | Reproducibility/evidence contract not fully met for emitted artifacts. |
| RECOMMENDATION | Extend provenance to include at least typed parameter tuples and limit-breach identity required by §20, without inventing serialization (AD-14 remains OPEN). |

### FINDING-M01

| Field | Value |
| --- | --- |
| ID | M01 |
| SEVERITY | **MEDIUM** |
| LOCATION | `parameters.rs` `apply_numeric_op` / `dimension_values` |
| REQUIREMENT | §17A.2 JustBelow/JustAbove out-of-domain → NON_APPLICABLE (or parameter error if `x` outside) |
| OBSERVATION | Out-of-domain Just± results are **dropped** from the candidate list (`None` → skip), not evaluated as `NON_APPLICABLE` application outcomes. |
| IMPACT | Observable classification/count differences vs frozen operator semantics. |
| RECOMMENDATION | Emit explicit NON_APPLICABLE (or parameter error) per operator evaluation as specified. |

### FINDING-M02

| Field | Value |
| --- | --- |
| ID | M02 |
| SEVERITY | **MEDIUM** |
| LOCATION | `engine.rs` `generation_status_at_emit: GenerationStatus::Complete` |
| REQUIREMENT | §18.2 / §20 GenerationStatus in provenance |
| OBSERVATION | Hardcoded `Complete` at emit time; not tied to final operation status / truncation context beyond that instant. |
| IMPACT | Provenance may misrepresent generation status semantics for consumers. |
| RECOMMENDATION | Record status consistently with §18.2 emission rules and final result status. |

### FINDING-M03

| Field | Value |
| --- | --- |
| ID | M03 |
| SEVERITY | **MEDIUM** |
| LOCATION | `tests/m04_adversarial.rs` |
| REQUIREMENT | TASK-11 / §39 testing requirements |
| OBSERVATION | Missing dedicated tests for `TRUNCATE_AT_N`, mutation-limit boundaries (`0/1/N/N+1`), `ReorderActions` invalid permutations, `DuplicateIdMode::{Inherit,ReplaceExplicitly}`, parameter lexicographic product, NFC vs NFD IdentifierToken pair. |
| IMPACT | Several implemented paths lack normative evidence; count=100 is necessary but not sufficient. |
| RECOMMENDATION | Add adversarial tests named to the frozen behaviors above. |

### FINDING-M04

| Field | Value |
| --- | --- |
| ID | M04 |
| SEVERITY | **MEDIUM** |
| LOCATION | `lib.rs` / `adversarial/mod.rs` public exports |
| REQUIREMENT | AD-15 OPEN — provisional types |
| OBSERVATION | Large public re-export surface from crate root. Mod docs say provisional; risk of accidental external API freezes remains. |
| IMPACT | Social/API freeze risk; not a frozen-spec semantic break by itself. |
| RECOMMENDATION | Narrow exports or mark crate-level provisional API explicitly until AD-15 closes. |

### FINDING-M05

| Field | Value |
| --- | --- |
| ID | M05 |
| SEVERITY | **MEDIUM** |
| LOCATION | commit `92316ad` deletions |
| REQUIREMENT | Authorization evidence chain (TASK-10H) |
| OBSERVATION | `TASK-10H-M04-IMPLEMENTATION-AUTHORIZATION.md` removed from tree in the same commit as implementation. Git history retains `d0fd041`. |
| IMPACT | Weakens on-tree audit trail; does not revoke authorization historically. |
| RECOMMENDATION | Retain authorization artifacts or point docs at the authorizing commit hash only (no silent restore in audit). |

### FINDING-L01

| Field | Value |
| --- | --- |
| ID | L01 |
| SEVERITY | **LOW** |
| LOCATION | `engine.rs` multi-mutation remainder branch |
| REQUIREMENT | §18.1 action mutation counting |
| OBSERVATION | Current kinds only report `action_mutations ∈ {0,1}`; multi-mutation remainder path is effectively dead. |
| IMPACT | Maintainability / latent off-by-one risk if kinds later report >1. |
| RECOMMENDATION | Unify mutation accounting or add kinds/tests that exercise >1. |

---

## 9. Positive controls (independently verified)

* Occurrence / EXACT_PAIR / lex first-pair / rule order / version wildcard: **PASS**
* IdentifierToken exactness (no trim/fold in code): **PASS**
* 4a before 4b fail-fast ordering: **PASS**
* ActionId missing / unique / ambiguous / malformed: **PASS**
* Applicability Allow vs Require for missing targets: **PASS**
* Incompatibility evidence indices: **PASS**
* Deterministic replay Eq test: **PASS**
* No M01/M02/M03 authority calls: **PASS**
* No serde / AD-14 closed accidentally: **PASS**
* AD-03 documented provisional (`bump_metadata` comment + mod docs): **PASS** (OPEN preserved)
* Workspace tests 100 / clippy `-D warnings`: **PASS**
* Frozen design files unchanged by implementation commit: **PASS**

---

## 10. Final status

```text
STATUS: FAIL

BLOCKERS: 0
HIGH: 3
MEDIUM: 5
LOW: 1

REASON:
  High-severity incompleteness vs frozen projection contract (§8.4),
  closed structural checklist (§8.5.1), and provenance completeness (§20).
  Core occurrence/EXACT_PAIR/determinism/authority controls pass, but
  implementation is not yet a complete faithful realization of the frozen
  checklist/projection/provenance surface.

NEXT ACTION:
  Remediation task required (implement enforceable projection + complete
  checklist/provenance semantics and missing tests; then independent re-audit).
  Do NOT freeze implementation, do NOT open Gate 5, do NOT close AD-03/14/15.
```

**STOP.**
