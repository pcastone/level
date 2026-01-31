# Open Tasks

## Active

### Phase 1: Project Foundation
- [ ] [TASK] Set up Python/FastAPI project structure with src/, tests/, config/ directories (added: 2026-01-30)
- [ ] [TASK] Configure Poetry or pip for dependency management with dev/prod separation (added: 2026-01-30)
- [ ] [TASK] Set up PostgreSQL database connection with SQLAlchemy async support (added: 2026-01-30)
- [ ] [TASK] Configure Alembic for database migrations (added: 2026-01-30)
- [ ] [TASK] Set up logging infrastructure with structured JSON logging (added: 2026-01-30)
- [ ] [TASK] Create base configuration system supporting Skinny/Standard/Enterprise modes (added: 2026-01-30)

### Phase 2: Database Schema Implementation
- [ ] [FEAT] Create Noun table with all 12 types (SOW, Item, Task, Request, Meeting, Deliverable, Event, Blocker, Artifact, Group, Project, MileStone) (added: 2026-01-30)
- [ ] [FEAT] Implement NounActor junction table for M:N actor-role relationships (owner, assignee, awareness, sme, stakeholder, resource) (added: 2026-01-30)
- [ ] [FEAT] Create NounAssignment table for Container assignments with sort_order for Kanban (added: 2026-01-30)
- [ ] [FEAT] Implement NounBlock table for M:N blocker-target relationships (added: 2026-01-30)
- [ ] [FEAT] Create NounHashTag table for global tagging system (added: 2026-01-30)
- [ ] [FEAT] Implement Transaction table with sequence, verb, actor, before/after snapshots, context jsonb (added: 2026-01-30)
- [ ] [FEAT] Create NounSequence table for per-SOW+type short_name generation (added: 2026-01-30)
- [ ] [FEAT] Implement SOWDatabase table for multi-DB federation routing (added: 2026-01-30)
- [ ] [FEAT] Create Alias table with id, actor, name, noun_id, created_at and unique (actor, name) constraint (added: 2026-01-30)
- [ ] [FEAT] Create Instruction table with id, scope (global/sow), sow_id, category, title, content, applies_to, created_by, updated_at (added: 2026-01-30)
- [ ] [FEAT] Create HelpContent table or static config for noun/verb documentation (type, abbrev, description, guards, side_effects, examples) (added: 2026-01-30)
- [ ] [FEAT] Create UserPreferences table or jsonb field for storing user profile and preferences (added: 2026-01-30)

### Phase 3: Actor System
- [ ] [FEAT] Create Person table with internal/LDAP source support (added: 2026-01-30)
- [ ] [FEAT] Implement Group table with role association (added: 2026-01-30)
- [ ] [FEAT] Create Vendor table for external vendor actors (added: 2026-01-30)
- [ ] [FEAT] Implement AI table for AI actor registration (added: 2026-01-30)
- [ ] [FEAT] Create GroupMember junction table for actor group membership (added: 2026-01-30)
- [ ] [FEAT] Build actor string resolution service (+Person, @Group, !Vendor, *AI prefixes) (added: 2026-01-30)

### Phase 4: Role & Permission System
- [ ] [FEAT] Create Role table with inherits_from enum and verbs array (added: 2026-01-30)
- [ ] [FEAT] Implement RoleMapping table linking roles to actors (added: 2026-01-30)
- [ ] [FEAT] Build permission evaluation engine: group resolution -> role lookup -> SOW allow/deny processing (added: 2026-01-30)
- [ ] [FEAT] Implement base role defaults (owner=*, sme=*, assignee, resource, stakeholder, awareness) (added: 2026-01-30)
- [ ] [FEAT] Create SOW roles jsonb allow/deny rule processor (added: 2026-01-30)

### Phase 5: Core Domain Services
- [ ] [FEAT] Implement Short Name generation service: {SOW.short_name}-{TYPE_ABBREV}-{SEQUENCE} (added: 2026-01-30)
- [ ] [FEAT] Build State Machine service with guards (Normal/Escalated for most verbs, not-blocked for Complete/Incomplete, Completed/Incompleted for Close) (added: 2026-01-30)
- [ ] [FEAT] Implement Blocking Propagation service (downward to children, upward is_blocked computation, Goal boundary handling) (added: 2026-01-30)
- [ ] [FEAT] Create Transaction/Event Sourcing service with before/after snapshots (added: 2026-01-30)
- [ ] [FEAT] Build MileStone auto-complete service (is_blocked if ANY blocked, Complete if ALL Completed) (added: 2026-01-30)
- [ ] [FEAT] Implement Project Goals tracking (blocked_goals counter, NOT is_blocked) (added: 2026-01-30)
- [ ] [FEAT] Implement Archived state transition with configurable auto-archive after n days from Close (added: 2026-01-30)
- [ ] [FEAT] Create background job for auto-archiving Closed nouns after configurable threshold (added: 2026-01-30)
- [ ] [FEAT] Implement Artifact leaf-only validation (reject parent_id assignment to Artifact type) (added: 2026-01-30)
- [ ] [FEAT] Implement SOW short_name update handling with sequence continuity (editable unlike Noun short_name) (added: 2026-01-30)
- [ ] [FEAT] Implement SOW short_name validation: required, globally unique, no spaces allowed (added: 2026-01-30)
- [ ] [FEAT] Implement Container type validation for NounAssignment (container_id must be Group, Project, or MileStone) (added: 2026-01-30)
- [ ] [FEAT] Implement Blocker type validation for NounBlock (blocker_id must be type=Blocker) (added: 2026-01-30)
- [ ] [FEAT] Implement SOW.roles jsonb schema validation (allow/deny arrays with actor, type, verbs structure) (added: 2026-01-30)
- [ ] [FEAT] Implement Transaction.context schema validation per verb type (target_id for Blocked, container_id for Assign, etc.) (added: 2026-01-30)

