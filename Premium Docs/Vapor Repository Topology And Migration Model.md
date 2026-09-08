> [!info]
> This document defines Vapor's intended first-party repository topology and the controlled migration from the current legacy/rewrite-era repository layout.
>
> Product/runtime terminology is defined by the **Vapor Client, Platform And App Runtime Model**.
>
> Canonical source identity, Superworkspace, Source Repo Container, Source Repo / Vapor Workspace, Project, Context, and Selection semantics are defined by the **Vapor Context, Identity, Session And Selection Model**.
>
> Source creation/acquisition and development behavior is defined by the **Vapor Development Experience Model**.
>
> Exact CLI projection is defined by the **Vapor CLI Model**.
>
> The **Vapor Repository Migration Execution Ledger** records actual execution state.
>
> This document defines the target architecture and migration semantics.
>
> The execution ledger records what has actually happened.

---

# Migration Thesis

The current Vapor repository layout reflects several generations of architecture.

Some repositories represent durable source boundaries.

Others exist because older Vapor designs assigned independent semantic ownership to applications such as:

```text
Launcher
SDK
Installer
Shell
Entrypoint
```

The current architecture instead follows:

> **Vapor Core owns shared Vapor semantics; applications and frontends project those semantics.**

Repository topology should increasingly reflect meaningful source/deployment ownership rather than executable count.

The Client and Platform sides intentionally have different migration shapes.

```text
Client
    consolidate obsolete application-oriented source boundaries

Platform Server
    preserve meaningful independently deployable service boundaries
```

The goal is not symmetry.

The goal is coherent ownership.

---

# Canonical Source Identity

The canonical source hierarchy is:

```text
Authority
└── Source Repo Container
    └── Source Repo
        └── Project
```

For first-party source, the Authority is currently:

```text
GHF-Studios
```

Examples of canonical identities after migration include:

```text
GHF-Studios/Vapor-Client
GHF-Studios/Vapor-Client/Vapor

GHF-Studios/Vapor-Platform-Server
GHF-Studios/Vapor-Platform-Server/Vapor-Registry-Server
```

A Source Repo Container name therefore participates directly in the canonical identities of every descendant Source Repo and Project.

This has an important migration consequence:

> **Renaming a Source Repo Container is a Vapor identity migration for the Container and all descendants.**

It is not merely a cosmetic structural-address change.

---

# Three Different Things Can Change

Repository migration must distinguish three independent domains:

```text
Provider Repository
Vapor Identity
Local Realization Path
```

For example, the Client migration may involve:

```text
GitHub provider repository:
    GHF-Studios/Vapor-Root
    →
    GHF-Studios/Vapor-Client

Vapor canonical identity:
    GHF-Studios/Vapor-Root
    →
    GHF-Studios/Vapor-Client

local canonical path:
    <old layout>/Vapor-Root
    →
    <Superworkspace>/GHF-Studios/Vapor-Client
```

These changes are related.

They are not the same operation.

They should not be forced into one irreversible step.

---

# Provider Rename Is Not Automatically Vapor Identity Migration

Git hosting is provider linkage.

Therefore Vapor may temporarily support:

```text
Vapor identity:
    GHF-Studios/Vapor-Root

provider repository:
    github.com/GHF-Studios/Vapor-Client
```

during migration.

This is legitimate because:

```text
Vapor identity
    ≠ provider repository name
```

The Registry may update provider linkage first while preserving the old canonical Vapor identity until the explicit identity-migration wave.

This gives migration a useful recovery boundary.

---

# Canonical Identity Migration

When the Vapor identity itself changes:

```text
GHF-Studios/Vapor-Root
→
GHF-Studios/Vapor-Client
```

every descendant receives the corresponding prefix migration.

For example:

```text
GHF-Studios/Vapor-Root/Vapor
→
GHF-Studios/Vapor-Client/Vapor
```

and a Project beneath that Source Repo:

```text
GHF-Studios/Vapor-Root/Vapor/Some-Project
→
GHF-Studios/Vapor-Client/Vapor/Some-Project
```

Likewise:

```text
GHF-Studios/Vapor-Server-Root
→
GHF-Studios/Vapor-Platform-Server
```

implies:

```text
GHF-Studios/Vapor-Server-Root/Vapor-Registry-Server
→
GHF-Studios/Vapor-Platform-Server/Vapor-Registry-Server
```

and corresponding Project descendant migrations.

These are real identity migrations.

They do not imply that Git history or repository provenance is discarded.

---

# Identity Migration Does Not Mean Source Recreation

Changing canonical Vapor identity does not require:

```text
delete Git repository
create unrelated Git repository
lose history
re-clone all source from scratch
pretend the old object never existed
```

The same continuing Git repository may receive a new canonical Vapor identity through an explicit migration.

Therefore:

```text
identity changed
```

does not imply:

```text
source provenance destroyed
```

Vapor should preserve explicit old→new identity mappings where needed for migration, diagnostics, historical references, and local-state reconciliation.

---

# Old Identity Mapping

