# Vapor Glossary

> [!info]
> Fast terminology reference.
>
> Definitions are intentionally concise. Focused model documents remain normative for edge cases, lifecycle, and operational rules.

---

## App Client Runtime
Client-side semantic role of a running Vapor App.  
See `Architecture/Vapor App Runtime Model.md`.

## App Server Runtime
Server-side semantic role of a running Vapor App.  
See `Architecture/Vapor App Runtime Model.md`.

## Authored State
State created or deliberately edited as user/project intent and therefore requiring preservation.  
See `Architecture/Vapor Installation And Local State Model.md`.

## Authentication
Establishing who is attempting an operation.  
See `Architecture/Vapor Role, Authentication And Authorization Model.md`.

## Authority
First globally meaningful segment of canonical Vapor source identity.  
See `Architecture/Vapor Source Identity And Topology Model.md`.

## Authorization
Permission for an authenticated identity to perform a protected operation against a target.  
See `Architecture/Vapor Role, Authentication And Authorization Model.md`.

## Base Engine
The one base Engine from which an Effective Engine is derived.  
See `Architecture/Vapor Content And Composition Model.md`.

## Base Game
The one base Game from which an Effective Game is derived.  
See `Architecture/Vapor Content And Composition Model.md`.

## Ban
Strong administrative publication/use restriction which preserves historical identity/provenance.  
See `Development/Vapor Publication Model.md`.

## Composer
Installed Role for authoring composition-oriented Content such as Packagepacks and packs.  
See `Architecture/Vapor Role, Authentication And Authorization Model.md`.

## Condition
One fact within/across operational state dimensions.  
See `Architecture/Vapor Operations And State Model.md`.

## Content Developer
Installed Role for authoring behavioral/reusable Content in addition to Composer capabilities.  
See `Architecture/Vapor Role, Authentication And Authorization Model.md`.

## Content ID / Vapor ID
For ordinary Vapor Content, the canonical Project identity of the Content Project.  
See `Architecture/Vapor Source Identity And Topology Model.md`.

## Context
Lightweight semantic resolution prefix used to resolve relative paths/selectors. Context does not define operation scope.  
See `Architecture/Vapor Resolution Context And Selection Model.md`.

## Derived State
Vapor-owned state intended to be safely reconstructable from authored/modelled truth.  
See `Architecture/Vapor Installation And Local State Model.md`.

## Dependency Binding
Stable local authored name used by one depender to refer to a Vapor dependency. Not canonical identity.  
See `Architecture/Vapor Content And Composition Model.md`.

## Effective Engine
One Base Engine plus all included compatible Engine-side behavioral constituents.  
See `Architecture/Vapor Content And Composition Model.md`.

## Effective Game
One Base Game plus all included compatible Game-side behavioral constituents.  
See `Architecture/Vapor Content And Composition Model.md`.

## Ecosystem Developer
Highest ordinary installed Role; enables development of Vapor itself but does not itself grant Root Authority.  
See `Architecture/Vapor Role, Authentication And Authorization Model.md`.

## Engine Mod
A Mod whose primary Extension Target is an Engine.  
See `Architecture/Vapor Content And Composition Model.md`.

## Extension Mod
A Mod whose primary Extension Target is another Mod.  
See `Architecture/Vapor Content And Composition Model.md`.

## Extension Target
Behavioral Content a Mod conceptually extends. The target owns the actual runtime extension API.  
See `Architecture/Vapor Content And Composition Model.md`.

## Game Mod
A Mod whose primary Extension Target is a Game.  
See `Architecture/Vapor Content And Composition Model.md`.

## Library
Reusable Vapor Content providing implementation/API/support without inherently being Engine/Game/Mod/pack runtime constituency.  
See `Architecture/Vapor Content And Composition Model.md`.

## Natural Operation Scope
Complete semantic scope implied by an Operation Subject when no explicit Selection is supplied.  
See `Architecture/Vapor Resolution Context And Selection Model.md`.

## Operation
Modeled Vapor action against a semantic subject, optionally with Selection and variant/parameters.  
See `Architecture/Vapor Operations And State Model.md`.

## Operation Recipe
Lightweight authored configuration describing how a modeled operation is realized.  
See `Architecture/Vapor Operations And State Model.md`.

## Operation Selection
Explicit per-operation intent narrowing/specifying topology beneath the Operation Subject.  
See `Architecture/Vapor Resolution Context And Selection Model.md`.

## Operation Subject
Modeled object/facility an operation is fundamentally about.  
See `Architecture/Vapor Resolution Context And Selection Model.md`.

## Operation Variant
Semantically distinct form of an operation, such as local vs VPS deployment.  
See `Architecture/Vapor Operations And State Model.md`.

## Operational Situation
Relevant combination of independent state dimensions at a given moment.  
See `Architecture/Vapor Operations And State Model.md`.

