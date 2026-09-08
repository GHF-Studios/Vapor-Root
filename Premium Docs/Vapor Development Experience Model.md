> [!info]
> This document defines Vapor's source-authoring and development experience.
>
> It covers Composer, Content Developer, Ecosystem Developer, the canonical Superworkspace, source acquisition and creation, Git/provider interaction, Project development, Rust/Cargo realization, external IDE integration, build/test/run workflows, diagnostics, and development-state safety.
>
> Canonical identity, Source Repo Container / Source Repo / Project hierarchy, Context, Selection, and ambiguity semantics are owned by the **Vapor Context, Identity, Session And Selection Model**.
>
> Exact command-line grammar is owned by the **Vapor CLI Model**.
>
> Publication itself is owned by the **Vapor Publishing And Distribution Model**.

---

# Core Principle

Vapor development should feel like developing the thing the user actually intends to create.

The developer should not need to manually orchestrate:

```text
Git hosting
→ Git repositories
→ submodules
→ Rust toolchains
→ Cargo workspaces/packages
→ generated configuration
→ build targets
→ Steam
→ Registry state
→ IDE configuration
```

merely to express:

```text
Create this Game.
Test this Project.
Develop Vapor Client.
Publish this Library.
```

Vapor exists to model and coordinate those underlying systems without pretending they do not exist.

The governing principle is:

> **Vapor owns the semantic development workflow; Git, Cargo, Rust, Steam, providers, and external IDEs remain visible underlying tools with their own legitimate semantics.**

---

# Development Capability Layers

Vapor development capability grows cumulatively:

```text
Player
→ Composer
→ Content Developer
→ Ecosystem Developer
```

These are locally installed Vapor Roles.

Root Authority is not another Role.

It is external authority over protected official Vapor resources.

Therefore:

```text
Role
    = what this local Vapor environment is equipped to do

Authority
    = what this identity is permitted to do to a protected target
```

An Ecosystem Developer can develop Vapor locally without possessing official Root Authority.

A Content Developer can develop local Content without being authenticated to GitHub.

Local capability must not unnecessarily depend on remote authorization.

---

# Composer Development

A Composer authors composition-oriented Projects.

These may include:

```text
Packagepack
Enginepack
Gamepack
Modpack
```

A Composer may consume existing behavioral/reusable Projects such as:

```text
Engine
Game
Engine Mod
Game Mod
Extension Mod
Library
```

without necessarily possessing authority to modify their implementation.

The normal Composer loop is approximately:

```text
acquire or create source
→ modify composition
→ resolve dependencies
→ build/test complete composition
→ inspect result
→ revise
→ optionally publish
```

The developer should think primarily in Vapor composition semantics.

Cargo and Git remain inspectable but should not define the ordinary workflow.

---

# Content Development

A Content Developer additionally authors behavioral and reusable Projects such as:

```text
Engine
Game
Engine Mod
Game Mod
Extension Mod
Library
```

Typical Content development is approximately:

```text
acquire or create source topology
→ create/open Project
→ edit implementation/configuration
→ resolve required dependencies
→ test/build in an appropriate composition context
→ inspect diagnostics/runtime behavior
→ revise
→ optionally publish
```

Because Vapor Apps are statically composed, behavioral Projects may ultimately need testing inside a complete Packagepack composition.

This does not require every local edit to trigger a physically complete rebuild.

Cargo/Vapor incremental compilation and caching should optimize iteration while preserving the semantic complete-composition model.

---

# Ecosystem Development

An Ecosystem Developer develops Vapor itself.

This may include:

```text
Vapor Core
Vapor Client
Vapor Installer
Vapor Launcher / SDK surfaces
Vapor CLI
managed toolchain integration
Registry
Identity
Diagnostics
documentation infrastructure
Platform Server services
deployment orchestration
distribution tooling
provider integrations
internal examples/tooling
```

Ecosystem Developer is locally attainable.

It does not inherently grant authority to:

```text
push to protected official repositories
publish into protected official namespaces
deploy official Steam branches/depots
modify production Platform infrastructure
administer Root-Authority-controlled resources
```

Those operations require separate authorization.

The normal first-party development experience should therefore support:

```text
acquire first-party source
→ modify locally
→ build/test locally
→ inspect
→ commit/push where authorized
→ deploy to an appropriate target where authorized
→ validate
```

without requiring protected authority merely to reach the local build/test stages.

---

# Canonical Development Topology

Vapor development uses one canonical source hierarchy:

```text
Authority
└── Source Repo Container
    └── Source Repo
        └── Project
```

For example:

