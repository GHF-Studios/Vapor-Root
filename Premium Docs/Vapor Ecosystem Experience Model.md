
> [!info]
> This document defines the high-level **User Experience (UX)** and **Developer Experience (DX)** of the Vapor Ecosystem.
>
> It sits between the **Ecosystem Model Glossary** and the more focused operational, development, publishing/distribution, architecture, and implementation documents.
>
> This document answers:
>
> * Who uses Vapor?
> * What are they trying to accomplish?
> * What concepts should they encounter?
> * What complexity should Vapor expose or hide?
> * How should the ecosystem feel as capability increases?
>
> Detailed state transitions and lifecycle semantics belong in the **Vapor Ecosystem Operational Model**.
>
> Detailed authoring and source-development workflows belong in the **Vapor Development Experience Model**.
>
> Detailed publication, source distribution, built distribution, and Registry behavior belong in the **Vapor Publishing and Distribution Model**.

---

# Purpose

Vapor exists to make a complex Rust/Cargo/Git/Steam-based content ecosystem behave like one coherent product.

Users should primarily interact with **Vapor concepts**, not with the incidental complexity of the tools used underneath them.

At a very high level:

> **Player:** consume complete compositions.

> **Composer:** assemble complete compositions from existing content.

> **Content Developer:** create and modify the behavioral content that compositions are assembled from.

> **Ecosystem Developer:** create and maintain Vapor itself.

The implementation may involve:

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
* Build systems and caches.
* Multiple repositories and workspaces.
* Vapor server infrastructure.

Those systems should remain accessible and inspectable where appropriate.

They should not dictate the ordinary mental model of Vapor.

The guiding principle is:

> **Expose the work the user actually intends to perform; automate the infrastructure required to make that work possible.**

Vapor is therefore neither:

* A thin graphical wrapper over Cargo.
* A replacement for every external development tool.
* A black box that prevents advanced users from understanding what happened.

Vapor is the semantic, experiential, and orchestration layer that turns the underlying systems into one ecosystem.

---

# Experience Principles

## Steam-First Player Experience

The normal Player experience begins and remains within Steam and Vapor.

Playing Loo Cast must not require:

* Visiting GitHub.
* Installing Git.
* Installing Rust.
* Installing Cargo.
* Installing SteamCMD.
* Understanding repositories.
* Understanding composition builds.
* Editing configuration files manually.

The default Steam installation should behave like a conventional, immediately playable Steam product.

---

## Progressive Capability

Vapor exposes progressively larger mental models as capability increases.

> **Player**
> sees Vapor Apps.

> **Composer**
> additionally sees Vapor Content, packs, source, dependencies, and composition.

> **Content Developer**
> additionally sees behavioral source, projects, programming, SDK tooling, build/test loops, and diagnostics.

> **Ecosystem Developer**
> additionally sees Vapor itself as an editable and deployable system.

> **Root Authority**
> additionally possesses ultimate administrative authority over the official ecosystem.

Higher capability is not merely a larger permission set.

Each capability level introduces new kinds of work and therefore new concepts that legitimately need to become visible.

---

## Golden Paths over Incidental Infrastructure

Vapor should provide strongly supported normal paths for common tasks.

A Composer should normally be able to express:

> Build this Packagepack.

rather than manually expressing:

> Find every repository, ensure every checkout is correct, configure the Rust toolchain, invoke Cargo in the correct workspace, locate the resulting artifacts, package them correctly, install them into the local Steam App Instance, update Vapor's local records, then launch the correct executable.

Those underlying operations may still happen.

Vapor should orchestrate them.

---

## Advanced Transparency

Abstraction must not require opacity.

As capability increases, users should gain increasing access to:

* Raw Git state.
* Cargo output.
* Build logs.
* Dependency information.
* Repository locations.
* Diagnostics.
* Generated files.
* Provider-native identifiers.
* Underlying commands where useful.

The default presentation may be Vapor-oriented.

The underlying reality should remain inspectable.

---

## Static Composition with Explicit Runtime Dynamicity

A Vapor App is fundamentally a statically resolved complete composition.

