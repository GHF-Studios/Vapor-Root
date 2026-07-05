# Workstream — Planning Assertion Representation

- State: complete-awaiting-close
- Goal: separate assertion metadata from compact one-line assertion lists while
  preserving stable names, provenance, coverage, hierarchy, and lifecycle
  semantics.
- Ordered successor: corpus atomization under the unchanged global gate.
- Depends on: none.
- Semantic mutation claim: planning assertion storage representation only; no
  deduplication, validation, rejection, or promotion.
- Path claims: `docs/planning/README.md`, `docs/planning/operations/**`, the two
  migrated compact assertion files under `docs/planning-atomized/`,
  `docs/planning-active/**`,
  `docs/planning-superseded/**`, `docs/planning-evidence/README.md`,
  the two migrated metadata files under
  `docs/planning-evidence/assertion-metadata/`,
  the two source-workstream records involved in this representation handoff,
  and this record. Shared indexes and new-source representation ownership were
  transferred to `source-atomization-raw-info-dump`.

## Completed

- Selected mirrored source-set/topic hierarchy.
- Selected compact linked-line assertions with separate metadata evidence.
- Preserved the global promotion gate and deferred all redundancy decisions.
- Migrated all 540 existing assertions to one linked line each.
- Created 540 mirrored metadata entries containing ID, stage, kind, source,
  temporal scope, extraction coverage, and assertion-list link.
- Updated the canonical protocol and all affected operation contracts.
- Verified one-to-one identity, metadata, link, anchor, and coverage mapping.
- Verified that planning-atomized contains no verbose metadata fields and that
  all operational links resolve.

## Owned uncommitted changes

- Representation protocol and current assertion migration.

## Remaining

- Include this completed representation migration in the next authorized
  commit, then close the workstream.

## Smallest resume action

Re-run the one-to-one representation audit before the next authorized commit.
