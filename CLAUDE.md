# CLAUDE.md

## Rules
- First think through the problem, read the codebase for relevant files, look for reuse vs create and write a plan to todo/tasks.md.
	- The plan should have a list of todo items that you can check off as you complete them
	- Before you begin working, check in with me and I will verify the plan.
	- Then, begin working on the todo items, marking them as complete as you go.
	- At every step of the way please give me a high level explanation of what changes you made
	- Make every task and code change you do as simple as possible. We want to avoid making any massive or complex changes. Every change should impact as little code as possible. Everything is about simplicity.
	- Before creating new, look for code, scripts and pattern to reuse.  
	- Please check through all the code you just wrote and make sure it follows security best practices. make sure there are no sensitive information in the front and there are no vulnerabilities that can be exploited
	- Do local git commit with summary of the changes you made tasks completed.
Look for existing script, function, or implementation and build on them before create new script, function and process. 


This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a project management system built around a Noun/Verb/Container grammar architecture with event sourcing, multi-database federation, and flexible deployment modes. The system competes with MS Project and supports three deployment modes: Skinny (single DB, hidden SOW), Standard (single DB per org), and Enterprise (multi-DB federation).

## Architecture

### Core Grammar Model

The system is built on a consistent Noun/Verb pattern:

- **12 Noun Types**: SOW (genesis noun), Item, Task, Request, Meeting, Deliverable, Event, Blocker, Artifact (leaf only), plus 3 Container types (Group, Project, MileStone)
- **12 Verbs**: Open, Complete, Incomplete, Normal, Escalate, Close, Update, Reparent, Assign, Unassign, Blocked, Release
- **Event Sourcing**: Every action creates an immutable Transaction record with before/after snapshots

### State Machine

Nouns flow through states: Normal → Escalated → Completed/Incompleted → Closed → Archived (terminal). Most verbs require Normal or Escalated state. Complete/Incomplete require noun not be blocked. Close requires Completed/Incompleted state and all children closed.

### Blocking Propagation

Blocking propagates both downward (blocker affects target + all children) and upward (blocked child makes parent is_blocked = true). For Projects with Goals (Deliverables/Events), blocked Goals increment blocked_goals counter but do NOT set Project.is_blocked.

### Short Name Generation

Format: `{SOW.short_name}-{TYPE_ABBREV}-{SEQUENCE}`
- Examples: IT-TASK-001, HR-REQ-002
- SOW short_name is globally unique, user-editable
- Sequence is per SOW+type
- Noun short_name is immutable once created

### Multi-Database Federation

- SOW → database routing via SOWDatabase table
- sow_id on every Noun enables routing
- Cross-DB operations use optimistic approach with background reconciliation
- Redis required for large/federated deployments (caches SOW→DB mappings, permissions, is_blocked computations)

### Actor System

Four actor types with prefixes:
- `+` Person (internal/LDAP)
- `@` Group (internal/LDAP)
- `!` Vendor
- `*` AI

LDAP is one-way sync to internal cache. Actors have roles (owner, sme, assignee, resource, stakeholder, awareness) with inherited verb permissions. SOW.roles field contains allow/deny rules that override base permissions.

### Permission Evaluation

1. Resolve actor's groups via GroupMember
2. Collect all actor strings (person + groups)
3. Get RoleMapping for each → roles with verbs
4. Check SOW.roles.allow for any actors
5. Get allowed verbs
6. Check SOW.roles.deny and remove denied verbs

**Critical**: No allow entry = no access to Level (SOW).

## Database Schema

Primary tables:
- **Noun**: Core entity with type, parent_id, sow_id, state, is_blocked (computed), custom_fields (jsonb)
- **Transaction**: Event log with sequence, verb, actor, before/after snapshots, context (jsonb)
- **NounActor**: M:N actor roles (owner, assignee, awareness, sme, stakeholder, resource)
- **NounAssignment**: M:N container assignments with sort_order (for Kanban)
- **NounBlock**: M:N blocker→target links
- **NounHashTag**: M:N global tags
- **NounSequence**: Per-SOW+type sequence counter for short_name generation

