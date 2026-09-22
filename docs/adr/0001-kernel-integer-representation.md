# ADR 0001 — Kernel integer representation

| Field | Value |
| --- | --- |
| Status | Accepted |
| Date | 2026-09-22 |
| Task | TASK-04 |
| Affects | M01 Economic Kernel |

## Context

The frozen Economic Kernel Specification (EK-NUM-01) requires signed integer
minor-unit amounts with checked arithmetic and `EngineError` on range
exhaustion. Machine width / bigint choice is an **implementation decision**,
not domain semantics.

## Decision

Use Rust `i128` as the authoritative amount representation for Gate 1 M01.

## Consequences

* Same-asset arithmetic uses `checked_*` operations only.
* Overflow/underflow of arithmetic → `EngineError`, never wrap/saturate.
* Domain semantics (Asset unit/scale, no float authority) remain unchanged.
* Wider values than `i128` are out of Gate 1 range; exhaustion is an engine
  error, not silent success.

## Alternatives considered

* `i64` — smaller headroom for intermediate products.
* Arbitrary-precision integers — would add a dependency; deferred unless a
  later gate requires unbounded range.
