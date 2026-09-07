> [!info]
> This document defines the semantic command model shared by Vapor's command-line surfaces.
>
> It describes command hierarchy, application projections, role visibility, and the relationship between CLI operations and Vapor Content kinds.
>
> Exact command leaves may evolve as implementation pressure reveals better semantics.

---

# Core Principle

The Vapor CLI models Vapor concepts rather than Cargo structure, repository layout, implementation modules, or arbitrary utility groupings.

A command namespace must represent a meaningful Vapor domain or first-class entity.

Namespaces must not exist merely to classify or cosmetically scope an operation.

The CLI should remain explicit, flattened, and predictable.

---

# Shared Core and CLI Surfaces

Vapor applications expose different projections of the same underlying Vapor Core operations.

The intended command-line binaries are:

* `vapor`
* `vapor-installer`
* `vapor-launcher`
* `vapor-sdk`

`vapor` is the broad universal CLI projection.

The dedicated binaries expose subsets appropriate to their application and installed Role.

For example, an Installer Role operation exposed as:

```text
vapor role status
```

is the same underlying operation exposed by:

```text
vapor-installer role status
```

Application binaries must not independently reimplement Vapor semantics.

The logical Vapor SDK may be integrated into the Launcher while also having a dedicated `vapor-sdk` executable/CLI projection.

Those are different surfaces over the same Vapor Core rather than independent SDK implementations.

---

# Top-Level Purity

Top-level namespaces should correspond to stable Vapor domains or first-class Vapor entity kinds.

Current system-oriented namespaces include:

```text
installation
role
authority
toolchain
source
ecosystem
```

Current Vapor Content namespaces are explicit:

```text
packagepack
enginepack
gamepack
modpack
engine
game
engine-mod
game-mod
extension-mod
library
```

There is intentionally no generic `content` namespace merely to mirror the Vapor Content taxonomy.

Likewise, ordinary workflows should not require the user to jump between separate `composition` and `app` namespaces merely because a Packagepack moves through resolution, build, installation, and runtime states.

Packagepack, Vapor App Composition, and Vapor App remain distinct model terms where that distinction matters.

`library` here means the first-class Vapor Content kind.

It is distinct from the Launcher/local **Content Library** surface.

---

# Content Command Model

Each Vapor Content kind exposes the operations semantically applicable to that kind.

The current command shape is:

```text
packagepack
    create
    list
    inspect
    resolve
    verify
    build
    test
    install
    select
    run
    remove
    publish

enginepack
    create
    list
    inspect
    resolve
    verify
    test
    publish

gamepack
    create
    list
    inspect
    resolve
    verify
    test
    publish

modpack
    create
    list
    inspect
    resolve
    verify
    test
    publish

engine
    create
    list
    inspect
    verify
    test
    publish

game
    create
    list
    inspect
    verify
    test
    publish

engine-mod
    create
    list
    inspect
    verify
    test
    publish

game-mod
    create
    list
    inspect
    verify
    test
    publish

extension-mod
    create
    list
    inspect
    verify
    test
    publish

library
    create
    list
    inspect
    resolve
    verify
    test
    publish
```

This tree is semantic rather than mechanically uniform.

An operation should be present wherever it makes sense rather than being artificially segregated by Content kind.

For packs, `test` runs the applicable tests contributed by the resolved Content graph together.

For a Library, `resolve` exposes its semantic dependency graph and the Rust/Cargo realization information relevant to that graph.

This does not mean Cargo defines Vapor dependency semantics.

---

# Packagepack Lifecycle

Packagepack is the complete-composition Vapor Content kind.

A Packagepack may therefore support complete-composition operations that subordinate Content kinds do not independently support, including:

* Build.
* Install.
* Select.
* Run.
* Remove.

A Packagepack remains the authored composition identity throughout these workflows.

A resolved Vapor App Composition and built Vapor App are precise derived states or realizations of that Packagepack rather than mandatory separate CLI namespaces.

---

# Resolution Model

Resolution is implemented as a generic Vapor Content graph operation.

Vapor semantic resolution is authoritative for:

* Vapor IDs.
* Vapor versions.
* Dependency bindings.
* Vapor Content kinds.
* Vapor-level dependency relationships.

CLI exposure remains explicit per Content kind where a standalone resolution operation is useful.

Examples include:

```text
vapor packagepack resolve ...
vapor enginepack resolve ...
vapor gamepack resolve ...
vapor modpack resolve ...
vapor library resolve ...
```

These commands use the same underlying semantic resolver rather than separate kind-specific dependency solvers.

Resolution recursively follows dependencies declared by Vapor Content.

Binding names do not determine Content semantics.

Resolved identities, kinds, and authored relationships do.

A Packagepack may obtain its effective Engine either directly or transitively through an Enginepack.

Likewise, it may obtain its effective Game directly or through a Gamepack.

Nested packs, Mods, Libraries, and other dependencies are resolved transitively.

A valid Packagepack ultimately yields exactly one effective Engine and exactly one effective Game together with all other required Content.

Packagepack-specific validation is layered on top of generic graph resolution.

---

# Vapor Resolution vs Cargo Resolution

Rust-backed Vapor Content introduces a second, distinct graph:

```text
Vapor semantic graph
        ↓
Rust/Cargo realization
        ↓
Cargo package graph
```

Vapor is authoritative for the semantic Vapor graph.

Cargo is authoritative for the physical Rust package graph it actually resolves and compiles.

Vapor should use Cargo-native mechanisms, including `cargo metadata` where useful, to:

* inspect Cargo workspace/package structure;
* identify physical Rust packages and targets;
* observe Cargo dependency declarations;
* observe Cargo's resolved package graph;
* verify that Vapor semantic dependencies are physically realized;
* diagnose divergence between Vapor and Cargo state.

Cargo Package IDs and similar identifiers are Cargo-domain physical identifiers.

They must not become replacements for Vapor semantic identity.

---

# Cargo Reconciliation

Vapor should supervise the Cargo entries that physically realize Vapor semantic dependencies without taking ownership of unrelated Cargo configuration.

Conceptually:

```text
Vapor.toml
    semantic dependency intent
        ↓
Vapor resolver
    exact desired Vapor graph
        ↓
Cargo reconciliation
        ↓
editable Cargo.toml
        ↓
Cargo resolution / metadata
        ↓
actual physical graph
        ↓
Vapor verification
```

A developer may continue to edit ordinary Cargo configuration directly.

Vapor should distinguish at least:

```text
valid
stale
conflicting
explicitly locally overridden
repairable
```

Verification should diagnose disagreement.

Repair may reconcile Vapor-managed physical dependency entries.

Explicit local/unlocked overrides may be valid development state but must not silently become publishable state.

---

# Creation and Templates

Creation is a first-class operation for authored Vapor entities.

Examples include:

```text
vapor packagepack create ...
vapor engine create ...
vapor game-mod create ...
vapor library create ...
```

Creation should establish canonical structural boilerplate for the selected Vapor entity.

This may include:

* Vapor manifests.
* Source/project structure.
* Cargo structure where applicable.
* Dependency declarations.
* Initial tests.
* Appropriate repository/workspace placement.

Creation may support templates.

The initial implementation may use built-in canonical templates.

More general versioned or externally supplied template systems should be introduced only when concrete pressure requires them.

The same creation model may eventually extend to Vapor ecosystem/root source structures and remote repository creation.

---

# Role and Authority

Role and authorization are separate concepts.

A Vapor Role describes the kinds of work for which the local Steam App Instance is equipped.

The installed Role progression is:

```text
Player
→ Composer
→ Content Developer
→ Ecosystem Developer
```

Ecosystem Developer is locally attainable.

An Ecosystem Developer may acquire, fork, create, modify, build, and test Vapor ecosystem source without official Vapor authorization.

Authorization determines whether a particular operation may affect a particular protected external target.

Examples include:

* Pushing to official GHF Studios repositories.
* Creating repositories in protected organizations.
* Publishing into official namespaces.
* Deploying official Steam branches or depots.
* Modifying production registry/server infrastructure.

The same operation may therefore be locally available while a particular target remains unauthorized.

