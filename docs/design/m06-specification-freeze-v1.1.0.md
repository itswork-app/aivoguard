# M06 Gate-5 Specification Freeze Record — v1.1.0

| Field | Value |
|---|---|
| Record | **TASK-70** |
| Decision date | 2026-11-07 |
| Module / Gate | M06 — Economic Regression Engine / Gate 5 |
| Decision | **FROZEN / NORMATIVE — v1.1.0** |
| Maintainer authorization | [TASK-68 — M06 Implementation Authorization](m06-implementation-authorization.md) |
| Amendment tracking | [TASK-69 — Issue #18](https://github.com/itswork-app/aivoguard/issues/18) |
| Current specification | [economic-regression-engine-specification.md](economic-regression-engine-specification.md) |
| Historical v1.0.0 | [archive/economic-regression-engine-specification-v1.0-task19.md](archive/economic-regression-engine-specification-v1.0-task19.md) |

## 1. Exact frozen artifact

The v1.1.0 specification content frozen by this record is identified by both its
repository commit and Git blob SHA:

- Specification commit: `47bb83b45e7093c1e1aa4cfdf653e01381f4f00b`
- Specification blob SHA: `bfaafd8ff9eaae1734c26709e4ed79cf1fa34ef2`
- Path: `docs/design/economic-regression-engine-specification.md`
- Branch at freeze: `governance/m06-implementation-ratification`
- Base branch: `main`

The exact original v1.0.0 TASK-19 specification text remains preserved in the
archive file. v1.1.0 supersedes v1.0.0 only for the clauses explicitly listed
in §32 of the v1.1.0 specification.

## 2. Accepted amendments

The following six amendments were accepted and frozen:

1. **M06-A01 — Positional index portability:** checked `u64` to `usize`
   conversion; unrepresentable or out-of-range indexes produce
   `ERROR / MissingObservationField`.
2. **M06-A02 — Invariant phase identity:** match the invariant ID and explicit
   `EvaluationPoint` corresponding to the declared before/after phase.
3. **M06-A03 — Identifier validation:** reject empty account, asset, facet, and
   invariant target tokens as `ERROR / InvalidExpectation`, including on
   non-applicable expectations; do not trim or normalize non-empty tokens.
4. **M06-A04 — Mismatch target evidence:** include canonical invariant location
   descriptors in mismatch evidence.
5. **M06-A05 — Ambiguous observation diagnostics:** include target descriptors
   and match counts for ambiguous lookups, and target descriptors for missing
   and not-executed invariant lookup errors.
6. **M06-A06 — Version identity:** clarify that the provisional
   `m06_spec_version_label` field currently carries the implementation engine
   version label, not the normative specification version.

Implementation and regression tests are in:

- `crates/aivoguard/src/regression/`
- `crates/aivoguard/tests/m06_regression.rs`

## 3. Verification evidence

GitHub Actions **run #102** passed on the exact v1.1.0 specification commit
`712c24dddd202534d62642e346dc3702740d76af`:

- `cargo fmt --all -- --check`: PASS
- `cargo check --workspace --all-targets`: PASS
- `cargo test --workspace --all-targets`: PASS
- deterministic M01 example and exact output assertion: PASS
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS

Run: https://github.com/itswork-app/aivoguard/actions/runs/42330110308

## 4. Authority and unchanged decisions

- TASK-68 is the explicit maintainer authorization for M06 and prospective
  ratification effective 2026-11-07. This does not fabricate historical
  authorization before that decision.
- Gate 5 is CLOSED / FROZEN for the v1.1.0 contract.
- M06 remains a deterministic consumer/comparator of authoritative M03
  observations; it is not an economic engine, invariant engine, simulator,
  adversarial generator, payment processor, or runtime enforcement layer.
- No M01–M04 economic semantics were changed.
- **DC-13 remains OPEN.**
- **AD-03, AD-12, AD-14, AD-15, M06-OD-01, M06-OD-02, and M06-OD-03 remain OPEN.**
- No serialization, CLI, SDK, HTTP, shrinking, network protocol, or new
  expectation class is authorized by TASK-70.
- This freeze is not an independent correctness/security audit, formal
  verification, or production-readiness certification.

## 5. Freeze decision

The maintainer accepts the v1.1.0 amendment set and freezes the exact
specification artifact identified in §1 as the current normative Gate-5 M06
contract. Implementation and tests must conform to this version. Future
semantic changes require a new explicit versioned specification amendment.
