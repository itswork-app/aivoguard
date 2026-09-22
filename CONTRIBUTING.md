# Contributing to AivoGuard

Thank you for your interest in contributing.

AivoGuard is in **foundation / bootstrap** stage. Most product behavior does
not exist yet. Contributions must respect the specification-first process.

## Before you start

1. Read [`README.md`](README.md) for current project status.
2. Read [`docs/design/SPECIFICATION-POLICY.md`](docs/design/SPECIFICATION-POLICY.md).
3. Read [`GOVERNANCE.md`](GOVERNANCE.md) and [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).
4. Inspect the repository as it is — do not invent missing modules.

## What to contribute right now

Useful foundation-stage contributions:

* Documentation clarity (without inventing product requirements)
* CI / tooling / repository hygiene fixes
* Spec or ADR drafts that are explicitly marked as drafts
* Bug fixes in existing foundation files

Not acceptable without a frozen specification:

* Economic primitives or simulators
* Invariant engines or adversarial frameworks
* CLI / SDK / payment adapters
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
  crates, LLM/AI SDKs, or web frameworks during foundation work.
* Prefer deterministic, explicit behavior. Hidden mutable global state is
  prohibited as a project principle.
* Tests are required for any implemented behavior.
* Do not use LLM output as authoritative economic truth (when economic
  semantics exist later).

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

## License

The project license is **undecided** (see [`LICENSE`](LICENSE)). By
contributing, you agree that your contributions may be relicensed under the
license eventually selected by the project maintainers for this repository.
If that is unacceptable, do not contribute until a license is chosen.

## Code of conduct

Participation is governed by [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).
