# Vapor Ecosystem Experience Model

> [!info]
> This document defines the intended **User Experience (UX)** and **Developer Experience (DX)** of the Vapor Ecosystem.
>
> It sits between the **Ecosystem Model Glossary** and lower-level system specifications, architecture documents, and TDDs.
>
> The glossary defines **what exists**.
>
> This document defines **how those things fit together from the perspective of people using and developing Vapor**.
>
> It intentionally stays above detailed implementation mechanics. It describes the product model, major workflows, capability boundaries, ownership boundaries, and intended experience strongly enough to constrain later technical design without attempting to encode the entire ecosystem as prose.
>
> Unresolved areas are left explicit rather than being filled with speculative decisions.

---

# 1. Purpose

Vapor exists to make a complex Rust/Cargo/Git/Steam-based ecosystem behave like one coherent product.

The user should primarily interact with **Vapor concepts**, not with the incidental complexity of the tools used underneath them.

At a very high level:

> **Player:** consume complete compositions.

> **Composer:** assemble complete compositions from existing content.

> **Content Developer:** create the content that compositions are assembled from.

> **Ecosystem Developer:** create and maintain Vapor itself.

The underlying implementation may involve:

* Git.
* Git hosting services.
* Rust.
* Cargo.
* Steam.
* SteamCMD.
* Steam Workshop.
* Bevy.
* ECS infrastructure.
* Generated code.
* Build caches.
* Multiple repositories and workspaces.
* Vapor server infrastructure.

Those systems should remain accessible and inspectable where appropriate.

They should not dictate the normal mental model of the product.

The guiding principle is:

> **Expose the work the user actually intends to perform; automate the infrastructure required to make that work possible.**

---

# 2. Capability Model

Vapor uses a strict progressive capability hierarchy:

> **Player ⊂ Composer ⊂ Content Developer ⊂ Ecosystem Developer ⊂ Root Authority**

Each level contains the capabilities of every level below it.

These are therefore not unrelated personas.

A Content Developer is also a Composer and Player.

An Ecosystem Developer is also a Content Developer.

Root Authority contains every lower capability.

Capability upgrades should be experientially meaningful.

Moving from Player to Composer or from Composer to Content Developer should feel like Vapor has acquired a substantial new class of abilities rather than merely exposing another obscure settings checkbox.

---

# 3. Core Product and Composition Model

Several nearby concepts must remain clearly separated.

---

## 3.1 Steam App

There is one Steam-distributed product:

> **Loo Cast**

This is the **Steam App**.

The Steam App provides the product/installation boundary through which the local Vapor ecosystem is accessed.

It includes or provides access to:

* Vapor Installer.
* Vapor Launcher.
* The default first-party composition.
* The selected/runnable Vapor Apps.
* The wider Vapor tooling made available through capability upgrades.

The Steam App is not synonymous with one particular runtime composition.

---

## 3.2 Steam App Instance

A **Steam App Instance** is one concrete local installation of the Steam App.

It owns or references the local state needed to operate Vapor, including:

* Its Steam-managed root location.
* Installed Vapor infrastructure.
* Installed capability level.
* Locally available Vapor Apps.
* Relevant local/source/build state.
* One selected Vapor App Composition.

The current model does not require user-created parallel Steam App Instances.

The normal assumption is:

> One Steam installation of Loo Cast = one Steam App Instance.

Installation movement should use Steam-supported mechanisms.

The ecosystem should not be designed around arbitrary manual copying or relocation of the Steam App directory.

---

## 3.3 Packagepack

A **Packagepack** represents one complete Vapor composition.

It is not merely a fragment waiting to become some separately authored "finished composition."

A valid Packagepack already defines the complete content composition.

It must resolve to:

* Exactly one effective Engine.
* Exactly one effective Game.
* The applicable Engine Mods.
* The applicable Game Mods.
* The applicable Extension Mods.

Those constituents may be referenced directly or through subordinate packs.

The Packagepack is therefore the composition-level artifact and identity from which the runnable result is produced.

---

## 3.4 Vapor App Composition

A **Vapor App Composition** is the effective resolved composition represented by a Packagepack in the context in which it is selected and used by a Steam App Instance.

It represents the actual effective content graph after composition and dependency resolution.

The Steam App Instance has one selected Vapor App Composition for current/default use.

