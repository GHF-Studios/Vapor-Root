> [!info]
> This document defines the intended graphical Vapor SDK product experience.
>
> The SDK is a Launcher-integrated development mode backed by Vapor Core.
>
> Structural identity, session, focus, selection, and ambiguity semantics are defined in the **Vapor Context, Identity, Session And Selection Model**.
>
> Broader source/development workflows are defined in the **Vapor Development Experience Model**.

---

# Product Thesis

The Vapor SDK should be the primary first-party graphical environment for developing Vapor Content and, where Role permits, Vapor itself.

The intended experience is not:

> Open a thin launcher, then leave Vapor and manually coordinate Git, Cargo, RustRover, SteamCMD, manifests, generated files, composition graphs, and deployment state yourself.

The intended experience is:

> **Enter Development and let Vapor become the development environment.**

The SDK is therefore an integrated IDE-like Vapor surface.

It may use or integrate external editors and tools where useful, but it should progressively own the Vapor-specific development experience itself.

---

# Launcher to SDK Transformation

The preferred product experience is one primary Vapor desktop application which can change between:

```text
Launcher Mode
↕
SDK / Development Mode
```

Conceptually:

```text
Vapor Launcher
    ↓ Enter Development
Vapor SDK
    ↑ Return to Launcher
```

The transition may visually reconfigure the application substantially.

A literal animated transformation is not required.

The important semantic property is:

> **SDK Mode is a development-oriented superset of the ordinary Launcher surface, not an unrelated application with a separate model.**

See [Vapor Desktop Surface Model](./Diagrams/Vapor%20Desktop%20Surface%20Model.puml).

---

# Superset Model

Launcher Mode exposes the workflows appropriate to ordinary use at the installed Role.

Depending on Role these may include:

* Vapor Apps;
* Content Library;
* Packagepacks/composition;
* accounts;
* settings;
* diagnostics;
* publication/discovery surfaces;
* ordinary Content management.

SDK Mode preserves access to relevant Launcher functionality and adds development capability such as:

* Superworkspace navigation;
* Workspace/Project development;
* source editing;
* structured manifest/configuration editing;
* composition/dependency graph views;
* Inspector;
* build/test/run workflows;
* Problems;
* logs;
* Git/provider state;
* Cargo/toolchain visibility;
* repair workflows;
* publication/deployment tooling;
* ecosystem-development surfaces.

SDK Mode is therefore an onion-like higher capability surface.

Returning to Launcher Mode removes or hides development chrome without requiring a separate semantic universe.

---

# Installer Boundary

The Vapor Installer remains distinct.

The SDK is a superset of operational Launcher functionality.

It is not a superset of fundamental Installation mutation.

The Installer remains responsible for operations such as:

* installing/removing Role capability;
* installing fundamental managed tooling;
* major environment provisioning;
* uninstall preparation;
* capability-level repair.

Conceptually:

```text
Installer
    changes what the Installation can do

Launcher
    uses installed capability

SDK
    uses higher installed capability for development
```

This boundary should remain visible even if some Installer status/diagnostics are inspectable from the Launcher/SDK.

---

# Role Projection

SDK functionality grows with Role.

A Content Developer may see:

* Content source;
* Projects;
* code;
* manifests;
* dependency/composition tooling;
* build/test/run;
* Content publication.

An Ecosystem Developer additionally sees:

* Vapor ecosystem source;
* Vapor Root development;
* SDK/Launcher/CLI development;
* server/root infrastructure where locally available;
* ecosystem deployment;
* provider/authorization-sensitive development operations.

Role controls local capability exposure.

Authority separately controls protected external operations.

---

# Build the SDK First

During the rewrite, the richer SDK surface may be implemented before the simplified final Launcher surface.

This is acceptable and desirable.

The SDK exercises more of Vapor Core:

```text
identity
context
source
toolchain
Cargo
Git
Content
composition
build
test
run
diagnostics
deployment
```

A mature Launcher can later be implemented largely as a reduction/projection of the same application shell.

This avoids building a simplistic Launcher architecture which later has to be awkwardly expanded into an IDE.

---

# Visual Design Ancestor

