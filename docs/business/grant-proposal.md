# AivoGuard — Public-Good Grant Proposal (Master Draft)

| Field | Value |
|---|---|
| Status | **DRAFT — not submission-ready** |
| Applicant model | Company-led; legal applicant details pending |
| Funding model | Hybrid: public-good engineering funded separately from commercial work |
| Planning request | USD 30,000 (planning envelope only) |
| Planning duration | Six months |
| Project | Deterministic economic testing infrastructure for autonomous agents |
| Initial wedge | Reproducible testing of agentic economic and payment workflows |
| Scope authority | Frozen product/design specifications and explicit implementation authorization |

> This is a master proposal, not an application to a named funder. Do not submit unchanged. The repository now declares Apache-2.0; private vulnerability reporting is not yet configured, M06 implementation is authorized by TASK-68 (with independent audit still open), and full third-party IP/provenance review is incomplete. Funder fit, eligibility, applicant details, IP terms, and budget eligibility must be verified before submission.

## 1. Executive summary

AivoGuard is a Rust-based deterministic economic testing library for developers building autonomous agents that can initiate or influence economically consequential workflows.

The current codebase includes an economic kernel (M01), invariant evaluator (M02), deterministic simulator (M03), adversarial scenario engine (M04), M06 regression engine (implemented and prospectively authorized by TASK-68; independent audit not yet claimed), payment/settlement testing module (M07), and a bounded inbound x402 semantic adapter (M08). M05 chaos and M09 multi-agent world are not implemented. The current public repository does not provide a CLI, SDK, hosted API, live payment processor, wallet, custody, transaction signer, or runtime enforcement proxy.

The project thesis is that agent developers need reproducible evidence about economic consequences—not only plausible model output or successful tool invocation. AivoGuard evaluates supported economic actions and scenarios using deterministic rules. LLM output is not authoritative for economic state or test verdicts.

Grant funding would support correctness hardening, independent review, reproducible public scenarios, benchmark methodology, documentation, and external developer evaluation. The requested work will remain within existing frozen semantics unless a separately reviewed specification amendment and implementation authorization are approved.

## 2. Problem and public value

Agent systems are increasingly connected to tools that can affect balances, fees, refunds, transfers, and settlement workflows. Conventional functional tests may prove that an API call returned successfully without proving that the resulting modeled economic state satisfies the developer's declared constraints.

A reproducible economic test foundation can help developers investigate questions such as:
- Does a supported action preserve the modeled balance and authorization rules?
- Does a fee-bearing transfer fail atomically when aggregate funds are insufficient?
- Do declared invariants hold before and after a transition?
- Can a deterministic simulation reproduce the same outcome from the same inputs?
- Does a code or integration change alter previously recorded economic expectations?
- Does a bounded external payment observation map to the expected semantic fields?

These examples must be interpreted only within the behavior implemented and specified by AivoGuard. They are not claims that all listed failure modes are already fully covered, nor guarantees that real-world loss is prevented.

## 3. Current technical baseline

| Component | Current evidence | Explicit limitation |
|---|---|---|
| M01 Economic Kernel | Rust source and integration tests | Only supported modeled actions; not a production ledger or payment processor |
| M02 Invariant Engine | Rust source and integration tests | Read-only deterministic evaluator; no claim of formal verification |
| M03 Simulator | Rust source and integration tests | Deterministic scenario orchestration; not a live environment sandbox |
| M04 Adversarial Engine | Rust source and integration tests | Candidate transformations within its implemented model; not exhaustive attack generation |
| M06 Regression | Rust source and integration tests are present | Implemented and prospectively ratified effective 2026-11-07 by TASK-68; bounded by frozen Gate-5 semantics. Independent correctness audit remains open. |
| M07 Payment/Settlement | Implementation master and tests; frozen bounded scope | No provider I/O, live payment, webhook, or wire protocol |
| M08 x402 Adapter | TASK-63…67 implementation record and tests | Inbound semantic translation only; no HTTP/wire codec, crypto verification, facilitator, outbound path, or M07 observation output |
| M05 / M09 | No source module or integration test | Not implemented |

