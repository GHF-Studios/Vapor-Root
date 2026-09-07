> [!info]
> This document defines the primary runtime/product topology of Vapor across the user-side Vapor Client, the online Vapor Platform, and future server-hosted Vapor App runtimes.
>
> It establishes the canonical meanings of **Client**, **Platform Server**, **App Client Runtime**, **App Server Runtime**, **Platform communication**, and **App-session communication**.
>
> Repository migration and physical source-layout changes are specified separately.
>
> Vapor Content composition remains defined by the existing Content, operational, and publishing models.

---

# Core Topology

Vapor has three major runtime/product domains:

```text
Vapor Client

Vapor Platform Server

Vapor App Server
```

They are not symmetrical binaries.

They represent different responsibilities.

Conceptually:

```text
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

The Vapor Client communicates with both:

* the Vapor Platform;
* running Vapor App server-side runtimes where applicable.

A Vapor App Server may likewise communicate with both:

* the Vapor Platform;
* connected App Client Runtimes.

---

# Vapor Client

The **Vapor Client** is the user-side Vapor software/product environment.

It is the Vapor software delivered to and operated on a user's machine through the Steam App Instance.

Its responsibilities may include:

* Vapor Launcher;
* Vapor SDK / Development Mode;
* Vapor CLI;
* Vapor Core;
* Vapor Entrypoint/bootstrap behavior;
* local Installation state;
* managed toolchain integration;
* Content discovery/acquisition;
* source/development workflows;
* build/test/run;
* publication workflows;
* local Vapor App management;
* execution of client-side Vapor App runtimes;
* communication with Vapor Platform services;
* communication with remote App Server Runtimes.

The Vapor Client is therefore broader than an **App Client Runtime**.

It is also a Platform client.

For this reason, the complete user-side product should not be called `Vapor-App-Client`.

---

# Vapor Client Source Domain

The source family currently represented by `Vapor-Root` is conceptually the **Vapor Client source domain**.

Its intended Container Repo name is:

```text
Vapor-Client
```

The Container Repo may contain several Workspaces during migration.

Long term, implementation should increasingly reflect the principle:

> **Binaries do not own Vapor semantics. Vapor Core owns semantics; binaries and graphical surfaces project those semantics.**

Historical standalone repositories such as Launcher, SDK, Installer, Entrypoint, or Shell may therefore be consolidated, archived, retained, or repurposed according to the later repository-migration design.

The name `Vapor-Client` describes the user-side source/product family.

It does not imply that every contained binary is itself a network client.

In particular, the Vapor Installer remains a distinct application/capability-management boundary even when its source and distribution belong to the Vapor Client product family.

---

# Vapor Platform

The **Vapor Platform** is Vapor's ecosystem-level service and control domain.

It deals primarily with questions about the Vapor ecosystem surrounding running Apps.

Examples include:

* identity;
* authentication;
* authorization;
* Content Registry;
* source/provider linkage;
* publication;
* distribution metadata;
* documentation publication;
* diagnostics;
* ecosystem administration;
* discovery;
* future multiplayer server/session discovery;
* future matchmaking;
* future entitlement or access decisions where required;
* future relay/broker coordination where appropriate.

The Platform is not the runtime implementation of a particular Engine/Game composition.

---

# Vapor Platform Server

The **Vapor Platform Server** is the server-side infrastructure implementing the Vapor Platform.

The intended Container Repo name is:

```text
Vapor-Platform-Server
```

This is the successor concept to `Vapor-Server-Root`.

The Platform Server is a logical server-side product/deployment domain.

It is **not required to be one process, one machine, or one executable**.

It may contain independently deployable services such as:

```text
Homepage
Documentation
Identity
Diagnostics
Registry
future Matchmaking
future Session Directory
future other Platform services
```

A single-VPS deployment may host several of these services initially.

A future production deployment may distribute them across multiple processes, containers, machines, regions, or providers without changing the semantic term **Vapor Platform Server**.

---

# Platform Server Orchestration

The `Vapor-Platform-Server` Container Repo owns coordination of the Platform Server source/deployment family.

It may own:

* service membership;
* deployment topology;
* reverse-proxy configuration;
* service-manager configuration;
* coordinated health checks;
* smoke checks;
* deployment;
* recovery;
* whole-platform state export/import orchestration;
* cross-service contracts;
* operational documentation.

Individual service Workspaces remain responsible for their service-specific business logic.

Therefore:

> **The Platform Server Container Repo owns platform-wide composition and operations; individual Platform services own their implementation semantics.**

---

# Platform Client

A **Platform Client** is a logical software capability for communicating with the Vapor Platform Server.

It is not necessarily an independently shipped application or repository.

Platform Client capability may be used by:

```text
Vapor Client
Vapor App Server
administrative tooling
automation
other authorized Vapor infrastructure
```

Common Platform Client concerns may include:

* identity/session credentials;
* Registry requests;
* publication;
* authorization;
* diagnostics;
* matchmaking/session registration;
* Platform protocol/version compatibility.

Reusable Platform Client implementation may eventually live in appropriate shared Libraries/crates.

The topology model does not require one universal physical library if implementation pressure suggests otherwise.

---

# Vapor App Runtime

A **Vapor App Runtime** is an executing realization of a Vapor App Composition.

The runtime semantics ultimately derive from the effective Engine/Game/Mods and their explicitly authored contracts.

A runtime may operate in different roles.

The primary future networked roles are:

```text
App Client Runtime
App Server Runtime
```

These roles describe runtime responsibility.

They do not necessarily describe physical machine boundaries.

---

# App Client Runtime

An **App Client Runtime** is the client-side runtime role of a running Vapor App.

It normally executes inside or under control of the Vapor Client.

It may own behavior such as:

* local presentation;
* input;
* client simulation;
* prediction;
* interpolation;
* local ECS/world state;
* network communication with an App Server Runtime.

Exact responsibilities are defined by the effective Engine/Game architecture.

Vapor does not universally prescribe them.

---

# Vapor App Server

A **Vapor App Server** is a future server-side Vapor product/environment capable of hosting server-side Vapor App runtimes.

The intended future Container Repo/product name is:

```text
Vapor-App-Server
```

A Vapor App Server may:

* acquire or receive appropriate App compositions/source/artifacts;
* resolve server-hostable App configuration;
* build/load/start an App Server Runtime;
* register its availability with the Vapor Platform;
* authenticate/authorize Platform operations;
* expose health/lifecycle information;
* host one or more App sessions depending on the App/runtime model;
* coordinate process/resource lifecycle;
* communicate with connected App Client Runtimes.

The exact distribution/install/deployment model for Vapor App Server remains open.

It may eventually involve dedicated-server Steam distribution, direct deployment, containers, or other infrastructure.

---

# App Server Runtime

An **App Server Runtime** is the server-side runtime role of a running Vapor App.

It may provide behavior such as:

* authoritative simulation;
* shared world state;
* session/player state;
* server-side ECS/world execution;
* App-defined networking;
* replication;
* validation;
* persistence hooks.

These are examples rather than universal Vapor requirements.

The effective Engine/Game defines the actual runtime semantics.

---

# Runtime Role Is Not Machine Identity

Client/server runtime roles must not be equated automatically with physical hosts.

For example, a future listen-server arrangement may run:

```text
Vapor Client
├── App Client Runtime
└── App Server Runtime
```

on one machine/process family.

A dedicated server may instead run:

```text
Vapor App Server
└── App Server Runtime
```

without a local App Client Runtime.

The semantic role remains useful independent of physical deployment.

---

# Platform Communication

**Platform communication** is communication with Vapor ecosystem/control services.

Examples include:

```text
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

