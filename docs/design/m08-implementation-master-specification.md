# AivoGuard M08 Implementation Master Specification
## (Inbound-Only Bounded Scope)

| Field | Value |
| --- | --- |
| Document | `docs/design/m08-implementation-master-specification.md` |
| Task | **TASK-59** |
| Module | **M08 — x402 Adapter** |
| Gate | **GATE 7** |
| Status | **DRAFT — REMEDIATED; READY FOR INDEPENDENT RE-AUDIT** |
| Normative source | [`x402-adapter-specification.md`](x402-adapter-specification.md) (**FROZEN BY TASK-54**) |
| Implementation scope | **INBOUND-ONLY** |
| Implementation authorization | **NOT AUTHORIZED** |
| Wire | **NOT FROZEN** |
| Open decisions | **PRESERVED** |

```text
STATUS:                  DRAFT — REMEDIATED; READY FOR INDEPENDENT RE-AUDIT
NORMATIVE SOURCE:        docs/design/x402-adapter-specification.md (FROZEN BY TASK-54)
IMPLEMENTATION SCOPE:    INBOUND-ONLY
IMPLEMENTATION:          NOT AUTHORIZED
WIRE:                    NOT FROZEN
OPEN DECISIONS:          PRESERVED

TASK-54: M08 semantic/protocol boundary FROZEN
TASK-55: freeze integrity CONFIRMED
TASK-56: architecture baseline SYNCHRONIZED
TASK-57: post-synchronization audit PASS
TASK-58: MASTER_SPEC_READY=YES; MINIMUM_DECISIONS_REQUIRED=NONE
         for inbound-only; IMPLEMENTATION_AUTHORIZATION_READY=NO
TASK-59: this implementation master specification (blueprint only)
TASK-60: independent audit CONDITIONAL (MINOR-01, MINOR-02)
TASK-61: remediation of MINOR-01 / MINOR-02 (+ observations)

This document is subordinate. If it conflicts with the frozen M08
specification, the frozen specification wins.

Writing this master specification does NOT authorize coding.
A separate implementation-authorization task is required before any
Rust module, test crate surface, or dependency change.
```

---

## 1. Authority hierarchy

```text
Product Scope FROZEN
        ↓
Domain Contract FROZEN
        ↓
M01 / M02 / M03 / M04 / M06 / M07 FROZEN (+ implemented where Closed)
        ↓
M08 FROZEN SPECIFICATION (TASK-54)
        ↓
This M08 Implementation Master Spec (TASK-59/61) — DRAFT REMEDIATED
        ↓
Implementation (NOT AUTHORIZED)
        ↓
Tests (NOT AUTHORIZED)
```

Anti-drift: do not amend `x402-adapter-specification.md` to make
implementation easier. Open decisions remain OPEN (§22).

---

## 2. Implementation target

```text
External / fixture observation
          ↓
M08 validation
          ↓
deterministic translation / classification
          ↓
provenance
          ↓
non-authoritative X402SemanticRecord
```

Properties of the authorized **blueprint** (not yet coding-authorized):

```text
INBOUND ONLY
DETERMINISTIC
OFFLINE
FIXTURE-DRIVEN
NON-AUTHORITATIVE
STD-ONLY (no new crates)
```

Public conceptual entrypoint (provisional name; AD-15 OPEN):

```text
adapt_x402_observation(
  observation: &X402ExternalObservation,
  config: &M08MappingConfiguration,
) -> Result<X402SemanticRecord, M08AdapterError>
```

---

## 3. Non-goals (forbidden in this master scope)

```text
HTTP transport / HTTP 402 / headers
JSON / Base64 as normative wire
facilitator API / cryptographic verification
wallet / blockchain / live provider connectivity
production payment or settlement execution
M07 SettlementObservation emission (M08-OD-02)
internal → external emission (M08-OD-06)
M01 EconomicState mutation / Actions
economic PASS/FAIL / M02 invariant evaluation
closing M08-OD-01…06 or OD-03/06/07/08 / DC-11/13 / AD-14/15
public API freeze
serde production codecs
Cargo dependency additions
```

---

## 4. Package / module boundary (not created)

Intended location (provisional; AD-15 OPEN):

```text
crates/aivoguard/src/x402_adapter/   ← NOT CREATED by TASK-59
  mod.rs
  types.rs      # provisional carriers
  config.rs     # declared mapping configuration
  validate.rs   # structural / amount / config checks
  translate.rs  # deterministic translation
  error.rs      # adapter-local errors
  provenance.rs

crates/aivoguard/tests/m08_x402_adapter.rs   ← NOT CREATED by TASK-59
```

