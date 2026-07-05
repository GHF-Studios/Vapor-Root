> Bootstrap source snapshot. These assertions are unresolved until processed through the intake queue.

# Raw Info Dump

This is intentionally raw information, grouped only enough to make individual
bits easier to verify, reject, or correct.

Each byte is a nearby handful of related bits. The grouping does not imply a
schema, hierarchy, lifecycle, dependency order, or derived proposal.

## Byte — Steam-installed control plane

- Vapor development starts from the Steam-installed Vapor application.
- The Steam app contains the SDK and Launcher.
- The Steam app contains Vapor-managed Rust and Cargo.
- `RUSTUP_HOME`, `CARGO_HOME`, and the managed toolchain belong with the Steam
  app rather than the user's ordinary development environment.
- Vapor should vendor or automatically install and manage Git as well.
- Vapor should not depend on the user's system Git for its normal workflow.
- Deleting, repairing, moving, or uninstalling the Steam app must not destroy
  unique project truth.
- The Steam app may own state that is reinstallable, reconstructible, or
  disposable.

## Byte — External source

- Source repositories must never live inside the Steam application directory.
- Repositories live in user data or another external location chosen by the
  user.
- Vapor opens and manages projects wherever they were validly cloned.
- External projects must follow a specific Vapor schema.
- Installed depot or Workshop artifacts are not editable source repositories.
- Editing an installed artifact means returning to an external source project.
- Individual projects and crates must remain workable without opening
  Vapor-Root.

## Byte — Installed runtime

- Depot and Workshop artifacts may participate in the same final installed
  Vapor runtime.
- Everything Vapor is acquired through a Steam app bundle named `Loo Cast`.
- The `Loo Cast` Steam bundle brings together the Spacetime Engine, Loo Cast
  game, and Vapor ecosystem.

## Byte — CLI and GUI surfaces

- Current Vapor workflows should be available as `vapor_*_cli` commands.
- Those workflows should eventually also have `vapor_*_gui` guides and
  tutorials.
- The GUI guidance should lead users through the same operations rather than
  eliminate their CLI form.

## Byte — General multi-repo development

- Multi-repo development must be a general Vapor capability.
- Multi-repo development must not be special behavior implemented only for
  Vapor-Root.
- Vapor-Root is one first-party instance and example of the general mechanism.
- Vapor-Root contains the first-party Vapor platform projects, not every project
  related to the wider ecosystem.
- Spacetime-Engine and Loo-Cast-Game must not be part of Vapor-Root.
- Spacetime-Engine and Loo-Cast-Game are custom-content projects whose shipped
  forms are Workshop items.
- Spacetime-Engine and Loo-Cast-Game may still need their own multi-repo
  development arrangement.

## Byte — Current Vapor-Root implementation

- Vapor-Root currently composes projects with Git submodules.
- The current submodules are Vapor, Vapor-SDK, Vapor-Launcher, and
  Vapor-Examples.
- Vapor-Root currently pins one exact Gitlink revision for each submodule.
- Vapor-Registry belongs to the intended first-party platform picture but is not
  currently a Vapor-Root submodule.
- Vapor-SDK currently reaches `vapor_core` through the relative path
  `../../../Vapor/crates/vapor_core`.
- Vapor-Launcher currently reaches `vapor_core` through the relative path
  `../../../Vapor/crates/vapor_core`.
- Those relative paths assume the current Vapor-Root sibling layout.
- The current relative paths do not yet prove that independently cloned projects
  resolve dependencies correctly.
- The reusable project-composition schema is not settled yet.
- Vapor-managed setup, synchronization, and cross-project release guarantees are
  not implemented yet.
- The current native setup path is `git clone --recurse-submodules`.
- An existing clone can reconcile its pinned projects with
  `git submodule update --init --recursive`.
- Separate sibling clones of Vapor, Vapor-SDK, Vapor-Launcher, and
  Vapor-Examples are not required when their Vapor-Root submodules are used.
- The Vapor-Root submodule checkouts can be used to commit and push directly to
  their individual remotes.

## Byte — Projects and repository boundaries

- A project is exactly one Git repository.
- A project may contain extremely varied things, but the repository boundary
  must remain strict.
- A repo coordinating other repos must itself be a repo.
- Vapor must not require a magical non-repo directory above a group of repos.
- `Workspace` may be the wrong word for the repo-composition concept.
- `Member` currently feels like the wrong word for a contained repo.
- Every project must clearly declare whether it can be built standalone or only
  through an orchestrating parent repo.
- The current project/submodule direction should be locked provisionally and
  changed later if real implementation pain justifies it.

## Byte — Submodule composition

- A project may include other projects through Git submodules.
- Git submodules are the currently preferred way for one root repo to include
  and pin other repos.
- Vapor should make submodule creation, setup, synchronization, and management
  easy through Vapor itself.
- Vapor should recognize and manage repo compositions rather than treating every
  submodule as an opaque accident.
- A repo may bundle other repos when those repos are intended as parts of a
  larger product rather than separately deployed products.

## Byte — The Vapor garden

- Vapor should provide a nice, shiny, streamlined workflow over the underlying
  tools and services.
- Normal users should not need to operate Git, GitHub, Cargo, SteamPipe, or
  Workshop plumbing directly.
- Vapor may aggressively consume Git, GitHub, Steam, and related systems to fit
  Vapor's needs.
- Vapor's model should not stray unnecessarily far from how those underlying
  systems actually work.
- The goal is a pleasant garden, not walls for their own sake.
- Vapor should wrap and manage real repositories rather than mangle them into a
  second unrelated source-control universe.

## Byte — Ownership and classification

- Git and GitHub authentication and ownership originate with repository owners.
- An individual owner is a GitHub user connected to the required Steam user.
- An organization owner is a GitHub organization with multiple authorized
  GitHub and Steam users.
- First-party versus second-party is meaningful independently of root/platform
  versus custom-content.

## Byte — Changes and publication

- Partial publication must not count as successful publication.
- A meaningful unit of work may span a large set of commits or repo-local PRs.

## Byte — Planning method

- Short owner-direction statements are currently the preferred unit for planning
  and review.
- Raw direction should be dumped before attempting detailed organization.
- Future question batches should present short concrete proposals for approval,
  denial, or correction rather than ask the owner to invent the model unaided.
