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
- [ ] [VALIDATE] Verify implementation of: Create Noun table with all 12 types (SOW, Item, Task, Request, Meeting, Deliverable, Event, Blocker, Artifact, Group, Project, MileStone)
- [ ] [TEST] Write unit tests for: Noun table creation and type constraints

### Task: [FEAT] Implement NounActor junction table for M:N actor-role relationships
- [ ] [VALIDATE] Verify implementation of: NounActor junction table for M:N actor-role relationships (owner, assignee, awareness, sme, stakeholder, resource)
- [ ] [TEST] Write unit tests for: NounActor junction table relationships

### Task: [FEAT] Create NounAssignment table for Container assignments
- [ ] [VALIDATE] Verify implementation of: NounAssignment table for Container assignments with sort_order for Kanban
- [ ] [TEST] Write unit tests for: NounAssignment table and sort_order functionality

### Task: [FEAT] Implement NounBlock table for M:N blocker-target relationships
- [ ] [VALIDATE] Verify implementation of: NounBlock table for M:N blocker-target relationships
- [ ] [TEST] Write unit tests for: NounBlock table relationships

### Task: [FEAT] Create NounHashTag table for global tagging system
- [ ] [VALIDATE] Verify implementation of: NounHashTag table for global tagging system
- [ ] [TEST] Write unit tests for: NounHashTag table functionality

### Task: [FEAT] Implement Transaction table with event sourcing
- [ ] [VALIDATE] Verify implementation of: Transaction table with sequence, verb, actor, before/after snapshots, context jsonb
- [ ] [TEST] Write unit tests for: Transaction table and event sourcing

### Task: [FEAT] Create NounSequence table for short_name generation
- [ ] [VALIDATE] Verify implementation of: NounSequence table for per-SOW+type short_name generation
- [ ] [TEST] Write unit tests for: NounSequence table and sequence generation

### Task: [FEAT] Implement SOWDatabase table for multi-DB federation routing
- [ ] [VALIDATE] Verify implementation of: SOWDatabase table for multi-DB federation routing
- [ ] [TEST] Write unit tests for: SOWDatabase table routing

### Task: [FEAT] Create Alias table
- [ ] [VALIDATE] Verify implementation of: Alias table with id, actor, name, noun_id, created_at and unique (actor, name) constraint
- [ ] [TEST] Write unit tests for: Alias table uniqueness constraints

### Task: [FEAT] Create Instruction table
- [ ] [VALIDATE] Verify implementation of: Instruction table with id, scope (global/sow), sow_id, category, title, content, applies_to, created_by, updated_at
- [ ] [TEST] Write unit tests for: Instruction table scope and category handling

### Task: [FEAT] Create HelpContent table or static config
- [ ] [VALIDATE] Verify implementation of: HelpContent table or static config for noun/verb documentation (type, abbrev, description, guards, side_effects, examples)
- [ ] [TEST] Write unit tests for: HelpContent data structure

### Task: [FEAT] Create UserPreferences table
- [ ] [VALIDATE] Verify implementation of: UserPreferences table or jsonb field for storing user profile and preferences
- [ ] [TEST] Write unit tests for: UserPreferences storage and retrieval

## Phase 3: Actor System

### Task: [FEAT] Create Person table with internal/LDAP source support
- [ ] [VALIDATE] Verify implementation of: Person table with internal/LDAP source support
- [ ] [TEST] Write unit tests for: Person table LDAP integration

### Task: [FEAT] Implement Group table with role association
- [ ] [VALIDATE] Verify implementation of: Group table with role association
- [ ] [TEST] Write unit tests for: Group table role associations

### Task: [FEAT] Create Vendor table for external vendor actors
- [ ] [VALIDATE] Verify implementation of: Vendor table for external vendor actors
- [ ] [TEST] Write unit tests for: Vendor table functionality

### Task: [FEAT] Implement AI table for AI actor registration
- [ ] [VALIDATE] Verify implementation of: AI table for AI actor registration
- [ ] [TEST] Write unit tests for: AI table registration

### Task: [FEAT] Create GroupMember junction table
- [ ] [VALIDATE] Verify implementation of: GroupMember junction table for actor group membership
- [ ] [TEST] Write unit tests for: GroupMember junction table membership

