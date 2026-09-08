> [!info]
> This document defines the cross-cutting operational state model of the Vapor ecosystem.
>
> Here, **ecosystem** means Vapor as a descriptive whole: its Client, Platform, source, Content, Apps, tooling, providers, publication systems, and related state.
>
> It does **not** define a generic modeled `Ecosystem` object with one universal lifecycle or public `vapor ecosystem ...` command family.
>
> Vapor operations belong to concrete semantic subjects such as:
>
> * Vapor Client;
> * Vapor Platform Server;
> * Source Repo Container;
> * Source Repo;
> * Project;
> * Packagepack / Vapor App;
> * Installation;
> * Toolchain;
> * publication;
> * derived-state maintenance.
>
> Canonical Context and Selection semantics are defined by the **Vapor Context, Identity, Session And Selection Model**.
>
> Exact CLI grammar is defined by the **Vapor CLI Model**.
>
> Source development is defined by the **Vapor Development Experience Model**.
>
> Publication is defined by the **Vapor Publishing And Distribution Model**.

---

# Purpose

Vapor contains many independently meaningful dimensions of state.

A naive lifecycle model would attempt to enumerate combinations such as:

```text id="86lca1"
Composer
+
toolchain installed
+
source dirty
+
Packagepack valid
+
build stale
+
old Vapor App installed
+
App selected
+
GitHub unauthenticated
+
Steam available
```

and then invent another named lifecycle state for every meaningful combination.

That does not scale.

Instead Vapor models:

```text id="dw6v3v"
Operational Situation
    =
    relevant conditions across independent state dimensions
```

Operations then read and modify only the dimensions they legitimately own.

This allows Vapor to represent combinations such as:

```text id="hlpyll"
source dirty

new build failed

previous Vapor App still installed

previous Vapor App still runnable

GitHub offline

Steam online
```

without inventing a special monolithic state name.

---

# Core Principle

> **Operations change the smallest meaningful set of state dimensions and preserve unrelated valid state.**

For example:

```text id="q0nyrk"
build fails
```

may change:

```text id="0hs3iy"
build-attempt result
diagnostics
```

without changing:

```text id="ctq7fm"
source
installed previous Vapor App
selected/default Vapor App
provider authentication
persistent Context
```

unless the requested operation explicitly owns one of those dimensions.

---

# Ecosystem Is Not an Operation Subject

The phrase:

```text id="ik73yl"
Vapor ecosystem
```

may describe the overall system.

It is not itself a useful universal operational object.

There is no coherent generic lifecycle:

```text id="psq40q"
ecosystem create
ecosystem acquire
ecosystem build
ecosystem test
ecosystem publish
ecosystem deploy
```

because those verbs refer to different semantic subjects.

Instead:

```text id="7yup1r"
source-repo-container acquire
    source topology

client build
    Vapor Client

platform-server deploy vps
    Platform Server

test <Project>
    existing Project

publish <Content Project>
    publication

installation repair
    derived/local Installation state
```

Operations should belong to the thing they actually operate on.

---

# Core Operational Concepts

## Operational Situation

An **Operational Situation** is the complete set of state relevant to a particular moment.

It does not need one serialized enum variant.

For example:

```text id="t6liyt"
Role:
    Content Developer

toolchain:
    ready

source:
    available

Git:
    dirty

Packagepack resolution:
    valid

newest build attempt:
    failed

previous Vapor App:
    installed + runnable

App selection:
    previous Vapor App selected

GitHub:
    unavailable
```

is one valid Operational Situation.

---

# State Dimension

A **State Dimension** is one independently meaningful aspect of Vapor's operational state.

Examples include:

```text id="zlydf1"
installed Role
toolchain readiness
source availability
Git state
derived-state health
Packagepack resolution
build state
Vapor App installation
Vapor App selection
runtime
authentication
authorization
provider availability
publication
deployment
```

Dimensions may interact.

They are separated so those interactions can be modeled explicitly.

---

# Condition

A **Condition** is one fact within or across state dimensions.

Examples:

```text id="bp2fzv"
Content Developer Role is installed

Source Repo Container is locally available

working tree contains uncommitted changes

Packagepack resolves successfully

Linux build is current

Vapor App version 1.4.2 is installed

GitHub authentication is unavailable

Registry is reachable

published Content version is Yanked
```

A Condition may be:

```text id="fqoe50"
observed
derived
temporarily unknown
provider-specific
operation-specific
```

---

# Readiness Predicate

A **Readiness Predicate** is a useful derived question over an Operational Situation.

Examples:

```text id="0g9vf3"
Player-Ready?
Editable?
Buildable?
Locally Runnable?
Source-Publishable?
Built-Publishable?
Deployable to VPS?
Repairable?
```

This term deliberately replaces the older operational use of **Context**.

In current Vapor terminology:

> **Context means identity/path-resolution context.**

It must not simultaneously mean:

> “a named combination of readiness conditions.”

---

# Operation

An **Operation** is a modeled Vapor action against a semantic subject.

An Operation should define or resolve:

```text id="uvjlvh"
subject

natural complete scope

optional explicit Selection

parameters / variant

preconditions

state dimensions read

state dimensions changed

state which must be preserved

success result

failure result

diagnostics
```

Examples:

```text id="zph4gr"
client test

platform-server deploy vps

source-repo-container acquire ...

test <Project>

publish <Content Project>

installation repair
```

---

# Operation Subject

An **Operation Subject** is the thing the operation is fundamentally about.

Examples:

```text id="73gvam"
Vapor Client
Vapor Platform Server
one Source Repo Container
one Source Repo
one Project
one Packagepack
one Vapor App
the local Installation
the managed toolchain
```

The subject determines the operation's natural complete scope.

---

# Operation Selection

**Operation Selection** explicitly narrows or specifies topology beneath an Operation Subject where the operation supports it.

Example:

```text id="l00bb8"
subject:
    Vapor Platform Server

operation:
    test

selection:
    Registry
```

This is distinct from both:

```text id="ezjmmo"
persistent Vapor Context
```

and:

```text id="qdwx9a"
selected/default Vapor App
```

All three use the intuitive idea of “selection/location,” but they belong to different domains and must not be conflated.

---

# Transition

A **Transition** is the meaningful state change caused by an Operation.

Example:

```text id="12xagg"
build state:
    Stale
    ↓ successful build
    Current
```

A failed operation may produce a different transition:

```text id="z854v2"
latest build attempt:
    Failed

diagnostics:
    Available
```

while preserving a previous installed valid Vapor App.

---

# Invariant

An **Invariant** is a relationship which must remain true across operational states and transitions.

Examples:

* Player operation does not require Rust/Cargo.
* Context does not silently narrow operation scope.
* Provider authentication changes do not silently modify source.
* A failed rebuild does not automatically destroy a previous working Vapor App.
* Role downgrade does not silently delete authored source.
* Repair does not silently reacquire authored source.
* Dirty Git source is not automatically invalid.
* A published version cannot later refer to different source.

---

# Operation Recipe

An **Operation Recipe** is lightweight authored configuration describing how an operation is realized.

It may define:

```text id="wmp1p6"
required topology
supported Selection scopes
steps
dependencies between steps
parallelism
build commands
validation
artifact relationships
deployment actions
health checks
small explicit script/process calls
```

Operation recipes are not arbitrary plugin applications.

Vapor Core remains responsible for the semantic operation framework and safe primitives.

---

# Operation Variant

An **Operation Variant** is a semantically distinct form of an operation.

For example:

```text id="a3yr6p"
platform-server deploy local

platform-server deploy vps
```

may have different:

```text id="7rl2mm"
preconditions
authorization requirements
supported Selection
deployment topology
validation
failure semantics
```

Operation legality therefore cannot be reduced to verb alone.

---

# State Dimension Inventory

The following dimensions form the current operational model.

The inventory may grow when real implementation pressure reveals missing state.

It should not grow merely to mirror incidental internal variables.

---

# Installed Role State

The installed Vapor Role is one of:

```text id="4yb0cv"
Player
Composer
Content Developer
Ecosystem Developer
```

Roles are cumulative capability levels.

Conceptually:

```text id="8807k9"
Player
⊂ Composer
⊂ Content Developer
⊂ Ecosystem Developer
```

Role determines what local workflows/tooling Vapor equips the environment to perform.

---

# Root Authority Is Not Role State

**Root Authority is not part of the Role ladder.**

It is external/trusted authority.

Therefore this is wrong:

```text id="htl03u"
Player
→ Composer
→ Content Developer
→ Ecosystem Developer
→ Root Authority
```

The correct model is:

```text id="twn5ek"
local Role:
    Ecosystem Developer

possibly independently:

authority:
    Root Authority
```

A developer can be fully equipped for local first-party development without possessing protected official authority.

---

# Toolchain State

Relevant conditions may include:

```text id="m4f1vl"
not required for current Role

required but absent

installing

present but unconfigured

ready

degraded

incompatible

repairable
```

Player operation must not require developer tooling.

Composer/Developer roles may require progressively richer tooling.

Root/Ecosystem development should use the Vapor-managed pinned/vendored toolchain where required by policy.

---

# Superworkspace Configuration State

The canonical Superworkspace may be:

```text id="md1buj"
configured + reachable

configured + missing/unreachable

not yet configured

migration/reconciliation required
```

There is only one canonical local Superworkspace.

This dimension concerns local environment configuration.

It is not a Vapor identity dimension.

---

# Source Availability State

For a registered Vapor source identity:

```text id="vcbrvc"
not locally available

locally available

partially unavailable because authored Git topology is incomplete

availability unknown

provider unavailable
```

The exact meaning depends on topology layer.

For example:

```text id="rbagfj"
Source Repo Container absent
    acquisition concern

Container present but declared Source Repo checkout missing
    Git reconciliation concern
```

These should not be collapsed.

---

# Source Topology Health

A locally present Container/Repo may have topology state such as:

```text id="9xg581"
healthy

declared Source Repo checkout missing

unexpected submodule state

manifest/topology disagreement

Registry disagreement

incompatible metadata

unknown / requires diagnosis
```

Topology health is separate from ordinary Git cleanliness.

---

# Git Modification State

Real Git state may include:

```text id="933ju3"
clean

modified / uncommitted

committed locally

unpushed commits

ahead / behind

detached HEAD

submodule differs from gitlink

merge conflict

other Git error
```

These are not a linear Vapor lifecycle.

Several may coexist.

For example:

```text id="4fx8r4"
detached HEAD
+
modified
+
unpushed
```

may be a legitimate development state.

---

# Git State Is Not Vapor Health

The following do not inherently mean Vapor is broken:

```text id="qnby4p"
dirty working tree
detached submodule HEAD
Source Repo ahead of parent gitlink
local unpushed branch
```

Vapor must distinguish:

```text id="w56pc4"
unusual or unsynchronized Git state
```

from:

```text id="46rpyv"
invalid Vapor topology
```

---

