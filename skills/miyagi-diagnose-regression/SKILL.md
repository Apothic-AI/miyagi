---
name: miyagi-diagnose-regression
description: Diagnose why a Miyagi patch or patch composition improves a target while degrading controls, held-out probes, or generations. Use for patch review, failed preservation gates, unexpected capability changes, or comparing candidate artifacts.
---

# Diagnose Patch Regression

Read [regression-report.md](references/regression-report.md). Start from saved
baseline and patched reports when available; rerun measurements only when the
model, seed, probes, and configuration are known.

## Establish the Regression

1. Parse and live-validate the candidate with `miyagi info`.
2. Evaluate identical target, control, and held-out probe files with identical
   token mode and change threshold.
3. Compare deterministic generations with identical seed and token limits.
4. Identify the first failing gate and the largest negative per-probe delta.

## Localize

Group failures by category, sign transition, prompt family, and generation
pattern. Compare candidate metadata, flip counts, architecture signature, and
composition history. If candidate patches are composed, evaluate each
constituent and the composition separately.

Do not claim a particular row caused a regression without an ablation artifact
or an equivalent controlled experiment. Use coordinate-level information to
describe overlap and scope, not unsupported causality.

## Resolve

Choose among removing a constituent, strengthening controls, lowering search
scope, changing fitness/penalty, re-searching with held-out gates, or rejecting
the patch. Re-run the same acceptance matrix after any change.

## Report

Include reproducible commands, before/after values, worst regressions, affected
categories, candidate metadata, likely hypotheses, evidence level, and the
decision. Separate observed facts from proposed causes.
