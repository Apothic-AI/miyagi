# Miyagi Bankai Port Progress

## Current State

- The Miyagi repository is an initial, clean jj checkout containing only an
  empty `README.md`.
- The evidence-based implementation plan is recorded in
  [`MIYAGI-BANKAI-PORT-SPRINT-PLAN.md`](MIYAGI-BANKAI-PORT-SPRINT-PLAN.md).
- No Rust crate files or implementation code have been added.
- The intended backend is the sibling [`wwama-bankai`](../wwama-bankai)
  workspace at the completed mutable-tensor implementation revision.

## Evidence Collected

- Bankai's reusable contract consists of patch serialization/application,
  probes, two fitness modes, screened scale-weighted greedy search, and the
  `search`, `apply`, `info`, and `eval` workflows.
- Bankai's Python GGUF backend is incomplete; it cannot serve as a runtime
  parity oracle for row scales or mutation.
- Bankai's checked-in patches use two schema variants: canonical
  `format: bankai_row_xor_v1` and legacy `type: row_flip`.
- Bankai's scale weighting is mean absolute scale per row, matching
  `wwama::Session::q1_0_row_scales`.
- wwama-bankai exposes the required safe APIs for tensor inventory, selected
  logits, logit gaps, Q1_0 scales, reversible row XOR, and generation.
- wwama mutation has passed local Bonsai 8B Q1_0 tests on CPU, CUDA, and Vulkan
  without changing llama.cpp source.
- MLX patch coordinates cannot be assumed behaviorally portable to GGUF solely
  because their layer/projection/row values are in bounds.
- Local Bonsai 8B and 27B GGUF fixtures are available under
  `/home/bitnom/Models/llm` for opt-in model tests.

## Next Work Item

Create the Rust crate foundation, bind it to the local wwama-bankai workspace,
and implement descriptor-driven Bonsai/Qwen architecture discovery before any
patch mutation or search logic.
