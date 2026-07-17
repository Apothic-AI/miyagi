---
name: miyagi-specialize-domain
description: Adapt a true-binary GGUF model for a user-defined domain such as mathematics, programming, terminology, or product knowledge using domain targets, adjacent-domain controls, general capabilities, held-out probes, and optional generation benchmarks. Use when the goal is broader than injecting isolated facts.
---

# Specialize a Domain

Read [domain-design.md](references/domain-design.md). Treat specialization as
a measured behavior change, not fine-tuning or a replacement for retrieval.

## Define the Domain

1. Require a domain source, task description, and success criteria.
2. Separate factual recall, terminology, reasoning format, and generation style;
   do not mix them into one unexplained target set.
3. Create domain targets, adjacent-domain controls, general capability controls,
   and held-out domain tasks.
4. Include representative prompts and failure cases rather than only easy
   examples.

## Adapt

Use `$miyagi-author-probes` to build files, `$miyagi-search-patch` to run a
bounded search, and `$miyagi-preserve-capabilities` to define rejection gates.
Choose explicit layers, projections, seed, penalty, and checkpoint. Keep source
provenance beside the patch.

## Evaluate

Run probe evaluation on every split and `$miyagi-benchmark-patch` when domain
generation can be scored. Test paraphrases with `$miyagi-test-generalization`.
Review raw generations for style or reasoning claims that selected-token gaps
cannot establish.

## Report

State which domain behaviors improved, which did not, what adjacent/general
capabilities changed, the held-out result, patch size, model signature, and
whether the artifact passed preservation gates. Do not call the model generally
specialized from a small probe set.