# Derived-State Health

Derived Vapor state may independently be:

```text id="h56u55"
healthy

missing

stale

inconsistent

repairable

currently regenerating
```

Examples include:

```text id="ulqjwc"
indexes
generated glue
IDE integration
operation realization
path caches
diagnostic indexes
generated Cargo/Vapor projections
```

Derived-state failure does not automatically imply authored source failure.

---

# Persistent Context State

Persistent Vapor **Context** is identity-resolution state.

It may be:

```text id="ggus9q"
unset

set to an exact Vapor path

set but target currently unavailable

requiring migration from an old canonical identity
```

Context affects resolution.

It does not define operation scope.

---

# Transient Context

An operation may supply an explicit transient Context override.

This is per-operation input rather than a broad ecosystem lifecycle.

It must not modify persistent Context unless explicitly requested.

---

# Context Does Not Change Operational Scope

Suppose:

```text id="bo7wkz"
persistent Context:
    GHF-Studios/Vapor-Platform-Server/Registry
```

Then:

```text id="n8vdlc"
platform-server deploy vps
```

still targets the natural complete Platform Server operation.

The Context must not secretly narrow deployment to Registry.

Only explicit Operation Selection may request that.

---

# Packagepack Resolution State

A Packagepack may be:

```text id="90xjz8"
unresolved

resolving

resolved valid

invalid authored composition

dependency conflict

required version unavailable

resolution failed because provider/Registry unavailable

resolved historically from frozen published graph
```

A valid complete resolution contains exactly:

```text id="a55jpn"
one effective Engine
one effective Game
```

plus the selected compatible/supporting Content.

---

# Resolution Constraint State vs Exact Resolution

For published/development composition it is useful to distinguish:

```text id="k2fxf3"
authored dependency constraints
```

from:

```text id="z6b95f"
exact resolved version graph
```

A published Packagepack's historical exact resolved graph does not change merely because newer dependency versions appear.

---

# Build State

Build state is scoped by meaningful build subject/target.

Possible conditions include:

```text id="8ut3me"
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

A build state should identify what it is relative to:

```text id="99b0dy"
subject
source/resolution inputs
target
build configuration
```

---

# Current vs Stale

A build is **Current** when it corresponds to the currently relevant semantic inputs.

A build becomes **Stale** when those inputs change.

For example:

```text id="5vt59e"
build = Current
    ↓ relevant source edit
build = Stale
```

Stale does not mean unusable.

It means:

> does not correspond to the current relevant source/composition state.

---

# Build Failure vs Existing Artifact

A new failed build attempt may coexist with an older valid artifact.

For example:

```text id="xyxx45"
source:
    modified

latest build:
    failed

previous Vapor App:
    installed

previous Vapor App:
    runnable
```

This is a normal operational situation.

---

# Vapor App Installation State

For a particular Vapor App identity/version/target:

```text id="t6byss"
not installed

acquiring

installing

installed

removing

installation degraded

installation invalid

repairable
```

Installation is independent of source availability and build currency.

---

# Vapor App Selection State

The Steam App Instance may have one selected/default Vapor App Composition for normal Play/direct-launch convenience.

Possible conditions:

```text id="goj17n"
no valid selection

selected installed Vapor App

selected App missing/unavailable

selection requires migration/reconciliation
```

This is **Vapor App Selection**.

It is not Operation Selection.

---

# Operation Selection Is Transient Intent

Operation Selection normally belongs to one invocation/action.

For example:

```text id="mhr48i"
platform-server test --select "Registry"
```

The selected Source Repo subtree is relevant to that operation.

It does not become the Steam App Instance's selected Vapor App.

It does not become persistent Context.

---

# Runtime State

For a Vapor App/runtime:

```text id="nbcul3"
stopped

starting

running

stopping

crashed

failed to start
```

Detailed Engine/Game internal runtime state belongs to the App architecture.

This dimension only describes Vapor-level runtime lifecycle.

---

# Authentication State

Authentication should be modeled per identity/provider.

Examples:

```text id="drjjzb"
Steam
Vapor Platform
GitHub
future Git providers
future signing infrastructure
```

Possible conditions:

```text id="f7t91q"
not authenticated

authenticated

credentials expired

authentication unavailable
```

Authentication answers:

> Who are you?

It does not itself answer:

> Are you allowed to do this?

---

# Authorization State

Authorization is target/action specific.

Examples:

```text id="5a1sbh"
may push this Git repository

may create repository under this Authority/provider

may publish this Content ID

may deploy Client to Steam

may deploy Platform Server to production VPS

may change first-party Registry trust
```

Authorization cannot be represented adequately as one global:

```text id="xmyra0"
authorized = true
```

---

# External Availability State

External systems may independently be:

```text id="1wkl5z"
available
degraded
unavailable
unknown
```

Examples:

```text id="736yk0"
Steam
Steam Workshop
Vapor Registry
GitHub
other Git provider
Vapor Platform services
production VPS
```

One outage should affect only operations which actually require that external system.

---

# Source Publication State

For one Content Project/version:

```text id="qc4ujc"
local/unpublished

version validation ready

source commit not remotely available

publication pending

published immutable version

publication partially complete

Yanked

Banned

publication reconciliation required
```

Exact publication semantics belong to the Publishing model.

---

# Built Publication State

For one Packagepack version/target:

```text id="7ls4gt"
no built artifact

build available

artifact validated

distribution pending

distributed

distribution linkage incomplete

distribution unavailable

