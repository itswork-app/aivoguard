# Architecture Decision Records (ADRs)

## Purpose

ADRs capture significant, durable decisions and their consequences so the
project does not rely on oral history or silent code-level invention.

## When an ADR is required

Create an ADR before (or as part of) changes that affect any of:

* architecture (component boundaries, crate layout, control flow ownership);
* public API surface;
* persistence model or storage contracts;
* determinism guarantees or sources of nondeterminism;
* serialization formats or compatibility;
* arithmetic semantics (once economic arithmetic is specified);
* security boundaries and trust assumptions;
* compatibility commitments (versioning, migration, wire formats);
* externally observable behavior that callers or tests rely on.

Routine typo fixes, internal refactors with no semantic or API impact, and
pure documentation clarifications do not require ADRs.

## Process (bootstrap)

1. Draft an ADR using a clear title and status (`Proposed`).
2. Reference related specifications. If the decision would change normative
   meaning, amend the specification first or in the same change set.
3. Seek review via pull request.
4. Mark `Accepted`, `Rejected`, `Deprecated`, or `Superseded` explicitly.

No fictional ADRs should be added. Do not decide economic architecture in
foundation bootstrap work.

## Suggested filename pattern

```text
NNNN-short-title.md
```

Example: `0001-workspace-layout.md` (only when a real decision is being made).

## Suggested sections

* Status
* Context
* Decision
* Consequences
* Alternatives considered
* Related specifications / ADRs

## Current records

| ADR | Title | Status |
| --- | --- | --- |
| [0001](0001-kernel-integer-representation.md) | Kernel integer representation (`i128`) | Accepted |
| [0002](0002-kernel-collection-ordering.md) | Kernel collection ordering (`BTreeMap`) | Accepted |
