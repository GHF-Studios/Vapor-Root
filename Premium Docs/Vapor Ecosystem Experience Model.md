> [!info]
> This document defines the high-level **User Experience (UX)** and **Developer Experience (DX)** of Vapor.
>
> Here, **ecosystem** describes Vapor as a complete product/content/development environment.
>
> It does not imply one generic modeled `Ecosystem` object or universal `vapor ecosystem ...` lifecycle.
>
> This document answers:
>
> * Who uses Vapor?
> * What are they trying to accomplish?
> * What concepts should they encounter?
> * What complexity should Vapor expose or hide?
> * How should the product change as installed Role increases?
> * How should GUI, CLI, Git, Cargo, Steam, and provider reality fit together?
>
> Detailed operational state belongs in the **Vapor Ecosystem Operational Model**.
>
> Canonical source identity, Context, Selection, and the Superworkspace belong in the **Vapor Context, Identity, Session And Selection Model**.
>
> Detailed authoring/source workflows belong in the **Vapor Development Experience Model**.
>
> Publication belongs in the **Vapor Publishing And Distribution Model**.
>
> Exact graphical SDK organization belongs in the **Vapor SDK Experience Model**.

---

# Purpose

Vapor exists to make a complex combination of:

```text
Rust
Cargo
Git
Git providers
Steam
SteamCMD
Steam Workshop
Registry services
build tooling
generated realization
Vapor Content
Vapor Apps
```

behave like one coherent product.

Users should primarily interact with **Vapor concepts** when performing Vapor work.

The underlying systems remain real.

They should remain visible and inspectable when useful.

The governing principle is:

> **Expose the work the user intends to perform; orchestrate the incidental infrastructure required to make that work possible.**

Vapor is therefore neither:

```text
a thin GUI over Cargo

nor

a replacement for Git/Rust/Steam

nor

a black box hiding every implementation detail
```

It is the semantic and orchestration layer connecting those systems.

---

# Role Experience Summary

At the highest level:

```text
Player
    consumes complete Vapor Apps

Composer
    assembles complete compositions from existing Content

Content Developer
    creates/modifies behavioral and reusable Content

Ecosystem Developer
    develops Vapor itself
```

These are installed local Roles.

Separately:

```text
Root Authority
    controls protected official Vapor authority
```

Root Authority is not another installed Role.

---

# Core Experience Principles

## Steam-First Player Experience

The normal Player begins in Steam.

Playing Loo Cast must not require:

```text
GitHub
Git
Rust
Cargo
SteamCMD
source repositories
source dependency resolution
local compilation
manual configuration
```

The default Steam installation behaves like a normal playable product.

---

# Progressive Disclosure

Vapor reveals larger mental models only when they become useful.

Conceptually:

```text
Player
    Vapor Apps

Composer
    + Content
    + packs
    + composition
    + source
    + dependencies
    + builds

Content Developer
    + Projects
    + implementation source
    + Git
    + Cargo/Rust
    + SDK
    + test/debug/diagnostics

Ecosystem Developer
    + Vapor Client source
    + Platform Server source
    + first-party tooling
    + infrastructure
    + deployment
    + Registry/identity systems
```

Higher Role should feel substantially more capable.

It should not merely reveal a few additional buttons.

---

# Golden Paths

Vapor should provide strong normal workflows for common tasks.

A Composer should be able to request:

```text
Build this Packagepack.
```

rather than manually performing:

```text
find every repository
→ determine exact versions
→ acquire source
→ configure Rust
→ configure Cargo
→ generate glue
→ invoke Cargo correctly
→ locate artifacts
→ package artifacts
→ register the App
→ select the correct executable
```

Those underlying actions may still occur.

Vapor orchestrates them.

---

# Advanced Transparency

Abstraction must not require opacity.

As Roles increase, Vapor should expose increasingly rich underlying state such as:

```text
canonical Vapor identities
Git repository/provider information
Git status
commit hashes
Cargo output
Cargo packages/targets
toolchain state
build logs
dependency resolution
generated realization
operation recipes
provider-native identifiers
deployment output
```

A user should be able to understand what Vapor did.

---

# Reuse Underlying Tools

Where Git, Cargo, Rust, Steam, or a provider already has a meaningful concept, Vapor should preserve that concept rather than inventing a weaker replacement.

Examples:

```text
Git branch
    stays Git branch

Cargo package
    stays Cargo package

Steam Workshop Item ID
    stays Steam provider identity
```

Vapor adds semantic context around those systems.

It does not need to rename everything.

---

# Human-Readable Identity

Users should primarily see canonical Vapor identities and names.

For source:

```text
Authority
/ Source Repo Container
/ Source Repo
/ Project
```

Example:

```text
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

Provider-native information remains available for diagnostics and advanced workflows.

---

# Minimal Sufficient Input

Users should not be forced to type a full canonical identity when Vapor already has enough information to resolve a shorter selector safely.

The experience principle is:

> **Ask for no more information than is needed to resolve intent unambiguously.**

Full canonical identity remains available and exact.

Shorter selectors are convenience.

Ambiguity is never guessed.

---

# Ambiguity Should Teach

When input matches multiple objects, Vapor should explain the topology.

Example:

```text
error: Project `core` is ambiguous within Vapor Platform Server

matches:

    Registry/core
    Identity/core
    Diagnostics/core

help:
    select one explicitly
```

Where useful, GUI and CLI should additionally offer:

```text
stronger selector

persistent Context

transient Context override
```

as safe ways to disambiguate.

---

# Diagnostics Are Product UX

Errors are not merely developer logging.

Vapor should explain:

```text
what was resolved

what could not be resolved

which candidates exist

which invariant failed

what state is missing

which provider is involved

which Role is required

which authority is missing

which safe operations can resolve the issue
```

This is especially important because Vapor should remain approachable to users who may also be learning:

```text
Git
Rust
Cargo
software development
game development
```

---

# Role Model

Installed Roles are cumulative:

```text
Player
⊂ Composer
⊂ Content Developer
⊂ Ecosystem Developer
```

A Content Developer is also a Composer and Player.

An Ecosystem Developer is also a Content Developer.

---

# Root Authority Is Orthogonal

Do not model:

```text
Player
→ Composer
→ Content Developer
→ Ecosystem Developer
→ Root Authority
```

Instead:

```text
local Role:
    Ecosystem Developer

authority:
    possibly Root Authority
```

Root Authority answers:

> What protected official Vapor resources may this identity control?

Role answers:

> What is this local Vapor environment equipped to do?

---

# Local Capability Without Remote Authentication

A user may have:

```text
Content Developer Role

managed toolchain ready

source available

GitHub logged out
```

and still legitimately:

```text
edit
compose
build
test
run
```

Only remote-provider operations should become unavailable.

Examples:

```text
push
publish
create remote repository
deploy protected infrastructure
```

may require authentication/authorization.

---

# Steam App

There is one Steam-distributed outer product:

```text
Loo Cast
```

This is the Steam App.

It provides the ordinary entry point into:

```text
Vapor Client
default Loo Cast Vapor App
Vapor Installer
additional Vapor Apps
higher installed Vapor Roles
```

Steam App is not synonymous with one Vapor App Composition.

---

# Steam App Instance

A **Steam App Instance** is one concrete Steam installation of Loo Cast.

Normal model:

```text
one Steam installation
=
one Steam App Instance
```

The Steam App Instance is primarily a product/distribution boundary.

It does not physically own the entire mutable Vapor environment.

---

# Local Storage Experience

The local Vapor environment should conceptually distinguish:

```text
Steam App Instance

Vapor User Data

canonical Superworkspace
```

## Steam App Instance

Primarily:

```text
Steam/depot-owned replaceable files
Vapor binaries
bootstrap
default shipped Vapor App
```

## Vapor User Data

May contain:

```text
Role state
configured Superworkspace location
persistent Context
managed toolchain state
indexes
caches
IDE integration
selected/default Vapor App
local App metadata
resume state
```

## Superworkspace

Contains authored development source.

This distinction must be reflected in uninstall/reinstall/recovery UX.

---

# Steam Reinstall Experience

Reinstalling through Steam may replace product files.

It must not imply:

```text
delete canonical Superworkspace
delete authored Git source
delete unpushed commits
```

After reinstall, Vapor should reconnect to appropriate persisted/recoverable local state and regenerate derived state where necessary.

---

# Packagepack

A **Packagepack** is the complete authored composition Project.

A valid Packagepack resolves to:

```text
exactly one effective Engine
exactly one effective Game
selected compatible Mods
Libraries/support dependencies
subordinate packs/dependencies
```

It is not merely a precursor to another authored “finished composition” entity.

---

# Vapor App Composition

Resolving a Packagepack produces its **Vapor App Composition**:

```text
Packagepack
    ↓ resolve
exact effective Content graph
```

The resolved composition is semantic state.

---

# Vapor App

Building a Vapor App Composition produces a **Vapor App**:

```text
Packagepack
    ↓ resolve
Vapor App Composition
    ↓ build
Vapor App
```

A Vapor App is a target-specific runnable realization.

Multiple Vapor Apps may coexist locally.

---

# Selected / Default Vapor App

The Steam App Instance may remember one selected/default Vapor App for:

```text
Play
direct launch
ordinary Launcher convenience
```

This concept is completely separate from:

```text
persistent development Context

operation Selection

GUI tree selection
```

The word “selected” should therefore always be qualified where ambiguity is possible.

---

# Player Experience

A Player primarily wants to:

```text
install Loo Cast

play default Loo Cast

discover additional Vapor Apps

acquire/install Apps

choose an App

launch it

remove it

