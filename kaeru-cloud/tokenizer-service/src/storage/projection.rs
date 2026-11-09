use contracts::{
    ApplyRightsRequest, ApplyRightsResponse, BootstrapRequest, POLICY_VERSION, ProjectionChange,
    ProjectionSnapshot, SCHEMA_VERSION,
};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::codec::{decode, encode};
use crate::error::{Error, Result};
use crate::policy::{normalize_changes, validate_snapshot, validate_transition};

use super::{Store, operations::digest, rows};

pub(super) fn version(schema: u16, policy: u16) -> Result<()> {
    if schema != SCHEMA_VERSION || policy != POLICY_VERSION {
        return Err(Error::Invalid("unsupported schema or policy version"));
    }
    Ok(())
}

fn revision(value: u64) -> Result<i64> {
    i64::try_from(value).map_err(|_| Error::Invalid("revision exceeds storage range"))
}

impl Store {
    pub async fn bootstrap(&self, mut request: BootstrapRequest) -> Result<ApplyRightsResponse> {
        version(request.schema_version, request.policy_version)?;
        revision(request.source_revision)?;
        request.snapshot.normalize();
        validate_snapshot(&request.snapshot)?;
        let hash = digest(&request)?;
        let mut tx = self.pool.begin().await?;
        let state = rows::state(&mut tx, true).await?;
        if let Some(result) = replay(
            &mut tx,
            request.source_generation,
            request.operation_id,
            &hash,
        )
        .await?
        {
            tx.commit().await?;
            return Ok(result);
        }
        if state.ready || state.source_generation.is_some() {
            return Err(Error::Conflict("bootstrap requires an empty projection"));
        }
        let changes = snapshot_changes(&request.snapshot);
        persist_changes(&mut tx, &changes).await?;
        let result = finish(
            &mut tx,
            request.source_generation,
            request.operation_id,
            request.source_revision,
            &hash,
        )
        .await?;
        tx.commit().await?;
        Ok(result)
    }

    pub async fn apply(&self, mut request: ApplyRightsRequest) -> Result<ApplyRightsResponse> {
        version(request.schema_version, request.policy_version)?;
        revision(request.source_revision)?;
        normalize_changes(&mut request.changes)?;
        let hash = digest(&request)?;
        let mut tx = self.pool.begin().await?;
        let state = rows::state(&mut tx, true).await?;
        if let Some(result) = replay(
            &mut tx,
            request.source_generation,
            request.operation_id,
            &hash,
        )
        .await?
        {
            tx.commit().await?;
            return Ok(result);
        }
        if !state.ready {
            return Err(Error::NotReady);
        }
        if state.source_generation != Some(request.source_generation)
            || state.source_revision != request.expected_revision
            || request.source_revision
                != request
                    .expected_revision
                    .checked_add(1)
                    .ok_or(Error::Invalid("revision overflow"))?
        {
            return Err(Error::Conflict("projection revision mismatch"));
        }
        let old = rows::snapshot(&mut tx).await?;
        let mut new = old.clone();
        for change in &request.changes {
            apply_model(&mut new, change);
        }
        validate_transition(&old, &new)?;
        persist_changes(&mut tx, &request.changes).await?;
        let result = finish(
            &mut tx,
            request.source_generation,
            request.operation_id,
            request.source_revision,
            &hash,
        )
        .await?;
        tx.commit().await?;
        Ok(result)
    }
}

