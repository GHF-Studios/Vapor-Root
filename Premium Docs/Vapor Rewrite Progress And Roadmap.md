# Vapor Rewrite Progress And Roadmap

> [!info]
> This document records the current implementation state of the Vapor rewrite and the next architecture-proving phases.
>
> It complements `Vapor Rewrite Bootstrap.md`.
>
> The Bootstrap document explains why the rewrite began and how Vertical Slice 0 was approached. This document tracks where the rewrite actually stands after implementation pressure refined the architecture.

---

# Current Position

The Vapor rewrite has crossed its first major architectural boundary.

Vapor can already:

* model Vapor Content identities and kinds;
* parse authored `Vapor.toml` manifests;
* discover local Content recursively;
* resolve SemVer dependency declarations;
* detect missing dependencies and cycles;
* derive a complete Packagepack composition;
* generate a Cargo realization;
* build that realization;
* run the resulting statically composed Vapor App;
* manage and use a pinned Rust toolchain;
* build and test Vapor using that Vapor-managed toolchain;
* use a Vapor-built Vapor binary to operate on Vapor itself;
* model local Vapor Roles independently from external authorization;
* expose the current core through a declarative Clap CLI;
* resolve arbitrary local Content graphs independently of Packagepack realization.
* model Library as first-class Vapor Content;
* inspect Rust/Cargo package realization for Libraries;
* verify Vapor semantic Library dependencies against Cargo realization;
* repair safely missing Cargo dependency bindings through managed Cargo;
* execute managed Cargo through the Installation-owned pinned toolchain;
* resolve Cargo execution to an appropriate Vapor Project rather than requiring shell CWD;
* locally deploy rewritten Vapor back into the canonical Steam App Instance;
* distinguish local and Steam ecosystem deployment targets explicitly;
* model structural identity, local realization, Development Session, Open/Focus/Selection, and frontend target resolution;
* define the Launcher-integrated Vapor SDK as the primary graphical development surface.

The current implementation is therefore no longer merely a prototype parser or conceptual model.

> **Vapor already has an operational semantic dependency graph which can affect a real Rust/Cargo build and produce a runnable application.**

The current architectural pressure is no longer whether Vapor composition, managed self-hosting, or basic Cargo supervision can work.

Those mechanisms have now produced enough real UX/DX pressure to expose another missing architectural layer:

> **How should canonical identity, local realizations, development sessions, selection, CLI automation, and the graphical Vapor SDK form one coherent development experience?**

That design checkpoint is intentionally being resolved before deeper Bevy/static-extension realization continues.

The broader runtime pressure remains:

> **How should Vapor statically realize arbitrarily extensible Engine/Game/Mod ecosystems without stealing their runtime or API semantics?**

---

# Proven Milestones

## 1. Core Content Model

Implemented:

* immutable human-readable Vapor IDs;
* SemVer Content versions;
* `ContentVersionId`;
* Content kinds;
* Vapor dependency declarations;
* local dependency bindings;
* manifest parsing;
* local Content discovery;
* exact resolved Content nodes.

The fundamental resolved identity remains:

```text
(Vapor ID, Version)
```

Dependency binding names are local aliases.

They are not semantic slots.

---

## 2. Vertical Slice 0

The original architecture-proving composition works:

```text
Terminal Engine
+ Hello World Game
+ Tiny Game Mod
+ Hello World Packagepack
```

Vapor can:

```text
discover
→ resolve
→ validate
→ generate Cargo realization
→ build
→ run
```

with visible Engine/Game/Mod participation.

The original slice remains intentionally small and should remain available as a known-good regression specimen while richer realization semantics are developed.

---

## 3. Cargo Realization

Vapor currently generates a small Cargo realization for a resolved Packagepack.

This proved the critical architectural distinction:

```text
Vapor Content graph
≠
Cargo package graph
```

and:

> **Vapor determines semantic composition; Cargo performs Rust package resolution/build execution for the physical realization Vapor asks it to build.**

The current realization is still deliberately narrow.

