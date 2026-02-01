---
name: speq-ext-research
description: |
  External documentation and research via Context7 and WebSearch.
  Use for: library APIs, framework patterns, design questions, best practices.
  Triggers: library docs, API reference, external patterns, design research,
  "how does X library work", "what's the best practice for Y".
  Priority: Context7 for library APIs, WebSearch for design/architecture.
  Rule: Never assume library behavior—verify first.
---

# External Research

External documentation and research via Context7 and WebSearch.

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

```
Need library API details?
├─ Yes → Context7
└─ No  → Need design guidance?
         ├─ Yes → WebSearch
         └─ No  → Proceed with existing knowledge
```

## Example

Adding rate limiting to Express API:

```
1. resolve-library-id
   query: "Express.js rate limiting"
   libraryName: "express-rate-limit"

2. query-docs
   libraryId: "/nfriedly/express-rate-limit"
   query: "middleware setup options"

3. Implement using verified current API
```
