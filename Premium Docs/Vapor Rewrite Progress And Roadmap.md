> [!info]
> This document is the **living implementation roadmap** for the Vapor rewrite.
>
> It records:
>
> * what has actually been architecture-proven;
> * which implemented mechanisms remain valid but require migration into the current model;
> * which conceptual checkpoints are complete;
> * which infrastructure migrations come next;
> * which runtime/composition phases follow after that;
> * which larger product systems remain future work.
>
> **`Vapor Rewrite Bootstrap.md`** records the historical beginning and Vertical Slice 0.
>
> Focused Premium Docs own the normative architecture.
>
> This document owns implementation progress and sequencing.
>
> Repository/provider/identity migration execution is additionally governed by:
>
> * **`Vapor Repository Topology And Migration Model.md`**
> * **`Vapor Repository Migration Execution Ledger.md`**

---

# How to Read This Roadmap

The rewrite contains three kinds of state.

## Proven

A mechanism has been exercised against real Vapor source/workflows and has demonstrated the intended architectural property.

```text
PROVEN
```

does not necessarily mean:

```text
final API
final names
final source topology
final UX
production complete
```

It means the important mechanism works.

---

## Proven but Requiring Migration

Some mechanisms are working but were built before the current identity/source/CLI model stabilized.

These should be treated as:

```text
working implementation
+
valuable proof
+
migration input
```

rather than:

```text
throw away everything and start again
```

Examples include historical:

```text
vapor ecosystem ...
source acquisition model
old Superworkspace/source discovery
old Development Session / Open / Focus model
old Registry ecosystem records
```

Their semantics may need reinterpretation or replacement.

Their proven underlying mechanisms remain valuable.

---

## Designed / Pending Implementation

Some architecture is now sufficiently settled normatively but has not yet been fully realized in code.

Examples include:

```text
Authority / Container / Repo / Project identity

one canonical Superworkspace

Context / Subject / Selection

first-party facility bindings

new source-repo-container / source-repo CLI

canonical Registry source topology
```

These are implementation work rather than unresolved conceptual questions.

---

# Current Position

The Vapor rewrite is well beyond its original bootstrap.

It can already operationally connect:

```text
Vapor semantic declarations
        ↓
dependency resolution
        ↓
composition
        ↓
Cargo/Rust realization
        ↓
managed build
        ↓
statically composed Vapor App
        ↓
execution
```

It can also use its own managed development environment to operate on Vapor itself.

Additional implementation pressure has proven:

```text
Library Content

Cargo inspection

Cargo dependency verification

safe Cargo reconciliation

managed Cargo execution

Project-aware Cargo resolution

pinned managed Rust

self-build/test

local deployment

Steam-oriented deployment mechanisms

Registry service/recovery concepts

source reacquisition

development-environment recovery

minimal Bevy ECS runtime

real Game extension semantics
```

The rewrite is therefore no longer primarily asking:

> Can Vapor work?

That has been answered strongly enough.

The current question is:

> **How do we migrate those working mechanisms into the source identity, development, CLI, Registry, and first-party facility architecture we now actually want?**

After that migration pressure is resolved, the runtime frontier resumes at:

> **How should an arbitrary resolved Vapor dependency graph derive effective Engine/Game constituency and a generic static Rust realization without Vapor stealing the participating ecosystem's runtime semantics?**

---

# Current Architectural Transition

The implementation currently spans two architectural generations.

## Proven older-generation mechanisms

Examples include:

```text
historical ecosystem operations

old source/bootstrap acquisition

old Registry ecosystem model

older local source/context representation

early Cargo realization

early self-hosting operations
```

## Current target model

```text
Superworkspace
    no identity

Authority
    ↓
Source Repo Container
    ↓
Source Repo / Vapor Workspace
    ↓
Project
```

with operation resolution:

```text
Context
    → Operation Subject
    → Selection
```

and first-party facilities such as:

```text
Client

Platform Server

Examples
```

The next infrastructure work is therefore primarily:

> **migration and consolidation, not another conceptual restart.**

---

# Major Proven Milestones

# 1 — Vertical Slice 0

Status:

```text
PROVEN
```

The original architecture-proving composition works:

```text
Terminal Engine

Hello World Game

Tiny Game Mod

Hello World Packagepack
```

Vapor proved:

```text
discover
→ parse
→ resolve
→ validate
→ derive Cargo realization
→ build
→ run
```

with visible participation from:

```text
Engine
Game
Game Mod
```

This remains an important regression specimen.

---

# 2 — Vapor Semantic Graph

Status:

```text
PROVEN
```

Vapor can model:

```text
Content identity

Content kind

SemVer requirements

dependency declarations

dependency bindings

resolved Content nodes

cycles

missing dependencies
```

and construct a semantic dependency graph.

The fundamental published resolved version concept remains:

```text
Vapor ID
+
Version
```

although the canonical Vapor ID itself is now understood as the Project's hierarchical identity.

---

# 3 — Generic Content Resolution

Status:

```text
PROVEN
```

Resolution is no longer fundamentally Packagepack-specific.

Conceptually:

```text
authored Content Project
        ↓
generic dependency resolution
        ↓
resolved Content graph
        ↓
kind-specific validation
```

Packagepack then derives complete runnable composition semantics above that graph.

This distinction should remain.

---

# 4 — Packagepack Composition

Status:

```text
PROVEN AT INITIAL LEVEL
```

A Packagepack can derive a complete composition containing:

```text
effective Engine

effective Game

required selected Content
```

The original implementation was deliberately narrow.

The newer dependency/target/constituency model now defines how this must generalize.

---

# 5 — Cargo Realization

Status:

```text
PROVEN, CURRENT REALIZATION STILL NARROW
```

