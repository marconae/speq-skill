---
name: speq-code-tools
description: Semantic code navigation and editing via Serena MCP — symbol-level tools required over grep/find/manual edits. Triggered by planner-agent, implementer-agent, implementer-expert-agent, code-reviewer, recorder-agent, and architecture-agent.
---

# Code Tools

Semantic code navigation and editing via Serena MCP, if available. The user installs Serena globally, so the tool prefix varies (for example `mcp__serena__find_symbol`). This skill refers to the tools by short name below.

## Before First Use Each Session

1. Call `initial_instructions` first, before any other Serena tool.
2. These tools are deferred in this harness — a bare tool name has no loaded schema yet. Call `ToolSearch` with the query `serena` to find them, then load the ones you need (`select:<full tool name>`, comma-separated for several) before calling them.
3. If `ToolSearch` finds no Serena tools, carry on with the generic tools (`Read`, `Grep`, `Edit`). Do not warn and do not stop.

## Tool Preference

When Serena is available, use these instead of the generic alternative for anything this table covers. Symbol-level tools return exact definitions and references and edit whole symbols, where text search and line edits miss or break them.

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
