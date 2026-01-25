---
name: External Documentation
description: Library and API documentation via Context7 MCP
---

# External Documentation (Context7)

Current library docs and API references. **Prefer over** training data, assumptions, or web search.

## When to Use

- API signatures and method parameters
- Library usage examples
- Framework patterns and conventions
- Verifying current behavior (vs. outdated knowledge)

## Rule

**Never assume** library behavior—verify with Context7 first.

## Example

```
User: "Add rate limiting to the Express API"

1. resolve-library-id → query: "Express.js rate limiting", libraryName: "express-rate-limit"
2. query-docs → libraryId: "/nfriedly/express-rate-limit", query: "middleware setup options"
3. Implement using verified current API
```
