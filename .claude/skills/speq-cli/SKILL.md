---
name: speq-cli
description: Query specs via the speq CLI: feature listing and reading, structure validation, and the absence check. Every speq orchestrator and sub-agent invokes this first for spec discovery or validation.
---

# speq CLI

The CLI is installed locally and on the path. Invoke via `speq`.

## Reading Specs

- Find a spec by listing the features with `speq feature list`, then read the relevant ones with `speq feature get`.
- Read a feature with `speq feature get <domain>/<feature>`. It prints the description, the Background, and the scenarios.
- Read the spec file directly only when exact text matters, and only the lines you need (`Read` with offset and limit). `speq feature get` does not print nested list items under a step or fenced code blocks inside a scenario.
- Find before you read: `speq feature get '<domain>/<feature>/<scenario>'` for one scenario. Never `cat` a whole spec.

## Command Reference

| Command | Purpose |
|---------|---------|
| `speq domain list` | List all domains |
| `speq feature list` | Tree view of all features |
| `speq feature list <domain>` | Features in a domain |
| `speq feature get <domain>/<feature>` | Full feature spec |
| `speq feature get '<domain>/<feature>/<scenario>'` | Single scenario |
| `speq decision-log show` | Show every accepted ADR (large in a mature repo; prefer the ADR digest below) |
| `speq decision-log validate` | Validate the ADR fragments |
| `speq feature validate` | Validate all specs |
| `speq feature validate <domain>/<feature>` | Validate single feature |
| `speq plan list` | List active plans |
| `speq plan validate <plan>` | Validate a plan |
| `speq record <plan>` | Merge a validated plan into the permanent specs |

Single-quote a scenario path. In double quotes, the shell runs the text between backticks as a command.

## Absence Check

Before you state that no spec or decision covers a topic, run all three steps:

1. List the features with `speq feature list`. Note each feature whose name relates to the topic or a synonym of it.
2. Read those candidate features or their scenarios with `speq feature get`.
3. Check the ADR digest (next section). Read only the fragments that match the topic.

Report absence only when all three steps find nothing. Name the features, scenarios, and ADRs you checked in the claim.

## ADR Digest

Do not dump the whole decision log. List the accepted ADRs once per run:

```bash
grep -rhE -A3 '^## ADR:|^\*\*(ID|Status):\*\*' specs/_decision/ 2>/dev/null || echo none
```

It prints each ADR's title, ID, status, and Decision line. Read a full fragment only when its title or a term grep matches the topic. Run `speq decision-log show` at most once per run, and never pipe it through repeated `sed` or `grep` passes.