See docs/project-management-schema.md for complete schema details.

## API Design

### Base URL Structure

- SOW management: `/sows`, `/sow/{sow_id}`
- Views: `/views/{sow_id}/{timeline|kanban|calendar}`
- Nouns: `/nouns/{sow_id}/nouns`, `/nouns/{sow_id}/noun/{noun_id}`
- Transactions: `/nouns/{sow_id}/noun/{noun_id}/transactions`, `/nouns/{sow_id}/batch/transactions`
- Admin: `/sys0/actors/*`, `/sys0/roles/*`, `/sys0/instructions/*`, `/sys0/help/*`
- User: `/user/{aliases|profile|preferences|roles|sows}`
- Help: `/help/{nouns|verbs|containers|roles|instructions}`

### Authentication

Stage 1: Static API key in `X-API-Key` header.

### Batch Operations

All-or-nothing transactional. Validate all nouns first, then execute. Return failed validations if any fail.

### Pagination

Uses cursor-based pagination: `?limit=50&sort=due_date&order=asc&cursor=2024-01-15T10:30:00Z_uuid-123`

## Directory Structure

```
level/
├── docs/                         # Documentation
│   ├── project-management-schema.md  # Complete technical spec (READ THIS FIRST)
│   ├── summary/                  # Summary markdown files
│   └── defects/                  # Major problem documentation
├── todo/                         # Task tracking
│   ├── tasks.md                  # Current tasks
│   └── bugfix.md                 # Bug tracking (references defects/)
├── src/                          # Source code (currently empty)
├── scripts/                      # Scripts (currently empty)
├── config/                       # Master config (copy to release/ at build)
├── release/                      # Final compiled code (uses release/config, release/logs)
├── logs/                         # Development logs (src/ runtime)
└── scratch/                      # Temporary files
```

## Client Implementations

### CLI Client

Noun reference resolution order:
1. UUID match
2. ShortName match (global, e.g., IT-TASK-001)
3. Alias match (*prefix, e.g., *budget)
4. Title search (quoted, requires --sow or default_sow)

Update syntax: `level update {reference} field=value custom.field=value`

Batch operations: `level complete {ref1} {ref2} {ref3}`

### MCP Client

Same commands as CLI but executed by *AI actor with AI-specific permissions. Full access to Help API for AI guidance and Instructions for operational context.

### Web Client

Standard SPA with no client-side compute. All logic server-side. Consumes same REST API.

## Skinny Mode

Skinny mode hides SOW management to compete with MS Project:
- Wizard creates SOW + Level silently
- No sub-Levels, multi-DB, budget/contract, or role.deny()
- Still supports Projects+Goals, MileStones, all Noun types, blocking
- Upgrade path: flip flag on SOW to reveal full features (no migration needed)

## Special Behaviors

### MileStone Auto-Complete

If `custom_fields.auto_complete = true`:
- `is_blocked = true` if ANY assigned Noun is blocked
- `is_completable = true` if ALL assigned Nouns are Completed
- System automatically calls Complete() when auto_complete && is_completable

### Project Goals

Goals stored in `custom_fields.goal_noun_ids` array (Deliverables or Events only). Blocked goals show in Project.blocked_goals but do NOT set Project.is_blocked.

### Meeting Fields

Meetings use due_date as "Start" time. Additional UI-only fields: duration, end_time.

## Development Notes

- This repository is currently in initial setup phase (no source code yet)
- The architecture is fully documented in docs/project-management-schema.md
- Key design principle: **Simplest wins** (flat structures, junction tables, pass-through logging)
- All state changes must go through Transactions (event sourcing)
- SOW is the genesis noun that creates and owns all other nouns in a Level
