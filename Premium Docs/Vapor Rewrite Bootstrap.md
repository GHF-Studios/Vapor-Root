# Vapor Rewrite Bootstrap

> [!info]
> This document defines the immediate implementation direction for the Vapor rewrite.
>
> It is intentionally small.
>
> Its purpose is not to fully specify Vapor before implementation, but to establish the boundary between the current conceptual model and the first architecture-proving implementation slice.

---

# Rewrite Position

The current Vapor implementation should generally be treated as legacy/reference material.

Existing code may still contain:

* Useful implementation ideas.
* Valid integration details.
* Steam/GitHub identifiers.
* Repository-layout knowledge.
* Toolchain configuration.
* Naming worth preserving.
* Small pieces that remain architecturally sound.

However:

> **The new implementation is not required to preserve compatibility with legacy Vapor internals.**

Existing implementation structure must not override the current Premium Docs model.

Where legacy code conflicts with the current model, the current model wins.

---

# Normative Design Source

The current `Premium Docs` define the intended Vapor domain and experience model.

Implementation should primarily proceed downward from:

* `Glossary - Ecosystem Model.md`
* `Glossary - USF Model.md`
* `Vapor Ecosystem Experience Model.md`
* `Vapor Ecosystem Operational Model.md`
* `Vapor Development Experience Model.md`
* `Vapor Publishing And Distribution Model.md`

These documents are not expected to answer every implementation question in advance.

Their purpose is to provide enough semantic structure that implementation can proceed coherently.

---

# Implementation Principle

Vapor should avoid recreating mechanisms already provided well by its underlying ecosystem.

Before introducing a new Vapor mechanism, ask:

1. Does Rust already solve this?
2. Does Cargo already solve this?
3. Does Git already solve this?
4. Does GitHub already solve this?
5. Does Steam already solve this?

If yes:

> **Prefer wrapping, orchestrating, constraining, or re-exposing the existing mechanism.**

If the existing mechanism almost fits:

> Adapt it minimally.

Only introduce genuinely new Vapor machinery where Vapor requires semantics not provided by the underlying systems.

---

# Current Ecosystem Position

Vapor is primarily a semantic and orchestration layer over:

* Rust.
* Cargo.
* Git.
* GitHub.
* Steam.
* Steam Workshop.
* Vapor-specific runtime/content semantics.

Vapor should own concepts such as:

* Vapor Content identity.
* Vapor Content kinds.
* Composition.
* Packagepack resolution.
* Vapor dependency semantics.
* Vapor App identity.
* Vapor Role- and Installation Profile-aware workflows.
* Publication coordination.
* Runtime integration semantics.

It should not unnecessarily replace the lower-level systems implementing compilation, source control, hosting, distribution, or language safety.

---

# Current Design Decisions to Preserve

The implementation should preserve the following current conclusions.

## Identity

* Every Vapor Content artifact has one immutable, human-readable, globally unique Vapor ID.
* The Vapor ID does not contain its version.
* Changing the Vapor ID creates a new identity.
* Display names may change independently.
* Provider IDs such as Steam Workshop Item IDs are linked external identities, not the Vapor semantic identity.

## Versioning

* Published Vapor Content uses SemVer.
* Git commits represent source evolution between published versions.
* Published versions are immutable.
* A version number may never be reused within the semantic history of one Vapor ID.
* Local development may use non-published/dirty source state without pretending it is a published release.

## Dependencies

* Vapor Content declares dependencies using SemVer constraints.

* Dependency semantics should behave broadly like Cargo.

* Compatible version requirements should preferably unify.

* Multiple SemVer-incompatible versions of the same Vapor ID may coexist when required.

* A resolved composition node is effectively identified by:

  `(Vapor ID, Version)`

* Dependency bindings may use local aliases to disambiguate multiple versions.

* Versions do not become part of the permanent Vapor namespace.

## Packagepack Resolution

