# AivoGuard

**AivoGuard is deterministic economic testing infrastructure for autonomous AI agents.**

It helps developers model economic state, execute deterministic test scenarios, evaluate explicit economic invariants, generate adversarial inputs, compare regression expectations, and test bounded payment/settlement observations before connecting an agent to real economic systems.

> **Product promise:** test economic behavior before real money moves.

AivoGuard is a testing and verification toolchain. It is **not** a production payment processor, wallet, custody service, transaction signer, live payment facilitator, agent orchestrator, or a formal security certification.

## Current repository status

This repository is beyond bootstrap: it contains implemented Rust modules for M01–M04 and M06–M08. M05 (chaos/fault injection) and M09 (multi-agent economic world) are not implemented.

The exact implementation boundary, authorization evidence, test coverage, and known limitations are maintained in the [Implementation Status Ledger](docs/implementation-status.md). Frozen normative specifications remain the authority for semantics; this ledger reports subsequent implementation status without changing those semantics.

| Module | Current status | Rust source | Integration test |
|---|---|---|---|
| M01 Economic Kernel | Implemented; deterministic economic authority | `src/kernel/` | `m01_kernel.rs` |
| M02 Invariant Engine | Implemented; read-only evaluation | `src/invariant/` | `m02_invariant.rs` |
| M03 Deterministic Simulator | Implemented; scenario orchestration | `src/simulator/` | `m03_simulator.rs` |
| M04 Adversarial Scenario Engine | Implemented; candidate scenario generation | `src/adversarial/` | `m04_adversarial.rs` |
| M05 Chaos | Not implemented | — | — |
| M06 Economic Regression | Source and tests present; authorization record requires reconciliation | `src/regression/` | `m06_regression.rs` |
| M07 Payment & Settlement Testing | Implemented and frozen for bounded current scope | `src/payment_settlement/` | `m07_payment_settlement.rs` |
| M08 x402 Adapter | Implemented and frozen for bounded inbound semantic scope | `src/x402_adapter/` | `m08_x402_adapter.rs` |
| M09 Multi-Agent World | Not implemented | — | — |

## Product architecture

```text
EconomicWorld + EconomicState + Action + ExecutionContext
                         |
                         v
                 M01 Economic Kernel
             authoritative economic outcome
                         |
             +-----------+-----------+
             |                       |
             v                       v
       M02 Invariants          M03 Simulator
       read-only checks        deterministic orchestration
             ^                       ^
             |                       |
             +-----------+-----------+
                         |
                  M04 Adversarial
                  candidate scenarios

M06 Regression: compares declared expectations with outputs.
M07 Payment/Settlement: deterministic testing of declared observations.
M08 x402: bounded inbound semantic translation; not live verification.
```

The M01 kernel is the economic authority. M02 is read-only. M03 orchestrates; it does not reimplement economics. M04 generates candidate inputs, not economic truth. M06–M08 remain bounded to their own specifications.

## What AivoGuard can and cannot claim

**Implemented foundations**
- Integer-based economic amounts and deterministic ordered collections.
- Deterministic economic state transitions and structured outcomes.
- Read-only invariant evaluation with explicit PASS / FAIL / ERROR semantics.
- Deterministic simulation and adversarial scenario generation.
- Regression expectation comparison.
- Payment/settlement test evaluation and a bounded x402 semantic adapter.
- CI checks for formatting, compilation, tests, and strict Clippy.

**Not currently provided**
- Production enforcement or interception of agent tool calls.
- ALLOW / DENY / REQUIRE_APPROVAL runtime policy enforcement.
- Live x402 HTTP/wire processing, cryptographic proof verification, facilitator connectivity, or outbound payment.
- Wallets, custody, signing, payment execution, or settlement authority.
- CLI, public SDK, hosted API, hosted dashboard, or enterprise control plane.
- M05 chaos injection or M09 multi-agent world.

Do not interpret test outcomes as guarantees that all real-world losses or attacks are prevented.

## Quick start

Requirements: stable Rust toolchain specified by `rust-toolchain.toml`.

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo run --example deterministic_transfer
```

The example is a deterministic local library demonstration, not a live payment or provider integration. Expected output:

```text
disposition: AcceptedEffective
effects: 2
alice USD minor units: 8500
bob USD minor units: 1500
```

The current crate is a Rust library workspace. A stable, supported public CLI or SDK is not yet part of the product contract.

## Documentation

- [Implementation Status Ledger](docs/implementation-status.md) — authoritative current module and gate status.
- [Product Strategy](docs/business/product-strategy.md) — positioning, customer wedge, product boundaries, and commercial hypotheses.
- [Funding Strategy](docs/business/funding-strategy.md) — hybrid public-good and commercial model.
- [Grant Proposal Draft](docs/business/grant-proposal.md) — planning draft; not submission-ready.
- [Commercial Roadmap](docs/business/commercial-roadmap.md) — validation and staged monetization.
- [Architecture Baseline](docs/architecture/architecture-baseline.md).
- [Specification Policy](docs/design/SPECIFICATION-POLICY.md).
- [Documentation Index](docs/README.md).

## Development and governance

AivoGuard follows specification-first development. Frozen specifications define normative semantics; accepted ADRs constrain implementation. Changes that alter economic meaning require an explicit specification amendment before code and tests are changed.

See [CONTRIBUTING.md](CONTRIBUTING.md), [GOVERNANCE.md](GOVERNANCE.md), and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Security

This repository is a pre-production testing library. Do not use it as a production security boundary. See [SECURITY.md](SECURITY.md) for current reporting limitations. A private vulnerability-reporting channel and response process must be established before a public production release or a grant claim that promises coordinated vulnerability handling.

## License and grant readiness

**No open-source license has been selected.** The repository is not yet granted Apache-2.0, MIT, or any other redistribution license. Do not treat it as an already licensed open-source release. License selection and contributor-rights review are explicit pre-submission gates; see [LICENSE](LICENSE) and the implementation/grant readiness ledger.

Grant documents are planning drafts only. No funder endorsement, award, partnership, eligibility, or product-market fit is implied.
