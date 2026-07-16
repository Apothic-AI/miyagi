# Miyagi Bankai Port Sprint Plan

## Objective

Create a Rust crate named `miyagi` that ports the reusable behavior of Python
Bankai to Rust and uses the completed `wwama-bankai` workspace as its only model
runtime. Miyagi will own Bankai's patch model, Bonsai/Qwen architecture mapping,
probe evaluation, fitness policy, search orchestration, CLI, and behavioral
validation. `wwama` will remain the generic llama.cpp execution and mutable
Q1_0 tensor backend.

The target is behavioral and format compatibility where the source behavior is
well-defined, not a mechanical translation of Python implementation details.
Where Bankai's source contains inconsistent formats, incomplete GGUF support,
hard-coded model assumptions, or process-local CLI behavior, Miyagi will define
and test an explicit Rust contract instead of preserving ambiguity.

## Evidence Baseline

The plan is based on the checked-out source and completed wwama work, not on the
Bankai README alone.

| Area | Verified evidence | Planning consequence |
| --- | --- | --- |
| Miyagi repository | [`README.md`](README.md) is empty and there is no `Cargo.toml` or Rust source. | The crate, module boundaries, dependency policy, tests, and documentation must be established from scratch. |
| Bankai reusable package | [`bankai/patch.py`](../../python/bankai/bankai/patch.py), [`probes.py`](../../python/bankai/bankai/probes.py), [`search.py`](../../python/bankai/bankai/search.py), and [`cli.py`](../../python/bankai/bankai/cli.py) contain the reusable patch, evaluation, search, and command behavior. | These modules define the primary port surface. The experiment scripts are evidence and validation workflows, not all automatically library APIs. |
| Bankai backend contract | [`backends/base.py`](../../python/bankai/bankai/backends/base.py) requires model loading, row discovery, row scales, tokenization, selected-token logit gaps, reversible row flips, and optional generation. | `wwama-bankai` now supplies every required primitive. Miyagi should wrap it with model-specific mapping and policy rather than duplicate llama.cpp integration. |
| Python GGUF backend | [`backends/gguf_backend.py`](../../python/bankai/bankai/backends/gguf_backend.py) leaves row scales and row mutation unimplemented and hard-codes Bonsai layer and row counts. | Python GGUF code is not a parity oracle. Miyagi must use live tensor descriptors and model-backed tests. |
| Scale weighting | The MLX backend computes each row's weight as the mean absolute value of its block scales. | `wwama::Session::q1_0_row_scales` uses the same mean-absolute aggregation and can drive equivalent candidate weighting. |
| Search lifecycle | Bankai pre-tokenizes probes, records target/control baselines, screens on the two worst baseline target probes, evaluates full fitness only after screening, accepts strictly improving candidates, and reverts rejected candidates with the same XOR. Accepted flips remain applied when search returns. | Miyagi needs an explicit search state machine and rollback guard so success, rejection, cancellation, and errors leave model state defined. |
| Fitness modes | Bankai provides mean target improvement and minimum target improvement, both subtracting a penalty for average one-sided control degradation. | Port both formulas with pure unit tests and explicit empty/non-finite input validation. |
| Candidate policy | Default search covers layers `[1, 2, 3, 4, 34]`, projections `gate_proj` and `up_proj`, samples proportional to row scale, and avoids retrying accepted or rejected coordinates. | Preserve these as named Bankai defaults, but discover and validate all coordinates against the loaded model before search. Do not encode them into wwama. |
| Patch format | Current library output uses `version: 1`, `format: bankai_row_xor_v1`, logical `(layer, proj, row)` flips, descriptive fields, derived stats, and free-form metadata. | Miyagi should read and write the established v1 format and validate every flip before mutation. |
| Format inconsistency | [`patch_math_v1.json`](../../python/bankai/patches/patch_math_v1.json) uses `type: row_flip`, while newer checked-in patches use `format: bankai_row_xor_v1`. | The reader needs deliberate legacy aliases; the writer should emit one canonical schema. |
| Statistics inconsistency | `Patch::n_bits_flipped` assumes every row is 4,096 bits, while Bankai documents different widths for `down_proj`; `size_bytes` is a conceptual 12-byte binary estimate although the stored artifact is JSON. | Compute logical bit counts from resolved live tensor widths and label compact-format estimates separately from actual serialized file size. |
| Probe token semantics | Bankai selects the last token when a correct or wrong answer string tokenizes to multiple tokens. It does not reject empty or ambiguous answer strings. | Preserve last-token compatibility as an explicit mode, record resolved token IDs, and provide strict validation so accidental multi-token probes are observable. |
| CLI surface | Bankai exposes `search`, `apply`, `info`, and `eval`. The no-prompt `apply` command mutates an in-memory model and then exits, so it does not persist a useful applied state. | Preserve recognizable commands but do not claim persistence. Applying must be tied to generation/evaluation or another explicit in-process operation. |
| Research workflows | Experiments cover structured row/layer effects, greedy search, held-out variations, diverse training probes, patch stacking, and generation-based safety evaluation. The first experiments also perform arbitrary bit and entire-layer mutations outside the reusable backend contract. | Port the workflows supported by row XOR and generation. Treat arbitrary bit/group mutation as a separate future backend decision, not a prerequisite for Miyagi. |
| wwama runtime | [`wwama-bankai/src/lib.rs`](../wwama-bankai/src/lib.rs) exposes mutable model loading, owned tensor descriptors, deterministic selected logits, logit gaps, Q1_0 row scales, row XOR, and generation. | Miyagi can remain safe Rust above wwama's public API and should not use wwama raw FFI. |
| Mutable loading | `SessionOptions::mutable_tensors` disables mmap. The completed wwama investigation found mapped writes faulted, while writable CPU, CUDA, and Vulkan loads passed mutation/restoration tests. | Search and patch application must opt into mutable tensors and document the memory/load tradeoff. Read-only inspection can use ordinary loading. |
| Tensor layout | The local Bonsai 8B Q1_0 fixture reports gate/up tensors as `[4096, 12288]` and down tensors as `[12288, 4096]`; GGML dimension 1 is the logical row. | Architecture mapping must derive row counts and widths from `TensorDescriptor`, not from Bankai constants. |
| Accelerator behavior | wwama's Bonsai fixture tests passed Q1_0 scale preservation, packed-byte mutation, deterministic logits, and exact double-XOR restoration on CPU, CUDA, and Vulkan. | Miyagi can include all three native paths in its validation matrix without changing llama.cpp. |
| llama.cpp boundary | `wwama-bankai` implements tensor access through a wwama-owned bridge and made no llama.cpp source changes. | llama.cpp changes are out of scope unless a new reproducible backend failure cannot be fixed at the wwama or Miyagi layer. |
| WebAssembly boundary | wwama compiles for CPU-only wasm32, but mutable tensor runtime access intentionally returns `UnsupportedTarget`. | Miyagi mutation/search is native-only initially. Do not advertise wasm patching based on compile-only evidence. |
| Local fixtures | Bonsai 8B Q1_0, Bonsai 8B, and Bonsai 27B Q1_0 GGUF models are available under `/home/bitnom/Models/llm`. | Model-backed tests can use local opt-in fixtures without downloading models or committing model files. |