* A Packagepack is the complete composition root.
* It resolves to exactly one effective Engine and one effective Game plus applicable Mods.
* Final dependency resolution happens in Packagepack context.
* The resolved Packagepack records the exact dependency graph used for that realized release/build.
* This resolved state functions conceptually like a lockfile.

## Definition vs Runtime Instance

A resolved Vapor dependency defines behavior/code/content.

It does not imply any particular number of runtime instances.

Multiple dependents resolving the same version share the same resolved definition node.

Runtime object/state multiplicity remains controlled by the runtime/domain model.

Different resolved versions of the same Vapor ID are separate definition environments by default.

## Publication

* Git/GitHub handle source history and source publication.
* Steam Workshop handles Player-facing distribution of complete Packagepack compositions.
* The Vapor Registry records semantic history and linkage per Vapor ID.
* Packagepack target artifacts belong to the GitHub release of that exact Packagepack version.
* Old Packagepack versions are not bundled into later GitHub releases.
* Steam Workshop publication remains focused on complete final compositions.
* Historical Workshop-version retrieval semantics remain implementation-dependent until Steamworks capabilities are proven.

## Withdrawal

* Yank is author-controlled withdrawal similar to Cargo.
* Ban is administrative/moderation-controlled hard denial through official Vapor tooling.
* Historical identity remains known even when content is yanked or banned.
* Banned content may be prevented from resolution, acquisition, installation, and launch.

---

# Development Method

The rewrite should follow this cycle:

> **Model enough → implement a vertical slice → observe real pressure → refine the model → continue implementation.**

Do not attempt to fully design every future Vapor subsystem before implementation begins.

Conceptual modeling should resume when implementation exposes a real unresolved semantic question.

---

# Vertical Slice 0

## Purpose

Vertical Slice 0 exists to prove that the current Vapor architecture can produce and run a complete composition from Vapor-level source declarations.

It is not intended to prove:

* Steam Workshop publication.
* Registry infrastructure.
* Launcher UX.
* Installer UX.
* Full SDK UX.
* Remote builds.
* Signing.
* Cross-platform publication.
* Historical version retrieval.
* Full USF integration.
* Production-grade dependency solving.

It should prove the core architectural path first.

---

# Scenario

Vertical Slice 0 contains:

## Terminal Engine

A minimal Engine which:

* Owns the main executable.
* Provides a tiny runtime/API surface.
* Starts the composition.
* Invokes or exposes registered Game behavior.
* Prints visible lifecycle information.

## Hello World Game

A minimal Game which:

* Targets the Terminal Engine.
* Is statically incorporated into the built Vapor App.
* Registers or exposes behavior consumed by the Engine.
* Produces visible output.

## Tiny Game Mod

A minimal Game Mod which:

* Targets the Hello World Game.
* Extends or changes the Game's behavior.
* Demonstrates that Mods participate in the composition rather than existing as unrelated Cargo crates.

## Hello World Packagepack

A Packagepack which:

* Selects the Terminal Engine.
* Selects the Hello World Game.
* Includes the Tiny Game Mod.
* Forms one complete Vapor App Composition.

---

# Expected Runtime Result

The exact text is unimportant, but execution should visibly prove that every layer participated.

For example:

```text
[Engine] starting
[Game] registered
[Mod] extended game
[Game] Hello from Vapor!
[Mod] ...on steroids.
[Engine] shutting down
```

---

# Required End-to-End Path

The slice should prove this flow:

```text
Vapor Content source
        ↓
Vapor manifests
        ↓
Vapor Content identities and dependency declarations
        ↓
Packagepack resolution
        ↓
exact resolved composition
        ↓
Rust/Cargo build orchestration
        ↓
Engine executable + statically incorporated Game/Mod
        ↓
Vapor App artifact
        ↓
launch
        ↓
visible Engine/Game/Mod interaction
```

---

# Primary Acceptance Goal

The implementation should eventually support an operation conceptually equivalent to:

```text
vapor run <packagepack>
```

The exact CLI syntax is not yet normative.

The important property is:

> A user identifies a Packagepack, and Vapor itself determines enough of the composition/build/run process to produce and launch the resulting Vapor App.

The user should not need to manually reconstruct the underlying Cargo composition.

---

# What Vapor Must Actually Prove

Vertical Slice 0 must prove that Vapor can:

* Identify Vapor Content.
* Parse the minimal required Vapor manifests.
* Understand Engine, Game, Game Mod, and Packagepack roles.
* Follow their dependency/target relationships.
* Resolve one complete Packagepack.
* Detect structural composition errors.
* Determine the effective Engine.
* Determine the effective Game.
* Include the Mod.
* Translate or expose the resolved composition to Cargo/Rust.
* Build the Engine-owned executable.
* Produce a runnable artifact.
* Launch it.
* Demonstrate actual interaction between the resolved content.

---

# What May Be Hardcoded Initially

Temporary hardcoding is acceptable where it does not bypass the architectural thing being tested.

Examples that may initially be hardcoded:

* Local content discovery paths.
* One workspace root.
* One host platform.
* Development-only versions.
* No Registry lookup.
* No GitHub lookup.
* No Workshop lookup.
* No remote source acquisition.
* Minimal target selection.
* Minimal diagnostics formatting.
* Only the subset of Vapor Content kinds used by the slice.

Temporary hardcoding must remain visible as temporary architecture scaffolding rather than becoming accidental permanent semantics.

---

# What Must Not Be Faked

The slice should not succeed merely because Cargo already manually encodes the entire composition.

In particular, avoid a situation where:

* Cargo directly wires Engine → Game → Mod,
* the Packagepack manifest merely describes the same relationship,
* and Vapor does not actually participate in resolving or constructing anything.

The purpose of the slice is to prove that Vapor's semantic composition model has operational consequences.

Cargo should perform compilation.

Vapor should determine what Cargo is being asked to compile.

---

# Manifest Strategy

Do not fully design the final Vapor manifest schema before implementing the slice.

Instead:

1. Start with only the fields required by the slice.
2. Implement them.
3. Observe which distinctions are actually required.
4. Expand the schema only when implementation pressure justifies it.

Existing `*.vapor.toml` files may be mined for ideas but are not automatically authoritative.

---

# Initial Manifest Requirements

The first implementation will probably need only concepts equivalent to:

## Common identity

```toml
id = "ghf-studios/example/foo"
version = "0.1.0"
```

## Engine

```toml
kind = "engine"
```

plus identification of the Engine-owned binary/build target.

## Game

```toml
kind = "game"
```

plus an Engine dependency/target.

## Game Mod

```toml
kind = "game-mod"
```

plus a Game dependency/target.

## Packagepack

```toml
kind = "packagepack"
```

plus dependencies selecting the intended Engine, Game, and Mod.

The exact syntax is deliberately undecided.

---

# First Resolver Scope

The first resolver does not need to implement every final Cargo-like dependency feature.

It only needs enough semantics to prove the architecture.

Initial support may include:

* Local source only.
* Exact versions first if useful.
* Simple SemVer constraints soon after.
* Acyclic dependency graph.
* One complete Engine.
* One complete Game.
* Game Mod targeting.
* Clear failure on invalid composition.

More advanced resolution semantics should be added incrementally.

---

# Legacy Code Policy

When implementing Vertical Slice 0, existing Vapor code should be evaluated case by case.

For each old component:

* Reuse unchanged if it still cleanly matches the new architecture.
* Refactor if the underlying idea remains valid.
* Extract small useful mechanisms if appropriate.
* Rewrite if the architecture is wrong.
* Delete/ignore if obsolete.

Avoid both extremes:

> Do not preserve legacy code merely because it exists.

and:

> Do not rewrite useful, correct code merely for the emotional satisfaction of a clean slate.

---