This is distinct from saying that only one Vapor App may exist locally.

Many built Vapor Apps may coexist.

---

## 3.5 Vapor App

A **Vapor App** is the built, deployable, runnable realization of a Vapor App Composition for a supported target.

Conceptually:

> **Packagepack**
> complete composition artifact
>
> ↓ resolve
>
> **Vapor App Composition**
> effective resolved composition
>
> ↓ build/deploy
>
> **Vapor App**
> runnable target-specific realization

These are not unrelated products.

They are different views/stages of the same complete composition.

---

# 4. Player

## 4.1 Purpose

A Player consumes finished Vapor Apps.

A Player wants to:

* Install Loo Cast through Steam.
* Play the default first-party composition.
* Discover other finished first-party or third-party Vapor Apps.
* Acquire them.
* Select between them.
* Launch them.
* Manage ordinary settings and account state.

A Player does not create or modify Vapor compositions.

---

## 4.2 Assumed Knowledge

Vapor may assume approximately:

* Basic computer literacy.
* Basic Steam literacy.
* Ordinary familiarity with installing and launching games.

Vapor should not assume:

* Programming experience.
* Git knowledge.
* Rust knowledge.
* Cargo knowledge.
* Repository/workspace knowledge.
* Build-system knowledge.

---

## 4.3 Player Tooling Boundary

A Player does not require:

* Git.
* Rust.
* Cargo.
* SteamCMD.
* Vapor SDK development tooling.
* Source checkouts.
* Development workspaces.

The Player receives built runnable Vapor Apps.

The Player does not rebuild compositions locally.

---

## 4.4 Player Discovery

The normal Player-facing discovery unit inside Vapor is a **complete Vapor App**.

Players do not need Vapor-level discovery of individual Engines, Games, Mods, or subordinate packs because they cannot compose them.

The underlying public source repositories may of course exist publicly outside the Player-facing Vapor workflow.

Vapor should present discovery according to what the current capability level can meaningfully do with the discovered object.

---

# 5. Composer / Content User

## 5.1 Purpose

A Composer is a Player with composition-authoring capability.

A Composer creates and modifies:

* Packagepacks.
* Enginepacks.
* Gamepacks.
* Modpacks.

A Composer uses existing:

* Engines.
* Games.
* Engine Mods.
* Game Mods.
* Extension Mods.

A Composer may not create or modify those behavioral content types themselves.

The fundamental boundary is:

> **Composer:** chooses and combines behavior.

> **Content Developer:** creates or changes behavior.

---

## 5.2 Composer Experience

The Composer experience should feel like working with the Vapor content/composition model rather than manually operating a Rust build system.

The Composer should work in terms of:

* Content.
* Packs.
* Dependencies.
* Compatibility.
* Composition.
* Packagepack validity.
* Build.
* Play.
* Publication.

The normal conceptual work is roughly:

> Acquire existing content source
> → create/modify packs
> → resolve a Packagepack
> → build the complete composition
> → run/test it
> → optionally publish it

This is a conceptual flow, not a complete workflow specification.

---

## 5.3 Composer Tooling

Composer capability requires more local infrastructure than Player capability because Packagepacks are statically built.

A Composer therefore requires at least:

* Git capability.
* Rust/Cargo toolchain.
* SteamCMD or other Steam tooling where relevant.

The Vapor Installer establishes and manages this capability.

Vapor should hide unnecessary command-level complexity during the normal Composer path.

---

## 5.4 Composer Publication

A Composer may publish the pack types they are allowed to author.

Most importantly, a Composer may publish a **Packagepack**, which already represents the complete composition.

There is no separate authored category called:

> "complete finished composition derived from a Packagepack."

The Packagepack is the complete composition artifact.

Publication then makes the source-side composition available through the Git-backed Vapor source model and makes the appropriate built Vapor App output available through the player-facing distribution model.

The exact publication transaction is intentionally not yet fixed.

---

# 6. Content Developer

## 6.1 Purpose

A Content Developer creates or modifies actual behavioral Vapor Content.

This includes:

* Engine.
* Game.
* Engine Mod.
* Game Mod.
* Extension Mod.

Engine Developer, Game Developer, and Mod Developer are not separate fundamental capability levels.

They are specializations of Content Development.

A Content Developer also inherits all Composer capabilities.

