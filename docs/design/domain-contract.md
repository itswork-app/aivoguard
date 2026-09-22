# AivoGuard Domain Contract

| Field | Value |
| --- | --- |
| Document | `docs/design/domain-contract.md` |
| Task | TASK-02 / TASK-02R / TASK-02F |
| **STATUS** | **FROZEN** |
| **NORMATIVE FREEZE** | **FROZEN** |
| **GATE** | **GATE 0 — CLOSED** |
| Authority basis | Frozen Product Scope (`docs/design/product-scope.md`) |
| Implementation | **BLOCKED** until TASK-03 authorization |
| Gate 1 (Economic Kernel) | **BLOCKED** — Gate 0 closure does **not** authorize kernel implementation |
| Remediation | TASK-02R — DC-R01…DC-R05 |
| Freeze task | TASK-02F |

### Freeze rationale

```text
TASK-02R semantic remediation completed.
Final Gate 0 review completed.
Remaining open decisions are explicitly tracked and do not invalidate the
current normative semantic contract.
```

This document is the **frozen** normative economic domain contract for AivoGuard
(Gate 0). Material semantic changes require an explicit specification amendment.

Gate 0 closure does **NOT** authorize Economic Kernel implementation.
The next authorized specification task is **TASK-03 — Economic Kernel
Specification**. TASK-03 must determine its own implementation-critical
decisions before any Rust domain implementation begins.

No Rust types, APIs, CLIs, SDKs, serialization formats, or integrations are
defined or selected by this document.

Related:

* Product scope (frozen): [`product-scope.md`](product-scope.md)
* Process policy: [`SPECIFICATION-POLICY.md`](SPECIFICATION-POLICY.md)
* ADR process: [`../adr/README.md`](../adr/README.md)

---

## 1. Status / Authority

### Hierarchy

```text
Product Scope (FROZEN)
      ↓
Domain Contract (this document; FROZEN; GATE 0 CLOSED)
      ↓
Economic Kernel Specification (TASK-03; not yet authorized)
      ↓
Implementation
      ↓
Tests
```

### Authority rule

This Domain Contract defines **semantic truth** for economic concepts.

Implementation must later conform to this frozen Domain Contract.

The contract must **not** be derived from implementation convenience.

No implementation decision may silently redefine domain meaning.

---

## 2. Scope

### In scope

This contract defines semantic meaning for:

* `EconomicWorld`, `EconomicState`
* `Actor`, `Account`, `Asset`, `Money`, `Balance`
* `Action`, `Transaction`, `Event`
* ordering and time
* `Scenario`, `Invariant`, `Violation`
* `Simulation`, `SimulationResult`, `RegressionCase`
* `Evidence`
* determinism, randomness, failure semantics
* economic authority and external/adapter boundaries
* conservation as an invariant category
* fees, price, settlement, reversal/refund (conceptual)
* idempotency, authorization boundary
* hidden-state prohibition and derived-state rules
* history/replay, serialization boundary, versioning
* LLM boundary

### Out of scope

* Rust / language type definitions
* crate layout for domain modules
* numeric representation selection (OD-02 remains open)
* scenario / evidence / API serialization formats
* CLI, SDK, HTTP, language bindings
* payment-provider, blockchain, x402, marketplace protocol details
* market microstructure
* production integrations
* universal hard-coded conservation laws for every world

---

## 3. Domain Principles

1. **Deterministic engine authority** — the deterministic domain engine is the
   economic authority inside AivoGuard's test domain.
2. **Explicit state** — economically material values must be represented in, or
   deterministically derived from, the authoritative state model.
3. **No hidden economic state** — undeclared mutable economic effects are
   prohibited.
4. **Action ≠ success** — an action request does not imply a successful
   economic transition.
5. **Error ≠ PASS** — engine/system errors must never silently become economic
   PASS.
6. **Violation ≠ engine error** — distinct failure classes with distinct meaning.
7. **Logical ordering over wall-clock** — wall-clock time is not authoritative
   ordering for economic correctness.
8. **LLM non-authority** — LLM output is never authoritative economic truth.
9. **Conservation is categorical** — conservation invariants are world-/scenario-
   defined, not universal hard-coded laws of every `EconomicWorld`.
10. **Adapters translate, they do not redefine truth** — external systems are not
    automatically authoritative inside the test domain.
11. **No binary floating-point as authoritative money** — floating-point values
    are not authoritative representations of economic amounts.
12. **Specification over convenience** — ambiguity is surfaced, not silently
    resolved for implementation ease.

---

## 4. EconomicWorld

### Purpose

`EconomicWorld` is the complete **rule environment** in which economic behavior
occurs for a test/simulation.

### Semantic meaning

The World holds **WORLD RULES** — configuration and policy that define what is
economically meaningful and permitted — distinct from the current
`EconomicState`.

Conceptually, a World may define (non-exhaustive):

```text
actors (or actor schemas)
accounts (or account schemas)
assets
prices (as rule/configuration where applicable)
fees (as rule/configuration where applicable)
markets
liquidity rules
payment rules
settlement rules
economic policies
authorization policies with economic effect
```

### Ownership / authority

World rules are authoritative for interpreting actions and transitions **within
a simulation bound to that World**.

World configuration is owned by the scenario/test authoring boundary, not by
agent intelligence.

### Required properties

* Explicit identity/versionability of the World used in a Simulation.
* Clear separation of WORLD RULES vs STATE.
* Deterministic interpretation under a fixed World version.

### Default mutability rule

**By default, World configuration is immutable for the duration of a
Simulation.**

An agent Action must **not** silently mutate World rules unless a frozen World
policy **explicitly** permits a named class of world-mutating actions, and those
mutations are recorded as inspectable economic/history effects.

### Prohibited ambiguity

* Treating ad-hoc code constants as undeclared World rules.
* Silently mutating fees/settlement policies mid-simulation without explicit
  world-mutation semantics.
* Conflating World rules with transient `EconomicState`.

### Relationships

* A `Scenario` binds a World (or World configuration) with initial state and
  execution inputs.
* A `Simulation` executes against one effective World for its run (unless an
  explicitly specified world-mutation model says otherwise).
* `EconomicState` evolves under World rules.

### Deferred

* Exact World schema, composition, inheritance, and packaging (`DC` open items
  and later kernel/spec work).
* Which World fields are mandatory for Gate 1 minimum kernel.

---

## 5. EconomicState

### Purpose

`EconomicState` is the **authoritative economically relevant state** at a point
in execution.

