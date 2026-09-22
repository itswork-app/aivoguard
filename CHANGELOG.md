# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
once releases begin.

## [Unreleased]

### Added

* Repository foundation bootstrap: Cargo workspace, minimal `aivoguard` crate,
  CI, contribution docs, specification policy, and documentation layout.
* Product scope specification (`docs/design/product-scope.md`) — **FROZEN**.
* Domain contract (`docs/design/domain-contract.md`) — **FROZEN** (Gate 0 **CLOSED**).
* Economic kernel specification — **FROZEN**; M01 implemented in `crates/aivoguard`.
* Economic invariant engine specification — **FROZEN** (Gate 2 **CLOSED**);
  M02 implemented in `crates/aivoguard`.
* Deterministic simulator specification
  (`docs/design/deterministic-simulator-specification.md`) — **FROZEN**
  (Gate 3 **CLOSED**); M03 implemented in `crates/aivoguard` (`simulator`).
* Adversarial scenario engine specification
  (`docs/design/adversarial-scenario-engine-specification.md`) —
  **FROZEN** (Gate 4 **CLOSED**). TASK-10G (audit TASK-10F-R3; normative
  baseline `58f0e01`). Implementation authorized by TASK-10H; implemented
  under `crates/aivoguard` (`adversarial`) in TASK-11.
* Economic regression engine specification
  (`docs/design/economic-regression-engine-specification.md`) —
  **FROZEN** (Gate 5 **CLOSED**). TASK-19 (audit TASK-18 READY_FOR_FREEZE).
  Implementation remains **BLOCKED**.

### Changed

* TASK-19: formal Gate-5 freeze of M06 economic regression engine
  specification (implementation remains blocked; DC-13 / AD-03/12/14/15 /
  M06-OD-01/02/03 remain OPEN).
* TASK-11: M04 production implementation (`adversarial` module) against the
  frozen Gate-4 specification. Open decisions AD-03/AD-14/AD-15 preserved.
* TASK-10G: formal Gate-4 freeze of M04 adversarial scenario engine
  specification (implementation remains blocked).
* TASK-10R: M04 Gate-4 specification semantic remediation (transformation
  projection, initial-state contract, parameter candidates, error phases,
  NonApplicable policy, composition targeting, ordering, limit boundaries).
* TASK-10RR: M04 final Gate-4 semantic remediation (ParameterValue types,
  composition emission counting, GenerationStatus/limit breach, M04↔M03
  structural validation boundary, identity stability vs algorithm).
* TASK-10RRR: M04 freeze-blocker remediation — declared incompatibility
  contract (B-01), closed structural validation checklist (B-02), duplicate
  ActionId resolution (B-03).
* TASK-10RRRR: targeted freeze remediation — incompatibility occurrence /
  self-match / `EXACT_PAIR` (R2-B-01); check-4a/4b order, IdentifierToken,
  invariant `{id, definition_version}` minimum (R2-B-02).
* Product scope status updated from `READY_FOR_REVIEW` to `FROZEN` after
  explicit approval.
* Domain contract progressed through TASK-02 / TASK-02R remediation and
  TASK-02F freeze.
* Economic kernel specification frozen (TASK-03F); M01 implementation (TASK-04).
* M03 specification frozen after TASK-07 / TASK-07R remediation and TASK-07F
  independent Gate-3 freeze audit.
* M03 deterministic simulator implemented (TASK-08).
* M03 scheduling / stop-condition remediation (TASK-08R).
* TASK-08F independent M03 implementation audit — **PASS**.
* TASK-09 Gate-3 closure: M03 implementation baseline `c4ce9bb`; M01/M02/M03
  recorded as Implemented / **PASS**; Gates 0–3 **CLOSED**.

### Deprecated

* None.

### Removed

* None.

### Fixed

* None.

### Security

* None.

---

No released versions yet. Version `0.0.0` denotes foundation stage only.
