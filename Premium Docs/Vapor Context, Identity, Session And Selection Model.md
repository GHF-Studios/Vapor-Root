> [!info]
> This document defines Vapor's normative model for structural identity, the local Superworkspace, contextual path resolution, operation subjects, explicit selection, ambiguity, and frontend context behavior.
>
> It applies across Vapor Core, the Vapor SDK, CLI surfaces, automation, external IDE integration, and future Vapor Shell workflows.
>
> The **Vapor Development Experience Model** owns broader source-development workflows.
>
> The **Vapor CLI Model** owns exact command-line grammar.
>
> The **Vapor SDK Experience Model** owns graphical interaction and presentation.

---

# Core Principle

Vapor must identify intended work with as little information as is safely sufficient without weakening its canonical source hierarchy.

The central rule is:

> **Vapor has exact hierarchical identities, but users may provide shorter paths or selectors whenever the missing information can be resolved uniquely.**

Vapor never guesses between ambiguous candidates.

Conceptually:

```text
context
    ↓
operation subject
    ↓
selection
    ↓
exact resolved Vapor object(s)
    ↓
operation
```

A useful shorthand is:

```text
context  →  operation subject  →  selection
  left            middle            right
```

Context supplies information missing to the **left** of an operation's subject.

Selection supplies information missing to the **right** of that subject.

Neither mechanism may create holes in Vapor's hierarchy.

---

# Canonical Source Hierarchy

The canonical source/development hierarchy is:

```text
Authority
└── Source Repo Container
    └── Source Repo
        └── Project
```

A complete Project identity therefore has the form:

```text
Authority/Source-Repo-Container/Source-Repo/Project
```

For example:

```text
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

means:

```text
GHF-Studios    Authority
Loo-Cast       Source Repo Container
Game           Source Repo
Loo-Cast       Project
```

The hierarchy is structural and meaningful.

Segments must not be omitted from the middle of an absolute identity.

For example:

```text
GHF-Studios/Game/Loo-Cast
```

cannot mean:

```text
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

by inference.

Context may remove a leading prefix.

It may never remove arbitrary interior segments.

---

# Identity Paths May End at Any Layer

A Vapor path does not inherently identify a Project.

The following are all meaningful exact paths:

```text
GHF-Studios
GHF-Studios/Loo-Cast
GHF-Studios/Loo-Cast/Game
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

They identify, respectively:

```text
Authority
Source Repo Container
Source Repo
Project
```

An operation determines which target kinds are meaningful for that operation.

Vapor must not automatically descend from a supplied parent node merely to find some leaf.

For example, if testing a Source Repo Container is a meaningful operation, then:

```text
GHF-Studios/Loo-Cast
```

means that Container scope.

It does not secretly mean one arbitrary Project contained by it.

---

# Authority

An **Authority** is the first globally meaningful segment of Vapor source identity.

Examples may include:

```text
GHF-Studios
Some-Studio
Some-Developer
```

Authority is a Vapor/Registry concept.

It may correspond to an external Git-host organization or user, but provider ownership and Vapor Authority must not be conflated automatically.

Conceptually:

```text
Vapor Authority
    ↓
Registry provider linkage
    ↓
GitHub organization/user or other provider identity
```

An Authority may own multiple Source Repo Containers.

---

# Superworkspace

A **Vapor Superworkspace** is the single canonical local root in which Vapor realizes managed development source.

The Superworkspace:

* has no Vapor identity;
* is not an ID segment;
* is not a Git repository merely because it is a Superworkspace;
* has one known local location;
* contains locally available Source Repo Containers;
* may contain source belonging to multiple Authorities;
* is development infrastructure rather than published ecosystem identity.

Conceptually:

```text
Superworkspace/
├── GHF-Studios/
│   ├── Vapor-Client/
│   ├── Vapor-Platform-Server/
│   └── Loo-Cast/
└── Some-Developer/
    └── Some-Mods/