Migration should maintain explicit mappings such as:

```text
GHF-Studios/Vapor-Root
→
GHF-Studios/Vapor-Client
```

and:

```text
GHF-Studios/Vapor-Server-Root
→
GHF-Studios/Vapor-Platform-Server
```

Descendant mappings may be derived by exact prefix replacement where Vapor can prove the hierarchy remained otherwise unchanged.

For example:

```text
old prefix:
    GHF-Studios/Vapor-Root/

new prefix:
    GHF-Studios/Vapor-Client/
```

allows exact migration of:

```text
GHF-Studios/Vapor-Root/Vapor
→
GHF-Studios/Vapor-Client/Vapor
```

This is deterministic hierarchy migration.

It is not fuzzy name matching.

---

# Historical Identity

Old canonical identities may need to remain recognizable after migration.

Recognition does not require treating old and new names as permanently equivalent canonical identities.

Possible migration state may conceptually contain:

```text
old canonical identity
    deprecated/migrated

migration target
    new canonical identity
```

Newly authored references should converge on the new identity.

Compatibility aliases/migration records must not become permanent architectural ambiguity.

---

# Canonical Superworkspace Layout

Vapor has one canonical local Superworkspace.

It has no identity.

The target local source layout mirrors the identity hierarchy sufficiently to avoid collisions:

```text
<Superworkspace>/
└── Authority/
    └── Source Repo Container/
        └── Source Repo/
```

For first-party source:

```text
<Superworkspace>/
└── GHF-Studios/
    ├── Vapor-Client/
    │   ├── Vapor/
    │   └── Vapor-Examples/
    │
    └── Vapor-Platform-Server/
        ├── Vapor-Homepage-Server/
        ├── Vapor-Docs-Server/
        ├── Vapor-Identity-Server/
        ├── Vapor-Diagnostics-Server/
        └── Vapor-Registry-Server/
```

The exact membership may change through the migration.

The Authority directory is part of local realization layout.

The Superworkspace itself is not part of global identity.

---

# Local Path and Identity

Moving:

```text
~/Development/Vapor
```

to:

```text
/mnt/Development/Vapor
```

does not change any Vapor identity.

That is Superworkspace relocation.

Moving a Container from:

```text
<Superworkspace>/GHF-Studios/Vapor-Root
```

to:

```text
<Superworkspace>/GHF-Studios/Vapor-Client
```

can be a filesystem realization step associated with an already-defined identity migration.

The physical filesystem move itself does not define the identity migration.

The Registry/model migration does.

This distinction is essential.

---

# Migration Safety

Repository migration is architecture work.

It is not cleanup-by-find-and-replace.

Migration must preserve:

* authored Git history;
* uncommitted local work;
* unpushed commits;
* branches;
* source provenance;
* meaningful submodule relationships;
* known-good self-hosting behavior;
* managed-toolchain behavior;
* Steam deployment knowledge;
* Platform deployment knowledge;
* Registry/provider mappings;
* migration history;
* normative documentation;
* useful legacy implementation;
* recoverability between migration waves.

No repository is destroyed merely because its current semantic boundary is obsolete.

Historical repositories should normally be archived only after replacement behavior and source ancestry are preserved.

---

# Migration Must Respect Git

Git repositories are authored source.

Migration must not casually:

```text
git reset --hard
delete dirty worktrees
discard branches
drop unpushed commits
force submodules to historical gitlinks
```

because those operations simplify migration.

Before moving or consolidating repositories, Git state must be inspected and intentionally preserved.

Dirty does not mean broken.

Detached submodule HEAD does not mean broken.

---

# Migration Dispositions

Repository migration uses the following dispositions:

```text
RENAME
    continuing repository/source responsibility receives a better name

KEEP
    current repository boundary remains meaningful

EXPAND
    repository remains active and absorbs additional responsibility

CONSOLIDATE
    still-valid implementation/responsibility moves into another active Source Repo

ARCHIVE
    repository leaves active topology but remains preserved for history

REVIEW
    current responsibility is ambiguous/conflicting and must be resolved first

FUTURE
    reserved concept; do not create merely to complete a diagram
```

These describe migration intent.

They are not necessarily one-step operations.

For example:

```text
Vapor-SDK

CONSOLIDATE
→ replacement proven
→ ARCHIVE
```

---

# Client Product / Source Family

The current first-party client-side Source Repo Container is historically named:

```text
Vapor-Root
```

Its target identity is:

```text
GHF-Studios/Vapor-Client
```

The new name describes the actual product/source responsibility:

> **the complete user-side Vapor Client source family**

rather than using `Root` as a vague synonym for “main.”

---

# Client Container Migration

Target:

```text
provider:
    GHF-Studios/Vapor-Root
    →
    GHF-Studios/Vapor-Client

Vapor identity:
    GHF-Studios/Vapor-Root
    →
    GHF-Studios/Vapor-Client
```

Disposition:

```text
RENAME
```

The Git repository continues.

Its canonical Vapor identity changes deliberately.

All contained Source Repo and Project identities migrate under the new Container prefix.

