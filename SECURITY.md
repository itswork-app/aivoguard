# Security Policy

## Supported versions

AivoGuard is in **foundation / bootstrap** stage. There is no production
release and no supported product version yet.

| Version | Supported |
| --- | --- |
| `0.0.0` (foundation) | Best-effort review of repository/tooling issues only |
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

There is currently no economic engine, network service, authentication system,
or deployment surface to exploit. Do not submit exploit instructions.

## Safe harbor

Good-faith security research against systems you are authorized to test is
welcome once a production surface and reporting channel exist. Unauthorized
access, data destruction, or privacy violations are not authorized.

## Secrets and credentials

If you discover credentials, tokens, or secrets in this repository, report them
through the future security channel once available, and do not reuse or share
them.