manage settings/accounts
```

A Player should not need to understand:

```text
Authority
Source Repo Container
Source Repo
Project
Git
Cargo
Rust
composition builds
source versions
```

unless they deliberately inspect advanced information.

---

# Player Discovery Unit

The normal Player-facing discovery unit is a complete Vapor App.

Individual:

```text
Engine
Game
Mod
Library
Enginepack
Gamepack
Modpack
```

are not ordinary Player composition objects because Player Role does not author compositions.

---

# Player Acquisition

Player-facing acquisition should feel approximately like:

```text
find App
→ acquire
→ install
→ launch
```

Provider-native Steam Workshop details should generally remain beneath the Vapor UX.

The Player does not compile.

---

# Default Loo Cast Experience

The default first-party Packagepack is:

```text
Loo Cast Packagepack
```

It resolves to at least:

```text
Spacetime Engine
Loo Cast Game
```

plus required dependencies.

Its built Vapor App ships directly in the Steam depot.

First launch must not require Steam Workshop.

---

# Steam Entry Points

The Steam product should expose three conceptual launch choices:

```text
Play Loo Cast

Start Vapor

Start Installer
```

---

# Play Loo Cast

`Play Loo Cast` directly enters the default first-party Vapor App path.

No:

```text
source acquisition
local build
Rust installation
```

is required.

---

# Start Vapor

`Start Vapor` opens the Vapor Launcher.

This is the normal explicit entry into Vapor's management/discovery/composition/development surfaces.

---

# Start Installer

`Start Installer` opens the Vapor Installer.

This is used to:

```text
establish higher Roles

change installed tooling

repair capability state

downgrade capability

perform setup/recovery tasks
```

---

# Composer Experience

A Composer creates and modifies composition-oriented Content.

Composer-authored kinds include:

```text
Packagepack
Enginepack
Gamepack
Modpack
```

Composer consumes existing:

```text
Engine
Game
Mods
Libraries
other packs
```

---

# Composer Mental Model

The Composer primarily reasons about:

```text
Content
dependencies
compatibility
packs
Packagepack composition
versions
resolution
build
Vapor App
publication
```

Not:

```text
manual Cargo workspace wiring
manual generated glue
manual toolchain environment setup
```

---

# Composer Source Exposure

Composer Role introduces enough source concepts to support authored packs.

This includes:

```text
registered source
canonical identities
source acquisition
Git-backed authored state
Projects
versioning/publication
```

The Composer may not need the same depth of implementation-source exposure as Content Developer.

---

# Composer Journey

A typical Composer journey may be:

```text
Enter Development

→ navigate/create/acquire relevant source

→ create/open Packagepack Project

→ choose Engine/Game/Mods/Libraries

→ resolve composition

→ inspect conflicts

→ build

→ install/register local Vapor App

→ run/test

→ revise

→ optionally publish
```

The exact frontend interactions may differ between GUI and CLI.

---

# Content Developer Experience

A Content Developer creates or modifies behavioral/reusable Content.

Examples:

```text
Engine
Game
Engine Mod
Game Mod
Extension Mod
Library
```

Specializations such as:

```text
Engine Developer
Game Developer
Mod Developer
Library Developer
```

are not separate fundamental installed Roles.

---

# Content Developer Mental Model

The developer now legitimately encounters:

```text
canonical Superworkspace

Authorities

Source Repo Containers

Source Repos / Vapor Workspaces

Projects

Content kind

Git repositories

Rust/Cargo realization

source implementation

dependencies

build/test/run

diagnostics

SDK

publication
```

---

# One Canonical Superworkspace

The developer sees one canonical local Superworkspace.

Not:

```text
multiple active Superworkspaces

implicit Superworkspace

configured Superworkspace

temporary Superworkspace
```

as competing normal models.

Conceptually:

```text
Superworkspace
└── Authorities
    └── Source Repo Containers
        └── Source Repos
            └── Projects
```

The Superworkspace itself has no Vapor identity.

---

# Source Repo = Vapor Workspace

The GUI may present both terms according to context:

```text
Source Repo
    emphasizes Git/source repository

Vapor Workspace
    emphasizes Vapor development context
```

They are the same hierarchy node.

They must not appear as two nested structural layers.

---

# Project Experience

A Project is the main Vapor development unit inside a Source Repo.

The developer may encounter deeper implementation detail such as:

```text
Cargo workspace
Cargo packages
crates
targets
files
modules
tests
```

beneath/inside the Project.

Vapor should keep these distinctions visible.

---

# Content Project Experience

For Content Projects:

```text
Project identity
=
Content identity
```

with:

```text
Project kind
=
Game / Engine / Library / Packagepack / ...
```

The UI should not present a second independent Content object nested beneath the Project merely to carry identity.

---

# Content Developer Goal

The intended experience is:

> **Develop the Content, not the machinery required to persuade Rust, Cargo, Git, Steam, and Vapor to cooperate.**

Vapor should provide:

```text
strong identity
minimal boilerplate
structural guardrails
managed environment
Vapor-aware build/test/run
specific diagnostics
Git/Cargo transparency
graph/relationship inspection
```

---

# External IDE Experience

Vapor SDK is the first-party integrated development environment.

External IDEs such as RustRover remain valuable.

The relationship should be complementary:

```text
Vapor
    owns semantic topology
    managed tooling
    operations
    generated integration

