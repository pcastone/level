# Level - Rust Backend & CLI Build Plan
Generated: 2026-01-30

## Overview
Rebuild the Level project management system in Rust for the backend API and CLI client. The system uses a Noun/Verb/Container grammar architecture with event sourcing, supporting Skinny/Standard/Enterprise deployment modes.

---

## Phase 1: Project Foundation

### Task 1.1: Initialize Rust workspace
- [ ] Create Cargo workspace with members: api, cli, core, db
- [ ] Configure shared dependencies in workspace Cargo.toml
- [ ] Set up rustfmt.toml and clippy.toml for code quality
- [ ] Create .cargo/config.toml for build settings

### Task 1.2: Core crate setup (level-core)
- [ ] Create level-core crate for shared domain types
- [ ] Define NounType enum (12 types)
- [ ] Define NounState enum (6 states)
- [ ] Define Verb enum (12 verbs)
- [ ] Define ActorType enum (Person, Group, Vendor, AI)
- [ ] Define DeploymentMode enum (Skinny, Standard, Enterprise)

### Task 1.3: Database crate setup (level-db)
- [ ] Create level-db crate for database operations
- [ ] Add sqlx with PostgreSQL feature
- [ ] Configure connection pooling
- [ ] Set up migrations directory
- [ ] Create database trait abstractions

### Task 1.4: API crate setup (level-api)
- [ ] Create level-api crate with axum framework
- [ ] Configure tower middleware stack
- [ ] Set up structured logging with tracing
- [ ] Configure CORS and security headers
- [ ] Create health check endpoint

### Task 1.5: CLI crate setup (level-cli)
- [ ] Create level-cli crate with clap framework
- [ ] Configure CLI argument parsing
- [ ] Set up config file handling (~/.level/config.toml)
- [ ] Create HTTP client wrapper with reqwest

---

## Phase 2: Core Domain Models (level-core)

### Task 2.1: Noun model
- [ ] Define Noun struct with all fields
- [ ] Implement serde Serialize/Deserialize
- [ ] Add validation methods
- [ ] Implement Display and Debug traits
- [ ] Add builder pattern for construction

### Task 2.2: Transaction model (Event Sourcing)
- [ ] Define Transaction struct for event log
- [ ] Add before/after snapshot fields (serde_json::Value)
- [ ] Define context field for verb-specific data
- [ ] Implement sequence numbering

### Task 2.3: Actor models
- [ ] Define Person struct
- [ ] Define Group struct with members
- [ ] Define Vendor struct
- [ ] Define AI struct
- [ ] Implement actor string parsing (+, @, !, * prefixes)

### Task 2.4: Role and Permission models
- [ ] Define Role struct with inheritance
- [ ] Define RoleMapping struct
- [ ] Define permission evaluation logic
- [ ] Implement SOW allow/deny rule processing

### Task 2.5: Relationship models
- [ ] Define NounActor junction (M:N actor-role)
- [ ] Define NounAssignment junction (container assignments)
- [ ] Define NounBlock junction (blocker-target)
- [ ] Define NounHashTag junction (tagging)

### Task 2.6: Auxiliary models
- [ ] Define Alias struct
- [ ] Define Instruction struct
- [ ] Define UserPreferences struct
- [ ] Define SOWDatabase struct (federation routing)
- [ ] Define NounSequence struct (short_name generation)

---

## Phase 3: Database Layer (level-db)

### Task 3.1: SQLx migrations
- [ ] Create nouns table migration
- [ ] Create transactions table migration
- [ ] Create actors tables (persons, groups, vendors, ais)
- [ ] Create roles and role_mappings tables
- [ ] Create junction tables (noun_actors, noun_assignments, noun_blocks, noun_hashtags)
- [ ] Create auxiliary tables (aliases, instructions, user_preferences, sow_databases, noun_sequences)

### Task 3.2: Repository traits
- [ ] Define NounRepository trait
- [ ] Define TransactionRepository trait
- [ ] Define ActorRepository trait
- [ ] Define RoleRepository trait

### Task 3.3: PostgreSQL implementations
- [ ] Implement NounRepository for PostgreSQL
- [ ] Implement TransactionRepository for PostgreSQL
- [ ] Implement ActorRepository for PostgreSQL
- [ ] Implement RoleRepository for PostgreSQL

