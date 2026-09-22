# TASK-12 — M04 TARGETED REMEDIATION REPORT

## STATUS

**IMPLEMENTED**

Evidence: F-02 / F-03 / F-04 remediations landed in M04 sources + tests;
`cargo test --workspace` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` both pass;
`git diff 0ff5684..HEAD -- docs/design/` is empty.

## BASELINE

| Role | Commit |
| ---- | ------ |
| Normative content | `58f0e01` |
| Formal freeze | `0ff5684` |
| Implementation authorization | `d0fd041` |
| Pre-remediation implementation | `92316ad` |
| Prior audit | TASK-11A (FAIL on F-02/F-03/F-04) |
| Baseline reconciliation | TASK-11B = RESOLVED |

Authoritative normative specification:

```text
docs/design/adversarial-scenario-engine-specification.md
```

## FINDINGS ADDRESSED

- F-02 — Projection contract is not represented/enforced → **PASS**
- F-03 — Parameter-candidate limit does not protect against combinatorial allocation → **PASS**
- F-04 — Limit-breach evidence is incomplete → **PASS**

## F-02 — PROJECTION CONTRACT

### Previous Defect

`TransformationDefinition` lacked an inspectable per-Scenario-field projection table.
Structural validation could not establish projection correctness from declared contract data; a boolean-style path was insufficient authority.

### Remediation

Added `crates/aivoguard/src/adversarial/projection.rs`:

- `ScenarioField` — closed enum of all required Scenario fields (ordered).
- `ProjectionMode` — `InheritUnchanged | ReplaceExplicitly | DeriveExplicitly | RemoveExplicitly | NotAllowed`.
- `FieldProjection` — typed `{ field, mode }` binding.
- `validate_projection_table` — rejects missing, duplicate, incomplete, or unknown field declarations independently of transform execution.
- `inherit_all_projections` — complete inherit-all table helper.

`TransformationDefinition` now carries `projections: Vec<FieldProjection>`.
Kind helpers (`projections_for_kind`, `definition_with_kind_projections`, `validate_definition_projections`) produce and check kind-consistent declarations.
Checklist check 3 validates `projection_declarations` via `validate_projection_table` (no `projection_consistent: bool` authority).
Phase-1 generation rejects definitions whose declared projections are incomplete or inconsistent with the kind.

### Validation

Validator derives validity from the declared table only. Undeclared / incomplete / duplicate / kind-inconsistent projections fail structurally or at phase-1.

### Tests

- `projection_valid_complete_declaration`
- `projection_missing_declaration_fails_check3`
- `projection_duplicate_field_rejected`
- `projection_inconsistent_with_kind_rejected_at_phase1`
- `projection_cannot_pass_via_boolean_flag`

## F-03 — BOUNDED PARAMETER GENERATION

### Previous Defect

Parameter generation materialized the full Cartesian product before enforcing `maximum_parameter_candidates`, so a tiny limit could still allocate a huge intermediate set.

### Remediation

Replaced full product materialization with `ParameterCandidateIter` (odometer / lexicographic stream) in `parameters.rs`.

- Per-dimension candidate lists only are allocated.
- Cross-product advances one tuple at a time.
- `take_parameter_candidates(domain, max_inclusive)` increments a counter per yielded tuple and returns structured breach on `N+1` without building the full product.
- Engine flattens plan domains and streams via the same iterator; limit checked incrementally via `try_increment`.

### Ordering Preservation

Ordering remains:

```text
dimension order → candidate order within dimension → lexicographic Cartesian
```

No `HashMap`/`HashSet` iteration, RNG, or parallel race ordering.

### Limit Semantics

```text
generate next → increment counter → compare limit → continue OR structured breach
```

Candidate generation and limit detection are distinct; breach uses `LimitBreachEvidence`.

### Tests

- `parameter_limit_one_on_huge_cartesian_is_bounded` — `200×200` domain, limit `1`
- `parameter_ordering_first_candidate_stable`
- `generation_parameter_limit_structured_and_deterministic`

## F-04 — STRUCTURED LIMIT-BREACH EVIDENCE

### Previous Defect

Limit failures exposed only human-readable `reason` strings (e.g. “maximum_parameter_candidates exceeded”) without machine-readable counter / observed / limit fields.

### Remediation

Added:

```text
LimitCounterId {
  MaximumGeneratedScenarios,
  MaximumTransformationsPerPlan,
  MaximumCompositionDepth,
  MaximumActionMutations,
  MaximumParameterCandidates,
}

