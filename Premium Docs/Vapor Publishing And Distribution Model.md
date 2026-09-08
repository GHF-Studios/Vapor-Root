> [!info]
> This document defines how Vapor Content source, immutable published versions, and built complete Vapor Apps become externally available.
>
> The central distinction is:
>
> **Git-backed source publication preserves and distributes authored Content source.**
>
> **Steam-backed built distribution delivers complete Vapor Apps to Players.**
>
> The Vapor Registry provides the canonical semantic identity, version, linkage, resolution, publication-state, Yank/Ban, and provider-association layer connecting those systems.
>
> Canonical source identity is defined by the **Vapor Context, Identity, Session And Selection Model**.
>
> Content dependency/composition semantics are defined by the **Vapor Dependency, Target, Constituency And Composition Model**.
>
> Source-authoring workflows are defined by the **Vapor Development Experience Model**.

---

# Core Publication Principle

Vapor has two fundamentally different publication concerns:

```text
Source Publication

Built Vapor App Distribution
```

They are connected.

They are not the same thing.

A developer may publish source without creating a Player-runnable artifact.

A Player-runnable artifact may only originate from a complete Packagepack composition.

Therefore:

```text
Content Project
    ↓ publish source/version
Published Content Version

Packagepack Version
    ↓ resolve exact composition
Vapor App Composition
    ↓ build
Vapor App artifact(s)
    ↓ distribute
Player-facing publication
```

---

# Canonical Content Identity

A Vapor Content identity is the canonical identity of its Content Project.

The canonical hierarchy is:

```text
Authority
└── Source Repo Container
    └── Source Repo
        └── Project
```

Therefore a Content ID may be:

```text
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

where:

```text
GHF-Studios
    Authority

Loo-Cast
    Source Repo Container

Game
    Source Repo

Loo-Cast
    Project / Content identity
```

Content is not an additional identity layer beneath Project.

---

# Content Project Is the Publication Unit

For ordinary Vapor Content:

> **One Content Project is one independently identified/versioned/published Vapor Content unit.**

A Project's Content kind may be:

```text
Packagepack
Enginepack
Gamepack
Modpack

Engine
Game

Engine Mod
Game Mod
Extension Mod

Library
```

The kind is a property of the Project.

It is not part of the canonical path syntax.

Therefore:

```text
GHF-Studios/Foo/Game/My-Game
```

already identifies the published Content.

Vapor does not need a second independently identified Content artifact nested inside that Project.

---

# Vapor ID

The canonical human-readable Content identity is the **Vapor ID**.

For Content Projects, the Vapor ID is the canonical Project identity.

A Vapor ID is not synonymous with:

```text
Git repository URL
GitHub repository ID
Git commit SHA
Git tag
GitHub Release ID
Steam Workshop Item ID
Steam account ID
filesystem path
```

Those are provider/version/realization identities.

The Vapor ID is the stable semantic anchor.

---

# Published Vapor ID Immutability

Once Content is published under a Vapor ID, that identity is immutable for the ordinary publication lifecycle.

Therefore ordinary publication must not support:

```text
rename published Vapor ID in place
```

as though identity were cosmetic metadata.

If an author wants materially different identity:

```text
new Vapor ID
```

is the normal model.

Historical old identity remains historical truth.

---

# Topology Migration and Published IDs

Because Source Repo Container, Source Repo, and Project names participate in canonical identity, moving or renaming a published Content Project would ordinarily change its Vapor ID.

Such changes must never happen accidentally through:

```text
moving a folder
renaming a repository
changing a submodule path
```

If first-party/topology migration ever requires changing already-published canonical identities, that is an explicit exceptional Registry migration requiring preservation of:

```text
historical identity
published versions
old references
migration relationship
provider linkage
```

It is not an ordinary Content rename feature.

---

# Published Version Identity

Published Content is identified by:

```text
Vapor ID
+
Semantic Version
```

Conceptually:

```text
GHF-Studios/Loo-Cast/Game/Loo-Cast @ 1.4.2
```

The exact machine-readable syntax for displaying this combination may vary by surface.

The semantic pair is stable.

---

# Semantic Versioning

All published Vapor Content versions use Semantic Versioning.

Conceptually:

```text
MAJOR.MINOR.PATCH
```

with normal pre-release/build metadata where appropriate.

Examples:

```text
1.0.0
1.4.2
2.0.0
2.1.0-alpha.3
```

The exact compatibility interpretation of major/minor/patch may additionally depend on Vapor Content kind and explicitly modeled API/compatibility contracts.

But the published version scheme itself is SemVer.

---

# Local Development Version

Unpublished local development does not need a globally immutable published version for every edit.

A development Project may use:

```text
a placeholder development version