```

The Authority directory prevents otherwise valid identities such as:

```text
GHF-Studios/Foo
Some-Developer/Foo
```

from colliding locally.

The Superworkspace's own filesystem name has no semantic meaning.

Its physical root may be configured or relocated as a local development-environment operation without changing any contained Vapor identity.

Therefore:

```text
Superworkspace location
    = local configuration

Authority/Container/Repo/Project
    = Vapor identity
```

Changing the former must not change the latter.

---

# Superworkspace Lifetime

The Superworkspace is persistent local development infrastructure.

It is not reconstructed as an arbitrary per-command destination.

Operations such as source acquisition place source into its canonical location inside the configured Superworkspace.

Users should not repeatedly provide arbitrary destination paths to ordinary Vapor acquisition operations.

The Superworkspace is also distinct from the Steam App Instance.

Steam-distributed files, Vapor user data, and authored development source have different ownership and lifetime rules.

Those relationships are specified by the Development Experience and Client/Runtime models.

---

# Source Repo Container

A **Source Repo Container** is a Registry-recognized top-level Vapor Git repository which groups related Source Repos.

A Source Repo Container:

* is a Git repository;
* has identity beneath one Authority;
* occupies a canonical location in the Superworkspace;
* may declare Source Repo members through Git submodules;
* is not itself nested as a submodule of another Source Repo Container;
* represents meaningful authored source topology.

Example:

```text
GHF-Studios/Loo-Cast
```

A Source Repo Container is the conservative default source-acquisition unit.

The exact acquisition/create behavior is defined by the Development Experience and CLI models.

---

# Source Repo

A **Source Repo** is the primary source-bearing Git repository beneath a Source Repo Container.

The normative relationship is:

> **One Source Repo = one Vapor Workspace.**

These are not two layers.

They are two names for the same unit from different perspectives:

```text
Git/source view:
    Source Repo

Vapor development view:
    Vapor Workspace
```

A Source Repo:

* is a Git repository;
* is normally represented as an authored submodule of its Source Repo Container;
* contains one or more Vapor Projects;
* owns Workspace-level authored Vapor metadata;
* has identity beneath its Source Repo Container.

Example:

```text
GHF-Studios/Loo-Cast/Game
```

Source Repo independent acquisition is not implied merely because Git can clone the repository independently.

Whether a Source Repo may be realized independently is explicit modeled policy and is conservative by default.

---

# Project

A **Vapor Project** is a modeled development unit inside one Source Repo.

A Project:

* is not itself a Git repository;
* has identity within its Source Repo;
* may correspond to one coherent Rust/Cargo development or execution context;
* may contain several Cargo packages or other supporting implementation structure;
* may be a typed Vapor Content Project;
* may also be an internal/non-Content Project where the owning subsystem requires one.

Example:

```text
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

For a typed Content Project, Vapor additionally knows its kind:

```text
Game
Engine
Library
Packagepack
Enginepack
Gamepack
Modpack
Engine Mod
Game Mod
Extension Mod
...
```

The kind is a property of the existing Project.

It is not an additional path segment.

Therefore an already-existing Project need not normally have its kind restated merely to identify it.

---

# Project Kind and Creation

Creation is different because the target object does not yet exist.

For example, conceptually:

```text
game create <new Project path>
```

supplies information Vapor cannot discover from a nonexistent Project:

```text
Project kind = Game
```

Once created, ordinary operations may resolve the Project and inspect its kind.

Conceptually:

```text
create:
    explicit kind + target identity

existing-object operation:
    target identity
    → resolve
    → inspect kind
    → validate operation legality
```

This distinction allows creation vocabulary to remain strongly typed without forcing existing-object operations to redundantly repeat known information.

Exact CLI grammar belongs to the Vapor CLI Model.

---

# Vapor Paths

A **Vapor Path** is a slash-separated structural path through the canonical hierarchy.

A full Vapor Path begins at Authority.

Examples:

```text
GHF-Studios/Loo-Cast
GHF-Studios/Loo-Cast/Game
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

A relative Vapor Path omits a complete leading prefix supplied by context.

For example, with context:

```text
GHF-Studios/Loo-Cast
```

the path:

```text
Game/Loo-Cast
```

resolves to:

```text
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