```text
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

represents:

```text
GHF-Studios    Authority
Loo-Cast       Source Repo Container
Game           Source Repo
Loo-Cast       Project
```

The hierarchy is owned normatively by the Context/Identity model.

This document focuses on how that hierarchy is realized and used during development.

---

# The Canonical Superworkspace

Vapor maintains one canonical local **Superworkspace**.

The Superworkspace is the development source root.

It is locally foundational but globally meaningless.

It:

* has no Vapor identity;
* is never part of an Authority/Container/Repo/Project ID;
* has one known configured physical location;
* contains canonical local source realizations;
* may contain multiple Authorities;
* is not itself a Git repository merely because it is a Superworkspace;
* is not supplied repeatedly to ordinary create/acquire operations.

Conceptually:

```text
<Superworkspace>/
├── GHF-Studios/
│   ├── Vapor-Client/
│   ├── Vapor-Platform-Server/
│   └── Loo-Cast/
└── Some-Author/
    └── My-Content/
```

Its filesystem name is irrelevant.

Its physical location may change without changing any Vapor identity.

---

# Superworkspace Location

The Superworkspace location is local development configuration.

For example, one developer might use:

```text
~/Development/Vapor
```

while another uses:

```text
/mnt/Development/Vapor
```

Those locations do not participate in Vapor identity.

Vapor should provide a deliberate way to configure or relocate the Superworkspace as one unit.

Acquisition itself should not accept arbitrary destination paths.

This avoids turning every source operation into filesystem-placement policy.

---

# Steam Installation, User Data, and Source

Development state should be separated into three conceptual storage roots:

```text
Steam App Instance
Vapor User Data
Superworkspace
```

## Steam App Instance

Owned primarily by Steam/depot distribution.

It contains replaceable shipped application/runtime files.

## Vapor User Data

Owned by Vapor for mutable and derived local state.

This may include:

```text
Role/tooling state
managed toolchain metadata
Context/resume information
caches
generated realization
local indexes
IDE integration state
diagnostics state
configured Superworkspace location
```

## Superworkspace

Contains authored development source.

This includes Git repositories and user-authored project files.

The central safety property is:

> **Steam installation replacement must not imply authored source replacement.**

---

# Authored vs Derived Development State

Development state must be classified by ownership.

## Authored

Examples include:

```text
Container metadata
Workspace.vapor.toml
Vapor.toml
Cargo.toml
source code
tests
scripts
Git commits/branches/remotes
explicit operation recipes
explicit developer configuration
```

Authored state requires strong preservation guarantees.

## Derived

Examples may include:

```text
indexes
caches
generated glue
resolved operation state
IDE project projections
generated Cargo/Vapor reconciliation
build caches
temporary artifacts
diagnostic indexes
```

The derived-state invariant is:

```text
deleteable
→ rediscoverable
→ regeneratable
→ idempotently repairable
```

If a piece of state cannot satisfy that standard, Vapor should be careful before labeling it derived.

---

# Source Repo Container

A **Source Repo Container** is a registered Vapor Git repository grouping related Source Repos.

It is the default independent source acquisition boundary.

A Container may exist initially without Source Repos.

Example:

```text
GHF-Studios/My-Stuff
```

may initially realize as:

```text
<Superworkspace>/
└── GHF-Studios/
    └── My-Stuff/
        └── .git/
```

A Container should not need a fake Cargo package, dummy Workspace, or placeholder Project merely to exist.

---

# Source Repo

A **Source Repo** is one Git repository and one Vapor Workspace.

The invariant is:

> **One Source Repo = one Vapor Workspace.**

Source Repos normally participate in a Container's authored Git topology through submodules.

For example:

```text
My-Stuff/
├── .gitmodules
├── Game/
└── Libraries/
```

where:

```text
Game
Libraries
```

are Source Repos / Vapor Workspaces.

This is real source topology.

It should remain visible as Git.

---

# Project

A **Project** is a modeled Vapor development unit inside one Source Repo.

A Project is not itself a Git repository.

A Project may represent a Content kind such as:

```text
Game
Engine
Library
Packagepack
...
```

or an internal/non-Content responsibility where appropriate for first-party development.

For Rust-backed Projects it may contain:

```text
one or more Cargo packages
one or more crates/targets
tests
examples
generated integration
supporting packages
```

Vapor Project boundaries should express meaningful development responsibility.

They should not merely duplicate repository names because that is mechanically convenient.

---

# Vapor Topology vs Cargo Topology

These are different layers.

Vapor source/development topology:

```text
Source Repo Container
→ Source Repo
→ Project
```

Rust/Cargo realization may then include:

```text
Cargo workspace
→ package
→ crate/target
```

The user should be able to descend into Cargo terminology where relevant.

Vapor should not rename Cargo concepts.

Likewise, Vapor should not call Source Repos Cargo workspaces merely because Rust implementation happens to use Cargo.

---

# Explicit Source Topology Creation

Vapor should not silently invent missing source hierarchy during higher-level creation.

The normal explicit sequence is:

```text
create Source Repo Container
→ create Source Repo
→ create typed Project
```

For example:

```text
vapor source-repo-container create GHF-Studios/My-Stuff

vapor source-repo create GHF-Studios/My-Stuff/Game

