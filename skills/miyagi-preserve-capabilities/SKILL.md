---
name: miyagi-preserve-capabilities
description: Search for or evaluate a Miyagi adaptation while enforcing explicit preservation controls for general capability, neighboring knowledge, code, math, or safety behavior. Use when a target improvement must not cause unacceptable collateral regressions.
---

# Preserve Capabilities

Read [preservation-matrix.md](references/preservation-matrix.md). Use
`$miyagi-search-patch` to optimize and `$miyagi-evaluate-patch` to measure. The
search control penalty is useful guidance, but it is not a hard preservation
constraint.

## Define Gates Before Search

1. Name the target behavior and the capabilities to preserve.
2. Build separate control files for general capabilities, adjacent behavior,
   and high-risk regressions.
3. Define acceptable per-probe delta and sign-transition limits before looking
   at patched results.
4. Reserve held-out controls that are not passed to search.

## Optimize

Run a bounded search with explicit controls and a nonzero control penalty. Use
`min` fitness when the weakest target matters more than average improvement.
Keep the seed, checkpoint, model signature, and probe manifests recorded.

## Enforce Preservation

1. Evaluate the resulting patch against every control set and held-out set.
2. Apply hard gates to per-probe regressions, not only aggregate fitness.
3. Compare deterministic generations for representative prompts.
4. Reject or revise a patch when it crosses a predeclared gate, even if target
   fitness improved.

## Report

Report target gains, each preservation gate, worst control delta, sign breaks,
held-out results, generation regressions, and whether the patch passed all gates.
Do not call a patch capability-preserving when only average control fitness was
measured.
