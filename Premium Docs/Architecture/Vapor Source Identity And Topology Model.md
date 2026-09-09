> [!info]
> **Owns:** canonical source hierarchy, Authority, Source Repo Container, Source Repo / Vapor Workspace, Project, Vapor Paths, canonical identity, and the fact that the Superworkspace has no Vapor identity.
>
> **Uses:** Installation/Local State and Role/Auth.
>
> **Does not own:** create/acquire workflow, Git reconciliation mechanics, CLI grammar, Context/Selection, or repository migration execution.

---

# Core Thesis

Vapor source has exact hierarchical identity.

The canonical hierarchy is:

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

Example:

```text
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

Shorter selectors may resolve to this identity, but shorthand never becomes a second canonical identity.

---

# Authority

An **Authority** is the first globally meaningful segment of Vapor source identity.

It is a Vapor/Registry concept.

It may link to a provider organization/user, but provider ownership and Vapor Authority are not the same identity system.

---

# Source Repo Container

A **Source Repo Container** is a registered top-level Vapor Git repository grouping related Source Repos.

It:

```text
is a Git repository
belongs beneath one Authority
occupies a canonical local location
may declare Source Repos through Git submodules
is the conservative default acquisition boundary
```

Example:

```text
GHF-Studios/Loo-Cast
```

---

# Source Repo / Vapor Workspace

A **Source Repo** is the primary source-bearing Git repository beneath a Source Repo Container.

Normative invariant:

> **One Source Repo = one Vapor Workspace.**

These are two views of the same hierarchy node:

```text
Git/source view:
    Source Repo

Vapor development view:
    Vapor Workspace
```

A Source Repo contains one or more Projects and owns Workspace-level Vapor metadata.

Example:

```text
GHF-Studios/Loo-Cast/Game
```

---

# Project

A **Vapor Project** is a modeled development unit inside one Source Repo.

A Project:

```text
is not itself a Git repository
has identity within its Source Repo
may map to one coherent Rust/Cargo context
may contain several Cargo packages/targets
may be typed Vapor Content
may be internal/non-Content where appropriate
```

Example:

```text
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

---

# Project Kind

Project kind is a property of an existing Project.

Examples:

```text
Packagepack
Enginepack
Gamepack
Modpack
Engine
Game
Engine Mod
Game Mod
Extension Mod
Library
```

Kind is **not** another identity path segment.

For ordinary Content:

```text
Project identity
=
Content identity
```

---

# Vapor ID / Content ID

For ordinary Vapor Content, the human-readable canonical Content identity is the canonical Project identity.

It is not synonymous with:

```text
Git URL
provider repository ID
Git commit SHA
Steam Workshop Item ID
filesystem path
```

Those are provider/version/realization identities.

---

# Vapor Paths

A **Vapor Path** is a slash-separated structural path through the canonical hierarchy.

Paths may end at any layer:

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

Operations decide which kinds are meaningful targets.

Vapor must not automatically descend from a supplied parent to an arbitrary leaf.

---

# Full and Relative Paths

A **Full Vapor Path** begins at Authority and contains every structural segment down to the target.

A **Relative Vapor Path** may omit a complete leading prefix supplied by Context.

Given Context:

```text
GHF-Studios/Loo-Cast
```

the relative path:

```text
Game/Loo-Cast
```

may resolve to:

```text
GHF-Studios/Loo-Cast/Game/Loo-Cast
```

Relative paths may omit a prefix.

They may not omit arbitrary middle segments.

---

# Canonical Superworkspace

Vapor has one canonical local **Superworkspace**.

It:

```text
has no Vapor identity
is not an ID segment
is not inherently a Git repository
contains canonical local source realizations
may contain multiple Authorities
has one configured physical location
```

Conceptually:

```text
<Superworkspace>/
├── GHF-Studios/
│   ├── Vapor-Client/
│   ├── Vapor-Platform-Server/
│   └── Loo-Cast/
└── Some-Developer/
    └── Some-Mods/
```

The Authority directory prevents otherwise valid identities from colliding locally.

---

# Identity vs Local Path

Changing:

```text
~/Development/Vapor
```

to:

```text
/mnt/Development/Vapor
```

does not change Vapor identity.

That is **identity-preserving relocation** of the Superworkspace.

Changing hierarchy such as:

```text
GHF-Studios/Old/Repo/Project
→
GHF-Studios/New/Repo/Project
```

changes canonical identity and requires explicit migration.

---

# Provider Linkage

Provider identity is separate from Vapor identity.

A valid migration state may be:

```text
Vapor identity:
    GHF-Studios/Old-Container

provider repo:
    GHF-Studios/New-Container
```

because:

```text
Vapor identity
≠ provider repository name
```

Provider migration belongs to Migration docs.

---

# Canonical Local Realization

A canonical local realization is the checkout corresponding to registered source identity beneath the configured Superworkspace.

Git state such as:

```text
dirty files
local commits
detached submodule HEAD
branch divergence
```

belongs to that realization.

It does not automatically change identity or invalidate topology.

---

# Structural Movement

Moving a Project to another Source Repo, moving a Source Repo to another Container, or renaming structural identity segments changes canonical identity.

Filesystem motion alone does not define the identity migration.

Published identities may have additional historical constraints.

---

# Ownership Boundary

This model answers:

> **What source object is this and what is its canonical identity?**

Source Development answers how it is created/acquired.

Resolution Context answers how shorthand resolves.

Migration answers how today's historical topology reaches the target.
