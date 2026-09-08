> [!info]
> This document is the live execution ledger for the repository/topology migration defined by **Vapor Repository Topology And Migration Model**.
>
> The topology model defines the target architecture and migration semantics.
>
> This ledger records how the real repositories reach that architecture without losing source, deployment state, working rewrite behavior, provider linkage, or recoverability.
>
> Update this document as migration waves are completed.
>
> An unchecked later-wave item is not permission to perform it opportunistically during an earlier wave.

---

# Governing Rule

The migration proceeds through independently recoverable waves.

The primary rule is:

> **Change one class of state, restore a known-good Vapor system, then continue.**

The following are distinct operations:

```text
provider repository rename
≠
Registry provider-linkage update
≠
Registry identity-model migration
≠
canonical Vapor identity migration
≠
local Superworkspace path migration
≠
manifest/schema migration
≠
public CLI migration
≠
Source Repo consolidation
≠
cleanroom source refactor
```

They may belong to one architectural migration.

They must not become one giant transformation.

---

# Target Identity Summary

Current first-party Container identities:

```text
GHF-Studios/Vapor-Root
GHF-Studios/Vapor-Server-Root
```

Target canonical identities:

```text
GHF-Studios/Vapor-Client
GHF-Studios/Vapor-Platform-Server
```

Target provider repository names:

```text
github.com/GHF-Studios/Vapor-Client
github.com/GHF-Studios/Vapor-Platform-Server
```

Future reserved concept:

```text
GHF-Studios/Vapor-App-Server
```

`Vapor-App-Server` must not be created merely to complete the topology.

---

# Canonical Identity Reminder

The target hierarchy is:

```text
Authority
└── Source Repo Container
    └── Source Repo
        └── Project
```

Therefore changing:

```text
GHF-Studios/Vapor-Root
→
GHF-Studios/Vapor-Client
```

is not merely renaming one directory.

It migrates the canonical identity prefix of every descendant:

```text
GHF-Studios/Vapor-Root/Vapor
→
GHF-Studios/Vapor-Client/Vapor
```

and likewise any Projects beneath that Source Repo.

The same applies to the Platform Server Container.

---

# Provider Name, Vapor Identity, and Local Path Are Separate

Migration must track three states independently.

Example Client migration:

```text
provider repository:
    GHF-Studios/Vapor-Root
    →
    GHF-Studios/Vapor-Client

canonical Vapor identity:
    GHF-Studios/Vapor-Root
    →
    GHF-Studios/Vapor-Client

canonical local path:
    <legacy-superworkspace>/Vapor-Root
    →
    <Superworkspace>/GHF-Studios/Vapor-Client
```

These do not need to change simultaneously.

That separation is the basis for the migration waves below.

---

# Current Client Source Inventory

Historical Client Source Repo Container:

```text
Vapor-Root
```

Historically authored Source Repos include:

```text
Vapor
Vapor-SDK
Vapor-Launcher
Vapor-Shell
Vapor-Examples
Vapor-Installer
Vapor-Entrypoint
```

Target dispositions:

```text
Vapor
    KEEP / EXPAND

Vapor-Examples
    KEEP

Vapor-Entrypoint
    CONSOLIDATE → Vapor → ARCHIVE

Vapor-Installer
    CONSOLIDATE → Vapor → ARCHIVE

Vapor-Launcher
    CONSOLIDATE → Vapor → ARCHIVE

Vapor-SDK
    CONSOLIDATE → Vapor → ARCHIVE

Vapor-Shell
    REINTERPRET / CONSOLIDATE → Vapor → ARCHIVE
```

This list is architectural intent.

Before execution, actual membership must be verified from:

```text
.gitmodules
git submodule status
Registry state
provider repositories
```

Do not manufacture a missing repository merely because an old document listed it.

---

# Current Platform Source Inventory

Historical Platform Source Repo Container:

```text
Vapor-Server-Root
```

Historically authored service Source Repos include:

```text
Vapor-Homepage-Server
Vapor-Docs-Server
Vapor-Identity-Server
Vapor-Diagnostics-Server
Vapor-Registry-Server
```

These remain meaningful independent service Source Repos unless implementation evidence shows otherwise.

Historical:

```text
Vapor-Registry
```

remains separate archaeology / responsibility-review work.

---

# Migration Status Vocabulary

Use:

```text
ACTIVE
BLOCKED
READY
NOT STARTED
COMPLETE
DEFERRED
```

A wave is `COMPLETE` only after its completion gate is satisfied.

---

# Wave 0 — Normative Model and Baseline

Status:

```text
ACTIVE
```

Purpose:

> Establish the architecture and evidence required to make later migration waves recoverable.

This wave includes both:

```text
normative documentation convergence
+
real repository/runtime baseline
```

---

# Wave 0A — Normative Documentation

Before provider or identity migration, the following models must agree on the new source hierarchy and migration semantics:

```text
Glossary - Ecosystem Model.md

Vapor Context, Identity, Session And Selection Model.md

Vapor CLI Model.md

Vapor Development Experience Model.md

Vapor Repository Topology And Migration Model.md

Vapor Repository Migration Execution Ledger.md
```

Relevant later docs must be reconciled before they become execution authorities.

Current documentation migration must eliminate obsolete normative assumptions such as:

```text
multiple ordinary Superworkspaces
Workspace semantic identity independent of Container
Structural Address as separate canonical hierarchy
multiple canonical local realizations
CWD-based Vapor targeting
generic ecosystem CLI object
generic source CLI namespace
```

---

# Wave 0B — Repository Source Safety Inventory

Before any provider rename:

* [ ] Client Container working tree inspected.
* [ ] Platform Container working tree inspected.
* [ ] `Vapor` Source Repo working tree inspected.
* [ ] Every active Client Source Repo inspected.
* [ ] Every active Platform Source Repo inspected.
* [ ] Dirty working trees identified.
* [ ] Local unpushed commits identified.
* [ ] Detached development states identified.
* [ ] Submodule gitlinks recorded.
* [ ] Important branches/remotes recorded.
* [ ] No unique local work is assumed recoverable merely because a remote repository exists.

Useful evidence:

```bash
git status
git rev-parse HEAD
git branch -vv
git remote -v
git submodule status --recursive
```

---

# Source-State Classification

Each active repository should be classifiable as one of:

```text
clean + remotely recoverable

committed + pushed

committed but unpushed

dirty / uncommitted

intentional detached state

archaeological / replaceable

unknown — investigate before migration
```

`unknown` blocks destructive or topology-changing migration.

---

# Wave 0C — Client Known-Good Baseline

Preserve evidence for the currently working Client/self-hosting chain.

At minimum, the baseline should prove the capabilities currently relied upon, such as:

```text
Steam App Instance discovery
→ managed Vapor toolchain
→ canonical/current source discovery
→ Vapor Project resolution
→ managed Cargo
→ Vapor build/test
→ local first-party operation/deployment
→ deployed Vapor execution
```

Historical CLI spelling may still include old commands.

The baseline records semantic capability, not permanent command syntax.

---

# Wave 0D — Platform Known-Good Baseline

Preserve evidence for the current Platform environment, where applicable:

```text
Homepage
Docs
Identity
Diagnostics
Registry

reverse proxy
service manager
deployment mechanism
health checks
state backup/restore
```

Record which provider repository URLs and local paths deployment currently depends on.

---

# Wave 0E — Registry Baseline

Record current Registry behavior and persistence before migrating its identity model.

At minimum inventory:

```text
current schema
current registered first-party identities
current provider linkage
current official seed/bootstrap data
current production persistence path
current API contracts
current migration mechanism
current recovery/backup behavior
```

The existing Registry currently embodies older topology concepts.

Do not destroy evidence of how the working recovery flow currently functions.

---

# Wave 0F — Superworkspace Baseline

Record:

```text
current configured/assumed local source root
current Client Container path
current Platform Container path
current direct-workspace/source assumptions
current RustRover integration paths
current Vapor user-data references
```

The target one-Superworkspace model may require physical migration later.

Wave 0 merely records current reality.

---

# Wave 0 Completion Gate

Wave 0 is complete when:

```text
normative migration model coherent
+
critical Git state known
+
unique authored source protected
+
Client known-good baseline recorded
+
Platform known-good baseline recorded
+
Registry baseline recorded
+
current local source topology recorded
```

Only then begin Wave 1.

---

# Wave 1 — Provider Repository Renames

Status:

```text
BLOCKED BY WAVE 0
```

Purpose:

> Change only the names of the two top-level GitHub repositories and reconcile provider linkage.

Perform:

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

---

# Wave 1 Non-Goals

Do not yet:

```text
change canonical Vapor Container identities

move local checkouts

introduce Authority directories

rename child Source Repo provider repositories

rename Projects

consolidate Source Repos

rewrite the Registry hierarchy

perform broad manifest-schema migration

perform broad CLI migration

move Premium Docs

create Vapor-App-Server

perform cleanroom source refactors
```

---

# Wave 1 Intermediate State

This is valid:

```text
Vapor canonical identity:
    GHF-Studios/Vapor-Root

provider repository:
    github.com/GHF-Studios/Vapor-Client
```

and:

```text
Vapor canonical identity:
    GHF-Studios/Vapor-Server-Root

provider repository:
    github.com/GHF-Studios/Vapor-Platform-Server
```

This temporary mismatch is intentional.

It proves that Vapor identity and provider repository identity are distinct.

---

# Wave 1 — Local Git Remote Reconciliation

After provider rename:

Client:

```bash
git remote set-url origin https://github.com/GHF-Studios/Vapor-Client.git
```

Platform:

```bash
git remote set-url origin https://github.com/GHF-Studios/Vapor-Platform-Server.git
```

Then inspect:

```bash
git remote -v
git fetch
git status
git submodule status --recursive
```

Do not change child submodule URLs merely because their parent repository changed name.

---

# Wave 1 — Registry Provider Linkage

Update Registry/provider information whose meaning is:

> Which provider repository backs this existing Vapor identity?

For the temporary migration state:

```text
GHF-Studios/Vapor-Root
    provider → GHF-Studios/Vapor-Client

GHF-Studios/Vapor-Server-Root
    provider → GHF-Studios/Vapor-Platform-Server
```

This must not yet silently rewrite canonical Vapor identity.

---

# Wave 1 — Other Provider References

Inspect and reconcile references whose semantic meaning is provider location:

```text
Git remotes
GitHub Actions references
deployment clone/fetch URLs
poll/update configuration
provider API configuration
automation
documentation links required operationally
```

Do not globally replace old names in fields which actually represent Vapor identity.

---

# Wave 1 — GitHub Redirects

GitHub rename redirects may keep old provider URLs temporarily working.

This is not completion.

A provider reference is reconciled when active authored configuration points to the intended new provider name directly.

---

# Wave 1 — Client Verification

After provider rename:

* [ ] Client Container fetches from new provider URL.
* [ ] Child Source Repo/submodule topology remains intact.
* [ ] `Vapor` checkout remains intact.
* [ ] Unique local source is unchanged.
* [ ] Steam App discovery still works.
* [ ] Managed toolchain still works.
* [ ] Managed Cargo still works.
* [ ] Vapor can still build/test itself.
* [ ] Existing local deployment capability still works.
* [ ] Deployed Vapor still runs.
* [ ] No physical source move was required solely because provider name changed.

---

# Wave 1 — Platform Verification

After provider rename:

* [ ] Platform Container fetches from new provider URL.
* [ ] Service submodules remain intact.
* [ ] Registry service still works.
* [ ] Identity service still works.
* [ ] Diagnostics service still works.
* [ ] Documentation service still works.
* [ ] Homepage still works where applicable.
* [ ] Deployment automation uses reconciled provider URLs.
* [ ] Production data remains untouched.
* [ ] State backup/restore remains valid.
* [ ] No Source Repo was recreated merely due to parent provider rename.

---

# Wave 1 Completion Gate

Wave 1 is complete when:

```text
provider repositories renamed
+
local remotes reconciled
+
Registry provider linkage reconciled
+
Client known-good behavior restored
+
Platform known-good behavior restored
+
canonical Vapor identities still intentionally old
+
local paths still intentionally unchanged
```

Only then begin Wave 2.

---

# Wave 2 — Registry Identity Model Readiness

Status:

```text
BLOCKED BY WAVE 1
```

Purpose:

> Make Vapor capable of representing the target canonical source identity model before changing existing identities.

The Registry/Core must support:

```text
Authority
→ Source Repo Container
→ Source Repo
→ Project
```

---

# Wave 2 — Authority Model

At minimum:

```text
GHF-Studios
```

must exist as a Vapor Authority identity independent of:

```text
GitHub organization GHF-Studios
```

The Registry must be able to associate the two through provider linkage.

---

# Wave 2 — Source Repo Container Model

The Registry must distinguish:

```text
Source Repo Container
```

from:

```text
Source Repo
```

without using one generic repository layer.

Required first-party Containers include the current identities and migration targets.

---

# Wave 2 — Source Repo Model

Source Repo must be represented as:

```text
one Git repository
=
one Vapor Workspace
```

It belongs beneath one Source Repo Container in canonical identity.

The Registry should know relevant acquisition policy such as:

```text
Container-acquired by default
independently acquirable where explicitly permitted
```

---

# Wave 2 — Project Model

Project must be represented beneath Source Repo.

For Content Projects, Project kind may include:

```text
Game
Engine
Library
Packagepack
Enginepack
Gamepack
Modpack
Engine Mod
Game Mod
Extension Mod
```

Content kind is not an additional identity layer.

---

# Wave 2 — Trust and Facility Model

The Registry must be able to represent trusted first-party state sufficiently for migration.

At minimum preserve the distinction between:

```text
standard source
first-party source
```

and support stable facility bindings such as:

```text
client
platform-server
examples
```

where implemented.

Third-party source must not be able to self-declare first-party trust.

---

# Wave 2 — Identity Migration Support

Before Wave 3, Vapor must be capable of recording:

```text
old canonical identity
→
new canonical identity
```

for at least the Container migrations and exact descendants.

Required prefix migrations:

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

---

# Wave 2 — Migration Safety

The migration mechanism must not depend on fuzzy name matching.

Allowed:

```text
exact old Container identity
+
known descendants
+
exact prefix transformation
```

Not allowed:

```text
"this repository looks like Vapor-Client,
so it is probably the same thing"
```

---

# Wave 2 Verification

* [ ] Authority modeled independently from provider organization.
* [ ] Source Repo Container modeled.
* [ ] Source Repo modeled.
* [ ] Project modeled.
* [ ] Source Repo = Vapor Workspace reflected consistently.
* [ ] Provider linkage independent from Vapor identity.
* [ ] First-party trust survives model transition.
* [ ] Facility bindings can survive topology migration.
* [ ] Old→new exact identity migration can be represented.
* [ ] Current working recovery/acquisition behavior remains possible or has a proven equivalent.
* [ ] Existing Registry production data can be migrated without blind recreation.

---

# Wave 2 Completion Gate

Wave 2 is complete when Vapor can represent both:

```text
current canonical identities
```

and:

```text
target canonical identities
```

plus the exact migration between them.

Only then begin Wave 3.

---

# Wave 3 — Canonical Vapor Identity Migration

Status:

```text
BLOCKED BY WAVE 2
```

Purpose:

> Change canonical first-party Container identities and deterministically migrate their descendants.

Perform:

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

---

# Wave 3 — Descendant Migration

For every known Source Repo beneath Client:

```text
GHF-Studios/Vapor-Root/<Source-Repo>
→
GHF-Studios/Vapor-Client/<Source-Repo>
```

For every known Project beneath those Source Repos:

```text
GHF-Studios/Vapor-Root/<Source-Repo>/<Project>
→
GHF-Studios/Vapor-Client/<Source-Repo>/<Project>
```

Equivalent prefix migration applies to Platform Server.

---

# Wave 3 — Preserve Provider Repositories

Child provider repository names do not need to change merely because their Vapor identity prefix changes.

Example:

```text
provider repository:
    github.com/GHF-Studios/Vapor
```

may remain exactly the same.

Its canonical Vapor Source Repo identity changes because its parent Container identity changed.

---

# Wave 3 — Update Canonical References

Inspect and migrate authored/model state which deliberately stores canonical Vapor identities:

```text
Registry rows/records
parent-child topology
first-party facility bindings
trusted policy
authored manifests
dependency/source references where relevant
operation recipes
remembered Context
local Vapor state
generated indexes
IDE projections
tests/fixtures
diagnostics expectations
```

Classify each occurrence before modification.

---

# Wave 3 — Old Identity Compatibility

Old identities may remain recognized temporarily as migrated identities.

Conceptually:

```text
GHF-Studios/Vapor-Root
    migrated-to GHF-Studios/Vapor-Client
```

This is compatibility/migration state.

It must not mean that both names remain equally canonical forever.

---

# Wave 3 — Client Verification

* [ ] New Client Container identity resolves.
* [ ] Old Client identity resolves through explicit migration compatibility where intended.
* [ ] Client Source Repos resolve under new prefix.
* [ ] Projects resolve under new prefix.
* [ ] `client` first-party facility resolves to target identity.
* [ ] Acquisition logic can resolve the renamed Client source.
* [ ] Managed build/test still works.
* [ ] No Git history was recreated.
* [ ] No local checkout had to move merely because canonical identity changed.

---

# Wave 3 — Platform Verification

* [ ] New Platform Server Container identity resolves.
* [ ] Platform service Source Repos resolve beneath it.
* [ ] Projects resolve beneath service Source Repos.
* [ ] `platform-server` facility resolves correctly.
* [ ] Platform build/test still works.
* [ ] Deployment identity resolution still works.
* [ ] Production persistence is unchanged.
* [ ] No service Git repository was recreated.

---

# Wave 3 Completion Gate

Wave 3 is complete when:

```text
new canonical identities authoritative
+
descendants migrated
+
old identities explicitly transitional
+
provider linkage remains correct
+
Client working
+
Platform working
+
source still physically safe
```

Only then begin Wave 4.

---

# Wave 4 — Canonical Superworkspace Layout

Status:

```text
BLOCKED BY WAVE 3
```

Purpose:

> Normalize local source realization beneath the one canonical Superworkspace.

Target:

```text
<Superworkspace>/
└── GHF-Studios/
    ├── Vapor-Client/
    └── Vapor-Platform-Server/
```

---

# Wave 4 — Authority Directory

The target local topology includes:

```text
GHF-Studios/
```

because Authority is a canonical identity segment and different Authorities may own identically named Containers.

Example:

```text
GHF-Studios/Foo
Other-Author/Foo
```

must not collide locally.

---

# Wave 4 — Local Move Safety

Before moving any Container directory:

* [ ] inspect parent Container Git state;
* [ ] inspect Source Repo/submodule state;
* [ ] preserve dirty source;
* [ ] preserve local branches;
* [ ] preserve detached development state;
* [ ] preserve remotes;
* [ ] preserve unpushed commits;
* [ ] stop processes holding critical old paths where necessary;
* [ ] record RustRover/current IDE integration state;
* [ ] record Vapor user-data path references.

---

# Wave 4 — Physical Migration

Conceptually:

```text
legacy source root/
├── Vapor-Root/
└── Vapor-Server-Root/
```

becomes:

```text
<Superworkspace>/
└── GHF-Studios/
    ├── Vapor-Client/
    └── Vapor-Platform-Server/
```

Exact source commands depend on the real current filesystem state.

Do not prescribe destructive shell moves without inspecting that state.

---

# Wave 4 — Derived-State Reconciliation

After physical movement, regenerate/reconcile derived state:

```text
Vapor indexes
generated operation state
managed path caches
RustRover integration
other IDE integration
diagnostic indexes
generated manifests/projections
```

This is legitimate Repair/reconciliation work.

Authored source itself must not be regenerated merely because its path changed.

---

# Wave 4 — Context Reconciliation

Persistent Context referring to old identities/paths should migrate to new canonical identities.

Context migration should use exact identity mappings.

It should not infer objects from directory names.

---

# Wave 4 — Canonical Superworkspace Verification

* [ ] Vapor knows exactly one canonical Superworkspace root.
* [ ] Superworkspace has no identity.
* [ ] Authority directory exists in canonical local topology.
* [ ] Client Container located beneath `GHF-Studios`.
* [ ] Platform Container located beneath `GHF-Studios`.
* [ ] Source Repo submodules intact.
* [ ] Dirty/local Git state preserved.
* [ ] Vapor source discovery works without arbitrary recursion.
* [ ] RustRover integration reconciled.
* [ ] managed Cargo works.
* [ ] Client build/test works.
* [ ] Platform build/test works.

---

# Wave 4 Completion Gate

Wave 4 is complete when the canonical local realization model is operational and no workflow relies on the old physical root as semantic identity.

---

# Wave 5 — Machine Vocabulary and Public CLI Migration

Status:

```text
BLOCKED BY WAVE 4
```

Purpose:

> Converge active machine-readable vocabulary and public CLI on the new model.

This wave may be divided into independently provable subwaves.

---

# Wave 5A — Manifest Vocabulary

Inspect historical terms such as:

```text
root
vapor-root
server-root
ecosystem
source
Workspace identity
Structural Address
```

Each occurrence must be classified.

Keep valid meanings:

```text
filesystem root
Installation root
Superworkspace root
Root Authority
```

Migrate obsolete product/source meanings.

Do not perform blind string replacement.

---

# Wave 5B — Source Terminology

Public/model terminology should converge on:

```text
Source Repo Container
Source Repo
Project
```

Retire bare generic `source` where it ambiguously means one of several layers.

Machine code may retain transitional internal names temporarily where rewriting them immediately would increase risk.

Public semantics should converge first.

---

# Wave 5C — Ecosystem CLI Retirement

Historical:

```text
vapor ecosystem ...
```

must no longer represent one generic public object/lifecycle.

Operations move to concrete owners:

```text
client ...
platform-server ...
examples ...

source-repo-container ...
source-repo ...

generic Project operations
```

Compatibility aliases may exist temporarily.

They require explicit retirement criteria.

---

# Wave 5D — Context and Selection CLI

Implement/converge on the CLI model:

```text
persistent Context
transient --context
explicit --select
```

with the fundamental invariant:

```text
Context
    supplies missing left-side path

operation subject
    establishes natural scope

Selection
    supplies/narrows right-side topology
```

No CWD semantic targeting.

No hidden generic `--all`.

---

# Wave 5E — Creation CLI

Converge on explicit parent-layer creation:

```text
source-repo-container create

source-repo create

game create
engine create
library create
...
```

Typed Project creation must not silently create missing Containers or Source Repos.

---

# Wave 5F — Diagnostics

Before considering the CLI migration complete, prove high-quality diagnostics for:

```text
missing Container
Container registered but not acquired
missing Source Repo
ambiguous Project
illegal operation scope
missing authority
missing provider access
missing Git submodule checkout
dirty Git state
```

Diagnostics should provide safe exact resolution paths where known.

---

# Wave 5 Completion Gate

Wave 5 is complete when:

```text
active machine vocabulary reflects new topology
+
public CLI no longer depends on generic ecosystem/source abstractions
+
Context/Selection semantics implemented coherently
+
typed creation follows explicit parent hierarchy
+
diagnostics teach the new model
```

---

# Wave 6 — Client Source Repo Consolidation

Status:

```text
BLOCKED BY WAVE 5
```

Purpose:

> Remove historical Client Source Repo boundaries which no longer own meaningful independent semantics.

Recommended conceptual order:

```text
Vapor-Entrypoint
→
Vapor-Installer
→
Vapor-Launcher
→
Vapor-SDK
→
Vapor-Shell
```

This order remains guidance, not an excuse to ignore actual implementation dependencies.

---

# Wave 6 General Per-Repository Procedure

For each historical Source Repo:

```text
inventory
→
understand current mechanism
→
classify useful vs obsolete semantics
→
identify new owner
→
cleanly reimplement/transplant valid behavior
→
prove replacement
→
remove authored submodule membership
→
update Registry/topology
→
archive historical repository
```

Do not blindly copy obsolete architecture.

---

# Wave 6A — Vapor-Entrypoint

Target:

```text
CONSOLIDATE → Vapor
```

Completion requirements:

* [ ] useful Entrypoint behavior exists in current Client source;
* [ ] executable/platform adapter remains where appropriate;
* [ ] shared semantics live in Vapor Core;
* [ ] Steam startup path works;
* [ ] submodule membership removed only after proof;
* [ ] historical repository archived.

---

# Wave 6B — Vapor-Installer

Target:

```text
CONSOLIDATE → Vapor
```

Product boundary remains:

```text
Vapor Installer
```

Completion requirements:

* [ ] Installer operations use current shared semantics;
* [ ] dedicated executable remains where intended;
* [ ] Role/tooling provisioning works;
* [ ] downgrade/uninstall safety works;
* [ ] authored source is preserved;
* [ ] historical Source Repo removed only after proof.

---

# Wave 6C — Vapor-Launcher

Target:

```text
CONSOLIDATE → Vapor
```

Completion requirements:

* [ ] Launcher product/mode remains represented;
* [ ] no independent Launcher semantic Core remains;
* [ ] useful UI/platform implementation preserved;
* [ ] shared semantics come from current Vapor Core;
* [ ] historical Source Repo archived after replacement.

---

# Wave 6D — Vapor-SDK

Target:

```text
CONSOLIDATE → Vapor
```

Special preservation requirement:

> The Figma-derived SDK GUI and visual-product ancestry must not be lost.

Inventory explicitly:

```text
vapor_sdk_gui
vapor_sdk_core
vapor_sdk_cli
other frontend/design material
```

Classify:

```text
visual/frontend implementation
    preserve/adapt

still-valid feature behavior
    reimplement against current Core

duplicate semantic model
    do not preserve as competing authority

obsolete architecture
    archive
```

Completion requirements:

* [ ] visual ancestry preserved;
* [ ] useful frontend implementation preserved or deliberately replaced;
* [ ] no second SDK Core;
* [ ] no second SDK CLI;
* [ ] Launcher↔SDK experience uses one semantic backend;
* [ ] historical Source Repo archived only after proof.

---

# Wave 6E — Vapor-Shell

Target:

```text
REINTERPRET / CONSOLIDATE → Vapor
```

Preserve useful:

```text
interactive shell UX
terminal integration
history/navigation lessons
workflow concepts
```

Do not preserve a second canonical `vapor` CLI.

Future Shell model:

```text
interactive frontend
+
Vapor Core operations
+
frontend-local session/navigation state
```

---

# Wave 6 — Client Consolidation Target

Expected active Client Container membership after consolidation:

```text
Vapor-Client/
├── Vapor/
└── Vapor-Examples/
```