Responsibility:

| Surface | Owns | Does not own |
| --- | --- | --- |
| `x402_adapter` | inbound validation, translation, provenance, adapter errors | economic truth, M07 expectations, wire, live I/O |
| Tests | fixture-driven determinism / invariant proofs | production integration |

Forbidden dependencies:

```text
any HTTP / TLS / serde_json / base64 / wallet / chain RPC / x402 SDK crates
```

Reuse existing crate conventions: `i128` amounts (ADR 0001), std-only
`crates/aivoguard`, integration tests under `tests/m0N_*.rs`.

---

## 5. Provisional types vs frozen semantic contract

```text
semantic contract (FROZEN)     ≠     provisional Rust representation (AD-15 OPEN)
```

Names below are **provisional internal shapes**. They may be renamed without
amendment to the frozen M08 specification. They are **not** a frozen public API.

### 5.1 `X402ExternalObservation` (input)

Conceptual carriers (all optional fields remain `Option` / absent when missing):

| Field | Provisional type | Notes |
| --- | --- | --- |
| `binding` / case identity | struct of opaque `String` tokens | Exact compare later; no trim/casefold |
| `protocol_version` | `Option<String>` | External **claim**; not product endorsement |
| `observation_kind` | enum or opaque token | requirement / payload / verification / settlement / … |
| `source_asset_id` | `Option<String>` | Opaque claim |
| `amount` | `Option<i128>` **or** exact integer string | See §6; no `f32`/`f64` |
| `network` | `Option<String>` | Opaque |
| `scheme` | `Option<String>` | Opaque |
| `payer` | `Option<String>` | Opaque |
| `payee` | `Option<String>` | Opaque |
| `payment_reference` | `Option<String>` | Opaque |
| `field_carriers` | map/list of opaque tokens | As declared by mapping config |
| `adapter_provenance` | optional provenance seed | Non-authoritative |
| `notes` | `Option<String>` | Non-authoritative |

Rules:

* Missing → remains missing (no silent `0` / `false` / empty asset).
* Live undeclared network fetch is **not** a valid input.
* Fixture/test construction is explicit (Domain DC-R05 style).

### 5.2 `M08MappingConfiguration` (declared configuration)

Must be an **explicit** input to every adaptation call:

| Field | Purpose |
| --- | --- |
| `mapping_version` | Deterministic pin required on outputs; empty → `ConfigurationFailure` |
| `configuration_id` / version | Provenance identity |
| `version_policy` | Explicit `VersionPolicy` (§5.2.1) — required |
| `require_amount` | Whether amount absence → error vs allowed missing |
| `require_asset` | Whether asset absence → error vs allowed missing |
| `observation_kind_allow_list` | Optional closed set for this config |

```text
Supported protocol versions are not hidden runtime state.
They must be explicitly declared as part of the mapping configuration
for any implementation that performs version support classification.
```

No runtime negotiation. **M08-OD-04** remains OPEN (product-endorsed catalogue).

#### 5.2.1 `VersionPolicy` (provisional; AD-15 OPEN)

Discriminated configuration carrier — reject-all MUST NOT be inferred from an
undocumented empty collection:

| Variant | Meaning | Version classification outcome |
| --- | --- | --- |
| `ClaimOnly` | Version is an opaque external claim; no support filter | Never reject on version alone; claim carried through if present |
| `AllowList(versions)` | Explicit non-empty allow-list | Claim present **and** listed → continue; claim absent or not listed → `Err(UnsupportedProtocolFeature)` |
| `RejectAll` | Explicit reject-all policy | Any observation (any/absent version claim) → `Err(UnsupportedProtocolFeature)` |

Invalid representations → `ConfigurationFailure` (not silent coercion):

* `AllowList` with empty `versions` → **invalid** (`ConfigurationFailure`);
  empty list is **not** `RejectAll` and **not** `ClaimOnly`.
* Missing / unspecified `version_policy` → **invalid** (`ConfigurationFailure`).

```text
VersionPolicy is configuration for this bounded adapter only.
It is NOT a product-endorsed x402 version catalogue (M08-OD-04 OPEN).
No runtime negotiation.
```

### 5.3 `X402SemanticRecord` (output)

Non-authoritative record produced **only** on `Ok(...)`:

| Field | Notes |
| --- | --- |
| translated protocol fields | Opaque/string/`i128` as validated |
| `mapping_version` | From config |
| `provenance` | Required on successful translation (§10) |
| `trust_class` | For this inbound master: always `untrusted claim` (§8) |
| `adapter_status` | Adapter-local success marker (e.g. `translated`) — not an economic verdict |
| classification metadata | Deterministic; non-economic |

MUST NOT imply M01/M07 authority.

---

## 6. Amount representation (inbound-only subset)

**Intentional subset:** this master supports **exact-integer only**.

| Input form | Behavior |
| --- | --- |
| Present `i128` in range | Accept as amount carrier |
| Exact integer decimal-digit string (optional sign) parsable to `i128` without loss | Accept after checked parse |
| Empty / whitespace-only string | `MalformedExternalInput` |
| Non-integer / fractional / decimal with fractional part | **Reject** (`InvalidMapping` or `UnsupportedProtocolFeature`) — **no conversion** |
| `f32` / `f64` / float string forms | **Prohibited** / reject |
| Overflow / underflow on parse | Deterministic numeric error (`InvalidMapping`) |
| Absent amount | Missing remains missing; if `require_amount` → `InvalidMapping` |

```text
M08-OD-01 remains OPEN.
Complete external amount/asset mapping (incl. authorized decimal conversion)
is PATH-A — not part of this bounded core.
```

Checked arithmetic only where needed for parse/bounds. No FX. No saturate.

---

## 7. Asset representation

| Rule | Behavior |
| --- | --- |
| Representation | Opaque `String` claim |
| Present | Carry through unchanged (exact; no trim/casefold) |
| Absent | Remains absent; if `require_asset` → `InvalidMapping` |
| Cross-asset conversion | **Forbidden** |
| Inferring AivoGuard Asset from network/scheme | **Forbidden** |
| Fabricating default asset | **Forbidden** |

Complete protocol→AivoGuard asset binding = **PATH-A** / **M08-OD-01**.

---

## 8. Trust classification and Result contract

### 8.1 Single auditable Result rule (MINOR-01 remediation)

For every inbound call to `adapt_x402_observation`:

```text
Structural / input / mapping / configuration / version-policy failure
    → Err(M08AdapterError)

Successfully translated observation
    → Ok(X402SemanticRecord)
         with trust_class = untrusted claim
```

There is **exactly one** outcome shape per input: `Ok(record)` **or** `Err(error)`.
Mapping / input rejection is **never** expressed as `Ok` with a special trust class.

### 8.2 Recognized trust classes vs this master's emissions

Frozen M08 recognizes three trust class *names*. This inbound master binds
emissions as follows (does **not** amend the frozen specification):

| Class | Frozen recognition | Emitted by this inbound master? |
| --- | --- | --- |
| `untrusted claim` | DEFAULT / EMITTABLE | **YES** — **only** trust class on every `Ok(X402SemanticRecord)` |
| `rejected` | Recognized / EMITTABLE at semantic freeze | **NOT EMITTED** by this inbound Result path — **RESERVED** for a future **separately specified** semantic use; **not required** by the minimal inbound path. Adapter/mapping rejection here uses `Err(M08AdapterError)` only (≠ economic FAIL) |
| `verified-under-policy` | RESERVED | **NOT_EMITTABLE** while **M08-OD-03** OPEN |

```text
trust_class = rejected
  → NOT produced by adapt_x402_observation under this master
  → RESERVED for future/explicit semantic use outside this Result rule
  → does NOT mean economic FAIL / M01 FAIL / M07 FAIL

verified-under-policy
  → RESERVED / NOT_EMITTABLE while M08-OD-03 OPEN

M08-OD-03 remains OPEN.
No cryptographic / facilitator verification path in this master.
ExternalVerificationFailure is not a successful verify outcome.
```

### 8.3 Provenance never upgrades trust

```text
PROVENANCE NEVER UPGRADES TRUST.
```

Attaching provenance MUST NOT transform `untrusted claim` → `verified-under-policy`
(or otherwise increase trust). Provenance explains translation; it is not a
verification proof.

---

## 9. Validation / translation pipeline

Deterministic order (must not be reordered by HashMap iteration or threads):

