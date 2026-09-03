
> [!info]
> This document defines how Vapor source and built complete compositions become externally available.
>
> The central distinction is:
>
> **Git-backed infrastructure distributes and preserves source.**
>
> **Steam Workshop distributes built complete compositions to Players.**
>
> The Vapor Content Registry provides the semantic identity/linkage layer connecting those systems.

---

# Core Distribution Principle

Vapor has two fundamentally different distribution concerns.

## Source Distribution

Source exists in Vapor-compatible Git repositories.

This includes source for:

* Packagepacks.
* Enginepacks.
* Gamepacks.
* Modpacks.
* Engines.
* Games.
* Engine Mods.
* Game Mods.
* Extension Mods.

Source-side distribution supports:

* Composition.
* Development.
* Collaboration.
* Rebuilding.
* Version history.
* Publication of authored changes.

Steam Workshop is not the canonical source store.

---

## Built Distribution

Players do not build Vapor compositions.

Player-facing distribution therefore provides built Vapor Apps.

Only a complete Packagepack can produce such a Vapor App.

Steam Workshop is the intended distribution backend for additional published built complete compositions.

The default first-party Loo Cast composition is a special case:

> It is shipped directly inside the Steam depot.

It does not need to be acquired from Workshop merely to make the purchased Steam App playable.

---

# Packagepack Publication Semantics

A Packagepack is already the complete composition artifact.

Publication must not invent an additional authored entity called:

> finished composition

or:

> published complete composition

as though it were separate from the Packagepack.

Instead:

* Packagepack source defines the complete composition.
* Resolving the Packagepack produces the effective Vapor App Composition.
* Building that composition produces target-specific Vapor App artifacts.
* Those built artifacts are distributed to Players.

Conceptually:

> **Packagepack source**
> ↓
>
> **Vapor App Composition**
> ↓
>
> **Vapor App build(s)**
> ↓
>
> **Player-facing built publication**

The overall publication topology is illustrated in [Vapor Publication Model](./Diagrams/Vapor%20Publication%20Model.puml).

---

# Source Publication Model

Vapor source publication is Git-backed.

A source publication minimally needs:

* A Vapor identity.
* Vapor-compatible source.
* An appropriate Git repository location.
* Sufficient repository/version identity to retrieve the intended source.
* Required ownership/authorization.
* Valid Vapor metadata.

The exact relationship between one Vapor Content identity and repository structure remains open.

One repository may contain one or more Vapor Projects.

A Container Repo may group multiple source-bearing Workspaces.

The Registry should provide enough semantic linkage that users do not need to manually reconstruct the repository topology from opaque URLs.

---

# Source Publication by Capability

## Composer

A Composer may publish source for:

* Packagepack.
* Enginepack.
* Gamepack.
* Modpack.

A Composer may not publish a new authored Engine/Game/Mod implementation unless they also possess Content Developer capability.

---

## Content Developer

A Content Developer may additionally publish source for:

* Engine.
* Game.
* Engine Mod.
* Game Mod.
* Extension Mod.

---

## Ecosystem Developer

An Ecosystem Developer may additionally publish/deploy changes to Vapor's own official code and infrastructure according to official ecosystem workflows.

This is not the same as ordinary Vapor Content publication.

---

# Source Identity

A published Vapor Content artifact should have a stable human-readable Vapor identity.

That identity should not be synonymous with:

* Repository URL.
* Git commit SHA.
* GitHub repository numeric ID.
* Steam Workshop Item ID.

Those are external/provider identities.

The Vapor identity provides the semantic anchor.

The Registry links that identity to appropriate external resources.

---

# Source Version Resolution

Source publication eventually requires a precise answer to:

> Which version of this source does a dependent composition mean?

Possible mechanisms include:

* Git commit identity.
* Tag/release identity.
* Vapor-level version identity linked to a Git commit.
* Version constraints resolved by the Registry.

The exact scheme is not yet decided.

However, a reproducible published Packagepack build ultimately needs deterministic enough source resolution that its intended composition can be reconstructed.

This is a major future design requirement.

---

# Registry Role

The Vapor Content Registry is not merely a Workshop-ID lookup table.

Its role is broader:

> **Provide one semantic Vapor identity/linkage model over external backing systems.**

The Registry may associate a Vapor identity with:

* Git source location.
* Repository identity.
* Source version metadata.
* Steam Workshop publication.
* Ownership.
* Steam identity.
* Git-host identity.
* Compatibility metadata.
* Publication metadata.

The exact schema remains open.

---

# Registry vs Git

Git remains the canonical version-control/source system.

The Registry should not duplicate Git history as a second version-control system.

Instead, the Registry provides semantic information needed to locate and interpret Vapor source.

