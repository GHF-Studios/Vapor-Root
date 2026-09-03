
> [!info]
> This document defines the operational model underlying the Vapor Ecosystem.
>
> It deliberately avoids representing Vapor as one giant serialized list of every valid workflow permutation.
>
> Instead, Vapor is modeled as a **multidimensional situation space** composed from state dimensions, conditions, contexts, actions, transitions, and invariants.
>
> Lifecycles are projections over that larger operational model rather than independent exhaustive stories.

---

# Purpose

A naive lifecycle specification tends to become a combinatorial enumeration:

> Player + installed + selected + online + build current + authenticated...

followed by:

> Composer + installed + selected + online + build current + authenticated...

followed by every other meaningful permutation.

That is neither readable nor useful.

Vapor instead models the relevant dimensions independently and specifies how actions interact with those dimensions.

A complete current situation is therefore conceptually:

> **Situation = combination of operational conditions across multiple state dimensions**

A named user-facing or system-facing context is:

> **Context = meaningful predicate over a Situation**

An operation is:

> **Action = guarded transition affecting a defined subset of the Situation**

An invariant is:

> **Invariant = relationship that must remain true across Situations and Actions**

A lifecycle is:

> **Lifecycle = projection of relevant Actions and state changes onto one area of concern**

This gives Vapor the equivalent of a multidimensional flowchart without requiring a node for every possible cross-product of conditions.

See [Vapor Operational State Model](./Diagrams/Vapor%20Operational%20State%20Model.puml).

---

# Core Operational Concepts

## State Dimension

A **State Dimension** describes one independently meaningful aspect of the current system situation.

Examples include:

* Installed capability.
* Toolchain readiness.
* Source availability.
* Source cleanliness.
* Composition resolution.
* Build currency.
* Vapor App installation.
* Selection.
* Runtime.
* Authentication.
* Network/provider availability.
* Publication state.

Dimensions are not required to be mathematically independent.

Some intentionally interact.

The purpose of separating them is to model those interactions explicitly rather than burying them in giant composite workflow states.

---

## Condition

A **Condition** is one fact that can hold within a state dimension or across several dimensions.

Examples:

* Composer capability is installed.
* Git is available.
* Packagepack source exists locally.
* Repository has uncommitted changes.
* Composition resolves successfully.
* A built Vapor App exists.
* The built Vapor App corresponds to the current source.
* A Vapor App is installed.
* A Vapor App is selected.
* Runtime is currently stopped.
* GitHub authentication is available.
* Steam is reachable.

Conditions may be:

* Mutually exclusive.
* Independent.
* Derived.
* Temporarily unknown.
* Provider-specific.

---

## Situation

A **Situation** is the complete relevant operational state at a particular moment.

Vapor does not need to assign a unique named type to every possible Situation.

For example:

> Composer capability installed
>
> * source present
> * source dirty
> * composition valid
> * build stale
> * previously built Vapor App installed
> * that Vapor App selected
> * runtime stopped
> * GitHub unauthenticated

is one perfectly meaningful Situation.

There is no need to create a bespoke state named:

> `DirtyStaleSelectedOfflineComposerState`.

---

## Context

A **Context** is a useful named interpretation of a Situation.

A Context is defined by the conditions relevant to some class of action or UX.

Examples might include:

* **Playable**
* **Composable**
* **Editable**
* **Buildable**
* **Locally Runnable**
* **Publishable Source**
* **Publishable Vapor App**
* **Repair Required**
* **Runtime Active**

A Context intentionally ignores irrelevant dimensions.

For example, local buildability need not depend on GitHub authentication unless the build itself requires a provider-specific remote operation.

---

## Action

An **Action** represents something the user or system can attempt to do.

An Action should conceptually define:

* Required preconditions.
* Which state dimensions it may read.
* Which dimensions it may change.
* Its meaningful success results.
* Its meaningful failure results.
* Which unrelated state it must preserve.

Examples:

* Upgrade capability.
* Acquire source.
* Modify pack.
* Resolve composition.
* Build.
* Install Vapor App.
* Select Vapor App.
* Launch.
* Stop.
* Publish source.
* Publish built composition.
* Authenticate provider.
* Repair toolchain.
* Remove Vapor App.

---

## Transition

A **Transition** is the resulting state change produced by an Action.

Transitions should be modeled only where meaningful state changes.

