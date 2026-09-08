> [!info]
> This document defines the semantic command-line model shared by Vapor's CLI surfaces.
>
> It describes command hierarchy, operation subjects, contextual resolution, explicit selection, first-party bespoke command surfaces, source-topology commands, typed creation, diagnostics, and the relationship between Vapor CLI semantics and underlying Git/Rust/Cargo tooling.
>
> Canonical identity, Superworkspace, Context, Selection, and operation-resolution semantics are defined normatively by the **Vapor Context, Identity, Session And Selection Model**.
>
> Broader authoring and development workflows are defined by the **Vapor Development Experience Model**.
>
> Exact command spelling may still evolve where explicitly marked open.

---

# Core Principle

The Vapor CLI should express the work the user intends to perform in Vapor terms.

It should neither:

* mirror Rust module structure;
* mirror Cargo structure;
* mirror Git repository implementation mechanically;
* invent generic namespaces merely to organize commands cosmetically;
* force users to restate information Vapor already knows from resolved identity.

The governing rule is:

> **Use explicit semantic nouns where the noun itself matters, and generic operations where the target identity already provides the missing semantic type.**

This deliberately produces an asymmetric command language.

That asymmetry is desirable when it reflects real semantics.

---

# Command Shape

Three major command shapes exist.

## Generic System Domains

Stable cross-cutting Vapor concepts may own generic namespaces.

Examples include:

```text
installation
role
authority
toolchain
source-repo-container
source-repo
```

These namespaces exist because the objects beneath them share meaningful generic semantics.

---

## Bespoke First-Party Facilities

Some trusted first-party Vapor facilities own dedicated command namespaces because their lifecycle and operation semantics are inherently bespoke.

Current intended examples include:

```text
client
platform-server
examples
```

These namespaces represent semantic facilities.

They are not merely aliases for particular Git directories.

Their underlying source topology may evolve without requiring their public names to change.

---

## Existing-Object Operations

Once a normal Vapor object already exists, Vapor can resolve its identity and determine its kind.

Therefore ordinary existing-object operations should not require users to redundantly restate that kind.

Conceptually:

```text
vapor test <TARGET>
vapor inspect <TARGET>
vapor publish <TARGET>
```

is preferable to requiring:

```text
vapor game test <TARGET>
vapor game inspect <TARGET>
vapor game publish <TARGET>
```

when `<TARGET>` already resolves to a Game Project.

Creation is different because the object does not yet exist.

---

# Typed Creation

Creation requires the user to state what kind of Project they intend to create.

Examples include:

```text
vapor packagepack create ...
vapor enginepack create ...
vapor gamepack create ...
vapor modpack create ...

vapor engine create ...
vapor game create ...
vapor engine-mod create ...
vapor game-mod create ...
vapor extension-mod create ...
vapor library create ...
```

Creation syntax supplies semantic information unavailable from a nonexistent target.

Conceptually:

```text
game create <NEW-PROJECT-PATH>
    ↓
create Project
kind = Game
```

After the Project exists:

```text
test <PROJECT>
inspect <PROJECT>
publish <PROJECT>
```

may resolve the Project and learn:

```text
kind = Game
```

without requiring the caller to repeat `game`.

---

# Canonical Source Hierarchy

The CLI follows the canonical hierarchy:

```text
Authority
└── Source Repo Container
    └── Source Repo
        └── Project
```

A complete Project path has the form:

```text
Authority/Source-Repo-Container/Source-Repo/Project
```

For example:

```text
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

The hierarchy is structural.

The Superworkspace is not part of this path.

---

# Superworkspace

Vapor has one canonical local Superworkspace.

The Superworkspace:

* has no Vapor identity;
* is not supplied as an argument to ordinary create/acquire commands;
* has a known local location;
* provides the local realization root for Source Repo Containers.

Therefore ordinary acquisition should not look like:

```text
vapor source-repo-container acquire GHF-Studios/Loo-Cast /some/random/path
```

Instead:

```text
vapor source-repo-container acquire GHF-Studios/Loo-Cast
```

places the acquired Container at its canonical location beneath the configured Superworkspace.

Superworkspace relocation/configuration is a separate local-development-environment concern.

---

# Vapor Paths

CLI targets should normally use Vapor Paths or unambiguous selectors.

A full path may identify any hierarchy layer:

```text
GHF-Studios
GHF-Studios/Loo-Cast
GHF-Studios/Loo-Cast/Game
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