RustRover
    editing
    Rust language intelligence
    refactoring
    debugging
    Git UI
```

A user should be able to use either without Vapor pretending the external IDE does not exist.

---

# Ecosystem Developer Experience

An Ecosystem Developer develops Vapor itself.

This may include:

```text
Vapor Client

Vapor Core

Launcher

SDK

Installer

CLI

managed toolchain

Platform Server

Registry

Identity

Diagnostics

documentation infrastructure

deployment

provider integration
```

---

# First-Party Development Subjects

Ecosystem Developer should encounter concrete first-party facilities rather than a generic editable `Ecosystem` object.

Primary current subjects include:

```text
Client

Platform Server

Examples
```

Examples of operations:

```text
client build

client test

platform-server test

platform-server deploy local

examples acquire
```

---

# Acquiring First-Party Source

The normal first-party flow may use stable shortcuts:

```text
client acquire

platform-server acquire

examples acquire
```

These resolve trusted registered source.

The generic source topology remains available:

```text
source-repo-container acquire ...
```

There is no need for:

```text
ecosystem acquire
```

as a generic umbrella operation.

---

# First-Party Source Remains Git

Developing Vapor should still expose:

```text
Git repositories
submodules
commits
branches
remotes
dirty state
provider links
```

Vapor should orchestrate first-party workflows without turning its own source into an opaque proprietary storage system.

---

# Ecosystem Developer Journey

A representative journey:

```text
Enter Development

→ navigate/acquire first-party source

→ inspect Client / Platform Server / Examples

→ choose work

→ modify source

→ build/test relevant subject

→ inspect diagnostics

→ commit/push where authorized

→ deploy local where relevant

→ deploy protected target where authorized

→ validate
```

No generic “whole ecosystem build/test/deploy” is implied.

---

# Local First-Party Development Without Root Authority

An Ecosystem Developer should be able to:

```text
acquire first-party source

edit

build

test

run local services

perform local deployment

inspect production-oriented recipes
```

where technically feasible.

They may still be denied:

```text
official repository push

Steam production deployment

VPS production deployment

Registry trust mutation
```

Those are authority concerns.

---

# Root Authority Experience

Root Authority normally uses an Ecosystem Developer environment.

It additionally has protected authorization.

Examples:

```text
official repository administration
official namespace publication
first-party trust
production deployment
Registry administration
protected recovery
ownership/authorization changes
```

Root Authority does not require a completely separate local IDE/product mode merely because authorization is stronger.

---

# Development Navigation Model

The development UX should follow the canonical resolution model:

```text
Context
    → operation subject
    → Selection
```

Conceptually:

```text
left
    Context

middle
    operation subject

right
    Selection
```

---

# Persistent Context

Persistent Context is a lightweight semantic navigation/resolution prefix.

It should feel approximately like:

```text
semantic cd
```

without using shell CWD itself.

A developer may navigate to:

```text
GHF-Studios/Loo-Cast/Game
```

so that:

```text
Loo-Cast
```

resolves naturally beneath it.

---

# Context Is Optional

Vapor must never require users to establish persistent Context merely to address something.

Users may always provide:

```text
full Vapor paths

stronger selectors

transient Context overrides
```

where appropriate.

Persistent Context is convenience.

---

# Transient Context

A GUI or CLI operation may provide a temporary resolution prefix without changing persistent navigation state.

This supports:

> “I am currently working here, but perform this one operation over there.”

That capability should be available in automation too.

---

# CWD Is Not Vapor Navigation State

Changing terminal directory does not silently alter Vapor semantic Context.

Likewise, opening a file through an external editor does not automatically change operation scope.

The developer should be able to stand physically anywhere and still target Vapor objects explicitly.

---

# Context Is Not Operation Scope

Suppose persistent Context is:

```text
GHF-Studios/Vapor-Platform-Server/Registry
```

Then:

```text
platform-server test
```

still means:

```text
test the natural complete Platform Server subject
```

not:

```text
test only Registry
```

Only explicit operation Selection narrows scope.

---

# GUI Navigation vs Persistent Context

The SDK may have:

```text
expanded tree nodes

active editor tabs

selected UI rows

navigation history
```

without treating every UI detail as persistent Vapor Context.

Persistent Context should change only through a deliberate semantic navigation action.

---

# No Open-Object Working Set Model

The old concept:

```text
many Workspaces Open
+
many Projects Focused
```

is no longer the canonical target-resolution model.

The SDK may still visually have:

```text
many tabs
many expanded trees
many documents
many windows
```

That is frontend/session state.

It does not create a durable semantic working-set hierarchy inside Vapor Core.

---

# No Durable Focus Set

Vapor should not maintain several simultaneously Focused Projects as hidden default operation targets.

This creates too much ambient operation scope.

Instead:

```text
persistent Context
    resolves names

