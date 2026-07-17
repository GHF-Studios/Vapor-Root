# Workstream — Workshop Content Superpass

- State: foreground
- Goal: implement and document the SteamUGC/Workshop-backed custom-content
  stack, using Loo-Cast as the first proving workspace.
- Ordered successor: return to `source-atomization-raw-info-dump` after a safe
  implementation checkpoint unless the user redirects.
- Depends on: owner direction in the current session.
- Semantic mutation claim: Vapor-Shell source/content command grammar,
  Workshop/content lifecycle implementation, Loo-Cast content publication
  metadata and scripts, Vapor-Root user/agent docs for the shell model.
- Path claims: `Vapor-Shell/crates/vapor_shell/**`, `Loo-Cast/**`,
  `README.md`, `AGENTS.md`, `.agents/workstreams/README.md`, and this record.

## Completed

- Read the requested Shell design and command docs.
- Read Vapor-Root README, AGENTS, planning startup files, and open workstream
  records.
- Confirmed target worktrees were clean before mutation.
- Added the Vapor-Shell content lifecycle module for discovery, validation,
  packaging, acquire/cache, install/update, verify, disable/enable, uninstall,
  repair/quarantine, fingerprints, locks, receipts, and Workshop create/publish/
  delete authority boundaries.
- Replaced public source selection/index commands with `source *` grammar in
  parser, host facade, diagnostics, docs, scripts, and tests.
- Added Loo-Cast Workshop intent metadata and two scripts:
  `content-roundtrip` and `content-publish-preview`.
- Added explicit `content subscribe` and `content download` verbs that share the
  controlled acquire/cache provider path until live SteamUGC is available.
- Added app-owned packagepack selection state with `content select`,
  `content selected`, and `content deselect`.
- Updated Vapor-Shell command, manifest, content, discovery, architecture,
  scripts, Steam development, and design docs.
- Updated Vapor-Root README as the human/product entry point and AGENTS with
  product-scope orientation.
- Verified `cargo test -p vapor_shell`.
- Verified real local proof through `/tmp/vapor-proof-app.AtkxhP/bin/vapor`:
  opened the real Loo-Cast checkout, ran `script run content-roundtrip`, and
  ran `script run content-publish-preview`.
- Re-ran `script run content-roundtrip` after adding subscribe/download verbs;
  it exercised package, subscribe/acquire, download/cache, install, verify,
  update, disable, enable, repair, and uninstall.
- Re-ran `script run content-roundtrip` after adding packagepack selection; it
  exercised selection write/read/clear through the installed binary.
- Resolved Workshop package layout to a single deployed artifact-root model:
  `content package` stages directly under `output/content/packages/<slug>`,
  writes a resolved deployed `Vapor.toml` at the artifact root, and no longer
  emits a separate `Vapor-package.toml` or `payload/` wrapper.
- Preserved the source/runtime distinction: custom content authoring remains
  workspace-bound, while packaged, installed, cached, and Workshop-downloaded
  runtime artifact roots carry deployed `Vapor.toml` metadata.
- Updated package, cache, install, and publish paths so Workshop VDF
  `contentfolder` points at the artifact root, cache installs read deployed
  `Vapor.toml`, dry-run package fingerprints match deployed artifacts, and
  Workshop tags are de-duplicated.
- Updated Vapor-Shell content, command, manifest-schema, and Workshop design
  docs plus content lifecycle tests for the deployed `Vapor.toml` model.
- Rebuilt and installed the Loo-Cast app-local `vapor` binary, then verified
  `script run content-roundtrip` and `script run content-publish-preview`
  against that final binary.
- Exposed the full `content *` and `root *` command surfaces through the direct
  app-local host facade so live authority operations can run without relying on
  the interactive shell PTY.
- Implemented controlled live SteamCMD-backed Workshop create, publish, and
  numeric PublishedFileId download/import paths for the current Steam provider
  bridge.
