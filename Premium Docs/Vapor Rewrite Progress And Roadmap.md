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
semantic identity / structural-address implementation
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

**Status: architecture proof complete.**

Vapor has now proven the required conservative reconciliation behavior using the real `wheel-rules → wheel-core` Library relationship.

Observed proof:

```text
valid semantic dependency
→ physical Cargo dependency present
→ verify reports Valid

physical Cargo dependency removed
→ verify reports Missing

repair invoked outside the source checkout
→ Vapor resolves source context
→ Installation-owned managed Cargo adds the missing binding
→ verify reports Valid

physical binding deliberately changed to a different real Cargo package
→ verify reports Conflict

repair attempted
→ Vapor refuses unsafe overwrite
→ conflicting developer-authored Cargo state remains untouched

correct physical dependency restored
→ verify reports Valid

Vapor-managed Cargo
→ builds/tests the consuming wheel-rules package successfully
```

This proves the intended principle:

> **Vapor may safely establish missing physical realization state when the answer is unambiguous, but it must refuse to overwrite conflicting developer intent when it lacks sufficient provenance to decide safely.**

Phase B is complete.

Do not expand Cargo reconciliation speculatively.

Future reconciliation semantics should arise from actual authored-project pressure.

---

## Architecture Checkpoint — Context, Identity, Session, and SDK

The self-hosting workflow exposed that filesystem path, shell CWD, short Project names, durable development state, and future graphical SDK behavior had been conflated.

The normalized model distinguishes:

```text
Semantic Identity
Structural Address
Local Realization
Development Session
Open
Focus
Transient Selection
Operation Target Cardinality
Selector Resolution
```

Important invariant:

> **Structural Address is not Semantic Identity, and Local Realization is neither of those.**

The current implementation only needs to preserve a clean path toward the complete model.

The complete SDK is not a prerequisite for continued runtime work.

---

## Architecture Checkpoint — Client, Platform, and Repository Topology

The ecosystem terminology now distinguishes:

```text
Vapor Client
Vapor Platform Server
future Vapor App Server
```

together with:

```text
Platform Communication
≠
App-Session Communication
```

Target repository naming is:

```text
Vapor-Root
→ Vapor-Client

Vapor-Server-Root
→ Vapor-Platform-Server
```

The target model is approved.

The actual provider/repository migration is temporarily deferred until repo-scale integrated refactoring can update code, manifests, scripts, deployment state, documentation references, tests, and remembered development state coherently.

The active legacy provider names may therefore remain temporarily without reopening the naming design.

Do not allow the deferred migration to block runtime development.

---

## Phase C — Minimal Bevy ECS Engine

**Status: implemented at the architecture-proving level.**

The Terminal Engine now owns a real Bevy ECS application rather than a temporary callback-only fake runtime.

It currently owns:

```text
Bevy App / World
terminal lifecycle
input acquisition
Engine timing
Terminal framebuffer
Engine lifecycle control

EngineSet::Configure
EngineSet::Input
EngineSet::Simulate
EngineSet::Present
```

The Engine exposes Engine-specific ECS integration surfaces.

Crossterm remains an Engine implementation detail.

The Engine does not establish Bevy Plugin or any other particular mechanism as a universal Vapor ABI.

This satisfies the intended Phase C proof.

---

## Phase D — Coherent Wheel Game

**Status: implemented at the architecture-proving level.**

The Game now implements a real tiny wheel/spin game over the Terminal Engine.

Its authored/runtime vocabulary includes:

```text
LootTable
Wheel
GameMode
SpinStrategy
LootTableRegistry
WheelRegistry
GameModeRegistry
ActiveGameMode

WheelGameSet::Register
WheelGameSet::Input
WheelGameSet::Spin
WheelGameSet::Resolve
WheelGameSet::Present
```

The Game owns the meaning of those extension surfaces.

The existing Tiny Game Mod already proves that downstream Content may consume those Game-defined extension points by:

```text
attaching a system to Game registration
extending a Game-owned LootTable
implementing SpinStrategy
registering a new GameMode
selecting that GameMode
```

That is a valid basic Game-Mod extension proof.

It is not yet the richer cross-layer Game Mod required later.

Phase D is complete.

---

## Architecture Checkpoint — Dependency Direction, Targeting, and Constituency

The richer Engine/Game/Mod example exposed an important missing semantic distinction.

The dependency graph remains one graph.

However, different views and semantic relationships must be distinguished:

```text
Outbound Requirements
Inbound Dependents
Runtime Foundation
Extension Target
Selected Dependency Closure
Effective Constituency
```

The governing model is:

> **Selection comes from dependency reachability. Constituency comes from semantic classification of selected Content.**

