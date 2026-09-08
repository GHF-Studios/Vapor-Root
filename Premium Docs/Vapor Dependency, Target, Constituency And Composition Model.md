> [!info]
> This document defines the semantic relationship between Vapor Content dependencies, resolved dependency closure, runtime foundation, extension targets, effective Engine/Game constituency, packs, and final Vapor App Composition.
>
> It owns the **semantic Content graph**.
>
> It does not own:
>
> * canonical source hierarchy;
> * operation Context or Selection;
> * physical Cargo realization;
> * runtime extension APIs;
> * publication transport/provider mechanics.
>
> Canonical Project/Content identity is defined by the **Vapor Context, Identity, Session And Selection Model**.
>
> Published version semantics are defined by the **Vapor Publishing And Distribution Model**.
>
> Physical Rust/Cargo realization is downstream of this model and must not define it retroactively.

---

# Core Thesis

Vapor has one semantic dependency graph.

It does not maintain unrelated graphs for:

```text
ordinary dependencies

Mods

packs

composition membership

effective Engine/Game state
```

Instead:

> **Vapor resolves one authored requirement graph and derives additional semantic views over that graph.**

Important derived views include:

```text
Outbound Requirements

Inbound Dependents

Runtime Foundation

Extension Targets

Resolved Dependency Closure

Effective Engine Constituency

Effective Game Constituency

Support Dependencies
```

These are different meanings/projections of the same resolved Content topology.

---

# Content Nodes

The nodes in the semantic graph are exact Vapor Content versions.

A Content node is therefore conceptually:

```text
Content Project identity
+
resolved version
```

For published Content:

```text
canonical Project / Vapor ID
+
SemVer
```

Example:

```text
GHF-Studios/Loo-Cast/Game/Loo-Cast
@ 1.4.2
```

The Project identity comes from:

```text
Authority
/ Source Repo Container
/ Source Repo
/ Project
```

The source hierarchy does not itself determine dependency meaning.

It identifies the Content participating in the graph.

---

# Content Project vs Content Artifact

Current Vapor terminology should prefer:

```text
Content Project

Content version

resolved Content node
```

over a vague independent:

```text
Content artifact
```

because ordinary Vapor Content identity is carried by the Project itself.

A Project may have kind:

```text
Engine
Game
Engine Mod
Game Mod
Extension Mod
Library
Enginepack
Gamepack
Modpack
Packagepack
```

and that kind contributes to semantic graph interpretation.

---

# Authored Requirement

A **Vapor Dependency** is an authored semantic requirement from one Content Project/version requirement to another.

Conceptually:

```text
A
↓ requires
B
```

means:

> A requires a compatible selected version of B to participate in its resolved dependency graph.

Every dependency therefore has the basic direction:

```text
depender
→ dependency
```

Examples:

```text
Game
→ Engine

Game
→ Library

Game
→ Engine Mod

Game Mod
→ Game

Game Mod
→ Library
```

All are requirements.

Some additionally carry higher-level semantic meaning.

---

# Dependency Binding

An authored dependency should have a stable local binding/name within the depender.

Conceptually:

```text
binding
    terminal_engine

requirement
    GHF-Studios/.../Terminal-Engine @ ^1.2
```

The binding is useful to authored configuration and implementation.

It is not the dependency's canonical identity.

The canonical identity remains the resolved Content Project/version.

---

# Outbound Requirements

The dependencies authored by one Content Project are its **Outbound Requirements**.

For example:

```text
Casino Game
├── Terminal Engine
├── Weighted Roll Engine Mod
├── Wheel Rules Library
└── Economy Library
```

Outbound requirements answer:

> **What must participate for this Content to exist coherently?**

They are authored.

---

# Inbound Dependents

An **Inbound Dependent** is the reverse view of a resolved requirement edge.

If:

```text
A → B
```

then within that resolved graph:

```text
A
```

is an inbound dependent of:

```text
B
```

Inbound relationships are derived.

They are not separately authored.

---

# Inbound Dependents Are Graph-Scoped

A Registry may contain thousands of Projects which can depend on the same Engine.

That does not mean those Projects participate in one composition.

Therefore:

> **Inbound Dependents are computed relative to the resolved graph being inspected, not by blindly treating every Registry-wide dependent as part of the composition.**

Registry discovery and composition inclusion are different concepts.

---

# Resolved Dependency Graph

Resolving a Content root produces a graph of exact Content versions.

Conceptually:

