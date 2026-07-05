# Operation — Capture Raw Input

## Invoke

```text
Capture: <exact raw input>
```

## Behavior

1. Preserve the exact input as one new source artifact under
   `planning-intake/source-corpus/raw-input/` with useful
   author/date/context provenance.
2. Register the artifact as `unprocessed` under the raw-input source set.
3. Atomize only that artifact using the canonical source operation.
4. Preserve wording, modality, uncertainty, temporality, questions, and owner
   direction without answering or reconciling them.
5. After complete atomization, retain the unchanged source under
   `planning-evidence/source-artifacts/raw-input/` with its coverage record.

## Boundaries

Do not compare new candidates with the corpus, answer embedded questions,
resolve conflicts, inspect implementation to validate content, or promote
anything.

## Pivot conditions

Pause when the user has not supplied the material to preserve, the requested
provenance is ambiguous, or exact preservation would expose unintended content.

## Result

The input is preserved, registered, and represented by entirely unvetted
atomized candidates or explicit extraction blockers.
