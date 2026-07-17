---
name: miyagi-benchmark-patch
description: Benchmark a Miyagi patch against JSON or JSONL generation datasets, compare deterministic baseline and patched accuracy, customize record fields, prompt templates, answer extraction, limits, and reports, and verify in-process model restoration. Use for regression, generalization, or bounded safety/capability evaluations.
---

# Benchmark a Miyagi Patch

Read [dataset-schema.md](references/dataset-schema.md) before selecting regexes
or interpreting accuracy.

## Prepare

1. Inspect the model with `$miyagi-inspect-model`.
2. Validate the patch with `$miyagi-evaluate-patch` when a patched comparison is
   requested.
3. Inspect several dataset records. Confirm the question and answer fields are
   strings and determine the exact gold-answer format.
4. Define a prompt template containing `{question}`.
5. Define answer and gold regexes with the desired value in capture group 1.
6. Run a small `--limit` smoke test before the full dataset.

## Run

```sh
cargo run --release --no-default-features -- --json benchmark \
  --model model.gguf \
  --n-gpu-layers 0 \
  --dataset dataset.jsonl \
  --patch patch.json \
  --question-field question \
  --answer-field answer \
  --prompt-template "Solve and end with 'The answer is [number]'.\n\n{question}" \
  --answer-regex '(?i)the answer is[:\s]*\$?([\-\d,]+)' \
  --gold-regex '^([\-\d,]+)$' \
  --limit 20 \
  --max-tokens 400 \
  --seed 42 \
  --report benchmark.json
```

Remove `--patch` for a baseline-only run. Keep seed, generation length, prompt,
fields, regexes, and record ordering fixed when comparing runs.

## Review

1. Verify total/correct counts and inspect every failed case's expected value,
   prediction, and raw response.
2. Distinguish extraction failures (`predicted: null`) from incorrect extracted
   answers.
3. Confirm `model_restored: true` for patched runs. Treat restoration errors as
   invalidating subsequent use of that process.
4. Report the dataset, limit, extraction rules, generation settings, baseline
   score, patched score, and per-case regressions.

Do not label a small smoke dataset as broad generalization or safety evidence.
