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
> It intentionally stays above detailed implementation mechanics. It describes the product model, major workflows, capability boundaries, ownership boundaries, lifecycle relationships, and intended experience strongly enough to constrain later technical design without attempting to encode the entire ecosystem as prose.
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

Vapor is therefore neither:

* A thin GUI over Cargo.
* A replacement for every underlying development tool.
* A hidden black box that prevents advanced users from understanding what happened.

It is the coherent experience and orchestration layer over the ecosystem.

---

# 2. Experience Principles

## 2.1 Steam-First Player Experience

A normal Player should be able to remain within the Steam/Vapor product experience.

Playing Loo Cast should not require:

* Visiting GitHub.
* Installing Git.
* Installing Rust.
* Installing Cargo.
* Installing SteamCMD.
* Editing configuration files.
* Understanding Vapor's repository model.

The default installation should behave like a conventional Steam product.

---

## 2.2 Progressive Capability

Complexity should appear only when the user acquires capabilities that require it.

The progression is not merely about permissions.

It progressively introduces new mental models:

> Player
> → Vapor Apps

> Composer
> → Vapor Content and composition

> Content Developer
> → source projects and behavioral development

> Ecosystem Developer
> → Vapor itself

Higher capability should expose greater power without making lower-capability experiences unnecessarily complicated.

---

## 2.3 Golden Paths over Incidental Infrastructure

Vapor should provide a strongly supported normal path for common operations.

The user should usually be able to say:

> "Build this."

rather than:

> "Locate the correct Cargo workspace, resolve the source repositories, ensure the correct toolchain is active, invoke the right commands, locate the resulting artifact, deploy it into the correct local state, then tell Vapor where I put it."

The underlying operations still exist.

Vapor should orchestrate them.

---

## 2.4 Advanced Transparency

Abstraction must not require opacity.

As capability increases, users should gain increasing access to:

* Raw tool output.
* Repository state.
* Build logs.
* Dependency information.
* Diagnostics.
* Filesystem locations.
* Underlying commands where useful.

A Vapor-friendly interpretation may be the default presentation.

The underlying reality should remain reachable.

---

## 2.5 Static Composition

A Vapor App represents one statically resolved complete composition.

Composition changes occur before launch and require rebuilding.

The normal runtime is not responsible for dynamically assembling an arbitrary collection of independently built Mods.

This is a foundational experience and architecture constraint.

---

## 2.6 Human-Readable Identity

Users should work with Vapor identities and names rather than provider-specific opaque IDs wherever practical.

Steam Workshop IDs, repository URLs, commit hashes, and similar provider-native identities may remain visible where useful.

They should not be the primary semantic identity model presented by Vapor.

---

# 3. Capability Model

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

# 4. Core Product and Composition Model

Several nearby concepts must remain clearly separated.

---

## 4.1 Steam App

There is one Steam-distributed product:

> **Loo Cast**

This is the **Steam App**.

The Steam App provides the product and installation boundary through which the local Vapor ecosystem is accessed.

It includes or provides access to:

* Vapor Installer.
* Vapor Launcher.
* The default first-party composition.
* Installed third-party Vapor Apps.
* The wider Vapor tooling made available through capability upgrades.

The Steam App is not synonymous with one particular runtime composition.

---

## 4.2 Steam App Instance

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

## 4.3 Packagepack

A **Packagepack** represents one complete Vapor composition.

It is not merely a fragment waiting to become some separately authored finished-composition object.

A valid Packagepack already defines the complete composition.

It must resolve to:

* Exactly one effective Engine.
* Exactly one effective Game.
* The applicable Engine Mods.
* The applicable Game Mods.
* The applicable Extension Mods.

Those constituents may be referenced directly or through subordinate packs.

The Packagepack is therefore the composition-level artifact and identity from which the runnable result is produced.

---

## 4.4 Vapor App Composition

A **Vapor App Composition** is the effective resolved composition represented by a Packagepack in the context in which it is selected and used by a Steam App Instance.

It represents the actual effective content graph after composition and dependency resolution.

The Steam App Instance has one selected Vapor App Composition for current/default use.

This is distinct from saying that only one Vapor App may exist locally.

Many built Vapor Apps may coexist.

---

## 4.5 Vapor App

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

They are different views or stages of the same complete composition.

---

# 5. Operational Responsibility Model

Vapor contains several user-facing applications and several external systems.

Their responsibilities must remain distinguishable even when Vapor presents them as one coherent experience.

---

## 5.1 Steam