---

# Current Client Source Repo Inventory

The historical Client Container has included Source Repos such as:

```text
Vapor
Vapor-SDK
Vapor-Launcher
Vapor-Shell
Vapor-Examples
Vapor-Installer
Vapor-Entrypoint
```

The exact active physical membership at execution time must be verified from the authoritative `.gitmodules` and Git state before migration.

Documentation must not substitute for that inventory.

The conceptual dispositions are:

| Source Repo        | Disposition                             | Intended responsibility                                                        |
| ------------------ | --------------------------------------- | ------------------------------------------------------------------------------ |
| `Vapor`            | **KEEP / EXPAND**                       | primary active Vapor Client implementation                                     |
| `Vapor-Examples`   | **KEEP**                                | official examples/architecture-proving facility                                |
| `Vapor-SDK`        | **CONSOLIDATE → ARCHIVE**               | preserve useful GUI/design/implementation ancestry, remove competing semantics |
| `Vapor-Launcher`   | **CONSOLIDATE → ARCHIVE**               | migrate useful Launcher implementation into current Client ownership           |
| `Vapor-Installer`  | **CONSOLIDATE → ARCHIVE**               | retain Installer product boundary while consolidating source                   |
| `Vapor-Entrypoint` | **CONSOLIDATE → ARCHIVE**               | retain executable/platform adapter as appropriate                              |
| `Vapor-Shell`      | **REINTERPRET / CONSOLIDATE → ARCHIVE** | preserve useful interactive concepts without a second canonical CLI            |

The migration is semantic.

Repository size does not decide whether a boundary survives.

---

# Vapor Source Repo

`Vapor` remains the primary Source Repo / Vapor Workspace implementing the Vapor Client.

Its responsibility may include:

```text
Vapor Core
universal CLI
Installer implementation
Launcher implementation
SDK implementation
Entrypoint
shared GUI/backend implementation
platform-client integration
managed toolchain behavior
shared first-party development primitives
```

The exact crate/package decomposition remains implementation-driven.

The guiding rule is:

> **Executable boundaries do not automatically require Git repository boundaries.**

---

# Shared Vapor Core

The active Client Source Repo should increasingly embody:

```text
Vapor Core
    shared semantics

frontends / executables
    projections/adapters
```

rather than:

```text
Launcher semantic implementation
SDK semantic implementation
Shell semantic implementation
Installer semantic implementation
```

all independently defining Vapor.

There should be one semantic authority in implementation.

---

# Vapor SDK Migration

`Vapor-SDK` is a migration source, not garbage.

Useful material may include:

* visual product language;
* Figma-derived GUI implementation;
* Explorer/work-area/Inspector concepts;
* tool-window behavior;
* useful Tauri/application infrastructure;
* frontend interaction lessons;
* code still compatible with current Vapor Core;
* design ancestry.

What should not survive as an independent authority is:

```text
separate SDK Core
separate SDK identity model
separate SDK CLI semantics
```

Useful frontend implementation should be adapted to current shared semantics.

Obsolete business logic should be documented/archived rather than transplanted blindly.

---

# Vapor Launcher Migration

`Vapor-Launcher` reflects an older source-ownership boundary.

Launcher remains a real user-facing mode/surface.

That does not require Launcher to retain a dedicated Source Repo.

Intended migration:

```text
Vapor-Launcher
    ↓ consolidate useful implementation
Vapor
    ↓ replacement proven
archive Vapor-Launcher
```

Source consolidation must not erase the Launcher product concept.

---

# Vapor Installer Migration

Installer remains a distinct product/application boundary.

Its responsibility is capability/tooling establishment and modification.

But:

> **Product boundary does not require Source Repo boundary.**

Intended migration:

```text
Vapor-Installer
    ↓ consolidate
Vapor
    ↓ replacement proven
archive Vapor-Installer
```

The resulting Client source may still produce:

```text
vapor-installer
```

as a dedicated executable.

---

# Vapor Entrypoint Migration

`Vapor-Entrypoint` is primarily an executable/platform adapter.

Its semantic behavior should rely on shared Client/Core logic.

Intended migration:

```text
Vapor-Entrypoint
    ↓ consolidate
Vapor
    ↓ replacement proven
archive Vapor-Entrypoint
```

A small dedicated binary may survive.

A dedicated repository need not.

---

# Vapor Shell Migration

The old `Vapor-Shell` must not remain a second canonical implementation of the universal Vapor CLI.

Useful material may include:

```text
interactive terminal UX
command lessons
history/session concepts
terminal integration
documentation
```

A future Vapor Shell may exist as:

```text
interactive frontend
+
Vapor Core operations
+
frontend-local interaction/session state
```

It must not own a competing semantic model.

Intended disposition:

```text
REINTERPRET
→ CONSOLIDATE useful work
→ ARCHIVE historical Source Repo
```

---

# Vapor Examples

`Vapor-Examples` remains useful.

Its purpose is broader than ordinary Client implementation.

It provides architecture-proving material for:

```text
Content Projects
dependency resolution
Library semantics
Cargo reconciliation
Packagepack composition
Engine/Game/Mod integration
development workflows
```

Disposition:

```text
KEEP
```

Its current placement beneath the Client Container may be reconsidered later.

Do not move it merely because another topology looks aesthetically cleaner.

Its stable first-party **Examples facility** may remain available even if its backing source identity changes later.

---

# Examples Facility vs Repository Placement

The public semantic facility:

```text
examples
```

is conceptually distinct from:

```text
current Source Repo:
    GHF-Studios/Vapor-Client/Vapor-Examples
```

A future topology migration may move Examples.

The trusted first-party facility binding can then be updated.

This is precisely why bespoke first-party facilities should not be defined solely by present Git topology.

---

# Vapor Platform Server Source Family

The historical first-party server-side Source Repo Container is:

```text
Vapor-Server-Root
```

Its target identity is:

```text
GHF-Studios/Vapor-Platform-Server
```

The new name describes its actual responsibility:

> **Vapor ecosystem/control-plane server infrastructure**

rather than using vague `Root` terminology.

---

# Platform Container Migration

Target:

```text
provider:
    GHF-Studios/Vapor-Server-Root
    →
    GHF-Studios/Vapor-Platform-Server

Vapor identity:
    GHF-Studios/Vapor-Server-Root
    →
    GHF-Studios/Vapor-Platform-Server
```

Disposition:

```text
RENAME
```

As on the Client side, provider rename and Vapor identity migration may be separate waves.

The Container's Source Repo and Project descendants migrate under the new canonical prefix.

---

# Platform Source Repo Inventory

The Platform Server family has included independently meaningful services such as:

```text
Vapor-Homepage-Server
Vapor-Docs-Server
Vapor-Identity-Server
Vapor-Diagnostics-Server
Vapor-Registry-Server
```

Actual execution-time membership must be verified from the authoritative Container Git topology.

The conceptual target is to preserve service Source Repos when they represent meaningful independent service boundaries.

---

# Platform Service Boundaries

Unlike several historical Client Source Repos, Platform service repositories may represent real independent boundaries.

Reasons include:

* independent deployment;
* separate persistence;
* independent scaling;
* narrower failure domains;
* service-specific configuration;
* service-specific release/deployment cadence;
* implementation isolation;
* independent evolution.

Therefore:

> **Do not consolidate Platform services merely to make the Platform topology resemble the Client topology.**

Symmetry is not an architectural goal.

---

# Platform Server Container Responsibility

`Vapor-Platform-Server` owns whole-Platform orchestration.

It may own:

```text
service membership
deployment topology
shared configuration
reverse proxy configuration
service-manager configuration
cross-service contracts
whole-platform test/build recipes
health checks
smoke tests
deployment orchestration
state backup/restore orchestration
operational documentation
```

Individual Source Repos own their service-specific implementation.

---

# Platform Service Project Structure

A service Source Repo may contain one or more Projects where useful.

For example:

```text
GHF-Studios/Vapor-Platform-Server/Vapor-Registry-Server/<Project>
```

The exact Project names remain subject to implementation pressure.

Do not introduce meaningless Projects merely to satisfy a visual hierarchy.

A Source Repo can have the minimal Project structure actually required by its development model.

---

# Vapor Registry Authority

The canonical runtime Registry service is the source/service responsible for Vapor Registry semantics.

Historical source includes both:

```text
Vapor-Registry
Vapor-Registry-Server
```

These must not remain independent canonical authorities for the same facts.

The old `Vapor-Registry` repository is therefore archaeology/review material unless a narrower legitimate purpose is identified.

---

# Vapor Registry Conflict Resolution

Valid outcomes for historical `Vapor-Registry` include:

## Archive

Migrate still-valid contracts/data into the production Registry system and archive the old repository.

## Repurpose

Retain only a narrowly defined static/bootstrap/policy responsibility and rename it accordingly.

Possible conceptual responsibilities might include:

```text
Registry bootstrap data
reviewed static policy
seed configuration
```

What must not survive is:

```text
two independently authoritative Vapor Registries
```

Production Registry authority must be singular.

---

# Registry Model Must Migrate Too

The current Registry implementation may still encode older concepts such as:

```text
ecosystem
top-level repository
provider repository
```

The target Registry model must eventually represent the canonical identity topology:

```text
Authority
→ Source Repo Container
→ Source Repo
→ Project
```

as well as:

```text
provider linkage
trust / first-party state
facility bindings
versions/publication
acquisition policy
```

Repository rename work should not pretend this schema migration has already happened.

It receives its own migration wave.

---

# First-Party Trust During Migration

Both target Containers:

```text
GHF-Studios/Vapor-Client
GHF-Studios/Vapor-Platform-Server
```

are trusted first-party source.

That trust is Registry/Root-Authority controlled.

Migration must preserve:

```text
first-party classification
facility bindings
protected operation authority
```

without reducing first-party status to:

```text
repository happens to be owned by GHF-Studios on GitHub
```

