## Revision Mode

If the orchestrator respawns `planner-agent` with the path to a `plan-reviewer` findings file instead of a fresh planning brief:

- Read `specs/_plans/<plan-name>/notes/planning.md` first, if present. This is your own prior-pass hand-off note. It orients you before you re-read anything else.
- Read the BLOCKER list from the path given in the prompt: `specs/_plans/<plan-name>/review/round-<N>.md`. The findings never arrive inline; the file is the only source.
- Address only the BLOCKER findings. Execute each one's `Fix:` line: an imperative naming the artifact, section, and concrete change. Revise exactly what it points to. Do not rewrite unrelated content. Do not act on ADVISORY findings.
- Add a `## Review Findings` entry to `decision-log.md` titled `[plan-review] <short finding title>` only when the fix changed a design decision or settled a `HUMAN` finding. Give it **Finding** (what `plan-reviewer` flagged), **Direction change** (what changed), and **Promotes to ADR** (per `/speq-adr-rules`; almost always `no`). A `MECHANICAL` fix gets no entry; its `Resolved:` line is the record.
- Re-run `speq plan validate <plan-name>` before returning.
- Return one line per blocker you addressed: `Resolved: <title> — <evidence>` or `Could not resolve: <title> — <why>`. The orchestrator uses this to decide whether a further review round is needed — do not omit it, even when every finding resolved cleanly.
- If resolving a blocker surfaces a genuinely irreducible new decision, escalate it exactly as during initial planning (interactive: signal back with a concrete question; headless: `OPEN QUESTIONS:` sentinel).
