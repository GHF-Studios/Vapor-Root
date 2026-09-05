## App Model

* **Steam App**: *The singular Steam-distributed "Loo Cast" product. It provides the installation and product boundary through which the Vapor Installer, Vapor Launcher, default first-party composition, and wider Vapor Ecosystem/Toolchain are accessed.*
* **Steam App Instance**: *One concrete local installation of the Steam App. It has a Steam-managed root location, one selected Vapor App Composition, local Vapor state, installed/built Vapor Apps, and an installed Vapor Role/tooling state. The normal model assumes one Steam App Instance per Steam installation; users do not manually create additional instances.*
* **Packagepack**: *A Vapor Content artifact type representing one complete Vapor composition. It must resolve to exactly one effective Engine and exactly one effective Game, whether provided directly or through an Enginepack and/or Gamepack, together with all effective Mods and other required dependencies. A valid Packagepack is the complete composition artifact from which a Vapor App Composition is resolved and a runnable Vapor App is built.*
* **Vapor App Composition**: *The selected and resolved effective Content composition of a Steam App Instance, defined by exactly one Packagepack. It represents the complete exact resolved Content graph relevant to the application, including the effective Engine, Game, Mods, subordinate packs, Libraries, and other required dependencies.*
* **Vapor App**: *A built, deployable, and launchable realization of a Packagepack's Vapor App Composition for a particular supported target. Multiple Vapor Apps may exist locally at once, while one Vapor App Composition may be selected as the current/default composition of the Steam App Instance.*

---

## User, Role, and Authority Model

* **Player**: *The base installed Vapor Role. A Player consumes finished Vapor Apps: discovering, acquiring, selecting, and launching complete compositions. A Player does not compose or develop Vapor Content and does not require Git, Rust/Cargo, SteamCMD, or development tooling.*
* **Composer / Content User**: *An installed Vapor Role above Player. A Composer may discover and use existing Vapor Content and may create, modify, build, test, and publish Packagepacks, Enginepacks, Gamepacks, and Modpacks. A Composer may consume Engines, Games, Mods, Libraries, and other existing Content but may not author their implementation. Composer workflows require the source/build tooling needed by static Vapor composition.*
* **Content Developer**: *An installed Vapor Role above Composer. A Content Developer may additionally create and modify Engines, Games, Engine Mods, Game Mods, Extension Mods, Libraries, and their associated APIs using Vapor development workflows.*
* **Ecosystem Developer**: *The highest ordinary installed Vapor Role. An Ecosystem Developer may develop Vapor itself, including its applications, SDK/tooling surfaces, CLI, root framework, server infrastructure, distribution machinery, and related ecosystem source. This Role is locally attainable and does not itself grant permission to modify protected official Vapor resources.*
* **Vapor Role**: *The locally installed level of Vapor workflows and tooling available through a Steam App Instance. Roles form the progression Player ⊂ Composer ⊂ Content Developer ⊂ Ecosystem Developer. Role determines which kinds of work Vapor equips the local installation to perform; it does not itself grant external authority.*
* **Authentication**: *Establishment of the identity attempting an operation. Authentication answers who is acting; it does not by itself imply that the operation is permitted.*
* **Authorization / Authority**: *Permission for a particular authenticated identity to perform a particular protected operation against a particular target. Examples include pushing to official repositories, creating repositories in protected organizations, publishing into official namespaces, deploying official Steam branches or depots, and administering production infrastructure. Authorization is separate from installed Role.*
* **Root Authority**: *The ultimate administrative and ownership authority over the protected official Vapor ecosystem, namespaces, repositories, distribution targets, and infrastructure. Root Authority is an authority state rather than an installed Role above Ecosystem Developer.*

---

## Vapor Applications and Tooling