```text
authored root
    ↓
version constraints
    ↓
deterministic resolution
    ↓
ResolvedContentGraph
```

Each resolved node contains an exact Content version.

Each resolved edge corresponds to an authored requirement.

---

# Resolved Dependency Closure

The **Resolved Dependency Closure** of a root is the transitive set of exact Content versions reachable from that root through resolved requirements.

Example:

```text
Packagepack
├── Game Mod
│   └── Game
│       ├── Engine Mod
│       │   └── Engine
│       └── Library
└── another Library
```

Everything reachable through those requirements belongs to the closure.

Content which merely exists in the Registry but is not reachable does not participate.

---

# Dependency Inclusion vs Operation Selection

This distinction is important because **Selection** now has a precise meaning elsewhere in Vapor.

In the CLI/Core operation model:

```text
Operation Selection
```

means:

> an explicit subset/refinement of an operation subject.

That is unrelated to dependency-graph reachability.

Therefore this document should prefer:

```text
resolved dependency inclusion

resolved dependency closure

composition inclusion
```

rather than calling graph reachability simply:

```text
Selection
```

The two concepts must not be conflated.

---

# Inclusion Rule

The governing composition rule is:

> **Dependency inclusion comes from resolved reachability; constituency comes from semantic classification of included Content.**

This is one of the central invariants of the model.

---

# Dependency Meaning Is Layered

Every resolved edge first means:

```text
requires
```

Some edges additionally have higher-level semantic meaning.

Examples:

```text
Game → Engine
    Runtime Foundation

Engine Mod → Engine
    Extension Target

Game Mod → Game
    Extension Target

Extension Mod → Mod
    Extension Target
```

These semantic roles refine dependency meaning.

They do not replace it.

An Extension Target is still a required dependency.

---

# Semantic Relationship Kinds

The current minimum useful relationship classes are approximately:

```text
Ordinary Requirement

Runtime Foundation

Extension Target
```

The exact Rust enum/type names are implementation details.

Do not invent additional authored relationship kinds unless real Content semantics require them.

---

# Runtime Foundation

A Game has exactly one Engine foundation.

Conceptually:

```text
Game
→ Engine
```

means:

```text
Game requires Engine
```

and additionally:

```text
Game is authored against this Engine foundation
```

This is a Vapor semantic relationship.

It does not prescribe how the Game programmatically integrates with the Engine.

---

# Exactly One Game Foundation Engine

For one Game definition/version:

```text
Game
→ exactly one Engine foundation
```

is the intended semantic model.

A Game should not resolve against several unrelated base Engines simultaneously unless a future genuinely different Content model explicitly introduces such semantics.

---

# Extension Target

A Mod has one primary **Extension Target** appropriate to its kind.

Conceptually:

```text
Engine Mod
→ Engine

Game Mod
→ Game

Extension Mod
→ another Mod
```

The target relationship answers:

> **What behavioral Content does this Mod conceptually extend?**

It does not answer:

> How does extension work at runtime?

---

# Extension API Ownership

The extension target owns the programmatic extension vocabulary.

Possible mechanisms include:

```text
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

callbacks

schemas

data-driven registration

other authored contracts
```

Vapor should not invent one universal Mod ABI merely because it understands that a Mod targets something.

---

# Additional Mod Dependencies

A Mod may require any number of additional dependencies.

For example:

```text
Engine Mod A
├── targets Engine
├── requires Engine Mod B
└── requires Library C
```

or:

```text
Game Mod
├── targets Game
├── requires Engine Mod
├── requires Library A
├── requires Library B
└── requires another compatible capability
```

Only one dependency is the primary Extension Target.

The others remain ordinary requirements unless another semantic role genuinely applies.

---

# No Artificial Dependency Direction Restrictions

A higher runtime layer may legitimately require Content associated with a lower runtime layer.

For example:

```text
Game
→ Engine Mod
→ Engine
```

may mean:

> This Game requires an Engine environment augmented by this Engine Mod.

Likewise:

```text
Game Mod
→ Engine Mod
→ Engine
```

may mean:

> This Game Mod requires a capability supplied by this Engine Mod.

This is valid dependency topology.

Vapor must not prohibit it merely because someone imagines dependencies should flow in one aesthetically simple layer direction.

---

# Constituency

A **Constituent** is included behavioral Content which participates in forming an effective Engine-side or Game-side runtime composition.

Constituency is normally derived from:

```text
Resolved Dependency Closure

+

Content kind

+

Extension Target chain
```

It should not require redundant authored configuration such as:

```text
role = "constituent"
```

when Vapor can derive the answer exactly.

---

# Constituency Example

Suppose:

```text
Casino Game
├── Terminal Engine
├── Diagnostics Engine Mod
└── Weighted Roll Engine Mod
```

and:

```text
Diagnostics Engine Mod
→ targets Terminal Engine

Weighted Roll Engine Mod
→ targets Terminal Engine
```

Then the Engine Mods are included because the Game requires them.

Their target relationships classify them as Engine-side constituents.

Therefore:

```text
Effective Engine
├── Terminal Engine
├── Diagnostics Engine Mod
└── Weighted Roll Engine Mod
```

No second explicit constituent list is required.

---

# Base Content vs Effective Content

The model distinguishes:

```text
Base Engine
```

from:

```text
Effective Engine
```

and:

```text
Base Game
```

from:

```text
Effective Game
```

Conceptually:

```text
Effective Engine
├── Base Engine
└── Engine-side Constituents
```

and:

```text
Effective Game
├── Base Game
└── Game-side Constituents
```

---

# Base Cardinality

One complete Vapor App Composition has:

```text
exactly one Base Engine

exactly one Base Game
```

Their effective forms may contain arbitrarily many compatible behavioral constituents.

---

# Effective Engine

The **Effective Engine** consists conceptually of:

```text
one Base Engine

+

all included Engine Mods whose target chain terminates on that Engine

+

included Extension Mods belonging to those Engine-side Mod chains
```

Example:

```text
Effective Engine
├── Terminal Engine
├── Diagnostics Engine Mod
├── Weighted Roll Engine Mod
└── Weighted Roll Debug Extension Mod
```

---

# Effective Game

The **Effective Game** consists conceptually of:

```text
one Base Game

+

all included Game Mods whose target chain terminates on that Game

+

included Extension Mods belonging to those Game-side Mod chains
```

Example:

```text
Effective Game
├── Wheel Game
├── Casino Game Mod
└── Roguelike Casino Extension Mod
```

---

# Extension Mod Constituency

An Extension Mod inherits Engine-side or Game-side constituency through its target chain.

For example:

```text
Extension C
→ Extension B
→ Engine Mod A
→ Engine
```

means:

```text
Engine Mod A
Extension B
Extension C
```

all belong to the Engine-side constituency if included in the resolved closure.

Likewise:

```text
Extension C
→ Game Mod B
→ Game
```

places that extension chain on the Game side.

---

# Invalid Target Chains

Target chains must eventually terminate on the appropriate base behavioral kind.

Valid examples:

```text
Extension Mod
→ Engine Mod
→ Engine
```

```text
Extension Mod
→ Extension Mod
→ Game Mod
→ Game
```

Invalid examples should be rejected if they cannot produce a coherent target classification.

For example:

```text
Extension Mod
→ Library
```

is invalid under the current Content-kind model because Library is not a Mod extension target.

---

# Target Compatibility

An included constituent must ultimately target the actual selected base appropriate to its side.

For example:

```text
Engine Mod
→ Engine X
```

cannot silently become a constituent of:

```text
Effective Engine
    Base Engine Y
```

Likewise:

```text
Game Mod
→ Game X
```

cannot silently become a constituent of:

```text
Effective Game
    Base Game Y
```

Target-chain validation must reject such compositions.

---

# Target Identity Is Versioned Content

Compatibility is evaluated against resolved Content identities/versions, not merely display names.

Conceptually:

```text
Engine Mod A @ 2.0.0
targets
Engine X @ compatible requirement
```

The resolver first determines exact versions.

Target validation then evaluates the resulting exact graph.

---

# No Registry-Wide Auto-Installation

This must never happen:

```text
Registry contains 900 Engine Mods targeting Engine X
↓
Effective Engine X contains 900 Mods
```

Target compatibility does not imply inclusion.

A Mod participates only if it belongs to the resolved dependency closure.

Again:

> **Reachability determines inclusion. Target semantics determine constituency.**

---

# Why Base Content Does Not Normally Require Its Own Mods

A base target generally cannot directly require a Mod which itself targets that same target without creating a dependency cycle.

For example:

```text
Game
→ Game Mod
→ Game
```

is cyclic.

Likewise:

```text
Engine
→ Engine Mod
→ Engine
```

is cyclic.

This is useful.

A base Engine or Game owns its intrinsic behavior directly.

External augmentation is selected from another depender in the graph.

---

# Valid Higher-Level Mod Inclusion

