> [!info]
> **Owns:** built complete Vapor App distribution: exact Packagepack composition provenance, target artifacts, Steam/Workshop/SteamPipe, depots/branches, Player acquisition/install, default depot-shipped App, and distribution history.
>
> **Uses:** Publication, Content/Composition, Product Topology, Installation.
>
> **Does not own:** source publication identity, Cargo internals, or Engine/Game runtime APIs.

---

# Core Thesis

Vapor has two distinct externalization concerns:

```text
source/version publication
built Vapor App distribution
```

This document owns the second.

Players consume complete built Vapor Apps, not arbitrary source Projects.

---

# Distribution Root

Only a valid complete Packagepack produces a Player-facing App.

```text
Packagepack version
→ exact Vapor App Composition
→ target build
→ Vapor App artifact
→ distribution/install
```

---

# Exact Composition Provenance

Distributed Apps must link to the exact composition used to build them:

```text
Packagepack ID/version
resolved Engine
resolved Game
resolved Mods
resolved Libraries
subordinate packs
all transitive Content versions
```

Machine serialization is implementation-defined; provenance is mandatory.

---

# Vapor App Artifact

A **Vapor App** is a built/deployable/runnable realization of a Vapor App Composition for a supported target.

Different target builds may realize the same logical composition.

---

# Player Distribution Unit

The normal Player-facing unit is a complete Vapor App.

Individual Engine/Game/Mod/Library/pack fragments may be source-published but are not independently runnable Player products merely because they exist.

---

# Default Loo Cast App

The default first-party Loo Cast App should ship directly in the Steam depot.

First launch must not require:

```text
Workshop
Rust
Cargo
source acquisition
local compilation
```

---

# Steam / SteamPipe

Official Steam deployment may use SteamCMD/SteamPipe.

Vapor owns the semantic deployment operation while preserving Steam's real concepts:

```text
App ID
Depot ID
branch
build
preview
account
staging/manifest
```

Steam remains the provider.

---

# Depots / Branches

Current deployment may use common/Linux/Windows depots and a development branch.

Exact provider IDs belong to configuration/operational state rather than timeless architecture.

Steam branch must not be conflated with Git branch, Content version, or Vapor Context.

---

# Workshop

Workshop may distribute additional complete Packagepacks/Apps.

Provider linkage should preserve stable Packagepack identity across version history rather than treating every version as unrelated identity.

---

# Built Distribution Record

For each distributed target, Vapor should be able to represent:

```text
Packagepack ID/version
exact resolved composition
target
build provenance
artifact identity/checksum where appropriate
provider linkage
provider revision/build
availability state
distribution timestamp/state
Yank/Ban implications
```

---

# Player Acquisition

Normal experience:

```text
find App
→ acquire
→ install
→ select
→ launch
```

Provider-native details remain available for diagnostics/advanced use.

---

# Local App Lifecycle

Changing selected/default App must not change development Context/source/Git.

Removing a built App must not delete source.

Source removal does not automatically invalidate an already-installed App.

---

# Offline Behavior

A locally installed App should remain runnable offline where its authored runtime does not inherently require online services.

The Vapor Platform must not become an accidental universal runtime dependency.

---

# Updates / Failure Preservation

A failed new update/build must not automatically destroy a previous working App.

Vapor should be able to reason about:

```text
installed version
available version
previous known-good artifact
selected/default version
rollback/history where supported
```

---

# First-Party Client Deployment

Deploying the Vapor Client itself through Steam may share SteamPipe primitives with App distribution while remaining a distinct semantic facility operation.

Do not force both into one generic "publish everything" abstraction merely because the provider is Steam.
