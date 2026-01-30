# Project Management System - Technical Architecture

## Overview

A project management system built around a Noun/Verb/Container grammar architecture with event sourcing, multi-database federation, and flexible deployment modes.

---

## Core Concepts

### Design Principles
- **Simplest wins**: Flat structures, junction tables, pass-through logging
- **Event sourced**: Every action creates an immutable Transaction
- **SOW as genesis**: Statement of Work creates and manages Levels
- **Grammar-based**: Consistent Noun/Verb patterns across all interactions

### Deployment Modes

| Mode | Description | Features |
|------|-------------|----------|
| Skinny | Single DB, hidden SOW | Compete with MS Project |
| Standard | Single DB per org | Full Level management |
| Enterprise | Multi-DB federation | SOW-level DB isolation |

---

## Nouns (12 Types)

### Regular Nouns

| Type | Abbrev | Description | Can Have Children |
|------|--------|-------------|-------------------|
| SOW | SOW | Genesis noun, creates Level | ✅ |
| Item | ITM | Organizational bucket | ✅ |
| Task | TASK | Work items | ✅ |
| Request | REQ | Approval/action requests | ✅ |
| Meeting | MTG | Scheduled gatherings | ✅ |
| Deliverable | DEL | Outputs/outcomes | ✅ |
| Event | EVT | Occurrences/launches | ✅ |
| Blocker | BLK | Impediments | ✅ |
| Artifact | ART | Files/documents (leaf only) | ❌ |

### Container Nouns

| Type | Abbrev | Description | Goals | Auto-Complete |
|------|--------|-------------|-------|---------------|
| Group | GRP | Organization, no goals | ❌ | ❌ |
| Project | PRJ | Has goals (Deliverables/Events) | ✅ | ❌ |
| MileStone | MLS | Checkpoint, auto-completes when all assigned complete | ❌ | ✅ |

---

## Verbs (12 Types)

| Verb | Description | Undone By |
|------|-------------|-----------|
| Open | Create new Noun | - |
| Complete | Mark finished | Normal |
| Incomplete | Mark cannot complete | Normal |
| Normal | Reset state | - |
| Escalate | Mark urgent | Normal |
| Close | Close and archive | Open (clones) |
| Update | Modify attributes | - |
| Reparent | Move to SOW root | - |
| Assign | Add to Container | Unassign |
| Unassign | Remove from Container | Assign |
| Blocked | Link Blocker to target | Release |
| Release | Remove block link | Blocked |

---

## State Machine

```
                    ┌─────────────┐
         Open() ──→ │   Normal    │←──────────────┐
                    └─────────────┘               │
                      │         │                 │
            Escalate()│         │Complete()       │Normal()
                      ▼         │Incomplete()     │
                    ┌─────────────┐    (if not blocked)
                    │  Escalated  │───────────┐   │
                    └─────────────┘           │   │
                          │                   ▼   │
                          │Normal()    ┌─────────────┐
                          └──────────→ │ Completed/  │
                                       │ Incompleted │
                                       └─────────────┘
                                              │
                                         Close()
                                              ▼
                                       ┌─────────────┐
                                       │   Closed    │
                                       └─────────────┘
                                              │
                                         (n days)
                                              ▼
                                       ┌─────────────┐
                                       │  Archived   │ (terminal)
                                       └─────────────┘
```

### State Guards

| Guard | Rule |
|-------|------|
| Most verbs | Noun must be Normal or Escalated |
| Complete/Incomplete | Noun must not be blocked |
| Close | Noun must be Completed or Incompleted |
| Close | All children must be Closed |

---

## Relationships (5 Types)

| Relationship | Cardinality | Storage | Description |
|--------------|-------------|---------|-------------|
| Parent-Child | 1:N | `Noun.parent_id` | Tree hierarchy, SOW is root |
| Actor Roles | M:N | `NounActor` | owner, assignee, awareness, sme, stakeholder, resource |
| Container Assignment | M:N | `NounAssignment` | Nouns assigned to Containers with sort order |
| Blocking | M:N | `NounBlock` | Blocker → Target links |
| HashTags | M:N | `NounHashTag` | Global tags for organization |

---

## Blocking Propagation

### Direction Rules

| Direction | Mechanism |
|-----------|-----------|
| Downward | Blocker.Blocked(Noun) affects Noun + all children |
| Upward (child) | Blocked child → parent is_blocked = true |
| Upward (assignment) | Blocked assigned Noun → Container is_blocked = true |
| Goal boundary | Blocked Goal → Project.blocked_goals (NOT Project.is_blocked) |

### Computation

