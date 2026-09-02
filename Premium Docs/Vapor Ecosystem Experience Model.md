
> [!info]
> This document defines the intended **User Experience (UX)** and **Developer Experience (DX)** of the Vapor Ecosystem.
>
> It sits between the **Ecosystem Model Glossary** and lower-level system specifications, architecture documents, and TDDs.
>
> The glossary defines **what exists**.
>
> This document defines **what using those things is supposed to be like**.
>
> It intentionally describes the ecosystem from the outside inward:
>
> * Who uses Vapor.
> * What they are trying to accomplish.
> * Which capabilities Vapor exposes to them.
> * Which applications and surfaces they encounter.
> * How content moves from discovery to execution.
> * How composing differs from development.
> * What Vapor automates and what remains under user control.
>
> This is a design baseline, not an exhaustive implementation specification. Areas that are not yet sufficiently designed are explicitly left open rather than being invented prematurely.

---

# 1. Experience Goals

Vapor exists to make a complex Rust/Cargo/Steam/Git-based ecosystem behave like one coherent product.

The user should primarily interact with **Vapor concepts**, not with the accidental complexity of the tools used underneath them.

A Player should think in terms of:

> Find App → Install → Play

A Composer should think in terms of:

> Find Content → Compose → Build → Play / Publish

A Content Developer should think in terms of:

> Create Content → Edit → Build → Run → Inspect → Repeat

An Ecosystem Developer should think in terms of:

> Modify Vapor → Build/Test → Integrate → Deploy

Git, Cargo, Rust, SteamCMD, Steam Workshop, GitHub, Bevy, ECS infrastructure, build caches, generated files, and similar systems remain available where useful, but should not define the primary user experience.

The intended principle is:

> **Expose the work the user intends to do; automate the infrastructure required to make that work possible.**

Vapor should provide a strong golden path without unnecessarily preventing advanced users from understanding or directly interacting with the underlying systems.

---

# 2. Capability Model

Vapor uses a strict progressive capability hierarchy:

> **Player ⊂ Composer ⊂ Content Developer ⊂ Ecosystem Developer ⊂ Root Authority**

Each level is a superset of the previous level.

These levels are not independent personas. They describe increasingly capable ways of interacting with the same ecosystem.

A Content Developer is also a Composer and a Player.

An Ecosystem Developer is also a Content Developer.

Root Authority contains every lower capability.

Capability progression should be experientially meaningful. Moving to a higher level should feel like Vapor has acquired a substantial new set of abilities rather than merely exposing an obscure advanced-options checkbox.

---

# 3. Player

## 3.1 Purpose

A Player consumes finished Vapor Apps.

The normal Player wants to:

* Install the Steam App.
* Play the default Loo Cast Vapor App.
* Discover other finished first-party or third-party Vapor Apps.
* Install them.
* Select between them.
* Launch them.
* Manage ordinary settings and accounts.

A Player is not expected to understand the internal composition of a Vapor App.

---

## 3.2 Assumed Knowledge

Vapor may assume approximately:

* Basic computer literacy.
* Basic familiarity with Steam.
* Ordinary understanding of installing and launching games.

Vapor should not assume:

* Programming experience.
* Git knowledge.
* Rust knowledge.
* Cargo knowledge.
* Build-system knowledge.
* Knowledge of Vapor's repository/workspace architecture.

---

## 3.3 Player Boundaries

A Player does not compose Vapor Content.

A Player therefore does not normally require:

* Git.
* Rust.
* Cargo.
* SteamCMD.
* SDK tooling.
* Source repositories.
* Workspaces.
* Projects.

A Player may technically encounter individual Vapor Content through Steam Workshop because Vapor Content is distributed using public Workshop Items.

However, individual-content discovery is not a primary Player capability inside Vapor itself.

Within Vapor, actively discovering and inspecting individual composable content belongs to Composer workflows because that discovery exists primarily to allow composition.

The normal Player discovery unit is therefore the **finished Vapor App**.

---