Composition changes normally occur before launch and require rebuilding.

This does not imply that the runtime itself can contain no dynamic systems.

In particular, the Spacetime Engine / USF Capability Model may provide explicitly modeled dynamic attachment points within an otherwise statically structured composition.

Static composition and controlled runtime dynamicity are therefore different concerns.

The composition determines **what runtime system exists**.

That runtime system may itself deliberately expose structured dynamic behavior.

---

## Human-Readable Identity

Users should work primarily with Vapor identities and names rather than provider-specific opaque identifiers.

Steam Workshop Item IDs, repository URLs, commit hashes, Steam IDs, and similar provider-native identifiers remain useful implementation and diagnostic information.

They should not become the ecosystem's primary semantic identity model.

---

# Capability Model

Vapor uses a cumulative capability hierarchy:

> **Player ⊂ Composer ⊂ Content Developer ⊂ Ecosystem Developer ⊂ Root Authority**

Each capability contains every capability below it.

A Content Developer is also a Composer and Player.

An Ecosystem Developer is also a Content Developer.

Root Authority contains every lower capability.

Installed local capability and external authorization are distinct.

A Content Developer may possess a fully working local development environment while currently being unauthenticated to GitHub or another remote provider.

Such a user can still:

* Edit.
* Compose.
* Build.
* Run.
* Test.

Only operations requiring remote authority become unavailable.

Capability establishment and removal belong primarily to the Vapor Installer.

Ordinary use of installed capability belongs primarily to the Vapor Launcher and its contained surfaces.

---

# Steam App and Vapor App Model

## Steam App

There is one Steam-distributed product:

> **Loo Cast**

This is the **Steam App**.

The Steam App is the outer installation and product boundary through which Vapor is entered.

It provides:

* Vapor Installer.
* Vapor Launcher.
* Required Vapor runtime/bootstrap infrastructure.
* The default first-party Loo Cast composition.
* Access to installed additional Vapor Apps.
* Access to higher Vapor capabilities when installed.

The Steam App is not synonymous with one particular composition.

---

## Steam App Instance

A **Steam App Instance** is one concrete local installation of the Steam App.

The normal model assumes:

> One Steam installation of Loo Cast = one Steam App Instance.

The Steam App Instance provides the local product context containing or referencing:

* Steam-managed installation state.
* Vapor application state.
* Installed capability.
* Installed/built Vapor Apps.
* Selected Vapor App Composition.
* Relevant caches and metadata.
* Source/development state where higher capabilities exist.

Users do not currently create arbitrary additional Steam App Instances.

Steam-supported installation movement should be preferred over arbitrary manual copying.

---

## Packagepack

A **Packagepack** is the complete composition artifact.

A valid Packagepack must resolve to:

* Exactly one effective Engine.
* Exactly one effective Game.
* The applicable Engine Mods.
* The applicable Game Mods.
* The applicable Extension Mods.

Those constituents may be expressed directly or through subordinate packs.

A Packagepack is not merely a precursor to some separately authored "finished composition."

It already represents the complete composition.

---

## Vapor App Composition

A **Vapor App Composition** is the effective resolved composition represented by a Packagepack in the context in which it is used by a Steam App Instance.

It represents the effective content graph after composition/dependency resolution.

A Steam App Instance has one selected Vapor App Composition for its current/default launch context.

---

## Vapor App

A **Vapor App** is a built, deployable, runnable realization of a Vapor App Composition for a supported target.

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
> runnable realization

Multiple Vapor Apps may coexist locally.

The conceptual relationship is illustrated in [Vapor Composition Model](./Diagrams/Vapor%20Composition%20Model.puml).

---

# Player Experience

A Player consumes finished Vapor Apps.

The normal Player wants to:

* Install Loo Cast through Steam.
* Launch the default Loo Cast Vapor App.
* Discover other complete Vapor Apps.
* Acquire them.
* Install them.
* Select them.
* Launch them.
* Remove them.
* Manage ordinary settings and account state.

A Player is not expected to understand:

* Git.
* Rust.
* Cargo.
* SteamCMD.
* Vapor Workspaces.
* Container Repos.
* Vapor Projects.
* Compilation.
* Source dependency resolution.
* Composition authoring.

The Player-facing discovery unit inside Vapor is therefore primarily a **complete Vapor App**.

Individual Engines, Games, Mods, and subordinate packs are not normal Player-facing discovery objects because the Player cannot meaningfully compose them inside Vapor.

The normal Player path should deal with built artifacts only.

---

# Composer Experience

A Composer is a Player who can create and modify compositions from existing Vapor Content.

A Composer may author:

* Packagepacks.
* Enginepacks.
* Gamepacks.
* Modpacks.

A Composer may consume existing:

* Engines.
* Games.
* Engine Mods.
* Game Mods.
* Extension Mods.

A Composer may not modify those behavioral artifact types as part of Composer capability.

The fundamental distinction is:

> **Composer:** selects and combines behavior.

> **Content Developer:** creates or changes behavior.

Composer capability introduces:

* Source-backed Vapor Content.
* Git.
* Packs.
* Dependencies.
* Compatibility.
* Composition resolution.
* Local builds.
* Publication.
* More detailed local state.

A Composer should primarily reason about Vapor composition concepts rather than Cargo invocation mechanics.

Typical Composer work includes:

* Discovering source-side Vapor Content.
* Acquiring appropriate source repositories.
* Creating or opening packs.
* Selecting compatible content.
* Resolving a Packagepack.
* Building the complete composition.
* Installing/registering the resulting Vapor App.
* Running/testing it.
* Publishing authored packs and Packagepacks.

The detailed source and repository experience is specified in the **Vapor Development Experience Model**.

The detailed publication path is specified in the **Vapor Publishing and Distribution Model**.

---

# Content Developer Experience

A Content Developer creates or modifies behavioral and reusable implementation Vapor Content.

This includes:

* Engine.
* Game.
* Engine Mod.
* Game Mod.
* Extension Mod.
* Library.

Engine Developer, Game Developer, Library Developer, and Mod Developer are specializations of Content Development rather than separate fundamental Roles.

Content Developer capability introduces active implementation/source authoring.

The developer now legitimately encounters:

* Vapor Superworkspaces.
* Workspaces.
* Projects.
* Content identities.
* Local realizations.
* Source editing.
* Programming.
* Cargo/Rust integration.
* Configuration.
* Semantic dependency resolution.
* Build/run/test iteration.
* Diagnostics.
* Git/provider state.
* SDK functionality.
* More direct underlying-tool visibility.

The intended experience is:

> **Develop the Content, not the machinery required to persuade Rust, Cargo, Bevy, Git, Steam, and Vapor to cooperate.**

Vapor should provide:

* Strong structural identity.
* Human-readable semantic selection.
* Guardrails.
* Minimal boilerplate.
* Automated environment integration.
* Vapor-aware build/run/test.
* Useful diagnostics.
* Explicit relationships between Projects and Content.
* Access to underlying Git/Cargo/provider reality when useful.
* A graphical SDK which makes the model discoverable rather than forcing developers to memorize shell syntax.

The Vapor SDK is the primary first-party environment for this experience.

External IDEs remain compatible and useful.

They are complementary rather than a reason for Vapor's own SDK to remain permanently shallow.

---

# Ecosystem Developer Experience

An Ecosystem Developer develops Vapor itself.

This may include:

* Vapor Installer.
* Vapor Launcher.
* Vapor SDK.
* Vapor CLI.
* Vapor Core.
* Vapor Root framework.
* Vapor server infrastructure.
* Vapor Content Registry.
* Identity/authentication infrastructure.
* Diagnostics.
* Toolchain integration.
* Deployment tooling.
* Provider integrations.

Ecosystem Developer is the highest ordinary locally installed Vapor Role.

It does **not** inherently grant official authority.

An Ecosystem Developer may locally:

* acquire Vapor ecosystem source;
* fork it;
* create independent source;
* modify Vapor;
* build/test Vapor;
* develop publication/deployment machinery;
* develop authentication/authorization machinery;
* run local equivalents of protected workflows where feasible.

