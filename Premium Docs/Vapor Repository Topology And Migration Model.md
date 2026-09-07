> [!info]
> This document defines the intended Vapor repository topology and the controlled migration from the current legacy/rewrite-era repository layout.
>
> Product/runtime terminology is defined by the **Vapor Client, Platform And App Runtime Model**.
>
> Identity, Structural Address, Local Realization, and development-context semantics are defined by the **Vapor Context, Identity, Session And Selection Model**.
>
> This document is both architectural model and migration ledger.
>
> Repository migration must follow it deliberately rather than being performed as a blind global rename.

---

# Migration Thesis

The current repository layout reflects several generations of Vapor architecture.

Some repositories represent durable source boundaries.

Others exist because an earlier architecture assigned semantics independently to Launcher, SDK, Installer, Shell, and similar applications.

The rewrite has since established the opposite governing principle:

> **Binaries and graphical surfaces do not own Vapor semantics. Vapor Core owns semantics; applications expose projections of them.**

Repository topology should increasingly reflect that principle.

The migration therefore has two different shapes:

```text
Client side
    consolidate historical application/source splits

Platform side
    preserve meaningful independently deployable service boundaries
```

Do not force both sides into symmetrical repository structures.

---

# Migration Safety

Repository migration is architecture work, not cleanup-by-find-and-replace.

The migration must preserve:

* authored Git history;
* uncommitted/unpushed local work;
* source provenance;
* useful legacy implementation;
* current working self-hosting behavior;
* existing Vapor Installation functionality;
* Steam deployment knowledge;
* Platform deployment knowledge;
* documentation required to understand the migration;
* old→new address information needed to repair remembered development state.

No repository is deleted merely because its semantics have moved elsewhere.

Retired repositories should normally be archived after their useful source/history has been migrated and replacement behavior is proven.

---

# Migration States

Every repository participating in this migration receives one of these dispositions:

```text
RENAME
    same continuing repository responsibility under a better name

KEEP
    active repository boundary remains semantically appropriate

CONSOLIDATE
    useful implementation moves into another active Workspace

ARCHIVE
    no longer active topology after migration; retained for history

REVIEW
    current responsibility conflicts with the target model and must
    be resolved before migration

FUTURE
    reserved concept; do not create merely to fill the topology
```

A repository may pass through several states.

For example:

```text
Vapor-SDK
    CONSOLIDATE
    → replacement proven
    → ARCHIVE
```

---

# Current Client-Side Topology

The current client-side Container Repo is:

```text
Vapor-Root
```

Its current direct Workspace/submodule members are:

```text
Vapor
Vapor-SDK
Vapor-Launcher
Vapor-Shell
Vapor-Examples
Vapor-Installer
Vapor-Entrypoint
```

This topology contains several historical application-oriented source boundaries.

Those boundaries should not all survive merely because they already exist.

---

# Target Client Container Repo

The continuing user-side Container Repo becomes:

```text
Vapor-Root
    ↓
Vapor-Client
```

Disposition:

```text
RENAME
```

This is the same continuing source-family/container responsibility under terminology matching the actual product model.

The rename changes Structural Addresses beneath the Container Repo.

It does not automatically create new semantic Workspace identities.

Conceptually:

```text
old:
    GHF-Studios/Vapor-Root/Vapor

new:
    GHF-Studios/Vapor-Client/Vapor
```

may describe the same semantic `GHF-Studios/Vapor` Workspace at a new Structural Address.

---

# Client Workspace Ledger

