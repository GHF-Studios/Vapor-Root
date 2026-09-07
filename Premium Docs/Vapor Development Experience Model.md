
> [!info]
> This document defines the source-authoring and development experience for Vapor.
>
> It covers Composer, Content Developer, Ecosystem Developer, Git/repository structure, Vapor Superworkspaces, Workspaces, Projects, SDK integration, and build/run/test iteration.
>
> Publication itself is specified separately in the **Vapor Publishing and Distribution Model**.

---

# Development Capability Layers

Development capability grows cumulatively.

> Player
> → Composer
> → Content Developer
> → Ecosystem Developer

These are locally installed Vapor Roles.

A Role describes the kinds of work for which the local Vapor environment is equipped.

External authority is separate.

A user may therefore become an Ecosystem Developer, acquire or fork Vapor ecosystem source, modify it, and build/test it locally without permission from the official Vapor ecosystem.

Specific operations against protected official resources may additionally require authentication and authorization.

Root Authority is an authority state rather than another installed development Role.

The development model begins meaningfully at Composer capability because composition authoring requires source and local builds.

---

# Composer Development

A Composer authors composition artifacts without authoring behavioral implementation.

A Composer may create and modify:

* Packagepacks.
* Enginepacks.
* Gamepacks.
* Modpacks.

A Composer may acquire and consume:

* Engines.
* Games.
* Engine Mods.
* Game Mods.
* Extension Mods.

A Composer may inspect behavioral source where useful.

Editing that behavioral source as authored Vapor Content requires Content Developer capability.

The primary Composer loop is:

> Acquire source
> → compose
> → resolve
> → build
> → run/test
> → revise
> → optionally publish

This is a conceptual loop rather than an exhaustive serialized workflow.

---

# Content Development

A Content Developer additionally authors:

* Engine.
* Game.
* Engine Mod.
* Game Mod.
* Extension Mod.

Content development involves changing behavior rather than merely changing composition.

The core loop is approximately:

> Open/create source
> → edit/configure
> → integrate into a Packagepack context
> → resolve/build
> → run/test
> → inspect diagnostics/runtime behavior
> → revise

Because Vapor compositions are static, behavioral development is ultimately tested through complete composition builds.

Incremental compilation should make this practical without changing the logical complete-build model.

---

# Ecosystem Development

An Ecosystem Developer develops Vapor itself.

This may include:

* Installer.
* Launcher.
* SDK.
* CLI.
* Vapor Root framework.
* Registry/server components.
* Distribution tooling.
* Build orchestration.
* Internal development tooling.

Ecosystem Developer capability is locally attainable and does not inherently imply official Vapor authority.

An Ecosystem Developer may:

* Acquire official ecosystem source where publicly available.
* Fork ecosystem source.
* Create independent ecosystem source.
* Modify Vapor locally.
* Build and test Vapor locally.
* Develop and test publication, deployment, authentication, and authorization machinery locally.

Operations against protected official targets may separately require authorization.

Examples include:

* Pushing to official repositories.
* Creating repositories in protected organizations.
* Publishing into official namespaces.
* Deploying official Steam branches or depots.
* Modifying production server or Registry infrastructure.

The intended eventual official-development loop is:

> Open ecosystem source context
> → modify
> → build/test locally
> → authenticate/authorize protected operation
> → commit/push
> → deploy to the appropriate development environment
> → validate integrated behavior

Vapor should progressively automate this loop without making local ecosystem development depend on official authorization.

---

# Repository and Development-Context Hierarchy

The structural development hierarchy is:

> **Vapor Superworkspace**
> → **Container Repo**
> → **Source Repo / Vapor Workspace**
> → **Vapor Project**

Vapor Content may then be authored inside Projects.

This hierarchy combines two different concerns:

```text
local development containment
    Superworkspace

canonical source/development structure
    Container Repo
    → Workspace
    → Project
```

The detailed identity, realization, Open/Focus, session, selector, and ambiguity rules are defined in the **Vapor Context, Identity, Session And Selection Model**.

---

## Vapor Superworkspace

A Vapor Superworkspace is primarily a local SDK/development container.

Its purpose is comparable to an IDE workspace containing several related repositories and Projects.

It is not:

* a Git repository;
* a canonical remote source identity;
* part of canonical Workspace or Project identity;
* required to understand every filesystem object around it.

A Superworkspace may contain:

* one or more Container Repos;
* related independently checked-out Vapor Workspaces;
* multiple local realizations of source needed for development;
* unrelated filesystem entries which Vapor does not own.

Creating equivalent checkouts in a second local container creates a second Superworkspace.

The source identities inside it may remain the same.

---

## Implicit and Configured Superworkspaces

A Superworkspace may be recognized implicitly from the Vapor development structures it contains.

A `Superworkspace.vapor.toml` may optionally provide durable local SDK configuration.

Such configuration may contain information such as:

* friendly display name;
* attached local roots;
* local aliases;
* presentation/layout preferences;
* recovery hints;
* other local development preferences.

It must not become canonical truth for:

* Workspace identity;
* Project identity;
* Content identity;
* Container Repo membership;
* Git history;
* Registry state;
* publication state;
* provider authority.

Likewise, Superworkspace-local mechanisms such as `.vaporignore` affect the local SDK/development view rather than canonical source topology.

A Superworkspace should therefore remain useful even when its optional local configuration is missing or regenerated.

---

## Container Repo

A Container Repo is a Vapor-managed Git repository grouping related Source Repos / Vapor Workspaces.

Container Repos normally coordinate these Workspaces using Git submodules.

A Container Repo:

* participates in canonical structural identity;
* is Git-managed;
* is not itself intended to be a submodule of another Container Repo;
* expresses meaningful source organization rather than arbitrary filesystem nesting.

For example:

```text
GHF-Studios/Vapor-Root
```

may identify the canonical `Vapor-Root` Container Repo.

---

## Source Repo / Vapor Workspace

A Source Repo and Vapor Workspace are the same primary source-bearing unit viewed from Git and Vapor respectively.

The normative relationship is:

> **One Source Repo = one Vapor Workspace.**

A Vapor Workspace:

* is a Git repository;
* belongs structurally to a Container Repo;
* contains one or more Vapor Projects;
* owns Workspace-level authored metadata;
* may have multiple local physical realizations/checkouts.

A provider URL identifies external source linkage.

It does not replace Vapor structural identity.

---

## Vapor Project

A Vapor Project is a coherent development unit inside a Vapor Workspace.

For Rust-backed development, it normally corresponds to a Cargo workspace or another coherent Cargo execution context.

A Project:

* is not itself a Git repository;
* is namespaced by its Workspace;
* should have a meaningful Project name;
* may contain several Cargo packages;
* may host zero, one, or multiple Vapor Content artifacts;
* may contain supporting packages which are not independently Vapor Content.

Project identity is therefore distinct from Vapor Content identity.

The Project answers:

> **Where and within which development structure is this work authored and built?**

A Content identity answers:

> **Which semantic Vapor artifact is this?**

The relationship may be one-to-one for some Projects.

It is not required to be one-to-one generally.

---

# Canonical Structural Identity

Structural development identities are human-readable, case-preserving, slash-separated, and hierarchically namespaced.

Conceptually:

```text
Namespace/Container-Repo/Workspace/Project
```

Examples:

```text
GHF-Studios/Vapor-Root
GHF-Studios/Vapor-Root/Vapor
GHF-Studios/Vapor-Root/Vapor/Client
```

The Superworkspace is intentionally absent because it is a local realization container.

Each identity segment should represent real semantic distinction.

Duplication such as:

```text
GHF-Studios/Vapor-Root/Vapor/Vapor
```

is legal only if the final `Vapor` is genuinely the meaningful Project name.

Current rewrite-era Project names may be renamed or decomposed as architecture pressure reveals more meaningful boundaries.

---

# Structural Identity vs Local Realization

Canonical identity does not uniquely determine one physical checkout.

A Workspace may have multiple local realizations, such as:

```text
GHF-Studios/Vapor-Root/Vapor

├── official checkout / main
└── developer fork / context-redesign
```

Filesystem paths, Git branches, provider repositories, and fork relationships describe realizations.

They are not substitutes for Vapor semantic identity.

This distinction allows Vapor to support:

* upstream and fork checkouts;
* experimental branches;
* isolated test realizations;
* relocation;
* multiple development environments.

When an operation changes or builds source, Vapor must ultimately resolve an exact local realization.

The GUI should normally make this a modeled choice rather than requiring raw filesystem paths.

---

# Workspace Types

## Vapor Root Workspace

The unique client/root Vapor Workspace.

It contains one or more meaningful Vapor Root Projects representing coherent parts of the local/client Vapor ecosystem.