### Semantic meaning

If a value can materially affect economic correctness, it must not exist as
hidden mutable economic state outside the authoritative state model (or a
declared deterministic derivation from it).

Conceptually, state may include (as applicable to a World):

```text
accounts
balances
assets held / positions
liabilities / commitments (when explicitly modeled)
prices that are state (not merely static world config)
fees currently accrued or outstanding (when modeled as state)
settlement state
relevant economically material metadata that the World declares as state
```

### Category distinctions (mandatory)

| Category | Meaning | Authority |
| --- | --- | --- |
| **State** | Authoritative economically relevant values at a logical point | Canonical truth for evaluation |
| **Configuration** | World rules / scenario settings that interpret and constrain behavior | Authoritative as rules, not as evolving balances |
| **Derived values** | Values recomputed from authoritative state (+ declared rules) | Not an independent truth source |
| **Metadata** | Descriptive/annotative data | Economic only if World declares it material; otherwise non-authoritative for economics |
| **Execution artifacts** | Engine operational outputs (logs, timings, diagnostics) | Not economic truth unless explicitly promoted into Evidence/Result contracts |

### Ownership / authority

The deterministic domain engine owns canonical `EconomicState` inside a
Simulation.

External systems and LLMs do not own canonical state.

### Required properties

* Inspectable at economically meaningful points (at least initial and final;
  intermediate points as required by Evidence/Invariants).
* Transitionable only through the deterministic transition model.
* Free of hidden mutable economic effects.

### Prohibited ambiguity

* “Soft state” in globals, thread-locals, or adapter caches that affect PASS/FAIL.
* Treating derived caches as independently mutable truth.
* Mixing configuration and balances in one undifferentiated bag without
  declared category.

### Relationships

* Produced/consumed by Transitions driven by evaluated Actions.
* Evaluated by Invariants.
* Recorded via Events / Evidence.
* Compared across Simulations for Determinism / Regression.

### Deferred

* Exact state schema and mandatory fields for Gate 1 (`DC` / kernel specs).
* Which prices/fees are configuration vs state in particular Worlds.

---

## 6. Actor

### Purpose

An `Actor` is an economically relevant participant.

### Semantic meaning

An Actor may be (non-exhaustive):

```text
human
AI agent
merchant
customer
protocol
market participant
service provider
other economic participant
```

**“AI agent” is not a special economic authority.**

The engine reasons about economically relevant actions and effects, not about
trusting the actor’s intelligence source.

### Ownership / authority

Actor identity for economic purposes is defined by the World/Scenario.

Authorization to affect accounts/assets is a World/Scenario economic rule, not
an IAM platform concern.

### Required properties

* Stable identity within a Scenario/Simulation.
* Explicit association to zero or more Accounts (see Account).
* No implied economic privileges from being labeled “AI”.

### Prohibited ambiguity

* Treating LLM provenance as proof of authorization.
* Assuming one Actor ≡ one Account.
* Collapsing Actor with Account or Asset.

### Relationships

* Actor → Account (many-to-many allowed).
* Actor initiates or is counterparty to Actions / Transactions.
* Actor may appear in authorization checks with economic effect.

### Deferred

* Identity federation, DID/SSO, and credential schemas.
* Exact actor taxonomy enums.

---

## 7. Account

### Purpose

An `Account` is a container of economic holdings/positions under World rules.

### Semantic meaning

Examples (non-exhaustive):

```text
wallet
cash account
escrow account
trading account
merchant account
settlement account
```

### Relationships (normative shape)

```text
Actor → Account   (zero, one, or many; not assumed 1:1)
Account → Asset/Balance  (zero, one, or many assets; not assumed single-asset)
```

### Ownership / authority

Account existence and ownership rules are World/Scenario-defined.

Balances held in an Account are part of `EconomicState` (or deterministically
derived therefrom).

### Required properties

* Explicit identity.
* Explicit ownership/control relationship to Actors as defined by World rules.
* Ability to hold multiple Assets/Balances unless a World forbids it.

### Prohibited ambiguity

* Assuming every Actor has exactly one Account.
* Assuming every Account holds exactly one Asset.
* Treating “account” as synonymous with “actor” or “wallet product”.

### Deferred

* Account subtypes and ledger chart-of-accounts structures.
* Cross-world account mapping.

---

## 8. Asset

### Purpose

An `Asset` identifies the economic thing / instrument / unit being held or
transferred.

### Semantic meaning

An Asset is an **identity**, not a quantity and not a valuation.

Examples (non-exhaustive):

```text
USD
EUR
IDR
BTC
ETH
inventory item
token
security
credit
claim
other explicitly defined economic instruments
```

Asset identity and asset semantics must be **explicit**.

### Mandatory separation (DC-R02)

```text
Asset
  ≠ Money
  ≠ Price

Asset identity
  ≠ Asset valuation
```

Conceptual example (illustrative only; not an API):

```text
BTC          = Asset
1 BTC        = Money (quantity of that Asset)
BTC/USD      = Price (valuation / exchange relationship)
```

### Required properties

* Explicit asset identity within a World.
* Explicit semantic declarations needed for correctness (as applicable):
  fungibility, decimal/unit semantics, transferability, etc.

### Prohibited assumptions

Do **not** silently assume all assets are:

```text
fungible
interchangeable
same-decimal
same-price
same-risk
```

### No silent conversion

Conversion or valuation between Assets requires an explicitly defined World
rule and/or Action. Do **not** assume:

```text
1 BTC → USD
```

has meaning without a defined conversion/valuation mechanism.

### Ownership / authority

Asset definitions are World configuration (rules). Holdings of assets are State
(as Money quantities / Balances).

### Relationships

* Asset ↔ Money / Balance quantities.
* Asset may appear in Price relationships, Fees, Settlement, and Conservation
  invariants.
* Price is **not** an Asset and **not** Money.

### Deferred

* Asset registry format.
* Cross-asset conversion algorithms (rules must be explicit when used).
* Risk models.

---

## 9. Money

### Purpose

`Money` is a quantified amount of one explicitly identified Asset/currency.

### Semantic meaning

Conceptually:

```text
Money = amount + Asset + unit semantics
```

Money is therefore a **quantity of an Asset**, not a different asset category by
default.

At minimum, Money conceptually identifies:

```text
amount
asset / currency
precision or unit semantics
```

### Mandatory separation (DC-R02)

```text
Money ≠ Asset
Money ≠ Price
```

