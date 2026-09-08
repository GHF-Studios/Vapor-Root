> [!info]
> This document defines the intended graphical Vapor SDK / Development Mode experience.
>
> The SDK is a Launcher-integrated development surface backed by Vapor Core.
>
> Canonical identity, Superworkspace, Context, Selection, and ambiguity semantics are defined by the **Vapor Context, Identity, Session And Selection Model**.
>
> Broader UX principles are defined by the **Vapor Ecosystem Experience Model**.
>
> Source/development workflows are defined by the **Vapor Development Experience Model**.
>
> Exact CLI grammar is defined by the **Vapor CLI Model**.

---

# Product Thesis

The Vapor SDK should be the primary first-party graphical environment for developing:

```text id="11soin"
Vapor Content

and, where Role permits,

Vapor itself
```

The intended experience is not:

```text id="fdh7gb"
open a launcher

→ leave Vapor

→ manually coordinate Git

→ manually coordinate Cargo/Rust

→ manually coordinate IDE configuration

→ manually reconstruct Vapor identities

→ manually invoke deployment/build systems
```

The intended experience is:

> **Enter Development and let Vapor become the Vapor-aware development environment.**

The SDK may integrate external editors and tools.

It should progressively own Vapor-specific workflow itself.

---

# Launcher → SDK

The preferred desktop product experience is:

```text id="hx6u01"
Launcher Mode
    ↕
SDK / Development Mode
```

Conceptually:

```text id="zijkg4"
Vapor Launcher
    ↓ Enter Development
Vapor SDK
    ↑ Return to Launcher
```

The exact implementation may use:

```text id="84d2at"
one process

multiple processes

one window

multiple windows

dynamic shell reconfiguration
```

The semantic property is:

> **SDK Mode is a richer projection of the same Vapor product and Core, not an unrelated application with a second architecture.**

---

# SDK Superset Principle

Launcher Mode exposes ordinary installed-Role functionality.

Depending on Role:

```text id="w47i9u"
Vapor Apps

Content Library

composition

accounts

settings

diagnostics

publication/discovery
```

SDK Mode additionally exposes development capabilities such as:

```text id="g2wlxd"
canonical Superworkspace

source topology

Projects

source editing

Vapor manifests

Cargo/Rust realization

dependency/composition graphs

Inspector

Problems

Build

Test

Run

Git

toolchain

logs

publication

first-party development/deployment
```

SDK Mode should feel like Vapor unfolding into a professional development environment.

---

# Installer Boundary

Installer remains distinct.

Conceptually:

```text id="le7f5m"
Installer
    changes what the local environment is equipped to do

Launcher
    uses installed capability

SDK
    uses installed development capability
```

Installer remains responsible for things such as:

```text id="1bk6ln"
Role installation/removal

fundamental managed-tool provisioning

major environment provisioning

capability repair
```

SDK may inspect Installer/tooling health without inheriting all Installation-mutation semantics.

---

# Role Projection

SDK capability grows with Role.

## Composer

May see:

```text id="kyn9rd"
composition Projects

source

packs

dependencies

Packagepack resolution

build/run

publication
```

## Content Developer

Additionally:

```text id="9x59zo"
behavioral Projects

Rust/source implementation

Cargo

tests

code diagnostics

advanced Git
```

## Ecosystem Developer

Additionally:

```text id="h8dk9r"
Vapor Client

Vapor Platform Server

Examples

first-party source

service topology

managed tooling internals

deployment

Registry/provider infrastructure
```

Role controls local capability.

Authority controls protected remote operations.

---

# Build the SDK First

During the rewrite, the richer SDK may be implemented before the polished final Launcher.

This remains desirable because SDK exercises more of Vapor Core:

```text id="m9cspb"
identity

Context

Selection

source topology

Registry

toolchain

Cargo

Git

Content

composition

build

test

run

diagnostics

publication

deployment
```

The final Launcher can then become a reduced projection of an already-proven application/Core architecture.

---

# Visual Design Ancestor

The existing Figma-derived / Tauri-hosted SDK prototype remains the visual ancestor.

Its obsolete data model is not normative.

Its successful visual language should be preserved aggressively.

Desired characteristics include:

```text id="bt28kp"
JetBrains / RustRover-like professional density

dark neutral surfaces

subtle separators

compact controls

tight readable spacing

restrained corner radii

clear active/inactive panel distinction

high information density

semantic success/warning/error states

monospace identities/code/tool output

restrained animation

precise hover/selection states

tool-window lower panels

contextual Inspector

integrated graph visualization
```

The prototype is design ancestry.

It is not semantic authority.

---

# Design-System Extraction

The implementation should extract the prototype styling into a reusable Vapor design system.

It should eventually define:

```text id="bf14fi"
surface tokens

editor surfaces

panels

separators

tree rows

tabs

buttons

inputs

selectors

Inspector components

badges

diagnostics

status states

graph nodes/edges

tool windows

typography

spacing

radii

hover/focus/selection states
```

The exact frontend stack is implementation-specific.

Tauri/React is not a permanent semantic requirement.

---

# Primary Window Anatomy

The broad prototype anatomy remains excellent:

```text id="qk0pwe"
┌────────────────────────────────────────────────────────────┐
│ Menu / application chrome                                  │
├────────────────────────────────────────────────────────────┤
│ Context / subject / selection / toolchain / operations     │
├──────────────┬───────────────────────────┬─────────────────┤
│ Explorer     │ Main Work Area            │ Inspector       │
│              │                           │                 │
│              │ editor / manifest / graph │                 │
│              │                           │                 │
├──────────────┴───────────────────────────┴─────────────────┤
│ Problems / Build / Test / Terminal / Git / Steam / Tools  │
├────────────────────────────────────────────────────────────┤
│ Status                                                     │
└────────────────────────────────────────────────────────────┘
```

Exact sizing/docking may be configurable.

The conceptual roles should remain recognizable.

---

# No Superworkspace Selector

The canonical model has exactly one local Superworkspace.

Therefore the SDK should **not** present:

```text id="vu3atg"
Vapor Development
Loo Cast
Experiments

Choose Superworkspace...
```

as several normal parallel development universes.

Instead, the SDK should show the configured canonical Superworkspace.

Example:

```text id="c7iiwl"
Development Source

Superworkspace:
    /home/.../Vapor
```

The physical path should usually be secondary.

---

# Superworkspace Configuration

Changing the Superworkspace location is an environment-level configuration/relocation operation.

It is not ordinary navigation.

The SDK may expose:

```text id="3r1hy9"
Settings
→ Development
→ Superworkspace
```

with actions such as:

```text id="2a6cu5"
Change Location

Relocate

Diagnose
```

where safely supported.

Ordinary create/acquire dialogs should not repeatedly ask for source destination.

---

# Explorer Thesis

The Explorer should show modeled Vapor structure.

It should not merely dump the filesystem.

Canonical source topology:

```text id="ezk3pe"
Superworkspace
└── Authority
    └── Source Repo Container
        └── Source Repo / Vapor Workspace
            └── Project
                └── implementation / files / Cargo detail
```

Example:

```text id="gldlnl"
Development
└── GHF-Studios
    ├── Vapor-Client
    │   ├── Vapor
    │   │   ├── Client
    │   │   └── ...
    │   └── Vapor-Examples
    │
    ├── Vapor-Platform-Server
    │   └── Vapor-Registry-Server
    │
    └── Loo-Cast
        └── Game
            └── Loo-Cast
```

---

# Explorer Hierarchy Must Match Identity

The Explorer must not show:

```text id="h4h9wm"
Source Repo
└── Workspace
```

because:

> **Source Repo = Vapor Workspace.**

Likewise it must not show:

```text id="z76fwl"
Project
└── independently identified Content
```

for ordinary typed Content Projects.

For a Content Project:

```text id="8z5pzr"
Project
    identity = Content identity
    kind = Game / Engine / Library / ...
```

---

# Authority Nodes

Authority is a real identity layer and should be visible where useful.

Example:

```text id="nmjd0l"
GHF-Studios
```

The GUI may collapse or visually de-emphasize a single Authority in simple cases.

It must not erase the identity model.

Authority becomes particularly important once several Authorities exist in one Superworkspace.

---

# Source Repo Container Nodes

A Source Repo Container node should expose:

```text id="bua20m"
canonical identity

provider linkage

local availability

Git state

contained Source Repos

first-party trust/facility where applicable
```

Examples:

```text id="9vql8e"
GHF-Studios/Vapor-Client

GHF-Studios/Loo-Cast
```

---

# Source Repo Nodes

A Source Repo node represents both:

```text id="2hrpbd"
Git Source Repo

and

Vapor Workspace
```

It may expose:

```text id="h85x9l"
canonical identity

provider repository

Git branch/commit/state

Projects

Workspace-level Vapor metadata

Cargo realization where relevant
```

---

# Project Nodes

A Project node represents the primary Vapor development unit.

It may display:

```text id="d0p4aw"
Project name

Project kind

canonical identity

health

build/test state

publication state

relevant Cargo realization
```

For example:

```text id="szvrh6"
Loo-Cast
Game

GHF-Studios/Loo-Cast/Game/Loo-Cast
```

---

# Project Implementation Expansion

Below a Project the SDK may show:

```text id="dqlefq"
Source

Cargo packages

crates/targets

tests

examples

generated/read-only realization

scripts

assets

configuration
```

This is implementation structure.

It is not another Vapor identity hierarchy unless a deeper object is explicitly modeled.

---

# Explorer vs Filesystem

The Explorer should default to semantic Vapor structure.

Raw filesystem navigation may exist as:

```text id="ir7sl6"
Files

Reveal in Filesystem

Source Files
```

but should not replace semantic topology.

The user should be able to understand both:

```text id="kpx0ju"
what this object means in Vapor

and

where its files physically live
```

---

# Explorer Health

Objects should remain visible when state is unhealthy.

Examples:

```text id="q9de8n"
Container registered but not acquired

Source Repo declared but submodule checkout missing

Project metadata invalid

derived IDE state stale

provider unavailable
```

The Explorer should expose the exact problem rather than disappearing the object.

---

# Unavailable Container

Example:

```text id="saem4m"
GHF-Studios/Loo-Cast
    Source Repo Container
    Not locally available
```