| Current repository | Disposition                           | Target                                                                                   |
| ------------------ | ------------------------------------- | ---------------------------------------------------------------------------------------- |
| `Vapor-Root`       | **RENAME**                            | `Vapor-Client`                                                                           |
| `Vapor`            | **KEEP / EXPAND**                     | primary active Vapor Client implementation Workspace                                     |
| `Vapor-SDK`        | **CONSOLIDATE → ARCHIVE**             | useful SDK implementation/design migrates into `Vapor`                                   |
| `Vapor-Launcher`   | **CONSOLIDATE → ARCHIVE**             | Launcher surface migrates into `Vapor`                                                   |
| `Vapor-Installer`  | **CONSOLIDATE → ARCHIVE**             | Installer binary/surface remains distinct; implementation moves into `Vapor`             |
| `Vapor-Entrypoint` | **CONSOLIDATE → ARCHIVE**             | Entrypoint implementation moves into `Vapor`                                             |
| `Vapor-Shell`      | **CONSOLIDATE/REINTERPRET → ARCHIVE** | useful concepts migrate; future Shell is a frontend/session projection over current Core |
| `Vapor-Examples`   | **KEEP**                              | remains active architecture-proving/examples Workspace for now                           |

The classification is semantic rather than size-based.

A tiny repository may remain independent if it owns a real boundary.

A large repository may be retired if its boundary is conceptually obsolete.

---

# Vapor Workspace

`Vapor` remains the primary active implementation Workspace for the Vapor Client.

Its existing rewrite already contains:

```text
vapor_core
vapor_macros
```

and the universal `vapor` binary surface.

It should progressively become the coherent source home for the Client applications and projections which share Vapor Core semantics.

The exact final crate decomposition is not frozen by this migration document.

Possible responsibilities include:

```text
Core semantics
CLI
desktop shell
Launcher projection
SDK projection
Installer projection
Entrypoint
shared GUI/backend integration
platform-client integration
```

The repository should be split by meaningful crate/module responsibility rather than by a historical assumption that every shipped executable needs its own Git repository.

---

# Vapor SDK Migration

`Vapor-SDK` is not discarded.

Its useful work must be classified before retirement.

Especially important migration material includes:

* the Figma-derived `vapor_sdk_gui`;
* the visual product language;
* application layout;
* Explorer/work-area/Inspector/tool-window mechanics;
* useful Tauri integration;
* SDK-specific implementation which still matches the new Core model;
* lessons encoded in obsolete SDK Core/CLI implementations.

The current repository contains distinct SDK CLI/Core/GUI crates.

Under the new architecture:

```text
SDK-specific semantic core
```

must not survive as a competing authority to:

```text
Vapor Core
```

The migration therefore distinguishes:

```text
visual/frontend implementation
    preserve and adapt

duplicated SDK business semantics
    port only where still valid

obsolete architecture
    document/archive rather than reproduce
```

`Vapor-SDK` is archived only after its valuable source and design ancestry are preserved.

---

# Vapor Launcher Migration

The standalone Launcher repository represents an older ownership boundary.

Its README currently assigns install/select/validate/compose/launch workflows to Launcher ownership.

The new model instead assigns semantics to Vapor Core and presents Launcher Mode as one graphical projection.

Therefore:

```text
Vapor-Launcher
    CONSOLIDATE → Vapor
    ARCHIVE afterward
```

Useful Launcher UI/platform integration may be migrated.

A separate Launcher executable may still exist.

A separate Git repository is not required merely because that executable exists.

---

# Vapor Installer Migration

Installer remains a real product/application boundary.

It does **not** follow that Installer requires an independent source repository.

The intended architecture is:

```text
Vapor Core
    owns shared semantics

Installer surface
    owns capability/install interaction and process boundary
```

Therefore:

```text
Vapor-Installer
    CONSOLIDATE → Vapor
    ARCHIVE afterward
```

The resulting source may still build:

```text
vapor-installer
```

as a dedicated executable.

Installer remains separate from Launcher/SDK at runtime/product level even though its source participates in the same coherent Workspace.

This distinction is important:

> **Source consolidation does not erase product boundaries.**

---

# Vapor Entrypoint Migration

`Vapor-Entrypoint` is an executable/platform-adapter boundary rather than an independent semantic domain.

Therefore:

```text
Vapor-Entrypoint
    CONSOLIDATE → Vapor
    ARCHIVE afterward
```

The Entrypoint may remain its own tiny binary crate.

Its implementation should depend on shared Client/Core behavior rather than owning a parallel model.

---

# Vapor Shell Migration

The existing `Vapor-Shell` repository requires conceptual reinterpretation.

It currently builds its own `vapor` executable and carries an older command/session model.

The current rewrite already owns the canonical universal `vapor` CLI through `Vapor`.

Therefore the old Shell must not remain a second canonical command implementation.

Disposition:

```text
CONSOLIDATE useful concepts
→ REINTERPRET future Shell semantics
→ ARCHIVE old repository
```

Useful material may include:

* interactive shell UX;
* commands/workflow lessons;
* session concepts;
* documentation;
* terminal integration.

A future Vapor Shell should be modeled as:

```text
interactive frontend
+
Development Session
+
Vapor Core operations
```

rather than a separate semantic implementation.

Its final executable/command spelling remains open.

---

# Vapor Examples

`Vapor-Examples` remains valuable as an architecture-proving Workspace.

It currently provides the fastest place to exercise:

* Content identity;
* resolution;
* Library;
* Cargo reconciliation;
* Packagepack realization;
* future Bevy Engine/Game/Mod behavior.

Disposition:

```text
KEEP
```

Its long-term structural placement may be reconsidered later because examples are ecosystem-wide rather than inherently Client-only.

Do not move it during the primary Client migration unless real pressure requires it.

Avoid combining topology cleanup with unnecessary example relocation.

---

# Current Platform-Side Topology

The current server-side Container Repo is:

```text
Vapor-Server-Root
```

It currently contains:

```text
Vapor-Homepage-Server
Vapor-Docs-Server
Vapor-Identity-Server
Vapor-Diagnostics-Server
Vapor-Registry-Server
```

Unlike the legacy Client application split, these repositories represent independently deployable Platform services.

That separation remains meaningful.

---

# Target Platform Container Repo

The Container Repo becomes:

```text
Vapor-Server-Root
    ↓
Vapor-Platform-Server
```

Disposition:

```text
RENAME
```

The new name states what the repository actually orchestrates:

```text
Vapor Platform server-side infrastructure
```

rather than using the vague term `Root`.

---

# Platform Workspace Ledger

| Current repository         | Disposition | Target                                             |
| -------------------------- | ----------- | -------------------------------------------------- |
| `Vapor-Server-Root`        | **RENAME**  | `Vapor-Platform-Server`                            |
| `Vapor-Homepage-Server`    | **KEEP**    | Platform Homepage service                          |
| `Vapor-Docs-Server`        | **KEEP**    | Platform Documentation-serving/publication service |
| `Vapor-Identity-Server`    | **KEEP**    | Platform Identity service                          |
| `Vapor-Diagnostics-Server` | **KEEP**    | Platform Diagnostics service                       |
| `Vapor-Registry-Server`    | **KEEP**    | Platform Registry runtime service                  |
| `Vapor-Registry`           | **REVIEW**  | resolve authority/data overlap before migration    |

The `*-Server` suffix on the service repositories is currently acceptable because each repository actually implements a server-side service.

Further naming normalization may happen later.

It is not required for the Container Repo migration.

---

# Platform Services Stay Independent

The Platform migration does not consolidate all services into one Cargo repository merely for aesthetic symmetry.

Independent service repositories allow:

* independent deployment;
* independent scaling;
* separate persistence boundaries;
* narrower failure domains;
* service-specific implementation choices;
* independent evolution.

The Platform Server Container Repo owns platform-wide orchestration.

The service Workspaces own service-specific behavior.

This is a real architectural boundary and should remain.

---

# Vapor Registry Conflict

There are currently two repositories with partially overlapping claims:

```text
Vapor-Registry

Vapor-Registry-Server
```

`Vapor-Registry` describes itself as a machine-readable Git-governed authority for identities, Steam metadata, and permission grants.