- Created real Loo-Cast Workshop items with build account `ghf_vapor_build`:
  `spacetime-engine` -> `3762162251`, `loo-cast-game` -> `3762162407`, and
  `loo-cast-packagepack` -> `3762162548`.
- Recorded those PublishedFileIds in the three Loo-Cast content `Vapor.toml`
  files.
- Published all three items through Steam once with the manifests that were
  then authored as `visibility = "private"`.
- Found the Workshop visibility contradiction: Steam had accepted private VDFs,
  so the items existed under the publisher account but were not visible in the
  public Workshop surface.
- Updated the three Loo-Cast manifests to `visibility = "public"` and generated
  public Workshop VDFs for the same PublishedFileIds.
- Added `content publish ARTIFACT...` so multiple Workshop VDFs can be uploaded
  through one SteamCMD provider session instead of one login per artifact.
- Rebuilt and installed that batch-publish binary into the live Loo-Cast Steam
  app folder.
- Retried the public visibility republish after the Steam auth cooldown. One
  SteamCMD provider session logged in as `ghf_vapor_build` and committed all
  three `workshop_build_item` updates successfully with `visibility = "public"`
  and change note `Set initial Vapor Workshop items public.`.
- Recorded the successful public republish receipts at timestamp `1783766095`
  for all three artifacts.
- Downloaded/imported the real `spacetime-engine` Workshop item
  (`3762162251`) into the app-owned cache after fixing SteamCMD download-folder
  resolution from the shared Steam workshop directory.
- Updated cached install recursion so a Workshop-downloaded packagepack can
  install required dependencies from app-owned cache entries after source close.
- Verified the three public Workshop items are visible on Steam after the
  app-level/item-level visibility gates were corrected.
- Added batch `content download TARGET...` and used one SteamCMD provider
  session to download/import `3762162251`, `3762162407`, and `3762162548` into
  the app-owned cache.
- Closed source, installed the Workshop-downloaded packagepack from cache with
  dependency recursion, selected it, verified, updated, disabled/enabled,
  repaired, and explicitly uninstalled packagepack, game, and engine. Installed
  state ended empty; cache/index state remained available.
- Cancelled the first real `root publish` attempt after SteamPipe scanned about
  2.6 GiB. The staged root depot was runtime plus `packages/setup`; the setup
  payload alone contained the Rust toolchain, Cargo home, Git, and SteamCMD.
- Split root depot staging into runtime default and explicit stacked setup mode:
  default `root package`/`root publish` stages runtime-only app payload;
  `--include-setup-payload` is now required to stage `packages/setup`.
- Added `[workspace].binaries` so Vapor-Shell declares `binaries = ["vapor"]`
  in its `Vapor.toml`; `root build` now promotes the installed shell through
  the manifest-driven Vapor workflow.
- Used the existing source-controlled bootstrap script once as a bridge because
  the previously installed binary could not yet parse `[workspace].binaries`.
  After that, `vapor root build` promoted one `vapor` binary through Vapor.
- Re-ran `root publish --dry-run`; staging reported `payload: runtime only`,
  staged 160 MiB under `output/root/content`, and contained only the app
  runtime surface.
- Ran the real runtime-only SteamPipe root upload interactively as
  `ghf_vapor_build`. SteamPipe successfully finished AppID `2122620` build
  `24160810`; depot `2122621` uploaded 1070 chunks, created manifest
  `8985564292651668226`, and staged 1427 files / 157.04 MiB. `packages/setup`
  was absent from the real publish staging tree.
- Verified the freshly reinstalled Steam app root from the runtime-only depot:
  it contains the shipped runtime surface, and `bin/vapor setup self status`
  correctly reports an unregistered app root plus missing app-local Rust, Git,
  SteamCMD, and self-setup payload until `setup self install` is run.
- Added explicit local-only development deploy verbs:
  `root deploy [--skip-docs]` builds/promotes root binaries and docs into the
  Steam app root without SteamPipe, and `content deploy ARTIFACT [--select]`
  builds/installs source content into app-owned installed content without
  Workshop publication.