or

an appropriate pre-release version following the latest published release
```

Examples:

```text
0.1.0
1.4.3-dev.1
2.0.0-alpha.1
```

according to workflow.

The important distinction is:

```text
local development version
    mutable development intent

published version
    immutable historical release identity
```

---

# Publication Locks the Version

Publishing:

```text
Content ID @ Version
```

creates immutable publication history.

Once:

```text
GHF-Studios/Foo/Game/My-Game @ 1.2.0
```

has been published, that version must not later be redefined to mean different source.

A correction requires:

```text
1.2.1
```

or another new valid version.

Never mutate historical version meaning in place.

---

# Git Commit Binding

Every published Content version is bound to an exact Git commit.

Conceptually:

```text
Vapor ID
+
SemVer
+
Source Repo identity
+
Git commit SHA
```

The Registry must be able to answer:

> Which exact authored source produced version `X` of this Content?

This binding is immutable after publication.

---

# Source Repo Relationship

Because a Content Project lives inside one Source Repo, publishing knows both:

```text
Content Project identity

and

owning Source Repo identity
```

For example:

```text
Content:
    GHF-Studios/Loo-Cast/Game/Loo-Cast

Source Repo:
    GHF-Studios/Loo-Cast/Game
```

The Registry then knows the provider repository backing that Source Repo.

Conceptually:

```text
Content ID
    ↓ parent topology
Source Repo
    ↓ Registry provider linkage
Git provider repository
    ↓ commit
Published source state
```

There is no need for every Project to own a separate Git repository.

---

# Source Publication

Source publication is Git-backed.

A valid source publication requires at least:

```text
registered Content identity
valid Project kind
valid authored metadata
published SemVer
exact Git commit
source provider linkage
required publication authority
valid dependency declarations
successful semantic validation
```

The source commit must be remotely recoverable through the registered provider before publication is considered complete.

---

# Git Remains the Source System

The Registry does not replace Git.

Git remains authoritative for:

```text
source files
commit history
branches
tags
repository state
```

The Registry is authoritative for Vapor semantics such as:

```text
canonical Vapor identity
published versions
version → commit binding
provider linkage
Content kind
dependency/version metadata
Yank/Ban state
publication state
first-party/authority policy
built-distribution linkage
```

Conceptually:

```text
Vapor ID @ Version
    ↓ Registry
Source Repo + commit
    ↓ provider
Git
```

---

# Source Release Representation

A published Vapor version should also receive an appropriate provider-side source release representation.

For GitHub-backed source, the intended model includes:

```text
Git commit
Git tag/release association
GitHub Release
source archive / appropriate release artifacts
```

The exact tag spelling may remain implementation-defined.

The semantic requirement is that:

```text
published Vapor version
↔ exact provider release/source commit
```

is unambiguous.

---

# One Provider Release per Published Version

Published versions should not be retroactively bundled into one ever-growing GitHub Release.

Conceptually:

```text
1.0.0
    → its release

1.1.0
    → its release

2.0.0
    → its release
```

Historical releases remain separately addressable.

Older releases are not replaced merely because a newer release exists.

---

# Registry Version Record

For each published Content version, the Registry should be able to represent at least:

```text
Vapor ID
Content kind
SemVer
Source Repo
Git provider linkage
Git commit SHA
provider release/tag linkage
published dependency requirements
publication timestamp/state
Yank state
Ban state
ownership/authority metadata
```

For Packagepack versions, additional built-distribution state is required.

---

# Dependency Requirements

Published Content may depend on version ranges rather than only exact versions.

Conceptually:

```text
requires Foo ^1.4
requires Bar >=2.0,<3.0
```

The exact authoring syntax remains subject to manifest design.

The semantic requirement is:

> Published dependency constraints are immutable parts of the published version.

Changing a dependency requirement requires publishing a new Content version.

---

# Deterministic Resolution

Given:

```text
same registry state/policy
same root requirements
same available version set
same resolution rules
```

Vapor dependency resolution should be deterministic.

A published Packagepack build must therefore record the exact resolved versions which were selected from authored constraints.

---

# Constraint vs Resolution

These are different facts.

Authored dependency:

```text
Library-A = "^2.1"
```

Resolved dependency:

```text
Library-A = 2.4.3
```

A published Content version preserves its authored constraint.

A published Packagepack realization additionally preserves the exact resolved graph used to build that release.

---

# Version Multiplicity

Vapor may allow different major versions of the same Content to exist across different compositions.

For example:

```text
Composition A
    Foo 1.x