Yanked/Banned by policy where applicable
```

Source publication and built distribution are separate dimensions.

---

# Deployment State

First-party facilities may have deployment state.

For example, Platform Server:

```text id="0jh2ll"
local:
    absent / deployed / degraded

vps:
    unknown / healthy / deploying / degraded / failed
```

Client Steam deployment may have a different state model.

Deployment state is facility/target-specific.

There is no universal ecosystem deployment state.

---

# Operation Progress State

Long-running operations may expose:

```text id="phlze5"
queued
preparing
running
waiting on external system
validating
succeeded
failed
cancelled
```

This is operation-instance state.

It should not be confused with the permanent state of the target itself.

For example:

```text id="jhs0mz"
build operation = failed
```

does not imply:

```text id="9v6csp"
previous installed App = invalid
```

---

# Important Readiness Predicates

These predicates are useful for UX and operation gating.

They are derived questions over state.

They are not identity Context.

---

# Player-Ready

A local Vapor environment is **Player-Ready** when:

```text id="d24jgf"
Player capability is healthy

and

required Vapor Client/runtime state exists

and

the selected/default Vapor App is locally runnable
```

Player-Ready does not require:

```text id="2zvn30"
Git
Rust
Cargo
GitHub authentication
canonical Superworkspace
developer source
```

---

# Source-Developable

A source target is **Source-Developable** when:

```text id="c52eao"
installed Role permits the intended authoring

required source is locally available

source is writable

required local tooling is ready

no unresolved topology condition blocks the requested operation
```

Dirty source may remain Source-Developable.

---

# Composable

A Packagepack source target is **Composable** when:

```text id="4eifem"
Composer-or-higher Role exists

Packagepack source is available

required source topology/tooling is usable

the user may edit the composition
```

Remote authentication is only necessary when the requested operation needs a remote system.

---

# Editable

A Content Project is **Editable** when:

```text id="ujlcg6"
installed Role permits authoring its kind

source is locally available

source is writable
```

Examples:

```text id="zrhezb"
Composer:
    Packagepack editable

Composer:
    Engine implementation not editable

Content Developer:
    Engine implementation editable
```

---

# Buildable

A subject is **Buildable** when its build operation's actual preconditions are satisfied.

For a Packagepack this may include:

```text id="yk740n"
appropriate Role

toolchain ready

required source available

composition resolution valid

required target supported

no blocking build invariant
```

Buildable does not mean:

```text id="23eu6p"
source clean
source published
GitHub authenticated
```

unless the specific build operation actually requires those conditions.

---

# Testable

A subject is **Testable** when its test operation can legally execute in the requested scope.

This may depend on:

```text id="1agcbc"
subject
Selection
required source
toolchain
test composition
operation recipe
external service dependencies
```

It is not one universal property of all Projects.

---

# Locally Runnable

A Vapor App is **Locally Runnable** when:

```text id="42spw3"
valid runnable artifact exists locally

App is appropriately installed/registered

runtime prerequisites are satisfied
```

Its source may simultaneously be:

```text id="wulhap"
clean
dirty
missing
newer
older
```

---

# Source-Publishable

A Content Project/version is **Source-Publishable** when:

```text id="yaivz5"
installed Role permits authoring

canonical Content ID exists

new SemVer is valid

source validation succeeds

exact source commit is available

provider linkage exists

required publication authorization exists

required external systems are reachable
```

Published-version immutability rules still apply.

---

# Built-Publishable

A Packagepack version/target is **Built-Publishable** when:

```text id="n9puua"
source version is valid/published according to policy

exact composition resolution is valid

required target build succeeds

artifact validates

distribution authority exists

distribution backend is available
```

This does not collapse source publication and built distribution into one state dimension.

---

# Locally Deployable

A first-party subject may be **Locally Deployable** when the specific local deployment operation's preconditions hold.

For example:

```text id="06lm95"
platform-server deploy local
```

may accept Source Repo-scoped Selection that production deployment does not.

Readiness is operation-specific.

---

# Production Deployable

A subject may be **Production Deployable** only when all production-specific requirements are satisfied.

For example:

```text id="ej00xi"
platform-server deploy vps
```

may require:

```text id="lmownc"
complete Platform Server scope

build/test validation

remote access

production authorization

deployment recipe readiness

required external systems available
```

This is not equivalent to local Buildable/Testable.

---

# Repairable

A state is **Repairable** when Vapor can safely reconstruct the affected state from known authoritative inputs.

Examples:

```text id="n08fcd"
generated indexes missing
IDE integration stale
derived operation state missing
generated glue stale
```

Not automatically Repairable:

```text id="jqs9uq"
deleted unique authored source
dirty Git conflict
unpushed commit loss
unknown branch intent
missing registered Container
```

Those belong to acquisition, Git recovery, or manual diagnosis as appropriate.

---

# Major Operations and State Effects

The following sections illustrate important cross-dimensional behavior.

They do not define one generic `Ecosystem` command lifecycle.

---

# Establish Role Capability

Examples:

```text id="o4fb3k"
Player → Composer

Composer → Content Developer

