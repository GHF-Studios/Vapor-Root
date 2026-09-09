> [!info]
> **Owns:** graphical SDK / Development Mode interaction, Explorer/Inspector/work-area organization, GUI projection of Context/Selection, visual design principles, Problems/tools surfaces, and external editor integration UX.
>
> **Uses:** all focused architecture/development models.
>
> **Does not own:** canonical identity, Role semantics, source acquisition mechanics, Cargo semantics, or operation legality.

---

# Product Thesis

The Vapor SDK should be the primary first-party graphical environment for developing Vapor Content and, where Role permits, Vapor itself.

> **Enter Development and let Vapor become the Vapor-aware development environment.**

External editors/tools may integrate, but Vapor-specific workflow belongs to Vapor.

---

# Launcher ↔ SDK

Conceptually:

```text
Launcher Mode
    ↕
SDK / Development Mode
```

The exact process/window architecture is implementation detail.

Development Mode is a richer projection of the same Client/Core, not a competing application architecture.

---

# Role Projection

Role definitions live elsewhere.

SDK projects them into capability.

```text
Composer:
    composition Projects, packs, resolution, build/run, publication

Content Developer:
    + implementation source, Rust/Cargo, tests, Git, code diagnostics

Ecosystem Developer:
    + Client, Platform Server, Examples, infrastructure, deployment
```

---

# Visual Design Ancestor

The existing Figma-derived/Tauri-hosted prototype remains visual ancestry.

Its obsolete semantic model is not normative.

Desired characteristics include:

```text
JetBrains-like professional density
dark neutral surfaces
subtle separators
compact controls
tight spacing
restrained radii
clear active/inactive states
semantic warning/error/success
monospace identity/code/output
precise hover/focus/selection
tool-window lower panels
contextual Inspector
graph visualization
```

---

# Primary Window Anatomy

A useful broad structure:

```text
┌────────────────────────────────────────────────────────────┐
│ application chrome                                          │
├────────────────────────────────────────────────────────────┤
│ Context / subject / selection / toolchain / operations      │
├──────────────┬───────────────────────────┬───────────────────┤
│ Explorer     │ Main Work Area            │ Inspector         │
├──────────────┴───────────────────────────┴───────────────────┤
│ Problems / Build / Test / Terminal / Git / Steam / Tools    │
├────────────────────────────────────────────────────────────┤
│ Status                                                       │
└────────────────────────────────────────────────────────────┘
```

Exact docking/sizing may evolve.

---

# Explorer Thesis

Default to modeled Vapor structure, not raw filesystem dumping:

```text
Superworkspace
└── Authority
    └── Source Repo Container
        └── Source Repo / Vapor Workspace
            └── Project
                └── implementation/Cargo/files
```

Do not show fake extra layers such as Source Repo → Workspace or Project → independently identified Content.

---

# Health Visibility

Objects should remain visible when unhealthy/unavailable:

```text
registered Container not acquired
declared Repo checkout missing
invalid Project metadata
provider unavailable
derived IDE state stale
```

Show object + problem instead of disappearing the object.

---

# Git State Presentation

States such as:

```text
Modified
Ahead 2
Detached HEAD
Differs from parent gitlink
```

may be visible without being labelled "Broken".

Git synchronization and Vapor health are different dimensions.

---

# Context Presentation

Persistent Context should be visible enough to understand, perhaps as a semantic breadcrumb:

```text
GHF-Studios › Loo-Cast › Game
```

Changing Context should be deliberate.

Merely clicking a tree row need not mutate it.

---

# GUI Selection

Transient row/node selection is frontend interaction state.

It may become explicit Operation Selection when an operation is invoked.

It is not persistent Context and not a durable hidden focus set.

---

# Session State

Open editors, expanded nodes, tool windows, navigation history, and pinned views are frontend session state.

They must not silently become future operation targets.

---

# Inspector / Work Area

Inspector may show:

```text
canonical identity
Project kind
provider linkage
Git state
dependency/composition
Cargo realization
build/test state
publication state
diagnostics
```

Main work area may host source editors, manifests, graphs, diffs, generated views, docs, runtime tools, or operation reports.

---

# Problems / Tool Windows

Useful tool surfaces may include:

```text
Problems
Build
Test
Terminal
Git
Cargo
Steam
Logs
Diagnostics
Operations
```

Diagnostics preserve provenance; a Rust compiler error should still look like a Rust compiler error with Vapor context attached.

---

# External IDE Integration

Vapor may integrate RustRover/IntelliJ/other editors.

External IDEs should receive:

```text
correct Project root
managed rust-analyzer/toolchain
Cargo context
source paths
```

without becoming authority for Vapor identity/context.

---

# Build SDK First

During the rewrite, implementing the richer SDK before final Launcher polish remains reasonable because SDK exercises more Core semantics.

Launcher can later become a reduced projection of already-proven architecture.

---

# Design-System Extraction

Preserve useful visual ancestry as reusable tokens/components:

```text
surfaces
panels
tree rows
tabs
buttons/inputs/selectors
Inspector components
badges
diagnostics/status
graphs
tool windows
typography/spacing/radii
interaction states
```

The frontend stack itself is not normative.

---

# UI/Core Boundary

GUI actions resolve into the same semantic operation requests used by CLI/automation:

```text
subject
Context
Selection
variant
parameters
```

Frontend convenience must not create a second operation architecture.