Composition B
    Foo 2.x
```

is legitimate.

Within one resolved composition, the default model should avoid resolving several incompatible major versions of the same Vapor dependency identity unless an explicit modeled disambiguation mechanism permits it.

Cargo-style dependency renaming/disambiguation may eventually support legitimate advanced cases.

The resolver must never silently create ambiguous semantic bindings.

---

# Resolution Failure

If dependency constraints cannot produce one legal deterministic composition, publication/build must fail.

Failure should explain:

```text
which requirements conflict
which Content introduced them
which candidate versions were considered
which rule prevented resolution
```

Vapor may support explicitly modeled fallback behavior where designed.

It should never resolve conflicts by arbitrary nondeterministic choice.

---

# Packagepack

A **Packagepack** is already the complete authored composition Project.

Publication must not invent another authored entity called:

```text
finished composition
published composition
distribution composition
```

after Packagepack.

Instead:

```text
Packagepack source
    ↓ resolve
Vapor App Composition
    ↓ build
Vapor App
    ↓ publish/distribute
Player availability
```

---

# Packagepack Version

A published Packagepack version contains:

```text
immutable Packagepack source version
+
immutable dependency constraints
+
exact source commit
```

When built for Player distribution, it additionally produces an exact resolved composition.

---

# Resolved Composition Record

A built published Packagepack release must record the exact Content graph used to produce it.

At minimum:

```text
Packagepack ID/version
resolved Engine ID/version
resolved Game ID/version
resolved Mods
resolved Libraries
resolved subordinate packs
all transitive resolved Content versions
```

This becomes part of the publication provenance.

---

# Machine-Readable Composition Manifest

Published Vapor Apps should carry or be linked to a machine-readable resolved composition manifest.

The manifest should be sufficient for Vapor to answer:

```text
What exact Content versions are in this App?

Which Packagepack produced it?

Which Engine/Game/Mods/Libraries were selected?

Which source releases correspond to those versions?
```

The exact serialization/layout remains implementation-defined.

The semantic requirement is not optional.

---

# Complete Composition Requirement

Only a valid complete Packagepack can produce a Player-facing Vapor App.

The resolved composition must contain exactly:

```text
one effective Engine

one effective Game
```

plus the selected compatible/supporting Content required by that composition.

Therefore:

```text
Enginepack
Gamepack
Modpack
Engine
Game
Mod
Library
```

are not independently Player-runnable published Vapor Apps merely because they can be source-published.

---

# Source Publication vs Built Publication

All publishable Content kinds may have source releases.

Only Packagepacks additionally produce complete built Vapor App distribution.

Therefore:

```text
Library publish
    source/version publication

Game publish
    source/version publication

Engine publish
    source/version publication

Packagepack publish
    source/version publication
    +
    complete composition resolution
    +
    built Vapor App publication
```

depending on requested publication stage and available target builds.

---

# Packagepack Publication State

Because Packagepack publication crosses several systems, its state is not one Boolean.

Useful semantic states may include:

```text
source version published
composition resolved
build pending
build succeeded
built artifacts validated
Workshop publication pending
Workshop publication available
Registry linkage complete
partially published
Yanked
Banned
```

Exact state-machine encoding belongs to implementation design.

The conceptual independence is required.

---

# Source Publication by Role

## Composer

A Composer may author and publish composition-oriented Content:

```text
Packagepack
Enginepack
Gamepack
Modpack
```

where authorization permits.

## Content Developer

A Content Developer may additionally publish:

```text
Engine
Game
Engine Mod
Game Mod
Extension Mod
Library
```

where authorization permits.

## Ecosystem Developer

An Ecosystem Developer may develop/publish first-party Vapor source according to first-party workflows.

That is not automatically ordinary public Content publication.

Protected official operations additionally require appropriate authority.

---

# Role Is Not Publication Authority

Installed Role answers:

> What is this local Vapor environment equipped to author?

Publication Authority answers:

> Is this identity allowed to publish this particular remote identity/version?

Therefore:

```text
Content Developer
```

does not automatically imply:

```text
may publish GHF-Studios/Loo-Cast/Game/Loo-Cast
```

Publication requires target-specific authorization.

---

# Publication Authority

Relevant authority may include:

```text
Vapor namespace/identity ownership
Git provider repository permission
Registry publication permission
Steam Workshop publication authority
first-party Root Authority
future artifact-signing authority
```

Vapor should identify the missing authority specifically.

Do not report generic:

```text
permission denied
```

when the actual missing grant is known.

---

# First Publication

The first publication of a new Content Project establishes:

```text
canonical Vapor Content ID
initial published SemVer
exact source commit
provider linkage
published metadata
ownership/publication authority relationship
```

For a Packagepack, it may additionally establish initial Player-facing distribution linkage.

After first publication, the Vapor ID is historical identity and must not be casually renamed.

---

# Publishing a New Version

Publishing a later version conceptually performs:

```text
validate local authored Project
→ choose/validate new SemVer
→ verify version does not already exist
→ verify dependencies
→ verify authority
→ ensure exact source commit exists remotely
→ bind Version to commit
→ create provider release representation
→ register immutable version metadata
→ for Packagepack: resolve/build/distribute as requested
```

Exact transaction ordering must account for external-system failures.

---

# Version Reuse Is Forbidden

If:

```text
1.4.2
```

has already been published for a Vapor ID, attempting to publish different source as `1.4.2` must fail.

Useful diagnostic:

```text
error: version `1.4.2` is already published for
       `GHF-Studios/Foo/Game/My-Game`