Steam owns the outer product installation boundary.

It is responsible for:

* Installing the Loo Cast Steam App.
* Updating the Steam-depot-owned portion of that installation.
* Managing Steam library locations.
* Providing Steam ownership/entitlement context.
* Providing Steam launch options.
* Hosting Steam Workshop as an external distribution facility.

Vapor should work with Steam rather than attempting to replace Steam's installation-management responsibilities.

---

## 5.2 Steam App Instance

The Steam App Instance is the local operating context within which Vapor works.

It provides the local boundary around:

* Vapor applications.
* Capability state.
* Selected composition.
* Installed Vapor Apps.
* Local metadata.
* Relevant caches.
* Links to source and development state where higher capabilities are installed.

The Steam App Instance is not itself an authoring tool.

It is the local product instance within which those tools operate.

---

## 5.3 Vapor Installer

The Vapor Installer owns **capability establishment and environment mutation**.

If an operation fundamentally changes what the local Vapor environment is capable of doing, it belongs conceptually to the Installer.

Examples include:

* Detecting required external tooling.
* Installing Rust/Cargo.
* Detecting or configuring Git.
* Installing SteamCMD.
* Establishing Composer capability.
* Establishing Content Developer capability.
* Repairing broken toolchain capability.
* Removing higher-level capability tooling.
* Establishing Ecosystem Developer-specific prerequisites.

The Installer should not become the ordinary day-to-day interface for using those capabilities.

---

## 5.4 Vapor Launcher

The Vapor Launcher owns **ordinary use of installed Vapor capabilities**.

It is the primary operating surface for:

* Playing.
* Selecting Vapor Apps.
* Discovering finished compositions.
* Accessing source-side content as capability permits.
* Composing.
* Building.
* Managing relevant local Vapor state.
* Entering the SDK.
* Accessing development projects.
* Publishing.
* Viewing diagnostics.
* Managing accounts/settings.

The Launcher is the main cohesive frontend over the Vapor ecosystem.

---

## 5.5 Vapor SDK

The Vapor SDK owns the **behavioral-content development experience**.

It is responsible for presenting and coordinating workflows around:

* Engine development.
* Game development.
* Engine Mod development.
* Game Mod development.
* Extension Mod development.
* Content project creation/opening.
* Build/run/test operations relevant to behavioral development.
* Developer-oriented configuration.
* Diagnostics and inspection.

The SDK does not need to replace every external editor or IDE.

It must provide the Vapor-specific development model and orchestration around those tools.

---

## 5.6 Vapor CLI

The Vapor CLI exposes developer-oriented Vapor capabilities through command-line workflows.

It is conceptually another interface to Vapor capabilities rather than an unrelated implementation path.

Where reasonable, GUI and CLI operations should map to the same underlying Vapor concepts.

---

## 5.7 Git

Git owns repository mechanics.

Vapor uses Git for:

* Container Repos.
* Vapor Workspaces.
* Source acquisition.
* Source history.
* Branching.
* Commits.
* Source collaboration.
* Remote synchronization.

Vapor may automate Git.

It should not invent a parallel version-control model unless a future requirement explicitly demands one.

---

## 5.8 Git Hosting Providers

Git hosting providers own remote hosting and provider-specific collaboration.

GitHub is currently a major provider within the Vapor ecosystem.

Provider-specific functionality may include:

* Repository creation.
* Authentication.
* Remote repository access.
* Issues.
* Pull requests.
* Access control.
* Organization/repository authorization.

These are not fundamental local capability requirements unless a specific operation needs them.

---

## 5.9 Steam Workshop

Steam Workshop owns Player-facing distribution containers for built complete compositions.

Steam Workshop does not own the canonical source model.

It is a distribution backend for built Vapor Apps.

---

## 5.10 Vapor Content Registry

The Vapor Content Registry owns Vapor's semantic identity/linkage layer across the external ecosystem.

It is responsible conceptually for allowing Vapor to reason in terms of Vapor identities instead of requiring every operation to use provider-native identifiers.

The exact registry schema remains open.

---

## 5.11 Engine Runtime

The effective Engine owns execution of the built Vapor App.

The Engine declares the composition's main binary.

At launch time, the runtime should receive an already built and resolved composition rather than being responsible for normal composition construction.

---

# 6. Player Experience

## 6.1 Purpose

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

## 6.2 Assumed Knowledge

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

## 6.3 What the Player Normally Sees

The Player-facing Vapor Launcher should primarily expose concepts such as:

* Current/default Vapor App.
* Play.
* Installed Vapor Apps.
* Discoverable Vapor Apps.
* Installation status.
* Basic composition information.
* Settings.
* Accounts.

Raw source-content management should not dominate the Player experience.

Development-oriented surfaces may be absent, visually deemphasized, or represented only as an explicit upgrade path.

---

## 6.4 What the Player Normally Does

Typical Player actions include:

* Play the default Loo Cast Vapor App.
* Open Vapor.
* Browse finished compositions.
* Install a third-party Vapor App.
* Remove a previously installed Vapor App.
* Select another Vapor App.
* Launch it.
* Adjust settings.
* Manage relevant account state.

---

## 6.5 What Vapor Hides from the Player

During normal Player operation, Vapor should hide or fully automate:

* Source repository lookup.
* Git operations.
* Cargo.
* Rust toolchains.
* Compilation.
* Composition builds.
* Workshop implementation details.
* Registry provider IDs.
* Build caches.
* Repository topology.
* Workspace/project topology.

A Player may inspect deeper information when useful, but the successful path should not require it.

---

## 6.6 Player Tooling Boundary

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

## 6.7 Player Discovery

The normal Player-facing discovery unit inside Vapor is a **complete Vapor App**.

Players do not need Vapor-level discovery of individual Engines, Games, Mods, or subordinate packs because they cannot compose them.

The underlying public source repositories may of course exist publicly outside the Player-facing Vapor workflow.

Vapor should present discovery according to what the current capability level can meaningfully do with the discovered object.

---

# 7. Composer Experience

## 7.1 Purpose

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

## 7.2 What Changes When Composer Capability Is Installed

Composer capability introduces several new conceptual layers.

The user now needs to reason about:

* Individual Vapor Content.
* Packs.
* Dependency relationships.
* Compatibility.
* Source acquisition.
* Composition validity.
* Builds.
* Git-backed authored pack state.
* Publication.

The Launcher therefore becomes meaningfully more capable.

This is not simply Player Mode with another button.

---

## 7.3 What the Composer Normally Sees

A Composer may additionally encounter:

* Source-backed content discovery.
* Individual Engines.
* Games.
* Mods.
* Packs.
* Packagepack composition views.
* Dependency information.
* Compatibility information.
* Build state.
* Local source state.
* Publication state.
* Git-related status where useful.

The exact surface organization remains open.

---

## 7.4 What the Composer Normally Does

Typical Composer activities include:

* Find an existing Engine.
* Find an existing Game.
* Find compatible Mods.
* Acquire their source.
* Create an Enginepack, Gamepack, Modpack, or Packagepack.
* Modify an existing pack.
* Resolve a complete Packagepack.
* Build the resulting Vapor App.
* Run/test it.
* Publish their authored packs/composition.
* Update their authored source and publication.

---

## 7.5 Composer Tooling

Composer capability requires more local infrastructure than Player capability because Packagepacks are statically built.

A Composer therefore requires at least:

* Git capability.
* Rust/Cargo toolchain.
* SteamCMD or other Steam tooling where relevant.

The Vapor Installer establishes and manages this capability.

Vapor should hide unnecessary command-level complexity during the normal Composer path.

---

## 7.6 Composer Source Boundary

The Composer consumes behavioral source but does not author it.

A Composer may:

* Check out an Engine repository.
* Build that Engine as part of a Packagepack.
* Reference it.
* Inspect it where useful.

A Composer may not conceptually edit that Engine as part of Composer capability.

Doing so is Content Development.

The same rule applies to Games and Mods.

---

## 7.7 Composer Publication

A Composer may publish the pack types they are allowed to author.

Most importantly, a Composer may publish a **Packagepack**, which already represents the complete composition.

There is no separate authored category called:

> "complete finished composition derived from a Packagepack."

The Packagepack is the complete composition artifact.

Publication then makes the source-side composition available through the Git-backed Vapor source model and makes the appropriate built Vapor App output available through the Player-facing distribution model.

The exact publication transaction is intentionally not yet fixed.

---

# 8. Content Developer Experience

## 8.1 Purpose

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

## 8.2 What Changes When Content Developer Capability Is Installed

Content Developer capability introduces active behavioral source authoring.

The user now needs access to:

* Vapor Projects.
* Source editing.
* Programming/configuration.
* SDK functionality.
* Build/run/test iteration.
* Diagnostics.
* More direct underlying tool visibility.

The core mental-model expansion is:

> Composer works with existing behavior.