Possible actions:

```text id="ww8hwx"
Acquire

Inspect Registry

Inspect Provider
```

There is no arbitrary `Locate` operation merely because the canonical checkout is missing.

---

# Missing Source Repo Checkout

Example:

```text id="m724y6"
Game
    Source Repo
    Git submodule checkout missing
```

Possible UI:

```text id="umyk1v"
Inspect Git

Show parent Container status

Possible reconciliation...
```

This is not generic independent reacquisition.

---

# Dirty Git State

A node may show:

```text id="d6nb8q"
Modified

Uncommitted

Ahead 2

Detached HEAD

Differs from parent gitlink
```

without being marked:

```text id="szn7pv"
Broken
```

Git state and Vapor health are distinct.

---

# No Durable Open Object Set

The old Core/UI model of:

```text id="jyw5n6"
Open Workspaces
Open Projects
Open Content
```

as durable semantic working-set membership is retired.

The GUI may still have:

```text id="1f4vds"
open editor tabs

expanded Explorer nodes

open tool windows

navigation history

pinned views
```

Those are frontend/session state.

They are not semantic target membership.

---

# No Durable Focus Set

Likewise, the SDK should not maintain:

```text id="0rdq6y"
Focused Workspace A
Focused Workspace B
Focused Project C
```

as hidden default operation targets.

There is one persistent resolution Context.

Operations use explicit Selection when narrowed.

---

# Persistent Context

Persistent Context is a lightweight semantic navigation location.

The SDK should make it visible enough to understand without turning it into constant visual noise.

Example toolbar/breadcrumb:

```text id="n6b7ky"
Context:
GHF-Studios / Loo-Cast / Game
```

This means relative selectors may resolve beneath that prefix.

It does not mean operations automatically target Game.

---

# Context Navigation

Explorer navigation may provide deliberate actions such as:

```text id="wnmvcp"
Set Context Here

Open as Context

Use as Working Context
```

Exact wording remains open.

Merely clicking a tree node should not necessarily mutate persistent Context.

Persistent Context change should be deliberate and discoverable.

---

# Context Breadcrumb

A breadcrumb may show:

```text id="gp9mjw"
GHF-Studios
  › Loo-Cast
    › Game
```

It may permit navigation upward/downward through exact hierarchy.

The breadcrumb represents semantic Vapor location.

It is not a raw filesystem breadcrumb.

---

# Clear Context

The SDK should allow persistent Context to be cleared.

Clearing Context does not:

```text id="9fysdx"
close files

remove source

change Git

change selected Vapor App

change operation Selection
```

It only removes the persistent resolution prefix.

---

# Transient Context Override

Individual GUI operations may deliberately use a temporary Context different from the persistent one.

Example:

```text id="fz437p"
persistent Context:
    GHF-Studios/Loo-Cast/Game

operation:
    Inspect My-Library

transient Context:
    GHF-Studios/Common/Libraries
```

The operation should expose this where relevant.

It must not secretly rewrite persistent Context.

---

# GUI Selection

Tree rows, graph nodes, search results, and other GUI elements may be selected transiently.

This is immediate frontend interaction state.

It may become **Operation Selection** when the user invokes an operation.

It is not persistent Context.

---

# Operation Selection

Suppose the user selects:

```text id="lrzn7i"
Registry
Identity
```

inside the Platform Server Explorer and chooses:

```text id="q4ba63"
Test
```

The SDK may construct:

```text id="xqumk3"
subject:
    Platform Server

operation:
    test

Selection:
    Registry
    Identity
```

This is equivalent in meaning to the corresponding explicit CLI selection.

---

# Clicking Does Not Rewrite Context

Ordinary Explorer clicking should not silently produce:

```text id="7xc0kb"
persistent Context changed

operation subject changed globally

future deploy scope changed
```

unless the user performs a deliberate Context navigation operation.

This prevents ambient UI state from becoming dangerous hidden scope.

---

# Operation Subject

The SDK should make the current operation subject obvious.

Examples:

```text id="7jr0mc"
Client

Platform Server

GHF-Studios/Loo-Cast

GHF-Studios/Loo-Cast/Game

Loo-Cast Project
```

The subject establishes natural operation semantics.

---

# Subject / Selection Presentation

The top toolbar may show something like:

```text id="7wowvm"
Subject:
    Platform Server

Selection:
    Registry, Identity

Context:
    GHF-Studios/Vapor-Platform-Server

Operation:
    Test
```

Not every field needs permanent full-width UI.

But the relationship should be inspectable.

---

# No Generic “All”

When no explicit Selection exists, the SDK should indicate:

```text id="cpnu97"
Scope:
    Complete Platform Server
```

rather than:

```text id="j89ukv"
All Projects
```

Natural complete scope belongs to the operation subject.

---

# Hierarchical Selection

Selecting a Source Repo means selecting that Source Repo operation scope.

The user should not need to manually multi-select every Project descendant merely to express:

```text id="vev4vy"
test Registry
```

The operation decides what Source Repo scope means.

---

# Mixed Selection

Some operations may accept mixed topology roots.

Example:

```text id="cnalbd"
Registry
Identity/core
```

Whether that is valid depends on the operation.

