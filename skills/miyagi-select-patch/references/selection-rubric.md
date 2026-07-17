# Candidate Selection Rubric

Use hard gates before weighted ranking:

| Gate | Evidence | Action |
| --- | --- | --- |
| structural | `info --model` succeeds | reject on failure |
| target | target delta or fitness threshold | reject if unmet |
| preservation | per-probe/category limits | reject if exceeded |
| held out | generalization threshold | reject or mark weak |
| generation | required output criteria | reject when required and unmet |
| restoration | command reports restored state | reject incomplete evidence |

For surviving candidates, rank in this order unless the user specifies
otherwise:

1. target success;
2. held-out transfer;
3. preservation and worst-case control behavior;
4. generation stability;
5. fewer flips or logical bits;
6. simpler, reproducible search metadata.

## Comparison table

```text
candidate | target | worst_control_delta | held_out | generation | flips | status
```

Keep raw report paths beside the table. A single scalar score may be useful for
sorting, but it must not replace the hard gates or per-case evidence.