```text
1. INPUT presence (observation + config present)
2. configuration validation (mapping_version non-empty; VersionPolicy well-formed;
   AllowList non-empty if used; observation_kind allow-list well-formed if present)
   → else Err(ConfigurationFailure)
3. structural validation of observation carriers
   → else Err(MalformedExternalInput)
4. required-field validation per config
   → else Err(InvalidMapping)
5. amount / asset validation (§6–§7)
   → else Err(InvalidMapping | UnsupportedProtocolFeature | MalformedExternalInput)
6. protocol/version classification per VersionPolicy (§5.2.1)
   → else Err(UnsupportedProtocolFeature)
7. observation_kind allow-list check (if configured)
   → else Err(UnsupportedProtocolFeature)
8. trust assignment: trust_class = untrusted claim only
   (never verified-under-policy; never rejected-on-Ok)
9. semantic translation into X402SemanticRecord fields
10. provenance attachment (must not change trust_class)
11. Ok(X402SemanticRecord)   OR earlier abort with Err(M08AdapterError)
```

On any error: abort; no partial authoritative side effects (there are none).
No `Ok` record is returned for mapping/input/configuration/version rejection.

Version classification (deterministic; identical observation + config → identical):

| `version_policy` | Behavior |
| --- | --- |
| `ClaimOnly` | Carry opaque claim if present; never fail on version alone |
| `AllowList(v)` with non-empty `v` | Absent or not-in-list claim → `Err(UnsupportedProtocolFeature)` |
| `RejectAll` | Always `Err(UnsupportedProtocolFeature)` for any observation |
| `AllowList([])` or missing policy | `Err(ConfigurationFailure)` — not coerced |

---

## 10. Provenance

Required on **successful** translation (`Ok` only). Deterministic fields only:

| Element | Source |
| --- | --- |
| `mapping_version` | Config |
| `configuration_id` / config version | Config |
| observation binding / identity | Input |
| adapter module label | Constant string (e.g. `m08_inbound_v0`) — not a clock |

Forbidden as required semantic provenance:

* wall-clock timestamps;
* process IDs;
* random UUIDs;
* network addresses.

```text
PROVENANCE NEVER UPGRADES TRUST. (§8.3)
Provenance attachment MUST leave trust_class = untrusted claim unchanged.
```

---

## 11. Error model (adapter-local)

Provisional enum (AD-15 OPEN):

| Error | Inbound-only use |
| --- | --- |
| `MalformedExternalInput` | Structural / parse failure |
| `UnsupportedProtocolFeature` | Kind/version outside declared config subset |
| `InvalidMapping` | Field/amount/asset rule failure; overflow |
| `ConfigurationFailure` | Invalid/incomplete mapping configuration |
| `ExternalVerificationFailure` | **PATH-C only** — not used as success path; must not appear as “verified” outcome while M08-OD-03 OPEN |
| `TransportFailure` | **PATH-E only** — out of scope for this master |

Adapter errors ≠ economic / M01 / M07 failures.

---

## 12. Determinism contract

```text
same X402ExternalObservation
+ same M08MappingConfiguration (incl. mapping_version)
= same Result<X402SemanticRecord, M08AdapterError>
```

Forbidden hidden authority: wall clock, RNG, network, filesystem enumeration
order as truth, HashMap/HashSet iteration affecting outcomes, threads, env,
float, LLM, provider/facilitator/wallet/chain state.

Use `Vec` / BTreeMap where order matters; never rely on HashMap iteration for
semantic ordering.

---

## 13. Serialization / fixture boundary

| Kind | Status |
| --- | --- |
| In-test fixture constructors / literal structs | Allowed as **test infrastructure** |
| Normative HTTP / JSON / Base64 / header wire | **NOT FROZEN** (**M08-OD-05**) |
| OD-03 scenario on-disk format | **OPEN** — not required for in-memory tests |

Do not present fixture helpers as an external x402 wire contract.

---

## 14. M01 boundary

MUST NOT:

* mutate `EconomicState`;
* execute M01 Actions;
* declare economic truth;
* determine economic PASS/FAIL;
* redefine M01 invariants.

Optional future harness may *consume* `X402SemanticRecord` as scenario input
only under separate authorization — not part of this core adapter API.

---

## 15. M07 boundary

MUST NOT:

* emit `SettlementObservation`;
* redefine Settled / fee / expectation taxonomies;
* modify M07 code or frozen M07 specification;
* use AD-14 as M08 wire authority.

**M08-OD-02** remains OPEN → **PATH-B**.

---

## 16. Future path markers