### Task 3.4: Query builders
- [ ] Implement cursor-based pagination
- [ ] Implement filtering (type, state, is_blocked)
- [ ] Implement sorting (due_date, created_at, title)
- [ ] Implement SOW-scoped queries

---

## Phase 4: Domain Services (level-core)

### Task 4.1: State Machine service
- [ ] Implement valid_transitions map
- [ ] Create is_valid_transition function
- [ ] Add state guards for verbs
- [ ] Implement state change validation

### Task 4.2: Blocking service
- [ ] Implement propagate_block_downward
- [ ] Implement propagate_block_upward
- [ ] Handle Goal boundary (blocked_goals counter)
- [ ] Implement compute_is_blocked

### Task 4.3: Transaction service
- [ ] Implement create_transaction with snapshots
- [ ] Add sequence number generation
- [ ] Implement transaction querying
- [ ] Add batch transaction support

### Task 4.4: Short Name service
- [ ] Define TYPE_ABBREVS map
- [ ] Implement generate_short_name
- [ ] Handle sequence incrementing
- [ ] Validate SOW short_name uniqueness

### Task 4.5: MileStone auto-complete service
- [ ] Implement is_blocked computation for MileStone
- [ ] Implement is_completable check
- [ ] Handle automatic Complete() trigger

### Task 4.6: Permission evaluation service
- [ ] Resolve actor groups via GroupMember
- [ ] Collect all actor strings
- [ ] Get RoleMapping for each actor
- [ ] Process SOW.roles allow/deny rules
- [ ] Return allowed verbs set

---

## Phase 5: Verb Handlers (level-core)

### Task 5.1: Open verb
- [ ] Validate noun data
- [ ] Generate short_name
- [ ] Create noun record
- [ ] Log transaction

### Task 5.2: Complete verb
- [ ] Check not blocked guard
- [ ] Validate state (Normal/Escalated)
- [ ] Set completed_at timestamp
- [ ] Release any Blocker side-effects
- [ ] Log transaction

### Task 5.3: Incomplete verb
- [ ] Check not blocked guard
- [ ] Validate state (Normal/Escalated)
- [ ] Log transaction

### Task 5.4: Normal verb
- [ ] Validate state (Escalated/Completed/Incompleted)
- [ ] Reset to Normal state
- [ ] Log transaction

### Task 5.5: Escalate verb
- [ ] Validate state (Normal)
- [ ] Set Escalated state
- [ ] Log transaction

### Task 5.6: Close verb
- [ ] Validate state (Completed/Incompleted)
- [ ] Check all children Closed
- [ ] Set closed_at timestamp
- [ ] Log transaction

### Task 5.7: Update verb
- [ ] Validate field updates
- [ ] Apply changes
- [ ] Log transaction with before/after

### Task 5.8: Reparent verb
- [ ] Validate new parent (or null for SOW root)
- [ ] Update parent_id
- [ ] Log transaction

### Task 5.9: Assign verb
- [ ] Validate container type (Group/Project/MileStone)
- [ ] Create NounAssignment with sort_order
- [ ] Log transaction

### Task 5.10: Unassign verb
- [ ] Remove NounAssignment
- [ ] Log transaction

### Task 5.11: Blocked verb
- [ ] Validate blocker type is Blocker
- [ ] Create NounBlock link
- [ ] Propagate is_blocked
- [ ] Log transaction

### Task 5.12: Release verb
- [ ] Remove NounBlock link
- [ ] Recompute is_blocked for affected nouns
- [ ] Log transaction

---

## Phase 6: REST API (level-api)

### Task 6.1: Authentication middleware
- [ ] Implement X-API-Key header extraction
- [ ] Create API key validation
- [ ] Add request context with actor info

### Task 6.2: SOW endpoints
- [ ] GET /sows - list SOWs
- [ ] POST /sows - create SOW
- [ ] GET /sow/{sow_id} - get SOW details

