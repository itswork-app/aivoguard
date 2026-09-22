# AivoGuard Product Scope Specification

| Field | Value |
| --- | --- |
| Document | `docs/design/product-scope.md` |
| Task | TASK-01 |
| **STATUS** | **FROZEN** |
| Normative freeze | **FROZEN** — approved after review |
| Freeze authority | Explicit approval recorded 2026-09-22 |
| Implementation | **BLOCKED** until later gates authorize it |
| Gate 0 | **BLOCKED** until TASK-02 (Domain Contract) is completed |
| Gate 1 | **BLOCKED** |

This document is the **frozen** normative product-scope contract for AivoGuard.
Material changes require an explicit specification amendment per §22.
Freeze does **not** authorize domain implementation. Gate 0 remains blocked
until TASK-02 is completed.

---

## 1. Scope

This specification defines:

* product identity and positioning;
* problem statement and value proposition;
* product category;
* primary users and primary job-to-be-done;
* conceptual execution model and developer workflow;
* planned product capabilities and module boundaries (M01–M09);
* economic authority boundary;
* determinism and evidence principles at product level;
* product boundaries and explicit non-goals;
* OSS product principles;
* implementation gate sequence;
* scope-control rules;
* specification hierarchy and amendment policy;
* open decisions and deferred semantics.

This specification does **not** define:

* exact domain types or economic accounting rules;
* numeric representation details;
* APIs, CLIs, SDKs, serialization formats;
* simulator, invariant language, or adversarial model internals;
* payment-provider or x402 protocol semantics;
* production integrations.

Those belong to later specifications (starting with TASK-02 / Domain Contract).

---

## 2. Product identity

### Product

**AivoGuard**

### Primary positioning

> AivoGuard is deterministic economic testing infrastructure for autonomous AI agents.

### Technical positioning

> AivoGuard is a deterministic economic sandbox, invariant engine, adversarial scenario framework, and regression-testing system for autonomous agents that can take economically consequential actions.

### Core value proposition

> Test the economics of autonomous agents before they touch real money.

---

## 3. Problem statement

Autonomous AI agents can make economically consequential decisions involving
money, assets, payments, transfers, purchases, sales, refunds, withdrawals,
deposits, allocation, pricing, settlement, multiple counterparties, and other
economic resources.

Traditional application testing may verify that code executes correctly while
failing to determine whether the resulting **economic state** is valid.

AivoGuard exists to test the **economic consequences** of autonomous agent
behavior before that behavior reaches real economic systems.

---

## 4. Product category

AivoGuard is:

```text
Economic Testing Infrastructure
```

More specifically:

```text
Deterministic Economic Simulation
+
Economic Invariant Testing
+
Adversarial Economic Testing
+
Economic Regression Testing
```

AivoGuard is a **developer infrastructure / testing** product.

AivoGuard is **not** itself an economic authority in production.

---

## 5. Primary users

The primary user is a developer or engineering team building autonomous systems
that can cause economically consequential state changes.

User categories (not separate products) include:

* AI-agent developers
* autonomous-agent platform developers
* fintech developers
* payment-system developers
* Web3 developers
* trading-system developers
* marketplace developers
* autonomous commerce developers
* infrastructure engineers
* protocol developers
* researchers building economically active agents

AivoGuard must not invent separate product architectures for each category.

---

## 6. Primary job-to-be-done

> Before allowing an autonomous agent to interact with a real economic system, a developer must be able to deterministically execute the agent's economically consequential behavior against a controlled economic environment and verify that defined economic invariants remain valid.

---

## 7. Core execution model

Canonical conceptual execution model:

```text
EconomicState(t)
        +
Agent Action(t)
        ↓
Economic Transition
        ↓
EconomicState(t+1)
        ↓
Invariant Evaluation
        ↓
PASS / FAIL
```

This is a **conceptual product contract**.

Detailed transition, state, and invariant semantics belong to the Domain
Contract and Economic Kernel specifications. This document does not implement
or fully define those semantics.

---

## 8. Core workflow

At the product level, AivoGuard supports this normative developer workflow:

```text
DEFINE
    ↓
CONNECT
    ↓
TEST
    ↓
ATTACK
    ↓
VERIFY
    ↓
REGRESS
    ↓
CI
```

