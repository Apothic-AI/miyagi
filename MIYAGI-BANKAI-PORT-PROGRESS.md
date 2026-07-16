# Miyagi Bankai Port Progress

## Current State

- Miyagi is now a Rust library and CLI using the canonical sibling `wwama` path
  dependency (`wwama v0.2.0`).
- Descriptor-driven Qwen/Bonsai MLP mapping is implemented for
  `blk.<layer>.ffn_{gate,up,down}.weight` Q1_0 tensors.
- Patch parsing, canonical serialization, legacy compatibility, validation,
  composition, transactional apply/remove, probes, fitness, evaluation,
  dataset benchmarking, and deterministic screened search are implemented.
- CLI commands are implemented: `inspect`, `info`, `compose`, `eval`, `apply`,
  `search`, and `benchmark`.
- Pure unit, CLI, checked-in artifact, fake-backend search, and opt-in model
  integration tests are present.

## Validation Completed

- `cargo test --no-default-features --all-targets`: passed.
- Checked-in Bankai patches: all three parse and validate against a synthetic
  36-layer Q1_0 architecture; both canonical and legacy schemas are covered.
- Bonsai 8B Q1_0 CPU fixture: writable row mutation changes a measured logit gap
  and exact double mutation restores all measured gaps.
- Bonsai 27B Q1_0 CPU fixture: descriptor-driven mapping loads 64 layers and
  does not assume the 8B dimensions.
- No llama.cpp source files were changed.

## Native Validation Matrix

- CPU: Bonsai 8B Q1_0 mutation test passed with exact row-byte and logit
  restoration.
- CUDA: the same Miyagi test passed with all 37 layers on CUDA0 on the RTX 4050.
- Vulkan: the same Miyagi test passed with all 37 layers on Vulkan0 on the RTX
  4050.
- wasm32: CPU-only compilation passed; mutable model access remains unsupported.
- CLI: `inspect`, `eval`, `apply`, and bounded `search` completed against the
  local Bonsai 8B Q1_0 fixture.
- All three checked-in Bankai patches applied and restored against Bonsai 8B
  GGUF with `--allow-model-mismatch`; probe effects were measurable but mixed,
  so behavioral MLX-to-GGUF parity is not claimed.
- Composed `calculus_v1` with `patch_math_v1` into a 144-flip patch, evaluated
  it, and restored the model successfully.
- A 12-iteration native search completed with one accepted flip and a written
  checkpoint.
- A two-case dataset benchmark completed with baseline accuracy `2/2`, patched
  accuracy `1/2`, and `model_restored: true`. This is a regression smoke test,
  not a safety or generalization claim.

## Completion Boundary

The implementation defined by the sprint plan is complete for native CPU,
CUDA, and Vulkan workflows. Miyagi now uses the canonical sibling `wwama`
crate. The bounded compatibility, stacking, search, generation, and dataset
smoke checks are complete. The remaining limitation is that the available
evidence does not establish MLX-to-GGUF behavioral parity, broad safety
generalization, or mutable WebAssembly runtime support.

## Evidence Gaps and Follow-up Validation

- Representative MLX-trained Bankai patch behavior on GGUF was observed to be
  mixed across the built-in probes; structural compatibility therefore does not
  establish behavioral parity.
- CUDA and Vulkan Miyagi integration has been validated against the local model
  and should remain recorded separately from CPU evidence.
- WebAssembly mutable runtime behavior remains unsupported pending a real
  fixture.