vapor game create GHF-Studios/My-Stuff/Game/My-Game
```

Each operation creates exactly the semantic layer it names.

This makes source topology intentional and comprehensible.

---

# Creating a Source Repo Container

Creating:

```text
GHF-Studios/My-Stuff
```

conceptually involves:

```text
verify/create Authority relationship
→ establish registered Container identity
→ create provider-backed Git repository
→ initialize Container metadata
→ place checkout in canonical Superworkspace
→ establish Registry/provider linkage
```

It creates zero Source Repos unless explicitly requested separately.

Provider visibility should be conservative.

New repositories should default to private where provider semantics and user authority permit that default.

Public source should be an intentional state.

---

# Creating a Source Repo

Creating:

```text
GHF-Studios/My-Stuff/Game
```

conceptually involves:

```text
verify parent Container
→ create provider-backed Git repository
→ initialize Vapor Workspace metadata
→ register Source Repo identity/topology
→ add repository as authored Git submodule
→ update parent Container Git topology
```

This is a Git operation as well as a Vapor operation.

Vapor should not pretend the submodule relationship is mystical internal state.

Advanced users should be able to inspect normal:

```text
.gitmodules
git status
git submodule status
```

and understand what happened.

---

# Creating a Project

Typed Project creation occurs only after its parent Source Repo exists.

For example:

```text
vapor game create GHF-Studios/My-Stuff/Game/My-Game
```

creates a Project with:

```text
kind = Game
```

within Source Repo:

```text
GHF-Studios/My-Stuff/Game
```

Project creation may generate appropriate:

```text
Vapor metadata
Cargo structure
source layout
tests
starter configuration
```

according to Project kind.

It must not silently create missing Containers or Source Repos.

---

# Missing Parent Experience

When a requested hierarchy does not exist, Vapor should teach the missing step.

For example:

```text
vapor game create GHF-Studios/My-Stuff/Game/My-Game
```

when the Container is absent:

```text
error: Source Repo Container `GHF-Studios/My-Stuff` does not exist

help: create it:
      vapor source-repo-container create GHF-Studios/My-Stuff
```

If the Container exists remotely but not locally:

```text
error: Source Repo Container `GHF-Studios/My-Stuff`
       is not available in the local Superworkspace

help: acquire it:
      vapor source-repo-container acquire GHF-Studios/My-Stuff
```

If only the Source Repo is missing:

```text
error: Source Repo `GHF-Studios/My-Stuff/Game` does not exist

help: create it:
      vapor source-repo create GHF-Studios/My-Stuff/Game
```

Errors should describe actual state rather than emitting canned generic advice.

---

# Source Acquisition

Vapor source acquisition is Registry-aware.

It is not merely a synonym for `git clone`.

The normal flow is:

```text
Vapor source identity
→ Registry
→ provider linkage
→ Git acquisition
→ authored topology initialization
→ Vapor validation
→ canonical Superworkspace availability
```

A generic acquisition operation should only accept registered Vapor source of the appropriate kind.

---

# Source Repo Container Acquisition

The normal source acquisition unit is a complete Source Repo Container.

Conceptually:

```text
vapor source-repo-container acquire GHF-Studios/Loo-Cast
```

means:

```text
resolve Registry identity
→ verify Source Repo Container kind
→ resolve provider
→ clone canonical Container repository
→ initialize authored Source Repo submodules
→ validate topology
→ recognize contained Workspaces and Projects
```

Acquisition is conservative at the Container boundary because nested Source Repos may depend on each other for coherent build/test/development behavior.

---

# Atomic Acquisition

The default principle is:

> **A Source Repo Container is the smallest independently acquirable unit unless a narrower unit is explicitly declared independently realizable.**

This is not a claim that Git technically cannot clone a Source Repo.

It is a Vapor semantic guarantee.

A Container acquisition provides the source family expected by its authored topology.

---

# Independently Acquirable Source Repos

Some Source Repos may legitimately be useful independently.

Independent acquisition is opt-in modeled behavior.

For example:

```text
vapor source-repo acquire GHF-Studios/Some-Container/Examples
```

may succeed when Registry/topology explicitly permits it.

Otherwise Vapor should explain:

```text
error: Source Repo `GHF-Studios/Some-Container/Internal`
cannot be acquired independently

it belongs to Source Repo Container:
    GHF-Studios/Some-Container

help: acquire the complete Container:
      vapor source-repo-container acquire GHF-Studios/Some-Container