explicit Selection
    chooses operation targets
```

That model is easier to inspect, automate, and reason about.

---

# GUI Selection

A GUI may temporarily select:

```text
one Project

several Projects

one Source Repo

several subtree roots
```

for an operation.

That transient GUI selection can become explicit operation Selection.

It does not need to become persistent Context.

---

# Multiple Selection

Where the operation supports multiple targets, the GUI should make this natural.

For example:

```text
select Registry
select Identity

→ Test Selected
```

may construct equivalent Core input to:

```text
--select "Registry, Identity"
```

The GUI should not force the user to think in CLI syntax.

The semantics are shared.

---

# Hierarchical Selection

Selecting a parent topology node means selecting that meaningful subtree/scope for the operation.

Example:

```text
Registry Source Repo selected
```

should not require additionally selecting all Projects beneath it merely to indicate Source Repo scope.

The operation decides what Source Repo-scoped testing/building means.

---

# Natural Complete Scope

No explicit operation Selection means:

> perform the operation on its natural complete subject.

For example:

```text
Platform Server
→ Test
```

means:

> run the complete Platform Server test operation.

It does not mean:

> mechanically iterate every descendant Project with a generic test command.

---

# Operation-Specific Legality

GUI operations should be enabled based on actual operation semantics.

For example:

```text
Platform Server test
    Source Repo Selection supported
```

but:

```text
Platform Server deploy vps
    partial Selection unsupported
```

The UI should explain why.

It should not simply gray out controls without explanation.

---

# Disabled Operation UX

A disabled operation should be inspectable.

For example:

```text
Deploy to VPS
    unavailable

Reason:
    production deployment requires complete Platform Server scope

Current Selection:
    Registry

Available:
    Deploy Local
```

This turns operation constraints into discoverable product knowledge.

---

# Ambiguous Selection UX

If a typed selector or search query is ambiguous, the GUI should show candidates.

Example:

```text
core
```

could resolve to:

```text
Registry/core
Identity/core
Diagnostics/core
```

The UI should let the user choose one or refine Context/qualification.

It must not silently pick the first result.

---

# Broken Source Experience

Development objects should remain understandable when local source is unhealthy.

Examples:

```text
Source Repo Container:
    registered
    not locally available

Source Repo:
    declared by Container
    submodule checkout missing

Project:
    known from metadata
    local files currently unavailable

derived IDE state:
    stale
```

These are different failures.

---

# Missing Container UX

If a registered Container is not local:

```text
GHF-Studios/Loo-Cast
    Not available locally
```

reasonable actions may include:

```text
Acquire
Inspect Registry
Inspect Provider
```

where permitted.

---

# Missing Source Repo Checkout UX

If the Container is local but a declared submodule checkout is missing:

```text
Game
    Git submodule checkout missing
```

the UX should present it as Git topology/reconciliation.

Not:

```text
Reacquire Source Repo
```

unless the Source Repo is independently acquirable and that is genuinely the intended operation.

---

# Dirty Source UX

Dirty Git state should be visible.

It should not automatically be presented as a fault.

Examples:

```text
Modified

Uncommitted changes

Local commits not pushed

Detached HEAD

Differs from parent gitlink
```

These may be ordinary development states.

---

# Repair UX

Repair should appear only where Vapor knows it can reconstruct derived state safely.

Examples:

```text
Regenerate IDE integration

Rebuild indexes

Regenerate derived operation state
```

Repair should not be a generic magic button for:

```text
dirty Git

missing authored source

deleted unique source

branch conflicts
```

---

# Git Reconciliation UX

When Vapor detects a Git-state issue, it should expose:

```text
what Git state exists

what Vapor expected

possible inspection commands

possible reconciliation action

risk/caution
```

Example:

```text
Source Repo `Registry`
    declared as submodule
    checkout missing

Inspect:
    git status
    git submodule status

Possible reconciliation:
    git submodule update --init -- Registry
```

with a warning if Vapor cannot prove it is safe.

---

# Vapor Applications

## Vapor Installer

The Installer changes what the local Vapor environment is equipped to do.

Its responsibility is:

> **Change installed Role/tooling capability.**

Examples:

```text
install Composer capability

install developer toolchain

establish Content Developer

establish Ecosystem Developer

downgrade capability

repair installed capability
```

Installer should not own ordinary development-source editing.

---

# Vapor Launcher

The Launcher is the main ordinary desktop surface.

It may expose:

```text
Play

Vapor Apps

Content Library

Discovery

Composition

Accounts

Settings

Diagnostics

Logs

Publication/management
```

according to installed Role.

---

# Enter Development

Where Composer-or-higher development capability exists, Launcher should expose a clear transition into Development Mode.

Conceptually:

```text
Launcher
    ↓ Enter Development