| Path | Capability | Open decision(s) |
| --- | --- | --- |
| **PATH-A** | Complete external amount/asset mapping (incl. authorized decimal conversion) | **M08-OD-01** |
| **PATH-B** | M07 `SettlementObservation` emission / composition | **M08-OD-02** |
| **PATH-C** | Cryptographic / facilitator verification; emit `verified-under-policy` | **M08-OD-03** |
| **PATH-D** | Product-endorsed supported-version catalogue | **M08-OD-04** |
| **PATH-E** | Wire / codec / HTTP integration | **M08-OD-05** (+ OD-03/06/07 soft) |
| **PATH-F** | Internal → external emission | **M08-OD-06** |

Core inbound master MUST NOT implement PATH-A…F behaviors beyond explicit
reject/forbid rules above.

---

## 17. Implementation invariants

```text
INV-M08-01: M08 output is non-authoritative.
INV-M08-02: External observations are untrusted by default.
INV-M08-03: verified-under-policy is not emittable while M08-OD-03 is OPEN.
INV-M08-04: Authoritative amount handling uses exact integer / i128 semantics.
INV-M08-05: No implicit cross-asset conversion occurs.
INV-M08-06: Missing external information is not fabricated.
INV-M08-07: M08 is deterministic for equal observation+config.
INV-M08-08: M08 has no live network dependency.
INV-M08-09: M08 does not mutate M01 EconomicState.
INV-M08-10: M08 does not emit M07 SettlementObservation in core scope.
INV-M08-11: Concrete wire format is not frozen.
INV-M08-12: All M08 Open Decisions remain OPEN.
INV-M08-13: No f32/f64 amount carriers in the bounded implementation.
INV-M08-14: Decimal/fractional amounts without authorized conversion → reject.
INV-M08-15: Adapter errors are not economic PASS/FAIL.
INV-M08-16: Provenance never upgrades trust_class.
INV-M08-17: Mapping/input/configuration/version rejection → Err only;
            Ok records always carry trust_class = untrusted claim;
            trust_class = rejected is not emitted by this inbound path.
INV-M08-18: VersionPolicy is explicit (ClaimOnly | AllowList | RejectAll);
            empty AllowList is ConfigurationFailure, never inferred RejectAll.
```

---

## 18. Testing master plan

When implementation is separately authorized, minimum tests:

### 18.1 Input

* valid exact-integer observation → `Ok` + `untrusted claim`;
* missing amount with `require_amount=false` → amount absent on record;
* missing amount with `require_amount=true` → `Err(InvalidMapping)`;
* missing asset (symmetric to amount rules);
* malformed structure → `Err(MalformedExternalInput)`;
* optional fields absent remain absent.

### 18.2 Amount

* valid `i128`;
* boundary `i128::MIN` / `i128::MAX` where representable as fixture;
* overflow string → `Err`;
* decimal / fractional → `Err` (no conversion);
* float-like input → `Err`.

### 18.3 Asset

* opaque asset accepted;
* absent handled per config;
* no conversion between two asset ids.

### 18.4 Version (`VersionPolicy`)

* `ClaimOnly`: opaque claim preserved; never fail on version alone;
* `AllowList` hit → `Ok` translate;
* `AllowList` miss / absent claim → `Err(UnsupportedProtocolFeature)`;
* `RejectAll`: any observation (any/absent version) → `Err(UnsupportedProtocolFeature)`;
* `AllowList([])` → `Err(ConfigurationFailure)` (not coerced to RejectAll/ClaimOnly);
* missing `version_policy` → `Err(ConfigurationFailure)`;
* no network/discovery/negotiation.

### 18.5 ConfigurationFailure

* empty `mapping_version` → `Err(ConfigurationFailure)`;
* invalid / incomplete config carriers → `Err(ConfigurationFailure)`;
* empty `AllowList` as above.

### 18.6 Trust / Result contract

* default successful `Ok` → `trust_class = untrusted claim` only;
* mapping / input / version rejection → `Err(...)` (not `Ok` with `rejected`);
* assert no API emits `trust_class = rejected` on this inbound path;
* assert no API emits `verified-under-policy`;
* adapter errors are not economic FAIL.

### 18.7 Provenance

* successful `Ok` carries: mapping_version, configuration_id/version,
  observation binding/identity, adapter module label;
* provenance attachment leaves `trust_class = untrusted claim` unchanged
  (**PROVENANCE NEVER UPGRADES TRUST**);