A Money value does not by itself define a Price. A Price may *refer* to Money
quantities of quote/base units, but remains a distinct semantic object (§29).

### Normative prohibition

> Binary floating-point values are **not** authoritative representations of
> economic amounts.

Floating-point may appear only as non-authoritative display/debug artifacts, never
as canonical economic truth.

### Ownership / authority

Canonical Money values, when present in State/Transactions, are under
deterministic domain authority.

### Required properties

* Association to an Asset.
* Explicit unit/precision semantics once a representation is chosen.
* Exactness suitable for deterministic comparison once representation is frozen.

### Explicitly unresolved (OD-02)

**OD-02 remains open.** This contract does **not** select:

```text
integer minor units
fixed-point decimal
arbitrary precision decimal
rational representation
integer-based domain units
other formally specified representation
```

Selection requires explicit later decision with ADR/spec amendment; not
implementation convenience.

### Prohibited ambiguity

* Using `f32`/`f64` as canonical balances.
* Implicit currency without Asset identity.
* Mixing unit scales without declared conversion.
* Treating Money as synonymous with Price or Asset identity.

### Relationships

* Money appears in Balances, Fees, and Transaction effects.
* Money may participate as the quantified side of a Price relationship.
* Money does not redefine Asset identity.

### Deferred

* Final numeric representation (OD-02).
* Rounding/allocation algorithms.

---

## 10. Balance

### Purpose

A `Balance` is an Account’s holding of an Asset expressed as Money quantities
together with economically meaningful facet / status character under World rules.

### Semantic meaning — quantity vs facet vs status (DC-R01)

The contract distinguishes:

```text
quantity
  = how much of an Asset (Money)

state / facet
  = which declared balance facet(s) that quantity occupies
    (e.g. available, reserved, pending, settled — names indicative)

economic status
  = World-defined economic meaning attached to those facets / holdings
    (e.g. spendable, encumbered, awaiting settlement finality)
```

These dimensions must not be silently conflated.

### Indicative facet names

At minimum, the domain recognizes these **indicative** facet names:

```text
available
reserved
pending
settled
```

A World may define additional facets, fewer facets, or a different architecture.
**Not every World must use the same balance architecture.**

### No universal facet algebra (DC-R01)

Do **NOT** globally assert:

```text
available + reserved + pending + settled = total
```

unless the World explicitly defines those facets as mutually exclusive components
of one quantity.

### World-declared facet model (mandatory)

A World **must** declare whether its balance model treats facets as:

```text
mutually exclusive buckets
```

or

```text
orthogonal state dimensions
```

or **another explicitly specified model**.

Silent conflation of these models is prohibited.

### Transition rule

The following must **not** happen implicitly:

```text
pending → available
reserved → available
unsettled → settled
```

Every transition between economically meaningful balance facets must have an
**explicit semantic cause** (Action evaluation / World rule / declared
settlement process).

### Settled ≠ available (DC-R01)

`settled` is **not** automatically synonymous with `available`.

A World may define settled holdings that remain restricted / non-spendable.

### Pending ≠ reserved ≠ available (DC-R01)

`pending` does **not** inherently mean `reserved` or `available`.

The World must define those relationships explicitly.

### Kernel requirement (DC-R01)

The eventual Economic Kernel must implement the balance model **declared by the
World**, rather than impose one universal facet algebra.

This contract does **not** choose the final data structure.

### Negativity rule (explicit)

**Negative balances are not a default stand-in for debt.**

Negative balance amounts are:

```text
permitted only under explicitly defined liability/credit semantics
```

within a given World.

They are **not** globally licensed by default. A World that permits them must
define the liability/credit meaning explicitly so that debt is not silently
conflated with “negative available cash”.

Preferred modeling guidance: represent debt/liability as explicit economic
constructs rather than overloading a simple negative available balance — unless
a World explicitly defines that overloading.

### Ownership / authority

Balances are authoritative State (or deterministic projections of State).

### Required properties

* Bound to Account + Asset (unless a World defines a different explicit binding).
* Facet model explicitly declared by the World.
* Transition causality explicit.

### Prohibited ambiguity

* Silent facet promotion.
* Universal arithmetic across facets without World declaration.
* Equating settled with available by default.
* Equating pending with reserved or available by default.
* Using negativity to mean debt without World declaration.
* Hidden reserve buckets outside State.

### Relationships

* Balance ↔ Money, Account, Asset, Settlement, Fees.
* Conservation/consistency invariants often constrain Balances **as defined by
  the World/Scenario**, not by a global facet sum law.

### Deferred

* Exact facet set and transition graph for particular Worlds (`DC-01`).
* Multi-leg reservation protocols.
* Concrete kernel data structures.

---

## 11. Action

### Purpose

An `Action` is economically consequential **intent** submitted for evaluation.

### Semantic meaning

Conceptual examples (non-exhaustive, not an API):

```text
Pay
Buy
Sell
Transfer
Refund
Withdraw
Deposit
Borrow
Repay
Allocate
SetPrice
```

### Mandatory distinction

```text
ACTION / INTENT
        ≠
SUCCESSFUL ECONOMIC TRANSITION
```

An action request must **not** automatically imply success.

### Evaluation outcomes (conceptual)

An Action, when evaluated under World + State, yields a deterministic
disposition, for example (indicative classes):

* rejected as invalid under World rules;
* accepted with successful economic effects;
* accepted with partial effects only if World explicitly defines partiality;
* failed due to economic conditions (e.g., insufficient available balance)
  under explicit failure semantics.

Invalid actions must have **deterministic** outcomes.

### Ownership / authority

Actions may be produced by Actors, scenario scripts, adapters, or (non-
authoritatively) LLM proposals. Only deterministic evaluation creates
authoritative effects.

### Required properties

* Explicit action type/meaning under a World.
* Explicit actor/counterparty references as required by the action type.
* Deterministic evaluation given World + State + configuration + seed scope.

### Prohibited ambiguity

* Treating request receipt as settlement.
* Non-deterministic rejection/acceptance under identical inputs.
* API invention in this document.

### Relationships

* Action → Transaction (dispositioned execution record).
* A Transaction may produce zero, one, or many State Transitions (DC-R03).
* Action may cause Events when recorded/evaluated.
* Actions appear in Scenarios and Evidence.

### Deferred

* Exact action catalog and parameter schemas.
* Idempotency keys binding (`DC-07`).

---

## 12. Transaction

### Purpose

A `Transaction` is an economically relevant **execution record** of an evaluated
Action attempt.

### Semantic meaning