async fn replay(
    tx: &mut Transaction<'_, Postgres>,
    generation: Uuid,
    id: Uuid,
    hash: &[u8],
) -> Result<Option<ApplyRightsResponse>> {
    let row:Option<(Vec<u8>,Vec<u8>)> = sqlx::query_as("SELECT digest,safe_result FROM projection_operations WHERE source_generation=$1 AND operation_id=$2")
        .bind(generation).bind(id).fetch_optional(&mut **tx).await?;
    if let Some((existing, bytes)) = row {
        if existing != hash {
            return Err(Error::Conflict("operation ID has different content"));
        }
        let mut result: ApplyRightsResponse = decode(&bytes, 32)?;
        result.replayed = true;
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

async fn finish(
    tx: &mut Transaction<'_, Postgres>,
    generation: Uuid,
    id: Uuid,
    source: u64,
    hash: &[u8],
) -> Result<ApplyRightsResponse> {
    let authz:i64=sqlx::query_scalar("UPDATE projection_state SET ready=true,source_generation=$1,source_revision=$2,authz_revision=authz_revision+1 WHERE singleton=true RETURNING authz_revision")
        .bind(generation).bind(revision(source)?).fetch_one(&mut **tx).await?;
    let response = ApplyRightsResponse {
        schema_version: SCHEMA_VERSION,
        operation_id: id,
        source_generation: generation,
        source_revision: source,
        authz_revision: authz as u64,
        replayed: false,
    };
    sqlx::query("INSERT INTO projection_operations(source_generation,operation_id,source_revision,digest,safe_result) VALUES($1,$2,$3,$4,$5)")
        .bind(generation).bind(id).bind(revision(source)?).bind(hash).bind(encode(&response)?).execute(&mut **tx).await?;
    Ok(response)
}

fn upsert<T: Clone>(values: &mut Vec<T>, value: &T, key: impl Fn(&T) -> (Uuid, Uuid)) {
    if let Some(i) = values.iter().position(|v| key(v) == key(value)) {
        values[i] = value.clone();
    } else {
        values.push(value.clone());
    }
}

fn apply_model(s: &mut ProjectionSnapshot, c: &ProjectionChange) {
    let nil = Uuid::nil();
    match c {
        ProjectionChange::Principal(v) => upsert(&mut s.principals, v, |x| (x.id, nil)),
        ProjectionChange::Organization(v) => upsert(&mut s.organizations, v, |x| (x.id, nil)),
        ProjectionChange::Membership(v) => upsert(&mut s.memberships, v, |x| (x.id, nil)),
        ProjectionChange::Project(v) => upsert(&mut s.projects, v, |x| (x.id, nil)),
        ProjectionChange::Initiative(v) => upsert(&mut s.initiatives, v, |x| (x.id, nil)),
        ProjectionChange::ProjectGrant(v) => upsert(&mut s.project_grants, v, |x| {
            (x.membership_id, x.project_id)
        }),
        ProjectionChange::InitiativeGrant(v) => upsert(&mut s.initiative_grants, v, |x| {
            (x.membership_id, x.initiative_id)
        }),
        ProjectionChange::RemoveProjectGrant {
            membership_id,
            project_id,
        } => s
            .project_grants
            .retain(|v| v.membership_id != *membership_id || v.project_id != *project_id),
        ProjectionChange::RemoveInitiativeGrant {
            membership_id,
            initiative_id,
        } => s
            .initiative_grants
            .retain(|v| v.membership_id != *membership_id || v.initiative_id != *initiative_id),
    }
}

fn snapshot_changes(s: &ProjectionSnapshot) -> Vec<ProjectionChange> {
    let mut changes = Vec::new();
    changes.extend(
        s.principals
            .iter()
            .cloned()
            .map(ProjectionChange::Principal),
    );
    changes.extend(
        s.organizations
            .iter()
            .cloned()
            .map(ProjectionChange::Organization),
    );
    changes.extend(
        s.memberships
            .iter()
            .cloned()
            .map(ProjectionChange::Membership),
    );
    changes.extend(s.projects.iter().cloned().map(ProjectionChange::Project));
    changes.extend(
        s.initiatives
            .iter()
            .cloned()
            .map(ProjectionChange::Initiative),
    );
    changes.extend(
        s.project_grants
            .iter()
            .cloned()
            .map(ProjectionChange::ProjectGrant),
    );
    changes.extend(
        s.initiative_grants
            .iter()
            .cloned()
            .map(ProjectionChange::InitiativeGrant),
    );
    changes
}

async fn persist_changes(
    tx: &mut Transaction<'_, Postgres>,
    changes: &[ProjectionChange],
) -> Result<()> {
    // Release old unique active-member slots inside this transaction before a
    // valid owner handoff or membership-incarnation change is installed.
    let member_ids = changes
        .iter()
        .filter_map(|c| {
            if let ProjectionChange::Membership(m) = c {
                Some(m.id)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    sqlx::query("UPDATE memberships SET state='left' WHERE id=ANY($1)")
        .bind(member_ids)
        .execute(&mut **tx)
        .await?;
    for c in changes {
        match c {
            ProjectionChange::Principal(v) => {
                sqlx::query("INSERT INTO principals VALUES($1,$2,$3) ON CONFLICT(id) DO UPDATE SET role=EXCLUDED.role,state=EXCLUDED.state")
                    .bind(v.id).bind(v.role.as_str()).bind(v.state.as_str()).execute(&mut **tx).await?;
            }
            ProjectionChange::Organization(v) => {
                sqlx::query("INSERT INTO organizations VALUES($1,$2,$3,$4) ON CONFLICT(id) DO UPDATE SET owner_id=EXCLUDED.owner_id,state=EXCLUDED.state,deletion_reason=EXCLUDED.deletion_reason")
                    .bind(v.id).bind(v.owner_id).bind(v.state.as_str()).bind(v.deletion_reason.map(|r|r.as_str())).execute(&mut **tx).await?;
            }
            ProjectionChange::Membership(v) => {
                sqlx::query("INSERT INTO memberships VALUES($1,$2,$3,$4,$5,$6,$7) ON CONFLICT(id) DO UPDATE SET principal_id=EXCLUDED.principal_id,organization_id=EXCLUDED.organization_id,role=EXCLUDED.role,state=EXCLUDED.state,all_projects=EXCLUDED.all_projects,all_actions=EXCLUDED.all_actions")
                    .bind(v.id).bind(v.principal_id).bind(v.organization_id).bind(v.role.as_str()).bind(v.state.as_str()).bind(v.all_projects).bind(v.all_actions.bits() as i64).execute(&mut **tx).await?;
            }
            ProjectionChange::Project(v) => {
                sqlx::query("INSERT INTO projects VALUES($1,$2,$3) ON CONFLICT(id) DO UPDATE SET organization_id=EXCLUDED.organization_id,state=EXCLUDED.state")
                    .bind(v.id).bind(v.organization_id).bind(v.state.as_str()).execute(&mut **tx).await?;
            }
            ProjectionChange::Initiative(v) => {
                sqlx::query("INSERT INTO initiatives VALUES($1,$2,$3,$4,$5) ON CONFLICT(id) DO UPDATE SET personal_owner_id=EXCLUDED.personal_owner_id,project_id=EXCLUDED.project_id,state=EXCLUDED.state,placement_revision=EXCLUDED.placement_revision")
                    .bind(v.id).bind(v.personal_owner_id).bind(v.project_id).bind(v.state.as_str()).bind(v.placement_revision as i64).execute(&mut **tx).await?;
            }
            ProjectionChange::ProjectGrant(v) => {
                sqlx::query("INSERT INTO project_grants VALUES($1,$2,$3) ON CONFLICT(membership_id,project_id) DO UPDATE SET actions=EXCLUDED.actions")
                    .bind(v.membership_id).bind(v.project_id).bind(v.actions.bits() as i64).execute(&mut **tx).await?;
            }
            ProjectionChange::InitiativeGrant(v) => {
                sqlx::query("INSERT INTO initiative_grants VALUES($1,$2,$3,$4) ON CONFLICT(membership_id,initiative_id) DO UPDATE SET actions=EXCLUDED.actions,placement_revision=EXCLUDED.placement_revision")
                    .bind(v.membership_id).bind(v.initiative_id).bind(v.actions.bits() as i64).bind(v.placement_revision as i64).execute(&mut **tx).await?;
            }
            ProjectionChange::RemoveProjectGrant {
                membership_id,
                project_id,
            } => {
                sqlx::query("DELETE FROM project_grants WHERE membership_id=$1 AND project_id=$2")
                    .bind(membership_id)
                    .bind(project_id)
                    .execute(&mut **tx)
                    .await?;
            }
            ProjectionChange::RemoveInitiativeGrant {
                membership_id,
                initiative_id,
            } => {
                sqlx::query(
                    "DELETE FROM initiative_grants WHERE membership_id=$1 AND initiative_id=$2",
                )
                .bind(membership_id)
                .bind(initiative_id)
                .execute(&mut **tx)
                .await?;
            }
        }
    }
    Ok(())
}