Project decomposition should reflect real architectural responsibility rather than duplicating Workspace naming without reason.

---

## Vapor Server Root Workspace

The unique server-root Vapor Workspace.

It contains Vapor Server Root Projects representing coherent server-side Vapor infrastructure.

---

## Vapor Content Workspace

A non-unique Workspace used for Vapor Content development.

A Content Workspace may contain one or more Projects.

---

## Vapor Content Project

A Vapor Content Project is a Project primarily concerned with authoring Vapor Content.

It may host one or more Content artifacts such as:

* Packagepack;
* Enginepack;
* Gamepack;
* Modpack;
* Engine;
* Game;
* Engine Mod;
* Game Mod;
* Extension Mod;
* Library.

One Project is not required to equal exactly one Content artifact.

The semantic relationship between Project, Cargo packages, and Content must remain explicit even when several artifacts share one Project.

---

# Git Model

Git is fundamental Composer-and-above infrastructure.

Vapor should automate Git without pretending Git does not exist.

Vapor may provide operations such as:

* Clone.
* Fetch.
* Pull.
* Commit.
* Push.
* Branch selection.
* Repository creation.
* Submodule initialization/update.
* Status.
* Diff.
* Conflict detection.

Advanced users must still be able to access normal Git directly.

---

# Git Safety

User-authored source is not disposable.

Vapor must assume that:

* Uncommitted changes may exist.
* Unpushed commits may exist.
* Detached/local branches may exist.
* External Git tools may modify repositories.
* The user may intentionally diverge from the state Vapor expected.
* Filesystem paths may change outside Vapor.

Therefore Vapor must not silently:

* Reset repositories destructively.
* Delete dirty repositories.
* Replace local source with remote state.
* Drop unpushed commits.
* Recreate a Superworkspace as though every checkout were disposable.
* Attach an arbitrary similarly named checkout in place of a missing realization.

Automation should become more conservative as destructive potential increases.

---

# Git vs Git Hosting

Git capability does not imply GitHub authentication.

Local authoring/building/testing should remain possible without GitHub or another hosting provider.

A Git host becomes relevant when an operation requires external hosting.

Provider-oriented operations may include:

* Clone private repository.
* Create repository.
* Push.
* Access protected source.
* Open collaboration workflows.
* Resolve fork/upstream relationships.
* Access official Vapor repositories.

The domain model should avoid hardcoding GitHub where the concept is really Git hosting.

Provider ownership and Vapor semantic identity must remain distinguishable.

---

# Source Acquisition

Composer-and-above users acquire source through Vapor-compatible Git repositories.

The ideal experience is not:

> Search for URLs manually, clone them to arbitrary directories, and repeatedly tell Vapor where they ended up.

Instead Vapor should be capable of resolving semantic identity into appropriate source/provider information.

A source-acquisition operation may conceptually include:

* Resolve Vapor identity.
* Determine source/provider linkage.
* Determine appropriate Container Repo / Workspace relationship.
* Reuse a compatible known realization where appropriate.
* Clone/fetch required source.
* Initialize submodules where required.
* Validate Vapor compatibility.
* Register local realization/availability.
* Expose the result in the current Superworkspace or SDK session.

Exact checkout/version/fork rules remain subject to provider and publication design.

---

# Opening Existing Development State

Opening development state is a session/context operation.

Vapor should support starting from:

* configured Superworkspace;
* implicit Superworkspace;
* Container Repo;
* Vapor Workspace;
* Vapor Project;
* known local realization;
* existing Git checkout not yet known to Vapor;
* freshly acquired source;
* newly created source.

The user should not need to reconstruct structural metadata manually.

When a deep exact target is opened, Vapor may automatically establish its exact ancestors.

For example, opening:

```text
GHF-Studios/Vapor-Root/Vapor/Client
```

may establish the relevant Workspace and Container context.

This is safe because the parent relationship is canonical structure rather than analogy.

---

# Open, Focus, and Selection

Development context is not represented by one global active Project.

Several Workspaces, Projects, or Content targets may be Open simultaneously.

Several may also be Focused.

These concepts are distinct:

```text
Open
    participates in the working set

Focused
    contributes durable targeting intent

Selected
    transient target for the immediate frontend action
```

A GUI click should therefore not automatically rewrite persisted Focus.

Likewise, multiple Focus is legitimate even when a particular operation ultimately requires exactly one target.