At minimum, a Transaction conceptually accounts for:

```text
transaction_id
actor
action_type
inputs
outputs
fees
state_effect
ordering information
execution result / disposition
```

### Attempted vs successful (normative choice)

This contract adopts the following **canonical distinction**:

```text
Action
  = intent submitted for evaluation

Transaction
  = authoritative record of that evaluation’s disposition
    (including failed/rejected attempts when recorded)

State Transition
  = an authoritative change in EconomicState

Successful economic effect
  = Transaction dispositions that apply accepted economic state effects
    under World rules
```

**Failed or rejected attempts, when recorded, are Transactions with a
non-success disposition.** They are not “successful transactions”.

They must **not** be silently omitted from history if Evidence/replay claims
depend on them. Whether a World/Scenario requires recording all attempts is a
scenario completeness concern; when recorded, semantics above apply.

(`DC-02` remains for finer disposition taxonomies and mandatory-recording
rules.)

### Cardinality: Transaction vs State Transition (DC-R03)

A Transaction may result in:

```text
zero authoritative economic state changes
one authoritative state change
multiple atomic / related state changes
```

depending on its disposition and World semantics.

Examples:

```text
Rejected Action
    ↓
Transaction recorded
    ↓
0 economic state changes
```

```text
Transfer
    ↓
Transaction
    ↓
debit + credit + fee effects
  (multiple related state changes)
```

The specification must **NOT** force:

```text
Transaction = exactly one State Transition
```

### Atomicity (DC-R03)

State effects belonging to one atomic Transaction must either:

```text
all apply according to World rules
```

or:

```text
none apply
```

when the World defines the operation as atomic.

If a World explicitly supports partial execution, partiality must be declared
and represented. This contract does not define the implementation mechanism for
atomicity.

### Transaction without economic effect (DC-R03)

Explicitly permitted:

```text
Transaction exists
+
no economic state effect
```

for rejected / failed / non-economic-effect dispositions when the Scenario
records the attempt.

### Ownership / authority

Transactions are produced by the deterministic engine as part of Simulation
history.

### Required properties

* Stable `transaction_id` within a Simulation.
* Explicit disposition/result.
* Explicit state_effect summary (including “no economic effect” and, when
  applicable, multiple related effects).
* Ordering information compatible with §14.

### Prohibited ambiguity

* Equating “transaction exists” with “funds moved”.
* Forcing one-transaction-one-transition as a universal model.
* Dropping failed attempts from claimed complete histories without declaration.
* Using Transaction as a generic log bag for unrelated metadata.
* Applying partial effects under an atomic World rule without declared
  partiality semantics.

### Relationships

* Transaction records Action evaluation.
* Transaction may emit/correlate Events.
* Transaction may produce zero, one, or many State Transitions.
* Invariants may evaluate over Transactions/history.

### Deferred

* Exact disposition enum (`DC-02`).
* Multi-leg/atomic transaction graph representation details.

---

## 13. Event

### Purpose

An `Event` is an economically relevant **occurrence** useful for history,
inspection, replay, causal analysis, and evidence.

### Semantic meaning

Events form an inspectable occurrence stream. They are not merely debug logs.

### Mandatory distinctions (DC-R03)

| Concept | Meaning |
| --- | --- |
| **Action** | Economically consequential intent |
| **Transaction** | Dispositioned Action evaluation record |
| **State Transition** | Authoritative change in `EconomicState` |
| **Event** | Occurrence in the economic/execution history stream |
| **Execution metadata** | Operational/non-economic artifact unless explicitly declared material |

These concepts must **not** be collapsed into one generic undifferentiated
record type at the semantic level.

Storage may later be shared, but semantics must remain distinct.

Cardinality reminder: one Transaction may correlate with zero, one, or many
State Transitions and zero or more Events.

### Typical event kinds (illustrative)

* action received
* transaction dispositioned
* state transition applied
* invariant evaluated
* violation recorded
* fault injected (chaos)
* world-mutation applied (only if explicitly permitted)

### Ownership / authority

Events are engine-authored under the Simulation.

### Required properties

* Ordering relative to the Simulation’s logical order.
* Correlation to Transactions / Actions / Invariants as applicable.
* Sufficient content for Evidence questions in §21.

### Prohibited ambiguity

* Calling everything an “event” without distinguishing Transactions.
* Using wall-clock-only event IDs as sole ordering.

### Relationships

* Events reference or encapsulate Transactions and transitions.
* Evidence is largely constructed from Events + State snapshots + Results.

### Deferred

* Exact event taxonomy and correlation model (`DC-03`).

---

## 14. Ordering and Time

### Purpose

Define how AivoGuard establishes **when** things happened for economic
correctness.

### Normative rule

AivoGuard must **not** depend on wall-clock time to establish economic
correctness.

### Distinctions

| Concept | Role |
| --- | --- |
| **Logical ordering** | Authoritative order of economic occurrences |
| **Sequence number / logical tick** | Carriers of logical order (conceptual) |
| **Event ordering** | Events must be orderable under the logical model |
| **Wall-clock timestamp** | Optional annotation for humans/integrations; **not** authoritative ordering |

If timestamps exist, they must not silently become the authoritative ordering
mechanism.

### Ownership / authority

Logical order is owned by the Simulation’s deterministic execution model.

### Required properties

* Total or otherwise explicitly defined order sufficient for replay claims.
* Independence from host clock nondeterminism for PASS/FAIL.

### Prohibited ambiguity

* Sorting solely by `SystemTime` for economic results.
* Unordered concurrent economic effects without a declared concurrency model.

### Deferred

* Exact ordering structure (totally ordered log vs logical time lattice)
  (`DC-04`, `DC-05`).
* Whether logical ticks are global or per-partition.

---

## 15. Scenario

### Purpose

A `Scenario` is a controlled **test specification**.

### Semantic meaning

Conceptually a Scenario contains:

```text
world configuration
initial state
actors
action source
action sequence
fault injections
adversarial conditions
invariants
seed / configuration
test / scenario expectations (when declared)
```

### Required properties

A Scenario must be:

```text
reproducible
serializable in principle
explicit
versionable
inspectable
```

### Initial-state authority (DC-R05)

```text
Scenario initial state is an explicit declared input.
```

The initial `EconomicState` used by a Simulation must be:

```text
declared
inspectable
versionable
reproducible
```

It must **NOT** be silently derived from:

```text
current wall-clock time
host environment
process state
global mutable state
undeclared external API calls
hidden randomness
machine-local configuration
```