```

Independent acquisition should default to disabled/private-to-Container.

---

# Arbitrary Git Repositories

Vapor acquisition is not intended to replace general Git cloning.

An arbitrary repository remains:

```text
git clone ...
```

If the repository is not registered as a Vapor Source Repo Container or independently acquirable Source Repo, Vapor should reject it from generic Vapor acquisition.

That restriction provides semantic guarantees.

---

# Manually Acquired Valid Source

Vapor must not require that it personally performed every Git operation.

An advanced user may manually:

```text
clone a registered Container
place it at its canonical Superworkspace location
initialize required submodules
```

using ordinary Git.

If the resulting source matches valid registered Vapor topology, Vapor should recognize it.

This preserves the principle:

> **Vapor orchestrates Git; it does not claim exclusive ownership of Git.**

---

# Provider Independence

GitHub may be the primary provider today.

The conceptual model should remain:

```text
Vapor identity
→ Registry
→ Git provider/repository
```

rather than:

```text
Vapor identity = GitHub URL
```

Provider-native IDs and URLs remain important linkage and diagnostic information.

They are not the canonical Vapor source identity.

---

# Forks

Git forks remain Git/provider concepts.

A fork does not automatically become a new Vapor Source Repo identity merely because its provider owner differs.

Conversely, an independently registered new Vapor source identity is a semantic operation, not merely a provider fork.

Vapor should preserve both facts:

```text
semantic Vapor identity
provider/fork relationship
```

without collapsing one into the other.

Exact collaboration/fork UX may evolve later.

---

# First-Party Source

Official Vapor facilities may receive bespoke development workflows.

Current examples include:

```text
Vapor Client
Vapor Platform Server
Vapor Examples
```

A first-party facility may map to one or more registered source identities.

For example:

```text
client acquire
```

may internally resolve:

```text
GHF-Studios/Vapor-Client
```

without requiring the developer to type that identity.

This is a semantic convenience over normal source machinery.

---

# First-Party Trust

There is an important distinction between:

```text
GHF-Studios/Vapor-Client
```

and:

```text
Some-Author/My-Content
```

The difference should not be modeled merely as a self-declared manifest flag.

First-party status is trusted Registry/Root-Authority state.

A third-party Container cannot grant itself authority to:

```text
introduce new Vapor root CLI namespaces
override Client semantics
override Platform Server semantics
become an official Vapor subsystem
```

First-party trust is a capability boundary.

---

# Source Repo Container Kind / Purpose

Source Repo Containers may eventually need modeled purpose/kind information in addition to trust.

For example:

```text
trust:
    first-party

facility:
    client
```

is different from:

```text
trust:
    standard

facility:
    none
```

There may also be useful distinctions among ordinary Content-oriented Containers and internal infrastructure Containers.

The exact taxonomy is intentionally not frozen yet.

What is frozen is:

> **Trusted first-party facility semantics cannot be self-declared by arbitrary third-party source.**

---

# Bespoke First-Party Development

A Root/Ecosystem Developer should be able to work directly with facilities such as:

```text
vapor client acquire
vapor client build
vapor client test
vapor client deploy local
vapor client deploy steam
```

and:

```text
vapor platform-server acquire
vapor platform-server build
vapor platform-server test
vapor platform-server deploy local
vapor platform-server deploy vps
```

These operations may internally reuse generic:

```text
source acquisition
selection resolution
operation recipes
Git execution
Cargo execution
deployment primitives
diagnostics
```

Bespoke UX does not require duplicated implementation.

---

# Operation Recipes

Complex first-party build/test/deploy behavior should not require every orchestration detail to be hardcoded permanently into Vapor Core.

Vapor should support lightweight authored operation recipes.

The desired character is:

```text
declarative workflow
+
Vapor-native primitives
+
small scripts/process calls
```

not:

```text
arbitrary application/plugin framework
```

An operation recipe may eventually describe:

```text
accepted topology scopes
required Source Repos
required Projects
step dependencies
parallelism
build/test steps
artifacts
environment requirements
deployment actions
health checks
validation
```

The exact schema remains unsettled.

---

# Authored Operation Scripts

Where declarative primitives are insufficient, small checked-in scripts are acceptable.

Examples:

```text
scripts/deploy-local.sh
scripts/deploy-vps.sh
scripts/smoke-test.sh
```

Such scripts should remain:

* visible;
* version controlled;
* inspectable;
* narrowly scoped;
* subordinate to the modeled Vapor operation.

Vapor should not encourage entire hidden applications to be smuggled into “operation recipes.”

---

# Operation Scope

A development operation has a natural subject.

For example:

```text
Vapor Client
Vapor Platform Server
one Source Repo
one Project
one Packagepack
```

No explicit selector means:

> operate on the subject according to that operation's natural complete semantics.

It does not generically mean:

```text
select every descendant and run one identical command against each
```

This distinction is important for first-party workflows.

---

# Explicit Selection

Developers may explicitly narrow an operation to a meaningful descendant/subtree where the operation supports that scope.

For example:

```text
Platform Server test
    subject = complete Platform Server

selection = Registry
```

may request a Registry-scoped test.

Selecting a Source Repo naturally includes the relevant descendants for that Source Repo-scoped operation.

A generic extra:

```text
--all-projects
```

is not required merely to say “include its children.”

---

# Operation-Specific Legality

Different operations may support different scopes.

For example:

```text
platform-server test --select Registry
```

may be valid.

```text
platform-server deploy local --select Registry
```

may also be valid.

But:

```text
platform-server deploy vps --select Registry
```

may be invalid because production deployment requires the complete configured Platform Server deployment unit.

These rules should come from actual operation semantics/recipes rather than a universal hardcoded assumption such as:

```text
deploy never supports partial selection
```

---

# Specific Development Diagnostics

Errors should explain the actual development invariant.

For example:

```text
error: Platform Server VPS deployment requires the complete
       Platform Server deployment scope

