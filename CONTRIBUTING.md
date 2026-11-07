# Contributing to AivoGuard

Thank you for your interest in contributing.

AivoGuard has implemented M01–M04 and M06–M08 within bounded scopes. M05 and M09 are not implemented. Contributions must respect the specification-first process and the current implementation status ledger.

## Before you start

1. Read [`README.md`](README.md) for current project status.
2. Read [`docs/design/SPECIFICATION-POLICY.md`](docs/design/SPECIFICATION-POLICY.md).
3. Read [`GOVERNANCE.md`](GOVERNANCE.md) and [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).
4. Inspect the repository as it is — do not invent missing modules.

## What to contribute right now

Useful contributions:

* Documentation clarity (without inventing product requirements)
* Correctness fixes and regression tests for implemented modules
* CI / tooling / repository hygiene fixes
* Spec or ADR drafts that are explicitly marked as drafts
* Bug fixes in existing foundation files

Not acceptable without the applicable frozen specification and explicit implementation authorization:

* New economic primitives or semantic changes
* New simulator, invariant, or adversarial semantics
* CLI / SDK / public API / live payment adapters
* M05 chaos or M09 multi-agent modules
* Speculative crates or “helpful” domain APIs

If a change requires new semantics, open a specification discussion first.

## Development setup

1. Install a stable Rust toolchain (`rust-toolchain.toml` pins `stable` with
   `rustfmt` and `clippy`).
2. Clone the repository.
3. From the repository root:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --workspace --all-targets -- -D warnings
```

All four must pass before opening a pull request.

## Coding standards

* Follow Rust 2021 edition conventions used in this workspace.
* Do not add dependencies unless a frozen specification requires them.
* Do not introduce async runtimes, databases, network stacks, blockchain
  crates, LLM/AI SDKs, or web frameworks unless a frozen specification and
  accepted implementation task explicitly require them.
* Prefer deterministic, explicit behavior. Hidden mutable global state is
  prohibited as a project principle.
* Tests are required for any implemented behavior.
* Do not use LLM output as authoritative economic truth.

## Specification and semantic changes

* Code must not silently redefine a specification.
* Ambiguity must be surfaced, not silently resolved.
* Semantic changes require an explicit specification amendment and, where
  applicable, an ADR.
* Cursor / AI assistants must not invent requirements.

## Pull requests

Use the pull request template. Include:

* Summary and motivation
* Exact changes
* Tests run
* Documentation updates
* Breaking-change assessment
* Specification impact

Keep PRs focused. Prefer small, reviewable changes.

## Issues

Use the issue templates when they apply. For specification questions, mark
ambiguity clearly rather than proposing silent resolutions.

## License and contributor rights

AivoGuard is distributed under the [Apache License, Version 2.0](LICENSE) (`Apache-2.0`). Read the license before using, modifying, or redistributing the project.

By intentionally submitting a contribution for inclusion in AivoGuard (including through a pull request, issue, or other project-managed contribution channel), you agree that the contribution is submitted under Apache-2.0 as described in Section 5, unless you clearly and explicitly state otherwise before the contribution is accepted. No separate Contributor License Agreement (CLA) is currently required.

You retain copyright in your original contributions. You must have the legal right to submit the contribution under these terms. Do not submit code, documentation, data, or other material copied from third parties unless its license permits inclusion and redistribution under the project’s licensing approach. Disclose third-party origin, applicable license, required notices, and any attribution in the pull request. Mark material that is not intended as a contribution clearly and in writing.

The repository's visible commit history has been reviewed for provenance, but this is not a complete legal audit of every source file or third-party right. Maintainers may request additional provenance or permission evidence before accepting a contribution.

## Code of conduct

Participation is governed by [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).