published commit:
    abcdef...

help:
    publish a new semantic version instead
```

---

# Source Commit Must Be Stable

The version binding must refer to an immutable Git commit SHA.

Branches such as:

```text
main
dev
release
```

are not sufficient publication identity because they move.

Tags/releases may provide human-facing provider navigation.

The immutable commit is the source provenance anchor.

---

# GitHub Release Artifacts

For GitHub-backed publication, a version's GitHub Release may contain appropriate artifacts such as:

```text
source archives
metadata
generated schemas/reference data
build outputs where useful
diagnostic/provenance files
```

depending on Content kind and publication pipeline.

A Packagepack may additionally have Player-facing Steam distribution.

GitHub Release artifacts do not replace Steam Workshop as the ordinary Player App-distribution backend.

---

# Built Vapor App Artifacts

A Packagepack release may produce multiple physical artifacts.

For example:

```text
Linux x86_64
Windows x86_64
future architectures
future server/client target distinctions
```

All may represent:

```text
same Packagepack ID
same Packagepack version
same logical resolved composition
```

They are target-specific physical realizations, not separate Content identities.

---

# Logical Build vs Physical Artifact

Vapor distinguishes:

```text
logical build identity
```

from:

```text
physical artifact bytes
```

The goal is:

> **Same logical inputs should produce semantically equivalent artifacts.**

Byte-for-byte reproducibility is desirable and should be pursued through controlled tooling.

It is not required to redefine logically equivalent builds as different Content versions merely because physical artifact bytes differ in irrelevant ways.

---

# Reproducibility Goal

Published builds should be reconstructible as strongly as practical.

Relevant inputs include:

```text
exact Packagepack version
exact resolved dependency graph
exact source commits
target
Vapor build-system version
managed Rust/Cargo toolchain version
relevant build configuration
generated realiza
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
* Whether published builds embed a complete machine-readable resolved composition manifest.tion inputs
```

First-party/root development should strongly prefer the locked/vendored Vapor-managed toolchain.

---

# Deterministic Semantics Before Bitwise Identity

Vapor prioritizes:

```text
deterministic resolution
deterministic semantic composition
controlled toolchain
known source commits
known build target/configuration
```

before claiming universal bit-for-bit artifact reproducibility.

Bitwise reproducibility is a strong target.

Semantic/build provenance correctness is the minimum requirement.

---

# Build Ownership

Player-facing builds require a trust model.

Possible implementation mechanisms include:

```text
local author build
Vapor-managed remote build
trusted build infrastructure
hybrid workflows
```

The exact infrastructure may evolve.

What is required is that Vapor can establish:

```text
which source/version was built
which resolved composition was used
which target was built
which toolchain/build environment was used
who/what produced the artifact
whether publication authority permits it
```

---

# Artifact Provenance

A published Vapor App artifact should be linkable to:

```text
Packagepack Vapor ID
Packagepack version
Packagepack Git commit
exact resolved Content graph
build target
build/toolchain metadata
publication metadata
distribution provider identity
```

This allows both diagnosis and historical reconstruction.

---

# Steam Workshop

Steam Workshop is the intended distribution backend for additional published Player-facing Vapor Apps.

It is not the canonical source store for:

```text
Engine
Game
Mod
Library
pack source
```

Workshop receives complete built Packagepack realizations.

---

# Steam Workshop Item ID

A Steam Workshop Item ID is a provider-native immutable identifier associated with its Workshop publication.

It is secondary to Vapor semantic identity.

Players should primarily see:

```text
Vapor App / Packagepack identity
version
metadata
```

rather than needing to manage:

```text
numeric Workshop Item ID
```

The Registry owns the linkage.

---

# Workshop Publication Cardinality

The precise long-term mapping between:

```text
Packagepack identity
Packagepack version
target builds
Workshop Item
Workshop revisions
```

remains implementation-specific enough to leave open.

Whatever representation is chosen must preserve:

```text
immutable Vapor version identity
historical version accessibility
deterministic Registry linkage
supported-target discovery
```

A new Packagepack release must not make historical publication provenance unintelligible.

---

# Historical Workshop Revisions

Older published Packagepack versions should remain identifiable and retrievable where the distribution backend permits it.

Vapor must preserve historical version metadata even if Steam's user-facing UI emphasizes only the newest revision.

Historical Vapor identity/version must not depend on the current Workshop listing alone.

---

# Steam Depot

The default first-party Loo Cast Vapor App is distributed through the Steam depot.

The depot ships the default Player-ready experience together with the Vapor Client/bootstrap.

This ensures first launch does not require:

```text
Workshop availability
Git
Rust
Cargo
local build
```

---

# Default Loo Cast Packagepack

The default depot-delivered Loo Cast App still corresponds to an ordinary first-party Packagepack identity/version.

Conceptually:

```text
first-party Loo Cast Packagepack version
    ↓ resolve/build