It assumes one effective Engine, one effective Game, and the original minimal Mod arrangement.

Generalizing this realization is still future work.

---

## 4. Managed Rust Toolchain

Vapor has a pinned Rust toolchain model.

Current root metadata pins a specific Rust version and supports a Vapor-managed environment containing the relevant Cargo/Rust/Rust Analyzer tooling.

The rewrite has proven:

```text
ambient bootstrap Cargo
→ build Vapor
→ Vapor-managed pinned Rust
→ build/test Vapor
→ managed-built Vapor operates Vapor
```

This is the first major self-hosting step.

The long-term installed location is the Vapor Steam App Instance rather than a repository-local toolchain.

Repository-local state remains useful bootstrap/development scaffolding until the installed environment is fully realized.

---

## 5. Workspace / Ecosystem Self-Hosting

Vapor understands its own source workspace well enough to perform managed:

```text
vapor ecosystem status
vapor ecosystem build
vapor ecosystem test
```

The broader direction is:

> **Vapor should progressively absorb the manual machinery required to reproduce and develop itself.**

This principle applies beyond compilation to source acquisition, installation, deployment, provider integration, authorization, and publication.

---

## 6. Role and Authorization Model

The local Role model is cumulative:

```text
Player
→ Composer
→ Content Developer
→ Ecosystem Developer
```

All of these are locally obtainable.

Role answers:

> What kinds of work is this installation equipped to perform?

Authorization separately answers:

> May this authenticated identity perform this particular operation on this particular external target?

Therefore:

```text
Role ≠ Authority
```

`Root Authority` is an authority relationship, not a locally promotable Vapor Role.

An Ecosystem Developer may locally acquire/fork/edit/build/test Vapor or server/root source without official GHF authorization.

Protected actions such as official deployment or publication remain authorization-dependent.

---

## 7. CLI Model

The Vapor CLI now uses declarative Clap parsing.

The universal `vapor` binary and dedicated application binaries are projections over the same logical Vapor Core.

Principle:

> **Binaries do not own semantics. Vapor Core owns semantics. Binaries expose subsets of those semantics.**

The CLI uses explicit domain nouns such as:

```text
installation
role
authority
toolchain
source
ecosystem

packagepack
enginepack
gamepack
modpack

engine
game
engine-mod
game-mod
extension-mod
```

rather than floating generic commands.

Several modeled commands intentionally exist before their underlying operation is implemented.

---

## 8. Generic Content Resolution

Resolution has now been separated from Packagepack composition.

Conceptually:

```text
authored Vapor Content
        ↓
generic dependency resolution
        ↓
ResolvedContentGraph
        ↓
kind-specific semantic validation
        ↓
optional Packagepack composition derivation
        ↓
physical realization
```

This distinction is important.

The dependency resolver recursively follows authored dependencies regardless of Content kind.

Packagepack-specific concepts such as an effective Engine and Game are derived above the generic graph rather than defining the graph itself.

Pack resolution is now conceptually available for:

```text
Packagepack
Enginepack
Gamepack
Modpack
```

while richer real-world example graphs still need to be authored.

---

# Architectural Conclusions Reached During the Current Slice

## Vapor Does Not Define Modding Semantics

A Vapor dependency edge means:

> One Vapor artifact requires another Vapor artifact.

It does **not** inherently mean:

> Vapor knows how the depender modifies or extends the dependency.

The thing being extended must explicitly define its own extension semantics.

This may involve:

* Rust APIs;
* traits;
* builders;
* registries;
* callbacks;
* ECS Components;
* ECS Resources;
* ECS Events;
* ECS Systems;
* ECS SystemSets;
* schedules;
* plugins;
* generated registration structures;
* data schemas;
* or any combination appropriate to that ecosystem.

Therefore:

> **Vapor composes extension participants. The participants define what extension means.**

---

# ECS Direction

The architecture-proving example should now use a deliberately slim Bevy ECS stack.

The intended baseline is approximately:

```text
bevy_app
bevy_ecs
```

without pulling in rendering, windows, assets, audio, or the full Bevy application stack unless implementation pressure requires them.

For the toy Terminal Engine:

```text
Engine
owns Bevy App / World / broad lifecycle

Game
adds Engine-compatible ECS behavior and its own extension surfaces

Engine Mod
adds Engine-compatible ECS behavior and may expose new capabilities

Game Mod
may consume Engine, Game, Engine-Mod, Library, or other explicit APIs

Extension Mod
may extend another Mod's explicitly published surface
```

However:

> **Bevy Plugin is not currently a universal Vapor Content ABI.**

It may be the natural integration mechanism for this particular Engine.

That Engine-specific decision must not automatically become a universal Vapor semantic rule.

---

# Extension Capability Is Not Necessarily Narrowing

An earlier working intuition was that extension surfaces become progressively narrower:

```text
Engine
→ Game
→ Game Mod
→ Extension Mod
```

This is not a valid general invariant.

A downstream artifact may expose a much larger or more general extension platform than the artifact it depends on.

The correct model is:

> **Any Vapor artifact may consume, implement, specialize, combine, or extend capabilities exposed by its dependencies, and may expose arbitrary new capabilities to its own dependents.**

For example:

```text
small Engine Mod
    ↓
large Game Mod
    ↓
large public addon API
    ↓
an ecosystem of Extension Mods
```

is completely legitimate.

The dependency structure is a graph of explicitly authored capabilities, not a hierarchy of monotonically decreasing extensibility.

---

# Library Content

A missing Content concept has now become apparent:

```text
Library
```

A Vapor Library is conceptually:

> A Vapor-managed, versioned reusable Rust library which participates in Vapor dependency/source/publication semantics without inherently representing runnable Engine/Game/Mod behavior.

A Library may provide:

* ordinary Rust types;
* traits;
* algorithms;
* builders;
* ECS vocabulary;
* Components;
* Resources;
* Events;
* SystemSets;
* registration interfaces;
* shared schemas;
* extension contracts.

Libraries make it possible to publish reusable APIs through Vapor without requiring crates.io publication.

Conceptually:

```text
Cargo crate
+ Vapor identity
+ Vapor SemVer
+ Vapor dependency resolution
+ Vapor source/publication lifecycle
- crates.io requirement
- inherent runtime/composition meaning
```

This should become a first-class Content kind before the richer Bevy extension example is developed.

---

# Vapor and Cargo

The current direction is **not** for Vapor to replace Cargo manifests entirely.

Instead:

> **Vapor should supervise and reconcile the portions of the Cargo graph that realize Vapor semantic dependencies while preserving ordinary editable Cargo projects.**

Conceptually:

```text
Vapor.toml
    semantic dependency intent
        ↓
Vapor resolver
    exact Vapor graph
        ↓
Cargo reconciliation
    desired physical Rust dependencies
        ↓
editable Cargo.toml / Cargo.lock
        ↓
normal Cargo
```

A Content project should remain an ordinary understandable Rust/Cargo project.

Developers should still be free to maintain ordinary Cargo configuration such as:

```text
package metadata
features
lints
build dependencies
target-specific dependencies
binary/library declarations
non-Vapor crates.io dependencies
profiles
other Cargo-native configuration
```

Vapor supervises only the subset whose meaning derives from Vapor semantic dependencies.

---

# Cargo Reconciliation

A Vapor dependency may eventually resemble:

```toml
[dependencies.weighted-roll]
id = "ghf-studios/example/weighted-roll-api"
version = "^0.1"
```

After resolution, Vapor knows the exact semantic dependency and its physical Rust source.

Vapor then ensures that Cargo has an equivalent usable dependency.

This should be reconciliation rather than blind manifest generation.

Conceptually Vapor compares:

```text
desired Vapor-resolved state
previous Vapor realization state
current editable Cargo.toml state
```

and can distinguish:

```text
valid
stale
conflicting
locally overridden
repairable
```