The operation's cardinality decides whether the candidate set is valid.

---

# Missing and Broken Development State

Known/Open/Focused objects may become unavailable or unhealthy.

Examples include:

* moved checkout;
* disconnected drive;
* deleted directory;
* incompatible Workspace metadata;
* invalid manifest;
* broken Git state;
* missing dependency.

Availability/health is independent of Open/Focus state.

A previously focused Workspace may therefore remain:

```text
Known
Missing
Open
Focused
```

so that the SDK can explain the problem and offer recovery.

Useful recovery actions may include:

```text
Locate
Repair
Reacquire
Close
Forget
```

A broken object should not simply disappear from the user's mental workspace.

---

# Closing Development State

Close is a session operation.

Closing a parent context closes its Open descendants.

For example, closing a Workspace may also close Projects inside it.

The frontend should communicate that cascade.

Close must remain distinct from stronger effects such as:

```text
Forget
Remove checkout
Delete authored source
Uninstall capability
```

Closing development context must never silently destroy authored Git source.

---

# Creating Development State

When creating new authored work, Vapor should automate structural boilerplate while asking the developer for semantic choices.

Creating Content may involve:

* selecting or creating an appropriate Workspace/Project;
* generating required Vapor manifests;
* establishing Cargo structure where appropriate;
* creating initial source/tests;
* declaring dependencies/targets;
* registering the new Content in the local development environment.

Creation should operate through semantic identity and modeled placement rather than requiring users to hand-build repository layouts.

---

# Development Session and Resume State

A live frontend uses a Development Session containing Open/Focused state and other live context.

The SDK has such a session.

A future Vapor Shell may have one.

A one-shot CLI invocation may construct an ephemeral one.

Durable Resume State exists to reconstruct a useful future session.

For example, reopening the SDK may restore:

* previous Superworkspace;
* Open Workspaces;
* Open Projects;
* exact Focus;
* useful SDK navigation state.

Resume State is not necessarily one globally shared mutable cursor used by every simultaneous frontend.

Exact multi-process synchronization remains an implementation concern.

---

# Vapor SDK

The Vapor SDK is the primary first-party graphical development environment for Vapor.

It is not merely a configuration companion beside a separate IDE.

The preferred product model is:

```text
Vapor Launcher
    ↓ Enter Development
Vapor SDK
    ↑ Return to Launcher
```

SDK Mode is a development-oriented superset of the ordinary Launcher experience.

It may expose:

* Superworkspace Explorer;
* Workspace/Project/Content navigation;
* source;
* structured configuration;
* semantic dependency/composition graphs;
* Inspector;
* Problems;
* Build/Test/Run;
* Git;
* toolchain;
* logs;
* publication/deployment;
* ecosystem development where Role permits.

Detailed graphical behavior is defined in the **Vapor SDK Experience Model**.

The SDK may still integrate external editors and IDEs.

That integration is complementary rather than proof that Vapor's own SDK must remain permanently shallow.

---

# SDK State Ownership

SDK state must distinguish:

## Authored state

Examples:

* source;
* Vapor manifests;
* Workspace metadata;
* Git history;
* explicit semantic configuration.

Authored state is not disposable.

## Durable local development configuration

Examples:

* configured Superworkspace metadata;
* aliases;
* SDK preferences;
* resume information.

This may be regenerated or reconfigured but should not be silently discarded without reason.

## Derived state

Examples:

* caches;
* indexes;
* generated views;
* diagnostics;
* Cargo metadata snapshots;
* temporary realization state.

Derived state should where practical be:

> discoverable
> → regeneratable
> → idempotently repairable

Derived state should normally live beneath explicit Vapor-managed storage boundaries.

---

# External IDE Integration

External IDE integration remains a Vapor SDK responsibility.

It is not fundamentally a Source-domain operation.

The division is increasingly:

**Vapor SDK / Core:**

* Understand canonical Vapor structure and identities.
* Track local realizations.
* Own Vapor-specific Open/Focus/session semantics.
* Coordinate semantic build/run/test.
* Understand which Projects host which Content.
* Provide diagnostics and repair.
* Manage the pinned Vapor toolchain relationship.
* Expose source to external tools.
* Reconcile supported IDE integration where a canonical mapping exists.

**External IDE:**

* May provide mature source editing.
* May provide refactoring/code navigation.
* May provide debugger/language tooling.
* May provide general Git UI.