### Phase 6: Verb Implementation (12 Verbs)
- [ ] [FEAT] Implement Open verb: create new Noun with validation, short_name generation, Transaction logging (added: 2026-01-30)
- [ ] [FEAT] Implement Complete verb: state transition with blocked guard, completed_at timestamp, Blocker release side-effect (added: 2026-01-30)
- [ ] [FEAT] Implement Incomplete verb: state transition with blocked guard (added: 2026-01-30)
- [ ] [FEAT] Implement Normal verb: reset state from Escalated/Completed/Incompleted (added: 2026-01-30)
- [ ] [FEAT] Implement Escalate verb: mark urgent with state guard (added: 2026-01-30)
- [ ] [FEAT] Implement Close verb: require Completed/Incompleted state and all children Closed (added: 2026-01-30)
- [ ] [FEAT] Implement Open verb on Closed noun: create clone with new short_name (reopen by cloning behavior) (added: 2026-01-30)
- [ ] [FEAT] Implement Update verb: modify attributes with field-level validation (added: 2026-01-30)
- [ ] [FEAT] Implement Reparent verb: move Noun to SOW root (added: 2026-01-30)
- [ ] [FEAT] Implement Assign verb: add Noun to Container with sort_order (added: 2026-01-30)
- [ ] [FEAT] Implement Unassign verb: remove Noun from Container (added: 2026-01-30)
- [ ] [FEAT] Implement Blocked verb: create NounBlock link with propagation (added: 2026-01-30)
- [ ] [FEAT] Implement Release verb: remove NounBlock link with propagation update (added: 2026-01-30)

### Phase 7: REST API - Core Endpoints
- [ ] [FEAT] Implement X-API-Key authentication middleware (added: 2026-01-30)
- [ ] [FEAT] Create SOW endpoints: GET/POST /sows, GET /sow/{sow_id} (added: 2026-01-30)
- [ ] [FEAT] Implement Noun endpoints: GET/POST /nouns/{sow_id}/nouns, GET /nouns/{sow_id}/noun/{noun_id} (added: 2026-01-30)
- [ ] [FEAT] Create Transaction endpoints: GET/POST /nouns/{sow_id}/noun/{noun_id}/transactions (added: 2026-01-30)
- [ ] [FEAT] Implement SOW-wide transaction query: GET /nouns/{sow_id}/transactions with actor, after, before filters (added: 2026-01-30)
- [ ] [FEAT] Implement batch transactions: POST /nouns/{sow_id}/batch/transactions with all-or-nothing semantics (added: 2026-01-30)
- [ ] [FEAT] Build cursor-based pagination: limit, sort, order, cursor (timestamp_uuid format) (added: 2026-01-30)
- [ ] [FEAT] Implement standard error responses with codes (NOUN_BLOCKED, INVALID_STATE_TRANSITION, etc.) (added: 2026-01-30)
- [ ] [FEAT] Implement logging endpoints: POST /logs/debug, POST /logs/error (pass-through to logging subsystem) (added: 2026-01-30)

### Phase 8: REST API - Views
- [ ] [FEAT] Implement Timeline view: GET /views/{sow_id}/timeline with start/end date filtering (added: 2026-01-30)
- [ ] [FEAT] Create Kanban view: GET /views/{sow_id}/kanban with container_id grouping (added: 2026-01-30)
- [ ] [FEAT] Implement Calendar view: GET /views/{sow_id}/calendar with month-based day buckets (added: 2026-01-30)

### Phase 9: REST API - Admin (/sys0)
- [ ] [FEAT] Implement Person CRUD: GET/POST/PUT/DELETE /sys0/actors/persons (added: 2026-01-30)
- [ ] [FEAT] Create Group CRUD with members: GET/POST/PUT/DELETE /sys0/actors/groups, /members endpoints (added: 2026-01-30)
- [ ] [FEAT] Implement Vendor CRUD: GET/POST/PUT/DELETE /sys0/actors/vendors (added: 2026-01-30)
- [ ] [FEAT] Create AI CRUD: GET/POST/PUT/DELETE /sys0/actors/ai (added: 2026-01-30)
- [ ] [FEAT] Implement LDAP commands: sync, clear_cache, set_interval, configure (added: 2026-01-30)
- [ ] [FEAT] Create Role management: GET/POST/PUT/DELETE /sys0/roles with actor associations (added: 2026-01-30)
- [ ] [FEAT] Implement Instructions management: GET/POST/PUT/DELETE /sys0/instructions (added: 2026-01-30)
- [ ] [FEAT] Create Help content management: PUT /sys0/help/nouns/{type}, PUT /sys0/help/verbs/{verb} (added: 2026-01-30)

### Phase 10: REST API - User (/user)
- [ ] [FEAT] Implement Alias CRUD: GET/POST/DELETE /user/aliases with (actor, name) uniqueness (added: 2026-01-30)
- [ ] [FEAT] Create Profile endpoints: GET/PUT /user/profile (added: 2026-01-30)
- [ ] [FEAT] Implement Preferences endpoints: GET/PUT /user/preferences (default_sow, timezone, theme, notifications) (added: 2026-01-30)
- [ ] [FEAT] Create access endpoints: GET /user/roles, GET /user/sows (added: 2026-01-30)

### Phase 11: REST API - Help
- [ ] [FEAT] Implement Help endpoints: GET /help/nouns, /help/nouns/{type} with full noun documentation (added: 2026-01-30)
- [ ] [FEAT] Create verb help: GET /help/verbs, /help/verbs/{verb} with guards, side-effects, examples (added: 2026-01-30)
- [ ] [FEAT] Implement container/role help: GET /help/containers, /help/roles (added: 2026-01-30)
- [ ] [FEAT] Create Instructions endpoint: GET /help/instructions with optional sow_id filter and category filtering (structure, workflow, naming, assignment, escalation) (added: 2026-01-30)

