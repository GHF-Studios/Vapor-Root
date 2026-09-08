> [!info]
> This glossary defines the canonical meanings of the primary Vapor ecosystem terms.
>
> It is a terminology reference, not an exhaustive workflow specification.
>
> When a term requires deeper behavioral rules, the relevant focused model document remains authoritative.

---

# Core Product Model

* **Steam App**: *The singular Steam-distributed Loo Cast product through which the Vapor Client, Vapor Installer, default first-party Vapor App, and wider Vapor environment are delivered.*

* **Steam App Instance**: *One concrete local Steam installation of the Steam App. It defines the Steam/depot-owned installation boundary. Vapor User Data, the canonical Superworkspace, and other mutable development state may live outside the Steam-managed installation root.*

* **Vapor Client**: *The complete user-side Vapor product/environment. It includes the Launcher, SDK/Development Mode, CLI/Core-facing local behavior, Vapor App management, development tooling integration, and communication with Vapor Platform services.*

* **Vapor App**: *A built, deployable, and launchable realization of one complete Packagepack-defined Vapor App Composition for a supported target.*

* **Vapor App Composition**: *The exact resolved complete Vapor Content graph represented by one Packagepack for a particular version/resolution context.*

* **Vapor App Runtime**: *An executing realization of a Vapor App Composition.*

---

# Role, Authentication, and Authority

* **Player**: *The base installed Vapor Role. A Player consumes finished Vapor Apps and does not require source-development tooling such as Git or Rust/Cargo.*

* **Composer / Content User**: *A Vapor Role above Player. A Composer may create and modify composition-oriented Content Projects such as Packagepacks, Enginepacks, Gamepacks, and Modpacks using existing behavioral/reusable Content.*

* **Content Developer**: *A Vapor Role above Composer. A Content Developer may additionally create and modify behavioral and reusable Content Projects such as Engines, Games, Mods, and Libraries.*

* **Ecosystem Developer**: *The highest ordinary installed Vapor Role. An Ecosystem Developer may develop Vapor itself, including Client, Platform, Registry, tooling, distribution, and other first-party infrastructure. This Role does not itself grant authority over protected official resources.*

* **Vapor Role**: *The locally installed level of Vapor capability/tooling. Roles form the progression Player → Composer → Content Developer → Ecosystem Developer.*

* **Authentication**: *Establishment of who is attempting an operation.*

* **Authorization**: *Permission for an authenticated identity to perform a particular protected operation against a particular target. Authorization is separate from installed Role.*

* **Root Authority**: *The ultimate trusted administrative authority over official Vapor namespaces, first-party classification, protected repositories, Registry administration, official distribution/deployment targets, and other protected Vapor resources. Root Authority is authority, not an installed Role above Ecosystem Developer.*

---

# Structural Source Identity

The canonical source hierarchy is:

```text id="md70i5"
Authority
└── Source Repo Container
    └── Source Repo
        └── Project
```

A complete Project identity therefore has the form:

```text id="m759n7"
Authority/Source-Repo-Container/Source-Repo/Project
```

For example:

```text id="fdj28u"
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

---

* **Authority**: *The first globally meaningful segment of a Vapor source identity. An Authority owns/names Source Repo Containers within the Vapor Registry. `GHF-Studios` is an example. A Vapor Authority may be linked to a Git-host organization/user but is not defined merely by that provider identity.*

* **Source Repo Container**: *A registered top-level Vapor Git repository which groups related Source Repos, normally through authored Git submodules. It is the conservative default independent source-acquisition unit.*

* **Container Repo**: *Legacy/short-form terminology for Source Repo Container. New normative documentation should prefer **Source Repo Container** because it distinguishes the layer explicitly from Source Repo.*

* **Source Repo**: *The primary source-bearing Git repository beneath a Source Repo Container.*

* **Vapor Workspace**: *The Vapor development view of a Source Repo. The normative relationship is exactly: **one Source Repo = one Vapor Workspace**. Workspace is not an additional hierarchy layer.*

* **Project / Vapor Project**: *A modeled development unit inside one Source Repo. A Project is not itself a Git repository. It may be a typed Vapor Content Project or an internal/non-Content development unit.*

* **Project Kind**: *The semantic kind of an existing Project. Content-oriented examples include Game, Engine, Library, Packagepack, and the various pack/mod kinds. The Project kind is metadata/semantics, not an additional identity-path segment.*

* **Vapor Path**: *A slash-separated path through Vapor's canonical structural identity hierarchy. A path may terminate at Authority, Source Repo Container, Source Repo, or Project.*

* **Full Vapor Path**: *A Vapor Path beginning at Authority and containing every hierarchy segment down to the target object.*

* **Relative Vapor Path**: *A Vapor Path whose missing leading prefix is supplied by Context. Relative paths may omit a prefix but may never omit arbitrary middle hierarchy segments.*

* **Canonical Identity**: *The exact resolved hierarchical Vapor identity of a modeled object. For a Project, this includes Authority, Source Repo Container, Source Repo, and Project.*

* **Local Name**: *One object's human-readable name inside its immediate parent scope. A Local Name is not globally unique.*

* **Content ID**: *The canonical Project identity of a Vapor Content Project. Content kind is not an extra ID segment.*

---

# Identity Movement

* **Identity-Preserving Relocation**: *Movement of local filesystem storage, such as relocating the canonical Superworkspace, which does not change Vapor identity.*

* **Identity-Changing Topology Movement**: *Moving a Project to another Source Repo, a Source Repo to another Source Repo Container, or otherwise changing canonical hierarchy segments. Because those segments participate in canonical identity, such movement changes the affected canonical identity and requires an explicit migration rather than being treated as transparent relocation.*

* **Provider Linkage Change**: *Changing an external Git-provider repository/linkage while preserving the modeled Vapor identity where explicitly supported.*

---

# Superworkspace and Local Storage

* **Vapor Superworkspace / Superworkspace**: *The single canonical local development source root known to Vapor. It has no Vapor identity, is never an ID segment, and provides the canonical local realization location for acquired/created Source Repo Containers.*

* **Superworkspace Location**: *The configured filesystem path of the canonical Superworkspace. Its physical path and directory name have no semantic Vapor identity meaning.*

* **Canonical Local Source Realization**: *The locally available realization of a registered Vapor source identity inside the canonical Superworkspace. Git state such as branch, dirty files, detached submodule HEADs, or local commits belongs to that realization and is not automatically invalid Vapor state.*

* **Vapor User Data**: *Mutable Vapor-owned user-level state stored outside the Steam depot where appropriate. It may contain Role/toolchain state, configured Superworkspace location, Context state, caches, indexes, generated realization, IDE integration, and similar mutable/derived information.*

* **Installation Root**: *The Steam-managed local root of the Steam App Instance.*

* **Authored State**: *State created or deliberately edited as user/project intent and therefore requiring preservation. Examples include source code, Git history, Vapor manifests, Cargo manifests, tests, scripts, explicit operation recipes, and explicit configuration.*

* **Derived State**: *State that Vapor can safely reconstruct from authored/modelled truth. Derived state should ideally be deleteable, rediscoverable, regeneratable, and idempotently repairable.*

---

# Context and Selection

The core resolution relationship is:

```text id="k4s7x8"
context  →  operation subject  →  selection
  left            middle            right
