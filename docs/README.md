# Documentation

This directory holds AivoGuard's version-controlled documentation.

## What belongs in `docs/`

| Path | Purpose |
| --- | --- |
| `docs/design/` | Normative and draft **design specifications** — the intended contracts for behavior and semantics |
| `docs/architecture/` | Architecture documents describing system structure, boundaries, and component relationships |
| `docs/reference/` | Non-normative reference material (glossaries, guides, explanatory notes) |
| `docs/adr/` | Architecture Decision Records for significant, durable decisions |

Do not place build artifacts, generated binaries, or secrets here.

## How these differ

* **Design specifications** define *what must be true*. When frozen, they are
  normative. Implementation must conform to them.
* **Architecture documents** explain *how the system is structured* to satisfy
  specifications. They do not silently replace normative specs.
* **Reference material** helps humans understand the project. It is
  non-normative unless a document explicitly says otherwise.
* **ADRs** record *why a significant decision was made*, including context and
  consequences. See [`adr/README.md`](adr/README.md).

## Specification authority

1. Normative specifications are **version-controlled** in this repository.
2. Implementation must follow **frozen** specifications.
3. Undocumented semantic changes are **prohibited**.
4. Discoveries that require semantic change must trigger a specification
   amendment — not a silent code-only fix.

Full hierarchy and process rules:
[`design/SPECIFICATION-POLICY.md`](design/SPECIFICATION-POLICY.md).

## Current status

| Document | Status |
| --- | --- |
| [`design/SPECIFICATION-POLICY.md`](design/SPECIFICATION-POLICY.md) | Process policy (normative for process) |
| [`design/product-scope.md`](design/product-scope.md) | Product scope — **FROZEN** |
| Gate 0 / Domain Contract (TASK-02) | Authorized to begin; Gate 0 blocked until complete |
| Gate 1 / economic semantics | Blocked |

Empty architecture/reference directories remain intentional placeholders for
upcoming specification work.
