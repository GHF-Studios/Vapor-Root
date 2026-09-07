> [!info]
> This document defines the semantic relationship between Vapor dependencies, extension targets, selected composition closure, effective Engine/Game constituency, and final Vapor App Composition.
>
> It refines the earlier model in which every resolved dependency edge was treated structurally alike and effective Engine/Game identity was represented primarily by one base Engine and one base Game.
>
> Physical Rust/Cargo realization is downstream of this model and must not define it retroactively.

---

# Core Thesis

Vapor has one semantic dependency graph.

It does not maintain separate unrelated graphs for:

```text
ordinary dependencies
Mods
composition membership
effective Engine/Game state
```

Instead:

> **Vapor resolves one authored requirement graph and derives additional semantic views over that graph.**

The main views are:

```text
Outbound Requirements
Inbound Dependents
Target Relationships
Selected Dependency Closure
Effective Constituency
```

These views describe different meanings of the same resolved Content topology.

---

# Dependency

A **Vapor Dependency** is an authored requirement from one Vapor Content artifact to another.

Conceptually:

```text
A
↓ requires
B
```

means:

> A requires a compatible selected version of B to participate in the resolved graph.

Every dependency therefore has the basic forward direction:

```text
depender
→ dependency
```

For example:

```text
Game
→ Engine

Game
→ Library

Game
→ Engine Mod
```

All three are requirements.

They do not necessarily have the same higher-level semantic role.

---

# Outbound Dependencies

The dependencies authored by one Content artifact are its **Outbound Dependencies**.

For example:

```text
Casino Game
├── Terminal Engine
├── Weighted Roll Engine Mod
├── Wheel Rules Library
└── Economy Library
```

From the Game's perspective these are requirements.

Outbound dependency information answers:

> What must be selected in order for this Content to exist coherently?

---

# Inbound Dependents

An **Inbound Dependent** is the reverse view of a resolved dependency edge.

If:

```text
A → B
```

then:

```text
A
```

is an inbound dependent of:

```text
B
```

within that resolved graph.

Inbound relationships are derived.

They are not separately authored.

This distinction is important because a registry may contain thousands of artifacts which depend on the same Engine.

Those artifacts do not become part of a composition merely because they exist.

Only dependencies reachable inside the selected resolved graph matter.

Therefore:

> **Inbound dependency views are scoped to a resolved selected graph, not the entire Vapor ecosystem.**

---

# Selected Dependency Closure

Resolving a root artifact recursively produces its **Selected Dependency Closure**.

For a Packagepack this closure represents all exact Content required by the composition.

Conceptually:

```text
Packagepack
├── Game Mod
│   └── Game
│       ├── Engine Mod
│       │   └── Engine
│       └── Library
└── another Library
```

Everything reachable through selected requirements belongs to the resolved closure.

Content which merely exists in the Registry but is not reachable does not participate.

---

# Dependency Meaning Is Layered

Every resolved edge first means:

```text
requires
```

Some edges additionally carry higher-level semantic meaning.

Examples include:

```text
Game → Engine
    runtime/foundation relationship

Engine Mod → Engine
    extension-target relationship

Game Mod → Game
    extension-target relationship

Extension Mod → Mod
    extension-target relationship
```

These meanings do not replace dependency semantics.

They refine them.

An extension target is still a required dependency.

---

# Runtime Foundation

A Game requires exactly one Engine foundation.

Conceptually:

```text
Game
→ Engine
```

means both:

```text
Game requires Engine
```

and:

```text
Game is authored against this Engine foundation
```

The exact code/API semantics remain defined by the participating Engine and Game.

Vapor only understands the structural semantic relationship.

---

# Extension Target

A Mod has one primary **Extension Target** appropriate to its Content kind.

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

> Which behavioral Content does this Mod conceptually extend?

It does not answer:

> How does extension work?

That remains defined by the target's authored APIs, ECS vocabulary, traits, callbacks, registries, schemas, or other extension contracts.

---

# Additional Mod Dependencies

A Mod may have dependencies in addition to its extension target.

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
├── requires Library
└── requires another compatible capability
```

These additional requirements are ordinary dependency edges unless another semantic relationship applies.

There is no hardcoded limit such as:

```text
one Mod dependency
```

or:

```text
one Engine Mod per Game
```

A Content artifact may require seventeen Engine Mods if that is its actual semantic requirement.

---

# Constituency

A **Constituent** is selected behavioral Content which participates in forming an effective higher-level runtime composition.

Constituency is normally derived from:

```text
selected dependency closure
+
Content kind
+
target relationships
```

It should not normally require a redundant authored declaration saying:

```text
this is a constituent
```

For example:

```text
Game
├── Engine
├── Engine Mod A
└── Engine Mod B
```

with:

```text
Engine Mod A → Engine
Engine Mod B → Engine
```

means that the selected effective Engine side contains:

```text
Engine
Engine Mod A
Engine Mod B
```

The Game's requirement on the Engine Mods selects them.

Their target relationships classify them as Engine-side constituents.

---

# Base Content and Constituents

The model distinguishes a base artifact from the effective behavioral constituency around it.

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

This avoids forcing the phrase **effective Engine** to mean only one raw Engine artifact.

Where precision matters, use:

```text
Base Engine
Effective Engine

