# Evaluation and Generation Semantics

## Probe gaps

Each probe compares the selected logit for `correct` against `wrong`:

```text
gap = correct_logit - wrong_logit
```

A positive gap means the correct token outranks the wrong token. A delta is:

```text
delta = patched_gap - baseline_gap
```

The `change_threshold` classifies delta magnitude:

- `delta > threshold`: improved
- `delta < -threshold`: degraded
- otherwise: unchanged

Sign transitions are independent of that threshold:

- `fixed`: baseline gap is non-positive and patched gap is positive
- `broke`: baseline gap is positive and patched gap is non-positive
- `stayed_right`: both gaps are positive
- `stayed_wrong`: both gaps are non-positive

Review per-probe values and category summaries. Aggregate counts can hide a
large regression on a single control or safety probe.

## Patch validation

`info --model` verifies live coordinates and computes logical bits from actual
tensor widths. An architecture signature mismatch blocks validation unless
`--allow-model-mismatch` is supplied. That override permits an experiment; it
does not establish that logical rows have equivalent meaning across models or
formats.

## Generation comparison

`apply` generates once without the patch and once with it using the same seed
and generation configuration, then removes the patch. Deterministic settings
make the comparison reproducible for the same runtime and model, but generated
text remains qualitative evidence unless prompts and scoring rules were chosen
in advance.

## Claims

Use precise conclusions:

- "Coordinates validate against this architecture."
- "The patch improved N probes and degraded M at threshold T."
- "The model was restored before command exit."

Do not substitute those observations for broad capability, safety, or
cross-format parity claims.