## Compatibility Target

### Required compatibility

- Read the checked-in Bankai row-XOR patch files.
- Write canonical `bankai_row_xor_v1` JSON.
- Preserve logical flip coordinates: layer, projection, and row.
- Preserve Bankai's built-in math, code, and knowledge probe definitions.
- Implement mean and minimum-improvement fitness.
- Implement scale-weighted greedy search with two-probe screening.
- Provide equivalent `search`, `apply`, `info`, and `eval` user workflows.
- Support generated-output comparisons and held-out probe evaluation.

### Compatibility that must be proven

Bankai's checked-in patches were produced against an MLX model. Matching layer,
projection, and row numbers in a GGUF model does not by itself prove that MLX
packing and GGUF conversion preserve logical row identity. Miyagi may claim
syntax compatibility as soon as it can parse and validate those patches, but it
must not claim behavioral portability until a fixture test demonstrates that a
known patch has the expected direction of effect and restores exactly after
removal.

### Deliberate departures

- Reject invalid and out-of-range patch coordinates before mutating any tensor.
- Make duplicate-flip handling explicit. Patch composition uses XOR symmetric
  difference; malformed single patches should not silently contain duplicates.
- Distinguish actual JSON size from a compact binary encoding estimate.
- Never report a patch as persistently applied after the process exits.
- Return structured errors rather than terminating library code.
- Keep execution policy configurable instead of hard-coding full GPU offload.
- Do not port Modal or MLX runners into the initial Rust crate.