```

---

* **Context**: *A lightweight persistent Vapor location/resolution prefix. Context supplies hierarchy information missing to the left of an operation subject or relative selector. Context is a convenience and is not required for Vapor functionality.*

* **Persistent Context**: *The currently remembered Vapor resolution prefix for a frontend/session.*

* **Transient Context**: *A per-operation Context override which changes resolution for one invocation/action without modifying persistent Context.*

* **Open**: *The conceptual operation which establishes or navigates persistent Vapor Context. Open is location/navigation state, not source acquisition or operation selection.*

* **Close**: *The conceptual operation which clears or navigates away from persistent Vapor Context. Close does not delete, remove, forget, or uninstall source.*

* **Operation Subject**: *The modeled object or facility an operation is fundamentally about. It establishes the operation's natural complete scope.*

* **Natural Operation Scope**: *The complete semantic scope implied by the operation subject when no explicit Selection is supplied. It is not a hidden generic `--all` expansion.*

* **Selection**: *Explicit per-operation intent which narrows or otherwise specifies descendants/constituents to the right of the operation subject.*

* **Hierarchical Selection**: *Selection of a topology node as an operation scope together with whatever descendants the operation defines as participating at that scope.*

* **Multiple Selection**: *Explicit selection of several topology nodes for one operation when that operation supports such multiplicity.*

* **Selector**: *User/frontend input which resolves to one or more exact modeled Vapor objects. A selector may be a full Vapor Path, a Context-relative path, or a shorter uniquely resolvable name/path.*

* **Minimal Sufficient Selector**: *The shortest selector which can be resolved uniquely within the applicable resolution universe.*

* **Ambiguity**: *A normal resolution result in which supplied information matches more than one valid candidate. Vapor must not guess between candidates.*

* **Ambiguity Diagnostic**: *An error/interactive result which enumerates matching exact candidates and the safe ways the user can disambiguate them, including stronger selectors and Context alternatives where applicable.*

* **CWD / Current Working Directory**: *Ordinary shell filesystem state. CWD is not normative Vapor Context and must not silently determine semantic Vapor operation scope.*

* **Transient UI Selection**: *A graphical frontend's immediate selected tree items/nodes/etc. It may be converted into explicit operation Selection without automatically mutating persistent Vapor Context.*

* **Development Session**: *Frontend-local interaction state such as UI navigation, open editors, logs, documents, and transient selections. It is not itself the canonical identity or operation scope of the modeled source objects.*

* **Resume State**: *Durable local information used to reconstruct useful frontend state after restart. Resume State must not silently become operation Selection.*

---

# Operations

* **Operation**: *A modeled Vapor action performed against an operation subject, optionally with explicit Selection and operation-specific parameters.*

* **Operation Variant**: *A semantically distinct form of an operation, such as `platform-server deploy local` versus `platform-server deploy vps`.*

* **Operation Legality**: *The rules determining whether a particular subject, operation, variant, Selection, topology, Role, authority state, and configuration form a valid request.*

* **Operation Recipe**: *Lightweight authored configuration describing how a modeled operation is realized using Vapor-native primitives and, where necessary, small explicit scripts/processes. It is not intended to become an arbitrary application/plugin runtime.*

* **Natural Complete Operation**: *Execution of an operation against its complete subject without explicit narrowing. For example, `platform-server test` means “perform the Platform Server test operation,” not mechanically “run the same test command against every descendant.”*

* **Specific Diagnostic**: *An operation failure which explains the actual violated invariant and offers specific safe resolutions rather than a generic `invalid scope`-style error.*

* **Diagnose**: *An operation which observes and explains modeled state, including source topology, Git, Cargo, toolchain, provider, generated state, and other relevant domains.*

* **Repair**: *Reconstruction or reconciliation of safely derivable Vapor-owned state. Repair does not silently reacquire authored source, reset Git, restore authored files, discard commits, or otherwise treat user source as disposable.*

---

# Source Acquisition and Creation

* **Source Acquisition**: *Registry-aware acquisition of modeled Vapor source into its canonical location in the Superworkspace.*

* **Source Repo Container Acquisition**: *Acquisition of a registered Source Repo Container together with initialization of its authored Source Repo/submodule topology. This is the conservative default acquisition boundary.*

* **Independent Source Repo Acquisition**: *Acquisition of one Source Repo without its complete parent Container. This is allowed only when explicitly modeled as independently realizable/acquirable.*

* **Acquisition Atomicity**: *The modeled minimum unit Vapor guarantees can be independently acquired and coherently realized. The conservative default is Source Repo Container.*

* **Source Repo Container Creation**: *Creation of a new registered Source Repo Container identity, provider-backed Git repository, local canonical checkout, and corresponding Registry/provider linkage. It does not implicitly create Source Repos or Projects.*

* **Source Repo Creation**: *Creation of a new Source Repo / Vapor Workspace beneath an existing Source Repo Container, including its provider Git repository, Workspace metadata, registration, and authored Git-submodule relationship.*

* **Typed Project Creation**: *Creation of a new Project with an explicitly supplied Project kind such as Game, Engine, or Library. Creation states the kind because it cannot be inferred from a nonexistent Project.*

* **Missing Parent**: *A creation state in which a required Authority/Source Repo Container/Source Repo parent does not exist or is unavailable. Vapor should report the exact missing layer and the appropriate safe create/acquire operation.*

* **Registered Vapor Source**: *Source whose identity, source kind/topology, and provider linkage are known through the Vapor Registry.*

* **Arbitrary Git Repository**: *A Git repository which is not necessarily registered/modelled Vapor source. Ordinary Git, not Vapor acquisition, is the general mechanism for arbitrary repositories.*

---

# Git Model

* **Git Source Model**: *Vapor's source model uses real Git repositories. Both Source Repo Containers and Source Repos are Git repositories, and normal Git concepts remain visible.*

* **Git Provider**: *An external host for Git repositories, such as GitHub. Provider identity/linkage is distinct from Vapor identity.*

* **Provider Repository**: *The external Git-host repository backing a Source Repo Container or Source Repo.*

* **Git Submodule Topology**: *The authored relationship by which Source Repos normally participate beneath their Source Repo Container.*

* **Git Reconciliation**: *Diagnosis and possible correction of authored Git topology/state, such as a declared but missing submodule checkout. Git reconciliation is distinct from source acquisition and Vapor Repair.*

* **Dirty Source**: *A Git working tree containing uncommitted changes. Dirty source is ordinary valid development state and is not synonymous with broken source.*

* **Detached HEAD**: *Normal Git state which may be expected for submodules or development workflows. It is not automatically Vapor corruption.*

* **Gitlink**: *The commit reference recorded by a parent Git repository for a submodule. A Source Repo checkout differing from the current gitlink may be intentional development state.*

* **Fork**: *A Git/provider relationship. A provider fork does not automatically create a new Vapor identity, and a new Vapor identity is not created merely by forking.*

---

# Source Trust and Facility Model

* **Standard Source**: *Ordinary registered Vapor source without first-party trusted facility privileges.*

* **First-Party Source**: *Source whose trusted first-party status is granted through Root-Authority/Registry-controlled state rather than self-declared by the source itself.*

* **First-Party Trust**: *The trusted classification which permits Vapor to attach protected/bespoke first-party semantics to a source facility. Third-party source cannot self-grant this classification.*

* **First-Party Facility**: *A stable semantic Vapor subsystem such as Client, Platform Server, or Examples whose public behavior may remain stable even if its backing repository topology changes.*

* **Facility Binding**: *Trusted Registry/model linkage between a first-party facility name and the Source Repo Container(s), Source Repo(s), Projects, and operation configuration currently realizing it.*

* **Bespoke CLI Surface**: *A dedicated first-party command namespace such as `client`, `platform-server`, or `examples`. First-party trust enables such a surface; it does not require every first-party repository to receive one.*

* **Source Repo Container Kind / Purpose**: *Potential modeled classification describing what a Container is for, distinct from whether it is trusted first-party source. The exact taxonomy remains open.*

* **Repository Visibility**: *Provider-level public/private accessibility. Newly created developer repositories should be conservative/private by default where supported.*

* **Independent Acquirability**: *Whether one Source Repo may be acquired independently from its Container. This is distinct from provider repository visibility and is opt-in by default.*

---

# Vapor Content Model

* **Vapor Content**: *The category of typed Vapor Projects which participate in Vapor composition, dependency, versioning, publication, and reuse semantics.*

* **Vapor Content Project / Content Project**: *A Vapor Project whose Project Kind is one of Vapor's Content kinds. The Project itself carries the Content identity; Content is not modeled as an additional independently identified artifact nested beneath the Project.*

* **Packagepack**: *A Content Project kind representing one complete authored Vapor composition. It resolves to exactly one effective Engine and exactly one effective Game plus all selected Mods, Libraries, subordinate packs, and other dependencies required for the complete composition.*

* **Enginepack**: *A declarative Content Project kind representing a reusable Engine-side composition fragment containing exactly one effective Engine together with compatible Engine-side Content.*

* **Gamepack**: *A declarative Content Project kind representing a reusable Game-side composition fragment containing exactly one effective Game together with compatible Game-side Content.*

* **Modpack**: *A declarative Content Project kind grouping Mods/dependencies into a reusable composition fragment.*

* **Engine**: *A behavioral Content Project kind defining the foundational technical/runtime model of a Vapor composition.*

* **Game**: *A behavioral Content Project kind defining game-specific behavior/content against an Engine foundation.*

* **Engine Mod**: *A Content Project kind which targets and extends an Engine through extension semantics/contracts explicitly made available by the participating Content.*

* **Game Mod**: *A Content Project kind which targets and extends a Game.*

* **Extension Mod**: *A Content Project kind which targets another Mod.*

* **Library**: *A reusable implementation Content Project kind without inherent Engine, Game, Mod, pack, or runnable-composition semantics. A Library may expose ordinary Rust APIs, traits, ECS vocabulary, algorithms, schemas, or other explicitly authored reusable contracts.*

---

# Dependency and Composition Model

* **Vapor Dependency**: *An authored semantic requirement from one Content Project/version to another.*

* **Outbound Vapor Dependency**: *A dependency viewed from the requiring Content Project.*

* **Inbound Vapor Dependent**: *The reverse view of a resolved dependency edge inside a particular selected graph. It is derived rather than independently authored.*

* **Selected Dependency Closure**: *The exact transitive set of Content Projects/versions reachable from a selected resolution root through outbound Vapor dependencies.*

* **Runtime Foundation Relationship**: *The special semantic role of the Engine dependency against which a Game is authored.*

* **Extension Target**: *The behavioral Content Project which a Mod conceptually extends.*

* **Constituent**: *Selected behavioral Content participating in the effective Engine or effective Game of a resolved composition.*

* **Base Engine**: *The singular Engine selected before Engine-side Mod constituency is applied.*

* **Effective Engine**: *The Base Engine together with selected compatible Engine Mods and Engine-side Extension Mods belonging to the composition.*

* **Base Game**: *The singular Game selected before Game-side Mod constituency is applied.*

* **Effective Game**: *The Base Game together with selected compatible Game Mods and Game-side Extension Mods belonging to the composition.*

* **Support Dependency**: *Selected Content required by other Content without automatically becoming an Engine/Game constituent, such as a Library.*

---

# Version Model

* **Vapor Version**: *The semantic version of a published Vapor Content Project release. Published Content uses SemVer.*

* **Published Content Version**: *An immutable released version of one Content ID bound to exact published source state, including the corresponding Git commit identity required by Vapor's publication model.*

* **Local Development Version**: *A development-time version state which may use a placeholder or appropriate pre-release form until publication establishes an immutable published version.*

* **Version Constraint**: *An authored dependency requirement permitting one or more compatible Content versions according to Vapor's version-resolution rules.*

* **Yank**: *Removal of a published version from ordinary new resolution while preserving its historical identity/data for existing references.*

* **Ban**: *A stronger Registry-level restriction preventing use/discovery according to policy while preserving historical identity rather than pretending the version never existed.*

---

# Registry Model

* **Vapor Registry / Registry**: *The central semantic identity, topology, trust, version, provider-linkage, publication, and resolution service of Vapor. It is broader than a lookup table for Content or Steam Workshop IDs.*

* **Registry Identity**: *A canonical Vapor identity known to the Registry, including Authorities, Source Repo Containers, Source Repos, Projects/Content, and other modeled identities as required.*

* **Provider Registration**: *Registry linkage between a Vapor identity and an external provider identity/resource.*

* **Source Provider Linkage**: *Registry information connecting a Vapor source identity to its backing Git provider/repository.*

* **First-Party Registry State**: *Trusted Registry-controlled information identifying official Vapor facilities/source. Arbitrary source cannot self-author this trust.*

* **Vapor ID**: *A human-readable canonical Vapor identity. For Content Projects, the Content ID is the canonical Project path.*

* **Provider-Native ID**: *An external identifier such as a Git repository ID, Steam Workshop Item ID, Steam account ID, or commit SHA. Provider-native IDs are linkage/realization information rather than replacements for Vapor identity.*

---

# Rust/Cargo Realization Model

* **Vapor Semantic Dependency Graph**: *The graph of Vapor Content identities, versions, bindings, and semantic dependency relationships produced by Vapor resolution.*

* **Cargo Physical Dependency Graph**: *The package/build graph actually resolved and compiled by Cargo.*

* **Cargo Workspace**: *Cargo's own workspace concept. It is not synonymous with Vapor Source Repo or Vapor Workspace merely because a Project may use one.*

* **Cargo Package**: *Cargo's package unit.*

* **Crate / Target**: *Rust/Cargo implementation-level compilation concepts below the Vapor Project layer.*

* **Cargo Metadata**: *Cargo-native project/resolution information, including information available through `cargo metadata`.*

* **Cargo Reconciliation**: *The process by which Vapor ensures that desired Vapor semantic dependencies are appropriately realized in editable Cargo configuration without taking ownership of unrelated Cargo state.*

* **Managed Cargo**: *Cargo executed through Vapor's managed/pinned Rust toolchain environment.*

* **Managed Rust Toolchain**: *The Vapor-controlled Rust/Cargo toolchain used for supported development and especially first-party/root development, rather than relying on an arbitrary ambient system toolchain.*

The conceptual relationship is:

```text id="fn3tge"
Vapor semantic resolution
        ↓
