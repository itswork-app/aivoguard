# Security Policy

## Supported versions

AivoGuard is a pre-production Rust testing library. M01–M04 and M06–M08 have source and integration tests within documented bounds. There is no production release, hosted service, or supported production version.

| Version | Supported |
| --- | --- |
| Current unreleased main branch | Best-effort review of repository and library issues; no production SLA |
| Production releases | Not yet available |

## Reporting a vulnerability

A dedicated security contact (email, private intake form, or GitHub Security
Advisories configuration) has **not** been established yet.

Until a reporting mechanism is configured:

* Do **not** assume a public GitHub issue is an appropriate channel for
  sensitive vulnerability reports.
* Do **not** expect a formal SLA for security response during foundation stage.
* A professional vulnerability-reporting process — including a contact path,
  acknowledgment expectations, and coordinated disclosure guidance — will be
  established **before any production release**.

When that process exists, this document will be updated with the contact path
and response expectations.

## Scope notes for this stage

Relevant reports during foundation stage are limited to:

* Repository hygiene issues that could later become security risks
  (for example accidental inclusion of secrets)
* CI or tooling misconfiguration that weakens integrity of validation

The repository includes local deterministic economic/testing modules. It currently has no hosted service, production payment execution, custody, or live x402 verification surface. Reports concerning code correctness, parser behavior, state transitions, invariant evaluation, simulation, regression, settlement testing, or adapter boundaries are in scope for maintainer triage. Do not submit exploit instructions in public issues.

## Safe harbor

Good-faith security research must be limited to systems you are authorized to test. Unauthorized access, data destruction, or privacy violations are not authorized. A private reporting channel and coordinated disclosure process are still required before a production release.

## Secrets and credentials

If you discover credentials, tokens, or secrets in this repository, report them
through the future security channel once available, and do not reuse or share
them.