Base Game
Effective Game
```

The Base Engine and Base Game retain exactly-one cardinality for one complete Packagepack composition.

Their effective forms may contain arbitrarily many compatible constituents.

---

# Effective Engine

The **Effective Engine** consists conceptually of:

```text
exactly one Base Engine
+
all selected Engine Mods targeting that Engine
+
selected Extension Mods whose target chain terminates on those Engine Mods
```

For example:

```text
Effective Engine
├── Terminal Engine
├── Diagnostics Engine Mod
├── Weighted Roll Engine Mod
└── Weighted Roll Debug Extension Mod
```

Libraries required by those artifacts belong to the resolved dependency closure but are not automatically called Engine constituents.

They are support dependencies.

---

# Effective Game

The **Effective Game** consists conceptually of:

```text
exactly one Base Game
+
all selected Game Mods targeting that Game
+
selected Extension Mods whose target chain terminates on those Game Mods
```

For example:

```text
Effective Game
├── Wheel Game
├── Casino Game Mod
└── Roguelike Casino Extension Mod
```

Again, required Libraries remain semantic dependencies without automatically becoming behavioral constituents.

---

# Extension-Mod Constituency

An Extension Mod inherits its Engine-side or Game-side constituency through its target chain.

For example:

```text
Extension C
→ Extension B
→ Engine Mod A
→ Engine
```

places:

```text
Engine Mod A
Extension B
Extension C
```

within the effective Engine-side constituency.

Likewise:

```text
Extension C
→ Game Mod B
→ Game
```

places the selected extension chain on the Game side.

---

# Selection Is Not Global Discovery

The following must never happen:

```text
Registry contains 900 Engine Mods targeting Engine X
↓
Effective Engine X contains 900 Mods
```

Target compatibility does not imply selection.

A Mod participates only when it is present in the selected dependency closure.

Therefore the governing rule is:

> **Selection comes from dependency reachability; constituency comes from semantic classification of selected Content.**

---

# Upward Selection

The dependency graph naturally permits higher-level Content to require constituents of lower-level runtime layers.

For example:

```text
Game
→ Engine Mod
→ Engine
```

is valid.

The Game can therefore require a particular augmented Engine environment.

Likewise:

```text
Game Mod
→ Engine Mod
→ Engine
```

can require Engine-level capabilities needed by that Game Mod.

Packs may also select Mods:

```text
Enginepack
→ Engine Mod
→ Engine

Gamepack
→ Game Mod
→ Game

Packagepack
→ Engine Mod
→ Engine

Packagepack
→ Game Mod
→ Game
```

This is a natural consequence of the ordinary requirement graph rather than a separate Mod-installation mechanism.

---

# Why Base Content Does Not Normally Require Its Own Mods

A target generally cannot require a Mod which itself targets that same target without creating a dependency cycle.

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

This is useful rather than accidental.

A base Engine or Game owns its intrinsic behavior directly.

External augmentation is selected from a higher composition layer.

Examples include:

```text
Game selecting Engine Mods
Enginepack selecting Engine Mods
Gamepack selecting Game Mods
Packagepack selecting Mods
other Mods selecting required lower-layer capabilities
```

This keeps extension and base implementation conceptually distinct.

---

# Packs as Composition Selectors

Packs remain declarative composition Content.

They may select constituents without themselves becoming runtime behavioral constituents.

For example:

```text
Enginepack
├── Engine
├── Engine Mod A
└── Engine Mod B
```

defines a reusable effective Engine fragment.

Likewise:

```text
Gamepack
├── Game
├── Game Mod A
└── Game Mod B
```

defines a reusable Game-side fragment.

A Packagepack combines the complete selected closure into one runnable composition.

---

# Modpack

A Modpack is a reusable selector/grouping artifact for Mods and their dependencies.

Its Mods retain their actual target relationships.

A Modpack does not create a new universal target.

For example:

```text
Modpack
├── Engine Mod
│   └── Engine
└── Game Mod
    └── Game
```

may contribute constituents to both effective sides.

---

# Support Dependencies

Not every dependency is a behavioral constituent.

Examples include:

```text
Library
schema Content
future reusable support artifacts
```

A support dependency is part of the selected semantic closure and may be physically linked into the final App.

That does not automatically make it an Engine Mod or Game Mod constituent.

This distinction prevents:

```text
everything reachable
=
effective Engine/Game constituent
```

which would erase useful semantic structure.

---

# Compatibility

A selected constituent must ultimately target the effective base appropriate to its side.

For example, an Engine Mod targeting Engine X cannot silently participate in a Packagepack whose Base Engine is Engine Y.

Likewise a Game Mod targeting Game X cannot silently become part of an effective Game based on Game Y.

Target-chain validation should reject incompatible compositions.

---

# One Graph, Multiple Views

The same resolved graph may be inspected as:

```text
forward requirements