> Content Developer can change what behavior exists.

---

## 8.3 What the Content Developer Normally Sees

The Launcher/SDK may expose:

* Content projects.
* Workspaces.
* Source structure.
* Content metadata.
* Dependency declarations.
* Build targets.
* Build output.
* Diagnostics.
* Logs.
* Test/run compositions.
* Git state.
* Toolchain state.
* External editor/IDE integration.

Not all of this must live in one screen or even one application window.

The Vapor-specific conceptual relationships should remain coherent.

---

## 8.4 Intended Development Experience

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

## 8.5 External IDE Boundary

Vapor does not currently need to become a complete general-purpose Rust IDE.

The SDK may provide:

* Vapor-specific project/configuration editing.
* Structured content editing.
* Run/build controls.
* Diagnostics.
* Generated-code visibility.
* Integration hooks.

An external IDE may still own:

* General Rust editing.
* Refactoring.
* Code navigation.
* Debugger UX.
* Other advanced editor capabilities.

The exact boundary remains open.

---

# 9. Ecosystem Developer Experience

## 9.1 Purpose

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

---

## 9.2 Additional Capability

The primary capability distinction is authorization to contribute to official Vapor repositories and internal ecosystem infrastructure.

Launcher Developer, Server Developer, Toolchain Developer, Registry Developer, and similar labels do not currently represent separate capability levels.

---

## 9.3 Expected Experience

Ecosystem development should eventually support a rapid loop such as:

> Modify Vapor
> → build/test
> → commit/push
> → deploy into an appropriate development environment
> → validate the real integrated result

This may wrap or orchestrate tools such as Cargo commands, `xtask`-style tooling, Git, service deployment mechanisms, and development branches.

The exact workflow remains open.

---

# 10. Root Authority

Root Authority contains every Ecosystem Developer capability plus ultimate administrative and ownership authority over the official Vapor ecosystem.

This may include authority over:

* Official repositories.
* Namespaces.
* Registry administration.
* Deployment infrastructure.
* Authorization systems.
* Ecosystem ownership state.
* Root-level destructive/administrative operations.

Root Authority is deliberately modeled even if only one real-world person normally occupies that role.

The exact Root-only operational model remains intentionally open.

---

# 11. Capability vs Authorization

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

# 12. Vapor Installer, Launcher, SDK, and CLI

## 12.1 Vapor Installer

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

## 12.2 Vapor Launcher

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

## 12.3 Vapor SDK

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

## 12.4 Vapor CLI

Vapor should provide a CLI for developer-oriented operations.

Its primary audience is:

* Content Developers.
* Ecosystem Developers.
* Root Authority.

Player and Composer experiences remain primarily graphical.

GUI and CLI should generally expose the same underlying capabilities where that is sensible, while being free to present them differently.

---

# 13. Steam Installation and Entry Points

The Steam App should expose three conceptual launch options:

1. **Play Loo Cast**
2. **Start Vapor**
3. **Start Installer**

---

## 13.1 Base Steam Payload

The Steam depot should directly ship:

* Vapor Installer.
* Vapor Launcher.
* Required bootstrap/runtime infrastructure.
* The default first-party Loo Cast composition and its required built constituents.

The default composition therefore does **not** depend on Steam Workshop acquisition merely to make the initially purchased Steam App playable.

This gives the normal Player path a conventional Steam installation model.

---

## 13.2 Play Loo Cast

This launches the default first-party Vapor App directly.

If the installation is healthy, the path should contain effectively no meaningful preparation.

The expensive work should already have happened.

---

## 13.3 Start Vapor

This launches the Vapor Launcher.

The Launcher exposes functionality according to the currently installed capability level.

---

## 13.4 Start Installer

This launches the Vapor Installer.

This is the explicit location for capability upgrades, downgrades, tooling configuration, and environment repair.

---

# 14. Default Composition

The default first-party Packagepack is:

> **Loo Cast Packagepack**

It currently contains or resolves to at least:

* Spacetime Engine.
* Loo Cast Game.

Additional mandatory first-party content may be added later.

Because the default composition is shipped in the Steam depot, a normal Player should be able to install Loo Cast through Steam and immediately possess the built first-party Vapor App required to play.

The first launch may still perform ordinary automatic initialization.

It should not require development-environment setup.

---

# 15. Vapor Content Model

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

# 16. Pack Semantics

## 16.1 Packagepack

A Packagepack represents one complete composition.

It must resolve to exactly one effective Engine and exactly one effective Game.

