# Implementation Summary Template

Used by `/speq-implement` (Phase 7), which prints this as terminal output, and `/speq-implement-pr` (Phase C, "Ship"), which posts the same content as a PR comment. Same content either way; only the destination differs — `/speq-implement`'s own print is not a PR comment, `speq-implement-pr` composes its comment straight from `verification-report.md`, not by parsing the printed text. A reader gets the verdict and three tables in one pass; `verification-report.md` holds the full evidence trail — Tool Evidence, per-command logs, Notes — for anyone who wants it.

A terminal print has no `<details>` equivalent; the "Full evidence" line is already plain text either way, so nothing changes between the two destinations.

## Skeleton

```markdown
## Verdict
<PASS|FAIL>: <one sentence>

## Requirements Implemented

| Feature | Scenario | Status |
|---|---|---|
| <domain/feature> | <scenario> | ✅ |
| <domain/feature> | <scenario> | ✅ |

## Test Coverage

| Type | Run | Passed | Failed |
|---|---:|---:|---:|
| Unit | <n> | <n> | <n> |
| Integration | <n> | <n> | <n> |

## Verification Performed

| Check | Result |
|---|---|
| Build | ✅ |
| Lint | ✅ |
| Format | ✅ |
| Manual tests | ✅ (<n>/<n>) |

Code review: <n> findings, <n> fixed

Full evidence: `verification-report.md`
```

## Rules

- **Pull every number from `verification-report.md`; never estimate or recompute.** If a number the table needs isn't in the report, the report is incomplete — fix that first, don't guess here.
- **`Requirements Implemented` is the report's Scenario Coverage table, minus the Test Location/Test Name columns.** A reviewer approving the PR needs to know a scenario is proven, not which file proves it — that's one click away in the report.
- **One row per scenario, always.** A large plan makes this table long; that's still faster to scan than the same fact in prose, and a missing row is exactly the gap a quick read should catch. Do not collapse it into a scenario count.
- **Never paste the report's Notes section, Tool Evidence output, or a command log.** Link to the file. If a Note describes something a reviewer must weigh before approving (not just evidence that testing happened), it belongs in the PR body's `Impact` section instead, per `pr-body-template.md` — not repeated here.
- **`Verification Performed` mirrors the report's own Verdict-table checks** (Build/Tests/Lint/Format/Manual Tests), same rows, same order — this isn't a new checklist, it's that one restated for the PR.
