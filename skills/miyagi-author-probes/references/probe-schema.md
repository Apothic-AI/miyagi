# Probe Schema and Validation

## JSON shape

Store probes as a JSON array:

```json
[
  {
    "prompt": "1 + 1 =",
    "correct": " 2",
    "wrong": " 3",
    "name": "addition_1",
    "category": "math"
  }
]
```

Required string fields are `prompt`, `correct`, `wrong`, and `name`.
`category` defaults to `general`. The aliases `correct_token` and
`wrong_token` are accepted, but prefer the canonical field names above.

Names must be unique within each loaded file. When multiple selectors are
combined, keep names globally unique so reports and fitness alignment remain
auditable.

## Token modes

- `compatibility`: tokenize each answer string and select its final token ID.
- `strict`: require each answer string to tokenize to exactly one token.

Strict mode catches accidental ambiguity. Compatibility mode preserves Bankai
behavior but can hide differences in answer prefixes. Keep leading whitespace
intentional because it can change tokenization.

## Structural checks

Use `jq` when available:

```sh
jq -e '
  type == "array" and length > 0 and
  all(.[];
    (.prompt | type == "string" and length > 0) and
    (.correct | type == "string" and length > 0) and
    (.wrong | type == "string" and length > 0) and
    (.name | type == "string" and length > 0)
  ) and
  ([.[].name] | length == (unique | length))
' probes.json
```

## Model-backed token validation

Miyagi has no standalone probe-validation subcommand. Use a canonical no-op
patch to exercise probe loading, token compilation, and logit measurement
without changing model rows:

```json
{
  "version": 1,
  "format": "bankai_row_xor_v1",
  "name": "probe-validation-noop",
  "description": "No-op patch used to validate probes",
  "base_model": "validation-only",
  "flips": [],
  "stats": {
    "n_flips": 0,
    "logical_bits_flipped": 0,
    "compact_binary_estimate_bytes": 0
  },
  "metadata": {}
}
```

Run:

```sh
cargo run --release --no-default-features -- --json eval \
  --model /path/to/model.gguf \
  --n-gpu-layers 0 \
  --patch /path/to/noop-patch.json \
  --probes /path/to/probes.json \
  --token-mode strict \
  --report /path/to/probe-validation-report.json
```

The no-op report should contain identical baseline and patched gaps and zero
deltas. A nonzero result indicates the artifact or invocation was not the
intended no-op validation.