### Task 6.3: Noun endpoints
- [ ] GET /nouns/{sow_id}/nouns - list nouns with filters
- [ ] POST /nouns/{sow_id}/nouns - create noun
- [ ] GET /nouns/{sow_id}/noun/{noun_id} - get noun details
- [ ] PUT /nouns/{sow_id}/noun/{noun_id} - update noun
- [ ] DELETE /nouns/{sow_id}/noun/{noun_id} - delete noun

### Task 6.4: Transaction endpoints
- [ ] GET /nouns/{sow_id}/noun/{noun_id}/transactions - noun history
- [ ] POST /nouns/{sow_id}/noun/{noun_id}/transactions - apply verb
- [ ] GET /nouns/{sow_id}/transactions - SOW-wide transactions
- [ ] POST /nouns/{sow_id}/batch/transactions - batch operations

### Task 6.5: View endpoints
- [ ] GET /views/{sow_id}/timeline - timeline view
- [ ] GET /views/{sow_id}/kanban - kanban view
- [ ] GET /views/{sow_id}/calendar - calendar view

### Task 6.6: Admin endpoints (/sys0)
- [ ] Person CRUD endpoints
- [ ] Group CRUD with members endpoints
- [ ] Vendor CRUD endpoints
- [ ] AI CRUD endpoints
- [ ] Role management endpoints
- [ ] Instructions management endpoints

### Task 6.7: User endpoints (/user)
- [ ] GET/POST/DELETE /user/aliases
- [ ] GET/PUT /user/profile
- [ ] GET/PUT /user/preferences
- [ ] GET /user/roles
- [ ] GET /user/sows

### Task 6.8: Help endpoints
- [ ] GET /help/nouns, /help/nouns/{type}
- [ ] GET /help/verbs, /help/verbs/{verb}
- [ ] GET /help/containers
- [ ] GET /help/roles
- [ ] GET /help/instructions

### Task 6.9: Error handling
- [ ] Define error types (LevelError enum)
- [ ] Implement IntoResponse for errors
- [ ] Create standard error response format
- [ ] Add error codes (NOUN_BLOCKED, INVALID_STATE_TRANSITION, etc.)

### Task 6.10: Pagination
- [ ] Implement cursor-based pagination
- [ ] Parse limit, sort, order, cursor params
- [ ] Generate next_cursor in responses

---

## Phase 7: CLI Client (level-cli)

### Task 7.1: Configuration
- [ ] Parse ~/.level/config.toml
- [ ] Support api_url, api_key, default_sow settings
- [ ] Add output_format preference (table, json, minimal)

### Task 7.2: HTTP client
- [ ] Create LevelClient struct with reqwest
- [ ] Inject X-API-Key header
- [ ] Handle response parsing
- [ ] Implement error handling

### Task 7.3: Reference resolution
- [ ] Implement UUID resolution
- [ ] Implement ShortName resolution (regex match)
- [ ] Implement Alias resolution (* prefix)
- [ ] Implement Title resolution (quoted strings)
- [ ] Create resolution chain

### Task 7.4: SOW commands
- [ ] level sows - list SOWs
- [ ] level sow create <short_name> <title>
- [ ] level sow show <reference>
- [ ] level sow set-default <reference>

### Task 7.5: Noun commands
- [ ] level nouns [--type TYPE] [--state STATE] [--blocked]
- [ ] level noun create <type> <title> [--parent REF]
- [ ] level noun show <reference>
- [ ] level noun update <reference> field=value...

### Task 7.6: Verb commands
- [ ] level complete <reference>...
- [ ] level incomplete <reference>...
- [ ] level escalate <reference>...
- [ ] level normal <reference>...
- [ ] level close <reference>...
- [ ] level blocked <blocker_ref> <target_ref>
- [ ] level release <blocker_ref> <target_ref>
- [ ] level assign <noun_ref> <container_ref>
- [ ] level unassign <noun_ref> <container_ref>
- [ ] level reparent <noun_ref> [parent_ref]

### Task 7.7: View commands
- [ ] level timeline [--start DATE] [--end DATE]
- [ ] level kanban [--container REF]
- [ ] level calendar [--month YYYY-MM]

### Task 7.8: Alias commands
- [ ] level aliases - list aliases
- [ ] level alias create <name> <noun_ref>
- [ ] level alias delete <name>

