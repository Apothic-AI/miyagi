# Domain Design

## Recommended matrix

| Set | Contents | Example |
| --- | --- | --- |
| domain target | Desired domain tasks | product API terminology |
| domain adjacent | Nearby but distinct tasks | related APIs or concepts |
| general control | Unrelated capabilities | math, code, knowledge |
| held-out domain | Unseen examples | new entities/templates |
| generation set | Full-output tasks | explanations or code |

Keep each set in a separate file so search cannot accidentally consume held-out
cases.

## Source and labels

Record source/version for factual domain content. For reasoning or style tasks,
define an explicit output scoring rule; a correct-vs-wrong token probe may only
measure one local decision.

## Acceptance

Predeclare target thresholds and preservation gates. A domain patch is useful
only when the target gain survives held-out tests and the collateral tradeoff is
acceptable for the user's application.

## Boundary

Miyagi mutates selected Q1_0 rows in memory and emits a patch artifact. It does
not update tokenizer vocabulary, add external documents, perform retrieval, or
guarantee broad domain competence.
