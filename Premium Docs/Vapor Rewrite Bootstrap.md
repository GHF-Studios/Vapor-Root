> [!info]
> This document records the architectural rationale and first proof milestone of the Vapor rewrite.
>
> It is intentionally historical.
>
> It explains:
>
> * why the rewrite was started;
> * how legacy Vapor source should be treated;
> * what Vertical Slice 0 was intended to prove;
> * what that slice actually proved;
> * which implementation principles emerged from it.
>
> It does **not** define Vapor's current architecture in full.
>
> Current normative semantics belong to the focused Premium Docs.
>
> Current implementation state and future work belong to:
>
> **`Vapor Rewrite Progress And Roadmap.md`**

---

# Rewrite Thesis

The rewrite began from a simple conclusion:

> **Existing Vapor implementation was useful evidence, but it was no longer a trustworthy architectural authority.**

Legacy source contained valuable:

```text id="ap22dy"
Steam integration

Git/provider knowledge

toolchain integration

repository-layout knowledge

implementation mechanisms

identifiers

experiments

working edge cases
```

but also embodied superseded models.

Therefore the rewrite would not preserve old internal architecture merely for compatibility.

The governing hierarchy became:

```text id="qt6znx"
current normative model
    ↓
implementation pressure
    ↓
new implementation
    ↓
legacy mechanism where still useful
```

not:

```text id="vt32jz"
legacy implementation
    ↓
new architecture must imitate it
```

---

# Legacy Source Policy

Historical Vapor source is **archaeology**.

For any old mechanism:

```text id="snz14n"
inspect
→ understand
→ identify useful invariant/mechanism
→ compare against current model
→ preserve, adapt, or cleanly reimplement
→ prove
→ fossilize obsolete source
```

Possible dispositions are:

```text id="ec1rmq"
reuse

adapt

extract

reimplement

archive

ignore
```

The choice depends on architectural fit.

---

# Avoid Both Rewrite Extremes

Do not preserve obsolete code merely because it exists.

Likewise, do not discard correct useful mechanisms merely because they are old.

The preferred question is:

> **What is the simplest implementation which satisfies the current model while preserving useful proven behavior?**

---

# Normative Design Source

The Premium Docs own the intended Vapor model.

The exact document organization may evolve, including future folderization of the documentation corpus.

Conceptually the normative areas include:

```text id="jwklja"
terminology

identity and source topology

Context and Selection

CLI semantics

development experience

operational state

Content/dependency/composition

publishing/distribution

Client / Platform / runtime

SDK experience

repository migration

implementation roadmap
```

Implementation should not reproduce architecture from this Bootstrap document where a newer focused model owns that concept.

---

# Reuse the Underlying Ecosystem

Before implementing new Vapor machinery, ask:

```text id="myn6hh"
Does Rust solve this?

Does Cargo solve this?

Does Git solve this?

Does the Git provider solve this?

Does Steam solve this?
```

If yes:

> **Use, wrap, orchestrate, constrain, or expose the existing mechanism.**

If almost:

> **Adapt minimally.**

Only create genuinely new Vapor machinery where Vapor owns semantics that the underlying tools do not provide.

---

# Vapor's Layer

Vapor primarily owns semantic relationships such as:

```text id="1tuvfm"
canonical Vapor identity

Project kinds

Content identity/versioning

dependency semantics

composition

Packagepack resolution

source topology

source acquisition policy

Context / Selection

operation semantics

managed development environment

Cargo realization orchestration

publication coordination

first-party facility semantics
```

Vapor does not need to replace:

```text id="j1u5b7"
Rust type safety

Cargo package resolution/build execution

Git source history

Git provider hosting

Steam distribution
```

---

# Semantic Graph vs Runtime Meaning

A Vapor dependency says:

> **This Project/version semantically depends on that Project/version.**

It does not automatically define the programmatic extension mechanism.

Engine, Game, Mod, Library, and other authored systems may expose extension behavior through ordinary mechanisms such as:

```text id="tkoqrs"
Rust APIs

traits

builders

registries

Bevy Plugins

Components

Resources

Events

Systems

SystemSets

data schemas

other explicitly authored contracts
```

Vapor ensures the required definitions participate in the composition.

The participating software defines what those relationships mean at runtime.

---

# Development Method

The rewrite follows:

```text id="ys4kb8"
model enough
→ implement a vertical slice
→ observe real pressure
→ refine model
→ implement next pressure point
→ repeat
```

The goal is not:

```text id="i0yxzd"
fully specify the entire future ecosystem
before writing implementation
```

Nor is it:

```text id="ocua99"
code first
and let accidental implementation choices become architecture
```

Model and implementation constrain each other deliberately.

---

# Vertical Slice 0

Vertical Slice 0 was the first architecture-proving rewrite milestone.

Its purpose was:

> **Prove that Vapor could operationally realize a complete statically composed Vapor App from Vapor-level declarations.**

Before this proof, Vapor's composition model could still have been merely descriptive.

The slice needed to demonstrate that Vapor semantics actually changed what Rust/Cargo built.

---

# Vertical Slice 0 Scenario

The original specimen contained:

```text id="wck66r"
Terminal Engine

Hello World Game

Tiny Game Mod

Hello World Packagepack
```

---

# Terminal Engine

The minimal Engine:

```text id="gq59n3"
owned the main executable

provided a tiny runtime/API

started the composition

consumed registered Game behavior

produced visible lifecycle output
```

---

# Hello World Game

The Game:

```text id="f77vwx"
depended on / targeted the Engine

participated statically in the build

exposed behavior to the Engine

produced visible output
```

---

# Tiny Game Mod

The Game Mod:

```text id="qft3oz"
targeted the Game

extended/modified its behavior

participated through the resolved composition
```

It existed to prove that a Mod was not merely an unrelated Cargo crate sitting beside the Game.

---

# Hello World Packagepack

The Packagepack:

```text id="k7nxee"
selected the Engine

selected the Game

included the Mod

formed one complete Vapor App composition root
```

---

# Intended Runtime Proof

Exact output never mattered.

Conceptually:

```text id="o4dj2b"
[Engine] starting
[Game] registered
[Mod] extended game
[Game] Hello from Vapor!
[Mod] ...on steroids.
[Engine] shutting down
```

The important fact was:

> Engine, Game, and Mod behavior all visibly participated in the one built executable produced from Vapor composition semantics.

---

# Required End-to-End Path

Vertical Slice 0 needed to prove:

```text id="gxblo6"
Vapor Projects
    ↓
authored Vapor manifests
    ↓
Content identities / kinds / dependencies
    ↓
Packagepack resolution
    ↓
exact semantic composition
    ↓
Cargo realization
    ↓
Rust/Cargo build
    ↓
Engine-owned executable
    ↓
statically incorporated Game + Mod
    ↓
Vapor App
    ↓
launch
    ↓
visible composed behavior
```

---

# The Critical Boundary

The most important requirement was:

> **Vapor determines what Cargo is asked to compile. Cargo performs the compilation.**

The slice must not be secretly pre-wired entirely through Cargo while Vapor merely described the same graph in parallel.

Bad proof:

```text id="3hzkbi"
Cargo.toml already wires:

Engine → Game → Mod

Vapor.toml describes it too

Vapor resolver has no effect
```

Real proof:

```text id="30luhv"
Vapor semantic graph
    ↓
Vapor resolution
    ↓
Cargo realization derived from that result
    ↓
Cargo build
```

---

# Vapor Graph vs Cargo Graph

Vertical Slice 0 established an important architectural distinction:

```text id="3nsv9c"
Vapor semantic dependency/composition graph

≠

Cargo physical package/build graph
```

They interact.

They are not the same model.

Vapor determines semantic intent.

Cargo remains authoritative for the Rust graph it actually compiles.

---

# Identity During the Bootstrap

The original slice initially used a much simpler Content-ID model.

The current canonical source hierarchy is now:

```text id="4kn52m"
Authority
└── Source Repo Container
    └── Source Repo
        └── Project
```

For a Content Project:

```text id="91bcri"
Project identity
=
Content identity
```

and:

```text id="q4po0o"
Project kind
=
Engine / Game / Game Mod / Packagepack / ...
```

Therefore a modern example identity may look like:

```text id="svz5a3"
GHF-Studios/Vapor-Examples/Games/Hello-World
```

according to its actual source topology.

The exact historical IDs used by Vertical Slice 0 are not normative for the current identity model.

---

# Versions During the Bootstrap

The slice used development versions sufficient to exercise dependency resolution.

The current publication model now distinguishes:

```text id="ydb7q9"
local development version/state

from

immutable published SemVer version
```

Published Content is identified semantically by:

```text id="pxpku3"
Vapor ID
+
SemVer
```

with the published version bound to an exact Git commit.

Vertical Slice 0 did not need to prove the complete publication lifecycle.

---

# Dependency Resolution Goal

The first resolver needed only enough functionality to prove composition.

It initially needed to model concepts such as:

```text id="rvwk1i"
Content identity

version

Project kind

dependency requirement

resolved node

dependency graph

Packagepack composition
```

It did not need production-grade solving from day one.

---

# Generic Resolution

One important refinement after the original slice is that dependency resolution is not fundamentally Packagepack-only.

Conceptually:

```text id="l0j09h"
any supported Content root
    ↓
generic Vapor dependency resolution
```

may produce a resolved semantic graph.

Packagepack then adds the requirements necessary for a complete runnable composition.

---

# Packagepack Validation

For a runnable Packagepack:

```text id="v97y2d"
generic resolved dependency graph
    ↓
Packagepack-specific validation
```

must ultimately produce:

```text id="ge594q"
exactly one effective Engine

exactly one effective Game

selected Mods

required Libraries/support dependencies
```

The Packagepack is the complete authored composition root.

---

# Definition vs Runtime Instance

A resolved dependency node identifies a definition participating in the composition.

It does not imply a particular number of runtime instances.

For example:

```text id="odghn0"
one Library definition
```

may be used by many systems at runtime.

Vapor's semantic dependency graph should not accidentally impose runtime object multiplicity.

---

# Minimal Manifest Strategy

The bootstrap intentionally avoided designing the final manifest language before implementation.

The rule was:

```text id="1nv2v9"
add only what the proof needs
→ implement
→ observe pressure
→ expand when justified
```

That principle remains useful.

---

# Modern Minimal Project Example

A modern Project manifest conceptually needs enough information to describe things such as:

```text id="jotx4s"
Project kind

development/published version metadata

dependencies

kind-specific relationships/configuration
```

Canonical identity should preferably derive from the known:

```text id="v7vknq"
Authority
/ Container
/ Source Repo
/ Project
```

topology rather than requiring every authored file to repeat global hierarchy redundantly.

Exact manifest ownership/schema belongs to the focused current model and implementation pressure.

---

# Temporary Hardcoding

The first slice was permitted to hardcode things not relevant to the proof.

Examples included:

```text id="kcraue"
local source only

one host platform

development-only versions

no Registry

no provider API

no Workshop

minimal Content-kind support

minimal diagnostics
```

Temporary scaffolding was acceptable only if it did not fake the thing being proven.

---

# What Could Not Be Hardcoded Away

Vertical Slice 0 had to genuinely prove:

```text id="bdc329"
Vapor manifest parsing

Vapor semantic identity/kind understanding

Vapor dependency relationships

Packagepack composition

Engine/Game validation

Mod participation

Vapor-derived Cargo realization

build

runnable artifact

runtime interaction
```

Without those, the slice would not have tested Vapor.

---

# First Implementation Sequence

Historically, the slice proceeded conceptually through:

```text id="zxb91o"
Bootstrap
    ↓
minimal Content model
    ↓
local discovery
    ↓
Packagepack resolution
    ↓
Cargo realization
    ↓
build
    ↓
run
    ↓
review
```

The exact implementation has since evolved substantially.

This sequence remains useful only as the origin story of the rewrite.

---

# Vertical Slice 0 Definition of Done

The slice was complete when:

> Starting from Vapor-level Packagepack declarations, Vapor could resolve the intended Engine + Game + Game Mod composition, derive the required Cargo/Rust realization, build a Vapor App, launch it, and visibly demonstrate behavior from all three content layers.

That boundary has been crossed.

---

# What Vertical Slice 0 Proved

The rewrite established that Vapor can meaningfully participate in:

```text id="ye3dey"
semantic Content modeling

dependency resolution

Packagepack composition

Cargo realization

Rust/Cargo build orchestration

static composition

Vapor App execution
```

This transformed Vapor from:

```text id="esdo11"
a composition description
```

into:

```text id="5an6xh"
a system capable of operationally realizing a composition
```

---

# What Vertical Slice 0 Did Not Prove

It deliberately did not require:

```text id="crw24f"
production Registry

source acquisition

Git/provider publication

Steam Workshop publication

full dependency solver

cross-platform builds

artifact signing

remote builders

production Launcher

production Installer

production SDK

complete diagnostics

final source topology

final CLI

full USF integration
```

Those concerns were allowed to emerge later under real implementation pressure.

---

# What Came After

Subsequent rewrite work proved substantially more than Vertical Slice 0, including areas such as:

```text id="gf1tyc"
generic Content resolution

Library modeling

Cargo inspection/reconciliation

managed Cargo

pinned managed Rust toolchain

self-hosting Vapor development

local deployment

source/recovery experiments

Registry-backed acquisition

CLI development

development-context pressure
```

Those milestones belong to the living Progress And Roadmap document rather than being duplicated here.

---

# Historical Terminology Warning

Some implementation and historical material surrounding Vertical Slice 0 may still contain superseded terms such as:

```text id="k1l7jw"
Vapor-Root

Vapor-Server-Root

ecosystem command

source command

Workspace semantic identity

Local Realization

Open

Focus

multiple Superworkspaces
```

Those terms should be interpreted as historical implementation context where they conflict with current normative models.

Do not update this Bootstrap by reintroducing those old semantics merely because the historical implementation used them.

---

# Current Source Home

The primary rewritten generic Vapor implementation lives in the Source Repo historically known as:

```text id="34hfw5"
GHF-Studios/Vapor
```

within the first-party Client source family.