These identify:

```text
Authority
Source Repo Container
Source Repo
Project
```

respectively.

The operation determines which target kinds it accepts.

Vapor does not automatically descend to an arbitrary leaf merely because a parent path was supplied.

---

# Minimal Information Principle

The CLI should require no more information than is safely necessary.

The rule is:

> **Accept the shortest selector that resolves unambiguously within the applicable operation context.**

A full Project path may be:

```text
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

but shorter input may be accepted when enough surrounding information is already known.

This shorthand never creates an alternative canonical identity.

It is only a selector which resolves to one exact canonical object.

---

# Context, Subject, and Selection

The CLI resolution model follows:

```text
context  →  operation subject  →  selection
  left            middle            right
```

These are different concepts.

## Context

Context supplies missing hierarchy to the left.

## Operation Subject

The command establishes what the operation is fundamentally about.

## Selection

Selection optionally supplies/narrows descendants to the right.

This model applies whether the operation subject comes from a bespoke noun or an explicit target argument.

---

# Persistent Context

Persistent Context is a lightweight convenience.

For example, conceptually:

```text
vapor source-repo open GHF-Studios/Loo-Cast/Game
```

may establish:

```text
GHF-Studios/Loo-Cast/Game
```

as the current Vapor resolution prefix.

A later selector:

```text
Loo-Cast
```

may therefore resolve to:

```text
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

Persistent Context must not be required for Vapor to function.

A fully specified or otherwise unambiguous command works without it.

---

# Context Does Not Imply Operation Scope

This is a fundamental safety rule.

Suppose the current persistent Context is:

```text
GHF-Studios/Vapor-Platform-Server/Registry
```

Then:

```text
vapor platform-server test
```

still means:

> Test the complete natural Vapor Platform Server subject.

It does not silently become:

```text
test only Registry
```

Likewise:

```text
vapor platform-server deploy vps
```

must not become a Registry-only deployment because the user happened to navigate there previously.

Context affects name/path resolution.

Selection affects operation scope.

---

# Transient Context Override

A single CLI invocation must be able to override persistent Context without changing it.

The intended generic shape is approximately:

```text
--context <VAPOR-PATH>
```

For example:

```text
vapor inspect My-Game \
    --context GHF-Studios/My-Stuff/Game
```

may resolve:

```text
My-Game
```

as:

```text
GHF-Studios/My-Stuff/Game/My-Game
```

The transient Context exists only for this invocation.

It does not rewrite persistent state.

The exact final flag spelling remains open, but the semantic capability is required.

---

# Context Resolution Rules

Conceptually:

```text
full exact path
    → use directly

relative selector + explicit transient context
    → resolve against transient context

relative selector + persistent context
    → resolve against persistent context

selector uniquely resolvable from operation subject
    → use unique candidate

otherwise
    → ambiguity / unresolved diagnostic
```

A context-relative path may omit only a complete leading prefix.

It may not omit middle hierarchy segments.

---

# Current Working Directory

CWD is not ordinary Vapor Context.

The CLI must not depend on shell location for semantic Vapor targeting when stronger modeled information exists.

A developer may be physically inside:

```text
some unrelated Git repository
```

while intentionally operating on:

```text
Vapor Client
Vapor Platform Server
another Project in the Superworkspace
```

Therefore:

> **Filesystem location and Vapor semantic location are separate.**

Raw filesystem paths remain valid where the physical filesystem itself is the operation's subject, such as:

* explicit import/bootstrap operations;
* low-level diagnostics;
* locating externally moved files;
* underlying Git/Cargo use.

CWD may still naturally matter to Git or Cargo themselves when those tools are used directly.

It is not Vapor's canonical semantic navigation mechanism.

---

# Explicit Selection

An operation may support explicit selection beneath its natural subject.

The intended generic shape is approximately:

```text
--select <SELECTION>
```

For example:

```text
vapor platform-server test \
    --select "Registry"
```

selects the `Registry` Source Repo scope beneath Platform Server.

And:

```text
vapor platform-server test \
    --select "Registry/core"
```

may select Project `core` beneath that Source Repo.

The selector itself expresses topology.

The user does not need to redundantly say:

```text
--source-repo Registry
--project core
```

when the resolved path already tells Vapor which object kinds were selected.

---

# Multiple Selection

