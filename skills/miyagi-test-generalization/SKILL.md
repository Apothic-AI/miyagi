---
name: miyagi-test-generalization
description: Measure whether a Miyagi patch transfers beyond its search prompts through paraphrases, held-out facts, related entities, changed contexts, and deterministic generation or dataset checks. Use after knowledge injection, factual correction, domain specialization, or any patch whose target may be overfit.
---

# Test Generalization

Read [generalization-splits.md](references/generalization-splits.md). Treat the
search target set as training data and keep evaluation probes out of the search
command.

## Build Splits

1. Recover the exact target manifest used to create the patch.
2. Create held-out probes that preserve the behavior but change wording,
   entities, order, context, or surface form.
3. Add related facts that should transfer and unrelated controls that should not
   move.
4. For generation behaviors, create prompts and a deterministic scoring rule.

Do not call paraphrases held out if the same wording, answer pair, or template
was used during search.

## Evaluate

1. Run `$miyagi-evaluate-patch` on training, paraphrase, related, and unrelated
   sets with the same threshold and token mode.
2. Run `$miyagi-benchmark-patch` when generation can be scored from a dataset.
3. Compare target and held-out deltas, sign transitions, category counts, and
   raw generations.
4. Repeat with a second seed or candidate patch when conclusions depend on one
   search run.

## Interpret

Classify results as exact-prompt effect, paraphrase transfer, related-fact
transfer, or broad generalization. Use the narrowest supported label. A positive
training fitness with negative held-out deltas is overfitting evidence, not a
successful general adaptation.