Failure does not require inventing an entirely separate parallel lifecycle.

A failed Action simply produces whatever failure-related state is meaningful while preserving state that the operation did not legitimately change.

---

## Invariant

An **Invariant** constrains the operational model.

Examples:

* Player capability cannot require a local Rust toolchain.
* A successful source edit may invalidate build currency.
* Authentication state must not silently alter source.
* A failed rebuild must not automatically destroy the previously installed working Vapor App.
* Selecting one Vapor App should not mutate unrelated Git source state.
* Removing Composer capability must not silently delete authored source.

---

# State Dimension Inventory

The following dimensions form the current initial operational inventory.

This inventory should grow when concrete workflows reveal missing dimensions.

It should not grow merely to encode incidental implementation details.

---

## Capability State

Relevant conditions include:

* Player.
* Composer.
* Content Developer.
* Ecosystem Developer.
* Root Authority.

The hierarchy is cumulative.

Capability changes are Installer-owned operations.

---

## Toolchain State

Relevant conditions may include:

* Required tool absent.
* Required tool detected.
* Required tool configured.
* Toolchain ready.
* Toolchain degraded.
* Repair required.

Different capability levels require different tool subsets.

Player capability should not require Composer/Developer tools.

---

## Source Availability State

For a given source artifact or repository context:

* Absent.
* Available locally.
* Availability unknown/unresolved.

Source may be acquired from Vapor-compatible Git repositories.

This dimension is primarily relevant at Composer capability and above.

---

## Source Modification State

For Git-managed source:

* Clean.
* Modified/uncommitted.
* Locally committed but unpushed.
* Synchronized with relevant remote.
* Conflict/repair required.

These states may require further refinement later.

The central invariant is that local-only source state can exist and must be protected.

---

## Composition Resolution State

For a Packagepack:

* Unresolved.
* Resolving.
* Resolved/valid.
* Invalid.
* Resolution failed.

A valid resolution identifies exactly one effective Engine and exactly one effective Game plus the effective Mods.

---

## Build State

For a Packagepack/target:

* Missing.
* Building.
* Current.
* Stale.
* Failed.

A source or composition change may transition a previously Current build to Stale.

A failed rebuild should not imply that an older valid installed Vapor App ceases to exist.

---

## Vapor App Installation State

For a built/published Vapor App:

* Not locally installed.
* Installing.
* Installed.
* Removal in progress.
* Installation invalid/repair required.

Installation is distinct from build currency.

A stale source tree may coexist with a previously installed runnable Vapor App.

---

## Selection State

At the Steam App Instance level:

* No meaningful selected composition.
* Vapor App Composition selected.

Selection identifies the current/default composition used by Launcher/direct-play convenience.

Selection should remain independent from unrelated source modification state.

---

## Runtime State

For a Vapor App:

* Stopped.
* Starting.
* Running.
* Stopping.
* Failed to start/crashed.

The detailed Engine/Game internal runtime lifecycle belongs elsewhere.

This dimension only models the Vapor-level execution relationship.

---

## Authentication / Authorization State

Authentication is provider-specific.

Relevant conditions may exist independently for:

* Steam.
* Vapor services.
* GitHub.
* Other Git hosts.
* Future signing/identity providers.

Possible coarse conditions include:

* Not authenticated.
* Authenticated.
* Authenticated but unauthorized for requested resource.
* Authorization available.

Local capability does not depend on all provider authentication being present.

---

## External Availability State

Relevant external systems may independently be:

* Available.
* Unavailable.
* Degraded.
* Unknown.

Examples:

* Steam.
* Steam Workshop.
* Vapor Registry.
* GitHub.
* Other Git hosts.

An outage in one provider should not unnecessarily invalidate unrelated local operations.

---

## Publication State

Source and built publication are distinct dimensions or subdimensions.

Source-side states may include:

* Unpublished.
* Published.
* Local changes since publication.
* Publication update pending.
* Publication failed.

Built-distribution states may include:

* No built publication.
* Build ready for publication.
* Published.
* Built publication update pending.
* Publication failed.

The Publishing and Distribution Model defines these more precisely.

---

# Important Derived Contexts

The following contexts are more useful to UX and operation gating than giant serialized workflow states.

They are conceptual predicates and may later receive more formal definitions.

---

## Player-Ready

A Steam App Instance is **Player-Ready** when:

* Player capability is healthy.
* Required base Steam/Vapor runtime state exists.
* The selected/default Vapor App is locally runnable.

It does not require:

* Git.
* Rust.
* Cargo.
* GitHub authentication.

---

## Composable

A source context is **Composable** when:

* Composer capability exists.
* Required composition source is locally available or can be acquired.
* Required tooling is healthy.
* The user has a writable composition-authoring context.

Remote provider authentication is required only if the operation being attempted needs a remote provider.

---

## Buildable

A Packagepack is **Buildable** when:

* Composer-or-higher capability is available.
* Required toolchain state is ready.
* Required source is available.
* The composition resolves validly for the requested build.
* No unresolved condition prevents compilation.

Buildable does not mean the source is clean or published.

Local dirty source should generally remain buildable.

---

## Locally Runnable

A Vapor App is **Locally Runnable** when:

* A valid built artifact exists locally.
* It is appropriately installed/registered.
* Runtime prerequisites are satisfied.

The corresponding source may be:

* Clean.
* Dirty.
* Missing.
* Newer than the installed build.

A locally runnable installed build is therefore distinct from "current relative to source."

---

## Editable

A Vapor Content artifact is **Editable** when:

* The installed capability permits authoring that artifact type.
* Appropriate source is locally available.
* The source context is writable.

For example:

* Composer can edit Packagepacks.
* Composer cannot edit Engine behavior.
* Content Developer can edit Engine behavior.

---

## Source-Publishable

A source artifact is **Source-Publishable** when:

* The user's capability permits authoring it.
* Required source state exists.
* Required identity/ownership authorization exists.
* Required source validation succeeds.
* Required remote provider access exists.

Exact version/release requirements remain part of the Publishing and Distribution Model.

---

## Built-Publishable

A Packagepack realization is **Built-Publishable** when:

* A valid complete Packagepack exists.
* Required target build(s) exist.
* Required publication validation succeeds.
* Required Workshop/Vapor authorization exists.

This does not imply source publication and built publication are the same operation.

---

# Major Actions and Their Operational Effects

## Establish Capability

Examples:

* Player → Composer.
* Composer → Content Developer.

Owned primarily by the Vapor Installer.

May change:

* Installed tooling.
* Capability state.
* Toolchain state.
* Launcher-visible functionality.

Must preserve:

* Existing user source.
* Existing saves.
* Existing installed Vapor Apps unless explicitly incompatible.
* Unrelated provider authentication where possible.

---

## Downgrade Capability

May remove capability-specific tooling or disable capability-specific surfaces.

Must not silently treat authored source as disposable.

A downgrade from Content Developer to Player does not conceptually mean:

> Delete every Git repository the user authored.

---

## Acquire Source

Reads:

* Vapor identity/Registry information.
* Git source linkage.
* Existing local repository state.

May change:

* Source availability.
* Git checkout state.
* Local dependency state.

Should not mutate unrelated installed Vapor Apps.

---

## Modify Source

May change:

* Source modification state.
* Composition validity.
* Build currency.

For example:

> Current build
>
> * relevant source edit
    >   → Stale build

It should not automatically destroy the old built Vapor App.

---

## Resolve Packagepack

Reads:

* Packagepack source.
* Dependencies.
* Compatibility constraints.
* Available source identities.

May change:

* Composition resolution state.

Successful resolution determines the effective composition.

---

## Build Packagepack

Requires a Buildable context.

May change:

* Build state.
* Build outputs.
* Build caches.
* Diagnostics.

On success:

> Missing/Stale/Failed
> → Current

On failure:

> build attempt fails

while preserving:

* Source.
* Previously installed Vapor App.
* Selection.
* Unrelated provider authentication.

unless a separate explicit operation legitimately changes those.

---

## Install/Register Vapor App

May operate on:

* Locally built Vapor App.
* Workshop-acquired Vapor App.
* Depot-shipped default Vapor App.

Changes:

* Vapor App installation state.
* Vapor-managed installation metadata.

Does not inherently change:

* Source.
* Capability.
* Git authentication.

---

## Select Vapor App Composition

Requires a selectable installed composition.

Changes:

* Selection state.

Should preserve:

* Source.
* Build state.
* Capability.
* Provider authentication.
* Other installed Vapor Apps.

---

## Launch Vapor App