Where an operation supports several explicit selections, Vapor should prefer one coherent selection expression rather than requiring excessive repeated flags.

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

The exact quoting/list delimiter remains open.

Vapor Core should receive exact resolved selections rather than command-line syntax.

---

# Hierarchical Selection

Selection is hierarchical.

Selecting a parent node selects that operation scope together with whatever descendants the operation considers part of that scope.

For example:

```text
--select "Registry"
```

does not need a second generic:

```text
--all-projects
```

flag merely to indicate that Registry's relevant descendants participate.

The operation definition decides what performing that operation at Source Repo scope actually means.

Likewise, no explicit selector means:

> use the natural complete operation subject

rather than:

> secretly act as though `--all` was supplied.

This distinction is important.

---

# Natural Subject Scope

Every operation has a natural complete subject.

For example:

```text
vapor platform-server test
```

means:

> Test Vapor Platform Server.

It does not mechanically mean:

```text
run `test` once against every Project
```

The Platform Server test operation may include:

* Project tests;
* integration tests;
* generated validation;
* service health checks;
* other authored steps.

The operation recipe defines the actual realization.

---

# Operation-Specific Legality

The existence of generic selection syntax does not imply every operation accepts every selection shape.

Whether a selection is legal may depend on:

```text
subject
operation
operation variant
deployment/run target
selected topology
authored operation recipe
Project/Source Repo relationships
first-party subsystem semantics
```

Therefore Vapor must avoid universal assumptions such as:

```text
deployment can never target a Project
```

A particular operation may support:

```text
platform-server deploy local
    selected Source Repo allowed
```

while rejecting:

```text
platform-server deploy vps
    selected Source Repo forbidden
```

because VPS deployment requires the whole configured deployment unit.

---

# Specific Failure Diagnostics

Invalid operation requests should explain the actual violated rule.

Bad:

```text
error: invalid scope
```

Good:

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

The UX principle is:

> **Specific operation legality → specific error → specific safe resolution.**

This should be a defining quality of Vapor.

---

# Exhaustive Ambiguity Diagnostics

When a selector is ambiguous, Vapor should enumerate the known relevant candidates and useful safe resolution methods.

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

      --select "Registry/core"
      --select "Identity/core"
      --select "Diagnostics/core"

help: or establish a narrower persistent Context and retry
```

Where exact persistent-context commands are known, Vapor should print them too.

Diagnostics should be exhaustive with respect to safe modeled alternatives Vapor can actually prove.

They should not provide speculative actions as guaranteed fixes.

---

# Source Repo Container Commands

A **Source Repo Container** is a Registry-recognized Vapor Container Repo.

The generic namespace is:

```text
source-repo-container
```

Current intended operations include:

```text
source-repo-container
    create
    acquire
    status / inspect
    open
    close
```

Exact final status/inspect/open/close spelling remains subject to later CLI polishing.

---

# Source Repo Container Acquisition

Generic acquisition accepts only a registered Vapor Source Repo Container identity.

Conceptually:

```text
vapor source-repo-container acquire GHF-Studios/Loo-Cast
```

means:

```text
resolve identity through Registry
→ verify it is a valid registered Source Repo Container
→ resolve provider linkage
→ acquire its Git repository into canonical Superworkspace location
→ initialize authored Source Repo/submodule topology
→ validate resulting Vapor source topology
```

It is not equivalent to arbitrary:

```text
git clone <URL>
```

Vapor acquisition must reject arbitrary unregistered Git repositories.

The default acquisition unit is the entire Source Repo Container.

---

# Acquisition Destination

Ordinary acquisition does not accept an arbitrary destination path.

Vapor already knows the canonical Superworkspace.

The resulting local path is derived from:

```text
Superworkspace
+ Authority
+ Source Repo Container
```

For example:

```text
<Superworkspace>/GHF-Studios/Loo-Cast
```

Acquisition into arbitrary filesystem locations is not the normal Vapor model.

---

# Source Repo Container Creation

Conceptually:

```text
vapor source-repo-container create GHF-Studios/My-Stuff
```

means:

```text
establish Source Repo Container identity
→ create provider-backed Git repository
→ initialize Vapor Container metadata
→ place local checkout into canonical Superworkspace
→ register/provider-link through Vapor Registry
```

Creating a Container does not implicitly create Source Repos or Projects.

A new Container may legitimately begin empty.

Provider visibility should be conservative/private by default unless explicitly changed.

---

# Source Repo Commands

A **Source Repo** is one Git repository and one Vapor Workspace inside a Source Repo Container.

Current intended generic namespace:

```text
source-repo
```

Possible operations include:

```text
source-repo
    create
    acquire
    status / inspect
    open
    close
