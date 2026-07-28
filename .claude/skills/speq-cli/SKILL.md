---
name: speq-cli
description: Query specs via the speq CLI — semantic search, feature listing, structure validation. Use before reading any spec file; every speq orchestrator and sub-agent invokes this first for spec discovery, search, or validation.
---

# speq CLI

The CLI is installed locally and on the path. Invoke via `speq`.

## Search-First Rule

**Never read full spec files without searching first.** Search narrows the target before you spend context budget.

## Command Reference

| Command | Purpose |
|---------|---------|
| `speq domain list` | List all domains |
| `speq feature list` | Tree view of all features |
| `speq feature list <domain>` | Features in a domain |
| `speq feature get <domain>/<feature>` | Full feature spec |
| `speq feature get "<domain>/<feature>/<scenario>"` | Single scenario |
| `speq search query "<query>"` | Semantic search |
| `speq feature validate` | Validate all specs |
| `speq feature validate <domain>/<feature>` | Validate single feature |

## Workflow

```
1. Search first
   speq search query "<relevant terms>"

2. Get specific content
   speq feature get "<domain>/<feature>/<scenario>"

3. Validate after changes
   speq feature validate
```

## Anti-Pattern

```
# Wrong: Loading entire spec files
cat specs/cli/validate/spec.md

# Right: Search and get specific
speq search query "validation"
speq feature get "cli/validate/Required field missing"
```
