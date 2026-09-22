# Specification Policy

**Status:** Normative process document for AivoGuard development.  
**Scope:** Process and authority only. This document does **not** define
economic semantics, product features, or domain APIs.

## Purpose

AivoGuard is developed specification-first. Code, tests, examples, and tools
must not invent or silently redefine product requirements.

## Authority hierarchy

From highest to lowest authority for *normative meaning*:

1. **Frozen normative specifications**  
   Version-controlled documents under `docs/design/` (and any future path
   explicitly designated as normative) that have been marked frozen for a
   given version or gate.
2. **Architecture decisions (ADRs)**  
   Accepted ADRs under `docs/adr/` that constrain how specifications may be
   realized. ADRs must not contradict frozen specifications; if they would,
   the specification must be amended first.
3. **Implementation**  
   Source code realizes frozen specifications and accepted ADRs. Code is not
   a source of new normative semantics.
4. **Tests**  
   Tests encode and guard the intended contract. They must reflect the
   specification; they must not invent alternate semantics.
5. **Examples**  
   Illustrative only. Examples are non-normative unless a frozen
   specification explicitly elevates a particular example.
6. **Non-normative documentation**  
   READMEs, guides, and reference notes that explain intent without defining
   binding contracts.

## Rules

### Code does not redefine the specification

Implementation must not silently change meaning that a frozen specification
already defines. If code and specification disagree, the specification wins
until an explicit amendment is accepted.

### Tests reflect the intended contract

Tests should assert behavior required by frozen specifications (and accepted
ADRs). A failing test that conflicts with a frozen specification indicates an
implementation defect, not permission to weaken the contract ad hoc.

### Semantic discoveries require amendments

If implementation work reveals that a frozen specification is incomplete,
ambiguous, or incorrect, contributors must:

1. surface the ambiguity or defect;
2. propose a specification amendment (and ADR when required);
3. only then change implementation and tests to match the amended contract.

### No invented requirements

Human contributors and automated assistants (including Cursor) must **not**
invent product requirements, economic rules, or architectural modules that are
not grounded in frozen specifications, accepted ADRs, or an explicitly scoped
bootstrap/foundation task.

### Ambiguity must be surfaced

Ambiguity must be recorded and resolved through specification/ADR process.
Silent resolution in code, comments, or assistant output is prohibited.

### Breaking semantic changes require explicit versioning

Breaking changes to frozen normative meaning require:

* an explicit specification amendment;
* clear versioning or gate/version labeling as defined by project release
  practice;
* documentation of impact for implementers and tests.

## What this policy does not define

This policy does **not** define:

* economic primitives or accounting rules;
* simulator or invariant semantics;
* agent, payment, or network protocols;
* public product APIs.

Those belong to later specification gates.

## Related documents

* [`../README.md`](../README.md) — documentation layout
* [`../adr/README.md`](../adr/README.md) — when ADRs are required
* [`../../CONTRIBUTING.md`](../../CONTRIBUTING.md) — contribution workflow
* [`../../GOVERNANCE.md`](../../GOVERNANCE.md) — decision principles