subject to later explicit Examples topology design.

Archived repositories remain remotely available for history.

---

# Wave 6 Completion Gate

Wave 6 is complete when:

```text
Vapor owns current Client implementation
+
historical app Source Repos no longer compete semantically
+
useful implementation/design preserved
+
archaeology remains available remotely
+
Client still builds/tests/deploys
+
Examples remain functional
```

---

# Wave 7 — Registry Archaeology Resolution

Status:

```text
BLOCKED BY REQUIRED PRIOR MODEL WORK
```

Conflict:

```text
Vapor-Registry
vs
Vapor-Registry-Server
```

There must be one canonical Registry authority.

---

# Wave 7 Required Ownership Decision

Define exactly which active system owns:

```text
Authority identity
Source Repo Container identity
Source Repo identity
Project identity
Content/version identity
provider linkage
first-party trust
facility bindings
publication state
yank/ban state
namespace policy
bootstrap/seed data
runtime Registry persistence
```

---

# Wave 7 Outcomes

Possible:

```text
Vapor-Registry
    ARCHIVE
```

or:

```text
Vapor-Registry
    REPURPOSE + RENAME
```

for a narrowly defined non-competing function such as static bootstrap/policy data.

Not allowed:

```text
two independently canonical Registries
```

---

# Wave 7 Completion Gate

* [ ] one canonical Registry authority;
* [ ] historical repository disposition explicit;
* [ ] no overlapping truth ownership;
* [ ] production Registry recovery remains proven.

---

# Wave 8 — Documentation Source Reconsideration

Status:

```text
DEFERRED
```

Only after Client/Platform topology has stabilized, reconsider:

```text
Premium Docs
→
Vapor-Documentation
```

The question is not:

> Would a docs repository look cleaner?

The question is:

> Does authored ecosystem documentation now have a sufficiently independent source responsibility to justify a Source Repo or Container identity?

If yes, design and migrate it explicitly.

If no, leave it where it coherently belongs.

---

# Documentation vs Docs Service

Keep distinct:

```text
authored normative documentation
```

and:

```text
Vapor-Docs-Server
    service which publishes/serves documentation
```

One does not require the other to share a Source Repo.

---

# Wave 9 — Source Cleanroom / Docsification

Status:

```text
NOT STARTED
```

Purpose:

> Harden active source only after repository ownership is stable.

For each active file/module:

```text
understand
→ compare with normative architecture
→ simplify
→ rename
→ restructure
→ strengthen ownership
→ strengthen types/invariants/tests
→ document useful contracts
→ remove stale archaeology
→ replace whole file where appropriate
```

The goal is not to preserve implementation shape for its own sake.

---

# Cleanroom Rule

Historical implementation is evidence.

It is not automatically the desired implementation.

Preferred approach:

```text
inspect old mechanism
→ understand why it worked
→ identify still-valid invariants
→ implement clean current version
→ prove behavior
→ fossilize obsolete source
```

---

# Documentation Layers

The cleanroom pass should support:

```text
clear code
↓
crate/module rustdoc
↓
public API documentation
↓
normative model docs
↓
generated reference
↓
Vapor Books / approachable learning material
```

Books should teach architecture which has survived implementation pressure.

---

# Wave 10 — Return to Runtime Roadmap

Status:

```text
NOT STARTED
```

The migration is infrastructure work in service of Vapor/Loo Cast development.

It must not become an infinite prerequisite.

After the agreed stabilization point:

```text
remaining Cargo/realization proof
→
minimal Bevy ECS Engine
→
Wheel Game
→
Engine/Game/Mod capability proof
→
static realization pressure
→
larger Vapor/Spacetime/Loo Cast work
```

The exact runtime roadmap remains owned by the rewrite roadmap.

---

# Cross-Wave Rule — Do Not Reacquire Source as Repair

Throughout every wave:

```text
missing entire registered Container
    → acquisition/recovery

missing declared submodule checkout
    → Git reconciliation

missing derived Vapor state
    → Repair
```

Do not use `repair` as a magical source-restoration command.

---

# Cross-Wave Rule — Git Remains Git

If migration encounters:

```text
dirty worktree
detached HEAD
unexpected gitlink
local branch divergence
unpushed commit
missing submodule
```

diagnose it as Git state.

Do not hide it behind vague Vapor corruption language.

---

# Cross-Wave Rule — Canonical Source Is Not Disposable

At no point may migration conclude:

```text
remote source exists
therefore local checkout can be deleted
```

without first proving that no unique local authored state would be lost.

---