The existing Figma-derived / Tauri-hosted Vapor SDK GUI prototype is the visual ancestor of the future SDK.

Its fake data and obsolete semantic assumptions are not normative.

Its visual language should be preserved aggressively where it continues to serve the real model.

The intended design character is:

* JetBrains/RustRover-like professional density;
* dark neutral surfaces;
* subtle separators;
* compact controls;
* small, restrained corner radii;
* clear active/inactive panel distinction;
* high information density without visual noise;
* semantic status colors;
* strong monospace treatment where identities/code/tool output benefit from it;
* restrained animation;
* precise hover/selection states;
* tool-window-like lower panels;
* contextual Inspector;
* graph visualization integrated with the same design system.

The mockup should not be dismissed as disposable merely because its underlying data model was incomplete.

---

# Design-System Extraction

The future implementation should extract the mockup's successful styling into a real Vapor design system rather than leaving it embedded in one generated component.

This should eventually define reusable tokens/components for:

* application surfaces;
* editor surfaces;
* panels;
* separators;
* active selections;
* tree rows;
* tabs;
* buttons;
* inputs/selectors;
* inspectors;
* badges;
* diagnostics;
* warning/error/success states;
* graph nodes/edges;
* tool windows;
* typography;
* spacing;
* radii;
* hover/focus states.

The exact frontend technology is not part of the normative design.

Tauri/React is a current implementation route rather than a permanent semantic requirement.

---

# Primary Window Anatomy

The SDK should retain the successful broad anatomy of the existing prototype:

```text
┌────────────────────────────────────────────────────────────┐
│ Menu / application chrome                                  │
├────────────────────────────────────────────────────────────┤
│ Context / target / toolchain / operation toolbar           │
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

Exact panel sizes and positions may become configurable.

The conceptual roles should remain recognizable.

---

# Development Environment / Superworkspace Selector

The SDK should provide an obvious development-environment selector.

This is the GUI projection of the Superworkspace model.

A configured Superworkspace may appear by friendly local name:

```text
Vapor Development
Loo Cast
Experiments
```

The user should not normally need to navigate to a filesystem path every time.

Opening another Superworkspace should be an explicit development-context operation.

Recent/configured Superworkspaces should be discoverable from the Launcher/SDK transition.

---

# Explorer

The Explorer should visualize modeled Vapor structure rather than merely dump the filesystem.

Conceptually it may show:

```text
Superworkspace
├── Container Repo
│   ├── Workspace
│   │   ├── Project
│   │   │   ├── Vapor Content
│   │   │   └── source/packages/files
│   │   └── ...
│   └── ...
└── ...
```

The user should be able to distinguish:

* semantic identities;
* physical realizations;
* Git/provider state;
* health;
* Open/Closed state;
* Focus;
* transient Selection.

Raw filesystem navigation may still be available below or alongside modeled entities.

---

# Explorer State

The Explorer should not hide objects merely because they are unhealthy.

A remembered Workspace may appear as:

```text
✕ Vapor-Examples
  Missing local realization

  [Locate] [Reacquire] [Close]
```

An incompatible Workspace may remain visible with diagnostics.

A dirty Git Workspace may show dirty status without being considered broken.

Health should augment the model rather than collapse it.

---

# Open and Focused Objects

Multiple Workspaces, Projects, and Content contexts may be Open.

Multiple objects may also be Focused.

The UI should represent this without implying one global `active_project`.

Focus should be visibly distinguishable from:

* tree expansion;
* transient Selection;
* editor-tab activation.

A user may inspect something without focusing it.

A user may focus several things without selecting all of them for the current action.

---

# Transient Selection

Tree selection, graph-node selection, Inspector target, or multi-selection may directly target an operation.

This is transient Selection.

For example:

```text
select three Libraries
→ Verify
```

may verify those exact Libraries without modifying persisted Focus.

Likewise:

```text
click one Project
→ Build
```

may explicitly target that Project for this action.

The UI should avoid turning ordinary clicking into surprising persistent context mutation.

---

# Main Work Area

The center Work Area should support multiple kinds of first-class Vapor documents/views.

These may include:

* Rust source;
* Vapor manifests;
* Workspace manifests;
* Packagepack configuration;
* structured Content configuration;
* composition/dependency graphs;
* validation views;
* build realization inspection;
* source/provider views;
* generated/read-only artifacts;
* documentation;
* future specialized Engine/Game tooling.

A text editor is one important view, not the entire SDK model.

---

# Source Editing

The long-term SDK may provide real source editing.

It does not need to reproduce every mature Rust IDE capability on day one.

Useful development stages include:

```text
initially
    Vapor-aware project/configuration environment
    + external Rust IDE integration