### Task: [FEAT] Build actor string resolution service
- [ ] [VALIDATE] Verify implementation of: Actor string resolution service (+Person, @Group, !Vendor, *AI prefixes)
- [ ] [TEST] Write unit tests for: Actor string resolution with all prefixes

## Phase 4: Role & Permission System

### Task: [FEAT] Create Role table with inherits_from enum and verbs array
- [ ] [VALIDATE] Verify implementation of: Role table with inherits_from enum and verbs array
- [ ] [TEST] Write unit tests for: Role table inheritance

### Task: [FEAT] Implement RoleMapping table linking roles to actors
- [ ] [VALIDATE] Verify implementation of: RoleMapping table linking roles to actors
- [ ] [TEST] Write unit tests for: RoleMapping table relationships

### Task: [FEAT] Build permission evaluation engine
- [ ] [VALIDATE] Verify implementation of: Permission evaluation engine: group resolution -> role lookup -> SOW allow/deny processing
- [ ] [TEST] Write unit tests for: Permission evaluation engine logic

### Task: [FEAT] Implement base role defaults
- [ ] [VALIDATE] Verify implementation of: Base role defaults (owner=*, sme=*, assignee, resource, stakeholder, awareness)
- [ ] [TEST] Write unit tests for: Base role default permissions

### Task: [FEAT] Create SOW roles jsonb allow/deny rule processor
- [ ] [VALIDATE] Verify implementation of: SOW roles jsonb allow/deny rule processor
- [ ] [TEST] Write unit tests for: SOW roles allow/deny processing

## Phase 5: Core Domain Services

### Task: [FEAT] Implement Short Name generation service
- [ ] [VALIDATE] Verify implementation of: Short Name generation service: {SOW.short_name}-{TYPE_ABBREV}-{SEQUENCE}
- [ ] [TEST] Write unit tests for: Short Name generation format and uniqueness

### Task: [FEAT] Build State Machine service with guards
- [ ] [VALIDATE] Verify implementation of: State Machine service with guards (Normal/Escalated for most verbs, not-blocked for Complete/Incomplete, Completed/Incompleted for Close)
- [ ] [TEST] Write unit tests for: State Machine transitions and guards

### Task: [FEAT] Implement Blocking Propagation service
- [ ] [VALIDATE] Verify implementation of: Blocking Propagation service (downward to children, upward is_blocked computation, Goal boundary handling)
- [ ] [TEST] Write unit tests for: Blocking Propagation (downward, upward, Goal boundary)

### Task: [FEAT] Create Transaction/Event Sourcing service
- [ ] [VALIDATE] Verify implementation of: Transaction/Event Sourcing service with before/after snapshots
- [ ] [TEST] Write unit tests for: Transaction/Event Sourcing snapshots

### Task: [FEAT] Build MileStone auto-complete service
- [ ] [VALIDATE] Verify implementation of: MileStone auto-complete service (is_blocked if ANY blocked, Complete if ALL Completed)
- [ ] [TEST] Write unit tests for: MileStone auto-complete logic

### Task: [FEAT] Implement Project Goals tracking
- [ ] [VALIDATE] Verify implementation of: Project Goals tracking (blocked_goals counter, NOT is_blocked)
- [ ] [TEST] Write unit tests for: Project Goals blocked_goals counter

### Task: [FEAT] Implement Archived state transition
- [ ] [VALIDATE] Verify implementation of: Archived state transition with configurable auto-archive after n days from Close
- [ ] [TEST] Write unit tests for: Archived state transition timing

### Task: [FEAT] Create background job for auto-archiving
- [ ] [VALIDATE] Verify implementation of: Background job for auto-archiving Closed nouns after configurable threshold
- [ ] [TEST] Write unit tests for: Auto-archiving background job

### Task: [FEAT] Implement Artifact leaf-only validation
- [ ] [VALIDATE] Verify implementation of: Artifact leaf-only validation (reject parent_id assignment to Artifact type)
- [ ] [TEST] Write unit tests for: Artifact leaf-only constraint

### Task: [FEAT] Implement SOW short_name update handling
- [ ] [VALIDATE] Verify implementation of: SOW short_name update handling with sequence continuity (editable unlike Noun short_name)
- [ ] [TEST] Write unit tests for: SOW short_name update sequence continuity