# 4. Composer / Content User

## 4.1 Purpose

A Composer is a Player who wants to create new compositions from existing Vapor Content.

A Composer may create and modify:

* Packagepacks.
* Enginepacks.
* Gamepacks.
* Modpacks.

A Composer may use existing:

* Engines.
* Games.
* Engine Mods.
* Game Mods.
* Extension Mods.

A Composer may not create or modify the underlying behavioral content itself.

The key distinction is:

> **A Composer chooses and combines behavior. A Content Developer creates or changes behavior.**

---

## 4.2 Composer Experience

The Composer experience should feel closer to assembling a sophisticated content ecosystem than to traditional software development.

A typical conceptual flow is:

> Discover Content
> → Acquire Content
> → Select/Combine Content
> → Create or Modify Pack Manifests
> → Resolve Composition
> → Build
> → Install Locally
> → Play/Test
> → Optionally Publish

The Composer should understand concepts such as:

* Engine.
* Game.
* Mods.
* Extension relationships.
* Packs.
* Packagepacks.
* Dependencies.
* Composition validity.

The Composer should not need to manually orchestrate Cargo commands, dependency checkout, build directories, or similar incidental infrastructure during the normal path.

---

## 4.3 Composer Tooling

Composer capability requires additional local infrastructure because Vapor Apps are statically built compositions.

A Composer therefore requires at least:

* Git capability.
* Rust/Cargo toolchain.
* SteamCMD or equivalent Steam publication/download tooling where required.

These are installed or configured through the Vapor Installer.

Git is part of the Composer-level source workflow.

GitHub is not conceptually required merely to possess Composer capability.

Authentication to a remote Git service, Steam, Vapor, GitHub, or another external provider is required only when a specific operation requires that provider.

---

# 5. Content Developer

## 5.1 Purpose

A Content Developer creates or modifies actual behavioral Vapor Content.

This includes:

* Engine.
* Game.
* Engine Mod.
* Game Mod.
* Extension Mod.

Engine Developer, Game Developer, and Mod Developer are not separate fundamental ecosystem roles.

They are forms of Content Development.

A Content Developer also inherits every Composer capability and may therefore create, modify, build, and publish packs and complete Packagepacks.

---

## 5.2 Intended Development Experience

Content development may involve:

* Programming.
* Configuration.
* Content creation.
* Dependency declaration.
* Build configuration.
* Testing.
* Runtime inspection.
* Debugging.

The intended experience should nevertheless feel substantially friendlier than manually constructing an arbitrary Rust workspace from scratch.

Vapor should provide:

* Strong project structure.
* Guardrails.
* Generated boilerplate where useful.
* Automated dependency setup.
* Consistent build/run operations.
* Integrated diagnostics.
* Clear relationships between a project and the content it models.
* Access to the underlying tools when needed.

The desired feeling is:

> **Develop the content, not the machinery required to persuade Rust, Cargo, Bevy, Git, Steam, and Vapor to cooperate.**

---

# 6. Ecosystem Developer and Root Authority

## 6.1 Ecosystem Developer

An Ecosystem Developer develops Vapor itself.

This may include:

* Vapor Installer.
* Vapor Launcher.
* Vapor SDK.
* Vapor CLI.
* Root/client framework code.
* Vapor Content Registry.
* Identity infrastructure.
* Diagnostics infrastructure.
* Server-side infrastructure.
* Other official Vapor tooling and services.

The fundamental additional capability is authorization to contribute to official Vapor repositories and internal development infrastructure.

The ecosystem does not currently model Launcher Developer, Server Developer, Registry Developer, Toolchain Developer, and similar specializations as separate capability levels.

They are all Ecosystem Developers.

---

## 6.2 Root Authority

Root Authority is the final ecosystem-level edge case.

Root Authority includes all Ecosystem Developer capabilities together with ultimate ownership and administrative authority over the official Vapor ecosystem.

This may include control over:

* Official repositories.
* Official namespaces.
* Registry administration.
* Infrastructure.
* Deployment.
* Authorization.
* Other root-level ecosystem state.