progressively
    integrated source editor
    language-server features
    navigation
    refactoring integration
    debugger integration where useful
```

External IDE support remains legitimate even as the built-in SDK becomes stronger.

The desired end state is not constrained to being merely a companion dashboard.

---

# Structured Editors

Where Vapor understands the schema, the SDK should offer structured editing without hiding the underlying authored representation.

For example a `Vapor.toml` dependency may be edited through:

* form/Inspector controls;
* graph interactions;
* raw TOML.

These are alternate projections over the same authored semantic state.

Advanced users should always be able to inspect the underlying files.

---

# Graph Views

Graph visualization is a first-class strength of the SDK.

Useful graph views may include:

* Vapor semantic dependency graph;
* Packagepack composition;
* Engine/Game/Mod relationships;
* Library dependencies;
* Cargo realization;
* publication/provider relationships;
* future capability/extension relationships.

Graph nodes should preserve the visual status language of the prototype:

```text
healthy
warning
error
selected
focused
```

A graph node is not a fake duplicate object.

It is another projection of the same Core identity.

Selecting it should therefore update the Inspector coherently.

---

# Inspector

The right-side Inspector should display contextual semantic information about the current transient Selection.

Possible tabs include:

```text
Properties
Dependencies
Validation
Source
Realization
Git
Publication
```

The exact tabs depend on object kind.

The Inspector is an important bridge between discoverability and power.

A user should be able to learn:

* what an object is;
* its canonical identity;
* where it comes from;
* whether it is healthy;
* what it depends on;
* what depends on it;
* what operations are available;
* why an operation is disabled.

---

# Operation Toolbar

The top operation area should expose the most relevant semantic Vapor operations for the current SDK state.

Examples include:

```text
Check
Verify
Build
Test
Run
Package
Publish
Repair
Deploy
```

Not every operation is always visible or enabled.

Availability depends on:

* installed Role;
* target kind;
* selection cardinality;
* health;
* local source availability;
* authority for protected targets.

Disabled operations should explain why.

---

# Target Cardinality in the GUI

The GUI should embrace multiplicity instead of forcing uniqueness globally.

For example:

```text
three selected Projects
+ Test supports Many<Project>
→ Test enabled
```

while:

```text
three selected Projects
+ Run requires ExactlyOne<Packagepack>
→ Run disabled / requires target selection
```

The UI should be able to show the reason directly.

This makes Vapor's Core cardinality model discoverable instead of mysterious.

---

# Run Context

Run deserves an explicit target model.

A development session may have several open/focused Content artifacts while one exact Packagepack or development composition is selected for Run.

The Run target should therefore be visible in the toolbar rather than inferred through hidden analogy.

Changing a Packagepack does not automatically retarget every focused Game/Engine/Mod to a “similar” object.

Exact identity wins.

---

# Toolchain

The active Vapor-managed toolchain should be visible and inspectable.

The SDK may expose:

* pinned Rust version;
* health;
* installation location where diagnostically useful;
* Cargo/Rust Analyzer state;
* repair/update operations.

The toolchain remains Installation-owned.

Its internal paths should not become ordinary UX vocabulary.

---

# Lower Tool Windows

The lower tool-window area should retain the prototype's strong IDE-like model.

Expected surfaces include:

## Problems

Aggregated Vapor/Cargo/Git/source diagnostics with semantic explanations.

## Build

Build progress and structured/raw build output.

## Test

Test runs and results.

## Terminal

A normal terminal surface where appropriate.

Vapor-managed tool commands may be available without globally exposing private toolchain binaries.

## Git / VCS

Status, diff, branches, remotes, conflicts, commit/push where Role/authority permits.

## Steam / Deployment

SteamPipe staging/deployment status, preview builds, branch/depot information, provider diagnostics.

## Toolchain

Managed Rust/Cargo/Rust Analyzer information.

## Logs

Vapor application/core/server/provider logs where relevant.

These may be rearrangeable/collapsible like familiar IDE tool windows.

---

# Diagnostics UX

Diagnostics should present Vapor meaning first and raw tool information second.

For example:

```text
Cargo realization for Wheel-Rules conflicts with its Vapor dependency.

