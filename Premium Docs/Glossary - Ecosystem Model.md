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

* **Vapor Installer**: *The application responsible for changing the fundamental Vapor Role/tooling installed in a Steam App Instance. It installs, detects, configures, upgrades, downgrades, repairs, and removes role-specific dependencies and tooling such as Git, SteamCMD, and the Rust/Cargo toolchain. The Installer changes what the Installation is equipped to do and remains a distinct application boundary from ordinary Launcher/SDK operation.*

* **Vapor Launcher**: *The primary Vapor desktop application used after the required capability/tooling is installed. In ordinary Launcher Mode it provides access to Vapor Apps, Content Library, composition, discovery, accounts, settings, diagnostics, and other workflows appropriate to the installed Role. Where development capability exists, the Launcher may transition into the richer integrated Vapor SDK / Development Mode without creating a separate semantic implementation.*

* **Vapor SDK**: *The Launcher-integrated development mode and logical Vapor developer surface for creating, programming, configuring, building, testing, inspecting, publishing, and otherwise developing Vapor Content and, where Role permits, Vapor ecosystem source. SDK Mode is functionally a development-oriented superset of the ordinary Launcher surface while the Installer remains a separate capability-management boundary. The SDK is backed by Vapor Core and may additionally expose a dedicated executable/CLI projection without creating independent semantics.*

* **Vapor SDK Session**: *One live graphical development session containing Open/Focused development context, transient selections, active documents/views, run/test context, and other live SDK state. Live Session State is distinct from durable Resume State and need not imply one globally shared focus cursor across every simultaneous Vapor frontend.*

* **Vapor CLI**: *The command-line projection of Vapor Core operations. The universal `vapor` executable exposes the broad command surface, while application-specific executables such as `vapor-installer`, `vapor-launcher`, and `vapor-sdk` expose appropriate subsets/projections of the same underlying operations. CLI/GUI equality means semantic capability equality rather than identical interaction mechanics.*

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
* **Outbound Vapor Dependency**: *An authored requirement from one Vapor Content artifact to another. Outbound dependencies answer what the depender requires to be selected and resolved.*
* **Inbound Vapor Dependent**: *The reverse view of one resolved dependency edge within a particular selected graph. If `A → B`, then A is an inbound dependent of B. Inbound dependents are derived rather than independently authored and must not be confused with every artifact in the global Registry which happens to depend on B.*
* **Selected Dependency Closure**: *The complete exact set of Vapor Content reachable recursively from a selected resolution root through outbound dependencies. For a Packagepack this closure provides the semantic source from which the complete Vapor App Composition is derived.*
* **Runtime Foundation Relationship**: *The special semantic meaning of the required Engine dependency of a Game. The edge remains an ordinary Vapor requirement while additionally identifying the Engine foundation against which the Game is authored.*
* **Extension Target**: *The required behavioral Content which a Mod conceptually extends. An Engine Mod targets an Engine, a Game Mod targets a Game, and an Extension Mod targets another Mod. Targeting defines semantic extension structure but does not define the target's programmatic extension mechanism.*
* **Constituent**: *Selected behavioral Vapor Content which participates in forming an effective Engine or effective Game. Constituency is normally derived from the selected dependency closure, Content kind, and target relationships rather than redundantly authored as a separate dependency type.*
* **Base Engine**: *The singular Engine artifact underlying one complete Vapor App Composition before selected Engine-side Mod constituency is considered.*
* **Effective Engine**: *The Base Engine together with all selected compatible Engine Mods and Engine-side Extension Mods belonging to that composition. Required Libraries and other support dependencies remain part of the resolved graph without automatically becoming Engine constituents.*
* **Base Game**: *The singular Game artifact underlying one complete Vapor App Composition before selected Game-side Mod constituency is considered.*
* **Effective Game**: *The Base Game together with all selected compatible Game Mods and Game-side Extension Mods belonging to that composition.*
* **Support Dependency**: *Selected Vapor Content required by behavioral or composition Content but which is not itself automatically an Engine/Game behavioral constituent, such as a Library.*
A Vapor dependency edge always means that one Content artifact requires another.

Some dependency edges additionally carry semantic meaning such as runtime foundation or extension target.

These semantic roles do not determine the programmatic integration mechanism between the participating artifacts.

Selection is determined by reachability from the chosen composition root.

Constituency is derived only from selected Content.

Therefore an Engine Mod which merely exists in the Registry and targets the effective Engine does not participate unless some selected Content actually requires it.