Provider ownership is evidence/linkage.

Vapor trust remains a Vapor concept.

---

# First-Party Facility Bindings

Target stable facilities include at least:

```text
Client
Platform Server
Examples
```

Conceptually:

```text
client
    → GHF-Studios/Vapor-Client

platform-server
    → GHF-Studios/Vapor-Platform-Server

examples
    → current trusted Examples source identity
```

The Registry/model should own these bindings.

Bespoke CLI semantics should resolve through them rather than hardcoding every repository detail throughout the codebase.

---

# Vapor Documentation

The current Premium Docs are ecosystem-wide normative documentation.

They currently live in the historical Client source family.

Their semantic scope is broader than Client.

Separately:

```text
Vapor-Docs-Server
```

is a Platform service responsible for serving/publishing documentation.

These are different concerns:

```text
authored documentation
    ≠
documentation-serving service
```

---

# Documentation Migration

Do not move Premium Docs during the initial Container/provider/identity migration.

They are one of the primary authorities describing that migration.

Moving the authority while simultaneously changing the topology it defines unnecessarily increases risk.

Preferred order:

```text
update normative docs
→ migrate core source identity/topology
→ prove/stabilize
→ reconsider documentation source ownership
→ extract only if still architecturally justified
```

A future:

```text
Vapor-Documentation
```

Source Repo/Container may be appropriate.

Its exact identity/placement should be designed explicitly rather than created merely because documentation is cross-cutting.

---

# Future Vapor App Server

`Vapor-App-Server` remains:

```text
FUTURE
```

Do not create an empty Source Repo Container merely to complete a diagram.

Create the product/source family when Vapor App Server implementation actually begins.

Its responsibilities are defined by the Client/Platform/App Runtime model.

---

# Target Local Topology — First Stable Shape

After the primary Container identity migrations and local-layout normalization, but before historical Client Source Repo consolidation is complete, the local shape may resemble:

```text
<Superworkspace>/
└── GHF-Studios/
    ├── Vapor-Client/
    │   ├── Vapor/
    │   ├── Vapor-SDK/
    │   ├── Vapor-Launcher/
    │   ├── Vapor-Shell/
    │   ├── Vapor-Examples/
    │   ├── Vapor-Installer/
    │   └── Vapor-Entrypoint/
    │
    └── Vapor-Platform-Server/
        ├── Vapor-Homepage-Server/
        ├── Vapor-Docs-Server/
        ├── Vapor-Identity-Server/
        ├── Vapor-Diagnostics-Server/
        └── Vapor-Registry-Server/
```

Actual Source Repo membership must reflect the authoritative Git topology at execution time.

This diagram describes conceptual target ownership, not permission to manufacture missing repositories.

---

# Target Local Topology — Consolidated Client

After Client consolidation:

```text
<Superworkspace>/
└── GHF-Studios/
    ├── Vapor-Client/
    │   ├── Vapor/
    │   └── Vapor-Examples/
    │
    └── Vapor-Platform-Server/
        ├── Vapor-Homepage-Server/
        ├── Vapor-Docs-Server/
        ├── Vapor-Identity-Server/
        ├── Vapor-Diagnostics-Server/
        └── Vapor-Registry-Server/
```

Again, service membership may evolve where actual architecture requires it.

Archived repositories remain available for history but are no longer authored members of the active Container topology.

---

# Source Repo Container Acquisition After Migration

After migration, first-party acquisition should map naturally to the new identities.

Generic:

```text
vapor source-repo-container acquire GHF-Studios/Vapor-Client
```

and:

```text
vapor source-repo-container acquire GHF-Studios/Vapor-Platform-Server
```

Bespoke equivalents may include:

```text
vapor client acquire
vapor platform-server acquire
```

Both paths should use the same underlying registered acquisition semantics.

---

# Migration Wave 0 — Documentation and Baseline

Before changing provider or Vapor identity:

* update normative docs to the new identity model;
* inventory actual active Git repositories;
* inspect actual `.gitmodules`;
* record current Container commits;
* record current Source Repo gitlinks;
* record dirty/unpushed state;
* record current managed-toolchain state;
* preserve a known-good Client self-hosting path;
* preserve known-good Platform deployment state;
* record Registry/provider state;
* record the current canonical Superworkspace configuration;
* do not mix unrelated cleanroom refactors into the migration baseline.

This wave establishes recoverability.

---

# Wave 0 — Source Safety Gate

Migration may proceed only when every unique authored state has been classified.

At minimum:

```text
clean + pushed
committed locally but unpushed
dirty/uncommitted
detached development state
archaeological/replaceable
```

No migration procedure may assume:

```text
remote exists
therefore local source is disposable
```

Unique local work must be preserved explicitly.

---

# Migration Wave 1 — Provider Repository Renames

Rename the two top-level provider repositories:

```text
GitHub:
GHF-Studios/Vapor-Root
→
GHF-Studios/Vapor-Client
```

and:

```text
GitHub:
GHF-Studios/Vapor-Server-Root
→
GHF-Studios/Vapor-Platform-Server
```

