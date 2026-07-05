> Bootstrap source snapshot. Unresolved and awaiting assertion-by-assertion intake.

# One Vapor World, Many Repositories

The goal is not “make Vapor-Root convenient.”

The goal is to make **multi-repo development a normal Vapor capability**, then
make Vapor-Root its first demanding first-party user.

Steam installs Vapor's complete control plane: SDK, Launcher, Rust, Cargo, Git,
service integrations, caches, output, and operational state. From there, someone
opens one crate, one project, or a declared graph of projects stored **outside**
the Steam application directory.

Vapor knows how to create, synchronize, change, validate, review, and publish
those projects without making the user operate Git, GitHub, Cargo, SteamPipe,
and Workshop separately. Move or repair the application, reconnect the external
projects, and continue working. Open a completely different multi-repo project
and use the same Vapor operations. That is the target.

## A workspace type, not a famous folder

```text
                         MULTI-REPO WORKSPACE
             declaration + resolution + local operation state
                               │
             ┌─────────────────┼─────────────────┐
             │                 │                 │
             ▼                 ▼                 ▼
        Vapor-Root       content project     another team's graph
        first-party      any party           any combination
        platform graph   Workshop outputs    of repositories
```

Vapor-Root is one instance. It is useful because it is complicated and because
we control all its repositories—not because its names or paths should be built
into the SDK.

A workspace may coordinate first-, second-, or third-party repositories. It may
build platform tools, custom content, or both. Membership says “these sources
are being developed and tested together.” It does not automatically say who
owns them or how their artifacts are distributed.

## The two example workspaces that must shape the design

### First-party platform workspace

```text
Vapor-Root
├── Vapor                shared foundation
├── Vapor-SDK            authors, builds, packages, publishes
├── Vapor-Launcher       installs, composes, launches
├── Vapor-Examples       proves public authoring workflows
└── Vapor-Registry       identity and publishing authority
```

This workspace integrates the platform itself. Its app-root artifacts are
published through the first-party release path, including SteamPipe where
appropriate.

`Vapor-Registry` is part of the intended picture but is not currently present
in Vapor-Root's `.gitmodules`.

### Custom-content development workspace

```text
Loo Cast development workspace        not Vapor-Root
├── Spacetime-Engine source repo ─┐
│                                 ├── build/test together
└── Loo-Cast-Game source repo ─────┘

Spacetime-Engine artifact ───────────► Workshop item
Loo-Cast-Game artifact ──────────────► Workshop item
```

Spacetime Engine and Loo Cast are consumers of Vapor, not pieces of the
first-party Vapor platform. Their source repos therefore do not become
Vapor-Root members. Their own workspace can still coordinate a cross-repo engine
and game change using the same generic machinery.

The source workspace produces Workshop artifacts. Installing those artifacts
does not install their Git repositories, branches, or development metadata.
The two-repo/two-item mapping shown here is this project's intended case, not a
framework rule: another repo may produce several items, or one item may require
sources from several repos.

These two examples are the minimum design test. If the model only expresses
Vapor-Root, it is still a special case. If it only expresses custom content, it
will miss first-party platform release needs.

## Two sides of a hard wall

```text
INSIDE THE STEAM APP                     OUTSIDE THE STEAM APP
Vapor-managed control plane              user-owned source
───────────────────────────              ─────────────────
SDK + Launcher                    opens   Vapor workspace declaration
vendored Git                     ───────► member source repositories
managed Rust + Cargo                      Cargo workspaces and crates
Steam/GitHub/package clients              project-local Vapor schema
caches + output + staging
depot + Workshop runtime artifacts
```

The wall is categorical: Vapor may place tools, generated output, published
artifacts, and installed content inside the Steam application. It must never
create or clone a source repository there. Workspace roots live in a user-data
location or another valid external location explicitly chosen by the user.

Several layers still exist on the source side, and each has one job:

- A Vapor workspace declares the multi-repo project graph.
- A source repository carries history, versioning, and a project boundary.
- A Cargo workspace coordinates Rust packages inside its owning repo.
- A crate or content source is a buildable/packageable source unit.
- A built artifact becomes a crate release, app-root file, or Workshop item.

Vapor connects these layers while hiding the backend mechanics. Installed
artifacts can compose into one final Steam application world; their source repos
remain outside it.

## Source graphs and distribution graphs are different

The multi-repo workspace answers:

> Which source revisions are we changing and proving together?

Distribution answers:

> Which built artifacts are published, through which channel, under whose
> authority?

```text
SOURCE SIDE                              DISTRIBUTION SIDE
───────────                              ─────────────────
repo ──build──► artifact ──────────────► crates.io / GitHub
repo ──build──► app package ───────────► SteamPipe
repo ──build──► content artifact ──────► Workshop
                                          │
                                          ▼
                                     installed content
```

