# Preservation Matrix

Define a matrix before search:

| Set | Purpose | Used in search | Held out | Gate |
| --- | --- | --- | --- | --- |
| target | Desired behavior | yes | no | minimum improvement |
| adjacent | Nearby facts or concepts | yes | yes | no unacceptable break |
| general | Broad capability probes | yes | yes | no large degradation |
| high-risk | Safety or critical behavior | yes | yes | zero tolerated breaks |

The exact gate values belong to the user and task. Examples include:

- no control delta below `-0.1`;
- no `broke` transition on a critical probe;
- no more than one degraded general-capability probe; or
- no generation regression on a required prompt set.

## Why aggregate fitness is insufficient

Miyagi's control penalty subtracts average one-sided control degradation. A
single severe regression can be hidden by many unchanged controls. Inspect every
`ProbeDelta`, sign transition, category summary, and generation case before
accepting a patch.

## Artifact record

Store the target/control/held-out manifests, thresholds, model architecture
signature, search seed/configuration, patch artifact, evaluation reports, and
the final pass/fail decision together.