### Task 7.9: Help commands
- [ ] level help nouns [TYPE]
- [ ] level help verbs [VERB]
- [ ] level help containers
- [ ] level help roles

### Task 7.10: Output formatting
- [ ] Implement table formatter (prettytable or tabled)
- [ ] Implement JSON formatter
- [ ] Implement minimal formatter
- [ ] Add color support with colored crate

---

## Phase 8: Testing

### Task 8.1: Unit tests (level-core)
- [ ] Test NounType and NounState enums
- [ ] Test StateTransition valid/invalid
- [ ] Test ShortNameService generation
- [ ] Test BlockingService propagation
- [ ] Test permission evaluation

### Task 8.2: Integration tests (level-db)
- [ ] Test CRUD operations for all entities
- [ ] Test pagination queries
- [ ] Test transaction logging
- [ ] Use testcontainers for PostgreSQL

### Task 8.3: API tests (level-api)
- [ ] Test all endpoint responses
- [ ] Test authentication
- [ ] Test error responses
- [ ] Test batch operations

### Task 8.4: CLI tests (level-cli)
- [ ] Test reference resolution
- [ ] Test command parsing
- [ ] Test config file handling
- [ ] Mock HTTP responses

---

## Phase 9: Deployment

### Task 9.1: Docker
- [ ] Create multi-stage Dockerfile for API
- [ ] Create Dockerfile for CLI
- [ ] Update docker-compose.yml for Rust services

### Task 9.2: Configuration
- [ ] Environment variable configuration
- [ ] Config file support (config.toml)
- [ ] Secrets management

### Task 9.3: Observability
- [ ] Structured logging with tracing
- [ ] Metrics with prometheus
- [ ] Health check endpoints

---

## Summary

| Phase | Tasks | Description |
|-------|-------|-------------|
| 1. Foundation | 5 | Cargo workspace, crate setup |
| 2. Domain Models | 6 | Core types in level-core |
| 3. Database | 4 | SQLx migrations, repositories |
| 4. Services | 6 | State machine, blocking, transactions |
| 5. Verb Handlers | 12 | All 12 verb implementations |
| 6. REST API | 10 | Axum endpoints |
| 7. CLI Client | 10 | Clap commands |
| 8. Testing | 4 | Unit, integration, API, CLI tests |
| 9. Deployment | 3 | Docker, config, observability |
| **Total** | **60** | |

---

## Tech Stack

### Backend (level-api)
- **Framework**: axum 0.7
- **Database**: sqlx 0.7 with PostgreSQL
- **Async Runtime**: tokio
- **Serialization**: serde, serde_json
- **Validation**: validator
- **Logging**: tracing, tracing-subscriber
- **Config**: config crate

### CLI (level-cli)
- **Framework**: clap 4.0 with derive
- **HTTP Client**: reqwest
- **Config**: toml
- **Tables**: tabled or prettytable-rs
- **Colors**: colored

### Shared (level-core)
- **UUID**: uuid
- **DateTime**: chrono
- **Error Handling**: thiserror, anyhow

---

## Directory Structure

```
level/
├── Cargo.toml              # Workspace manifest
├── crates/
│   ├── level-core/         # Shared domain types
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── models/
│   │       │   ├── mod.rs
│   │       │   ├── noun.rs
│   │       │   ├── transaction.rs
│   │       │   ├── actor.rs
│   │       │   └── role.rs
│   │       ├── services/
│   │       │   ├── mod.rs
│   │       │   ├── state_machine.rs
│   │       │   ├── blocking.rs
│   │       │   └── short_name.rs
│   │       └── verbs/
│   │           ├── mod.rs
│   │           └── handlers.rs
│   ├── level-db/           # Database layer
│   │   ├── Cargo.toml
│   │   ├── migrations/
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── pool.rs
│   │       └── repositories/
│   ├── level-api/          # REST API
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── routes/
│   │       ├── middleware/
│   │       └── error.rs
│   └── level-cli/          # CLI client
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── commands/
│           ├── client.rs
│           └── reference.rs
├── config/
├── docs/
├── frontend/               # Keep SvelteKit frontend
└── todo/
```