It may directly or indirectly include the applicable Mods and subordinate packs.

It is the only pack type representing a complete composition and therefore the only pack type that can produce a Vapor App.

---

## 16.2 Enginepack

An Enginepack contains:

* Exactly one Engine.
* Any number of compatible Engine Mods.

It is a reusable composition fragment.

It cannot independently produce a complete runnable Vapor App.

---

## 16.3 Gamepack

A Gamepack contains:

* Exactly one Game.
* Any number of compatible Game Mods.

It is a reusable composition fragment.

It cannot independently produce a complete runnable Vapor App.

---

## 16.4 Modpack

A Modpack contains:

* Engine Mods.
* Game Mods.
* Extension Mods.

Its dependency chain must ultimately be compatible with the effective Engine and/or Game of the containing Packagepack.

It is a reusable composition fragment.

It cannot independently produce a complete runnable Vapor App.

---

# 17. Engine and Game Boundary

The effective Engine defines the foundational runtime model of the composition.

The Engine **declares the composition's main binary**.

The Game defines game-specific behavior/content within the Engine-defined foundation.

The Game does not declare the composition's main binary.

Launching a Vapor App therefore ultimately means launching the effective Engine binary with the statically built composition it represents.

---

# 18. Source Model

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

# 19. Steam Workshop Distribution Model

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

# 20. Vapor Content Registry

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

# 21. Library and Discovery

## 21.1 Library

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

## 21.2 Discovery

Player discovery is primarily discovery of finished Vapor Apps.

Composer/Developer discovery additionally concerns source-side Vapor Content and packs.

That source discovery may be mediated through:

* Vapor Registry information.
* Git repositories.
* Git hosting providers.
* Vapor Launcher workflows.

The exact discovery/search UX remains open.

---

# 22. Static Composition Build Model

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

# 23. Player Build Boundary

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

# 24. Composer Build Model

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

# 25. Content Development Model

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

# 26. Development Storage Model

The current development-storage hierarchy is:

> **Vapor Superworkspace**
> → **Container Repo**
> → **Source Repo / Vapor Workspace**
> → **Vapor Project**

---

## 26.1 Vapor Superworkspace

A Vapor Superworkspace is a disposable local checkout container.

It is not itself a Git repository or primary source-bearing unit, **as in:** losing it primarily risks local unpushed/uncommitted state rather than canonical remote source.

This does not mean deleting one is automatically harmless.

Uncommitted or unpushed local work may still exist inside the checked-out repositories it contains.

---

## 26.2 Container Repo

A Container Repo is a Vapor-managed top-level Git repository.

It organizes related Vapor Workspaces using Git submodules.

Git therefore manages Container Repos as well as source-bearing Vapor Workspaces.

A Container Repo is not itself used as a submodule of another Container Repo.

---

## 26.3 Vapor Workspace

A Source Repo / Vapor Workspace is a source-bearing Git repository contained by a Container Repo as a Git submodule.

It contains one or more Vapor Projects.

It does not itself contain nested Git submodules.

---

## 26.4 Vapor Project

A Vapor Project is a Rust/Cargo workspace contained within a Vapor Workspace.

It is not itself a Git repository.

The precise UX around creating, cloning, adopting, opening, and managing these structures remains an open design area.

---

# 27. Local State and Ownership Model

The Vapor ecosystem involves several kinds of local state with different owners and recovery expectations.

These distinctions matter because operations such as repair, cleanup, upgrade, uninstall, and rebuild must not treat all local files equivalently.

---

## 27.1 Steam-Depot-Owned State

This includes files delivered and maintained as part of the Steam App installation.

Examples include:

* Vapor Launcher binaries.
* Vapor Installer binaries.
* Required bootstrap/runtime files.
* The depot-shipped default Loo Cast Vapor App and its required constituents.

Steam is the primary installation authority for this state.

Vapor should avoid treating these files as ordinary user-authored mutable state.

---

## 27.2 Vapor-Managed Product State

This includes local state Vapor maintains in order to operate the Steam App Instance.

Examples may include:

* Selected Vapor App Composition.
* Installed third-party Vapor App metadata.
* Registry resolution metadata.
* Launcher configuration.
* Capability state.
* Tool-detection state.
* Account linkage state.
* Local installation records.

This state should generally be reconstructible or repairable without destroying user-authored source.

---

## 27.3 User Gameplay State

Examples include:

* Savegames.
* Quicksaves.
* Engine/Game settings.
* Keybinds.
* User-created runtime configuration.