desired semantic Content graph
        ↓
Project/Cargo inspection
        ↓
Cargo reconciliation
        ↓
Cargo physical resolution
        ↓
validated build graph
```

---

# Build, Test, Run, and Deployment

* **Build**: *A semantic Vapor operation whose exact realization depends on its subject. Building a Packagepack, Project, Vapor Client, and Platform Server need not mean the same physical sequence.*

* **Vapor App Build**: *Resolution and compilation of a complete Packagepack-defined Vapor App Composition into a target-specific Vapor App.*

* **Test**: *A semantic Vapor operation whose subject/recipe defines the relevant tests, validation, integration work, and accepted Selection scopes.*

* **Run**: *Execution-oriented Vapor operation which may resolve/build/install a suitable Vapor App or invoke another subject-specific runnable workflow as defined by the subject.*

* **Deployment**: *Movement/realization of a built first-party or service subject into a particular operational environment. Deployment targets may have different scope, authority, validation, and recovery requirements.*

* **Local Deployment**: *Deployment into a local development/test environment.*

* **Steam Deployment**: *Deployment of relevant first-party Client/Steam-owned artifacts through the official Steam distribution path.*

* **VPS Deployment**: *The currently intended remote Platform Server deployment target for the production/development VPS. The name may later generalize if infrastructure topology changes.*

* **Vapor App Installation**: *Registration/placement of a built or externally acquired Vapor App so it is locally available for selection and launch. This is distinct from first-party infrastructure deployment.*

---

# Build and Runtime State

* **Build Currency**: *Whether a previously produced build still corresponds to the currently relevant source/composition state.*

* **Current Build**: *A build corresponding to the current relevant source/resolution state.*

* **Stale Build**: *A previously valid build which no longer corresponds to current source/resolution state.*

* **Failed Build Attempt**: *A failed attempt to produce a new build. It does not imply that an older installed valid Vapor App was destroyed.*

* **Content Library**: *The user-facing/local organizational view over available Vapor Apps, Content Projects, source, and related artifacts according to installed Role. It is distinct from the `Library` Content Project kind.*

---

# Vapor Applications and Tooling

* **Vapor Installer**: *The application/surface responsible for establishing, changing, repairing, or removing installed Vapor Role/tooling capability.*

* **Vapor Launcher**: *The primary ordinary Vapor desktop surface used for playing, managing Vapor Apps, discovery, settings, accounts, and other installed-capability workflows.*

* **Vapor SDK / Development Mode**: *The Launcher-integrated first-party graphical development environment for Vapor Content and, where Role permits, Vapor itself.*

* **Vapor CLI**: *The command-line projection of shared Vapor Core semantics. The universal `vapor` executable exposes the broad semantic CLI surface.*

* **Vapor Core**: *The shared implementation/domain layer owning Vapor semantics used by CLI, GUI, automation, and other frontends.*

* **External IDE Integration**: *Vapor-managed/reconciled integration with an external IDE such as RustRover for editing, language tooling, debugging, and related workflows.*

---

# Vapor Platform Model

* **Vapor Platform**: *The ecosystem-level service/control domain surrounding Vapor Apps, including identity, authentication, authorization, Registry, discovery, publication, diagnostics, documentation services, administration, and future matchmaking/session discovery.*

* **Vapor Platform Server**: *The logical server-side infrastructure implementing Vapor Platform services. It may contain multiple independently meaningful services/processes and is distinct from a Vapor App Server.*

* **Platform Service**: *One independently meaningful server-side service participating in the Vapor Platform, such as Registry, Identity, Diagnostics, or Documentation.*

* **Platform Client**: *A logical capability/library for communicating with Vapor Platform services.*

* **Vapor App Server**: *A future server-side Vapor product/environment for hosting server-side Vapor App runtimes. It is distinct from Vapor Platform Server.*

* **App Client Runtime**: *The client-side runtime role of a running Vapor App.*

* **App Server Runtime**: *The server-side runtime role of a running Vapor App.*

* **Platform Communication**: *Communication with ecosystem/control services such as Registry, Identity, authorization, publication, and discovery.*

* **App-Session Communication**: *Communication belonging to a running Vapor App/session, such as simulation inputs, replication, events, or state updates. Its semantics are defined primarily by the effective Engine/Game rather than universally by Vapor.*

---

# Source and Built Distribution

* **Source Distribution**: *Git-backed preservation/distribution of Vapor source. Source distribution is distinct from Player-facing built Vapor App distribution.*

* **Steam Workshop**: *The intended external distribution backend for published built complete Vapor Apps beyond the depot-shipped default composition.*

* **Steam Workshop Item**: *A Steam-native publication/distribution container associated with a published Vapor App/Packagepack release according to the publication model.*

* **Steam Depot**: *Steam-managed distribution storage for the Vapor Client/bootstrap/runtime files and the default first-party Loo Cast Vapor App.*

* **Source Publication**: *Publication of authored Vapor source/version state through Git/provider/Registry-backed workflows.*

* **Built Publication**: *Publication of a complete built Packagepack-derived Vapor App through the appropriate player-facing distribution backend.*

* **Publication Authority**: *Authorization required to publish source, versions, built artifacts, or official first-party outputs under a protected identity/namespace.*

---

# Preservation and Recovery

* **Source Availability**: *Whether the required registered source is locally available in the canonical Superworkspace.*

* **Missing Source**: *Registered authored source which is not locally available. Missing source is an acquisition/recovery concern, not generic derived-state Repair.*

* **Missing Submodule Checkout**: *A Source Repo declared by an already-acquired Container's Git topology whose local submodule checkout is absent. This is primarily a Git reconciliation concern.*

* **Disaster Recovery**: *Recovery of replaceable Vapor installation/user/derived state and reacquisition of remotely recoverable source while preserving and distinguishing unique local authored work.*

* **Unique Local Work**: *Authored source state which may not exist remotely, such as uncommitted changes or unpushed commits. Vapor must never assume such state is safely disposable.*

---

# Terminology Boundaries

The following distinctions are intentional:

```text id="p30v2i"
Superworkspace
    ≠ identity