This is why Loo-Cast-Game and Spacetime-Engine can participate in a shared
development workspace without participating in Vapor-Root, and why their
relationship to an installed Vapor instance is through Workshop items. Their
source relationship to their own development workspace is a separate choice;
submodules could still be one way that workspace materializes its members.

## Identity is not role is not location

Several independent questions describe each member and output:

| Question | Example answers |
| --- | --- |
| Who controls it? | first-party, delegated second-party, third-party |
| What does it do? | platform tool, engine, game, mod, pack, registry data |
| Where is its source? | workspace member, standalone checkout, remote dependency |
| What is produced? | Rust crate, executable, app package, content artifact |
| How is it distributed? | crates.io, GitHub, SteamPipe, Workshop, local only |

No column determines another. A first-party repo may produce Workshop examples.
A second-party engine may be a Workshop item. A third-party project may use a
multi-repo workspace without receiving first-party authority.

`Second-party` still needs a precise definition. The likely shape is delegated,
registry-recorded authority over a bounded namespace and publishing scope—not a
folder under Vapor-Root.

## Three kinds of workspace truth

“Put metadata in `.vapor`” is really pointing at three different lifecycles:

```text
What this workspace means          The exact graph that passed
─────────────────────────          ───────────────────────────
human-authored declaration         reproducible resolution
members + roles + requirements     repo commits + dependency/toolchain pins
perhaps Vapor.toml                 perhaps Vapor.lock
                  \                /
                   \              /
                    ▼            ▼
                  This machine right now
                  ──────────────────────
                  ignored mutable state
                  paths + caches + worktrees
                  perhaps .vapor/
```

The filenames are not decided. The split is important:

- Declaration is reviewable and can describe any multi-repo workspace.
- Resolution reconstructs one exact constellation in CI or on another machine.
- Local state maps that constellation onto external checkouts and app-local
  tools on this machine.
- Credentials are referenced from local/platform-secure state, not committed in
  the declaration or resolution.
- None of these state classes grants permission to place a Git repo beneath the
  Steam application directory.

For Vapor-Root, the current `.gitmodules` supplies part of the resolution and
checkout behavior, but almost none of the semantic declaration. Submodules may
remain one backend; they cannot be the whole framework.

## The real atom is larger than a commit

A meaningful change can cross repository boundaries in any workspace.

```text
First-party change: portable workspace support
├── Vapor             shared schema or primitives
├── Vapor-SDK         generic workspace behavior
├── Vapor-Examples    public author proof
└── Vapor-Root        exact integrated revision set

Custom-content change: new engine/game contract
├── Spacetime-Engine  engine-side change
├── Loo-Cast-Game     game-side adoption
└── content workspace exact integrated revision set
```

Git cannot make either example one atomic commit, and GitHub cannot make it one
atomic PR. That is fine. The human unit of intent can be one change while review
and merge remain repo-local.

The generic workspace mechanism needs a small cross-repo change record:

- the overall intent;
- participating repos;
- the branch, PR, or commit representing each part;
- dependency and merge order;
- the exact integrated revision set that passed;
- the artifacts and distribution steps affected;
- whether the change is integrated, merged, released, or distributed.

```text
planned → being built → integrated together → merged → released → distributed
```

Those final states remain separate. A merged engine change is not automatically
a released Workshop update.

## The development loops

### One crate or one repository

Open an external standalone project. Vapor supplies the app-local toolchain and
environment. The project resolves honest canonical dependencies and retains its
own history, version, and release boundary. Vapor presents project changes,
checks, review, and publishing without requiring the user to operate the backing
Git/GitHub concepts directly.

No multi-repo workspace is required.

### Any multi-repo workspace

Open its external declaration. Vapor creates or finds the member repos outside
the app directory, resolves their backing revisions, and redirects selected
dependencies to members being developed together.

Vapor translates backend state into product-level questions:

- Which projects have changes, conflicts, unpublished work, or remote updates?
- Which dependencies are released, resolved remotely, or being developed here?
- Is this workspace coherent, and what blocks its next check or release?

Checks run in dependency order over one member, an affected subgraph, or the
whole workspace. A successful resolution says, “these exact source revisions
worked together.”

Vapor-Root uses this loop. A Loo Cast/Spacetime workspace uses this loop. The
behavior should differ only where their declarations and publishing authority
differ.

### Promotion and distribution

Integration produces a known-good source graph. Promotion turns that graph into
versioned artifacts. Distribution sends each artifact through its real channel.

```text
one Vapor change
   │
   ▼
hidden repo histories + remote reviews
   │
   ▼
resolved multi-repo integration
   ├── standalone-member proof
   ├── whole-workspace proof
   ├── app-root relocation proof, when relevant
   └── artifact packaging dry runs
   │
   ▼
versioned artifacts
   ├── crates.io / GitHub releases
   ├── SteamPipe: first-party app root
   └── Workshop: engines, games, mods, and packs
```

SteamPipe and Workshop are not two buttons for the same operation. SteamPipe
updates the application world; Workshop introduces content into that world.
Registry policy may authorize publishing without merging the pipelines.