Likely operations include:

```text
verify
repair
build
test
```

where `verify` diagnoses disagreement and `repair` may reconcile managed entries.

Explicit local/unlocked overrides remain useful for development but must not silently become publishable state.

---

# Build Script Boundary

Cargo `build.rs` should not be treated as the primary Vapor dependency manager.

Dependency resolution occurs before package build scripts can meaningfully alter the normal dependency graph.

Additionally, Vapor must not steal the package's build-script mechanism from projects which legitimately require their own build script.

A future build-time hook may still be useful for stale-state detection or integrity checks, but dependency realization belongs before Cargo execution.

---

# Steam App Instance Direction

The long-term installed Vapor environment is the Steam App Instance.

Conceptually:

```text
Steam App Instance
├── bin/
├── rustup/
├── rustup-home/
├── cargo-home/
├── steam/
├── cache/
├── apps/
└── state/
```

Authored source does **not** belong there.

Development source belongs in a separate Superworkspace because Steam uninstall/repair/update operations must never destroy unpushed authored work.

Therefore:

```text
Steam App Instance
= installed/disposable Vapor environment

Superworkspace
= durable authored Git source
```

Vapor binaries should primarily discover the installed environment relative to their own executable location.

External Steam APIs may additionally verify or locate installations where necessary.

---

# Local Deployment Before Steam Deployment

Real deployment semantics should be tested locally before involving Steam publication.

Distinguish:

```text
build
= produce artifacts

local deploy
= materialize a realistic Vapor installation/staging layout

official deploy
= deliver that validated layout to an externally protected target
```

A future local workflow should be able to do something conceptually equivalent to:

```text
Vapor source
→ ecosystem build
→ ecosystem local deploy
→ staged Vapor installation
→ run staged Vapor
→ install/promote tooling
→ build/test/develop Vapor again
```

Steam deployment should then become primarily a provider-specific delivery step over an already-valid installation artifact.

---

# Current Boundary

The implementation is currently between two generations of architecture:

## Already generalized

```text
Vapor Content identity/versioning
Vapor manifests
local Content discovery
semantic dependency graph
generic graph resolution
Library Content
initial Cargo inspection/reconciliation
role / authority separation
declarative CLI model
Installation-owned managed toolchain
managed Cargo handoff
workspace/ecosystem self-hosting
local ecosystem deployment
Steam deployment backend
source / Superworkspace discovery
initial Project-context resolution
```

## Still Vertical-Slice-specific / incomplete

```text
final Cargo-reconciliation edge cases
local-override persistence/publishability
final App realization
Engine/Game/Mod static integration
example runtime APIs
multi-Mod realization
provider-backed general source acquisition
complete context/session implementation
canonical structural-ID implementation
multi-realization selection
full Launcher/SDK GUI
production Registry/publication/distribution
```

This remains the correct broad pressure boundary.

A bounded Context / Identity / Session / SDK design checkpoint has been inserted before the Bevy extension slice because the rewrite's self-hosting workflows exposed real ambiguity around Project identity, source realization, CWD, persistent context, CLI targeting, and future GUI behavior.

---

# Revised Forward Roadmap

## Phase A — Library Content

Implemented at the initial architecture-proving level.

Add the `Library` Content kind and its basic semantics.

Minimum proof:

```text
Library
→ discovered
→ resolved
→ listed/inspected
→ usable as a dependency
```

Do not prematurely invent a separate library registry or package manager.

Reuse Vapor identity/version/dependency machinery.

---

## Phase B — Vapor-Managed Cargo Reconciliation

Prove:

```text
Vapor dependency
→ exact resolved Content
→ physical Cargo dependency
→ ordinary Rust import succeeds
```

Use one tiny Library and one consuming Content artifact.

Requirements:

* `Vapor.toml` expresses the Vapor dependency.
* `Cargo.toml` remains editable.
* Vapor can establish the required Cargo dependency.
* unrelated Cargo configuration survives unchanged.
* Vapor can detect manual conflicting edits.
* verification does not blindly overwrite.
* an explicit repair/reconciliation path can restore expected state.
* local overrides remain distinguishable from publishable state.

