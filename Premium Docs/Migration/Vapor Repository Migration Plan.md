> [!info]
> **Owns:** controlled transition from historical/current first-party repository/provider layout to the target Client/Platform source topology.
>
> **Uses:** Product Topology and Source Identity as stable target architecture.
>
> **Does not own:** generic identity semantics, product semantics, or ordinary source-development rules.

---

# Migration Thesis

The current first-party repository layout reflects several generations of architecture.

Some boundaries remain meaningful. Others exist because older designs assigned independent semantic ownership to executables/frontends such as:

```text
Launcher
SDK
Installer
Shell
Entrypoint
```

Target principle:

> **Vapor Core owns shared semantics; source repositories reflect meaningful source/deployment ownership rather than executable count.**

---

# Stable Target Facilities

Stable semantic facilities:

```text
Vapor Client
Vapor Platform Server
Examples
```

Migration exists to make concrete source/provider topology better reflect those responsibilities.

---

# Target Containers

Target canonical Source Repo Container identities:

```text
GHF-Studios/Vapor-Client
GHF-Studios/Vapor-Platform-Server
```

Future reserved concept:

```text
GHF-Studios/Vapor-App-Server
```

Do not create the future App Server Container merely to complete topology.

---

# Historical Current Containers

Current/historical top-level repositories/identities include:

```text
GHF-Studios/Vapor-Root
GHF-Studios/Vapor-Server-Root
```

These names are migration facts, not stable product terminology.

---

# Three Independent Changes

Migration must distinguish:

```text
provider repository name/linkage
canonical Vapor identity
local realization path
```

Example:

```text
provider:
    github.com/GHF-Studios/Vapor-Root
    → github.com/GHF-Studios/Vapor-Client

canonical identity:
    GHF-Studios/Vapor-Root
    → GHF-Studios/Vapor-Client

local path:
    <legacy>/Vapor-Root
    → <Superworkspace>/GHF-Studios/Vapor-Client
```

These are related but not the same operation.

---

# Provider Rename Is Not Identity Migration

Valid intermediate state:

```text
canonical identity:
    GHF-Studios/Vapor-Root

provider:
    github.com/GHF-Studios/Vapor-Client
```

because:

```text
Vapor identity
≠ provider repository identity
```

Do not globally replace matching strings and collapse these meanings.

---

# Canonical Identity Migration

Changing canonical Container identity changes descendant identity prefixes.

Example:

```text
GHF-Studios/Vapor-Root/Vapor
→
GHF-Studios/Vapor-Client/Vapor
```

and likewise Projects below.

This is explicit identity migration, not a side effect of provider rename or filesystem movement.

---

# Historical Identity Mapping

Maintain exact mappings:

```text
GHF-Studios/Vapor-Root
→ GHF-Studios/Vapor-Client

GHF-Studios/Vapor-Server-Root
→ GHF-Studios/Vapor-Platform-Server
```

Descendant mappings may be derived by exact prefix replacement when topology is otherwise unchanged.

Old IDs may remain recognizable as migrated history without becoming permanent alternate canonical identities.

---

# Client Source Family

Historical membership has included:

```text
Vapor
Vapor-SDK
Vapor-Launcher
Vapor-Shell
Vapor-Examples
Vapor-Installer
Vapor-Entrypoint
```

Target dispositions remain approximately:

```text
Vapor
    KEEP / EXPAND

Vapor-Examples
    KEEP

Vapor-SDK
Vapor-Launcher
Vapor-Installer
Vapor-Entrypoint
    CONSOLIDATE useful implementation → Vapor
    then ARCHIVE historical repos

Vapor-Shell
    REINTERPRET useful interactive concepts
    CONSOLIDATE → Vapor
    then ARCHIVE
```

Actual membership must be verified from Git/.gitmodules before execution.

---

# Platform Source Family

Historical Platform services include:

```text
Vapor-Homepage-Server
Vapor-Docs-Server
Vapor-Identity-Server
Vapor-Diagnostics-Server
Vapor-Registry-Server
```

These remain meaningful independent Source Repo boundaries unless implementation evidence says otherwise.

Do not force Client/Platform symmetry.

---

# Migration Safety

Preserve:

```text
Git history
dirty work
unpushed commits
branches
submodule state
source provenance
known-good self-hosting
managed toolchain behavior
Steam deployment knowledge
Platform deployment knowledge
Registry/provider mappings
migration history
recoverability between waves
```

Topology cleanup never justifies destroying unique authored state.

---

# Recommended Waves

## Wave A — Documentation / Semantic Ownership

Establish coherent focused docs and migration vocabulary.

No provider rename.

## Wave B — Baseline Inventory

Record:

```text
Git status/branches/remotes
submodules
provider URLs
Registry schema/linkage
Steam deployment config
Platform deployment config
Superworkspace/path assumptions
known-good Client/Platform operations
```

## Wave C — Provider Repository Renames

Rename only:

```text
Vapor-Root → Vapor-Client
Vapor-Server-Root → Vapor-Platform-Server
```

and reconcile provider URLs/remotes/linkage.

Canonical Vapor identities and local paths intentionally remain old.

## Wave D — Registry/Core Identity-Model Readiness

Ensure target hierarchy and migration mappings can be represented.

## Wave E — Canonical Vapor Identity Migration

Migrate Container identity prefixes and exact descendants.

## Wave F — Canonical Local Path / Superworkspace Reconciliation

Move/recognize source under the target canonical local realization.

## Wave G — Client Source Repo Consolidation

Consolidate obsolete Client boundaries only after replacement behavior is proven.

## Wave H — Cleanup

Remove compatibility aliases/scaffolding when no longer required.

---

# Current Documentation Pass Non-Goals

This documentation restructure does **not**:

```text
rename GitHub repositories
change Git remotes
change canonical Vapor IDs
move local checkouts
consolidate Source Repos
rewrite Registry production data
change Steam deployment targets
```

---

# Rename Blast Radius

Future rename work must search beyond docs/repo titles:

```text
provider repository URLs
Git remotes
.gitmodules

Registry seeds/records/schema
canonical Vapor IDs
migration mappings

Rust constants/string comparisons
repository detection
first-party facility bindings

shell/bootstrap/deployment scripts
SteamCMD/service configuration
CI / GitHub Actions

Workspace/Container/App manifests
generated templates/configuration

Vapor User Data
Context/resume state
source indexes
IDE paths

tests/fixtures/snapshots
diagnostic strings
documentation
```

Each match must be classified by meaning before editing.

---

# Provisional Meaning Classes

Useful but adaptable tags:

```text
PROVIDER
VAPOR_IDENTITY
LOCAL_PATH
SEMANTIC_FACILITY
HISTORICAL
DISPLAY_ONLY
UNKNOWN
```

`UNKNOWN` blocks blind replacement.

---

# No Global Find-and-Replace

This is unsafe:

```text
replace every "Vapor-Root" with "Vapor-Client"
```

Some occurrences mean provider, some canonical identity, some path, some history, some facility detection.

Change the correct class of state in the correct wave.

---

# Completion Principle

> **Change one class of state, restore a known-good Vapor system, then continue.**
