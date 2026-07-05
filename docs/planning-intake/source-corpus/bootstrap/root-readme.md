> Bootstrap source snapshot. Unresolved and awaiting assertion-by-assertion intake.

<div align="center">

<h1>Vapor-Root</h1>

<p><strong>The first-party source root for the Vapor platform.</strong></p>

<p>
  <a href="docs/raw-info-dump/README.md"><img alt="Status: early development" src="https://img.shields.io/badge/status-early_development-f59e0b"></a>
  <a href=".gitmodules"><img alt="Projects: four submodules" src="https://img.shields.io/badge/projects-4_submodules-6f42c1"></a>
  <img alt="Source: external to Steam" src="https://img.shields.io/badge/source-external_to_Steam-1f6feb">
</p>

<p>
  <a href="#projects">Projects</a> ·
  <a href="#getting-started">Getting started</a> ·
  <a href="#documentation">Documentation</a>
</p>

</div>

Vapor-Root brings the first-party Vapor repositories into one pinned source
tree. It supports changes that cross project boundaries while keeping every
project in its own Git repository.

It is also the first proving ground for Vapor-managed multi-repo development;
the resulting workflow must remain useful beyond Vapor-Root itself.

> [!WARNING]
> Vapor-Root is in early development. The current source tree works through Git
> submodules, while the streamlined Vapor-managed workflow is still being
> designed.

## Projects

| Project | Purpose |
| --- | --- |
| [`Vapor`](Vapor/) | Shared platform concepts and core crates |
| [`Vapor-SDK`](Vapor-SDK/) | Authoring, validation, building, and publishing |
| [`Vapor-Launcher`](Vapor-Launcher/) | Installing, composing, repairing, and launching |
| [`Vapor-Examples`](Vapor-Examples/) | First-party examples of public authoring workflows |

Vapor-Root is scoped to the first-party platform. Games, engines, mods, and
other custom-content projects remain separate projects.

## Getting started

### Acquire Vapor

Everything Vapor is acquired through the Steam app bundle named
**[Loo Cast](https://store.steampowered.com/app/2122620/Loo_Cast/)**. The bundle
brings together the Spacetime Engine, Loo Cast game, and Vapor ecosystem needed
for the complete installed experience. The Steam store page is not live yet.

### Work on the first-party source

The Steam installation provides Vapor's tools and runtime. Clone Vapor-Root
outside the Steam application directory when working on the platform itself:

```bash
git clone --recurse-submodules https://github.com/GHF-Studios/Vapor-Root.git
cd Vapor-Root
```

To initialize or restore the pinned projects in an existing clone:

```bash
git submodule update --init --recursive
```

Vapor-Root already contains the first-party project checkouts as submodules;
separate sibling clones are not required.

### Choose an interface

Current workflows should be available as `vapor_*_cli` commands. The intended
GUI equivalents are future `vapor_*_gui` guides and tutorials that lead users
through the same operations without replacing their CLI form.

## Documentation

- [Raw info dump](docs/raw-info-dump/README.md) — small, owner-verifiable facts
  about the intended system and its current implementation.
- [Multi-repo development notes](cross-repo/portable-multi-repo-development/README.md)
  — exploratory architecture and workflow material.
- [Submodule configuration](.gitmodules) — the current project paths and
  remotes.
