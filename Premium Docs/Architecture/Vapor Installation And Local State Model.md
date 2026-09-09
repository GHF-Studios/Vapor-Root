> [!info]
> **Owns:** local storage/lifetime domains, Steam App Instance, Vapor User Data, Superworkspace lifetime relationship, authored/derived/replaceable state, reinstall/downgrade/repair/recovery safety, and machine disposability.
>
> **Uses:** Product Topology, Role/Auth, Source Identity.
>
> **Does not own:** source identity, acquisition, Git workflow, Cargo implementation, or Steam publication transport.

---

# Core Thesis

The local Vapor environment contains state with different owners and lifetimes.

It must not be treated as one disposable directory.

Primary local domains:

```text
Steam App Instance
Vapor User Data
canonical Superworkspace
```

---

# Steam App Instance

A **Steam App Instance** is one concrete local installation of the Steam App.

Ordinary model:

```text
one Steam installation of Loo Cast
=
one Steam App Instance
```

Its root is primarily Steam/depot-owned and replaceable.

Examples:

```text
Vapor binaries
bootstrap files
Installer executable
shipped resources
default depot-delivered Vapor App
```

Steam may replace/remove these during update, verify, uninstall, or reinstall.

Unique authored source must not live here merely for convenience.

---

# Vapor User Data

**Vapor User Data** is mutable Vapor-owned user/environment state outside the replaceable Steam depot where appropriate.

It may include:

```text
installed Role state
configured Superworkspace location
managed toolchain metadata/state
persistent Context
resume/frontend state
local Vapor App metadata
selected/default App
indexes/caches
generated realization
IDE integration
diagnostic state
provider/account integration
```

Exact OS paths are implementation-specific.

---

# Canonical Superworkspace

The **Superworkspace** is the single canonical local development source root.

From a lifetime perspective it:

```text
contains authored Git/source state
is not Steam depot state
is not ordinary disposable cache
may survive reinstall and Role downgrade
may be relocated explicitly
```

Its identity semantics belong to the Source Identity model.

---

# Ownership Classes

## Authored State

Created or deliberately edited as user/project intent.

Examples:

```text
source code
Vapor/Cargo manifests
tests/scripts/assets
explicit operation recipes
Git commits/branches/remotes
local uncommitted changes
explicit configuration
```

Requires strong preservation.

## Derived State

Reconstructable from authored/modelled truth.

Examples:

```text
indexes
caches
generated glue
generated Cargo realization
IDE projections
diagnostic indexes
build artifacts
```

Intended invariant:

```text
deleteable
→ rediscoverable
→ regeneratable
→ idempotently repairable
```

If state cannot satisfy that standard, do not casually label it derived.

## Replaceable Product State

Delivered by Steam/other product distribution.

Examples:

```text
Client binaries
Installer binary
bootstrap
default shipped App artifacts
```

---

# Machine Disposability

A development machine should be close to disposable.

Desired recovery chain:

```text
fresh/replaced machine
→ install/acquire Vapor
→ restore local Role/tooling
→ acquire registered recoverable source
→ regenerate derived state
→ build/test
→ resume development
```

Exception: unique local authored state.

> **Only state actually recoverable may be treated as disposable.**

Unpushed commits, dirty files, and unique local branches must never be assumed recoverable merely because a remote repository exists.

---

# Steam Reinstall

Reinstall may recreate:

```text
product binaries
depot state
bootstrap
default shipped artifacts
```

It must not automatically delete:

```text
canonical Superworkspace
authored source
unpushed commits
persistent User Data intended to survive
```

After reinstall, Vapor should reconnect to persistent state and regenerate derived state where needed.

---

# Role Promotion / Downgrade

Promotion may provision additional tools such as Git, managed Rust/Cargo, rust-analyzer, SteamCMD, and development surfaces.

Downgrade may remove/disable higher-capability tooling.

Downgrade must not mean:

```text
delete source
delete Projects
delete local commits
```

---

# Diagnose vs Repair

**Diagnose** observes/explains state.

**Repair** reconstructs/reconciles safely derivable Vapor-owned state.

Repair must not silently:

```text
reset Git
discard dirty work
drop commits
reclone over authored source
replace authored manifests
```

Source reacquisition is a separate source-development operation.

---

# Local Vapor Apps

Multiple built Apps may coexist locally.

Their lifecycle is distinct from source.

Therefore:

```text
remove App
≠ delete source

delete source
≠ invalidate existing installed App automatically
```

---

# Selected / Default App

The environment may remember one selected/default App for Play convenience.

This is unrelated to:

```text
persistent Vapor Context
Operation Selection
GUI row selection
```

---

# Recovery Principle

A healthy Vapor environment should increasingly be reconstructable from:

```text
provider-delivered product state
+
persistent user configuration
+
registered provider/source truth
+
authored Git state
```

rather than undocumented machine-specific setup.
