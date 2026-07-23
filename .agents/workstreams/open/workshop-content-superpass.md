# Workstream — Workshop Content Superpass

- State: foreground
- Goal: implement and document the SteamUGC/Workshop-backed custom-content
  stack, using Loo-Cast as the first proving workspace.
- Ordered successor: return to `source-atomization-raw-info-dump` after a safe
  implementation checkpoint unless the user redirects.
- Depends on: owner direction in the current session.
- Semantic mutation claim: Vapor-Shell source/content command grammar,
  Workshop/content lifecycle implementation, Loo-Cast content publication
  metadata and scripts, Vapor-Root user/agent docs for the shell model, and
  Vapor-Installer bootstrap/lifecycle ownership.
- Path claims: `Vapor-Shell/crates/vapor_shell/**`, `Loo-Cast/**`,
  `Vapor-Installer/**`, `.vapor/launch/**`, `README.md`, `AGENTS.md`,
  `.agents/workstreams/README.md`, and this record.

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
- Captured the Steam installscript decision: because Valve installscript
  support does not provide the same Windows/Linux/SteamOS bootstrap behavior,
  Vapor does not use `installscript.vdf` for runtime bootstrap.
- Added `Vapor-Installer` as a root-local Cargo/Vapor project with a
  visual-first no-argument installer surface plus narrow headless
  `install`, `uninstall`, and `dev-env` install/uninstall/status commands.
- Implemented installer-owned player-mode install for app-local Git, SteamCMD,
  `.vapor/registry` Vapor-Registry checkout/update, installer logs,
  diagnostics directories, content cache/download directories, and receipts.
- Implemented explicit installer-owned development environment install/uninstall
  for Rust/Cargo and cross-build tooling; normal player-mode install does not
  install Rust/Cargo. Top-level `vapor-installer uninstall` removes both
  developer-mode tooling and player-mode mutable app-root state, while
  `dev-env uninstall` is only a downgrade back to player mode.
- Updated Steam launch wrappers so Play/Shell run headless
  `vapor-installer install --app-root <app-root>` before handing off to
  target-specific `vapor`, reporting installer failure through the first
  visible Shell with the installer log path.
- Removed the user-facing Shell `setup` command group. Metadata/workflow
  diagnostics point to `vapor-installer`, and normal workflow validation no
  longer blocks on legacy app-root acceptance state.
- Removed root setup-payload staging from Shell and SteamPipe generation:
  `root package`/`root publish` are runtime-only and no longer accept
  `--include-setup-payload`.
- Updated distribution staging/tests so root-local Vapor projects such as
  `Vapor-Installer` are discovered even before they become Git submodules, and
  staged target directories include `vapor-installer` alongside `vapor` when
  promoted.
- Refactored `Vapor-Installer` from monolithic source files into documented
  modules for CLI/TUI, model, app-root discovery, acquisition, filesystem
  confinement, Git bootstrap, runtime bootstrap, development environment, and
  path policy.
- Initialized and pushed the standalone `GHF-Studios/Vapor-Installer` Git
  repository at `b0e9e78`, then registered `Vapor-Installer` as a
  `Vapor-Root` submodule.
- Verified installer and Shell slices with `cargo fmt --check`, focused
  Vapor-Shell command/workspace/distribution tests, strict
  `cargo clippy -p vapor_shell --all-targets -- -D warnings`,
  `cargo test` and strict Clippy in `Vapor-Installer`, plus a local
  `vapor-installer uninstall --dry-run` preview against the local Steam app
  root.
- Built/promoted Linux and Windows runtime-matrix binaries for `vapor` and
  `vapor-installer` into the local Steam app root, staged a runtime-only depot
  containing both platform launch wrappers and no `packages/setup`, and
  published AppID `2122620` to `vapor-dev` as build `24274710`; depot
  `2122621` manifest `6914390332829415382`.
- Ran the newly shipped local `vapor-installer install` against the
  local Steam app root. Local player-mode status is ready: app-local Git,
  SteamCMD, Vapor-Registry, and generated state directories are present.
- Republished the simplified installer command model to Steam AppID `2122620`
  branch `vapor-dev` as build `24275153`; depot `2122621` manifest
  `7777478068318923421`, baseline manifest `6914390332829415382`.
- Added a first-class Steam Installer wrapper mode. Linux/Windows wrapper
  argument `installer` opens `vapor-installer` directly and skips the quiet
  player-mode install used by Play/Shell. Published to Steam AppID `2122620`
  branch `vapor-dev` as build `24275599`; depot `2122621` manifest
  `8367995713019312839`, baseline manifest `7777478068318923421`.
- Began the hard pivot away from the mono-depot runtime shape: `Vapor.toml`
  now requires `[root.steam.depots.<name>]` with common, Linux, and Windows
  depot IDs before root publication can proceed.