The authoritative current evidence register is [Implementation Status Ledger](../implementation-status.md). The exact commit and CI run must be updated at submission time.

## 4. Proposed public-good work

The grant will not fund a speculative expansion into a generic agent platform. It will fund a narrow technical assurance and reproducibility program:

1. **Correctness hardening:** independently review M01/M02 edge cases and close confirmed defects with regression coverage.
2. **Specification-to-code traceability:** reconcile implementation authorization records and maintain a source/spec/test evidence matrix without changing frozen semantics silently.
3. **Reproducible public scenarios:** curate versioned scenarios using only existing authorized interfaces and supported module behavior.
4. **Benchmark methodology:** publish repeatable test inputs, expected outcomes, environment requirements, and measurement methodology.
5. **Bounded ecosystem relevance:** where the selected funder requires it, demonstrate concrete value to its developers using existing authorized integration boundaries. Any new adapter or wire feature requires a separate specification and implementation authorization before work begins.
6. **Independent review and public reporting:** commission a scoped correctness/security review and publish findings, limitations, and remediation status.

No grant milestone will claim delivery of an unimplemented CLI, SDK, hosted service, runtime enforcement layer, live x402 verification, M05, or M09.

## 5. Six-month milestone plan

### M1 — Baseline integrity and traceability (Month 1)
- Verify the exact repository baseline and required CI.
- Reconcile README, architecture, documentation index, module status, and implementation evidence.
- Record M06 implementation authorization and align source, tests, and public claims with TASK-68; keep independent audit limitations explicit.
- Publish known limitations and open risks.

**Acceptance evidence:** versioned implementation ledger, exact commit SHA, green CI run links, and unresolved-risk register.

### M2 — Independent correctness review (Month 2)
- Review M01 aggregate debit, state validation boundaries, arithmetic/error classification, and atomicity.
- Review M02 history ranges, quantifier/binding semantics, deterministic ordering, and error precedence.
- Review cross-module assumptions where M03/M04/M06 consume M01/M02 results.
- Fix accepted defects without changing frozen semantics silently.

**Acceptance evidence:** reviewer report, issue-to-fix traceability, regression tests, and green required CI.

### M3 — Public reproducible scenario pack (Month 3)
- Curate scenarios for behavior already supported by the current engine.
- Include expected outcomes, fixture inputs, version requirements, and reproduction instructions.
- Separate implemented cases from planned/unsupported cases.

**Acceptance evidence:** tagged scenario pack, expected-output documentation, and clean-environment reproduction record.

### M4 — Benchmark and developer evaluation (Month 4)
- Define deterministic benchmark inputs and methodology.
- Measure runtime and scaling only for declared workloads and hardware/software conditions.
- Invite external developers to reproduce scenarios and record setup friction and findings.

**Acceptance evidence:** benchmark methodology, raw reproducible inputs, results with environment metadata, and external feedback log.

### M5 — Ecosystem-facing public integration evidence and review (Month 5)
- Select one funder-relevant ecosystem only after validating actual developer demand.
- Use existing authorized integration scope; do not implement live protocol functionality without separate approval.
- Commission independent review of the funded changes and publish findings.

**Acceptance evidence:** documented ecosystem use case, reproducible reference workflow, independent review report, and remediation register.

### M6 — Public release and impact report (Month 6)
- Publish versioned public scenarios, documentation, benchmark methodology/results, and review summary.
- Report actual external participation and feedback, including negative results.
- Provide milestone accounting and a follow-on maintenance plan.

**Acceptance evidence:** tagged release or versioned source snapshot, public deliverables, final technical report, and funder report.

## 6. Planning budget

**USD 30,000 over six months** is a planning envelope, not a validated quote, approved budget, or grant award.