## Ownership and Architecture

### wwama-bankai owns

- llama.cpp build and FFI integration;
- model/session/context lifetimes;
- tokenization, selected logits, logit gaps, and generation;
- generic tensor inventory and descriptors;
- backend-aware tensor transfer;
- validated Q1_0 row scale extraction and row XOR;
- native backend synchronization and mutation capability errors.

### Miyagi owns

- model architecture discovery and `(layer, projection) -> tensor name` mapping;
- patch schema, validation, normalization, composition, application, and removal;
- probe schema, built-in sets, token compilation, measurement, and reports;
- fitness functions and search configuration;
- candidate enumeration, weighting, screening, acceptance, rollback, and progress;
- CLI behavior and output formats;
- experiment-style behavioral evaluation and safety checks;
- documentation of Bankai compatibility and known deviations.

### Proposed crate layout

```text
miyagi/
  Cargo.toml
  README.md
  src/
    lib.rs
    error.rs
    architecture.rs
    backend.rs
    patch.rs
    probe.rs
    fitness.rs
    search.rs
    evaluation.rs
    cli.rs
    main.rs
  tests/
    fixtures/
    patch_format.rs
    search_engine.rs
    model_backend.rs
    cli.rs
  examples/
    inspect_model.rs
    evaluate_patch.rs
```

Use a narrow backend trait around only the operations search requires so pure
search tests can use a deterministic fake backend. The production
implementation should be `WwamaBackend`; no alternate runtime is required.
Keep the trait independent of raw llama.cpp and GGML types.

## Sprint Sequence

### Sprint 0: Establish the crate and dependency contract

**Work**

- Create the Rust library and `miyagi` binary in the existing repository.
- Depend on the local [`wwama-bankai`](../wwama-bankai) workspace revision that
  contains commits `2526a3ff` and `b443d53f`.
- Define Miyagi features that forward native backend selection to wwama, with a
  CPU-capable baseline and explicit CUDA/Vulkan options.
- Add formatting, lint, unit-test, and fixture-test commands.
- Establish the project README and maintain the progress document alongside
  this plan.
- Record how the local path dependency will be replaced by an integrated or
  published wwama revision before Miyagi is distributed.

**Exit criteria**

- `cargo check`, formatting, and an empty test suite pass.
- The crate compiles against the exact wwama mutation API used by the plan.
- Feature forwarding does not accidentally enable multiple incompatible native
  accelerator builds.
- No raw FFI or llama.cpp source dependency appears in Miyagi.

### Sprint 1: Discover and validate Bonsai/Qwen tensor mapping

**Work**

- Define a typed `Projection` enum with canonical Rust names and serde aliases
  for Bankai's `gate_proj`, `up_proj`, and `down_proj` strings.
- Inspect wwama tensor inventory and resolve Qwen/Bonsai names such as
  `blk.{layer}.ffn_gate.weight`, `ffn_up`, and `ffn_down`.
- Discover available layer indices from tensor names instead of assuming 36.
- Build an `ArchitectureMap` containing tensor names, dimensions, row counts,
  logical row widths, type IDs, and backend placement.
- Require every mapped mutation tensor to be a supported two-dimensional Q1_0
  matrix and reject missing, duplicate, sparse, or inconsistent layer maps.
- Expose inspection output for users and fixture diagnostics.

**Exit criteria**

- The 8B Q1_0 fixture resolves all expected MLP projection tensors.
- Gate/up/down row counts and widths match live descriptors.
- Unknown architectures and non-Q1_0 models produce structured capability
  errors without mutation.
- Mapping tests use synthetic inventories as well as the local model fixture.

### Sprint 2: Implement the wwama-backed runtime adapter

**Work**

- Add `WwamaBackend` around a `wwama::Session` loaded with
  `mutable_tensors: true` for search/application workflows.
- Keep model inspection and patch-info paths capable of avoiding mutable loads.
- Implement layer/row queries through `ArchitectureMap`.
- Implement row scale retrieval, row XOR, deterministic logit gaps, tokenization,
  answer-token resolution, and generation through safe wwama APIs.
- Define prompt special-token behavior explicitly and test it against the GGUF
  tokenizer.
- Represent pre-tokenized probes with prompt tokens, selected token IDs, and
  source strings so reports remain auditable.
- Surface CPU/GPU configuration including context size, batch sizes, thread
  settings, and `n_gpu_layers` without leaking `SessionOptions` throughout the
  domain model.

