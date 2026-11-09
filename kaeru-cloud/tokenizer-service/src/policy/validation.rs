use std::collections::{BTreeMap, BTreeSet};

use contracts::{
    ActionMask, DeletionReason, MembershipState, OrganizationRole, OrganizationState,
    ProjectionChange, ProjectionSnapshot,
};
use uuid::Uuid;

use crate::error::{Error, Result};

fn unique(ids: impl Iterator<Item = Uuid>) -> Result<()> {
    let mut seen = BTreeSet::new();
    if ids.into_iter().any(|id| !seen.insert(id)) {
        Err(Error::Invalid("duplicate entity ID"))
    } else {
        Ok(())
    }
}

pub fn validate_snapshot(s: &ProjectionSnapshot) -> Result<()> {
    unique(s.principals.iter().map(|v| v.id))?;
    unique(s.organizations.iter().map(|v| v.id))?;
    unique(s.memberships.iter().map(|v| v.id))?;
    unique(s.projects.iter().map(|v| v.id))?;
    unique(s.initiatives.iter().map(|v| v.id))?;
    let principals = s.principals.iter().map(|v| v.id).collect::<BTreeSet<_>>();
    let orgs = s
        .organizations
        .iter()
        .map(|v| (v.id, v))
        .collect::<BTreeMap<_, _>>();
    let members = s
        .memberships
        .iter()
        .map(|v| (v.id, v))
        .collect::<BTreeMap<_, _>>();
    let projects = s
        .projects
        .iter()
        .map(|v| (v.id, v))
        .collect::<BTreeMap<_, _>>();
    let initiatives = s
        .initiatives
        .iter()
        .map(|v| (v.id, v))
        .collect::<BTreeMap<_, _>>();
    let mut active = BTreeSet::new();
    for org in &s.organizations {
        if !principals.contains(&org.owner_id) {
            return Err(Error::Invalid("unknown organization owner"));
        }
        if org.deletion_reason.is_some() != (org.state == OrganizationState::Deleted) {
            return Err(Error::Invalid(
                "organization deletion reason does not match state",
            ));
        }
        if org.deletion_reason == Some(DeletionReason::LastMemberLeft)
            && s.memberships
                .iter()
                .any(|m| m.organization_id == org.id && m.state == MembershipState::Active)
        {
            return Err(Error::Invalid(
                "orphaned organization still has active memberships",
            ));
        }
    }
    for m in &s.memberships {
        let org = orgs
            .get(&m.organization_id)
            .ok_or(Error::Invalid("unknown membership organization"))?;
        if !principals.contains(&m.principal_id)
            || !m.all_actions.is_subset(ActionMask::PROJECT_GRANT)
        {
            return Err(Error::Invalid("invalid membership principal or grant"));
        }
        if m.state == MembershipState::Active {
            if !active.insert((m.principal_id, m.organization_id)) {
                return Err(Error::Invalid("duplicate active membership"));
            }
            if m.role == OrganizationRole::Admin && m.principal_id != org.owner_id {
                return Err(Error::Invalid("organization admin must be its owner"));
            }
        }
    }
    for org in &s.organizations {
        if org.state != OrganizationState::Deleted
            && !s.memberships.iter().any(|m| {
                m.organization_id == org.id
                    && m.principal_id == org.owner_id
                    && m.role == OrganizationRole::Admin
                    && m.state == MembershipState::Active
            })
        {
            return Err(Error::Invalid(
                "live organization requires its active owner membership",
            ));
        }
    }
    for p in &s.projects {
        if !orgs.contains_key(&p.organization_id) {
            return Err(Error::Invalid("unknown project organization"));
        }
    }
    for i in &s.initiatives {
        if i.personal_owner_id.is_some() == i.project_id.is_some()
            || i.placement_revision > i64::MAX as u64
        {
            return Err(Error::Invalid("initiative must have one valid placement"));
        }
        if i.personal_owner_id
            .is_some_and(|id| !principals.contains(&id))
            || i.project_id.is_some_and(|id| !projects.contains_key(&id))
        {
            return Err(Error::Invalid("unknown initiative parent"));
        }
    }
    let mut keys = BTreeSet::new();
    for g in &s.project_grants {
        let m = members
            .get(&g.membership_id)
            .ok_or(Error::Invalid("unknown grant membership"))?;
        let p = projects
            .get(&g.project_id)
            .ok_or(Error::Invalid("unknown grant project"))?;
        if !keys.insert((g.membership_id, g.project_id))
            || m.organization_id != p.organization_id
            || !g.actions.is_subset(ActionMask::PROJECT_GRANT)
        {
            return Err(Error::Invalid("invalid project grant"));
        }
    }
    keys.clear();
    for g in &s.initiative_grants {
        let m = members
            .get(&g.membership_id)
            .ok_or(Error::Invalid("unknown grant membership"))?;
        let i = initiatives
            .get(&g.initiative_id)
            .ok_or(Error::Invalid("unknown grant initiative"))?;
        let current_org = i
            .project_id
            .and_then(|id| projects.get(&id))
            .map(|p| p.organization_id);
        if !keys.insert((g.membership_id, g.initiative_id))
            || !g.actions.is_subset(ActionMask::INITIATIVE_GRANT)
            || g.placement_revision > i.placement_revision
            || (g.placement_revision == i.placement_revision
                && current_org != Some(m.organization_id))
        {
            return Err(Error::Invalid("invalid initiative grant"));
        }
    }
    Ok(())
}