| Category | Planning amount | Intended use |
|---|---:|---|
| Core engineering and correctness | USD 12,000 | Focused remediation, regression coverage, traceability |
| Independent review | USD 6,000 | External correctness/security review |
| Public scenarios and benchmark work | USD 4,500 | Scenario curation, reproducibility, benchmark methodology |
| Documentation and developer onboarding | USD 3,000 | Guides, limitations, public reports |
| CI and test infrastructure | USD 1,500 | Reproducible validation infrastructure |
| External developer evaluation | USD 1,500 | Pilot support and feedback collection |
| Contingency | USD 1,500 | Funder-approved unforeseen eligible costs |
| **Total** | **USD 30,000** | |

Before submission, replace this planning envelope with named contributor effort, locally validated rates, milestone cash flow, eligible-cost mapping, tax/accounting treatment, and funder-required currency. No expense may be charged to two funding sources. Founder compensation is eligible only if the selected funder explicitly permits it and the budget transparently justifies it.

## 7. Measurable outcomes

Target outcomes for the funded period:
- 100% of claimed implemented capabilities mapped to source, tests, and a declared specification/authorization record.
- All required CI checks passing on the release baseline.
- Every accepted correctness defect linked to a regression test.
- A public scenario pack with reproduction instructions and expected results.
- At least three external developer teams invited to evaluate; report actual participation and repeat usage rather than assuming adoption.
- One independently reviewed funded scope, with findings and remediation status published.
- A public limitations statement that distinguishes deterministic testing from formal verification or production security assurance.

These are proposed targets, not achieved results.

## 8. Sustainability and separation of funds

AivoGuard follows a company-led hybrid model:
- Public-good funding supports reusable correctness work, scenarios, benchmark methodology, and documentation.
- Commercial revenue may later support customer-specific integration assessment, support, hosted execution, and team workflows after customer validation and separate technical/security specifications.
- The essential inspectability of the deterministic core should remain accessible under the license selected by the project.
- Grant and commercial deliverables, time, invoices, and infrastructure costs will be tracked separately. Shared work must be allocated transparently and disclosed where required.

VC funding is not a prerequisite for this stage. Institutional fundraising should be considered only after repeated developer usage and willingness to pay are demonstrated.

## 9. Risks and mitigations

| Risk | Mitigation |
|---|---|
| License and contributor-rights ambiguity | Select and publish a license, verify ownership/authority, and establish contributor terms before submission |
| M06 authorization gap | Reconcile the formal implementation task record; do not overclaim until closed |
| Hidden correctness defects | Independent review, edge-case tests, fuzz/property testing where justified, transparent issue register |
| Ecosystem mismatch | Apply only when a measurable developer/ecosystem benefit is demonstrated |
| Scope expansion | Frozen specs and explicit implementation gates; no grant document silently authorizes code |
| Weak adoption | External evaluation, repeat-use measurement, and publish negative findings |
| Overstated security claims | State model assumptions and limitations; no certification or loss-prevention guarantee |
| Funding overlap | Separate cost centers, time records, invoices, and deliverable mapping |

## 10. Mandatory pre-submission checklist

- [ ] Select a named funder and verify current eligibility, jurisdiction, applicant type, allowable costs, IP terms, and reporting requirements.
- [ ] Resolve the repository license and contributor-rights policy; publish actual license text and metadata.
- [ ] Configure private vulnerability reporting or provide a verified private contact and response process.
- [ ] Reconcile M06 implementation authorization record (TASK-68).
- [ ] Verify green CI on the exact main commit proposed as the application baseline.
- [ ] Produce a clean-environment reproducible demonstration from documented existing capabilities.
- [ ] Obtain external developer feedback or clearly state that validation is not yet complete.
- [ ] Replace planning budget with a contributor/time/rate/cash-flow budget.
- [ ] Complete legal applicant identity, signatory authority, tax/accounting, and payment details.
- [ ] Review every application claim against source, tests, specifications, and authorization records.

## 11. Closing statement

AivoGuard seeks to provide reusable, deterministic infrastructure for testing modeled economic behavior in autonomous-agent workflows. The public value is a transparent and reproducible way for developers to explore supported economic transitions, evaluate declared invariants, and detect regressions before connecting agents to real systems. The project will publish its assumptions, limits, and review findings rather than presenting test infrastructure as a guarantee of security.