The UI may allow constructing the Selection and then explain incompatibility if the operation rejects it.

It should not invent universal hierarchy-cardinality rules beyond actual operation semantics.

---

# Multiple Selection

The GUI should exploit multi-selection naturally.

For example:

```text id="606i99"
select Registry

Ctrl-select Identity

Test
```

If valid:

```text id="p4d4ll"
Test enabled
```

If invalid:

```text id="99j4rc"
Test unavailable

Reason:
    this operation requires one common Source Repo scope
```

Specific reason > generic disabled state.

---

# Operation Legality

Operation availability may depend on:

```text id="7ogwrq"
subject

operation

variant

Selection

Role

authority

topology

toolchain

health

recipe/configuration

external availability
```

The GUI should expose the actual reason.

---

# Disabled Operations

A disabled action should normally be explainable through hover, Inspector, or adjacent detail.

Example:

```text id="mp4nkg"
Deploy to VPS
    unavailable

Reason:
    VPS deployment requires complete Platform Server scope.

Selected:
    Registry

Available:
    Deploy Local
```

This makes Vapor's rules discoverable.

---

# Main Work Area

The central Work Area should support several first-class view/document kinds.

Examples:

```text id="agq5kz"
Rust source

Vapor manifests

Cargo manifests

Packagepack editor

structured Content configuration

dependency/composition graph

build realization view

Registry/provider view

generated read-only artifact

documentation

first-party operation dashboard

future Engine/Game-specific tooling
```

The SDK is not merely a text editor.

---

# Document Tabs

Documents may remain open across navigation.

Example:

```text id="16e2cl"
Loo-Cast/src/lib.rs

Packagepack Graph

Vapor.toml

Registry Health
```

Document-tab persistence belongs to Resume State.

It is not canonical operation Context.

---

# Source Editing

The long-term SDK may provide increasingly capable integrated source editing.

Possible progression:

```text id="1a091u"
initial:
    Vapor-aware project/config environment
    + external Rust IDE integration

later:
    integrated source editor

    language server

    navigation

    refactoring

    debugger integration
```

External IDEs remain valid even when SDK editing becomes strong.

---

# Structured Editors

Where Vapor knows a schema, the SDK should offer structured editing.

For example:

```text id="k9ypc6"
Vapor.toml dependency
```

may be edited via:

```text id="i3cx1w"
Inspector form

graph interaction

raw TOML
```

These are projections over one authored state.

Users should remain able to inspect/edit the underlying representation.

---

# Graph Views

Graph visualization is a first-class SDK capability.

Potential graphs include:

```text id="c51oe1"
Vapor semantic dependency graph

Packagepack composition

Engine/Game/Mod relationships

Library dependencies

Cargo realization

source topology

publication/provider relationships

future capability/extension relationships
```

---

# Graph Identity

Graph nodes are projections of real Core identities.

They are not duplicate fake entities.

Selecting:

```text id="xkn3m5"
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

in a graph and in the Explorer refers to the same Project.

Inspector behavior should remain coherent.

---

# Graph Status

Useful graph node/edge states include:

```text id="xtxp4l"
healthy

warning

error

selected

stale

unavailable

Yanked/Banned where relevant
```

The old durable `focused` graph status should not remain a semantic targeting concept.

A UI-specific keyboard focus indicator may of course still exist.

---

# Inspector

The right-side Inspector displays semantic detail for current GUI selection.

Potential tabs:

```text id="fclgw5"
Overview

Properties

Dependencies

Validation

Source

Cargo / Realization

Git

Build

Publication

Operations
```

Exact tabs depend on object kind.

---

# Inspector Goals

The user should be able to learn:

```text id="909mqr"
What is this?

What is its canonical identity?

What kind is it?

Where does its source come from?

Which Source Repo owns it?

Which provider repository backs that Repo?

Is it locally available?

What Git state exists?

What does it depend on?

What depends on it?

What build/publication state exists?

Which operations are legal?

Why is an operation unavailable?
```

The Inspector is a major discoverability surface.

---

# Inspector vs Operation Context

Inspector selection does not automatically become persistent Context.

For example:

```text id="ohe6gk"
click Project in Explorer
→ Inspector displays Project
```

does not imply:

```text id="5s5v0i"
persistent Context = Project
```

A deliberate action is required to establish Context.

---

# Operation Toolbar

The top operation area should expose relevant semantic operations.

Possible operations include:

```text id="2iedj9"
Inspect

Verify

Build

Test

Run

Publish

Diagnose

Repair

Deploy
```

Different subjects expose different operations.

The toolbar should not imply every verb is universally meaningful.

---

# Natural Operation Presentation

Example for first-party Platform Server:

```text id="8q5rt2"
Platform Server
    [Build] [Test] [Deploy ▾]

Deploy:
    Local
    VPS
```

With no explicit Selection:

```text id="kmm47p"
scope:
    Complete Platform Server
```

---

# Selection-Aware Toolbar

With:

```text id="ikid5l"
Selection:
    Registry
```

the UI may update:

```text id="wkb6k6"
Test
    enabled

Deploy Local
    enabled

Deploy VPS
    disabled
```

and explain why.

---

# Run Target

Run may require an explicit complete runnable target.

For Content development, this may mean:

```text id="wnsunj"
Packagepack