- Implemented split root staging under `output/root/content/{common,linux,windows}`:
  common receives OS-neutral app files, Linux receives Linux launch/runtime
  files, and Windows receives Windows launch/runtime files plus required runtime
  DLLs.
- Updated SteamPipe generation so `root publish --dry-run` writes one app-build
  VDF plus one depot-build VDF per staged depot, and command output prints the
  staged depot roots and Steamworks OS rule expectation.
- Updated distribution, command, manifest, and Steam development docs for the
  package/depot pivot and current launch-option model.
- Verified the split-depot Shell slice with `cargo fmt --all --check`,
  `cargo test -p vapor_shell --test distribution`,
  `cargo clippy -p vapor_shell --all-targets -- -D warnings`, and
  `cargo test -p vapor_shell --all-targets`.
- Recorded the real Steamworks DepotIDs in `App-Source.vapor.toml`: common
  `2122621`, Linux `2122622`, and Windows `2122623`.
- Replaced hardcoded depot file membership in Shell staging with manifest-owned
  depot include lists. Each depot now declares its own `include` entries with
  source/installation roots, from/to mappings, exclusions, and optional runtime
  target filters.
- Re-verified the manifest-driven depot staging slice with
  `cargo clippy -p vapor_shell --all-targets -- -D warnings` and
  `cargo test -p vapor_shell --all-targets`; root `Vapor.toml` also parses as
  valid TOML.
- Moved generated SteamPipe VDF file bodies out of Rust format strings and into
  checked-in templates under `resources/steam/steampipe-templates/`.
- Removed the bulky distribution staging tests and kept validation on real
  staging/dry-run command paths instead of historical negative-path assertions.
- Clarified the Windows runtime DLL contract: `libunwind.dll` is still required
  for the Windows GNU runtime, but build promotion/import must place it under
  `bin/x86_64-pc-windows-gnullvm/`; depot staging includes that runtime bin
  directory instead of copying directly from development toolchain paths.
- Removed the obsolete Shell self-setup package copy/status module. Setup
  packages are no longer reported in metadata and are no longer an alternate
  install path; install/uninstall lifecycle belongs to Vapor-Installer.
- Re-verified after the template/sample cleanup with `cargo fmt --all --check`,
  `cargo clippy -p vapor_shell --all-targets -- -D warnings`, and
  `cargo test -p vapor_shell --all-targets`.
- Split app-source and installed-app root manifests: Vapor-Root now uses
  `App-Source.vapor.toml`; the shipped app root uses `App.vapor.toml`; ordinary
  workspaces/content/registry still use `Vapor.toml`.
- Removed source-root `.vapor` payloads. Source-controlled launch/scripts now
  live under `resources/vapor/shell-scripts/` and
  `resources/vapor/vapor-scripts/`; installed app-root `.vapor/` is reserved
  for generated disposable state.
- Removed Shell-owned public setup command surfaces, setup package staging,
  PATH/location registration, and the remaining setup install/repair/uninstall
  implementation. Shell now only inspects app-local tool readiness and points
  missing prerequisites at Vapor-Installer.
- Re-verified the renamed manifests, resource layout, and setup trim with
  `cargo fmt --all --check`, `cargo clippy -p vapor_shell --all-targets -- -D
  warnings`, and `cargo test -p vapor_shell --all-targets`.
- Re-verified Vapor-Installer with `cargo fmt --all --check`, `cargo clippy
  --all-targets -- -D warnings`, and `cargo test --all-targets`.
- Ran a temp app-root staging smoke with fake app-local Rust/Git readiness:
  `root package --host-only` staged common and Linux depots, then
  `root publish --dry-run --skip-build` generated app/common/linux/windows VDFs
  and confirmed the Windows depot stages `vapor.exe`, `vapor-installer.exe`,
  and `libunwind.dll`.
- Simplified Steam launch ownership: `vapor-entrypoint[.exe]` now owns only
  terminal launch and argument forwarding, `bin/vapor-launch.*` owns
  launch-target dispatch and terminal hold behavior, and Vapor Shell no longer
  performs its own no-terminal relaunch.
- Replaced launch wrapper environment IPC with explicit wrapper arguments and
  app-local bootstrap failure state under
  `.vapor/state/installer/bootstrap-failure.txt`.
- Routed installer-launched helper stdout/stderr through
  `<app-root>/.vapor/logs/installer.log` so launch-time player bootstrap no
  longer leaks curl/tar/PowerShell/rustup output into the Steam-visible shell.
- Drafted an unapproved private-test diagnostics rewrite away from
  Git/Vapor-Registry transport:
  `--send-diagnostics` now captures a local structured run under
  `.vapor/diagnostics/runs/YYYY-MM-DD/<unix_timestamp>-<machine_id>-<platform>-<run_id>/`
  with `metadata.toml`, `vapor.log`, and `.vapor/diagnostics/latest.toml`.