pub fn validate_transition(old: &ProjectionSnapshot, new: &ProjectionSnapshot) -> Result<()> {
    for m in &new.memberships {
        if let Some(before) = old.memberships.iter().find(|v| v.id == m.id)
            && (before.principal_id != m.principal_id
                || before.organization_id != m.organization_id
                || (before.state == MembershipState::Left && m.state == MembershipState::Active))
        {
            return Err(Error::Invalid("membership identity cannot be reused"));
        }
    }
    for p in &new.projects {
        if old
            .projects
            .iter()
            .any(|v| v.id == p.id && v.organization_id != p.organization_id)
        {
            return Err(Error::Invalid("project organization is immutable"));
        }
    }
    for i in &new.initiatives {
        if let Some(before) = old.initiatives.iter().find(|v| v.id == i.id) {
            let moved = before.personal_owner_id != i.personal_owner_id
                || before.project_id != i.project_id;
            if i.placement_revision < before.placement_revision
                || (moved && i.placement_revision == before.placement_revision)
            {
                return Err(Error::Invalid("placement revision must advance on move"));
            }
        }
    }
    validate_snapshot(new)
}

/// Normalize by semantic key. Repeated changes to one key are rejected.
pub fn normalize_changes(changes: &mut [ProjectionChange]) -> Result<()> {
    changes.sort_by_key(change_key);
    if changes
        .windows(2)
        .any(|w| change_key(&w[0]) == change_key(&w[1]))
    {
        return Err(Error::Invalid("duplicate change target"));
    }
    Ok(())
}

fn change_key(c: &ProjectionChange) -> (u8, Uuid, Uuid) {
    let nil = Uuid::nil();
    match c {
        ProjectionChange::Principal(v) => (0, v.id, nil),
        ProjectionChange::Organization(v) => (1, v.id, nil),
        ProjectionChange::Membership(v) => (2, v.id, nil),
        ProjectionChange::Project(v) => (3, v.id, nil),
        ProjectionChange::Initiative(v) => (4, v.id, nil),
        ProjectionChange::ProjectGrant(v) => (5, v.membership_id, v.project_id),
        ProjectionChange::RemoveProjectGrant {
            membership_id,
            project_id,
        } => (5, *membership_id, *project_id),
        ProjectionChange::InitiativeGrant(v) => (6, v.membership_id, v.initiative_id),
        ProjectionChange::RemoveInitiativeGrant {
            membership_id,
            initiative_id,
        } => (6, *membership_id, *initiative_id),
    }
}