Exact APIs and implementation mechanisms are deferred.

---

## 9. Product capabilities

Planned product capabilities are organized as modules M01–M09 below.

Module identifiers define **product boundaries**, not crate names, APIs, or
implementation schedules beyond the gate sequence in this document.

---

## 10. Module boundaries (M01–M09)

### M01 — Economic Kernel

Responsible for deterministic economic state and state transitions.

High-level responsibility:

* represent economic state
* apply economically consequential actions
* produce deterministic transitions
* preserve economic authority boundaries

Detailed economic semantics are **not** defined by this document.

### M02 — Economic Invariant Engine

Responsible for expressing and evaluating properties that must remain true.

Conceptual invariant categories (illustrative, not an exhaustive language):

* balance conservation
* conservation of assets
* settlement correctness
* authorization-related economic constraints
* fee correctness
* liability consistency
* supply constraints
* collateral constraints
* accounting consistency

Exact invariant language and implementation are deferred.

### M03 — Deterministic Economic Simulator

Responsible for executing controlled economic scenarios reproducibly.

Conceptually:

```text
World
+
Initial State
+
Scenario
+
Action Sequence
+
Configuration
+
Seed
+
Engine Version
=
Deterministic Simulation Result
```

Detailed simulation semantics are deferred.

### M04 — Adversarial Scenario Engine

Responsible for testing economically hostile or abnormal conditions.

Conceptual categories include:

* malicious actions
* invalid sequences
* extreme prices
* fee manipulation
* timing manipulation
* liquidity stress
* counterparty failures
* unexpected responses
* economically adversarial agent behavior

Exact adversarial model is deferred.

### M05 — Economic Chaos Engine

Responsible for controlled economic fault injection.

Conceptual examples:

* delayed settlement
* failed settlement
* unavailable liquidity
* stale price
* missing response
* partial execution
* counterparty failure
* infrastructure fault affecting economic execution

Chaos testing must remain deterministic/reproducible when configured for
deterministic execution.

### M06 — Economic Regression Testing

Responsible for preserving previously verified economic behavior.

Conceptual workflow:

```text
Known Economic Scenario
        ↓
Execute
        ↓
Record Expected Result
        ↓
Future Change
        ↓
Replay
        ↓
Detect Economic Regression
```

### M07 — Payment & Settlement Testing

Responsible for testing economic behavior around payment and settlement flows.

May eventually include:

* payments
* refunds
* settlement
* escrow
* fees
* failed payments
* partial settlement
* delayed settlement

Payment-provider integrations are **out of scope for current implementation**
and are not authorized by this document alone.

### M08 — x402 Adapter

AivoGuard may provide an adapter layer for testing economic behavior involving
x402-style payment flows.

The adapter must remain an **integration boundary**.

It must **not** redefine AivoGuard's economic authority.

Detailed x402 semantics and integration requirements are deferred.

### M09 — Multi-Agent Economic World

Responsible for testing multiple economically interacting agents.

Conceptual participants may include:

* autonomous agents
* merchants
* customers
* liquidity providers
* counterparties
* protocols
* service providers

Multi-agent economics must remain deterministic where deterministic execution
is required. Detailed multi-agent semantics are deferred.

---

## 11. Economic authority model

AivoGuard maintains a strict separation between:

```text
ECONOMIC TRUTH
```

and

```text
AI-GENERATED INTERPRETATION
```

The **deterministic engine** is the economic authority inside AivoGuard's
testing model.

External AI/LLM systems may eventually assist with:

* generating candidate actions
* generating scenarios
* generating adversarial cases
* interpreting human-readable requirements
* explaining failures

An LLM must **NOT** become the authoritative source for:

* balances
* transaction validity
* state transitions
* settlement correctness
* invariant results
* PASS / FAIL decisions
* canonical economic state

This boundary is fundamental to the product.

---

## 12. Determinism principle

Determinism is a core product property.

Under an equivalent execution configuration:

```text
same world
+
same initial state
+
same scenario
+
same action sequence
+
same configuration
+
same seed
+
same engine version
```

AivoGuard should produce equivalent:

```text
state transitions
events
transaction outcomes
invariant results
violations
```