* **Vapor Installer**: *The application responsible for changing the fundamental Vapor Role/tooling installed in a Steam App Instance. It installs, detects, configures, upgrades, downgrades, repairs, and removes role-specific dependencies and tooling such as Git, SteamCMD, and the Rust/Cargo toolchain.*
* **Vapor Launcher**: *The primary Vapor application used after the required tooling is installed. It provides access to Vapor Apps, local/source Vapor Content, composition workflows, accounts, settings, development surfaces, diagnostics, and external ecosystem services, and launches selected Vapor Apps.*
* **Vapor SDK**: *The logical Vapor developer surface for creating, programming, configuring, building, testing, and inspecting Vapor Content and, where the installed Role permits it, Vapor ecosystem source. It is integrated with the Vapor Launcher and backed by the same Vapor Core. A dedicated `vapor-sdk` executable or CLI projection may expose that same logical surface without creating an independent semantic implementation.*
* **Vapor CLI**: *The command-line projection of Vapor Core operations. The universal `vapor` executable exposes the broad command surface, while application-specific executables such as `vapor-installer`, `vapor-launcher`, and `vapor-sdk` expose appropriate subsets/projections of the same underlying operations.*

---

## Vapor Content Model

* **Vapor Content**: *The common category for Vapor's semantic source, behavior, reusable-code, and composition artifacts.*
* **Packagepack**: *A Vapor Content artifact type representing one complete Vapor composition. It is the only pack type that defines a complete composition and can therefore be realized as a Vapor App.*
* **Enginepack**: *A declarative Vapor Content artifact type containing exactly one effective Engine together with compatible Engine-side Content. It is a reusable composition fragment and cannot independently produce a Vapor App.*
* **Gamepack**: *A declarative Vapor Content artifact type containing exactly one effective Game together with compatible Game-side Content. It is a reusable composition fragment and cannot independently produce a Vapor App.*
* **Modpack**: *A declarative Vapor Content artifact type grouping Mods and their dependencies into a reusable composition fragment. It cannot independently produce a Vapor App.*
* **Engine**: *A Vapor Content artifact type defining the foundational technical/runtime model of a composition. The effective Engine ultimately owns the executable/runtime integration model of the resulting Vapor App.*
* **Game**: *A Vapor Content artifact type defining game-specific behavior and content within an Engine-defined foundation. A Game does not inherently own the composition's main executable.*
* **Engine Mod**: *A Vapor Content artifact type that targets an Engine and extends it using extension semantics explicitly exposed by that Engine and/or its dependencies.*
* **Game Mod**: *A Vapor Content artifact type that targets a Game and extends it using extension semantics explicitly exposed by that Game and/or its dependencies.*
* **Extension Mod**: *A Vapor Content artifact type that targets another Engine Mod, Game Mod, or Extension Mod and extends it using extension semantics explicitly exposed by the targeted artifact and/or its dependencies.*
* **Library**: *A Vapor Content artifact type representing reusable Rust/library implementation without inherent Engine, Game, Mod, pack, or runnable-composition semantics. A Library may expose ordinary Rust APIs, traits, algorithms, data structures, schemas, ECS Components, Resources, Events, SystemSets, plugin APIs, or other explicitly authored extension contracts. Libraries participate in normal Vapor identity, versioning, dependency, source, resolution, and publication semantics and do not require publication through crates.io.*

A dependency edge between Vapor Content artifacts means that one artifact requires another.

It does not by itself define how one artifact programmatically extends another.

The programmatic meaning of a dependency is defined by the APIs, ECS vocabulary, registries, traits, schemas, or other extension contracts authored by the participating Content.

---

## Source, Distribution, and Registry Model

* **Git Source Model**: *Vapor Content source lives in Vapor-compatible Git repositories. Git is used not only for source-bearing Vapor Workspaces but also for Container Repos that organize those Workspaces. Source distribution and collaboration therefore belong to the Git/repository side of the ecosystem rather than Steam Workshop.*
* **Steam Workshop**: *The external distribution system used by Vapor for built, published complete compositions. Steam Workshop does not serve as the canonical source-code distribution mechanism for individual Vapor Content.*
* **Steam Workshop Item**: *A Steam Workshop publication/distribution container for a built published complete Vapor composition / Vapor App.*
* **Vapor Content Registry**: *The central semantic identity and linkage layer of the Vapor ecosystem. It associates human-readable Vapor IDs/namespaces with the relevant external resources and identities used by the ecosystem, including Git-backed source and Steam Workshop-backed built composition distribution. Its exact persistence and mapping schema remains an implementation/design concern.*
* **Server**: *An official Vapor server application hosting part of the central Vapor service infrastructure, such as the Vapor Content Registry. User-hosted Vapor registries are not currently part of the ecosystem model.*