- Added content runtime output declarations in `Vapor.toml`: authored and
  deployed content manifests can carry `binaries` and `libraries`; package and
  deploy copy declared Cargo outputs into deployed artifact `bin/` and `lib/`
  directories instead of inventing a proprietary payload metadata format.
- Declared `spacetime-engine` as a content-shipped executable through
  `spacetime-engine/Vapor.toml` with `binaries = ["spacetime-engine"]`.
- Removed the leftover public-source-navigation implementation surface after
  the `cd`/`up`/`pwd`/`ls` command removal: the shell now keeps source
  selection/root context without retaining dead path-changing helpers or docs
  that describe them as user-facing commands.
- Bumped the canonical Vapor Rust toolchain intent to stable Rust `1.97.0`
  dated `2026-07-09`, retaining the channel/date model for dated channels but
  using the release version as the stable install identifier.
- Verified focused checks after the local deploy/runtime-output/toolchain
  changes: `cargo fmt --check` in Vapor, Vapor-Shell, and Vapor-SDK;
  `cargo clippy -p vapor_core --all-targets -- -D warnings`;
  `cargo test -p vapor_core`;
  `cargo test -p vapor_shell --test command --test content`;
  focused Vapor-Shell state/command/installation/content integration tests;
  `cargo clippy -p vapor_shell --all-targets -- -D warnings`; and
  `cargo check -p vapor_sdk_cli`.
- Corrected the immediate Steam Shell launch slice: bare `bin/vapor` now
  detects a GUI/no-terminal start, spawns a real terminal emulator, runs Vapor
  there, then starts an interactive shell so the window remains usable until the
  user closes it manually.
- Tightened the Steam Shell terminal path after live testing: KDE sessions now
  prefer Konsole, the spawned terminal gets the app `bin/` directory in `PATH`,
  terminal launch attempts are logged under `.vapor/logs/terminal-launch.log`,
  and the original Steam-launched Vapor process waits for the terminal session
  so Steam does not tear the window down after startup.
- Simplified the immediate Steam Shell launch path further to Linux/Konsole
  only: `/usr/bin/konsole` is now the sole terminal target, Steam runtime linker
  variables are scrubbed before launching it, and the runner skips the fallback
  interactive shell when a terminal close signal is observed.
- Replaced the noisy REPL startup diagnostics with a compact first-run overview:
  `Status`, one `Next` command, an optional `Then` command, and a short
  `help`/`exit` reminder. Detailed setup and package-payload diagnostics remain
  in explicit setup commands instead of appearing on every Shell launch.
- Began applying the same UX shape beyond startup: `setup self status`,
  `source status`, `source list`, `source add`, `source open`, `source close`,
  empty `content verify`, and top-level host help now use clearer `Status` /
  `Next` sections and avoid default diagnostic dumps where normal runtime users
  do not need them.
- Found and repaired the installed docs breakage: the aggregate docs index
  linked to per-project `index.html` files that the docs builder did not
  generate. Vapor Shell now writes a section index after each Rustdoc copy, and
  the installed app docs were repaired and verified locally over HTTP.
- Rebuilt `vapor` from source and copied it into the Steam app root because the
  fresh runtime-only app root still lacks self-setup tools and cannot run the
  normal `root deploy` path yet.
- Corrected the workspace/content project model boundary: `[workspace]`
  manifests now register Vapor-managed child projects through
  `[[workspace.projects]]`, while each child `Vapor.toml` owns its own
  `[project]` or content identity. Content discovery now uses those
  registrations instead of recursively guessing publishable artifacts from
  arbitrary nested manifests.
- Registered the three live Loo-Cast first-party content projects in
  `Loo-Cast/Vapor.toml`: `spacetime-engine`, `loo-cast-game`, and
  `loo-cast-packagepack`. The real public PublishedFileIds remain in the child
  content manifests.
