> [!info]
> This document defines the semantic command model shared by Vapor's command-line surfaces.
>
> It describes command hierarchy, application projections, role visibility, and the relationship between CLI operations and Vapor Content kinds.
>
> Exact command leaves may evolve as implementation pressure reveals better semantics.

---

# Core Principle

The Vapor CLI models Vapor concepts rather than Cargo structure, repository layout, implementation modules, or arbitrary utility groupings.

A command namespace must represent a meaningful Vapor domain or first-class entity.

Namespaces must not exist merely to classify or cosmetically scope an operation.

The CLI should remain explicit, flattened, and predictable.

---

# Shared Core and CLI Surfaces

Vapor applications expose different projections of the same underlying Vapor Core operations.

The intended command-line binaries are:

* `vapor`
* `vapor-installer`
* `vapor-launcher`
* `vapor-sdk`

`vapor` is the broad universal CLI projection.

The dedicated binaries expose subsets appropriate to their application and installed Role.

For example, an Installer Role operation exposed as:

```text
vapor role status
```

is the same underlying operation exposed by:

```text
vapor-installer role status
```

Application binaries must not independently reimplement Vapor semantics.

The logical Vapor SDK may be integrated into the Launcher while also having a dedicated `vapor-sdk` executable/CLI projection.

Those are different surfaces over the same Vapor Core rather than independent SDK implementations.

---

# Top-Level Purity

Top-level namespaces should correspond to stable Vapor domains or first-class Vapor entity kinds.

Current system-oriented namespaces include:

```text
installation
role
authority
toolchain
source
ecosystem
```

Current Vapor Content namespaces are explicit:

```text
packagepack
enginepack
gamepack
modpack
engine
game
engine-mod
game-mod
extension-mod
library
```

There is intentionally no generic `content` namespace merely to mirror the Vapor Content taxonomy.

Likewise, ordinary workflows should not require the user to jump between separate `composition` and `app` namespaces merely because a Packagepack moves through resolution, build, installation, and runtime states.

Packagepack, Vapor App Composition, and Vapor App remain distinct model terms where that distinction matters.

`library` here means the first-class Vapor Content kind.

It is distinct from the Launcher/local **Content Library** surface.

---

# Content Command Model

Each Vapor Content kind exposes the operations semantically applicable to that kind.

The current command shape is:

```text
packagepack
    create
    list
    inspect
    resolve
    verify
    build
    test
    install
    select
    run
    remove
    publish

enginepack
    create
    list
    inspect
    resolve
    verify
    test
    publish

gamepack
    create
    list
    inspect
    resolve
    verify
    test
    publish

modpack
    create
    list
    inspect
    resolve
    verify
    test
    publish

engine
    create
    list
    inspect
    verify
    test
    publish

game
    create
    list
    inspect
    verify
    test
    publish

engine-mod
    create
    list
    inspect
    verify
    test
    publish

game-mod
    create
    list
    inspect
    verify
    test
    publish

extension-mod
    create
    list
    inspect
    verify
    test
    publish

library
    create
    list
    inspect
    resolve
    verify
    test
    publish
```

This tree is semantic rather than mechanically uniform.

An operation should be present wherever it makes sense rather than being artificially segregated by Content kind.

For packs, `test` runs the applicable tests contributed by the resolved Content graph together.

For a Library, `resolve` exposes its semantic dependency graph and the Rust/Cargo realization information relevant to that graph.

This does not mean Cargo defines Vapor dependency semantics.

---

# Packagepack Lifecycle

Packagepack is the complete-composition Vapor Content kind.

A Packagepack may therefore support complete-composition operations that subordinate Content kinds do not independently support, including:

* Build.
* Install.
* Select.
* Run.
* Remove.

A Packagepack remains the authored composition identity throughout these workflows.

A resolved Vapor App Composition and built Vapor App are precise derived states or realizations of that Packagepack rather than mandatory separate CLI namespaces.

