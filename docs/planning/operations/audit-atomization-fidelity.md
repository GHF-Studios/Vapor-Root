# Operation — Audit Extraction

## Invoke

```text
Audit extraction <registered-source-path>.
```

## Availability

The source already has atomized candidates or an extraction-coverage record.

## Behavior

1. Read the source, coverage record, and every candidate attributed to it.
2. Map every substantive passage to candidates or an explicit classification:
   non-assertive structure, within-source duplicate, source metadata, or
   unresolved extraction ambiguity.
3. Find omissions, combined non-atomic units, invented meaning, semantic drift,
   unnecessary paraphrase, missing modality or temporal framing, bad source
   locations, duplicate names, and incomplete coverage.
4. Repair only the extraction, candidate metadata, register state, and coverage.
5. Retain the source unchanged, moving it from intake to
   `planning-evidence/source-artifacts/<source-set>/` only when complete.
6. Mark it `atomized` only when extraction is complete, faithful, and stored as
   evidence.

## Boundaries

Do not determine whether claims are true, compare against the wider corpus,
resolve contradictions, inspect implementation, or promote candidates.

## Pivot conditions

Pause when fidelity cannot be repaired without choosing between materially
different interpretations of the source.

## Result

The extraction is either certified `atomized` or left with explicit unresolved
coverage/fidelity blockers.