SDK / Development Mode
```

The exact process/window mechanics are implementation detail.

---

# Vapor SDK

The Vapor SDK is the integrated first-party Development Mode.

It should expose the richer source/development model appropriate to the Role.

Examples:

```text
Superworkspace Explorer

Authorities

Source Repo Containers

Source Repos

Projects

source editor

Content editors

dependency/composition graphs

Inspector

Problems

Build

Test

Run

Git

Cargo

toolchain

logs

publication

first-party operations
```

---

# SDK Superset Principle

Conceptually:

```text
Launcher capability surface
    ⊂
SDK / Development surface
```

This means shared product semantics.

It does not require every Launcher control to remain permanently visible while coding.

Installer remains separate because Installer modifies installed capability itself.

---

# SDK-First Rewrite Strategy

Implementing the richer SDK before the polished simplified Launcher can be reasonable.

SDK exercises more Core behavior:

```text
identity

source topology

Context

Selection

Git

Cargo/toolchain

Content

build/test/run

diagnostics

publication

first-party development
```

A mature Launcher can later project a smaller portion of the same backend.

---

# SDK Visual Character

The Figma-derived historical Vapor SDK prototype remains a visual ancestor.

Its fake data and obsolete semantics are not normative.

Useful qualities include:

```text
JetBrains/RustRover-like density

dark neutral surfaces

subtle separators

compact toolbars

tight readable spacing

clear tree selection

restrained corner radii

semantic status indicators

monospace technical output

central work area

left Explorer

right Inspector

bottom Problems/Build/tool windows

graph visualization
```

These qualities should become a coherent Vapor design system rather than remain tied to one generated prototype.

---

# SDK Interaction Model

The graphical SDK should use GUI-native interaction:

```text
tree navigation

tabs

multi-selection

context menus

Inspector actions

dialogs

graphs

drag/drop where semantically meaningful

rich diagnostics

inline recovery controls
```

Those interactions construct the same Core operations available to CLI/automation.

---

# GUI and CLI Equality

The principle is:

> **GUI and CLI share semantics and capabilities, not interaction mechanics.**

The GUI may use:

```text
click
selection
tree navigation
dialog
graph
```

where the CLI uses:

```text
canonical path
selector
--context
--select
```

Both resolve exact Core inputs.

---

# CLI Experience

The CLI should be especially strong for:

```text
developers

automation

coding agents

scripts

advanced users
```

Its strengths should include:

```text
canonical identity

minimal unambiguous selectors

explicit Context override

explicit Selection

predictable operation grammar

specific diagnostics

machine-readable output

stable exit/failure semantics
```

---

# Coding-Agent Experience

Coding agents should be able to use Vapor without reverse-engineering GUI state.

Operations should be expressible explicitly through:

```text
canonical paths

transient Context

explicit Selection

deterministic commands

machine-readable diagnostics

Git-visible source
```

This is another reason not to make persistent GUI Focus state semantically required.

---

# Plain Git Experience

Advanced users should be able to use Git normally.

Examples:

```text
git status

git diff

git commit

git switch

git rebase

git submodule ...
```

Vapor should understand the resulting valid state rather than requiring that every source mutation originated inside Vapor.

---

# Plain Cargo Experience

Developers may also invoke Cargo directly where useful.

Vapor-managed Cargo exists to provide:

```text
correct managed toolchain

appropriate environment

Vapor-aware Project resolution
```

while retaining normal Cargo semantics.

---

# Managed Toolchain Experience

Developer Roles should not require manually assembling the exact Rust/Cargo environment expected by Vapor.

Vapor should manage:

```text
version

installation/detection

environment

diagnostics

repair
```

Root/first-party development should strongly prefer the pinned/vendored Vapor-managed toolchain.

---

# Tool Failure UX

If the managed toolchain is unhealthy, Vapor should say what is wrong.

Examples:

```text
Rust toolchain missing

Cargo unavailable

configured toolchain path missing

toolchain version incompatible

managed environment incomplete
```

and offer safe repair where possible.

---

# Build Experience

Users ask Vapor to build semantic subjects.

Examples:

```text
Build Packagepack

Build Project

Build Client

Build Platform Server
```

The resulting operation may involve several Cargo/build steps.

The UX should emphasize:

```text
subject

current step

diagnostics

artifacts

success/failure

what remains runnable
```

---

# Failed Build Experience

A failed rebuild must not imply the previous working App vanished.

The UI may show:

```text
Source:
    Modified

Latest Build:
    Failed

Installed App:
    Loo Cast 0.4.1
    Still Runnable
```

This is more truthful than one global red “broken” state.

---

# Test Experience

Testing should be semantic to the subject.

Examples:

```text
Test Project

Test Source Repo scope

Test Client

Test Platform Server
```

The UI should make clear:

```text
what is being tested

what Selection is applied

