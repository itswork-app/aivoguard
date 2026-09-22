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

### Changed

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