---

## 6.2 Intended Development Experience

Content development may involve:

* Programming.
* Configuration.
* Content creation.
* Dependency declaration.
* Build configuration.
* Testing.
* Runtime inspection.
* Debugging.

The intended experience should nevertheless avoid requiring the developer to manually construct and coordinate all underlying tooling.

Vapor should aim to provide:

* Strong project structure.
* Guardrails.
* Minimal boilerplate.
* Dependency setup.
* Integrated build/run operations.
* Useful diagnostics.
* Clear relationships between a project and the Vapor Content it models.
* Direct access to underlying tooling when useful.

The intended feeling is:

> **Develop the content, not the machinery required to persuade Rust, Cargo, Bevy, Git, Steam, and Vapor to cooperate.**

---

# 7. Ecosystem Developer and Root Authority

## 7.1 Ecosystem Developer

An Ecosystem Developer develops Vapor itself.

This includes areas such as:

* Vapor Installer.
* Vapor Launcher.
* Vapor SDK.
* Vapor CLI.
* Root/client framework code.
* Vapor server infrastructure.
* Vapor Content Registry.
* Identity infrastructure.
* Diagnostics infrastructure.
* Other official Vapor applications and services.

The primary capability distinction is authorization to contribute to official Vapor repositories and internal ecosystem infrastructure.

Launcher Developer, Server Developer, Toolchain Developer, Registry Developer, and similar labels do not currently represent separate capability levels.

---

## 7.2 Root Authority

Root Authority contains every Ecosystem Developer capability plus ultimate administrative and ownership authority over the official Vapor ecosystem.

This may include authority over:

* Official repositories.
* Namespaces.
* Registry administration.
* Deployment infrastructure.
* Authorization systems.
* Other ecosystem-root state.

The exact Root-only operational model remains intentionally open.

---

# 8. Capability vs Authorization

Installed capability and external authorization are separate concepts.

A Steam App Instance may contain a fully configured Composer or Content Developer environment even if the current user is not authenticated to a remote Git host, GitHub, or another external provider.

That user may still:

* Compose locally.
* Create content locally.
* Modify content.
* Build.
* Run.
* Test.

Operations requiring remote authority become unavailable until the relevant authentication is present.

Examples include:

* Creating or pushing remote repositories.
* Publishing source.
* Updating remote publications.
* Accessing restricted repositories.
* Collaborating through provider-specific features.
* Contributing to official Vapor repositories.
* Signing builds if signing is introduced.

Therefore:

> **Capability determines what Vapor can technically do locally.**

> **Authorization determines which privileged external operations may currently succeed.**

Git itself is infrastructure.

GitHub is one provider/service used by the current ecosystem.

The two should not be conflated.

---

# 9. Vapor Installer, Launcher, SDK, and CLI

## 9.1 Vapor Installer

The Vapor Installer changes what fundamental Vapor capabilities exist in the Steam App Instance.

Its responsibility is:

> **Change what Vapor can do on this installation.**

Examples include:

* Establish Composer capability.
* Detect/install/configure Git.
* Install/configure Rust/Cargo.
* Install/configure SteamCMD.
* Establish Content Developer capability.
* Configure development prerequisites.
* Repair capability-specific tooling.
* Downgrade/remove higher-level capability tooling.
* Establish Ecosystem Developer tooling where applicable.

---

## 9.2 Vapor Launcher

The Vapor Launcher uses the capabilities already installed.

Its responsibility is:

> **Operate the ecosystem from the user's current capability level.**

Depending on capability level, this may include:

* Launching Vapor Apps.
* Managing locally available Vapor Apps.
* Discovering published Vapor Apps.
* Accessing source-backed Vapor Content.
* Composing packs.
* Managing projects/workspaces.
* Entering the SDK.
* Building.
* Running.
* Testing.
* Publishing.
* Inspecting diagnostics/logs.
* Managing accounts/settings.

The Launcher may wrap or coordinate external systems such as Steam, Git services, and GitHub rather than requiring the user to leave Vapor for every ordinary operation.

---

## 9.3 Vapor SDK

The Vapor SDK is not fundamentally a separate application.

It is the Content Developer-oriented development environment inside the Vapor Launcher.

It is concerned with the programming and configuration of behavioral Vapor Content:

* Engines.
* Games.
* Engine Mods.
* Game Mods.
* Extension Mods.

This includes the workflows required to:

* Create/open content projects.
* Configure content.
* Program behavior.
* Build.
* Run.
* Test.
* Inspect.
* Debug.

Pack authoring belongs to Composer capability and therefore does not inherently require entering the SDK.

The exact boundary between Vapor's own editing surfaces and external IDEs/editors remains open.

---

## 9.4 Vapor CLI

Vapor should provide a CLI for developer-oriented operations.

Its primary audience is:

* Content Developers.
* Ecosystem Developers.
* Root Authority.

Player and Composer experiences remain primarily graphical.

GUI and CLI should generally expose the same underlying capabilities where that is sensible, while being free to present them differently.

---

# 10. Steam Installation and Entry Points

The Steam App should expose three conceptual launch options:

1. **Play Loo Cast**
2. **Start Vapor**
3. **Start Installer**

---

## 10.1 Base Steam Payload

The Steam depot should directly ship:

* Vapor Installer.
* Vapor Launcher.
* Required bootstrap/runtime infrastructure.
* The default first-party Loo Cast composition and its required built constituents.

The default composition therefore does **not** depend on Steam Workshop acquisition merely to make the initially purchased Steam App playable.

This gives the normal Player path a conventional Steam installation model.

---

## 10.2 Play Loo Cast

This launches the default first-party Vapor App directly.

If the installation is healthy, the path should contain effectively no meaningful preparation.

The expensive work should already have happened.

---

## 10.3 Start Vapor

This launches the Vapor Launcher.

The Launcher exposes functionality according to the currently installed capability level.

---

## 10.4 Start Installer

This launches the Vapor Installer.

This is the explicit location for capability upgrades, downgrades, tooling configuration, and environment repair.

---

# 11. Default Composition

The default first-party Packagepack is:

> **Loo Cast Packagepack**

It currently contains/resolves to at least:

* Spacetime Engine.
* Loo Cast Game.

Additional mandatory first-party content may be added later.

Because the default composition is shipped in the Steam depot, a normal Player should be able to install Loo Cast through Steam and immediately possess the built first-party Vapor App required to play.

The first launch may still perform ordinary automatic initialization.

It should not require development-environment setup.

---

# 12. Vapor Content Model

Vapor Content currently contains nine major artifact types.

## Behavioral Content

* Engine.
* Game.
* Engine Mod.
* Game Mod.
* Extension Mod.

## Packs

* Packagepack.
* Enginepack.
* Gamepack.
* Modpack.

Behavioral Content requires Content Developer capability to author.

Packs require Composer capability to author.

---

# 13. Pack Semantics

## 13.1 Packagepack

A Packagepack represents one complete composition.

It must resolve to exactly one effective Engine and exactly one effective Game.

It may directly or indirectly include the applicable Mods and subordinate packs.

It is the only pack type representing a complete composition and therefore the only pack type that can produce a Vapor App.

---

## 13.2 Enginepack

An Enginepack contains:

* Exactly one Engine.
* Any number of compatible Engine Mods.

It is a reusable composition fragment.

It cannot independently produce a complete runnable Vapor App.

---

## 13.3 Gamepack

A Gamepack contains:

* Exactly one Game.
* Any number of compatible Game Mods.

It is a reusable composition fragment.

It cannot independently produce a complete runnable Vapor App.

---

## 13.4 Modpack

A Modpack contains:

* Engine Mods.
* Game Mods.
* Extension Mods.

Its dependency chain must ultimately be compatible with the effective Engine and/or Game of the containing Packagepack.

It is a reusable composition fragment.

It cannot independently produce a complete runnable Vapor App.

---

# 14. Engine and Game Boundary

The effective Engine defines the foundational runtime model of the composition.

The Engine **declares the composition's main binary**.

The Game defines game-specific behavior/content within the Engine-defined foundation.

The Game does not declare the composition's main binary.

Launching a Vapor App therefore ultimately means launching the effective Engine binary with the statically built composition it represents.

---

# 15. Source Model

Vapor source lives in Git.

Steam Workshop is not the canonical source-code distribution system for Vapor Content.

The source side of the ecosystem is built around Vapor-compatible Git repositories and the hierarchy already defined by the ecosystem model.

This includes both:

* Container Repos.
* Source Repos / Vapor Workspaces.

