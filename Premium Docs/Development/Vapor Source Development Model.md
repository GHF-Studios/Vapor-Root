> [!info]
> **Owns:** source creation/acquisition, Git/provider interaction, Container/Repo/Project authoring workflow, source safety, submodule topology, local source recognition, and authored-source preservation.
>
> **Uses:** Source Identity, Role/Auth, Operations.
>
> **Does not own:** canonical identity definitions, Cargo/toolchain realization, CLI grammar, publication, or repository-migration waves.

---

# Core Thesis

Vapor development should let the user express the thing they intend to develop while preserving the reality of Git/provider source.

> **Vapor owns the semantic source-development workflow; Git remains the source-history system.**

---

# Development Topology

Development operates on:

```text
Authority
└── Source Repo Container
    └── Source Repo / Vapor Workspace
        └── Project
```

inside the one canonical Superworkspace.

Ordinary create/acquire operations should not ask for arbitrary destination paths when Vapor already knows the canonical location.

---

# Explicit Topology Creation

Vapor should not silently invent missing hierarchy while creating a deeper object.

Normal sequence:

```text
create Source Repo Container
→ create Source Repo
→ create typed Project
```

Each operation creates exactly the semantic layer it names.

---

# Creating a Container

Conceptually:

```text
validate Authority
→ establish Container identity
→ create provider-backed Git repo where requested/authorized
→ initialize Container metadata
→ place checkout in canonical Superworkspace
→ register/provider-link
```

A Container may legitimately begin with zero Source Repos.

---

# Creating a Source Repo

Conceptually:

```text
verify parent Container
→ create provider-backed Git repo
→ initialize Workspace metadata
→ register Source Repo topology
→ add authored Git submodule relationship
→ update parent Container source state
```

The submodule relationship is real Git state and must remain inspectable.

---

# Creating a Project

Typed Project creation occurs inside an existing Source Repo.

Creation supplies kind because the object does not yet exist.

It may generate:

```text
Vapor metadata
Cargo structure
source layout
tests
starter configuration
```

according to kind.

It must not silently create missing parent layers.

---

# Missing Parent Diagnostics

Vapor should identify the exact missing layer and distinguish:

```text
does not exist
exists remotely but not locally
exists locally but is unhealthy
```

The corresponding safe action may be create, acquire, or diagnose/repair.

---

# Source Acquisition

Vapor acquisition is Registry-aware.

Normal flow:

```text
Vapor source identity
→ Registry
→ provider linkage
→ Git acquisition
→ authored topology initialization
→ Vapor validation
→ canonical Superworkspace availability
```

This is not merely a synonym for `git clone`.

---

# Acquisition Atomicity

Default guarantee:

> **A Source Repo Container is the smallest independently acquirable unit unless a narrower Source Repo is explicitly modeled as independently realizable.**

This is a Vapor coherence guarantee, not a technical limitation of Git.

---

# Independent Source Repo Acquisition

Independent acquisition is opt-in modeled behavior.

Do not infer it merely because a provider offers a clone URL.

---

# Arbitrary Git Repositories

Vapor acquisition does not replace general Git.

Unregistered arbitrary source remains ordinary:

```text
git clone ...
```

If manually acquired source corresponds to valid registered Vapor topology at its canonical location, Vapor should recognize it.

Vapor need not be the only actor capable of Git operations.

---

# Submodule Topology

Source Repos normally participate beneath Containers through authored Git submodules.

Distinguish:

```text
declared Source Repo identity/topology
```

from:

```text
current submodule checkout state
```

A missing checkout is not an unknown Source Repo.

A detached submodule HEAD is not automatically broken.

---

# Git Transparency

Advanced users should be able to inspect:

```text
git status
git log
git branch
git remote -v
.gitmodules
git submodule status
```

and understand what Vapor did.

Vapor must not maintain a hidden parallel source-history system.

---

# Dirty / Unique Local Source

States such as:

```text
modified
committed locally
unpushed
detached
submodule differs from gitlink
```

may be legitimate development state.

Before destructive/topology-changing operations, preserve:

```text
uncommitted changes
unpushed commits
local branches
detached-but-unique commits
submodule state
source provenance
```

Remote existence alone does not prove local work is recoverable.

---

# Reconciliation

Source-topology reconciliation may diagnose:

```text
declared checkout missing
.gitmodules disagreement
Registry disagreement
unexpected submodule URL
```

Safe repair is allowed only when intended state is unambiguous and authored work is preserved.

---

# Development Loops

Typical Content loop:

```text
acquire/create source
→ create/open Project
→ edit
→ resolve dependencies
→ build/test in suitable context
→ inspect
→ revise
→ optionally publish
```

First-party Vapor development follows the same source-safety principles.

Role/Auth defines who is locally equipped for each workflow; this document only projects those Roles into source work.

---

# Removal and Recovery

Removing local source is a source lifecycle action.

It is not:

```text
Close Context
remove built App
downgrade Role
repair derived state
```

Remotely recoverable source may be reacquired.

Unique local authored work which was never pushed/backed up cannot be reconstructed and must never be treated as disposable.