Likewise, with context:

```text
GHF-Studios/Loo-Cast/Game
```

the path:

```text
Loo-Cast
```

may resolve to the Project.

Again:

> **Relative paths may omit a prefix. They may not omit arbitrary middle segments.**

---

# Minimal Sufficient Information

Vapor should not force users to type a complete absolute path when the requested object is already uniquely identifiable.

The principle is:

> **Require no more information than is necessary to resolve the intended object safely.**

This creates three useful forms of input.

## Full Path

```text
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

Exact and context-independent.

## Context-Relative Path

With context:

```text
GHF-Studios/Loo-Cast
```

the selector:

```text
Game/Loo-Cast
```

is exact relative to that prefix.

## Minimal Unique Selector

An operation may already establish a sufficiently narrow resolution universe.

For example, within the known Vapor Platform Server subject:

```text
Registry
```

may uniquely identify one Source Repo.

In that case Vapor should accept it.

This is not an alternative canonical identity.

It is an unambiguous selector resolved to one exact canonical object.

---

# Ambiguity

Vapor never chooses arbitrarily between multiple matching objects.

Ambiguity is normal resolution information.

A useful ambiguity diagnostic should be exhaustive enough to teach the relevant topology.

For example:

```text
error: Project `core` is ambiguous within Vapor Platform Server

matches:

  Source Repo `Registry`
      Project `core`

  Source Repo `Identity`
      Project `core`

  Source Repo `Diagnostics`
      Project `core`

help: select one explicitly:

      Registry/core
      Identity/core
      Diagnostics/core

help: or establish a narrower context before retrying
```

Where the frontend has exact commands for persistent or transient context selection, diagnostics should also present those safe alternatives.

Errors should provide all useful modeled resolutions Vapor can confidently identify.

They should not provide speculative or potentially destructive instructions as though they were guaranteed safe.

---

# Context

A **Context** is a lightweight persistent resolution prefix.

Context exists for convenience.

Vapor does not require context in order to function.

An operation with sufficient explicit information works without any previously opened context.

Conceptually:

```text
persistent context:
    GHF-Studios/Loo-Cast/Game

relative selector:
    Loo-Cast

resolved:
    GHF-Studios/Loo-Cast/Game/Loo-Cast
```

There is one effective persistent location context for the relevant frontend/session.

It behaves more like a semantic navigation location than a set of simultaneously active operation targets.

---

# Open and Close

An **Open** operation establishes persistent context.

A **Close** operation removes or moves away from that persistent context according to the final frontend grammar.

Opening context does not:

* acquire source;
* build;
* test;
* deploy;
* publish;
* change Git state;
* select operation scope automatically.

Closing context does not:

* delete source;
* forget Registry identity;
* remove Git repositories;
* uninstall anything;
* discard authored work.

Open/Close is navigation/context state.

It is not source lifecycle.

Exact CLI spelling remains owned by the CLI Model.

---

# Transient Context Override

A one-shot operation must be able to override persistent context without modifying it.

Conceptually, the CLI may provide:

```text
--context <Vapor Path>
```

The exact spelling belongs to the CLI Model.

Resolution precedence is:

```text
full explicit path
    → no context required

relative path + explicit transient context
    → use transient context

relative path + persistent context
    → use persistent context

insufficient relative path + no usable context
    → unresolved / ambiguity diagnostic
