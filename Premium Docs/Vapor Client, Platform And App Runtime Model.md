> [!info]
> This document defines Vapor's primary runtime/product topology across the user-side **Vapor Client**, the online **Vapor Platform Server**, and future **Vapor App Server** environments.
>
> It establishes the canonical meanings of:
>
> * Vapor Client;
> * Vapor Platform;
> * Vapor Platform Server;
> * Vapor App;
> * Vapor App Runtime;
> * App Client Runtime;
> * Vapor App Server;
> * App Server Runtime;
> * Platform communication;
> * App-session communication.
>
> Source identity and repository topology are defined by the **Vapor Context, Identity, Session And Selection Model** and **Vapor Repository Topology And Migration Model**.
>
> Development-state ownership and the Superworkspace/User Data split are defined in greater detail by the **Vapor Development Experience Model**.
>
> Vapor Content composition remains defined by the dependency/composition, operational, and publishing models.

---

# Core Product Topology

Vapor has three major runtime/product domains:

```text id="znv3u2"
Vapor Client

Vapor Platform Server

Vapor App Server
```

They are not symmetrical binaries.

They represent different semantic responsibilities.

Conceptually:

```text id="68rx7t"
                         Vapor Platform Server
                    identity / registry / discovery
                    publication / authorization
                    diagnostics / matchmaking / ...
                               ↑       ↑
                               │       │
                    Platform communication
                               │       │
             ┌─────────────────┘       └─────────────────┐
             │                                           │
             ▼                                           ▼
      Vapor Client                               Vapor App Server
             │                                           │
             │                                           │
      App Client Runtime  ═══ App-session traffic ═══ App Server Runtime
```

The Vapor Client may communicate with:

* Vapor Platform services;
* remote App Server Runtimes;
* local App runtimes.

A Vapor App Server may communicate with:

* Vapor Platform services;
* connected App Client Runtimes;
* other App-defined runtime infrastructure where appropriate.

---

# Vapor Client

The **Vapor Client** is the complete user-side Vapor software/product environment.

It is the Vapor product delivered to and operated on a user's machine through the Steam App.

Its responsibilities may include:

* Vapor Launcher;
* Vapor SDK / Development Mode;
* Vapor CLI;
* Vapor Core;
* Vapor Entrypoint/bootstrap behavior;
* Vapor Installer integration;
* local Vapor App management;
* local runtime management;
* managed toolchain integration;
* source/development workflow orchestration;
* Content discovery;
* source acquisition;
* composition;
* build/test/run;
* publication workflows;
* diagnostics;
* local user-facing state;
* communication with Vapor Platform services;
* communication with App Server Runtimes.

The Vapor Client is therefore broader than:

```text id="y183rw"
App Client Runtime
```

and broader than:

```text id="z7pi7n"
Vapor Launcher
Vapor SDK
Vapor CLI
```

Those are applications/surfaces/runtime roles within or associated with the broader Client environment.

---

# Vapor Client Is Also a Platform Client

The Vapor Client communicates with Vapor Platform services.

Therefore it acts as a **Platform Client**.

This does not mean:

```text id="prxkl3"
Vapor Client
=
Platform Client library
```

`Platform Client` is a communication capability.

`Vapor Client` is a complete user-side product/environment.

For this reason the complete user-side product should not be called:

```text id="bwy2m1"
Vapor App Client
```

or:

```text id="uoqnfg"
Vapor-App-Client
```

Those names incorrectly imply that its only responsibility is one running App's client runtime.

---

# Vapor Client Source Facility

The stable first-party **Client** facility is backed by trusted first-party Vapor source.

Its target Source Repo Container identity is:

```text id="o0bwxf"
GHF-Studios/Vapor-Client
```

historically migrated from:

```text id="tt45yx"
GHF-Studios/Vapor-Root
```

The public semantic facility:

```text id="w3dqd1"
Client
```

and the current source identity:

```text id="m12lc4"
GHF-Studios/Vapor-Client
```

are related but conceptually distinct.

The first-party facility may expose stable operations such as:

```text id="oc7w01"
client acquire
client build
client test
client deploy ...
```

while its internal Source Repo / Project topology evolves.

The facility binding is trusted Vapor state.

It is not merely inferred from a GitHub repository name.

---

# Vapor Client Source Ownership

Long-term implementation should follow:

> **Vapor Core owns shared Vapor semantics; binaries and graphical surfaces project those semantics.**

