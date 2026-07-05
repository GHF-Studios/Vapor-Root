# Workstream — Agent Workstreams

- State: closed
- Goal: support persistence-heavy concurrent conversational workstreams with
  exclusive mutation claims, safe switching, queues, and resumable handoffs.
- Ordered successor: `vapor-shell-exploration`, activated.
- Semantic mutation claim: Vapor-Root agent interaction and workstream policy.
- Path claims: `AGENTS.md`, `.agents/workstreams/README.md`,
  `.agents/workstreams/open/agent-workstreams.md`, and
  `.agents/workstreams/closed/README.md`.

## Completed

- Selected a hybrid model heavily biased toward repository persistence.
- Defined persistence triggers, exclusive claims, switching, closure, queue,
  exit, and shared-commit behavior.
- Created the persistent workstream registry and initial records.
- Verified exactly one foreground workstream, complete registry coverage,
  non-overlapping path claims, valid navigation links, and clean Markdown
  whitespace.

## Commit disposition

The Agent bootstrap and persistent workstream registry are included in the
shared Agent-upgrade commit.

## Remaining

None.

## Smallest resume action

No resume action. Reopen only by explicit user direction.
