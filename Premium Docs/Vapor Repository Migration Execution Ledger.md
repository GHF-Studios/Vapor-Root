> [!info]
> This document is the live execution ledger for the repository/topology migration defined by **Vapor Repository Topology And Migration Model**.
>
> The topology model defines what the target architecture means.
>
> This ledger records how the current repositories reach that architecture without losing source, deployment state, or working rewrite behavior.
>
> Update this document as migration waves are completed.
>
> Do not treat unchecked later-wave items as permission to perform them opportunistically during an earlier wave.

---

# Governing Rule

The repository migration proceeds by independently recoverable waves.

The primary rule is:

> **Change one class of identity/topology at a time, restore a known-good system, then continue.**

In particular:

```text
GitHub repository rename
≠
local checkout rename
≠
manifest-schema migration
≠
Workspace identity migration
≠
Client source consolidation
≠
code cleanroom refactor
```

These operations may ultimately belong to one architectural migration.

They must not be executed as one blind transformation.

---

# Target Summary

The primary provider-side rename is:

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

The future name:

```text
GHF-Studios/Vapor-App-Server
```

is reserved but **must not be created during this migration** merely to complete the topology.

---

# Current Client Container Membership

Before migration, `Vapor-Root` directly contains:

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

No child repository is consolidated during Wave 1.

---

# Current Platform Container Membership

Before migration, `Vapor-Server-Root` directly contains:

```text
Vapor-Homepage-Server
Vapor-Docs-Server
Vapor-Identity-Server
Vapor-Diagnostics-Server
Vapor-Registry-Server
```

These service Workspaces remain active during the Container Repo migration.

Target disposition:

```text
Vapor-Homepage-Server
    KEEP

Vapor-Docs-Server
    KEEP

Vapor-Identity-Server
    KEEP

Vapor-Diagnostics-Server
    KEEP

Vapor-Registry-Server
    KEEP
```

The independent `Vapor-Registry` repository remains under separate REVIEW.

---

# Wave 0 — Baseline And Freeze

Status:

```text
IN PROGRESS
```

Purpose:

> Establish enough evidence that every later migration wave can be checked against a known-good pre-migration state.

## Repository baseline

Before provider renaming:

* [ ] `Vapor-Root` working tree is intentionally clean or all intentional changes are committed/pushed.
* [ ] `Vapor-Server-Root` working tree is intentionally clean or all intentional changes are committed/pushed.
* [ ] `Vapor` working tree is intentionally clean or all intentional changes are committed/pushed.
* [ ] Current Client Container Repo commit SHA is recorded.
* [ ] Current Platform Container Repo commit SHA is recorded.
* [ ] Current `Vapor` rewrite commit SHA is recorded.
* [ ] Current Client submodule revisions are recorded.
* [ ] Current Platform submodule revisions are recorded.
* [ ] No child repository rename is mixed into this baseline.

Useful local evidence:

```bash
git status
git rev-parse HEAD
git submodule status --recursive
```

## Vapor baseline

Record that the current rewrite still proves the known-good chain:

```text
installed Vapor
→ canonical Steam App Instance
→ managed Cargo/toolchain
→ Vapor Project resolution
→ source-built Vapor
→ local ecosystem operation/deployment
```

At minimum preserve evidence that the currently relied-upon commands still work before migration.

The exact smoke commands may evolve with the CLI.

The important point is that the same semantic capabilities must survive Wave 1.

## Platform baseline

Record the currently known-good Platform state, including where applicable:

```text
Homepage
Docs
Identity
Diagnostics
Registry
reverse proxy
service manager
deployment poll/update mechanism
health checks
state backup/restore
```

Provider rename must not silently invalidate production deployment automation.

## Documentation baseline

The following models should exist and be committed before Wave 1:

```text
Vapor Client, Platform And App Runtime Model.md
Vapor Context, Identity, Session And Selection Model.md
Vapor Repository Topology And Migration Model.md
Vapor Repository Migration Execution Ledger.md
```

The existing Premium Docs remain in the Client Container Repo throughout the first migration waves.

---

# Wave 0 Completion Gate

Wave 0 is complete when:

```text
critical repositories committed/pushed
+
baseline revisions known
+
Client self-hosting workflow known-good
+
Platform deployment state known
+
migration models committed
```

Only then begin Wave 1.

---

# Wave 1 — Provider / Container Repo Rename

Status:

```text
DEFERRED
```