Historical repositories such as:

```text id="uv4a0g"
Vapor-SDK
Vapor-Launcher
Vapor-Installer
Vapor-Entrypoint
Vapor-Shell
```

may therefore be consolidated into the primary active Client Source Repo where their independent repository boundaries no longer represent meaningful source ownership.

This does not erase their product/application roles.

For example:

```text id="b0hths"
Installer product boundary
    may remain

Installer Source Repo boundary
    may disappear
```

Source topology and product topology are related but not identical.

---

# Vapor Installer

The **Vapor Installer** is the application/process boundary responsible for changing what the local Vapor environment is equipped to do.

It may:

* establish or change installed Vapor Role;
* install/configure development tooling;
* repair installed capability;
* downgrade higher capability;
* manage required platform integration;
* participate in setup/recovery.

Installer remains semantically distinct from ordinary Launcher/SDK use.

Its source may still live inside the broader Client implementation Source Repo.

---

# Vapor Launcher

The **Vapor Launcher** is the primary ordinary graphical Vapor surface.

It may expose:

```text id="y01nqy"
Play
Vapor Apps
Content Library
composition
discovery
accounts
settings
diagnostics
logs
publication/management surfaces
```

according to installed Role.

Where development capability exists, Launcher may expose:

```text id="i0ybkr"
Enter Development
```

into Vapor SDK / Development Mode.

Launcher is a frontend projection over shared Vapor semantics.

---

# Vapor SDK / Development Mode

The **Vapor SDK** is the integrated first-party development environment.

Conceptually:

```text id="f9p7jv"
Vapor Launcher
    ↓ Enter Development
Vapor SDK / Development Mode
    ↑ Return
Vapor Launcher
```

The exact process/window architecture is implementation-defined.

The semantic requirement is:

> **Development Mode is a development-oriented projection/superset of the relevant Client capabilities, not a competing Vapor implementation.**

The SDK may expose:

* canonical Superworkspace navigation;
* Authorities;
* Source Repo Containers;
* Source Repos / Vapor Workspaces;
* Projects;
* source editing;
* Content authoring;
* build/test/run;
* Git state;
* Cargo realization;
* diagnostics;
* logs;
* operation recipes;
* publication;
* first-party development operations.

---

# Vapor CLI

The **Vapor CLI** is the command-line projection over shared Vapor Core semantics.

The universal:

```text id="alfo3b"
vapor
```

binary exposes the primary semantic CLI surface.

Application-specific executables may expose narrower surfaces where appropriate.

CLI semantics must not become a second independent architecture beside the GUI.

---

# Vapor Core

**Vapor Core** is the shared semantic/orchestration implementation layer.

It should own reusable behavior such as:

```text id="3cew2q"
identity resolution
Context resolution
Selection resolution
source topology
Registry interaction
source acquisition
toolchain management
Cargo integration
operation legality
operation recipes
diagnostics
build/test orchestration
publication orchestration
first-party facility resolution
```

Frontends call into this shared model.

They do not independently redefine it.

---

# Steam App

The outer consumer distribution product is currently:

```text id="s0q8cv"
Steam App: Loo Cast
```

The Steam App delivers the Vapor Client experience together with the default first-party Loo Cast Vapor App.

The Steam App is not synonymous with:

```text id="d7ujpl"
Vapor Client
```

and is not synonymous with:

```text id="fhm86j"
Loo Cast Vapor App
```

It is the outer Steam product/distribution boundary containing or bootstrapping both.

---

# Steam App Instance

A **Steam App Instance** is one concrete local installation of the Steam App.

The ordinary model assumes:

> **One Steam installation of Loo Cast = one Steam App Instance.**

Steam owns the depot-managed installation root.

Conceptually:

```text id="sfq3f4"
Steam App: Loo Cast
└── Steam App Instance
    ├── Vapor Client binaries/bootstrap
    ├── Vapor Installer
    ├── default first-party Vapor App
    └── other Steam/depot-owned application files
```

Mutable Vapor state may refer to the Steam App Instance without physically living inside its depot root.

---

# Steam App Instance Is Not the Entire Local Vapor Universe

This distinction is fundamental.

The local Vapor environment contains at least three different storage/lifetime domains:

```text id="vvixpv"
Steam App Instance

Vapor User Data

Vapor Superworkspace
```

These must not be collapsed into one filesystem tree merely because they belong to one user's Vapor installation.

