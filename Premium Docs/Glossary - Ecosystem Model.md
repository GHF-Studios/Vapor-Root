# Glossary - Ecosystem Model

## App Model
- **Steam App**: *The singular Steam-distributed "Loo Cast" product. It installs and provides access to the Vapor Installer, Vapor Launcher, default first-party Vapor App, and the wider Vapor Ecosystem/Toolchain.*
- **App Instance**: *One concrete local installation of the Steam App. It has a Steam-managed root location, local Vapor state, installed Vapor Content and Vapor Apps, and an installed capability level. The normal model assumes exactly one App Instance per Steam installation; users do not manually create additional instances.*
- **Packagepack**: *A Vapor Content artifact type that declaratively defines one complete composition. It must resolve to exactly one effective Engine and exactly one effective Game, whether provided directly or through an Enginepack and/or Gamepack. A Packagepack is the root from which an App Composition is resolved and a Vapor App can be built.*
- **App Composition**: *The complete resolved effective content composition represented by a Packagepack. It includes the effective Engine, Game, and all effective Engine Mods, Game Mods, Extension Mods, and subordinate packs after dependency resolution.*
- **Vapor App**: *A built, deployable, and launchable realization of an App Composition for a particular supported target. A Vapor App is ultimately derived from exactly one Packagepack. Multiple Vapor Apps may be installed locally at once, while one may be selected as the current/default Vapor App.*

---

## User and Capability Model
- **Player**: *The base Vapor capability level. A Player consumes finished Vapor Apps: discovering, installing, selecting, and launching them. A Player does not compose content and does not require Git, Rust/Cargo, SteamCMD, or development tooling.*
- **Composer / Content User**: *A Vapor capability level above Player. A Composer may discover and compose existing Vapor Content and may create, modify, build, and publish Packagepacks, Enginepacks, Gamepacks, and Modpacks. A Composer may not create or modify Engines, Games, Engine Mods, Game Mods, or Extension Mods. Composer capability requires the source/build tooling needed to statically rebuild compositions, including Git and Rust/Cargo.*
- **Content Developer**: *A Vapor capability level above Composer. A Content Developer may additionally create and modify Engines, Games, Engine Mods, Game Mods, and Extension Mods, using the Vapor SDK and associated development workflows.*
- **Ecosystem Developer**: *A Vapor capability level above Content Developer. An Ecosystem Developer develops Vapor itself, including its client applications, SDK/toolchain, CLI, root framework, server infrastructure, and related official repositories. Ecosystem development additionally requires the relevant authorization to contribute to official Vapor repositories and infrastructure.*
- **Root Authority**: *The highest Vapor capability and authority level. It includes all Ecosystem Developer capabilities plus ultimate administrative and ownership authority over the official Vapor ecosystem, namespaces, repositories, and infrastructure.*
- **Capability Level**: *The locally installed set of Vapor capabilities available through an App Instance. Capability levels form a strict progression: Player ⊂ Composer ⊂ Content Developer ⊂ Ecosystem Developer ⊂ Root Authority. External authentication and authorization may gate individual operations without changing which local capabilities are installed.*

---

## Vapor Applications and Tooling
- **Vapor Installer**: *The application responsible for changing the fundamental Vapor capabilities installed in an App Instance. It installs, configures, upgrades, downgrades, and removes capability-specific dependencies and tooling such as Git, SteamCMD, and the Rust/Cargo toolchain.*
- **Vapor Launcher**: *The primary Vapor application used after the required capabilities are installed. It provides access to Vapor Apps, Vapor Content, composition workflows, accounts, settings, and development capabilities, and launches selected Vapor Apps.*
- **Vapor SDK**: *The Content Developer-oriented portion of the Vapor Launcher concerned with creating, programming, configuring, building, testing, and inspecting Engines, Games, Engine Mods, Game Mods, and Extension Mods. It is a capability surface of the Launcher rather than a separate application.*
- **Vapor CLI**: *The command-line interface to developer-oriented Vapor capabilities. It is intended primarily for Content Developers, Ecosystem Developers, and Root Authority, and should expose approximately the same underlying operations as the corresponding graphical tooling where reasonable.*

---