- Updated `metadata`, `source status`, and `source repair` output to separate
  Cargo workspaces from registered Vapor projects, so the shell no longer
  presents Cargo package membership, source roots, and content artifacts as one
  implied concept.
- Verified the registered-project slice with `cargo fmt --check`,
  `cargo test -p vapor_shell --test workspace --test content --test metadata --test command`,
  `cargo clippy -p vapor_shell --all-targets -- -D warnings`, and
  `cargo build -p vapor_shell --bin vapor`; then copied the rebuilt binary into
  the Steam app root and smoked installed `source open`, `metadata`,
  `source status`, `source repair`, and `content list` against the real
  Loo-Cast source.
- Added the first product Play entrypoint as a Vapor Shell one-shot command:
  `launch loo-cast`. Steam launch options should target the platform wrapper
  under `.vapor/launch/<platform>/vapor.*`; Play uses wrapper argument `play`
  and Shell uses wrapper argument `shell`.
- Added app-root first-party content seeds in `Vapor-Root/Vapor.toml` under
  `[[root.content]]` for the public Loo-Cast engine, game, and packagepack
  Workshop items. This lets a runtime-only install resolve default content
  before any source checkout or registry client is open.
- Changed Workshop acquisition to use SteamCMD `anonymous` login by default
  when no account is passed, so public Workshop downloads are attempted instead
  of rejected before provider execution.
- Changed missing-cache install behavior so `content install` can resolve a
  root content seed, download/cache the Workshop item, then continue the normal
  install path. Cached packagepack dependencies now install by dependency
  `workshop-id` when present, falling back to content ID/seed lookup.
- Added `workshop-id` hints to the Loo-Cast packagepack's engine and game
  references so the next packagepack Workshop update can carry dependency
  download hints directly in its deployed `Vapor.toml`.
- Verified the launch/Workshop seed slice with `cargo fmt --check`,
  `cargo test -p vapor_shell --test command --test content --test metadata`,
  `cargo clippy -p vapor_shell --all-targets -- -D warnings`,
  `cargo build -p vapor_shell --bin vapor`, and `git diff --check`; then copied
  the rebuilt binary and updated root `Vapor.toml` into the Steam app root and
  smoked installed `vapor launch --help`.
- Added `--account ACCOUNT` to `launch loo-cast` and `content install` so
  unreleased/private Workshop testing can use `ghf_vapor_build` or another
  account with app/content access instead of relying on anonymous SteamCMD.
  The selected account is threaded through root-seed acquisition and dependency
  install recursion.
- Corrected the first playable terminal runtime proof after rejecting the
  invalid `game.runtime` model: `loo-cast-game` now declares its engine through
  `[game.engine]`, depends on `spacetime-engine` through Cargo, ships
  `libloo_cast_game.so`, and owns the WIP game definition in Rust code. The
  installed `spacetime-engine` binary resolves the packagepack's game, verifies
  the game's engine dependency, loads the installed game library, and calls a
  Rust-native entrypoint exported from that library.
- Locally deployed the updated Loo-Cast packagepack into the Steam app and
  verified `vapor launch loo-cast` hands off to the new terminal runtime.
- Fixed `content deploy ARTIFACT --select` so selection falls back from the
  authored/source selector to the installed packagepack artifact ID emitted by
  the deploy reports.
- Published the refreshed Loo-Cast Workshop items again after the terminal
  runtime and wording fixes. Verified from an uninstalled/cacheless app state
  that `launch loo-cast --account ghf_vapor_build` downloaded all three live
  Workshop IDs, installed dependencies, selected the packagepack, and reached
  the game handoff.
- Published the refreshed runtime-only Steam app/depot to `vapor-dev` as build
  `24258893`, depot manifest `7291224128649500980`.
- Added a first friend-grade source bootstrap path: `source init basic-content
  PATH --organization ORG --name NAME [--app-id APPID]` creates a normal
  Cargo/Vapor workspace with engine, game, and packagepack projects; it writes
  a lockfile so app-local `--locked` Cargo workflows work immediately.
