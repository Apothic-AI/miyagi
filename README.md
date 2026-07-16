<p align="center">
  <img src="assets/miyagi-4.png" alt="Miyagi" width="360">
</p>

# Miyagi

Miyagi is the Rust port of the reusable Bankai XOR-patch workflow for true
binary language models. It uses the sibling [`wwama`](../wwama)
crate for GGUF loading, deterministic selected-logit evaluation, Q1_0 row-scale
inspection, reversible row mutation, and generation.

## Status

The native CPU path is implemented and validated against the local Bonsai 8B
Q1_0 fixture. Descriptor-driven architecture discovery also passes against the
local Bonsai 27B Q1_0 fixture. Miyagi has passed the same mutation/restoration
check through wwama on CPU, CUDA, and Vulkan. Mutation is intentionally
native-only until wwama has a runtime WebAssembly transfer fixture.

Miyagi does not modify llama.cpp source files. Model mutation is in-memory and
requires a writable wwama session, so it does not rewrite GGUF files.

## Commands

Inspect a model without enabling mutable tensors:

```sh
cargo run -- inspect \
  --model ~/Models/llm/Bonsai-8B-gguf/Bonsai-8B-Q1_0.gguf \
  --n-gpu-layers 0
```

Read a Bankai patch without a model:

```sh
cargo run -- info --patch ../python/bankai/patches/calculus_v1.json
```

Evaluate a patch against built-in probes. The model must be loaded with
`mutable_tensors`, because patch application needs writable backend storage:

```sh
cargo run -- eval \
  --model ~/Models/llm/Bonsai-8B-gguf/Bonsai-8B-Q1_0.gguf \
  --n-gpu-layers 0 \
  --patch ../python/bankai/patches/calculus_v1.json \
  --probes math,knowledge
```

Search uses Bankai's scale-weighted, screened greedy row search. A search leaves
the accepted patch applied in the current process and writes the patch artifact;
the model file itself is never changed.

```sh
cargo run -- search \
  --model ~/Models/llm/Bonsai-8B-gguf/Bonsai-8B-Q1_0.gguf \
  --n-gpu-layers 0 \
  --target math \
  --output math_patch.json \
  --iters 200 \
  --checkpoint math_patch.checkpoint.json
```

Use `--json` on commands for machine-readable output. `--resume` continues a
checkpoint with the same search policy and may use a larger iteration ceiling.
Search cancellation writes the checkpoint when `--checkpoint` is present.

## Features

- Default build: native CPU through wwama.
- `--features cuda`: forward CUDA support to wwama.
- `--features vulkan`: forward Vulkan support to wwama.

Run the pure and CLI tests with:

```sh
cargo test --no-default-features --all-targets
```

Large model tests are opt-in:

```sh
MIYAGI_TEST_MODEL=~/Models/llm/Bonsai-8B-gguf/Bonsai-8B-Q1_0.gguf \
MIYAGI_TEST_GPU_LAYERS=0 \
cargo test --no-default-features --test model_backend
```

## Patch Compatibility

Miyagi reads Bankai's canonical `bankai_row_xor_v1` format and the legacy
`type: row_flip` artifact. It writes one canonical schema. Patch coordinates are
validated against the live Q1_0 tensor descriptors, and logical bit counts use
the resolved tensor width rather than assuming every projection has the same
shape.

Syntax compatibility does not prove that an MLX-trained patch has the same
behavior on a converted GGUF model. Miyagi reports that distinction and requires
an architecture signature match unless the caller explicitly overrides it.

## Boundaries

- Q1_0 row XOR is supported; arbitrary bit, group, floating-point, and ternary
  mutation are not.
- MLP tensor naming belongs to Miyagi; generic tensor access belongs to wwama.
- Concurrent mutation and inference on one session is unsupported.
- WebAssembly builds may compile through wwama, but mutable model operations are
  capability-gated until runtime behavior is proven.
- No llama.cpp source change is required by the current implementation.

See [`MIYAGI-BANKAI-PORT-SPRINT-PLAN.md`](MIYAGI-BANKAI-PORT-SPRINT-PLAN.md)
for the evidence, validation matrix, and remaining boundaries.
