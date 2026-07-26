---
name: speq-design-philosophy
description: Complexity-management design principles — deep modules, information hiding and leakage, module boundaries, dependency direction, strategic vs. tactical tradeoffs. Triggered by planner-agent, plan-reviewer, implementer-agent, implementer-expert-agent, and code-reviewer before any design, implementation, or review work.
---

# Design Philosophy

**A Philosophy of Software Design** (Ousterhout) complexity-management framework, extended with **Clean Architecture**'s (Martin) dependency rule.

## Core Principle

Complexity is what erodes a system's understandability over time. Weigh every design decision by whether it adds to that erosion or reduces it — not by line count, module count, or whether it follows a named pattern.

## Deep Modules

Weigh a module by how much it does for callers relative to what it costs them to learn. A deep one earns that cost back many times over — its interface is far easier to use than its internals would be to rebuild from scratch. A shallow one barely earns it back at all: learning the interface takes almost as much effort as writing the equivalent code yourself.

| Signal | Fix |
|--------|-----|
| Interface as complex as a caller would write themselves | Deepen it — absorb more complexity, expose less |
| Many small modules named for a role, not a responsibility (`-Manager`, `-Handler`, `-Processor`) | "Classitis" — merge related shallow modules into one deeper one |
| A function whose entire body is a call to another function with the same arguments | Merge it into whichever side actually holds logic |

Small is not automatically good. Depth, not size, decides whether an abstraction earns its place.

## Information Hiding & Leakage

A module is well-designed when the rest of the system stays ignorant of one particular decision it makes internally. Trouble starts when that same decision surfaces again somewhere else in the codebase — among every kind of defect, this recurrence deserves the closest attention.

- **Temporal leakage** — organizing modules around execution order (read, then parse, then write) rather than around what each stage actually knows means every stage ends up carrying the same format knowledge.
- **Back-door leakage** — two modules independently assume the same data format, protocol, or convention with nothing enforcing agreement between them.

Fix: combine the modules that share the decision, or give the decision its own home and have both existing modules depend on it.

## General- vs Special-Purpose Modules

The governing test: an interface should handle every need the code has *today* (see `/speq-code-guardrails`' YAGNI Checks for the "not tomorrow's" half) without forcing a pile of narrow, special-case methods onto callers.

A configuration parameter is a decision the module declined to make. Prefer a sensible default, auto-detection, or elimination over adding one.

## Strategic vs Tactical Programming

Tactical: ship the feature, leave the module harder to work with next time. Strategic: invest roughly 10-20% of the time in design quality as part of every change, not as a separate cleanup phase. A pattern of shipping fastest by leaving every touched module harder to work with is a risk to flag, not a contribution to emulate.

## Comments as Design Intent

Per `/speq-code-guardrails`' Comments rule — a public/interface doc comment states design intent, not just purpose. Struggling to write that comment is itself a signal: usually the abstraction underneath doesn't have a coherent shape yet. This doesn't relax the ban on inline or private-method comments.

## Dependencies & Boundaries

Per Martin's *Clean Architecture*:

- Dependencies point from volatile to stable. Business logic never names a delivery mechanism, storage engine, or framework.
- Business logic performs no I/O and reads no ambient state — both are injected.
- The consumer defines the abstraction it needs, in its own vocabulary — never shaped around a provider's API (Dependency Inversion).
- Construct concrete implementations only at the entry point; no module builds its own dependencies.
- No cycles in the module dependency graph.
- Group modules by reason to change, not by technical role.
- Extend by adding a case, not by editing a dispatch (Open/Closed).

## Quick Diagnostic

No silent pass when reviewing a design — a new module, interface, or boundary. A change that introduces none of these may skip this table; answer every question for anything that does, and a "no" names the fix.

| Question | If no |
|----------|-------|
| Does a one-sentence summary capture what a module is responsible for? | It's doing too much, or nothing coherent — split it |
| Is calling the module noticeably easier than reimplementing it would be? | The interface costs more than it saves — hide more, expose less |
| Would changing how a module works internally force an edit anywhere outside it? | A decision has leaked across a boundary — pull it into one owning module |
| Does a public doc comment explain the reasoning behind an abstraction, not only restate its name? | Rewrite it — resistance to writing it usually means the design needs rework |
| Is there exactly one module that owns each significant design decision? | Reorganize by what code knows, not by when it executes |
| Could someone unfamiliar with the codebase tell where one module ends and the next begins, without reading either's internals? | Simplify the interfaces, or state the boundary in the interface doc comment |
| Does any tactical shortcut in this change have a scheduled follow-up to revisit it? | Schedule one, or reconsider taking the shortcut at all |
| Does business logic depend only inward — never on a delivery mechanism, storage engine, or framework? | Invert the dependency; the consumer defines the abstraction |

## Attribution

Concepts from John Ousterhout's *A Philosophy of Software Design* and Robert C. Martin's *Clean Architecture*, adapted here. Inspired by the `wondelai/skills` collection.
