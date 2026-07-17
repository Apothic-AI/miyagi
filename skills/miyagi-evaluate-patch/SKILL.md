---
name: miyagi-evaluate-patch
description: Inspect and validate a Miyagi row-XOR patch, measure its effects on built-in or custom probes, and compare deterministic baseline versus patched generation. Use for patch review, compatibility checks, regression analysis, generation examples, or diagnosing model-signature and coordinate failures.
---

# Evaluate a Miyagi Patch

Use [report-semantics.md](references/report-semantics.md) when interpreting
probe deltas and generation comparisons.

## Preflight

1. Parse the artifact without loading a model:

```sh
cargo run --release --no-default-features -- --json info --patch patch.json
```

2. Inspect the model with `$miyagi-inspect-model`.
3. Validate live coordinates, logical bit count, and architecture signature:

```sh
cargo run --release --no-default-features -- --json info \
  --patch patch.json --model model.gguf --n-gpu-layers 0
```

4. Do not add `--allow-model-mismatch` automatically. Use it only when the user
   explicitly accepts structural testing without an architecture identity match.

## Measure Probes

Run built-in selectors, custom probe files, or both:

```sh
cargo run --release --no-default-features -- --json eval \
  --model model.gguf \
  --patch patch.json \
  --probes math,code,knowledge \
  --token-mode compatibility \
  --change-threshold 0.1 \
  --report evaluation.json
```

Prefer strict token mode for newly authored probes. Use the report file as the
stable machine-readable artifact.

## Compare Generation

Use identical deterministic generation settings for baseline and patched text:

```sh
cargo run --release --no-default-features -- --json apply \
  --model model.gguf \
  --patch patch.json \
  --prompt 'Solve 7 * 8.' \
  --max-tokens 100 \
  --seed 42
```

Repeat across representative and adversarial prompts; one example is not an
evaluation suite.

## State Boundaries

`eval` and `apply` load mutable tensors, apply the patch in memory, and remove
it before successful exit. They do not rewrite the GGUF. Treat restoration
failure as a hard error and do not claim the model returned to baseline.

Report structural validation separately from measured behavioral effects and
from any claim of MLX-to-GGUF parity.