Content Developer → Ecosystem Developer
```

Primarily Installer-owned.

May change:

```text id="x2fq81"
installed Role
installed tooling
managed toolchain state
available Client surfaces
```

Must preserve:

```text id="unr54t"
authored source
user gameplay state
installed Vapor Apps where compatible
unrelated provider state
```

---

# Downgrade Role Capability

A downgrade may:

```text id="uvqwzi"
remove/disable higher-level tooling
remove higher-capability UI
change Role metadata
```

It must not silently:

```text id="qbj34d"
delete Superworkspace
delete Git source
drop commits
remove authored Projects
erase user saves
```

---

# Configure / Relocate Superworkspace

A deliberate Superworkspace configuration operation may change:

```text id="nr0cnb"
configured canonical source root
derived path indexes
IDE integration
```

It must not change canonical Vapor identities.

Relocation should preserve authored Git state.

---

# Acquire Source Repo Container

Reads:

```text id="fbl7lo"
canonical Container identity
Registry
provider linkage
local canonical Superworkspace state
existing local path/state
```

May change:

```text id="hifvqv"
source availability
Git checkout state
submodule initialization
local discovery/index state
```

Must not alter unrelated installed Vapor Apps.

---

# Acquire Source Repo

Independent Source Repo acquisition is permitted only where modeled.

It reads the same relevant identity/provider policy and must respect the parent Container/acquisition rules.

Technical Git clone capability is not enough.

---

# Create Source Repo Container

May change:

```text id="0z773z"
Registry identity/topology
provider repository state
local canonical source realization
Container metadata
```

It does not implicitly create:

```text id="um6mbt"
Source Repo
Project
Cargo package
```

---

# Create Source Repo

May change:

```text id="7pe55f"
Registry Source Repo topology
provider repository state
parent .gitmodules / gitlink
Workspace metadata
local source realization
```

This is both Vapor topology and Git-authored topology.

---

# Create Typed Project

Example:

```text id="265go3"
game create ...
```

May change:

```text id="mlf7cw"
Project topology
Project metadata
Project source
Cargo realization
```

It must not silently create missing parent Containers or Source Repos.

---

# Modify Source

May change:

```text id="qr7zyt"
Git modification state
Cargo graph
Packagepack resolution validity
build currency
diagnostics
```

It does not automatically:

```text id="na6mjf"
install a new Vapor App
remove old Vapor App
publish
change App selection
change Context
```

---

# Reconcile Git Topology

A Git reconciliation operation/possible procedure may change:

```text id="b87tuo"
submodule checkout state
Git working state
```

It is distinct from:

```text id="i67tui"
acquisition

Repair
```

Potentially state-changing Git actions require appropriate caution.

---

# Diagnose

Diagnosis reads state.

It may inspect:

```text id="97pdwo"
Vapor identity/topology
Registry linkage
provider state
Git
Cargo
toolchain
derived state
build state
publication
deployment
```

Diagnosis should not silently perform destructive corrections.

---

# Repair

Repair may change only safely derivable Vapor-owned state.

Examples:

```text id="2m4uxi"
indexes
generated glue
IDE integration
operation realization
derived caches
```

Repair must not silently:

```text id="cag7by"
clone missing Container
reset Git
restore authored source
change branch
discard local changes
```

---

# Resolve Packagepack

Reads:

```text id="lhj27w"
Packagepack source
dependency constraints
Registry/version information
compatibility rules
available/published versions
```

May change:

```text id="8th51v"
current resolution state
derived exact graph
diagnostics
```

Published historical Packagepack resolutions remain frozen.

---

# Build

Build semantics depend on the subject.

Examples:

```text id="0af461"
build Packagepack
build Project
client build
platform-server build
```

May change:

```text id="2eidc1"
build state
build artifacts
caches
generated realization
diagnostics
```

It must preserve unrelated valid state.

---

# Test

Test semantics also depend on the subject/recipe.

Possible state effects include:

```text id="vg87wn"
test-run state
diagnostics
generated test artifacts
temporary runtime state
```

Test should not mutate persistent source unless the operation explicitly owns generated derived files and does so deterministically.

---

# Install Vapor App

May operate on:

```text id="nch4qf"
depot-shipped artifact
Workshop-acquired artifact
local build
```

May change:

```text id="uwnu1o"
Vapor App installation state
local App metadata
```

It does not inherently change source.

---

# Select Default Vapor App

Changes:

```text id="jiqcbj"
Steam App Instance Vapor App selection
```

May need to clear/migrate selection if the selected App is removed.

Must not modify Git source or persistent development Context.

---

# Launch Vapor App

Requires Locally Runnable state.

Changes:

```text id="l08o22"
runtime state
runtime diagnostics/logs
runtime-generated user state as appropriate
```

Normal Player launch must not require:

```text id="6fgcu5"
source acquisition
Cargo
local compilation
Git
```

---

# Stop Vapor App

Changes runtime state.

Preserves:

```text id="u9dw28"
installation
source
build artifacts
selected/default App
```

except for explicitly persisted runtime-generated data.

---

# Remove Vapor App

Changes:

```text id="e009af"
App installation
possibly default App selection
local App metadata
```

Must not remove authored Project source merely because the built App is removed.

---

# Authenticate Provider

Changes provider-specific credentials/session state.

May enable previously blocked remote operations.

Must not silently change:

```text id="k63s4p"
source
build
installation
Context
App selection
```

---

# Authorize Protected Operation

Authorization may be checked before operations such as:

```text id="sq7c1h"
push official source
create provider repo under protected Authority
publish protected Content ID
deploy Client to Steam
deploy Platform Server VPS
modify first-party Registry trust
```

Authorization checks should not alter local Role.

---

# Publish Content Version

May change:

```text id="3ae0f7"
remote Git/provider state
immutable Registry version state
provider release state
publication metadata
```

A published version becomes bound to one exact commit.

Failure after partial external success must be recoverable without rewriting historical source.

---

# Publish Packagepack Vapor App

In addition to source publication, may change:

```text id="yh5o92"
resolved historical composition record
built target artifacts
Steam Workshop distribution
Registry built-publication linkage
```

Only complete Packagepack-derived Apps may enter Player-facing built distribution.

---

# Deploy Vapor Client

A Client deployment operation may have variants such as:

```text id="12rtdg"
local
steam
```

Its state effects belong to Client deployment.

It is not a generic ecosystem deployment.

---

# Deploy Platform Server

Platform Server deployment may include:

```text id="g995k3"
local