unless those dependencies are explicitly captured by the Scenario and included
in its reproducibility identity.

### Generated initial state (future-capable; not designed here)

A Scenario may later support generated initial state, but if so:

```text
generator
configuration
seed
version
inputs
```

must become explicit deterministic inputs. This contract does **not** design the
generator.

### External snapshots (DC-R05)

If an initial state is imported from an external system, the imported snapshot
must be explicitly captured as Scenario input.

The external system does **not** automatically remain a hidden live dependency of
the Simulation.

### Ownership / authority

Scenario authors (humans/tools) own Scenario definitions. LLM-generated
scenarios are proposals until validated against this contract’s requirements.

### Prohibited ambiguity

* Implicit initial state.
* Hidden seeds.
* Live undeclared external dependencies for initial state.
* Unstated invariants that later appear in economic evaluation.
* Collapsing economic FAIL with test FAIL without declared expectations (§16–§19).

### Relationships

* Scenario → Simulation input.
* Scenario + expected result → RegressionCase.
* Scenario binds World + initial EconomicState + execution plan + expectations.

### Deferred

* Serialization format (OD-03).
* Scenario DSL/language (related OD-04/OD-07).
* Exact expectation/assertion language (`DC-13`).

---

## 16. Invariant

### Purpose

An `Invariant` is an economic property that must hold for a defined evaluation
scope.

### Semantic meaning

Simple conceptual form:

```text
Invariant(state) -> PASS | FAIL
```

However, invariants may require:

```text
state
+
transition
+
history
+
world rules
```

Therefore invariants must **not** be constrained to only single-state predicates.

### Economic truth vs test expectation (DC-R04)

Invariant evaluation answers **economic truth**:

```text
Did the defined invariant hold?
```

If it did not:

```text
Invariant evaluation = FAIL
Violation = recorded
```

This is an **economic evaluation result**.

It does **NOT** automatically mean the Scenario/test failed.

A test may intentionally create a hostile or invalid scenario. Therefore:

```text
Economic invariant failure
        ≠
Test failure
```

unless the Scenario/Test expectation declares that relationship.

Conceptual layers (mandatory):

```text
Economic Evaluation Result
        ↓
Invariant PASS / FAIL
        ↓
Violation Evidence
        ↓
Scenario / Test Expectation
        ↓
Simulation / Test Outcome
```

Example (illustrative):

```text
Scenario:
    attempt double spend

Economic observation:
    second spend rejected / double-spend invariant fails as expected

Test expectation:
    satisfied
```

Exact assertion mechanism remains deferred (`DC-13`). Do not invent an API here.

### Required conceptual fields

```text
invariant identity
expected condition
evaluation scope
result
failure evidence
```

### Ownership / authority

Invariant definitions are Scenario/World/test artifacts.

**Invariant evaluation is deterministic and engine-authoritative.**

LLMs must **not** be authoritative invariant evaluators.

### Evaluation questions that must be answerable

```text
Which invariant?
Which version?
Against which state?
Against which transition/history?
What was expected?
What was observed?
Why did it fail?  (machine-verifiable basis; prose optional)
```

### Relationships

* Invariant FAIL → Violation (economic truth layer).
* Evaluated during/after Simulation per Scenario.
* Simulation/Test Outcome depends on declared expectations, not solely on
  Violation presence.
* Categories listed in §27 (conservation etc.) are optional categories, not
  universal laws.

### Deferred

* Invariant language/API (OD-04).
* Scheduling of evaluation points.
* Expectation/assertion composition (`DC-13`).

---

## 17. Violation

### Purpose

A `Violation` is a deterministic record that an Invariant did not hold.

### Semantic meaning

A Violation establishes at least:

```text
which invariant
which scenario
which simulation
where in execution (logical order)
what was expected  (by the invariant)
what was observed
relevant causal context
```

A Violation is part of **economic truth / evidence**, not automatically a test
failure (DC-R04).

### Ownership / authority

Produced by deterministic invariant evaluation.

### Required properties

* Machine-verifiable linkage to Invariant identity/version.
* Sufficient causal context for Evidence.
* Distinct from Engine Error (§24, §40).
* Distinct from Scenario/Test expectation mismatch.

### Prohibited ambiguity

* Encoding engine crashes as Violations without explicit mapping (default: no).
* Vague “something went wrong” without expected vs observed.
* Collapsing `Violation` into `test FAIL` without declared expectations.

### Relationships

* Violation records economic invariant failure.
* Whether the overall Simulation/Test Outcome is FAIL depends on Scenario/Test
  expectation and the declared result contract (§19), not solely on Violation
  existence.
* Violation feeds Evidence and Regression baselines carefully.

### Deferred

* Serialization (OD-06).
* Expected-violation / adversarial expectation modes (`DC-13`).

---

## 18. Simulation

### Purpose

A `Simulation` executes a Scenario against an EconomicWorld and EconomicState.

### Semantic meaning

```text
EconomicWorld
+
Initial EconomicState
+
Scenario
+
Action Sequence
+
Deterministic Configuration
↓
Simulation
↓
Result
```

### Boundary

The Simulation **must not** mutate external production systems as part of the
core domain model.

External adapters are separate boundaries (§26).

### Ownership / authority

The deterministic engine runs the Simulation and owns intermediate canonical
state.

### Required properties

* Closed over declared inputs for reproducibility claims.
* Explicit configuration + seed scope.
* Initial `EconomicState` is a declared Scenario input (DC-R05); not silently
  derived from host/wall-clock/live external state.
* Produces SimulationResult + Evidence inputs.

### Deferred

* Execution scheduling model (OD-05).
* Parallelism/concurrency model.

---

## 19. SimulationResult

### Purpose

`SimulationResult` is the authoritative outcome summary of a Simulation.

### Status classes (mandatory minimum)

```text
PASS
FAIL
ERROR
```

### Normative rule

> An engine/system error must never silently become an economic PASS.

### Result layers must not be conflated (DC-R04)

The meaning of PASS / FAIL / ERROR depends on the declared result contract.
At minimum, the following layers must remain conceptually distinct:

```text
economic evaluation status
  (invariant PASS/FAIL, violations recorded)

execution status
  (completed evaluation vs ENGINE ERROR / CONFIGURATION ERROR / INVALID SCENARIO)

scenario / test expectation status
  (whether observed economic + execution outcomes match declared expectations)
```

Do **not** collapse:

```text
economic FAIL
```

into:

```text
test FAIL
```