# Immediate Implementation Sequence

## Phase 0 — Bootstrap

* Decide where the rewritten core should physically live.
* Establish the smallest clean Rust crate/module boundary.
* Ensure the rewrite can evolve independently from legacy assumptions.

## Phase 1 — Content Model

Implement only the minimal runtime-independent model required for the slice:

* Vapor ID.
* SemVer version.
* Content kind.
* Dependency reference.
* Local dependency alias.
* Content manifest.
* Resolved content node.

## Phase 2 — Local Discovery

Discover the slice's local Engine, Game, Game Mod, and Packagepack.

No Registry.

No GitHub API.

No Workshop.

## Phase 3 — Packagepack Resolution

Resolve:

```text
Packagepack
├── Engine
├── Game
└── Game Mod
```

Validate:

* exactly one effective Engine;
* exactly one effective Game;
* valid Mod target;
* no broken references.

Produce an explicit resolved composition.

## Phase 4 — Cargo Bridge

Translate the resolved Vapor composition into whatever Cargo needs to build it.

Prefer Cargo-native mechanisms wherever possible.

Avoid inventing a new compiler/build graph.

## Phase 5 — Build

Produce the Engine-owned executable containing the resolved Game and Mod behavior.

## Phase 6 — Run

Launch the built Vapor App.

Verify visible Engine/Game/Mod interaction.

## Phase 7 — Review

After the first successful vertical slice:

* identify architectural pain;
* identify accidental legacy assumptions;
* update Premium Docs where reality exposed missing semantics;
* decide the next slice.

---

# Definition of Done

Vertical Slice 0 is complete when:

> Starting from a Packagepack's Vapor-level declaration, the new Vapor implementation can resolve a valid Engine + Game + Game Mod composition, orchestrate its Rust/Cargo build, produce a Vapor App executable, launch it, and visibly demonstrate behavior from all three content layers.

At that point, the rewrite has crossed the most important early boundary:

> **Vapor is no longer merely describing compositions. It can operationally realize one.**

---

# Explicitly Deferred

Do not expand Vertical Slice 0 to solve these unless absolutely required:

* Steam Workshop.
* SteamCMD publication.
* GitHub Releases.
* Registry server implementation.
* Account linkage.
* Namespace ownership enforcement.
* Yank/Ban infrastructure.
* Historical package retrieval.
* Full dependency backtracking.
* Remote dependencies.
* Multiple incompatible versions of the same Vapor ID.
* Production lockfile format.
* Build provenance.
* Binary reproducibility.
* Signing.
* CI builders.
* Launcher UI.
* Installer UI.
* SDK UI.
* Production diagnostics UX.
* Full Content Library.
* Automatic source acquisition.
* Cross-compilation.
* Multi-target builds.
* Update policy.
* USF integration.

These remain valid future concerns.

They are not allowed to prevent Vapor from first proving that one local composition can exist end to end.

---

# Next Action

The next implementation-focused task is:

> **Inspect the current repository structure only far enough to choose the clean physical home and crate boundaries for Vertical Slice 0.**

After that:

> **Begin writing the new core.**

No additional broad conceptual model is required before implementation starts.

---

# Phase 0 — Bootstrap Decisions

## Terminology Boundary

Vapor's product/user-level access model and USF's runtime Capability model are separate concepts.

The following terminology should be preferred:

- **Vapor Role** — a user's ecosystem role, such as Player, Composer, Content Developer, Ecosystem Developer, or Root Authority.
- **Vapor Installation Profile** — the locally installed Vapor tooling/features appropriate to a role or workflow, managed by the Vapor Installer.
- **Capability** / **USF Capability** — the runtime Capability concept used by USF, Rhai/runtime authoring, and related simulation/runtime graphs.

Bare `Capability` should not be used for Launcher/Installer access levels.

---

## Rewrite Home

The rewritten generic Vapor implementation lives in:

```text
GHF-Studios/Vapor