vps
```

The operation recipe defines:

```text id="02w28l"
required scope
services
artifacts
remote changes
health checks
verification
```

A production deployment may reject partial Selection even when local deployment supports it.

---

# Interaction Rules

The multidimensional model becomes useful through explicit interaction rules.

---

# Source Change → Build Currency

A relevant source change may produce:

```text id="cxowc6"
Build:
    Current
    ↓
    Stale
```

It does not automatically remove the old built artifact.

---

# Dependency Change → Resolution Currency

Changing Packagepack dependency constraints may invalidate a previous development resolution.

Conceptually:

```text id="n42l4x"
development resolution:
    current
    ↓ dependency edit
    stale / requires re-resolution
```

Historical published exact resolutions remain unchanged.

---

# Failed Build ≠ Destroyed Previous App

If a new build fails:

```text id="fy7ck2"
latest build attempt:
    Failed
```

while:

```text id="gqs716"
previous installed Vapor App:
    Installed + Runnable
```

may remain true.

This is a foundational preservation invariant.

---

# Authentication ≠ Local Capability

Examples:

```text id="pabkqk"
GitHub logout
    ≠ remove Content Developer Role

Registry outage
    ≠ make existing local source disappear

Workshop outage
    ≠ prevent local source editing

Steam Workshop outage
    ≠ invalidate previously installed Vapor App
```

Only operations actually requiring the provider should be blocked.

---

# Authorization ≠ Role

An Ecosystem Developer may be able to:

```text id="5s8ns3"
build Platform Server locally
test Client locally
```

while being denied:

```text id="nd8b9v"
production VPS deployment
official Steam deployment
Root-authority Registry operation
```

That is expected.

---

# Context ≠ Operation Selection

Persistent Context:

```text id="df8fva"
helps resolve names
```

Explicit Operation Selection:

```text id="q8bpgo"
changes requested operation scope
```

One must never silently substitute for the other.

---

# Operation Selection ≠ App Selection

Example:

```text id="55tkon"
platform-server test --select Registry
```

does not change which Vapor App the Launcher plays by default.

These are unrelated state domains.

---

# App Selection ≠ Source Mutation

Selecting another Vapor App must not:

```text id="6ynfud"
checkout a Git branch
modify Packagepack source
rewrite dependencies
change development Context
```

unless an explicitly separate user operation requests such behavior.

---

# Git Change ≠ App Selection

Changing:

```text id="ri7f8z"
branch
commit
working tree
```

does not implicitly switch the selected/default installed Vapor App.

A build/install/select operation must occur explicitly.

---

# Role Downgrade ≠ Source Destruction

Capability controls operations/tooling.

It does not determine whether user-authored source is disposable.

---

# Runtime ≠ Composition Assembly

Launching an App operates on a built installed App.

Normal runtime startup is not an implicit Composer/build pipeline.

---

# Missing Container ≠ Repair

If registered Container source is absent:

```text id="8w7z59"
acquire
```

is the relevant operation.

Do not classify the absence as generic derived-state Repair.

---

# Missing Submodule Checkout ≠ Acquisition

If an already-acquired Container declares a Source Repo submodule whose checkout is missing:

```text id="vbvmbw"
Git reconciliation
```

is the relevant domain.

Do not pretend it is independent source acquisition.

---

# Dirty Git ≠ Broken Vapor

Dirty/unsynchronized source should remain:

```text id="pwu4mn"
editable
often buildable
often testable
```

unless the specific operation has a real reason to require clean source.

For example, publication may impose stricter source-state requirements than local testing.

---

# Published Version ≠ Current Source

After publication:

```text id="9sv9to"
published version:
    immutable historical commit

current development source:
    may continue changing
```

The two states coexist naturally.

---

# Yank ≠ Delete

A Yank changes resolution/discovery policy.

It does not erase:

```text id="qqb8un"
version identity
source commit binding
historical dependency graph
```

---

# Deployment Failure ≠ Source Failure

A failed production deployment may leave:

```text id="8rt5zp"
source:
    healthy

build:
    healthy

local deployment:
    healthy

production deployment:
    failed/degraded
```

Different operational dimensions must remain separate.

---

# Failure Modeling

Failures should affect only the state genuinely touched by the failed operation.

For example, a build failure may produce:

```text id="87pwpp"
build attempt:
    failed

diagnostics:
    available
```

while preserving:

```text id="700nug"
source:
    dirty but intact

installed App:
    installed

App selection:
    selected

runtime:
    stopped

Context:
    unchanged
```

This principle applies to:

```text id="oi8l7b"
Git operations
downloads
source acquisition
build
publication
deployment
toolchain repair
Registry access
Workshop access
```

---

# Partial External Failure

Operations spanning external systems may enter partial states.

Examples:

```text id="wwxwkg"
Git push succeeded
Registry publication failed

Workshop upload succeeded
Registry linkage failed

