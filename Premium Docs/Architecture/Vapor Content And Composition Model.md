> [!info]
> **Owns:** Content kinds, semantic dependencies, resolved closure, Runtime Foundation, Extension Targets, constituency, packs, Effective Engine/Game, Packagepack, and Vapor App Composition.
>
> **Uses:** Source Identity for Project identity and Publication for published versions.
>
> **Does not own:** physical Cargo realization, runtime extension APIs, Context/Selection, or publication transport.

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

> **Vapor resolves one authored requirement graph and derives additional semantic views over that graph.**

---

# Content Nodes and Kinds

A resolved Content node represents:

```text
Content Project identity
+
exact resolved version
```

Current kinds include:

```text
Packagepack
Enginepack
Gamepack
Modpack
Engine
Game
Engine Mod
Game Mod
Extension Mod
Library
```

Kind contributes to graph interpretation.

---

# Authored Requirement

A Vapor dependency is an authored semantic requirement:

```text
depender
→ dependency
```

Examples:

```text
Game → Engine
Game → Library
Game → Engine Mod
Game Mod → Game
Game Mod → Library
```

Every edge first means `requires`.

Some edges additionally carry higher-level semantic meaning.

---

# Dependency Binding

A depender may assign a stable local binding/name.

Example:

```text
binding:
    terminal_engine

requirement:
    GHF-Studios/.../Terminal-Engine @ ^1.2
```

Binding is local authored vocabulary, not canonical identity.

---

# Outbound Requirements / Inbound Dependents

Outbound Requirements are authored dependencies.

Inbound Dependents are the reverse view inside one resolved graph.

Registry-wide potential dependents must not be mistaken for composition participation.

---

# Resolved Dependency Closure

Resolving a root yields exact versions/edges.

The closure is the transitive set reachable from the root.

Only reachable Content participates.

Registry presence or target compatibility alone does not include Content.

---

# Inclusion Rule

> **Dependency inclusion comes from resolved reachability; constituency comes from semantic classification of included Content.**

This is a core invariant.

---

# Semantic Relationship Kinds

Every edge first means:

```text
requires
```

Some additionally mean:

```text
Runtime Foundation
Extension Target
```

These refine dependency meaning without replacing it.

---

# Runtime Foundation

A Game has exactly one Engine foundation.

```text
Game → Engine
```

means both:

```text
Game requires Engine
Game is authored against this Engine foundation
```

Vapor understands the relationship, not the exact Rust/runtime integration API.

---

# Extension Target

A Mod has one primary Extension Target appropriate to its kind:

```text
Engine Mod → Engine
Game Mod → Game
Extension Mod → another Mod
```

The target answers what behavioral Content is extended.

It does not define how extension works programmatically.

---

# Extension API Ownership

The target owns the actual extension vocabulary.

Possible mechanisms:

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
```

Vapor must not invent one universal Mod ABI merely because it understands target semantics.

---

# Additional Mod Dependencies

A Mod may have arbitrary additional requirements.

Example:

```text
Game Mod
├── targets Game
├── requires Engine Mod
├── requires Library A
└── requires Library B
```

Only one dependency is the primary Extension Target.

---

# No Artificial Layer-Direction Restriction

Valid topology may include:

```text
Game → Engine Mod → Engine
Game Mod → Engine Mod → Engine
```

Vapor validates semantic target compatibility; it does not impose aesthetic layer-direction rules.

---

# Constituency

A **Constituent** is included behavioral Content participating in an effective Engine-side or Game-side runtime composition.

Constituency is derived from:

```text
resolved closure
+
Content kind
+
extension-target chain
```

Do not require redundant authored constituent flags when derivable.

---

# Effective Engine / Game

Conceptually:

```text
Effective Engine
├── one Base Engine
└── included Engine-side constituents
```

and:

```text
Effective Game
├── one Base Game
└── included Game-side constituents
```

Extension Mod chains inherit constituency through their target chain.

---

# Target Compatibility

An included constituent must ultimately target the actual selected base.

```text
Engine Mod → Engine X
```

cannot silently become a constituent of Base Engine Y.

Validation occurs against exact resolved identities/versions.

---

# No Registry-Wide Auto-Inclusion

This must never happen:

```text
Registry contains 900 Mods targeting Engine X
→ Effective Engine includes all 900
```

Reachability determines inclusion.

Target semantics classify included Content.

---

# Support Dependencies

Not every included dependency is behavioral constituency.

A **Library** may participate physically in the final build while remaining a support dependency.

---

# Packs

Packs participate in the same requirement graph.

There is no separate "pack graph".

Examples:

```text
Enginepack
Gamepack
Modpack
```

may require Content which enters the same resolved closure.

---

# Packagepack

A **Packagepack** is the complete authored composition root.

It must derive:

```text
exactly one Effective Engine
exactly one Effective Game
all required included Content
```

It is not merely a precursor to another authored "finished composition" object.

---

# Vapor App Composition

Resolving/validating a Packagepack produces:

```text
Packagepack
→ exact resolved graph
→ semantic classification
→ Effective Engine
→ Effective Game
→ support dependencies
```

This exact semantic state is the **Vapor App Composition**.

It is not yet necessarily a built artifact.

---

# Definition vs Runtime Instance

A resolved node identifies a definition/version participating in composition.

It does not define runtime instance multiplicity.

---

# Cargo Boundary

```text
Vapor semantic Content graph
≠
Cargo physical package/build graph
```

They interact.

Physical realization must not retroactively define Content semantics.
