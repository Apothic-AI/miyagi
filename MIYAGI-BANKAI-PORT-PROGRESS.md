# Miyagi Bankai Port Progress

## Current State

- Miyagi is now a Rust library and CLI using the sibling `wwama-bankai` path
  dependency.
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

## Completion Boundary

The implementation defined by the sprint plan is complete for native CPU,
CUDA, and Vulkan workflows. The remaining items are evidence boundaries rather
than unimplemented core functionality: proving MLX-to-GGUF behavioral parity,
obtaining a mutable WebAssembly runtime fixture, and running user-selected
long-form generalization or dataset benchmarks.

## Evidence Gaps and Follow-up Validation

- MLX-trained Bankai patch behavior on GGUF remains an empirical compatibility
  question even when coordinates validate structurally.
- CUDA and Vulkan Miyagi integration should be run with the local model and
  recorded separately; wwama has already passed the underlying tensor path on
  both backends.
- WebAssembly mutable runtime behavior remains unsupported pending a real
  fixture.