For example:

```text
Game
├── requires Engine
├── requires Engine Mod A
└── requires Engine Mod B

Engine Mod A
└── targets Engine

Engine Mod B
└── targets Engine
```

derives:

```text
Effective Engine
├── Base Engine
├── Engine Mod A
└── Engine Mod B
```

without requiring the Packagepack to redundantly restate those Mods.

Likewise:

```text
Packagepack
→ Game Mod
→ Game
```

selects the Game Mod into the effective Game constituency.

A Registry entry targeting the same Engine/Game does not participate unless it is reachable from the selected dependency closure.

The complete normative model is defined by:

```text
Vapor Dependency, Target, Constituency And Composition Model.md
```

This checkpoint should be reflected in the resolver/composition model before generic static realization is implemented.

---

## Phase E — Effective Engine Constituency

Create the first meaningful Engine Mod and prove it as an actual constituent of the effective Engine.

A deliberately small candidate is:

```text
Engine Diagnostics Mod
```

which may consume Engine-owned facilities such as:

```text
EngineTime
TerminalFrame
EngineSet::Present
```

and visibly present diagnostic information.

The exact feature matters less than the semantic proof:

```text
Engine Mod
→ directly targets Terminal Engine
→ selected into Packagepack closure
→ derived as Effective Engine constituent
→ uses Engine-defined extension API
```

The Engine Mod should have its own Vapor manifest and ordinary Cargo project.

Do not add hardcoded Engine-Mod fields to Vapor realization.

---

## Phase F — Cross-Layer Game Mod

Create a Game Mod which genuinely requires capabilities across layers.

For example:

```text
Casino Game Mod
├── targets Wheel Game
├── requires Engine Diagnostics / Weighted Roll Engine Mod
└── requires supporting Libraries as appropriate
```

A stronger eventual candidate remains a Weighted Roll capability because it creates useful cross-layer semantics rather than purely visual demonstration.

The Game Mod should exercise:

```text
Game-owned extension points
Engine-owned API
Engine-Mod-owned capability
ordinary Library APIs
```

This proves that Vapor's dependency model can express:

> A Game-side constituent requiring a particular effective Engine constituency.

No hardcoded notion of which Mod is allowed to depend on which other Mod count should exist.

---

## Phase G — Manually Prove Real Static Integration

Before teaching Vapor a generic static-realization algorithm, manually prove one ordinary Rust composition containing at least:

```text
Base Engine
Engine Mod
Base Game
Game Mod
Libraries
```

The manually assembled composition should be derived conceptually from the same selected dependency graph Vapor resolves.

Observe:

```text
which Rust packages must be linked
which APIs are called
which Content requires initialization
which ordering constraints are genuine
which constraints belong to Engine/Game semantics
which information Vapor actually needs to generate
```

Do not prematurely decree:

```text
all behavioral Content implements one universal Vapor trait
all Mods are Bevy Plugins
all Content exposes install(&mut App)
```

unless implementation pressure actually proves such a contract necessary.

The purpose of Phase G is to discover the smallest correct static integration contract.

---

## Phase H — Generic Graph-Derived Static Realization

Replace the current vertical-slice realization machinery.

The following model must disappear:

```rust
struct AppPackages {
    engine: RustPackage,
    game: RustPackage,
    game_mod: RustPackage,
}
```

and the generated composition must no longer assume:

```rust
engine::run(|app| {
    game::install(app);
    game_mod::install(app);
});
```

Those structures were valid architecture-proving scaffolding.

They are not the production model.

The new realization should derive from:

```text
ResolvedContentGraph
        ↓
validated target relationships
        ↓
Effective Engine constituency
+
Effective Game constituency
+
support dependency closure
        ↓
physical Rust package realization
        ↓
smallest Engine-defined static integration contract discovered in Phase G
```

The number of Mods must not be represented in Vapor Core by fixed struct fields.

Conceptually the composition model should evolve toward:

```text
ResolvedComposition
├── graph
├── EffectiveEngine
│   ├── base
│   └── constituents
└── EffectiveGame
    ├── base
    └── constituents
```

Exact Rust ownership/storage types should be chosen during implementation rather than copied literally from documentation.

---

## Phase I — Rich Pack Examples

After generic realization can handle graph-derived constituency, introduce meaningful:

```text
Enginepack
Gamepack
Modpack
multiple Packagepacks
```

Examples should exercise real selection behavior.

For example:

```text
Enginepack
├── Terminal Engine
├── Diagnostics Engine Mod
└── Weighted Roll Engine Mod

Gamepack
├── Wheel Game
└── Casino Game Mod
```

A Packagepack may then select these reusable fragments into one complete closure.

