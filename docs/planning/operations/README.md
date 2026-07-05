# Planning Operations

These are transparent user-facing operations, not prompts that must be pasted
into a conversation. The user invokes an operation through the session menu or
ordinary language; the agent reads and follows the corresponding contract.

The canonical [planning protocol](../README.md) and [agent bootstrap](../../../AGENTS.md)
govern every operation. If an operation conflicts with either, processing stops
at a `PIVOT` decision instead of silently choosing a direction.

## Available operations

- [`start`](start-session.md) — bootstrap or reopen the interactive session.
- [`register sources`](register-source-artifacts.md) — preserve new source
  documents and list them in the corpus register.
- [`atomize <source>`](atomize-source-artifact.md) — extract one source into
  unvetted atomic planning candidates.
- [`audit extraction <source>`](audit-atomization-fidelity.md) — repair one
  existing extraction without vetting its claims.
- [`capture <raw input>`](capture-new-raw-input.md) — preserve and atomize new
  owner input without vetting it.
- [`promote <candidate>`](promote-candidate.md) — exhaustively review one
  candidate for active planning after the promotion gate opens.
- [`audit framework`](audit-framework.md) — inspect operational coherence
  without vetting knowledge content.

Natural-language equivalents are valid. Operation names are navigation aids,
not a command parser or mandatory syntax.

## Interaction contract

- Starting an operation while idle opens a workstream; it is not a pivot.
- A clear direct request bypasses unnecessary menus.
- Switching away from active incomplete work packages that branch first.
- Material ambiguity or protocol conflict opens a timely `PIVOT` decision.
- `menu`, `back`, `status`, `park`, `pivot`, and `exit` remain available.
