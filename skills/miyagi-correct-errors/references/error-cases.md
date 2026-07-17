# Error Case Format

Keep observed errors, desired corrections, and source evidence explicit:

```json
[
  {
    "id": "capital-australia",
    "prompt": "The capital of Australia is",
    "observed_wrong": " Sydney",
    "desired_correct": " Canberra",
    "source": "user-approved geography ledger, entry 12",
    "category": "geography",
    "neighboring_cases": ["capital-new-zealand"]
  }
]
```

## Baseline classification

Record both:

- the selected-token baseline gap for `desired_correct` versus
  `observed_wrong`; and
- a deterministic generation using the same seed/configuration used later.

These can disagree. A model can have a positive selected-token gap while
generating a different continuation because decoding considers a sequence.

## Correction target

Convert each case into a Miyagi probe with:

```text
prompt = case.prompt
correct = case.desired_correct
wrong = case.observed_wrong
```

Preserve leading whitespace and use strict token mode when the strings are
intended to be single-token alternatives. Do not silently replace a multi-token
answer with its final token.

## Evidence boundary

A corrected probe demonstrates a changed local comparison. Generation checks and
held-out neighboring cases are required before describing the correction as
useful model behavior.
