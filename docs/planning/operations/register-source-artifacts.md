# Operation — Register Sources

## Invoke

```text
Register sources <paths> as <source-set>.
```

## Inputs

- one or more exact source paths;
- destination source-set name;
- provenance or selection context the user wants retained.

## Behavior

1. Read the canonical protocol and corpus register.
2. Preserve each selected document in full under
   `planning-intake/source-corpus/<source-set>/`.
3. Preserve filenames and source-set boundaries. If paths collide, pause for a
   naming decision rather than silently overwriting or renaming.
4. Add every preserved artifact to the matching `unprocessed` register group.
5. Do not interpret or atomize the imported content.

## Boundaries

Do not infer assertions, validate content, modify source wording, or promote
anything. Registration records availability, not truth or completeness.

## Pivot conditions

Pause when selection scope is unclear, a destination would overwrite content,
or preserving the source requires a materially different layout.

## Result

The source documents are retained unchanged and visibly listed as
`unprocessed`.
