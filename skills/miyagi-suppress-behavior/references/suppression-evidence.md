# Suppression Evidence

## Three evidence layers

1. **Preference:** the desired token/alternative gap increases relative to the
   unwanted token.
2. **Generation:** deterministic generations no longer exhibit the unwanted
   response under specified prompts and decoding settings.
3. **Transfer:** paraphrases, contexts, and held-out prompts show the intended
   behavior without topic shifting or collateral regressions.

Report these separately. Passing the first layer does not imply passing the
second or third.

## Required controls

- benign prompts containing similar vocabulary;
- prompts where the suppressed phrase is appropriate;
- nearby behaviors that should remain available;
- general capability probes; and
- high-risk or policy controls when the task concerns safety.

## Failure modes

Look for refusal inflation, evasive wording, generic answers, activation on
benign contexts, and movement to synonyms or later tokens. Capture raw output so
these failures are not hidden by a binary regex score.