This state is user-valued and must not be treated as disposable build/cache state.

Its exact location and migration policy remain open.

---

## 27.4 Git-Managed Source State

This includes:

* Container Repos.
* Vapor Workspaces.
* Pack source.
* Behavioral-content source.
* Commits.
* Branches.
* Uncommitted modifications.

Remote commits may be recoverable from Git hosting.

Uncommitted or unpushed state may exist only locally.

Vapor must therefore treat source operations conservatively.

---

## 27.5 Build State

Build state may include:

* Cargo target state.
* Downloaded dependencies.
* Incremental compilation data.
* Generated intermediates.
* Vapor build metadata.
* Temporary packaging artifacts.

Much of this state should be considered regenerable.

It may be aggressively cached for performance without becoming the canonical source of authored content.

---

## 27.6 Installed Vapor Apps

Built Vapor Apps occupy a special position.

They are not canonical source, but they are first-class runnable local products.

A locally built Vapor App may be regenerated from source.

A Workshop-acquired Vapor App may be reacquired from its publication.

That does not mean every installed Vapor App should be treated as disposable at every moment.

Vapor should explicitly manage their install/remove state.

---

## 27.7 Authentication and Credentials

Credential storage should remain distinct from ordinary Vapor content and configuration.

Examples may include:

* Git-host authentication.
* GitHub authentication.
* Vapor account/session state.
* Steam-linked account state.

The exact secure-storage strategy is a lower-level concern.

The Experience Model requires that capability removal, repair, and content cleanup not casually destroy unrelated credentials or account identity.

---

## 27.8 Disposable vs Authoritative State

The broad intended distinction is:

**Potentially authoritative or user-valued:**

* Git source.
* Uncommitted changes.
* Unpushed commits.
* Gameplay saves.
* User configuration.
* Authored packs/projects.

**Generally reconstructible:**

* Build intermediates.
* Download caches.
* Registry metadata caches.
* Tool-detection caches.
* Installed copies of externally published Vapor Apps.

This is not yet an exhaustive ownership specification.

It establishes the direction later cleanup/recovery systems must respect.

---

# 28. Lifecycle Hierarchy

Vapor does not have one universal linear lifecycle.

It contains several related lifecycles operating at different levels.

The Experience Model should describe their relationships without pretending to encode every possible transition.

---

## 28.1 Steam App Instance Lifecycle

At the broadest local level:

> Steam installation
> → initial usable Player environment
> → optional capability upgrades
> → ordinary operation
> → updates/repairs/moves
> → optional capability downgrades
> → eventual uninstall

This lifecycle concerns the local product instance rather than any one composition.

---

## 28.2 Capability Lifecycle

Capability evolves independently of ordinary content selection.

Conceptually:

> Player
> → Composer
> → Content Developer
> → Ecosystem Developer
> → Root Authority

Higher capability may require:

* Additional tooling.
* Additional configuration.
* Additional authorization.

Capability may also be downgraded.

Downgrading capability should not automatically imply destruction of user-authored source.

---

## 28.3 Packagepack / Vapor App Lifecycle

At the composition level:

> Packagepack source exists
> → composition resolves
> → Vapor App is built
> → Vapor App is installed/registered
> → Vapor App may be selected
> → Vapor App may be launched
> → composition source may change
> → rebuilt Vapor App supersedes or accompanies older build state

Publication adds additional remote states but does not redefine what the Packagepack is.

---

## 28.4 Source Content Lifecycle

For source-side Vapor Content:

> Source created/acquired
> → source modified
> → source committed
> → source synchronized/published
> → source consumed by compositions
> → source updated over time

The exact relationship between Vapor content identity and repository history/versioning remains open.

---

## 28.5 Development Lifecycle

For behavioral-content development:

> Create/Open
> → Edit/Configure
> → Build
> → Run/Test
> → Inspect
> → Modify
> → Repeat
> → optionally publish

This lifecycle nests inside composition/build workflows because behavioral content must ultimately be exercised as part of some complete composition.

---

## 28.6 Publication Lifecycle

At a coarse level:

> Validate authored source
> → publish/synchronize source through Git-backed infrastructure
> → build complete Packagepack composition
> → publish built Vapor App through Steam Workshop
> → register/link publication through Vapor infrastructure

This is intentionally incomplete.

Exact ordering, atomicity, build ownership, versioning, signing, and update semantics remain open.

---

## 28.7 Runtime Lifecycle

Runtime operation begins after composition/build work is already complete.

Conceptually:

> Select installed Vapor App
> → launch effective Engine main binary
> → run complete static composition
> → exit
> → preserve appropriate user/runtime state

Normal launch should not become an implicit composition-authoring workflow.

---

# 29. Experience Invariants and Guarantees

The following statements are intended to constrain later design and implementation.

They are stronger than illustrative workflows.

---

## 29.1 Player Invariants

* A Player must not require Git.
* A Player must not require Rust/Cargo.
* A Player must not require SteamCMD.
* A Player must not be required to compile a Vapor App.
* The depot-shipped default composition must be playable without acquiring source.
* Player-facing third-party distribution concerns built complete compositions.

---

## 29.2 Composition Invariants

* A valid Packagepack represents one complete Vapor composition.
* Every complete composition resolves to exactly one effective Engine.
* Every complete composition resolves to exactly one effective Game.
* The effective Engine declares the composition's main binary.
* Only a Packagepack represents a complete composition capable of producing a Vapor App.
* Enginepacks, Gamepacks, and Modpacks remain reusable composition fragments.
* A Vapor App represents one statically resolved complete composition.
* Changing the effective composition requires a logical rebuild.

---

## 29.3 Source and Distribution Invariants

* Canonical Vapor source lives in Git-backed repository structures.
* Steam Workshop is not the canonical source-code distribution mechanism.
* Steam Workshop distributes built complete compositions for Player consumption.
* Container Repos are Git-managed.
* Vapor Workspaces are Git-managed and source-bearing.
* Git and GitHub are not synonymous concepts.

---

## 29.4 Capability Invariants

* Capability levels are cumulative.
* Composer capability includes Player capability.
* Content Developer capability includes Composer capability.
* Ecosystem Developer capability includes Content Developer capability.
* Root Authority includes Ecosystem Developer capability.
* Installed local capability and remote authorization are distinct.
* Capability establishment/removal belongs conceptually to the Vapor Installer.
* Ordinary use of installed capabilities belongs conceptually to the Vapor Launcher and its contained surfaces.

---

## 29.5 Authorship Invariants

* Composers may author packs.
* Composers may not author or modify Engines, Games, or Mods as part of Composer capability.
* Content Developers may author behavioral Vapor Content.
* Content Developers retain all Composer capabilities.
* Ecosystem Developers may develop Vapor itself.

---

## 29.6 Source-Safety Invariants

* Vapor must not silently destroy uncommitted user source.
* Capability downgrade must not inherently imply deleting authored source.
* Rebuild/cleanup operations must distinguish regenerable build state from user-authored state.
* Repository automation must respect the possibility of local-only Git state.

---

## 29.7 Launch Invariants

* Normal Player launch should operate on an already built Vapor App.
* Launch should not require source resolution or compilation in Player capability.
* The effective Engine main binary is the runtime entrypoint for the composition.
* Runtime launch should not normally perform heavy composition construction.

---

# 30. Progressive Disclosure

Vapor should expose complexity progressively.

A Player should primarily see a game/application ecosystem.

A Composer should additionally see a content/composition ecosystem.

A Content Developer should additionally see a programming/development ecosystem.

An Ecosystem Developer should additionally see Vapor itself as a development target.

The distinction should be visually and conceptually substantial.

Higher-level capabilities should appear because the user has acquired a reason to interact with them.

Progressive disclosure does not mean hiding the existence of higher capability.

The Launcher may advertise or provide entry points toward additional capabilities.

The Installer remains responsible for establishing those capabilities.

---

# 31. Automation and Transparency

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

# 32. External Ownership Boundaries

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

# 33. Representative Experience Flows

> [!important]
> The following flows are **illustrative sanity checks**, not an exhaustive workflow specification, state machine, test suite, or complete enumeration of every valid Vapor interaction.
>
> They are intentionally incomplete.
>
> Their purpose is to pressure-test whether the major concepts can compose into plausible experiences.
>
> Exact tasks, intermediate states, alternate paths, failure paths, and implementation-level transitions belong in later focused designs.

---

## 33.1 Player — Default Composition

> Install Loo Cast through Steam
> → launch Play Loo Cast
> → run the depot-shipped default Vapor App

---

## 33.2 Player — Additional Composition

> Start Vapor
> → discover a finished published Vapor App
> → acquire/install its built Workshop distribution
> → select it
> → launch it

---

## 33.3 First-Time Composer

> Start Installer
> → establish Composer tooling
> → Start Vapor
> → acquire existing Vapor source
> → create/modify packs
> → build Packagepack
> → run/test resulting Vapor App