The exact Root-only administrative workflow is intentionally not yet specified.

---

# 7. Capability vs Authorization

Installed capability and external authorization are separate concerns.

An App Instance may contain all required Content Developer tooling even if the current user is not authenticated to GitHub or another remote provider.

That user may still:

* Create content.
* Modify content.
* Compose.
* Build.
* Run.
* Test.
* Work locally.

Operations that require external authority remain unavailable until the relevant authentication is present.

Examples include:

* Publishing to a remote service.
* Updating an existing remote publication.
* Pushing to a remote Git repository.
* Collaborating through provider-specific features.
* Accessing private repositories.
* Contributing to official Vapor repositories.
* Build signing, if signing is introduced.

Therefore:

> **Capability determines what Vapor can technically do locally. Authorization determines which externally privileged operations the user may perform.**

---

# 8. Steam App, App Instance, Packagepack, App Composition, and Vapor App

These concepts represent different levels of the product/composition model.

## 8.1 Steam App

There is one Steam-distributed product:

> **Loo Cast**

The Steam App is the installation and distribution container for entering the Vapor ecosystem.

It is not itself synonymous with the currently running game/composition.

---

## 8.2 App Instance

The installed Steam App is the local App Instance.

The current model assumes:

* One App Instance per Steam installation.
* No user-created parallel instances.
* A Steam-managed installation path.
* Movement through Steam's supported library/move mechanisms rather than arbitrary manual relocation.

Vapor should tolerate Steam moving the installation.

The system should not be designed around user-facing management of multiple manually created App Instances unless a future requirement actually demands it.

---

## 8.3 Packagepack

A Packagepack is the declarative root definition of one complete composition.

It references enough Vapor Content and/or subordinate packs to resolve to:

* Exactly one effective Engine.
* Exactly one effective Game.
* Any effective Engine Mods.
* Any effective Game Mods.
* Any effective Extension Mods.

---

## 8.4 App Composition

Resolving a Packagepack produces its effective App Composition.

The App Composition is the complete logical content graph that is intended to be built and run.

---

## 8.5 Vapor App

Building and deploying an App Composition produces a Vapor App.

A Vapor App is the actual launchable realization of the composition for a supported target.

Conceptually:

> **Packagepack**
> declarative definition
>
> ↓ resolve
>
> **App Composition**
> effective logical composition
>
> ↓ build/deploy
>
> **Vapor App**
> runnable realization

These are distinct concepts, but intentionally represent different sides/stages of the same complete composition.

---

# 9. Vapor Applications

## 9.1 Vapor Installer

The Vapor Installer changes which fundamental Vapor capabilities are present in the App Instance.

Its responsibility is:

> **Change what Vapor can do on this installation.**

Examples include:

* Establish Composer capability.
* Find/install/configure Git.
* Install/configure Rust/Cargo.
* Install/configure SteamCMD.
* Establish Content Developer capability.
* Configure development prerequisites.
* Remove or downgrade capability-specific tooling.
* Establish additional Ecosystem Developer infrastructure where applicable.

The Installer is not the normal place to use those capabilities after installation.

---

## 9.2 Vapor Launcher

The Vapor Launcher is the primary user-facing Vapor application.

Its responsibility is:

> **Use the capabilities already installed.**

Depending on capability level, this includes:

* Playing Vapor Apps.
* Discovering Vapor Apps.
* Managing installed Vapor Apps.
* Managing local Vapor Content.
* Composing packs.
* Managing projects.
* Entering the SDK.
* Building.
* Running.
* Testing.
* Publishing.
* Inspecting logs and diagnostics.
* Managing accounts.
* Managing settings.

---

## 9.3 Vapor SDK

The Vapor SDK is not fundamentally a separate application.

It is the Content Developer-oriented development environment inside the Vapor Launcher.

The SDK is concerned with:

> **Creating, programming, configuring, building, testing, and inspecting behavioral Vapor Content.**

