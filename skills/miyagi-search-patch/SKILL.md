---
name: miyagi-search-patch
description: Run Miyagi's deterministic scale-weighted screened greedy search to create a Q1_0 row-XOR patch, with explicit target/control probes, layers, projections, fitness policy, checkpoints, cancellation, and exact resume handling. Use when training a new sparse patch or continuing an interrupted Miyagi search.
---

# Search for a Miyagi Patch

Read [search-contract.md](references/search-contract.md) before launching or
resuming a model-backed search.

## Prepare

1. Inspect the model with `$miyagi-inspect-model` and record its layer range and
   architecture signature.
2. Create target and control probes with `$miyagi-author-probes`. Reserve held-
   out probes that search will not optimize.
3. Choose existing layers and projections explicitly. Do not rely on the
   Bankai defaults `[1,2,3,4,34]` when the inspected architecture differs.
4. Choose `mean` fitness for average target improvement or `min` to optimize the
   worst target improvement. Keep the control penalty finite and non-negative.
5. Start with a bounded iteration count and a checkpoint path.

## Run

```sh
cargo run --release --no-default-features -- search \
  --model model.gguf \
  --n-gpu-layers 0 \
  --target target-probes.json \
  --control control-capability.json \
  --control control-safety.json \
  --layers 1,2,3,4 \
  --projections gate_proj,up_proj \
  --iters 50 \
  --fitness mean \
  --penalty 2.0 \
  --seed 42 \
  --screen-probes 2 \
  --token-mode strict \
  --name targeted-behavior-v1 \
  --description 'Bounded search with explicit controls' \
  --checkpoint targeted-behavior.checkpoint.json \
  --output targeted-behavior.patch.json \
  --report targeted-behavior.search.json
```

Use accelerator Cargo features and `--n-gpu-layers` only when that backend has
been selected and validated.

## Resume

Resume with the same model, probes, layers, projections, seed, fitness,
penalty, screen count, token mode, name, description, and base-model path. Only
increase `--iters`. Pass the checkpoint through `--resume`; it will also be
updated unless a separate `--checkpoint` path is supplied.

## Verify

1. Inspect and live-validate the emitted patch with `$miyagi-evaluate-patch`.
2. Evaluate target, control, and held-out probes.
3. Benchmark representative datasets when applicable.
4. Record accepted flips, final fitness, screened candidates, completed
   iterations, seed, architecture signature, and restoration claims precisely.