selected:
    Source Repo `Registry`

required:
    entire Vapor Platform Server

help: deploy the complete Platform Server:
      vapor platform-server deploy vps

note:
    Source Repo-scoped deployment is supported by `deploy local`
```

This style is especially important for developers who may still be learning:

```text
Vapor
Git
Rust
Cargo
game development
software development
```

Vapor should teach rather than merely reject.

---

# Context During Development

Persistent Vapor Context is a lightweight location/resolution convenience.

It is not the development source itself.

It is not operation scope.

It is not a required global “active Project.”

A developer may navigate to:

```text
GHF-Studios/Loo-Cast/Game
```

so that:

```text
Loo-Cast
```

can later resolve to the Project beneath it.

But unrelated commands such as:

```text
vapor client test
```

remain unaffected.

This is a deliberate safety property.

---

# CWD During Development

Vapor semantic targeting must not depend on shell CWD.

A developer may be physically inside one repository while intentionally operating on another modeled object.

Therefore Vapor should use:

```text
canonical IDs
persistent Context
explicit per-command Context
operation subjects
explicit selections
```

rather than silently interpreting CWD as semantic location.

Underlying tools remain free to use CWD when invoked directly.

For example:

```text
git status
cargo test
```

remain ordinary shell-tool behavior.

---

# Git Is Fundamental

Git is Composer-and-above development infrastructure.

Vapor should automate Git without pretending Git does not exist.

Vapor may orchestrate operations such as:

```text
clone
fetch
pull
commit
push
repository creation
submodule initialization
status inspection
provider linkage
```

where appropriate.

Advanced developers must remain able to use Git directly.

---

# Git Safety

User-authored Git source is not disposable.

Vapor must assume:

```text
dirty working trees may be intentional
unpushed commits may exist
detached HEADs may be intentional
submodules may be on development commits
branches may diverge
external Git tools may modify repositories
conflicts may exist
```

Therefore Vapor must not silently:

```text
reset repositories
discard uncommitted files
delete repositories
drop commits
switch branches
force submodules back to gitlinks
replace source with remote state
```

merely because local Git state differs from the last expected state.

---

# Dirty Does Not Mean Broken

A dirty repository is ordinary development state.

Likewise:

```text
detached submodule HEAD
submodule ahead of parent gitlink
local unpushed branch
```

may be completely valid during development.

Vapor should distinguish:

```text
state differs from recorded baseline
```

from:

```text
source is invalid/unusable
```

Diagnosis should expose the difference clearly.

---

# Missing Source Repo Checkout

Suppose a Container declares a Source Repo through Git submodule topology, but the local checkout is missing.

This is not fresh Source Repo acquisition.

The Source Repo already belongs to the acquired Container topology.

It is a Git reconciliation problem.

Vapor may report:

```text
Source Repo `Registry` is declared by the Container's Git topology
but its local submodule checkout is missing.

inspect:
    git status
    git submodule status
```

and may offer:

```text
possible reconciliation:
    git submodule update --init -- Registry
```

with appropriate caution.

---

# Cautious Git Remediation

Vapor should distinguish:

```text
inspection
possible action
proven-safe action
```

If Vapor cannot guarantee that a Git command preserves the user's intended development state, it should say so.

For example:

```text
warning:
    this command changes local Git checkout state.
    Vapor cannot guarantee that applying it is appropriate for
    your current development branch/worktree.
```

This is preferable to either extreme:

```text
say nothing
```

or:

```text
RUN THIS NOW OR EVERYTHING IS BROKEN
```

---

# Diagnose

Diagnosis observes and explains.

Development diagnosis may inspect:

```text
source availability
Registry linkage
Container topology
Git status
submodule state
Vapor manifests
Cargo metadata
toolchain readiness
build state
operation recipes
IDE integration
generated state
```

Diagnosis should distinguish the ownership domain of each problem.

For example:

```text
Git problem
Cargo problem
Vapor manifest problem
derived Vapor state problem
provider problem
toolchain problem
```

This helps users understand which subsystem actually owns the fix.

---

# Repair

Repair acts only where Vapor can safely reconstruct or reconcile derived state.

Examples may include:

```text
regenerate indexes
recompute generated glue
reconcile deterministic Vapor-owned Cargo entries
rebuild IDE integration
rebuild operation realization
recreate caches
re-register observable existing source
```

Repair must not silently:

```text
clone missing authored Containers
restore deleted authored source
reset Git
discard dirty files
drop commits
change branch
force submodule commits
```

Repair is not disaster-recovery source acquisition.

---

# Source Acquisition vs Repair

The distinction should remain explicit:

```text
source absent
    → acquire

authored Container exists, submodule missing
    → Git diagnosis/reconciliation

derived Vapor state missing/broken
    → repair