Examples include:

```text
Game
→ Engine Mod
→ Engine
```

```text
Enginepack
→ Engine Mod
→ Engine
```

```text
Gamepack
→ Game Mod
→ Game
```

```text
Packagepack
→ Engine Mod
→ Engine
```

```text
Packagepack
→ Game Mod
→ Game
```

```text
Game Mod
→ Engine Mod
→ Engine
```

There is no separate universal “install Mod” graph.

These are ordinary requirements plus semantic target classification.

---

# Support Dependencies

Not every included dependency is a behavioral constituent.

The clearest current example is:

```text
Library
```

A Library may be required by:

```text
Engine

Game

Mod

another Library

pack-related implementation
```

and participate physically in the final build.

That does not make it an Engine or Game constituent.

---

# Support Dependency Rule

Conceptually:

```text
Resolved Dependency Closure
=
behavioral constituents
+
support dependencies
+
declarative composition nodes
```

The exact categories depend on Content kinds.

Do not collapse:

```text
everything reachable
```

into:

```text
effective Engine/Game constituent
```

---

# Libraries

A **Library** is reusable Content without inherent:

```text
Engine

Game

Mod

pack

runnable-composition
```

semantics.

Its inclusion answers:

> This API/implementation is required.

It does not answer:

> This Library modifies the Engine.

Only explicit behavioral Content kinds/relationships carry constituency semantics.

---

# Packs

Packs are declarative composition selectors/fragments.

Current pack kinds include:

```text
Enginepack

Gamepack

Modpack

Packagepack
```

They participate in dependency resolution.

They do not automatically become runtime behavioral constituents.

---

# Enginepack

An **Enginepack** describes a reusable Engine-side composition fragment.

Example:

```text
Enginepack
├── Terminal Engine
├── Diagnostics Engine Mod
└── Weighted Roll Engine Mod
```

This can contribute:

```text
Base Engine

Engine-side constituents

support dependencies
```

to a larger composition.

---

# Gamepack

A **Gamepack** describes a reusable Game-side composition fragment.

Example:

```text
Gamepack
├── Wheel Game
├── Casino Game Mod
└── Economy Library
```

This can contribute:

```text
Base Game

Game-side constituents

support dependencies
```

to a larger composition.

Its Game must still ultimately resolve against a coherent Engine foundation.

---

# Modpack

A **Modpack** groups Mods and their requirements into a reusable composition fragment.

Example:

```text
Modpack
├── Engine Mod
│   └── Engine
└── Game Mod
    └── Game
```

A Modpack may therefore contribute to both effective sides.

It does not create one new universal Mod target.

---

# Packagepack

A **Packagepack** is the complete authored Vapor App composition root.

It resolves to a complete graph from which Vapor derives:

```text
exactly one Base Engine

exactly one Base Game

Effective Engine

Effective Game

support dependency closure
```

and therefore one:

```text
Vapor App Composition
```

---

# Packagepack Does Not Need Redundant Lists

If a Packagepack requires a Game, and that Game requires an Engine Mod, the Packagepack need not also list the Engine Mod merely to make it participate.

Example:

```text
Packagepack
└── Game
    ├── Engine
    └── Engine Mod
        └── Engine
```

The Engine Mod is already in the resolved dependency closure.

Its target relationship classifies it into Effective Engine constituency.

Redundant duplicate composition declarations should not be required.

---

# Multiple Paths to the Same Content

The same exact Content version may be reachable through several requirement paths.

Conceptually:

```text
Packagepack
├── Game
│   └── Library X
└── Game Mod
    └── Library X
```

The resolved graph contains one semantic node for the selected exact version of Library X, with multiple inbound edges.

Reachability multiplicity does not imply duplicate Content identity.

---

# Version Conflict

If requirement paths demand incompatible versions which cannot coexist under Vapor's resolution policy, resolution fails.

Example:

```text
Game
→ Library X ^1

Game Mod
→ Library X ^2
```

If one legal composition cannot satisfy both, Vapor should report the exact conflicting requirements.

It must not choose arbitrarily.

---

# Duplicate Major Versions

Vapor may permit different major versions of one Content identity across different Vapor App Compositions.

Within one composition, the conservative model avoids multiple incompatible majors of the same canonical Content identity unless an explicit disambiguation mechanism permits it.

Cargo-style renaming/disambiguation may eventually provide advanced support where genuinely required.

---

# Cycles

Ordinary Vapor dependency cycles are invalid unless a future explicitly modeled relationship provides semantics for them.