---

# Steam App Instance Storage

The Steam App Instance primarily owns files whose lifecycle follows Steam installation/depot management.

Examples include:

```text id="myguhq"
Vapor binaries
bootstrap files
shipped resources
default depot-delivered Vapor App
other replaceable product files
```

Steam may replace/remove these during:

```text id="xum5mx"
update
verify files
uninstall
reinstall
```

Therefore unique authored development source must not live here merely for convenience.

---

# Vapor User Data

**Vapor User Data** is mutable Vapor-owned state associated with the user/local Vapor environment.

It may include:

```text id="oiy4kk"
installed Role state
configured canonical Superworkspace location
managed toolchain metadata/state
persistent Context
resume/frontend state
Vapor App metadata
local indexes
generated realization
IDE integration
diagnostic state
caches
operation realization
provider/account integration state where appropriate
```

Exact operating-system paths remain implementation-specific.

The conceptual boundary is more important:

> **Vapor User Data survives ordinary Steam application replacement where appropriate.**

---

# Canonical Superworkspace

The **Vapor Superworkspace** is the single canonical local source-development root.

It:

* contains authored development source;
* has no Vapor identity;
* is not part of the Steam App Instance;
* is not automatically disposable application data;
* may contain multiple Authorities;
* is configured/referenced through local Vapor state;
* persists independently from ordinary Steam reinstall.

Conceptually:

```text id="bifdx0"
<Superworkspace>/
└── GHF-Studios/
    ├── Vapor-Client/
    ├── Vapor-Platform-Server/
    └── Loo-Cast/
```

The full source semantics are owned by the Context/Identity and Development Experience models.

---

# Storage Ownership Summary

The intended ownership split is:

```text id="8p186f"
Steam App Instance
    replaceable product/distribution state

Vapor User Data
    mutable Vapor-managed user/environment state

Superworkspace
    authored development source
```

This separation gives Vapor clear uninstall, reinstall, repair, and recovery semantics.

---

# Steam Reinstall

Steam reinstall may recreate:

```text id="jr643l"
Vapor binaries
Steam/depot state
bootstrap files
default shipped artifacts
```

It must not automatically delete:

```text id="wwpzzv"
canonical Superworkspace
authored Git source
unique local work
persistent Vapor User Data that is not intentionally reset
```

After reinstall, Vapor should be able to reconnect to existing User Data/Superworkspace configuration and regenerate derived state.

---

# Role Downgrade

Role downgrade may remove or disable:

```text id="my7cgv"
development tooling
higher-capability surfaces
managed toolchain components where appropriate
```

It must not interpret:

```text id="1afj3k"
Content Developer → Player
```

as permission to delete:

```text id="0k30qz"
Git repositories
Projects
authored manifests
scripts
source code
local commits
```

Capability and authored source have independent lifetimes.

---

# Local Vapor Apps

Multiple built Vapor Apps may coexist locally.

Their physical artifact storage may be Vapor-managed outside or alongside Steam-depot state according to the installation model.

Their lifecycle is distinct from the canonical development source which produced them.

Therefore:

```text id="k8infs"
delete/remove Vapor App
```

must not imply:

```text id="5c63wl"
delete source Project
```

and:

```text id="f0eycl"
delete source
```

does not automatically imply that an already-installed valid Vapor App ceases to run.

---

# Packagepack

A **Packagepack** is the complete authored composition Content Project.

A valid Packagepack resolves to:

* exactly one effective Engine;
* exactly one effective Game;
* applicable Engine Mods;
* applicable Game Mods;
* applicable Extension Mods;
* required Libraries;
* subordinate packs/dependencies;
* all other required selected Content.

A Packagepack is already the complete composition artifact.

Vapor should not invent a second authored concept named:

```text id="didgnt"
finished composition
```

after Packagepack.

---

# Vapor App Composition

A **Vapor App Composition** is the exact effective resolved Content graph represented by one Packagepack under a particular resolution/version context.

Conceptually:

```text id="yuo406"
Packagepack
    ↓ resolve
Vapor App Composition
```

This is semantic/resolved composition state.

It is not yet necessarily a built executable artifact.

---

# Vapor App

A **Vapor App** is a built/deployable/runnable realization of a Vapor App Composition for a supported target.

Conceptually:

```text id="y8uue7"
Packagepack
    ↓ resolve
Vapor App Composition
    ↓ build
Vapor App
```