Container Repos are themselves Git repositories.

They organize related Vapor Workspaces as Git submodules.

Vapor Workspaces are the primary source-bearing Git repositories containing Vapor Projects.

Source acquisition, modification, collaboration, and source publication therefore belong to this Git-backed side of the ecosystem.

---

# 16. Steam Workshop Distribution Model

Steam Workshop is used for **built final compositions**.

It does not distribute the canonical source of individual Engines, Games, Mods, or reusable subordinate packs.

Player-facing third-party distribution therefore looks naturally like:

> Published Packagepack
> → built Vapor App artifact(s)
> → Steam Workshop
> → Player acquisition/install
> → launch

This avoids requiring Players to possess or understand the source/build ecosystem.

Steam Workshop acts primarily as an external distribution container for complete built compositions.

The exact packaging of multiple platform/architecture variants inside that distribution model remains open.

---

# 17. Vapor Content Registry

The Vapor Content Registry provides the semantic identity/linkage layer over the external systems Vapor uses.

Vapor should expose human-readable IDs/namespaces rather than forcing users or other Vapor systems to deal directly with provider-specific opaque identifiers.

The Registry may associate Vapor identities with things such as:

* Git-backed source locations.
* Repository identities.
* Steam Workshop publication identities.
* Steam accounts.
* Linked Git-hosting identities.
* Ownership/authorization information.

The exact registry schema is intentionally not defined here.

The important experience-level requirement is:

> **Vapor presents one coherent identity model over multiple external backing systems.**

---

# 18. Library and Discovery

## 18.1 Library

The Vapor Library is the user-facing/local view over artifacts currently available to the Steam App Instance.

What that means depends on capability level.

A Player primarily cares about:

* Installed Vapor Apps.
* Available finished compositions.

A Composer or Developer may additionally care about:

* Locally checked-out source.
* Packs.
* Engines.
* Games.
* Mods.
* Dependency content.
* Build results.

The current model does not introduce a distinct top-level artifact category called **Development Content**.

That is a statement about what the model presently requires, not a permanent assertion that development-state distinctions will never become useful.

Future implementation or UX work may introduce additional state distinctions if concrete workflows require them.

---

## 18.2 Discovery

Player discovery is primarily discovery of finished Vapor Apps.

Composer/Developer discovery additionally concerns source-side Vapor Content and packs.

That source discovery may be mediated through:

* Vapor Registry information.
* Git repositories.
* Git hosting providers.
* Vapor Launcher workflows.

The exact discovery/search UX remains open.

---

# 19. Static Composition Build Model

Vapor Apps are built as complete static compositions.

Changing the effective Packagepack composition requires rebuilding its resulting Vapor App.

The logical build scope is the complete composition.

This does **not** imply wastefully recompiling every unchanged source file from scratch.

Vapor and Cargo should use:

* Incremental compilation.
* Dependency caches.
* Build caches.
* Reusable unchanged intermediates.

The important invariant is:

> **The final runnable result is one statically resolved composition rather than a runtime collection of independently injected Mods.**

Dynamic runtime Mod injection is not the current core composition model.

---

# 20. Player Build Boundary

Players do not build compositions.

They receive already-built Vapor Apps suitable for the current supported target.

A Player therefore does not require:

* Git source.
* Cargo.
* Rust.
* Compiler toolchains.
* Composition rebuild infrastructure.

For the default Loo Cast composition, this built output is shipped directly through the Steam depot.

For additional published third-party compositions, Steam Workshop is the intended built-output distribution mechanism.

---

# 21. Composer Build Model

A Composer works from Git-backed source.

The broad conceptual flow is:

> Acquire source
> → create/modify packs
> → resolve Packagepack
> → build complete composition
> → install/register local Vapor App
> → run/test
> → optionally publish

This is a conceptual model, not a literal complete sequence that every UI workflow must expose step-for-step.

A Composer may produce a completely new Vapor App while reusing only pre-existing Engines, Games, and Mods.

Composition changes require a logical complete rebuild.

Incremental/cached compilation should make repeated builds practical.

---

# 22. Content Development Model

Content Development adds behavioral-content authoring underneath the broader composition/build model.

The core development activity is approximately:

> Create/Open Content
> → Edit/Configure
> → Build
> → Run in relevant composition
> → Inspect/Test
> → Modify
> → Repeat