- Added `source repair --write` source metadata repair for dependency
  `workshop-id` hints derived from sibling artifacts' own
  `published-file-id` values. This supports the first Workshop publication
  loop where engine/game PublishedFileIds must be copied into dependent
  game/packagepack metadata before dependent items are created or updated.
- Registered `Vapor-Examples` projects as real content examples and staged
  `examples/vapor-examples/` in the runtime app payload while excluding `.git`
  and `target`.
- Published the template/examples app/depot slice to `vapor-dev` as Steam build
  `24259447`, depot manifest `3853568432132708180`, baseline
  `7291224128649500980`.
- Verified the template slice with focused Vapor-Shell tests, strict clippy,
  Vapor-Examples checks/tests, installed-app `source init`, `content validate`,
  `content deploy ... --select`, `content verify`, and `root package` staging
  that includes examples without `.git` or `target`.
- Extracted the terminal runtime proof out of the first-party Loo-Cast content
  workspace and into `Vapor-Examples` as three explicit example projects:
  `terminal-engine`, `hello-world-on-steroids-game`, and
  `hello-world-on-steroids-packagepack`.
- Gutted the current first-party Loo-Cast Rust payload back to product
  placeholders while preserving the engine binary and game library outputs so
  the packagepack still has real installable runtime artifacts.
- Updated Loo-Cast, Vapor-Examples, and Shell docs so the dynamic terminal
  proof is no longer framed as the Loo-Cast product runtime.
- Verified the extraction with `cargo fmt --check`, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, and `git diff
  --check` in the touched content workspaces/docs.
- Fixed source selection for explicitly opened nested source workspaces:
  ambient invocation from root submodules still resolves to the containing
  Vapor-Root, but `source open /path/to/Vapor-Examples` and saved source state
  now honor the exact nested `[workspace]` marker.
- Verified installed-app local deploy smoke after the topology correction:
  opened `Vapor-Examples`, validated 6 registered content projects, locally
  deployed `hello-world-on-steroids-packagepack`, verified installed content,
  opened `Loo-Cast`, validated the 3 product placeholder artifacts, locally
  deployed/selects `loo-cast-packagepack`, verified content, and ran
  `launch loo-cast` with terminal relaunch suppressed. The handoff reached the
  placeholder Spacetime Engine message.
- Added a Windows `LoadLibraryW`/`GetProcAddress` dynamic-loader path to the
  terminal engine example so the example runtime is no longer Linux-only in
  source. The Linux host only has Linux Rust targets installed, so the Windows
  cfg still needs to be compiled on the owner's Windows/MSVC machine.
- Added minimal OS-facing Steam launch wrappers under
  `.vapor/launch/linux/vapor.sh` and `.vapor/launch/windows/vapor.cmd`. The
  wrappers only select Play vs Shell mode and hand off to the installed
  target-specific `bin/<target>/vapor[.exe]`; Vapor Shell remains the central
  implementation surface.
- Changed content runtime output staging from flat `bin/`/`lib/` paths to
  target-specific `bin/<target>/` and `lib/<target>/` directories. Deployed
  `Vapor.toml` files now record actual staged files through target-specific
  `runtime` entries.
- Added `content package/create/publish --target TARGET` support, including
  repeated `--target` values for one package/Workshop item containing multiple
  platform payloads. Focused tests cover explicit Windows/MSVC package staging
  and a two-target Workshop dry-run payload.
- Extended the same target-specific layout to root/app binaries:
  release-mode Vapor Shell binaries are promoted and staged under
  `bin/<target>/vapor[.exe]`, while `.vapor/launch/...` wrappers are the stable
  Steam-facing entrypoints. Installation discovery now accepts both bootstrap
  `bin/vapor[.exe]` and release `bin/<target>/vapor[.exe]`.
