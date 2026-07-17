---
name: miyagi-select-patch
description: Compare multiple Miyagi row-XOR patch artifacts against the same model, target probes, preservation controls, held-out probes, and optional generation datasets, then choose the smallest or safest candidate that passes predeclared gates. Use when several search results or composed patches compete.
---

# Select a Patch

Read [selection-rubric.md](references/selection-rubric.md). Treat candidate
selection as a controlled comparison: hold model, probes, token mode, threshold,
generation settings, and reports constant.

## Preflight

1. List candidates and record their names, sources, flip counts, metadata, and
   architecture signatures.
2. Use `miyagi info --patch` and `--model` to reject malformed, incompatible, or
   duplicate-coordinate artifacts before behavioral evaluation.
3. Define target thresholds, preservation gates, held-out tests, and tie-breakers
   before inspecting final scores.

## Evaluate Every Candidate

For each candidate, run the same target/control/held-out probe evaluations and,
when relevant, the same dataset benchmark and deterministic generation prompts.
Record per-probe deltas, sign transitions, category summaries, raw generations,
patch size, and restoration status.

Do not compare one candidate's training score with another candidate's held-out
score. Do not let a candidate skip a failing preservation gate because its target
average is higher.

## Choose

1. Reject candidates that fail structural validation or hard preservation gates.
2. Among survivors, prefer the candidate meeting the target threshold with the
   best held-out and preservation evidence.
3. Use flip count, logical bits, and generation stability as tie-breakers.
4. Preserve the full comparison table and the reason for selection.

## Report

State selected artifact, rejected candidates and reasons, target/control/held-out
results, patch size, architecture signature, generation evidence, and remaining
uncertainty. The skill selects an artifact; it does not merge or persist model
changes.
