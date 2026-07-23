# Vapor diagnostics, registry, and infrastructure planning capture

- Source date: 2026-07-22
- Source type: raw owner/assistant planning capture
- Context: continuation after an oversized diagnostics implementation draft
  exposed too many hidden product, privacy, CLI, registry, server, and tooling
  decisions at once.
- Planning status: intake only; unprocessed; not active doctrine.
- Implementation status: the current diagnostics changeset is an unapproved
  draft and must not be treated as accepted architecture.

## Purpose

Persist the planning surface opened by the diagnostics/registry/server
discussion without trying to solve every topic in one session.

The immediate working rule is:

```text
One topic active.
Everything else captured, not solved.
No further implementation until the active topic is explicitly scoped.
```

The current active topic is:

```text
Local diagnostics capture MVP.
```

All other topics remain visible in this artifact as unresolved planning work,
not as deferred or forgotten work.

## Current draft assessment

The existing diagnostics draft made some directionally correct moves:

- removed Git-backed diagnostics as normal player infrastructure;
- removed/demoted arbitrary `--diagnostics-registry PATH` from normal UX;
- preserved local diagnostics capture;
- added redaction for obvious secrets;
- moved conceptually toward future server upload instead of Git transport;
- cleaned some obvious app-local raw-tool PATH exposure.

However, the draft is too decision-dense and should not be approved as-is. It
bundled local capture, metadata schema design, privacy decisions, upload
semantics, CLI behavior, registry direction, server architecture assumptions,
and tooling/PATH policy into one changeset.

The most important correction is process-level: split the decision surface
before continuing implementation.

## Hard constraints already established

These constraints are carried forward from the owner direction that led to this
planning capture:

- Diagnostics upload must be explicit opt-in only.
- There must be no public/default telemetry.
- Diagnostics must not harvest secrets.
- Obvious tokens, passwords, credentials, auth tickets, cookies, and similar
  values must be redacted from args, env-like text, and logs where practical.
- Local logs should stay small and text-based.
- Git-backed Vapor-Registry transport is legacy/prototype context, not the
  default product direction.
- Normal players must not require Git.
- Normal diagnostics upload must not be implemented as an arbitrary local
  registry checkout path.
- Future diagnostics upload should be designed around a server endpoint, not
  Git push/copy.
- Vapor-Installer owns install, uninstall, repair, and dev-env lifecycle.
- Vapor Shell setup/self-setup shims must not be resurrected.
- Runtime bootstrap must not depend on installscript.vdf unless it works for
  both Windows and Linux/SteamOS.
- Launch/bootstrap state must not be passed through ambient environment
  variables.
- App-local tools may exist on disk, but normal users and normal developer
  workflows should call Vapor-facing commands rather than raw Cargo, Git,
  SteamCMD, rustup, Zig, LLVM-mingw, or cross binaries directly.

## Active topic: local diagnostics capture MVP

The next planning section should answer only this question:

```text
What is the smallest local diagnostics run Vapor should be comfortable
creating now?
```

Candidate MVP shape:

```text
.vapor/diagnostics/
  runs/
    YYYY-MM-DD/
      <timestamp>-<run-id>/
        metadata.toml
        vapor.log
  latest.toml
```

Candidate MVP metadata:

- schema version;
- run id;
- timestamp;
- platform and architecture;
- app root;
- Vapor executable path;
- launch/startup mode when known;
- redacted args;
- exit status or error summary.

Candidate MVP behavior:

- local capture is explicit or narrowly scoped;
- successful capture reports `diagnostics: captured run <id> at <path>`;
- diagnostics capture failure does not break normal launch unless the user
  explicitly requested a diagnostic-only command;
- upload behavior is not implemented as a product feature yet;
- Git is not involved.

Candidate exclusions for the MVP:

- hostname;
- persistent generated machine identity;
- Steam identity;
- GitHub identity;
- source/packagepack/content inventory;
- installer/player/dev readiness inventory;
- future server upload client;
- `diagnostics upload` command semantics;
- broad environment capture;
- large automatic log bundles.

Open MVP decisions:

- Whether `latest.toml` replaces `latest.txt` immediately or compatibility is
  required.
- Whether date folders use UTC or local time.
- Whether absolute paths are acceptable in local-only metadata.
- Whether `--send-diagnostics` should exist before upload exists.
- Whether local capture should be controlled by `--capture-diagnostics`,
  `--diagnostics`, `--send-diagnostics`, command-specific flags, or only
  internal developer commands for now.