# Cross-Wave Rule — Derived State Is Regeneratable

Migration may freely prefer regeneration over preservation for truly derived state such as:

```text
indexes
IDE integration
generated operation realization
caches
generated glue
derived source-discovery state
```

provided the regeneration path is proven.

---

# Cross-Wave Rule — Restore Known-Good After Every Wave

Every migration wave should end with:

```text
diagnose
→ prove source topology
→ prove managed toolchain
→ prove relevant build/test
→ prove relevant deployment/runtime behavior
→ record results
```

Do not stack broken migration waves.

---

# Cross-Wave Rule — Exact Evidence Beats Documentation

If this ledger says:

```text
five Platform Source Repos
```

but actual `.gitmodules` or Registry state says otherwise:

```text
investigate actual state
```

Do not modify reality to match stale text.

The order of authority for current execution evidence is approximately:

```text
Git/source state
Registry state
provider state
deployment state
current working binaries
normative target model
execution ledger inventory text
```

The target model defines where we are going.

Current evidence defines where we actually are.

---

# First-Party Facility Verification

As first-party bindings become available, prove:

```text
client
    resolves intended Client facility

platform-server
    resolves intended Platform Server facility

examples
    resolves intended Examples facility
```

Their implementations may change during migration.

Their stable facility semantics should remain coherent.

---

# Acquisition Verification

After the target Registry/source model exists, prove both generic and bespoke acquisition paths.

Generic:

```text
vapor source-repo-container acquire GHF-Studios/Vapor-Client

vapor source-repo-container acquire GHF-Studios/Vapor-Platform-Server
```

Bespoke:

```text
vapor client acquire
vapor platform-server acquire
vapor examples acquire
```

They should share underlying modeled acquisition machinery where appropriate.

---

# Destructive-Recovery Proof

The existing recovery milestone demonstrated the broad concept:

```text
working source/development environment
→ replace recoverable source/application state
→ Steam/Vapor reacquisition
→ Registry/provider source recovery
→ derived-state regeneration
→ build/test
```

After the identity/topology migration, repeat an equivalent proof against the new source model.

The new proof must distinguish:

```text
registered remotely recoverable source

unique local authored work

derived Vapor state
```

Unique local authored work must never be intentionally destroyed as part of the proof.

---

# Current Live Status

Update this block as the actual work proceeds.

```text
Wave 0 — Normative Model and Baseline
    ACTIVE

Wave 1 — Provider Repository Renames
    BLOCKED BY WAVE 0

Wave 2 — Registry Identity Model Readiness
    BLOCKED BY WAVE 1

Wave 3 — Canonical Vapor Identity Migration
    BLOCKED BY WAVE 2

Wave 4 — Canonical Superworkspace Layout
    BLOCKED BY WAVE 3

Wave 5 — Machine Vocabulary and Public CLI Migration
    BLOCKED BY WAVE 4

Wave 6 — Client Source Repo Consolidation
    BLOCKED BY WAVE 5

Wave 7 — Registry Archaeology Resolution
    NOT STARTED

Wave 8 — Documentation Source Reconsideration
    DEFERRED

Wave 9 — Source Cleanroom / Docsification
    NOT STARTED

Wave 10 — Return to Runtime Roadmap
    NOT STARTED
```

---

# Immediate Wave-0 Documentation Checklist

During the current documentation convergence pass:

```text
[ ] Context / Identity / Selection model
[ ] CLI model
[ ] Development Experience model
[ ] Ecosystem Glossary
[ ] Repository Topology and Migration model
[ ] Repository Migration Execution Ledger
[ ] Client / Platform / App Runtime model
[ ] Publishing and Distribution model
[ ] Ecosystem Operational model
[ ] Ecosystem Experience model
[ ] SDK Experience model
[ ] Rewrite Bootstrap
[ ] Rewrite Progress and Roadmap
[ ] affected diagrams
```

A checked item means it has been reconciled with the current model, not merely reread.

---

# Migration Completion Principle

The migration is not successful merely because repository names look nicer.

It is successful when:

```text
canonical identities match the intended hierarchy

+

provider linkage is explicit and independent

+

one canonical Superworkspace realizes source predictably

+

source boundaries match meaningful responsibilities

+

Vapor Core owns shared semantics

+

first-party facilities remain coherent

+

Git state remains safe and understandable

+

Registry owns one coherent identity model

+

CLI exposes concrete Vapor concepts

+

development state can be diagnosed and recovered

+

documentation teaches one architecture

+

known-good Client and Platform workflows still work

+

future implementation no longer requires archaeology merely to know
what the system means
```
