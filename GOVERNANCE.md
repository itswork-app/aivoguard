# Governance

## Current status

AivoGuard has implemented deterministic economic testing modules, but remains a pre-production project. Formal governance structures (roles, voting, membership, release authority) are intentionally minimal and company-led during this stage.

This document records the operating principles that apply until a richer
governance model is adopted.

## Principles

1. **Specification authority** — Frozen normative specifications outrank
   implementation convenience. See
   [`docs/design/SPECIFICATION-POLICY.md`](docs/design/SPECIFICATION-POLICY.md).
2. **No silent invention** — Product requirements, economic semantics, and
   architecture must not be invented in code or by tooling assistants without
   documented specification or ADR review.
3. **Honesty over momentum** — Documentation must not claim features that do
   not exist.
4. **Determinism as a project principle** — Design and implementation choices
   must preserve deterministic, reproducible behavior required by the frozen economic and testing specifications.
5. **Security before production** — Vulnerability reporting and release
   controls must exist before any production release.

## Roles (bootstrap)

| Role | Responsibility |
| --- | --- |
| Maintainers | Repository stewardship, review of PRs, specification and implementation-gate integrity |
| Contributors | Propose changes via PRs/issues per `CONTRIBUTING.md` |

Named maintainer roster, committer election, and decision quorums are **not**
defined yet. They will be documented when the project begins accepting broader
external participation.

## Decision types

| Decision | Required artifact |
| --- | --- |
| Normative product/semantic rules | Frozen specification under `docs/design/` |
| Architecture / compatibility / determinism / API surface | ADR under `docs/adr/` (see ADR README) |
| License selection | Replace `LICENSE` placeholder; update metadata |
| Security contact / process | Update `SECURITY.md` before production release |
| Routine foundation fixes | PR review only |

## Conflict resolution

During the current pre-production stage, unresolved disagreements must be recorded as open questions in issues or draft specifications rather than resolved silently in code. Normative semantics require a specification amendment; implementation authorization requires an explicit task record.

## Amendments

Changes to this governance document require a pull request and maintainer
review. Substantive governance expansion (voting, membership, trademark, etc.)
should be accompanied by an ADR or dedicated governance proposal.