Protected external operations separately require authorization.

Examples include:

* pushing to official repositories;
* publishing into official namespaces;
* deploying official Steam branches/depots;
* modifying production Registry/server infrastructure.

The desired Ecosystem Developer experience is the same integrated SDK model used for Content Development, expanded by Role to expose Vapor itself as editable source.

Conceptually:

```text
Enter Development
    ↓
open/resume Vapor Superworkspace
    ↓
navigate Vapor Workspaces / Projects
    ↓
modify
    ↓
build / test
    ↓
inspect diagnostics
    ↓
commit / push where authorized
    ↓
deploy local / Steam where authorized
    ↓
validate
```

Vapor should progressively absorb the manual machinery required to reproduce and develop itself.

---

# Root Authority

Root Authority is not a locally installed capability level above Ecosystem Developer.

It is an authority state.

A Root Authority normally uses Ecosystem Developer capability together with ultimate authorization over protected official Vapor targets.

Root Authority may authorize operations involving:

* official repositories;
* official namespaces;
* Registry administration;
* production deployment;
* authorization systems;
* root-level recovery;
* ownership state;
* destructive protected administrative operations.

Role and Authority remain orthogonal.

---

# Development Context Experience

Developers should not need to think in raw filesystem roots once Vapor already understands their source.

The product should favor:

```text
Superworkspace
Container Repo identity
Workspace identity
Project identity
Content identity
local realization
```

over repeated path entry.

Paths remain useful for:

* initial import;
* Locate/recovery;
* diagnostics;
* explicit advanced overrides.

The graphical SDK should make context visible through navigation and selection.

The CLI should use strong identities/selectors and deterministic resolution.

The detailed rules are defined in the **Vapor Context, Identity, Session And Selection Model**.

---

# Vapor Applications

## Vapor Installer

The Vapor Installer changes which fundamental capabilities and managed tools exist in the Steam App Instance.

Its responsibility is:

> **Change what Vapor is equipped to do on this Installation.**

Examples include:

* establish Composer capability;
* detect/install/configure Git;
* install/configure Rust/Cargo;
* install/configure SteamCMD;
* establish Content Developer capability;
* establish Ecosystem Developer prerequisites;
* downgrade/remove higher Role capability;
* repair Installation-level capability state;
* prepare correct Vapor uninstall behavior where required.

The Installer remains a distinct application boundary.

SDK Mode is not a superset of Installer semantics.

---

## Vapor Launcher

The Vapor Launcher is the primary ordinary Vapor desktop surface.

Its responsibility is:

> **Operate the Vapor ecosystem using the capabilities already installed.**

Launcher Mode may expose:

* Play.
* Vapor Apps.
* Content Library.
* Vapor App selection.
* discovery.
* Packagepack/composition workflows.
* accounts.
* settings.
* diagnostics.
* logs.
* publication/management surfaces appropriate to Role.

Where development capability exists, the Launcher additionally offers an **Enter Development** transition.

---

## Vapor SDK / Development Mode

The Vapor SDK is the integrated development mode of the Vapor Launcher.

The preferred product experience is:

```text
Vapor Launcher
    ↓ Enter Development
Vapor SDK
    ↑ Return to Launcher
```

This may substantially reconfigure the visible application.

A literal animation or process-level transformation is not required.

The semantic requirement is:

> **SDK Mode is a development-oriented superset of relevant Launcher functionality.**

The SDK may expose:

* everything relevant from ordinary Launcher Mode;
* Superworkspace Explorer;
* Container Repo / Workspace / Project navigation;
* Content development;
* source editing;
* structured manifests/configuration;
* composition/dependency graphs;
* Inspector;
* Problems;
* Build;
* Test;
* Run;
* Git/VCS;
* toolchain;
* Cargo output;
* logs;
* publication;
* Steam development/deployment;
* ecosystem development where Role permits.

Returning to Launcher Mode should hide/collapse development surfaces while preserving enough Resume State to continue later.

---

# SDK Superset Principle

