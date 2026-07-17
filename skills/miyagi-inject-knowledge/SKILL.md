---
name: miyagi-inject-knowledge
description: Adapt a true-binary GGUF model toward a user-supplied, verified knowledge ledger using target, control, and held-out probes. Use when injecting facts, terminology, or short factual associations into a model with Miyagi row-XOR search, while preserving source provenance and avoiding invented answers.
---

# Inject Verified Knowledge

Read [knowledge-ledger.md](references/knowledge-ledger.md) before converting
facts into probes. Use `$miyagi-author-probes`, `$miyagi-search-patch`, and
`$miyagi-test-generalization` for the lower-level steps.

## Require a Source of Truth

1. Require a user-supplied fact ledger, approved document, or other authoritative
   source. Do not invent facts, wrong alternatives, or provenance.
2. Require a correct answer and an explicit contrasting answer for each target
   probe. If the source only provides the correct answer, ask for the wrong
   answer or obtain explicit approval for how to construct one.
3. Record source identifiers and fact versions in the probe or patch metadata.

## Build the Experiment

1. Create training targets from a subset of facts and several paraphrases.
2. Hold out different paraphrases and facts for evaluation.
3. Create controls for neighboring facts, unrelated knowledge, general
   capability, and any behavior the user must preserve.
4. Validate baseline gaps before searching. A target that is already correct is
   not evidence that injection is needed.

## Search and Evaluate

Run a bounded search with explicit layers, projections, seed, controls, and
checkpoint. Then evaluate the emitted patch on training targets, held-out facts,
controls, and generation prompts. Use dataset benchmarking when the source
contains enough examples to support it.

## Report

Report source coverage, target and held-out splits, target improvement, control
degradation, exact patch artifact, architecture signature, and restoration
status. Say "behavior changed on these probes" rather than claiming the model
now knows the facts universally. Miyagi changes in-memory model behavior and
does not rewrite the GGUF file.
