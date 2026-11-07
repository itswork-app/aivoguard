# AivoGuard Commercial Roadmap

| Field | Value |
|---|---|
| Status | **DRAFT — commercial planning, non-normative** |
| Horizon | 12 months from roadmap approval |
| Strategy | Developer-first, services-led validation, then recurring product |
| Product boundary | Frozen AivoGuard product scope |
| Technical authority | Applicable frozen specifications, ADRs, and implementation gates |

## 1. Commercial thesis

AivoGuard should sell confidence in reproducible economic testing—not generic LLM evaluation, production custody, or an agent orchestration platform.

Initial customer problem:

> Before an autonomous agent is connected to a real economic workflow, its developer needs reproducible evidence that supported economic invariants hold across normal, adversarial, and regression scenarios.

The first commercial wedge is **agentic payment and economic workflow testing**. This is a target market hypothesis to validate, not a claim of established product-market fit.

## 2. Customer segments and jobs

| Segment | Job to be done | Initial validation offer |
|---|---|---|
| Agent/payment startups | Test economic edge cases before launch | Paid scenario design and integration assessment |
| Agent framework developers | Add deterministic economic regression tests | Reference integration and developer support |
| Fintech/payment engineering teams | Reproduce and investigate economic failure cases | Scoped test harness assessment |
| Web3/agent commerce teams | Test protocol-specific economic flows | Ecosystem scenario and adapter pilot |

Start with one segment where the founder can reach engineering decision-makers. Do not build separate architectures for each segment.

## 3. Commercial phases

### Phase 0 — Technical trust (Months 1–2)
- Reconcile implementation and specification status.
- Resolve confirmed correctness defects.
- Achieve clean required CI.
- Resolve license and security-contact prerequisites.
- Prepare a reproducible demo using verified existing capabilities.

**Exit criteria:** a third party can reproduce the demo from a clean environment; no unresolved critical correctness issue is hidden.

### Phase 1 — Developer validation (Months 3–4)
- Conduct structured discovery with 10–15 relevant developers.
- Recruit 3–5 design partners.
- Observe developers running real scenarios.
- Measure setup friction, time-to-first-result, useful failure classes, and repeat use.
- Validate the developer interface through the specification process before implementation.

**Exit criteria:** at least three teams complete an evaluation and provide actionable feedback; one repeated use case is identified.

### Phase 2 — Services-led paid pilots (Months 5–6)
- Offer fixed-scope scenario design, integration assessment, and test-suite setup.
- Charge for engineering outcomes and documented evidence, not unverified security guarantees.
- Capture recurring test cases and integration requirements.
- Keep customer-specific work separate from public grant deliverables.

**Exit criteria:** target two paid pilots or assessments; document willingness to pay, delivery effort, and repeat demand.

### Phase 3 — Commercial beta (Months 7–9)
Only if repeat usage is demonstrated:
- Specify and build hosted test execution or team workflows.
- Add test history and regression comparison.
- Add team access and billing.
- Add CI integration where the interface is authorized.
- Establish data retention, tenant isolation, and operational security requirements before hosting customer data.

**Exit criteria:** 5–10 active paying customers is a stretch target; prioritize retention and repeated workflows over sign-up count.

### Phase 4 — Enterprise readiness (Months 10–12)
Only after beta usage:
- Specify private deployment and enterprise controls.
- Establish support processes and service commitments.
- Add evidence export and administrative controls.
- Conduct threat modeling and operational review.
- Validate enterprise procurement and security requirements.

**Exit criteria:** at least one enterprise evaluation with a documented technical buyer, deployment requirement, and budget owner.

## 4. Pricing hypotheses

All prices below are experiments, not validated market rates.

| Offer | Initial price hypothesis | Notes |
|---|---:|---|
| Community | Free | Open-source core and public scenarios |
| Developer Cloud | USD 19–49/month | Only after hosted product exists |
| Team | USD 99–299/month | Only after team workflows exist |
| Enterprise | From USD 5,000/year | Subject to deployment, support, and security scope |
| Integration assessment | Fixed quote per scope | Recommended first revenue path |

For the first two paid engagements, prefer a clearly scoped fixed-fee assessment over an early subscription. Define deliverables, exclusions, data handling, and acceptance criteria in writing.

## 5. Commercial metrics

Track:

- Qualified developer conversations.
- Demo completion rate and time to first successful test.
- Number of teams that run a second test session.
- Number of repeatable scenarios contributed or requested.
- Paid pilot conversion and gross delivery effort.
- Monthly recurring revenue only after subscriptions exist.
- Retention, expansion, support burden, and infrastructure cost.
- Defects found before production and reproducibility of their evidence.

Do not treat GitHub stars, downloads, or social impressions as proof of willingness to pay.

## 6. Revenue targets (hypotheses)

Initial operating targets:

- Month 4: 3–5 design partners.
- Month 6: two paid pilots or assessments.
- Month 9: 5–10 paying users/teams if beta is justified.
- Month 12: USD 1,000–3,000 MRR as an exploratory target, conditional on a hosted product and repeat demand.

These are targets for learning and execution, not forecasts or guarantees.

## 7. Product and scope guardrails

- Do not build a large dashboard before a CLI or other developer workflow is specified, authorized, and validated.
- Do not implement a CLI, SDK, hosted API, or adapter solely because this commercial roadmap mentions it. Relevant open decisions and implementation gates remain authoritative.
- Do not make AivoGuard a production payment processor, wallet, custody service, or settlement authority.
- Do not claim that deterministic test results constitute a formal security certification or guarantee against all economic loss.
- Do not build protocol-specific integrations without a customer or ecosystem reason.
- Keep the core product protocol-neutral.

## 8. Relationship to grant work

Grant and commercial work share the same technical core but have separate deliverables and accounting.

| Workstream | Primary beneficiary | Funding |
|---|---|---|
| Core correctness and public scenarios | Developer ecosystem | Grant / company |
| Public documentation and benchmarks | Developer ecosystem | Grant / company |
| Customer-specific integration | Contracting customer | Commercial |
| Hosted team workspace | Paying teams | Commercial |
| Private deployment and support | Enterprise customer | Commercial |

Any dual-funded work must be disclosed and allocated transparently. Grant terms and customer contracts must not conflict.

## 9. Review gates

At the end of each phase, decide to proceed, narrow, or stop based on evidence. Do not automatically advance because a calendar month has passed.

A commercial feature requires:
1. A validated customer problem.
2. A written product and technical specification.
3. Security and data-handling requirements.
4. A credible operating-cost model.
5. A measurable acceptance criterion.

## 10. Immediate next actions

1. Complete repository and CI re-audit.
2. Resolve confirmed technical and documentation inconsistencies.
3. Establish license and security-reporting prerequisites.
4. Produce a clean, reproducible demonstration.
5. Interview target developers and recruit design partners.
6. Finalize the first paid assessment offer.
