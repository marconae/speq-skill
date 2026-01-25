# Task Flow

Task lifecycle management for implementation work.

## Task Lifecycle

```
pending → in_progress → completed
```

| Status | Meaning |
|--------|---------|
| `pending` | Task created, not started |
| `in_progress` | Currently working on task |
| `completed` | Task finished with evidence |

## Creating Tasks

From plan's `## Implementation Tasks`, create one task per item:

```
TaskCreate(
  subject: "Implement user authentication",
  description: "Add login endpoint with JWT tokens per spec",
  activeForm: "Implementing user authentication"
)
```

### Task Naming

| Part | Format | Example |
|------|--------|---------|
| subject | Imperative verb phrase | "Add validation for email field" |
| activeForm | Present continuous | "Adding validation for email field" |
| description | Context and acceptance criteria | "Per spec scenario X, must..." |

## Working Tasks

### Starting Work

```
TaskUpdate(taskId: "1", status: "in_progress")
```

### Completing Work

Only mark complete when:
- TDD cycle finished (RED-GREEN-REFACTOR)
- Evidence shown for all claims
- Tests passing (with output)

```
TaskUpdate(taskId: "1", status: "completed")
```

### If Blocked

Do NOT mark as completed. Instead:
- Keep as `in_progress`
- Create new task for blocker
- Add dependency if needed

## Parallel Groups

From plan's `## Parallelization` section:

### Independent Tasks

Tasks with no dependencies can run in parallel:

```
| Parallel Group | Tasks |
|----------------|-------|
| Group A | Task 1, Task 2 |  ← Run concurrently
| Group B | Task 3, Task 4 |  ← Run after Group A
```

### Sequential Dependencies

When task B depends on task A:

```
TaskUpdate(taskId: "B", addBlockedBy: ["A"])
```

Task B cannot start until Task A completes.

## Task Checklist

Before marking any task complete:

- [ ] Read task requirements
- [ ] Search for existing code
- [ ] Write failing test (RED)
- [ ] Show test failure output
- [ ] Write minimal code (GREEN)
- [ ] Show test pass output
- [ ] Refactor if needed
- [ ] Show all tests pass
- [ ] Show lint passes
- [ ] Update task status

## Error Handling

| Situation | Action |
|-----------|--------|
| Task unclear | Re-read plan, ask user if needed |
| Test won't fail | Check test is actually running |
| Test won't pass | Debug implementation |
| Other tests break | Fix regression before continuing |
| Blocked by dependency | Work on unblocked tasks first |

## Progress Tracking

Use `TaskList` to see current state:

```
TaskList()
→ Shows all tasks with status
→ Identify next pending task
→ Check for blocked tasks
```

After completing a task:

```
1. TaskUpdate(taskId, status: "completed")
2. TaskList()  # Find next task
3. TaskUpdate(nextTaskId, status: "in_progress")
4. Begin TDD cycle
```
