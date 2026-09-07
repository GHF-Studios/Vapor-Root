> [!info]
> This document defines Vapor's normative model for structural identity, local development realizations, Superworkspaces, development sessions, open/focused state, target selection, ambiguity, and frontend context resolution.
>
> It applies across Vapor Core, the Launcher-integrated Vapor SDK, CLI surfaces, automation, external IDE integration, and future Vapor Shell workflows.
>
> The **Vapor Development Experience Model** describes broader development workflows.
>
> The **Vapor SDK Experience Model** describes the primary graphical projection of this model.
>
> The **Vapor CLI Model** describes its command-line projection.

---

# Core Principle

Vapor must allow users and tools to identify intended work with as little information as is safely sufficient.

This convenience must not weaken semantic identity.

The central rule is:

> **Vapor uses strong canonical identities internally and accepts weaker contextual selectors only when they resolve unambiguously.**

A short name is therefore not secretly promoted into a globally unique ID.

Instead:

```text
human input
    ↓
selector
    ↓
contextual resolution
    ↓
exact canonical identity / exact local realization
    ↓
Vapor Core operation
```

If the supplied information does not identify a valid target unambiguously for the requested operation, Vapor does not guess.

---

# Structural Development Topology

The normal development topology is:

```text
Vapor Superworkspace
└── Container Repo
    └── Source Repo / Vapor Workspace
        └── Vapor Project
            └── zero or more Vapor Content artifacts
```

These layers do not all have the same kind of identity.

The Superworkspace is primarily a **local physical development container**.

Container Repos, Workspaces, Projects, and Vapor Content participate in stronger semantic or structural identity models.

The distinction is intentional.

See [Vapor Development Context Model](./Diagrams/Vapor%20Development%20Context%20Model.puml).

---

# Vapor Superworkspace

A **Vapor Superworkspace** is a local development container comparable in purpose to an IDE workspace containing several related repositories/projects.

It provides a place in which the Vapor SDK can understand and present a larger development environment.

A Superworkspace:

* is not itself a Git repository;
* is not a canonical remote source unit;
* is not part of a Workspace or Content identity;
* may contain one or more Container Repos;
* may contain independently checked-out Vapor Workspaces where useful;
* may contain several local realizations relevant to one development environment;
* may contain unrelated filesystem entries which Vapor does not own or need to interpret.

Two independently created directories containing equivalent repository checkouts are two different Superworkspaces.

There is no requirement that they share a globally meaningful Superworkspace identity.

---

# Configured and Implicit Superworkspaces

A Superworkspace does not need a manifest merely to exist.

Vapor may recognize an **implicit Superworkspace** from the development structures it contains.

A `Superworkspace.vapor.toml` may optionally turn that local container into a **configured Superworkspace**.

That file exists for durable local SDK/development configuration.

Appropriate Superworkspace-local information may include:

* a human-friendly display name;
* explicitly attached local roots;
* local aliases;
* preferred presentation;
* SDK layout/configuration;
* local recovery hints;
* other intentionally local development preferences.

It must not become the canonical source of truth for:

* Workspace identity;
* Project identity;
* Content identity;
* Container Repo membership;
* Git history;
* publication state;
* Registry state;
* authority;
* remote provider truth.

A missing `Superworkspace.vapor.toml` should therefore degrade a configured Superworkspace into a less-configured but still discoverable development container wherever the contained source remains recognizable.

The file is durable local configuration, not irreplaceable ecosystem truth.

Superworkspace-local files such as `.vaporignore` likewise affect the local development view rather than canonical ecosystem membership.

---

# Container Repo

A **Container Repo** is a Vapor-managed Git repository which groups related Vapor Workspaces.

A Container Repo participates in the modeled source topology.

A Container Repo may declare its Source Repo / Workspace members through Git submodules.

A Container Repo is not itself used as a submodule of another Container Repo.

Conceptually:

```text
GHF-Studios/Vapor-Client
```

may be the structural address of the `Vapor-Client` Container Repo within the `GHF-Studios` namespace.

Container Repo membership is meaningful modeled structure rather than merely a consequence of current filesystem nesting.