Examples:

```text
A → B → A
```

and:

```text
Game → Game Mod → Game
```

should fail.

Cycle diagnostics should include the actual path.

---

# Generic Resolution Roots

Dependency resolution is not inherently Packagepack-only.

A supported Content Project may be resolved as a root for purposes such as:

```text
inspection

development

dependency analysis

build/test preparation

publication validation
```

Conceptually:

```text
Content root
    ↓
Resolved Dependency Graph
```

Packagepack adds complete-composition requirements on top.

---

# Complete Composition Validation

Only a Packagepack needs to satisfy the complete runnable Vapor App invariant.

That validation includes:

```text
exactly one Base Engine

exactly one Base Game

coherent Runtime Foundation

valid constituent target chains

compatible dependency graph
```

A Library resolved by itself obviously need not produce an Engine/Game composition.

---

# One Graph, Multiple Views

The same resolved graph may be projected as:

```text
forward requirements

reverse dependents

Runtime Foundation

Extension Targets

Effective Engine constituency

Effective Game constituency

support dependency closure

pack contribution

Cargo realization requirements
```

These are projections.

They should not become separately authored or separately maintained sources of truth.

---

# Graph-Derived Semantics

Whenever Vapor can derive a semantic relationship exactly from:

```text
resolved graph

Content kind

known relationship rules
```

prefer deriving it over duplicating it in authored manifests.

Examples:

```text
Game's Engine foundation

Engine Mod target

Game Mod target

Extension Mod target

Engine/Game side constituency
```

Do not make users synchronize redundant truths.

---

# Current Manifest Direction

The current basic dependency shape remains conceptually sufficient:

```toml
[dependencies.some-binding]
id = "..."
version = "..."
```

The exact future manifest schema may move/rename fields.

The architectural point remains:

```text
binding

canonical dependency identity

version requirement
```

are enough for ordinary dependency edges.

---

# Inferring Special Relationships

Under the current Content-kind model, some special relationships can be inferred structurally.

For example, an Engine Mod must identify/require an Engine target.

A Game Mod must identify/require a Game target.

An Extension Mod must identify/require another Mod target.

A Game must identify/require one Engine foundation.

The exact authored syntax may distinguish these relationships more explicitly if that improves clarity.

What should not happen is introducing redundant fields solely because the resolver implementation finds them convenient.

---

# Explicit Relationship Fields

An explicit authored relationship field becomes justified if real Content semantics create ambiguity that cannot be resolved safely from kind and dependency structure.

For example, a future kind might require several dependencies of the same eligible kind but only one should be considered its target.

At that point explicit syntax may be correct.

The rule is:

> **Derive when semantics are unambiguous; author explicitly when intent cannot otherwise be known.**

---

# Resolved Edge Model

The implementation should eventually represent more than:

```text
binding
→ ContentVersionId
```

Conceptually, a resolved edge may expose:

```text
binding

depender

dependency

authored version requirement

resolved version

semantic relationship
```

with relationship approximately:

```text
Ordinary Requirement

Runtime Foundation

Extension Target
```

This makes semantic views explicit without creating several unrelated graphs.

---

# Resolved Node Model

A resolved node conceptually contains:

```text
canonical Content Project identity

resolved exact version

Content kind

relevant authored metadata
```

Source Repo/provider information can be reached through canonical source/Registry topology when needed.

It need not become the dependency graph's primary identity model.

---

# Resolved Composition Model

A complete Packagepack composition should evolve conceptually toward:

```text
ResolvedComposition
├── graph
├── EffectiveEngine
│   ├── base
│   └── constituents
├── EffectiveGame
│   ├── base
│   └── constituents
└── support dependencies
```

rather than:

```text
one Engine ID

one Game ID

one hardcoded Game Mod

other unstructured nodes
```

---

# Conceptual Rust Shape

Illustratively:

```rust
ResolvedComposition {
    graph,
    engine: EffectiveEngine {
        base,
        constituents,
    },
    game: EffectiveGame {
        base,
        constituents,
    },
}
```

The actual Rust type layout is an implementation decision.

Do not fossilize this pseudocode as a required storage design.

---

# Constituents Should Reference Graph Nodes

Where practical, Effective Engine/Game views should reference the canonical nodes already present in the resolved graph.

They should not duplicate full Content records into a second independently mutable structure.

Conceptually:

```text
Resolved graph
    authoritative exact nodes/edges

EffectiveEngine
    references graph nodes

EffectiveGame
    references graph nodes
```

