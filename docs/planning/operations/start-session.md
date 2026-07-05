# Operation — Start Session

## Invoke

```text
Read AGENTS.md.
```

When the conversation starts outside the repository:

```text
Work from /path/to/project and read AGENTS.md.
```

## Behavior

The agent reads the bootstrap files, inspects current state without editing,
and renders the smallest useful interactive screen. Direct task instructions
skip the menu. Natural language remains valid at every screen.

## Result

The session is either waiting at a menu/decision screen or working on the exact
operation named by the user.