However, Container Repo containment is not automatically part of a child Workspace's durable semantic identity.

---

# Source Repo / Vapor Workspace

A **Source Repo** and a **Vapor Workspace** are two views of the same source-bearing unit.

The normative relationship is:

> **One Source Repo = one Vapor Workspace.**

A Vapor Workspace:

* is a Git repository;
* has a durable semantic identity;
* may belong to a Container Repo in the current source topology;
* contains one or more Vapor Projects;
* owns Workspace-level authored metadata;
* may have one or more local realizations/checkouts.

The provider repository URL is linkage information.

Its Container Repo placement is structural information.

Neither should be confused automatically with the Workspace's durable semantic identity.

---

# Vapor Project

A **Vapor Project** is a structural development unit inside one Vapor Workspace.

For Rust-backed development it normally corresponds to a coherent Cargo workspace/project execution context.

A Vapor Project:

* is not itself a Git repository;
* has identity within the semantic namespace of its Workspace;
* should have a meaningful local name;
* may contain zero, one, or multiple Vapor Content artifacts;
* may contain supporting Rust packages which are not independently Vapor Content.

Project identity and Content identity remain distinct.

For example, an examples Project may host several independently identified Libraries, Games, Mods, and Packagepacks.

The Project answers:

> Where and within which development unit is this work authored/built?

The Content identity answers:

> Which semantic Vapor artifact is this?

Those are related questions, not the same question.

---

# Identity, Address, and Realization

Vapor distinguishes three related but non-equivalent concepts:

```text
Semantic Identity
Structural Address
Local Realization
```

This distinction is fundamental.

---

# Semantic Identity

A **Semantic Identity** describes which modeled Vapor object something is.

Semantic identity should survive changes which do not conceptually create a new object.

Examples may include:

* moving a Workspace to another Container Repo;
* renaming a Container Repo around it;
* relocating a checkout;
* changing Git provider URL;
* checking out a different local realization.

For a Workspace, a conceptual semantic identity may resemble:

```text
GHF-Studios/Vapor
```

For a Project inside that Workspace:

```text
GHF-Studios/Vapor/Client
```

The exact persisted representation of non-Content semantic identities remains subject to implementation work.

The important invariant is that current structural placement must not be mistaken for durable identity.

---

# Structural Address

A **Structural Address** describes where an object is located in the currently modeled source topology.

Structural addresses are:

* human-readable;
* case-preserving;
* slash-separated;
* hierarchical.

A conceptual structural form is:

```text
Namespace/Container-Repo/Workspace/Project
```

For example:

```text
GHF-Studios/Vapor-Client
GHF-Studios/Vapor-Client/Vapor
GHF-Studios/Vapor-Client/Vapor/Client
```

These describe, respectively:

```text
Container Repo
Workspace placement
Project placement
```

A Superworkspace is deliberately absent.

It is local development storage rather than canonical source topology.

---

# Identity vs Structural Address

A semantic identity and structural address may look similar.

They answer different questions.

Example:

```text
Workspace semantic identity:
    GHF-Studios/Vapor

Workspace structural address:
    GHF-Studios/Vapor-Client/Vapor
```

If the Container Repo is renamed:

```text
Vapor-Root
→ Vapor-Client
```

the Workspace may remain:

```text
GHF-Studios/Vapor
```

while its structural address migrates:

```text
GHF-Studios/Vapor-Root/Vapor
→
GHF-Studios/Vapor-Client/Vapor
```

This is a topology migration, not necessarily destruction/recreation of the Workspace identity.

---

# Structural Address Segments

Each address segment should carry actual structural distinction.

Accidental repetition should not be introduced merely because several layers historically share a name.

For example:

```text
GHF-Studios/Vapor-Client/Vapor/Vapor
```

may be a valid structural address only if the final `Vapor` is genuinely the meaningful Project name.

It should not arise merely because the current Cargo workspace copied its repository name.

Project names should describe their actual development responsibility.

---

# Structural Movement

Moving an object in source topology changes its Structural Address.

It does not automatically change Semantic Identity.

Therefore:

```text
GHF-Studios/Vapor-Client/Vapor
```

and:

```text
GHF-Studios/Other-Container/Vapor
```

may describe two structural placements of the same semantic Workspace identity at different points in time or, where the model permits, different realizations.

Structural migration must be explicit when durable state refers to old addresses.

Vapor should be capable of migrating remembered structural references without pretending the semantic object became unrelated.

---

# Semantic Rename

Changing the semantic identity/name of an object is different from changing only its Structural Address.

For example:

```text
Workspace ID:
    GHF-Studios/Vapor
→
    GHF-Studios/Vapor-NG
```

is an identity-level rename/migration.

That operation may have stronger compatibility, publication, or reference consequences.

The exact allowed rename rules vary by object kind.

In particular, immutable published Vapor Content identity remains subject to stricter Content rules.

---

# Vapor Content Identity

Vapor Content maintains its established semantic identity model.

Content identity is not mechanically derived from Project Structural Address.

A single Project may host several Content IDs.

A Content artifact may therefore be selected semantically without requiring the user to identify its physical Project first when Vapor can resolve that relationship.

Content identity may be stricter than development-structure identity.

Published Content IDs remain immutable unless a future explicit migration model says otherwise.

---

# Local Realization

A **Local Realization** is one physical local realization of a semantic source/development object.

Conceptually:

```text
Workspace semantic identity:
    GHF-Studios/Vapor

Structural address:
    GHF-Studios/Vapor-Client/Vapor

Realization A:
    provider: GHF-Studios/Vapor
    branch: main
    path: /some/local/path

Realization B:
    provider: developer fork
    branch: experiment
    path: /another/local/path
```

Both local realizations may correspond to the same semantic Workspace.

Filesystem path, provider URL, branch, and checkout state are realization properties.

They are not semantic identity.

A realization may move physically while preserving both semantic identity and structural role once Vapor safely relocates/recognizes it.

# Multiple Realizations

A Superworkspace may contain multiple realizations of the same canonical Workspace identity.

Examples include:

* upstream and fork checkouts;
* different working branches;
* isolated experimental checkouts;
* migration/testing realizations.

When an operation requires a physical source target, identifying only the semantic Workspace may still be ambiguous if multiple usable realizations exist.

Vapor must then resolve the realization as well.

The UX may distinguish realizations using information such as:

* local alias;
* provider/repository linkage;
* remote;
* branch;
* checkout status;
* local realization key.

The exact persisted realization-key format remains an implementation concern.

Raw filesystem path should remain an escape hatch and diagnostic property rather than the preferred everyday selector.

---

# Fork Semantics

A Git fork does not automatically require a new Vapor Workspace identity.

A fork may initially be another source realization of the same semantic Workspace.

If a fork intentionally becomes an independently named/published Vapor Workspace, that is a separate semantic act.

Provider ownership and Vapor identity must not be conflated automatically.

---

# Names and Selectors

A **Canonical Identity** is exact.

A **Selector** is user/frontend input used to find one or more exact targets.

Examples of possible Project selectors include:

```text
GHF-Studios/Vapor-Root/Vapor/Client
Vapor-Root/Vapor/Client
Vapor/Client
Client
```

The shorter forms are not alternative canonical IDs.

They are selectors.

Vapor may accept the shortest exact selector which is unambiguous inside the effective context.

This is the **minimal information principle**:

> **Require no more identifying information than is necessary to resolve the requested target safely.**

---

# Selector Matching

Selector convenience must remain deterministic.

Normal shorthand matching should be based on modeled names, identity segments, scopes, aliases, and exact known relationships.

It should not depend on fuzzy semantic guessing such as:

> This object has a similar name, so the user probably meant it.

If multiple candidates remain, resolution fails or the frontend asks the user to choose.

---

# State Is Multidimensional

Development-object state must not be represented as one enum mixing unrelated concepts.

At minimum, Vapor distinguishes several axes.

## Knowledge

```text
Unknown
Known
```

A Known object has durable identity/context known to Vapor.

## Availability

Examples include:

```text
Available
Missing
Unreachable
```

Availability describes whether the relevant realization can currently be accessed.

## Health

Examples include:

```text
Healthy
Degraded
Invalid
Incompatible
```