Different target builds may realize the same Packagepack composition.

---

# Vapor App vs Vapor Client

These are fundamentally different.

```text id="gv9cb2"
Vapor Client
    manages/runs/develops Vapor Apps

Vapor App
    one built runnable Packagepack realization
```

Therefore:

```text id="m12m8e"
Vapor Client
≠ Vapor App
```

The distinction becomes especially important with multiplayer/server hosting.

---

# Vapor App Runtime

A **Vapor App Runtime** is an executing realization of a Vapor App Composition.

Its actual runtime semantics derive from:

```text id="o4cvre"
effective Engine
effective Game
effective Mods
explicit authored contracts
```

Vapor orchestrates/hosts runtime realization.

Vapor does not universally prescribe all application runtime behavior.

---

# Runtime Roles

A Vapor App Runtime may operate in different roles.

Primary future networked roles include:

```text id="5ss2rw"
App Client Runtime
App Server Runtime
```

These terms describe semantic runtime responsibility.

They do not necessarily correspond one-to-one with:

```text id="pvg8fm"
machines
processes
containers
executables
```

---

# App Client Runtime

An **App Client Runtime** is the client-side runtime role of a running Vapor App.

It normally runs under/inside the Vapor Client.

Possible responsibilities include:

```text id="yui473"
presentation
input
client-side simulation
prediction
interpolation
local ECS/world state
network communication with App Server Runtime
```

Exact responsibilities are Engine/Game-defined.

Vapor does not impose one universal client runtime architecture.

---

# Vapor Platform

The **Vapor Platform** is Vapor's ecosystem-level service/control domain.

It deals primarily with coordination around Vapor Apps/source/content rather than implementing one particular App's simulation.

Potential responsibilities include:

* Registry;
* identity;
* authentication;
* authorization;
* source/provider linkage;
* publication;
* distribution metadata;
* diagnostics;
* documentation services;
* discovery;
* ecosystem administration;
* future session discovery;
* future matchmaking;
* future entitlement/access decisions;
* future relay/broker coordination where appropriate.

---

# Vapor Platform Server

The **Vapor Platform Server** is the server-side infrastructure implementing the Vapor Platform.

Its target trusted first-party Source Repo Container identity is:

```text id="n0uw6m"
GHF-Studios/Vapor-Platform-Server
```

historically migrated from:

```text id="ft30m8"
GHF-Studios/Vapor-Server-Root
```

The public first-party facility:

```text id="yl7sm5"
Platform Server
```

is stable semantic functionality.

Its current backing source topology may evolve.

---

# Platform Server Is a Logical Deployment Domain

Vapor Platform Server is not required to be:

```text id="gh1otx"
one executable
one service
one process
one container
one machine
one provider
```

It may contain independent Platform services such as:

```text id="0lwuu3"
Homepage
Documentation
Identity
Diagnostics
Registry
future Matchmaking
future Session Directory
...
```

A current deployment may place many services on one VPS.

A future deployment may distribute them across machines/regions/providers without changing the semantic term:

```text id="8lxtbf"
Vapor Platform Server
```

---

# Platform Service

A **Platform Service** is one independently meaningful service within the Platform Server domain.

Examples include:

```text id="eogbby"
Registry
Identity
Diagnostics
Documentation
Homepage
```

A service may own:

* service-specific persistence;
* service-specific business logic;
* service-specific API;
* service-specific deployment/runtime behavior.

Its source may be an independently meaningful Source Repo.

---

# Platform Server Orchestration

The Platform Server first-party facility / Source Repo Container owns whole-Platform orchestration.

Possible responsibilities include:

```text id="03mpav"
service membership
deployment topology
reverse proxy configuration
service-manager configuration
cross-service contracts
whole-platform build/test
coordinated health checks
smoke checks
deployment
recovery
state export/import orchestration
operational documentation
```

Individual Platform services own their service-specific semantics.

The distinction is:

```text id="tn1u7o"
Platform Server Container/facility
    whole-platform orchestration

Platform service Source Repo
    service implementation
```

---

# Platform Client

A **Platform Client** is a logical software capability used to communicate with Vapor Platform Server services.

Potential users include:

```text id="q31jne"
Vapor Client
Vapor App Server
administrative tooling
automation
other authorized infrastructure
```

Common concerns may include:

* Registry requests;
* authentication/session credentials;
* publication;
* authorization;
* diagnostics;
* matchmaking/session registration;
* Platform protocol compatibility.