## Packagepack
Complete authored composition Content Project which resolves to exactly one Effective Engine, one Effective Game, and all required included Content.  
See `Architecture/Vapor Content And Composition Model.md`.

## Player
Base installed Role for consuming finished Vapor Apps.  
See `Architecture/Vapor Role, Authentication And Authorization Model.md`.

## Project / Vapor Project
Modeled development unit inside one Source Repo / Vapor Workspace. A Project is not itself a Git repository.  
See `Architecture/Vapor Source Identity And Topology Model.md`.

## Project Kind
Semantic kind/property of an existing Project, such as Game, Engine, Library, or Packagepack. Not an additional identity-path segment.  
See `Architecture/Vapor Source Identity And Topology Model.md`.

## Readiness Predicate
Derived operational question such as Buildable?, Locally Runnable?, or Repairable?.  
See `Architecture/Vapor Operations And State Model.md`.

## Resolved Dependency Closure
Transitive set of exact Content versions reachable from a resolved Content root through requirements.  
See `Architecture/Vapor Content And Composition Model.md`.

## Root Authority
Ultimate trusted administrative authority over protected official Vapor resources. It is authority, not an installed Role.  
See `Architecture/Vapor Role, Authentication And Authorization Model.md`.

## Runtime Foundation
Semantic dependency relationship from a Game to its one base Engine foundation.  
See `Architecture/Vapor Content And Composition Model.md`.

## Selection
Explicit per-operation scope intent. Not persistent Context, selected/default App, or transient GUI row selection.  
See `Architecture/Vapor Resolution Context And Selection Model.md`.

## Source Repo
Primary source-bearing Git repository beneath a Source Repo Container. Normatively one Source Repo equals one Vapor Workspace.  
See `Architecture/Vapor Source Identity And Topology Model.md`.

## Source Repo Container
Registered top-level Vapor Git repository grouping related Source Repos.  
See `Architecture/Vapor Source Identity And Topology Model.md`.

## State Dimension
One independently meaningful aspect of Vapor operational state.  
See `Architecture/Vapor Operations And State Model.md`.

## Steam App
Outer Steam-distributed Loo Cast product through which the Vapor Client/default App/Installer are delivered.  
See `Architecture/Vapor Product Topology Model.md`.

## Steam App Instance
One concrete local installation of the Steam App.  
See `Architecture/Vapor Installation And Local State Model.md`.

## Superworkspace / Vapor Superworkspace
Single canonical local development source root. It has no Vapor identity.  
See `Architecture/Vapor Source Identity And Topology Model.md`.

## Transition
Meaningful operational state change caused by an Operation.  
See `Architecture/Vapor Operations And State Model.md`.

## Vapor App
Built/deployable/runnable realization of one complete Vapor App Composition for a supported target.  
See `Development/Vapor Distribution Model.md`.

## Vapor App Composition
Exact resolved complete Content graph represented by one Packagepack under a resolution/version context.  
See `Architecture/Vapor Content And Composition Model.md`.

## Vapor App Runtime
Executing realization of a Vapor App Composition.  
See `Architecture/Vapor App Runtime Model.md`.

## Vapor Client
Complete user-side Vapor product/environment.  
See `Architecture/Vapor Product Topology Model.md`.

## Vapor Core
Shared semantic/orchestration implementation layer used by Vapor frontends/applications.  
See `Architecture/Vapor Product Topology Model.md`.

## Vapor Installer
Application/process boundary responsible for changing what the local Vapor environment is equipped to do.  
See `Architecture/Vapor Product Topology Model.md`.

## Vapor Launcher
Primary ordinary graphical Vapor surface.  
See `Architecture/Vapor Product Topology Model.md`.

## Vapor Path
Slash-separated structural path through Authority → Container → Source Repo → Project.  
See `Architecture/Vapor Source Identity And Topology Model.md`.

## Vapor Platform
Online Vapor services providing ecosystem-wide capabilities such as identity, Registry, authorization, and discovery.  
See `Architecture/Vapor Product Topology Model.md`.

## Vapor Platform Server
First-party server-side facility realizing Vapor Platform services.  
See `Architecture/Vapor Product Topology Model.md`.

## Vapor Role
Locally installed capability/tooling level: Player → Composer → Content Developer → Ecosystem Developer.  
See `Architecture/Vapor Role, Authentication And Authorization Model.md`.

## Vapor SDK / Development Mode
Integrated first-party graphical development projection of the Vapor Client/Core.  
See `Experience/Vapor SDK Experience Model.md`.

## Vapor User Data
Mutable Vapor-owned local user/environment state distinct from Steam depot state and authored Superworkspace source.  
See `Architecture/Vapor Installation And Local State Model.md`.

## Vapor Workspace
Development view of a Source Repo. Normatively one Source Repo = one Vapor Workspace.  
See `Architecture/Vapor Source Identity And Topology Model.md`.

## Yank
Publication state removing a version from ordinary future selection policy without rewriting historical identity.  
See `Development/Vapor Publication Model.md`.