This reinforces one-graph/multiple-views architecture.

---

# Composition Does Not Define Integration API

Knowing:

```text
Engine Mod A
is an Effective Engine constituent
```

does not tell Vapor:

```text
call install(app)
```

Knowing:

```text
Game Mod B
is an Effective Game constituent
```

does not tell Vapor:

```text
register Bevy Plugin
```

Those are separate runtime-integration semantics.

---

# Composition Does Not Define Runtime Order

Constituency membership also does not automatically define one universal registration order.

Ordering constraints may arise from:

```text
dependency edges

target-defined extension contracts

generated integration requirements

Engine/Game lifecycle semantics
```

Vapor must preserve genuine dependencies.

It should not invent arbitrary universal ordering solely from Content kind.

---

# Vapor's Responsibility

At this layer Vapor determines:

```text
which exact Content versions participate

which requirements connect them

which Engine is the runtime foundation

which Mods target which behavioral Content

which behavioral nodes form Effective Engine

which behavioral nodes form Effective Game

which nodes are support dependencies

whether the complete graph is semantically valid
```

---

# Participating Ecosystem's Responsibility

The Engine/Game/Mod/Library ecosystem determines:

```text
what APIs exist

what extension means

what registration means

how runtime state is represented

what lifecycle exists

what integration order is required beyond dependency constraints

what Bevy/Rust concepts are used
```

This boundary is fundamental.

---

# Physical Realization

Physical Cargo/Rust realization is downstream of semantic composition.

The intended direction is:

```text
authored Vapor manifests
        ↓
semantic dependency resolution
        ↓
Resolved Dependency Closure
        ↓
semantic relationship classification
        ↓
target validation
        ↓
effective constituency derivation
        ↓
Resolved Vapor App Composition
        ↓
Cargo physical realization
        ↓
minimal generated/static integration contract
        ↓
Engine-defined runtime
```

Cargo does not decide constituency.

Generated Rust glue does not decide constituency.

They realize semantics Vapor has already resolved.

---

# Cargo Graph Is Distinct

The resulting Cargo package graph may not structurally resemble the Vapor graph one-to-one.

For example:

```text
one Vapor Project
    may contain several Cargo packages

several Vapor dependencies
    may share transitive Cargo dependencies

Vapor semantic relationship
    may require generated glue not represented as one dependency edge
```

Therefore:

```text
Vapor semantic graph
≠
Cargo physical graph
```

while both remain inspectable and reconcilable.

---

# Static Composition

Vapor's current application model is statically composed.

That means:

> The resolved Content graph determines what participates in the built Vapor App.

It does not mean every extension mechanism must be compile-time Rust generics or direct calls.

The resulting Engine may still provide dynamic runtime facilities.

---

# Static Composition vs Runtime Dynamicity

Example:

```text
Packagepack selects Engine Mod
    ↓
Engine Mod compiled into Vapor App
    ↓
Engine Mod registers runtime ECS systems
    ↓
those systems behave dynamically during runtime
```

Static inclusion and dynamic runtime behavior coexist naturally.

---

# Published Packagepack Resolution

For a published Packagepack release, the exact resolved dependency graph used to build that release becomes historical provenance.

Later publication of newer compatible dependency versions must not retroactively alter the old release's meaning.

Therefore:

```text
authored requirements
    may contain ranges

published resolved composition
    contains exact versions
```

---

# Development Re-Resolution

During development, dependency edits or newly available versions may cause a fresh resolution.

That may produce a different development graph.

This does not mutate historical published compositions.

---

# Worked Example — Engine Constituency

Suppose:

```text
Casino Game
├── Terminal Engine
├── Weighted Roll Engine Mod
└── Wheel Rules Library
```

and:

```text
Weighted Roll Engine Mod
→ Terminal Engine
```

Resolved closure:

```text
Casino Game
Terminal Engine
Weighted Roll Engine Mod
Wheel Rules Library
```

Classification:

```text
Base Engine:
    Terminal Engine

Effective Engine constituents:
    Weighted Roll Engine Mod

Support dependencies:
    Wheel Rules Library
```

No additional constituent declaration is required.

---

# Worked Example — Cross-Layer Game Mod

Suppose:

```text
Casino Game Mod
├── targets Wheel Game
├── requires Weighted Roll Engine Mod
└── requires Casino API Library
```

and:

```text
Wheel Game
→ Terminal Engine

Weighted Roll Engine Mod
→ Terminal Engine
```