Platform Client does not need to be one universal physical library.

Shared implementation should be factored according to implementation pressure.

---

# Vapor App Server

A **Vapor App Server** is a future server-side Vapor product/environment capable of hosting server-side Vapor App runtimes.

Its reserved future first-party name is:

```text id="jp32o2"
Vapor-App-Server
```

Do not create the Source Repo Container/product merely to make the topology visually complete.

Create it when server-side App runtime implementation requires it.

---

# Vapor App Server Responsibilities

A future Vapor App Server may:

* acquire/receive compatible App compositions/artifacts;
* resolve server-hostable configuration;
* build/load/start App Server Runtimes;
* register server/session availability with the Vapor Platform;
* authenticate/authorize Platform operations;
* expose lifecycle/health information;
* coordinate process/resource lifecycle;
* host one or more App sessions;
* communicate with App Client Runtimes.

Exact installation/distribution remains open.

Potential future mechanisms include:

```text id="ujpr15"
dedicated-server Steam distribution
direct deployment
containers
service deployment
other infrastructure
```

---

# App Server Runtime

An **App Server Runtime** is the server-side runtime role of a running Vapor App.

Possible responsibilities include:

```text id="mznn08"
authoritative simulation
shared world state
session/player state
server-side ECS/world execution
replication
validation
persistence hooks
App-defined networking
```

These are examples.

The effective Engine/Game defines the actual runtime architecture.

---

# Runtime Role Is Not Machine Identity

Client/server runtime roles do not automatically imply separate physical hosts.

A future listen-server arrangement may run:

```text id="qmljhr"
Vapor Client
├── App Client Runtime
└── App Server Runtime
```

on one machine and potentially within one process family.

A dedicated server may run:

```text id="qgt0u4"
Vapor App Server
└── App Server Runtime
```

without an App Client Runtime.

The semantic role remains useful independent of placement.

---

# Platform Communication

**Platform communication** is communication with ecosystem/control services.

Examples:

```text id="cpi4gc"
identity
authentication
authorization
Registry
publication
diagnostics
discovery
matchmaking
session registration
```

Platform communication is conceptually separate from one running App's application-specific session traffic.

The distinction is analogous to a control plane where useful.

The analogy is descriptive rather than normative.

---

# App-Session Communication

**App-session communication** belongs to one running networked Vapor App/session.

Examples may include:

```text id="96zu5j"
input commands
simulation state
replication
gameplay events
entity state
session-specific messages
voice/data channels
```

These semantics belong primarily to the effective Engine/Game/runtime architecture.

---

# No Universal App Networking Model

Vapor may provide common hosting, transport, discovery, session, or lifecycle facilities.

It must not assume every Vapor App uses one universal networking model.

Possible App-defined architectures include:

```text id="m0780p"
client/server
peer-to-peer
listen server
dedicated server
lockstep
rollback
state replication
event replication
authoritative server
custom transport
offline/no networking
```

The governing principle is:

> **Vapor hosts, composes, and coordinates App runtimes; the effective Engine/Game defines what the App runtime actually means.**

---

# Platform / App Boundary

The Vapor Platform may help establish an App session without becoming that App session.

A future flow may resemble:

```text id="q6165f"
Vapor Client
    ↓ authenticate / discover
Vapor Platform Server
    ↓ match / authorize / resolve endpoint
Vapor App Server
    ↓ establish session
App Client Runtime ⇄ App Server Runtime
```

After session establishment, high-frequency App traffic should not need to pass through unrelated Platform services such as Registry merely because the Platform helped create the session.

Exceptions may exist for future relay/security/network infrastructure.

---

# Platform Server vs App Server

These terms must remain distinct.

```text id="fnyh9r"
Vapor Platform Server
    ecosystem/control infrastructure

Vapor App Server
    host/environment for running Vapor Apps
```

A Platform Server may discover or authorize an App Server.

That does not make the Platform Server the App Server.

An App Server may authenticate to the Platform.

That does not make it Platform infrastructure.

---

# First-Party Facility Model

Stable first-party product/facility concepts may include:

```text id="g4da60"
Client
Platform Server
Examples
future App Server
```

Each may have trusted backing source relationships.

Conceptually:

```text id="ry1zyq"
first-party facility
    stable semantic subject

facility binding
    trusted linkage to current source realization
```

This allows public Vapor semantics to remain stable while repository topology evolves.