Under the target canonical topology, that Source Repo belongs beneath:

```text id="ay84tu"
GHF-Studios/Vapor-Client
```

following migration from the historical:

```text id="j2bk1m"
GHF-Studios/Vapor-Root
```

The repository migration documents own the actual rename/identity/local-layout sequence.

---

# Server-Side Separation

Client and Platform source remain separate responsibilities.

Conceptually:

```text id="tmxtf6"
Vapor Client
    user-side product/tooling

Vapor Platform Server
    Registry / Identity / Diagnostics / services / infrastructure
```

The fact that both participate in Vapor does not justify one generic repository or lifecycle.

---

# Self-Hosting Direction

A major rewrite principle which emerged after the first slice is:

> **Vapor should progressively absorb the manual machinery required to develop Vapor itself.**

This includes areas such as:

```text id="q71o8c"
managed toolchain

managed Cargo

source acquisition

source topology

build/test

IDE integration

deployment

diagnostics

provider integration
```

Root/first-party development should use the Vapor-managed locked/vendored toolchain where required rather than relying accidentally on ambient tooling.

---

# Authored vs Derived State

Later implementation pressure also established an important boundary:

```text id="u4bp5w"
AUTHORED
    source
    Git state
    manifests
    tests
    scripts
    explicit configuration

DERIVED
    generated realization
    indexes
    caches
    IDE integration
    generated glue
```

Derived Vapor state should be safely regeneratable.

Authored source must not be treated as disposable repair state.

---

# Source Model Direction

The current source hierarchy is:

```text id="x5gug6"
Superworkspace
    local root, no identity

Authority
    ↓
Source Repo Container
    ↓
Source Repo / Vapor Workspace
    ↓
Project
```

The Bootstrap does not own this architecture.

It records it here only to prevent historical bootstrap terminology from being mistaken for current truth.

---

# CLI Direction

The historical rewrite temporarily used commands such as:

```text id="ic8lqa"
vapor ecosystem ...
```

Those commands were valid implementation milestones.

They are not the target public model.

Current CLI semantics are owned by **Vapor CLI Model**.

Likewise, this Bootstrap's historical:

```text id="a7evmm"
vapor run <packagepack>
```

should be read as:

> Vapor must provide a semantic way to run the Packagepack-derived composition.

not as a permanent syntax mandate.

---

# Documentation Structure

The Premium Docs currently contain many focused model documents which grew organically during architecture work.

Their physical folder structure is not part of Vertical Slice 0.

A later repository-structure pass may:

```text id="d7agkd"
group documents into topic folders

create one central/index document per folder

repair cross-links

move diagrams alongside their owning topics where appropriate
```

That work should preserve conceptual ownership rather than create duplicate normative sources.

It belongs to deliberate repository/documentation maintenance, ideally performed together with coding-agent-assisted structural changes once the content model has stabilized.

---

# Bootstrap Status

Vertical Slice 0 is complete.

This document is therefore **historical architectural context**.

It should change rarely.

The living implementation frontier belongs to:

```text id="ncjmsq"
Vapor Rewrite Progress And Roadmap.md
```

---

# Bootstrap Invariants

* Current normative Premium Docs outrank historical implementation structure.
* Legacy source is archaeology and proven mechanism, not automatic architecture.
* Reuse Rust/Cargo/Git/provider/Steam mechanisms where they already solve the problem well.
* Vapor owns semantics which the underlying systems do not provide.
* Vapor semantic dependency graphs and Cargo physical package graphs are distinct.
* Vapor determines semantic composition; Cargo performs Rust build realization.
* Vapor dependency edges do not invent runtime extension APIs.
* Packagepack is the complete composition root.
* Vertical Slice 0 had to make Vapor semantics operationally affect the resulting build.
* Temporary hardcoding was valid only where it did not bypass the proof.
* Implementation pressure should refine the model.
* Historical bootstrap syntax and identity examples are not current normative grammar.
* The current canonical source hierarchy is owned by the focused identity/development models.
* The generic public `ecosystem` object is not part of the current target CLI model.
* Root Authority is authority rather than an installed Role.
* Authored source is not disposable derived state.
* Vapor should increasingly self-host its own development workflows through the managed toolchain and source model.
* Vertical Slice 0 is complete and should remain a known-good architecture/regression specimen.
* Current implementation planning belongs to the Progress And Roadmap document.

---

# Historical Non-Goals

The original Bootstrap intentionally did not block on:

```text id="4n4sh7"
complete Registry infrastructure

provider publication

Workshop publication

production build provenance/signing

cross-compilation

remote builds

final SDK

final Launcher

final Installer

full source-acquisition system

complete dependency solving

full multiplayer/App Server design

USF integration
```

Some of these have since been partially or substantially explored.

Their current state must be read from the living roadmap rather than inferred from this historical list.