**Exit criteria**

- One local model can execute inventory -> tokenize -> baseline gap -> flip ->
  changed gap -> flip -> exactly restored gap.
- Empty prompts, empty answer strings, invalid token IDs, context overflow,
  unsupported models, and mutation-disabled sessions have typed errors.
- The adapter never copies an entire model tensor for a row operation.

### Sprint 3: Port and harden the patch domain

**Work**

- Define serde models for patch version, format, name, description, base-model
  identity, flips, stats, and metadata.
- Read canonical v1 patches and the legacy `type: row_flip` variant found in
  `patch_math_v1.json`.
- Emit only canonical `format: bankai_row_xor_v1` output.
- Validate version, format, projection names, layer presence, row bounds,
  duplicates, and supported tensor types before applying the first flip.
- Implement transactional apply/remove behavior. On an intermediate failure,
  reverse already-applied flips before returning the error.
- Implement patch composition as XOR symmetric difference with deterministic
  ordering and conflict/accounting reports.
- Compute logical bits from mapped tensor widths. Report JSON bytes from the
  serialized artifact and label any compact representation estimate separately.
- Decide and document model identity validation after testing what stable GGUF
  identity is available without a new wwama API. At minimum, include the live
  architecture signature and require an explicit override for a known mismatch.

**Exit criteria**

- All checked-in Bankai patches parse.
- Canonical save/load round trips preserve semantic patch content.
- Invalid patches cannot leave partial model mutation behind.
- Applying and removing every valid fixture patch restores exact sampled bytes
  and deterministic logits.
- Composition is order-independent and applying a patch twice restores state.

### Sprint 4: Port probes, fitness, and evaluation reports

**Work**

- Port `Probe` and the built-in math, code, and knowledge sets.
- Support custom JSON probe files with duplicate-name, missing-field, and empty
  set validation.
- Compile probes once into token IDs before search.
- Provide Bankai-compatible last-token answer selection plus strict mode that
  rejects multi-token answers.
- Port mean and minimum target-improvement fitness exactly.
- Represent baselines and measurements by stable probe identity rather than
  relying on unordered maps.
- Add sign-transition classification: fixed, broke, stayed right, stayed wrong.
- Add delta-threshold summaries used by Bankai's variation experiments.

**Exit criteria**

- Formula tests cover improvements, one-sided control penalties, negative gaps,
  empty inputs, missing probes, and non-finite values.
- Built-in probes serialize and load consistently.
- Repeated measurement on an unmodified model returns deterministic selected
  logits.
- Reports expose the prompt, answer strings, resolved token IDs, baseline,
  patched gap, and delta when requested.

### Sprint 5: Implement a transactional deterministic search engine

**Work**

- Define `SearchConfig`, `SearchState`, `SearchEvent`, `SearchResult`, and a
  cancellation/checkpoint contract.
- Enumerate candidates from validated architecture descriptors and configured
  layers/projections.
- Build finite, non-negative scale weights and define behavior when all weights
  are zero or non-finite.
- Use a reproducible seeded RNG with a documented algorithm rather than relying
  on platform-dependent default randomness.
- Preserve Bankai's two-worst-baseline-probe screening semantics.
- Evaluate remaining targets and controls only after a candidate passes the
  screen.
- Accept only strict fitness improvement; use an RAII-style mutation guard so a
  rejected candidate or evaluation error is reverted automatically.
- Keep accepted flips applied on successful return, matching Bankai, while
  making final state explicit in the result type.
- Write checkpoints atomically with accepted flips, tried candidates, baselines,
  current fitness, RNG state or equivalent deterministic continuation data, and
  configuration.
- Emit structured progress events separately from terminal rendering.

**Exit criteria**

- A fake backend proves deterministic candidate order, screening, acceptance,
  rejection, rollback, exhaustion, cancellation, and resume behavior.
- Injected errors at every mutation/evaluation boundary leave only previously
  accepted flips applied.
- A fixed seed and fixed fake backend produce an identical patch across runs.
- Candidate memory use and sampling behavior are measured on the Bonsai 8B
  search space before the representation is finalized.

### Sprint 6: Build the CLI around explicit model state

**Work**

- Implement `miyagi search`, `apply`, `info`, and `eval` using the library API.
- Add an `inspect` command for architecture mapping and backend capability
  diagnostics.