VPS artifact transfer succeeded
service restart failed
```

Vapor should preserve enough operation/provenance state to:

```text id="z2xq7u"
diagnose
retry
reconcile
verify
```

without blindly repeating destructive steps.

---

# Operation Idempotence

Where practical, derived/reconciliation operations should be idempotent.

Examples:

```text id="6tjbfp"
repair indexes
regenerate IDE integration
reconcile generated operation realization
verify existing publication linkage
```

Repeated execution should converge on the same intended state.

Authored-state-changing operations may require stronger explicit confirmation/guards.

---

# Lifecycle Projections

A lifecycle is a projection over relevant dimensions.

It is not the complete state of Vapor.

---

# Steam App Instance Lifecycle

Conceptually:

```text id="zwu9e1"
Steam install
→ Player-ready Client
→ optional Role upgrades
→ ordinary operation
→ Steam update/verify/move
→ optional Role downgrade
→ uninstall/reinstall
```

This lifecycle concerns Steam/depot-owned product state.

It does not own the Superworkspace lifecycle.

---

# Vapor User Data Lifecycle

Conceptually:

```text id="8q37zr"
created/configured
→ evolves with local Vapor use
→ repaired/regenerated partly as needed
→ survives ordinary Steam replacement where appropriate
→ may be explicitly reset/removed
```

Individual User Data fields have different recoverability.

---

# Superworkspace Lifecycle

Conceptually:

```text id="0cf65g"
configure canonical Superworkspace
→ create/acquire source
→ develop
→ optionally relocate root
→ continue development
```

Ordinary Steam uninstall does not define the end of this lifecycle.

---

# Role Lifecycle

Conceptually:

```text id="38qwvb"
Player
↔ Composer
↔ Content Developer
↔ Ecosystem Developer
```

Tooling may be installed/removed accordingly.

Authored source persists independently.

Root Authority is not a state in this lifecycle.

---

# Source Repo Container Lifecycle

Conceptually:

```text id="k587db"
not registered
→ created/registered

registered but not local
→ acquired

local
→ developed / topology changed deliberately

possibly archived/deprecated according to policy
```

Creation, acquisition, Git development, and publication are distinct operations.

---

# Source Repo Lifecycle

Conceptually:

```text id="wth3ru"
created beneath Container
→ registered
→ authored submodule relationship
→ developed
→ possibly independently published/used according to contained Projects
```

Independent acquisition is optional policy, not a universal lifecycle step.

---

# Project Development Lifecycle

Conceptually:

```text id="xs82e3"
typed create
→ author
→ build/test
→ revise
→ optionally publish versions
```

A Project may remain permanently local/unpublished.

---

# Git Lifecycle

Git state does not form one clean linear Vapor chain.

Common transitions may include:

```text id="rwa335"
clean
→ modified
→ committed
→ pushed
```

but branches, rebases, detached HEADs, merges, conflicts, and local-only work make Git intrinsically richer.

Vapor should expose Git rather than invent a fake simplified replacement model.

---

# Packagepack Development Resolution Lifecycle

Conceptually:

```text id="egmr3f"
authored constraints
→ unresolved
→ resolved
→ source/dependency edit
→ stale/unresolved again
```

Published historical resolved graphs are frozen separately.

---

# Build Lifecycle

Conceptually:

```text id="r6sdoy"
Missing
→ Building
→ Current
→ Stale
→ Building
→ Current
```

with failures/cancellation represented as operation outcomes rather than alternate universes.

---

# Vapor App Installation Lifecycle

Conceptually:

```text id="j1lurf"
not installed
→ acquiring/installing
→ installed
→ optionally removed
```

Source may or may not exist locally throughout this lifecycle.

---

# Vapor App Selection Lifecycle

Conceptually:

```text id="d6hyck"
installed candidate Apps
→ choose selected/default App
→ change selection
→ reconcile if selected App removed/unavailable
```

This is Player/Launcher convenience state.

---

# Runtime Lifecycle

Conceptually:

```text id="ocj8ig"
Stopped
→ Starting
→ Running
→ Stopping
→ Stopped
```

Crash/failure conditions are represented where relevant.

---

# Content Publication Lifecycle

Conceptually:

```text id="pnubnx"
local development
→ validate new SemVer
→ publish immutable source version
→ future development continues
→ publish later version
```

Historical versions remain immutable.

---

# Packagepack Built Distribution Lifecycle

Conceptually:

```text id="nd0cv3"
published Packagepack version
→ exact composition resolution
→ target build
→ artifact validation
→ distribution
→ installed by Players
```

Partial publication states are explicitly recoverable.

---

# First-Party Deployment Lifecycle

There is no one universal deployment lifecycle.

Examples:

```text id="1t7vml"
Client Steam deployment lifecycle

Platform Server local deployment lifecycle