```

A transient override exists only for that operation.

It does not rewrite the persistent context.

---

# Current Working Directory

The shell current working directory is not Vapor context.

Vapor must not rely on CWD to determine semantic identity or operation scope during ordinary modeled workflows.

A user may be physically inside one repository while intentionally operating on an unrelated Vapor object elsewhere in the Superworkspace.

Therefore:

> **Filesystem location and Vapor location context are separate concepts.**

CWD may remain useful to ordinary underlying tools such as Git and Cargo when the user invokes those tools directly.

Vapor itself should prefer its modeled Superworkspace, canonical identities, explicit context, and selectors.

Advanced/bootstrap commands may deliberately accept raw filesystem paths where a physical path is actually the object of the operation.

That is an explicit exception rather than ordinary identity resolution.

---

# Operation Subject

Every operation has a **subject**.

The subject establishes what the operation is fundamentally about.

Examples:

```text
Vapor Client
Vapor Platform Server
Examples
a specific Project
a Source Repo Container
```

The subject may be established by:

* a bespoke first-party command noun;
* an explicit positional Vapor object selector;
* another operation-specific semantic target.

The subject occupies the middle of Vapor's resolution model:

```text
context  →  subject  →  selection
```

The subject also establishes the operation's **natural complete scope**.

No explicit selection does not generically mean:

```text
--all
```

It means:

> Perform this operation on its natural subject.

For example:

```text
platform-server test
```

means:

> Test the Vapor Platform Server according to the Platform Server test operation.

The operation recipe determines what that entails.

It may involve all Projects, some Projects, generated validation, external checks, or other explicitly modeled work.

---

# Selection

A **Selection** explicitly changes or narrows the operation scope to descendants or constituent topology relative to the operation subject.

Conceptually, the CLI may provide:

```text
--select <selector or selection expression>
```

The exact syntax belongs to the CLI Model.

For example:

```text
platform-server test
    subject:
        Platform Server

selection:
    Registry
```

means:

> perform the Platform Server test operation with the Registry Source Repo selected as its requested scope.

Likewise:

```text
Registry/core
```

may identify a particular Project beneath that Source Repo.

Selection is per-operation intent.

It does not mutate persistent context.

---

# Context Is Not Selection

This distinction is fundamental.

Suppose the persistent context is:

```text
GHF-Studios/Vapor-Platform-Server/Registry
```

Then:

```text
platform-server test
```

still means:

> test the natural complete Platform Server subject.

The persistent context does not secretly transform the operation into:

```text
platform-server test --select Registry
```

Likewise, a destructive or deployment operation cannot accidentally become narrower merely because the user previously navigated somewhere else.

This is an intentional safety property.

Context affects **resolution**.

Selection affects **operation scope**.

---

# Hierarchical Selection

Selecting a parent topology node selects that meaningful operation scope together with whatever descendants the operation considers part of that scope.

For example, selecting Source Repo:

```text
Registry
```

does not require a redundant:

```text
all Projects
```

flag merely to include its Project descendants.

The operation recipe determines how a Source Repo-scoped operation is executed.

Therefore Vapor should avoid generic `--all-projects` or `--all-source-repos` concepts where the hierarchy and natural subject scope already express the intended operation.

Explicit batch selection is meaningful primarily when selecting several non-total sibling/subtree roots.

---

# Multiple Selection

Some operations may support multiple explicit selections.

The CLI should favor one coherent selection expression over requiring the same flag to be repeated excessively.

Conceptually:

```text
--select "Registry, Identity"
```

may select two Source Repo subtrees.

And:

```text
--select "Registry/core, Identity/core"
```

may select two Projects.

The exact delimiter and quoting grammar remain a CLI-level design detail.

After resolution, Vapor Core receives exact selected objects rather than parsing command-line syntax itself.

Operations remain free to reject mixed or otherwise invalid selection shapes.

---

# Operation Legality

Selection vocabulary may be generic.

Operation legality is not.

Whether a particular scope is valid may depend on:

* the subject;
* the operation;
* the operation variant;
* the deployment/run target;
* the selected topology;
* authored operation configuration;
* Project/Source Repo relationships;
* first-party subsystem semantics;
* other explicitly modeled invariants.

Vapor must not reduce this to overly broad rules such as:

```text
Deploy never supports Project selection.
```

A particular operation may support:

```text
platform-server deploy local
    Source Repo selection allowed
```

while rejecting:

```text
platform-server deploy vps
    Source Repo selection forbidden
```

because the VPS deployment recipe requires the complete Platform Server.

---

# Operation Diagnostics

Invalid operation requests should fail with specific explanations of the actual violated invariant.

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

note: Source Repo-scoped deployment is supported by `deploy local`
```