Platform communication is conceptually separate from the real-time/application-specific traffic of one running Vapor App session.

This distinction is sometimes analogous to a **control plane**.

The analogy is useful but should not force every future service into networking-industry terminology where it does not fit.

---

# App-Session Communication

**App-session communication** is communication belonging to one running networked Vapor App/session.

Examples may include:

```text
input commands
simulation state
replication
gameplay events
entity updates
voice/data channels
session-specific messages
```

These semantics belong primarily to the App's effective Engine/Game/runtime architecture.

This distinction is sometimes analogous to a **data plane**.

Again, the analogy is descriptive rather than a requirement that all Apps implement one universal Vapor wire protocol.

---

# Vapor Does Not Define Universal App Networking Semantics

Vapor may define common hosting and lifecycle contracts for App networking.

It must not assume that every Vapor App uses one universal runtime networking model.

The governing principle is:

> **Vapor hosts and composes networked App runtimes; the participating Engine/Game defines what its runtime networking means.**

This may include App-defined choices such as:

* client/server;
* peer-to-peer;
* listen server;
* dedicated server;
* lockstep;
* state replication;
* event replication;
* rollback;
* authoritative server;
* custom transport;
* no networking at all.

Vapor may provide reusable Libraries or Platform services which Apps can choose to use.

