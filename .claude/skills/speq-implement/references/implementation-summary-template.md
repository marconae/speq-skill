# Implementation Summary Template

`/speq-implement` prints this in Phase 7. `/speq-implement-pr` posts it as a PR comment in Phase C, composed from `verification-report.md` and not from the printed text. The comment summarizes the tests run and the test evidence collected. The report keeps the full evidence trail.

## Skeleton

```markdown
## Verdict
<PASS|FAIL>: <one sentence>

## Requirements Implemented

| Feature | Scenario | Status |
|---|---|---|
| <domain/feature> | <scenario> | ✅ |
| <domain/feature> | <scenario> | ✅ |

## Tests Run

| Type | Run | Passed | Ignored | Coverage |
|---|---:|---:|---:|---:|
| Unit | <n> | <n> | <n> | <n>% |
| Integration | <n> | <n> | <n> | <n>% |

## Test Evidence

| Check | Result |
|---|---|
| Build | ✅ |
| Lint | ✅ |
| Format | ✅ |
| Manual test: <what was run> | ✅ <observed output, one line> |

Code review: <n> findings, <n> fixed
```

## Rules

- **Pull every number from `verification-report.md`; never estimate or recompute.** If a number the table needs isn't in the report, the report is incomplete — fix that first, don't guess here.
- **`Requirements Implemented` is the report's Scenario Coverage table, minus the Test Location/Test Name columns.** A reviewer approving the PR needs to know a scenario is proven, not which file proves it — that's one click away in the report.
- **One row per scenario, always.** A large plan makes this table long; that's still faster to scan than the same fact in prose, and a missing row is exactly the gap a quick read should catch. Do not collapse it into a scenario count.
- **Report tests and test evidence only.** State which tests ran, their results, and what the manual tests observed. Never mention `speq` commands that check the plan or specs, such as `speq plan validate` or `speq feature get`. They belong to the workflow, not to the software under test. A command that exercises the software itself is test evidence and stays.
- **Never paste the report's Notes section, Tool Evidence output, or a command log.** If a Note describes something a reviewer must weigh before approving, state it in one line under the Verdict.
- **`Test Evidence` mirrors the report's own checks** (Build, Lint, Format, Manual Tests), same rows, same order. Add one Manual test row per manual test, with the observed output in a few words.