## The non-negotiable placement rule

The downloaded Steam application contains the tools and the final composed
runtime. Source projects always live elsewhere:

```text
Steam application directory/
├── SDK + Launcher
├── vendored Git
├── managed Rust + Cargo
├── depot and Workshop artifacts used by the runtime
├── caches, output, package staging, and app-local state
└── NEVER a source repository

user data or user-chosen external location/
├── Vapor-Root/
├── Loo-Cast workspace/
└── any other single-repo or multi-repo Vapor project/
```

Vapor must reject creating, cloning, importing, or relocating a managed repo
inside the app directory. A Steam update, repair, uninstall, or “verify files”
must never gain ownership of source history.

App portability and project portability are separate. The app brings its known
toolchain—including Git—to an external project. The project brings its Vapor
schema and source graph to any compatible Vapor installation. Reconnecting them
must not depend on an old absolute app path.

## Vapor's interface consumes the backends

Vapor is not a menu of thin wrappers. It presents its own small workflow and
aggressively maps that workflow onto vendored tools and remote services.

| The user asks Vapor to... | Vapor may internally... |
| --- | --- |
| create a project/workspace | choose an external location, write the schema, initialize repos, create authorized GitHub resources |
| add a member | clone or initialize a repo, configure remotes, create/manage a submodule or other resolved-member representation |
| start a change | create coordinated branches/worktrees and record the affected graph |
| save/synchronize work | commit, fetch, merge/rebase as policy permits, push, and update resolutions |
| share for review | create and connect the required GitHub PRs and aggregate their state |
| check the workspace | invoke Cargo and other validators locally or through GitHub Actions in graph order |
| release | version, tag, build, create GitHub/crate releases, and update downstream resolutions |
| publish | drive SteamPipe or Workshop according to artifact role and authority |

The right column is intentionally not the normal UX. Users work with Vapor
projects, members, changes, checks, releases, and publications. Git branches,
submodules, PR sets, VDFs, and service-specific state are backend projections of
that model.

Vapor therefore owns strong conventions and may regenerate or repair managed
backend state. It still needs safe failure behavior and valid underlying repos
for recovery and interoperability; that does not make raw Git the product.

## What must be designed first

Schema work is not active yet. First capture and verify the raw information about
what survives, what one repo means, and how Vapor relates to the underlying
systems.

→ **[Read and correct the raw info dump](../../docs/raw-info-dump/README.md)**

The bytes in that dump—not the surrounding framework prose—are the active
planning surface. The following table remains background context only:

| Concern | The model must answer |
| --- | --- |
| placement | external workspace roots; an enforced ban on repos inside the app directory |
| membership | repository identity, source URL, role, optional/required status |
| resolution | exact revisions and the graph known to work together |
| dependency modes | published, remote/unreleased, and local member override |
| operation scope | one repo, affected subgraph, or whole workspace |
| visible lifecycle | Vapor-native project/change/check/release/publish states |
| backend projection | generated Git/submodule/GitHub/Cargo/Steam state |
| outputs | artifact identity and crates.io/GitHub/SteamPipe/Workshop lane |
| local state | paths, caches, worktrees, and credential references |

The current sibling path dependencies are not a valid fourth mode. They are an
implicit Vapor-Root local override stored as canonical child-repo configuration.

## Planning order from here

```text
NOW
 │
 ├─ 1. Raw info dump
 │     Capture small, trivially correctable bits and group them into bytes.
 │
 ├─ 2. Vapor-visible lifecycle and failure states
 │     Define change, synchronization, review, release, and recovery.
 │
 ├─ 3. Composed-project schema + Git projection
 │     Define subprojects, pins, build/publication context, and local state.
 │
 ├─ 4. Repo, crate, content, and artifact graphs
 │     Separate source composition from runtime/distribution composition.
 │
 ├─ 5. App-local control plane + external-project proof
 │     Vendor tools; test relocation, repair, and app deletion.
 │
 ├─ 6. Reconciliation, CI, and publication recovery
 │     Handle native changes and non-atomic remote services honestly.
 │
 └─ 7. Adversarial proof + service automation
       Prove private, nested, shared, offline, and partially authorized cases.
```

Do not open detailed decision batches yet. First make the raw direction complete
enough that later organization reflects the owner's actual model rather than a
framework generated to fill gaps.

## Current evidence

- [`Vapor-Root/.gitmodules`](../../.gitmodules) is the current first-party
  special-case source set.
- [`Vapor-SDK/crates/vapor_sdk_core/Cargo.toml`](../../Vapor-SDK/crates/vapor_sdk_core/Cargo.toml)
  assumes that special-case sibling layout.
- [`Vapor-Launcher/crates/vapor_launcher_core/Cargo.toml`](../../Vapor-Launcher/crates/vapor_launcher_core/Cargo.toml)
  makes the same assumption.