default Loo Cast Vapor App
    ↓ Steam depot
Player
```

Depot delivery is a special distribution channel.

It does not exempt the composition from normal Vapor identity/version/provenance semantics.

---

# Depot vs Workshop

Conceptually:

```text
default first-party Vapor App
    → Steam depot

additional published Vapor Apps
    → Steam Workshop
```

Both remain Vapor Apps with canonical Packagepack identity/version.

The distribution provider is not the semantic identity.

---

# Player Acquisition

For an additional Vapor App:

```text
discover Vapor App
→ choose/resolve desired version
→ resolve supported target artifact
→ acquire through Steam-backed distribution
→ verify/register locally
→ install
→ select if desired
→ launch
```

The Player does not:

```text
clone Git
resolve source repositories
install Rust
run Cargo
compile dependencies
```

---

# Player Version Selection

The default Player experience may select the newest appropriate non-Yanked/non-Banned compatible release according to policy.

Advanced/version-management UX may allow selecting older available versions where supported.

Exact Player UI policy remains separate from publication identity.

The Registry must retain enough history to make such decisions possible.

---

# Composer / Developer Source Acquisition

Source consumers need Git-backed source rather than only built Player artifacts.

Conceptually:

```text
resolve Content identity/version
→ Registry resolves Source Repo/provider/commit
→ acquire appropriate Source Repo Container or independently acquirable Source Repo
→ obtain source
→ validate Project/version
```

This uses the canonical source-acquisition model.

Publication does not introduce an unrelated source-download hierarchy.

---

# Acquisition Unit vs Content Unit

A useful distinction:

```text
publication/version unit:
    Content Project

source acquisition unit:
    normally Source Repo Container

source repository unit:
    Source Repo
```

Therefore acquiring one published Library may physically acquire a Container containing other Source Repos/Projects.

That does not mean those Projects share one publication identity.

---

# Packagepack Build Resolution

To publish a built Packagepack version:

```text
load immutable Packagepack source version
→ resolve dependency constraints
→ choose exact versions deterministically
→ validate complete composition
→ acquire exact source
→ reconcile physical Cargo/build graph
→ build supported target(s)
→ validate artifacts
→ record provenance
→ distribute
```

---

# Published Resolved Graph Is Immutable

Once a particular Packagepack release/build publication records:

```text
exact resolved version graph
```

that graph is historical provenance.

Later newly published dependency versions must not silently mutate what an already-published App release means.

For example:

```text
Packagepack 1.0.0
originally resolved Foo 1.4.2
```

must not later be reinterpreted as:

```text
Foo 1.7.0
```

merely because `^1.4` would resolve differently today.

The published resolution record freezes historical meaning.

---

# Rebuilding an Old Packagepack Version

When rebuilding historical:

```text
Packagepack 1.0.0
```

Vapor should be able to use its historical exact resolved graph rather than performing a fresh unconstrained modern resolution.

This is essential for reproducibility and diagnosis.

---

# Publication Is Multi-System

A Packagepack publication may span:

```text
local authored source
Git provider
Vapor Registry
build infrastructure
GitHub Release/artifact storage
Steam Workshop
```

Vapor should present a coherent publication workflow.

Internally it must preserve the meaningful boundaries.

---

# Publication Transaction Semantics

Publication cannot assume distributed external systems form one atomic transaction.

Therefore Vapor needs explicit staged state.

Conceptually:

```text
validate
→ reserve/check version
→ source commit available
→ source release created
→ Registry version committed
→ resolve composition
→ build target artifacts
→ validate artifacts
→ distribute built artifacts
→ record final linkage
→ mark availability
```

Exact ordering may evolve to minimize irrecoverable partial states.

---

# Partial Publication

Examples include:

```text
Git push succeeds
build fails