The built-in SDK may progressively absorb more editing/IDE capability over time.

External IDE compatibility remains valuable even if the integrated SDK eventually becomes capable enough for most Vapor work.

IDE-specific integration should be created/reconciled only when present or selected.

Once configured, safe synchronization should be proactive.

Explicit diagnosis and repair remain available when synchronization fails.

Legacy, incompatible, missing, or ignored Workspaces may remain visible in the SDK's development model without automatically participating in builds or editor integration.

---

# Build Experience

The user should normally request a semantic Vapor build:

> Build this Packagepack.

Vapor then coordinates the underlying work.

A build may involve:

* Composition resolution.
* Required source availability checks.
* Cargo invocation.
* Generated code/configuration.
* Build caching.
* Target selection.
* Final composition artifact generation.
* Vapor App packaging.
* Local installation/registration.

The developer should be able to inspect:

* Underlying Cargo output.
* Build commands.
* Diagnostics.
* Artifact locations.
* Cache behavior where useful.

---

# Build Context during Development

Because behavioral content is not independently runnable as a complete Vapor App, development requires an effective Packagepack context.

A Game Mod, for example, must ultimately be exercised within a complete composition containing:

* An effective Engine.
* An effective Game.
* The Mod.
* Any required dependencies.

The SDK therefore needs some concept of a **test/run composition context**.

Exactly how that context is represented remains open.

Possible future forms include:

* Explicit Packagepack selection.
* Generated temporary Packagepack.
* Development-only composition overlay.
* Project-defined preferred test composition.

This must be designed carefully because it directly affects iteration speed and conceptual clarity.

---

# Run and Test Experience

A developer-oriented Run operation should generally:

* Determine intended Packagepack/test context.
* Resolve composition.
* Build if necessary.
* Install/register resulting local Vapor App if necessary.
* Launch it.
* Associate diagnostics/runtime information with the originating development context.

This does not imply every Run must blindly rebuild everything.

Build currency and incremental compilation should avoid unnecessary work.

---

# Diagnostics

Vapor should distinguish between:

* Vapor-level diagnostic meaning.
* Underlying tool output.

Examples:

* Invalid composition dependency.
* Missing required source.
* Cargo compilation failure.
* Git checkout conflict.
* Incompatible Engine/Game relationship.
* Missing required capability.
* Broken toolchain.
* Runtime failure.

The user should receive a concise Vapor-oriented explanation while retaining access to raw output.

---

# Local Source vs Installed Vapor Apps

Source and installed runnable output are different concerns.

A developer may simultaneously have:

* Dirty source.
* Failed latest build.
* Previous successful Vapor App installed.
* Previous successful Vapor App selected.

Vapor should not collapse these states.

The Operational Model defines the state relationship.

---

# Development-State Removal

Removing or downgrading development capability must distinguish:

* Tooling.
* Caches.
* Generated build state.
* User-authored Git repositories.

Tooling may be removable.

Caches may be disposable.

Authored source requires explicit user intent before destructive removal.

---

# Open Development Questions

* Exact Superworkspace creation and registration workflow.
* Whether multiple Container Repos commonly coexist in one Superworkspace.
* Exact Container Repo version/submodule policy.
* How Vapor IDs map to repository-relative project identities.
* How source versions are pinned in pack dependencies.
* How branch/commit selection appears to Composers.
* Exact project-generation structure.
* Test composition model.
* SDK/external IDE boundary.
* Debugger integration.
* Automated testing model.
* Hot-reload/runtime dynamicity boundaries.
* How generated source/config is represented.
* How Git conflicts are surfaced and repaired.
* How Vapor handles repositories modified externally.
* How local-only projects become publishable remote projects.
* Exact Ecosystem Developer live-deployment workflow.

---

# Development Invariants

* Git is used for both Container Repos and Vapor Workspaces.
* Vapor Projects are not themselves Git repositories.
* A Superworkspace is not itself the canonical source-bearing repository.
* Composer capability can author packs.
* Content Developer capability can author behavioral content.
* Local development does not inherently require GitHub authentication.
* Dirty source remains legitimate local development state.
* Vapor must not silently destroy uncommitted or unpushed source.
* Behavioral content is ultimately tested within a complete composition.
* A semantic Vapor build targets a complete Packagepack composition.
* Incremental compilation optimizes the static build model rather than replacing it.
* External IDE usage must remain compatible with Vapor's repository/source model.