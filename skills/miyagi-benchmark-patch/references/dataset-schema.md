# Dataset Schema and Extraction

## Supported files

Miyagi accepts either a JSON array or newline-delimited JSON. Each selected
record must contain string values for the configured question and answer fields.

JSON array example:

```json
[
  {"question": "What is 2 + 2?", "answer": "4"},
  {"question": "What is 3 + 3?", "answer": "6"}
]
```

JSONL example:

```jsonl
{"question":"What is 2 + 2?","answer":"#### 4"}
{"question":"What is 3 + 3?","answer":"#### 6"}
```

## Prompt template

Miyagi replaces every literal `{question}` occurrence with the record's
question. Ensure the template asks for an output format matched by the answer
regex. The CLI does not reject a template missing `{question}`, so verify it
explicitly.

## Regex contract

Both `--answer-regex` and `--gold-regex` must compile and contain capture group
1. Miyagi compares those captured strings after removing commas.

The default gold regex expects GSM8K-style answers such as `#### 42`:

```text
####\s*([\-\d,]+)
```

For plain numeric answer fields such as `"42"`, override it:

```text
^([\-\d,]+)$
```

If the generated response does not match the answer regex, `predicted` is null
and the case is incorrect. If a gold answer does not match, the command fails
for that record instead of silently scoring it wrong.

## Report shape

The report contains:

- `baseline`: correct count, total count, and per-case results
- `patched`: the same report when `--patch` is supplied; otherwise null
- `model_restored`: true after a successful run

Each case includes its index, question, expected capture, optional predicted
capture, correctness, and full generated response.

## Determinism and interpretation

Miyagi uses the configured seed and generation settings for each case. Preserve
the full invocation for reproducibility. Accuracy depends on both model behavior
and extraction rules; audit raw responses before attributing score changes to
the patch.