development Packagepack

generated development composition
```

The SDK should expose the exact Run target.

It should not infer one from a hidden Focus set.

---

# Run Configuration

A visible Run configuration may include:

```text id="ncwf37"
runnable Packagepack

target

profile

arguments

environment

development overrides
```

Exact representation remains open.

The important rule is:

> **Run target is explicit inspectable operation configuration, not ambient Focus.**

---

# Toolchain

The Vapor-managed toolchain should be visible and inspectable.

Possible information:

```text id="7xjefv"
Rust version

Cargo version

toolchain provenance

health

managed status

Rust Analyzer state

diagnostics

repair
```

Physical paths should remain secondary unless diagnostically useful.

---

# Pinned/Vendored Toolchain

For first-party/root development, the SDK should clearly indicate whether the current operation uses the expected Vapor-managed pinned/vendored toolchain.

Example:

```text id="szb6qu"
Toolchain
    Vapor Managed

Rust
    1.97.0

Status
    Ready
```

An ambient system Rust installation must not silently override required managed toolchain policy.

---

# Problems Tool Window

Problems aggregates Vapor-aware diagnostics.

It may group by:

```text id="s1oq99"
Project

Source Repo

Container

build

Cargo

Git

toolchain

publication

deployment

provider
```

The Vapor-level explanation should come before raw underlying tool output.

---

# Build Tool Window

Build displays:

```text id="osn6e5"
subject

Selection

current step

recipe step

Cargo invocation/output

artifact result

warnings/errors

timing/progress
```

Raw Cargo output remains inspectable.

---

# Test Tool Window

Test displays:

```text id="vl6ob5"
operation subject

Selection

test groups

individual results

logs

failure diagnostics

artifacts
```

For first-party systems, it may represent multi-step test recipes rather than merely Cargo tests.

---

# Terminal

An integrated terminal may exist.

It should be a real terminal.

Vapor may initialize useful environment/tool access.

It must not make shell CWD the canonical Vapor Context.

This distinction should remain true even inside the SDK terminal.

---

# Git / VCS Tool Window

Git should be first-class for Composer-and-above.

The SDK should expose:

```text id="hlgubt"
repository

branch

commit

dirty state

ahead/behind

diff

remotes

submodule state

conflicts

commit

push

history
```

according to Role/authority.

---

# Git Ownership

Source Repo Container and Source Repo are real Git repositories.

The SDK should make repository boundaries clear.

Example:

```text id="j2rfbx"
Vapor-Client
    Git repository

Vapor
    Git submodule / Source Repo
```

The user should be able to tell which repository owns a file/change.

---

# Git Safety

Potentially destructive actions should require explicit intent.

Examples:

```text id="6xco1z"
reset

discard changes

switch branch with conflicts

force submodule commit

delete branch
```

Vapor should never simplify UX by pretending authored Git state is disposable.

---

# Git Reconciliation

For a missing declared submodule checkout, the SDK may show:

```text id="bbqp7x"
Registry
    Source Repo checkout missing

Git topology:
    declared by parent Container

Suggested inspection:
    git status
    git submodule status

Possible reconciliation:
    git submodule update --init -- Registry
```

A warning is appropriate when Vapor cannot prove the operation preserves intended Git state.

---

# No Multiple Canonical Realizations

The old model allowed:

```text id="3n31ja"
one Workspace identity
├── official realization
└── fork realization
```

inside the canonical Vapor development model.

That is retired.

The canonical Superworkspace has one canonical local realization of a registered source identity.

---

# Forks Are Distinct Provider/Source Situations

A Git fork may still exist.

But it should not masquerade as another physical realization of the same canonical Source Repo identity inside the managed topology.

Depending on how collaboration is modeled, a fork may be:

```text id="cu27k6"
provider relationship to the same development lineage

or

source registered under another Vapor identity
```

The exact collaboration model remains open.

The SDK should not normalize several arbitrary working trees as one canonical identity.

---

# External Extra Checkouts

Advanced users may maintain arbitrary external Git checkouts outside the canonical Superworkspace.

The SDK may offer:

```text id="jbumkt"
Reveal

Open externally

Inspect provider/fork
```

where useful.

Those checkouts do not automatically become canonical Vapor source realizations.

---

# Steam / Deployment Tool Window

For relevant Roles, this surface may expose:

```text id="6wm3bc"
local Vapor App installation

Steam publication

Client deployment

Workshop publication

Platform deployment

branch/depot information

health checks

provider diagnostics
```

The exact contents depend on subject.

---

# Client Development Surface

The SDK may expose a dedicated first-party Client view.

Possible information:

```text id="l4f5zo"
Client facility

backing source identity

Git health

toolchain

build

test

local deployment

Steam deployment

operation history/logs
```

The facility remains stable even if internal source topology changes.

---

# Platform Server Surface

The SDK may expose:

```text id="knq28m"
Platform Server overview

service topology

Source Repos

health

Build

Test

Deploy Local

Deploy VPS

logs

operation recipes
```

This is a first-party semantic facility view.

---

# Platform Service Surface

Selecting a service such as:

```text id="bcxxe7"
Registry
Identity
Diagnostics
Docs
Homepage
```

may show:

```text id="crxax5"
source

