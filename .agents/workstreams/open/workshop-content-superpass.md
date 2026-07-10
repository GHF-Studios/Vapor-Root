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

## Owned uncommitted changes

- Vapor-Root workstream registry/record updates, README, and AGENTS.
- Vapor-Shell content lifecycle implementation, source/content grammar updates,
  docs, bootstrap hint, and tests.
- Loo-Cast content metadata, README, and `.vapor/scripts/` proof scripts.

## Remaining

- Perform a real private Steam Workshop roundtrip after credentials/provider
  access are available: create/resolve PublishedFileIds, publish/update through
  Steam, subscribe/download through a live SteamUGC provider, install, verify,
  update, uninstall, repair, and delete/retire if appropriate.
- Replace dry-run-only create/delete provider stubs with a controlled SteamUGC
  provider when the app links or brokers the required API.
- Record real PublishedFileIds in Loo-Cast `Vapor.toml` after private items
  exist.

## Smallest resume action

Open Loo-Cast in a SteamUGC-enabled Vapor session and create or resolve private
Workshop items for the three artifacts, then record their PublishedFileIds.