which recipe/steps ran

which tests failed

what scope is required
```

---

# Run Experience

Running development Content may require a complete composition.

The SDK should help resolve that test/run composition rather than pretending all Projects are independently runnable.

Possible sources include:

```text
explicit Packagepack

Project-preferred development Packagepack

generated development composition
```

Exact mechanics remain open.

---

# Publication Experience

Publishing should feel like one coherent Vapor workflow while still exposing meaningful stages.

For Content:

```text
validate

choose SemVer

verify source commit

publish source version

Registry registration
```

For Packagepack built publication:

```text
resolve exact composition

build targets

validate artifacts

publish distribution

Registry linkage
```

Partial failures should remain diagnosable/retryable.

---

# Version UX

Published versions are immutable SemVer releases.

The UI should make this explicit.

Example:

```text
Version 1.4.2
    Published
    Commit abc123...
    Immutable
```

A modified working tree after publication represents new development.

It does not modify `1.4.2`.

---

# Yank / Ban UX

Yank and Ban should be visibly distinct from deletion.

A user inspecting an old release should still understand:

```text
what it was

which source produced it

why it is unavailable/restricted

whether an alternative exists
```

Historical identity should not disappear.

---

# First-Party Client Experience

The SDK may expose a dedicated `Client` development surface.

It may show:

```text
source status

build

test

local deployment

Steam deployment where authorized

operation output

current backing source
```

This is a stable first-party facility surface.

It need not mirror the exact underlying repository structure.

---

# Platform Server Experience

Likewise, Platform Server may have a dedicated development/operations surface.

It may show:

```text
services

Source Repos

health

build/test

local deployment

VPS deployment

logs

service status

operation recipes
```

Selection may allow testing/deploying meaningful subscopes where legal.

---

# Platform Service UX

Individual services such as:

```text
Registry
Identity
Diagnostics
Docs
Homepage
```

should remain inspectable as meaningful units.

Platform Server whole-operation semantics should coexist with service-level detail.

---

# Examples Experience

Official Examples should be easy to acquire and browse.

They should teach/prove:

```text
Project kinds

composition

dependencies

Cargo realization

build/test workflows

architecture contracts
```

The stable Examples facility should survive source-topology changes.

---

# Progressive Transparency

A useful default progression:

## Player

Primarily:

```text
Apps
Play
Discovery
Install
Settings
```

## Composer

Additionally:

```text
Content
packs
source
dependencies
composition
build
publication
```

## Content Developer

Additionally:

```text
Superworkspace
Source Repo Containers
Source Repos
Projects
implementation source
Git
Cargo
toolchain
tests
diagnostics
```

## Ecosystem Developer

Additionally:

```text
Client
Platform Server
Examples
first-party source
services
deployment
Registry
toolchain internals
provider integrations
```

---

# Terminology in the UI

Prefer exact current terms:

```text
Source Repo Container

Source Repo

Project

Context

Selection

Vapor App

Packagepack

Platform Server
```

Avoid resurfacing legacy terms such as:

```text
Vapor Root

Server Root

generic Ecosystem object

generic Source object

Focused Project set

Implicit Superworkspace

Structural Address
```

---

# “Source” in User-Facing Copy

The generic word **source** remains useful in prose.

Example:

```text
Source is not available locally.
```

But when the UI refers to a modeled topology object, it should say which one:

```text
Source Repo Container

Source Repo

Project
```

instead of exposing an ambiguous object called merely `Source`.

---

# “Workspace” in User-Facing Copy

`Workspace` may still be used where the **Vapor Workspace** development perspective is useful.

But the structural tree must not show:

```text
Source Repo
    └── Workspace
```

because those are the same object.

---

# First-Time Developer Experience

When upgrading to Composer/Content Developer, Vapor should explain the new model progressively.

Do not immediately require the user to understand every layer.

A possible progression:

```text
Choose/Create something to develop

→ Vapor shows where it belongs

→ introduce Project

→ introduce Source Repo when repository behavior matters

→ introduce Container when acquisition/organization matters

→ expose Authority when global identity/provider ownership matters
```

The canonical model remains precise even if UI teaching is progressive.

---

# Create Experience

Creation should make parent structure explicit.

Example:

```text
Create Game
```

If required Source Repo does not exist, Vapor should explain that rather than silently invent it.

GUI may offer a guided sequence:

```text
This Game requires a Source Repo.

Create:
    GHF-Studios/My-Stuff/Game
```

then perform the separate explicit operation if the user chooses it.

The underlying model remains:

```text
Container creation
≠ Source Repo creation
≠ Project creation
```

---

# Acquire Experience

A user normally acquires a Source Repo Container.

Example GUI:

```text
Acquire GHF-Studios/Loo-Cast

Contains:
    Game
    Engine
    ...
