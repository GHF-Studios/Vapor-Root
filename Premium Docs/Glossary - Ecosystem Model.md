- **App**: *There is only one App. The "Loo Cast" Steam App. The App provides access not only to the default/first-party "Spacetime Engine" "Loo Cast Game", but also allows usage of and access to the Vapor Ecosystem/Toolchain.*
- **App Composition**: *The selected content composition of an App Instance. The main app binary is just a slim entrypoint. The actual content resides in exactly one selected Packagepack.*
- **App Instance**: *One concrete local instance/copy of the App. It has a root location, a selected App Composition, and the local installation/runtime state needed to discover, download, install/mount, and launch that composition. An App Instance may also be upgraded into (and downgraded from) a developer-mode setup, in which it additionally includes a set of third-party development dependencies/tools like Git, SteamCMD, and the Rust/Cargo Toolchain.*

- **Packagepack**: *A content artifact type. Must resolve to exactly one effective Engine and exactly one effective Game, whether those are provided directly or via an Enginepack and/or Gamepack.*
- **Enginepack**: *A content artifact type. Contains exactly one Engine and any number of compatible Engine Mods.*
- **Gamepack**: *A content artifact type. Contains exactly one Game and any number of compatible Game Mods.*
- **Modpack**: *A content artifact type. Contains Engine Mods, Game Mods, and/or Extension Mods whose dependency chain ultimately resolves to the effectively selected Engine and/or Game.*
- **Engine**: *A content artifact type. Defines the foundational technical/runtime model of a composition and may declare its main binary.*
- **Game**: *A content artifact type. Defines game-specific behavior/content within an engine-defined foundation and may not declare the composition’s main binary.*
- **Engine Mod**: *A content artifact type. Targets and extends an Engine.*
- **Game Mod**: *A content artifact type. Targets and extends a Game.*
- **Extension Mod**: *A content artifact type. Targets and extends another Engine Mod, Game Mod, or Extension Mod.*

- **Server**: *Hosts a Vapor Content Registry.*
- **Steam Workshop**: *Acts as a dumb container for Vapor Content, based on random numeric IDs.*
- **Steam Workshop Item**: *A Steam Workshop publication/distribution container for Vapor Content.*
- **Vapor Content Registry**: *Maps each Workshop Item ID to a human-readable string-based ID/namespace.*

- **Vapor Superworkspace**: *A disposable local checkout container that holds checked-out Vapor repositories. It is not itself a Git repository or primary source-bearing unit, so losing it mainly means losing local, unpushed state.*
- **Container Repo**: *A Vapor-managed organizational top-level Git repository whose main role is to group related Source Repos, often as Git submodules.*
- **Source Repo**: *A Vapor-managed source-bearing Git repository whose main role is to hold actual source code, tests, docs, and related project assets.*
- **Vapor Workspace**: *A type of Container Repo that groups one or more Vapor Projects.*
- **Vapor Project**: *One concrete Vapor-managed project inside a Vapor Workspace, structured as a Rust/Cargo workspace.*

- **Vapor-Root Workspace**: *The unique Vapor Workspace for the Vapor-Root codebase.*
- **Vapor-Server-Root Workspace**: *The unique Vapor Workspace for the Vapor-Server-Root codebase.*
- **Vapor Content Workspace**: *A non-unique Vapor Workspace for content projects, with "Loo-Cast" as the first-party example.*
