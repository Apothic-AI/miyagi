# Regression Report

Use this structure for a diagnosis:

```text
Candidate:
Model/signature:
Patch/artifact:
Baseline report:
Patched report:
Generation report:

Observed regressions:
- probe/category:
- baseline gap:
- patched gap:
- delta/transition:
- raw generation evidence:

Candidate explanations:
- evidence-backed:
- unverified hypothesis:

Decision:
Next controlled experiment:
```

## Evidence levels

- **Observed:** directly present in a report or reproducible command output.
- **Correlated:** appears with a patch, category, or constituent but has not been
  isolated.
- **Causal:** supported by a controlled ablation or independently repeated
  experiment.

Keep these labels in the report. A larger patch, overlapping coordinate, or
search metadata field is not by itself causal evidence for a regression.