GitHub Release succeeds
Registry update fails

build succeeds
Workshop upload fails

Workshop upload succeeds
final Registry linkage fails
```

These are recoverable reconciliation states.

They are not reasons to rewrite historical Git commits or pretend the version never existed without explicit publication-state rules.

---

# Publication Repair

Publication repair may:

```text
retry missing external upload
reconcile Registry linkage
validate an existing provider release
re-register an existing successful build
repair derived publication metadata
```

It must not silently:

```text
rewrite immutable version source
move a version to another commit
delete valid Git history
publish different source under same SemVer
```

---

# Validation — All Content

Before source publication, Vapor should validate at least:

```text
canonical Content identity
Content kind
new valid SemVer
authored manifest validity
dependency requirements
source availability
Git commit availability
provider linkage
publication authority
Registry policy
```

Kind-specific validation may add further requirements.

---

# Validation — Packagepack

Packagepack built publication additionally requires:

```text
valid complete dependency resolution
exactly one effective Engine
exactly one effective Game
compatible selected Mods
valid support dependencies
exact resolved composition manifest
successful supported-target build
artifact validation
distribution authorization
```

---

# Publication Diagnostics

Publication failures should explain the exact failed invariant.

For example:

```text
error: version `2.0.0` cannot be published

Content:
    GHF-Studios/Foo/Game/My-Game

reason:
    source commit `abc123...` is not available from the registered
    provider repository

help:
    push the commit to the registered Source Repo provider before retrying
```

Or:

```text
error: Packagepack `1.4.0` cannot be built

dependency conflict:
    Foo requires Bar ^1.0
    Baz requires Bar ^2.0

no legal single Bar major version satisfies this composition
```

Publication UX should teach the underlying model rather than return opaque provider errors where Vapor can interpret them.

---

# Yank

A published Content version may be **Yanked**.

Yanking means approximately:

```text
preserve identity/version/history
preserve source release
preserve existing references

but

exclude from ordinary new dependency resolution/discovery
```

Exact resolver treatment may include explicitly requested historical versions where policy allows.

Yank is not deletion.

---

# Ban

A **Ban** is a stronger Registry restriction.

A Banned version/content may be excluded from:

```text
new resolution
new publication linkage
ordinary acquisition/discovery
execution/distribution where policy requires
```

according to policy.

Historical identity remains preserved.

Ban is not “pretend this release never existed.”

---

# Deprecation

Deprecation is advisory metadata.

It may communicate:

```text
obsolete
superseded
no longer recommended
migration target available
```

without necessarily removing a version from resolution.

Deprecation, Yank, Ban, and deletion are different concepts.

---

# Deletion

Published identity/version records should not normally be physically erased merely because they are no longer recommended.

Historical state may be needed to understand:

```text
old dependency graphs
installed Vapor Apps
old save/runtime environments
published Packagepacks
security incidents
migration history
```

Provider artifact removal may sometimes be necessary for legal/security reasons.

Registry historical identity should still preserve enough provenance to explain what existed where policy permits.

---

# Source Repository Deletion

Deleting or losing a backing provider repository does not erase the semantic fact that a published Vapor version existed.

It creates a source-availability/provenance failure.

The Registry should distinguish:

```text
published historical identity exists

but

source provider currently unavailable
```

rather than silently removing the release.

---

# Private Source

Vapor source development does not require public Git repositories.

New developer repositories should be conservative/private by default where provider behavior permits.

A Project may be:

```text
local-only

private provider-backed

internal first-party

pre-release