Requires a Locally Runnable context.

Changes:

* Runtime state.

Normal Player launch must not require:

* Source resolution.
* Git acquisition.
* Local compilation.

---

## Stop Vapor App

Changes:

* Runtime state.

Preserves:

* Installation.
* Selection.
* Build/source state.

Appropriate runtime-generated user state may be persisted separately.

---

## Remove Vapor App

Changes:

* Installation state.
* Potentially selection if the removed App was selected.

Must not silently remove canonical source merely because the corresponding built App is removed.

---

## Authenticate Provider

Changes provider-specific authentication/authorization state.

Must not silently mutate:

* Source.
* Build.
* Installation.
* Selection.

Authentication may unlock previously unavailable remote Actions.

---

## Publish Source

Defined in detail by the Publishing and Distribution Model.

May change:

* Source publication state.
* Remote Git state.
* Registry linkage.

Should not inherently publish a built Workshop artifact unless the larger publication operation explicitly includes that stage.

---

## Publish Built Vapor App

Defined in detail by the Publishing and Distribution Model.

May change:

* Workshop publication state.
* Registry built-publication linkage.

Requires a complete Packagepack-derived build.

---

# Interaction Rules

The usefulness of the multidimensional model comes primarily from explicit interaction rules.

---

## Source Change → Build Currency

A relevant source or composition change may invalidate the currency of an existing build.

Conceptually:

> Build = Current
>
> * relevant source change
    >   → Build = Stale

The installed old Vapor App may remain runnable.

---

## Failed Build ≠ Destroyed Previous Build

If rebuilding fails:

* New build state may be Failed.
* Diagnostics become available.
* Previously installed valid Vapor App should remain unaffected unless explicitly replaced.

This allows:

> dirty/new source + failed new build + still-runnable previous Vapor App

without inventing a special composite state.

---

## Authentication ≠ Local Capability

GitHub logout must not remove Content Developer capability.

Steam Workshop unavailability must not prevent editing local source.

Registry unavailability should not unnecessarily prevent already-resolved local runtime execution.

---

## Selection ≠ Source Mutation

Selecting another Vapor App should not mutate Git repositories.

Changing a Git branch should not implicitly change the current/default Vapor App unless an explicit composition/build/install/select operation follows.

---

## Capability Downgrade ≠ Source Destruction

Capability controls what operations Vapor exposes.

It does not redefine authored repositories as disposable.

---

## Runtime ≠ Composition Assembly

Starting a Vapor App operates on an already built composition.

The runtime does not normally become an implicit Composer.

---

# Failure Modeling

Failures should be modeled as operation results affecting only relevant dimensions.

Vapor should avoid duplicating the full state space into:

* success lifecycle,
* failure lifecycle,
* failure-then-success lifecycle,
* nested failure lifecycle,
* and every combination thereof.

For example, a build attempt can:

> Read current source/composition state
> → attempt build
> → succeed or fail

A failure may produce:

* Build = Failed.
* Diagnostics = Available.

while leaving:

* Source = Dirty.
* Installed App = Installed.
* Selection = Selected.
* Runtime = Stopped.

unchanged.

This makes failure compositional.

The same principle should apply to:

* Git operations.
* Downloads.
* Publication.
* Toolchain repair.
* Registry access.
* Steam Workshop access.

---

# Lifecycle Projections

A lifecycle is a useful projection over the larger Situation model.

It is not the complete state of Vapor.

---

## Steam App Instance Lifecycle

Conceptually:

> Steam install
> → usable Player environment
> → optional capability upgrades
> → normal operation
> → update/repair/move
> → optional capability downgrade
> → uninstall

This lifecycle concerns the product instance itself.

---

## Capability Lifecycle

Conceptually:

> Player
> → Composer
> → Content Developer
> → Ecosystem Developer
> → Root Authority

Downgrades may move in the opposite direction.

The exact meaning of Root Authority establishment may differ from ordinary local tooling upgrades.

---

## Source Lifecycle

Conceptually:

> Absent
> → acquired/created
> → clean
> → modified
> → committed
> → synchronized/published

These conditions need not form one strictly linear chain.

For example, a repository may be committed locally but not pushed.

---

## Composition Lifecycle

Conceptually:

> Packagepack source
> → unresolved
> → resolved valid/invalid
> → source changes
> → requires re-resolution