The exact SDK/project workflow remains deliberately underdesigned.

Important unresolved questions include:

* What Vapor itself edits versus what external IDEs edit.
* How content projects are created.
* How compositions are selected for testing.
* How local changes participate in composition builds.
* How debugging/testing is exposed.
* How build/run configurations are represented.

Those questions should be designed together when the development loop becomes the implementation focus.

---

# 23. Development Storage Model

The current development-storage hierarchy is:

> **Vapor Superworkspace**
> → **Container Repo**
> → **Source Repo / Vapor Workspace**
> → **Vapor Project**

---

## 23.1 Vapor Superworkspace

A Vapor Superworkspace is a disposable local checkout container.

It is not itself a Git repository or primary source-bearing unit, **as in:** losing it primarily risks local unpushed/uncommitted state rather than canonical remote source.

This does not mean deleting one is automatically harmless.

Uncommitted or unpushed local work may still exist inside the checked-out repositories it contains.

---

## 23.2 Container Repo

A Container Repo is a Vapor-managed top-level Git repository.

It organizes related Vapor Workspaces using Git submodules.

Git therefore manages Container Repos as well as source-bearing Vapor Workspaces.

A Container Repo is not itself used as a submodule of another Container Repo.

---

## 23.3 Vapor Workspace

A Source Repo / Vapor Workspace is a source-bearing Git repository contained by a Container Repo as a Git submodule.

It contains one or more Vapor Projects.

It does not itself contain nested Git submodules.

---

## 23.4 Vapor Project

A Vapor Project is a Rust/Cargo workspace contained within a Vapor Workspace.

It is not itself a Git repository.

The precise UX around creating, cloning, adopting, opening, and managing these structures remains an open design area.

---

# 24. Git and Git Hosting

Git is fundamental infrastructure beginning at Composer capability.

It is used for:

* Container Repos.
* Vapor Workspaces.
* Source acquisition.
* Source modification.
* Source publication.
* Collaboration.
* History/version control.

Vapor may orchestrate Git operations.

It should not pretend Git does not exist.

GitHub is one Git hosting/collaboration provider currently important to the Vapor ecosystem.

Local Composer or Developer capability does not inherently require GitHub authentication.

Operations against a remote Git host require whatever authentication that host demands.

Official ecosystem development additionally requires authorization to official first-party repositories.

---

# 25. Publishing Model

Publishing has two fundamentally different sides:

## Source Publication

Source-authored Vapor artifacts live in the Git-backed ecosystem.

This includes:

* Behavioral Vapor Content.
* Packs.
* Packagepacks.

Their canonical source is maintained through Vapor-compatible Git repository structures.

## Player-Facing Built Publication

Complete compositions are built from Packagepacks.

Their runnable output is distributed to Players through Steam Workshop.

Therefore publication of a Packagepack is not publication of:

1. a Packagepack, and
2. some unrelated "finished composition."

The Packagepack **is** the complete composition artifact.

Its publication has both:

* A source-side existence in the Git-backed ecosystem.
* A built player-facing realization distributed through Steam Workshop.

The exact orchestration between those sides remains open.

This includes unresolved details such as:

* Build ownership.
* Platform matrices.
* Versioning.
* Signing.
* Release immutability.
* Registry transactions.
* Steam Workshop item structure.
* Updating existing publications.
* Ownership transfer.
* Collaboration.
* Deprecation.

Those details should be designed together rather than guessed independently.

---

# 26. Persistent Local State

Vapor must maintain several broad categories of local state.

## Player State

Examples:

* Savegames.
* Quicksaves.
* Engine/Game settings.
* Audio/video settings.
* Keybinds.
* UI preferences.

## Vapor State

Examples:

* Selected Vapor App Composition.
* Installed Vapor Apps.
* Launcher settings.
* Account state.
* Capability state.
* Registry/cache metadata.

## Source / Composition State

For Composer and Developer capability:

* Checked-out repositories.
* Pack source.
* Content source.
* Git state.
* Relevant locally available dependencies.

## Build State

Examples:

* Cargo caches.
* Dependency caches.
* Incremental compilation state.
* Build intermediates.
* Built Vapor Apps.

The exact filesystem/ownership boundaries among these categories remain open.

---

# 27. Progressive Disclosure