wheel_core
    Vapor requires:
        GHF-Studios/.../Wheel-Core@0.1.0

    Cargo currently declares:
        ...

[Inspect] [Repair]
```

The user may then expand raw Cargo output.

The SDK should not require users to decode a Cargo failure before learning the Vapor-level problem.

---

# Repair UX

Repair should normally be contextual.

If one selected object has a known safe repair:

```text
Inspector
    Cargo realization: Missing

    [Repair]
```

is preferable to making the user discover a broad repair command.

Global Diagnose/Repair surfaces remain useful for ecosystem-wide health.

Safe canonical repair may also happen proactively during ordinary workflows.

---

# Git UX

Git should be visible, first-class infrastructure for Composer-and-above development without becoming the primary Vapor mental model.

The SDK should understand:

* repository identity;
* local realization;
* branch;
* dirty state;
* ahead/behind state;
* remotes;
* fork/upstream relationships;
* submodules;
* conflicts.

Destructive Git operations require explicit intent.

The user must always be able to access raw Git state.

---

# Multiple Realizations

If two realizations of the same Workspace identity exist, the Explorer should make the distinction obvious.

For example:

```text
Vapor
├── official · main
└── Leslie fork · context-redesign
```

Both may project the same semantic Workspace identity while differing in realization/provider state.

Actions which modify files/build source must ultimately resolve one physical realization.

The GUI should normally make this a visual choice rather than exposing filesystem paths as the primary discriminator.

---

# Launcher Surfaces Remain Reachable

SDK Mode should not imprison the user inside development chrome.

The user should be able to access ordinary Vapor surfaces such as:

* Apps;
* Content Library;
* Accounts;
* Settings;
* composition;
* installed artifacts.

This may happen through persistent navigation or a deliberate mode transition back to Launcher Mode.

Returning to Launcher Mode should preserve enough SDK Resume State to return later.

---

# Enter Development

A normal Ecosystem Developer experience should become approximately:

```text
Start Vapor
    ↓
Launcher
    ↓ Enter Development
SDK resumes previous Superworkspace/session
    ↓
continue work
```

If no development environment exists yet, the SDK provides:

```text
Open existing
Acquire
Create/configure Superworkspace
Recent development environments
```

The ordinary path should not begin by asking the user for Cargo paths.

---

# Return to Launcher

Returning to Launcher Mode should:

* preserve recoverable SDK session/resume state;
* stop or warn about explicitly SDK-owned live processes where required;
* hide development surfaces;
* retain ordinary Launcher/Product state.

It should not destroy source or reset development context.

---

# Closing Development Context

Closing a Workspace/Project inside the SDK follows the shared Context Model.

If descendant contexts close as a consequence, the GUI communicates the cascade.

If live/volatile work is at risk, the SDK prompts.

The UI should distinguish:

```text
Close
Forget
Remove checkout
Delete
```

rather than using one destructive “Remove” button for every meaning.

---

# GUI vs CLI

The SDK and CLI are equal in semantic capability where their Role surface permits it.

They are not required to share interaction grammar.

The GUI may use:

* browsing;
* selection;
* drag/drop;
* context menus;
* inspectors;
* dialogs;
* graphs;
* visual status.

The CLI/automation may use:

* canonical identities;
* selectors;
* explicit flags;
* machine-readable results.

Both call the same Vapor Core operations.

The GUI should not be made worse merely because some interaction would be awkward to spell in a shell.

---

# External IDE Integration

External IDE integration remains useful.

The SDK may provide:

```text
Open in RustRover
Open in IntelliJ IDEA
Open in VS Code
Reveal Project
```

where supported.

The external IDE may use the same Vapor-managed Rust toolchain configuration.

Vapor remains authoritative for Vapor semantics even when code is edited externally.

Changes made externally should be reflected through filesystem/Git/project observation rather than assuming the SDK is the only editor.

---

# First Rewrite SDK Slice

The first serious SDK implementation should prioritize the richest architecture-proving path rather than attempting feature completeness.

A useful order is:

```text
1. preserve/extract prototype visual system
2. Launcher → SDK mode shell
3. real Installation / Role / toolchain status
4. real Superworkspace Explorer
5. real Workspace/Project identities and local realizations
6. Open / Focus / Selection session model
7. real Problems/diagnostics
8. managed build/test/Cargo operations
9. real Content/Packagepack views
10. real semantic graph view
11. structured Inspector
12. Git/provider/deployment surfaces
13. progressively stronger integrated editing
```

The fake prototype data should disappear incrementally as real Vapor Core state replaces it.

---

# Worked Journey — Start Work

The user starts Vapor.

Launcher Mode appears.

They choose:

```text
Development
```

The window reconfigures into SDK Mode.

The previous `Vapor Development` Superworkspace resumes.

The Explorer restores the previous Open Workspaces/Projects.

The Work Area restores useful documents.

Problems/Inspector update from current Core state.

No terminal setup is required.

---

# Worked Journey — Build Vapor

The user selects the Vapor Client Project.

The Inspector shows:

```text
Project
GHF-Studios/Vapor-Root/Vapor/Client