```
Noun.is_blocked = 
  has active Blocker targeting me
  OR any child is_blocked
  OR any assigned Noun is_blocked (Containers only)
```

Leadership sees: "2 of 5 Goals blocked" not "Project blocked"

---

## Database Schema

### Noun

```sql
Noun {
  id: uuid (PK)
  type: enum (SOW, Item, Task, Request, Meeting, Deliverable, Event, Blocker, Artifact, Group, Project, MileStone)
  short_name: string (globally unique, auto-generated, user-editable)
  parent_id: uuid (FK → Noun)
  sow_id: uuid (FK → Noun, for DB routing)
  
  title: string
  description: text
  state: enum (Normal, Escalated, Completed, Incompleted, Closed)
  
  created_at: timestamp
  due_date: timestamp
  completed_at: timestamp
  closed_at: timestamp
  
  is_blocked: boolean (computed)
  
  custom_fields: jsonb (flat key-value)
  roles: jsonb (SOW only - allow/deny rules)
}
```

### Short Name Generation

Format: `{SOW.short_name}-{TYPE_ABBREV}-{SEQUENCE}`

Examples:
```
IT-TASK-001
IT-TASK-002
HR-REQ-001
```

- SOW.short_name: Required, globally unique, no spaces, editable
- Sequence: Per SOW+type, continues on prefix change
- Noun short_name: Immutable once created

### NounActor

```sql
NounActor {
  id: uuid (PK)
  noun_id: uuid (FK → Noun)
  actor: string ("+alice", "@devops", "!accenture", "*autobot")
  role: enum (owner, assignee, awareness, sme, stakeholder, resource)
}
```

### NounAssignment

```sql
NounAssignment {
  id: uuid (PK)
  noun_id: uuid (FK → Noun)
  container_id: uuid (FK → Noun, must be Container type)
  sort_order: int
}
```

### NounBlock

```sql
NounBlock {
  id: uuid (PK)
  blocker_id: uuid (FK → Noun, type=Blocker)
  blocked_id: uuid (FK → Noun)
  created_at: timestamp
}
```

### NounHashTag

```sql
NounHashTag {
  id: uuid (PK)
  noun_id: uuid (FK → Noun)
  tag: string
}
```

### Transaction

```sql
Transaction {
  id: uuid (PK)
  noun_id: uuid (FK → Noun)
  sequence: int (per-Noun, monotonic)
  verb: enum
  actor: string
  timestamp: timestamp
  before: jsonb (full snapshot)
  after: jsonb (full snapshot)
  context: jsonb (verb-specific: target_id, container_id)
}
```

### NounSequence

```sql
NounSequence {
  id: uuid (PK)
  sow_id: uuid (FK → Noun)
  noun_type: enum
  next_value: int
}
```

---

## Actor System

### Actor Types

| Prefix | Type | Source |
|--------|------|--------|
| + | Person | Internal table or LDAP |
| @ | Group | Internal table or LDAP |
| ! | Vendor | Internal table |
| * | AI | Internal table |

### Tables

```sql
Person {
  id: uuid (PK)
  source: enum (internal, ldap)
  external_id: string (LDAP DN or null)
  name: string
  email: string
  created_at: timestamp
}

Group {
  id: uuid (PK)
  source: enum (internal, ldap)
  external_id: string
  name: string
  role_id: uuid (FK → Role, nullable)
  created_at: timestamp
}

Vendor {
  id: uuid (PK)
  name: string
  contact_email: string
  created_at: timestamp
}

AI {
  id: uuid (PK)
  name: string
  type: string
  created_at: timestamp
}

GroupMember {
  id: uuid (PK)
  group_id: uuid (FK → Group)
  actor: string
}
```

### LDAP Integration

- One-way sync: LDAP → internal cache
- Cache can be refreshed or disabled
- Validation checks internal table first, then LDAP if enabled

---

## Role & Permission System

### Role Definition

```sql
Role {
  id: uuid (PK)
  name: string
  inherits_from: enum (owner, sme, assignee, resource, stakeholder, awareness)
  verbs: string[]
  created_at: timestamp
}

RoleMapping {
  id: uuid (PK)
  role_id: uuid (FK → Role)
  actor: string ("+alice", "@IT_Lead", "!accenture", "*autobot")
}
```

### Base Role Defaults

| Role | Default Verbs |
|------|---------------|
| owner | * (all) |
| sme | * (all) |
| assignee | Update, Complete, Incomplete, Normal, Escalate, Blocked, Release |
| resource | Update, Complete, Incomplete, Normal, Blocked, Release |
| stakeholder | (view only) |
| awareness | (view only) |