Project(s)

health

test state

deployment support

logs

provider/configuration
```

Service-level detail coexists with whole-Platform operations.

---

# Examples Surface

Official Examples should be easy to acquire, inspect, build, and learn from.

Possible capabilities:

```text id="ldf2jr"
Acquire Examples

Browse Projects

Open architecture examples

Run validation

Inspect dependency graphs

Compare authored vs Cargo realization
```

The Examples facility remains semantic even if backing source topology changes later.

---

# Diagnostics UX

Vapor diagnostics should explain Vapor meaning first.

Example:

```text id="p991av"
Cargo realization for `Wheel-Rules` conflicts with its
Vapor dependency declaration.

Vapor requires:
    GHF-Studios/.../Wheel-Core @ ^0.1

Cargo currently resolves:
    ...

[Inspect Vapor Dependency]
[Inspect Cargo]
```

Raw Cargo output may be expanded beneath that.

---

# Specific Errors

An error should identify:

```text id="rfr91d"
resolved object

failed invariant

actual state

required state

safe next actions
```

Example:

```text id="gnjdnl"
Cannot build `My-Game`.

Parent Source Repo:
    GHF-Studios/My-Stuff/Game

State:
    Source Repo is not locally available.

Action:
    Acquire Source Repo Container
    GHF-Studios/My-Stuff
```

---

# Ambiguity UI

When user input is ambiguous:

```text id="ywsqg2"
core
```

the SDK may show:

```text id="9cc912"
3 matches

Registry/core
Identity/core
Diagnostics/core
```

The user may:

```text id="x5yt3z"
choose candidate

type more qualification

change transient Context

change persistent Context deliberately
```

No arbitrary first-match selection.

---

# Repair UX

Repair should be shown only for safely derivable Vapor-owned state.

Good:

```text id="jk9812"
IDE integration stale

[Repair]
```

Good:

```text id="9o1lbb"
generated operation realization missing

[Regenerate]
```

Bad:

```text id="tn11c7"
Git repository dirty

[Repair]
```

unless a very specific safe reconciliation is actually defined.

---

# Diagnose vs Repair

The SDK should distinguish:

```text id="gu7f9e"
Diagnose
    observe/explain

Repair
    safely regenerate derived state
```

This distinction should be visually and semantically obvious.

---

# Launcher Surfaces Remain Reachable

Development Mode should not trap the user.

Ordinary Vapor surfaces remain accessible:

```text id="eryhgc"
Apps

Content Library

Accounts

Settings

composition

installed Vapor Apps
```

through:

```text id="out7rt"
persistent application navigation

or

Return to Launcher
```

depending on final shell design.

---

# Resume State

Returning to Launcher may preserve:

```text id="itmpa2"
editor tabs

panel layout

Explorer expansion

navigation history

scroll positions

tool-window state

persistent Context
```

where appropriate.

Resume State does not imply persistent operation Selection.

---

# Enter Development

Typical:

```text id="mqcfvc"
Start Vapor
    ↓
Launcher
    ↓ Enter Development
SDK
    ↓
canonical Superworkspace available
    ↓
restore useful Resume State
```

If the canonical Superworkspace is not configured:

```text id="fvhtoa"
Development setup required
```

may direct the user to an appropriate environment/configuration flow.

It should not invent several named development universes.

---

# Returning to Launcher

Returning should:

```text id="v6bquk"
preserve recoverable Resume State

preserve authored source

preserve persistent Context

warn/stop explicitly SDK-owned processes where required

hide/collapse development chrome
```

It should not:

```text id="pmz03m"
reset Git

clear Context automatically

delete source

discard work
```

---

# Closing a Document

Closing an editor/view means:

```text id="4qmdy0"
remove this frontend document from the Work Area
```

It does not mean:

```text id="2fm7bj"
close Project semantically

remove Project from Core

clear Context

delete source
```

This distinction should eliminate much of the old overloaded “Close” behavior.

---

# Removing Source

Source lifecycle operations should use explicit words.

Possible meanings include:

```text id="a5agka"
remove local Container checkout

delete registered identity

archive repository

delete Project

remove Vapor App
```

These are materially different.

The UI must not collapse them into one generic `Remove`.

---

# Context Clear vs Source Removal

Clearing persistent Context:

```text id="dveca7"
Clear Context
```

must be obviously harmless relative to source.

It does not remove any local object.

---

# External IDE Integration

The SDK should support external IDE workflows.

Examples:

```text id="twxtdb"
Open Project in RustRover

Open Source Repo in RustRover

Reveal in Files

Open Terminal Here
```

where appropriate.

---

# RustRover Integration

Vapor may reconcile RustRover project/toolchain configuration based on:

```text id="2sq72t"
canonical Superworkspace

Source Repo / Project topology

managed Rust toolchain

Cargo realization
```

IDE configuration should remain derived where possible.

Unrelated IDE configuration should be preserved.

---

# External Edits

Vapor must expect source to change outside the SDK.

Examples:

```text id="vjptc8"
RustRover edit

Git CLI branch change

manual Cargo.toml edit

