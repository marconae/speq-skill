---
name: speq-cli
description: Query specs via speq CLI. Search scenarios, list features, validate structure.
---

# speq CLI

Spec-driven development via the speq CLI.

## Search-First Rule

**Never read full spec files without searching first.**

Benefits:
- Reduces context window usage
- Finds relevant content faster
- Avoids loading irrelevant scenarios

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

## Example

Finding error handling scenarios:

```bash
# Search for related scenarios
speq search query "error handling validation"

# Get specific scenario
speq feature get "cli/validate/Validation fails on missing field"

# Validate structure
speq feature validate cli/validate
```

## Anti-Pattern

```
# Wrong: Loading entire spec files
cat specs/cli/validate/spec.md

# Right: Search and get specific
speq search query "validation"
speq feature get "cli/validate/Required field missing"
```
