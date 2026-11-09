use std::collections::{BTreeMap, BTreeSet};

use chrono::Utc;
use contracts::{
    AccessCheck, AuthorizeRequest, AuthorizeResponse, MembershipState, PrincipalState,
    ProjectionSnapshot, Resource, SCHEMA_VERSION,
};
use sha2::{Digest, Sha256};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::codec::encode;
use crate::config::defaults::MAX_BATCH;
use crate::credentials::{matches, parse};
use crate::error::{Error, Result};
use crate::policy::{Evaluator, check_shape, purpose_allows};

use super::{
    Store,
    rows::{
        self, InitiativeGrantRow, InitiativeRow, MembershipRow, OrganizationRow, PrincipalRow,
        ProjectGrantRow, ProjectRow, TokenRow,
    },
};

impl Store {
    pub async fn authorize(&self, request: AuthorizeRequest) -> Result<AuthorizeResponse> {
        if request.schema_version != SCHEMA_VERSION
            || request.checks.is_empty()
            || request.checks.len() > MAX_BATCH
        {
            return Err(Error::Invalid("invalid authorization batch"));
        }
        for check in &request.checks {
            check_shape(check, request.mode)?;
        }
        let (id, verifier) = parse(&request.credential)?;
        let mut tx = self.read_transaction().await?;
        let state = rows::state(&mut tx, false).await?;
        if !state.ready {
            return Err(Error::NotReady);
        }
        let generation = state.source_generation.ok_or(Error::Corrupt)?;
        let row = sqlx::query_as::<_, TokenRow>("SELECT * FROM tokens WHERE id=$1")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(Error::Unauthorized)?;
        if row.format_version != 1
            || !matches(&row.verifier, &verifier)
            || row.revoked_at.is_some()
            || row.expires_at <= Utc::now()
        {
            return Err(Error::Unauthorized);
        }
        let principal = sqlx::query_as::<_, PrincipalRow>("SELECT * FROM principals WHERE id=$1")
            .bind(row.principal_id)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(Error::Unauthorized)?
            .wire()?;
        if principal.state != PrincipalState::Active {
            return Err(Error::Unauthorized);
        }
        let metadata = rows::metadata(&mut tx, &row).await?;
        let mut normalized_checks = request.checks.clone();
        normalized_checks.sort_by_key(|c| (c.action, c.resource));
        normalized_checks.dedup();
        let hash = Sha256::digest(encode(&(
            state.policy_version,
            &normalized_checks,
            request.mode,
        ))?);
        let key = format!(
            "{}:facts:{}:{generation}:{}:{}:{hash:x}",
            self.namespace, state.instance_id, state.authz_revision, principal.id
        );
        let cached = if let Some(cache) = &self.cache {
            cache.get(&key).await
        } else {
            None
        };
        let hit = cached.is_some();
        let facts = if let Some(facts) = cached {
            facts
        } else {
            load_facts(&mut tx, principal.id, &request.checks).await?
        };
        let evaluator = Evaluator::new(&facts);
        let allowed = request.checks.iter().all(|c| {
            purpose_allows(metadata.purpose, c.action)
                && evaluator.ceiling_allows(&metadata.ceiling, c.resource)
                && evaluator.allows(principal.id, c, request.mode)
        });
        let result = AuthorizeResponse {
            schema_version: SCHEMA_VERSION,
            request_id: request.request_id,
            allowed,
            principal_id: principal.id,
            checks: request.checks,
            mode: request.mode,
            source_generation: generation,
            source_revision: state.source_revision,
            authz_revision: state.authz_revision,
        };
        tx.commit().await?;
        if !hit && let Some(cache) = &self.cache {
            cache.put(&key, &facts).await;
        }
        Ok(result)
    }
}

fn ids(set: &BTreeSet<Uuid>) -> Vec<Uuid> {
    set.iter().copied().collect()
}