Conceptually:

> Vapor ID
> → Registry
> → source location/version metadata
> → Git

---

# Registry vs Steam Workshop

Steam Workshop remains a built-artifact distribution backend.

The Registry gives Vapor semantic meaning to Workshop publications.

Conceptually:

> Vapor Packagepack identity
> → Registry
> → appropriate built publication metadata
> → Steam Workshop Item/artifact

Players should not need to reason directly about numeric Workshop IDs.

---

# Build Boundary

A built Player-facing publication must originate from a valid complete Packagepack.

The broad build process is:

> Resolve Packagepack
> → acquire exact required source
> → validate composition
> → compile complete composition
> → produce target-specific Vapor App
> → package for distribution

Only the Packagepack represents enough composition information to produce a complete Vapor App.

Enginepacks, Gamepacks, and Modpacks are not independently Player-distributable runnable units.

---

# Target-Specific Builds

A published Packagepack may require multiple built realizations.

Potential target dimensions include:

* Operating system.
* CPU architecture.
* ABI.
* Toolchain/runtime compatibility.
* Steam platform requirements.
* Other future target distinctions.

These different built outputs remain realizations of the same Packagepack composition.

They are not automatically separate authored Vapor Content identities.

The exact target matrix and artifact identity rules remain open.

---

# Build Ownership

The publication system eventually needs a clear answer to:

> Who produces trusted Player-facing builds?

Possible models include:

* Local author build uploaded through Vapor.
* Vapor-controlled remote build infrastructure.
* Hybrid local/remote build flow.
* First-party trusted builders for some publication classes.

This is currently unresolved.

The Experience Model should not accidentally assume a specific CI/build-farm architecture.

What is already required is:

* The built output corresponds to a valid Packagepack.
* The output can be associated with the intended source/publication identity.
* Players receive an appropriate supported target build.

---

# Steam Workshop Model

Steam Workshop hosts built published complete compositions.

It does not host canonical individual-content source.

A Workshop publication therefore corresponds conceptually to Player-facing Vapor App distribution.

The exact Workshop representation remains open.

Questions include:

* One Workshop Item per Packagepack?
* One Workshop Item per release?
* One Workshop Item containing multiple target builds?
* Multiple Items linked by one Vapor publication identity?
* How Steam Workshop update mechanics interact with Vapor versions.

These should be resolved together with versioning and target-artifact design.

---

# Steam Depot Model

The default first-party Loo Cast Vapor App is distributed differently.

The Steam depot directly ships:

* Vapor Launcher.
* Vapor Installer.
* Required bootstrap/runtime state.
* Default Loo Cast built composition.

This makes initial Player experience independent from Workshop availability.

Future Steam depot updates may update the default composition.

How that default first-party composition also participates in normal Vapor identity/version semantics remains open.

---

# Player Acquisition

For an additional published Vapor App, the Player-facing conceptual flow is:

> Discover Vapor App
> → resolve suitable built publication
> → acquire through Steam Workshop
> → verify/register locally
> → install
> → optionally select
> → launch

The Player does not:

* Clone source.
* Resolve Git dependencies.
* Compile.
* Install Rust.
* Install Cargo.

The Registry/Launcher should hide provider-native distribution details where possible.

---

# Composer Source Acquisition

Composer acquisition is fundamentally different.

The Composer wants source, not merely the Player-facing built App.

Conceptually:

> Discover Vapor Content
> → resolve source identity
> → locate Vapor-compatible Git source
> → clone/fetch required source
> → make it available within the local development/source model

A Composer may still separately install a built published Vapor App.

The presence of a built Player artifact does not replace source acquisition when composition authoring is intended.

---

# Publication as a Multi-System Operation

Publishing a Packagepack potentially crosses:

* Local Git source.
* Remote Git hosting.
* Vapor Registry.
* Build tooling/infrastructure.
* Steam Workshop.

This means publication should not be conceptualized as one opaque "upload file" operation.

At the same time, the user should not be forced to manually orchestrate every backing system.

Vapor should present one coherent publication operation while internally tracking the meaningful stages.

---

# Coarse Publication Lifecycle

A Packagepack publication broadly involves:

> Validate authored source
> → ensure source publication
> → resolve intended Packagepack version
> → build required Vapor App target(s)
> → validate built artifact(s)
> → publish built distribution
> → link/update Registry metadata
> → publication becomes discoverable

The exact ordering may change once atomicity and failure recovery are designed.

The current flow is intentionally conceptual.

See [Vapor Publication Model](./Diagrams/Vapor%20Publication%20Model.puml).

---

# Partial Publication and Failure

Because publication spans multiple systems, partial success is possible.

Examples:

* Git push succeeds, build fails.
* Build succeeds, Workshop upload fails.
* Workshop upload succeeds, Registry update fails.
* Registry reachable, Git host unavailable.
* Authentication expires midway.

The operational model should therefore represent publication through independent conditions rather than one boolean:

> `published = true/false`

Vapor needs eventual recovery/reconciliation behavior.

A failed later stage must not automatically undo valid source history.

Similarly, a published Workshop artifact without correct Registry linkage may need repair rather than destructive recreation.

---

# Publication Validation

Before public availability, Vapor should validate at least the semantic properties it can reliably guarantee.

For source artifacts this may include:

* Valid Vapor artifact metadata.
* Valid identity.
* Required dependency declarations.
* Artifact-type correctness.
* Required repository/source availability.

For Packagepack built publication this additionally includes:

* Complete valid composition.
* Exactly one effective Engine.
* Exactly one effective Game.
* Successful target build.
* Valid distribution package.
* Required ownership/authorization.

Detailed validation belongs in future technical design.

---

# Ownership and Authorization

Publication requires authority separate from local capability.

A Composer may possess everything needed to create and build a Packagepack locally while lacking authority to publish under a particular remote identity.

Relevant authority may include:

* Git repository push permission.
* Vapor namespace ownership.
* Steam Workshop publication authority.
* Official first-party authorization.
* Future signing authority.

The Registry may participate in linking:

* Steam account.
* Git-host account.
* Vapor identity.
* Publication ownership.

The exact identity model remains open.

---

# Updating Publications

Updating published content requires separating source updates from built-output updates.

A source update to an Engine does not automatically imply that every Packagepack consuming that Engine instantly changes.

Published compositions need source/version-resolution semantics.

Likewise, a new Packagepack source version may require new built artifacts before Players can consume it.

The update model therefore eventually needs:

* Source version identity.
* Dependency version semantics.
* Build version/release identity.
* Player update behavior.
* Compatibility policy.

This is one of the largest remaining publishing design areas.

---

# Immutability and Reproducibility

The ecosystem should strongly prefer published compositions being reconstructible.

A historical Packagepack release should ideally identify enough source state to reproduce its intended composition.

That likely requires stable references to:

* Packagepack source version.
* Dependency source versions.
* Build target/toolchain assumptions.
* Relevant Vapor build-system version.

The exact reproducibility guarantee is not yet defined.

---

# Deprecation and Removal

Future publication design must distinguish:

* Stop recommending.
* Mark deprecated.
* Hide from new discovery.
* Remove built distribution.
* Remove source access.
* Delete identity entirely.

These actions have very different consequences.

A previously installed Vapor App may remain locally runnable even if its publication is later deprecated.

The Registry should not casually erase historical identity needed to understand existing local state.

---

# Public vs Private Source

The current model primarily assumes publicly usable Vapor content.

However, source publication architecture should avoid unnecessarily preventing:

* Private development repositories.
* Unpublished local content.
* Restricted official repositories.
* Pre-release/internal development.

Player-facing public Workshop publication is a later distribution concern.

Local source development does not require public publication.

---

# Publication Invariants

* Canonical Vapor source lives in Git-backed repositories.
* Steam Workshop is not the canonical source store.
* Steam Workshop distributes built complete compositions.
* The default Loo Cast built composition ships through the Steam depot.
* A Packagepack is the complete authored composition artifact.
* Only a Packagepack can produce a complete Vapor App.
* Enginepacks, Gamepacks, and Modpacks are not independently runnable Player distributions.
* Source publication and built publication are distinct concerns.
* Vapor identity is distinct from Git and Steam provider identities.
* The Registry links semantic Vapor identity to external systems.
* Local build capability does not imply remote publication authority.
* Player acquisition does not require source or compiler tooling.
* A source update does not automatically mutate already installed Player builds.
* Publication failure must be recoverable across partially successful external stages.
* Vapor must preserve historical identity sufficiently to reason about existing published/installed artifacts.

---

# Major Open Publishing Questions

* Exact Vapor version model.
* Exact source dependency/version syntax.
* How Packagepack source pins or constrains dependency versions.
* One Workshop Item vs multiple Items per Packagepack/release/target.
* Build ownership and trust model.
* Remote build infrastructure.
* Artifact signing.
* Publication atomicity.
* Recovery from partial publication.
* Source-release immutability.
* Registry schema.
* Ownership transfer.
* Multi-author collaboration.
* Steam/Git identity linkage.
* Provider independence beyond GitHub.
* Deprecation.
* Removal.
* Rollback.
* Player update policy.
* Compatibility policy.
* Default depot composition version semantics.
* Historical reproducibility guarantees.
* Whether published builds embed a complete machine-readable resolved composition manifest.