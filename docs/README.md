# Documentation

This directory contains AivoGuard's version-controlled specifications, architecture records, implementation evidence, and non-normative product/business planning.

## Document authority

| Document class | Authority |
|---|---|
| Frozen design specifications under `design/` | Normative economic/product semantics for their declared gate/version |
| Accepted ADRs under `adr/` | Architectural constraints subordinate to frozen specifications |
| Architecture baseline | Describes implemented structure; does not redefine semantics |
| Implementation status ledger | Current code/test/authorization evidence register; does not amend frozen semantics |
| Reference and business documents | Explanatory or strategic only; cannot authorize product scope or APIs |

Read [Specification Policy](design/SPECIFICATION-POLICY.md) before changing normative behavior.

## Current implementation and gates

The current source tree contains M01–M04 and M06–M08. M05 and M09 are not implemented. See the [Implementation Status Ledger](implementation-status.md) for source paths, integration tests, gate evidence, limitations, and unresolved authorization reconciliation.

| Module | Specification | Current implementation status |
|---|---|---|
| M01 Economic Kernel | [Frozen](design/economic-kernel-specification.md), Gate 1 closed | Implemented; TASK-04 evidence |
| M02 Invariant Engine | [Frozen](design/economic-invariant-engine-specification.md), Gate 2 closed | Implemented; TASK-06 / TASK-06R evidence |
| M03 Deterministic Simulator | [Frozen](design/deterministic-simulator-specification.md), Gate 3 closed | Implemented; TASK-08 / TASK-08R / TASK-08F evidence |
| M04 Adversarial Scenario Engine | [Frozen](design/adversarial-scenario-engine-specification.md), Gate 4 closed | Implemented; TASK-10H / TASK-11 evidence |
| M05 Chaos | Future boundary | Not implemented |
| M06 Economic Regression | [Frozen](design/economic-regression-engine-specification.md), Gate 5 closed | Source and tests present; implementation authorization record requires reconciliation |
| M07 Payment & Settlement Testing | [Frozen](design/payment-settlement-testing-specification.md), Gate 6 | Bounded implementation recorded by [implementation master](design/m07-implementation-master-specification.md), TASK-37…45 |
| M08 x402 Adapter | [Frozen](design/x402-adapter-specification.md), Gate 7 | Bounded inbound semantic implementation recorded by [implementation master](design/m08-implementation-master-specification.md), TASK-63…67 |
| M09 Multi-Agent World | Future boundary | Not implemented |

A frozen specification may contain a historical “BLOCKED” or “NOT AUTHORIZED” marker that reflects the state when the specification was frozen. Later implementation authority must be supported by a separate accepted implementation task/master record. Do not infer authorization from source code alone. M06's authorization evidence remains an explicit open reconciliation item.

## Architecture and engineering

- [Architecture Baseline](architecture/architecture-baseline.md) — module boundaries and dependency direction.
- [Implementation Status Ledger](implementation-status.md) — current evidence and grant-readiness gates.
- [ADR index](adr/README.md) — durable architecture decisions.
- [Specification Policy](design/SPECIFICATION-POLICY.md) — semantic change control.

## Product and funding strategy

All business documents are non-normative hypotheses. They do not modify frozen product scope or authorize interfaces/features.

| Document | Status | Purpose |
|---|---|---|
| [Product Strategy](business/product-strategy.md) | Draft | Product positioning, target user, boundaries, and validation |
| [Funding Strategy](business/funding-strategy.md) | Draft | Hybrid public-good, grant, and commercial funding |
| [Grant Proposal](business/grant-proposal.md) | Draft — not submission-ready | Six-month public-good work plan and planning budget |
| [Commercial Roadmap](business/commercial-roadmap.md) | Draft | Developer validation, paid assessments, and staged commercialization |

## Grant readiness

The repository is not yet grant-submission-ready. The blocking items are explicit: license and contributor-rights decision, private vulnerability-reporting channel, M06 authorization reconciliation, post-merge CI evidence on the exact main baseline, reproducible external demo, external validation, and funder-specific eligibility/budget review. Track these in [Implementation Status Ledger](implementation-status.md).

Business planning documents must not claim that grant funds are awarded, that a funder endorses the project, or that a future feature already exists.