Health describes whether an available or remembered object is currently usable according to Vapor's model.

The exact diagnostic taxonomy may grow over time.

## Session Participation

```text
Closed
Open
```

An Open object participates in the current development working set.

## Focus

```text
Unfocused
Focused
```

Focus expresses durable targeting intent inside the current development session.

These dimensions may combine.

For example:

```text
Known
Missing
Open
Focused
```

is valid.

A Workspace which disappeared from disk does not become magically Closed merely because it is unhealthy.

This allows the SDK to preserve the user's mental workspace and offer recovery.

---

# Open

**Open** means that a structural or semantic object participates in the current development working set.

Several sibling Workspaces, Projects, or Content objects may be Open simultaneously.

Opening is not equivalent to globally activating exactly one object.

Opening a child requires an enclosing resolvable hierarchy.

When the user explicitly opens a deep target, Vapor may automatically open its exact required ancestors.

For example, opening:

```text
GHF-Studios/Vapor-Root/Vapor/Client
```

may establish the relevant:

```text
Container Repo
Workspace
Project
```

working context automatically.

This is safe because the ancestors are exact structural parents rather than guessed analogues.

---

# Focus

**Focus** expresses default targeting intent.

Multiple objects may be Focused simultaneously.

This is deliberate.

Vapor must not enforce artificial global uniqueness merely to simplify CLI implementation.

Instead, each operation declares the target cardinality it accepts.

Focus does not mean that every operation automatically applies to every focused object.

The requested operation determines whether multiplicity is meaningful.

---

# Transient Selection

**Selection** is different from Focus.

Selection is immediate frontend interaction used for a particular action.

Examples include:

* clicking one tree item;
* multi-selecting three Projects;
* choosing one graph node;
* invoking an Inspector action on the currently selected Content.

Selection may target an operation without changing durable session Focus.

This distinction is particularly important in the GUI.

A user should be able to inspect or operate on something without accidentally rewriting the entire remembered development context.

---

# Focus Does Not Guess Analogues

Vapor must never silently replace one focused object with another merely because the replacement:

* has the same local name;
* has the same Content kind;
* occupies a similar position;
* appears to be the equivalent object under another parent.

For example, switching from one Packagepack to another does not imply:

```text
focused Game A
    ↓
automatically focus Game B
```

Exact remembered identities may be restored.

Analogical replacement is not identity resolution.

---

# Parent and Descendant Relationships

Open and Focus state must respect structural containment.

A child cannot be newly established without a resolvable parent scope.

Closing a parent closes its Open descendants.

Focus on descendants is cleared when those descendants are closed.

This should be implemented as a hierarchy invariant rather than scattered field cleanup.

---

# Closing

Closing is a session operation.

It does not mean:

* forget;
* delete;
* remove source;
* uninstall;
* discard Git history.

Those are separate concepts.

Closing a parent cascades through its Open descendants.

The frontend should surface that consequence.

For example:

```text
Closed Workspace GHF-Studios/Vapor-Root/Vapor.

Also closed:
    Project Client
    Project SDK
```

---

# Closing Safety

Cascade behavior should distinguish context-only effects from meaningful volatile state.

A reasonable model is:

## Context-only cascade

If closing only changes session/open/focus state:

```text
close immediately
→ summarize descendants closed
```

## Live but safely interruptible activity

If closing would stop active development processes or otherwise interrupt ongoing work:

```text
GUI / interactive frontend
    → confirmation

non-interactive frontend
    → explicit refusal or required override
```

## Potentially destructive or unrecoverable state

If closing would cause loss of unsaved in-application edits or other non-recoverable work:

```text
refuse by default
→ require explicit acknowledgement / force semantics
```

Authored Git source must never be destroyed merely because its context is closed.

---

# Forget, Remove, Delete, and Uninstall

These concepts must remain distinguishable from Close.

Conceptually:

```text
Close
    remove from current working session

Forget
    remove remembered local knowledge/configuration

Remove
    remove a managed local realization/artifact

Delete
    destructive removal of authored/local data where explicitly supported

Uninstall
    remove an installed product/tooling capability
```

