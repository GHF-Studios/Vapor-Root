> [!info]
> **Owns:** stable product/facility topology: Vapor Client, Vapor Core, Launcher, SDK, CLI, Installer, Vapor Platform, Platform Server, Steam outer-product relationship, and trusted first-party facilities.
>
> **Uses:** Role/Auth, Installation/Local State, Source Identity, App Runtime.
>
> **Does not own:** repository/provider migration, source acquisition, runtime extension APIs, or frontend grammar.

---

# Core Thesis

Vapor is one product ecosystem with several semantic facilities and application/frontend boundaries.

Executable count, process count, Git repository count, and semantic product boundaries are not the same thing.

> **Vapor Core owns shared Vapor semantics; applications and frontends project those semantics.**

A Launcher, SDK, CLI, Installer, Entrypoint, or service may remain a meaningful product/application boundary without requiring an independent semantic implementation or Git repository.

---

# Vapor Client

The **Vapor Client** is the complete user-side Vapor product/environment.

It includes or coordinates responsibilities such as:

```text
Launcher
SDK / Development Mode
CLI
shared Vapor Core semantics
Installer integration
local Vapor App management
managed toolchain integration
source/development orchestration
Content discovery/composition
build/test/run
publication workflows
diagnostics
Platform communication
App Server communication where appropriate
```

The Client is broader than any one executable or frontend.

---

# Vapor Core

**Vapor Core** is the shared semantic/orchestration implementation layer.

It should increasingly own reusable behavior such as:

```text
identity resolution
Context / Selection resolution
source topology
Registry interaction
source acquisition
toolchain management
Cargo integration
operation legality
diagnostics
build/test orchestration
publication/distribution orchestration
first-party facility resolution
```

Frontends must not independently redefine these semantics.

---

# Launcher

The **Vapor Launcher** is the primary ordinary graphical surface.

It may expose, according to installed Role:

```text
Play
Vapor Apps
Content Library
composition
discovery
accounts
settings
diagnostics/logs
publication/management
```

Launcher is a frontend projection over shared Core semantics.

---

# SDK / Development Mode

The **Vapor SDK** is the integrated first-party development environment.

Conceptually:

```text
Launcher Mode
    ↕
SDK / Development Mode
```

Development Mode is a richer projection of the same Client/Core, not a competing Vapor implementation.

---

# CLI

The **Vapor CLI** is the command-line projection over shared Core semantics.

The universal `vapor` binary exposes the primary semantic CLI surface. Application-specific executables may expose narrower surfaces where appropriate.

---

# Installer

The **Vapor Installer** is the application/process boundary responsible for changing what the local Vapor environment is equipped to do.

It may:

```text
establish/change installed Role
provision managed tooling
repair installed capability
downgrade higher capability
participate in setup/recovery
```

Installer remains semantically distinct even if its implementation source is consolidated into the primary Client Source Repo.

---

# Vapor Platform

The **Vapor Platform** is the collection of online Vapor services which provide ecosystem-wide capabilities such as:

```text
identity
Registry
discovery
publication coordination
authorization
diagnostics
future matchmaking / service discovery
```

The **Vapor Platform Server** is the first-party server-side facility realizing those services.

Individual Platform services may remain independent Source Repos/services where their deployment or ownership boundary is meaningful.

---

# First-Party Facilities

Stable trusted first-party facilities may own bespoke operations because their lifecycle is inherently specific.

Current important examples:

```text
Client
Platform Server
Examples
```

A facility is trusted semantic state, not merely a repository-name convention.

Commands such as:

```text
client build
platform-server deploy
examples test
```

may remain stable while underlying source topology evolves.

---

# Product Boundary vs Source Boundary

Product topology must not mechanically mirror Git.

Examples:

```text
Installer product boundary
    may remain

Installer Source Repo boundary
    may disappear
```

while:

```text
Platform Registry service
    may remain an independent Source Repo
```

because its operational/deployment boundary is meaningful.

Source topology belongs elsewhere.

---

# Steam Outer Product

The current outer consumer product is:

```text
Steam App: Loo Cast
```

It delivers or bootstraps:

```text
Vapor Client
default first-party Loo Cast Vapor App
Vapor Installer
Steam/depot-owned product files
```

The Steam App is neither the Vapor Client nor the Loo Cast Vapor App.

---

# Steam Entry Points

Conceptual launch choices:

```text
Play Loo Cast
Start Vapor
Start Installer
```

`Play Loo Cast` enters the default first-party App without source/toolchain requirements.

`Start Vapor` opens ordinary Vapor management.

`Start Installer` opens capability/tooling establishment and repair.

---

# Vapor App vs Client

A **Vapor App** is one built runnable realization of a complete Packagepack-defined composition.

The **Vapor Client** manages, launches, acquires, develops, and otherwise interacts with Vapor Apps.

Therefore:

```text
Vapor Client
≠
Vapor App
```

---

# Platform Client Capability

The Vapor Client communicates with Vapor Platform services and therefore acts as a **Platform Client**.

`Platform Client` describes a communication capability.

`Vapor Client` describes the complete user-side product.

---

# Stability Rule

Stable product architecture should use semantic facility names:

```text
Vapor Client
Vapor Platform Server
Examples
```

Temporary provider repository names belong to Migration.

A GitHub rename must not churn the product model.