That means primarily:

* Engines.
* Games.
* Engine Mods.
* Game Mods.
* Extension Mods.

Pack creation is Composer functionality and therefore does not inherently belong to the SDK, although the SDK may naturally expose pack-related features because every Content Developer is also a Composer.

The SDK should eventually feel substantial enough to serve as a coherent development environment, while avoiding unnecessary duplication of full-purpose source-code editors and IDEs where external tools remain more appropriate.

The exact SDK editing/IDE boundary remains open.

---

## 9.4 Vapor CLI

Vapor should expose a CLI for developer-oriented workflows.

Its intended users are:

* Content Developers.
* Ecosystem Developers.
* Root Authority.

Composer and Player workflows remain primarily graphical.

The GUI and CLI should expose approximately equivalent underlying developer capabilities where reasonable, while being free to present them differently.

The exact division between CLI-first and GUI-first workflows is intentionally not yet fixed.

---

# 10. Steam Entry Points

The Loo Cast Steam App should expose three stable conceptual entry points:

1. **Play Loo Cast**
2. **Start Vapor**
3. **Start Installer**

---

## 10.1 Play Loo Cast

This launches the default first-party Vapor App.

The normal direct-play path should require no Launcher interaction.

If the required default Vapor App is already installed and valid, pressing Play should result in effectively immediate execution.

Heavy work should not normally occur between Play and launch.

---

## 10.2 Start Vapor

This launches the Vapor Launcher.

The Launcher exposes the capabilities currently installed in the App Instance.

Higher capabilities may be visible as upgrade paths even when not yet installed, but actual installation/configuration belongs to the Installer.

---

## 10.3 Start Installer

This launches the Vapor Installer.

The Installer is the explicit place for:

* Capability upgrades.
* Capability downgrades.
* Toolchain installation.
* Toolchain repair/configuration.
* Other fundamental environment changes.

---

# 11. First Installation and Bootstrap

Immediately after Steam installs the Steam App, the local App Instance should contain:

* Vapor Installer.
* Vapor Launcher.
* Enough bootstrap infrastructure to acquire the default first-party Vapor App.

The complete default composition does not need to be shipped directly inside the Steam depot.

The default composition is:

> **Loo Cast Packagepack**

which currently resolves to at least:

* Spacetime Engine.
* Loo Cast Game.

and may include additional mandatory first-party content later.

On first use, Vapor should automatically acquire whatever is required to make the default Vapor App available.

For a normal Player, this process should be:

* Automatic.
* Minimal.
* Opaque where appropriate.
* Free of manual developer tooling.
* Contained inside the Steam/Vapor experience.

The Player should not have to configure Vapor before being able to play.

---

# 12. Launcher Experience

The exact UI is intentionally not yet fixed.

The Launcher should nevertheless expose a small set of clear conceptual areas.

A likely top-level structure includes:

* Home.
* Vapor Apps / Compositions.
* Library.
* Discover.
* Development.
* Settings.
* Accounts.

These are conceptual surfaces, not committed final tab names.

---

## 12.1 Home

The Home experience should provide a compact starting point.

Likely information/actions include:

* Current/default Vapor App.
* Quick composition selection.
* Play.
* Recent/relevant status.
* Shortcuts into deeper surfaces.

---

## 12.2 Vapor Apps / Compositions

This surface manages complete runnable compositions.

It should allow users to:

* View available/installed Vapor Apps.
* Install finished Vapor Apps.
* Select the current/default Vapor App.
* Launch Vapor Apps.
* Inspect basic composition information.

Many Vapor Apps may be installed simultaneously.

Selecting one does not fundamentally activate it globally.

Selection primarily changes Launcher/direct-play convenience defaults.

---

## 12.3 Library

The Library represents locally available Vapor Content and packs.

The local model deliberately avoids unnecessary categories such as:

* "installed dependency"
* "cached dependency"
* "development content"

If a piece of Vapor Content exists locally, it exists locally.

