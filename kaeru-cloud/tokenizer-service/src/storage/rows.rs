use chrono::{DateTime, Utc};
use contracts::{
    ActionMask, DeletionReason, Initiative, InitiativeGrant, Membership, MembershipState,
    Organization, OrganizationRole, OrganizationState, POLICY_VERSION, PlatformRole, Principal,
    PrincipalState, Project, ProjectGrant, ProjectionSnapshot, ResourceState, TokenCeiling,
    TokenMetadata, TokenPurpose,
};
use sqlx::{FromRow, Postgres, Transaction};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::ProjectionState;

#[derive(FromRow)]
struct StateRow {
    instance_id: Uuid,
    source_generation: Option<Uuid>,
    source_revision: i64,
    authz_revision: i64,
    policy_version: i32,
    ready: bool,
}

pub async fn state(tx: &mut Transaction<'_, Postgres>, lock: bool) -> Result<ProjectionState> {
    let sql = if lock {
        "SELECT * FROM projection_state WHERE singleton=true FOR UPDATE"
    } else {
        "SELECT * FROM projection_state WHERE singleton=true"
    };
    let row: StateRow = sqlx::query_as(sql).fetch_one(&mut **tx).await?;
    if row.policy_version != i32::from(POLICY_VERSION) {
        return Err(Error::NotReady);
    }
    Ok(ProjectionState {
        instance_id: row.instance_id,
        source_generation: row.source_generation,
        source_revision: row.source_revision as u64,
        authz_revision: row.authz_revision as u64,
        policy_version: row.policy_version as u16,
        ready: row.ready,
    })
}

#[derive(FromRow)]
pub struct PrincipalRow {
    pub id: Uuid,
    pub role: String,
    pub state: String,
}
impl PrincipalRow {
    pub fn wire(self) -> Result<Principal> {
        Ok(Principal {
            id: self.id,
            role: PlatformRole::parse(&self.role).ok_or(Error::Corrupt)?,
            state: PrincipalState::parse(&self.state).ok_or(Error::Corrupt)?,
        })
    }
}
#[derive(FromRow)]
pub struct OrganizationRow {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub state: String,
    pub deletion_reason: Option<String>,
}
impl OrganizationRow {
    pub fn wire(self) -> Result<Organization> {
        Ok(Organization {
            id: self.id,
            owner_id: self.owner_id,
            state: OrganizationState::parse(&self.state).ok_or(Error::Corrupt)?,
            deletion_reason: self
                .deletion_reason
                .as_deref()
                .map(|s| DeletionReason::parse(s).ok_or(Error::Corrupt))
                .transpose()?,
        })
    }
}
#[derive(FromRow)]
pub struct MembershipRow {
    pub id: Uuid,
    pub principal_id: Uuid,
    pub organization_id: Uuid,
    pub role: String,
    pub state: String,
    pub all_projects: bool,
    pub all_actions: i64,
}
impl MembershipRow {
    pub fn wire(self) -> Result<Membership> {
        Ok(Membership {
            id: self.id,
            principal_id: self.principal_id,
            organization_id: self.organization_id,
            role: OrganizationRole::parse(&self.role).ok_or(Error::Corrupt)?,
            state: MembershipState::parse(&self.state).ok_or(Error::Corrupt)?,
            all_projects: self.all_projects,
            all_actions: ActionMask::try_from(self.all_actions as u64)
                .map_err(|_| Error::Corrupt)?,
        })
    }
}
#[derive(FromRow)]
pub struct ProjectRow {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub state: String,
}
impl ProjectRow {
    pub fn wire(self) -> Result<Project> {
        Ok(Project {
            id: self.id,
            organization_id: self.organization_id,
            state: ResourceState::parse(&self.state).ok_or(Error::Corrupt)?,
        })
    }
}
#[derive(FromRow)]
pub struct InitiativeRow {
    pub id: Uuid,
    pub personal_owner_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub state: String,
    pub placement_revision: i64,
}
impl InitiativeRow {
    pub fn wire(self) -> Result<Initiative> {
        Ok(Initiative {
            id: self.id,
            personal_owner_id: self.personal_owner_id,
            project_id: self.project_id,
            state: ResourceState::parse(&self.state).ok_or(Error::Corrupt)?,
            placement_revision: self.placement_revision as u64,
        })
    }
}
#[derive(FromRow)]
pub struct ProjectGrantRow {
    pub membership_id: Uuid,
    pub project_id: Uuid,
    pub actions: i64,
}
impl ProjectGrantRow {
    pub fn wire(self) -> Result<ProjectGrant> {
        Ok(ProjectGrant {
            membership_id: self.membership_id,
            project_id: self.project_id,
            actions: ActionMask::try_from(self.actions as u64).map_err(|_| Error::Corrupt)?,
        })
    }
}
#[derive(FromRow)]
pub struct InitiativeGrantRow {
    pub membership_id: Uuid,
    pub initiative_id: Uuid,
    pub actions: i64,
    pub placement_revision: i64,
}
impl InitiativeGrantRow {
    pub fn wire(self) -> Result<InitiativeGrant> {
        Ok(InitiativeGrant {
            membership_id: self.membership_id,
            initiative_id: self.initiative_id,
            actions: ActionMask::try_from(self.actions as u64).map_err(|_| Error::Corrupt)?,
            placement_revision: self.placement_revision as u64,
        })
    }
}