### SOW Roles (allow/deny)

```json
{
  "allow": [
    { "actor": "@IT_Lead", "type": "direct", "verbs": ["*"] },
    { "actor": "+contractor", "type": "direct", "verbs": ["Update"] }
  ],
  "deny": [
    { "actor": "+alice", "type": "inheritance", "verbs": ["Close"] }
  ]
}
```

### Permission Evaluation

```
1. Get actor's groups from GroupMember
2. Collect actors: ["+alice", "@IT_Lead", ...]
3. Get RoleMapping for each actor → roles with verbs
4. Check SOW.roles.allow for any of these actors
5. Get verbs from allow rules
6. Check SOW.roles.deny
7. Remove denied verbs
8. Result: permitted verbs
```

**Access rule**: No allow entry = no access to Level

---

## Multi-DB Federation

### Routing

```sql
SOWDatabase {
  id: uuid (PK)
  sow_id: uuid (FK → Noun)
  db_connection: string
}
```

Flow:
```
Request with noun_id
  → Look up noun_id → sow_id
  → Look up sow_id → db_connection (cached in Redis)
  → Route to correct DB
```

### Cross-DB Operations

- Writes to multiple DBs use optimistic approach
- Background reconciliation job compares timestamps
- Stale side synced if mismatch > threshold

---

## Caching Strategy

| Scale | Caching |
|-------|---------|
| Small (skinny) | None/App memory |
| Medium | Optional Redis |
| Large (federated) | Redis required |

Cached:
- SOW → DB mappings
- Permission evaluations
- is_blocked computations

---

## API Design

### Authentication

Stage 1: Static API key in header
```
X-API-Key: {key}
```

### Base URLs

```
# SOW Management
GET    /sows
POST   /sows
GET    /sow/{sow_id}

# Views
GET    /views/{sow_id}/timeline?start={}&end={}
GET    /views/{sow_id}/kanban?container_id={}
GET    /views/{sow_id}/calendar?month={}

# Nouns
GET    /nouns/{sow_id}/nouns?type={}&state={}&assignee={}&hashtag={}
POST   /nouns/{sow_id}/nouns
GET    /nouns/{sow_id}/noun/{noun_id}

# Transactions
GET    /nouns/{sow_id}/noun/{noun_id}/transactions
GET    /nouns/{sow_id}/transactions?actor={}&after={}&before={}
POST   /nouns/{sow_id}/noun/{noun_id}/transactions
POST   /nouns/{sow_id}/batch/transactions

# Logs (pass-through to logging subsystem)
POST   /logs/debug
POST   /logs/error
```

### Transaction Payload

```json
{
  "verb": "Complete",
  "actor": "+alice",
  "context": {}
}

// With context
{
  "verb": "Assign",
  "actor": "+alice",
  "context": { "container_id": "uuid" }
}

// Batch
{
  "verb": "Complete",
  "actor": "+alice",
  "noun_ids": ["uuid-1", "uuid-2"],
  "context": {}
}
```

### Batch Operations

- All or nothing (transactional)
- Validate all, then execute
- Failure returns which nouns failed validation

### Pagination

```
GET /nouns/{sow_id}/nouns?limit=50&sort=due_date&order=asc&cursor=2024-01-15T10:30:00Z_uuid-123
```

| Parameter | Description | Default |
|-----------|-------------|---------|
| limit | Max results (1-1000) | 50 |
| cursor | timestamp_uuid | - |
| sort | Field to sort | created_at |
| order | asc/desc | desc |

Response:
```json
{
  "data": [...],
  "pagination": {
    "limit": 50,
    "sort": "due_date",
    "order": "asc",
    "cursor": "2024-01-20T00:00:00Z_uuid-456",
    "has_more": true
  }
}
```

### Error Response

```json
{
  "error": {
    "code": "NOUN_BLOCKED",
    "message": "Cannot complete noun: noun is blocked",
    "details": {
      "noun_id": "uuid-123",
      "blocked_by": ["uuid-blocker-1"]
    }
  }
}
```

Error codes:
- Auth: `UNAUTHORIZED`, `FORBIDDEN`, `INVALID_API_KEY`, `MISSING_API_KEY`
- Not Found: `SOW_NOT_FOUND`, `NOUN_NOT_FOUND`
- State: `NOUN_BLOCKED`, `NOUN_CLOSED`, `INVALID_STATE_TRANSITION`
- Validation: `INVALID_PAYLOAD`, `MISSING_FIELD`, `INVALID_NOUN_TYPE`, `INVALID_SORT_FIELD`
- Batch: `BATCH_VALIDATION_FAILED`
- Permission: `VERB_DENIED`, `NO_LEVEL_ACCESS`

