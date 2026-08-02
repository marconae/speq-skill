---
name: speq-code-tools
description: Semantic code navigation and editing via Serena MCP — symbol-level tools required over grep/find/manual edits. Triggered by planner-agent, implementer-agent, implementer-expert-agent, code-reviewer, and recorder-agent.
---

# Code Tools

Semantic code navigation and editing via Serena MCP. Tools are called `mcp__plugin_speq_serena__<name>` (e.g. `mcp__plugin_speq_serena__find_symbol`); this skill refers to them by short name below.

## Before First Use Each Session

1. Call `initial_instructions` first, before any other Serena tool.
2. These tools are deferred in this harness — a bare tool name has no loaded schema yet. Call `ToolSearch` with `select:mcp__plugin_speq_serena__<name>` (comma-separated for several) to load the ones you need before calling them.

## Tool Preference

You MUST use these instead of the generic alternative — do not fall back to `Read`/`Grep`/`Edit`/`ls`/`find` for anything this table covers.

| Task | Use | Not |
|------|-----|-----|
| List directory | `list_dir` | `ls`, `find` |
| Find files | `find_file` | `find`, `rg --files` |
| File symbols | `get_symbols_overview` | `rg "class\|function"` |
| Symbol definition | `find_symbol` | `rg "function foo"` |
| Symbol declaration | `find_declaration` | `rg` guesswork |
| Symbol references | `find_referencing_symbols` | `rg "foo("` |
| Interface/method implementations | `find_implementations` | `rg "impl .* for"` |
| LSP diagnostics for a file | `get_diagnostics_for_file` | manual build/check |
| Update function | `replace_symbol_body` | read/edit/write |
| Insert after | `insert_after_symbol` | read/edit/write |
| Insert before | `insert_before_symbol` | read/edit/write |
| Rename symbol | `rename_symbol` | `rg` + manual edits |
| Delete symbol | `safe_delete_symbol` | manual delete + grep for refs |
| In-file text/regex replace | `replace_content` | read/edit/write |
| Multi-file pattern replace | `replace_in_files` | multiple manual edits |

Memory tools (`write_memory`, `read_memory`, `list_memories`, `delete_memory`, `edit_memory`, `rename_memory`) exist and are available, but this project's specs (`specs/`, `specs/_plans/.../notes/`) are the source of truth — they are not part of this workflow.

## Workflow

```
Session start → initial_instructions
Explore → find_symbol, get_symbols_overview, find_declaration
Understand → find_referencing_symbols, find_implementations
Edit → replace_symbol_body, insert_*_symbol, replace_content, replace_in_files
Verify → find_referencing_symbols, get_diagnostics_for_file
```