Exact determinism semantics belong to the Domain Contract.

Additionally, as a product scope-control rule: binary floating-point
representation must **not** be used as authoritative economic truth.

---

## 13. Evidence model

AivoGuard is not merely an execution engine. A test must produce inspectable
evidence.

Conceptually, evidence should allow a developer to determine:

```text
WHAT happened?
WHEN did it happen?
WHICH action caused it?
WHAT was the state before?
WHAT was the state after?
WHICH invariant was evaluated?
WHAT was expected?
WHAT was observed?
WHY did it fail?
CAN it be reproduced?
```

Exact evidence schema belongs to the Domain Contract.

---

## 14. Product boundaries

### AivoGuard is

```text
Testing infrastructure
Simulation infrastructure
Economic invariant testing
Adversarial economic testing
Economic regression testing
Controlled economic experimentation
```

### AivoGuard is NOT

```text
A bank
A payment processor
An exchange
A wallet provider
A lending platform
An accounting system
A production settlement authority
A generic cybersecurity platform
A generic AI governance platform
A generic IAM system
A generic observability platform
A generic LLM evaluation platform
A generic chatbot
A generic AI-agent orchestration platform
A generic workflow automation platform
```

These exclusions are intentional.

---

## 15. Production authority boundary

AivoGuard must not silently become the production economic authority of a
customer's system.

Adapters and integrations must translate between external systems and
AivoGuard's testing model.

They must not silently redefine:

* economic state
* transaction meaning
* invariant semantics
* PASS / FAIL authority

Production integration is therefore **downstream** of the deterministic testing
model.

### Testing vs production

Primary role:

```text
TEST BEFORE PRODUCTION
```

not:

```text
BECOME THE PRODUCTION ECONOMIC SYSTEM
```

A production system may use AivoGuard to test behavior before deployment.

AivoGuard may eventually observe or replay production-derived scenarios where
explicitly supported; that does **not** make AivoGuard the production source of
economic truth.

---

## 16. Explicit non-goals

The following are non-goals for AivoGuard as a product (and remain blocked for
implementation until separately specified and authorized):

* acting as a bank, payment processor, exchange, wallet, lender, or accounting
  system of record in production;
* becoming a customer's production settlement authority;
* generic cybersecurity, IAM, observability, LLM-evaluation, chatbot,
  agent-orchestration, or workflow-automation product scope;
* LLM authority over economic truth, balances, transitions, settlement
  correctness, invariant results, or PASS/FAIL;
* silent expansion into production economic systems via adapters;
* inventing APIs, CLIs, SDKs, crates, or integrations ahead of frozen
  contracts for those surfaces;
* using binary floating-point as authoritative economic truth;
* requiring proprietary hidden behavior for the core deterministic testing
  model.

---

## 17. Developer experience

Intended developer experience:

```text
1. Define economic model
2. Define invariants
3. Connect agent behavior
4. Run deterministic scenarios
5. Attack the economic model
6. Inspect evidence
7. Preserve important cases
8. Run regressions in CI
```

Exact CLI, SDK, configuration format, API surface, and language bindings are
**not** decided by this document.

---

## 18. Open-source product principles

AivoGuard is intended as serious open-source infrastructure.

Product specifications must favor:

* reproducibility
* transparency
* deterministic behavior
* inspectable evidence
* testability
* explicit versioning
* stable contracts
* minimal hidden behavior
* clear extension points
* documented architectural decisions
* contributor-friendly development
* machine-verifiable behavior

No proprietary hidden behavior should be required for the core deterministic
testing model.

**License is not chosen by this document.** The repository currently has a
pending license decision (`LICENSE`).

---

## 19. Implementation gate sequence

Planned sequence:

```text
TASK-00  Repository Foundation
        ↓
TASK-01  Product Scope Specification   ← this document
        ↓
TASK-02  Domain Contract / Gate 0
        ↓
GATE 1   Economic Kernel
        ↓
GATE 2   Economic Invariant Engine
        ↓
GATE 3   Deterministic Simulator
        ↓
GATE 4   Scenario / Adversarial Engine
        ↓
GATE 5   Regression Engine
        ↓
GATE 6   Payment / Settlement Testing
        ↓
GATE 7   x402 Adapter
        ↓
GATE 8   Multi-Agent Economic World
```