### Phase 12: SOW-Level Features
- [ ] [FEAT] Implement SOW Instructions: GET/POST/PUT/DELETE /nouns/{sow_id}/instructions with category field (added: 2026-01-30)
- [ ] [FEAT] Create Skinny Mode wizard: hidden SOW/Level creation, limited features (added: 2026-01-30)
- [ ] [FEAT] Implement Skinny-to-Full upgrade: flip flag, reveal controls, no migration (added: 2026-01-30)
- [ ] [FEAT] Implement Sub-Levels support for Full mode: nested SOW/Level hierarchies (added: 2026-01-30)
- [ ] [FEAT] Add Budget/Contract fields to SOW for Full mode (custom_fields or dedicated columns) (added: 2026-01-30)
- [ ] [FEAT] Implement Skinny Mode Role.deny() restriction (deny rules disabled in Skinny mode) (added: 2026-01-30)

### Phase 13: Multi-DB Federation (Enterprise)
- [ ] [FEAT] Implement SOW-to-database routing lookup via SOWDatabase table (added: 2026-01-30)
- [ ] [FEAT] Create Redis caching layer for SOW->DB mappings, permissions, is_blocked (added: 2026-01-30)
- [ ] [FEAT] Build cross-DB write handling with optimistic approach (added: 2026-01-30)
- [ ] [FEAT] Implement background reconciliation job for cross-DB sync (added: 2026-01-30)

### Phase 14: Testing
- [ ] [TEST] Write unit tests for State Machine transitions and guards (added: 2026-01-30)
- [ ] [TEST] Create unit tests for Blocking Propagation (downward, upward, Goal boundary) (added: 2026-01-30)
- [ ] [TEST] Write unit tests for Permission Evaluation engine (added: 2026-01-30)
- [ ] [TEST] Create unit tests for Short Name generation with sequence handling (added: 2026-01-30)
- [ ] [TEST] Write integration tests for all 12 Verb implementations (added: 2026-01-30)
- [ ] [TEST] Create API endpoint tests for SOW, Noun, Transaction routes (added: 2026-01-30)
- [ ] [TEST] Write API tests for Admin, User, Help endpoints (added: 2026-01-30)
- [ ] [TEST] Create batch operation tests with rollback verification (added: 2026-01-30)

