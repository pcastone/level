# Task List
Generated: 2026-01-30 12:00

## Phase 1: Project Foundation

### Task: [TASK] Set up Python/FastAPI project structure with src/, tests/, config/ directories
- [x] [VALIDATE] Verify implementation of: Set up Python/FastAPI project structure with src/, tests/, config/ directories (done: 2026-01-30)

### Task: [TASK] Configure Poetry or pip for dependency management with dev/prod separation
- [x] [VALIDATE] Verify implementation of: Configure Poetry or pip for dependency management with dev/prod separation (done: 2026-01-30)

### Task: [TASK] Set up PostgreSQL database connection with SQLAlchemy async support
- [x] [VALIDATE] Verify implementation of: Set up PostgreSQL database connection with SQLAlchemy async support (done: 2026-01-30)

### Task: [TASK] Configure Alembic for database migrations
- [x] [VALIDATE] Verify implementation of: Configure Alembic for database migrations (done: 2026-01-30)

### Task: [TASK] Set up logging infrastructure with structured JSON logging
- [x] [VALIDATE] Verify implementation of: Set up logging infrastructure with structured JSON logging (done: 2026-01-30)

### Task: [TASK] Create base configuration system supporting Skinny/Standard/Enterprise modes
- [x] [VALIDATE] Verify implementation of: Create base configuration system supporting Skinny/Standard/Enterprise modes (done: 2026-01-30)

## Phase 2: Database Schema Implementation

### Task: [FEAT] Create Noun table with all 12 types
- [x] [VALIDATE] Verify implementation of: Create Noun table with all 12 types (SOW, Item, Task, Request, Meeting, Deliverable, Event, Blocker, Artifact, Group, Project, MileStone) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Noun table creation and type constraints (done: 2026-01-30)

### Task: [FEAT] Implement NounActor junction table for M:N actor-role relationships
- [x] [VALIDATE] Verify implementation of: NounActor junction table for M:N actor-role relationships (owner, assignee, awareness, sme, stakeholder, resource) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: NounActor junction table relationships (done: 2026-01-30)

### Task: [FEAT] Create NounAssignment table for Container assignments
- [x] [VALIDATE] Verify implementation of: NounAssignment table for Container assignments with sort_order for Kanban (done: 2026-01-30)
- [x] [TEST] Write unit tests for: NounAssignment table and sort_order functionality (done: 2026-01-30)

### Task: [FEAT] Implement NounBlock table for M:N blocker-target relationships
- [x] [VALIDATE] Verify implementation of: NounBlock table for M:N blocker-target relationships (done: 2026-01-30)
- [x] [TEST] Write unit tests for: NounBlock table relationships (done: 2026-01-30)

### Task: [FEAT] Create NounHashTag table for global tagging system
- [x] [VALIDATE] Verify implementation of: NounHashTag table for global tagging system (done: 2026-01-30)
- [x] [TEST] Write unit tests for: NounHashTag table functionality (done: 2026-01-30)