The point is to prove that Packs are declarative composition selectors rather than special runtime containers.

---

## Phase J — Extension Ecosystem Explosion

Deliberately demonstrate that an addon may itself become an extension platform.

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

Extension Mods should derive their effective Engine/Game side through their target chain.

The example implementation may remain tiny.

The dependency/composition topology should become intentionally ridiculous.

---

## Phase K — Content Creation and Templates

Implement creation workflows for:

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

Use small built-in canonical templates first.

Do not create a generalized remote template ecosystem until real pressure requires it.

---

## Phase L — Local Ecosystem Deployment

**Status: initial architecture-proving implementation already exists ahead of roadmap order.**

Vapor can build the active Vapor Workspace and deploy the resulting Vapor binaries back into the canonical Steam App Instance.

Continue refining this only when runtime/development pressure requires it.

Do not reopen it merely because its historical roadmap phase appears later.

---

## Phase M — SDK / IDE Integration

Continue toward plug-and-play development using:

```text
managed pinned Rust
stdlib source
rust-analyzer
Cargo environment
Workspace/Project attachment
SDK
external IDE integration
```

The integrated SDK remains the primary first-party graphical development surface.

External IDEs remain complementary.

---

## Phase N — Provider and Authorization Integration

Add provider-backed operations for:

```text
Git
GitHub
Steam
SteamCMD
Registry
```

while preserving:

```text
Role
≠
Authority
```

Local Ecosystem Developer workflows must remain possible without official authority.

---

## Phase O — Steam Development Deployment

**Status: backend work exists ahead of roadmap order; complete production proof remains future work.**

Target:

```text
build Vapor installation
→ local stage/deploy
→ SteamPipe preview
→ publish vapor-dev
→ launch Steam-installed Vapor
→ use installed Vapor to develop/build/deploy Vapor again
```

---

## Phase P — Registry / Publication / Distribution

Expand from the proven local semantic model into:

```text
remote Content acquisition
Registry resolution
provider linkage
GitHub release/source publication
Packagepack release artifacts
Workshop publication
yank/ban
historical versions
production provenance/locking
```

These systems should consume the established dependency/composition model rather than redefine it.

---

# Immediate Next Work

The current immediate sequence is:

```text
Phase B
    COMPLETE

Phase C
    COMPLETE

Phase D
    COMPLETE

        ↓

dependency / target / constituency model correction
        ↓
update resolver/composition structures enough to express it cleanly
        ↓
Phase E — first Engine Mod
        ↓
derive Effective Engine constituency
        ↓
Phase F — cross-layer Game Mod
        ↓
Phase G — manually observe real static integration
        ↓
Phase H — DELETE fixed AppPackages realization
        ↓
generic graph-derived static realization
```

Do not perform the deferred repository migration as a prerequisite.

Do not build the complete SDK as a prerequisite.

Do not expand Cargo reconciliation without new pressure.

The immediate architecture question is now:

> **How does one resolved Vapor dependency graph derive a complete effective Engine/Game constituency which can later be realized statically without Vapor inventing the participating ecosystem's runtime semantics?**

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
preserve vertical-slice scaffolding until it accidentally becomes architecture
```

The current constituency correction is a direct example of the method working correctly.

The existing hardcoded App realization was useful because it produced enough real Engine/Game/Mod pressure to reveal the model it now needs to become.

---

# Current Summary

```text
Semantic Content model                  ✅
Local discovery                         ✅
Packagepack vertical slice              ✅
Pinned managed Rust                     ✅
Vapor self-build/test                   ✅
Role / authority model                  ✅
CLI model                               ✅
Generic Content resolution              ✅
Library Content                         ✅
Cargo inspection/reconciliation         ✅ proven
Managed Cargo Project resolution        ✅
Local ecosystem deployment              ✅ initial
Steam deployment backend                ✅ initial

Minimal Bevy ECS Engine                 ✅
Coherent Wheel Game                     ✅
Basic Game Mod extension                ✅

Dependency target semantics             MODEL CORRECTION NOW
Effective Engine/Game constituency       NEXT
Engine Mod capability                    NEXT
Cross-layer Game Mod                     queued
Manual rich static integration           queued
Generic graph-derived realization        queued
Rich Pack ecosystem                      queued
Extension ecosystem explosion            queued

Repository migration                    designed / deferred
Full source cleanroom/docsification      planned
SDK graphical implementation             later checkpoint
Registry/publication                     later
```

The rewrite has now crossed from merely resolving Content graphs into modeling how those graphs describe real effective runtime compositions.

> **The next milestone is not “support more Mods.” It is making arbitrary Mod cardinality and constituency a natural consequence of the graph rather than a special case in Vapor's code.**
