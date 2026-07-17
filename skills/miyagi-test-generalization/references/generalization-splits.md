# Generalization Splits

## Probe families

Build evaluation families deliberately:

| Family | Example change | What it tests |
| --- | --- | --- |
| exact | original prompt | training fit |
| lexical | synonyms or reordered wording | prompt variation |
| structural | question form or sentence frame | compositional transfer |
| entity | related subject or value | local generalization |
| context | added neutral context | context robustness |
| generation | full completion prompt | sequence behavior |
| unrelated | different capability | collateral movement |

## Leakage checks

- Keep held-out files out of `--target` and `--control` during search when they
  are intended as unbiased evaluation.
- Do not reuse target prompts with only punctuation changes and call them
  independent.
- Record the split manifest and any transformations.
- Keep answer tokens and categories visible in reports so accidental overlap is
  auditable.

## Scoring

Report per-family counts and deltas, not only one overall average. For datasets,
inspect raw responses and extraction failures. For probe gaps, report both
continuous delta and sign transition. Miyagi does not produce a built-in
generalization score, so derive summaries from saved reports and state the
formula used.
