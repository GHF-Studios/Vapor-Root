> [!info]
> Live execution ledger for `Vapor Repository Migration Plan.md`.
>
> The Plan owns migration semantics. This file records real evidence, state, gates, and discovered blast radius.
>
> An unchecked later-wave item is not permission to perform it opportunistically.

---

# Status Vocabulary

Use a small adaptable set such as:

```text
ACTIVE
READY
BLOCKED
COMPLETE
DEFERRED
REVIEW
```

The point is safe sequencing, not bureaucracy.

---

# Target Summary

Historical/current Containers:

```text
GHF-Studios/Vapor-Root
GHF-Studios/Vapor-Server-Root
```

Targets:

```text
GHF-Studios/Vapor-Client
GHF-Studios/Vapor-Platform-Server
```

Future reserved:

```text
GHF-Studios/Vapor-App-Server
```

---

# Wave A — Documentation Information Architecture

Status:

```text
ACTIVE
```

Goal:

> Establish clean normative ownership before provider/identity migration.

Checklist:

- [ ] `Premium Docs/README.md` established.
- [ ] Architecture owner docs established.
- [ ] Development owner docs established.
- [ ] Experience docs reduced to projections.
- [ ] Migration docs use current→target truth explicitly.
- [ ] Process docs remain historical/stateful.
- [ ] Glossary reduced to reference role.
- [ ] Every old semantic block has a destination.
- [ ] Cross-document references audited.
- [ ] Transitional pointer stubs removed or explicitly scheduled.

Completion gate:

```text
one normative owner per concept
+
new corpus browsable from README
+
migration documents point to new owners
```

---

# Wave B — Baseline Inventory

Status:

```text
BLOCKED BY WAVE A
```

## Client / Platform Containers

For both top-level Containers:

- [ ] `git status`
- [ ] HEAD recorded
- [ ] branches/remotes recorded
- [ ] recursive submodule status recorded
- [ ] dirty/unpushed work classified
- [ ] unique local work protected

Useful evidence:

```bash
git status
git rev-parse HEAD
git branch -vv
git remote -v
git submodule status --recursive
```

## Active Source Repos

For each:

```text
clean + pushed
committed but unpushed
dirty / uncommitted
intentional detached state
archaeological / replaceable
unknown → investigate
```

`unknown` blocks destructive/topology-changing migration.

---

# Known-Good Client Baseline

Record proof for currently relied-upon capabilities, for example:

```text
Steam App Instance discovery
→ managed toolchain
→ source/Project resolution
→ managed Cargo
→ Vapor build/test
→ local deployment
→ deployed Vapor execution
```

Record semantic capability separately from historical CLI spelling.

---

# Known-Good Platform Baseline

Record current truth for applicable:

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

Record exact provider URLs and paths used by deployment.

---

# Registry Baseline

Inventory:

```text
schema
registered first-party identities
provider linkage
seed/bootstrap data
production persistence path
API contracts
migration mechanism
backup/recovery
```

Do not destroy evidence of working recovery/acquisition behavior.

---

# Superworkspace / Path Baseline

Record:

```text
current source root assumptions
Client Container path
Platform Container path
direct-workspace assumptions
RustRover/IDE paths
Vapor User Data references
hardcoded script paths
```

---

# Wave C — Provider Repository Renames

Status:

```text
BLOCKED BY WAVE B
```

Perform only provider-level renames:

```text
GHF-Studios/Vapor-Root
→ GHF-Studios/Vapor-Client

GHF-Studios/Vapor-Server-Root
→ GHF-Studios/Vapor-Platform-Server
```

Non-goals:

```text
canonical Vapor identity migration
local checkout move
Authority-directory migration
child repo rename
Project rename
Source Repo consolidation
Registry hierarchy rewrite
broad manifest migration
broad CLI migration
```

---

# Provider Reference Reconciliation

After provider rename, update references whose meaning is provider location:

```text
Git remotes
.gitmodules provider URLs
GitHub Actions references
deployment clone/fetch URLs
provider API config
poll/update config
operational provider links
```

Do **not** edit canonical Vapor IDs merely because spelling matches.

---

# Provider-Rename Verification

Client:

- [ ] new parent remote fetches
- [ ] submodules intact
- [ ] Vapor source intact
- [ ] managed toolchain works
- [ ] managed Cargo works
- [ ] self-build/test works
- [ ] local deploy works
- [ ] deployed Vapor runs

Platform:

- [ ] new parent remote fetches
- [ ] service submodules intact
- [ ] Registry works
- [ ] Identity works
- [ ] Diagnostics works
- [ ] Docs/Homepage work where applicable
- [ ] deployment automation reconciled
- [ ] production data untouched

Completion condition:

```text
provider names new
+
canonical Vapor IDs intentionally still old
+
local paths intentionally still old
+
known-good behavior restored
```

---

# Wave D — Identity Model Readiness

Status:

```text
BLOCKED BY WAVE C
```

Required support:

- [ ] Authority independent from provider org.
- [ ] Container and Source Repo distinct.
- [ ] Source Repo = Vapor Workspace relationship represented.
- [ ] Projects represented under Source Repos.
- [ ] Content kind is Project metadata, not ID layer.
- [ ] trusted first-party facility state represented.
- [ ] old→new identity migration records supported.
- [ ] exact descendant prefix migration supported.

---

# Wave E — Canonical Identity Migration

Status:

```text
BLOCKED BY WAVE D
```

Required prefix migrations:

```text
GHF-Studios/Vapor-Root
→ GHF-Studios/Vapor-Client

GHF-Studios/Vapor-Server-Root
→ GHF-Studios/Vapor-Platform-Server
```

Checklist:

- [ ] migrate Container records
- [ ] migrate descendant Source Repos
- [ ] migrate descendant Projects
- [ ] preserve old→new records
- [ ] reconcile canonical IDs in manifests/config
- [ ] reconcile Context/resume state
- [ ] reconcile Registry references
- [ ] check published-ID implications
- [ ] re-prove facility resolution

---

# Wave F — Local Canonical Realization

Status:

```text
BLOCKED BY WAVE E
```

Target shape:

```text
<Superworkspace>/
└── GHF-Studios/
    ├── Vapor-Client/
    └── Vapor-Platform-Server/
```

Checklist:

- [ ] canonical Superworkspace known
- [ ] Authority directory introduced as required
- [ ] Git source moved/recognized without metadata loss
- [ ] User Data paths reconciled
- [ ] IDE paths reconciled
- [ ] scripts/config reconciled
- [ ] source discovery uses canonical hierarchy
- [ ] physical move not mistaken for identity definition

---

# Wave G — Client Source Consolidation

Status:

```text
DEFERRED
```

Potential dispositions:

```text
Vapor
    KEEP / EXPAND

Vapor-Examples
    KEEP

Vapor-SDK
Vapor-Launcher
Vapor-Installer
Vapor-Entrypoint
Vapor-Shell
    migrate useful responsibility
    prove replacement
    archive historical repo
```

Before archive:

- [ ] useful implementation inventoried
- [ ] relevant history preserved
- [ ] replacement proven
- [ ] active references removed
- [ ] ancestry documented

---

# Rename Blast-Radius Inventory

## PROVIDER

Search:

```text
GitHub URLs
provider repository names
Git remotes
.gitmodules
provider API config
```

These move during provider rename.

## VAPOR_IDENTITY

Search:

```text
Registry data
manifests
constants
tests/fixtures
Context state
source metadata
```

These do **not** move during provider-only rename.

## LOCAL_PATH

Search:

```text
absolute/relative historical root paths
parent-directory checks
IDE config
deployment staging
scripts
```

These move only during local realization/path migration.

## SEMANTIC_FACILITY

Search for code using repository names as proxies for:

```text
Client
Platform Server
Examples
```

These should generally move toward trusted facility binding rather than a new hardcoded repo-name comparison.

## HISTORICAL

Old names in Bootstrap/migration history/archaeology may be intentionally correct.

Do not "fix" history.

## DISPLAY_ONLY

Non-semantic labels may be updated when stable semantic terminology should be shown.

## UNKNOWN

Anything ambiguous is `UNKNOWN` and blocks blind replacement.

---

# Search Surfaces

Before rename waves search at least:

```text
Rust
TOML/YAML/JSON
shell scripts
PowerShell/batch if present
GitHub Actions
.gitmodules
service files
reverse proxy config
SteamCMD/VDF/config
deployment scripts
tests/snapshots
docs
generated templates
Registry seeds/migrations
IDE config
```

Also search indirect assumptions, not merely literal old names.

---

# Final Cleanup

Eventually:

- [ ] old provider names absent from active provider config
- [ ] old canonical IDs appear only in explicit migration/history
- [ ] old local path assumptions removed
- [ ] facility resolution independent of historical repo names
- [ ] obsolete Source Repos archived
- [ ] documentation pointer stubs removed
- [ ] ledger preserved as historical execution evidence