publicly published
```

Publication state is independent from merely having source.

---

# Private Publication

Future Vapor may support privately distributed Content or Vapor Apps.

The current public publication model should not unnecessarily prevent that architecture.

However:

```text
private source
```

does not automatically imply:

```text
private Vapor Registry publication
```

The exact private-distribution/entitlement model remains open.

---

# Public Publication

Public publication makes a Content version available according to Registry/distribution policy.

For normal public Content this requires appropriate:

```text
source availability
Registry visibility
authority
version metadata
provider linkage
```

Packagepack Player availability additionally requires built distribution.

---

# Git Provider Independence

GitHub is the primary provider today.

The semantic architecture is:

```text
Vapor source identity
→ Registry provider linkage
→ Git provider
```

not:

```text
Vapor identity = GitHub repository URL
```

Future Git providers may therefore be supported without redefining Content identity.

---

# Steam Provider Independence

Likewise:

```text
Vapor Packagepack identity/version
→ Registry distribution linkage
→ Steam Workshop/depot
```

Steam provider-native IDs remain secondary linkage.

If future built-distribution providers are introduced, Vapor semantic identity need not change merely because distribution backend changes.

---

# Publication Ownership Transfer

Canonical Content identity and publication authority are different.

A future ownership-transfer operation may change:

```text
who is authorized to publish future versions
```

without rewriting historical:

```text
Content ID
published versions
source commits
publication history
```

Exact ownership-transfer policy remains open and should be highly explicit.

---

# Multi-Author Collaboration

Multiple developers may collaborate on one Source Repo/Project through ordinary Git/provider collaboration.

That does not imply every collaborator owns publication authority.

Vapor publication permissions may be narrower than Git write permission.

The exact grants model belongs to Registry/Identity/Authorization design.

---

# First-Party Publication

First-party Vapor source/publication uses the same core identity/version/provenance principles where applicable.

Additional protected operations may require Root Authority or first-party-specific grants.

Examples include:

```text
publishing official Loo Cast Content
deploying Vapor Client Steam artifacts
publishing first-party Workshop content
deploying Platform Server
altering Registry first-party trust
```

These are not unlocked merely by local Ecosystem Developer Role.

---

# Source Publication Is Not Deployment

Publishing source:

```text
creates immutable released source/version state
```

Deploying first-party infrastructure:

```text
changes a running/installed operational environment
```

These are distinct operations.

For example:

```text
publish Vapor Registry Server source version
```

is conceptually different from:

```text
deploy Platform Server VPS
```

even if deployment consumes published source/artifacts.

---

# Packagepack Publication Is Not App Installation

Likewise:

```text
publish Packagepack/Vapor App
```

means make a release available through distribution.

```text
install Vapor App
```

means realize/register that release locally.

One is ecosystem publication.

One is local state.

---

# Updating Content

A source edit after version `1.2.0` does not modify `1.2.0`.

Instead:

```text
edit
→ test
→ choose new version
→ publish new immutable version
```

Dependents continue to mean whatever their existing constraints/resolved historical graphs specify.

---

# Updating Packagepacks

Publishing Packagepack `1.1.0` does not mutate installed Packagepack `1.0.0`.

Both versions remain distinct historical releases.

Player update policy may choose whether/when an installation moves to `1.1.0`.

That policy is separate from publication identity.

---

# Dependency Updates

Publishing:

```text
Library Foo 1.5.0
```

does not retroactively alter an already-published Packagepack release whose historical resolved graph used:

```text
Foo 1.4.2
```

A future Packagepack version may freshly resolve the compatible newer version.

Historical Packagepack meaning remains frozen.

---

# Compatibility Metadata

Published Content may carry explicit compatibility information beyond SemVer.

Possible examples:

```text
supported Engine/API compatibility
minimum Vapor runtime/toolchain requirements
supported operating targets
feature/capability requirements
```

Exact schemas remain kind-specific and implementation-driven.

They must not undermine immutable historical version meaning.

---

# Published Build Target Matrix

A Packagepack release may support:

```text
one target
several targets
different targets published at different times
```

The Registry should represent target availability independently.

For example:

```text
Packagepack 1.0.0

Linux x86_64
    available

Windows x86_64
    available

future target
    unavailable