Vapor should expose complexity progressively.

A Player should primarily see a game/application ecosystem.

A Composer should additionally see a content/composition ecosystem.

A Content Developer should additionally see a programming/development ecosystem.

An Ecosystem Developer should additionally see Vapor itself as a development target.

The distinction should be visually and conceptually substantial.

Higher-level capabilities should appear because the user has acquired a reason to interact with them.

---

# 28. Automation and Transparency

Vapor should aggressively automate incidental infrastructure when the intended outcome is unambiguous.

Examples include:

* Tool detection.
* Tool setup.
* Dependency acquisition.
* Repository checkout where explicitly requested.
* Composition resolution.
* Build orchestration.
* Cache management.
* Launch orchestration.
* Environment validation.

Vapor should be more conservative around:

* Destructive source operations.
* Overwriting local changes.
* Remote publication.
* Authentication.
* Irreversible migration.
* Root-level infrastructure operations.

Players should see high-level outcomes.

Composers should see enough structure to understand composition.

Developers should be able to inspect substantially more underlying detail.

Raw Git/Cargo/tool output should remain accessible for advanced diagnosis even where Vapor also presents higher-level interpretation.

---

# 29. External Ownership Boundaries

Vapor coordinates several external systems.

Their responsibilities should remain conceptually distinct.

* **Steam** owns Steam installation/product mechanics.
* **Steam Workshop** owns its built-artifact distribution containers.
* **Git** owns repository/version-control mechanics.
* **Git hosting providers** own their remote hosting/authentication mechanics.
* **Rust/Cargo** own the underlying Rust build ecosystem.
* **External IDEs/editors** own their own editing/debugging environments.
* **Vapor** owns the coherent ecosystem model, orchestration, user experience, and invariants connecting those systems.

The guiding principle is:

> **Vapor should own the experience and ecosystem semantics without unnecessarily pretending to own every underlying tool or every byte of user source.**

---

# 30. Representative Experience Flows

> [!important]
> The following flows are **illustrative sanity checks**, not an exhaustive workflow specification, state machine, or complete enumeration of every valid Vapor interaction.
>
> They exist to show whether the major concepts compose into a sensible user experience.
>
> Detailed workflow behavior belongs in later system/design work when the relevant area becomes an implementation focus.

---

## 30.1 Player — Default Composition

> Install Loo Cast through Steam
> → launch Play Loo Cast
> → run the depot-shipped default Vapor App

---

## 30.2 Player — Additional Composition

> Start Vapor
> → discover a finished published Vapor App
> → acquire/install its built Workshop distribution
> → select it
> → launch it

---

## 30.3 First-Time Composer

> Start Installer
> → establish Composer tooling
> → Start Vapor
> → acquire existing Vapor source
> → create/modify packs
> → build Packagepack
> → run/test resulting Vapor App

---

## 30.4 Returning Composer

> Open existing composition source
> → change Packagepack/packs
> → rebuild
> → run/test
> → optionally publish

---

## 30.5 Content Developer

> Open/create behavioral Vapor Content
> → edit/configure
> → incorporate into a composition
> → build/run
> → inspect/test
> → repeat

---

## 30.6 Ecosystem Developer

> Obtain authorized official Vapor source
> → modify ecosystem code
> → build/test
> → integrate/deploy through the appropriate official development workflow

These examples deliberately omit many valid intermediate and alternative operations.

---

# 31. Current Non-Goals

The current model does not require:

* User-hosted Vapor Content Registries.
* Arbitrary user-created parallel Steam App Instances.
* Runtime dynamic injection of independently built Mods as the primary composition model.
* Forcing Players to install source/build tooling.
* Using Steam Workshop as the canonical source-code distribution mechanism.
* Treating every Engine/Game/Mod/Server/Launcher developer specialization as a separate capability level.
* Replacing Git with a Vapor-specific version-control system.
* Replacing Rust/Cargo with a Vapor-specific compiler ecosystem.
* Requiring GitHub authentication merely to compose/build/develop locally.

The current model also does **not** permanently forbid introducing useful distinctions such as development-state categories later.

Such distinctions should be introduced only when a concrete workflow requires them.

---

# 32. Open Design Areas

The following areas remain intentionally unresolved.

They are significant, but do not need to be solved merely to make the current model coherent.