Purpose:

> Rename only the two top-level Container Repos at the Git provider.

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

Do not during this operation:

```text
rename child repositories
rename local checkout directories
change Workspace semantic identities
change Project names
change manifest schemas
consolidate repositories
move Premium Docs
create Vapor-App-Server
perform cleanroom refactors
```

---

# Wave 1 — Local Checkout Policy

The local checkout directories should **not initially be renamed**.

Immediately after the GitHub rename it is valid to have:

```text
local directory:
    .../Vapor-Root/

remote:
    https://github.com/GHF-Studios/Vapor-Client
```

and:

```text
local directory:
    .../Vapor-Server-Root/

remote:
    https://github.com/GHF-Studios/Vapor-Platform-Server
```

This temporary mismatch is deliberate.

It preserves:

* RustRover paths;
* current Superworkspace assumptions;
* scripts containing local paths;
* remembered development state;
* manual shell habits;
* known-good self-hosting behavior.

Local checkout naming is a **Local Realization migration** and occurs separately.

---

# Wave 1 — Remote Reconciliation

After the provider rename, update the two Container Repo local remotes explicitly.

Client Container Repo:

```bash
git remote set-url origin https://github.com/GHF-Studios/Vapor-Client.git
```

Platform Container Repo:

```bash
git remote set-url origin https://github.com/GHF-Studios/Vapor-Platform-Server.git
```

Then verify:

```bash
git remote -v
git fetch
git status
git submodule status --recursive
```

Child submodule URLs do not change merely because their parent Container Repo was renamed.

Do not rewrite child `.gitmodules` entries unless a child repository itself changes.

---

# Wave 1 — Provider References

Provider-level references to the renamed Container Repos must be reconciled during Wave 1 where they are operationally required.

Examples include:

```text
Git remotes
GitHub Actions references
deployment polling/clone URLs
provider configuration
external automation which names the Container Repo
documentation links needed to operate the migration
```

This differs from Wave 2.

Wave 1 changes references whose meaning is:

> Which GitHub repository should I contact?

Wave 2 changes authored Vapor vocabulary/schema whose meaning is:

> What semantic concept does this field model?

---

# Wave 1 — GitHub Redirect Policy

GitHub rename redirects may temporarily preserve old URLs.

They are a migration aid.

They are not the target architecture.

Therefore:

```text
old provider URL works because redirect exists
```

does not count as final reconciliation.

Authored active configuration should eventually point directly to the new repository name.

---

# Wave 1 — Client Verification

After the Client Container Repo rename:

* [ ] Local Client Container Repo can fetch from the new provider URL.
* [ ] Existing child submodules remain intact.
* [ ] Existing `Vapor` checkout remains intact.
* [ ] Installed Vapor still discovers the Steam App Instance.
* [ ] Managed Cargo/toolchain operation still works.
* [ ] Vapor can still resolve the relevant Project context.
* [ ] Vapor can still build/test itself through the managed toolchain.
* [ ] Local ecosystem deployment still works where it worked before.
* [ ] Deployed Vapor still runs.
* [ ] No source had to move physically merely to satisfy the provider rename.

A failure here blocks Wave 2.

---

# Wave 1 — Platform Verification

After the Platform Container Repo rename:

* [ ] Local Platform Container Repo can fetch from the new provider URL.
* [ ] All five service submodules remain intact.
* [ ] Homepage source remains buildable/deployable.
* [ ] Docs service source remains buildable/deployable.
* [ ] Identity source remains buildable/deployable.
* [ ] Diagnostics source remains buildable/deployable.
* [ ] Registry Server source remains buildable/deployable.
* [ ] Deployment automation no longer relies on an obsolete provider URL except through a deliberately temporary compatibility path.
* [ ] Existing state/storage locations remain untouched.
* [ ] Existing production data is not migrated merely because the repository was renamed.

A failure here blocks Wave 2.

---

# Wave 1 Completion Gate

Wave 1 is complete only when:

```text
provider repos renamed
+
local remotes point directly at new names
+
Client self-hosting still works
+
Platform source/deployment still works
+
child repository topology unchanged
```

At this point the ecosystem may temporarily contain old `Root` terminology in machine-readable manifests.

That is expected.

It is Wave 2 work.

---

# Wave 2 — Machine Vocabulary And Schema Migration

Status:

```text
NOT STARTED
```

Purpose:

> Replace old product/domain vocabulary with Client/Platform terminology without confusing it with ordinary filesystem-root concepts or Root Authority.

Known current migration targets include:

```text
App-Source.vapor.toml
    [root]
    name = "vapor-root"
    repository = ".../Vapor-Root"

App.vapor.toml
    [root]
    name = "vapor-root"

server-root.toml
    [server_root]
    name = "Vapor Server Root"

repository / documentation terminology
    Vapor Root Workspace
    Vapor Root Project
    Vapor Server Root Workspace
    Vapor Server Root Project
```

These changes require deliberate schema design.

Do not transform:

```text
root
```

globally.

Legitimate remaining meanings include:

```text
filesystem root
Installation root
source root
state root
document root
Root Authority
```

---

# Wave 2 — Current Vapor Workspace Naming

The current active `Vapor/Workspace.vapor.toml` contains conceptually:

```text
Workspace:
    vapor

Project:
    vapor
```

This is now a known naming-pressure point.

The Workspace and Project names must not be changed merely as part of the Container Repo provider rename.

Wave 2 or the following identity/project-model implementation pass must decide meaningful canonical casing/naming.

The desired invariant is:

> A Project receives a meaningful name within its Workspace namespace rather than duplicating the Workspace name by accident.

---

# Wave 2 — Identity / Address Implementation

Wave 2 should begin representing:

```text
Semantic Identity
≠
Structural Address
≠
Local Realization
```

at least enough that future Container Repo/local-checkout migration does not rely on raw paths as identity.

Conceptual example:

```text
Workspace identity:
    GHF-Studios/Vapor

Structural address:
    GHF-Studios/Vapor-Client/Vapor

Local realization:
    /home/.../Vapor
```

The implementation need not support every future selector/session feature before migration continues.

It must stop encoding the old assumption that current physical/Container placement *is* the durable object identity.

---

# Wave 3 — Client Repository Consolidation

Status:

```text
NOT STARTED
```

Recommended order:

```text
Vapor-Entrypoint
→ Vapor-Installer
→ Vapor-Launcher
→ Vapor-SDK
→ Vapor-Shell
```

Each repository is migrated independently.

For every repository:

```text
inventory behavior/design
→ classify valid vs obsolete semantics
→ migrate/reimplement useful pieces into Vapor
→ clean/document against current architecture
→ test replacement
→ remove submodule from Vapor-Client
→ archive historical repository
```

Do not preserve an obsolete repository boundary merely to preserve Git history.

The archived repository already preserves its history.

---

# Wave 3A — Vapor-Entrypoint

Target:

```text
CONSOLIDATE → Vapor
```

Desired end state:

```text
Entrypoint remains a tiny executable/platform adapter
Core owns shared semantics
```

Completion:

* [ ] Useful Entrypoint behavior exists in `Vapor`.
* [ ] Target binary builds.
* [ ] Steam/client startup path still works.
* [ ] `Vapor-Entrypoint` submodule removed from `Vapor-Client`.
* [ ] Old repository archived.

---

# Wave 3B — Vapor-Installer

Target:

```text
CONSOLIDATE → Vapor
```

Product boundary remains:

```text
Vapor Installer
```

Source boundary does not need to.

Completion:

* [ ] Installer capability semantics use shared Vapor Core.
* [ ] Dedicated `vapor-installer` binary remains available.
* [ ] Install/uninstall/Role provisioning behavior works.
* [ ] Global Vapor command exposure integration remains repairable/removable.
* [ ] Old repository archived only after replacement proof.

---

# Wave 3C — Vapor-Launcher

Target:

```text
CONSOLIDATE → Vapor
```

The standalone Launcher Core must not remain a competing semantic authority.

Completion:

* [ ] Launcher semantics are provided through shared Core.
* [ ] Launcher Mode exists or has a clear replacement path inside the desktop application architecture.
* [ ] Old Launcher-specific reusable UI work has been preserved where useful.
* [ ] Old repository archived after replacement proof.

---

# Wave 3D — Vapor-SDK

Target:

```text
CONSOLIDATE → Vapor
```

Special preservation requirement:

> The Figma-derived GUI is a visual design ancestor and must not be lost.

Inventory explicitly:

```text
vapor_sdk_gui
vapor_sdk_core
vapor_sdk_cli
```

Classify:

```text
visual/frontend implementation
    preserve/adapt

SDK-specific duplicate business semantics
    migrate only if still valid

obsolete assumptions
    archive/document rather than reproduce
```