### Task: [FEAT] Implement SOW short_name validation
- [ ] [VALIDATE] Verify implementation of: SOW short_name validation: required, globally unique, no spaces allowed
- [ ] [TEST] Write unit tests for: SOW short_name validation rules

### Task: [FEAT] Implement Container type validation for NounAssignment
- [ ] [VALIDATE] Verify implementation of: Container type validation for NounAssignment (container_id must be Group, Project, or MileStone)
- [ ] [TEST] Write unit tests for: Container type validation

### Task: [FEAT] Implement Blocker type validation for NounBlock
- [ ] [VALIDATE] Verify implementation of: Blocker type validation for NounBlock (blocker_id must be type=Blocker)
- [ ] [TEST] Write unit tests for: Blocker type validation

### Task: [FEAT] Implement SOW.roles jsonb schema validation
- [ ] [VALIDATE] Verify implementation of: SOW.roles jsonb schema validation (allow/deny arrays with actor, type, verbs structure)
- [ ] [TEST] Write unit tests for: SOW.roles jsonb schema validation

### Task: [FEAT] Implement Transaction.context schema validation
- [ ] [VALIDATE] Verify implementation of: Transaction.context schema validation per verb type (target_id for Blocked, container_id for Assign, etc.)
- [ ] [TEST] Write unit tests for: Transaction.context schema validation

## Phase 6: Verb Implementation (12 Verbs)

### Task: [FEAT] Implement Open verb
- [ ] [VALIDATE] Verify implementation of: Open verb: create new Noun with validation, short_name generation, Transaction logging
- [ ] [TEST] Write unit tests for: Open verb functionality

### Task: [FEAT] Implement Complete verb
- [ ] [VALIDATE] Verify implementation of: Complete verb: state transition with blocked guard, completed_at timestamp, Blocker release side-effect
- [ ] [TEST] Write unit tests for: Complete verb with blocked guard

### Task: [FEAT] Implement Incomplete verb
- [ ] [VALIDATE] Verify implementation of: Incomplete verb: state transition with blocked guard
- [ ] [TEST] Write unit tests for: Incomplete verb state transition

### Task: [FEAT] Implement Normal verb
- [ ] [VALIDATE] Verify implementation of: Normal verb: reset state from Escalated/Completed/Incompleted
- [ ] [TEST] Write unit tests for: Normal verb state reset

### Task: [FEAT] Implement Escalate verb
- [ ] [VALIDATE] Verify implementation of: Escalate verb: mark urgent with state guard
- [ ] [TEST] Write unit tests for: Escalate verb state guard

### Task: [FEAT] Implement Close verb
- [ ] [VALIDATE] Verify implementation of: Close verb: require Completed/Incompleted state and all children Closed
- [ ] [TEST] Write unit tests for: Close verb with children check

### Task: [FEAT] Implement Open verb on Closed noun (reopen)
- [ ] [VALIDATE] Verify implementation of: Open verb on Closed noun: create clone with new short_name (reopen by cloning behavior)
- [ ] [TEST] Write unit tests for: Open verb reopen cloning

### Task: [FEAT] Implement Update verb
- [ ] [VALIDATE] Verify implementation of: Update verb: modify attributes with field-level validation
- [ ] [TEST] Write unit tests for: Update verb field validation

### Task: [FEAT] Implement Reparent verb
- [ ] [VALIDATE] Verify implementation of: Reparent verb: move Noun to SOW root
- [ ] [TEST] Write unit tests for: Reparent verb functionality

### Task: [FEAT] Implement Assign verb
- [ ] [VALIDATE] Verify implementation of: Assign verb: add Noun to Container with sort_order
- [ ] [TEST] Write unit tests for: Assign verb with sort_order

### Task: [FEAT] Implement Unassign verb
- [ ] [VALIDATE] Verify implementation of: Unassign verb: remove Noun from Container
- [ ] [TEST] Write unit tests for: Unassign verb functionality

### Task: [FEAT] Implement Blocked verb
- [ ] [VALIDATE] Verify implementation of: Blocked verb: create NounBlock link with propagation
- [ ] [TEST] Write unit tests for: Blocked verb propagation

### Task: [FEAT] Implement Release verb
- [ ] [VALIDATE] Verify implementation of: Release verb: remove NounBlock link with propagation update
- [ ] [TEST] Write unit tests for: Release verb propagation update