reverse dependents

target relationships

effective Engine constituency

effective Game constituency

support dependency closure
```

These are projections over one semantic graph.

They should not be maintained as unrelated sources of truth.

---

# Current Manifest Direction

The current authored manifest syntax:

```toml
[dependencies.some-binding]
id = "..."
version = "..."
```

is sufficient for the immediate architecture-proving slice.

Do not add an authored field such as:

```toml
role = "constituent"
```

merely to encode information Vapor can already derive.

The current special relationships are structurally inferable from Content kinds:

```text
Game → Engine
Engine Mod → Engine
Game Mod → Game
Extension Mod → Mod
```

If future Content semantics produce genuine ambiguity, an explicit dependency-role field may be introduced then.

That should arise from real pressure rather than anticipation.

---

# Resolved Dependency Model

The implementation should eventually represent more than:

```text
binding
→ ContentVersionId
```

Conceptually, one resolved edge may expose:

```text
binding
depender
dependency
semantic relationship
```

with relationships approximately equivalent to:

```text
Ordinary Requirement
Runtime Foundation
Extension Target
```

The exact Rust names are implementation details and should be chosen during the clean implementation pass.

Every relationship remains a dependency requirement.

---

# Resolved Composition Model

The current composition model should evolve conceptually toward:

```text
ResolvedComposition
├── ResolvedContentGraph
├── EffectiveEngine
│   ├── base
│   └── constituents
└── EffectiveGame
    ├── base
    └── constituents
```

rather than:

```text
one Engine ID
one Game ID
some unstructured Mods elsewhere in nodes
```

For example, conceptually:

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

The exact storage type and ownership arrangement remain implementation concerns.

---

# Constituency Does Not Define Integration API

Knowing that a Mod is an Engine constituent does not tell Vapor how to install it into the Engine.

Knowing that a Mod is a Game constituent does not tell Vapor how to install it into the Game.

Therefore:

> **Composition semantics and runtime integration semantics remain separate.**

Vapor determines:

```text
what is selected
what targets what
what constitutes the effective Engine/Game
what must be physically available
```

The Engine/Game ecosystem determines:

```text
how those participants integrate
what APIs they implement or call
what registration means
what runtime ordering is required
```

---

# Constituency Does Not Automatically Define Integration Order

Constituent membership and runtime registration order are separate concerns.

Dependency structure provides ordering constraints where genuine requirements exist.

Target-defined extension APIs may impose additional ordering semantics.

Vapor must not invent a universal order merely from Content kind.

A later static-realization contract should preserve all required dependency relationships while allowing the effective Engine/Game architecture to define actual runtime integration behavior.

---

# Physical Realization

Physical Cargo realization is downstream of resolved composition.

The intended direction is:

```text
authored Vapor manifests
        ↓
semantic dependency resolution
        ↓
selected dependency closure
        ↓
target validation
        ↓
effective constituency derivation
        ↓
resolved Vapor App Composition
        ↓
Rust/Cargo physical realization
        ↓
Engine-defined static runtime integration
```

Cargo does not decide constituency.

Generated Rust glue does not decide constituency.

They realize a composition Vapor has already resolved.

---

# Invariants

* Vapor has one semantic dependency graph.
* Every dependency edge represents an outbound requirement.
* Inbound dependents are a derived reverse view.
* Inbound relationships are scoped to the selected resolved graph, not the entire Registry.
* Some dependency edges additionally represent runtime-foundation or extension-target relationships.
* Extension-target semantics do not define the target's programmatic extension API.
* Selection derives from dependency reachability.
* Constituency derives from selected behavioral Content and target relationships.
* Registry availability alone never selects a Mod.
* A Packagepack has exactly one Base Engine and exactly one Base Game.
* An Effective Engine may contain arbitrarily many compatible selected Engine-side constituents.
* An Effective Game may contain arbitrarily many compatible selected Game-side constituents.
* Extension Mods inherit side constituency through their target chain.
* Libraries and similar support dependencies are not automatically behavioral constituents.
* Higher-level Content may require lower-layer Mods and thereby select them into the composition.
* Engine/Game base artifacts do not normally depend on Mods targeting themselves because that produces a dependency cycle.
* Constituency and runtime integration mechanism are distinct.
* Constituency and runtime integration order are distinct.
* Cargo physical realization is downstream of semantic composition.
* The current fixed `Engine + Game + one Game Mod` realization is temporary vertical-slice machinery, not the production model.