Whether the user is consuming it, composing with it, or editing it does not create a separate ontological version of that content.

The Library may contain:

* Engines.
* Games.
* Engine Mods.
* Game Mods.
* Extension Mods.
* Enginepacks.
* Gamepacks.
* Modpacks.
* Packagepacks.

Built Vapor Apps remain associated with the Packagepacks from which they were produced.

---

## 12.4 Discover

Player discovery should primarily expose finished Vapor Apps.

Composer capability expands discovery to individual Vapor Content so that the user can actively search for components to compose.

Steam Workshop remains independently browsable and may expose all public Workshop Items regardless of Vapor capability level.

Vapor itself should present discovery according to what the current capability level can meaningfully do with the discovered object.

---

## 12.5 Development

Development functionality appears as capability increases.

For Content Developers this may include:

* Projects.
* SDK.
* Toolchain state.
* Build/run operations.
* Testing.
* Diagnostics.
* Logs.

For Ecosystem Developers it may additionally expose official Vapor development/integration workflows.

The detailed Development surface remains an open design area.

---

# 13. Content Model

Vapor Content has nine primary artifact types:

## Behavioral Content

* Engine.
* Game.
* Engine Mod.
* Game Mod.
* Extension Mod.

## Packs

* Enginepack.
* Gamepack.
* Modpack.
* Packagepack.

Behavioral Content is created and modified by Content Developers.

Packs may already be created and modified by Composers.

---

# 14. Pack Model

Packs are primarily declarative manifests over other Vapor Content.

## 14.1 Enginepack

Contains:

* Exactly one Engine.
* Any number of compatible Engine Mods.

It is a reusable composition fragment.

It cannot independently become a runnable Vapor App.

---

## 14.2 Gamepack

Contains:

* Exactly one Game.
* Any number of compatible Game Mods.

It is a reusable composition fragment.

It cannot independently become a runnable Vapor App.

---

## 14.3 Modpack

Contains:

* Engine Mods.
* Game Mods.
* Extension Mods.

Its dependency chains must ultimately resolve against the effective Engine and/or Game of the containing composition.

It is a reusable composition fragment.

It cannot independently become a runnable Vapor App.

---

## 14.4 Packagepack

A Packagepack defines the entire composition.

It is the only pack type that can be resolved and built into a complete Vapor App.

That makes the Packagepack both:

* A source-distributed declarative artifact.
* The identity/root of the built finished composition.

---

# 15. Source Distribution Model

Vapor Content is fundamentally source-distributed.

The current model does not require a separate Vapor intermediate representation.

Published behavioral content therefore provides source material sufficient for downstream composition builds.

Published packs provide their declarative source/manifest representation.

This allows Composers and Content Developers to acquire published material and rebuild new Packagepack compositions from source.

The major exception in distribution behavior is the finished Packagepack/Vapor App path:

> A Packagepack intended for Players must also provide suitable prebuilt Vapor App artifacts for supported targets.

Players consume those prebuilt artifacts.

They do not rebuild them locally.

---

# 16. Static Build Model

Vapor compositions are fundamentally static build units.

An App Composition is logically built as one complete composition.

The effective Engine/Game/Mod code forms the final monolithic native composition artifact used by the Engine executable.

Changing the effective composition requires rebuilding the composition.

This does **not** mean that every source file must always be recompiled from scratch.

Vapor and Cargo should use:

* Dependency caches.
* Incremental compilation.
* Build caches.
* Reusable unchanged intermediates.

The important invariant is logical rather than wasteful:

> **The runnable result represents one complete statically resolved composition, not a runtime collection of independently injected mods.**

Dynamic runtime assembly of separately compiled content modules is not the current Vapor model.

---

# 17. Player Distribution Model

Players do not have a build toolchain.

A Player therefore receives a prebuilt Vapor App suitable for the current target.

Relevant target dimensions may include:

* Operating system.
* CPU architecture.
* Applicable ABI/toolchain constraints.
* Other future platform distinctions.

Conceptually:

> Discover Vapor App
> → Resolve Packagepack publication
> → Acquire target-compatible prebuilt Vapor App
> → Verify
> → Install/Register Locally
> → Select
> → Launch

No local composition build occurs.

---

# 18. Composer Build Model

A Composer works from source-distributed Vapor Content.

The normal conceptual flow is:

> Discover Content
> → Acquire Source/Dependencies
> → Create/Modify Packs
> → Resolve Packagepack
> → Build Complete Composition
> → Produce Deployable Vapor App
> → Install/Register Locally
> → Play/Test
> → Optionally Publish

A Composer can therefore create an entirely new Vapor App without having created a new Engine, Game, or Mod.

The build result has two layers:

1. Disposable/cacheable build intermediates.
2. A locally installed/registered Vapor App that is a first-class runnable result.

---

# 19. Content Development Lifecycle

Content Development contains an additional lifecycle beneath the broader composition lifecycle.

Conceptually:

> Create/Open Content Project
> → Edit/Configure Content
> → Build
> → Run in a Composition
> → Inspect
> → Change
> → Repeat
> → Publish

The exact development loop remains incomplete because it depends on later decisions about:

* Project creation.
* Workspace management.
* IDE integration.
* SDK editing.
* Local dependency selection.
* Test composition management.
* Debugging.
* Diagnostics.

Those details should be designed as a coherent workflow rather than piecemeal implementation decisions.

---

# 20. Repository and Workspace Model

The current development-storage model consists of:

> Vapor Superworkspace
> → Container Repo
> → Source Repo / Vapor Workspace
> → Vapor Project

A Vapor Superworkspace is a disposable local checkout container.

A Container Repo groups related Vapor Workspaces using Git submodules.

A Vapor Workspace is the actual source-bearing Git repository.

A Vapor Project is a Rust/Cargo workspace within a Vapor Workspace.

This model currently supports:

* Vapor Root development.
* Vapor Server Root development.
* Vapor Content development.

The precise UX for:

* Creating these structures.
* Cloning them.
* Adopting existing repositories.
* Opening projects.
* Managing submodules.
* Selecting target App Instances.
* Handling local Git state.

remains an open design area.

Vapor should eventually make the intended structure easy to use without pretending that Git does not exist.

---

# 21. Git, GitHub, Steam, and Vapor

These systems have different responsibilities.

## Git

Git is part of Composer and Developer source workflows.

It is used to manage source-bearing Vapor repositories and content collections.

Vapor may automate Git operations, but should not conceptually replace Git.

---

## GitHub

GitHub is an external Git hosting/collaboration provider used by the current ecosystem.

GitHub authentication is not required merely to build or develop locally.

It becomes relevant for operations such as:

* Remote repository access.
* Push/pull.
* Collaboration.
* Issues.
* Publishing workflows that use GitHub.
* Official Vapor repository authorization.

---

## Steam Workshop

Steam Workshop distributes Vapor Content and finished Vapor App artifacts.

Vapor treats Workshop as a relatively dumb external storage/distribution layer whose native identity is an opaque numeric Workshop Item ID.

---

## Vapor Content Registry

The Vapor Content Registry provides Vapor's semantic identity layer.

It maps human-readable Vapor IDs/namespaces to the corresponding external Workshop identities.

Conceptually:

> Vapor Content ID
> → Vapor Content Registry
> → Steam Workshop Item ID
> → Distributed Vapor Content

The registry may additionally participate in:

* Ownership.
* Steam/GitHub account linkage.
* Authorization.
* Publication metadata.

The exact server-side identity model remains an open design area.

---

# 22. Publishing Boundaries

Publishing capability follows the same capability hierarchy.

A Composer may publish:

* Packagepacks.
* Enginepacks.
* Gamepacks.
* Modpacks.
* Complete finished compositions derived from Packagepacks.

A Content Developer may additionally publish:

* Engines.
* Games.
* Engine Mods.
* Game Mods.
* Extension Mods.