---

# Resolution Model

Resolution is implemented as a generic Vapor Content graph operation.

Vapor semantic resolution is authoritative for:

* Vapor IDs.
* Vapor versions.
* Dependency bindings.
* Vapor Content kinds.
* Vapor-level dependency relationships.

CLI exposure remains explicit per Content kind where a standalone resolution operation is useful.

Examples include:

```text
vapor packagepack resolve ...
vapor enginepack resolve ...
vapor gamepack resolve ...
vapor modpack resolve ...
vapor library resolve ...
```

These commands use the same underlying semantic resolver rather than separate kind-specific dependency solvers.

Resolution recursively follows dependencies declared by Vapor Content.

Binding names do not determine Content semantics.

Resolved identities, kinds, and authored relationships do.

A Packagepack may obtain its effective Engine either directly or transitively through an Enginepack.

Likewise, it may obtain its effective Game directly or through a Gamepack.

Nested packs, Mods, Libraries, and other dependencies are resolved transitively.

A valid Packagepack ultimately yields exactly one effective Engine and exactly one effective Game together with all other required Content.

Packagepack-specific validation is layered on top of generic graph resolution.

---

# Vapor Resolution vs Cargo Resolution

Rust-backed Vapor Content introduces a second, distinct graph:

```text
Vapor semantic graph
        ↓
Rust/Cargo realization
        ↓
Cargo package graph
```

Vapor is authoritative for the semantic Vapor graph.

Cargo is authoritative for the physical Rust package graph it actually resolves and compiles.

Vapor should use Cargo-native mechanisms, including `cargo metadata` where useful, to:

* inspect Cargo workspace/package structure;
* identify physical Rust packages and targets;
* observe Cargo dependency declarations;
* observe Cargo's resolved package graph;
* verify that Vapor semantic dependencies are physically realized;
* diagnose divergence between Vapor and Cargo state.

Cargo Package IDs and similar identifiers are Cargo-domain physical identifiers.

They must not become replacements for Vapor semantic identity.

---

# Cargo Reconciliation

Vapor should supervise the Cargo entries that physically realize Vapor semantic dependencies without taking ownership of unrelated Cargo configuration.

Conceptually:

```text
Vapor.toml
    semantic dependency intent
        ↓
Vapor resolver
    exact desired Vapor graph
        ↓
Cargo reconciliation
        ↓
editable Cargo.toml
        ↓
Cargo resolution / metadata
        ↓
actual physical graph
        ↓
Vapor verification
```

A developer may continue to edit ordinary Cargo configuration directly.

Vapor should distinguish at least:

```text
valid
stale
conflicting
explicitly locally overridden
repairable
```

Verification should diagnose disagreement.

Repair may reconcile Vapor-managed physical dependency entries.

Explicit local/unlocked overrides may be valid development state but must not silently become publishable state.

---

# Creation and Templates

Creation is a first-class operation for authored Vapor entities.

Examples include:

```text
vapor packagepack create ...
vapor engine create ...
vapor game-mod create ...
vapor library create ...
```

Creation should establish canonical structural boilerplate for the selected Vapor entity.

This may include:

* Vapor manifests.
* Source/project structure.
* Cargo structure where applicable.
* Dependency declarations.
* Initial tests.
* Appropriate repository/workspace placement.

Creation may support templates.

The initial implementation may use built-in canonical templates.

More general versioned or externally supplied template systems should be introduced only when concrete pressure requires them.

The same creation model may eventually extend to Vapor ecosystem/root source structures and remote repository creation.

---

# Role and Authority

Role and authorization are separate concepts.

A Vapor Role describes the kinds of work for which the local Steam App Instance is equipped.

The installed Role progression is:

```text
Player
→ Composer
→ Content Developer
→ Ecosystem Developer
```

Ecosystem Developer is locally attainable.

