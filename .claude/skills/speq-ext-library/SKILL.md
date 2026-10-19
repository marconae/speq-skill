---
name: speq-ext-library
description: "Looks up current library documentation (APIs, method signatures, usage examples) through Context7 when it is installed. Use when a task depends on a library's current API, version-specific behavior, or setup."
---

# Library Research

Context7 (MCP) provides the documentation of external libraries. Use it when it is installed. The user installs it globally, so its tool prefix varies. Find the tools (e.g., with `ToolSearch`) (query `context7`) and load them before calling. If none exist, skip this skill.

```
1. resolve-library-id
   query: "<what you need>"
   libraryName: "<library name>"

2. query-docs
   libraryId: "<from step 1>"
   query: "<specific question>"
```