Completion:

* [ ] Visual source/prototype safely preserved.
* [ ] Useful Tauri/web integration preserved or intentionally superseded.
* [ ] No second SDK Core competes with Vapor Core.
* [ ] Launcher↔SDK transformation architecture has one semantic backend.
* [ ] Old repository archived only after the useful material is migrated.

---

# Wave 3E — Vapor-Shell

Target:

```text
REINTERPRET / CONSOLIDATE → Vapor
```

The historical Shell currently represents an earlier universal command architecture.

It must not compete with the current `vapor` CLI.

Preserve useful:

```text
interactive shell UX
session ideas
terminal integration
workflow lessons
documentation
```

Future model:

```text
Vapor Shell
=
interactive frontend
+
Development Session
+
Vapor Core operations
```

Completion:

* [ ] No competing canonical CLI implementation remains.
* [ ] Useful interactive/session concepts preserved.
* [ ] Old repository archived.

---

# Wave 4 — Registry Authority Resolution

Status:

```text
NOT STARTED
```

Conflict:

```text
Vapor-Registry
vs
Vapor-Registry-Server
```

Current architecture must choose one authoritative ownership model.

Before changing either repository, define ownership for:

```text
Vapor semantic identities
provider/resource linkage
official namespace policy
Steam product metadata
publishing grants
bootstrap data
runtime Registry state
```

There must not remain two independently canonical Registries.

---

# Wave 5 — Documentation Source Extraction

Status:

```text
NOT STARTED
```

After the topology stabilizes, reconsider:

```text
Premium Docs
→ Vapor-Documentation
```

`Vapor-Documentation` would own authored ecosystem documentation.

`Vapor-Docs-Server` would remain the Platform service which publishes/serves documentation.

Do not perform this migration while the Premium Docs are still actively serving as the repository-migration authority.

---

# Wave 6 — Source Cleanroom / Docsification

Status:

```text
NOT STARTED
```

This is a major architecture-hardening task.

The rule is not:

```text
add comments to existing files
```

The rule is:

```text
understand responsibility
→ compare with normative architecture
→ rename/refactor/restructure
→ document useful contracts/context
→ strengthen invariants/tests
→ remove dead archaeology
→ replace whole file when appropriate
```

Full-file replacement is preferred for substantial refactors, including large files.

Documentation should explain:

```text
conceptual role
ownership
non-ownership
invariants
context
usage
failure semantics
relationships to other subsystems
```

It should not repeat facts already obvious from good names/types.

---

# Wave 6 — Documentation Layers

The cleanroom pass should progressively support:

```text
good code structure
    ↓
crate/module rustdoc
    ↓
public API rustdoc
    ↓
normative model docs
    ↓
generated reference
    ↓
Vapor Books
```

Books are not authored against knowingly unstable architecture merely to create volume.

They should increasingly teach architecture that has survived implementation pressure.

---

# Wave 7 — Return To Runtime Roadmap

After the stabilization/migration work reaches the agreed stopping point:

```text
finish remaining Phase B adversarial Cargo proof
    ↓
Phase C — Minimal Bevy ECS Engine
    ↓
Wheel Game
    ↓
Engine/Game/Mod capability proof
    ↓
static realization pressure
```

This migration must ultimately reduce architectural friction rather than becoming an infinite prerequisite to runtime development.

---

# Current Live Status

Update this block as work proceeds.

```text
Wave 0 — Baseline And Freeze
    ACTIVE

Wave 1 — Provider / Container Repo Rename
    BLOCKED BY WAVE 0

Wave 2 — Machine Vocabulary And Schema Migration
    BLOCKED BY WAVE 1

Wave 3 — Client Repository Consolidation
    BLOCKED BY WAVE 2

Wave 4 — Registry Authority Resolution
    NOT STARTED

Wave 5 — Documentation Source Extraction
    NOT STARTED

Wave 6 — Source Cleanroom / Docsification
    NOT STARTED

Wave 7 — Return To Runtime Roadmap
    NOT STARTED
```

---

# Migration Completion Principle

The migration is not successful merely because everything has the new name.

It is successful when:

```text
names match responsibilities
+
repo boundaries match semantic/deployment boundaries
+
Core owns shared semantics
+
source remains safe
+
development state can be understood/recovered
+
documentation explains the resulting architecture
+
the known-good Vapor workflows still work
+
future contributors/agents no longer need archaeology to understand it
```