without considering the declared Scenario/Test expectation.

Gate 1 (and/or a subsequent result-contract specification) must define the exact
result composition before implementation (`DC-13`). This document does not invent
additional enums beyond the minimum three unless later freeze review requires it.

### Conceptual contents

```text
initial state
final state
events
transactions
invariant results
violations
execution metadata
reproduction information
expectation comparison inputs (when expectations are declared)
```

### Ownership / authority

Engine-produced economic/execution facts are authoritative.

Scenario/Test expectation matching is authoritative only relative to declared
expectations; it does not rewrite economic truth.

### Prohibited ambiguity

* Boolean-only outcomes that hide ERROR vs FAIL.
* PASS on partial/aborted engine failure.
* Treating any Violation as automatic test FAIL without expectations.
* Treating absence of Violations as automatic test PASS when expectations
  require specific economic outcomes.

### Deferred

* Wire format (OD-06 / related).
* Exact result composition / expectation matching (`DC-13`).
* Extended status taxonomies beyond the minimum three.

---

## 20. RegressionCase

### Purpose

A `RegressionCase` preserves economically important behavior for future replay.

### Semantic meaning

Defines relationship among:

```text
scenario
expected result
engine version
comparison semantics
```

### Required properties

* **Immutable by default.**
* If changed, reason and version transition must be explicit.
* Comparison must reference declared authoritative outputs (not incidental
  metadata).

### Ownership / authority

Test/maintainers own RegressionCase definitions; engine evaluates replay.

### Prohibited ambiguity

* Quiet edits to expected results without version/reason.
* Comparing non-authoritative fields as if economic truth.

### Deferred

* Exact comparison operators and tolerance rules (none for money via floats).
* Storage layout.

---

## 21. Evidence

### Purpose

Evidence is a first-class domain output enabling inspection of economic truth
claims.

### Questions Evidence must support

```text
WHAT happened?
WHEN / IN WHAT ORDER?
WHICH action caused it?
WHAT was the state before?
WHAT was the state after?
WHICH invariant was evaluated?
WHAT was expected?   (invariant expectation and, separately, test expectation)
WHAT was observed?
WHY did it fail?     (economic failure and/or expectation mismatch — distinguished)
CAN the result be reproduced?
```

Evidence must preserve the DC-R04 separation between economic truth and test
expectation.

### Authority

Evidence facts are deterministic artifacts.

Evidence must **not** depend on an LLM explanation to establish factual truth.

An LLM may later provide human-readable explanation **derived from**
deterministic evidence.

### Relationships

* Built from States, Events, Transactions, Invariant results, Violations,
  SimulationResult, Scenario identity, engine version, seeds/config.

### Deferred

* Evidence serialization schema (OD-06).

---

## 22. Determinism

### Deterministic equivalence inputs

At minimum, equivalence claims require:

```text
same EconomicWorld
+
same initial EconomicState   (declared Scenario input; DC-R05)
+
same Scenario
+
same action sequence
+
same configuration
+
same seed
+
same engine version
```

Initial state must be part of the reproducibility identity. Hidden live
dependencies for initial state are prohibited unless explicitly captured.

### Authoritative outputs for equivalence

“Equivalent” means semantic equality of at least:

```text
state transitions (authoritative EconomicState changes)
transaction dispositions and economically material fields
event occurrences that are declared economically material
invariant results
violations
SimulationResult status (PASS/FAIL/ERROR) and economically material result fields
```

Non-authoritative differences (e.g., wall-clock timestamps, host paths, log
colors, non-material execution metadata) must **not** break determinism claims
unless a Scenario explicitly includes them in comparison scope.

### Deferred

* Formal equality operators and canonicalization rules (`DC-12`).
* Engine version compatibility policy details.

---

## 23. Randomness

### Normative rule

Randomness must never become **hidden** economic behavior.

### If randomness is permitted

The following must be controllable and reproducible:

```text
seed
source
scope
consumption / order
```

Hidden randomness is prohibited.

### Deferred

* RNG algorithm/implementation choice.
* Which economic processes may draw randomness.

---

## 24. Failure Semantics

### Mandatory classes

```text
VALID ECONOMIC OUTCOME
INVALID ECONOMIC OUTCOME
ENGINE ERROR
CONFIGURATION ERROR
INVALID SCENARIO
```

These must not collapse into one boolean.

### Mapping to result layers (conceptual; DC-R04)

| Class | Economic / execution meaning | Test outcome |
| --- | --- | --- |
| Valid economic outcome satisfying invariants | Economic evaluation PASS; execution completed | Depends on Scenario/Test expectation |
| Validly executed economics that violate invariants | Economic Violation recorded; execution completed | Depends on whether Violation was expected |
| Engine cannot correctly execute/evaluate | ENGINE ERROR | Must not become economic PASS; typically ERROR |
| Configuration unusable | CONFIGURATION ERROR | Typically ERROR |
| Scenario not well-formed | INVALID SCENARIO | Typically ERROR |

### Normative inequalities

```text
ENGINE ERROR ≠ ECONOMIC PASS
INVALID INPUT ≠ VALID ECONOMIC EXECUTION
ECONOMIC VIOLATION ≠ ENGINE ERROR   (default; no silent mapping)
ECONOMIC VIOLATION ≠ TEST FAIL      (without declared expectation)
```

### Deferred

* Exhaustive error taxonomy codes.
* Exact composition of SimulationResult statuses from these layers (`DC-13`).

---

## 25. Economic Authority

```text
DETERMINISTIC DOMAIN ENGINE
        =
ECONOMIC AUTHORITY
```

LLM output is never authoritative over:

```text
economic state
balances
transactions
state transitions
settlement correctness
invariant evaluation
PASS / FAIL
```

Adapters must translate.

Adapters must not redefine economic truth.

---

## 26. External System Boundary

### Conceptual flow

```text
External System
      ↓
Adapter
      ↓
AivoGuard Domain Model
      ↓
Deterministic Economic Engine
      ↓
Evidence / Result
```

### External systems (examples)

```text
payment systems
blockchains
x402 systems
trading systems
marketplaces
agent runtimes
databases
```

No external system is automatically authoritative inside the AivoGuard test
domain.

If an initial state is imported from an external system, the imported snapshot
must be explicitly captured as Scenario input (DC-R05). The external system does
not automatically remain a hidden live dependency of the Simulation.

Mapping semantics must eventually be explicitly specified (`DC-11`).

### State transition authority

All authoritative economic transitions originate from the deterministic domain
transition model.