An Ecosystem Developer may acquire, fork, create, modify, build, and test Vapor ecosystem source without official Vapor authorization.

Authorization determines whether a particular operation may affect a particular protected external target.

Examples include:

* Pushing to official GHF Studios repositories.
* Creating repositories in protected organizations.
* Publishing into official namespaces.
* Deploying official Steam branches or depots.
* Modifying production registry/server infrastructure.

The same operation may therefore be locally available while a particular target remains unauthorized.

Protected operations should be visibly distinguishable rather than conceptually hidden.

---

# Root Authority

Root Authority is not an installed development Role above Ecosystem Developer.

It is an authority state granting ultimate administrative and ownership authority over protected official Vapor ecosystem resources.

A Root Authority normally operates with Ecosystem Developer Role plus Root Authority authorization.

---

# Role-Based Surface Exposure

Underlying Vapor Core operations may exist even when a particular user-facing surface does not expose them.

Installed Role influences which operations and tooling are presented.

For example:

* Player surfaces focus on consuming and running finished Vapor Apps.
* Composer surfaces expose pack composition workflows.
* Content Developer surfaces expose Engine/Game/Mod/Library creation and development.
* Ecosystem Developer surfaces expose Vapor ecosystem source development.

Role-based visibility is distinct from authorization.

An operation visible to an Ecosystem Developer may still reject a protected target for lack of authority.

---

# System-Oriented Namespaces

The current system namespaces are approximately:

```text
installation
    status
    diagnose
    repair

role
    status
    promote
    demote

authority
    status

toolchain
    status
    install
    diagnose
    repair

source
    status
    list
    acquire
    fork

ecosystem
    status
    acquire
    fork
    create
    build
    test
    publish
    deploy
```

The exact `ecosystem` / root-source terminology remains open.

`publish` / `deploy` semantics also remain subject to implementation pressure.

---

# Context Discovery

Vapor should discover obvious source and development context automatically.

Users should not normally need to repeatedly provide implementation-level source roots or Cargo paths.

Explicit selectors and path overrides should remain available when context is ambiguous or the user intentionally targets another source context.

For Rust-backed Content, context discovery may use Cargo-native project inspection rather than inventing parallel project-layout detection unnecessarily.

---

# CLI Invariants

* CLI structure does not mirror Rust module structure.
* CLI structure does not mirror Cargo workspace structure.
* CLI structure does not automatically mirror the Vapor type hierarchy.
* Vapor Content kinds are explicit first-class CLI namespaces.
* A generic `content` namespace is not used merely as a taxonomy bucket.
* `library` means the Vapor Content kind; Content Library is a separate local/Launcher concept.
* Packagepack workflows do not require ordinary users to manually switch between Packagepack, Composition, and Vapor App namespaces.
* Shared semantic operations are implemented once in Vapor Core.
* Dedicated Vapor binaries expose projections of those shared operations.
* Packs may aggregate testing across their resolved Content.
* Vapor dependency resolution is generic, recursive, and transitive.
* Packagepack validation is layered above generic resolution.
* Packagepacks may obtain Engine/Game through Enginepack/Gamepack.
* Cargo metadata may describe physical realization but does not define Vapor semantic identity.
* Role controls installed tooling/workflow exposure.
* Authorization controls protected operations against protected targets.
* Root Authority is an authority rather than an installed Role.
* User-authored source remains outside the disposable Steam App Instance by default.

---

# Open CLI Questions

* `verify` versus `validate` terminology.
* Exact `publish` versus `deploy` boundary.
* Exact `ecosystem` / root-source namespace terminology.
* Whether `source` requires a general `create` operation.
* Exact Packagepack install/select/remove lifecycle terminology.
* Exact template-selection syntax.
* Exact selectors and context-override syntax.
* Whether standalone `resolve` should eventually be exposed for every dependency-bearing Content kind.
* Exact CLI presentation of Cargo-reconciliation and local-override state.