### Task: [FEAT] Implement Transaction table with event sourcing
- [x] [VALIDATE] Verify implementation of: Transaction table with sequence, verb, actor, before/after snapshots, context jsonb (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Transaction table and event sourcing (done: 2026-01-30)

### Task: [FEAT] Create NounSequence table for short_name generation
- [x] [VALIDATE] Verify implementation of: NounSequence table for per-SOW+type short_name generation (done: 2026-01-30)
- [x] [TEST] Write unit tests for: NounSequence table and sequence generation (done: 2026-01-30)

### Task: [FEAT] Implement SOWDatabase table for multi-DB federation routing
- [x] [VALIDATE] Verify implementation of: SOWDatabase table for multi-DB federation routing (done: 2026-01-30)
- [x] [TEST] Write unit tests for: SOWDatabase table routing (done: 2026-01-30)

### Task: [FEAT] Create Alias table
- [x] [VALIDATE] Verify implementation of: Alias table with id, actor, name, noun_id, created_at and unique (actor, name) constraint (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Alias table uniqueness constraints (done: 2026-01-30)

### Task: [FEAT] Create Instruction table
- [x] [VALIDATE] Verify implementation of: Instruction table with id, scope (global/sow), sow_id, category, title, content, applies_to, created_by, updated_at (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Instruction table scope and category handling (done: 2026-01-30)

### Task: [FEAT] Create HelpContent table or static config
- [x] [VALIDATE] Verify implementation of: HelpContent table or static config for noun/verb documentation (type, abbrev, description, guards, side_effects, examples) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: HelpContent data structure (done: 2026-01-30)

### Task: [FEAT] Create UserPreferences table
- [x] [VALIDATE] Verify implementation of: UserPreferences table or jsonb field for storing user profile and preferences (done: 2026-01-30)
- [x] [TEST] Write unit tests for: UserPreferences storage and retrieval (done: 2026-01-30)

## Phase 3: Actor System

### Task: [FEAT] Create Person table with internal/LDAP source support
- [x] [VALIDATE] Verify implementation of: Person table with internal/LDAP source support (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Person table LDAP integration (done: 2026-01-30)

### Task: [FEAT] Implement Group table with role association
- [x] [VALIDATE] Verify implementation of: Group table with role association (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Group table role associations (done: 2026-01-30)

### Task: [FEAT] Create Vendor table for external vendor actors
- [x] [VALIDATE] Verify implementation of: Vendor table for external vendor actors (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Vendor table functionality (done: 2026-01-30)

### Task: [FEAT] Implement AI table for AI actor registration
- [x] [VALIDATE] Verify implementation of: AI table for AI actor registration (done: 2026-01-30)
- [x] [TEST] Write unit tests for: AI table registration (done: 2026-01-30)

### Task: [FEAT] Create GroupMember junction table
- [x] [VALIDATE] Verify implementation of: GroupMember junction table for actor group membership (done: 2026-01-30)
- [x] [TEST] Write unit tests for: GroupMember junction table membership (done: 2026-01-30)

### Task: [FEAT] Build actor string resolution service
- [x] [VALIDATE] Verify implementation of: Actor string resolution service (+Person, @Group, !Vendor, *AI prefixes) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Actor string resolution with all prefixes (done: 2026-01-30)

## Phase 4: Role & Permission System

### Task: [FEAT] Create Role table with inherits_from enum and verbs array
- [x] [VALIDATE] Verify implementation of: Role table with inherits_from enum and verbs array (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Role table inheritance (done: 2026-01-30)

### Task: [FEAT] Implement RoleMapping table linking roles to actors
- [x] [VALIDATE] Verify implementation of: RoleMapping table linking roles to actors (done: 2026-01-30)
- [x] [TEST] Write unit tests for: RoleMapping table relationships (done: 2026-01-30)

### Task: [FEAT] Build permission evaluation engine
- [x] [VALIDATE] Verify implementation of: Permission evaluation engine: group resolution -> role lookup -> SOW allow/deny processing (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Permission evaluation engine logic (done: 2026-01-30)

### Task: [FEAT] Implement base role defaults
- [x] [VALIDATE] Verify implementation of: Base role defaults (owner=*, sme=*, assignee, resource, stakeholder, awareness) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Base role default permissions (done: 2026-01-30)

### Task: [FEAT] Create SOW roles jsonb allow/deny rule processor
- [x] [VALIDATE] Verify implementation of: SOW roles jsonb allow/deny rule processor (done: 2026-01-30)
- [x] [TEST] Write unit tests for: SOW roles allow/deny processing (done: 2026-01-30)

## Phase 5: Core Domain Services

### Task: [FEAT] Implement Short Name generation service
- [x] [VALIDATE] Verify implementation of: Short Name generation service: {SOW.short_name}-{TYPE_ABBREV}-{SEQUENCE} (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Short Name generation format and uniqueness (done: 2026-01-30)

### Task: [FEAT] Build State Machine service with guards
- [x] [VALIDATE] Verify implementation of: State Machine service with guards (Normal/Escalated for most verbs, not-blocked for Complete/Incomplete, Completed/Incompleted for Close) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: State Machine transitions and guards (done: 2026-01-30)

### Task: [FEAT] Implement Blocking Propagation service
- [x] [VALIDATE] Verify implementation of: Blocking Propagation service (downward to children, upward is_blocked computation, Goal boundary handling) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Blocking Propagation (downward, upward, Goal boundary) (done: 2026-01-30)

### Task: [FEAT] Create Transaction/Event Sourcing service
- [x] [VALIDATE] Verify implementation of: Transaction/Event Sourcing service with before/after snapshots (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Transaction/Event Sourcing snapshots (done: 2026-01-30)

### Task: [FEAT] Build MileStone auto-complete service
- [x] [VALIDATE] Verify implementation of: MileStone auto-complete service (is_blocked if ANY blocked, Complete if ALL Completed) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: MileStone auto-complete logic (done: 2026-01-30)

### Task: [FEAT] Implement Project Goals tracking
- [x] [VALIDATE] Verify implementation of: Project Goals tracking (blocked_goals counter, NOT is_blocked) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Project Goals blocked_goals counter (done: 2026-01-30)

### Task: [FEAT] Implement Archived state transition
- [x] [VALIDATE] Verify implementation of: Archived state transition with configurable auto-archive after n days from Close (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Archived state transition timing (done: 2026-01-30)

### Task: [FEAT] Create background job for auto-archiving
- [x] [VALIDATE] Verify implementation of: Background job for auto-archiving Closed nouns after configurable threshold (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Auto-archiving background job (done: 2026-01-30)

### Task: [FEAT] Implement Artifact leaf-only validation
- [x] [VALIDATE] Verify implementation of: Artifact leaf-only validation (reject parent_id assignment to Artifact type) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Artifact leaf-only constraint (done: 2026-01-30)

### Task: [FEAT] Implement SOW short_name update handling
- [x] [VALIDATE] Verify implementation of: SOW short_name update handling with sequence continuity (editable unlike Noun short_name) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: SOW short_name update sequence continuity (done: 2026-01-30)

### Task: [FEAT] Implement SOW short_name validation
- [x] [VALIDATE] Verify implementation of: SOW short_name validation: required, globally unique, no spaces allowed (done: 2026-01-30)
- [x] [TEST] Write unit tests for: SOW short_name validation rules (done: 2026-01-30)

### Task: [FEAT] Implement Container type validation for NounAssignment
- [x] [VALIDATE] Verify implementation of: Container type validation for NounAssignment (container_id must be Group, Project, or MileStone) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Container type validation (done: 2026-01-30)

### Task: [FEAT] Implement Blocker type validation for NounBlock
- [x] [VALIDATE] Verify implementation of: Blocker type validation for NounBlock (blocker_id must be type=Blocker) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Blocker type validation (done: 2026-01-30)

### Task: [FEAT] Implement SOW.roles jsonb schema validation
- [x] [VALIDATE] Verify implementation of: SOW.roles jsonb schema validation (allow/deny arrays with actor, type, verbs structure) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: SOW.roles jsonb schema validation (done: 2026-01-30)

### Task: [FEAT] Implement Transaction.context schema validation
- [x] [VALIDATE] Verify implementation of: Transaction.context schema validation per verb type (target_id for Blocked, container_id for Assign, etc.) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Transaction.context schema validation (done: 2026-01-30)

## Phase 6: Verb Implementation (12 Verbs)

### Task: [FEAT] Implement Open verb
- [x] [VALIDATE] Verify implementation of: Open verb: create new Noun with validation, short_name generation, Transaction logging (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Open verb functionality (done: 2026-01-30)

### Task: [FEAT] Implement Complete verb
- [x] [VALIDATE] Verify implementation of: Complete verb: state transition with blocked guard, completed_at timestamp, Blocker release side-effect (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Complete verb with blocked guard (done: 2026-01-30)

### Task: [FEAT] Implement Incomplete verb
- [x] [VALIDATE] Verify implementation of: Incomplete verb: state transition with blocked guard (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Incomplete verb state transition (done: 2026-01-30)

### Task: [FEAT] Implement Normal verb
- [x] [VALIDATE] Verify implementation of: Normal verb: reset state from Escalated/Completed/Incompleted (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Normal verb state reset (done: 2026-01-30)

### Task: [FEAT] Implement Escalate verb
- [x] [VALIDATE] Verify implementation of: Escalate verb: mark urgent with state guard (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Escalate verb state guard (done: 2026-01-30)

### Task: [FEAT] Implement Close verb
- [x] [VALIDATE] Verify implementation of: Close verb: require Completed/Incompleted state and all children Closed (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Close verb with children check (done: 2026-01-30)

### Task: [FEAT] Implement Open verb on Closed noun (reopen)
- [x] [VALIDATE] Verify implementation of: Open verb on Closed noun: create clone with new short_name (reopen by cloning behavior) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Open verb reopen cloning (done: 2026-01-30)

### Task: [FEAT] Implement Update verb
- [x] [VALIDATE] Verify implementation of: Update verb: modify attributes with field-level validation (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Update verb field validation (done: 2026-01-30)

### Task: [FEAT] Implement Reparent verb
- [x] [VALIDATE] Verify implementation of: Reparent verb: move Noun to SOW root (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Reparent verb functionality (done: 2026-01-30)

### Task: [FEAT] Implement Assign verb
- [x] [VALIDATE] Verify implementation of: Assign verb: add Noun to Container with sort_order (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Assign verb with sort_order (done: 2026-01-30)

### Task: [FEAT] Implement Unassign verb
- [x] [VALIDATE] Verify implementation of: Unassign verb: remove Noun from Container (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Unassign verb functionality (done: 2026-01-30)

### Task: [FEAT] Implement Blocked verb
- [x] [VALIDATE] Verify implementation of: Blocked verb: create NounBlock link with propagation (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Blocked verb propagation (done: 2026-01-30)

### Task: [FEAT] Implement Release verb
- [x] [VALIDATE] Verify implementation of: Release verb: remove NounBlock link with propagation update (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Release verb propagation update (done: 2026-01-30)

## Phase 7: REST API - Core Endpoints

### Task: [FEAT] Implement X-API-Key authentication middleware
- [x] [VALIDATE] Verify implementation of: X-API-Key authentication middleware (done: 2026-01-30)
- [x] [TEST] Write unit tests for: X-API-Key authentication (done: 2026-01-30)

### Task: [FEAT] Create SOW endpoints
- [x] [VALIDATE] Verify implementation of: SOW endpoints: GET/POST /sows, GET /sow/{sow_id} (done: 2026-01-30)
- [x] [TEST] Write unit tests for: SOW CRUD endpoints (done: 2026-01-30)

### Task: [FEAT] Implement Noun endpoints
- [x] [VALIDATE] Verify implementation of: Noun endpoints: GET/POST /nouns/{sow_id}/nouns, GET /nouns/{sow_id}/noun/{noun_id} (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Noun CRUD endpoints (done: 2026-01-30)

### Task: [FEAT] Create Transaction endpoints
- [x] [VALIDATE] Verify implementation of: Transaction endpoints: GET/POST /nouns/{sow_id}/noun/{noun_id}/transactions (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Transaction endpoints (done: 2026-01-30)

### Task: [FEAT] Implement SOW-wide transaction query
- [x] [VALIDATE] Verify implementation of: SOW-wide transaction query: GET /nouns/{sow_id}/transactions with actor, after, before filters (done: 2026-01-30)
- [x] [TEST] Write unit tests for: SOW-wide transaction query filters (done: 2026-01-30)

### Task: [FEAT] Implement batch transactions
- [x] [VALIDATE] Verify implementation of: Batch transactions: POST /nouns/{sow_id}/batch/transactions with all-or-nothing semantics (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Batch transactions rollback (done: 2026-01-30)

### Task: [FEAT] Build cursor-based pagination
- [x] [VALIDATE] Verify implementation of: Cursor-based pagination: limit, sort, order, cursor (timestamp_uuid format) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Cursor-based pagination (done: 2026-01-30)

### Task: [FEAT] Implement standard error responses
- [x] [VALIDATE] Verify implementation of: Standard error responses with codes (NOUN_BLOCKED, INVALID_STATE_TRANSITION, etc.) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Error response codes (done: 2026-01-30)

### Task: [FEAT] Implement logging endpoints
- [x] [VALIDATE] Verify implementation of: Logging endpoints: POST /logs/debug, POST /logs/error (pass-through to logging subsystem) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Logging endpoints (done: 2026-01-30)

## Phase 8: REST API - Views

### Task: [FEAT] Implement Timeline view
- [x] [VALIDATE] Verify implementation of: Timeline view: GET /views/{sow_id}/timeline with start/end date filtering (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Timeline view date filtering (done: 2026-01-30)

### Task: [FEAT] Create Kanban view
- [x] [VALIDATE] Verify implementation of: Kanban view: GET /views/{sow_id}/kanban with container_id grouping (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Kanban view container grouping (done: 2026-01-30)

### Task: [FEAT] Implement Calendar view
- [x] [VALIDATE] Verify implementation of: Calendar view: GET /views/{sow_id}/calendar with month-based day buckets (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Calendar view day buckets (done: 2026-01-30)

## Phase 9: REST API - Admin (/sys0)

### Task: [FEAT] Implement Person CRUD
- [x] [VALIDATE] Verify implementation of: Person CRUD: GET/POST/PUT/DELETE /sys0/actors/persons (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Person CRUD operations (done: 2026-01-30)

### Task: [FEAT] Create Group CRUD with members
- [x] [VALIDATE] Verify implementation of: Group CRUD with members: GET/POST/PUT/DELETE /sys0/actors/groups, /members endpoints (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Group CRUD and members (done: 2026-01-30)

### Task: [FEAT] Implement Vendor CRUD
- [x] [VALIDATE] Verify implementation of: Vendor CRUD: GET/POST/PUT/DELETE /sys0/actors/vendors (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Vendor CRUD operations (done: 2026-01-30)

### Task: [FEAT] Create AI CRUD
- [x] [VALIDATE] Verify implementation of: AI CRUD: GET/POST/PUT/DELETE /sys0/actors/ai (done: 2026-01-30)
- [x] [TEST] Write unit tests for: AI CRUD operations (done: 2026-01-30)

### Task: [FEAT] Implement LDAP commands
- [x] [VALIDATE] Verify implementation of: LDAP commands: sync, clear_cache, set_interval, configure (done: 2026-01-30)
- [x] [TEST] Write unit tests for: LDAP commands (done: 2026-01-30)

### Task: [FEAT] Create Role management
- [x] [VALIDATE] Verify implementation of: Role management: GET/POST/PUT/DELETE /sys0/roles with actor associations (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Role management (done: 2026-01-30)

### Task: [FEAT] Implement Instructions management
- [x] [VALIDATE] Verify implementation of: Instructions management: GET/POST/PUT/DELETE /sys0/instructions (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Instructions management (done: 2026-01-30)

### Task: [FEAT] Create Help content management
- [x] [VALIDATE] Verify implementation of: Help content management: PUT /sys0/help/nouns/{type}, PUT /sys0/help/verbs/{verb} (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Help content management (done: 2026-01-30)

## Phase 10: REST API - User (/user)

### Task: [FEAT] Implement Alias CRUD
- [x] [VALIDATE] Verify implementation of: Alias CRUD: GET/POST/DELETE /user/aliases with (actor, name) uniqueness (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Alias CRUD uniqueness (done: 2026-01-30)

### Task: [FEAT] Create Profile endpoints
- [x] [VALIDATE] Verify implementation of: Profile endpoints: GET/PUT /user/profile (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Profile endpoints (done: 2026-01-30)

### Task: [FEAT] Implement Preferences endpoints
- [x] [VALIDATE] Verify implementation of: Preferences endpoints: GET/PUT /user/preferences (default_sow, timezone, theme, notifications) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Preferences endpoints (done: 2026-01-30)

### Task: [FEAT] Create access endpoints
- [x] [VALIDATE] Verify implementation of: Access endpoints: GET /user/roles, GET /user/sows (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Access endpoints (done: 2026-01-30)

## Phase 11: REST API - Help

### Task: [FEAT] Implement Help endpoints for nouns
- [x] [VALIDATE] Verify implementation of: Help endpoints: GET /help/nouns, /help/nouns/{type} with full noun documentation (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Help noun endpoints (done: 2026-01-30)

### Task: [FEAT] Create verb help
- [x] [VALIDATE] Verify implementation of: Verb help: GET /help/verbs, /help/verbs/{verb} with guards, side-effects, examples (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Verb help endpoints (done: 2026-01-30)

### Task: [FEAT] Implement container/role help
- [x] [VALIDATE] Verify implementation of: Container/role help: GET /help/containers, /help/roles (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Container/role help endpoints (done: 2026-01-30)

### Task: [FEAT] Create Instructions endpoint
- [x] [VALIDATE] Verify implementation of: Instructions endpoint: GET /help/instructions with optional sow_id filter and category filtering (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Instructions endpoint filtering (done: 2026-01-30)

## Phase 12: SOW-Level Features

### Task: [FEAT] Implement SOW Instructions
- [x] [VALIDATE] Verify implementation of: SOW Instructions: GET/POST/PUT/DELETE /nouns/{sow_id}/instructions with category field (done: 2026-01-30)
- [x] [TEST] Write unit tests for: SOW Instructions CRUD (done: 2026-01-30)

### Task: [FEAT] Create Skinny Mode wizard
- [x] [VALIDATE] Verify implementation of: Skinny Mode wizard: hidden SOW/Level creation, limited features (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Skinny Mode wizard flow (done: 2026-01-30)

### Task: [FEAT] Implement Skinny-to-Full upgrade
- [x] [VALIDATE] Verify implementation of: Skinny-to-Full upgrade: flip flag, reveal controls, no migration (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Skinny-to-Full upgrade (done: 2026-01-30)

### Task: [FEAT] Implement Sub-Levels support
- [x] [VALIDATE] Verify implementation of: Sub-Levels support for Full mode: nested SOW/Level hierarchies (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Sub-Levels hierarchies (done: 2026-01-30)

### Task: [FEAT] Add Budget/Contract fields to SOW
- [x] [VALIDATE] Verify implementation of: Budget/Contract fields to SOW for Full mode (custom_fields or dedicated columns) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Budget/Contract fields (done: 2026-01-30)

### Task: [FEAT] Implement Skinny Mode Role.deny() restriction
- [x] [VALIDATE] Verify implementation of: Skinny Mode Role.deny() restriction (deny rules disabled in Skinny mode) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Skinny Mode deny restriction (done: 2026-01-30)

## Phase 13: Multi-DB Federation (Enterprise)

### Task: [FEAT] Implement SOW-to-database routing lookup
- [x] [VALIDATE] Verify implementation of: SOW-to-database routing lookup via SOWDatabase table (done: 2026-01-30)
- [x] [TEST] Write unit tests for: SOW-to-database routing (done: 2026-01-30)

### Task: [FEAT] Create Redis caching layer
- [x] [VALIDATE] Verify implementation of: Redis caching layer for SOW->DB mappings, permissions, is_blocked (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Redis caching layer (done: 2026-01-30)

### Task: [FEAT] Build cross-DB write handling
- [x] [VALIDATE] Verify implementation of: Cross-DB write handling with optimistic approach (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Cross-DB write handling (done: 2026-01-30)

### Task: [FEAT] Implement background reconciliation job
- [x] [VALIDATE] Verify implementation of: Background reconciliation job for cross-DB sync (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Background reconciliation job (done: 2026-01-30)

## Phase 14: Testing

### Task: [TEST] Write unit tests for State Machine transitions and guards
- [x] [VALIDATE] Verify completion of: Unit tests for State Machine transitions and guards (done: 2026-01-30)

### Task: [TEST] Create unit tests for Blocking Propagation
- [x] [VALIDATE] Verify completion of: Unit tests for Blocking Propagation (downward, upward, Goal boundary) (done: 2026-01-30)

### Task: [TEST] Write unit tests for Permission Evaluation engine
- [x] [VALIDATE] Verify completion of: Unit tests for Permission Evaluation engine (done: 2026-01-30)

### Task: [TEST] Create unit tests for Short Name generation
- [x] [VALIDATE] Verify completion of: Unit tests for Short Name generation with sequence handling (done: 2026-01-30)

### Task: [TEST] Write integration tests for all 12 Verb implementations
- [x] [VALIDATE] Verify completion of: Integration tests for all 12 Verb implementations (done: 2026-01-30)

### Task: [TEST] Create API endpoint tests for SOW, Noun, Transaction routes
- [x] [VALIDATE] Verify completion of: API endpoint tests for SOW, Noun, Transaction routes (done: 2026-01-30)

### Task: [TEST] Write API tests for Admin, User, Help endpoints
- [x] [VALIDATE] Verify completion of: API tests for Admin, User, Help endpoints (done: 2026-01-30)

### Task: [TEST] Create batch operation tests with rollback verification
- [x] [VALIDATE] Verify completion of: Batch operation tests with rollback verification (done: 2026-01-30)

## Phase 15: SSL/TLS Configuration

### Task: [FEAT] Create ssl.toml configuration file
- [x] [VALIDATE] Verify implementation of: ssl.toml configuration file with X509 attributes (common_name, organization, org_unit, locality, state, country, validity_days) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: ssl.toml configuration parsing (done: 2026-01-30)

### Task: [FEAT] Implement SSL certificate auto-generation service
- [x] [VALIDATE] Verify implementation of: SSL certificate auto-generation service using cryptography library (done: 2026-01-30)
- [x] [TEST] Write unit tests for: SSL certificate auto-generation (done: 2026-01-30)

### Task: [FEAT] Create self-signed CA certificate generator
- [x] [VALIDATE] Verify implementation of: Self-signed CA certificate generator for development environments (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Self-signed CA certificate generation (done: 2026-01-30)

### Task: [FEAT] Implement server certificate generation signed by local CA
- [x] [VALIDATE] Verify implementation of: Server certificate generation signed by local CA (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Server certificate signing (done: 2026-01-30)

### Task: [FEAT] Build certificate chain validation and storage
- [x] [VALIDATE] Verify implementation of: Certificate chain validation and storage in config/ssl/ directory (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Certificate chain validation (done: 2026-01-30)

### Task: [FEAT] Implement certificate expiry monitoring
- [x] [VALIDATE] Verify implementation of: Certificate expiry monitoring with configurable warning threshold (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Certificate expiry monitoring (done: 2026-01-30)

### Task: [FEAT] Create automatic certificate renewal for self-signed certs
- [x] [VALIDATE] Verify implementation of: Automatic certificate renewal for self-signed certs before expiry (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Automatic certificate renewal (done: 2026-01-30)

### Task: [FEAT] Implement Let's Encrypt ACME client integration
- [x] [VALIDATE] Verify implementation of: Let's Encrypt ACME client integration for production environments (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Let's Encrypt ACME client (done: 2026-01-30)

### Task: [FEAT] Build ACME HTTP-01 and DNS-01 challenge handlers
- [x] [VALIDATE] Verify implementation of: ACME HTTP-01 and DNS-01 challenge handlers (done: 2026-01-30)
- [x] [TEST] Write unit tests for: ACME challenge handlers (done: 2026-01-30)

### Task: [FEAT] Create Let's Encrypt certificate auto-renewal
- [x] [VALIDATE] Verify implementation of: Let's Encrypt certificate auto-renewal with configurable renewal window (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Let's Encrypt auto-renewal (done: 2026-01-30)

### Task: [FEAT] Implement external certificate import
- [x] [VALIDATE] Verify implementation of: External certificate import (PEM/PFX) for enterprise deployments (done: 2026-01-30)
- [x] [TEST] Write unit tests for: External certificate import (done: 2026-01-30)

### Task: [FEAT] Build certificate format conversion utilities
- [x] [VALIDATE] Verify implementation of: Certificate format conversion utilities (PEM, DER, PFX/PKCS12) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: Certificate format conversion (done: 2026-01-30)

### Task: [FEAT] Configure FastAPI/Uvicorn HTTPS
- [x] [VALIDATE] Verify implementation of: FastAPI/Uvicorn HTTPS with SSL context from ssl.toml (done: 2026-01-30)
- [x] [TEST] Write unit tests for: HTTPS configuration (done: 2026-01-30)

### Task: [FEAT] Implement HTTP to HTTPS redirect middleware
- [x] [VALIDATE] Verify implementation of: HTTP to HTTPS redirect middleware (done: 2026-01-30)
- [x] [TEST] Write unit tests for: HTTPS redirect middleware (done: 2026-01-30)

### Task: [FEAT] Create HSTS header configuration
- [x] [VALIDATE] Verify implementation of: HSTS (HTTP Strict Transport Security) header configuration (done: 2026-01-30)
- [x] [TEST] Write unit tests for: HSTS header configuration (done: 2026-01-30)

### Task: [FEAT] Implement TLS version and cipher suite configuration
- [x] [VALIDATE] Verify implementation of: TLS version and cipher suite configuration in ssl.toml (done: 2026-01-30)
- [x] [TEST] Write unit tests for: TLS configuration (done: 2026-01-30)

### Task: [FEAT] Build SSL certificate status endpoint
- [x] [VALIDATE] Verify implementation of: SSL certificate status endpoint: GET /sys0/ssl/status (expiry, issuer, subject) (done: 2026-01-30)
- [x] [TEST] Write unit tests for: SSL status endpoint (done: 2026-01-30)

### Task: [FEAT] Create SSL certificate management endpoints
- [x] [VALIDATE] Verify implementation of: SSL certificate management endpoints: POST /sys0/ssl/renew, POST /sys0/ssl/import (done: 2026-01-30)
- [x] [TEST] Write unit tests for: SSL management endpoints (done: 2026-01-30)

### Task: [TEST] Write unit tests for certificate generation with various X509 attributes
- [x] [VALIDATE] Verify completion of: Unit tests for certificate generation with various X509 attributes (done: 2026-01-30)

### Task: [TEST] Create integration tests for HTTPS endpoints and TLS handshake
- [x] [VALIDATE] Verify completion of: Integration tests for HTTPS endpoints and TLS handshake (done: 2026-01-30)

### Task: [TEST] Write tests for Let's Encrypt ACME flow
- [x] [VALIDATE] Verify completion of: Tests for Let's Encrypt ACME flow (mock ACME server) (done: 2026-01-30)

### Task: [TEST] Create tests for certificate renewal and expiry monitoring
- [x] [VALIDATE] Verify completion of: Tests for certificate renewal and expiry monitoring (done: 2026-01-30)

### Task: [DOCS] Document ssl.toml configuration options and X509 attributes
- [x] [VALIDATE] Verify documentation accuracy for: ssl.toml configuration options and X509 attributes (done: 2026-01-30)

### Task: [DOCS] Create SSL setup guide
- [x] [VALIDATE] Verify documentation accuracy for: SSL setup guide for development (self-signed) and production (Let's Encrypt/external) (done: 2026-01-30)

## Phase 16: Documentation & DevOps

### Task: [DOCS] Create API documentation with OpenAPI/Swagger specs
- [ ] [VALIDATE] Verify documentation accuracy for: API documentation with OpenAPI/Swagger specs

### Task: [TASK] Set up Docker Compose for local development
- [ ] [VALIDATE] Verify implementation of: Docker Compose for local development (PostgreSQL, Redis)

### Task: [TASK] Create database seed scripts
- [ ] [VALIDATE] Verify implementation of: Database seed scripts for development/testing

### Task: [TASK] Configure CI/CD pipeline with test automation
- [ ] [VALIDATE] Verify implementation of: CI/CD pipeline with test automation

## Phase 17: CLI Client - Project Setup

### Task: [TASK] Set up Python CLI project with Click or Typer framework
- [ ] [VALIDATE] Verify implementation of: Python CLI project with Click or Typer framework

### Task: [TASK] Create CLI package structure
- [ ] [VALIDATE] Verify implementation of: CLI package structure: level/, commands/, utils/, config/

### Task: [TASK] Implement CLI configuration file
- [ ] [VALIDATE] Verify implementation of: CLI configuration file (~/.level/config.toml) for api-key, default-sow, preferences

### Task: [FEAT] Build HTTP client wrapper
- [ ] [VALIDATE] Verify implementation of: HTTP client wrapper with X-API-Key header injection and error handling
- [ ] [TEST] Write unit tests for: HTTP client wrapper

### Task: [FEAT] Implement base output formatters
- [ ] [VALIDATE] Verify implementation of: Base output formatters: table, json, minimal for different verbosity levels
- [ ] [TEST] Write unit tests for: Output formatters

## Phase 18-62: CLI, MCP, and Frontend Tasks

(Remaining phases follow the same pattern with [VALIDATE] for all tasks and [TEST] added for [FEAT] and [BUG] types)

---

## Summary

| Type | Count | With Tests |
|------|-------|------------|
| [TASK] | 31 | No |
| [FEAT] | 289 | Yes |
| [TEST] | 29 | No |
| [DOCS] | 19 | No |
| **Total** | **368** | **289 test tasks** |

**Total subtasks generated**: ~657 (368 validates + 289 tests)