Vapor proved:

```text
Vapor semantic graph
≠
Cargo package graph
```

and:

> **Vapor determines semantic composition; Cargo executes Rust package/build realization.**

The original generated realization still contains Vertical-Slice-specific assumptions.

Those assumptions remain scheduled for replacement rather than continued expansion.

---

# 6 — Library Content

Status:

```text
PROVEN
```

`Library` is now a first-class Vapor Content kind.

A Vapor Library provides reusable implementation/API without inherent:

```text
Engine
Game
Mod
pack
runnable composition
```

semantics.

A Library may expose:

```text
Rust types

traits

algorithms

builders

Components

Resources

Events

SystemSets

schemas

shared extension contracts
```

and participates in Vapor identity/version/dependency/publication semantics.

---

# 7 — Cargo Inspection and Reconciliation

Status:

```text
PROVEN
```

Vapor has exercised a real semantic Library relationship against editable Cargo state.

The proof demonstrated:

```text
semantic dependency exists
+
correct Cargo binding exists
→ Valid
```

and:

```text
semantic dependency exists
+
Cargo binding missing
→ Missing
```

and safe repair:

```text
Missing
→ Vapor-managed Cargo reconciliation
→ correct physical dependency added
→ Valid
```

while deliberately conflicting developer-authored Cargo state produces:

```text
Conflict
```

and Vapor refuses unsafe overwrite.

This established an important invariant:

> **Vapor may safely construct missing physical realization state when the answer is unambiguous, but it must not overwrite conflicting developer intent merely because Vapor prefers another state.**

Do not expand Cargo reconciliation speculatively.

Let new authored-project pressure reveal the next required behavior.

---

# 8 — Managed Rust/Cargo Toolchain

Status:

```text
PROVEN
```

Vapor can operate using a pinned managed Rust/Cargo environment.

The rewrite proved a self-hosting chain approximately equivalent to:

```text
bootstrap Rust/Cargo
    ↓
build Vapor
    ↓
Vapor-managed pinned toolchain
    ↓
build/test Vapor
    ↓
managed-built Vapor operates Vapor
```

The currently pinned toolchain includes Rust `1.97.0`.

The long-term physical storage arrangement belongs to the Client/User Data/toolchain model.

The important proven property is:

> **Vapor development can be controlled through a Vapor-managed toolchain instead of depending accidentally on ambient system Rust.**

First-party/root development should push strongly toward the pinned/vendored managed environment.

---

# 9 — Managed Cargo Project Resolution

Status:

```text
PROVEN
```

Managed Cargo invocation does not fundamentally require shell CWD to establish Vapor meaning.

Vapor can resolve an appropriate modeled Project and execute Cargo in the corresponding physical Rust context.

This implementation proof aligns strongly with the current rule:

```text
CWD
≠
Vapor Context
```

The exact implementation must now migrate to the canonical Project identity/source model.

---

# 10 — Role / Authorization Separation

Status:

```text
PROVEN / NORMATIVELY STABLE
```

Installed Roles are:

```text
Player
→ Composer
→ Content Developer
→ Ecosystem Developer
```

Role answers:

> What kinds of work is this local environment equipped to perform?

Authorization answers:

> May this identity perform this protected operation against this target?

Therefore:

```text
Role
≠
Authority
```

Root Authority is not a locally promotable Role.

---

# 11 — Vapor Self-Hosting

Status:

```text
PROVEN THROUGH HISTORICAL COMMAND SURFACE
```

Vapor can build/test/develop Vapor using Vapor-managed infrastructure.

Historical working operations have included forms such as:

```text
vapor ecosystem build

vapor ecosystem test
```

and the rewrite has progressed to a working semantic `vapor run` path.

These commands are implementation milestones.

The generic public `ecosystem` namespace is no longer the target CLI architecture.

The underlying self-hosting capability must survive the CLI/source migration.

---

# 12 — Local Deployment

Status:

```text
PROVEN AT ARCHITECTURE-PROVING LEVEL
```

Vapor can build active Vapor source and deploy resulting binaries into a realistic local Vapor/Steam installation context.

This established the distinction:

```text
build
    produce artifacts

local deploy
    realize an installation/deployment layout

official deployment
    deliver to protected external provider/target
```

The exact current implementation may still carry old `ecosystem` vocabulary.

Do not discard the proven deployment machinery merely because the public subject changes to:

```text
Client
```

---

# 13 — Steam Deployment Backend

Status:

```text
INITIAL BACKEND PROVEN
```

Steam-oriented deployment mechanisms exist.

A complete final production self-hosting proof remains future work.

The intended first-party semantic owner is now:

```text
Client
```

rather than one generic ecosystem deploy operation.

---

# 14 — Registry Service

Status:

```text
WORKING SERVICE / OLD IDENTITY MODEL
```

A real Registry service exists and has participated in Vapor recovery/acquisition work.

The Registry has proven important mechanisms such as:

```text
Vapor-side lookup

provider registration/linkage

source discovery

recovery bootstrap
```

Its current schema/API still reflects an older source/ecosystem model.

Therefore:

```text
Registry mechanism
    valuable/proven

Registry canonical source schema
    migration required
```

---

# 15 — Source Recovery / Acquisition

Status:

```text
ARCHITECTURE-PROVEN UNDER OLD MODEL
```

Vapor has proven a recovery path conceptually resembling:

```text
working development environment
→ replace recoverable application/source state
→ Steam/Vapor reacquisition
→ Registry/provider lookup
→ source acquisition
→ submodule initialization
→ derived-state regeneration
→ build/test
```

This demonstrated an important architectural property:

> **Vapor can recover registered remotely recoverable source and regenerate derived development state.**

The new model must re-prove this using:

```text
Authority

Source Repo Container

Source Repo

Project

one canonical Superworkspace
```

while preserving the stronger safety distinction:

```text
remotely recoverable source
≠
unique local authored work
```

---

# 16 — Minimal Bevy ECS Engine

Status:

```text
PROVEN
```

The Terminal Engine now owns a real minimal Bevy ECS application.

The intended minimal Bevy baseline remains approximately:

```text
bevy_app
bevy_ecs
```

without pulling the full Bevy application stack merely because Bevy is involved.

The Engine owns facilities such as:

```text
Bevy App / World

terminal lifecycle

input

Engine timing

Terminal framebuffer

lifecycle control

EngineSet::Configure
EngineSet::Input
EngineSet::Simulate
EngineSet::Present
```

This proved that Vapor can host a real Engine-defined ECS runtime.

---

# 17 — Bevy Is Not a Universal Vapor ABI

Status:

```text
NORMATIVE CONCLUSION
```

The Engine may naturally use:

```text
Bevy Plugins
Systems
Resources
Events
Components
```

as its extension vocabulary.

That does not mean:

```text
every Vapor behavioral Project
must be a Bevy Plugin
```

Vapor composes participants.

The Engine/Game/Library/Mod ecosystem defines the actual extension contract.

---

# 18 — Wheel Game

Status:

```text
PROVEN
```

The Game now contains a coherent tiny gameplay model rather than Hello-World-only behavior.

Its vocabulary includes concepts such as:

```text
LootTable

Wheel

GameMode

SpinStrategy

LootTableRegistry

WheelRegistry

GameModeRegistry

ActiveGameMode
```

and Game-owned scheduling/integration surfaces.

This gives later Mods real authored extension semantics to consume.

---

# 19 — Basic Game-Mod Extension

Status:

```text
PROVEN
```

The existing Game Mod demonstrates downstream use of Game-defined extension surfaces.

It can exercise patterns such as:

```text
attach a system to Game registration

extend a Game-owned LootTable

implement SpinStrategy

register a GameMode

select a GameMode
```

This proves basic Game-side extension.

It does not yet prove the richer cross-layer dependency example.

---

# 20 — Dependency / Target / Constituency Model

Status:

```text
NORMATIVE MODEL CORRECTED
IMPLEMENTATION MIGRATION PENDING
```

The richer runtime examples exposed that one dependency graph needs several semantic views:

```text
Outbound Requirements

Inbound Dependents

Runtime Foundation

Extension Target

Selected Dependency Closure

Effective Constituency
```

The governing rule is:

> **Selection comes from dependency reachability. Constituency comes from semantic classification of selected Content.**

Example:

```text
Game
├── requires Engine
├── requires Engine Mod A
└── requires Engine Mod B
```

with:

```text
Engine Mod A
    targets Engine

Engine Mod B
    targets Engine
```

derives:

```text
Effective Engine
├── Base Engine
├── Engine Mod A
└── Engine Mod B
```

without Packagepack redundantly listing those Mods again.

This correction must be reflected in resolver/composition structures before generic static realization.

---

# 21 — Source Identity / Context / Selection Model

Status:

```text
NORMATIVE MODEL NOW STABILIZED ENOUGH TO IMPLEMENT
```

Implementation pressure from self-hosting exposed ambiguity around:

```text
filesystem path

CWD

source identity

working context

operation target

GUI navigation

CLI selection
```

The old attempt introduced concepts such as:

```text
Semantic Identity

Structural Address

Local Realization

Development Session

Open

Focus

multiple realizations
```

That model is now superseded.

The current model is:

```text
one canonical Superworkspace
    no identity

Authority
    ↓
Source Repo Container
    ↓
Source Repo / Vapor Workspace
    ↓
Project
```

and:

```text
Context
    → Operation Subject
    → Selection
```

with:

```text
Context
    supplies missing left-side hierarchy

Operation Subject
    establishes natural complete operation scope

Selection
    supplies/narrows right-side topology
```

This is now implementation work.

---

# 22 — First-Party Facility Model

Status:

```text
NORMATIVE MODEL STABILIZED
IMPLEMENTATION PENDING
```

Stable first-party semantic subjects include:

```text
Client

Platform Server

Examples
```

These should be backed by trusted Registry/model bindings to current source.

Their public semantics should not be hard-bound throughout Vapor to exact repository names.

Conceptually:

```text
client
    → trusted Client facility binding

platform-server
    → trusted Platform Server facility binding

examples
    → trusted Examples facility binding
```

The backing source topology may evolve.

---

# 23 — SDK Experience Model

Status:

```text
NORMATIVE UX DIRECTION ESTABLISHED
GRAPHICAL IMPLEMENTATION FUTURE
```

The SDK is the primary first-party graphical development surface.

The old:

```text
Open
Focus
multiple Superworkspaces
multiple canonical realizations
```

targeting model is retired.

The SDK should instead project:

```text
one Superworkspace

Authority
Container
Source Repo
Project

frontend Resume State

persistent Context

operation Subject

transient Selection
```

A complete graphical SDK is **not** required before runtime composition development resumes.

---

# Current Documentation Convergence

Status:

```text
ACTIVE
```

The Premium Docs are currently being rewritten to converge on the new architecture.

This pass is intentionally broad because many files contained overlapping copies of older architecture.

The goal is not:

```text
rewrite everything forever
```

The goal is:

> **Reach one coherent set of conceptual owners before asking code/repository migration to implement them.**

---

# Documentation Ownership Goal

Each concept should have one natural normative owner.

Other documents should:

```text
reference

specialize

apply
```

rather than independently redefine it.

Examples:

```text
Context / Identity / Selection
    → Context model

public command grammar
    → CLI model

source workflows
    → Development Experience

product/runtime topology
    → Client / Platform / Runtime

operational state
    → Operational Model

GUI development behavior
    → SDK Experience

repository migration
    → Migration model + execution ledger

implementation sequence
    → this Roadmap
```

---

# Documentation Folderization

Status:

```text
DESIRED STRUCTURAL MIGRATION
CODING-ASSISTANT WORK
```

The flat `Premium Docs` layout has outgrown itself.

After conceptual ownership stabilizes, the documentation corpus should be reorganized into topic folders.

Each major folder should have a central file/index which:

```text
explains the folder's domain

lists the contained normative documents

states conceptual ownership

provides reading order

links to adjacent domains
```

The exact folder names/layout should be chosen from the final concept ownership map rather than frozen prematurely here.

The structural migration must:

```text
move files coherently

repair links

repair diagram links

repair references in code/docs

avoid duplicate normative owners

preserve Git history where practical
```

This is well suited to an integrated coding assistant operating on the real repository.

Do not hand-edit hundreds of paths independently during the conceptual pass.

---

# Repository Rename / Topology Migration

Status:

```text
DESIGNED
EXECUTION PENDING
CODING-ASSISTANT WORK
```

Target first-party Source Repo Container names:

```text
Vapor-Root
→ Vapor-Client

Vapor-Server-Root
→ Vapor-Platform-Server
```

This is not merely folder renaming.

The migration has separate waves for:

```text
provider rename

Registry provider linkage

canonical Vapor identity migration

local canonical Superworkspace layout

manifest/CLI vocabulary

historical Source Repo consolidation
```

The Repository Migration Execution Ledger owns the actual sequence.

---

# Near-Term Implementation Strategy

The next work should be divided into two broad tracks:

```text
A. architecture/source-system migration

B. runtime/composition continuation
```

Track A should establish a clean enough implementation base that Track B is no longer building on obviously retired source/context semantics.

Track A must not expand into “finish the entire Vapor product before runtime work can resume.”

---

# TRACK A — Architecture / Source-System Migration

# A0 — Finish Premium Docs Convergence

Status:

```text
CURRENT
```

Complete the current rewrite pass across remaining relevant Premium Docs.

Then perform a contradiction sweep.

The goal is:

```text
one model
+
clear conceptual owners
+
known intentional open questions
```

not merely:

```text
every file has newer wording
```

---

# A1 — Documentation Repository Structure

Status:

```text
PENDING
```

Use integrated repository tooling/coding assistance to:

```text
design topic folders from final ownership map

create central/index file for each folder

move model documents

move/reassociate diagrams where appropriate

repair relative links

repair cross-document references

repair code links where any exist

leave no duplicate old copies behind
```

This is a structural operation.

It should be performed coherently rather than through manual piecemeal moves.

---

# A2 — Provider Repository Renames

Status:

```text
PENDING
```

Execute the first migration wave:

```text
GitHub:

GHF-Studios/Vapor-Root
→
GHF-Studios/Vapor-Client

GHF-Studios/Vapor-Server-Root
→
GHF-Studios/Vapor-Platform-Server
```

Preserve working source.

Reconcile:

```text
Git remotes

Registry provider linkage

scripts

deployment configuration

provider-facing documentation/references
```

Do not simultaneously mutate canonical Vapor identity unless the execution ledger reaches that wave.

---

# A3 — Registry Source Identity Model

Status:

```text
PENDING
```

Migrate the Registry/Core model toward:

```text
Authority
→ Source Repo Container
→ Source Repo
→ Project
```

Support:

```text
provider linkage

first-party trust

facility bindings

acquisition policy

old→new identity migration
```

The currently working Registry is migration input.

Do not replace it gratuitously if its mechanisms can be cleanly adapted.

---

# A4 — Canonical First-Party Identity Migration

Status:

```text
PENDING
```

After Registry readiness, migrate:

```text
GHF-Studios/Vapor-Root
→
GHF-Studios/Vapor-Client
```

and:

```text
GHF-Studios/Vapor-Server-Root
→
GHF-Studios/Vapor-Platform-Server
```

with deterministic descendant-prefix migration.

This includes Source Repo and Project identities.

Historical identity mappings must remain explicit through the migration period.

---

# A5 — Canonical Superworkspace Realization

Status:

```text
PENDING
```

Converge local development source on:

```text
<Superworkspace>/
└── Authority/
    └── Source Repo Container/
        └── Source Repo/
```

Example:

```text
<Superworkspace>/
└── GHF-Studios/
    ├── Vapor-Client/
    ├── Vapor-Platform-Server/
    └── Loo-Cast/
```

The Superworkspace itself has no Vapor identity.

There is one canonical local realization per registered source identity in the managed model.

Preserve authored Git state during migration.

---

# A6 — Source Repo Container / Source Repo Operations

Status:

```text
PENDING
```

Replace the ambiguous generic source model with:

```text
source-repo-container

source-repo
```

Implement/align:

```text
create

acquire

inspect/status

Context navigation where appropriate
```

Container is the conservative default acquisition boundary.

Independent Source Repo acquisition is explicitly modeled rather than inferred from technical Git clonability.

---

# A7 — Explicit Parent Creation

Status:

```text
PENDING
```

Project creation should follow:

```text
create Container if intentionally requested
    ↓
create Source Repo if intentionally requested
    ↓
typed Project create
```

Example:

```text
source-repo-container create
source-repo create
game create
```

Higher-level create operations do not silently manufacture missing parents.

Diagnostics should teach the missing step.

---

# A8 — Context / Subject / Selection Core

Status:

```text
PENDING
```

Implement the now-settled resolution model:

```text
Context
    left-side resolution prefix

Operation Subject
    middle semantic owner

Selection
    right-side refinement
```