External observations may inform scenario setup or adapter mappings.

They must not silently mutate canonical economic state.

---

## 27. Conservation

### Principle

Conservation is an **invariant category**, not a universal hard-coded law of
every possible `EconomicWorld`.

Examples of conservation-related categories:

```text
asset conservation
balance conservation
supply conservation
liability consistency
```

These may apply in some Worlds/Scenarios and not others.

### Prohibited ambiguity

* Hard-coding global conservation into the kernel as if all Worlds conserve the
  same quantities.
* Claiming conservation without stating conserved quantity and scope.

---

## 28. Fees

### Principle

Fees are economically consequential state effects.

A fee must not disappear from economic accounting merely because it is
implementation metadata.

### Explicit representation (conceptual)

The domain must allow representation of:

```text
fee payer
fee recipient
fee amount
fee asset
fee timing
fee state effect
```

### Deferred

* Provider-specific fee schedules.
* Fee accrual algorithms.

---

## 29. Price

### Principle

Price expresses a **valuation / exchange relationship** between quantities of
different or identical economic units.

Conceptually:

```text
Price = quantity of quote unit per quantity of base unit
```

### Mandatory separation (DC-R02)

```text
Asset ≠ Money ≠ Price
Asset identity ≠ Asset valuation
```

Illustrative example (not an API):

```text
BTC = Asset
1 BTC = Money quantity of that Asset
BTC/USD = Price relationship
```

### Price is not automatically State

A price may be categorized as any of the following, and the category must be
**explicitly declared** by the World/Scenario:

```text
World configuration
EconomicState
External observation
Derived value
```

Derived or externally observed prices must not silently become independent
authoritative economic truth (§35, §26).

### Conceptual distinctions among price kinds

Do not assume identity among:

```text
quoted price
execution price
reference price
observed market price
```

### No silent conversion

Asset conversion/valuation via Price requires an explicitly defined World
rule/action. Presence of a Price object does not by itself authorize conversion.

### Deferred

* Market microstructure (`DC-08` and later).
* Oracle/external price ingestion mappings (`DC-11`).
* Exact Price representation (depends on OD-02 for quantities).

---

## 30. Settlement

### Principle

Settlement is economically meaningful state where applicable.

### Conceptual statuses (applicability not universal)

```text
authorized
executed
pending
settled
failed
reversed
```

Do **not** assume every economic action has all of these statuses.

### Deferred

* Payment-provider settlement semantics (`DC-09`).
* Finality models.

---

## 31. Reversal / Refund

### Principle

Reversal and refund must **not** be treated as generic deletion of history.

Economic history must remain inspectable.

A reversal/refund must have explicit economic semantics (compensating
transactions/effects), not silent rewrite of the past.

### Deferred

* Final accounting implementation (`DC-06`).

---

## 32. Idempotency

### Principle

Idempotency is a required semantic consideration for economically consequential
operations.

Do **not** assume every Action is idempotent.

### Distinctions to preserve

```text
same action
same action identifier
same economic intent
repeated execution
```

### Deferred

* Exact idempotency mechanism (`DC-07`).

---

## 33. Authorization Boundary

### Principle

Authorization may affect economic validity, but AivoGuard is **not** a generic
IAM system.

### In-scope examples

```text
actor may spend account funds
actor may transfer asset
actor may execute action
actor may settle transaction
```

### Out-of-scope

* Full identity platforms, SSO products, policy engines-as-product, etc.

Authorization rules with economic effect belong in World/Scenario economics.

---

## 34. Hidden State

### Prohibition

No hidden mutable economic state may exist outside the authoritative domain
model.

Prohibited examples:

```text
global mutable balance
implicit fee accumulator
hidden settlement flag
undeclared inventory
untracked liability
implicit debt
implicit reserve
```

If it materially affects economic correctness, it must be represented or
deterministically derived.

---

## 35. Derived State

### Distinction

```text
authoritative state
        ≠
derived state
```

Derived values may be recomputed from authoritative state (+ declared rules).

Derived values must not silently become an independent source of economic truth.

### Deferred

* Caching rules and invalidation (`DC-10`).

---

## 36. History / Replay

### Principle

Economic history must be sufficient for deterministic replay where replay is
claimed.

### Required semantic inputs (conceptual)

Replay claims depend on capturing at least the determinism inputs in §22 —
including the **declared initial EconomicState** (DC-R05) — and the
authoritative history needed to re-derive compared outputs.

### Prohibited undeclared dependencies

A result must not depend on:

```text
hidden wall-clock state
hidden randomness
hidden mutable global state
undocumented external calls
undeclared live external snapshots for initial state
```

unless those dependencies are explicitly captured by the Scenario/configuration
contract.

---

## 37. Serialization Boundary

### Normative requirement

> Serialization must preserve all semantically authoritative information
> required to reconstruct or verify the represented domain object.

### Explicitly not selected

This task does **not** choose:

```text
JSON
CBOR
MessagePack
Protobuf
Serde format
database schema
```

(OD-03 / OD-06 remain open.)

---

## 38. Versioning

Domain semantics must be versionable.

A change affecting any of:

```text
state meaning
action meaning
transaction semantics
event semantics
invariant semantics
determinism
result semantics
```

requires an explicit specification amendment.

Implementation versions must not silently alter economic meaning.

Engine version is part of determinism/regression identity (§20, §22).

---

## 39. LLM Boundary

```text
LLM = NON-AUTHORITATIVE
DETERMINISTIC DOMAIN ENGINE = AUTHORITATIVE
```

An LLM may propose:

```text
actions
scenarios
invariants
adversarial mutations
natural-language explanations
```

Proposals must pass deterministic validation before becoming authoritative
inputs/effects.

An LLM must **never** directly mutate canonical economic state.

---

## 40. Error vs Violation (restated)

```text
ECONOMIC VIOLATION
```

means: the system executed/evaluated an economic scenario and a defined
economic invariant was not satisfied.

```text
ENGINE ERROR
```

means: the system could not correctly execute or evaluate the requested
operation.

An engine error must not be reported as an economic violation unless a later
frozen specification explicitly defines such mapping.

An engine error must never become PASS.

---

## 41. Security / Economic Safety Boundary

AivoGuard is not a generic cybersecurity engine.

Economically adversarial behavior is in scope.

Distinguish:

```text
security exploit
        vs
economic consequence
```

AivoGuard’s core concern is the **economic consequence** (state, invariants,
evidence), not becoming a generic vulnerability scanner.

