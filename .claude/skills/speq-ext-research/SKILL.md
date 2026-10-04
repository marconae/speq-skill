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

Use Context7 if available. User installs it globally, so its tool prefix varies. Find the tools with `ToolSearch` (query `context7`) and load them before calling. If none exist, skip Context7 and use WebSearch. Do not warn and do not stop.

```
1. resolve-library-id
   query: "<what you need>"
   libraryName: "<library name>"

2. query-docs
   libraryId: "<from step 1>"
   query: "<specific question>"
```

Prefer primary documentation over secondary commentary. Research only when existing knowledge does not settle the question.
