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
  **READY_FOR_REVIEW** (Gate 4 **OPEN**; not frozen; implementation
  **BLOCKED**). TASK-10; TASK-10R; TASK-10RR; TASK-10RRR (B-01…B-03).

### Changed

* TASK-10R: M04 Gate-4 specification semantic remediation (transformation
  projection, initial-state contract, parameter candidates, error phases,
  NonApplicable policy, composition targeting, ordering, limit boundaries).
* TASK-10RR: M04 final Gate-4 semantic remediation (ParameterValue types,
  composition emission counting, GenerationStatus/limit breach, M04↔M03
  structural validation boundary, identity stability vs algorithm).
* TASK-10RRR: M04 freeze-blocker remediation — declared incompatibility
  contract (B-01), closed structural validation checklist (B-02), duplicate
  ActionId resolution (B-03).
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