- Support model path, target/control probe files or built-ins, layers,
  projections, iterations, fitness mode, penalty, seed, output path, backend
  feature/configuration, and checkpoint/resume inputs.
- Make `apply` perform an observable in-process action such as generation or
  evaluation; do not imply that an in-memory mutation persists after exit.
- Compare baseline and patched generation with controlled generation options.
- Write patch and report files atomically.
- Provide human-readable output by default and structured JSON output for
  automation.
- Return distinct nonzero exit statuses for argument, patch, model capability,
  evaluation, and search failures.

**Exit criteria**

- CLI integration tests cover help, valid commands, malformed files, invalid
  coordinates, unsupported models, and output file behavior.
- `info` can inspect a patch without loading a model and can add live validation
  when a model is supplied.
- `eval` restores the baseline state before exit, including on errors.
- Terminal output never reports conceptual compact size as actual JSON size.

### Sprint 7: Validate with local Bonsai fixtures and Bankai artifacts

**Work**

- Gate large model tests behind environment variables so ordinary unit tests do
  not require local model files.
- Use `/home/bitnom/Models/llm/Bonsai-8B-gguf/Bonsai-8B-Q1_0.gguf` for the main
  Q1_0 integration suite.
- Use `/home/bitnom/Models/llm/Bonsai-8B-gguf/Bonsai-8B.gguf` to prove clear
  rejection or capability reporting for unsupported quantization.
- Use `/home/bitnom/Models/llm/Bonsai-27B-gguf/Bonsai-27B-Q1_0.gguf` to prove
  architecture discovery is not hard-coded to the 8B layer count or dimensions.
- Import every checked-in Bankai patch and validate all logical coordinates
  against the 8B architecture map.
- Test one-candidate and short deterministic searches on CPU.
- Repeat mutation/restoration and a bounded search smoke test on CUDA and Vulkan
  when those features and devices are available.
- Record model identity, tensor map summary, backend placement, tokenization,
  baseline gaps, accepted flips, and restoration evidence in test artifacts.

**Exit criteria**

- CPU fixture tests prove byte and logit restoration after patch operations and
  rejected search candidates.
- CUDA and Vulkan results are separately reported rather than inferred from CPU.
- The 27B fixture either maps generically or yields a precise evidence-backed
  unsupported-architecture result.
- No test downloads a model implicitly.
- No llama.cpp source modification is needed.

### Sprint 8: Port supported Bankai behavioral workflows

**Work**

- Reproduce the reusable calculus search workflow with explicit training,
  control, and held-out probe files.
- Port variation and sign-transition evaluation.
- Port generalization-oriented training/validation separation.
- Port patch stacking tests for order independence, exact reversibility, overlap,
  and behavioral interference reporting.
- Port generation-based before/after comparison.
- Port the GSM8K-style evaluator as a generic JSON/JSONL dataset command with
  configurable prompt template and answer extractor; keep dataset acquisition
  outside the core crate.
- Compare outcomes relative to the GGUF baseline. Do not require exact MLX
  logits or claim reproduction when model format/runtime differences have not
  been controlled.
- Record whether checked-in MLX-trained patches transfer behaviorally to GGUF.

**Exit criteria**

- Training probes, controls, and held-out probes are reported separately.
- Safety reports distinguish changed gaps from sign changes and generated-answer
  accuracy changes.
- Patch stacking reports coordinate overlap and behavioral outcomes instead of
  inferring behavioral composition from XOR algebra.
- Behavioral claims are backed by saved machine-readable reports.

### Sprint 9: Documentation, API review, and handoff

**Work**

- Document installation, feature selection, local model configuration, patch
  format, architecture support, search semantics, CLI workflows, and limits.
- Document increased memory/load requirements from mutable wwama sessions.
- Explain syntax compatibility versus behavioral portability for Bankai patches.
- Review public types for unnecessary exposure of wwama internals.
- Review error recovery and model restoration paths.
- Run formatting, linting, unit, CLI, model fixture, CPU, CUDA, and Vulkan checks
  applicable to the environment.
- Update the progress document with the final validation matrix and unresolved
  evidence gaps.

**Exit criteria**

- A user can inspect a model, validate a patch, evaluate it, run a search, save
  the result, remove it, and verify restoration using documented commands.
- Library and CLI behavior agree on patch validation and model state.
- All compatibility claims identify their supporting tests.
- Remaining unsupported work is explicit and does not masquerade as parity.