```

Independent Source Repo acquisition is more constrained than Container acquisition.

---

# Source Repo Creation

Conceptually:

```text
vapor source-repo create GHF-Studios/My-Stuff/Game
```

means:

```text
create provider Git repository
→ initialize Vapor Workspace
→ register Source Repo
→ add it as authored Git submodule of the parent Container
→ update parent Container topology
```

The operation is explicit.

Creating a Game/Engine/Library Project must not silently create a missing Source Repo.

---

# Source Repo Independent Acquisition

The conservative default is:

```text
Source Repo Container
    = atomic acquisition unit
```

A Source Repo may be independently acquired only when the Registry/topology explicitly permits it.

Conceptually:

```text
vapor source-repo acquire GHF-Studios/Foo/Examples
```

is valid only if `Examples` is independently acquirable.

Otherwise Vapor should explain:

```text
error: Source Repo `GHF-Studios/Foo/Internal`
cannot be acquired independently

it belongs to Source Repo Container:
    GHF-Studios/Foo

help: acquire the complete Container:
      vapor source-repo-container acquire GHF-Studios/Foo
```

The fact that Git can technically clone the repository is not sufficient.

---

# Missing Source vs Missing Git Submodule

Acquisition and Git reconciliation are different.

If an entire registered Source Repo Container is not locally available:

```text
acquire
```

is appropriate.

If the Container already exists and declares a Source Repo submodule whose checkout is missing:

```text
Git topology reconciliation
```

is appropriate.

Vapor must not disguise the second case as fresh Source Repo acquisition.

---

# Git-Oriented Diagnostics

Git remains fundamental and visible.

Vapor may diagnose:

```text
dirty working tree
missing submodule checkout
unexpected gitlink
detached HEAD
unpushed commits
merge conflict
branch divergence
```

using recognizably Git-oriented language.

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

Vapor should make stronger corrective recommendations only when it can actually prove they are safe.

---

# Diagnose and Repair

Diagnose and Repair are not synonyms for source restoration.

Diagnosis may inspect:

* Vapor metadata;
* Registry linkage;
* Git topology/state;
* Cargo realization;
* generated state;
* toolchain state;
* IDE integration;
* operation configuration.

Repair is intentionally conservative.

Repair must not silently:

* clone missing authored source;
* reacquire Source Repo Containers;
* restore authored files from Git;
* reset dirty repositories;
* discard unpushed commits;
* change branches;
* overwrite authored configuration.

Repair is primarily for safely derivable Vapor-owned state such as:

* generated metadata;
* caches;
* indexes;
* deterministic Cargo reconciliation owned by Vapor;
* IDE integration;
* generated operation realization;
* derived configuration.

---

# First-Party Bespoke Namespaces

First-party source may receive bespoke top-level command surfaces when that improves Vapor UX.

Current intended examples:

```text
client
platform-server
examples
```

This is a capability available to trusted first-party facilities.

It is not required for every first-party repository.

It is not available to arbitrary third-party source.

---

# First-Party Trust Boundary

A third-party Source Repo Container cannot declare:

```text
I am first-party
```

and thereby inject new root CLI commands.

First-party trust is controlled through Vapor Root Authority / trusted Registry state.

Conceptually, a Container may have independent dimensions such as:

```text
identity:
    GHF-Studios/Vapor-Client

trust:
    first-party

facility:
    client
```

versus:

```text
identity:
    Some-Author/My-Content

trust:
    standard

facility:
    none
```

First-party trust makes bespoke command semantics possible.

Actual semantic usefulness determines whether such a namespace exists.

---

# Vapor Client Namespace

The Vapor Client is a known singleton first-party facility.

The intended command family is approximately:

```text
client
    acquire
    status
    build
    test

    deploy
        local
        steam
```

`client acquire` resolves the trusted registered source identity backing the Client and invokes the appropriate generic acquisition machinery.

Users do not need to repeat:

```text
GHF-Studios/Vapor-Client
```

because `client` already identifies the singleton facility.

Its deployment semantics remain Client-specific.

---

# Vapor Platform Server Namespace

The Vapor Platform Server is another known singleton first-party facility.

The intended command family is approximately:

```text
platform-server
    acquire
    status
    build
    test

    deploy
        local
        vps