Then the Mod requires:

```text
Game-side capability:
    Wheel Game

Engine-side capability:
    Weighted Roll Engine Mod

support API:
    Casino API Library
```

This is valid.

The Game Mod can therefore require an augmented Effective Engine environment.

---

# Worked Example — Extension Chain

Suppose:

```text
Roguelike Casino Extension
→ Casino Game Mod
→ Wheel Game
```

Then, if all are included:

```text
Effective Game
├── Wheel Game
├── Casino Game Mod
└── Roguelike Casino Extension
```

The Extension Mod's side is inherited from its target chain.

---

# Worked Example — Unrelated Registry Mod

Suppose the Registry contains:

```text
100 Game Mods targeting Wheel Game
```

but the resolved Packagepack closure includes only:

```text
Casino Game Mod
```

Then:

```text
Effective Game
├── Wheel Game
└── Casino Game Mod
```

The other 99 do not participate.

Compatibility/discoverability does not imply composition inclusion.

---

# Worked Example — Pack Contribution

Suppose:

```text
Enginepack
├── Terminal Engine
└── Diagnostics Engine Mod

Gamepack
├── Wheel Game
└── Casino Game Mod

Packagepack
├── Enginepack
├── Gamepack
└── Economy Library
```

After resolution and flattening semantic views:

```text
Effective Engine
├── Terminal Engine
└── Diagnostics Engine Mod

Effective Game
├── Wheel Game
└── Casino Game Mod

Support Dependencies
└── Economy Library
```

The packs remain declarative graph nodes/provenance.

They are not runtime behavioral constituents merely because they selected the Content.

---

# Diagnostic Requirements

Composition failures should explain the graph relationship which failed.

Examples:

```text
dependency version conflict

cycle

missing required Content

multiple Base Engines

multiple Base Games

missing Runtime Foundation

Engine Mod targets wrong Engine

Game Mod targets wrong Game

Extension Mod target chain invalid

Packagepack does not resolve to complete composition
```

Diagnostics should show relevant canonical identities and dependency paths.

---

# Target Mismatch Diagnostic

Example:

```text
error: Engine Mod target is incompatible with resolved Base Engine

Engine Mod:
    Some-Author/Mods/Engine/My-Mod @ 1.2.0

targets:
    Some-Author/Engines/Core/Engine-X @ ^2

resolved Base Engine:
    Other-Author/Engine/Core/Engine-Y @ 4.0.0
```

The error should explain the actual topology rather than saying merely:

```text
incompatible mod
```

---

# Dependency Conflict Diagnostic

Example:

```text
error: dependency requirements cannot be resolved

Wheel Game requires:
    Economy Library ^1

Casino Game Mod requires:
    Economy Library ^2

no legal single version satisfies both requirements
```

Where more sophisticated disambiguation is supported later, diagnostics may explain those alternatives.

---

# Cycle Diagnostic

Example:

```text
error: Vapor dependency cycle detected

Game
→ Casino Game Mod
→ Game
```

This teaches why a base Game cannot normally directly require a Mod which targets itself.

---

# Graph Inspection

The SDK/CLI should eventually permit inspection of several views over the same resolved graph.

Examples:

```text
Requirements

Dependents

Targets

Effective Engine

Effective Game

Support Dependencies

Pack Contribution

Physical Cargo Realization
```

These views should remain linked to the same exact nodes.

---

# Operation Selection Is Orthogonal

A user may run:

```text
test <Packagepack>
```

with an Operation Selection that narrows which Project/subtree is tested.

That operation-scoping Selection does not mutate the Packagepack's semantic dependency graph.

Likewise, choosing a Source Repo in the SDK does not mean that only dependencies from that Repo participate in composition.

Operational targeting and Content dependency resolution are separate layers.

---

# Source Topology Is Orthogonal

Moving from:

```text
Content semantic graph
```

to:

```text
Authority
/ Container
/ Source Repo
/ Project
```

is an identity/source lookup concern.

The dependency graph may connect Projects across:

```text
different Source Repos

different Source Repo Containers

different Authorities
```

provided their identities/versions and compatibility rules permit it.

Source grouping does not define semantic dependency boundaries.

---

# Registry Is Discovery/Resolution Infrastructure

The Registry may know:

```text
Content identities

versions

dependencies

provider linkage

Yank/Ban state

compatibility metadata
```

but Registry presence does not create composition membership.

The graph begins from authored requirements and deterministic resolution.

---

# Dependency Model and Publication