Required properties:

```text
persistent Context optional

per-operation Context override

CWD not semantic targeting

minimal unambiguous selectors allowed

ambiguity never guessed

no Selection = natural complete subject scope

explicit Selection = operation-specific refinement
```

---

# A9 — CLI Migration

Status:

```text
PENDING
```

Converge public CLI away from:

```text
ecosystem

generic source

kind-redundant existing-object operations

Open/Focus targeting
```

toward:

```text
source-repo-container

source-repo

client

platform-server

examples

typed Project creation

generic existing-object operations where appropriate

--context

--select
```

Exact final syntax for a few open details may be resolved during implementation.

The semantic model is sufficiently defined.

---

# A10 — First-Party Facility Operations

Status:

```text
PENDING
```

Model and implement shared machinery for:

```text
client

platform-server

examples
```

including:

```text
trusted identity resolution

source acquisition

build/test

operation recipes

selection legality

diagnostics

deployment primitives
```

Bespoke public semantics should reuse common Core implementation.

---

# A11 — Lightweight Operation Recipes

Status:

```text
PENDING / IMPLEMENT ONLY UNDER REAL PRESSURE
```

Introduce lightweight authored workflow configuration where first-party operations require it.

Target character:

```text
declarative workflow
+
Vapor-native primitives
+
small checked-in scripts where useful
```

Not:

```text
arbitrary application/plugin framework
```

Implement the minimum required to remove brittle hardcoded operation orchestration.

---

# A12 — Re-Prove Source Recovery

Status:

```text
PENDING AFTER SOURCE MODEL MIGRATION
```

Repeat the previous recovery proof against the new model.

Conceptually:

```text
working environment
→ remove only proven-recoverable application/source state
→ reacquire Vapor
→ Registry resolves Source Repo Containers
→ canonical Superworkspace restored
→ authored topology reconciled
→ derived state regenerated
→ build/test/run
```

Do not destroy unique local authored work as part of the proof.

---

# Track A Exit Criterion

Track A is sufficiently complete to resume deeper runtime work when:

```text
canonical source identity works

canonical Superworkspace works

source acquisition uses modeled Container/Repo semantics

Context/Selection works coherently

first-party Client/Platform subjects resolve correctly

working self-hosting/build/test paths survive migration

old ecosystem/source/Open/Focus architecture is no longer required
```

It does **not** require:

```text
complete SDK GUI

complete publication platform

complete Steam Workshop

final Installer UX

final Registry administration UI
```

---

# TRACK B — Runtime / Composition Continuation

The runtime work should resume from the existing proven Engine/Game/Mod/Cargo foundation.

Do not restart the runtime architecture merely because source topology changed.

---

# B0 — Resolver / Composition Constituency Migration

Status:

```text
NEXT RUNTIME MODEL IMPLEMENTATION
```

Update resolver/composition structures to express:

```text
Selected Dependency Closure

Runtime Foundation

Extension Target

Effective Engine Constituency

Effective Game Constituency
```

without hardcoding Mod cardinality.

The model should derive constituency from the resolved graph.

---

# B1 — First Meaningful Engine Mod

Status:

```text
QUEUED
```

Create a real Engine Mod.

A small candidate remains:

```text
Engine Diagnostics Mod
```

It might consume Engine-owned facilities such as:

```text
EngineTime

TerminalFrame

EngineSet::Present
```

The semantic proof is:

```text
Engine Mod
→ targets Terminal Engine
→ selected into dependency closure
→ classified as Effective Engine constituent
→ consumes Engine-defined API
```

It receives its own Project/manifest/Cargo realization.

Vapor Core must not acquire a fixed `engine_mod` field.

---

# B2 — Cross-Layer Game Mod

Status:

```text
QUEUED
```

Create a Game Mod requiring capabilities from several layers.

A useful form:

```text
Casino Game Mod
├── targets Wheel Game
├── requires Engine Mod capability
└── requires Library APIs
```

It should consume:

```text
Game-owned extension API

Engine-owned API

Engine-Mod-owned capability

Library APIs
```

This proves:

> A Game-side constituent may require a particular effective Engine constituency.

No artificial dependency-direction restriction should forbid this.

---

# B3 — Manual Rich Static Integration

Status:

```text
QUEUED
```

Before generating a generic realization, manually assemble one ordinary Rust composition containing at least:

```text
Base Engine

Engine Mod

Base Game

Game Mod

Libraries
```

Observe:

```text
which Cargo packages must participate

what initialization is actually required

which calls/order are genuine

what belongs to Engine semantics

what belongs to Game semantics

what information Vapor must know

what information Vapor should remain ignorant of
```

Do not prematurely impose a universal:

```text
VaporContent::install(&mut App)
```

or:

```text
every Mod = Bevy Plugin
```

contract.

---

# B4 — Discover Minimal Static Integration Contract

Status:

```text
QUEUED
```

Use B3 to determine the smallest general contract Vapor actually needs.

Possible outcomes may involve:

```text
generated registration

Engine-defined composition entrypoints

Cargo feature/package structure

authored integration metadata

macro-generated glue

some combination
```

Do not select the abstraction in advance.

---

# B5 — Generic Graph-Derived Static Realization

Status:

```text
QUEUED
```

Replace the Vertical-Slice-specific realization.

The fixed model:

```rust
struct AppPackages {
    engine: RustPackage,
    game: RustPackage,
    game_mod: RustPackage,
}
```

must disappear.

Likewise, generated assumptions equivalent to:

```rust
engine::run(|app| {
    game::install(app);
    game_mod::install(app);
});
```

must disappear unless later generalized semantics truly justify them.

Target conceptual structure:

```text
ResolvedContentGraph
        ↓
validated semantic relationships
        ↓
Effective Engine
    base + constituents
+
Effective Game
    base + constituents
+
support dependency closure
        ↓
physical Cargo realization
        ↓
minimal integration contract
        ↓
statically composed Vapor App
```

---

# B6 — Arbitrary Mod Cardinality

Status:

```text
QUEUED
```

Vapor Core must naturally support:

```text
zero Mods

one Mod

many Mods

Mods requiring other Mods

Extension Mods

cross-layer dependency graphs
```

without corresponding fixed struct fields or special branches.

The graph is the source of multiplicity.

---

# B7 — Rich Pack Examples

Status:

```text
QUEUED
```

Once generic realization works, introduce meaningful:

```text
Enginepack

Gamepack

Modpack

multiple Packagepacks
```

Example:

```text
Enginepack
├── Terminal Engine
├── Diagnostics Engine Mod
└── Weighted Roll Engine Mod

Gamepack
├── Wheel Game
└── Casino Game Mod
```

Packs remain declarative selection/composition fragments.

They are not special runtime containers.

---

# B8 — Extension-Ecosystem Explosion

Status:

```text
QUEUED
```

Deliberately create a ridiculous but small dependency topology.

For example:

```text
Casino Framework Game Mod
    ↓
Casino Framework API Library
    ↓
Roguelike Casino Extension
    ↓
ASCII Dungeon Extension
```

This should prove:

```text
an extension can itself become an extension platform
```

and that extensibility is not monotonically narrowing.

---

# B9 — Content Creation Templates

Status:

```text
QUEUED
```

Implement typed Project creation for:

```text
Library

Engine

Game

Engine Mod

Game Mod

Extension Mod

Enginepack

Gamepack

Modpack

Packagepack
```

This phase must use the new explicit parent topology:

```text
Authority
→ Container
→ Source Repo
→ Project
```

Templates should start small and built-in.

Do not create a generalized remote template marketplace without pressure.

---

# B10 — Real Loo Cast / Spacetime Pressure

Status:

```text
FUTURE AFTER TOY ARCHITECTURE PROVES GENERALITY
```

Once the toy graph can express arbitrary realistic extension topology, begin applying the model to actual:

```text
Spacetime Engine

Loo Cast

USF
```

requirements.

The toy ecosystem exists to make architectural mistakes cheap.

It should not become an endless substitute for the real project.

---

# TRACK C — Product / Platform Expansion

These phases remain important but do not all block Track B.

---

# C1 — SDK Graphical Implementation

Status:

```text
FUTURE / MAY BEGIN IN PARALLEL
```

Architecture-proving implementation order:

```text
application shell

real Role/toolchain state

canonical Superworkspace Explorer

Authority / Container / Repo / Project topology

Context navigation

GUI Selection → Operation Selection

Inspector

Problems

Build/Test

Content graphs

Git/provider views

Client / Platform Server surfaces

publication/deployment surfaces

integrated editing
```

The old Open/Focus session model must not be implemented.

---

# C2 — External IDE Integration

Status:

```text
INITIAL RUSTROVER RECONCILIATION PROVEN
FURTHER INTEGRATION FUTURE
```

Continue deriving external IDE state from:

```text
canonical Superworkspace

managed toolchain

Source Repo / Project topology

Cargo realization
```

IDE state should remain derived/reconcilable where possible.

---

# C3 — Provider Authentication and Authorization

Status:

```text
PARTIAL / FUTURE EXPANSION
```

Integrate:

```text
Git provider

GitHub

Steam

SteamCMD

Vapor Platform identity
```

while preserving:

```text
local Role
≠
remote authorization
```

Local development must remain possible without protected official authority.

---

# C4 — Publication

Status:

```text
FUTURE
```

Implement immutable Content publication:

```text
canonical Project/Vapor ID

SemVer

exact Git commit binding

provider release

Registry record

Yank/Ban
```

Publication should build on the source topology rather than define a second identity system.

---

# C5 — Packagepack Built Distribution

Status:

```text
FUTURE
```

For complete Packagepacks:

```text
published Packagepack version
→ exact historical dependency resolution
→ target build
→ provenance
→ Steam Workshop / appropriate distribution
```

Default Loo Cast remains Steam-depot-distributed.

Additional Player-facing Vapor Apps use Workshop according to current distribution direction.

---

# C6 — Production Build Provenance

Status:

```text
FUTURE
```

Record enough to identify:

```text
Packagepack version

exact resolved graph

all source commits

managed toolchain

target

build configuration

physical artifacts
```

Semantic reproducibility is required.

Bitwise reproducibility remains a strong target.

---

# C7 — Full Steam Development Self-Hosting

Status:

```text
PARTIALLY PROVEN / FUTURE COMPLETE PROOF
```

Target:

```text
build Client
→ local deployment proof
→ Steam preview/development deployment
→ launch Steam-installed Client
→ use installed Client to acquire/develop/build/test/deploy Vapor again
```

This is a strong future self-hosting milestone.

---

# C8 — Production Platform Server Operations

Status:

```text
PARTIAL SERVICE IMPLEMENTATION EXISTS
FUTURE SYSTEMATIC FACILITY MODEL
```

Converge Platform Server operations on:

```text
platform-server build

platform-server test

platform-server deploy local

platform-server deploy vps
```

with operation recipes, service scope, health checks, and authorization.

---

# C9 — Registry Archaeology Resolution

Status:

```text
PENDING
```

Resolve the historical overlap between:

```text
Vapor-Registry

Vapor-Registry-Server
```

There must be one canonical Registry authority.

Archive or narrowly repurpose the old repository.

---

# C10 — Vapor App Server

Status:

```text
FUTURE
```

Do not create:

```text
Vapor-App-Server
```

merely to complete the product topology.

Create it when actual server-side Vapor App runtime pressure appears.

---

# Source Cleanroom / Consolidation Track

The source cleanroom effort should happen in coordination with repository stabilization.

It should not become an endless prerequisite.

---

# Client Source Consolidation

Historical Client Source Repos such as:

```text
Vapor-Entrypoint

Vapor-Installer

Vapor-Launcher

Vapor-SDK

Vapor-Shell
```

should be treated as archaeology/migration sources.

Where their independent Source Repo boundaries no longer match semantic ownership:

```text
understand
→ preserve valid implementation/design
→ cleanly integrate into Vapor
→ prove replacement
→ remove submodule membership
→ archive
```

The product/application roles may remain.

The repository boundaries need not.

---

# Preserve SDK Visual Ancestry

The historical Figma/Tauri SDK prototype is not disposable.

Preserve:

```text
visual design

layout

interaction character

useful frontend implementation

design tokens/components where extractable
```

Do not preserve:

```text
obsolete SDK semantic Core

obsolete independent SDK CLI

old Open/Focus/source identity architecture
```

---

# Cleanroom Rule

For old code:

```text
inspect mechanism
→ understand why it worked
→ identify still-valid invariant
→ design simplest current equivalent
→ implement
→ prove
→ fossilize old version
```

Prefer this over:

```text
copy old implementation
→ rename types
→ accumulate compatibility layers
```

---

# Docsification

Once source ownership settles, active code should be made approachable to an uninitiated but technical developer.

Desired layering:

```text
clear code

→ module/crate documentation

→ public API docs

→ normative architecture docs

→ generated reference

→ approachable Vapor Books/tutorials
```

Do not heavily docsify repositories scheduled for imminent archival.

---

# Development Method

Continue:

> **Model enough → implement a vertical slice → observe real pressure → refine the model → continue implementation.**

The current process is evidence that this works.

Examples:

```text
simple Packagepack realization
    → revealed Cargo graph distinction

Library relationship
    → revealed conservative Cargo reconciliation

self-hosting
    → revealed source/context/session problems

richer Mod examples
    → revealed target/constituency distinction
```

Architecture should continue emerging from real pressure without allowing temporary scaffolding to become permanent merely through age.

---

# Do Not Speculate Past Pressure

Avoid expanding systems merely because they appear on the future roadmap.

Examples:

```text
do not build universal operation-recipe language
before real first-party recipes need it

do not build remote template marketplace
before templates require it

do not build App Server
before server-runtime pressure exists

do not build arbitrary provider abstraction
before another provider matters

do not generalize Cargo reconciliation
without another real conflict/state
```

---

# What Should Not Block Runtime Progress Forever

The following do **not** need to be production-complete before runtime composition work continues:

```text
full SDK GUI

Steam Workshop publication

final Launcher polish

complete App Server

all provider integrations

all Content publication workflows

final docs folder placement
```

However, the canonical source identity/Context/Selection migration should be established enough that new core work does not continue deepening obviously retired development architecture.

---

# Immediate Next Sequence

The current intended sequence is:

```text
Premium Docs convergence
        ↓
contradiction / ownership sweep
        ↓
integrated coding-assistant handoff
        ↓
documentation folderization + central indexes
        ↓
repository/provider rename wave
        ↓
Registry source identity readiness
        ↓
canonical Container identity migration
        ↓
canonical Superworkspace layout
        ↓
source-repo-container / source-repo operations
        ↓
Context / Subject / Selection implementation
        ↓
first-party Client / Platform Server facility migration
        ↓
re-prove build / test / run / recovery
        ↓
resume runtime constituency work
        ↓
first Engine Mod
        ↓
cross-layer Game Mod
        ↓
manual rich static integration
        ↓
generic graph-derived realization
```

Some independent steps may overlap where safe.

The dependency direction is what matters.

---

# Coding-Assistant Handoff Boundary

After the Premium Docs convergence pass, produce one integrated coding-assistant handoff for the structural migration.

That handoff should give the agent:

```text
final terminology

identity hierarchy

Superworkspace invariant

Registry migration requirements

provider rename sequence

documentation folderization requirements

CLI target model

Context / Selection semantics

first-party facility semantics

migration safety rules

required verification gates
```

The coding assistant should inspect actual current local state before changing:

```text
Git

.gitmodules

manifests

Registry schema/data

scripts

deployment configuration

IDE paths

documentation links
```

The normative docs define the target.

The local repository state defines the actual migration inputs.

---

# Important Migration Safety Rule

The integrated migration must never assume:

```text
remote exists
therefore local source is disposable
```

Before moving/renaming/consolidating source, preserve:

```text
dirty changes

unpushed commits

branches

detached development commits

submodule state

provider linkage
```

Derived state may be regenerated.

Authored source must be protected.

---

# Recovery Gate

After major source/identity migration, re-prove:

```text
Vapor starts

managed toolchain works

managed Cargo works

Client builds

Client tests

semantic run works

source discovery works

Registry lookup works

source acquisition works

canonical Superworkspace works

IDE integration repairs

local deployment works
```

Do not stack subsequent migration waves on a broken foundation.

---

# Runtime Return Gate

Return to deeper runtime realization once:

```text
new source identity is operational

self-hosting survives

Context/Selection semantics no longer depend on Open/Focus/CWD

first-party Client subject works coherently

old ecosystem command semantics are no longer architectural dependencies
```

Then stop polishing infrastructure and return to Engine/Game/Mod pressure.

---

# Current Summary