#[derive(FromRow)]
pub struct TokenRow {
    pub id: Uuid,
    pub principal_id: Uuid,
    pub verifier: Vec<u8>,
    pub format_version: i32,
    pub purpose: String,
    pub ceiling_mode: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

pub async fn metadata(tx: &mut Transaction<'_, Postgres>, row: &TokenRow) -> Result<TokenMetadata> {
    let purpose = match row.purpose.as_str() {
        "memory" => TokenPurpose::Memory,
        "control" => TokenPurpose::Control,
        _ => return Err(Error::Corrupt),
    };
    let ceiling = match row.ceiling_mode.as_str() {
        "all_current" => TokenCeiling::AllCurrent,
        "projects" => TokenCeiling::Projects(
            sqlx::query_scalar(
                "SELECT project_id FROM token_projects WHERE token_id=$1 ORDER BY project_id",
            )
            .bind(row.id)
            .fetch_all(&mut **tx)
            .await?,
        ),
        _ => return Err(Error::Corrupt),
    };
    Ok(TokenMetadata {
        id: row.id,
        principal_id: row.principal_id,
        name: row.name.clone(),
        purpose,
        ceiling,
        created_at: row.created_at,
        expires_at: row.expires_at,
        revoked_at: row.revoked_at,
    })
}

pub async fn snapshot(tx: &mut Transaction<'_, Postgres>) -> Result<ProjectionSnapshot> {
    Ok(ProjectionSnapshot {
        principals: sqlx::query_as::<_, PrincipalRow>("SELECT * FROM principals")
            .fetch_all(&mut **tx)
            .await?
            .into_iter()
            .map(PrincipalRow::wire)
            .collect::<Result<_>>()?,
        organizations: sqlx::query_as::<_, OrganizationRow>("SELECT * FROM organizations")
            .fetch_all(&mut **tx)
            .await?
            .into_iter()
            .map(OrganizationRow::wire)
            .collect::<Result<_>>()?,
        memberships: sqlx::query_as::<_, MembershipRow>("SELECT * FROM memberships")
            .fetch_all(&mut **tx)
            .await?
            .into_iter()
            .map(MembershipRow::wire)
            .collect::<Result<_>>()?,
        projects: sqlx::query_as::<_, ProjectRow>("SELECT * FROM projects")
            .fetch_all(&mut **tx)
            .await?
            .into_iter()
            .map(ProjectRow::wire)
            .collect::<Result<_>>()?,
        initiatives: sqlx::query_as::<_, InitiativeRow>("SELECT * FROM initiatives")
            .fetch_all(&mut **tx)
            .await?
            .into_iter()
            .map(InitiativeRow::wire)
            .collect::<Result<_>>()?,
        project_grants: sqlx::query_as::<_, ProjectGrantRow>("SELECT * FROM project_grants")
            .fetch_all(&mut **tx)
            .await?
            .into_iter()
            .map(ProjectGrantRow::wire)
            .collect::<Result<_>>()?,
        initiative_grants: sqlx::query_as::<_, InitiativeGrantRow>(
            "SELECT * FROM initiative_grants",
        )
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(InitiativeGrantRow::wire)
        .collect::<Result<_>>()?,
    })
}
