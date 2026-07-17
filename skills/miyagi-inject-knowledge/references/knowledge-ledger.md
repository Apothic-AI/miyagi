# Knowledge Ledger

Use a ledger to keep source facts separate from generated probes. The ledger is
an input convention for the skill, not a Miyagi CLI schema.

```json
[
  {
    "id": "country-capital-australia-v1",
    "category": "geography",
    "subject": "Australia",
    "prompt": "The capital of Australia is",
    "correct": " Canberra",
    "wrong": " Sydney",
    "source": "user-approved geography ledger, entry 12",
    "paraphrases": [
      "Australia's capital city is",
      "Which city is the capital of Australia?"
    ]
  }
]
```

## Requirements

- `id` must remain stable across revisions.
- `correct` must be supported by the cited source.
- `wrong` must be an explicit contrasting answer, not an invented factual
  claim. It is used only as the comparison token/string in a probe.
- Keep source text or a source identifier outside the model prompt unless the
  experiment intentionally studies context-conditioned recall.
- Keep paraphrases separate from training prompts when testing generalization.

## Split guidance

Use one ledger version for a reproducible run. Do not put every paraphrase into
the search target set. A useful minimum is one training prompt and one held-out
paraphrase per fact, plus a separate held-out fact when the ledger is large
enough.

## Provenance

Copy ledger IDs, source identifiers, and version information into a sidecar
manifest or patch metadata. Miyagi's patch format does not verify that a source
is authoritative.