```

This separation prevents Vapor from treating user-authored source as disposable cache state.

---

# Rust Toolchain

Rust/Cargo used for Vapor development should be managed as part of Vapor's installed development capability where appropriate.

The toolchain should be:

```text
versioned
controlled
discoverable
diagnosable
repairable
reproducible enough for Vapor development
```

Root/Ecosystem development should strongly prefer the vendored/pinned Vapor-managed toolchain rather than an arbitrary ambient system toolchain.

Underlying Rust and Cargo remain real tools.

Vapor manages their relationship to its development environment.

---

# Managed Cargo

Vapor may expose managed Cargo through:

```text
vapor toolchain cargo -- ...
```

This should establish the appropriate managed toolchain environment and then preserve normal Cargo semantics.

Vapor may help resolve the correct Project before invoking Cargo.

Cargo-native concepts remain Cargo-native:

```text
workspace
package
crate
target
feature
profile
```

Vapor should not rebrand them unnecessarily.

---

# Cargo Reconciliation

Vapor semantic dependency intent and Cargo physical dependency realization remain distinct.

Conceptually:

```text
Vapor semantic dependency intent
        ↓
Vapor resolution
        ↓
desired physical relationship
        ↓
Cargo reconciliation
        ↓
editable Cargo manifests
        ↓
Cargo resolution / metadata
        ↓
verified physical graph
```

Cargo remains authoritative for the graph it will actually compile.

Vapor remains authoritative for Vapor semantic relationships.

---

# Cargo Configuration Ownership

Vapor should manage only the parts of Cargo configuration it semantically owns.

Developers may continue editing normal Cargo configuration.

Vapor should distinguish:

```text
Vapor-managed relationship
user-authored Cargo configuration
local override
conflict
stale generated relationship
```

Repair/reconciliation must not casually overwrite unrelated Cargo intent.

---

# External IDE Integration

Vapor's SDK is the primary first-party development environment.

External IDEs such as RustRover remain valuable.

The relationship should be:

```text
Vapor
    owns semantic source topology,
    managed toolchain,
    generated integration,
    build/test operations,
    diagnostics

External IDE
    provides mature editing,
    refactoring,
    navigation,
    debugging,
    Git UI,
    language tooling
```

The two should cooperate rather than compete.

---

# IDE State Is Derived

IDE configuration generated/reconciled by Vapor should be treated as derived integration state where possible.

The desired properties are:

```text
deleteable
→ rediscoverable
→ regeneratable
→ idempotently repairable
```

Vapor should preserve unrelated user IDE state.

It should only own the integration fragments for which it has a clear semantic mapping.

---

# RustRover Integration

For supported RustRover integration, Vapor may manage appropriate project/toolchain configuration while preserving unrelated IDE components.

The integration should use:

```text
canonical Superworkspace/source topology
managed Rust/Cargo toolchain
Vapor-aware Project structure
```

rather than requiring the user to manually recreate IDE configuration after every source acquisition or recovery.

IDE integration should still remain recoverable derived state.

---

# Build Semantics

Users should request semantic Vapor builds.

For example:

```text
build this Packagepack
build Vapor Client
build this Project
```

depending on operation support.

Vapor then coordinates the required physical steps.

These may include:

```text
dependency resolution
source availability checks
Cargo invocation
generated code
operation recipes
artifact creation
validation
local registration
```

The user should retain access to underlying commands/output where useful.

---

# Natural Operation Scope

An operation subject defines its normal scope.

For example:

```text
vapor client build
```

means:

> Build Vapor Client according to the authored Client build operation.

It does not mechanically mean:

> Find every Project beneath Client and run `cargo build` separately against all of them.

The authored operation may be richer.

Likewise:

```text
vapor platform-server test
```

means:

> Test Platform Server according to its complete test semantics.

---

# Project Build/Test

Where a Project supports standalone build/test operations, Vapor may target that Project directly.

For example:

```text
vapor test GHF-Studios/My-Stuff/Game/My-Game
```

resolves the Project, discovers its kind and realization, validates operation legality, then performs the appropriate test operation.

The user should not need to redundantly write:

```text
vapor game test ...
```

merely because Vapor already knows that Project is a Game.

---

# Test Composition Context

Behavioral Content may require a complete composition to exercise meaningfully.

For example, a Game Mod cannot necessarily run independently.

Vapor therefore needs a test/run composition mechanism.

Possible sources may include:

```text
explicit Packagepack
Project-authored preferred test Packagepack
generated development Packagepack
development composition overlay
```

The exact representation remains open.

What is required is:

> **Vapor must make complete-composition testing practical without pretending every behavioral Project is independently runnable.**

---

# Run Experience

A development Run operation may need to:

```text
resolve effective Packagepack/test composition
→ validate source
→ build stale required artifacts
→ install/register local Vapor App
→ launch
→ associate diagnostics/runtime logs with source context
```

This semantic sequence does not imply every invocation physically repeats every stage.

Current state and caching should eliminate unnecessary work.

---

# Failed Build Preservation

A failed rebuild must not destroy a previous working Vapor App.

Development state may legitimately be:

```text
source:
    newer / dirty