```

`vps` is the current descriptive deployment-target spelling.

It may later be generalized if the production deployment model outgrows a single VPS.

The semantic distinction must remain:

```text
deploy local
    local Platform Server realization/deployment

deploy vps
    actual remote production/development VPS deployment
```

Platform Server operations may have different selection legality per deployment target.

---

# Examples Namespace

Official Vapor Examples may receive a lightweight bespoke namespace:

```text
examples
    acquire
    status
    ...
```

The namespace represents the official Examples facility rather than its current Git topology.

Today Examples might be backed by one Source Repo inside another Container.

Later it could move elsewhere.

The user-facing `examples` subject need not change solely because source topology changes.

---

# Bespoke Does Not Mean Duplicated Implementation

Bespoke public semantics should reuse shared Vapor Core mechanisms.

For example:

```text
client acquire
platform-server acquire
examples acquire
```

may all share:

* trusted identity resolution;
* Registry access;
* provider-backed acquisition;
* Git orchestration;
* topology validation;
* diagnostics.

Likewise bespoke build/test/deploy operations may share:

* operation recipe execution;
* selection resolution;
* process execution;
* health checks;
* reporting.

The public semantics are bespoke.

The implementation primitives should be shared where meaningful.

---

# Operation Recipes

Some operation behavior should be configurable/authored rather than permanently hardcoded into Rust.

This is especially important for first-party build/test/deployment orchestration.

The intended model is lightweight:

```text
authored declarative operation recipe
+
Vapor-native safe primitives
+
small explicit scripts/process calls where necessary
```

It should resemble:

```text
task/workflow configuration
```

more than:

```text
arbitrary application/plugin runtime
```

Possible authored information includes:

* supported selection shapes;
* required Source Repos;
* required Projects;
* build steps;
* ordering;
* parallelism;
* validation;
* artifact relationships;
* deployment steps;
* health checks.

The exact schema is intentionally not frozen here.

---

# Vapor Metadata and Generated Operation State

Authored operation intent should live in Vapor-owned authored configuration such as appropriate `*.vapor.toml` manifests.

Generated/resolved realization may live under `.vapor/`.

The general principle is:

```text
authored Vapor manifests
    semantic intent

.vapor/
    derived/generated realization
```

Vapor should not casually rewrite human-authored sections merely to store generated state.

---

# Existing-Object Generic Operations

The final complete set of generic existing-object operations is not frozen.

Likely candidates include:

```text
inspect
verify
resolve
build
test
run
publish
```

where applicable.

The key semantic rule is frozen:

> **Existing-object operations should resolve the target first and validate its kind/operation compatibility rather than requiring the kind to be redundantly encoded in the command grammar.**

For example:

```text
vapor test GHF-Studios/My-Stuff/Game/My-Game
```

may resolve:

```text
Project
kind = Game
```

and invoke Game-compatible test semantics.

If the resolved kind cannot support `test`, Vapor should explain why.

---

# Project Kind-Specific Operations

Some operations may remain kind-specific where the operation itself is semantically inseparable from that kind.

Creation is the clearest example.

Packagepack also has lifecycle semantics unavailable to ordinary subordinate Projects, such as complete-composition installation/selection/runtime behavior.

Therefore the CLI must not pursue syntactic uniformity at the cost of semantic clarity.

---

# Rust/Cargo Vocabulary

Vapor source topology is:

```text
Source Repo Container
→ Source Repo
→ Project
```

Rust/Cargo realization may contain:

```text
Cargo workspace
package
crate
target
binary
example
...
```

The CLI should use these names only when it genuinely means the Rust/Cargo object.

For example, managed Cargo may expose Cargo-native selectors.

A generic Vapor Source Repo selector should not be called `--workspace` merely because Source Repo also equals Vapor Workspace.

---

# Managed Cargo

`vapor toolchain cargo -- ...` exposes Vapor's managed Cargo environment.

Its job may include:

* discovering the Vapor-managed Rust/Cargo toolchain;
* applying Vapor's managed environment;
* resolving a Project when Cargo requires one;
* invoking Cargo in the correct physical context;
* preserving Cargo-native arguments;
* returning Cargo output faithfully.

Cargo-native `-p/--package` remains Cargo vocabulary.

Vapor Project identity remains Vapor vocabulary.

The two may assist each other's resolution without becoming the same identity system.

---

# Role and Authority

Installed Role and external Authority remain separate.

Current Role progression:

```text
Player
→ Composer
→ Content Developer
→ Ecosystem Developer
```

Root Authority is not another installed Role.

A locally equipped Ecosystem Developer may develop Vapor source and run local first-party workflows without automatically possessing permission to modify official remote infrastructure.

Protected actions may additionally require authorization such as:

* creating/pushing official repositories;
* publishing under protected namespaces;
* deploying Steam branches/depots;
* deploying production Platform infrastructure;
* administering Registry/identity infrastructure.

CLI visibility and remote authorization are separate concerns.

---

# System-Oriented Namespaces

The intended system-facing shape is approximately:

```text
installation
    status
    diagnose
    repair