---

## Source, Build, and Local-State Model

* **Content Library**: *The user-facing/local view over Vapor artifacts available to the Steam App Instance. Depending on installed Role and local state this may include installed Vapor Apps, locally available packs, Libraries, behavioral Content, source acquired through Git-backed workflows, and build outputs. Content Library is a local/product organizational concept and is distinct from the `Library` Vapor Content kind.*
* **Build**: *The process of resolving and compiling a complete Packagepack-defined Vapor App Composition into a target-specific Vapor App. Composition builds are logically complete static builds, while Cargo/Vapor caching and incremental compilation may avoid physically rebuilding unchanged work.*
* **Deploy / Install**: *The process of taking a built or externally acquired Vapor App and registering/placing it locally so that it is available for selection and launch.*

---

## Rust/Cargo Realization Model

* **Vapor Semantic Dependency Graph**: *The graph of Vapor identities, versions, bindings, and Content relationships produced by Vapor dependency resolution. Vapor is authoritative for the semantic meaning of these nodes and edges.*
* **Cargo Physical Dependency Graph**: *The Rust/Cargo package graph that physically realizes Rust-backed Vapor Content for compilation. Cargo is authoritative for the package/build graph it actually resolves and compiles.*
* **Cargo Reconciliation**: *The process by which Vapor compares a desired Vapor-resolved dependency relationship with the editable Cargo project and ensures that an appropriate physical Cargo dependency realizes that relationship without taking ownership of unrelated Cargo configuration.*
* **Cargo Metadata**: *Cargo-native project and resolution information, such as that exposed by `cargo metadata`, used by Vapor to inspect workspace/package structure and validate the physical Cargo graph. Cargo package IDs, paths, and similar metadata are Cargo-domain realization identifiers and do not replace Vapor semantic identity.*

For Rust-backed Vapor Content:

```text
Vapor semantic resolution
        ↓
exact Vapor Content graph
        ↓
Cargo/project inspection
        ↓
Cargo reconciliation
        ↓
Cargo physical resolution
        ↓
validated Rust package graph
```

Vapor determines which Vapor Content is required.

Cargo determines the concrete Rust package graph it will compile.

---

## Development Storage Model

* **Vapor Superworkspace**: *A local checkout container holding checked-out Vapor repositories. It is not itself a Git repository or primary source-bearing unit. Losing it primarily risks local unpushed/uncommitted development state rather than canonical remote source.*
* **Container Repo**: *A Vapor-managed top-level Git repository that groups related Source Repos / Vapor Workspaces as Git submodules. A Container Repo is itself Git-managed but is not used as a submodule of another Container Repo.*
* **Source Repo / Vapor Workspace**: *A Vapor-managed source-bearing Git repository contained by a Container Repo as a Git submodule. It contains one or more Vapor Projects and does not itself contain nested Git submodules.*
* **Vapor Project**: *A Rust/Cargo workspace contained inside a Source Repo / Vapor Workspace. It is not itself a Git repository.*
* **Vapor Root Workspace**: *The unique Vapor Workspace containing the client-side/root Vapor codebase and bootstrapping model of the Vapor ecosystem.*
* **Vapor Root Project**: *A Vapor Project inside the Vapor Root Workspace modeling part of the client-side/root Vapor ecosystem.*
* **Vapor Server Root Workspace**: *The unique Vapor Workspace containing the server-side root Vapor codebase.*
* **Vapor Server Root Project**: *A Vapor Project inside the Vapor Server Root Workspace modeling part of the server-side/root Vapor ecosystem.*
* **Vapor Content Workspace**: *A non-unique Vapor Workspace containing Vapor Content Projects, with first-party Content Workspaces serving as concrete examples.*
* **Vapor Content Project**: *A Vapor Project inside a Vapor Content Workspace that models a Packagepack, Enginepack, Gamepack, Modpack, Engine, Game, Engine Mod, Game Mod, Extension Mod, or Library.*