The programmatic meaning of extension remains defined by the APIs, ECS vocabulary, registries, traits, schemas, callbacks, or other extension contracts authored by the participating Content.

---

## Source, Distribution, and Registry Model

* **Git Source Model**: *Vapor Content source lives in Vapor-compatible Git repositories. Git is used not only for source-bearing Vapor Workspaces but also for Container Repos that organize those Workspaces. Source distribution and collaboration therefore belong to the Git/repository side of the ecosystem rather than Steam Workshop.*
* **Steam Workshop**: *The external distribution system used by Vapor for built, published complete compositions. Steam Workshop does not serve as the canonical source-code distribution mechanism for individual Vapor Content.*
* **Steam Workshop Item**: *A Steam Workshop publication/distribution container for a built published complete Vapor composition / Vapor App.*
* **Vapor Content Registry**: *The central semantic identity and linkage layer of the Vapor ecosystem. It associates human-readable Vapor IDs/namespaces with the relevant external resources and identities used by the ecosystem, including Git-backed source and Steam Workshop-backed built composition distribution. Its exact persistence and mapping schema remains an implementation/design concern.*
* **Vapor Client**: *The complete user-side Vapor product/environment delivered through the Steam App Instance. It encompasses the Launcher/SDK/CLI/Core-facing local Vapor experience, manages local Vapor Apps and development state, communicates with Vapor Platform services, and hosts or launches App Client Runtimes. It is broader than an App Client Runtime and should not be called `Vapor-App-Client`.*
* **Vapor Platform**: *The ecosystem-level service/control domain surrounding Vapor Apps. It includes concerns such as identity, authentication, authorization, Registry, discovery, publication, diagnostics, documentation services, ecosystem administration, and future matchmaking/session discovery.*
* **Vapor Platform Server**: *The logical server-side infrastructure implementing the Vapor Platform. It may consist of many independently deployable services/processes/hosts. The intended Container Repo/product-family name is `Vapor-Platform-Server`, replacing the old `Vapor-Server-Root` terminology.*
* **Platform Service**: *One independently meaningful server-side service participating in the Vapor Platform, such as Identity, Registry, Diagnostics, or Documentation. Platform services may be deployed together initially and distributed independently later.*
* **Platform Client**: *A logical capability/library for communicating with Vapor Platform services. It may be used by the Vapor Client, Vapor App Server, administrative tooling, or automation and is not necessarily an independently shipped application.*
* **Vapor App Runtime**: *An executing realization of a Vapor App Composition. Runtime behavior is ultimately governed by the effective Engine/Game/Mods and may operate in client-side, server-side, local-only, or other explicitly supported roles.*
* **App Client Runtime**: *The client-side runtime role of a running Vapor App, normally executed under the Vapor Client. Exact input, presentation, prediction, simulation, replication, and networking semantics remain Engine/Game-defined rather than universally imposed by Vapor.*
* **Vapor App Server**: *A future server-side Vapor product/environment for hosting networked/server-side Vapor App runtimes. It is distinct from the Vapor Platform Server. The name `Vapor-App-Server` is reserved for this responsibility.*
* **App Server Runtime**: *The server-side runtime role of a running Vapor App. It may provide authoritative simulation, shared state, replication, persistence hooks, or other App-defined behavior. Client/server runtime role does not necessarily imply a separate physical machine.*
* **Platform Communication**: *Communication with ecosystem/control services such as Identity, Registry, publication, diagnostics, matchmaking, and authorization. It is conceptually distinct from App-session communication.*
* **App-Session Communication**: *Communication belonging to a running networked Vapor App/session, such as App-defined simulation inputs, replication, events, or state updates. Vapor may provide common hosting or transport facilities, but the effective Engine/Game defines the actual runtime networking semantics.*


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

## Development Storage, Identity, Context, and Session Model

* **Vapor Superworkspace**: *A local physical development container comparable to an IDE workspace which groups related Vapor development checkouts for SDK use. It is not itself a Git repository, canonical ecosystem identity, or primary source-bearing unit. Two separately created local containers are distinct Superworkspaces even when they contain equivalent source realizations.*

* **Implicit Superworkspace**: *A local directory which Vapor can recognize as a Superworkspace from the Vapor development structures it contains even without an explicit Superworkspace manifest.*

* **Configured Superworkspace**: *A Superworkspace with explicit durable local configuration, potentially including a `Superworkspace.vapor.toml`. Superworkspace configuration may describe friendly names, attached roots, local aliases, layout/preferences, and recovery hints but does not define canonical Workspace/Project/Content identity or remote ecosystem truth.*