Resolution is a logical composition concern, not itself a build.

---

## Build Lifecycle

Conceptually:

> Missing
> → Current
> → Stale
> → Current

with failures possible from build attempts without requiring a second parallel lifecycle.

The initial projection is shown in [Vapor Operational State Model](./Diagrams/Vapor%20Operational%20State%20Model.puml).

---

## Installation Lifecycle

Conceptually:

> Not installed
> → installing
> → installed
> → removed

Installation may occur from:

* Steam depot.
* Steam Workshop.
* Local build output.

---

## Selection Lifecycle

Conceptually:

> Candidate installed Vapor Apps
> → one selected/default composition

Selection is a Steam App Instance convenience/context concern.

It is not content ownership or source identity.

---

## Runtime Lifecycle

Conceptually:

> Stopped
> → Starting
> → Running
> → Stopping
> → Stopped

with failure/crash conditions represented separately where relevant.

---

## Publication Lifecycle

Publication spans multiple external systems and therefore has its own dedicated model.

See:

[Vapor Publishing and Distribution Model](./Vapor%20Publishing%20and%20Distribution%20Model.md)

---

# Local State and Ownership

Operational behavior must distinguish state by ownership and recoverability.

---

## Steam-Managed State

Examples:

* Depot-shipped Vapor binaries.
* Default built composition.
* Steam-owned installation metadata.

Steam is the installation authority.

---

## Vapor-Managed Product State

Examples:

* Capability metadata.
* Selected composition.
* Installed Vapor App records.
* Registry caches.
* Launcher settings.
* Tool detection/configuration metadata.

Much of this should be repairable or reconstructible.

---

## User Gameplay State

Examples:

* Saves.
* Quicksaves.
* User settings.
* Keybinds.

This is user-valued persistent state.

---

## Git-Managed Authored State

Examples:

* Container Repos.
* Vapor Workspaces.
* Pack source.
* Behavioral source.
* Commits.
* Branches.
* Uncommitted changes.

Local-only authored state may not be recoverable from any remote source.

Vapor must treat it conservatively.

---

## Build/Cache State

Examples:

* Cargo target directories.
* Downloaded dependencies.
* Incremental build state.
* Generated intermediates.
* Packaging temporaries.

This state is generally regenerable.

---

## Installed Vapor Apps

Installed Vapor Apps are not canonical source but are first-class local runnable products.

They may often be reacquired or rebuilt.

Removal should nevertheless be explicit.

---

## Authentication State

Authentication and credentials must remain distinct from ordinary content, build, and cache state.

The exact secure-storage mechanism belongs to lower-level implementation design.

---

# Operational Invariants

* Player operation does not require Git.
* Player operation does not require Rust/Cargo.
* Player launch does not require source.
* Player launch does not perform composition builds.
* A valid Packagepack resolves to exactly one effective Engine.
* A valid Packagepack resolves to exactly one effective Game.
* The effective Engine declares the main binary.
* A Vapor App is one complete statically resolved composition.
* Relevant composition/source changes may stale a build.
* A stale source/build relationship does not automatically invalidate an older installed Vapor App.
* A failed rebuild must not silently destroy the previous valid installed App.
* Selecting a Vapor App must not silently mutate Git source.
* Provider authentication changes must not silently alter local source/build/install state.
* Capability downgrade must not silently delete authored source.
* Build/cache cleanup must distinguish regenerable state from user-authored state.
* Runtime launch must operate on an already built composition.
* External-provider outages should interfere only with operations that actually require those providers.
* User-authored local Git state must be protected from destructive automation.

---

# Open Operational Questions

The current model still needs deeper work in several areas:

* Exact complete state-dimension inventory.
* Exact formal definition of each named Context.
* Whether Contexts should become first-class runtime/domain-model objects or remain derived concepts.
* Exact concurrency/locking semantics when multiple Actions overlap.
* How long-running Actions expose progress and cancellation.
* How partial install/download states are represented.
* How operation history and diagnostics are modeled.
* How externally modified Git/Steam/filesystem state is reconciled.
* How selected Vapor App Composition identity relates to installed build identity over time.
* How updates and version changes affect Build/Installation/Selection dimensions.
* How publication rollback interacts with local source/build state.
* Whether a future formal state-machine representation should be generated from shared model definitions rather than maintained separately.

These should be resolved as concrete workflow implementation approaches.