## Verification Matrix

| Layer | Required verification |
| --- | --- |
| Pure domain logic | Patch parsing, canonical serialization, duplicate handling, composition, probe parsing, fitness formulas, candidate validation, deterministic RNG, and rollback state machine. |
| Architecture mapping | Synthetic tensor inventories, Bonsai 8B mapping, Bonsai 27B mapping, type/shape rejection, sparse layer detection, and projection aliases. |
| wwama adapter | Mutable load, tokenization, selected logits, row scales, row XOR, generation, error translation, and exact restoration. |
| Patch compatibility | All checked-in Bankai patches parse; legacy and canonical schemas are distinguished; coordinates are validated against live GGUF descriptors. |
| Search | Screening behavior, strict improvement, scale weighting, no repeated candidates, error rollback, cancellation, checkpoints, resume, and final applied-state contract. |
| CPU model | Baseline determinism, patch apply/remove, rejected-candidate restore, accepted-patch persistence, bytes restored, logits restored, and short search. |
| CUDA model | Capability detection, device-resident row transfer, serialized mutation/evaluation, restoration, and bounded search smoke test. |
| Vulkan model | Same checks as CUDA, reported independently. |
| Unsupported model | Clear rejection for non-Q1_0 mutation while read-only inspection remains useful where possible. |
| CLI | Argument validation, exit statuses, atomic output, JSON output, model-state claims, and interrupted/error cleanup. |
| Behavioral evaluation | Training/control/held-out separation, sign transitions, generation comparison, patch overlap, and machine-readable reports. |

## Decision Gates and Risks

### Dependency integration

Miyagi can initially use `../wwama-bankai` as a path dependency, but that path is
a task workspace rather than a durable distribution coordinate. Before release
or canonical monorepo integration, the wwama commits must be available through
the intended wwama repository revision. Do not copy wwama source into Miyagi.

### MLX-to-GGUF patch identity

The highest compatibility risk is logical row identity across model formats.
Bounds checks prove that a patch is structurally applicable, not that it targets
the same learned weights. Behavioral compatibility remains gated on evidence
from known patch application and, if necessary, comparison of conversion
metadata or decoded row content.

### Tokenization parity

Special-token insertion and last-subtoken selection can change measured gaps.
Store resolved IDs in reports, keep prompt tokenization policy explicit, and
avoid comparing results across runtimes without confirming token sequences.

### Model identity

Bankai's free-form `base_model` string cannot prevent applying a valid-looking
patch to the wrong conversion. Miyagi needs a practical identity policy, but it
should not add an expensive or brittle fingerprint without measuring its cost
and stability. Architecture signature validation is the minimum gate.

### Mutation recovery

XOR is self-inverse only if every intended operation completes exactly once.
Partial I/O failure, cancellation, or a process crash can invalidate naive
cleanup assumptions. In-process operations need rollback guards; checkpoints
must describe accepted state and never imply that a crashed process mutated the
model file, because wwama mutation is in-memory.

### Search determinism

Seeded candidate selection can be deterministic while GPU floating-point logits
near an acceptance boundary differ by backend. Record backend and measurements,
use strict comparison as Bankai does, and describe reproducibility at the
appropriate hardware/runtime level.

### Search performance

wwama's device path may transfer a row through host memory for each mutation.
Do not repeat Bankai's zero-cost or latency claims for Miyagi without measuring
the actual CPU, CUDA, and Vulkan paths. Optimize only after profiling shows the
dominant cost.

### Mutable model resources

Disabling mmap increases resource usage. CLI defaults and diagnostics must make
the mutable load intentional. Search should fail before model load when config
or patch input is already invalid.

### llama.cpp changes

No llama.cpp change is currently justified. Escalation requires a reproducible
Miyagi capability failure, evidence that wwama's public abstraction cannot
address it, and a regression test that defines the required upstream behavior.

### WebAssembly

Miyagi can keep domain types portable, but mutable search/application must
return a clear unsupported capability on wasm until wwama has runtime mutation
evidence. Do not create a second mutation implementation in Miyagi.

## Explicit Non-Goals for This Port

- MLX runtime support.
- Modal/cloud runner support.
- Arbitrary individual-bit or quantization-group mutation.
- Entire-layer snapshot and mutation APIs used only by Bankai's exploratory
  experiments.