Protected operations should be visibly distinguishable rather than conceptually hidden.

---

# Root Authority

Root Authority is not an installed development Role above Ecosystem Developer.

It is an authority state granting ultimate administrative and ownership authority over protected official Vapor ecosystem resources.

A Root Authority normally operates with Ecosystem Developer Role plus Root Authority authorization.

---

# Role-Based Surface Exposure

Underlying Vapor Core operations may exist even when a particular user-facing surface does not expose them.

Installed Role influences which operations and tooling are presented.

For example:

* Player surfaces focus on consuming and running finished Vapor Apps.
* Composer surfaces expose pack composition workflows.
* Content Developer surfaces expose Engine/Game/Mod/Library creation and development.
* Ecosystem Developer surfaces expose Vapor ecosystem source development.

Role-based visibility is distinct from authorization.

An operation visible to an Ecosystem Developer may still reject a protected target for lack of authority.

---

# System-Oriented Namespaces

System-oriented namespaces represent stable Vapor domains or first-class system concepts.

The currently intended shape is approximately:

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

source
    status
    list
    acquire
    fork

ecosystem
    status
    acquire
    fork
    create
    build
    test
    publish

    deploy
        local
        steam
```

The exact future CLI projection of Development Session operations such as Open, Focus, Close, Forget, and realization selection is deliberately not frozen yet.

Those semantics exist independently in the **Vapor Context, Identity, Session And Selection Model**.

The CLI should gain syntax for them only when the command shape has been pressure-tested against the SDK/Core model.

`source` represents source acquisition, source availability, provider linkage, and source-oriented operations.

It must not become a generic bucket for:

* every development-context operation;
* Project selection;
* SDK session state;
* IDE integration;
* arbitrary filesystem navigation.

Likewise, `ecosystem` represents Vapor ecosystem/root development as a semantic domain rather than merely a directory containing Vapor's own repositories.

---

# Diagnose and Repair

Diagnosis and repair may appear at meaningful owning scopes.

Examples include:

```text
vapor installation diagnose
vapor installation repair

vapor toolchain diagnose
vapor toolchain repair
```

Narrow operations should diagnose or reconcile the domain they own.

The graphical SDK may additionally provide aggregated Problems/health views spanning several domains without requiring one artificial CLI namespace for every cross-cutting presentation.

Repair must remain conservative.

It must not treat authored source as disposable.

In particular, repair must not silently:

* reset dirty Git repositories;
* destroy uncommitted source;
* discard unpushed commits;
* attach unrelated similarly named checkouts;
* rewrite explicit authored configuration merely because a generated state differs.

Where a canonical managed answer exists, ordinary operations may proactively reconcile safe derived state.

Diagnose/Repair remain especially valuable for observability and failure recovery.

---

# Semantic Target Model

CLI operations should target Vapor semantic or structural objects rather than implementation paths wherever possible.

Examples include:

```text
GHF-Studios/Vapor-Root/Vapor/Client
GHF-Studios/Vapor-Examples/Wheel-Rules
```

depending on target domain.

Filesystem paths, Cargo manifest paths, provider URLs, and Cargo Package IDs remain legitimate implementation/diagnostic selectors where an operation explicitly needs them.

They are not preferred semantic identity.

---

# Canonical Identity and Selectors

A CLI argument which identifies a Vapor object is conceptually a **selector** unless the command explicitly requires a canonical ID.

The selector may be:

```text
full canonical identity
qualified suffix
local name
local alias
exact realization selector
```

depending on the operation and available context.

For example, Project selectors might conceptually include:

```text
GHF-Studios/Vapor-Root/Vapor/Client
Vapor-Root/Vapor/Client
Vapor/Client
Client
```

The shorter forms are not alternate canonical IDs.

They are context-dependent selectors.

Once resolved, Vapor Core operates on exact canonical identity and, when physical work is required, an exact local realization.

---

# Minimal Information Principle

The CLI should accept the shortest selector which safely resolves the requested target.

The principle is:

> **Require no more identifying information than is necessary to perform the operation unambiguously.**

This allows ergonomic commands without weakening identity.

For example:

```text
vapor library resolve Wheel-Rules
```

may be valid when the effective development context contains exactly one Library matching that selector.

If several candidates exist, Vapor reports ambiguity.

It does not guess.

---

# Ambiguity

Ambiguity is a normal resolution result.

A useful CLI error should explain:

* what selector was ambiguous;
* which exact candidates matched;
* what additional qualification would distinguish them.

Example:

```text
error: Project selector `Client` is ambiguous

