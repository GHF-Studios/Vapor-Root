# Vapor Premium Docs

> [!info]
> This directory is Vapor's architecture, development, experience, migration, process, and reference corpus.
>
> It is organized by **semantic ownership**, not by the order in which ideas happened to be discovered.

---

# Core Documentation Rule

> **Every concept has one normative owner.**

Other documents may summarize an imported concept only as much as their own argument requires. They should not reproduce another owner's full edge cases, lifecycle, diagnostics, and invariants.

Focused normative documents should make three relationships obvious:

```text
OWNS
    concepts whose normative definition lives here

USES
    concepts imported from another owner

DOES NOT OWN
    nearby concepts which belong elsewhere
```

The exact headings may vary. The ownership must not.

---

# Corpus Structure

## Architecture

`Architecture/` owns Vapor's stable semantic model:

```text
Product Topology
Role / Authentication / Authorization
Installation / Local State
Source Identity / Topology
Resolution Context / Selection
Operations / State
Content / Composition
App Runtime
```

These documents should remain stable across ordinary UI, repository-layout, provider, and implementation changes.

## Development

`Development/` owns development and externalization mechanics:

```text
Source Development
Rust / Cargo / Toolchain
Publication
Distribution
```

## Experience

`Experience/` projects the normative models into human-facing surfaces:

```text
Product Experience
CLI
SDK / Development Mode
```

Experience docs do not redefine architecture.

## Migration

`Migration/` owns temporary current→target transition truth.

Historical provider/repository names such as:

```text
Vapor-Root
Vapor-Server-Root
```

belong here while migration is incomplete. Stable architecture should prefer semantic facilities such as:

```text
Vapor Client
Vapor Platform Server
Examples
```

## Process

`Process/` contains historical rewrite rationale and living implementation progress.

## Reference

`Reference/` is fast lookup material. The glossary summarizes definitions and points to normative owners; it is not a second architecture corpus.

---

# Architecture Ownership Map

```text
Product Topology
    Vapor Client / Core / Launcher / SDK / CLI / Installer
    Vapor Platform / Platform Server
    trusted first-party facilities

Role, Authentication And Authorization
    Player → Composer → Content Developer → Ecosystem Developer
    authentication
    authorization
    Root Authority

Installation And Local State
    Steam App Instance
    Vapor User Data
    Superworkspace lifetime relationship
    authored / derived / replaceable state
    reinstall / downgrade / repair / recovery

Source Identity And Topology
    Authority
    Source Repo Container
    Source Repo / Vapor Workspace
    Project
    Vapor Paths
    canonical identity
    Superworkspace identity semantics

Resolution Context And Selection
    Context
    Operation Subject interaction
    Selection
    selectors
    ambiguity
    CWD separation
    frontend/session distinction

Operations And State
    Operational Situation
    State Dimensions
    Operations / Variants / Recipes
    Transitions / Invariants
    Diagnose / Repair

Content And Composition
    Content kinds
    dependencies
    resolved closure
    Runtime Foundation
    Extension Target
    constituency
    packs / Packagepack
    Vapor App Composition

App Runtime
    Vapor App Runtime
    App Client Runtime
    App Server Runtime
    platform communication
    app-session communication
    Engine/Game-owned runtime semantics
```

---

# Suggested Reading Paths

## New to Vapor

1. `Experience/Vapor Product Experience Model.md`
2. `Reference/Vapor Glossary.md`
3. `Architecture/Vapor Product Topology Model.md`
4. `Architecture/Vapor Source Identity And Topology Model.md`
5. `Architecture/Vapor Content And Composition Model.md`

## Developing Vapor

1. `Architecture/Vapor Source Identity And Topology Model.md`
2. `Architecture/Vapor Operations And State Model.md`
3. `Development/Vapor Source Development Model.md`
4. `Development/Vapor Rust, Cargo And Toolchain Model.md`
5. `Experience/Vapor CLI Model.md`
6. `Experience/Vapor SDK Experience Model.md`
7. `Process/Vapor Rewrite Progress And Roadmap.md`

## Repository migration

1. `Architecture/Vapor Source Identity And Topology Model.md`
2. `Architecture/Vapor Product Topology Model.md`
3. `Migration/Vapor Repository Migration Plan.md`
4. `Migration/Vapor Repository Migration Execution Ledger.md`

---

# Historical Documents

`Process/Vapor Rewrite Bootstrap.md` is intentionally historical.

Current implementation state and sequencing belong to:

`Process/Vapor Rewrite Progress And Roadmap.md`.

Historical terminology must not override newer focused owners.

---

# Stable Semantics vs Temporary Reality

Stable documents describe what Vapor **means**.

Migration documents describe how today's concrete repositories, provider names, paths, persisted state, scripts, Registry records, and deployment assumptions reach that stable model.

This separation is deliberate.

---

# Final Rule

> **If two documents both need the complete definition of the same concept, the ownership boundary is probably wrong.**