Platform Server VPS deployment lifecycle
```

Each is owned by its facility/recipe.

---

# Local State Ownership

Operational safety depends on knowing who owns which state.

---

# Steam-Managed State

Examples:

```text id="sehoze"
depot binaries
bootstrap files
default shipped Vapor App
Steam installation metadata
```

Steam is authoritative for depot replacement.

---

# Vapor User Data

Examples:

```text id="la3egi"
Role state
configured Superworkspace root
persistent Context
local indexes
managed toolchain state
IDE integration
App selection
local App metadata
derived operation state
```

Some User Data is durable preference/state.

Some is safely regeneratable.

Ownership must be known per field.

---

# Authored Superworkspace State

Examples:

```text id="oeef5l"
Source Repo Containers
Source Repos
Projects
source code
Vapor manifests
Cargo manifests
scripts
tests
Git commits
branches
uncommitted changes
```

This is authored development state.

Vapor must treat it conservatively.

---

# User Gameplay / Runtime State

Examples:

```text id="5ufcd0"
saves
settings
keybinds
App-generated persistent data
```

This is user-valued state and must have explicit ownership/lifetime rules.

It is not developer source.

---

# Build and Cache State

Examples:

```text id="9etdnc"
Cargo target outputs
download caches
incremental build data
generated intermediates
packaging temporary files
```

This is generally regeneratable.

---

# Installed Vapor Apps

Installed Apps are built runnable local products.

They are not canonical source.

They may often be reacquired/rebuilt.

Removal should still be explicit.

---

# Authentication State

Credentials/session state belongs to the relevant secure identity/provider mechanism.

It must not be mixed casually into authored Content or build outputs.

---

# Diagnostics State

Diagnostics may include:

```text id="7n88av"
current findings
operation logs
historical operation results
health checks
provider errors
build/test output
```

Some may be ephemeral.

Some may be useful durable User Data.

The exact retention model remains open.

---

# Operation Legality

An operation's legality may depend on:

```text id="5d4kcp"
subject
operation
variant
Selection
Role
authority
source topology
Git state
toolchain state
external availability
authored recipe/configuration
```

There is no need to force all of this into one generic:

```text id="1vtofx"
operational state enum
```

---

# Specific Legality Diagnostics

When an operation is rejected, Vapor should explain the actual invariant.

Example:

```text id="5jtgnb"
error: Platform Server VPS deployment requires the complete
       Platform Server deployment scope

selected:
    Source Repo `Registry`

help:
    vapor platform-server deploy vps

note:
    Source Repo-scoped deployment is supported by `deploy local`
```

This is better than:

```text id="k7njg2"
error: operation unavailable in current state
```

---

# Unknown State

Vapor should permit:

```text id="4csk6w"
unknown
unverified
unreachable
```

where reality cannot currently be observed.

For example, Registry outage may make remote publication state unknown.

Unknown is better than guessing.

---

# External Modification

Vapor must expect source and local state to change outside Vapor.

Examples:

```text id="4rg09a"
Git CLI modifies repository

RustRover modifies source/Cargo manifests

Steam updates files

user moves filesystem data

provider changes remotely
```

Diagnosis/reconciliation should compare authoritative state rather than assuming all changes were initiated through Vapor.

---

# Operational Observability

For meaningful operations Vapor should expose enough information to answer:

```text id="7yn8rp"
what subject is being operated on?

what exact canonical object(s) were resolved?

what explicit Selection is active?

what Context was used for resolution?

what precondition failed?

which external system is involved?

which state changed?

which state was intentionally preserved?

what safe recovery/retry options exist?
```

This is part of the developer/user experience, not merely debug logging.

---

# Core Operational Invariants

* “Ecosystem” describes Vapor as a whole; it is not one generic public operation subject.
* Concrete operations belong to concrete semantic owners.
* Operational state is multidimensional rather than one giant serialized lifecycle.
* Context means identity/path resolution, not a readiness predicate.
* Readiness Predicates are derived questions over Operational Situations.
* Operation Subject establishes natural complete scope.
* Explicit Operation Selection changes requested operation scope where legal.
* Persistent Context does not silently change operation scope.
* Operation Selection is distinct from selected/default Vapor App.
* Root Authority is distinct from installed Role.
* Player operation does not require Git.
* Player operation does not require Rust/Cargo.
* Player launch does not require source.
* Player launch does not build compositions.
* The canonical Superworkspace is independent from the Steam App Instance lifecycle.
* Source Repo Container absence is an acquisition concern.
* Missing declared Source Repo checkout is a Git reconciliation concern.
* Derived-state failure is Repair territory only when safe regeneration is possible.
* Dirty Git state is not automatically broken Vapor state.
* Relevant source/resolution changes may make builds stale.
* A stale build may remain runnable.
* A failed rebuild must not automatically destroy a previous valid Vapor App.
* Provider authentication must not silently alter source/build/install state.
* Authorization is target/action-specific and distinct from Role.
* Role downgrade must not silently delete authored source.
* App selection must not mutate source.
* Git branch/source changes must not silently change selected installed App.
* Runtime launch operates on a built runnable App rather than implicitly composing/building.
* External outages interfere only with operations which require those systems.
* Published Content versions are immutable historical state.
* Historical published Packagepack resolutions do not change when newer dependencies appear.
* Deployment is facility/target-specific rather than a universal ecosystem lifecycle.
* Failures change only state dimensions legitimately owned by the failed operation.
* Partial external success must be diagnosable and reconcilable.
* Unique authored local state must be protected from destructive automation.
* Vapor should represent unknown external state as unknown rather than guessing.

---

# Open Operational Questions

The following remain intentionally open:

* Exact complete state-dimension inventory.
* Exact machine representation of Operational Situations.
* Which Readiness Predicates deserve first-class Core APIs versus remaining derived UI concepts.
* Exact operation-progress/cancellation model.
* Exact operation-history persistence.
* Exact concurrency/locking semantics between overlapping operations.
* Exact conflict behavior when two operations request overlapping topology.
* Exact representation of partially completed acquisitions/installations.
* Exact reconciliation behavior after externally modified filesystem/Git state.
* Exact durable diagnostics/log retention model.
* Exact Vapor App selection behavior when selected versions become unavailable/Yanked/Banned.
* Exact Player update interaction with installed/selected App state.
* Exact first-party deployment-state representation.
* Exact publication/deployment retry and reconciliation transaction model.
* Whether a formal generated state/operation diagram should be produced from shared Core definitions once implementation stabilizes.