`Vapor-Registry-Server` describes itself as the canonical Registry service for Vapor semantic identity and provider linkage.

Both must not remain independently canonical for the same facts.

Therefore `Vapor-Registry` is placed in:

```text
REVIEW
```

until its intended role is explicitly resolved.

Possible legitimate outcomes include:

### Retire

Migrate still-valid data/contracts into the Registry service and archive `Vapor-Registry`.

### Repurpose

Keep a Git-reviewed static policy/bootstrap dataset with a more precise name and explicitly make the Registry Server consume/import it.

Possible conceptual names include:

```text
Vapor-Registry-Policy
Vapor-Registry-Bootstrap
Vapor-Platform-Policy
```

The exact choice is not yet frozen.

What is frozen is:

> **There must not be two independently canonical Vapor Registries.**

Do not add the current `Vapor-Registry` repository to the new Platform Container Repo before this responsibility conflict is resolved.

---

# Vapor Documentation

The current Premium Docs are ecosystem-wide normative documentation even though they presently live inside the Client Container Repo.

Long term, authored ecosystem documentation should have a source identity distinct from the Platform service which publishes/serves documentation.

Conceptually:

```text
Vapor-Documentation
    authored source:
    books
    models
    reference source
    diagrams

Vapor-Docs-Server
    Platform service:
    publishing
    hosting
    version serving
    documentation service state
```

These are different responsibilities.

---

# Documentation Migration Timing

Do **not** move the Premium Docs during the first Container Repo rename.

They are currently the normative migration authority.

Moving them at the same time as the source topology they describe introduces unnecessary failure modes.

Migration order:

```text
freeze/update normative docs
    ↓
rename/migrate Client + Platform topology
    ↓
stabilize
    ↓
extract authored docs deliberately
    ↓
introduce Vapor-Documentation
```

The eventual Structural Address/container placement of `Vapor-Documentation` remains open until cross-cutting ecosystem source organization is designed.

---

# Future Vapor App Server

`Vapor-App-Server` is:

```text
FUTURE
```

Do not create an empty repository merely to complete a visual topology.

Create it only when server-side Vapor App runtime work begins.

Its eventual responsibilities are defined in the **Vapor Client, Platform And App Runtime Model**.

---

# First Target Topology

After the first rename wave, before Client consolidation:

```text
Superworkspace
├── Vapor-Client
│   ├── Vapor
│   ├── Vapor-SDK
│   ├── Vapor-Launcher
│   ├── Vapor-Shell
│   ├── Vapor-Examples
│   ├── Vapor-Installer
│   └── Vapor-Entrypoint
│
└── Vapor-Platform-Server
    ├── Vapor-Homepage-Server
    ├── Vapor-Docs-Server
    ├── Vapor-Identity-Server
    ├── Vapor-Diagnostics-Server
    └── Vapor-Registry-Server
```

This first state intentionally changes naming without simultaneously changing every child boundary.

That makes the migration inspectable and recoverable.

---

# Consolidated Client Target

After Client consolidation:

```text
Superworkspace
├── Vapor-Client
│   ├── Vapor
│   └── Vapor-Examples
│
└── Vapor-Platform-Server
    ├── Vapor-Homepage-Server
    ├── Vapor-Docs-Server
    ├── Vapor-Identity-Server
    ├── Vapor-Diagnostics-Server
    └── Vapor-Registry-Server
```

Archived historical repositories remain available on GitHub but are no longer active members of the Container Repo.

Later additions may include:

```text
Vapor-Documentation
Vapor-App-Server
```

when their structural placement and implementation pressure justify them.

---

# Client Workspace Target Shape

The exact crate names remain subject to code-cleanroom pressure, but the intended source ownership resembles:

```text
Vapor/
├── crates/
│   ├── vapor_core/
│   ├── vapor_macros/
│   ├── ... shared implementation ...
│   └── ... frontend-specific adapters ...
│
└── binaries / applications
    ├── vapor
    ├── vapor-installer
    ├── vapor-entrypoint
    └── desktop Launcher/SDK surface
```

Additional executables such as a dedicated SDK projection or future Shell may exist when useful.

The invariant is:

> **Executable count does not determine repository count.**

---

# Migration Wave 0 — Freeze and Inventory

Before any GitHub rename:

* all active repositories are committed/pushed or deliberately recorded as dirty;
* current submodule revisions are recorded;
* current working Steam installation remains known-good;
* current Platform deployment state is recorded;
* old repository URLs are recorded;
* migration docs are current;
* current structural-address assumptions are inventoried;
* no unrelated architecture work is mixed into the rename commit.

A tag or clearly identified pre-migration commit should exist for critical repositories.

---

# Migration Wave 1 — Container Repo Renames

Perform only:

```text
Vapor-Root
→ Vapor-Client

Vapor-Server-Root
→ Vapor-Platform-Server
```

Then reconcile:

* GitHub repository names;
* local remotes;
* Container Repo checkout directory names where desired;
* Superworkspace discovery;
* documentation;
* links;
* CI references;
* deployment references;
* provider metadata.

Do not simultaneously consolidate child repositories.

The system should return to a healthy known state before Wave 2.

---

# Migration Wave 2 — Manifest and Vocabulary Migration

Remove overloaded `Root` terminology from active machine-readable configuration.

Known migration targets include concepts currently spelled like:

```text
[root]
vapor-root
[server_root]
Vapor Server Root
server-root.toml
```

Do not apply a blind textual substitution.

Each field must be classified by responsibility.

For example, a field referring to the Client product/source family should become Client-oriented.

A field referring to a filesystem root should retain the ordinary word `root`.

A field referring to Root Authority should remain unchanged.

Schema migration should be explicit and version-aware where persisted data is involved.

---

# Migration Wave 3 — Client Consolidation

Migrate client repositories one at a time.

Recommended sequence:

```text
Vapor-Entrypoint
    ↓
Vapor-Installer
    ↓
Vapor-Launcher
    ↓
Vapor-SDK
    ↓
Vapor-Shell
```

This order starts with narrower adapters and leaves the richest historical/prototype surfaces until the Core/Desktop model is ready to absorb them.

For each repository:

```text
inventory
→ identify still-valid behavior
→ transplant/reimplement against current Vapor Core
→ test replacement
→ remove Container Repo submodule
→ archive old repository
```

Do not copy obsolete duplicated business semantics merely for preservation.

History is preserved by the archived repository.

The active codebase should preserve only behavior/design which still belongs in the current architecture.

---

# Migration Wave 4 — Registry Authority Resolution

Resolve `Vapor-Registry` versus `Vapor-Registry-Server`.

The output must state exactly which system owns:

* semantic Vapor identity;
* provider/resource linkage;
* official namespace policy;
* Steam product metadata;
* publishing permission grants;
* bootstrap data;
* runtime Registry state.

Only then:

```text
retire
or
rename/repurpose
```

the old `Vapor-Registry`.

---

# Migration Wave 5 — Documentation Extraction

After Client/Platform topology is stable:

```text
Premium Docs
    ↓
Vapor-Documentation
```

if the Documentation source model remains desirable after another design check.

This migration should preserve:

* Git provenance where practical;
* normative filenames/links or redirects;
* diagrams;
* future book source;
* generated-reference integration.

`Vapor-Docs-Server` remains separate as the Platform publication/service implementation.

---

# Migration Wave 6 — Cleanroom Source Pass

Once active repository boundaries match the new topology, perform the full source cleanroom/docsification pass.

This is not a comment-only sweep.

For each active source file:

```text
understand
→ rename
→ restructure
→ tighten ownership
→ add useful crate/module/item documentation
→ strengthen types/invariants/tests
→ remove stale compatibility archaeology
→ replace the whole file where appropriate
```