---

## 42. Domain invariant categories

Potential invariant categories (optional to adopt per World/Scenario):

```text
conservation
balance consistency
asset conservation
liability consistency
fee correctness
settlement correctness
authorization-related economic validity
supply constraints
collateral constraints
position constraints
state-transition validity
```

Do **not** hard-code all as universal invariants.

---

## 43. OPEN DECISIONS

Open decisions remain **OPEN**. They do **not** invalidate this frozen Gate 0
contract. They block Gate 1 / later gates only when required by the specific
implementation scope of that gate.

Gate-dependency classes:

```text
GATE-0 NON-BLOCKING  — does not prevent Domain Contract freeze (already frozen)
GATE-1 DEPENDENT     — must be resolved for the Economic Kernel subset that needs it
LATER-GATE           — deferred to post-kernel gates / product surfaces
```

### Carried from TASK-01

| ID | Description | Gate dependency | Status |
| --- | --- | --- | --- |
| OD-01 | Exact domain model (type-level / schema-level finalization) | GATE-1 DEPENDENT | OPEN |
| OD-02 | Economic numeric representation | GATE-1 DEPENDENT | OPEN |
| OD-03 | Scenario serialization format | LATER-GATE | OPEN |
| OD-04 | Invariant definition language / API | LATER-GATE | OPEN |
| OD-05 | Simulation execution model | LATER-GATE | OPEN |
| OD-06 | Evidence serialization format | LATER-GATE | OPEN |
| OD-07 | CLI / API surface | LATER-GATE | OPEN |
| OD-08 | Plugin / adapter architecture | LATER-GATE | OPEN |
| OD-09 | Language bindings | LATER-GATE | OPEN |
| OD-10 | License | GATE-0 NON-BLOCKING | OPEN |

### Domain-contract decisions

| ID | Description | Gate dependency | Status |
| --- | --- | --- | --- |
| DC-01 | Exact balance-facet sets/transition graphs per World; World must declare facet model | GATE-1 DEPENDENT | OPEN |
| DC-02 | Failed-action transaction disposition taxonomy / mandatory recording | GATE-1 DEPENDENT | OPEN |
| DC-03 | Event-vs-transaction correlation details; cardinality with State Transitions | GATE-1 DEPENDENT | OPEN |
| DC-04 | Logical ordering model details | GATE-1 DEPENDENT | OPEN |
| DC-05 | Time semantics details | GATE-1 DEPENDENT | OPEN |
| DC-06 | Reversal/refund accounting semantics | LATER-GATE | OPEN |
| DC-07 | Idempotency semantics | GATE-1 DEPENDENT | OPEN |
| DC-08 | Price representation details / microstructure | LATER-GATE | OPEN |
| DC-09 | Settlement state model details | GATE-1 DEPENDENT | OPEN |
| DC-10 | Derived-state rules / caching | GATE-1 DEPENDENT | OPEN |
| DC-11 | External-observation mapping | LATER-GATE | OPEN |
| DC-12 | Deterministic-equivalence formal operators / canonicalization | GATE-1 DEPENDENT | OPEN |
| DC-13 | Result composition: economic evaluation × execution × Scenario/Test expectation | GATE-1 DEPENDENT | OPEN |

These are not silently resolved for implementation convenience.

---

## 44. Deferred Semantics

Deferred to later specifications/ADRs:

* Rust/domain type layouts and crate boundaries
* Numeric representation (OD-02)
* Serialization formats (OD-03, OD-06)
* Invariant language (OD-04)
* Simulator scheduling (OD-05)
* CLI/API/SDK/bindings (OD-07, OD-09)
* Adapter/plugin architecture (OD-08)
* License (OD-10)
* Detailed DC-01…DC-13 resolutions
* Payment-provider / blockchain / x402 protocol mappings
* Market microstructure
* Multi-agent interaction algorithms (product module M09 details)
* Chaos/adversarial engines beyond conceptual Scenario hooks
* Initial-state generators (deterministic inputs required if introduced; DC-R05)

---

## 45. Gate 1 Preconditions

**Gate 0 is CLOSED.** This Domain Contract is **FROZEN**.

Gate 0 closure does **NOT** authorize Economic Kernel implementation.

Gate 1 — Economic Kernel — may begin **only when**:

1. **TASK-03 — Economic Kernel Specification** is explicitly authorized and
   completed/frozen as required by project process;
2. **all implementation-critical semantic ambiguities required by the chosen
   Economic Kernel subset are resolved** (via amendment/ADR as needed) — at
   minimum those the kernel cannot implement without guessing, including as
   applicable:
   * **OD-02** if Money/Balance are in Gate 1 scope;
   * **DC-01**, **DC-02**, **DC-03**, **DC-13**, and other GATE-1 DEPENDENT
     items required by that subset.

Until then:

```text
GATE 1 IMPLEMENTATION = NOT AUTHORIZED
IMPLEMENTATION OF DOMAIN TYPES = BLOCKED
```

---

## 46. Relationship to prior tasks

| Task | Status | Role |
| --- | --- | --- |
| TASK-00 | Foundation established | Buildable OSS workspace |
| TASK-01 | Product scope **FROZEN** | Product boundary & modules M01–M09 |
| TASK-02 | Domain Contract drafted | Economic domain semantics (Gate 0) |
| TASK-02R | Remediation complete | DC-R01…DC-R05 *(historical status was READY_FOR_FREEZE_REVIEW)* |
| TASK-02F | **PASS** | Domain Contract **FROZEN**; Gate 0 **CLOSED** |

This freeze does not redesign repository foundation or rewrite product scope.

---

## 47. Next step

```text
TASK-03 — Economic Kernel Specification
```

Do **not** begin TASK-03 implementation automatically from this freeze.

TASK-03 must first determine its own exact implementation-critical decisions
before any Rust domain implementation begins.

Gate 1 implementation remains **NOT AUTHORIZED** until explicitly authorized
after TASK-03.

---

## Document control

| Item | Value |
| --- | --- |
| Created by | TASK-02 |
| Remediated by | TASK-02R |
| Frozen by | TASK-02F |
| Status | **FROZEN** |
| Normative freeze | **FROZEN** |
| Gate | **GATE 0 — CLOSED** |
| Implementation | **BLOCKED** until TASK-03 authorization |
| Supersedes | None (first Domain Contract; remediated then frozen) |
| Depends on | `product-scope.md` (FROZEN) |
| Related process | `SPECIFICATION-POLICY.md` |
