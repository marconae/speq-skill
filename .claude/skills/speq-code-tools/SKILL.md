---
name: speq-code-tools
description: |
  Semantic code navigation and editing via Serena MCP.
  Use for: exploring codebase structure, finding symbol definitions,
  understanding references, refactoring, inserting/replacing code.
  Triggers: code navigation, find symbol, symbol references, code editing,
  refactoring, list directory, file search, code exploration.
  Prefer over: grep, find, ast-grep, read/edit/write cycles.
---

# Code Tools

Semantic code navigation and editing via Serena MCP.

## Tool Preference

| Task | Use | Not |
|------|-----|-----|
| List directory | `list_dir` | `ls`, `find` |
| Find files | `find_file` | `find`, `rg --files` |
| File symbols | `get_symbols_overview` | `rg "class\|function"` |
| Symbol definition | `find_symbol` | `rg "function foo"` |
| Symbol references | `find_referencing_symbols` | `rg "foo("` |
| Update function | `replace_symbol_body` | read/edit/write |
| Insert after | `insert_after_symbol` | read/edit/write |
| Insert before | `insert_before_symbol` | read/edit/write |
| Rename symbol | `rename_symbol` | `rg` + manual edits |

## Reflection Checkpoints

Call these tools at key moments:

| Tool | When |
|------|------|
| `think_about_collected_information` | After exploration sequence |
| `think_about_task_adherence` | Before writing code |
| `think_about_whether_you_are_done` | Before reporting completion |

## Workflow

```
Explore → find_symbol, get_symbols_overview
Understand → find_referencing_symbols
Reflect → think_about_collected_information
Edit → replace_symbol_body, insert_*_symbol
Verify → find_referencing_symbols
Complete → think_about_whether_you_are_done
```