latest build attempt:
    failed

previous installed Vapor App:
    valid and runnable
```

Vapor should preserve these dimensions separately.

This is important for both developer safety and understandable diagnostics.

---

# Development Deployment

Deployment is not one universal operation.

Different first-party facilities may define distinct deployment targets.

Examples:

```text
client deploy local
client deploy steam

platform-server deploy local
platform-server deploy vps
```

Each target may have its own:

```text
accepted scope
validation
artifact requirements
authorization
scripts
health checks
rollback/recovery behavior
```

Deployment rules should remain specific rather than being flattened into one generic “deploy everything” lifecycle.

---

# Local Deployment

Local deployment should be usable by ordinary Ecosystem Developers without production authority where technically possible.

This allows first-party systems to be developed and tested safely.

Examples:

```text
local Client realization
local Platform Server stack
local Registry service
local generated environment
```

Local equivalents should approximate production semantics enough to reveal integration problems without requiring access to protected infrastructure.

---

# Protected Deployment

Production/official deployment requires target-specific authority.

For example:

```text
Steam deployment
production Platform VPS
protected Registry infrastructure
official hosted services
```

Authorization failure should not be confused with missing local development capability.

A developer may be fully capable of building/testing locally while correctly being denied production deployment.

---

# Examples Facility

Official Examples serve as architecture-proving development material.

They may be acquired through a stable first-party operation such as:

```text
vapor examples acquire
```

The Examples facility may contain:

```text
Content Projects
Library examples
composition examples
Cargo reconciliation tests
integration examples
architecture-proving scenarios
```

Its current Git topology may evolve.

The semantic purpose of `examples` should remain stable.

---

# Source Visibility and Privacy

New developer source should be conservative by default.

Where provider support permits:

```text
new repository
    → private by default
```

Public visibility should be intentional.

Likewise, independently acquirable Source Repo behavior should be opt-in.

This makes accidental publication less likely.

---

# Development Removal

Removing development capability must distinguish:

```text
installed tools
derived caches
generated state
authored source
```

A Role downgrade may remove tooling or hide higher-capability UI.

It must not silently remove authored Git source.

Likewise, uninstalling/reinstalling the Steam App should not be treated as permission to erase the Superworkspace.

---

# Steam Reinstall

Because authored source lives outside the Steam depot boundary:

```text
Steam uninstall/reinstall
```

may replace:

```text
Vapor binaries
bootstrap files
depot-managed runtime files
```

while preserving:

```text
Superworkspace
authored source
Vapor user data where appropriate
```

On reinstallation, Vapor can reconnect to its configured Superworkspace and regenerate derived state.

---

# Derived-State Loss

If Vapor-owned derived development state is deleted, Vapor should reconstruct it from:

```text
authored source
Registry state
Git state
managed configuration
installed capability
```

For example:

```text
delete IDE integration
delete indexes
delete generated operation realization
```

should be recoverable through diagnosis/repair.

Deleting authored source is fundamentally different.

---

# Disaster Recovery

A robust Vapor development environment should support recovery from loss of replaceable local state.

Conceptually:

```text
Steam/Vapor application reacquired
→ user data reconstructed/reattached
→ registered source reacquired where actually missing
→ authored Git topology restored explicitly/safely
→ derived state regenerated
→ toolchain/IDE integration reconciled
→ build/test restored
```

Recovery must distinguish:

```text
recoverable remote-authored source
unique local authored changes
derived local state
```

Unique local work must never be treated as safely disposable merely because remote source exists.

---

# Development Context UX

The user should not need to repeatedly enter raw filesystem paths once Vapor understands the topology.

Preferred concepts are:

```text
Authority
Source Repo Container
Source Repo
Project
Context
Selection
```

Raw paths remain useful for:

```text
explicit filesystem diagnostics
bootstrap/import
locating moved data
underlying Git/Cargo tools
```

but are not the ordinary semantic interface.

---

# Context Is Optional Convenience

Persistent Context may make repetitive work shorter.

For example:

```text
open:
    GHF-Studios/My-Stuff/Game
```

then:

```text
My-Game
```

may be sufficient to identify:

```text
GHF-Studios/My-Stuff/Game/My-Game
```

But a developer can always provide enough explicit information directly.

Context must never be the sole route to expressing location.

---

# Per-Operation Context Override

A one-shot command or GUI action must be able to provide a temporary alternate Context.

This is analogous to:

> being navigated somewhere in a UI or shell, but deliberately operating on another path.

Transient Context exists for one operation.

It should not mutate persistent navigation state.

This makes scripts and automation independent of forgotten interactive Context.

---

# Selection Is Operation Intent

Selection is separate from Context.

For example:

```text
Context:
    GHF-Studios/Vapor-Platform-Server/Registry