Publishing should validate an artifact before it becomes part of the public Vapor ecosystem.

Published Packagepacks should not represent unresolved or structurally invalid compositions.

The exact publication pipeline—including versioning, ownership, target builds, remote Git coordination, signing, updates, and deprecation—is not yet sufficiently designed to specify here.

Those details should be resolved later as one coherent publishing model.

---

# 23. Persistent Local State

Vapor needs to preserve several broad kinds of local state.

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

* Installed Vapor Content.
* Installed Vapor Apps.
* Current/default Vapor App.
* Launcher settings.
* Account state.
* Capability state.

## Build State

For Composer and Developer capability:

* Source/dependency caches.
* Cargo caches.
* Incremental compilation state.
* Build intermediates.
* Built Vapor Apps.

## Development State

Examples:

* Repositories.
* Workspaces.
* Projects.
* Superworkspaces.
* Local changes.
* Development configuration.

The exact filesystem ownership boundaries between these categories remain to be formalized.

---

# 24. Progressive Disclosure

Vapor should not expose all complexity to all users.

A Player should see a game ecosystem.

A Composer should additionally see a content ecosystem.

A Content Developer should additionally see a development ecosystem.

An Ecosystem Developer should additionally see Vapor itself as a development target.

This progression should be visually and conceptually strong.

The same Launcher may therefore feel substantially more capable after an Installer-mediated capability upgrade.

Complexity should appear because the user has acquired a reason to interact with it.

---

# 25. Automation and Transparency

Vapor should automate incidental setup aggressively where the intended result is unambiguous.

Examples include:

* Tool detection.
* Tool configuration.
* Dependency acquisition.
* Composition resolution.
* Build orchestration.
* Cache management.
* Launch orchestration.
* Routine environment validation.

Vapor should be considerably more conservative around:

* Destructive source operations.
* Overwriting local changes.
* Publishing.
* Authentication.
* Irreversible migration.
* Root-level infrastructure operations.

Players and Composers should normally see simple high-level outcomes.

Content and Ecosystem Developers should be able to inspect increasingly low-level detail when needed.

Raw underlying tool output should remain reachable for advanced diagnosis even when Vapor also presents a friendlier interpretation.

---

# 26. Ownership Principle

Vapor should orchestrate external systems without pretending to own everything they contain.

Broadly:

* Steam owns Steam installation/distribution state.
* Git owns Git repository mechanics.
* Git hosting providers own their remote repositories/authentication.
* Rust/Cargo own the Rust build ecosystem.
* External IDEs own their editing/debugger environments.
* Vapor owns the coherent model and orchestration connecting those pieces.

The exact line between Vapor-managed and user-managed files must eventually be made explicit.

The guiding principle is:

> **Vapor should own the experience and the invariants of the ecosystem without unnecessarily owning every underlying tool or every byte of user source.**

---

# 27. Golden Paths

The intended happy paths currently look approximately like this.

## Player

> Install Steam App
> → Default Vapor App becomes available
> → Play

or:

> Start Vapor
> → Discover Vapor App
> → Install
> → Select
> → Play

---

## First-Time Composer

> Start Installer
> → Establish Composer Capability
> → Start Vapor
> → Discover Content
> → Compose Packagepack
> → Build
> → Play

---

## Returning Composer

> Start Vapor
> → Open Existing Packagepack
> → Modify Composition
> → Rebuild
> → Play
> → Optionally Publish

---

## First-Time Content Developer

> Start Installer
> → Establish Content Developer Capability
> → Start Vapor
> → Enter Development/SDK
> → Create/Open Content
> → Develop
> → Build/Run
> → Inspect
> → Repeat

---

## Ecosystem Developer

> Establish Authorized Ecosystem Environment
> → Obtain Official Vapor Sources
> → Modify Vapor
> → Build/Test
> → Integrate/Push
> → Deploy to Development Infrastructure Where Applicable

The detailed implementations of these paths remain future work.

---

# 28. Current Non-Goals

The current Vapor model does not attempt to support:

* User-hosted Vapor Content Registries.
* Arbitrary user-created parallel App Instances.
* Runtime dynamic injection of independently built Mods as the core composition model.
* A separate ontology for "development content".
* A separate role for every Engine/Game/Mod/Server/Launcher developer specialization.
* Replacing Git with a Vapor-specific version-control system.
* Replacing Cargo/Rust with a Vapor-specific compiler ecosystem.
* Forcing Players to install development tooling.
* Requiring GitHub authentication merely to develop or build locally.

These may be revisited only when concrete requirements justify changing the model.

---

# 29. Open Design Areas

The following areas remain intentionally unresolved.

They are important, but the current ecosystem model does not depend on settling them immediately.

## Launcher UX

* Final navigation.
* Exact surfaces and tab structure.
* Search/filtering.
* Player vs Composer presentation.
* Detailed Library UX.

## SDK UX

* Source editor vs external IDE boundary.
* Project creation experience.
* Run configurations.
* Debugging.
* Testing.
* Diagnostics.
* Code generation.
* Configuration tooling.

## Development Storage UX

* Superworkspace creation.
* Repository adoption.
* Git/submodule automation.
* Project discovery.
* Local source ownership.
* Workspace recovery.

## Publishing

* Exact remote Git workflow.
* Versioning.
* Release immutability.
* Platform artifact production.
* Build signing.
* Ownership transfer/collaboration.
* Deprecation/removal.
* Update publication.

## Updates and Migration

* Content updates.
* Vapor App updates.
* Toolchain updates.
* Compatibility rules.
* Version pinning.
* Rollback.
* Migration policy.

## Failure and Recovery

* Partial downloads.
* Registry outages.
* Steam outages.
* Git failures.
* Broken builds.
* Corrupt content.
* Interrupted operations.
* Invalid compositions.
* Protection of local changes.

## Root/Ecosystem Development

* Internal deployment model.
* Dev branches/environments.
* Root-only operations.
* Production safeguards.

These should be designed when they become the next implementation constraint, not merely because an exhaustive questionnaire can imagine questions about them.

---

# 30. Design Baseline

The current committed direction can be summarized as follows:

1. Vapor is entered through one Loo Cast Steam App.
2. That Steam App provides access to multiple launchable Vapor Apps.
3. A Packagepack declaratively defines a complete composition.
4. Resolving it produces an App Composition.
5. Building/deploying that composition produces a Vapor App.
6. Vapor Apps are complete statically built compositions.
7. Players consume prebuilt Vapor Apps.
8. Composers compose existing source-distributed Vapor Content and build new Vapor Apps.
9. Content Developers additionally author Engines, Games, and Mods.
10. Ecosystem Developers additionally develop Vapor itself.
11. Root Authority represents ultimate official ecosystem ownership.
12. Capability levels are cumulative.
13. External authorization gates operations rather than defining local capability.
14. The Installer changes installed capability.
15. The Launcher uses installed capability.
16. The SDK is the behavioral-content development environment inside the Launcher.
17. Git enters the experience at Composer level.
18. Rust/Cargo enter the experience at Composer level because composition builds are static.
19. GitHub is provider-specific and is not required for local development.
20. Steam Workshop provides external distribution containers.
21. The Vapor Content Registry provides human-readable Vapor identity and registry semantics.
22. Individual Vapor Content is source-distributed.
23. Enginepacks, Gamepacks, and Modpacks are declarative composition fragments.
24. Only Packagepacks define complete buildable Vapor Apps.
25. Locally present content is simply local content; dependencies and "development content" do not require separate ontological categories.
26. Multiple Vapor Apps may coexist locally.
27. Selecting a Vapor App primarily changes the current/default launch target.
28. Vapor should hide incidental complexity while retaining transparency for users who need it.
29. Unresolved lower-level workflows should be designed coherently when required rather than prematurely specified piecemeal.
30. This Experience Model is the primary UX/DX bridge between the Ecosystem Glossary and future system-level designs and TDDs.