role
    status
    promote
    demote

authority
    status

toolchain
    status
    install
    diagnose
    repair
    cargo

source-repo-container
    create
    acquire
    status / inspect
    open
    close

source-repo
    create
    acquire
    status / inspect
    open
    close

client
    acquire
    status
    build
    test
    deploy local
    deploy steam

platform-server
    acquire
    status
    build
    test
    deploy local
    deploy vps

examples
    acquire
    status
```

This is not frozen as an exhaustive implementation checklist.

It captures the current semantic command ownership.

Notably absent:

```text
ecosystem
```

There is no generic public `ecosystem` object whose `create`, `fork`, `build`, `test`, or `deploy` operations form one coherent universal lifecycle.

---

# No Generic Ecosystem Namespace

“Vapor ecosystem” remains a useful descriptive phrase for Vapor as a whole.

It is not a sufficiently concrete CLI object.

The old shape:

```text
vapor ecosystem ...
```

should therefore be retired.

Operations previously grouped there must move to their actual semantic owners:

```text
Client work
    → client

Platform Server work
    → platform-server

source topology
    → source-repo-container / source-repo

derived-state reconciliation
    → diagnose / repair at appropriate owner

ordinary Content Projects
    → semantic Project operations
```

---

# No Arbitrary Ecosystem Create/Fork

The CLI should not expose:

```text
ecosystem create
ecosystem fork
```

Creating an entire alternative Vapor ecosystem through Vapor is not an ordinary development operation.

Likewise, forking all of Vapor as one conceptual umbrella operation is too extreme and semantically unclear.

Normal development should operate on concrete registered source topology or first-party facilities.

---

# No Generic Arbitrary Git Acquisition

Vapor acquisition does not replace ordinary Git.

If a user wants an arbitrary Git repository, they can use Git.

Vapor's acquisition commands are for modeled registered Vapor source.

Therefore:

```text
git clone torvalds/linux
```

is ordinary Git.

But:

```text
vapor source-repo-container acquire torvalds/linux
```

must fail unless that identity is actually a registered Vapor Source Repo Container.

---

# Plain Git Remains Valid

Advanced users may continue to use Git directly.

Vapor should not require that every valid checkout was created by Vapor itself.

If a user manually performs the equivalent correct Git operations inside the canonical Superworkspace and the resulting topology matches registered Vapor source, Vapor should be capable of recognizing it.

The acquisition operation is a convenient modeled orchestration path.

It is not magical provenance.

---

# Helpful Missing-Parent Diagnostics

Creation commands should explain missing hierarchy precisely.

For example:

```text
vapor game create GHF-Studios/My-Stuff/Game/My-Game
```

when `My-Stuff` does not exist may produce:

```text
error: Source Repo Container `GHF-Studios/My-Stuff` does not exist

help: create it:
      vapor source-repo-container create GHF-Studios/My-Stuff
```

If it exists remotely/Registry-side but is not locally available:

```text
error: Source Repo Container `GHF-Studios/My-Stuff`
       is not available in the local Superworkspace

help: acquire it:
      vapor source-repo-container acquire GHF-Studios/My-Stuff
```

If the Container exists but Source Repo `Game` does not:

```text
error: Source Repo `GHF-Studios/My-Stuff/Game` does not exist

help: create it:
      vapor source-repo create GHF-Studios/My-Stuff/Game