Published dependency requirements are immutable parts of a published Content version.

For example:

```text
Casino Game Mod @ 1.0.0

requires:
    Wheel Game ^2
    Casino API Library ^1
```

Those requirements do not later change in-place.

Publishing new requirements requires a new version.

---

# Historical Exact Composition

For a published Packagepack:

```text
Packagepack @ 1.0.0
```

Vapor should preserve:

```text
authored requirements

exact resolved versions

effective Engine

effective Game

support dependencies
```

so historical builds can be understood/reproduced.

---

# Implementation Pressure

The immediate implementation target is not to build every possible graph feature.

It is to migrate away from the fixed vertical-slice shape:

```text
Engine

Game

one Game Mod
```

toward:

```text
ResolvedContentGraph

EffectiveEngine {
    base,
    constituents[],
}

EffectiveGame {
    base,
    constituents[],
}
```

with arbitrary support dependencies.

---

# Next Runtime Proofs

The most useful next pressure comes from real small examples:

```text
one Engine Mod

one Game Mod requiring that Engine Mod

Libraries shared across layers

one Extension Mod

several Mods on one side

packs contributing constituents
```

These examples should pressure the generic model before more abstraction is added.

---

# Do Not Prematurely Define Integration ABI

The composition model becoming generic does not justify prematurely imposing:

```text
trait VaporContent {
    fn install(...)
}
```

or:

```text
every constituent is a Bevy Plugin
```

or any equivalent universal ABI.

First prove the rich semantic graph.

Then manually realize one rich composition.

Then discover the smallest integration contract that actually exists.

---

# Core Invariants

* Vapor has one semantic Content dependency graph.
* Graph nodes are exact Content Project versions.
* Content identity is canonical Project identity.
* Content kind is Project metadata/semantics, not an identity segment.
* Every dependency edge first represents an outbound requirement.
* Inbound Dependents are a derived reverse view.
* Inbound Dependents are scoped to a particular resolved graph.
* Dependency inclusion derives from resolved reachability.
* Operation Selection is unrelated to dependency inclusion.
* Some dependency edges additionally represent Runtime Foundation or Extension Target semantics.
* A Game has exactly one Engine Runtime Foundation.
* A Mod has one primary Extension Target under the current Content-kind model.
* Mods may have arbitrarily many additional ordinary dependencies.
* Higher-layer Content may require lower-layer constituents/capabilities.
* Registry availability alone never includes Content in a composition.
* Constituency derives from included behavioral Content and target chains.
* One complete Packagepack has exactly one Base Engine and exactly one Base Game.
* Effective Engine may have arbitrarily many compatible Engine-side constituents.
* Effective Game may have arbitrarily many compatible Game-side constituents.
* Extension Mods inherit side constituency through their Extension Target chain.
* Libraries/support dependencies do not automatically become behavioral constituents.
* Packs select/group composition Content but do not automatically become runtime behavioral constituents.
* Target chains must resolve to the selected compatible base Content.
* Base Engine/Game do not normally require Mods which target themselves because that forms a dependency cycle.
* One resolved graph may support many semantic views.
* Derived semantic views should not become independently authored sources of truth.
* Constituency does not define runtime integration API.
* Constituency does not inherently define runtime registration order.
* Vapor semantic graph is distinct from Cargo physical graph.
* Cargo realization is downstream of semantic composition.
* Published Packagepack exact resolutions are immutable historical provenance.
* Development resolutions may change without mutating published historical composition.
* The current fixed `Engine + Game + one Game Mod` realization is temporary architecture-proving machinery, not the target model.

---

# Open Questions

The following remain intentionally open and should be resolved through runtime implementation pressure:

* Exact Rust representation of resolved nodes and edges.
* Exact semantic-relationship enum/type names.
* Exact authored syntax distinguishing Runtime Foundation / Extension Target where kind inference eventually proves insufficient.
* Exact representation of packs inside the final resolved graph versus flattened derived views.
* Exact support for multiple incompatible major versions of one Content identity within one composition.
* Exact future Cargo-style dependency disambiguation model.
* Exact ordering data required by the eventual static realization contract.
* Exact generic static integration contract between Vapor composition and arbitrary Engine/Game ecosystems.
* Exact treatment of future behavioral Content kinds beyond Engine/Game/Mods.
* Exact compatibility metadata beyond target identity/version requirements.
* Exact development Run-composition mechanism for non-Packagepack Projects.
* Exact graph-inspection API exposed by Vapor Core.