- Drafted diagnostics metadata for run identity, generated local machine identity,
  hostname when available, platform, app root, executable, startup/direct/script
  context, installation identity/readiness, selected source, launch target,
  selected packagepack, engine handoff, command/script steps, redacted errors,
  upload request state, and exit state.
- Drafted removal of normal UX for arbitrary `--diagnostics-registry PATH` and Git-backed
  diagnostics push. `diagnostics upload` now uses a future HTTP-server transport
  seam that currently reports not configured; `diagnostics upload --dry-run`
  previews the local run and transport without sending.
- Drafted redaction for obvious password/token/secret/key/auth/ticket/credential/
  cookie arguments and log text, including entrypoint forwarded-argument logs;
  stopped capturing broad diagnostics registry environment values, avoided
  full-command launch-wrapper logging, and disallowed real diagnostics uploads
  from scripts.
- Drafted launch PATH boundary cleanup: Steam launch scripts and the Linux
  entrypoint now expose only app-facing `bin/<target>` and `bin` paths, while
  workflow child commands filter inherited app-local raw tool paths and invoke
  low-level tools by exact app-local paths instead of PATH exposure.
- Verified the diagnostics/PATH slice with app-local Cargo by absolute path:
  `cargo fmt --all --check` in Vapor-Shell and Vapor-Entrypoint,
  `cargo test -p vapor_shell --lib`,
  `cargo test -p vapor_shell --test command`,
  `cargo clippy -p vapor_shell --all-targets -- -D warnings`,
  `cargo test --all-targets` in Vapor-Entrypoint,
  `cargo clippy --all-targets -- -D warnings` in Vapor-Entrypoint, and
  `git diff --check` in Vapor-Root, Vapor-Shell, and Vapor-Entrypoint.
- Captured the diagnostics, registry, server, privacy, GitHub issue strategy,
  launch UX, installer lifecycle, and app-local tooling planning surface as
  unprocessed planning intake under
  `docs/planning-intake/source-corpus/raw-input/vapor-diagnostics/`.
- Captured the intended Vapor server-root topology as unprocessed planning
  intake: `Vapor-Server-Root` beside `Vapor-Root`, with service submodules for
  homepage, docs, identity, and diagnostics, single-domain path routing,
  composed export/import, and future MCP/ACP capability-surface notes.
- Initialized the new Vapor server repository set locally and on GitHub:
  `Vapor-Homepage-Server`, `Vapor-Docs-Server`,
  `Vapor-Identity-Server`, and `Vapor-Diagnostics-Server` each have initial
  README/AGENTS scaffolds, while `Vapor-Server-Root` has the orchestration
  scaffold, route map, example Caddy config, systemd notes, `server-root.toml`,
  and service submodules pointing at the pushed service commits.
- Added and pushed minimal no-dependency Rust service binaries for the four
  server services: homepage serves health/home/license/Impressum placeholders,
  docs serves health/current docs plus token-protected current-doc upload and
  export, identity serves health/status plus token-protected init/export
  scaffolds, and diagnostics serves health plus opt-in upload and
  token-protected list/download/export scaffolds. `Vapor-Server-Root` now
  points its submodules at those service commits.
- Migrated the four server service binaries from hand-rolled standard-library
  HTTP parsing to Axum/Tokio scaffolds, added service `.gitignore` files for
  `target/` and local `state/`, and advanced `Vapor-Server-Root` gitlinks to
  those commits. Updated local RustRover project metadata so the new server
  root and service submodules are visible as Git/Cargo project roots.
- Added `Vapor-Server-Root/docs/decisions-and-backlog.md` to persist the server
  boundaries, current scaffold behavior, near-term backlog, non-goals, and open
  questions. Corrected the local workspace shape so service repositories are
  checked out as root-level `Vapor-Server-Root/Vapor-*-Server` submodules,
  matching `Vapor-Root`, not as sibling top-level checkouts.
- Recorded the current VPS baseline direction without secrets: direct
  Caddy/systemd hosting can use Ubuntu 26.04 LTS, while choosing Plesk would
  require a currently supported Plesk OS such as Ubuntu 24.04 LTS.
- Confirmed key-based root SSH reachability to the new Ubuntu 26.04 VPS without
  persisting secrets, moved `Vapor-Identity-Server` from an ad-hoc registry-file
  scaffold to SQLite/SQLx schema bootstrap, and added first-pass
  `Vapor-Server-Root` deployment automation for no-Plesk Caddy/systemd hosting,
  root-level submodule deployment, server-local env files, SQLite state, systemd
  units, Caddy routing for `vapor.ghf-studios.site`, and local health checks.