```

Vapor must not silently create missing parents during higher-level Project creation.

---

# Error Philosophy

CLI errors are part of Vapor's teaching surface.

A strong error should explain, where applicable:

* what Vapor resolved;
* what it could not resolve;
* exact candidate identities;
* target kinds;
* violated operation invariant;
* required Role;
* missing authority;
* missing local source;
* relevant source parent;
* safe acquisition/create operations;
* contextual-resolution alternatives;
* explicit-selection alternatives;
* relevant Git inspection;
* relevant operation variant which does support the requested scope.

This is particularly important because Vapor should remain approachable to developers who may also be learning:

* Git;
* Rust;
* Cargo;
* game development;
* software development generally.

---

# Safety of Suggested Actions

Vapor should distinguish safe deterministic guidance from potentially destructive actions.

Safe:

```text
Source Repo Container not local
→ acquire registered Container
```

Potentially state-changing Git reconciliation:

```text
missing submodule checkout
→ inspect Git state
→ possible `git submodule update --init`
→ explicit caution
```

Vapor must not present destructive Git actions as guaranteed fixes unless their safety is actually known.

---

# CLI / GUI / Automation Equality

All frontends share Vapor Core semantics.

Their interaction mechanics differ.

## CLI

Uses:

* Vapor Paths;
* selectors;
* `--context`;
* `--select`;
* explicit commands;
* deterministic diagnostics.

## SDK / GUI

May use:

* tree navigation;
* dialogs;
* Inspector actions;
* multi-selection;
* persistent navigation state.

## Automation

May prefer:

* exact canonical paths;
* explicit structured selection;
* machine-readable output;
* deterministic operation results.

The GUI should not be crippled merely because some gesture is awkward to spell in CLI syntax.

The CLI should not imitate GUI gestures unnecessarily.

---

# CLI Invariants

* CLI grammar reflects semantic Vapor concepts rather than code layout.
* There is no public generic `ecosystem` namespace.
* Superworkspace is not a CLI identity segment.
* Ordinary acquisition never asks for a destination path.
* Source Repo Container acquisition requires Registry-recognized Vapor source.
* Container acquisition is conservative/atomic by default.
* Source Repo independent acquisition requires explicit modeled permission.
* Source Repo equals Vapor Workspace.
* Project is the final Vapor source hierarchy layer before deeper Rust/Cargo realization.
* Typed Content kinds are Project kinds, not identity segments.
* Creation states the new Project kind explicitly.
* Existing-object operations should not redundantly restate already-resolved kind.
* Context supplies missing information to the left.
* Selection supplies/refines information to the right.
* Persistent Context does not imply operation scope.
* Explicit transient Context overrides persistent Context for one invocation.
* CWD is not ordinary semantic Vapor Context.
* No selector means natural complete subject scope, not hidden `--all`.
* Parent selection naturally includes relevant descendants according to operation semantics.
* Selection legality is operation-specific.
* Ambiguity is never guessed.
* Ambiguity diagnostics should enumerate useful candidates and safe resolutions.
* Git remains visible and Git-oriented.
* Repair does not reacquire authored source or reset Git.
* First-party bespoke TLDs are trusted Vapor facilities.
* Third-party source cannot inject root CLI grammar.
* Bespoke semantics should reuse shared Vapor Core mechanisms.
* Operation recipes should remain lightweight rather than becoming arbitrary application/plugin runtimes.
* GUI, CLI, and automation invoke shared Core semantics.

---

# Open CLI Questions

The following remain intentionally unsettled:

* Final public spelling of `source-repo-container`, if a shorter equally precise term proves better.
* Final exact spellings of persistent Context operations such as `open` and `close`.
* Final spelling of `--context`.
* Final spelling and expression grammar of `--select`.
* Exact list delimiter/quoting syntax for multiple selections.
* Exact generic existing-object operation inventory.
* Exact relationship between Project-generic operations and kind-specific Packagepack lifecycle commands.
* Exact machine-readable output format.
* Exact first-party recipe configuration schema.
* Exact provider creation/authentication UX for `source-repo-container create` and `source-repo create`.
* Exact public/private visibility flags and defaults.
* Exact Superworkspace relocation/configuration command.
* Exact CLI treatment of independently acquirable Source Repos.
* Exact deeper Rust/Cargo target-selection grammar under a selected Vapor Project.
* Exact generic command for low-level Git reconciliation wrappers, if Vapor eventually provides one.