Candidates:
    GHF-Studios/Vapor-Root/Vapor/Client
    Leslie/Vapor-Fork/Vapor/Client

Use a more qualified selector or establish a narrower development context.
```

Non-interactive CLI behavior must remain deterministic.

Optional interactive selection for explicitly human-oriented workflows may be introduced later, but automation must never depend on it.

---

# Operation Cardinality

The CLI does not require every development domain to have exactly one globally active object.

Vapor Core operations define the target cardinality they accept.

Conceptually:

```text
ExactlyOne<Project>
Many<Project>
ExactlyOne<Packagepack>
Many<Content>
```

If resolution produces:

```text
0 candidates
```

the operation is unresolved.

If it produces:

```text
1 candidate
```

a singular operation may proceed.

If it produces multiple candidates and the operation accepts multiplicity, all explicitly resolved candidates may participate.

If it produces multiple candidates and the operation requires one, the CLI reports ambiguity.

CLI implementation convenience must not distort the underlying development model into one global `active_project`.

---

# CLI Context Resolution

The CLI constructs an ephemeral operation context from modeled state and command input.

Conceptually it may use:

```text
explicit selector
    ↓
qualified/local selector resolution
    ↓
relevant persisted/resumed development context
    ↓
current working directory as a safe physical hint
    ↓
unique usable candidate
    ↓
ambiguity / unresolved
```

This is a CLI projection of the shared Context Model rather than a separate identity system.

Exact precedence may vary where an explicit selector provides stronger information than ambient context.

The invariant is:

> **Explicit semantic intent outranks accidental environment.**

---

# Current Working Directory

CWD is useful because shell users naturally invoke commands from inside repositories and Projects.

CWD may therefore help identify:

* Superworkspace;
* Container Repo;
* Workspace realization;
* Project realization.

But:

> **CWD is an ambient hint, not durable Vapor identity.**

CWD must not silently:

* change persisted SDK Focus;
* replace exact remembered identities;
* become an implicit source root recorded as truth;
* force unrelated commands to depend on the shell's current directory.

A command issued inside a known Project may naturally operate there.

The same command issued from `~` should still work when stronger modeled context and selectors make the target unambiguous.

---

# Raw Path Overrides

Path arguments remain legitimate escape hatches for operations which intentionally target local storage or bootstrap unknown source.

Examples may include:

* registering a previously unknown checkout;
* locating a moved realization;
* opening/importing an arbitrary local directory;
* debugging local source discovery.

They should not be required on ordinary already-modeled Content operations.

A normal command should prefer:

```text
Vapor ID
Project selector
Workspace selector
realization selector
```

over:

```text
../../some/local/root
```

---

# Cargo Execution Context

Cargo requires a physical Project/workspace execution context.

Vapor's managed Cargo wrapper should therefore resolve the relevant Vapor Project before spawning Cargo where a Project is required.

Conceptually:

```text
vapor toolchain cargo
    ↓
Cargo arguments
    ↓
explicit Project selector if supplied
    ↓
Cargo package selector if it uniquely identifies a Vapor Project
    ↓
ephemeral CLI/CWD context
    ↓
unique Project
    ↓
