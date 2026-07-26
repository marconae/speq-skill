# Code Review Findings Template

`code-reviewer` writes this document to `specs/_plans/<plan-name>/review-findings.md` and returns only a one-line verdict. Primary consumers: `implementer-agent` (§ Standard fixes) and `implementer-expert-agent` (§ Expert fixes), which derive fix tasks directly from the `Fix:` lines — no human or orchestrator summarizes this file first.

## Rules

1. Two top-level sections partition every finding: `## Standard fixes` and `## Expert fixes`, routed per `/speq-code-review` § Routing. An empty section keeps its heading with a single `[none]` line.
2. Within a section, group findings by file (`### <path>`). Every finding: `#### [TAG] <short title>` plus `Location`, `Issue`, `Fix`.
3. `Fix:` is an imperative instruction addressed to the consuming implementer agent — file path, symbol, concrete change — phrased so the agent can append it to `tasks.md` near-verbatim as a fix task and execute it without re-reading `Issue`.
4. Use the tag taxonomy from `/speq-code-review`; the category travels with the tag, not the section structure.

## Skeleton

```markdown
# Code Review Findings: <plan-name>

## Summary
- Files reviewed: <N>
- Total findings: <M> (standard: <X>, expert: <Y>)

## Standard fixes

### <path/to/file>

#### [<TAG>] <short title>
- Location: line <n>
- Issue: <defect, with evidence>
- Fix: <imperative instruction to implementer-agent>

## Expert fixes
[none]
```

## Verdict Line

After writing the document, return exactly one line and nothing else:

```
CODE REVIEW: <n> findings — standard: <n>, expert: <n> — specs/_plans/<plan-name>/review-findings.md
```

## Example

```markdown
## Standard fixes

### src/ingest/parser

#### [MAGIC_NUMBER] Retry count is a bare literal
- Location: line 42
- Issue: the retry loop in `parse_with_retry` bounds itself on the literal 3
- Fix: In src/ingest/parser, extract the literal 3 in `parse_with_retry` into a module-scope named constant MAX_PARSE_RETRIES and use it in the loop bound

## Expert fixes

### src/store/wal

#### [SIDE_EFFECT] Read path mutates the write-ahead-log index
- Location: line 118
- Issue: `lookup()` compacts the index as a side effect, racing concurrent writers
- Fix: In src/store/wal, move the compaction out of `lookup()` into the existing `maintain()` path, take the writer lock there, and add a stress test covering concurrent lookup/append
```