The cleanroom pass is deliberately performed **after** repository ownership is stabilized.

Otherwise high-quality documentation would be authored against boundaries already scheduled for deletion.

---

# GitHub Rename Redirects

GitHub repository renames normally provide useful redirect behavior.

The migration may benefit from that transitional behavior.

Vapor must not treat redirects as the permanent architecture.

All authored links/configuration should eventually reference the new canonical repository names directly.

---

# Submodule Migration

Container Repo `.gitmodules` files are canonical Git topology inputs.

During a Container Repo rename, child submodule identities need not change.

During Client consolidation, retired submodules must be removed deliberately.

The final Client Container Repo should not retain dead historical submodule entries merely as memorials.

Historical source remains discoverable through archived repositories and migration documentation.

---

# Local Checkout Migration

Filesystem directory renames are local-realization operations.

They are not semantic identity changes.

Vapor should eventually support repairing remembered development state from:

```text
old Structural Address
old local path
```

to:

```text
new Structural Address
new or unchanged local path
```

without requiring the user to recreate every Project/Workspace context manually.

Until that machinery exists, migration may use explicit local repair/bootstrap procedures.

---

# Structural Address Migration Ledger

The migration should preserve explicit address transformations such as:

```text
GHF-Studios/Vapor-Root
→ GHF-Studios/Vapor-Client

GHF-Studios/Vapor-Root/Vapor
→ GHF-Studios/Vapor-Client/Vapor

GHF-Studios/Vapor-Server-Root
→ GHF-Studios/Vapor-Platform-Server
```

Descendant mappings may be derived structurally when safe.

These address migrations do not automatically mean the semantic objects changed identity.

---

# Migration Proof

A migration wave is complete only when the relevant real workflows work again.

For the Client side this includes, as applicable:

```text
discover Installation
discover Superworkspace
discover Workspace/Project
managed Cargo
build/test Vapor
local ecosystem deploy
run deployed Vapor
Content resolution
```

For the Platform side this includes, as applicable:

```text
service build/test
deployment configuration
health checks
Registry
Identity
Diagnostics
Docs
Homepage
state backup/restore
```

Documentation correctness alone does not constitute migration proof.

---

# Cleanup Policy

Compatibility aliases and old-name support may exist temporarily when needed.

Every compatibility mechanism must have an intended retirement point.

Do not permanently accumulate:

```text
Root
Client
old path
new path
legacy alias
legacy legacy alias
```

throughout the architecture.

The purpose of the migration is convergence.

---

# Migration Invariants

* `Vapor-Root` becomes `Vapor-Client`.
* `Vapor-Server-Root` becomes `Vapor-Platform-Server`.
* `Vapor` remains the primary active Client implementation Workspace.
* Client executable/product boundaries do not automatically require separate repositories.
* `Vapor-SDK`, `Vapor-Launcher`, `Vapor-Installer`, `Vapor-Entrypoint`, and `Vapor-Shell` are migration sources, not permanent competing semantic authorities.
* Installer remains a distinct runtime/product boundary despite source consolidation.
* The old SDK GUI remains a visual design ancestor and must not be lost.
* Platform service repositories remain independently meaningful.
* Platform services are not consolidated merely for symmetry.
* `Vapor-Registry` and `Vapor-Registry-Server` must not remain overlapping canonical authorities.
* `Vapor-Examples` remains active during the primary migration.
* Premium Docs remain in place during the initial rename and consolidation work.
* Authored ecosystem documentation may later move to `Vapor-Documentation`.
* `Vapor-App-Server` is created only when implementation pressure requires it.
* Structural Address migration is distinct from Semantic Identity migration.
* Repository renames do not imply arbitrary semantic-object recreation.
* No active repository is retired until its replacement behavior/design has been intentionally migrated and proven.
* The final topology should contain fewer accidental repositories and clearer responsibility boundaries than the starting topology.