- Added repeatable `root build --target TARGET`, `root deploy --target TARGET`,
  and `root publish --target TARGET` for root/app binary builds. Depot smoke
  now rejects shipped launch wrappers when the matching target-specific Vapor
  binary is missing.
- Added `[root.runtime].targets` and `[workspace.runtime].targets` as the
  canonical release target matrices. `root *` and content build/deploy/package/
  create/publish now accept `--release-targets` to consume those matrices,
  while no target flag remains host-only for quick local Linux iteration.
- Root/depot staging now copies selected `bin/<target>/` directories instead
  of the whole `bin/` tree and copies only launch-wrapper platform directories
  implied by the selected targets.

## Owned uncommitted changes

- Vapor-Shell content artifact-root packaging/cache/install/publish
  implementation, live SteamCMD provider bridge, batch publish/download grammar,
  root runtime-vs-stacked depot staging, manifest-declared binary promotion,
  local root/content deploy verbs, content runtime output declarations/copying,
  docs, and tests.
- Vapor core/SDK toolchain metadata updated for stable Rust `1.97.0`.
- Loo-Cast content manifests with real PublishedFileIds and public visibility
- Loo-Cast content manifests with real PublishedFileIds and public visibility
  intent, the `spacetime-engine` shipped executable declaration, and placeholder
  first-party Rust payloads.
- Vapor-Examples now includes the extracted terminal runtime proof projects and
  updated README/workspace registration.
- Vapor-Root workstream record update.

## Remaining

- Do not publish the extracted topology until the owner explicitly starts that
  authority boundary; the live Workshop items currently still reflect the last
  published terminal-proof-in-Loo-Cast state.
- Implement and test the Windows/MSVC content build/runtime path before friend
  testing: Windows depot binary availability, app-local setup, target-specific
  runtime outputs, and ABI policy. The example dynamic loader now has a Windows
  implementation, but it has not yet been compiled or run under MSVC.
- Do not point live Steam launch options at `.vapor/launch/...` until a new
  depot containing those wrapper files is published. Existing published depots
  can only launch files they already contain. Windows launch options also need
  a shipped Windows `bin\vapor.exe`; wrapper scripts alone are not enough.
  After the depot includes them, use Linux Play
  `.vapor/launch/linux/vapor.sh play`, Linux Shell
  `.vapor/launch/linux/vapor.sh shell`, Windows Play
  `.vapor\launch\windows\vapor.cmd play`, and Windows Shell
  `.vapor\launch\windows\vapor.cmd shell`. Those wrappers expect
  `bin/x86_64-unknown-linux-gnu/vapor` and
  `bin\x86_64-pc-windows-msvc\vapor.exe` respectively.
- Run the owner-controlled long bootstrap from a fresh runtime-only depot
  install: `setup self install`, open the Vapor and Loo-Cast sources, locally
  deploy root and selected first-party content, then iterate on the startup
  path.
- Continue the content model pass: packagepack/game/engine dependency semantics,
  legal modding declarations, content-shipped executable/library semantics,
  command-domain naming, contextual help, and an explicit command design for
  adding/removing `[[workspace.projects]]` entries without hidden source edits.
- Continue the beginner development loop: turn the template into a playable
  reusable engine/game contract rather than only a buildable/publishable content
  skeleton, add import/adopt/repair operations for existing arbitrary source
  trees, and test the first Workshop create sequence with a non-owner account
  once an appropriate account is available.
- Continue the game/modding demo slice: packagepack-declared mod legality,
  installed optional mod content, and runtime discovery through Vapor-managed
  installed composition state.
- Keep final create, publish, and delete authority-changing operations manual
  and interactive; scripts may automate only dry-run/preflight.

## Smallest resume action

Use the owner's Windows laptop to compile the corrected `Vapor-Examples`
terminal proof and Vapor Shell under `x86_64-pc-windows-msvc`, then stage one
dry-run content package and one app/depot package with both Linux and Windows
runtime targets before deciding whether to republish Workshop content or update
the app/depot payload.