The final command vocabulary may evolve, but these effects must not be conflated.

---

# Development Session

A **Development Session** is a frontend-local working context containing Open and Focus state and other active development interaction state.

The session is not itself the semantic identity of the things being worked on.

A live SDK instance has a Development Session.

A future Vapor Shell may have its own Development Session.

One-shot CLI invocation may construct a short-lived ephemeral session.

---

# Live Session State

Live Session State may include:

* Open Superworkspaces;
* Open Workspaces;
* Open Projects;
* Open Content contexts;
* Focus sets;
* transient operation context;
* selected run/test targets;
* UI navigation state where semantically useful.

Not all UI presentation details belong in Vapor Core.

The Core should own only the state required for coherent Vapor behavior.

---

# Resume State

**Resume State** is durable state used to reconstruct a useful future session.

Resume State is not necessarily a continuously shared global live session.

A GUI may persist enough information so reopening Vapor tomorrow restores:

* the previous configured Superworkspace;
* previously Open Workspaces/Projects;
* previous exact Focus;
* useful SDK navigation state.

If a remembered realization is now missing, the identity may remain remembered and unhealthy rather than being silently discarded.

---

# Multi-Frontend Sessions

Vapor should not initially require all simultaneous frontends to share one mutable global focus cursor.

A GUI, future Vapor Shell, and one-shot CLI may have different live interaction state.

The initial invariant is:

> **Frontends share Vapor Core semantics and canonical identity, not necessarily one globally mutable live selection.**

Exact cross-process live-session synchronization remains an implementation question.

---

# Frontend Equality

GUI/CLI equality means **capability and semantic equality**, not identical interaction mechanics.

Different frontends may construct the same Core operation differently.

```text
GUI
    click / tree / inspector / multi-select
        ↓
selectors + session + explicit targets

CLI
    arguments / shorthand / current directory
        ↓
selectors + ephemeral context

Automation / Codex
    canonical IDs / structured requests
        ↓
selectors + explicit context

                    ↓
              Vapor Core
```

Vapor Core should not contain separate business semantics for each frontend.

---

# Operation Target Specification

Every Vapor Core operation that acts on selectable objects should conceptually define:

* accepted target kind(s);
* target cardinality;
* required availability/health;
* required Role;
* required authority where external protected targets are involved.

Example cardinalities include:

```text
ExactlyOne<Project>
ZeroOrOne<Project>
Many<Project>

ExactlyOne<Packagepack>
Many<Content>
```

The exact Rust representation is an implementation concern.

The semantic requirement is not.

---

# Cardinality Resolution

After candidate resolution:

```text
0 candidates
    → unresolved

1 candidate
    → valid for ExactlyOne and Many

N candidates
    → valid only when the operation accepts multiplicity
```

If an operation requires exactly one target and several candidates remain, Vapor reports ambiguity.

It does not arbitrarily choose one.

---

# Context Resolution

Context resolution combines explicit intent, session state, scopes, and frontend-specific ambient hints.

See [Vapor Selection Resolution Model](./Diagrams/Vapor%20Selection%20Resolution%20Model.puml).

The precise implementation may evolve, but the conceptual precedence is:

```text
explicit operation target
    ↓
transient frontend selection
    ↓
relevant Focus
    ↓
explicit/open enclosing scopes
    ↓
safe frontend-specific ambient anchor
    ↓
unique usable candidate
    ↓
ambiguity / unresolved
```

Resume State normally seeds a session rather than acting as a hidden final override on every operation.

---

# Explicit Targets

Explicit targets have the strongest ordinary selection authority.

An explicit target may be:

* a canonical ID;
* a qualified selector;
* a local selector whose enclosing context makes it unique;
* an explicit local realization selector;
* an advanced raw path where the operation deliberately supports one.

Explicit selectors do not need to mutate durable Focus.

A frontend may offer a separate “Focus” or “Open” operation when persistence is intended.

---

# Enclosing Scope

Short selectors gain meaning from enclosing scopes.

For example:

```text
Client
```

may identify exactly one Project inside a focused/open Workspace.

If several Workspaces provide Projects named `Client` and no stronger context distinguishes them, the selector is ambiguous.