Authority
    ≠ authorization

Source Repo Container
    ≠ Source Repo

Source Repo
    = Vapor Workspace

Project
    ≠ Git repository

Project
    ≠ Cargo package

Content kind
    = Project kind
    ≠ identity path segment

Context
    ≠ Selection

Context
    ≠ CWD

Selection
    ≠ persistent Context

no Selection
    ≠ hidden --all

acquire
    ≠ arbitrary git clone

acquire
    ≠ repair

Git reconciliation
    ≠ repair

dirty
    ≠ broken

Role
    ≠ Authority

first-party
    ≠ self-declarable

Vapor identity
    ≠ provider identity

Vapor topology
    ≠ Cargo topology

Steam App Instance
    ≠ Vapor User Data
    ≠ Superworkspace

Vapor Platform Server
    ≠ Vapor App Server
```

---

# Retired / Superseded Terms and Models

The following older concepts should not be treated as current normative Vapor architecture:

* **Multiple ordinary Superworkspaces** as independently modeled local development universes.
* **Implicit Superworkspace** versus **Configured Superworkspace** as separate core modes.
* **Superworkspace identity or display-name identity**.
* **Separate Workspace Semantic Identity vs Container-derived Structural Address** where Source Repo Container placement can change without identity consequences.
* **Multiple canonical local realizations of one Source Repo identity inside the managed Superworkspace**.
* **Open sets of many source objects** as the primary semantic context model.
* **Multiple durable Focus targets** as the primary CLI/Core targeting model.
* **CWD as ordinary Vapor semantic context**.
* **Projects hosting several independently identified Vapor Content artifacts**.
* **Generic public `ecosystem` CLI object/lifecycle**.
* **Bare generic `source` CLI namespace** where the intended layer is actually Source Repo Container or Source Repo.
* **Generic Repair as source reacquisition or Git-reset machinery**.

Historical documents/code may still contain these terms during migration.

Where they conflict with the current Context/Identity, CLI, and Development Experience models, the newer definitions above take precedence.