No later gate may silently redefine an earlier frozen contract.

---

## 20. Scope-control rules

Mandatory rules:

1. No speculative modules.
2. No implementation before the relevant specification is frozen.
3. No hidden architecture decisions.
4. No semantic requirements inferred from marketing language.
5. No silent expansion of scope.
6. No dependency added merely because it may be useful later.
7. No API invented before its contract is specified.
8. No production integration implemented before its boundary is specified.
9. No LLM authority over deterministic economic truth.
10. No binary floating-point representation as authoritative economic truth.

---

## 21. Specification hierarchy

Authority for normative meaning:

```text
Frozen Normative Specifications
        ↓
ADRs
        ↓
Implementation
        ↓
Tests
        ↓
Examples
        ↓
Non-normative Documentation
```

Code must implement the specification.

Code must not silently redefine the specification.

Tests must verify the specification.

If implementation reveals a semantic contradiction, the contradiction must be
surfaced and resolved through an explicit specification amendment.

Process details: [`SPECIFICATION-POLICY.md`](SPECIFICATION-POLICY.md).

---

## 22. Amendment / versioning policy

Product scope is a controlled contract.

A change affecting any of the following requires an **explicit specification
amendment**:

* product identity
* core purpose
* product boundary
* economic authority
* module scope
* deterministic guarantees
* testing model
* major non-goals

Do not silently edit frozen semantics.

Every material amendment must document:

```text
WHAT changed
WHY it changed
WHAT existing behavior is affected
WHICH specifications depend on it
WHETHER an ADR is required
WHETHER implementation must change
```

This document's status is **`FROZEN`**.
The amendment rules above apply in full to frozen meaning.

---

## 23. OPEN DECISIONS

The following decisions are **unresolved**. This section records them; it does
**not** answer them.

| ID | Decision |
| --- | --- |
| OD-01 | Exact domain model |
| OD-02 | Economic numeric representation |
| OD-03 | Scenario serialization format |
| OD-04 | Invariant definition language / API |
| OD-05 | Simulation execution model |
| OD-06 | Evidence serialization format |
| OD-07 | CLI / API surface |
| OD-08 | Plugin / adapter architecture |
| OD-09 | Language bindings |
| OD-10 | License |

Do not invent answers to these in code, ADRs, or informal docs without an
explicit decision process.

---

## 24. Deferred semantics

Deferred to later specifications (not defined here):

* `EconomicState`, `EconomicWorld`, money/balance/transaction/event types
* arithmetic and numeric representation semantics beyond the floating-point ban
  for authoritative truth
* invariant language and evaluation algorithm
* simulation scheduling, seeding, and replay internals
* adversarial and chaos models
* regression case format and comparison rules
* payment / settlement protocol details
* x402 protocol semantics
* multi-agent interaction semantics
* evidence schema
* CLI / SDK / HTTP / language-binding surfaces
* production adapters and external-system mappings

---

## 25. Relationship to TASK-00

TASK-00 established the repository foundation:

* buildable / testable Rust workspace
* CI, formatting, Clippy policy
* contribution, security, governance docs
* specification process policy
* Cursor foundation rules

TASK-01 does **not** redesign that foundation.

TASK-01 produced this **product-scope** specification on top of TASK-00.
Product scope is **FROZEN**. Implementation remains blocked until later gates
after Domain Contract (TASK-02 / Gate 0) and subsequent authorized work.

---

## 26. Next required task

**TASK-02 — Domain Contract / Gate 0**

**AUTHORIZED TO BEGIN** by explicit project approval after product-scope freeze.

Gate 0 remains **BLOCKED until TASK-02 is completed**.
Gate 1 remains **BLOCKED**.

Do not invent Domain Contract contents without the issued TASK-02 brief.

---

## Document control

| Item | Value |
| --- | --- |
| Created by | TASK-01 |
| Status | **FROZEN** |
| Freeze date | 2026-09-22 |
| Supersedes | None (first product-scope document) |
| Related process | [`SPECIFICATION-POLICY.md`](SPECIFICATION-POLICY.md) |
| Related ADR guide | [`../adr/README.md`](../adr/README.md) |
