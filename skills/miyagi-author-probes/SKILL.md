---
name: miyagi-author-probes
description: Create and validate custom Miyagi probe JSON files for target behavior, control behavior, patch evaluation, and patch search. Use when defining prompts with correct-versus-wrong answer tokens, choosing compatibility or strict token semantics, separating target and control sets, or diagnosing ambiguous probe tokenization.
---

# Author Miyagi Probes

Create small, auditable probe sets whose logit gaps directly represent the
behavior being measured. Read [probe-schema.md](references/probe-schema.md)
before writing or validating a file.

## Design

1. Define one behavior per target file. Put unrelated capabilities and safety
   invariants in separate control files.
2. Give every probe a unique stable `name` and a meaningful `category`.
3. Write prompts that place the answer at the model's next-token decision.
4. Choose `correct` and `wrong` strings that differ only in the intended answer;
   preserve leading spaces when tokenization requires them.
5. Prefer several varied probes over repeated wording. Include held-out probes
   outside the search target set for later evaluation.

## Validate

1. Validate JSON structure, non-empty fields, and unique names locally.
2. Inspect the model with `$miyagi-inspect-model` before model-backed validation.
3. Prefer `--token-mode strict` when each answer must be exactly one token.
   Use compatibility mode only deliberately; it selects the final token from a
   multi-token answer string.
4. Validate compilation and measurement with `miyagi eval` and the no-op patch
   from the reference. Baseline and patched gaps should be identical for that
   patch.
5. Fix empty prompts, duplicate names, empty answers, and strict-mode ambiguous
   token errors before starting a search.

## Deliver

Provide the target/control JSON files, the chosen token mode, structural and
model-backed validation results, and any probes reserved for held-out testing.
Do not infer broad behavior from a small or lexically repetitive set.