external formatter/refactor
```

The SDK should observe/reconcile current filesystem/Git/Cargo state.

It must not assume it is the sole editor.

---

# GUI vs CLI

SDK and CLI should share semantic capability where practical.

GUI:

```text id="vbgrqb"
tree navigation

visual multi-selection

Inspector

graphs

dialogs

context menus
```

CLI:

```text id="474fts"
canonical paths

short selectors

--context

--select

machine-readable output
```

Both resolve exact operations in Vapor Core.

---

# Coding-Agent Integration

The SDK/source model should remain friendly to integrated coding agents.

Coding agents should not need hidden GUI session knowledge.

They should be able to use:

```text id="2g835y"
canonical source paths

Vapor manifests

Git

CLI operations

explicit Context

explicit Selection

machine-readable diagnostics
```

This makes automated development interoperable with human SDK usage.

---

# Folder/Docs Integration

The SDK may eventually expose the reorganized Premium Documentation corpus directly.

Once documentation is structurally reorganized into topic folders with central index files, the SDK could project that hierarchy.

For example:

```text id="8g872b"
Documentation
├── Identity & Source
├── Development
├── Client & Runtime
├── Publishing
├── Platform
└── Architecture
```

The exact docs repository/folder migration is separate repository-structure work.

The SDK model should not hardcode today's flat file layout.

---

# First Serious SDK Slice

The architecture-proving implementation order should now be approximately:

```text id="cghh8f"
1. preserve/extract prototype design system

2. Launcher → SDK application shell

3. real Installation / Role / toolchain state

4. real canonical Superworkspace discovery

5. real Authority / Container / Source Repo / Project Explorer

6. persistent Context navigation

7. transient GUI Selection → explicit Operation Selection

8. real Inspector

9. real Problems/diagnostics

10. managed Build/Test/Cargo operations

11. real Content / Packagepack views

12. semantic dependency/composition graph

13. Git/provider surfaces

14. Client / Platform Server first-party surfaces

15. publication/deployment surfaces

16. progressively stronger integrated editing
```

Not:

```text id="6y9kfa"
Open / Focus / Selection session model
```

That architecture is retired.

---

# Worked Journey — Resume Development

The user starts Vapor.

Launcher appears.

They choose:

```text id="irfe8q"
Enter Development
```

SDK Mode opens.

Vapor reconnects to the one configured canonical Superworkspace.

The SDK restores:

```text id="58zztm"
editor tabs

Explorer expansion

panel layout

navigation history

persistent Context
```

from Resume State.

Current Core state is refreshed.

No terminal setup is required.

---

# Worked Journey — Navigate Context

The Explorer contains:

```text id="qws8ch"
GHF-Studios
└── Loo-Cast
    └── Game
        └── Loo-Cast
```

The user deliberately chooses:

```text id="gt8t98"
Set Context Here
```

on:

```text id="4eq96f"
GHF-Studios/Loo-Cast/Game
```

Toolbar shows:

```text id="kcn38w"
Context:
    GHF-Studios/Loo-Cast/Game
```

Typing/searching:

```text id="j5jty4"
Loo-Cast
```

now resolves uniquely to the Project.

No operation has been performed merely by changing Context.

---

# Worked Journey — Build a Project

The user selects:

```text id="i1aejp"
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

and presses:

```text id="la7rgh"
Build
```

The GUI constructs an exact Project build operation.

The Inspector shows:

```text id="ylul2j"
Project
    Loo-Cast

Kind
    Game

Source Repo
    GHF-Studios/Loo-Cast/Game

Toolchain
    Vapor Managed
```

Build output appears below.

Cargo details remain inspectable.

---

# Worked Journey — Build Vapor Client

The user opens the first-party Client surface.

It shows:

```text id="2zesy1"
Vapor Client

Backing Source:
    GHF-Studios/Vapor-Client

Toolchain:
    Vapor Managed
```

The user presses:

```text id="f58rmr"
Build
```

The operation subject is:

```text id="kefg40"
Client
```

not an arbitrarily inferred Project from the currently selected Explorer row.

The Client operation recipe decides what the build entails.

---

# Worked Journey — Test Two Platform Repos

The user enters the Platform Server surface.

They select:

```text id="mc5tu7"
Registry
Identity
```

and choose:

```text id="eeuhnk"
Test
```

The operation resolves:

```text id="85hwiq"
Subject:
    Platform Server

Selection:
    Registry
    Identity
```

If supported, Test runs.

No durable Focus state is created.

---

# Worked Journey — Invalid Partial Deployment

The user selects:

```text id="nzal4r"
Registry
```

and chooses:

```text id="4ldtyj"
Deploy → VPS
```

The operation is rejected.

The SDK shows:

```text id="qp265c"
VPS deployment requires the complete Platform Server scope.

Current Selection:
    Registry

Available actions:
    Deploy complete Platform Server to VPS
    Deploy Registry locally
```

The UI teaches the specific invariant.

---

# Worked Journey — Missing Container

The Registry knows:

```text id="wexqms"
GHF-Studios/Loo-Cast
```

but the Container is absent locally.

Explorer shows:

```text id="30tnym"
Loo-Cast
    Not locally available
```

The user chooses:

```text id="21at4y"
Acquire
```