---

## Admin API (/sys0)

### Actor Management

```
# Persons
GET    /sys0/actors/persons
POST   /sys0/actors/persons
GET    /sys0/actors/persons/{id}
PUT    /sys0/actors/persons/{id}
DELETE /sys0/actors/persons/{id}

# Groups
GET    /sys0/actors/groups
POST   /sys0/actors/groups
GET    /sys0/actors/groups/{id}
PUT    /sys0/actors/groups/{id}
DELETE /sys0/actors/groups/{id}
GET    /sys0/actors/groups/{id}/members
POST   /sys0/actors/groups/{id}/members
DELETE /sys0/actors/groups/{id}/members/{actor}

# Vendors
GET    /sys0/actors/vendors
POST   /sys0/actors/vendors
GET    /sys0/actors/vendors/{id}
PUT    /sys0/actors/vendors/{id}
DELETE /sys0/actors/vendors/{id}

# AI
GET    /sys0/actors/ai
POST   /sys0/actors/ai
GET    /sys0/actors/ai/{id}
PUT    /sys0/actors/ai/{id}
DELETE /sys0/actors/ai/{id}

# LDAP
GET    /sys0/actors/ldap
POST   /sys0/actors/ldap
```

### LDAP Commands

```json
{ "command": "sync" }
{ "command": "clear_cache" }
{ "command": "set_interval", "value": 30 }
{ "command": "configure", "config": { "enabled": true, "server": "...", "base_dn": "...", "cache_enabled": true } }
```

### Role Management

```
GET    /sys0/roles
POST   /sys0/roles
GET    /sys0/roles/{id}
PUT    /sys0/roles/{id}
DELETE /sys0/roles/{id}
GET    /sys0/roles/{id}/actors
POST   /sys0/roles/{id}/actors
DELETE /sys0/roles/{id}/actors/{actor}
```

### Instructions Management

```
GET    /sys0/instructions
POST   /sys0/instructions
PUT    /sys0/instructions/{id}
DELETE /sys0/instructions/{id}
```

### Help Management

```
PUT    /sys0/help/nouns/{type}
PUT    /sys0/help/verbs/{verb}
```

---

## User API (/user)

```
# Aliases
GET    /user/aliases
POST   /user/aliases
DELETE /user/aliases/{name}

# Profile
GET    /user/profile
PUT    /user/profile

# Preferences
GET    /user/preferences
PUT    /user/preferences

# Access
GET    /user/roles
GET    /user/sows
```

### Alias

```sql
Alias {
  id: uuid (PK)
  actor: string
  name: string
  noun_id: uuid (FK → Noun)
  created_at: timestamp
}
```

Unique constraint: (actor, name)

### Preferences

```json
{
  "default_sow": "uuid-123",
  "timezone": "America/New_York",
  "date_format": "YYYY-MM-DD",
  "theme": "dark",
  "notifications": {
    "email": true,
    "escalations": true,
    "assignments": true
  },
  "default_view": "kanban"
}
```

---

## Help API

```
GET /help/nouns
GET /help/nouns/{type}
GET /help/verbs
GET /help/verbs/{verb}
GET /help/containers
GET /help/roles
GET /help/instructions
GET /help/instructions?sow_id={sow_id}
```

### Help Noun Response

```json
{
  "type": "Task",
  "abbrev": "TASK",
  "description": "Individual work items that can be assigned and tracked",
  "container": false,
  "leaf_only": false,
  "common_verbs": ["Complete", "Assign", "Blocked", "Escalate"],
  "typical_parents": ["Item", "Project", "Deliverable", "SOW"],
  "typical_children": ["Task", "Artifact", "Meeting"],
  "custom_fields": ["task_status", "estimated_hours", "actual_hours"],
  "example_usage": "Create tasks for actionable work."
}
```

### Help Verb Response

```json
{
  "verb": "Complete",
  "description": "Mark a noun as finished",
  "guards": ["Must be Normal or Escalated state", "Must not be blocked"],
  "side_effects": ["Sets completed_at timestamp", "If Blocker: releases all blocked nouns"],
  "undone_by": "Normal",
  "example": "level complete IT-TASK-001"
}
```

### Instructions

```sql
Instruction {
  id: uuid (PK)
  scope: enum (global, sow)
  sow_id: uuid (nullable)
  category: string (structure, workflow, naming, assignment, escalation)
  title: string
  content: text
  applies_to: string[]
  created_by: string
  updated_at: timestamp
}
```

