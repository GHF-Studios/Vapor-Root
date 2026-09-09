> [!info]
> **Owns:** Operational Situation, State Dimensions, Conditions, Readiness Predicates, Operations, Variants, Recipes, transitions, invariants, diagnostics, repair, and preservation semantics.
>
> **Uses:** Role/Auth, Source Identity, Context/Selection, Content/Composition, Installation.
>
> **Does not own:** the full definition of those imported concepts or exact CLI/UI grammar.

---

# Core Thesis

Vapor does not have one giant linear lifecycle.

Its real state is a combination of independently meaningful dimensions.

```text
Operational Situation
=
relevant conditions across independent state dimensions
```

Operations should change the smallest meaningful set of dimensions and preserve unrelated valid state.

---

# Operational Situation

Example:

```text
Role:
    Content Developer

toolchain:
    ready

source:
    locally available

Git:
    dirty + local commits

Packagepack:
    resolved valid

latest build:
    failed

previous App:
    installed + runnable

Git provider:
    unauthenticated
```

This does not require one giant enum variant.

---

# State Dimension

A **State Dimension** is an independently meaningful aspect of operational state.

Examples:

```text
installed Role
toolchain readiness
Superworkspace configuration
source availability
source topology health
Git state
derived-state health
persistent Context
Packagepack resolution
build state
local App installation
selected/default App
runtime
authentication
authorization
provider availability
publication
distribution/deployment
```

The inventory grows only under real implementation pressure.

---

# Condition

A **Condition** is one fact within/across dimensions.

Examples:

```text
Content Developer Role installed
Source Repo locally available
working tree modified
Packagepack resolves
Linux artifact current
GitHub unavailable
published version Yanked
```

Conditions may be observed, derived, unknown, provider-specific, or operation-specific.

---

# Readiness Predicate

A **Readiness Predicate** asks a useful derived question over an Operational Situation.

Examples:

```text
Player-ready?
Editable?
Buildable?
Locally runnable?
Source-publishable?
Steam-deployable?
Repairable?
```

This term must not be called Context.

---

# Operation

An **Operation** is a modeled Vapor action against a semantic subject.

An Operation should define or resolve:

```text
subject
natural complete scope
optional Selection
variant / parameters
preconditions
state dimensions read
state dimensions changed
state which must be preserved
success result
failure result
diagnostics
```

---

# Operation Variant

Example:

```text
platform-server deploy local
platform-server deploy vps
```

Variants may differ in:

```text
authorization
supported Selection
preconditions
deployment topology
health checks
failure semantics
```

Operation legality cannot be reduced to the verb alone.

---

# Operation Recipe

An **Operation Recipe** is lightweight authored configuration describing how a modeled operation is realized.

It may describe:

```text
required topology
supported Selection scopes
steps / dependencies / parallelism
build commands
validation
artifact relationships
deployment actions
health checks
small explicit scripts/processes
```

Recipes are not an arbitrary plugin/application runtime.

---

# Transition

A **Transition** is a meaningful state change caused by an Operation.

Example:

```text
build:
    Stale
    ↓ success
    Current
```

A failed build may instead change:

```text
latest build attempt:
    Failed

diagnostics:
    Available
```

while preserving a previous valid App artifact.

---

# Invariants

Important invariants include:

```text
Player operation does not require Rust/Cargo.
Context does not silently narrow operation scope.
Authentication changes do not modify source.
Failed rebuild does not destroy previous working App by default.
Role downgrade does not delete authored source.
Repair does not silently reacquire/reset source.
Dirty Git state is not automatically invalid Vapor topology.
Published version meaning is immutable.
```

---

# State Projections

Imported domains may appear as state dimensions without being redefined here.

Examples:

```text
Role state
toolchain state
source availability
Git state
Packagepack resolution
publication state
```

Their semantic definitions live with their owners.

---

# Source Availability vs Topology Health

Keep distinct:

```text
Container absent
    availability concern

Container present, Source Repo checkout missing
    topology concern

Source Repo dirty
    Git state concern
```

Do not collapse them into one "source broken" flag.

---

# Git State Is Not Vapor Health

Real Git states include:

```text
clean
modified
local commits
unpushed
ahead/behind
detached HEAD
submodule differs from gitlink
merge conflict
```

Several may coexist.

Unusual Git state may need attention without invalidating Vapor topology.

---

# Derived-State Health

Derived state may be:

```text
healthy
missing
stale
inconsistent
repairable
regenerating
```

Derived-state failure does not automatically imply authored-source failure.

---

# Build State

Build state is relative to:

```text
subject
source/resolution inputs
target
configuration
toolchain
```

Possible conditions:

```text
missing
queued
building
current
stale
failed latest attempt
cancelled
artifact available
artifact validation failed
```

Stale means "not corresponding to current inputs", not necessarily unusable.

---

# Previous Artifact Preservation

A failed new build should normally preserve a previous known-good built/installed artifact.

This supports:

```text
source changed
→ new build failed
→ old App still runnable
```

unless the requested operation explicitly replaces/removes it.

---

# Diagnose

**Diagnose** observes modeled state and explains what it finds.

It may inspect source topology, Git, Cargo, toolchain, provider state, generated state, installation state, publication, and deployment.

Unknown must be distinguished from unhealthy.

---

# Repair

**Repair** reconstructs/reconciles safely derivable Vapor-owned state.

Repair must define exactly which dimensions it may mutate.

It must not silently:

```text
discard source edits
reset Git
drop commits
overwrite conflicting authored manifests
```

---

# Specific Diagnostics

Errors should identify the violated invariant.

Prefer:

```text
Platform Server VPS deployment requires the complete Platform Server scope.

selected:
    Registry Source Repo

note:
    Source Repo-scoped deployment is supported by `deploy local`.
```

over:

```text
invalid scope
```

---

# Preservation Rule

For each operation ask:

```text
What state does this operation own?
What state does it only read?
What survives success?
What survives failure?
```

This is the antidote to monolithic lifecycle transitions.