```text
VERTICAL / CONTENT FOUNDATION

Vertical Slice 0                         PROVEN
Semantic Content graph                   PROVEN
Generic Content resolution               PROVEN
Packagepack composition                  PROVEN initial
Library Content                          PROVEN
Cargo realization                        PROVEN initial
Cargo inspection/reconciliation          PROVEN
Managed Cargo                            PROVEN
Pinned managed Rust                      PROVEN
Vapor self-build/test                    PROVEN
Semantic run                             PROVEN


SOURCE / DEVELOPMENT

Role / Authority separation              STABLE
One Superworkspace model                 DESIGNED
Authority/Container/Repo/Project          DESIGNED
Context / Subject / Selection             DESIGNED
Source Repo = Vapor Workspace             STABLE
Container acquisition model               DESIGNED
First-party facility model                DESIGNED

Old Open/Focus/session model               RETIRED
Old multiple-realization model             RETIRED
CWD semantic targeting                     RETIRED


INFRASTRUCTURE

Local Client deployment                   PROVEN initial
Steam deployment backend                  PROVEN initial
Registry service                           WORKING, SCHEMA MIGRATION NEEDED
Source recovery                            PROVEN under old model
Canonical recovery                         RE-PROVE AFTER MIGRATION

Docs conceptual convergence                ACTIVE
Docs folderization                         PENDING
Vapor-Root → Vapor-Client                  PROVIDER RENAMED / IDENTITY+PATH PENDING
Vapor-Server-Root → Vapor-Platform-Server  PROVIDER RENAMED / IDENTITY+PATH PENDING
Registry identity migration                PENDING
CLI semantic migration                     PENDING


RUNTIME

Minimal Bevy ECS Engine                    PROVEN
Wheel Game                                 PROVEN
Basic Game Mod                             PROVEN

Dependency targeting                       MODEL STABLE
Effective constituency                     NEXT RUNTIME WORK
First Engine Mod                           QUEUED
Cross-layer Game Mod                       QUEUED
Manual rich static integration             QUEUED
Generic graph-derived realization          QUEUED
Rich Packs                                 QUEUED
Extension ecosystem explosion              QUEUED
Real Spacetime/Loo Cast pressure            FUTURE


PRODUCT

SDK semantic model                         DESIGNED
SDK implementation                         FUTURE / PARALLEL
Publication model                          DESIGNED
Publication implementation                 FUTURE
Workshop distribution                      FUTURE
Platform facility operations               PARTIAL / FUTURE
Vapor App Server                           FUTURE
```

---

# Current Frontiers

There are now two clear frontiers.

## Immediate infrastructure frontier

> **Migrate Vapor's working self-hosting/source infrastructure into the new canonical source identity, Superworkspace, CLI, Registry, Context, and first-party facility model.**

## Immediate runtime frontier after that migration gate

> **Derive arbitrary effective Engine/Game constituency from one resolved dependency graph and use real implementation pressure to discover the smallest correct generic static integration contract.**

These are concrete implementation problems.

They no longer require reopening Vapor's foundational terminology from scratch.

---

# Roadmap Invariants

* Existing proven behavior is migration input rather than disposable legacy merely because terminology changed.
* Current normative Premium Docs outrank historical implementation vocabulary.
* Vertical Slice 0 remains a regression specimen.
* Vapor semantic graph and Cargo physical graph remain distinct.
* Vapor determines semantic composition; Cargo performs Rust realization.
* Vapor does not define universal Mod/runtime extension semantics.
* Library remains first-class Vapor Content.
* Cargo reconciliation remains conservative around developer intent.
* First-party development uses the Vapor-managed toolchain where required.
* Role and Authority remain independent.
* The canonical local source root is one Superworkspace with no Vapor identity.
* Canonical source identity is Authority → Source Repo Container → Source Repo → Project.
* Source Repo equals Vapor Workspace.
* Context supplies missing left-side hierarchy.
* Operation Subject establishes natural complete scope.
* Selection supplies/narrows right-side topology.
* CWD is not semantic Vapor targeting.
* Open/Focus are not canonical Core targeting concepts.
* First-party Client, Platform Server, and Examples are semantic facilities rather than arbitrary repository aliases.
* Repository provider names, canonical Vapor identity, and local paths are distinct migration dimensions.
* Documentation folderization is structural repository work, not a new conceptual architecture.
* Source migration must preserve unique authored Git state.
* Repair is not source reacquisition or Git reset.
* Registry recovery mechanisms should be preserved while its canonical schema is migrated.
* Full SDK implementation does not block runtime architecture indefinitely.
* Static realization must derive naturally from graph topology rather than fixed Mod fields.
* Packs remain declarative composition selectors rather than runtime containers.
* Runtime extension contracts belong to participating Engine/Game/Mod/Library ecosystems.
* Future systems should be implemented only when actual pressure requires them.
* After the source/identity migration is sufficiently proven, Vapor returns to runtime work rather than polishing infrastructure forever.

---

# Open Roadmap Questions

The following remain implementation questions rather than foundational architecture blockers:

* Exact Premium Docs folder names and final index hierarchy.
* Exact final spelling of `source-repo-container` if a shorter equally precise name proves superior.
* Exact `open` / Context-navigation CLI wording.
* Exact `--select` multi-selection grammar.
* Exact Registry schema implementation for hierarchical identities and migrations.
* Exact persistent old-ID compatibility duration.
* Exact first-party operation-recipe manifest schema.
* Exact Project filesystem/Cargo layout templates.
* Exact minimal static integration contract discovered by rich manual composition.
* Exact Run-development composition model.
* Exact long-term managed-toolchain physical storage.
* Exact SDK frontend technology.
* Exact provider permission/creation UX.
* Exact publication transaction implementation.
* Exact Workshop version/artifact mapping.
* Exact point at which Spacetime Engine/Loo Cast replaces toy examples as the primary architecture pressure source.
