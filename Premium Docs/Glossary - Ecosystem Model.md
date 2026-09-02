# Glossary - Ecosystem Model

## App Model
- **Steam App**: *The singular Steam-distributed "Loo Cast" product. It provides the installation and product boundary through which the Vapor Installer, Vapor Launcher, default first-party composition, and wider Vapor Ecosystem/Toolchain are accessed.*
- **Steam App Instance**: *One concrete local installation of the Steam App. It has a Steam-managed root location, one selected Vapor App Composition, local Vapor state, installed/built Vapor Apps, and an installed capability level. The normal model assumes one Steam App Instance per Steam installation; users do not manually create additional instances.*
- **Packagepack**: *A Vapor Content artifact type representing one complete Vapor composition. It must resolve to exactly one effective Engine and exactly one effective Game, whether provided directly or through an Enginepack and/or Gamepack, together with all effective Mods. A valid Packagepack is the complete composition artifact from which a Vapor App Composition is resolved and a runnable Vapor App is built.*
- **Vapor App Composition**: *The selected and resolved effective content composition of a Steam App Instance, defined by exactly one Packagepack. It represents the effective Engine, Game, Engine Mods, Game Mods, Extension Mods, and subordinate packs after composition/dependency resolution.*
- **Vapor App**: *A built, deployable, and launchable realization of a Packagepack's Vapor App Composition for a particular supported target. Multiple Vapor Apps may exist locally at once, while one Vapor App Composition may be selected as the current/default composition of the Steam App Instance.*

---

## User and Capability Model
- **Player**: *The base Vapor capability level. A Player consumes finished Vapor Apps: discovering, acquiring, selecting, and launching complete compositions. A Player does not compose or develop Vapor Content and does not require Git, Rust/Cargo, SteamCMD, or development tooling.*
- **Composer / Content User**: *A Vapor capability level above Player. A Composer may discover and use existing Vapor Content and may create, modify, build, and publish Packagepacks, Enginepacks, Gamepacks, and Modpacks. A Composer may not create or modify Engines, Games, Engine Mods, Game Mods, or Extension Mods. Composer capability requires source/build tooling such as Git and Rust/Cargo because Packagepacks are statically built into Vapor Apps.*
- **Content Developer**: *A Vapor capability level above Composer. A Content Developer may additionally create and modify Engines, Games, Engine Mods, Game Mods, and Extension Mods using the Vapor SDK and associated development workflows.*
- **Ecosystem Developer**: *A Vapor capability level above Content Developer. An Ecosystem Developer develops Vapor itself, including its applications, SDK/toolchain, CLI, root framework, server infrastructure, and official repositories. Ecosystem development additionally requires the relevant authorization to contribute to official Vapor repositories and infrastructure.*
- **Root Authority**: *The highest Vapor capability and authority level. It includes all Ecosystem Developer capabilities plus ultimate administrative and ownership authority over the official Vapor ecosystem, namespaces, repositories, and infrastructure.*
- **Capability Level**: *The locally installed set of Vapor capabilities available through a Steam App Instance. Capability levels form a strict progression: Player ⊂ Composer ⊂ Content Developer ⊂ Ecosystem Developer ⊂ Root Authority. External authentication and authorization may gate individual operations without determining which local capability level is installed.*

---

## Vapor Applications and Tooling
- **Vapor Installer**: *The application responsible for changing the fundamental Vapor capabilities installed in a Steam App Instance. It installs, detects, configures, upgrades, downgrades, repairs, and removes capability-specific dependencies and tooling such as Git, SteamCMD, and the Rust/Cargo toolchain.*
- **Vapor Launcher**: *The primary Vapor application used after the required capabilities are installed. It provides access to Vapor Apps, local/source Vapor Content, composition workflows, accounts, settings, development capabilities, and external ecosystem services, and launches selected Vapor Apps.*
- **Vapor SDK**: *The Content Developer-oriented portion of the Vapor Launcher concerned with creating, programming, configuring, building, testing, and inspecting Engines, Games, Engine Mods, Game Mods, and Extension Mods. It is a capability surface of the Vapor Launcher rather than a separate application.*
- **Vapor CLI**: *The command-line interface to developer-oriented Vapor capabilities. It is intended primarily for Content Developers, Ecosystem Developers, and Root Authority and should expose approximately the same underlying operations as corresponding graphical tooling where reasonable.*

---

