# TASK-10H — M04 Implementation Authorization

## 1. Status

```text
STATUS: AUTHORIZED
MODULE: M04 — Adversarial Scenario Engine
GATE: POST-GATE-4 IMPLEMENTATION AUTHORIZATION
M04: FROZEN
GATE-4: CLOSED
IMPLEMENTATION: AUTHORIZED FOR SUBSEQUENT TASK
```

## 2. Authority Chain

```text
NORMATIVE_BASELINE: 58f0e01
AUDIT: TASK-10F-R3
FREEZE: TASK-10G
FREEZE_COMMIT: 0ff5684
CURRENT_HASH_STAMP: 0dec02d
HEAD_AT_AUTHORIZATION: 0dec02d
AUTHORITATIVE_SPEC: docs/design/adversarial-scenario-engine-specification.md
```

## 3. Preconditions

```text
R-19: PASS
R-20: PASS
R-21: PASS
R-22: PASS
A-J: CLOSED
CROSS_SECTION: PASS
CLEAN_ROOM: PASS
DETERMINISM: PASS
M01: INTACT
M02: INTACT
M03: INTACT
M04_STATUS: FROZEN
GATE_4: CLOSED
M04_IMPLEMENTATION_BEFORE_AUTHORIZATION: ABSENT
TEST_BASELINE: 73
BLOCKERS: NONE
SPECIFICATION_CHANGED_AFTER_FREEZE: NO
```

All prerequisites verified at HEAD `0dec02d` with freeze commit `0ff5684` in ancestry.

## 4. Authorization

M04 implementation is **AUTHORIZED** beginning with a subsequent
implementation task.

**TASK-10H itself performs NO implementation.**

No M04 crate, module, runtime code, production dependency, adapter, CLI, API,
serialization format, or tests are created by this task.

## 5. Frozen Contract

Implementation MUST conform to the frozen M04 specification, including:

```text
occurrence semantics (zero-based plan positions; repeats distinct)
EXACT_PAIR ordered (a,b) matching (no implicit (b,a))
deterministic lexicographic candidate ordering
first-match semantics
structural validation 4a → 4b (fail-fast primary reason)
IdentifierToken (UTF-8; length ≥ 1; exact; case-sensitive; no trim/normalize/fold)
invariant envelope { id, definition_version } (non-evaluating)
ActionId resolution (N=0 / N=1 / N>1 / AMBIGUOUS_ACTION_ID)
generation limits and GenerationStatus
NonApplicable vs Error / ApplicabilityPolicy
evidence and provenance (incl. winning occurrence indices)
reproducibility
authority boundaries vs M01 / M02 / M03
invariants A–J
```

## 6. Protected Contract

The frozen M04 specification **MUST NOT** be changed by implementation.

Any semantic change requires:

```text
explicit specification amendment
re-audit
re-freeze
before continuing implementation
```

Hierarchy:

```text
FROZEN M04 SPECIFICATION
        ↓
IMPLEMENTATION CONTRACT (this authorization)
        ↓
M04 IMPLEMENTATION
        ↓
TESTS / VERIFICATION
```

Never reverse this hierarchy.

## 7. Open Decisions

Remain **OPEN** and **MUST NOT** be silently resolved by implementation:

```text
AD-03 — identity algorithm
AD-14 — serialization
AD-15 — Rust types
CLI / API / SDK decisions
other explicitly open implementation-only decisions in the M04 AD register
```

If implementation requires closing one of these:

```text
STOP
REPORT THE DEPENDENCY
REQUEST A SEPARATE DECISION / AMENDMENT
```

## 8. Implementation Constraints

Subsequent M04 implementation MUST be:

```text
deterministic
free of hidden environment authority
free of LLM semantic authority
free of RNG / wall-clock / filesystem-order / HashMap-iteration semantic authority
free of occurrence/identity collapse
free of heuristic matching
free of validation-order drift
free of M01/M02/M03 semantic duplication
free of unrelated refactor
```

Integrate with existing repository architecture (Rust workspace conventions).
Do not invent a parallel architecture.

## 9. Test Boundary

```text
Existing baseline: 73 (M01 19 / M02 26 / M03 28)
TASK-10H test modifications: NONE
```

The subsequent M04 implementation task MUST add dedicated M04 tests covering
the frozen contract (occurrence, EXACT_PAIR, ordering, first-match, evidence
indices, 4a→4b, IdentifierToken, invariant envelope, determinism, ActionId,
limits, authority boundaries, etc.).

## 10. Amendment Rule

If implementation discovers a missing normative decision:

```text
STOP
DO NOT GUESS
OPEN AMENDMENT / CLARIFICATION
RE-AUDIT IF SEMANTICS CHANGE
RE-FREEZE BEFORE CONTINUING
```

## 11. Final Authorization

```text
M04 = FROZEN
GATE-4 = CLOSED
IMPLEMENTATION = AUTHORIZED
IMPLEMENTATION_TASK = REQUIRED (e.g. TASK-11)
SEMANTIC_AMENDMENT = NOT AUTHORIZED
TASK-10H_CODE_CREATED = NONE
```

**STOP.**
