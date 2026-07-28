---
name: speq-ext-research
description: External documentation and research via Context7 and WebSearch — library APIs and design patterns. Triggered by /speq-mission, planner-agent, implementer-agent, and implementer-expert-agent when current external sources are needed.
---

# External Research

## When to Use

| Source | Use For |
|--------|---------|
| Context7 | Library APIs, method signatures, usage examples |
| WebSearch | Design patterns, architecture decisions, best practices |

## Context7 Workflow

```
1. resolve-library-id
   query: "<what you need>"
   libraryName: "<library name>"

2. query-docs
   libraryId: "<from step 1>"
   query: "<specific question>"
```

## WebSearch Workflow

```
1. WebSearch(query: "<design question>")
2. Extract relevant patterns
3. Apply to implementation
```

## Priority Decision

Prefer primary documentation over secondary commentary:

```
Need library API details?
├─ Yes → Context7
└─ No  → Need design guidance?
         ├─ Yes → WebSearch
         └─ No  → Proceed with existing knowledge
```