The progression between Launcher and SDK resembles Vapor's cumulative capability philosophy.

Conceptually:

```text
Launcher surface
    ⊂
SDK / Development surface
```

This does not mean every Launcher control must remain visible at every moment in SDK layout.

It means the SDK belongs to the same product model and does not require an unrelated semantic implementation.

The Installer remains outside this particular superset relationship because it changes installed capability itself.

---

# SDK-First Rewrite Strategy

The rewrite may implement the rich SDK surface before the simplified final Launcher surface.

This is acceptable.

The SDK exercises more of Vapor Core:

```text
identity
context
source
Git
toolchain
Cargo
Content
composition
build
test
run
diagnostics
publication
deployment
```

A mature Launcher can later be realized largely as a simpler projection of the same desktop/Core architecture.

This reduces the risk of building a Launcher architecture which later resists becoming a real development environment.

---

# SDK Visual Experience

The existing Figma-derived Vapor SDK GUI prototype is the visual ancestor of the intended SDK.

Its fake data and obsolete semantic assumptions are not normative.

Its visual character should nevertheless be preserved where practical.

Especially valuable qualities include:

* JetBrains/RustRover-like professional density.
* Dark neutral surfaces.
* Subtle borders and separators.
* Tight but readable spacing.
* Small restrained corner radii.
* Compact toolbars.
* Clear tree-selection treatment.
* Semantic green/warning/red status.
* Monospace identity/tool output where useful.
* Central editor/work area.
* Left-side Explorer.
* Right-side contextual Inspector.
* Bottom Problems/Build/tool windows.
* Integrated graph visualization.

The final implementation should extract these qualities into a real Vapor design system rather than treating the generated prototype component as permanent architecture.

---

# SDK Interaction Experience

The SDK should exploit GUI-native interaction.

It may use:

* tree navigation;
* tabs;
* multi-selection;
* graph selection;
* Inspector actions;
* context menus;
* dialogs;
* drag/drop where semantically meaningful;
* rich errors and recovery controls.

These interactions construct the same typed Vapor Core operations exposed by CLI/automation.

A click is not a different semantic implementation.

---

# Open, Focused, and Selected

The SDK must distinguish:

```text
Open
    participates in the working set

Focused
    contributes durable default targeting intent

Selected
    transient current GUI target
```

Multiple Workspaces/Projects/Content objects may be Open.

Multiple may be Focused.

An immediate multi-selection may target one operation without rewriting durable Focus.

This makes large development environments practical without forcing one artificial globally active Project.

---

# Broken-State Experience

Development objects should not disappear merely because their local realization is unhealthy.

A previously Open Workspace may remain visible as:

```text
Vapor-Examples
    Missing local realization
```

with actions such as:

```text
Locate
Reacquire
Repair
Close
Forget
```

This preserves the user's mental workspace and makes failures discoverable.

---

# Operation Experience

Operations should be enabled according to semantic target compatibility.

For example:

```text
three selected Projects
+ operation accepts Many<Project>
→ enabled
```

while:

```text
three selected Projects
+ operation requires ExactlyOne<Project>
→ ambiguity / target choice required
```

The GUI should explain disabled operations rather than hiding the underlying rule.

The CLI reports the same semantic problem textually.

---

## Vapor CLI

The Vapor CLI is the command-line and automation-oriented projection of Vapor Core.

Its primary users include:

* Content Developers.
* Ecosystem Developers.
* automation.
* coding agents.
* advanced users.

The CLI should be excellent at:

* canonical identities;
* unambiguous shorthand selectors;
* deterministic operation targeting;
* machine-readable output;
* explicit failure semantics;
* scripting;
* automation.

The CLI does not need to imitate every graphical SDK interaction.

Likewise, SDK UX must not be degraded merely because a graphical interaction has no elegant one-line shell equivalent.

The governing principle is:

> **GUI and CLI share capabilities and semantics, not interaction mechanics.**

Both should ultimately resolve exact targets and invoke the same Vapor Core operations.

---

# Steam Entry Points

The Steam App should expose three conceptual entry points:

* **Play Loo Cast**
* **Start Vapor**
* **Start Installer**

## Play Loo Cast

This directly launches the default first-party Vapor App.

The default composition should already exist after Steam installation.

No source acquisition or local compilation should be required.

## Start Vapor

This launches the Vapor Launcher.

## Start Installer

This launches the Vapor Installer for capability establishment, removal, repair, and environment configuration.

---

# Default Composition

The default first-party Packagepack is:

> **Loo Cast Packagepack**

It currently resolves to at least:

* Spacetime Engine.
* Loo Cast Game.

The default built composition should be shipped directly through the Steam depot.

The purchased Steam App must therefore not depend on Steam Workshop merely to become playable.

Steam Workshop is used for additional built published compositions, not for bootstrapping the default Steam purchase.

---

# Vapor Content

Vapor currently models nine primary content artifact types.

## Complete Composition

* Packagepack.

## Reusable Pack Fragments

* Enginepack.
* Gamepack.
* Modpack.

## Behavioral Content

* Engine.
* Game.
* Engine Mod.
* Game Mod.
* Extension Mod.

A Packagepack represents exactly one complete composition.

An Enginepack, Gamepack, or Modpack represents a reusable composition fragment.

The effective Engine declares the composition's main binary.

The Game does not declare the main binary.

---

# Static Build Model

Vapor Apps are built as complete static compositions.

Changing the effective Packagepack composition requires rebuilding the resulting Vapor App.

The logical build scope is the complete composition.

This does not require wastefully recompiling all unchanged source.

Vapor and Cargo should make use of:

* Incremental compilation.
* Dependency caches.
* Build caches.
* Reusable intermediates.

The important invariant is:

> **The runnable Vapor App represents one statically resolved composition.**

Normal launch does not dynamically assemble the composition from independently built Mods.

Runtime systems may still deliberately support structured dynamic behavior inside that already-built composition.

---

# Progressive Disclosure

Vapor should expose complexity because the user's capability gives that complexity meaning.

A Player should primarily see:

* Vapor Apps.
* Play.
* Discovery.
* Installation.
* Settings.

A Composer should additionally see:

* Vapor Content.
* Packs.
* Source.
* Composition.
* Builds.
* Publication.

A Content Developer should additionally see:

* Projects.
* Programming/configuration.
* SDK.
* Diagnostics.
* Build/test internals.

An Ecosystem Developer should additionally see:

* Vapor's own repositories.
* Infrastructure.
* Internal deployment and integration.

This progression should feel substantial.

---

# High-Level Experience Guarantees

* A Player does not require Git.
* A Player does not require Rust/Cargo.
* A Player does not build compositions.
* The default Loo Cast composition ships in the Steam depot.
* Third-party Player-facing distribution uses built complete compositions.
* A Packagepack represents one complete composition.
* The effective Engine declares the composition's main binary.
* Vapor Apps are statically resolved complete compositions.
* Composer capability is required for pack authoring.
* Content Developer capability is required for behavioral-content authoring.
* Capability levels are cumulative.
* Local capability and remote authorization are distinct.
* Capability establishment belongs primarily to the Installer.
* Ordinary capability use belongs primarily to the Launcher.
* Vapor should automate incidental infrastructure while retaining advanced transparency.
* Vapor must not silently destroy user-authored source.

Detailed operational invariants belong in the **Vapor Ecosystem Operational Model**.

---

# Related Models

This document intentionally does not attempt to encode every workflow or state permutation.

Use:

* **Vapor Ecosystem Operational Model** for situations, conditions, contexts, transitions, lifecycle projections, state interaction, local-state ownership, and operational invariants.
* **Vapor Development Experience Model** for Git, repositories, Superworkspaces, Workspaces, Projects, Composer development, Content Development, SDK/IDE integration, and build/run/test iteration.
* **Vapor Publishing and Distribution Model** for Git-backed source publication, Registry linkage, built Vapor App publication, Steam Workshop, Player acquisition, versioning, and publishing lifecycle.
* **Ecosystem Model Glossary** for compact canonical term definitions.