This wave changes provider repository names.

It does **not yet need to change canonical Vapor identities**.

Immediately reconcile:

* local Git remotes;
* Registry provider linkage;
* deployment configuration referring to provider URLs;
* CI/provider references;
* scripts which genuinely refer to provider URLs;
* documentation describing provider state.

Then restore a known-good working system.

---

# Why Provider Rename Comes First

This intermediate state is valid:

```text
Vapor identity:
    GHF-Studios/Vapor-Root

provider:
    github.com/GHF-Studios/Vapor-Client
```

because the provider repository is linkage.

Separating the two changes gives us a rollback point.

If provider rename causes unexpected breakage, we can fix that class of issue without simultaneously debugging:

```text
canonical ID migration
local directory migration
manifest migration
Client consolidation
```

---

# Migration Wave 2 — Registry Identity Model Readiness

Before changing canonical Container identities, the Registry must be capable of representing the new hierarchy and identity migration safely.

It must understand at least:

```text
Authority
Source Repo Container
Source Repo
Project
```

and must support the required migration mapping:

```text
old canonical identity
→ new canonical identity
```

for the two Container prefixes and their descendants.

Do not migrate canonical IDs before Vapor can recognize the result.

---

# Migration Wave 3 — Canonical Container Identity Migration

Perform the explicit Vapor identity migrations:

```text
GHF-Studios/Vapor-Root
→
GHF-Studios/Vapor-Client
```

and:

```text
GHF-Studios/Vapor-Server-Root
→
GHF-Studios/Vapor-Platform-Server
```

This changes the canonical identities of:

```text
Containers
Source Repo descendants
Project descendants
```

through exact hierarchy-prefix migration.

The migration should update:

* Registry canonical identities;
* parent/child relationships;
* first-party trust state;
* facility bindings;
* remembered Context;
* authored references where identity is intentionally persisted;
* generated references;
* relevant manifests;
* local Vapor indexes/state;
* operation configuration;
* IDE-derived projections where necessary.

---

# Migration Wave 4 — Canonical Local Superworkspace Layout

After canonical identity has migrated, normalize local source paths to the canonical Superworkspace layout:

```text
<Superworkspace>/
└── GHF-Studios/
    ├── Vapor-Client/
    └── Vapor-Platform-Server/
```

This may involve both:

```text
introducing the Authority directory
renaming/moving Container checkout directories
```

depending on the current local layout.

The physical move is a local realization migration.

It must preserve Git working state.

---

# Local Move Safety

Before moving a Container checkout:

* inspect Git state;
* preserve dirty files;
* preserve submodule working state;
* preserve local branches;
* preserve remotes;
* preserve unpushed commits;
* account for IDE/open-file paths;
* account for active processes;
* stop operations which assume the old path where necessary.

Moving a Git worktree directory is not inherently destructive.

But surrounding tooling may retain path references.

Those references must be reconciled deliberately.

---

# Derived State After Local Move

After canonical local paths change, Vapor should regenerate or reconcile derived state such as:

```text
IDE integration
indexes
generated path caches
operation realization
toolchain environment references
diagnostic indexes
```

This is appropriate Repair/reconciliation territory because these states are derived.

Do not solve stale path references by resetting authored source.

---

# Migration Wave 5 — Manifest and Vocabulary Migration

Once the identity hierarchy and local topology are stable, migrate machine-readable terminology.

Historical concepts may include names such as:

```text
root
vapor-root
server-root
ecosystem
source
workspace identity
structural address
```

Each occurrence must be classified by meaning.

Examples:

```text
filesystem root
    keep "root"

Root Authority
    keep "Root"

Vapor-Root product/source family
    migrate to Client terminology

Vapor-Server-Root product/source family
    migrate to Platform Server terminology

generic ecosystem object
    remove/reassign to concrete semantic owner

bare source namespace
    distinguish Source Repo Container vs Source Repo
```

Do not globally replace strings.

---

# Migration Wave 6 — Public CLI Migration

The public CLI should converge from historical shapes such as:

```text
vapor ecosystem ...
vapor source ...
```

toward the current model:

```text
source-repo-container
source-repo

client
platform-server
examples

typed create operations

generic existing-object operations
```

Compatibility aliases may exist temporarily where migration pressure requires them.

They must have explicit retirement intent.

---

# Migration Wave 7 — Client Source Consolidation

Only after the active Client identity/topology is stable should historical Client Source Repos be consolidated.

Recommended conceptual sequence remains:

```text
narrow executable adapters first
→ richer frontend repositories later
```

For each Source Repo:

```text
inventory useful source/design
→ identify current semantic owner
→ cleanly reimplement/transplant valid behavior
→ prove replacement
→ remove authored submodule membership
→ archive historical repository
```

Do not blindly copy obsolete architecture merely to preserve it.

Git history already preserves archaeology.

---

# Consolidation Is Not File Copying

The preferred pattern is:

```text
understand mechanism
→ identify still-valid semantics
→ implement them cleanly in current architecture
→ prove behavior
→ fossilize old source
```