* **Container Repo**: *A Vapor-managed top-level Git repository that groups related Source Repos / Vapor Workspaces, normally through Git submodules. A Container Repo participates in canonical structural identity and is itself not used as a submodule of another Container Repo.*

* **Source Repo / Vapor Workspace**: *The primary source-bearing Git repository unit. One Source Repo equals one Vapor Workspace. A Workspace belongs structurally to a Container Repo, contains one or more Vapor Projects, owns Workspace-level authored metadata, and may have multiple local physical realizations/checkouts.*

* **Vapor Project**: *A structural development unit inside a Vapor Workspace, normally corresponding to a coherent Rust/Cargo workspace or execution context for Rust-backed development. A Project is not itself a Git repository. Project identity is namespaced by Workspace identity. A Project may host zero, one, or multiple Vapor Content artifacts and supporting non-Content packages.*

* **Vapor Content Project**: *A Vapor Project primarily concerned with authoring Vapor Content. A Content Project may host one or more independently identified Vapor Content artifacts; Project identity and Content identity are not required to be one-to-one.*

* **Vapor Content Workspace**: *A non-unique Vapor Workspace containing one or more Projects used for Vapor Content development.*

* **Vapor Semantic Identity**: *A durable identity describing which modeled Vapor object something is, independent of incidental local checkout location and, where appropriate, independent of current Container Repo placement. Different object kinds may impose different rename/immutability rules.*

* **Vapor Structural Address**: *A human-readable, case-preserving, slash-separated address describing an object's current modeled location in source topology, such as `GHF-Studios/Vapor-Client/Vapor/Client`. Structural Address is distinct from Semantic Identity and Local Realization and may change during an explicit source-topology migration.*

* **Local Name**: *The final human-readable name of an object within its enclosing semantic or structural scope. A Local Name such as `Client` is not globally unique and must not be treated as a canonical semantic identity outside a context which makes it unambiguous.*

* **Selector**: *Frontend/user input used to resolve one or more exact Vapor targets. A selector may be a full canonical identity or a shorter context-dependent form. Short selectors are conveniences rather than alternate canonical IDs. Vapor may accept the shortest selector which resolves unambiguously for the requested operation.*

* **Local Realization**: *One physical local checkout/realization of a semantic source/development object. Filesystem path, branch, provider repository, fork relationship, and local alias are realization properties rather than semantic identity or structural address. Multiple local realizations of the same Workspace identity may coexist.*

* **Development Session**: *One frontend-local working context containing Open/Focused development objects and other active context required to perform development operations coherently. A GUI, future Vapor Shell, and one-shot CLI invocation need not share one globally mutable live focus cursor.*

* **Resume State**: *Durable local state used to reconstruct a useful future Development Session, such as previously Open Workspaces/Projects and exact remembered Focus. Resume State may preserve missing/broken identities so that recovery remains possible.*

* **Open**: *Session participation state indicating that an object belongs to the current development working set. Multiple sibling objects may be Open. Opening a deep exact target may automatically establish its required exact ancestor contexts.*

* **Focused**: *Session targeting state expressing durable default intent. Multiple objects may be Focused simultaneously. Focus does not imply that every operation accepts multiplicity; operation cardinality decides whether the focused candidate set is valid.*

* **Transient Selection**: *Immediate frontend selection used to target a particular operation without necessarily mutating durable Focus. Examples include selected tree items, graph nodes, or an Inspector target.*

* **Availability State**: *The independent state describing whether a remembered/local realization can currently be accessed, such as Available, Missing, or Unreachable. Availability is distinct from Open/Focused session state.*

* **Health State**: *The independent state describing whether a known object/realization is currently semantically usable, such as Healthy, Degraded, Invalid, or Incompatible. Health is distinct from knowledge, availability, Open state, and Focus.*

* **Context Resolution**: *The process by which Vapor resolves explicit selectors, transient selection, Focus, enclosing scopes, frontend-specific ambient hints, and available candidates into the exact semantic/local targets required by an operation.*

* **Operation Cardinality**: *The number of targets an operation semantically accepts, such as exactly one Project or many Content artifacts. Multiple focused/selected candidates are valid when the operation accepts multiplicity and ambiguous when the operation requires one.*

* **Ambient Context Hint**: *Non-canonical frontend-specific information which may help resolve one operation. The command-line current working directory is an example. Ambient hints may narrow an ephemeral operation context but must not silently redefine persisted Vapor identity or SDK Focus.*