## Steam App Instance / Vapor App Composition Details

* Exact persistence of the selected composition.
* Exact relationship between installed Vapor Apps and selected Vapor App Composition.
* Repair/reconciliation when local state changes externally.
* Precise location/ownership of composition-local state.

## Launcher UX

* Final navigation.
* Exact tab/surface structure.
* Search/filtering.
* Library presentation.
* Player vs Composer presentation.

## SDK UX

* Built-in editing versus external IDE integration.
* Project creation.
* Run configurations.
* Debugging.
* Testing.
* Diagnostics.
* Code generation.
* Configuration tooling.

## Development Storage UX

* Superworkspace creation.
* Repository adoption.
* Clone/submodule automation.
* Project discovery.
* Local source ownership.
* Recovery from missing/broken checkouts.

## Source Discovery and Registry

* How source repositories are discovered.
* How Vapor IDs resolve to Git-backed source.
* Provider independence.
* Repository ownership metadata.
* Exact Registry schema.

## Publishing

* Remote Git workflow.
* Steam Workshop publication structure.
* Build infrastructure.
* Platform/architecture artifact matrices.
* Versioning.
* Signing.
* Ownership/collaboration.
* Updates.
* Deprecation/removal.

## Updates and Migration

* Vapor App updates.
* Source-content updates.
* Toolchain updates.
* Compatibility.
* Version pinning.
* Rollback.
* Migration.

## Failure and Recovery

* Interrupted operations.
* Download failures.
* Git failures.
* Build failures.
* Corrupt artifacts.
* Registry outages.
* Steam outages.
* Invalid compositions.
* Protection of local changes.

## Ecosystem / Root Development

* Dev deployment.
* Development branches/environments.
* Root-only operations.
* Production safeguards.

---

# 33. Current Design Baseline

The current design baseline can be summarized as follows:

1. Vapor is entered through the singular Loo Cast Steam App.
2. One local Steam installation is one Steam App Instance.
3. A Steam App Instance has one selected Vapor App Composition.
4. Multiple built Vapor Apps may coexist locally.
5. A Packagepack represents one complete Vapor composition.
6. A Vapor App Composition is the effective resolved composition represented by that Packagepack in local use.
7. A Vapor App is the built runnable realization of that composition.
8. The effective Engine declares the composition's main binary.
9. Packagepacks, Enginepacks, Gamepacks, and Modpacks are Composer-authored Vapor Content.
10. Engines, Games, Engine Mods, Game Mods, and Extension Mods are Content-Developer-authored Vapor Content.
11. Capability levels are strictly cumulative.
12. Players consume built complete compositions.
13. Players do not require Git/Rust/Cargo.
14. Composers use existing source content to construct complete compositions.
15. Composer capability requires Git and Rust/Cargo because compositions are statically built.
16. Content Developers additionally author behavioral content.
17. Ecosystem Developers additionally develop Vapor itself.
18. Root Authority represents ultimate official ecosystem ownership.
19. Local capability and remote authorization are distinct.
20. The Installer changes installed capability.
21. The Launcher uses installed capability.
22. The SDK is the behavioral-content development environment within the Launcher.
23. Git is the source-side foundation of the Vapor ecosystem.
24. Git manages both Container Repos and Vapor Workspaces.
25. Vapor Content source lives in Vapor-compatible Git repositories.
26. Steam Workshop distributes built complete compositions, not canonical individual-content source.
27. The default Loo Cast composition is shipped directly through the Steam depot.
28. Third-party Player-facing compositions are intended to be distributed as built Vapor Apps through Steam Workshop.
29. The Vapor Content Registry provides semantic identity/linkage across the external systems Vapor uses.
30. Vapor Apps are complete statically resolved compositions.
31. Composition changes require a logical complete rebuild, optimized through caching/incremental compilation.
32. The Packagepack is the complete composition artifact; no additional authored "finished composition" entity is required.
33. The current model does not require a separate fundamental Development Content artifact category, but does not prohibit future development-state distinctions.
34. Representative experience flows are illustrative rather than exhaustive specifications.
35. Remaining detailed workflows should be designed coherently when they become implementation constraints rather than prematurely encoded as pseudo-state-machines.
36. This document is the UX/DX bridge between the Ecosystem Glossary and future system specifications, architecture documents, and TDDs.