This phase should answer how much Cargo metadata Vapor must persist to supervise the relationship reliably.

---

## Architecture Checkpoint — Context, Identity, Session, and SDK

This checkpoint was introduced by real self-hosting/Cargo workflow pressure.

The implementation had reached a state where commands could:

```text
global installed Vapor
→ Installation-owned managed Cargo
→ source-built Vapor
→ remembered development source
→ semantic Content resolution

---

## Phase C — Minimal Bevy ECS Engine

Replace the temporary closure-style example runtime with an intentionally tiny Bevy ECS Engine.

Use only the Bevy crates actually required.

The Terminal Engine should own:

* the App/World;
* lifecycle semantics;
* broad schedules/SystemSets;
* explicit Engine extension points.

The exact extension API belongs to the Engine, not Vapor.

---

## Phase D — Coherent Wheel Game

Replace the generic Hello World behavior with a tiny terminal wheel/spin game.

The Game should introduce deliberately moddable ECS/Rust vocabulary such as:

```text
Wheel
Spin
LootTable
GameMode
WheelRegistry
LootTableRegistry
GameModeRegistry
Game-specific SystemSets
```

The Game explicitly defines what downstream Game Mods may attach, add, modify, replace, or remove.

---

## Phase E — Engine Mod Capability

Create one meaningful Engine Mod.

Candidate:

```text
Weighted Roll Engine Mod
```

It should add a real Engine-level capability which the base Engine does not provide.

Its public surface may combine:

```text
ordinary Rust algorithms/types
+
Bevy ECS Components/Resources/Events/SystemSets
```

If appropriate, its reusable API should be exposed through a Vapor Library.

---

## Phase F — Cross-Layer Game Mod

Create one Game Mod which genuinely requires several layers.

For example:

```text
Casino Game Mod
├── Wheel Game
├── Terminal Engine
└── Weighted Roll Engine Mod / API
```

It should use:

* Game-owned extension points;
* Engine-owned ECS behavior;
* Engine-Mod-provided capabilities.

It should register real toy functionality such as:

```text
weighted casino wheel
loot table
jackpot
casino game mode
```

This becomes the first truthful demonstration that Vapor can organize a semantically meaningful cross-layer mod ecosystem.

---

## Phase G — Observe Static Integration

Before teaching Vapor a universal realization mechanism, manually prove the smallest ordinary Rust composition of:

```text
Engine
+ Game
+ Engine Mod
+ Game Mod
+ Libraries
```

Observe what the Engine actually requires to integrate these artifacts.

Do not prematurely decree that all behavioral Content must implement one universal Vapor trait or Bevy Plugin interface.

The implementation pressure from this composition should determine the smallest correct realization contract.

---

## Phase H — Generic Static Realization

Teach Vapor to construct the real Cargo/App composition discovered in Phase G.

Remove the current assumptions of:

```text
exactly one hardcoded Game Mod
fixed generated function calls
fixed Engine/Game/Mod wiring
```

The resolved Vapor graph should determine the concrete physical Rust graph and final application realization.

---

## Phase I — Rich Pack Examples

Only after the underlying behavioral artifacts are meaningful, introduce:

```text
Enginepack
Gamepack
Modpack
multiple Packagepacks
```

These should represent useful curated compositions, not artificial graph fixtures.

The example ecosystem should remain intentionally toy-sized in implementation while becoming absurdly rich in composition.

A target demonstration is:

> **A tiny terminal wheel game becomes a bootleg substantial game through layers of Mods and Extension Mods, while Vapor successfully manages the organizational complexity.**

---

## Phase J — Extension Ecosystem Explosion

Deliberately demonstrate that an addon can become a platform.

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

Each layer may introduce new ECS and ordinary Rust extension capabilities.

The point is not graphical sophistication.

The point is to demonstrate extreme compositional complexity using extremely cheap toy implementations.

---

## Phase K — Content Creation and Templates

Implement:

```text
vapor library create
vapor engine create
vapor game create
vapor engine-mod create
vapor game-mod create
vapor extension-mod create
vapor enginepack create
vapor gamepack create
vapor modpack create
vapor packagepack create
```

Initially use small built-in canonical templates.

Do not build a generalized remote template ecosystem until real pressure requires one.

---

## Phase L — Local Ecosystem / Root Deployment

Make root/ecosystem development a first-class Vapor-managed workflow.

Prove:

```text
acquire/fork/create source
→ build
→ test
→ stage local installation
→ run staged Vapor
→ operate on source using staged Vapor
```

This phase should include the canonical installed layout without requiring Steam deployment yet.

---

## Phase M — SDK / IDE Integration

Use the installed managed environment to make Content/Ecosystem development plug-and-play.

Especially:

* pinned Rust;
* stdlib source;
* rust-analyzer;
* Cargo environment;
* workspace/project attachment;
* RustRover integration.

Avoid fragile private IDE configuration until actual generated IDE state has been observed.

---

## Phase N — Provider and Authorization Integration

Add the infrastructure required for authenticated provider operations:

```text
Git
GitHub
Steam
SteamCMD
registry
```

Keep authentication/authorization separate from local Role.

Local Ecosystem Developer workflows must remain possible without official authority.

---

## Phase O — Steam Development Deployment

Revive the real Steam App/depot topology.

Target path:

```text
build Vapor installation
→ local staging succeeds
→ package depot
→ publish to vapor-dev
→ run Vapor from Steam installation
→ use installed Vapor to develop/build/deploy Vapor again
```

This is the next major self-hosting boundary after local deployment.

---

## Phase P — Registry / Publication / Distribution

Expand into:

* remote Vapor dependency acquisition;
* Registry resolution;
* GitHub Releases;
* source publication;
* Packagepack release artifacts;
* Workshop publication;
* yank/ban state;
* provider linkage;
* historical versions;
* production locking/provenance.

These should build on the already-proven local semantic model rather than defining it retroactively.

---

# Immediate Next Work

The immediate next implementation phase is:

> **Introduce Vapor Library Content and prove Vapor-managed Cargo dependency reconciliation using one tiny Library and one consuming project.**

Do not expand the Bevy example yet.

The sequence should be:

```text
Library Content
→ Cargo reconciliation
→ minimal Bevy ECS Engine
→ coherent Wheel Game
→ Engine Mod capability
→ cross-layer Game Mod
→ observe real static integration
→ generic Vapor realization
→ rich Packs / absurd example composition
```

This sequence ensures that when ECS extension APIs begin depending on one another, Vapor can already express those dependencies as real usable Rust package relationships.

---

# Development Method — Restated

Continue using:

> **Model enough → implement a vertical slice → observe real pressure → refine the model → continue.**

Avoid both extremes:

```text
design everything first
```

and:

```text
code arbitrary examples until semantics accidentally emerge
```

The current rewrite has progressed specifically because implementation and model refinement have repeatedly corrected one another.

That should remain intentional.

---

# Current Summary

In compressed form:

```text
Semantic Content model          ✅
Local discovery                 ✅
Packagepack vertical slice      ✅
Build/run realization           ✅ narrow
Pinned managed Rust             ✅
Vapor self-build/test           ✅
Role model                      ✅
CLI model + Clap                ✅
Generic Content resolution      ✅

Library Content                 NEXT
Cargo reconciliation            NEXT
Minimal Bevy ECS                queued
Explicit extension APIs         queued
Multi-layer meaningful Mods     queued
Generic static realization      queued
Pack ecosystem examples         queued
Creation/templates              queued
Local root deployment           queued
SDK/IDE integration             queued
Steam dev deployment            later
Registry/publication            later
```

The rewrite is no longer in bootstrap.

> **It is now in the transition from proving Vapor's semantic graph to making that graph supervise a real, extensible Rust ecosystem.**