The desired UX principle is:

> **Specific operation legality, specific error, specific safe solution.**

This is particularly important for users who are new not only to Vapor, but also to Rust, Git, game development, or software development generally.

---

# Operation Recipes

Some operation semantics, especially bespoke first-party development/deployment operations, should be configurable rather than permanently encoded as Rust constants.

Vapor may support lightweight authored operation recipes consisting of:

```text
declarative Vapor operation configuration
+
Vapor-native execution primitives
+
small explicit scripts/process calls where appropriate
```

The goal is closer to:

```text
workflow/task description
```

than:

```text
arbitrary application/plugin runtime
```

The exact schema remains unsettled.

The broad ownership model is:

```text
Vapor Core
    defines safe capabilities and execution primitives

authored Vapor metadata
    declares subsystem-specific operation composition

small checked-in scripts
    implement narrow procedural steps where useful

.vapor/
    may contain generated/resolved realization
```

Machine-managed derived state should not casually rewrite human-authored manifest intent.

---

# First-Party Subjects

Some official Vapor facilities have stable bespoke semantic subjects independent of their current source topology.

Examples include:

```text
Client
Platform Server
Examples
```

These may receive dedicated top-level UX such as:

```text
client ...
platform-server ...
examples ...
```

Their backing Source Repo Container/Source Repo topology may evolve without requiring the public facility name to change.

First-party status enables this bespoke semantic treatment.

It does not require every first-party repository to receive a dedicated command namespace.

The trust and authority model determining first-party status is Registry/Root-Authority controlled and is not self-declarable by arbitrary source.

---

# Source Topology vs Rust Topology

Vapor source topology and Rust/Cargo realization are related but distinct.

Vapor topology is:

```text
Source Repo Container
→ Source Repo
→ Project
```

Rust realization beneath or within a Project may include:

```text
Cargo workspace
Cargo package
crate
binary target
example target
...
```

Vapor should use Vapor terminology when discussing Vapor topology.

It should use Rust/Cargo terminology when discussing actual Rust/Cargo objects.

A Source Repo should not be called a Cargo Workspace merely because it also acts as a Vapor Workspace.

---

# Local Realization

The configured Superworkspace contains Vapor's canonical managed local realization of available source identities.

Git state within that realization may legitimately vary:

* branches;
* detached submodule HEADs;
* dirty working trees;
* unpushed commits;
* local modifications;
* provider remotes.

These states are not automatically Vapor corruption.

Vapor must distinguish:

```text
unusual Git state
```

from:

```text
invalid Vapor state
```

Advanced users may maintain additional arbitrary Git checkouts outside the managed Superworkspace.

Those checkouts do not automatically participate in Vapor's canonical local source model merely because their files resemble registered Vapor source.

Future explicit import/relocation mechanisms may extend this model where concrete pressure requires them.

---

# Git State and Reconciliation

Git remains visible and authoritative for Git source state.

Examples include:

```text
missing submodule checkout
dirty repository
unexpected gitlink
detached submodule HEAD
local branch divergence
merge conflict
```

Vapor may diagnose these states in Vapor context.

It should use recognizably Git-oriented vocabulary.

Where corrective Git actions are possible, Vapor should be cautious.

For example:

```text
Source Repo `Registry` is declared as a Git submodule but its
checkout is missing.

inspect:
    git status
    git submodule status

possible reconciliation:
    git submodule update --init -- Registry

warning:
    this changes the local Git checkout. Vapor cannot guarantee that
    applying it is appropriate for your current development state.
```

Vapor should make stronger recommendations only when it can actually prove the action is safe.

---

# Diagnose and Repair Boundary

Diagnosis may inspect:

* Vapor metadata;
* source topology;
* Git state;
* Registry linkage;
* Cargo realization;
* generated state;
* toolchain state;
* IDE integration;
* operation realization.

Repair is narrower.

> **Repair reconstructs or reconciles safely derivable Vapor-owned state.**

Repair must not silently:

* acquire missing authored source;
* clone missing Source Repo Containers;
* reset dirty Git repositories;
* discard local commits;
* restore authored files from Git;
* change branches;
* rewrite authored configuration merely because it differs from generated state.

Repair is most appropriate for:

* generated metadata;
* caches/indexes;
* derived Cargo realization owned by Vapor;
* generated operation realization;
* IDE integration;
* deterministic toolchain/configuration projections;
* other clearly derived Vapor-owned state.

Missing authored source is an availability/acquisition problem.

Missing authored Git topology inside an already-acquired Container is a Git reconciliation problem.

Neither is generic repair.

---

# Frontend Semantics

GUI, CLI, automation, and future shell surfaces share the same identity, context, selection, and operation semantics.

Their interaction mechanics may differ.

## CLI

The CLI favors:

* exact Vapor Paths;
* minimal unambiguous selectors;
* explicit transient context;
* explicit selection;
* deterministic diagnostics.

## GUI / SDK

The GUI may use:

* tree navigation;
* selected items;
* inspectors;
* dialogs;
* visual multi-selection.

Transient GUI selection may be converted into explicit operation selection without changing persistent Vapor context unless the user deliberately performs the equivalent of Open.

## Automation

Automation should prefer:

* full canonical paths where convenient;
* explicit context where needed;
* exact machine-readable selection;
* deterministic non-interactive failures.

All surfaces eventually invoke the same Vapor Core operation model.

---

# Development Session

A frontend may maintain ordinary live interaction state such as:

* selected UI nodes;
* open editors;
* navigation history;
* log panels;
* active documents;
* transient operation selections.

This presentation state is not automatically Vapor's persistent semantic Context.

The Core should own only the session information needed for coherent Vapor behavior.

A GUI may persist useful Resume State, but restored UI state must not silently become operation scope.

Persistent Vapor Context remains a deliberate navigation/resolution convenience.

---

# Core Invariants

* There is one canonical local Vapor Superworkspace.
* The Superworkspace has no Vapor identity.
* The Superworkspace is never an ID segment.
* Superworkspace location is local configuration.
* Canonical source identity begins at Authority.
* The source hierarchy is Authority → Source Repo Container → Source Repo → Project.
* One Source Repo equals one Vapor Workspace.
* A Project is not itself a Git repository.
* Typed Vapor Content kinds are properties of Projects rather than extra path segments.
* A full path may terminate at any hierarchy layer.
* Context may omit only a leading identity prefix.
* Context never fills holes in the middle of a path.
* Minimal unambiguous selectors are allowed.
* Ambiguity is never resolved by guessing.
* Ambiguity diagnostics should enumerate useful exact candidates and safe resolution methods.
* Persistent context is a lightweight resolution convenience.
* Persistent context does not automatically alter operation scope.
* Per-operation context may override persistent context.
* CWD is not ordinary Vapor semantic context.
* The operation subject defines the natural complete scope.
* Explicit selection refines operation scope.
* Parent selection naturally includes its meaningful descendants according to the operation.
* Operation legality is specific to the resolved subject, operation, variant, topology, and authored rules.
* Git problems remain Git-oriented.
* Repair does not reacquire authored source or reset Git state.
* First-party facilities may expose stable bespoke semantics over changing source topology.
* Vapor topology and Rust/Cargo topology remain distinct.

---

# Open Questions

The following remain intentionally unsettled:

* Final CLI spelling of persistent context operations.
* Final CLI spelling of transient context and selection flags.
* Exact grammar for multiple selection in one argument.
* Exact local Superworkspace relocation/configuration operation.
* Exact persistence location/schema for Vapor user-data state.
* Exact authored schema for lightweight operation recipes.
* Exact distinction between first-party trust classification and other Source Repo Container kinds/purposes.
* Exact rules governing independently acquirable Source Repos.
* Exact generic command vocabulary for Project-kind-independent existing-object operations.
* Exact representation of non-Content/internal Projects in first-party subsystems.
* Exact interaction between Vapor Project scope and deeper Cargo workspace/package/target selectors.
* Exact GUI terminology for navigation state which must remain distinct from persistent Vapor Context.
