# Search Contract

## Algorithm

Miyagi builds candidates from live row scales for the configured layer and
projection pairs. It samples untried rows with scale weighting, tests the worst
baseline target probes first, fully measures candidates that pass screening,
and accepts only strictly improving fitness.

Rejected and screened candidates are XORed again to restore their row. Accepted
flips remain applied in the process so later candidates are evaluated on the
current patch.

Mean fitness uses average target improvement. Minimum fitness uses the worst
target improvement. Both subtract:

```text
control_penalty * average_one_sided_control_degradation
```

## Output and process state

On successful completion, Miyagi writes the canonical patch and optional full
search report. The accepted patch remains applied only until the search process
exits; the GGUF file is never rewritten.

On cancellation, Miyagi writes the checkpoint when configured and returns a
search-cancelled error. The final patch output is not written by the CLI because
the command did not complete.

With `--json`, progress events are emitted as JSON lines before the final JSON
result. Treat stdout as an event stream, not one JSON document. Prefer
`--report` for a clean result artifact.

## Checkpoints

A checkpoint binds:

- checkpoint version
- architecture signature
- search configuration
- completed iterations
- accepted and tried coordinates
- current fitness and RNG state
- target and control baselines

Resume remeasures the baseline and rejects drift, probe changes, architecture
changes, or configuration changes. The only supported configuration change is
raising `max_iters` above the completed count.

## Resource consequences

Search requires mutable tensors, which disables mmap in the wwama session and
can materially increase memory use and load time. Candidate construction reads
row scales across every selected layer/projection pair. Expand the search space
only after a bounded run confirms the model, probes, controls, and checkpoint
workflow.

## Evidence boundary

Positive training fitness shows improvement under the configured target and
control measurements. It does not establish held-out generalization, safety, or
portability to another GGUF or MLX model. Evaluate those separately.