## Phase 7: REST API - Core Endpoints

### Task: [FEAT] Implement X-API-Key authentication middleware
- [ ] [VALIDATE] Verify implementation of: X-API-Key authentication middleware
- [ ] [TEST] Write unit tests for: X-API-Key authentication

### Task: [FEAT] Create SOW endpoints
- [ ] [VALIDATE] Verify implementation of: SOW endpoints: GET/POST /sows, GET /sow/{sow_id}
- [ ] [TEST] Write unit tests for: SOW CRUD endpoints

### Task: [FEAT] Implement Noun endpoints
- [ ] [VALIDATE] Verify implementation of: Noun endpoints: GET/POST /nouns/{sow_id}/nouns, GET /nouns/{sow_id}/noun/{noun_id}
- [ ] [TEST] Write unit tests for: Noun CRUD endpoints

### Task: [FEAT] Create Transaction endpoints
- [ ] [VALIDATE] Verify implementation of: Transaction endpoints: GET/POST /nouns/{sow_id}/noun/{noun_id}/transactions
- [ ] [TEST] Write unit tests for: Transaction endpoints

### Task: [FEAT] Implement SOW-wide transaction query
- [ ] [VALIDATE] Verify implementation of: SOW-wide transaction query: GET /nouns/{sow_id}/transactions with actor, after, before filters
- [ ] [TEST] Write unit tests for: SOW-wide transaction query filters

### Task: [FEAT] Implement batch transactions
- [ ] [VALIDATE] Verify implementation of: Batch transactions: POST /nouns/{sow_id}/batch/transactions with all-or-nothing semantics
- [ ] [TEST] Write unit tests for: Batch transactions rollback

### Task: [FEAT] Build cursor-based pagination
- [ ] [VALIDATE] Verify implementation of: Cursor-based pagination: limit, sort, order, cursor (timestamp_uuid format)
- [ ] [TEST] Write unit tests for: Cursor-based pagination

### Task: [FEAT] Implement standard error responses
- [ ] [VALIDATE] Verify implementation of: Standard error responses with codes (NOUN_BLOCKED, INVALID_STATE_TRANSITION, etc.)
- [ ] [TEST] Write unit tests for: Error response codes

### Task: [FEAT] Implement logging endpoints
- [ ] [VALIDATE] Verify implementation of: Logging endpoints: POST /logs/debug, POST /logs/error (pass-through to logging subsystem)
- [ ] [TEST] Write unit tests for: Logging endpoints

## Phase 8: REST API - Views

### Task: [FEAT] Implement Timeline view
- [ ] [VALIDATE] Verify implementation of: Timeline view: GET /views/{sow_id}/timeline with start/end date filtering
- [ ] [TEST] Write unit tests for: Timeline view date filtering

### Task: [FEAT] Create Kanban view
- [ ] [VALIDATE] Verify implementation of: Kanban view: GET /views/{sow_id}/kanban with container_id grouping
- [ ] [TEST] Write unit tests for: Kanban view container grouping

### Task: [FEAT] Implement Calendar view
- [ ] [VALIDATE] Verify implementation of: Calendar view: GET /views/{sow_id}/calendar with month-based day buckets
- [ ] [TEST] Write unit tests for: Calendar view day buckets

## Phase 9: REST API - Admin (/sys0)

### Task: [FEAT] Implement Person CRUD
- [ ] [VALIDATE] Verify implementation of: Person CRUD: GET/POST/PUT/DELETE /sys0/actors/persons
- [ ] [TEST] Write unit tests for: Person CRUD operations

### Task: [FEAT] Create Group CRUD with members
- [ ] [VALIDATE] Verify implementation of: Group CRUD with members: GET/POST/PUT/DELETE /sys0/actors/groups, /members endpoints
- [ ] [TEST] Write unit tests for: Group CRUD and members

### Task: [FEAT] Implement Vendor CRUD
- [ ] [VALIDATE] Verify implementation of: Vendor CRUD: GET/POST/PUT/DELETE /sys0/actors/vendors
- [ ] [TEST] Write unit tests for: Vendor CRUD operations

### Task: [FEAT] Create AI CRUD
- [ ] [VALIDATE] Verify implementation of: AI CRUD: GET/POST/PUT/DELETE /sys0/actors/ai
- [ ] [TEST] Write unit tests for: AI CRUD operations

