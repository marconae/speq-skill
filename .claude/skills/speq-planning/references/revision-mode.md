## Revision Mode

If the orchestrator respawns `planner-agent` with the path to a feedback file instead of a fresh planning brief. The file is either a `plan-reviewer` findings file (`review/review.md`) or PR comments the user left (`notes/pr-feedback.md`):

- Read `specs/_plans/<plan-name>/notes/planning.md` first, if present. This is your own prior-pass hand-off note. It orients you before you re-read anything else.
- Read the feedback from the path given in the prompt. It never arrives inline; the file is the only source.
- **Review findings:** address only the BLOCKER findings. Execute each one's `Fix:` line: an imperative naming the artifact, section, and concrete change. Revise exactly what it points to. Do not rewrite unrelated content. Do not act on FINDINGS-severity findings.
- **PR comments:** treat each comment as user feedback on the plan files. Make the change it asks for, or report `Could not resolve:` when it conflicts with the user's intent or another comment.
- Add a `## Review Findings` entry to `decision-log.md` titled `[plan-review] <short finding title>` only when the fix changed a design decision. Give it **Finding** (what `plan-reviewer` flagged), **Direction change** (what changed), and **Promotes to ADR** (per `/speq-adr-rules`; almost always `no`). Any other fix gets no entry; its `Resolved:` line is the record.
- Re-run `speq plan validate <plan-name>` before returning.
- Return one line per blocker or comment you addressed: `Resolved: <title> — <evidence>` or `Could not resolve: <title> — <why>`. The orchestrator uses this to decide what goes to the user — do not omit it, even when every finding resolved cleanly.
- If resolving a blocker surfaces a genuinely irreducible new decision, escalate it exactly as during initial planning (signal back to the orchestrator with a concrete question).