- Ran the first real VPS bootstrap/deploy on Ubuntu 26.04: installed Caddy,
  Rust/Cargo/build dependencies, created the `vapor` system user, generated
  server-local env/token files, cloned/built the root-level service submodules,
  installed/restarted systemd units, initialized the SQLite identity database,
  tightened state permissions, and verified Caddy plus all four Vapor services
  running with local health checks. DNS for `vapor.ghf-studios.site` remains
  the external/public-HTTPS blocker.
- Added and deployed domain-independent operations scripts: pre-DNS public HTTP
  fallback routing, SSH/UFW hardening, public HTTP health checks, token-protected
  docs upload smoke testing, and diagnostics upload/redaction smoke testing.
  Verified key-only SSH still works, UFW allows only SSH/HTTP/HTTPS inbound,
  Caddy serves the IP fallback, docs placeholder upload succeeds, and diagnostics
  redaction is applied on disk.

## Owned uncommitted changes

- Vapor-Shell content artifact-root packaging/cache/install/publish
  implementation, live SteamCMD provider bridge, batch publish/download grammar,
  root runtime-vs-stacked depot staging, manifest-declared binary promotion,
  local root/content deploy verbs, content runtime output declarations/copying,
  docs, and tests.
- Vapor core/SDK toolchain metadata updated for stable Rust `1.97.0`.
- Loo-Cast content manifests with real PublishedFileIds and public visibility
  intent, the `spacetime-engine` shipped executable declaration, and placeholder
  first-party Rust payloads.
- Vapor-Examples now includes the extracted terminal runtime proof projects and
  updated README/workspace registration.
- Vapor-Root workstream record update.
- Vapor-Installer submodule registration, root launch wrapper install calls,
  installer-owned setup/Steam/distribution docs, and Shell removal of setup
  command/setup-payload surfaces.
- Manifest-driven split-depot root staging, SteamPipe VDF generation, manifest
  schema docs, root `App-Source.vapor.toml` configured with DepotIDs 2122621,
  2122622, and 2122623, runtime `App.vapor.toml`, and source resources under
  `resources/vapor/` and `resources/steam/`.
- Local-first diagnostics capture/metadata/upload seam, diagnostics command
  grammar/docs/tests, and launch/workflow PATH boundary cleanup. This slice is
  an unapproved draft under planning review, not accepted architecture.
- Raw planning intake capture for diagnostics, registry, server, privacy,
  GitHub issue strategy, launch UX, installer lifecycle, and app-local tooling
  boundaries.
- Raw planning intake capture for Vapor server-root/service repository
  topology and future MCP/ACP capability-surface notes.
- Initial pushed scaffolds for `Vapor-Server-Root`,
  `Vapor-Homepage-Server`, `Vapor-Docs-Server`,
  `Vapor-Identity-Server`, and `Vapor-Diagnostics-Server`.

## Remaining

- Do not publish the extracted topology until the owner explicitly starts that
  authority boundary; the live Workshop items currently still reflect the last
  published terminal-proof-in-Loo-Cast state.
- Run `root publish --skip-build --dry-run` from the real installed app root
  after local deploy, then publish to `vapor-dev` only after reviewing the real
  staged payload. Publication is no longer blocked on missing DepotIDs.
- Implement and test the Windows/MSVC content build/runtime path before friend
  testing: Windows depot binary availability, app-local setup, target-specific
  runtime outputs, and ABI policy. The example dynamic loader now has a Windows
  implementation, but it has not yet been compiled or run under MSVC.
- Steam launch options for the runtime depot use native entrypoints: Linux Play
  executable `bin/x86_64-unknown-linux-gnu/vapor-entrypoint` with argument
  `play`, Linux Shell the same executable with argument `shell`, Linux
  Installer the same executable with argument `installer`, Windows Play
  executable `bin\x86_64-pc-windows-gnullvm\vapor-entrypoint.exe` with
  argument `play`, Windows Shell the same executable with argument `shell`, and
  Windows Installer the same executable with argument `installer`. Play/Shell
  launch targets run quiet `vapor-installer install` first; the Installer
  launch target opens `vapor-installer` directly. Those wrappers expect
  `bin/x86_64-unknown-linux-gnu/vapor` and
  `bin/x86_64-unknown-linux-gnu/vapor-installer`, plus
  `bin\x86_64-pc-windows-gnullvm\vapor.exe` and
  `bin\x86_64-pc-windows-gnullvm\vapor-installer.exe` respectively.
- Run the owner-controlled installer install path from a fresh runtime-only depot
  install, open the Vapor and Loo-Cast sources only for development work,
  locally deploy root and selected first-party content, then iterate on the
  startup path.
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

Keep implementation paused for the diagnostics slice. First review the
persisted planning intake capture, then scope only the local diagnostics capture
MVP before shrinking or replacing the current unapproved draft.