### Task: [FEAT] Implement LDAP commands
- [ ] [VALIDATE] Verify implementation of: LDAP commands: sync, clear_cache, set_interval, configure
- [ ] [TEST] Write unit tests for: LDAP commands

### Task: [FEAT] Create Role management
- [ ] [VALIDATE] Verify implementation of: Role management: GET/POST/PUT/DELETE /sys0/roles with actor associations
- [ ] [TEST] Write unit tests for: Role management

### Task: [FEAT] Implement Instructions management
- [ ] [VALIDATE] Verify implementation of: Instructions management: GET/POST/PUT/DELETE /sys0/instructions
- [ ] [TEST] Write unit tests for: Instructions management

### Task: [FEAT] Create Help content management
- [ ] [VALIDATE] Verify implementation of: Help content management: PUT /sys0/help/nouns/{type}, PUT /sys0/help/verbs/{verb}
- [ ] [TEST] Write unit tests for: Help content management

## Phase 10: REST API - User (/user)

### Task: [FEAT] Implement Alias CRUD
- [ ] [VALIDATE] Verify implementation of: Alias CRUD: GET/POST/DELETE /user/aliases with (actor, name) uniqueness
- [ ] [TEST] Write unit tests for: Alias CRUD uniqueness

### Task: [FEAT] Create Profile endpoints
- [ ] [VALIDATE] Verify implementation of: Profile endpoints: GET/PUT /user/profile
- [ ] [TEST] Write unit tests for: Profile endpoints

### Task: [FEAT] Implement Preferences endpoints
- [ ] [VALIDATE] Verify implementation of: Preferences endpoints: GET/PUT /user/preferences (default_sow, timezone, theme, notifications)
- [ ] [TEST] Write unit tests for: Preferences endpoints

### Task: [FEAT] Create access endpoints
- [ ] [VALIDATE] Verify implementation of: Access endpoints: GET /user/roles, GET /user/sows
- [ ] [TEST] Write unit tests for: Access endpoints

## Phase 11: REST API - Help

### Task: [FEAT] Implement Help endpoints for nouns
- [ ] [VALIDATE] Verify implementation of: Help endpoints: GET /help/nouns, /help/nouns/{type} with full noun documentation
- [ ] [TEST] Write unit tests for: Help noun endpoints

### Task: [FEAT] Create verb help
- [ ] [VALIDATE] Verify implementation of: Verb help: GET /help/verbs, /help/verbs/{verb} with guards, side-effects, examples
- [ ] [TEST] Write unit tests for: Verb help endpoints

### Task: [FEAT] Implement container/role help
- [ ] [VALIDATE] Verify implementation of: Container/role help: GET /help/containers, /help/roles
- [ ] [TEST] Write unit tests for: Container/role help endpoints

### Task: [FEAT] Create Instructions endpoint
- [ ] [VALIDATE] Verify implementation of: Instructions endpoint: GET /help/instructions with optional sow_id filter and category filtering
- [ ] [TEST] Write unit tests for: Instructions endpoint filtering

## Phase 12: SOW-Level Features

### Task: [FEAT] Implement SOW Instructions
- [ ] [VALIDATE] Verify implementation of: SOW Instructions: GET/POST/PUT/DELETE /nouns/{sow_id}/instructions with category field
- [ ] [TEST] Write unit tests for: SOW Instructions CRUD

### Task: [FEAT] Create Skinny Mode wizard
- [ ] [VALIDATE] Verify implementation of: Skinny Mode wizard: hidden SOW/Level creation, limited features
- [ ] [TEST] Write unit tests for: Skinny Mode wizard flow

### Task: [FEAT] Implement Skinny-to-Full upgrade
- [ ] [VALIDATE] Verify implementation of: Skinny-to-Full upgrade: flip flag, reveal controls, no migration
- [ ] [TEST] Write unit tests for: Skinny-to-Full upgrade

### Task: [FEAT] Implement Sub-Levels support
- [ ] [VALIDATE] Verify implementation of: Sub-Levels support for Full mode: nested SOW/Level hierarchies
- [ ] [TEST] Write unit tests for: Sub-Levels hierarchies