- What max size applies to `vapor.log`.
- What retention/cleanup policy applies to local runs.

## Parking lot: diagnostics privacy policy

This topic must be decided before the metadata schema grows beyond the MVP.

Fields needing explicit approval before collection:

- hostname;
- persistent machine id or generated local identity;
- absolute app root paths;
- executable paths;
- SteamID64 or Steam auth/session ticket references;
- GitHub account identity;
- selected source path;
- packagepack/content state;
- installer readiness state;
- command/script step traces;
- environment variables;
- installer, entrypoint, and wrapper logs.

Policy questions:

- What is collected by default?
- What is collected only after explicit upload consent?
- What is never collected?
- What is redacted?
- What is size-limited?
- What is previewed to the user before upload?
- What is retained locally and for how long?

## Parking lot: diagnostics upload transport

Future direction:

```text
local capture -> explicit upload client -> Vapor server endpoint
```

Rejected normal-player direction:

```text
local capture -> Git checkout -> commit -> push
```

Open questions:

- Is anonymous upload allowed, or must upload require verified identity?
- If identity is required, is Steam auth sufficient for players?
- What is the upload bundle format?
- Does upload send raw files, compressed bundles, structured TOML/JSON, or a
  hybrid?
- How does the user preview what will be uploaded?
- How are retries handled?
- What happens offline?
- What server response should Vapor show?
- What local failure message is used when upload fails?

Target future UX:

```text
diagnostics: captured run <id> at <path>
diagnostics: sent run <id>
diagnostics: upload failed: <reason>
diagnostics: local run is at <path>
```

## Parking lot: Vapor server / VPS runway

The owner has already rented a small VPS and wants to use it eventually. The
server should remain visible, but it should not pull unresolved client
diagnostics, registry, identity, and publishing decisions into premature
infrastructure.

Possible eventual server responsibilities:

- static docs hosting;
- health/status endpoint;
- diagnostics upload endpoint;
- identity registry;
- lightweight product/license page;
- developer pipeline/tool relay APIs;
- future protected publishing pipeline.

Likely implementation stack:

- cheap Linux VPS;
- Rust backend using Axum or Actix;
- PostgreSQL or MySQL behind firewall;
- Caddy or Nginx for HTTPS;
- Vapor-specific JWTs after identity validation.

Safe first milestone later:

- static docs deployed;
- health endpoint;
- version/status page;
- no privileged secrets;
- no player data;
- no diagnostics ingestion;
- no client dependency on the server.

Explicit non-goals for the current diagnostics MVP:

- no Steam account linking implementation;
- no GitHub Device Flow implementation;
- no production diagnostics upload;
- no publishing pipeline;
- no player dependency on a server.

## Parking lot: identity and roles

Future Steam identity idea:

- client gets Steam auth/session ticket through Steamworks SDK or steamworks-rs;
- server validates ticket with Steam WebAPI;
- server stores verified SteamID64.

Future GitHub identity idea:

- developer CLI uses GitHub Device Flow;
- server links verified GitHub account to a Vapor profile.

Potential roles:

- root;
- content-developer;
- future publisher;
- future toolchain/pipeline roles.

Open questions:

- Are Steam and GitHub identities linked into one Vapor profile?
- Who may upload diagnostics?
- Who may view diagnostics?
- Who may publish content?
- Who may trigger protected server-side pipeline actions?
- What is local-only authority versus server-mediated authority?

## Parking lot: registry direction

The Git-backed Vapor-Registry remains useful historical/prototype context but
is not the default product infrastructure for normal players.

Separate these concepts:

- local installed manifests;
- Steam-delivered app/package/content state;
- developer source/provider links;
- legacy Git-backed registry prototype;
- future Vapor server registry.

Rules to preserve:

- Git is not player infrastructure.
- Git can remain a developer provider/linking concern.
- Git-backed registry transport can exist only as an explicit dev/prototype
  mode if intentionally re-scoped.
- Diagnostics upload must not require the user to understand or repair Git.

## Parking lot: GitHub issues and multi-repo tracking

GitHub issues may be useful after the planning map exists, but they should not
become the only architecture memory.

Suggested ownership model:

```text
Vapor-Root:
  canonical cross-repo planning, product direction, architecture decisions

Vapor-Shell:
  CLI implementation and diagnostics command/capture behavior

Vapor-Installer:
  install, uninstall, repair, provider setup, dev-env lifecycle

Vapor-Entrypoint:
  native Steam launch behavior and argument forwarding

Vapor-Registry:
  legacy/prototype registry context unless explicitly revived
```

Suggested flow:

1. Persist canonical planning in Vapor-Root.
2. Split implementation tasks by repo.
3. Create GitHub issues only after the owning repo and decision dependency are
   clear.
4. Link implementation issues back to Vapor-Root planning.

## Parking lot: app-local tooling and PATH boundary

Hard rule:

```text
Normal users and normal developer workflows call Vapor.
Vapor internally resolves app-local tools.
PATH exposes only Vapor-facing app binaries.
```

Raw tool directories should not be normalized into user/developer PATH:

- rustup toolchain `bin`;
- cargo-home `bin`;
- tools/steamcmd;
- tools/git/bin;
- tools/zig;
- tools/llvm-mingw;
- tools/cross/bin.

Open questions:

- How does Vapor invoke Cargo without exposing raw toolchain directories?
- Does Vapor need to set `RUSTC` or related environment variables internally?
- Does Cargo require sibling Rust tools on PATH for clippy/rustdoc/build-script
  behavior?
- How are Git, SteamCMD, Zig, LLVM-mingw, and cross resolved internally?
- What exact emergency-debug exceptions are allowed?
- What tests prove Windows target publishing and runtime checks still work?

This topic should be handled separately from diagnostics capture.

## Parking lot: installer lifecycle and readiness reporting

Vapor-Installer owns install, uninstall, repair, and dev-env lifecycle.
Diagnostics may report installer state, but must not become installer logic.

Open questions:

- What installer readiness summary is safe to include in local diagnostics?
- Should installer readiness appear only in explicit diagnostic commands?
- Which installer logs are included or referenced?
- How is bootstrap failure state referenced?
- How do diagnostics point users toward repair without reintroducing Shell setup
  shims?

Known paths:

```text
<app-root>/.vapor/state/installer/bootstrap-failure.txt
<app-root>/.vapor/logs/installer.log
<app-root>/.vapor/logs/launch-wrapper.log
<app-root>/.vapor/logs/entrypoint.log
```

## Parking lot: Steam launch UX

Current intended launch model:

- Steam launch options use native entrypoints, not direct shell scripts.
- Linux Play/Shell/Installer launch
  `bin/x86_64-unknown-linux-gnu/vapor-entrypoint` with `play`, `shell`, or
  `installer`.
- Windows Play/Shell/Installer launch
  `bin\x86_64-pc-windows-gnullvm\vapor-entrypoint.exe` with `play`, `shell`,
  or `installer`.
- Entrypoint opens Konsole or cmd, invokes `bin/vapor-launch.*` with internal
  hold behavior, then forwards Steam args.
- Launch scripts derive app root from their own location.
- Launch/bootstrap state must not be passed through ambient environment
  variables.

Open diagnostics launch UX questions:

- Should Steam expose `play --send-diagnostics` and
  `shell --send-diagnostics` eventually?
- Should `--send-diagnostics` mean local capture, upload, or both?
- What exact success/failure messages appear in Steam-launched terminals?
- How do diagnostics failures avoid confusing players with Git or server
  internals?

## Parking lot: current draft cleanup

Before new implementation, classify the current diagnostics-related changes by:

- keep;
- revise;
- remove;
- requires owner decision.

Likely keep:

- removal of Git-backed normal diagnostics transport;
- local capture concept;
- basic redaction tests;
- documentation that Git-backed registry is not the normal product transport.

Likely revise:

- diagnostics directory shape and latest pointer;
- metadata schema;
- `--send-diagnostics` behavior;
- diagnostics status/upload command grammar;
- PATH cleanup verification.

Likely remove or postpone:

- hostname capture;
- persistent generated machine id;
- broad readiness/source/content metadata;
- future HTTP upload trait machinery without a server API;
- upload aliases and non-dry-run command behavior.

Requires owner decision:

- exact local metadata fields;
- identity/privacy policy;
- upload consent UX;
- server identity model;
- multi-repo issue ownership;
- PATH/toolchain invocation model.

## Immediate next planning step

Write or review a focused local diagnostics capture MVP section. Do not solve
server, identity, registry, GitHub issue strategy, installer readiness, or PATH
policy during that section except where a direct boundary is required.