The user may then provide more information:

```text
Vapor/Client
```

or:

```text
GHF-Studios/Vapor-Root/Vapor/Client
```

The shortest sufficient selector wins ergonomically.

The canonical identity wins internally.

---

# Current Working Directory

The current working directory is a CLI-specific physical hint.

It is not canonical Vapor state.

CWD may:

* identify a local realization;
* narrow an ephemeral CLI context;
* allow a command issued inside a known Project to use that Project naturally.

CWD must not:

* silently rewrite persisted SDK Focus;
* become a canonical identity;
* be required when Vapor already possesses stronger modeled context;
* cause arbitrary path-relative semantics to leak into ordinary Vapor identity workflows.

The principle is:

> **CWD may resolve one CLI operation. CWD does not silently redefine the user's durable Vapor context.**

---

# GUI Context

The SDK has richer explicit interaction evidence than the CLI.

Examples include:

* selected tree items;
* current Inspector target;
* selected graph nodes;
* active editor/document;
* focused Workspaces/Projects;
* selected run configuration.

The GUI should use that information directly rather than forcing the user to type identities that the interface already knows.

---

# CLI Context

The CLI should remain deterministic and automation-friendly.

It should prefer:

* canonical IDs;
* minimal unambiguous selectors;
* explicit operation options;
* useful structured diagnostics.

It should not attempt to reproduce every GUI interaction as shell syntax.

Interactive ambiguity selection may be offered selectively in human-oriented contexts, but non-interactive behavior must remain deterministic.

---

# Automation Context

Automation should be able to bypass most human shorthand.

Codex and other tooling should be able to use:

* canonical identities;
* exact local realization keys;
* machine-readable status;
* deterministic operation results.

The same underlying operations remain available to humans through richer GUI workflows.

---

# Ambiguity

Ambiguity is normal information, not an exceptional architectural failure.

A useful ambiguity diagnostic should explain the candidates and the missing distinction.

For example:

```text
Project selector `Client` is ambiguous.

Candidates:
    GHF-Studios/Vapor-Root/Vapor/Client
    Leslie/Vapor-Fork/Vapor/Client

Provide a more qualified selector or focus an enclosing Workspace.
```

A GUI should normally present the equivalent choice visually.

---

# Missing and Broken Realizations

A remembered Open/Focused object may become unavailable because:

* a directory was moved;
* a drive is disconnected;
* a checkout was deleted externally;
* Git state became incompatible;
* required metadata became invalid.

Vapor should preserve the remembered identity where useful and expose recovery actions such as:

```text
Locate
Repair
Reacquire
Close
Forget
```

The exact available actions depend on the failure.

A broken object should not simply disappear from the SDK merely because its realization cannot currently be opened.

---

# External Filesystem Changes

Vapor must assume that advanced users and external tools may manipulate repositories and directories.

Diagnosis may attempt safe rediscovery using:

* canonical identity;
* known Superworkspace membership;
* repository metadata;
* provider linkage;
* previously known realization information.

Automatic repair must remain conservative.

Vapor must not silently attach an unrelated checkout merely because its directory name looks similar.

---

# Worked Journey — Resume SDK

Yesterday:

```text
Superworkspace: Vapor Development
Open:
    GHF-Studios/Vapor-Root/Vapor
    GHF-Studios/Vapor-Root/Vapor-Examples

Focused:
    GHF-Studios/Vapor-Root/Vapor/Client
    GHF-Studios/Vapor-Root/Vapor-Examples/Examples
```

The SDK closes normally.

Enough state is persisted as Resume State.

Tomorrow the Launcher enters SDK mode.

Vapor reconstructs the previous working set.

If all realizations remain healthy, the SDK returns to that state.

No shell working directory is required.

---

# Worked Journey — Missing Workspace

The user previously had:

```text
GHF-Studios/Vapor-Root/Vapor-Examples
```

Open and Focused.

The directory is moved externally.

The next SDK session shows the Workspace in its previous hierarchy with a Missing diagnostic.

The user may:

```text
Locate
Reacquire
Close
Forget
```

The Workspace is not silently replaced by another directory named `Vapor-Examples`.