### Task: [FEAT] Add Budget/Contract fields to SOW
- [ ] [VALIDATE] Verify implementation of: Budget/Contract fields to SOW for Full mode (custom_fields or dedicated columns)
- [ ] [TEST] Write unit tests for: Budget/Contract fields

### Task: [FEAT] Implement Skinny Mode Role.deny() restriction
- [ ] [VALIDATE] Verify implementation of: Skinny Mode Role.deny() restriction (deny rules disabled in Skinny mode)
- [ ] [TEST] Write unit tests for: Skinny Mode deny restriction

## Phase 13: Multi-DB Federation (Enterprise)

### Task: [FEAT] Implement SOW-to-database routing lookup
- [ ] [VALIDATE] Verify implementation of: SOW-to-database routing lookup via SOWDatabase table
- [ ] [TEST] Write unit tests for: SOW-to-database routing

### Task: [FEAT] Create Redis caching layer
- [ ] [VALIDATE] Verify implementation of: Redis caching layer for SOW->DB mappings, permissions, is_blocked
- [ ] [TEST] Write unit tests for: Redis caching layer

### Task: [FEAT] Build cross-DB write handling
- [ ] [VALIDATE] Verify implementation of: Cross-DB write handling with optimistic approach
- [ ] [TEST] Write unit tests for: Cross-DB write handling

### Task: [FEAT] Implement background reconciliation job
- [ ] [VALIDATE] Verify implementation of: Background reconciliation job for cross-DB sync
- [ ] [TEST] Write unit tests for: Background reconciliation job

## Phase 14: Testing

### Task: [TEST] Write unit tests for State Machine transitions and guards
- [ ] [VALIDATE] Verify completion of: Unit tests for State Machine transitions and guards

### Task: [TEST] Create unit tests for Blocking Propagation
- [ ] [VALIDATE] Verify completion of: Unit tests for Blocking Propagation (downward, upward, Goal boundary)

### Task: [TEST] Write unit tests for Permission Evaluation engine
- [ ] [VALIDATE] Verify completion of: Unit tests for Permission Evaluation engine

### Task: [TEST] Create unit tests for Short Name generation
- [ ] [VALIDATE] Verify completion of: Unit tests for Short Name generation with sequence handling

### Task: [TEST] Write integration tests for all 12 Verb implementations
- [ ] [VALIDATE] Verify completion of: Integration tests for all 12 Verb implementations

### Task: [TEST] Create API endpoint tests for SOW, Noun, Transaction routes
- [ ] [VALIDATE] Verify completion of: API endpoint tests for SOW, Noun, Transaction routes

### Task: [TEST] Write API tests for Admin, User, Help endpoints
- [ ] [VALIDATE] Verify completion of: API tests for Admin, User, Help endpoints

### Task: [TEST] Create batch operation tests with rollback verification
- [ ] [VALIDATE] Verify completion of: Batch operation tests with rollback verification

## Phase 15: SSL/TLS Configuration

### Task: [FEAT] Create ssl.toml configuration file
- [ ] [VALIDATE] Verify implementation of: ssl.toml configuration file with X509 attributes (common_name, organization, org_unit, locality, state, country, validity_days)
- [ ] [TEST] Write unit tests for: ssl.toml configuration parsing

### Task: [FEAT] Implement SSL certificate auto-generation service
- [ ] [VALIDATE] Verify implementation of: SSL certificate auto-generation service using cryptography library
- [ ] [TEST] Write unit tests for: SSL certificate auto-generation

### Task: [FEAT] Create self-signed CA certificate generator
- [ ] [VALIDATE] Verify implementation of: Self-signed CA certificate generator for development environments
- [ ] [TEST] Write unit tests for: Self-signed CA certificate generation

### Task: [FEAT] Implement server certificate generation signed by local CA
- [ ] [VALIDATE] Verify implementation of: Server certificate generation signed by local CA
- [ ] [TEST] Write unit tests for: Server certificate signing

### Task: [FEAT] Build certificate chain validation and storage
- [ ] [VALIDATE] Verify implementation of: Certificate chain validation and storage in config/ssl/ directory
- [ ] [TEST] Write unit tests for: Certificate chain validation

### Task: [FEAT] Implement certificate expiry monitoring
- [ ] [VALIDATE] Verify implementation of: Certificate expiry monitoring with configurable warning threshold
- [ ] [TEST] Write unit tests for: Certificate expiry monitoring