Toolchain
Rust 1.97.0 · ready
```

The user presses Build.

The SDK invokes the same managed Core build operation available through CLI automation.

Build output appears below.

Cargo details remain inspectable.

---

# Worked Journey — Two Projects

Both:

```text
Vapor/Client
Vapor-Examples/Examples
```

are Open and Focused.

The user multi-selects both and presses Test.

If Test supports multiple Projects, both participate.

The user presses an operation requiring exactly one target.

The SDK requests an exact target rather than silently selecting one.

---

# Worked Journey — Broken Checkout

A previously Open Workspace has been moved externally.

The Explorer keeps it visible with Missing state.

The Inspector explains the problem and offers:

```text
Locate
Reacquire
Close
Forget
```

The user chooses Locate and points Vapor at the moved checkout.

If canonical identity matches, the existing realization record is repaired.

---

# Worked Journey — Return to Play

The user finishes development and selects:

```text
Return to Launcher
```

Development chrome collapses/disappears.

The normal Apps/Library surface returns.

SDK Resume State remains available for the next Development transition.

---

# SDK Invariants

* SDK Mode is a Launcher-integrated development superset.
* Installer remains a distinct capability-management boundary.
* SDK and Launcher share Vapor Core semantics.
* The SDK may be implemented before the simplified final Launcher.
* The existing Figma/Tauri prototype is a visual design ancestor.
* Prototype fake semantics are not normative.
* Visual density, borders, colors, spacing, radii, panels, and Inspector character should be preserved where practical.
* Superworkspace is the primary graphical development-environment container.
* Explorer shows modeled Vapor structure, not merely a filesystem dump.
* Open, Focused, and transient Selected are distinct.
* Multiple objects may be Open and Focused.
* Operation cardinality determines whether multi-target execution is valid.
* Broken remembered contexts stay visible where recovery is useful.
* The Inspector explains selected semantic objects and available actions.
* Graph views project real Core identity rather than duplicate fake objects.
* Vapor-oriented diagnostics precede raw tool output.
* Raw Git/Cargo/provider reality remains inspectable.
* SDK and CLI may use radically different interaction mechanics while invoking the same operations.
* External IDE integration remains supported.
* The built-in SDK may progressively become a stronger IDE over time.

---

# Open SDK Questions

* Exact built-in source editor technology/capability timeline.
* Exact navigation layout between Launcher and SDK modes.
* Exact multi-window support.
* Exact persistence granularity of document tabs/layout.
* Exact debugger integration.
* Exact run-configuration model.
* Exact visual component library/frontend implementation.
* Exact graphical Git workflow depth.
* Exact graph editor interaction model.
* Exact Composer exposure to partial SDK surfaces.
