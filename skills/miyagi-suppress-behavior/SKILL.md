---
name: miyagi-suppress-behavior
description: Reduce a user-identified unwanted completion or response tendency with Miyagi row-XOR adaptation while measuring an explicit replacement or neutral behavior and preservation controls. Use for experimental behavior suppression, refusal-style changes, phrase reduction, or unwanted associations; do not use it to make unsupported safety claims.
---

# Suppress an Unwanted Behavior

Read [suppression-evidence.md](references/suppression-evidence.md). Define what
should replace the unwanted behavior before searching; suppression without a
replacement can merely move the behavior elsewhere.

## Define the Target

1. Provide observed prompts, unwanted continuations, desired alternatives, and
   source or policy evidence.
2. For local token probes, set `correct` to the desired alternative and `wrong`
   to the unwanted token/string.
3. For sequence behavior, add deterministic generation prompts and an explicit
   scoring rule. Do not infer full-response suppression from one logit gap.
4. Add paraphrases, benign contexts, adjacent behaviors, and general capability
   controls. Reserve some for held-out evaluation.

## Search and Test

Run a bounded search with strong controls and a checkpoint. Evaluate target,
replacement, preservation, and held-out sets. Compare baseline and patched
generations using the same seed and token limits. Use `$miyagi-preserve-capabilities`
for hard gates and `$miyagi-test-generalization` for context variation.

## Interpret Carefully

Classify results as reduced selected-token preference, changed deterministic
generation, or robust behavior across held-out prompts. Check for refusal,
evasion, topic shifting, or unrelated degradation. A patch that avoids one
phrase is not automatically safer or more aligned.

## Report

Include the desired replacement, unwanted behavior definition, target/control
split, raw generation examples, probe deltas, preservation gates, patch metadata,
and evidence limitations. Do not make broad safety claims from a small prompt
set.
