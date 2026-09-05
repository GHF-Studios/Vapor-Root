
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

# Repository Hierarchy

The current source hierarchy is:

> **Vapor Superworkspace**
> → **Container Repo**
> → **Source Repo / Vapor Workspace**
> → **Vapor Project**

---

## Vapor Superworkspace

A Vapor Superworkspace is a local checkout container.

It is not itself a Git repository or primary source-bearing unit, as in:

> Losing the Superworkspace primarily risks local unpushed/uncommitted state inside the repositories it contains rather than destroying the canonical remote source model itself.

A Superworkspace may contain multiple related Container Repos or other Vapor-managed development checkouts.

The exact rules for multi-Container layouts remain open.

---

## Container Repo

A Container Repo is a Vapor-managed Git repository.

It groups related Vapor Workspaces using Git submodules.

Git therefore manages both:

* Container Repos.
* Vapor Workspaces.

A Container Repo is not itself intended to be a submodule of another Container Repo.

Its purpose is organization and coordinated checkout/versioning of related source repositories.

---

## Source Repo / Vapor Workspace

A Vapor Workspace is the primary source-bearing Git repository.

It exists as a submodule of a Container Repo.

A Vapor Workspace contains one or more Vapor Projects.

It does not itself contain nested Git submodules.

---

## Vapor Project

A Vapor Project is a Rust/Cargo workspace contained inside a Vapor Workspace.

It is not itself a Git repository.

A project models some coherent Vapor development artifact or area.

---

# Workspace Types

## Vapor Root Workspace

The unique client/root Vapor Workspace.

It contains Vapor Root Projects representing parts of the local/client Vapor ecosystem.

---

## Vapor Server Root Workspace

The unique server-root Vapor Workspace.

It contains Vapor Server Root Projects representing server-side Vapor infrastructure.

---

## Vapor Content Workspace

A non-unique Workspace containing Vapor Content Projects.

`Loo-Cast` is the first-party example.

---

## Vapor Content Project

A Vapor Content Project models one Vapor Content artifact:

* Packagepack.
* Enginepack.
* Gamepack.
* Modpack.
* Engine.
* Game.
* Engine Mod.
* Game Mod.
* Extension Mod.

Exactly how one Project maps onto crates/packages/generated code remains an implementation concern.

The semantic relationship should remain explicit regardless of Cargo layout.

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

Therefore Vapor must not silently:

* Reset repositories destructively.
* Delete dirty repositories.
* Replace local source with remote state.
* Drop unpushed commits.
* Recreate a Superworkspace as if every contained checkout were disposable.

Automation should become more conservative as destructive potential increases.

---

# Git vs Git Hosting

Git capability does not imply GitHub authentication.

Local authoring/building/testing should remain possible without GitHub.

A Git host becomes relevant when an operation requires remote hosting.

Current GitHub-related operations may include:

* Clone private repository.
* Create repository.
* Push.
* Pull protected content.
* Open issue.
* Create collaboration flow.
* Access official Vapor repositories.

Other Git hosts may eventually participate.

The domain model should avoid unnecessarily hardcoding GitHub where the concept is really Git hosting.

---

# Source Acquisition

Composer-and-above users acquire source through Vapor-compatible Git repositories.

The ideal experience is not:

> Search the web manually for repository URLs.

Instead Vapor should be capable of resolving semantic Vapor identities into appropriate source locations through Registry/source metadata.

A normal acquisition operation may therefore conceptually include:

* Resolve Vapor identity.
* Determine source repository.
* Reuse existing compatible checkout if present.
* Clone/fetch required repository.
* Initialize required Container/Workspace relationships.
* Validate Vapor compatibility.
* Register local availability.

The exact checkout/version-resolution rules remain open.

---

# Opening Existing Development State

Vapor should support opening development state from several realistic starting points:

* Existing registered Superworkspace.
* Existing Container Repo.
* Existing Vapor Workspace.
* Existing Vapor Project.
* Existing Git checkout not yet known to Vapor.
* Freshly cloned source.
* Newly created source.

The UX should detect and explain what Vapor recognizes rather than requiring the user to reconstruct metadata manually.

---

# Creating Development State

When creating new authored content, Vapor should automate the structural boilerplate required by the chosen artifact type.

For example, creating a new Game Mod should ideally establish:

* Appropriate Vapor Project structure.
* Required manifest/configuration.
* Repository/Workspace placement.
* Cargo structure where applicable.
* Appropriate target/dependency declarations.
* Initial test/composition integration hooks.

The developer should make explicit semantic choices.

Vapor should make mechanical choices where there is a canonical answer.

---

# Vapor SDK

The Vapor SDK is the Content Developer-oriented development surface within the Launcher.

Its job is not necessarily to replace IntelliJ IDEA, RustRover, VS Code, or other full IDEs.

Its job is to provide the Vapor-specific development environment.

This may include:

* Project/content browser.
* Vapor artifact metadata.
* Structured configuration.
* Dependency/composition views.
* Build controls.
* Run/test controls.
* Diagnostics.
* Logs.
* Content validation.
* Test-composition selection.
* Generated source/config visibility.
* External IDE integration.
* Publication entry points.

---

# External IDE Integration

The likely division of responsibility is:

**Vapor SDK:**

* Understands Vapor semantic structure.
* Owns Vapor-specific configuration.
* Coordinates build/run/test.
* Knows which Project models which Vapor Content.
* Knows composition/test context.
* Provides Vapor diagnostics.
* Can open/reveal source in external tools.

**External IDE:**

* General Rust editing.
* Refactoring.
* Code navigation.
* Language-server features.
* General debugger/editor capabilities.
* General Git UI if the developer prefers it.

This is not yet a final boundary.

Vapor may grow stronger editing capabilities where they materially improve the Vapor-specific workflow.

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