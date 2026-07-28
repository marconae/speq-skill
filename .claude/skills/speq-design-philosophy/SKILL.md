---
name: speq-design-philosophy
description: Complexity-management design principles — deep modules, information hiding and leakage, module boundaries, dependency direction, strategic vs. tactical tradeoffs. Triggered by planner-agent, plan-reviewer, implementer-agent, implementer-expert-agent, and code-reviewer before any design, implementation, or review work.
---

# Design Philosophy

**A Philosophy of Software Design** (Ousterhout) complexity-management framework, extended with **Clean Architecture**'s (Martin) dependency rule.

## Core Principle

Complexity is what erodes a system's understandability over time. Judge every design decision by whether it adds or removes complexity. Do not judge by line count, module count, or named patterns.

## Deep Modules

Weigh a module by what it does for callers relative to what its interface costs them to learn. Deep: the interface is far easier to use than the internals would be to rebuild. Shallow: learning the interface costs almost as much as writing the code yourself.

| Signal | Fix |
|--------|-----|
| Interface as complex as a caller would write themselves | Deepen it — absorb more complexity, expose less |
| Many small modules named for a role, not a responsibility (`-Manager`, `-Handler`, `-Processor`) | "Classitis" — merge related shallow modules into one deeper one |
| A function whose entire body is a call to another function with the same arguments | Merge it into whichever side actually holds logic |

Depth, not size, decides whether an abstraction earns its place.

## Information Hiding & Leakage

A module is well designed when the rest of the system stays ignorant of a decision it makes internally. Leakage: the same decision surfaces in more than one module. Treat leakage as the defect class that deserves the closest attention.

- **Temporal leakage**: modules organized around execution order (read, then parse, then write) instead of around what each stage knows. Every stage then carries the same format knowledge.
- **Back-door leakage**: two modules independently assume the same data format, protocol, or convention, with nothing enforcing agreement.

Fix: combine the modules that share the decision, or give the decision its own module and make both depend on it.

## General- vs Special-Purpose Modules

Test: an interface handles every need the code has *today* (see `/speq-code-guardrails`' YAGNI Checks for the "not tomorrow's" half) without forcing narrow, special-case methods onto callers.

A configuration parameter is a decision the module declined to make. Prefer a sensible default, auto-detection, or elimination.

## Strategic vs Tactical Programming

Tactical: ship the feature, leave the module harder to work with. Strategic: invest 10-20% of every change in design quality, not a separate cleanup phase. Flag a pattern of tactical shipping as a risk. Do not emulate it.

## Comments as Design Intent

Per `/speq-code-guardrails`' Comments rule: a public/interface doc comment states design intent, not just purpose. If the comment is hard to write, the abstraction usually has no coherent shape yet. This does not relax the ban on inline or private-method comments.

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

If a change introduces a new module, interface, or boundary, answer every question. No silent pass. A "no" names the fix. A change that introduces none of these can skip the table.

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