Vapor acquires it into the canonical Superworkspace location.

No destination chooser is required.

---

# Worked Journey — Missing Submodule

`Vapor-Platform-Server` exists locally.

Its `Registry` Source Repo is declared but not checked out.

Explorer shows:

```text id="vfr6rn"
Registry
    Git submodule checkout missing
```

Inspector explains:

```text id="g3iiht"
This Source Repo is already part of the Container's authored Git topology.
```

It offers Git inspection and cautious reconciliation.

It does not pretend the correct solution is generic Source Repo reacquisition.

---

# Worked Journey — Dirty Source

The user edits `Vapor`.

Explorer shows:

```text id="h8pkhw"
Vapor
    Modified
```

The Project remains:

```text id="sbwqlh"
Buildable

Testable
```

where actual operation requirements permit.

Dirty does not become a red “broken source” state.

---

# Worked Journey — Failed Build

The user modifies Loo Cast.

A rebuild fails.

SDK shows:

```text id="p1mxl8"
Source
    Modified

Latest Build
    Failed

Previous Installed Vapor App
    Valid

Run Previous App
    Available
```

The user can inspect the new build failure without losing the previous runnable App.

---

# Worked Journey — External RustRover Edit

The user chooses:

```text id="8alx59"
Open in RustRover
```

They edit/commit source externally.

Returning to SDK:

```text id="kqj6ho"
filesystem/Git state refreshes

Project status updates

Cargo metadata reconciles where necessary
```

No re-import is required simply because another editor changed the canonical source.

---

# Worked Journey — Return to Play

The user finishes development.

They choose:

```text id="vypou4"
Return to Launcher
```

Development chrome disappears/collapses.

The ordinary Apps/Library experience returns.

SDK Resume State remains available.

The default/selected Vapor App is unchanged unless the user explicitly changed it.

---

# SDK Invariants

* SDK Mode is a Launcher-integrated development superset.
* Installer remains a distinct capability-management boundary.
* SDK and Launcher share Vapor Core semantics.
* The SDK may be implemented before the simplified final Launcher.
* The historical Figma/Tauri SDK prototype remains visual design ancestry.
* Prototype fake/old semantics are not normative.
* The canonical SDK development root is one Superworkspace.
* The SDK does not expose several ordinary named Superworkspaces as parallel development contexts.
* Authority → Source Repo Container → Source Repo → Project is the primary Explorer topology.
* Source Repo equals Vapor Workspace.
* Content identity for a Content Project equals Project identity.
* Project kind is shown as metadata/semantics rather than another hierarchy node.
* Explorer projects semantic topology rather than merely filesystem structure.
* Persistent Context is one semantic resolution prefix.
* Persistent Context is not an operation target set.
* CWD is not canonical Vapor Context.
* There is no durable Open-object semantic working set.
* There is no durable multi-Focus target set.
* Editor tabs/expanded nodes/layout are frontend Resume State.
* Transient GUI selection may become explicit Operation Selection.
* Clicking/inspecting does not silently change persistent Context.
* Operation Subject establishes natural complete scope.
* No Selection means natural complete operation scope, not hidden `all`.
* Selection is hierarchical and operation-specific.
* Multi-selection legality depends on the actual operation.
* Disabled operations should explain the real violated invariant.
* Graph nodes project Core identities rather than duplicate semantic objects.
* The Inspector explains canonical identity, relationships, state, and operations.
* Run target is explicit inspectable configuration, not hidden Focus.
* Vapor-managed toolchain state is visible.
* First-party/root development uses the expected pinned/vendored managed toolchain where required.
* Git remains visible and first-class.
* Dirty Git state is not automatically broken Vapor state.
* Missing declared submodule checkout is Git reconciliation.
* Repair is for safely derivable Vapor-owned state.
* There is one canonical local realization per registered source identity in the managed Superworkspace.
* Arbitrary external extra Git checkouts are not automatically canonical Vapor realizations.
* Client, Platform Server, and Examples may expose dedicated first-party SDK surfaces.
* Vapor-oriented diagnostics precede raw underlying tool output.
* Failed builds preserve previous valid installed Apps.
* SDK and CLI share semantics without sharing interaction grammar.
* External IDE integration remains first-class.
* Coding-agent workflows must not depend on hidden GUI session state.
* Future documentation-folder structure may be projected by SDK without hardcoding today's flat Premium Docs layout.

---

# Open SDK Questions

* Exact persistent Context navigation wording and affordance.
* Exact toolbar presentation of Context vs Subject vs Selection.
* Exact multi-selection interaction for hierarchical scopes.
* Exact built-in source editor technology/capability timeline.
* Exact Launcher↔SDK navigation shell.
* Exact multi-window model.
* Exact persistence granularity of tabs/layout/navigation.
* Exact debugger integration.
* Exact Run configuration model.
* Exact visual component-library/frontend implementation.
* Exact Git workflow depth.
* Exact graph editor interactions.
* Exact Composer subset of SDK functionality.
* Exact first-party Client dashboard.
* Exact Platform Server/service dashboard.
* Exact operation-progress/cancellation UX.
* Exact external coding-agent integration.
* Exact documentation-browser integration after Premium Docs folderization.