managed Cargo
```

`--project` is therefore a **Project selector**, not inherently a globally unique Project ID argument.

A short selector such as:

```text
--project Client
```

is valid only when the effective context resolves it unambiguously.

A canonical Project identity remains valid independent of local shorthand.

Cargo-native selectors such as `-p/--package` may assist physical Project resolution, but Cargo package identity does not become Vapor semantic identity.

---

# Managed Cargo

`vapor toolchain cargo -- ...` exposes the Installation-owned managed Cargo tool without globally exposing the managed Rust/Cargo toolchain through the user's shell PATH.

The wrapper should:

* discover the active Vapor Installation;
* establish the Installation-owned managed toolchain environment for the child;
* resolve a Vapor Project when the Cargo operation requires one;
* preserve Cargo arguments;
* execute Cargo in the selected Project context;
* return Cargo's success/failure meaningfully.

Global Vapor command exposure and private managed-tool exposure remain distinct concerns.

---

# Development Session and CLI

A one-shot CLI invocation does not need to become a permanently live session like the SDK.

It may construct an ephemeral Development Session from:

* exact command targets;
* persisted Resume/default state;
* known local realizations;
* CWD;
* other safe context.

The CLI should not silently mutate durable SDK Focus merely because an operation resolved successfully.

Commands which deliberately mutate Open/Focus/Resume state may be added explicitly once their CLI grammar is settled.

---

# GUI / CLI / Automation Equality

GUI and CLI equality means semantic and operational equality rather than interaction equality.

The GUI may use:

```text
Explorer
multi-selection
Inspector
dialogs
graph interactions
context menus
```

The CLI may use:

```text
selectors
flags
canonical IDs
machine-readable output
```

Automation may prefer:

```text
canonical identities
exact realization identifiers
structured output
non-interactive deterministic behavior
```

All should call the same underlying Vapor Core operations.

The CLI should not imitate awkward GUI gestures merely for symmetry.

The GUI should not be crippled because a particular interaction is awkward to spell in shell syntax.

---

# SDK / CLI Discoverability

CLI errors should teach the model where possible.

Good errors should expose:

* canonical candidate identities;
* missing Role/capability;
* missing realization;
* required target cardinality;
* possible repair;
* relevant qualification syntax.

The SDK may present the same semantic problem visually.

For example, the GUI may show an ambiguous target picker while the CLI lists exact candidates.

The semantic reason is identical.

---

# CLI Invariants

* CLI structure does not mirror Rust module structure.
* CLI structure does not mirror Cargo workspace structure.
* CLI structure does not automatically mirror the Vapor type hierarchy.
* Vapor Content kinds remain explicit first-class CLI namespaces where useful.
* A generic `content` namespace is not introduced merely as a taxonomy bucket.
* `library` means the Vapor Content kind; Content Library is a separate product concept.
* Packagepack workflows do not require ordinary users to manually switch between Packagepack, Composition, and Vapor App namespaces.
* Shared semantic operations are implemented once in Vapor Core.
* Dedicated Vapor binaries expose projections of those operations.
* CLI selectors are not automatically canonical IDs.
* Short selectors are accepted only when unambiguous.
* Canonical structural identities are case-preserving and slash-separated.
* Project selectors resolve to exact Project identity before physical execution.
* Cargo package identity does not replace Vapor Project identity.
* Raw paths are realization/bootstrap escape hatches rather than normal semantic selectors.
* CWD is an ephemeral hint rather than durable context.
* Multiple objects may be Open/Focused even when an operation requires one.
* Operation cardinality determines whether multiplicity is valid.
* Ambiguous singular operations fail rather than choosing arbitrarily.
* CLI operations must remain deterministic for automation.
* GUI, CLI, and automation share Core semantics but may use different interaction mechanics.
* Role controls locally equipped workflow capability.
* Authorization controls protected operations against protected targets.
* Root Authority is authority rather than an installed Role.
* User-authored source remains outside the disposable Steam App Instance by default.

---

# Open CLI Questions

* Exact CLI spelling of Open / Focus / Close / Forget.
* Exact syntax for selecting one local realization among several.
* Whether human-oriented mode offers optional interactive ambiguity resolution.
* Exact machine-readable output modes.
* Exact selector grammar beyond canonical slash-separated structural IDs.
* Exact Packagepack install/select/remove lifecycle terminology.
* Exact template-selection syntax.
* Whether standalone `resolve` should eventually be exposed for every dependency-bearing Content kind.
* Exact CLI presentation of Cargo-reconciliation and local-override state.
* Exact provider/fork qualification syntax.
* Exact relationship between future `vapor-sdk` CLI/session commands and the universal `vapor` surface.
