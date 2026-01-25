---
name: spec-recorder
description: |
  Apply approved plan deltas to permanent feature specs.
  Use after a plan is approved to merge deltas from specs/_plans/<plan-name>/ into specs/.
  Invoke explicitly with /spec-recorder <plan-name> to record approved changes.
  Handles NEW, CHANGED, and REMOVED scenarios from delta specs.
---

# Spec Recorder

Merge plan deltas into permanent specs, optimize spec library, archive completed plans.

Get the plan name from user prompt or ask if none specified.

## Workflow Position

```
/spec-planner → /spec-implementer → /spec-recorder
     │                  │                  │
     ▼                  ▼                  ▼
Creates plan.md    Implements code    Merges deltas
with deltas        + verification     to permanent specs
```

## 6-Phase Workflow

### Phase 1: Verify Implementation

```
Check: specs/_plans/<plan-name>/verification-report.md exists?
├─ Yes → Proceed
└─ No  → STOP: "Plan not implemented. Run /spec-implementer <plan-name> first."
```

Read verification report to confirm implementation is complete.

### Phase 2: Load Plan

```bash
# Discover existing spec library
speq domain list              # List all domains
speq feature list             # List all features
speq feature list <domain>    # List features in domain

# Load plan
Read: specs/_plans/<plan-name>/plan.md
List: specs/_plans/<plan-name>/**/spec.md
```

Extract:
- Feature deltas to merge
- Mission changes (if any)

### Phase 3: Apply Deltas

For each delta spec in `specs/_plans/<plan-name>/<domain>/<feature>/spec.md`:

```
Check: specs/<domain>/<feature>/spec.md exists?
├─ No  → Copy delta (strip markers)
└─ Yes → Merge using delta markers
```

#### Delta Markers

| Marker | Action |
|--------|--------|
| `DELTA:NEW` | Append scenario to Scenarios section |
| `DELTA:CHANGED` | Replace scenario with same name |
| `DELTA:REMOVED` | Delete scenario with same name |

#### Merge Steps

1. Parse delta for marked scenarios
2. Apply each operation
3. Strip all `<!-- DELTA:* -->` markers
4. Validate: `speq feature validate <domain>/<feature>`

### Phase 4: Optimize Spec Library

After merging, analyze spec library health:

```bash
speq domain list              # Review domain organization
speq feature list             # Review feature distribution
```

#### Size Thresholds

| Metric | Threshold | Action |
|--------|-----------|--------|
| Scenarios per spec | >10 | Suggest split |
| Spec file size | >500 lines | Suggest split |
| Domain breadth | >8 features | Suggest sub-domains |

#### Split Strategy

When thresholds exceeded, use `AskUserQuestion`:

```
"Feature X now has 12 scenarios. Suggest splitting into:
  - X-core (basic operations)
  - X-advanced (complex workflows)
Proceed with split?"
```

**Never assume** — always confirm splits with user.

#### Domain Organization

Specs must be meaningfully organized:
- Domain = logical grouping (e.g., `cli`, `auth`, `core`)
- Feature = specific capability (e.g., `validate`, `search`)

If organization unclear, ask:
```
"Where should feature X belong?
  - Existing domain: cli
  - Existing domain: core
  - New domain: <suggest>"
```

### Phase 5: Mission Update

Check plan.md for mission impact:

```
Plan mentions mission/scope change?
├─ Yes → Delegate to /mission-creator for update
└─ No  → Skip
```

### Phase 6: Finalize

1. **Validate entire library:**
   ```bash
   speq validate
   ```

2. **Archive plan:**
   ```bash
   mkdir -p specs/_recorded
   mv specs/_plans/<plan-name> specs/_recorded/<plan-name>
   ```

3. **Confirm completion:**
   ```
   ✓ Verification report confirmed
   ✓ All deltas merged
   ✓ Spec library validated
   ✓ Plan archived to specs/_recorded/<plan-name>/
   ```

## Spec Hierarchy

```
specs/
├── mission.md                    # Project mission
├── <domain>/
│   └── <feature>/
│       └── spec.md               # Permanent specs
├── _plans/
│   └── <plan-name>/
│       ├── plan.md
│       ├── verification-report.md
│       └── <domain>/<feature>/spec.md
└── _recorded/
    └── <plan-name>/              # Archived plans
```

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Record without verification report | Implementation not proven |
| Assume domain/split decisions | User must confirm organization |
| Skip speq validation | Broken specs may result |
| Leave DELTA markers | Pollutes permanent specs |
