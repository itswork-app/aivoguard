# ADR 0002 — Kernel collection ordering

| Field | Value |
| --- | --- |
| Status | Accepted |
| Date | 2026-09-22 |
| Task | TASK-04 |
| Affects | M01 Economic Kernel |

## Context

Determinism requires that collection iteration must not influence authoritative
economic output.

## Decision

Use `BTreeMap` / `BTreeSet` for all economically material keyed collections in
M01 (balances, assets, accounts, prices, effects grouping keys).

## Consequences

* Iteration order is deterministic by key ordering.
* `HashMap` is not used for authoritative economic state.

## Alternatives considered

* `HashMap` with sorted snapshots before emission — more error-prone.
* `IndexMap` — extra dependency; unnecessary for Gate 1.
