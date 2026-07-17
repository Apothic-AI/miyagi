---
name: miyagi-compose-patches
description: Compose two or more Miyagi row-XOR patch JSON files into one canonical patch using XOR symmetric-difference semantics, then inspect and live-validate the result. Use for patch stacking, conflict cancellation analysis, or producing a combined patch artifact.
---

# Compose Miyagi Patches

## Preflight

1. Run `miyagi --json info --patch ...` on every input.
2. Confirm each artifact parses and review its name, base model, flip count, and
   metadata.
3. Require exact `base_model` equality across inputs. Composition rejects mixed
   values before writing output.

## Compose

```sh
cargo run --release --no-default-features -- --json compose \
  --patch first.patch.json second.patch.json \
  --name combined-v1 \
  --output combined-v1.patch.json
```

Pass all input paths after one `--patch`; the option requires at least two.

## Understand XOR Semantics

- Keep a coordinate present in an odd number of inputs.
- Cancel a coordinate present in an even number of inputs.
- Reject a composition that cancels to an empty XOR mask.
- Expect deterministic coordinate ordering and `composed_from` metadata.
- Treat the output stats as artifact-level until live model validation computes
  logical bits from actual tensor widths.

## Validate

1. Inspect the output without a model.
2. Validate against the intended model:

```sh
cargo run --release --no-default-features -- --json info \
  --patch combined-v1.patch.json \
  --model model.gguf \
  --n-gpu-layers 0
```

3. Do not use `--allow-model-mismatch` without an explicit experimental reason.
4. Evaluate the combined patch against every constituent target, shared
   controls, and held-out probes with `$miyagi-evaluate-patch`.

Composition does not load or mutate a model. Behavioral interactions can still
be nonlinear, so do not infer combined effects from the input reports alone.