Those facilities do not automatically become mandatory runtime semantics.

---

# Platform/App Boundary

The Vapor Platform may help establish an App session without becoming the App session itself.

A future flow may resemble:

```text
Vapor Client
    ↓ authenticate
Vapor Platform Server
    ↓ discover/match/authorize
Vapor App Server
    ↓ connect
App Client Runtime ⇄ App Server Runtime
```

After establishment, high-frequency App-session communication should not need to pass through unrelated Platform services such as Registry or Identity.

The exact architecture may vary for relays, NAT traversal, anti-abuse infrastructure, or other future requirements.

---

# Platform Server and App Server Are Distinct

The terms must not be conflated.

```text
Vapor Platform Server
    ecosystem/control infrastructure

Vapor App Server
    host for running networked Vapor Apps
```

A Platform Server may help users find an App Server.

It does not thereby become the App Server.

An App Server may authenticate/register with the Platform Server.

It does not thereby become Platform infrastructure.

---

# Steam App Relationship

The current Steam App remains the outer distribution/product boundary for the consumer Vapor experience.

Conceptually:

```text
Steam App: Loo Cast
└── Steam App Instance
    ├── Vapor Client
    ├── Vapor Installer
    ├── default first-party Vapor App
    ├── additional local Vapor Apps
    └── managed Vapor state/tooling
```

This does not mean the Vapor Client and one particular Vapor App are the same thing.

The Vapor Client manages and runs Vapor Apps.

---

# Vapor App vs Vapor Client

A **Vapor App** is one built runnable realization of a Packagepack composition.

The **Vapor Client** is the broader user-side platform/environment which manages and runs Apps.

Therefore:

```text
Vapor Client
≠ Vapor App
```

and:

```text
App Client Runtime
⊂ runtime behavior associated with a running Vapor App
```

The distinction becomes particularly important once multiplayer/server execution exists.

---

# Naming Rules

The following names are preferred:

```text
Vapor Client
Vapor Platform
Vapor Platform Server
Vapor App
Vapor App Runtime
App Client Runtime
Vapor App Server
App Server Runtime
Platform Client
Platform Service
```

Avoid using `Root` as a product/domain name.

`Root` remains appropriate where it has a precise independent meaning, for example:

```text
filesystem root
Installation root
source root
Root Authority
```

It should not be used as a vague synonym for:

```text
main repository
client product
server product
top-level architecture
```

---

# Repository Naming Direction

The intended high-level migration is:

```text
Vapor-Root
    → Vapor-Client

Vapor-Server-Root
    → Vapor-Platform-Server
```

and, if/when required:

```text
NEW:
Vapor-App-Server
```

These are semantic repository migrations rather than blind string replacements.

The repository migration must account for:

* GitHub repository names;
* Container Repo identity/address;
* submodule URLs;
* local checkout names;
* manifests;
* deployment configuration;
* CI;
* scripts;
* docs;
* diagrams;
* Vapor-managed remembered state;
* provider linkage;
* code/type/module terminology.

The detailed migration sequence is defined separately.

---

# Non-Goals of This Naming Pass

This model does not yet freeze:

* exact App Server distribution;
* exact multiplayer composition format;
* whether client/server App runtimes use one or two Packagepacks;
* dedicated-server Steam App/Tool topology;
* networking transport;
* replication model;
* matchmaking implementation;
* session-directory implementation;
* relay/NAT architecture;
* App Server scaling model;
* exact Platform service decomposition.

Those should be decided from implementation pressure.

---

# Core Invariants

* Vapor Client is the complete user-side Vapor product/environment, not merely one App runtime.
* Vapor Client may communicate with both Platform services and App Server runtimes.
* Vapor Platform Server owns server-side ecosystem/control infrastructure.
* Vapor Platform Server may consist of many services/processes/hosts.
* Vapor App Server is conceptually separate from Vapor Platform Server.
* Vapor App Server hosts server-side Vapor App runtimes.
* Runtime Client/Server roles are not necessarily physical-machine roles.
* Platform communication and App-session communication are different domains.
* Vapor may provide shared networking/platform facilities without imposing one universal App networking model.
* Effective Engine/Game semantics remain authoritative for App-specific runtime behavior.
* `Root` is not used as a vague product/repository taxonomy term.
* `Vapor-Root` is intended to migrate to `Vapor-Client`.
* `Vapor-Server-Root` is intended to migrate to `Vapor-Platform-Server`.
* `Vapor-App-Server` is reserved for future App-hosting server infrastructure.