* no wall-clock / RNG / network fields required in provenance.

### 18.8 Determinism

Repeat identical `(observation, config)` → byte-equal / field-equal `Result`.

### 18.9 Authority

* no call path mutates `EconomicState`;
* no function returns M07 `SettlementObservation`;
* no economic PASS/FAIL type in M08 module.

### 18.10 Property / invariant (minimal)

Use ordinary unit/integration asserts (no new property framework required):

```text
no floating-point amount type in module surface
no fabricated zero amount on missing
no fabricated asset on missing
no verified-under-policy emission
no trust_class=rejected on Ok path
no provenance trust upgrade
no network types / tokio / reqwest in module
empty AllowList is ConfigurationFailure
RejectAll always Err
```

Integration file (when authorized): `tests/m08_x402_adapter.rs`.

---

## 19. Open decisions (PRESERVED)

### 19.1 M08-local (all OPEN)

| ID | Bound in this master |
| --- | --- |
| **M08-OD-01** | Exact-integer subset only; reject decimal without conversion; complete mapping = PATH-A |
| **M08-OD-02** | No SettlementObservation emission; PATH-B |
| **M08-OD-03** | Untrusted default on Ok; verified NOT_EMITTABLE; PATH-C |
| **M08-OD-04** | Explicit `VersionPolicy` (ClaimOnly/AllowList/RejectAll); no product catalogue; PATH-D |
| **M08-OD-05** | No wire/codec; PATH-E |
| **M08-OD-06** | Inbound only; PATH-F |

### 19.2 Parent / global (all OPEN)

| ID | Bound in this master |
| --- | --- |
| OD-03 | In-memory fixtures sufficient; no on-disk scenario freeze |
| OD-06 | No evidence wire freeze |
| OD-07 | No CLI/HTTP surface |
| OD-08 | In-crate module only; no plugin architecture freeze |
| DC-11 | Use frozen M08 subset constraints; global DC-11 remains OPEN |
| DC-13 | No M06/M07 verdict composition in core |
| AD-14 | Not M08 wire authority |
| AD-15 | Types provisional; no public API freeze |

---

## 20. Dependency policy

```text
REQUIRED: std only (existing aivoguard crate policy)
FORBIDDEN: new crates for this bounded scope
FUTURE: any x402/HTTP/serde/wallet/chain dependency requires separate authorization
```

---

## 21. Acceptance criteria (for later implementation audit)

Implementation (when authorized) passes only if it matches this master **and**:

* frozen M08 semantic contract;
* inbound-only scope;
* open decisions still OPEN;
* wire still NOT FROZEN;
* Result contract §8.1 (Ok=untrusted only; rejections=Err);
* VersionPolicy §5.2.1 explicit;
* trust / amount / asset / version / determinism / error / provenance rules above;
* INV-M08-01…18 held by tests;
* no M01/M07 leakage.

---

## 22. Document control

| Item | Value |
| --- | --- |
| Created | **TASK-59** |
| Remediated | **TASK-61** (MINOR-01, MINOR-02; OBSERVATION-01/02) |
| Status | **DRAFT — REMEDIATED; READY FOR INDEPENDENT RE-AUDIT** |
| Normative source | M08 FROZEN BY TASK-54 |
| Implementation | **NOT AUTHORIZED** |
| Wire | **NOT FROZEN** |
| Open decisions | **PRESERVED** |
| Traceability | TASK-54 → 55 → 56 → 57 → 58 → 59 → 60 → **61** |
| TASK-58 result | `MINIMUM_DECISIONS_REQUIRED: NONE` for inbound-only; `MASTER_SPEC_READY: YES`; `IMPLEMENTATION_AUTHORIZATION_READY: NO` |
| TASK-60 result | `CONDITIONAL` (MINOR-01 dual rejection; MINOR-02 version policy) |
| Next | **TASK-62 — M08 implementation master specification independent re-audit** |

```text
MASTER_SPEC_STATUS: REMEDIATED_DRAFT_READY_FOR_INDEPENDENT_REAUDIT
IMPLEMENTATION: NOT AUTHORIZED
WIRE: NOT FROZEN
OPEN_DECISIONS: PRESERVED
RESULT_CONTRACT: Ok(untrusted) | Err(adapter) — trust_class=rejected not emitted
VERSION_POLICY: ClaimOnly | AllowList(non-empty) | RejectAll
PROVENANCE: NEVER UPGRADES TRUST
```
