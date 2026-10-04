---
name: speq-cli
description: Query specs via the speq CLI: semantic search, feature listing, structure validation, and the absence check. Every speq orchestrator and sub-agent invokes this first for spec discovery, search, or validation.
---

# speq CLI

The CLI is installed locally and on the path. Invoke via `speq`.

## What Search Covers

`speq search query` is semantic. It ranks scenario names and steps by meaning. It does not cover feature descriptions, Backgrounds, or ADRs, and it can miss an exact identifier.

## Reading Specs

- Read a feature with `speq feature get <domain>/<feature>`. It prints the description, the Background, and the scenarios.
- Read the spec file directly when exact text matters. `speq feature get` does not print nested list items under a step or fenced code blocks inside a scenario.

## Command Reference

| Command | Purpose |
|---------|---------|
| `speq domain list` | List all domains |
| `speq feature list` | Tree view of all features |
| `speq feature list <domain>` | Features in a domain |
| `speq feature get <domain>/<feature>` | Full feature spec |
| `speq feature get '<domain>/<feature>/<scenario>'` | Single scenario |
| `speq search query "<query>"` | Semantic search |
| `speq search index` | Rebuild the search index |
| `speq decision-log show` | Show the accepted ADRs |
| `speq decision-log validate` | Validate the ADR fragments |
| `speq feature validate` | Validate all specs |
| `speq feature validate <domain>/<feature>` | Validate single feature |
| `speq plan list` | List active plans |
| `speq plan validate <plan>` | Validate a plan |
| `speq record <plan>` | Merge a validated plan into the permanent specs |

Single-quote a scenario path. In double quotes, the shell runs the text between backticks as a command.

## Absence Check

Before you state that no spec or decision covers a topic, run all three checks:

```bash
speq search query "<concept>"          # repeat with each synonym and exact term from the request
speq feature get <domain>/<feature>    # read each feature the search names or that is nearby
speq decision-log show                 # read the accepted ADRs for the topic
```

Report absence only when all three find nothing. Name the queries and terms you checked in the claim.