## Vapor Content Model
- **Vapor Content**: *The common category for Vapor's source-distributed content artifacts and pack artifacts.*
- **Engine**: *A Vapor Content artifact type that defines the foundational technical/runtime model of a composition and may declare its main binary.*
- **Game**: *A Vapor Content artifact type that defines game-specific behavior and content within an Engine-defined foundation. A Game may not declare the composition's main binary.*
- **Engine Mod**: *A Vapor Content artifact type that targets and extends an Engine.*
- **Game Mod**: *A Vapor Content artifact type that targets and extends a Game.*
- **Extension Mod**: *A Vapor Content artifact type that targets and extends another Engine Mod, Game Mod, or Extension Mod.*
- **Enginepack**: *A declarative Vapor Content artifact type containing exactly one Engine and any number of compatible Engine Mods. It is a reusable composition fragment and is not independently buildable into a Vapor App.*
- **Gamepack**: *A declarative Vapor Content artifact type containing exactly one Game and any number of compatible Game Mods. It is a reusable composition fragment and is not independently buildable into a Vapor App.*
- **Modpack**: *A declarative Vapor Content artifact type containing Engine Mods, Game Mods, and/or Extension Mods whose dependency chains ultimately resolve to the effectively selected Engine and/or Game. It is a reusable composition fragment and is not independently buildable into a Vapor App.*

---

## Distribution and Registry Model
- **Steam Workshop**: *A distribution system used by Vapor as a relatively dumb container for publicly distributed Vapor Content and Vapor Apps, identified externally by opaque numeric Workshop Item IDs.*
- **Steam Workshop Item**: *A Steam Workshop publication/distribution container associated with Vapor Content or a distributed Vapor App.*
- **Vapor Content Registry**: *The central official Vapor registry that maps human-readable Vapor IDs/namespaces to the corresponding external distribution identities, primarily Steam Workshop Item IDs. It may also participate in Vapor identity, ownership, and account-linking workflows.*
- **Server**: *An official Vapor server application hosting part of the central Vapor service infrastructure, such as the Vapor Content Registry. User-hosted Vapor registries are not currently part of the ecosystem model.*

---

## Source, Build, and Library Model
- **Vapor Library**: *The locally available collection of Vapor Content and packs known to the App Instance. Locally present dependencies are not modeled separately from installed or cached content: if Vapor Content exists locally, it is part of the local content model.*
- **Build**: *The process of resolving and compiling a complete Packagepack-defined App Composition into a target-specific Vapor App. Composition builds are logically complete static builds, while Cargo/Vapor caching and incremental compilation may avoid physically rebuilding unchanged work.*
- **Deploy / Install**: *The process of taking a built or acquired Vapor App and registering/placing it locally so that it is available for selection and launch.*

---

## Development Storage Model
- **Vapor Superworkspace**: *A disposable local checkout container holding checked-out Vapor repositories. It is not itself a Git repository or primary source-bearing unit. Losing it primarily risks local unpushed/uncommitted development state rather than canonical remote source.*
- **Container Repo**: *A Vapor-managed top-level Git repository that groups related Source Repos / Vapor Workspaces as Git submodules. A Container Repo is not itself used as a submodule of another repository.*
- **Source Repo / Vapor Workspace**: *A Vapor-managed source-bearing Git repository contained by a Container Repo as a Git submodule. It contains one or more Vapor Projects and does not itself contain nested Git submodules.*
- **Vapor Project**: *A Rust/Cargo workspace contained inside a Source Repo / Vapor Workspace. It is not itself a Git repository.*
- **Vapor Root Workspace**: *The unique Vapor Workspace containing the client-side/root Vapor codebase and bootstrapping model of the Vapor ecosystem.*
- **Vapor Root Project**: *A Vapor Project inside the Vapor Root Workspace modeling part of the client-side/root Vapor ecosystem.*
- **Vapor Server Root Workspace**: *The unique Vapor Workspace containing the server-side root Vapor codebase.*
- **Vapor Server Root Project**: *A Vapor Project inside the Vapor Server Root Workspace modeling part of the server-side/root Vapor ecosystem.*
- **Vapor Content Workspace**: *A non-unique Vapor Workspace containing Vapor Content Projects, with "Loo-Cast" as the first-party example.*
- **Vapor Content Project**: *A Vapor Project inside a Vapor Content Workspace that models a Packagepack, Enginepack, Gamepack, Modpack, Engine, Game, Engine Mod, Game Mod, or Extension Mod.*
