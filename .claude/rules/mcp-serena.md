---
name: Code Comprehension
description: Semantic code navigation and editing via Serena MCP
---

# Code Comprehension (Serena)

Use Serena first for code navigation, Semantic understanding and editing.
**Always prefer over** grep, find, ast-grep, or read/edit/write cycles.

## Tools

| Task | Use | Not |
|------|-----|-----|
| List directory | `list_dir` | `ls`, `find` |
| Find files | `find_file` | `find`, `rg --files` |
| File symbols | `get_symbols_overview` | `rg "class\|function"` |
| Symbol definition | `find_symbol` | `rg "function foo"` |
| Symbol references | `find_referencing_symbols` | `rg "foo("` |
| Update function | `replace_symbol_body` | read → edit → write |
| Insert after | `insert_after_symbol` | read → edit → write |
| Insert before | `insert_before_symbol` | read → edit → write |
| Rename symbol | `rename_symbol` | `rg` + manual edits |

## Reflection Checkpoints

- `think_about_collected_information` — after exploration
- `think_about_task_adherence` — during implementation
- `think_about_whether_you_are_done` — before completion

## Workflow

```
Explore → find_symbol, get_symbols_overview
Understand → find_referencing_symbols
Reflect → think_about_collected_information
Edit → replace_symbol_body, insert_*_symbol
Verify → find_referencing_symbols
Complete → think_about_whether_you_are_done
```