---

## 33.4 Returning Composer

> Open existing composition source
> → change Packagepack/packs
> → rebuild
> → run/test
> → optionally publish

---

## 33.5 Content Developer

> Open/create behavioral Vapor Content
> → edit/configure
> → incorporate into a composition
> → build/run
> → inspect/test
> → repeat

---

## 33.6 Ecosystem Developer

> Obtain authorized official Vapor source
> → modify ecosystem code
> → build/test
> → integrate/deploy through the appropriate official development workflow

These examples deliberately omit many valid intermediate and alternative operations.

---

# 34. Current Non-Goals

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

# 35. Open Design Areas

The following areas remain intentionally unresolved.

They are significant, but do not need to be solved merely to make the current model coherent.

---

## 35.1 Steam App Instance / Vapor App Composition Details

* Exact persistence of the selected composition.
* Exact relationship between installed Vapor Apps and selected Vapor App Composition.
* Whether the selected concept is always Packagepack-based identity, built-App identity, or both.
* Repair/reconciliation when local state changes externally.
* Precise location/ownership of composition-local state.

---

## 35.2 Launcher UX

* Final navigation.
* Exact tab/surface structure.
* Search/filtering.
* Library presentation.
* Player vs Composer presentation.
* How unavailable higher-capability features are represented.
* How external services appear within Vapor.

---

## 35.3 SDK UX

* Built-in editing versus external IDE integration.
* Project creation.
* Run configurations.
* Debugging.
* Testing.
* Diagnostics.
* Code generation.
* Configuration tooling.
* Test-composition management.

---

## 35.4 Development Storage UX

* Superworkspace creation.
* Repository adoption.
* Clone/submodule automation.
* Project discovery.
* Local source ownership.
* Recovery from missing/broken checkouts.
* Multiple Container Repo relationships.
* How users select/open the correct source context.

---

## 35.5 Source Discovery and Registry

* How source repositories are discovered.
* How Vapor IDs resolve to Git-backed source.
* Provider independence.
* Repository ownership metadata.
* Exact Registry schema.
* How Pack dependencies refer to source identities.
* How historical/versioned source is resolved.

---

## 35.6 Publishing

* Remote Git workflow.
* Steam Workshop publication structure.
* Build infrastructure.
* Platform/architecture artifact matrices.
* Versioning.
* Signing.
* Ownership/collaboration.
* Updates.
* Deprecation/removal.
* Publication atomicity across Git/Registry/Workshop.
* Who is authorized to update an existing publication.

---

## 35.7 Updates and Migration

* Vapor App updates.
* Source-content updates.
* Toolchain updates.
* Compatibility.
* Version pinning.
* Rollback.
* Migration.
* Whether older locally built Vapor Apps remain runnable after source changes.

---

## 35.8 Failure and Recovery

* Interrupted operations.
* Download failures.
* Git failures.
* Build failures.
* Corrupt artifacts.
* Registry outages.
* Steam outages.
* Invalid compositions.
* Protection of local changes.
* Partial capability installation.
* Toolchain corruption.
* Failed publication across multiple backing services.

---

## 35.9 Ecosystem / Root Development

* Dev deployment.
* Development branches/environments.
* Root-only operations.
* Production safeguards.
* Server deployment.
* Official repository synchronization.
* Emergency/recovery administration.

---

# 36. Current Design Baseline

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
32. The Packagepack is the complete composition artifact; no additional authored finished-composition entity is required.
33. The current model does not require a separate fundamental Development Content artifact category, but does not prohibit future development-state distinctions.
34. Steam, Git, Git hosting, Steam Workshop, Vapor applications, and the Engine runtime have distinct operational responsibilities.
35. Player, Composer, Content Developer, Ecosystem Developer, and Root Authority experiences expose progressively larger mental models rather than merely progressively more permissions.
36. Steam-depot state, Vapor-managed state, user gameplay state, Git source, build state, installed Vapor Apps, and authentication state require different ownership/recovery treatment.
37. Vapor contains several nested/related lifecycles rather than one universal workflow.
38. User-authored Git state must be protected from destructive automation.
39. Player launch operates on already-built compositions.
40. Representative experience flows are illustrative rather than exhaustive specifications.
41. Remaining detailed workflows should be designed coherently when they become implementation constraints rather than prematurely encoded as pseudo-state-machines.
42. This document is the UX/DX bridge between the Ecosystem Glossary and future system specifications, architecture documents, and TDDs.
