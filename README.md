# AivoGuard

**Status: repository foundation / bootstrap stage.**

AivoGuard is intended to become deterministic economic testing infrastructure
for autonomous AI agents — a deterministic economic sandbox, invariant engine,
adversarial scenario framework, and regression-testing system for agents that
can take economically consequential actions.

**None of that product scope is implemented yet.**

This repository currently provides only an open-source Rust workspace
foundation suitable for subsequent specification-driven development.

## What exists today

* Cargo workspace with a minimal anchor crate (`crates/aivoguard`)
* Formatting, Clippy, and CI validation for the foundation
* Documentation layout for design specs, architecture, reference material, and ADRs
* Contribution, governance, security-reporting, and code-of-conduct stubs
* Specification authority policy (`docs/design/SPECIFICATION-POLICY.md`)

## What does **not** exist yet

* Economic simulator / economic kernel
* Invariant engine
* Adversarial scenario framework
* CLI, SDK, or agent adapters
* Payment adapters (including x402 or similar)
* Production APIs or network services
* Gate 0 / Gate 1 specifications
* Implemented economic modules

The frozen product-scope specification is
[`docs/design/product-scope.md`](docs/design/product-scope.md)
(**FROZEN**). Claiming implemented product behavior would be dishonest.

## Technical direction (informational)

These statements describe intended direction for later gates. They are **not**
implemented contracts:

* Rust for the deterministic core
* Exact / controlled arithmetic for economic values
* Deterministic, reproducible execution and tests
* CLI-first developer experience
* OSS-first repository
* LLMs are never authoritative over economic state or economic truth

## Repository layout

```text
aivoguard/
├── .github/          # CI, PR template, issue templates
├── .cursor/rules/    # Cursor agent constraints for this repo
├── docs/             # Design, architecture, reference, ADRs
├── crates/           # Rust workspace members
├── examples/         # Examples (none yet; reserved)
└── tests/            # Workspace-level integration tests (none yet; reserved)
```

See [`docs/README.md`](docs/README.md) for documentation conventions.

## Repository

Canonical remote: [`itswork-app/aivoguard`](https://github.com/itswork-app/aivoguard)

```bash
git clone git@github.com:itswork-app/aivoguard.git
```

## Build and validate

Requires a stable Rust toolchain with `rustfmt` and `clippy`
(see `rust-toolchain.toml`).

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --workspace --all-targets -- -D warnings
```

CI runs the same foundation checks.

## License

**License undecided.** See [`LICENSE`](LICENSE). Do not assume redistribution
terms until a license decision is recorded.

## Contributing

Read [`CONTRIBUTING.md`](CONTRIBUTING.md), [`GOVERNANCE.md`](GOVERNANCE.md),
and [`docs/design/SPECIFICATION-POLICY.md`](docs/design/SPECIFICATION-POLICY.md)
before proposing changes.

Specification-first: do not invent product requirements or domain modules
ahead of frozen specs.

## Security

See [`SECURITY.md`](SECURITY.md). A dedicated vulnerability-reporting contact
has not been configured yet and will be established before any production
release.

## Specifications

* Process: [`docs/design/SPECIFICATION-POLICY.md`](docs/design/SPECIFICATION-POLICY.md)
* Product scope (**FROZEN**): [`docs/design/product-scope.md`](docs/design/product-scope.md)

## Next steps

**TASK-02 — Domain Contract / Gate 0** is authorized to begin.
Gate 0 remains blocked until TASK-02 completes. Gate 1 remains blocked.
Do not treat this README as authorization to implement economic modules.