### Phase 15: SSL/TLS Configuration
- [ ] [FEAT] Create ssl.toml configuration file with X509 attributes (common_name, organization, org_unit, locality, state, country, validity_days) (added: 2026-01-30)
- [ ] [FEAT] Implement SSL certificate auto-generation service using cryptography library (added: 2026-01-30)
- [ ] [FEAT] Create self-signed CA certificate generator for development environments (added: 2026-01-30)
- [ ] [FEAT] Implement server certificate generation signed by local CA (added: 2026-01-30)
- [ ] [FEAT] Build certificate chain validation and storage in config/ssl/ directory (added: 2026-01-30)
- [ ] [FEAT] Implement certificate expiry monitoring with configurable warning threshold (added: 2026-01-30)
- [ ] [FEAT] Create automatic certificate renewal for self-signed certs before expiry (added: 2026-01-30)
- [ ] [FEAT] Implement Let's Encrypt ACME client integration for production environments (added: 2026-01-30)
- [ ] [FEAT] Build ACME HTTP-01 and DNS-01 challenge handlers (added: 2026-01-30)
- [ ] [FEAT] Create Let's Encrypt certificate auto-renewal with configurable renewal window (added: 2026-01-30)
- [ ] [FEAT] Implement external certificate import (PEM/PFX) for enterprise deployments (added: 2026-01-30)
- [ ] [FEAT] Build certificate format conversion utilities (PEM, DER, PFX/PKCS12) (added: 2026-01-30)
- [ ] [FEAT] Configure FastAPI/Uvicorn HTTPS with SSL context from ssl.toml (added: 2026-01-30)
- [ ] [FEAT] Implement HTTP to HTTPS redirect middleware (added: 2026-01-30)
- [ ] [FEAT] Create HSTS (HTTP Strict Transport Security) header configuration (added: 2026-01-30)
- [ ] [FEAT] Implement TLS version and cipher suite configuration in ssl.toml (added: 2026-01-30)
- [ ] [FEAT] Build SSL certificate status endpoint: GET /sys0/ssl/status (expiry, issuer, subject) (added: 2026-01-30)
- [ ] [FEAT] Create SSL certificate management endpoints: POST /sys0/ssl/renew, POST /sys0/ssl/import (added: 2026-01-30)
- [ ] [TEST] Write unit tests for certificate generation with various X509 attributes (added: 2026-01-30)
- [ ] [TEST] Create integration tests for HTTPS endpoints and TLS handshake (added: 2026-01-30)
- [ ] [TEST] Write tests for Let's Encrypt ACME flow (mock ACME server) (added: 2026-01-30)
- [ ] [TEST] Create tests for certificate renewal and expiry monitoring (added: 2026-01-30)
- [ ] [DOCS] Document ssl.toml configuration options and X509 attributes (added: 2026-01-30)
- [ ] [DOCS] Create SSL setup guide for development (self-signed) and production (Let's Encrypt/external) (added: 2026-01-30)

### Phase 16: Documentation & DevOps
- [ ] [DOCS] Create API documentation with OpenAPI/Swagger specs (added: 2026-01-30)
- [ ] [TASK] Set up Docker Compose for local development (PostgreSQL, Redis) (added: 2026-01-30)
- [ ] [TASK] Create database seed scripts for development/testing (added: 2026-01-30)
- [ ] [TASK] Configure CI/CD pipeline with test automation (added: 2026-01-30)

### Phase 17: CLI Client - Project Setup
- [ ] [TASK] Set up Python CLI project with Click or Typer framework (added: 2026-01-30)
- [ ] [TASK] Create CLI package structure: level/, commands/, utils/, config/ (added: 2026-01-30)
- [ ] [TASK] Implement CLI configuration file (~/.level/config.toml) for api-key, default-sow, preferences (added: 2026-01-30)
- [ ] [FEAT] Build HTTP client wrapper with X-API-Key header injection and error handling (added: 2026-01-30)
- [ ] [FEAT] Implement base output formatters: table, json, minimal for different verbosity levels (added: 2026-01-30)

### Phase 18: CLI Client - Noun Reference Resolution
- [ ] [FEAT] Implement UUID reference matcher (direct uuid lookup) (added: 2026-01-30)
- [ ] [FEAT] Build ShortName reference matcher (IT-TASK-001 format, global lookup) (added: 2026-01-30)
- [ ] [FEAT] Create Alias reference matcher (*prefix lookup from /user/aliases) (added: 2026-01-30)
- [ ] [FEAT] Implement Title reference matcher (quoted strings, requires --sow or default_sow) (added: 2026-01-30)
- [ ] [FEAT] Build reference resolution chain: UUID -> ShortName -> Alias -> Title (added: 2026-01-30)

### Phase 19: CLI Client - SOW Commands
- [ ] [FEAT] Implement `level sow list` command with table/json output (added: 2026-01-30)
- [ ] [FEAT] Create `level sow create --name="..." --short-name="..."` command (added: 2026-01-30)
- [ ] [FEAT] Implement `level sow show {sow_id}` with full SOW details display (added: 2026-01-30)

### Phase 20: CLI Client - Noun Commands
- [ ] [FEAT] Implement `level list nouns --sow={} --type={} --state={} --assignee={} --hashtag={}` with filtering (added: 2026-01-30)
- [ ] [FEAT] Create `level show {reference}` command with noun details, actors, assignments display (added: 2026-01-30)
- [ ] [FEAT] Implement `level create --sow={} --type={} --parent={} --title="..."` with all noun types (added: 2026-01-30)

### Phase 21: CLI Client - Verb Commands (12 Verbs)
- [ ] [FEAT] Implement `level complete {reference}` with blocked guard error display (added: 2026-01-30)
- [ ] [FEAT] Create `level incomplete {reference}` command (added: 2026-01-30)
- [ ] [FEAT] Implement `level escalate {reference}` command (added: 2026-01-30)
- [ ] [FEAT] Create `level normal {reference}` command (added: 2026-01-30)
- [ ] [FEAT] Implement `level close {reference}` with children-closed guard display (added: 2026-01-30)
- [ ] [FEAT] Create `level update {reference} field=value custom.field=value` with key=value parsing (added: 2026-01-30)
- [ ] [FEAT] Implement `level reparent {reference}` command (added: 2026-01-30)

### Phase 22: CLI Client - Relationship Commands
- [ ] [FEAT] Implement `level assign {reference} --container={id}` command (added: 2026-01-30)
- [ ] [FEAT] Create `level unassign {reference} --container={id}` command (added: 2026-01-30)
- [ ] [FEAT] Implement `level block {blocker_ref} --target={noun_ref}` command (added: 2026-01-30)
- [ ] [FEAT] Create `level release {blocker_ref} --target={noun_ref}` command (added: 2026-01-30)

### Phase 23: CLI Client - Batch Operations
- [ ] [FEAT] Implement batch verb commands: `level complete {ref1} {ref2} {ref3}` (added: 2026-01-30)
- [ ] [FEAT] Create batch assign: `level assign {ref1} {ref2} --container={id}` (added: 2026-01-30)
- [ ] [FEAT] Build batch error handling with per-noun failure reporting (added: 2026-01-30)

### Phase 24: CLI Client - View Commands
- [ ] [FEAT] Implement `level timeline --sow={} --start={} --end={}` with ASCII timeline rendering (added: 2026-01-30)
- [ ] [FEAT] Create `level kanban --container={}` with column-based ASCII display (added: 2026-01-30)
- [ ] [FEAT] Implement `level calendar --sow={} --month={}` with day-bucket rendering (added: 2026-01-30)

### Phase 25: CLI Client - Transaction & History Commands
- [ ] [FEAT] Implement `level history {reference}` showing noun transaction log (added: 2026-01-30)
- [ ] [FEAT] Create `level transactions --sow={} --actor={} --after={} --before={}` query (added: 2026-01-30)

### Phase 26: CLI Client - Alias & Config Commands
- [ ] [FEAT] Implement `level alias set {name} {noun_reference}` command (added: 2026-01-30)
- [ ] [FEAT] Create `level alias list` with table output (added: 2026-01-30)
- [ ] [FEAT] Implement `level alias remove {name}` command (added: 2026-01-30)
- [ ] [FEAT] Create `level config set {key} {value}` for api-key, default-sow (added: 2026-01-30)
- [ ] [FEAT] Implement `level config get {key}` and `level config list` commands (added: 2026-01-30)

### Phase 27: CLI Client - Help Commands
- [ ] [FEAT] Implement `level help` showing available commands and grammar overview (added: 2026-01-30)
- [ ] [FEAT] Create `level help nouns` listing all 12 noun types with descriptions (added: 2026-01-30)
- [ ] [FEAT] Implement `level help noun {type}` with detailed noun documentation (added: 2026-01-30)
- [ ] [FEAT] Create `level help verbs` listing all 12 verbs with descriptions (added: 2026-01-30)
- [ ] [FEAT] Implement `level help verb {verb}` with guards, side-effects, examples (added: 2026-01-30)

### Phase 28: MCP Server - Project Setup
- [ ] [TASK] Set up MCP server project with Python mcp SDK (added: 2026-01-30)
- [ ] [TASK] Create MCP package structure: level_mcp/, tools/, resources/, prompts/ (added: 2026-01-30)
- [ ] [FEAT] Implement MCP server configuration for Level API connection (api-key, base-url) (added: 2026-01-30)
- [ ] [FEAT] Build *AI actor context injection for all MCP operations (added: 2026-01-30)

### Phase 29: MCP Server - Tool Definitions (Noun Operations)
- [ ] [FEAT] Implement `level_list_sows` tool returning SOW list for AI context (added: 2026-01-30)
- [ ] [FEAT] Create `level_show_sow` tool with sow_id parameter (added: 2026-01-30)
- [ ] [FEAT] Implement `level_list_nouns` tool with type, state, sow_id, assignee, hashtag filters (added: 2026-01-30)
- [ ] [FEAT] Create `level_show_noun` tool with reference resolution (uuid, short_name, alias) (added: 2026-01-30)
- [ ] [FEAT] Implement `level_create_noun` tool with type, sow_id, parent_id, title, custom_fields parameters (added: 2026-01-30)

### Phase 30: MCP Server - Tool Definitions (Verb Operations)
- [ ] [FEAT] Implement `level_complete` tool with reference parameter and blocked error handling (added: 2026-01-30)
- [ ] [FEAT] Create `level_incomplete` tool with reference parameter (added: 2026-01-30)
- [ ] [FEAT] Implement `level_escalate` tool with reference parameter (added: 2026-01-30)
- [ ] [FEAT] Create `level_normal` tool with reference parameter (added: 2026-01-30)
- [ ] [FEAT] Implement `level_close` tool with reference parameter and children guard handling (added: 2026-01-30)
- [ ] [FEAT] Create `level_update` tool with reference and fields object parameter (added: 2026-01-30)
- [ ] [FEAT] Implement `level_reparent` tool with reference parameter (added: 2026-01-30)

### Phase 31: MCP Server - Tool Definitions (Relationship Operations)
- [ ] [FEAT] Implement `level_assign` tool with noun_reference and container_id parameters (added: 2026-01-30)
- [ ] [FEAT] Create `level_unassign` tool with noun_reference and container_id parameters (added: 2026-01-30)
- [ ] [FEAT] Implement `level_block` tool with blocker_reference and target_reference parameters (added: 2026-01-30)
- [ ] [FEAT] Create `level_release` tool with blocker_reference and target_reference parameters (added: 2026-01-30)

### Phase 32: MCP Server - Tool Definitions (Batch & Views)
- [ ] [FEAT] Implement `level_batch_complete` tool with references array parameter (added: 2026-01-30)
- [ ] [FEAT] Create `level_batch_assign` tool with references array and container_id (added: 2026-01-30)
- [ ] [FEAT] Implement `level_timeline` tool with sow_id, start, end parameters (added: 2026-01-30)
- [ ] [FEAT] Create `level_kanban` tool with container_id parameter (added: 2026-01-30)
- [ ] [FEAT] Implement `level_calendar` tool with sow_id and month parameters (added: 2026-01-30)

### Phase 33: MCP Server - Tool Definitions (History & Aliases)
- [ ] [FEAT] Implement `level_history` tool showing noun transaction log (added: 2026-01-30)
- [ ] [FEAT] Create `level_transactions` tool with sow_id, actor, date range filters (added: 2026-01-30)
- [ ] [FEAT] Implement `level_alias_set` tool for AI-specific aliases (added: 2026-01-30)
- [ ] [FEAT] Create `level_alias_list` tool returning AI's aliases (added: 2026-01-30)
- [ ] [FEAT] Implement `level_alias_remove` tool (added: 2026-01-30)

### Phase 34: MCP Server - Resources
- [ ] [FEAT] Implement `level://sows` resource listing all accessible SOWs (added: 2026-01-30)
- [ ] [FEAT] Create `level://sow/{sow_id}/nouns` resource with noun tree (added: 2026-01-30)
- [ ] [FEAT] Implement `level://sow/{sow_id}/blocked` resource listing blocked nouns (added: 2026-01-30)
- [ ] [FEAT] Create `level://sow/{sow_id}/escalated` resource listing escalated nouns (added: 2026-01-30)
- [ ] [FEAT] Implement `level://noun/{reference}` resource with full noun details (added: 2026-01-30)

### Phase 35: MCP Server - Prompts (AI Guidance)
- [ ] [FEAT] Implement `level_help` prompt with grammar overview, noun/verb summary (added: 2026-01-30)
- [ ] [FEAT] Create `level_help_noun` prompt with detailed noun type documentation (added: 2026-01-30)
- [ ] [FEAT] Implement `level_help_verb` prompt with verb details, guards, examples (added: 2026-01-30)
- [ ] [FEAT] Create `level_instructions` prompt fetching SOW-specific operational instructions (added: 2026-01-30)
- [ ] [FEAT] Implement `level_context` prompt with current SOW state summary for AI context (added: 2026-01-30)

### Phase 36: MCP Server - AI-Specific Features
- [ ] [FEAT] Implement AI permission scoping based on *AI actor role mappings (added: 2026-01-30)
- [ ] [FEAT] Create AI action logging with *AI actor attribution in transactions (added: 2026-01-30)
- [ ] [FEAT] Build AI-friendly error messages with suggested corrections (added: 2026-01-30)
- [ ] [FEAT] Implement AI context prefetch for common operations (SOW list, blocked items) (added: 2026-01-30)

### Phase 37: CLI & MCP Testing
- [ ] [TEST] Write unit tests for CLI noun reference resolution chain (added: 2026-01-30)
- [ ] [TEST] Create CLI command tests for all SOW operations (added: 2026-01-30)
- [ ] [TEST] Write CLI tests for all 12 verb commands (added: 2026-01-30)
- [ ] [TEST] Create CLI tests for batch operations with error scenarios (added: 2026-01-30)
- [ ] [TEST] Write CLI tests for view rendering (timeline, kanban, calendar) (added: 2026-01-30)
- [ ] [TEST] Create MCP tool tests for noun operations (added: 2026-01-30)
- [ ] [TEST] Write MCP tool tests for verb operations with permission checks (added: 2026-01-30)
- [ ] [TEST] Create MCP resource tests for all resource URIs (added: 2026-01-30)
- [ ] [TEST] Write MCP prompt tests for help and instructions (added: 2026-01-30)

### Phase 38: CLI & MCP Documentation
- [ ] [DOCS] Create CLI user guide with command reference and examples (added: 2026-01-30)
- [ ] [DOCS] Write CLI installation and configuration guide (added: 2026-01-30)
- [ ] [DOCS] Create MCP server setup and integration guide (added: 2026-01-30)
- [ ] [DOCS] Write MCP tool/resource/prompt reference documentation (added: 2026-01-30)
- [ ] [DOCS] Create AI integration examples for Claude, GPT, and other LLMs (added: 2026-01-30)

### Phase 39: SvelteKit Frontend - Project Setup
- [ ] [TASK] Initialize SvelteKit project with TypeScript, Tailwind CSS, and shadcn-svelte (added: 2026-01-30)
- [ ] [TASK] Configure project structure: routes/, lib/components/, lib/stores/, lib/api/, lib/types/ (added: 2026-01-30)
- [ ] [TASK] Set up environment configuration for API base URL and authentication (added: 2026-01-30)
- [ ] [FEAT] Create typed API client with fetch wrapper, X-API-Key injection, error handling (added: 2026-01-30)
- [ ] [FEAT] Implement base TypeScript types for all 12 Noun types, Verbs, States, Actors (added: 2026-01-30)
- [ ] [FEAT] Create Svelte stores for current user, active SOW, preferences, and global state (added: 2026-01-30)

### Phase 40: SvelteKit Frontend - Authentication & Layout
- [ ] [FEAT] Implement API key authentication flow with secure storage (added: 2026-01-30)
- [ ] [FEAT] Create root layout with navigation sidebar, header, and content area (added: 2026-01-30)
- [ ] [FEAT] Build responsive navigation with SOW selector dropdown (added: 2026-01-30)
- [ ] [FEAT] Implement user profile menu with preferences, aliases, logout (added: 2026-01-30)
- [ ] [FEAT] Create breadcrumb navigation component for noun hierarchy (added: 2026-01-30)

### Phase 41: SvelteKit Frontend - Skinny Mode Wizard
- [ ] [FEAT] Create "Create Workspace" wizard landing page (added: 2026-01-30)
- [ ] [FEAT] Implement Step 1: Name, Description, Timezone input form (added: 2026-01-30)
- [ ] [FEAT] Build Step 2: Add SME and Stakeholder actor selection (added: 2026-01-30)
- [ ] [FEAT] Create Step 3: Confirmation and hidden SOW+Level creation (added: 2026-01-30)
- [ ] [FEAT] Implement redirect to Level dashboard after wizard completion (added: 2026-01-30)

### Phase 42: SvelteKit Frontend - SOW Management (Full Mode)
- [ ] [FEAT] Create /sows route with SOW list table (name, short_name, created_at) (added: 2026-01-30)
- [ ] [FEAT] Implement SOW creation modal with name, short_name, description fields (added: 2026-01-30)
- [ ] [FEAT] Build /sow/[sow_id] route with SOW details and settings (added: 2026-01-30)
- [ ] [FEAT] Create SOW roles management UI for allow/deny rule editing (added: 2026-01-30)
- [ ] [FEAT] Implement Skinny-to-Full upgrade button with confirmation (added: 2026-01-30)

### Phase 43: SvelteKit Frontend - Noun Components (Base)
- [ ] [FEAT] Create NounCard component with type icon, title, state badge, is_blocked indicator (added: 2026-01-30)
- [ ] [FEAT] Build NounListItem component for table/list views (added: 2026-01-30)
- [ ] [FEAT] Implement NounTypeIcon component with icons for all 12 noun types (added: 2026-01-30)
- [ ] [FEAT] Create StateBadge component with color-coded states (Normal, Escalated, Completed, etc.) (added: 2026-01-30)
- [ ] [FEAT] Build BlockedIndicator component showing blocker count and tooltip (added: 2026-01-30)
- [ ] [FEAT] Implement ActorChip component for +Person, @Group, !Vendor, *AI display (added: 2026-01-30)
- [ ] [FEAT] Create HashTagChip component for tag display and filtering (added: 2026-01-30)

### Phase 44: SvelteKit Frontend - Noun Detail View
- [ ] [FEAT] Create /sow/[sow_id]/noun/[noun_id] route for noun detail page (added: 2026-01-30)
- [ ] [FEAT] Build noun header with title, short_name, type, state, action buttons (added: 2026-01-30)
- [ ] [FEAT] Implement noun metadata panel: created_at, due_date, completed_at, closed_at (added: 2026-01-30)
- [ ] [FEAT] Create description editor with markdown support (added: 2026-01-30)
- [ ] [FEAT] Build custom_fields display and editor for jsonb fields (added: 2026-01-30)
- [ ] [FEAT] Implement actors panel with role-grouped actor chips (owner, assignee, sme, etc.) (added: 2026-01-30)
- [ ] [FEAT] Create assignments panel showing Container memberships (added: 2026-01-30)
- [ ] [FEAT] Build blockers panel showing blocking/blocked-by relationships (added: 2026-01-30)
- [ ] [FEAT] Implement children tree view for parent nouns (added: 2026-01-30)

### Phase 45: SvelteKit Frontend - Noun Creation & Editing
- [ ] [FEAT] Create noun creation modal with type selector, title, parent picker (added: 2026-01-30)
- [ ] [FEAT] Build type-specific form fields (Meeting: duration/end_time, Project: goals, MileStone: auto_complete, etc.) (added: 2026-01-30)
- [ ] [FEAT] Implement parent noun picker with tree navigation (added: 2026-01-30)
- [ ] [FEAT] Create actor assignment UI with role selector and actor search (added: 2026-01-30)
- [ ] [FEAT] Build container assignment UI with sort_order drag handle (added: 2026-01-30)
- [ ] [FEAT] Implement hashtag input with autocomplete from existing tags (added: 2026-01-30)
- [ ] [FEAT] Create due_date picker with date/time selection (added: 2026-01-30)

### Phase 46: SvelteKit Frontend - Verb Action UI
- [ ] [FEAT] Create verb action button group component (Complete, Escalate, Close, etc.) (added: 2026-01-30)
- [ ] [FEAT] Implement Complete action with blocked guard error display (added: 2026-01-30)
- [ ] [FEAT] Build Incomplete action button with confirmation (added: 2026-01-30)
- [ ] [FEAT] Create Escalate/Normal toggle button with state feedback (added: 2026-01-30)
- [ ] [FEAT] Implement Close action with children-not-closed error display (added: 2026-01-30)
- [ ] [FEAT] Build Update action with inline field editing (added: 2026-01-30)
- [ ] [FEAT] Create Reparent action with parent picker modal (added: 2026-01-30)

### Phase 47: SvelteKit Frontend - Relationship Actions
- [ ] [FEAT] Implement Assign action with container picker modal (added: 2026-01-30)
- [ ] [FEAT] Create Unassign action with confirmation (added: 2026-01-30)
- [ ] [FEAT] Build Block action: create Blocker noun and link to target (added: 2026-01-30)
- [ ] [FEAT] Implement Release action with blocker selection (added: 2026-01-30)
- [ ] [FEAT] Create bulk action toolbar for multi-select operations (added: 2026-01-30)

### Phase 48: SvelteKit Frontend - Timeline View
- [ ] [FEAT] Create /sow/[sow_id]/timeline route with date range picker (added: 2026-01-30)
- [ ] [FEAT] Build vertical timeline component with date axis (added: 2026-01-30)
- [ ] [FEAT] Implement timeline noun cards positioned by due_date (added: 2026-01-30)
- [ ] [FEAT] Create timeline filters: Container, State, Noun type, Actor (added: 2026-01-30)
- [ ] [FEAT] Build timeline zoom controls (day, week, month granularity) (added: 2026-01-30)
- [ ] [FEAT] Implement drag-to-reschedule for due_date updates (added: 2026-01-30)

### Phase 49: SvelteKit Frontend - Kanban View
- [ ] [FEAT] Create /sow/[sow_id]/kanban route with container selector (added: 2026-01-30)
- [ ] [FEAT] Build Kanban board component with draggable columns (added: 2026-01-30)
- [ ] [FEAT] Implement Kanban column for each state or hashtag grouping (added: 2026-01-30)
- [ ] [FEAT] Create drag-and-drop noun cards with sort_order persistence (added: 2026-01-30)
- [ ] [FEAT] Build Kanban card component with compact noun info (added: 2026-01-30)
- [ ] [FEAT] Implement WIP limits per column with visual warning (added: 2026-01-30)
- [ ] [FEAT] Create quick-add card input at column bottom (added: 2026-01-30)

### Phase 50: SvelteKit Frontend - Calendar View
- [ ] [FEAT] Create /sow/[sow_id]/calendar route with month navigation (added: 2026-01-30)
- [ ] [FEAT] Build calendar grid component with day cells (added: 2026-01-30)
- [ ] [FEAT] Implement day cell with noun count and preview (added: 2026-01-30)
- [ ] [FEAT] Create day detail popover with full noun list (added: 2026-01-30)
- [ ] [FEAT] Build calendar filters: Container, Noun type (added: 2026-01-30)
- [ ] [FEAT] Implement drag-to-date for due_date updates (added: 2026-01-30)
- [ ] [FEAT] Create Meeting-specific calendar with time slots (added: 2026-01-30)

### Phase 51: SvelteKit Frontend - Transaction History
- [ ] [FEAT] Create noun history panel with transaction log (added: 2026-01-30)
- [ ] [FEAT] Build transaction entry component showing verb, actor, timestamp (added: 2026-01-30)
- [ ] [FEAT] Implement before/after diff view for Update transactions (added: 2026-01-30)
- [ ] [FEAT] Create /sow/[sow_id]/transactions route for SOW-wide history (added: 2026-01-30)
- [ ] [FEAT] Build transaction filters: actor, verb, date range (added: 2026-01-30)

### Phase 52: SvelteKit Frontend - Project & MileStone Features
- [ ] [FEAT] Create Project detail page with Goals panel (Deliverables/Events) (added: 2026-01-30)
- [ ] [FEAT] Build blocked_goals counter display with drill-down (added: 2026-01-30)
- [ ] [FEAT] Implement Goal assignment UI with Deliverable/Event picker (added: 2026-01-30)
- [ ] [FEAT] Create MileStone detail page with assigned nouns list (added: 2026-01-30)
- [ ] [FEAT] Build MileStone auto-complete status indicator (is_completable, is_blocked) (added: 2026-01-30)
- [ ] [FEAT] Implement MileStone progress bar based on assigned noun states (added: 2026-01-30)

### Phase 53: SvelteKit Frontend - Container Views
- [ ] [FEAT] Create Group detail page with member nouns (added: 2026-01-30)
- [ ] [FEAT] Build Container assignment manager with drag-drop reordering (added: 2026-01-30)
- [ ] [FEAT] Implement Container tree view showing nested structure (added: 2026-01-30)

### Phase 54: SvelteKit Frontend - User Preferences & Aliases
- [ ] [FEAT] Create /user/preferences route with settings form (added: 2026-01-30)
- [ ] [FEAT] Implement default_sow selector from accessible SOWs (added: 2026-01-30)
- [ ] [FEAT] Build timezone picker with common timezone list (added: 2026-01-30)
- [ ] [FEAT] Create theme toggle (light/dark) with system preference detection (added: 2026-01-30)
- [ ] [FEAT] Implement notification preferences toggles (email, escalations, assignments) (added: 2026-01-30)
- [ ] [FEAT] Create /user/aliases route with alias list and management (added: 2026-01-30)
- [ ] [FEAT] Build alias creation modal with noun picker (added: 2026-01-30)
- [ ] [FEAT] Implement date_format preference with format picker and apply formatting throughout UI (added: 2026-01-30)
- [ ] [FEAT] Implement default_view preference to load preferred view (timeline/kanban/calendar) on SOW navigation (added: 2026-01-30)

### Phase 55: SvelteKit Frontend - Search & Filtering
- [ ] [FEAT] Create global search bar with noun reference resolution (added: 2026-01-30)
- [ ] [FEAT] Implement search results dropdown with noun type grouping (added: 2026-01-30)
- [ ] [FEAT] Build advanced filter panel with type, state, actor, hashtag, date range (added: 2026-01-30)
- [ ] [FEAT] Create saved filter presets with user persistence (added: 2026-01-30)
- [ ] [FEAT] Implement URL-based filter state for shareable views (added: 2026-01-30)

### Phase 56: SvelteKit Frontend - Notifications & Real-time
- [ ] [FEAT] Create notification dropdown with recent activity feed (added: 2026-01-30)
- [ ] [FEAT] Implement escalation alerts with visual prominence (added: 2026-01-30)
- [ ] [FEAT] Build assignment notification with quick-action buttons (added: 2026-01-30)
- [ ] [FEAT] Create blocking notification when user's nouns become blocked (added: 2026-01-30)
- [ ] [TASK] Implement SSE or WebSocket connection for real-time updates (optional) (added: 2026-01-30)

### Phase 57: SvelteKit Frontend - Help & Onboarding
- [ ] [FEAT] Create /help route with grammar overview (Nouns, Verbs, Containers) (added: 2026-01-30)
- [ ] [FEAT] Build noun type reference cards with descriptions and examples (added: 2026-01-30)
- [ ] [FEAT] Implement verb reference cards with guards and side-effects (added: 2026-01-30)
- [ ] [FEAT] Create contextual help tooltips throughout the UI (added: 2026-01-30)
- [ ] [FEAT] Build first-time user onboarding tour (added: 2026-01-30)
- [ ] [FEAT] Implement keyboard shortcuts guide modal (added: 2026-01-30)

### Phase 58: SvelteKit Frontend - Admin Panel (sys0)
- [ ] [FEAT] Create /admin route with admin navigation (added: 2026-01-30)
- [ ] [FEAT] Build /admin/actors/persons page with Person CRUD table (added: 2026-01-30)
- [ ] [FEAT] Implement /admin/actors/groups page with Group CRUD and member management (added: 2026-01-30)
- [ ] [FEAT] Create /admin/actors/vendors page with Vendor CRUD table (added: 2026-01-30)
- [ ] [FEAT] Build /admin/actors/ai page with AI actor CRUD table (added: 2026-01-30)
- [ ] [FEAT] Implement /admin/ldap page with LDAP config and sync controls (added: 2026-01-30)
- [ ] [FEAT] Create /admin/roles page with Role CRUD and actor mapping (added: 2026-01-30)
- [ ] [FEAT] Build /admin/instructions page with global instruction management and category selector (structure, workflow, naming, assignment, escalation) (added: 2026-01-30)
- [ ] [FEAT] Implement /admin/help page with help content editor (added: 2026-01-30)

### Phase 59: SvelteKit Frontend - Error Handling & Loading States
- [ ] [FEAT] Create error boundary component with user-friendly messages (added: 2026-01-30)
- [ ] [FEAT] Implement API error display with error codes (NOUN_BLOCKED, VERB_DENIED, etc.) (added: 2026-01-30)
- [ ] [FEAT] Build loading skeleton components for all major views (added: 2026-01-30)
- [ ] [FEAT] Create optimistic UI updates for verb actions (added: 2026-01-30)
- [ ] [FEAT] Implement retry logic for failed API requests (added: 2026-01-30)
- [ ] [FEAT] Build offline indicator with reconnection status (added: 2026-01-30)

### Phase 60: SvelteKit Frontend - Accessibility & Polish
- [ ] [FEAT] Implement keyboard navigation for all interactive elements (added: 2026-01-30)
- [ ] [FEAT] Add ARIA labels and roles throughout the application (added: 2026-01-30)
- [ ] [FEAT] Create high-contrast theme option (added: 2026-01-30)
- [ ] [FEAT] Implement focus management for modals and popovers (added: 2026-01-30)
- [ ] [FEAT] Build screen reader announcements for state changes (added: 2026-01-30)
- [ ] [TASK] Perform accessibility audit and fix WCAG 2.1 AA issues (added: 2026-01-30)

### Phase 61: SvelteKit Frontend - Testing
- [ ] [TEST] Write unit tests for API client and type guards (added: 2026-01-30)
- [ ] [TEST] Create component tests for NounCard, StateBadge, ActorChip (added: 2026-01-30)
- [ ] [TEST] Write component tests for verb action buttons and modals (added: 2026-01-30)
- [ ] [TEST] Create integration tests for noun CRUD flows (added: 2026-01-30)
- [ ] [TEST] Write integration tests for Timeline, Kanban, Calendar views (added: 2026-01-30)
- [ ] [TEST] Create E2E tests for Skinny Mode wizard flow (added: 2026-01-30)
- [ ] [TEST] Write E2E tests for complete noun lifecycle (create, assign, complete, close) (added: 2026-01-30)
- [ ] [TEST] Create E2E tests for admin panel CRUD operations (added: 2026-01-30)

### Phase 62: SvelteKit Frontend - Documentation & Deployment
- [ ] [DOCS] Create frontend architecture documentation (added: 2026-01-30)
- [ ] [DOCS] Write component library documentation with Storybook (added: 2026-01-30)
- [ ] [DOCS] Create user guide with screenshots and workflows (added: 2026-01-30)
- [ ] [TASK] Configure SvelteKit adapter for target deployment (node, static, vercel) (added: 2026-01-30)
- [ ] [TASK] Set up frontend build pipeline with type checking and linting (added: 2026-01-30)
- [ ] [TASK] Configure production environment variables and API endpoints (added: 2026-01-30)

## Completed