---

## SOW-Level Instructions

```
GET    /nouns/{sow_id}/instructions
POST   /nouns/{sow_id}/instructions
PUT    /nouns/{sow_id}/instructions/{id}
DELETE /nouns/{sow_id}/instructions/{id}
```

---

## CLI Client

### Commands

```bash
# SOW
level sow list
level sow create --name="..."
level sow show {sow_id}

# Nouns
level list nouns --sow={sow_id} --type=Task --state=Normal
level show {reference}
level create --sow={sow_id} --type=Task --parent={id} --title="..."

# Verbs (via transactions)
level complete {reference}
level incomplete {reference}
level escalate {reference}
level normal {reference}
level close {reference}
level update {reference} title="New Title" due="2024-02-01" custom.severity="HIGH"
level reparent {reference}

# Relationships
level assign {reference} --container={id}
level unassign {reference} --container={id}
level block {blocker_ref} --target={noun_ref}
level release {blocker_ref} --target={noun_ref}

# Batch
level complete {ref1} {ref2} {ref3}
level assign {ref1} {ref2} --container={id}

# Views
level timeline --sow={sow_id} --start=2024-01-01 --end=2024-01-31
level kanban --container={id}
level calendar --sow={sow_id} --month=2024-01

# Transactions
level history {reference}
level transactions --sow={sow_id} --actor=+alice

# Aliases
level alias set budget uuid-123
level alias list
level alias remove budget

# Config
level config set api-key {key}
level config set default-sow {sow_id}

# Help (AI guidance)
level help
level help nouns
level help verbs
level help noun Task
level help verb Complete
```

### Noun Reference Resolution

| Reference | Format | Example |
|-----------|--------|---------|
| UUID | uuid | `level show uuid-123` |
| ShortName | unquoted | `level show IT-TASK-001` |
| Alias | *name | `level show *budget` |
| Title | quoted | `level show "Budget Report"` |

Resolution order:
1. UUID match
2. ShortName match (global)
3. Alias match (*prefix)
4. Title search (requires --sow or default_sow)

### Update Syntax

```bash
level update {reference} field=value field2=value2 custom.field=value
```

---

## MCP Client

Same commands as CLI, plus:
- Full access based on *AI actor permissions
- Help command for AI guidance
- Instructions for operational context

---

## Web Client

- Standard SPA
- No compute on client side
- All logic server-side
- Consumes same REST API

---

## Skinny Mode

### Wizard Flow

1. "Create Workspace"
2. Name, Description, Timezone
3. Add SME, Stakeholders
4. System creates SOW + Level (hidden)
5. User lands in Level dashboard

### Features Comparison

| Feature | Skinny | Full |
|---------|--------|------|
| SOW visible | ❌ | ✅ |
| Sub-Levels | ❌ | ✅ |
| Multi-DB | ❌ | ✅ |
| Budget/Contract | ❌ | ✅ |
| Role.deny() | ❌ | ✅ |
| SME/Stakeholder | ✅ | ✅ |
| Projects + Goals | ✅ | ✅ |
| MileStones | ✅ | ✅ |
| All Noun types | ✅ | ✅ |
| Blocking | ✅ | ✅ |

### Upgrade Path

- Flip flag on SOW
- Reveal hidden SOW controls
- Enable sub-Levels, advanced features
- No data migration needed

---

## Temporal Fields

| Field | Set By | Notes |
|-------|--------|-------|
| created_at | auto on Open() | |
| due_date | user | Meetings display as "Start" |
| completed_at | auto on Complete() | |
| closed_at | auto on Close() | Archive clock starts |

Meeting additional fields (UI only):
- duration
- end_time

---

## Project Goals

```json
{
  "custom_fields": {
    "goal_noun_ids": ["uuid-deliverable-1", "uuid-event-2"]
  }
}
```

- Goals are Deliverables or Events
- Blocked Goal shows in Project.blocked_goals
- Does NOT set Project.is_blocked

---

## MileStone Auto-Complete

```json
{
  "custom_fields": {
    "auto_complete": true
  }
}
```

- is_blocked: true if ANY assigned Noun is blocked
- is_completable: true if ALL assigned Nouns are Completed
- If auto_complete && is_completable → system calls Complete()

---

## Views

### Timeline
- Primary index: due_date
- Vertical cards
- Filter by Container, State, Actor

### Kanban
- Primary index: HashTag, State, or user drag-drop
- Sort order per Container
- Stored in NounAssignment.sort_order

### Calendar
- Primary index: due_date (day buckets)
- Filter by Container, Noun type
