# Operation — Promote Candidate

## Invoke

```text
Promote <candidate-name>.
```

## Availability

Every registered source is `atomized`, and the named candidate exists uniquely
in atomized planning with mirrored metadata and extraction evidence.

## Behavior

1. Enumerate every other atomized candidate, every active assertion, and every
   superseded assertion.
2. Compare the candidate with every enumerated item using exactly one canonical
   primary disposition per pair.
3. Explain every non-`unrelated` relation and record useful secondary relations.
4. Add every newly discovered assertion to atomized planning without vetting
   it, then include it in the candidate's comparison work.
5. Report higher-order implications when noticed without claiming exhaustive
   higher-order coverage.
6. Write the review record and apply exactly one terminal outcome:
   `promoted`, `rejected`, or `duplicate`.
7. Remove the processed candidate line from atomized planning. A promotion
   moves it into the mirrored active hierarchy, updates its metadata, and
   updates superseded planning without leaving a known-stale active assertion
   behind.

## Pivot conditions

Pause for the user when a conflict, uncertainty, supersession, temporal issue,
or required active-set change depends on owner judgment. Do not choose a policy
merely to force a terminal outcome.

## Boundaries

Do not batch candidates, omit apparently irrelevant comparisons, mutate source
artifacts, or call unresolved work promoted.

## Result

The candidate has one recorded terminal outcome, or the operation is packaged
at an explicit decision/blocker with no false completion.
