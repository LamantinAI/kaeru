CREATE TABLE projection_state (
    singleton boolean PRIMARY KEY DEFAULT true CHECK (singleton),
    instance_id uuid NOT NULL DEFAULT gen_random_uuid(),
    source_generation uuid,
    source_revision bigint NOT NULL DEFAULT 0 CHECK (source_revision >= 0),
    authz_revision bigint NOT NULL DEFAULT 0 CHECK (authz_revision >= 0),
    policy_version integer NOT NULL DEFAULT 1,
    ready boolean NOT NULL DEFAULT false
);
INSERT INTO projection_state(singleton) VALUES (true);

CREATE TABLE principals (
    id uuid PRIMARY KEY,
    role text NOT NULL CHECK (role IN ('user', 'operator', 'admin')),
    state text NOT NULL CHECK (state IN ('active', 'blocked', 'deleted'))
);
CREATE TABLE organizations (
    id uuid PRIMARY KEY,
    owner_id uuid NOT NULL REFERENCES principals(id),
    state text NOT NULL CHECK (state IN ('active', 'blocked', 'deleted')),
    deletion_reason text CHECK (deletion_reason IN ('administrative', 'last_member_left')),
    CHECK ((state = 'deleted') = (deletion_reason IS NOT NULL))
);
CREATE TABLE memberships (
    id uuid PRIMARY KEY,
    principal_id uuid NOT NULL REFERENCES principals(id),
    organization_id uuid NOT NULL REFERENCES organizations(id),
    role text NOT NULL CHECK (role IN ('member', 'operator', 'admin')),
    state text NOT NULL CHECK (state IN ('active', 'left')),
    all_projects boolean NOT NULL,
    all_actions bigint NOT NULL CHECK (all_actions >= 0 AND (all_actions & ~31::bigint) = 0)
);
CREATE UNIQUE INDEX memberships_active ON memberships(principal_id, organization_id) WHERE state = 'active';
CREATE UNIQUE INDEX memberships_owner ON memberships(organization_id) WHERE state = 'active' AND role = 'admin';
CREATE INDEX memberships_organization ON memberships(organization_id, id);
CREATE TABLE projects (
    id uuid PRIMARY KEY,
    organization_id uuid NOT NULL REFERENCES organizations(id),
    state text NOT NULL CHECK (state IN ('active', 'pending', 'moving', 'deleted'))
);
CREATE INDEX projects_organization ON projects(organization_id, id);
CREATE TABLE initiatives (
    id uuid PRIMARY KEY,
    personal_owner_id uuid REFERENCES principals(id),
    project_id uuid REFERENCES projects(id),
    state text NOT NULL CHECK (state IN ('active', 'pending', 'moving', 'deleted')),
    placement_revision bigint NOT NULL CHECK (placement_revision >= 0),
    CHECK ((personal_owner_id IS NULL) <> (project_id IS NULL))
);
CREATE INDEX initiatives_project ON initiatives(project_id, id);
CREATE INDEX initiatives_personal ON initiatives(personal_owner_id, id);
CREATE TABLE project_grants (
    membership_id uuid NOT NULL REFERENCES memberships(id),
    project_id uuid NOT NULL REFERENCES projects(id),
    actions bigint NOT NULL CHECK (actions >= 0 AND (actions & ~31::bigint) = 0),
    PRIMARY KEY (membership_id, project_id)
);
CREATE TABLE initiative_grants (
    membership_id uuid NOT NULL REFERENCES memberships(id),
    initiative_id uuid NOT NULL REFERENCES initiatives(id),
    actions bigint NOT NULL CHECK (actions >= 0 AND (actions & ~11::bigint) = 0),
    placement_revision bigint NOT NULL CHECK (placement_revision >= 0),
    PRIMARY KEY (membership_id, initiative_id)
);
CREATE TABLE tokens (
    id uuid PRIMARY KEY,
    principal_id uuid NOT NULL REFERENCES principals(id),
    verifier bytea NOT NULL CHECK (octet_length(verifier) = 32),
    format_version integer NOT NULL CHECK (format_version = 1),
    purpose text NOT NULL CHECK (purpose IN ('memory', 'control')),
    ceiling_mode text NOT NULL CHECK (ceiling_mode IN ('all_current', 'projects')),
    name text NOT NULL,
    created_at timestamptz NOT NULL,
    expires_at timestamptz NOT NULL,
    revoked_at timestamptz,
    CHECK (expires_at > created_at)
);
CREATE INDEX tokens_principal ON tokens(principal_id, id);
CREATE TABLE token_projects (
    token_id uuid NOT NULL REFERENCES tokens(id),
    project_id uuid NOT NULL REFERENCES projects(id),
    PRIMARY KEY(token_id, project_id)
);
CREATE TABLE projection_operations (
    source_generation uuid NOT NULL,
    operation_id uuid NOT NULL,
    source_revision bigint NOT NULL,
    digest bytea NOT NULL CHECK (octet_length(digest) = 32),
    safe_result bytea NOT NULL,
    PRIMARY KEY(source_generation, operation_id),
    UNIQUE(source_generation, source_revision)
);
CREATE TABLE token_operations (
    service_id text NOT NULL,
    actor_id uuid NOT NULL REFERENCES principals(id),
    operation_id uuid NOT NULL,
    kind text NOT NULL CHECK (kind IN ('create', 'revoke')),
    digest bytea NOT NULL CHECK (octet_length(digest) = 32),
    token_id uuid NOT NULL REFERENCES tokens(id),
    safe_result bytea NOT NULL,
    PRIMARY KEY(service_id, actor_id, operation_id)
);