## Vapor Content Model
- **Vapor Content**: *The common category for Vapor's content and composition artifacts.*
- **Packagepack**: *A Vapor Content artifact type representing one complete Vapor composition. It is the only pack type that defines a complete composition and can therefore be built into a Vapor App.*
- **Enginepack**: *A declarative Vapor Content artifact type containing exactly one Engine and any number of compatible Engine Mods. It is a reusable composition fragment and cannot independently produce a Vapor App.*
- **Gamepack**: *A declarative Vapor Content artifact type containing exactly one Game and any number of compatible Game Mods. It is a reusable composition fragment and cannot independently produce a Vapor App.*
- **Modpack**: *A declarative Vapor Content artifact type containing Engine Mods, Game Mods, and/or Extension Mods whose dependency chains ultimately resolve to the effectively selected Engine and/or Game. It is a reusable composition fragment and cannot independently produce a Vapor App.*
- **Engine**: *A Vapor Content artifact type defining the foundational technical/runtime model of a composition. The effective Engine declares the composition's main binary.*
- **Game**: *A Vapor Content artifact type defining game-specific behavior and content within an Engine-defined foundation. A Game does not declare the composition's main binary.*
- **Engine Mod**: *A Vapor Content artifact type that targets and extends an Engine.*
- **Game Mod**: *A Vapor Content artifact type that targets and extends a Game.*
- **Extension Mod**: *A Vapor Content artifact type that targets and extends another Engine Mod, Game Mod, or Extension Mod.*

---

## Source, Distribution, and Registry Model
- **Git Source Model**: *Vapor Content source lives in Vapor-compatible Git repositories. Git is used not only for source-bearing Vapor Workspaces but also for Container Repos that organize those Workspaces. Source distribution and collaboration therefore belong to the Git/repository side of the ecosystem rather than Steam Workshop.*
- **Steam Workshop**: *The external distribution system used by Vapor for built, published complete compositions. Steam Workshop does not serve as the canonical source-code distribution mechanism for individual Vapor Content.*
- **Steam Workshop Item**: *A Steam Workshop publication/distribution container for a built published complete Vapor composition / Vapor App.*
- **Vapor Content Registry**: *The central semantic identity and linkage layer of the Vapor ecosystem. It associates human-readable Vapor IDs/namespaces with the relevant external resources and identities used by the ecosystem, including Git-backed source and Steam Workshop-backed built composition distribution. Its exact persistence and mapping schema remains an implementation/design concern.*
- **Server**: *An official Vapor server application hosting part of the central Vapor service infrastructure, such as the Vapor Content Registry. User-hosted Vapor registries are not currently part of the ecosystem model.*

---

## Source, Build, and Local-State Model
- **Vapor Library**: *The user-facing/local view over Vapor artifacts available to the Steam App Instance. Depending on capability level this may include installed Vapor Apps, locally available packs, source content acquired through Git-backed workflows, and build outputs. The model does not currently require a separate top-level "Development Content" artifact type, although future development-state distinctions may still become useful.*
- **Build**: *The process of resolving and compiling a complete Packagepack-defined Vapor App Composition into a target-specific Vapor App. Composition builds are logically complete static builds, while Cargo/Vapor caching and incremental compilation may avoid physically rebuilding unchanged work.*
- **Deploy / Install**: *The process of taking a built or externally acquired Vapor App and registering/placing it locally so that it is available for selection and launch.*

---

## Development Storage Model
- **Vapor Superworkspace**: *A disposable local checkout container holding checked-out Vapor repositories. It is not itself a Git repository or primary source-bearing unit, as in: losing it primarily risks local unpushed/uncommitted development state rather than canonical remote source.*
- **Container Repo**: *A Vapor-managed top-level Git repository that groups related Source Repos / Vapor Workspaces as Git submodules. A Container Repo is itself Git-managed but is not used as a submodule of another Container Repo.*
- **Source Repo / Vapor Workspace**: *A Vapor-managed source-bearing Git repository contained by a Container Repo as a Git submodule. It contains one or more Vapor Projects and does not itself contain nested Git submodules.*
- **Vapor Project**: *A Rust/Cargo workspace contained inside a Source Repo / Vapor Workspace. It is not itself a Git repository.*
- **Vapor Root Workspace**: *The unique Vapor Workspace containing the client-side/root Vapor codebase and bootstrapping model of the Vapor ecosystem.*
- **Vapor Root Project**: *A Vapor Project inside the Vapor Root Workspace modeling part of the client-side/root Vapor ecosystem.*
- **Vapor Server Root Workspace**: *The unique Vapor Workspace containing the server-side root Vapor codebase.*
- **Vapor Server Root Project**: *A Vapor Project inside the Vapor Server Root Workspace modeling part of the server-side/root Vapor ecosystem.*
- **Vapor Content Workspace**: *A non-unique Vapor Workspace containing Vapor Content Projects, with "Loo-Cast" as the first-party example.*
- **Vapor Content Project**: *A Vapor Project inside a Vapor Content Workspace that models a Packagepack, Enginepack, Gamepack, Modpack, Engine, Game, Engine Mod, Game Mod, or Extension Mod.*