### Task: [FEAT] Create automatic certificate renewal for self-signed certs
- [ ] [VALIDATE] Verify implementation of: Automatic certificate renewal for self-signed certs before expiry
- [ ] [TEST] Write unit tests for: Automatic certificate renewal

### Task: [FEAT] Implement Let's Encrypt ACME client integration
- [ ] [VALIDATE] Verify implementation of: Let's Encrypt ACME client integration for production environments
- [ ] [TEST] Write unit tests for: Let's Encrypt ACME client

### Task: [FEAT] Build ACME HTTP-01 and DNS-01 challenge handlers
- [ ] [VALIDATE] Verify implementation of: ACME HTTP-01 and DNS-01 challenge handlers
- [ ] [TEST] Write unit tests for: ACME challenge handlers

### Task: [FEAT] Create Let's Encrypt certificate auto-renewal
- [ ] [VALIDATE] Verify implementation of: Let's Encrypt certificate auto-renewal with configurable renewal window
- [ ] [TEST] Write unit tests for: Let's Encrypt auto-renewal

### Task: [FEAT] Implement external certificate import
- [ ] [VALIDATE] Verify implementation of: External certificate import (PEM/PFX) for enterprise deployments
- [ ] [TEST] Write unit tests for: External certificate import

### Task: [FEAT] Build certificate format conversion utilities
- [ ] [VALIDATE] Verify implementation of: Certificate format conversion utilities (PEM, DER, PFX/PKCS12)
- [ ] [TEST] Write unit tests for: Certificate format conversion

### Task: [FEAT] Configure FastAPI/Uvicorn HTTPS
- [ ] [VALIDATE] Verify implementation of: FastAPI/Uvicorn HTTPS with SSL context from ssl.toml
- [ ] [TEST] Write unit tests for: HTTPS configuration

### Task: [FEAT] Implement HTTP to HTTPS redirect middleware
- [ ] [VALIDATE] Verify implementation of: HTTP to HTTPS redirect middleware
- [ ] [TEST] Write unit tests for: HTTPS redirect middleware

### Task: [FEAT] Create HSTS header configuration
- [ ] [VALIDATE] Verify implementation of: HSTS (HTTP Strict Transport Security) header configuration
- [ ] [TEST] Write unit tests for: HSTS header configuration

### Task: [FEAT] Implement TLS version and cipher suite configuration
- [ ] [VALIDATE] Verify implementation of: TLS version and cipher suite configuration in ssl.toml
- [ ] [TEST] Write unit tests for: TLS configuration

### Task: [FEAT] Build SSL certificate status endpoint
- [ ] [VALIDATE] Verify implementation of: SSL certificate status endpoint: GET /sys0/ssl/status (expiry, issuer, subject)
- [ ] [TEST] Write unit tests for: SSL status endpoint

### Task: [FEAT] Create SSL certificate management endpoints
- [ ] [VALIDATE] Verify implementation of: SSL certificate management endpoints: POST /sys0/ssl/renew, POST /sys0/ssl/import
- [ ] [TEST] Write unit tests for: SSL management endpoints

### Task: [TEST] Write unit tests for certificate generation with various X509 attributes
- [ ] [VALIDATE] Verify completion of: Unit tests for certificate generation with various X509 attributes

### Task: [TEST] Create integration tests for HTTPS endpoints and TLS handshake
- [ ] [VALIDATE] Verify completion of: Integration tests for HTTPS endpoints and TLS handshake

### Task: [TEST] Write tests for Let's Encrypt ACME flow
- [ ] [VALIDATE] Verify completion of: Tests for Let's Encrypt ACME flow (mock ACME server)

### Task: [TEST] Create tests for certificate renewal and expiry monitoring
- [ ] [VALIDATE] Verify completion of: Tests for certificate renewal and expiry monitoring

### Task: [DOCS] Document ssl.toml configuration options and X509 attributes
- [ ] [VALIDATE] Verify documentation accuracy for: ssl.toml configuration options and X509 attributes

### Task: [DOCS] Create SSL setup guide
- [ ] [VALIDATE] Verify documentation accuracy for: SSL setup guide for development (self-signed) and production (Let's Encrypt/external)

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