- Ternary or non-Q1_0 patching.
- Persistently rewriting GGUF model files.
- Concurrent mutation and inference on one session.
- Raw GPU pointer access.
- llama.cpp source changes without an evidence-backed escalation.
- WebAssembly mutable tensor support without a runtime fixture.

## Definition of Done

The port is complete when:

1. Miyagi is a tested Rust library and CLI using wwama-bankai through safe APIs.
2. It reads all checked-in Bankai patches and writes canonical v1 patches.
3. It discovers Bonsai/Qwen tensor mappings from live descriptors without
   hard-coded 8B dimensions or layer count.
4. It evaluates probes, computes both Bankai fitness modes, and runs screened,
   scale-weighted, deterministic greedy search with transactional rollback.
5. Patch application, removal, composition, evaluation, and search state are
   explicit and recover correctly from ordinary errors and cancellation.
6. Local Bonsai fixture tests prove CPU behavior and separately record available
   CUDA and Vulkan behavior.
7. Behavioral reports distinguish structural compatibility, syntax
   compatibility, and demonstrated GGUF effects.
8. Supported Bankai generalization, stacking, generation, and safety workflows
   are available without requiring Python.
9. Documentation records limitations, resource tradeoffs, model identity, and
   unsupported targets.
10. No llama.cpp source changes are required by the completed implementation.

## First Implementation Slice

1. Create the crate and pin the local wwama-bankai dependency.
2. Implement typed projection and Bonsai/Qwen architecture discovery.
3. Wrap wwama in a testable Miyagi backend and prove one reversible candidate
   trial on the local 8B Q1_0 model.
4. Implement strict patch parsing/validation and import the checked-in Bankai
   artifacts.
5. Implement pure probe, fitness, and fake-backend search tests before running a
   model-backed search.

This ordering resolves model mapping, patch identity, and rollback risks before
the CLI or behavioral experiments can hide them.

## Implementation Results

The planned native port is implemented in this workspace.

| Capability | Result | Evidence |
| --- | --- | --- |
| Crate foundation | Passed | `Cargo.toml`, `Cargo.lock`, library modules, binary, `.gitignore`, and README are present. |
| wwama integration | Passed | `WwamaBackend` uses the safe wwama session, tensor descriptors, row scales, row XOR, selected logits, and generation APIs. |
| Architecture discovery | Passed | Bonsai 8B reports 36 layers and 399 tensors; Bonsai 27B reports 64 layers and 851 tensors without hard-coded dimensions. |
| Patch compatibility | Passed | All three checked-in Bankai patches parse; canonical `format` and legacy `type` schemas are covered. |
| Patch safety | Passed | Coordinates, Q1_0 type, rows, signatures, duplicate flips, atomic writes, composition, and rollback are validated. |
| Probe and fitness port | Passed | Built-in math/code/knowledge probes, custom JSON, last-token compatibility, strict token mode, mean fitness, minimum fitness, and reports are implemented. |
| Search | Passed | Deterministic SplitMix64 sampling, scale weighting, two-probe screening, acceptance/rejection rollback, checkpoints, resume, cancellation, and applied-state reporting are implemented and fake-backend tested. |
| CLI | Passed | `inspect`, `info`, `compose`, `eval`, `apply`, `search`, and `benchmark` are implemented; CLI help, legacy info, model inspect, eval, apply, and bounded search were exercised. |
| CPU model | Passed | Bonsai 8B Q1_0 mutation changes measured logits, preserves row scales, changes packed bytes, and restores exact bytes and logits. |
| CUDA model | Passed | Miyagi fixture test passed with all 37 layers on CUDA0 on the RTX 4050. |
| Vulkan model | Passed | Miyagi fixture test passed with all 37 layers on Vulkan0 on the RTX 4050. |
| wasm32 compilation | Passed | CPU-only `wasm32-unknown-unknown` check passes; mutable runtime remains capability-gated by wwama. |
| llama.cpp changes | Not required | No llama.cpp source file was changed. |

### Remaining evidence boundaries

- Bankai patches trained against MLX are structurally importable into GGUF, and
  `calculus_v1` was evaluated through Miyagi, but cross-format behavioral parity
  is not claimed without a controlled conversion comparison.
- Full generalization and safety experiments are available through custom probe
  files and the dataset benchmark command; their scientific outcomes remain
  model- and dataset-dependent rather than being hard-coded into the crate.
- WebAssembly mutable tensor runtime behavior remains unsupported until wwama
  has a model-backed transfer fixture.
