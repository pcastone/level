# Level Project - Build Tasks

## Overview
Build the Level project management system with Rust backend and SvelteKit frontend.

---

## Phase 1: Rust Backend Foundation

### 1.1 Workspace Setup
- [ ] Create Cargo workspace in `src/`
- [ ] Create core crates structure:
  - `level-core` - Domain models (nouns, verbs, actors, state machine)
  - `level-db` - Database layer (PostgreSQL, migrations)
  - `level-api` - REST API (axum)
  - `level-cli` - CLI client

### 1.2 Domain Models (`level-core`)
- [ ] Define Noun types enum (12 types: SOW, Item, Task, Request, Meeting, Deliverable, Event, Blocker, Artifact, Group, Project, MileStone)
- [ ] Define Verb types enum (12 verbs: Open, Complete, Incomplete, Normal, Escalate, Close, Update, Reparent, Assign, Unassign, Blocked, Release)
- [ ] Define State enum (Normal, Escalated, Completed, Incompleted, Closed)
- [ ] Define Actor types and role enums
- [ ] Implement Noun struct with all fields
- [ ] Implement Transaction struct for event sourcing
- [ ] Implement state machine validation logic
- [ ] Implement blocking propagation logic

### 1.3 Database Layer (`level-db`)
- [ ] Set up sqlx with PostgreSQL
- [ ] Create migrations for all tables:
  - Noun
  - Transaction
  - NounActor
  - NounAssignment
  - NounBlock
  - NounHashTag
  - NounSequence
  - Person, Group, Vendor, AI
  - GroupMember
  - Role, RoleMapping
  - Alias
  - Instruction
- [ ] Implement repository traits and implementations
- [ ] Implement short_name generation logic

### 1.4 REST API (`level-api`)
- [ ] Set up axum with basic middleware (CORS, logging)
- [ ] Implement API key authentication
- [ ] SOW endpoints (`/sows`, `/sow/{id}`)
- [ ] Noun endpoints (`/nouns/{sow_id}/nouns`, `/nouns/{sow_id}/noun/{id}`)
- [ ] Transaction endpoints (`/nouns/{sow_id}/noun/{id}/transactions`)
- [ ] Batch operations endpoint
- [ ] View endpoints (timeline, kanban, calendar)
- [ ] Admin endpoints (`/sys0/*`)
- [ ] User endpoints (`/user/*`)
- [ ] Help endpoints (`/help/*`)
- [ ] Error response formatting
- [ ] Cursor-based pagination

### 1.5 CLI Client (`level-cli`)
- [ ] Basic CLI structure with clap
- [ ] SOW commands (list, create, show)
- [ ] Noun commands (list, show, create)
- [ ] Verb commands (complete, escalate, etc.)
- [ ] Alias management
- [ ] Config management

---

## Phase 2: SvelteKit Frontend

### 2.1 Project Setup
- [ ] Create SvelteKit project in `src/frontend/`
- [ ] Configure TypeScript
- [ ] Set up API client for backend
- [ ] Configure routing

### 2.2 Core Components
- [ ] Layout with navigation
- [ ] Authentication handling
- [ ] Noun card component
- [ ] Noun form components
- [ ] Transaction history component

### 2.3 Views
- [ ] Timeline view
- [ ] Kanban view
- [ ] Calendar view
- [ ] SOW/Level dashboard

### 2.4 Pages
- [ ] SOW list page
- [ ] Noun detail page
- [ ] Settings/preferences page
- [ ] Help pages

---

## Phase 3: Integration & Polish

- [ ] End-to-end testing setup
- [ ] Docker compose for local development
- [ ] Configuration management (config/)
- [ ] Basic documentation

---

## Notes

- **Simplicity first**: Each change impacts minimal code
- **Event sourcing**: All state changes via Transaction records
- **SOW as genesis**: Every noun belongs to a SOW for DB routing
- **Skinny mode**: Initial deployment hides SOW complexity

