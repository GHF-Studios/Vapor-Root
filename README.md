# Vapor-Root

Vapor-Root is the first-party application source root for Vapor. It assembles
the Steam-installed Vapor app, SDK, launcher, shell, tools, docs, and app/depot
payload. It is source, not the Steam installation itself.

Most day-to-day work happens through the installed Vapor Shell. Bootstrap
installs may start from `bin/vapor`; release app payloads use
`bin/<target>/vapor[.exe]` through `.vapor/launch/<platform>/` wrappers. Source
repositories stay outside the Steam app root and are opened explicitly:

```text
source open /path/to/Vapor-Root
metadata
validate
root package
root publish --dry-run
```

Quick local commands are host-only by default. Release app/depot and content
workflows use `--release-targets` to consume the target matrix declared in
`Vapor.toml`, such as Linux plus Windows/MSVC.

Real app/depot publication still requires manual interactive confirmation:

```text
root publish --account ACCOUNT --yes
```

## Product Topology

- **Steam installation/app root**: the installed app directory that owns
  `bin/<target>/vapor[.exe]`, optional bootstrap `bin/vapor`, app-local
  Rust/Cargo, Git, SteamCMD, caches, generated output, installed content,
  indexes, locks, and receipts.
- **Vapor-Root**: this source-stage application root. It packages and publishes
  the complete Steam app/depot.
- **Vapor-Registry**: separate registry authority infrastructure. It is not a
  Cargo workspace and not normal content.
- **Loo-Cast**: first-party custom content workspace. It is normal Workshop
  content, not app/depot source.

The current Shell command model is documented in
[`Vapor-Shell/crates/vapor_shell/docs/commands.md`](Vapor-Shell/crates/vapor_shell/docs/commands.md).
The product model behind it is in
[`product-topology.md`](Vapor-Shell/crates/vapor_shell/docs/design/product-topology.md).

## Repository Checkouts

Initialize pinned project checkouts with:

```bash
git submodule update --init --recursive
```

Key checkouts:

- [`Vapor-Shell`](Vapor-Shell/) — installed shell and command model.
- [`Vapor-SDK`](Vapor-SDK/) — SDK surface.
- [`Vapor-Launcher`](Vapor-Launcher/) — launcher surface.
- [`Vapor`](Vapor/) — core application workspace.
- [`Vapor-Examples`](Vapor-Examples/) — examples when present.

## Custom Content

Workshop-backed content uses `content *` commands from an opened content
workspace. Loo-Cast currently proves the first-party path with:

```text
source open /path/to/Loo-Cast
content validate
script run content-roundtrip
script run content-publish-preview
```

The local roundtrip packages, caches, installs, verifies, updates, repairs, and
uninstalls content without changing Steam authority. Publication dry-runs write
provider scripts but perform no upload. Real Workshop create, publish, and
delete operations remain manual authority-changing actions.

See
[`Vapor-Shell/crates/vapor_shell/docs/content.md`](Vapor-Shell/crates/vapor_shell/docs/content.md)
for layout, receipts, fingerprints, and repair behavior.

## Setup And Recovery

Installed setup is explicit:

```text
setup self status
setup self install
setup self repair
setup self package install
```

Setup manages the app-local command environment inside the Steam installation.
It does not silently repair prerequisites during builds, publishes, or content
operations.

## Planning

Planning files are review infrastructure, not a substitute for user docs:

- [Protocol](docs/planning/README.md) — canonical planning lifecycle.
- [Intake](docs/planning-intake/README.md) — retained original sources.
- [Atomized](docs/planning-atomized/README.md) — unvetted atomic candidates.
- [Active](docs/planning-active/README.md) — current vetted assertions.
- [Superseded](docs/planning-superseded/README.md) — historical assertions that
  remain mandatory review material.
- [Evidence](docs/planning-evidence/README.md) — extraction and review records.
