---
name: miyagi-correct-errors
description: Repair user-identified factual or completion errors in a true-binary GGUF model by turning verified wrong-versus-correct cases into Miyagi targets and testing nearby facts for collateral damage. Use when a model repeatedly answers a known question incorrectly or emits a specific unwanted completion.
---

# Correct Known Errors

Read [error-cases.md](references/error-cases.md). Use `$miyagi-author-probes`
for case and control files, `$miyagi-search-patch` for adaptation, and
`$miyagi-preserve-capabilities` when regressions matter.

## Establish the Error

1. Require evidence for the desired answer and record its source.
2. Run a baseline probe evaluation and, when the issue is generative, a
   deterministic baseline generation.
3. Distinguish a wrong generated answer from a negative selected-token gap. A
   logit probe is a diagnostic, not a complete generation judgment.
4. Do not search against a case that is already correct without documenting why
   a different behavior is required.

## Construct Targets and Controls

1. Make each known error a target with `correct` equal to the verified answer
   and `wrong` equal to the observed answer or unwanted completion.
2. Add neighboring facts that should remain correct, especially entities that
   differ by one attribute or token.
3. Add unrelated capability and safety controls. Reserve paraphrases and
   related cases for held-out evaluation.

## Adapt and Verify

Run a bounded, checkpointed search with explicit controls. Evaluate baseline and
patched target/control/held-out gaps, then compare deterministic generations.
Reject a patch that fixes the listed error by breaking adjacent facts unless the
user explicitly accepts that tradeoff.

## Report

Include the source for every correction, baseline evidence, patched evidence,
collateral changes, held-out results, patch coordinates, and restoration status.
Do not claim the model's entire knowledge state was corrected from a small case
list.
