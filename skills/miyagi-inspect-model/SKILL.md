---
name: miyagi-inspect-model
description: Inspect a GGUF model with Miyagi, determine whether its Qwen/Bonsai Q1_0 MLP tensors are supported, and report architecture layers, row dimensions, tensor placement, and the architecture signature. Use before patch validation, evaluation, search, benchmarking, or when diagnosing unsupported model errors.
---

# Inspect a Miyagi Model

Work from the Miyagi crate root. Use an existing `miyagi` binary when the user
provides one; otherwise invoke the crate with Cargo.

## Inspect

1. Confirm the model path exists and identify the requested execution backend.
2. Default to CPU inspection with `--n-gpu-layers 0` unless the user asks to
   inspect accelerator placement.
3. Run machine-readable inspection:

```sh
cargo run --release --no-default-features -- --json inspect \
  --model /path/to/model.gguf \
  --n-gpu-layers 0
```

4. Read `miyagi_supported`, `architecture_error`, `architecture.layer_count`,
   `architecture.signature`, and the mapped tensor entries.
5. Run `--all-tensors` only when mapped tensors are missing, malformed, or need
   backend-placement diagnosis; full inventories can be large.

## Interpret

- Treat `miyagi_supported: true` as structural support for descriptor-driven
  `blk.<layer>.ffn_{gate,up,down}.weight` two-dimensional Q1_0 tensors.
- Verify every layer has gate, up, and down projections. Record row counts,
  logical widths, and backend placement rather than assuming an 8B layout.
- Preserve the architecture signature for later patch/search validation.
- Treat `architecture_error` as the exact capability failure. Do not proceed to
  mutation, patch application, or search on an unsupported map.
- State that inspection is read-only and does not require mutable tensors or
  rewrite the GGUF file.

## Report

Summarize the model path, support decision, layer count, signature, projection
dimensions, backend placement, and any capability error. Distinguish structural
support from evidence that a particular patch improves behavior.