---

# First-Party Trust

A Source Repo Container does not become an official Vapor facility merely by declaring itself so.

First-party trust must come from trusted Vapor Registry / Root Authority state.

Therefore third-party source cannot self-declare:

```text id="809f6r"
facility = client
```

and acquire:

```text id="93h99g"
vapor client ...
```

semantics.

First-party product facilities are protected Vapor concepts.

---

# Source Facility vs Runtime Product

A source facility and runtime product are related but distinct.

For example:

```text id="1wr6sa"
GHF-Studios/Vapor-Client
    first-party source family

Vapor Client
    runtime/product environment
```

The Source Repo Container exists to develop/build the product.

The product does not cease to exist if the repository topology later changes.

Likewise:

```text id="d9ukp8"
GHF-Studios/Vapor-Platform-Server
    source/deployment family

Vapor Platform Server
    logical running Platform infrastructure
```

---

# Product Topology vs Source Topology

Do not conflate:

```text id="5o8pp3"
product/runtime hierarchy
```

with:

```text id="xucf3s"
Authority
→ Source Repo Container
→ Source Repo
→ Project
```

Product boundaries may survive source consolidation.

Source boundaries may exist because of deployment ownership without becoming separate consumer products.

Examples:

```text id="ahtbkc"
Installer
    product/application boundary
    may share Client Source Repo

Registry service
    service/runtime boundary
    may retain independent Source Repo
```

---

# Product Topology vs Process Topology

Likewise:

```text id="3rbxcx"
product
≠ process
≠ executable
≠ Source Repo
```

One product may use several executables/processes.

One Source Repo may produce several executables.

One logical Platform Server may span many processes.

Vapor should model each layer according to its real responsibility.

---

# Local Runtime and Source Independence

A runnable installed Vapor App does not require its source to remain locally available.

Conceptually:

```text id="glfxus"
installed Vapor App:
    present / runnable

source:
    absent
```

may be valid.

Likewise:

```text id="cnq3z0"
source:
    dirty/newer

installed Vapor App:
    older but still runnable
```

may be valid.

Runtime/install/source state are independent dimensions.

---

# Build Failure Does Not Destroy Runtime State

A development rebuild may fail while a previous Vapor App remains valid.

For example:

```text id="isj2aa"
source
    modified

new build attempt
    failed

previous installed Vapor App
    valid

runtime
    launchable
```

The Client should preserve these distinctions.

A failed build is not permission to destroy a previous usable artifact.

---

# Installation, Build, and Source Are Distinct

The local Vapor environment may simultaneously contain:

```text id="h1cbbx"
authored source
built artifacts
installed Vapor Apps
running Vapor Apps
```

These have different ownership/lifetimes.

Therefore:

```text id="h17l17"
build
install
run
remove App
acquire source
remove source
```

must remain distinct operations.

---

# Development Toolchain Relationship

Higher Vapor Roles may install/manage a controlled development toolchain.

This toolchain is part of the **Vapor Client development capability**, but its mutable state does not have to live physically inside the Steam depot.

Root/Ecosystem development should strongly prefer the Vapor-managed pinned/vendored toolchain.

The Client owns the integration semantics.

The storage location belongs to the User Data / managed tooling model.

---

# Recovery Boundary

The product/storage split enables a clean recovery model.

Conceptually:

```text id="uk7oky"
Steam App Instance lost/replaced
    ↓
reinstall Vapor Client binaries

Vapor User Data survives or is reconstructed
    ↓
reconnect configured Superworkspace

registered missing source reacquired where necessary
    ↓
derived state regenerated

build/test/runtime restored
```

Unique local authored source must not be treated as automatically recoverable.

---

# Runtime User Data

Running Vapor Apps may themselves generate:

```text id="i2whq6"
saves
settings
cache
logs
user-created data
session state
```

Those data should have an explicit ownership/lifetime model.

They should not be conflated automatically with:

```text id="cz927w"
Vapor User Data
developer Superworkspace
Steam depot files
```

The exact runtime-data layout may remain App/Engine-defined where appropriate.

---

# Default First-Party Vapor App

The Steam App ships a default first-party Loo Cast Vapor App.

This provides the ordinary Player experience without requiring:

```text id="za1ws1"
Git
Rust/Cargo
source acquisition
Steam Workshop availability
local compilation
```

The Player can install the Steam App and play.