---

# Worked Journey — Multiple Projects Focused

The user opens and focuses:

```text
GHF-Studios/Vapor-Root/Vapor/Client
GHF-Studios/Vapor-Root/Vapor-Examples/Examples
```

A multi-target diagnostic operation may operate over both.

An operation requiring exactly one Cargo execution Project reports ambiguity unless:

* the user explicitly selects one;
* transient GUI selection supplies one;
* another exact operation scope identifies one.

Vapor does not globally forbid multiple Focus merely because some operations are singular.

---

# Worked Journey — CLI from Inside a Project

The user enters a known Project directory and runs:

```text
vapor toolchain cargo -- test
```

The CLI may use CWD as an ephemeral anchor to identify that Project realization.

The operation runs there.

The SDK's persisted Focus is not silently changed.

---

# Worked Journey — CLI from an Unrelated Directory

The user runs:

```text
vapor library resolve Wheel-Rules
```

from the home directory.

CWD provides no useful Vapor anchor.

The ephemeral CLI session uses modeled Open/Focused/Resume context.

If `Wheel-Rules` uniquely identifies the intended Library within that context, it resolves.

If not, Vapor reports the candidates and requests a stronger selector.

No `--root ../../some/path` should be required on the ordinary happy path.

---

# Worked Journey — Open Deep Identity

The user explicitly opens:

```text
GHF-Studios/Vapor-Root/Vapor/Client
```

while no development hierarchy is currently Open.

Vapor resolves the exact identity and local realization.

It may automatically Open the required containing Workspace/Container context.

This is not guesswork because the ancestor relationship is part of canonical structure.

---

# Worked Journey — Switch Packagepack

The user had Game A focused while working with Packagepack A.

The user then selects Packagepack B.

Vapor does not automatically focus Game B merely because both Packagepacks contain exactly one Game.

The user may:

* keep the previous focus where still meaningful;
* explicitly select Game B;
* restore an exact remembered Game B focus if one exists for that exact identity.

Equivalent role is not equivalent identity.

---

# Core Invariants

* Superworkspace is local development storage, not canonical ecosystem identity.
* `Superworkspace.vapor.toml` is optional local configuration rather than semantic source truth.
* Container Repo identity participates in child structural identity.
* One Source Repo equals one Vapor Workspace.
* A Vapor Project is namespaced by its Workspace.
* A Project may host multiple Vapor Content artifacts.
* Project identity and Content identity are distinct.
* Structural identities are case-preserving and slash-separated.
* Local filesystem paths are realization properties rather than semantic identities.
* Multiple realizations of one semantic Workspace may coexist.
* Short names are selectors, not secretly global IDs.
* Vapor accepts minimal unambiguous selectors.
* Ambiguity never resolves by arbitrary guessing.
* State is multidimensional.
* Open and Focus are distinct.
* Selection and Focus are distinct.
* Multiple objects may be Focused.
* Operation cardinality determines whether multiplicity is valid.
* Opening a child may establish exact ancestors.
* Changing context never performs analogical child replacement.
* Closing a parent closes its Open descendants.
* Close is distinct from Forget, Remove, Delete, and Uninstall.
* Missing/broken remembered objects may remain Open/Focused for recovery.
* Live Session State and durable Resume State are distinct.
* GUI, CLI, automation, and future Shell surfaces call shared Vapor Core semantics.
* CWD is a CLI hint rather than durable Vapor state.

---

# Open Questions

The following remain intentionally open until implementation or UX pressure requires them:

* Exact `Superworkspace.vapor.toml` schema.
* Exact local realization-key persistence format.
* Exact CLI spelling for Open, Focus, Close, Forget, and realization selection.
* Exact cross-process synchronization of simultaneous live frontend sessions.
* Exact Project decomposition/naming inside the Vapor Root Workspace.
* Exact migration rules for existing lowercase structural/workspace metadata.
* Exact relationship between current Content ID casing conventions and future case-preserving identity conventions.
* Exact provider/fork display and realization-alias UX.
* Exact thresholds for confirmation versus immediate cascade on Close.
* Whether human CLI mode offers optional interactive ambiguity selection.