```

does not cause:

```text
platform-server test
```

to become Registry-scoped.

Only explicit operation selection does that.

This protects against forgotten navigation state silently changing build/test/deploy behavior.

---

# Development Error Philosophy

Vapor should make failure states useful.

A good development error may explain:

```text
what object was resolved
what parent is missing
what kind it is
why this operation is illegal
what local state is missing
what Git state differs
what authority is absent
what safe options are available
```

Errors are part of the developer learning experience.

They should be written for someone who may not yet know the underlying tool well.

---

# Exhaustive Safe Guidance

Where Vapor knows several safe resolutions, diagnostics should present them.

For ambiguous Project selection, this may include:

```text
fully qualified candidates
shorter disambiguating selectors
explicit --select examples
persistent Context/open alternatives
transient --context alternatives
```

Where a suggested action may change authored Git state, Vapor should label it as a possible action rather than guaranteed safe remediation.

---

# Development-State Observability

The SDK should expose enough state for developers to understand:

```text
what source exists
where it belongs
which Git repository owns it
what branch/commit state exists
which Project is being operated on
which Cargo packages realize it
which dependencies are resolved
what build state exists
which operation recipe ran
what artifacts were produced
why an operation was rejected
```

Convenience should not require opacity.

---

# SDK Development Experience

The Vapor SDK is the primary first-party graphical development surface.

Its development model should project the canonical hierarchy:

```text
Superworkspace
→ Authorities
→ Source Repo Containers
→ Source Repos
→ Projects
→ deeper Project/Cargo/content detail
```

The GUI may present this with:

```text
Explorer
Inspector
editors
problems
logs
run/test actions
Git state
dependency graphs
operation output
```

The exact visual organization belongs to the SDK Experience Model.

---

# CLI / SDK Equality

CLI and SDK should expose equivalent semantic capability where practical.

That does not require identical interaction.

Example:

```text
SDK:
    navigate tree
    select Registry
    click Test

CLI:
    vapor platform-server test --select "Registry"
```

Both should invoke the same underlying Vapor Core operation semantics.

---

# Automation

Automation should not depend on hidden interactive state.

It should be able to provide:

```text
full canonical paths
explicit transient context
explicit selection
machine-readable output
exact operation variants
```

Persistent interactive Context may remain available but should not be required.

---

# Development Invariants

* Vapor has one canonical local Superworkspace.
* The Superworkspace has no Vapor identity.
* The Superworkspace location is local configuration, not an ID segment.
* Authored source lives outside the disposable Steam depot boundary.
* Authority → Source Repo Container → Source Repo → Project is the canonical source hierarchy.
* One Source Repo equals one Vapor Workspace.
* Projects are not Git repositories.
* Container creation does not implicitly create Source Repos.
* Source Repo creation does not implicitly create Projects.
* Typed Project creation does not silently create missing parent topology.
* Source Repo Container is the default atomic acquisition unit.
* Source Repo independent acquisition is opt-in.
* Vapor acquisition accepts registered Vapor source, not arbitrary Git repositories.
* Plain Git remains valid and inspectable.
* Vapor need not have performed the clone for valid source to be recognized.
* Git provider identity is distinct from Vapor identity.
* First-party status is trusted Registry/Root-Authority state.
* Third-party source cannot self-grant bespoke root Vapor semantics.
* Operation recipes should remain lightweight and inspectable.
* Persistent Context affects resolution, not operation scope.
* Explicit selection affects operation scope.
* CWD is not ordinary Vapor semantic Context.
* Dirty Git source is legitimate development state.
* Repair must not reset authored Git state or reacquire authored source.
* Missing submodule checkouts are Git reconciliation problems.
* Authored and derived state must remain distinguishable.
* Derived state should be safely regeneratable.
* External IDE integration should be derived/reconcilable where possible.
* Vapor Project topology and Cargo topology remain distinct.
* Failed rebuilds must not destroy prior valid installed builds.
* Local development should remain possible without production authority.
* Steam reinstall/downgrade must not silently delete authored source.

---

# Open Development Questions

The following remain intentionally open:

* Exact Superworkspace configuration/relocation command and persisted schema.
* Exact Vapor User Data root layout on each operating system.
* Exact Source Repo Container purpose/kind taxonomy.
* Exact relationship between first-party trust and Container facility classification.
* Exact provider UX for repository creation and visibility.
* Exact collaboration/fork workflow.
* Exact independently-acquirable Source Repo policy representation.
* Exact Project filesystem/manifest layout.
* Exact typed Project templates.
* Exact test/run Packagepack context model.
* Exact authored operation-recipe schema.
* Exact boundary between declarative operation recipes and checked-in scripts.
* Exact local Platform Server deployment mechanism.
* Exact future generalization of `deploy vps`.
* Exact source publication/version locking behavior.
* Exact IDE/debugger integration beyond currently supported RustRover reconciliation.
* Exact generated-state directory layout.
* Exact automated test taxonomy across Project, Source Repo, Container, and first-party facility scopes.
* Exact Git wrapper/reconciliation commands Vapor should expose in addition to diagnostics.