LimitBreachEvidence {
  counter: LimitCounterId,
  observed: u64,
  limit: u64,
}
```

`AdversarialError::limit_breach` attaches typed evidence; `reason` remains a human diagnostic derived from the evidence (not the authority). Provenance/result path surfaces `limit_breach` on the error.

### Structured Fields

| Field | Meaning |
| ----- | ------- |
| `counter` | Which bounded counter breached |
| `observed` | Exact deterministic count at breach (`N+1` under FAIL_ON_EXCEED) |
| `limit` | Exact configured maximum `N` |

### Provenance

Consumers read `error.limit_breach` programmatically without parsing `reason` / Debug / logs.

### Tests

- `parameter_limit_one_on_huge_cartesian_is_bounded` — counter / observed / limit
- `generation_parameter_limit_structured_and_deterministic` — repeated identical breach
- `action_mutation_limit_breach_structured` — `MaximumActionMutations`

## DETERMINISM

Verified:

- no RNG
- no wall-clock dependence for economic/generation truth
- no filesystem / HashMap / HashSet ordering for candidate order
- no thread-scheduling dependence
- identical inputs → identical outputs / breach evidence (tested)

## M01/M02/M03 BOUNDARY

| Module | Modified |
| ------ | -------- |
| M01 Kernel | **NO** |
| M02 Invariants | **NO** |
| M03 Simulator | **NO** |
| M04 Adversarial | **YES** (targeted remediation only) |

M04 still does not recalculate economic truth, replace invariant evaluation, or own simulator termination.

## SPECIFICATION INTEGRITY

```bash
git diff 0ff5684..HEAD -- docs/design/
```

Result: **EMPTY** (0 bytes).

No frozen normative specification was modified.

## TEST RESULTS

```text
cargo test --workspace
  → PASS (includes prior M01/M02/M03 suites + M04 35 tests)

cargo clippy --workspace --all-targets --all-features -- -D warnings
  → PASS

Focused M04:
  crates/aivoguard/tests/m04_adversarial.rs → 35 passed
```

New F-02 / F-03 / F-04 tests included in the 35.

## CHANGE SCOPE

Expected / actual implementation files:

```text
crates/aivoguard/src/adversarial/projection.rs   (new)
crates/aivoguard/src/adversarial/engine.rs
crates/aivoguard/src/adversarial/error.rs
crates/aivoguard/src/adversarial/mod.rs
crates/aivoguard/src/adversarial/parameters.rs
crates/aivoguard/src/adversarial/provenance.rs
crates/aivoguard/src/adversarial/transform.rs
crates/aivoguard/src/adversarial/types.rs
crates/aivoguard/src/adversarial/validation.rs
crates/aivoguard/src/lib.rs
crates/aivoguard/tests/m04_adversarial.rs
TASK-12-M04-TARGETED-REMEDIATION-REPORT.md
```

Unrelated dirty / untracked artifacts intentionally **not** part of the remediation commit:

```text
TASK-11A-M04-IMPLEMENTATION-AUDIT.md
TASK-11B-M04-FREEZE-BASELINE-RECONCILIATION.md
```

## OPEN DECISIONS

AD-03: OPEN  
AD-14: OPEN  
AD-15: OPEN  

Not resolved by this task.

## CONCLUSION

TASK-12 targeted remediation of F-02, F-03, and F-04 is **IMPLEMENTED** against the reconciled freeze baseline. Spec untouched. M01/M02/M03 untouched. Open decisions remain open. Next action is an independent M04 re-audit (outside this task).