/// Fetch only the requested resources, their ancestors and relevant grants.
async fn load_facts(
    tx: &mut Transaction<'_, Postgres>,
    actor: Uuid,
    checks: &[AccessCheck],
) -> Result<ProjectionSnapshot> {
    let mut pids = BTreeSet::from([actor]);
    let mut orgs = BTreeSet::new();
    let mut projects = BTreeSet::new();
    let mut initiatives = BTreeSet::new();
    let mut memberships = BTreeSet::new();
    for c in checks {
        match c.resource {
            Resource::Principal(id) | Resource::PersonalSpace(id) => {
                pids.insert(id);
            }
            Resource::Organization(id) => {
                orgs.insert(id);
            }
            Resource::Project(id) => {
                projects.insert(id);
            }
            Resource::Initiative(id) => {
                initiatives.insert(id);
            }
            Resource::Membership(id) => {
                memberships.insert(id);
            }
            Resource::Platform => {}
        }
    }
    let is = sqlx::query_as::<_, InitiativeRow>("SELECT * FROM initiatives WHERE id=ANY($1)")
        .bind(ids(&initiatives))
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(InitiativeRow::wire)
        .collect::<Result<Vec<_>>>()?;
    for i in &is {
        if let Some(id) = i.project_id {
            projects.insert(id);
        }
        if let Some(id) = i.personal_owner_id {
            pids.insert(id);
        }
    }
    let ps = sqlx::query_as::<_, ProjectRow>("SELECT * FROM projects WHERE id=ANY($1)")
        .bind(ids(&projects))
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(ProjectRow::wire)
        .collect::<Result<Vec<_>>>()?;
    for p in &ps {
        orgs.insert(p.organization_id);
    }
    let targets = sqlx::query_as::<_, MembershipRow>("SELECT * FROM memberships WHERE id=ANY($1)")
        .bind(ids(&memberships))
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(MembershipRow::wire)
        .collect::<Result<Vec<_>>>()?;
    for m in &targets {
        orgs.insert(m.organization_id);
        pids.insert(m.principal_id);
    }
    let os = sqlx::query_as::<_, OrganizationRow>("SELECT * FROM organizations WHERE id=ANY($1)")
        .bind(ids(&orgs))
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(OrganizationRow::wire)
        .collect::<Result<Vec<_>>>()?;
    let active=sqlx::query_as::<_,MembershipRow>("SELECT * FROM memberships WHERE principal_id=$1 AND organization_id=ANY($2) AND state='active'").bind(actor).bind(ids(&orgs)).fetch_all(&mut **tx).await?.into_iter().map(MembershipRow::wire).collect::<Result<Vec<_>>>()?;
    let actor_members = active.iter().map(|m| m.id).collect::<Vec<_>>();
    let mut member_map = targets
        .into_iter()
        .map(|m| (m.id, m))
        .collect::<BTreeMap<_, _>>();
    for m in active {
        member_map.insert(m.id, m);
    }
    let gs = sqlx::query_as::<_, ProjectGrantRow>(
        "SELECT * FROM project_grants WHERE membership_id=ANY($1) AND project_id=ANY($2)",
    )
    .bind(&actor_members)
    .bind(ids(&projects))
    .fetch_all(&mut **tx)
    .await?
    .into_iter()
    .map(ProjectGrantRow::wire)
    .collect::<Result<Vec<_>>>()?;
    let igs = sqlx::query_as::<_, InitiativeGrantRow>(
        "SELECT * FROM initiative_grants WHERE membership_id=ANY($1) AND initiative_id=ANY($2)",
    )
    .bind(&actor_members)
    .bind(ids(&initiatives))
    .fetch_all(&mut **tx)
    .await?
    .into_iter()
    .map(InitiativeGrantRow::wire)
    .collect::<Result<Vec<_>>>()?;
    let users = sqlx::query_as::<_, PrincipalRow>("SELECT * FROM principals WHERE id=ANY($1)")
        .bind(ids(&pids))
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(PrincipalRow::wire)
        .collect::<Result<Vec<_>>>()?;
    let ms = member_map
        .into_values()
        .filter(|m| m.state == MembershipState::Active || memberships.contains(&m.id))
        .collect();
    Ok(ProjectionSnapshot {
        principals: users,
        organizations: os,
        memberships: ms,
        projects: ps,
        initiatives: is,
        project_grants: gs,
        initiative_grants: igs,
    })
}
