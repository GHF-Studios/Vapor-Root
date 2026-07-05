# Operation — Atomize Source

## Invoke

```text
Atomize <registered-source-path>.
```

## Availability

The source exists in the corpus register and is not already `atomized`.

## Behavior

1. Read the entire source and its current register state.
2. Extract every independently assessable information unit, including units
   joined inside sentences, bullets, tables, warnings, captions, and examples.
3. Preserve source wording verbatim when already atomic. When splitting is
   necessary, add only enough grammar for each candidate to stand alone.
4. Give each candidate a globally unique descriptive name and record its kind,
   source location, temporal scope, and coverage link.
5. Place candidates under `planning-atomized/<topic>/` without comparing or
   reconciling them.
6. Account for every source passage in one extraction-coverage record.
7. When extraction and fidelity are complete, move the unchanged source to
   `planning-evidence/source-artifacts/<source-set>/`.
8. Mark the source `atomized` only after that evidence move; otherwise leave
   its incomplete state explicit in intake.

## Boundaries

Do not check truth, freshness, consistency, implementation, or wider-corpus
duplication. Do not resolve questions or promote/reject candidates.

## Pivot conditions

Pause when source wording cannot be split without material interpretation,
candidate naming requires an owner distinction, or the source/register state
disagrees.

## Result

One source has complete or explicitly incomplete extraction coverage, and every
extracted candidate remains entirely unvetted in atomized planning.
