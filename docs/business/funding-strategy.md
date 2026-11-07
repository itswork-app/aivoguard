# AivoGuard Funding Strategy

| Field | Value |
|---|---|
| Status | **DRAFT — strategic planning, non-normative** |
| Model | Hybrid: open-source public good + grants + commercial revenue |
| Governance direction | Company-led |
| Initial grant planning envelope | USD 30,000 over six months |
| Review trigger | Before any external funding submission |

## 1. Purpose

This document defines a funding and sustainability strategy for AivoGuard. It does not amend the frozen product scope, authorize implementation, select a software license, or make a commitment to any grant provider.

AivoGuard's product position remains the frozen product-scope statement:

> Deterministic economic testing infrastructure for autonomous AI agents.

The funding thesis is to make the essential deterministic economic testing foundation openly inspectable and reusable, while earning revenue from operational convenience, scale, deployment, integration, and support.

## 2. Funding model

### 2.1 Open-source public-good layer

Intended public-good assets, subject to the repository's specification and implementation gates:

- Deterministic economic kernel and invariant evaluation capabilities.
- Reproducible testing and regression foundations.
- Basic developer tooling and documented interfaces, once specified and authorized.
- Public reference scenarios and benchmark methodology.
- Core technical documentation and examples.

The essential ability to understand and verify economic test outcomes must not depend on a paid subscription.

**License gate:** The repository license is currently undecided. No proposal may claim that the repository is already licensed under Apache-2.0, MIT, or another license. A license decision, rights review, and repository update are prerequisites to a public open-source funding claim.

### 2.2 Grant-funded work

Grant funding should support clearly scoped public deliverables, such as:

- Correctness, determinism, and regression work.
- Independent security and correctness review.
- Public scenario and benchmark development.
- Documentation and developer onboarding.
- Ecosystem-specific reference adapters only where a frozen specification authorizes them and the grant ecosystem receives concrete value.

Grant-funded work must be tracked separately by scope, milestone, expense, and evidence. Grant terms take precedence for the funds awarded. Commercial features must not be charged to a public-good grant unless the funder explicitly permits that scope.

### 2.3 Commercial layer

Potential commercial offerings, subject to customer validation and later product specifications:

- Hosted execution and team workspaces.
- Private deployment and controlled execution environments.
- Advanced evidence, reporting, retention, and collaboration.
- Enterprise access controls, operational support, and service commitments.
- Custom integrations, migration, and economic safety assessments.

These are commercial hypotheses, not currently implemented capabilities or approved technical contracts.

## 3. Company-led governance

The company is the intended commercial operator and steward of the AivoGuard product. It is responsible for:

- Product strategy, roadmap, brand, customer relationships, and commercial contracts.
- Budgeting, financial controls, and grant compliance.
- Release operations and support commitments.
- Managing commercial intellectual property and service delivery.

The open-source project should retain transparent contribution and release practices:

- Public issue and pull-request process.
- Documented technical decisions and release notes.
- Clear separation between normative specifications and non-normative business plans.
- Security disclosure process before production release.
- Explicit license and contributor-rights policy before accepting broad external contributions.

Company-led does not mean grant-funded public assets may be made unavailable contrary to grant terms. Any future foundation or independent governance structure is deferred until community scale and ecosystem neutrality create a concrete need.

## 4. Funding sequence

1. **Bootstrap:** stabilize the repository, reconcile documentation, close correctness findings, and establish a reproducible demonstration.
2. **Grant readiness:** choose the license, prepare a scoped public-good work plan, budget, milestones, and evidence package.
3. **Design partners:** validate the economic safety problem with at least three external developer teams.
4. **Paid pilots:** sell scoped integration assessments or pilot engagements before building a full hosted service.
5. **Recurring revenue:** introduce hosted/team features only after repeat usage and willingness to pay are demonstrated.
6. **Institutional capital:** consider an accelerator or pre-seed only after product usage, retention, and a credible commercial wedge are evidenced.

VC fundraising is not a near-term dependency.

## 5. Grant portfolio strategy

Grant applications must be ecosystem-specific and truthful.

- **Ethereum ecosystem:** apply only when the funded work has a concrete, measurable benefit for Ethereum developers or infrastructure.
- **Solana ecosystem:** apply only for a scoped Solana-related adapter, scenario set, or benchmark with demonstrated ecosystem relevance.
- **Public-goods funding:** emphasize reusable open tooling, transparent methodology, documentation, and broad developer access.
- **x402 ecosystem:** pursue technical collaboration and integration opportunities; do not imply endorsement, partnership, or grant availability without written confirmation.

One technical core can support multiple applications, but each application must have distinct ecosystem outcomes, milestones, and eligible costs. Never submit the same generic proposal with only the ecosystem name changed.

## 6. Financial controls

Maintain three separate internal cost/revenue categories:

1. Restricted grant funds and grant-funded deliverables.
2. Company operating expenses and commercial product development.
3. Customer revenue, including subscriptions, pilots, assessments, and support.

For every grant-funded expense, retain the invoice or proof of payment, category, milestone mapping, and approval record. Do not count a grant award as recurring commercial revenue or assume it is unrestricted company cash.

## 7. Planning assumptions

The initial USD 30,000 / six-month envelope is a proposal-planning assumption, not a confirmed award, market benchmark, or approved company budget. Validate contributor capacity, local tax/accounting treatment, funder eligibility, and allowable costs before submission.

Indicative allocation:

| Category | Planning amount |
|---|---:|
| Core engineering and correctness | USD 12,000 |
| Independent security/correctness review | USD 6,000 |
| Developer integration work | USD 4,500 |
| Documentation and onboarding | USD 3,000 |
| CI and test infrastructure | USD 1,500 |
| Pilot/community support | USD 1,500 |
| Contingency | USD 1,500 |
| **Total** | **USD 30,000** |

## 8. Preconditions and decision gates

No external grant submission should be made until the following are satisfied:

- Repository implementation status is independently re-audited and documentation reconciled.
- Required CI checks pass on the proposed submission baseline.
- Known correctness findings are resolved or explicitly disclosed with a remediation plan.
- License and contributor-rights questions are resolved.
- Grant-funded scope is compatible with frozen specifications and authorized implementation gates.
- A demo and evidence package are reproducible from a clean environment.
- Budget, milestones, and reporting obligations are reviewed for the selected funder.

## 9. Metrics

Track outcomes rather than vanity metrics:

- Milestones accepted by the funder.
- Public releases and reproducible test scenarios delivered.
- External teams completing a test workflow.
- Repeat usage and integration retention.
- Paid pilot conversion and expansion.
- Grant-funded costs reconciled to eligible deliverables.
- Correctness issues discovered, fixed, and prevented by regression tests.

## 10. Non-goals

This strategy does not authorize a token, foundation formation, production payment authority, custody, a generic agent orchestration product, or a full hosted SaaS build before validation.