rather than:

```text
copy every old file
→ patch until it compiles
→ carry historical architecture forever
```

The migration should reduce conceptual debt.

---

# Migration Wave 8 — Registry Archaeology Resolution

Resolve the historical `Vapor-Registry` repository.

The result must clearly identify the canonical owner of:

```text
Vapor identity
source topology
provider linkage
first-party trust
facility bindings
versions
publication state
yank/ban state
authorization/policy where applicable
```

If historical `Vapor-Registry` has no unique continuing responsibility, archive it.

If it has a real narrower purpose, name and model that purpose explicitly.

---

# Migration Wave 9 — Documentation Source Reconsideration

After the main Client/Platform topology is proven:

```text
review current Premium Docs ownership
```

Only then decide whether authored normative docs should move into something such as:

```text
Vapor-Documentation
```

Do not create the repository in advance of that decision.

---

# Migration Wave 10 — Cleanroom Source Pass

Once repository ownership is stable, perform the broad cleanroom/docsification pass.

For active source:

```text
understand
→ simplify
→ rename
→ restructure
→ strengthen ownership
→ strengthen types
→ strengthen tests
→ document
→ remove dead compatibility archaeology
```

Do not invest heavily in polishing source boundaries already scheduled for retirement.

---

# GitHub Rename Redirects

GitHub repository rename redirects may provide useful transition behavior.

They are not the permanent model.

Authored provider links should eventually use the new canonical provider repository names directly.

Vapor identity resolution must not depend indefinitely on GitHub redirect behavior.

---

# Submodule Migration

Source Repo Container `.gitmodules` remains authored Git topology.

During top-level provider/identity rename:

* child Git repositories do not need to be recreated;
* child provider repository names do not need to change merely because their Container identity prefix changes;
* their canonical Vapor Source Repo identities **do** migrate because Container identity is part of the canonical hierarchy.

This distinction is important.

Example:

```text
provider child repository:
    GHF-Studios/Vapor

may remain unchanged
```

while its Vapor identity changes:

```text
GHF-Studios/Vapor-Root/Vapor
→
GHF-Studios/Vapor-Client/Vapor
```

Provider identity and Vapor identity are separate.

---

# Removing Historical Submodules

When Client Source Repos are successfully consolidated:

```text
remove the submodule from active Container topology
```

deliberately.

This includes:

* `.gitmodules`;
* gitlink;
* authored topology metadata;
* Registry active-source relationships;
* relevant operation recipes;
* documentation.

Historical repositories remain preserved remotely/archived.

Do not keep dead submodules merely as memorials.

---

# Source Repo Movement Between Containers

Moving a Source Repo from one Source Repo Container to another is an identity migration.

For example:

```text
GHF-Studios/Vapor-Client/Vapor-Examples
```

to:

```text
GHF-Studios/Some-Other-Container/Vapor-Examples
```

changes the canonical Source Repo identity.

Therefore such a move requires:

```text
new canonical identity
old→new migration
parent topology update
Registry update
local realization move
descendant Project identity migration
```

It is not merely “move the folder.”

This is one reason the Examples relocation should not be mixed into the primary Client migration.

---

# Project Movement

Likewise, moving a Project between Source Repos changes canonical Project identity.

For example:

```text
GHF-Studios/Foo/Repo-A/My-Project
```

to:

```text
GHF-Studios/Foo/Repo-B/My-Project
```

is an identity migration.

Project movement should therefore be rare, intentional, and explicitly migrated.

---

# Renaming Source Repo or Project

Because hierarchy segments participate in identity:

```text
rename Source Repo
rename Project
```

are also identity migrations.

Published Content identity may impose even stronger restrictions.

The migration model must not casually introduce renames merely for aesthetic consistency.

---

# Provider Repository Rename Without Vapor Rename

The reverse distinction also matters.

A Source Repo's backing GitHub repository may be renamed while preserving its Vapor identity if the modeled Vapor hierarchy is intentionally unchanged.

For example:

```text
Vapor Source Repo identity:
    GHF-Studios/Foo/Game

provider repository:
    github.com/GHF-Studios/Old-Repo
    →
    github.com/GHF-Studios/New-Repo
```

may be only provider-linkage migration.

This is legitimate when the Vapor identity path itself is not being changed.

---

# Migration Proof

A migration wave is complete only when the relevant workflows work again.

Documentation updates alone do not prove migration.

---

# Client Proof

As applicable, prove:

```text
Steam App Instance discovery
Vapor User Data discovery
canonical Superworkspace discovery
Authority/Container/Repo/Project discovery
managed toolchain
managed Cargo
Vapor CLI
Client build
Client test
local Client deployment
Steam deployment path
Content resolution
Project operations
Examples workflows
IDE reconciliation
```

The exact smoke commands evolve with the CLI.

The semantic capabilities are what matter.

---

# Platform Proof

As applicable, prove:

```text
Platform Container discovery
Source Repo/service discovery
service build/test
whole-Platform build/test
local deployment
VPS deployment path
Registry
Identity
Diagnostics
Docs
Homepage
reverse proxy
service manager
health checks
state backup/restore
```

A Platform migration is not complete merely because repositories have the right names.

---

# Recovery Proof

Migration must also preserve/reprove recovery properties.

At minimum:

```text
replaceable application state can be reacquired
registered source can be reacquired when truly absent
authored local Git work is preserved
derived state can be regenerated
canonical Superworkspace can be rediscovered
provider linkage can be restored
managed toolchain can be restored
build/test can be restored
```

---

# Compatibility Policy

Compatibility support may temporarily recognize:

```text
old provider names
old Vapor identities
old local paths
old manifest vocabulary
old CLI aliases
```

where necessary to perform migration safely.

Every compatibility layer must have:

```text
purpose
scope
migration target
retirement condition
```

Do not let migration aliases become a second permanent architecture.

---

# Naming Rules

Preferred active product/source-family names are:

```text
Vapor Client
Vapor Platform Server
Vapor App Server
```

Avoid vague `Root` product naming.

The word `root` remains appropriate for precise unrelated concepts such as:

```text
filesystem root
Installation root
Superworkspace root
Root Authority
```

---

# First-Party Source Naming

First-party provider repositories should use names which communicate their actual responsibility.

Container names should correspond to stable source/product families where possible.

Source Repo names should describe meaningful repository responsibility.

Project names should describe meaningful development units.

Do not introduce redundant path repetition merely for mechanical symmetry.

---

# Exact Current Inventory Is Runtime Evidence

This document intentionally describes architectural intent.

When migration actually executes, the authoritative current inventory is obtained from:

```text
Git
.gitmodules
Registry
local source
provider repositories
current deployment state
```

not from remembered diagrams.

If documentation says a Source Repo exists but authoritative Git topology says otherwise, investigate the discrepancy.

Do not manufacture a repository merely to satisfy stale documentation.

---

# Migration Invariants

* The canonical source hierarchy is Authority → Source Repo Container → Source Repo → Project.
* Source Repo equals Vapor Workspace.
* The Superworkspace has no canonical Vapor identity.
* The canonical local Superworkspace layout includes Authority to prevent collisions.
* Provider repository identity, Vapor identity, and local realization path are distinct.
* Provider rename may occur before canonical Vapor identity migration.
* Source Repo Container rename is an explicit canonical Vapor identity migration.
* Container identity migration changes descendant Source Repo and Project canonical identities.
* Exact old→new identity mappings must be preserved during migration.
* Identity migration does not require destroying/recreating Git repositories.
* Local Superworkspace relocation does not change canonical identity.
* Local canonical path normalization is separate from provider and identity migration.
* `Vapor-Root` migrates to `Vapor-Client`.
* `Vapor-Server-Root` migrates to `Vapor-Platform-Server`.
* Client application/executable boundaries do not automatically require separate Source Repos.
* `Vapor` remains the primary active Client implementation Source Repo.
* Historical Client application Source Repos are archaeology/migration inputs until replacements are proven.
* Installer remains a distinct product boundary even if its source is consolidated.
* The old SDK GUI/design ancestry must be preserved.
* `Vapor-Examples` remains active during the primary migration.
* Examples may later move only through an explicit Source Repo identity migration.
* Platform service Source Repos remain independent where the service boundary is meaningful.
* Platform Server owns whole-platform orchestration.
* `Vapor-Registry` and `Vapor-Registry-Server` must not remain independent canonical Registry authorities.
* First-party trust is Vapor Registry/Root-Authority state, not merely GitHub ownership.
* Client, Platform Server, and Examples may have stable first-party facility bindings over changing source topology.
* Premium Docs remain in place through the primary topology migration.
* `Vapor-App-Server` is not created until implementation pressure requires it.
* Git state must be preserved through migration.
* Dirty/unpushed source must not be treated as disposable.
* Repository consolidation must prove replacement before archival.
* Compatibility aliases are transitional and require retirement conditions.
* Actual Git/Registry/provider state outranks stale inventory text during execution.

---

# Open Migration Questions

The following remain intentionally open:

* Exact final Project decomposition inside `GHF-Studios/Vapor-Client/Vapor`.
* Exact Project decomposition inside each Platform service Source Repo.
* Exact future Source Repo Container kind/purpose taxonomy.
* Exact Registry schema for canonical hierarchy migration.
* Exact representation and expiry of old→new identity aliases.
* Exact compatibility behavior for persisted old canonical IDs.
* Exact canonical Superworkspace migration tooling.
* Exact provider-side timing of child-repository renames, if any are later desired.
* Exact long-term placement of `Vapor-Examples`.
* Exact long-term ownership/identity of authored Vapor documentation.
* Exact disposition of historical `Vapor-Registry`.
* Exact cleanroom consolidation order after the main identity/topology migration.
* Exact migration behavior for published Content whose canonical Project identity would otherwise change.
* Exact CLI compatibility lifetime for historical `ecosystem` / `source` commands.