The default composition still participates in normal Vapor Content/App semantics even though its built artifact is delivered through the Steam depot.

---

# Additional Vapor Apps

Additional published Vapor Apps may be discovered/acquired through Vapor's built-distribution model.

Current intended backend:

```text id="rbuyzk"
Steam Workshop
```

The Vapor Client resolves and manages those App installations.

Players should primarily deal with Vapor semantic identities rather than Workshop numeric IDs.

---

# Naming Rules

Preferred names:

```text id="k4j70l"
Vapor Client
Vapor Platform
Vapor Platform Server
Platform Service
Platform Client
Vapor App
Vapor App Composition
Vapor App Runtime
App Client Runtime
Vapor App Server
App Server Runtime
```

Avoid vague product/domain use of:

```text id="gsjr17"
Root
```

The word remains valid where it has a precise independent meaning:

```text id="c8yhl9"
filesystem root
Installation root
Superworkspace root
Root Authority
```

---

# Repository Naming Direction

The target top-level first-party Source Repo Container identities are:

```text id="x3qlhw"
GHF-Studios/Vapor-Client

GHF-Studios/Vapor-Platform-Server
```

with future:

```text id="ivpmdl"
GHF-Studios/Vapor-App-Server
```

only when required.

Historical provider/source names:

```text id="am1e8d"
Vapor-Root
Vapor-Server-Root
```

are migration inputs.

Their exact provider/identity/local-path migration sequence is owned by the repository migration documents.

---

# Non-Goals

This model does not freeze:

* exact Vapor App Server distribution;
* exact dedicated-server Steam topology;
* exact multiplayer composition format;
* whether client/server runtime roles use identical or distinct built App artifacts;
* exact transport;
* replication architecture;
* matchmaking implementation;
* relay/NAT strategy;
* Platform scaling model;
* exact Platform service decomposition;
* exact Vapor User Data filesystem paths;
* exact runtime save/data directory model;
* exact long-term placement of managed toolchain files.

These should be resolved from implementation pressure.

---

# Core Invariants

* Vapor Client is the complete user-side Vapor product/environment.
* Vapor Client is broader than any one frontend or App Client Runtime.
* Vapor Core owns shared Vapor semantics.
* Launcher, SDK, CLI, Installer, and Entrypoint do not independently redefine Vapor.
* Source repository count is not determined by executable count.
* Product topology, source topology, process topology, and storage topology are distinct layers.
* Steam App is the outer consumer product/distribution boundary.
* Steam App Instance primarily owns replaceable Steam/depot state.
* Vapor User Data is distinct from the Steam App Instance.
* The canonical Superworkspace is distinct from both Steam App Instance and Vapor User Data.
* Authored development source must not be implicitly owned by Steam uninstall/reinstall.
* Role downgrade must not silently delete authored source.
* Vapor Client may communicate with Platform services and App Server Runtimes.
* Vapor Platform Server owns ecosystem/control infrastructure.
* Vapor Platform Server may consist of many services/processes/hosts.
* Platform services may retain independent Source Repos where their boundaries are meaningful.
* Vapor App Server is distinct from Vapor Platform Server.
* App Client Runtime and App Server Runtime describe runtime roles, not necessarily machines.
* Platform communication and App-session communication are distinct.
* Vapor does not impose one universal App networking model.
* Effective Engine/Game semantics remain authoritative for App-specific runtime behavior.
* First-party facility semantics are trusted Vapor state, not self-declared third-party source.
* Client and Platform Server facility semantics may remain stable while backing source topology evolves.
* Installed Vapor App state, source state, build state, and runtime state are distinct.
* A failed rebuild must not automatically destroy a previous valid Vapor App.
* `Root` is not used as vague product/source-family terminology.
* `Vapor-App-Server` remains future-only until implementation pressure requires it.

---

# Open Questions

The following remain intentionally open:

* Exact Vapor User Data filesystem layout.
* Exact managed toolchain physical storage location.
* Exact runtime save/config/log storage ownership.
* Exact local additional Vapor App artifact storage.
* Exact App Server installation/distribution model.
* Exact App Server first-party Source Repo Container topology.
* Exact future Platform service decomposition.
* Exact first-party facility-binding Registry schema.
* Exact first-party Platform Client shared-library decomposition.
* Exact process architecture of Launcher/SDK/Installer/Entrypoint after Client source consolidation.
* Exact multiplayer/runtime-role artifact relationship.
