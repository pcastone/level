-- Initial Level database schema

-- Enum types
CREATE TYPE noun_type AS ENUM (
    'sow', 'item', 'task', 'request', 'meeting',
    'deliverable', 'event', 'blocker', 'artifact',
    'group', 'project', 'milestone'
);

CREATE TYPE noun_state AS ENUM (
    'normal', 'escalated', 'completed', 'incompleted', 'closed', 'archived'
);

CREATE TYPE actor_source AS ENUM ('internal', 'ldap');

CREATE TYPE actor_role AS ENUM (
    'owner', 'sme', 'assignee', 'resource', 'stakeholder', 'awareness'
);

CREATE TYPE instruction_scope AS ENUM ('global', 'sow');

-- Nouns table (core entity)
CREATE TABLE nouns (
    id UUID PRIMARY KEY,
    type VARCHAR(50) NOT NULL,
    sow_id UUID NOT NULL REFERENCES nouns(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES nouns(id) ON DELETE SET NULL,
    short_name VARCHAR(255) NOT NULL UNIQUE,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    state VARCHAR(50) NOT NULL DEFAULT 'Normal',
    is_blocked BOOLEAN NOT NULL DEFAULT FALSE,
    due_date TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    closed_at TIMESTAMPTZ,
    custom_fields JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_nouns_sow_id ON nouns(sow_id);
CREATE INDEX idx_nouns_parent_id ON nouns(parent_id);
CREATE INDEX idx_nouns_type ON nouns(type);
CREATE INDEX idx_nouns_state ON nouns(state);
CREATE INDEX idx_nouns_is_blocked ON nouns(is_blocked);
CREATE INDEX idx_nouns_short_name ON nouns(short_name);
CREATE INDEX idx_nouns_due_date ON nouns(due_date);

-- Transactions table (event sourcing)
CREATE TABLE transactions (
    id UUID PRIMARY KEY,
    noun_id UUID NOT NULL REFERENCES nouns(id) ON DELETE CASCADE,
    sow_id UUID NOT NULL REFERENCES nouns(id) ON DELETE CASCADE,
    sequence BIGINT NOT NULL,
    verb VARCHAR(50) NOT NULL,
    actor VARCHAR(255) NOT NULL,
    before_snapshot JSONB NOT NULL,
    after_snapshot JSONB NOT NULL,
    context JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_transactions_noun_id ON transactions(noun_id);
CREATE INDEX idx_transactions_sow_id ON transactions(sow_id);
CREATE INDEX idx_transactions_actor ON transactions(actor);
CREATE INDEX idx_transactions_verb ON transactions(verb);
CREATE INDEX idx_transactions_created_at ON transactions(created_at);
CREATE UNIQUE INDEX idx_transactions_noun_sequence ON transactions(noun_id, sequence);

-- Persons table
CREATE TABLE persons (
    id UUID PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    source actor_source NOT NULL DEFAULT 'internal',
    ldap_dn VARCHAR(500),
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_persons_username ON persons(username);
CREATE INDEX idx_persons_email ON persons(email);

-- Groups table
CREATE TABLE groups (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    source actor_source NOT NULL DEFAULT 'internal',
    ldap_dn VARCHAR(500),
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_groups_name ON groups(name);

-- Group members junction
CREATE TABLE group_members (
    id UUID PRIMARY KEY,
    group_id UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    person_id UUID NOT NULL REFERENCES persons(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(group_id, person_id)
);

-- Vendors table
CREATE TABLE vendors (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    contact_email VARCHAR(255),
    description TEXT,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- AIs table
CREATE TABLE ais (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    model VARCHAR(255) NOT NULL,
    description TEXT,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Roles table
CREATE TABLE roles (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    inherits_from UUID REFERENCES roles(id),
    verbs TEXT[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Role mappings
CREATE TABLE role_mappings (
    id UUID PRIMARY KEY,
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    actor VARCHAR(255) NOT NULL,
    sow_id UUID REFERENCES nouns(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_role_mappings_actor ON role_mappings(actor);
CREATE INDEX idx_role_mappings_sow_id ON role_mappings(sow_id);

-- NounActor junction (M:N actor roles on nouns)
CREATE TABLE noun_actors (
    id UUID PRIMARY KEY,
    noun_id UUID NOT NULL REFERENCES nouns(id) ON DELETE CASCADE,
    actor VARCHAR(255) NOT NULL,
    role actor_role NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(noun_id, actor, role)
);

CREATE INDEX idx_noun_actors_noun_id ON noun_actors(noun_id);
CREATE INDEX idx_noun_actors_actor ON noun_actors(actor);

-- NounAssignment junction (container assignments)
CREATE TABLE noun_assignments (
    id UUID PRIMARY KEY,
    noun_id UUID NOT NULL REFERENCES nouns(id) ON DELETE CASCADE,
    container_id UUID NOT NULL REFERENCES nouns(id) ON DELETE CASCADE,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(noun_id, container_id)
);

CREATE INDEX idx_noun_assignments_noun_id ON noun_assignments(noun_id);
CREATE INDEX idx_noun_assignments_container_id ON noun_assignments(container_id);

-- NounBlock junction (blocker-target)
CREATE TABLE noun_blocks (
    id UUID PRIMARY KEY,
    blocker_id UUID NOT NULL REFERENCES nouns(id) ON DELETE CASCADE,
    target_id UUID NOT NULL REFERENCES nouns(id) ON DELETE CASCADE,
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(blocker_id, target_id)
);

CREATE INDEX idx_noun_blocks_blocker_id ON noun_blocks(blocker_id);
CREATE INDEX idx_noun_blocks_target_id ON noun_blocks(target_id);

-- NounHashTag junction (tagging)
CREATE TABLE noun_hashtags (
    id UUID PRIMARY KEY,
    noun_id UUID NOT NULL REFERENCES nouns(id) ON DELETE CASCADE,
    tag VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(noun_id, tag)
);

CREATE INDEX idx_noun_hashtags_noun_id ON noun_hashtags(noun_id);
CREATE INDEX idx_noun_hashtags_tag ON noun_hashtags(tag);

-- Aliases table
CREATE TABLE aliases (
    id UUID PRIMARY KEY,
    actor VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    noun_id UUID NOT NULL REFERENCES nouns(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(actor, name)
);

CREATE INDEX idx_aliases_actor ON aliases(actor);

-- Instructions table
CREATE TABLE instructions (
    id UUID PRIMARY KEY,
    scope instruction_scope NOT NULL DEFAULT 'global',
    sow_id UUID REFERENCES nouns(id) ON DELETE CASCADE,
    category VARCHAR(255) NOT NULL,
    title VARCHAR(500) NOT NULL,
    content TEXT NOT NULL,
    applies_to TEXT[] NOT NULL DEFAULT '{}',
    created_by VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_instructions_scope ON instructions(scope);
CREATE INDEX idx_instructions_sow_id ON instructions(sow_id);
CREATE INDEX idx_instructions_category ON instructions(category);

-- UserPreferences table
CREATE TABLE user_preferences (
    id UUID PRIMARY KEY,
    actor VARCHAR(255) NOT NULL UNIQUE,
    default_sow UUID REFERENCES nouns(id) ON DELETE SET NULL,
    timezone VARCHAR(100) NOT NULL DEFAULT 'UTC',
    theme VARCHAR(50) NOT NULL DEFAULT 'light',
    date_format VARCHAR(50) NOT NULL DEFAULT 'YYYY-MM-DD',
    notifications JSONB NOT NULL DEFAULT '{"email": true, "in_app": true}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- SOWDatabase table (federation routing)
CREATE TABLE sow_databases (
    id UUID PRIMARY KEY,
    sow_id UUID NOT NULL REFERENCES nouns(id) ON DELETE CASCADE,
    database_url VARCHAR(500) NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sow_databases_sow_id ON sow_databases(sow_id);

-- NounSequence table (short_name generation)
CREATE TABLE noun_sequences (
    id UUID PRIMARY KEY,
    sow_id UUID NOT NULL REFERENCES nouns(id) ON DELETE CASCADE,
    noun_type VARCHAR(50) NOT NULL,
    next_value INTEGER NOT NULL DEFAULT 1,
    UNIQUE(sow_id, noun_type)
);

CREATE INDEX idx_noun_sequences_sow_id ON noun_sequences(sow_id);

-- Updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply updated_at triggers
CREATE TRIGGER trg_nouns_updated_at BEFORE UPDATE ON nouns FOR EACH ROW EXECUTE FUNCTION update_updated_at();
CREATE TRIGGER trg_persons_updated_at BEFORE UPDATE ON persons FOR EACH ROW EXECUTE FUNCTION update_updated_at();
CREATE TRIGGER trg_groups_updated_at BEFORE UPDATE ON groups FOR EACH ROW EXECUTE FUNCTION update_updated_at();
CREATE TRIGGER trg_vendors_updated_at BEFORE UPDATE ON vendors FOR EACH ROW EXECUTE FUNCTION update_updated_at();
CREATE TRIGGER trg_ais_updated_at BEFORE UPDATE ON ais FOR EACH ROW EXECUTE FUNCTION update_updated_at();
CREATE TRIGGER trg_roles_updated_at BEFORE UPDATE ON roles FOR EACH ROW EXECUTE FUNCTION update_updated_at();
CREATE TRIGGER trg_instructions_updated_at BEFORE UPDATE ON instructions FOR EACH ROW EXECUTE FUNCTION update_updated_at();
CREATE TRIGGER trg_user_preferences_updated_at BEFORE UPDATE ON user_preferences FOR EACH ROW EXECUTE FUNCTION update_updated_at();
CREATE TRIGGER trg_sow_databases_updated_at BEFORE UPDATE ON sow_databases FOR EACH ROW EXECUTE FUNCTION update_updated_at();