```

The canonical Superworkspace destination is automatic.

No arbitrary destination chooser is needed for ordinary acquisition.

---

# Independent Source Repo Acquisition UX

If a Source Repo is explicitly independently acquirable, the UI may expose that.

Otherwise:

```text
Acquire Source Repo
```

should explain that the Repo belongs to a Container and offer:

```text
Acquire GHF-Studios/Foo
```

instead.

---

# Superworkspace Configuration UX

The Superworkspace root may be configurable.

This should be one environment-level setting.

It should not appear as a destination field on every create/acquire dialog.

Changing it should be treated as a deliberate relocation/configuration operation.

---

# Recovery UX

Vapor should distinguish:

```text
Steam product missing

Vapor User Data missing/stale

registered source missing

Git topology unhealthy

derived state missing

unique authored source potentially lost
```

Different problems require different recovery actions.

A single generic:

```text
Repair Everything
```

should not conceal destructive uncertainty.

---

# Safety Before Convenience

Where Vapor cannot prove a corrective action is safe, it should say so.

For example:

```text
Possible Git reconciliation:
    git submodule update --init -- Registry

This modifies local Git checkout state.
```

Transparency is preferable to false confidence.

---

# Derived-State UX

Derived state may support convenient:

```text
Regenerate
Repair
Refresh
Reindex
```

operations.

Those operations should be safe because the state is reconstructible.

The same language should not be applied indiscriminately to authored source.

---

# Destructive Operations

Operations which may delete or overwrite user-authored state should:

```text
identify exactly what is affected

show whether remote recovery exists

show whether local-only work exists

refuse where safety cannot be established

require explicit intent where necessary
```

Vapor should not optimize UX by hiding the difference between cache and source.

---

# Static Composition Experience

A Vapor App is one statically resolved complete composition.

Changing effective composition normally requires rebuilding.

This does not mean every unchanged dependency physically recompiles.

Cargo/Vapor caching and incremental compilation optimize implementation work.

The semantic invariant is:

```text
one runnable Vapor App
=
one resolved composition
```

---

# Runtime Dynamicity

Static composition does not forbid dynamic runtime systems.

An Engine/Game may deliberately provide runtime extension/capability systems.

The distinction is:

```text
static composition
    determines what runtime exists

runtime dynamicity
    behavior intentionally supported inside that runtime
```

---

# Experience Guarantees

* Player use begins with a conventional Steam-ready product.
* Player Role requires neither Git nor Rust/Cargo.
* The default Loo Cast Vapor App ships through the Steam depot.
* Players primarily discover complete Vapor Apps.
* Composer exposes composition rather than implementation machinery.
* Content Developer exposes meaningful source/development machinery.
* Ecosystem Developer exposes Vapor itself through concrete first-party facilities.
* Root Authority is authority, not an installed Role.
* Local Role and remote authorization remain distinct.
* There is one canonical local Superworkspace.
* The Superworkspace is distinct from Steam App Instance and Vapor User Data.
* Source Repo equals Vapor Workspace.
* Project is the canonical Vapor development unit beneath Source Repo.
* Content identity is the Content Project identity.
* Persistent Context is optional resolution convenience.
* Context does not imply operation scope.
* CWD does not silently become Vapor Context.
* Explicit Selection defines per-operation narrowing.
* GUI tree selection may become Operation Selection without becoming persistent Context.
* There is no canonical durable Open/Focused object-set targeting model.
* No explicit Selection means natural complete operation scope.
* Ambiguity is never guessed.
* Diagnostics should explain safe disambiguation.
* Git remains visible and usable.
* Dirty Git state is not automatically broken state.
* Missing authored submodule checkout is Git reconciliation, not generic Repair.
* Repair is for safely derivable Vapor-owned state.
* A failed rebuild does not destroy a previous valid Vapor App.
* App selection, development Context, and Operation Selection are distinct.
* First-party Client/Platform Server/Examples facilities may remain stable while source topology evolves.
* GUI and CLI share Core semantics without requiring identical interaction mechanics.
* Coding agents can operate without hidden GUI state.
* Vapor automates incidental infrastructure while preserving advanced transparency.

---

# Open Experience Questions

The following remain intentionally open:

* Exact Launcher information architecture.
* Exact SDK information architecture.
* Exact initial onboarding for each Role.
* Exact persistent Context interaction in GUI.
* Exact visual affordance for changing Context versus transient Selection.
* Exact multi-selection presentation for hierarchical operation scopes.
* Exact guided create-parent UX.
* Exact Superworkspace relocation UI.
* Exact Git reconciliation UI and confirmation policy.
* Exact structured diagnostics presentation.
* Exact operation-progress/cancellation UX.
* Exact coding-agent integration UX.
* Exact test/run composition-selection UX.
* Exact Player version-selection/update UX.
* Exact publication wizard/staging UX.
* Exact Platform Server deployment/health UX.
* Exact treatment of provider authentication/account connections.
* Exact visual design-system implementation derived from the historical SDK prototype.
* Exact role of embedded terminal/shell functionality inside SDK.
