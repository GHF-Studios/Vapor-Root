> [!info]
> **Owns:** Vapor App Runtime, App Client Runtime, App Server Runtime, platform communication, app-session communication, and the boundary between Vapor composition and Engine/Game-defined runtime behavior.
>
> **Uses:** Product Topology and Content/Composition.
>
> **Does not own:** universal Engine/Game APIs, source topology, build realization, or network protocol implementation.

---

# Core Thesis

A Vapor App is statically composed by Vapor semantics, but the participating Engine/Game/Mods define the actual runtime architecture.

Vapor must know enough to realize/host the composition without stealing runtime semantics from the authored ecosystem.

---

# Vapor App Runtime

A **Vapor App Runtime** is an executing realization of a Vapor App Composition.

Its behavior derives from:

```text
Effective Engine
Effective Game
effective Mods/constituents
supporting Libraries
authored contracts
```

Vapor ensures the intended definitions participate.

The authored ecosystem defines how they interact.

---

# No Universal Vapor Runtime ABI

Vapor must not conclude that every:

```text
Engine exposes one universal trait
Game is one universal plugin type
Mod is one universal installer ABI
```

merely because Vapor understands Engine/Game/Mod semantics.

Different Engines may expose different extension vocabularies.

---

# Engine-Owned Runtime Semantics

The Engine conceptually owns application runtime/lifecycle semantics such as:

```text
application loop
world/runtime host
timing
input
render/presentation
network integration points
extension surfaces
```

Exact responsibilities vary by Engine.

Vapor composes; it does not replace the Engine.

---

# Game / Mod Semantics

The Game consumes the Engine foundation's extension/runtime vocabulary and may expose its own extension surfaces for Game Mods.

A Mod's target relationship is semantic.

The target owns the actual extension API.

---

# Bevy Is Not a Universal Vapor ABI

An Engine may naturally use Bevy and expose:

```text
App
Plugin
System
Resource
Event
Component
SystemSet
```

That is valid Engine-specific architecture.

It does not require every Vapor Engine/Project to use Bevy.

---

# Runtime Roles

A running Vapor App may operate in semantic roles:

```text
App Client Runtime
App Server Runtime
```

These roles do not necessarily map one-to-one to process, machine, container, or executable.

---

# App Client Runtime

Possible responsibilities include:

```text
presentation
input
client-side simulation
prediction/interpolation
local world state
communication with App Server Runtime
```

Exact behavior is Engine/Game-defined.

---

# App Server Runtime

Possible responsibilities include:

```text
authoritative simulation
session state
gameplay authority
persistence
connected-client management
match-specific services
```

Exact behavior is Engine/Game-defined.

---

# Vapor App Server

A future **Vapor App Server** product/environment may host one or more App Server Runtimes.

It is distinct from the Vapor Platform Server.

Do not create infrastructure merely to make topology symmetrical before implementation pressure requires it.

---

# Platform Communication

Client/App Server environments may communicate with the Vapor Platform for:

```text
identity
Registry/discovery
authorization
publication metadata
diagnostics
future matchmaking/service discovery
```

Platform communication is not automatically gameplay/session traffic.

---

# App-Session Communication

App Client Runtime and App Server Runtime may exchange App-defined session traffic.

The protocol/runtime semantics are Engine/Game-defined unless Vapor later owns an explicitly shared layer.

---

# Platform Is Not the Game Server

```text
Platform:
    ecosystem-wide services

App Server Runtime:
    one App/session's runtime authority
```

They may communicate without being the same system.

---

# Static Composition vs Dynamic Runtime

Static source/build composition does not imply static runtime state.

A statically included Engine/Game/USF system may dynamically create entities, sessions, capabilities, resources, or runtime objects.

---

# Runtime Instance Multiplicity

The resolved graph identifies definitions.

Runtime multiplicity belongs to runtime semantics.

One resolved Game or Library definition may support many runtime instances.

---

# Realization Boundary

Conceptually:

```text
Packagepack
→ Vapor App Composition
→ Rust/Cargo realization
→ built Vapor App
→ Vapor App Runtime
```

This document begins at execution/runtime meaning.