```

The Packagepack version remains one version.

---

# Adding a Target Later

If the exact same immutable Packagepack version/resolved composition later receives a build for an additional target, that may add a physical distribution artifact without redefining source identity/version.

The publication system must preserve that:

```text
same immutable source version
same logical composition
new physical target realization
```

relationship.

Whether policy permits post-publication target additions without creating a new Packagepack version should be explicit and auditable.

---

# Built Artifact Replacement

Replacing already-published physical artifact bytes for the same target/version is more dangerous than adding a previously absent target.

Vapor should strongly prefer artifact immutability.

If artifact replacement is ever permitted for non-semantic packaging reasons, provenance/history must preserve both old and replacement artifact identities.

Do not silently overwrite historical build provenance.

---

# Security Response

A compromised or unsafe release may require:

```text
Yank
Ban
distribution removal
security advisory
forced update policy
```

depending on severity.

Security response should preserve historical auditability while preventing unsafe new use.

Exact forced-update/entitlement semantics remain future design.

---

# Registry as Publication Authority

The Registry is the authoritative Vapor view over publication state.

It must know enough to answer questions such as:

```text
Does this Content ID exist?

Which versions exist?

Which versions are Yanked/Banned?

Which source commit corresponds to this version?

Which provider release contains it?

What dependencies were authored?

For Packagepack:
    what exact composition was resolved?
    which target artifacts exist?
    where are they distributed?
```

---

# Registry Is Not Artifact Storage

The Registry does not need to store every source/artifact byte.

It stores semantic truth and linkage.

Actual bytes may live in:

```text
Git provider
GitHub Releases
Steam Workshop
Steam depot
future artifact infrastructure
```

The Registry tells Vapor what those bytes mean.

---

# Publication Discovery

Discovery should operate primarily on:

```text
Vapor identity
Content kind
version
metadata
compatibility
publication state
```

rather than exposing provider implementation details as the primary user experience.

Provider details remain available for diagnostics/advanced users.

---

# Publication Invariants

* The canonical publication unit for ordinary Content is the Content Project.
* Content identity is the canonical Project identity.
* Content kind is a Project property, not an identity segment.
* Published Vapor IDs are immutable in the ordinary publication lifecycle.
* All published Vapor Content uses SemVer.
* A published version is immutable.
* Every published version is bound to one exact Git commit.
* Git remains the canonical source/version-control system.
* Registry version records link Vapor versions to Git/provider state.
* Published dependency requirements are immutable.
* Resolution is deterministic.
* Published Packagepack builds record an exact resolved Content graph.
* Historical Packagepack resolutions do not change when new dependency versions appear.
* A Packagepack is the complete authored composition artifact.
* Only Packagepacks produce complete Player-facing Vapor Apps.
* Source publication and built Vapor App distribution are distinct.
* Steam Workshop is not the canonical source store.
* Steam Workshop distributes additional built complete Vapor Apps.
* The default first-party Loo Cast Vapor App is distributed through the Steam depot.
* Depot-delivered Loo Cast still participates in normal Vapor ID/version/provenance semantics.
* Steam Workshop Item IDs are provider-native linkage, not canonical Vapor identity.
* Target-specific artifacts do not create new Content identities.
* Same logical build may have multiple physical artifacts.
* Deterministic semantics/provenance are mandatory; bit-for-bit reproducibility is a strong target.
* Published historical GitHub/provider releases remain separately addressable.
* Older versions are not overwritten by newer versions.
* Yank and Ban preserve historical identity.
* Publication authority is distinct from installed Vapor Role.
* Source acquisition unit and publication unit are different concepts.
* Publication may span multiple external systems and therefore requires recoverable staged state.
* Publication Repair must never rewrite immutable version→source meaning.
* Git/provider URLs and IDs are secondary to Vapor semantic identity.
* Registry is the semantic publication authority, not a replacement byte store.

---

# Open Publishing Questions

The following remain intentionally open:

* Exact manifest syntax for SemVer constraints.
* Exact Git tag naming convention.
* Exact GitHub Release artifact inventory by Content kind.
* Exact Workshop Item/revision/version/target cardinality.
* Exact build-signing model.
* Exact remote/trusted build infrastructure.
* Exact publication transaction/state-machine implementation.
* Exact handling of adding new physical targets to an already-published immutable Packagepack version.
* Exact artifact replacement prohibition/exception policy.
* Exact private Registry publication/distribution model.
* Exact ownership-transfer/grants model.
* Exact multi-author publication authorization UI.
* Exact artifact-signing/security-advisory mechanism.
* Exact Player automatic-update/version-selection policy.
* Exact forced-security-update policy.
* Exact compatibility-metadata schemas by Content kind.
* Exact long-term provider abstraction beyond GitHub/Steam.
* Exact retention guarantees when a provider removes historical bytes.
* Exact exceptional migration procedure for already-published IDs affected by first-party source-topology migration.
