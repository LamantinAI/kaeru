use std::collections::BTreeMap;

use contracts::{
    AccessCheck, AccessMode, Action, ActionMask, DeletionReason, Initiative, InitiativeGrant,
    Membership, MembershipState, Organization, OrganizationRole, OrganizationState, PlatformRole,
    Principal, PrincipalState, Project, ProjectGrant, ProjectionSnapshot, Resource, ResourceState,
    TokenCeiling, TokenPurpose,
};
use uuid::Uuid;

use crate::error::{Error, Result};

pub fn purpose_allows(purpose: TokenPurpose, action: Action) -> bool {
    !action.is_session_only()
        && match purpose {
            TokenPurpose::Memory => action.is_memory(),
            TokenPurpose::Control => !action.is_memory(),
        }
}

pub fn check_shape(check: &AccessCheck, mode: AccessMode) -> Result<()> {
    use Action::{
        AcceptInitiative, BlockOrganization, BlockPrincipal, CreateInitiative, CreateOrganization,
        CreateOwnToken, CreatePrincipal, CreateProject, DeleteInitiative, DeleteOrganization,
        InspectDeleted, InviteMember, LeaveOrganization, ListOwnTokens, MoveOut, Read,
        RemoveMember, RestoreOrphanedOrganization, RevokeOwnToken, SetMemberGrants, SetMemberRole,
        SetPrincipalRole, TransferOrganization, UnblockOrganization, UnblockPrincipal, Write,
    };
    let valid = match check.action {
        Read | Write | MoveOut | DeleteInitiative | InspectDeleted => {
            matches!(check.resource, Resource::Initiative(_))
        }
        CreateInitiative | AcceptInitiative => matches!(
            check.resource,
            Resource::PersonalSpace(_) | Resource::Project(_)
        ),
        CreateOrganization | CreatePrincipal => matches!(check.resource, Resource::Platform),
        DeleteOrganization
        | LeaveOrganization
        | TransferOrganization
        | InviteMember
        | CreateProject
        | BlockOrganization
        | UnblockOrganization
        | RestoreOrphanedOrganization => matches!(check.resource, Resource::Organization(_)),
        RemoveMember | SetMemberGrants | SetMemberRole => {
            matches!(check.resource, Resource::Membership(_))
        }
        SetPrincipalRole | BlockPrincipal | UnblockPrincipal | CreateOwnToken | ListOwnTokens
        | RevokeOwnToken => matches!(check.resource, Resource::Principal(_)),
    };
    if !valid {
        return Err(Error::Invalid("action and resource do not match"));
    }
    if (check.action == InspectDeleted) != (mode == AccessMode::InspectDeleted) {
        return Err(Error::Invalid(
            "inspection requires its explicit action and mode",
        ));
    }
    Ok(())
}

/// Evaluates already-scoped facts. No database calls or mutable session state.
pub struct Evaluator<'a> {
    principals: BTreeMap<Uuid, &'a Principal>,
    organizations: BTreeMap<Uuid, &'a Organization>,
    memberships: BTreeMap<Uuid, &'a Membership>,
    active_memberships: BTreeMap<(Uuid, Uuid), &'a Membership>,
    projects: BTreeMap<Uuid, &'a Project>,
    initiatives: BTreeMap<Uuid, &'a Initiative>,
    project_grants: BTreeMap<(Uuid, Uuid), &'a ProjectGrant>,
    initiative_grants: BTreeMap<(Uuid, Uuid), &'a InitiativeGrant>,
}

impl<'a> Evaluator<'a> {
    pub fn new(facts: &'a ProjectionSnapshot) -> Self {
        Self {
            principals: facts.principals.iter().map(|v| (v.id, v)).collect(),
            organizations: facts.organizations.iter().map(|v| (v.id, v)).collect(),
            memberships: facts.memberships.iter().map(|v| (v.id, v)).collect(),
            active_memberships: facts
                .memberships
                .iter()
                .filter(|m| m.state == MembershipState::Active)
                .map(|m| ((m.principal_id, m.organization_id), m))
                .collect(),
            projects: facts.projects.iter().map(|v| (v.id, v)).collect(),
            initiatives: facts.initiatives.iter().map(|v| (v.id, v)).collect(),
            project_grants: facts
                .project_grants
                .iter()
                .map(|v| ((v.membership_id, v.project_id), v))
                .collect(),
            initiative_grants: facts
                .initiative_grants
                .iter()
                .map(|v| ((v.membership_id, v.initiative_id), v))
                .collect(),
        }
    }

    fn membership(&self, principal: Uuid, org: Uuid) -> Option<&Membership> {
        self.active_memberships.get(&(principal, org)).copied()
    }

    fn org_admin(&self, m: &Membership, org: &Organization) -> bool {
        m.role == OrganizationRole::Admin && org.owner_id == m.principal_id
    }

    fn project_mask(&self, m: &Membership, project: Uuid) -> ActionMask {
        let base = if m.all_projects {
            m.all_actions
        } else {
            ActionMask::EMPTY
        };
        base.union(
            self.project_grants
                .get(&(m.id, project))
                .map_or(ActionMask::EMPTY, |g| g.actions),
        )
    }

    fn initiative_mask(&self, actor: &Principal, i: &Initiative) -> ActionMask {
        if i.state != ResourceState::Active {
            return ActionMask::EMPTY;
        }
        if let Some(owner) = i.personal_owner_id {
            return if actor.role == PlatformRole::Admin || owner == actor.id {
                ActionMask::INITIATIVE_OWNER
            } else {
                ActionMask::EMPTY
            };
        }
        let Some(project) = i.project_id.and_then(|id| self.projects.get(&id)) else {
            return ActionMask::EMPTY;
        };
        let Some(org) = self.organizations.get(&project.organization_id) else {
            return ActionMask::EMPTY;
        };
        if project.state != ResourceState::Active || org.state != OrganizationState::Active {
            return ActionMask::EMPTY;
        }
        if actor.role == PlatformRole::Admin {
            return ActionMask::INITIATIVE_OWNER;
        }
        let Some(m) = self.membership(actor.id, org.id) else {
            return ActionMask::EMPTY;
        };
        if self.org_admin(m, org) {
            return ActionMask::INITIATIVE_OWNER;
        }
        let direct = self
            .initiative_grants
            .get(&(m.id, i.id))
            .filter(|g| g.placement_revision == i.placement_revision)
            .map_or(ActionMask::EMPTY, |g| g.actions);
        let mut rights = self
            .project_mask(m, project.id)
            .intersection(ActionMask::INITIATIVE_GRANT)
            .union(direct);
        if m.role == OrganizationRole::Operator {
            rights = rights.union(ActionMask::of(&[Action::DeleteInitiative]));
        }
        rights
    }

    pub fn allows(&self, actor_id: Uuid, check: &AccessCheck, mode: AccessMode) -> bool {
        let Some(actor) = self.principals.get(&actor_id) else {
            return false;
        };
        if actor.state != PrincipalState::Active || check_shape(check, mode).is_err() {
            return false;
        }
        let admin = actor.role == PlatformRole::Admin;
        let moderator = matches!(actor.role, PlatformRole::Admin | PlatformRole::Operator);
        match check.resource {
            Resource::Platform => match check.action {
                Action::CreateOrganization => true,
                Action::CreatePrincipal => admin,
                _ => false,
            },
            Resource::Initiative(id) => {
                let Some(i) = self.initiatives.get(&id) else {
                    return false;
                };
                if check.action == Action::InspectDeleted {
                    return moderator && i.state == ResourceState::Deleted;
                }
                self.initiative_mask(actor, i).contains(check.action)
            }
            Resource::PersonalSpace(owner) => {
                self.principals
                    .get(&owner)
                    .is_some_and(|p| p.state == PrincipalState::Active)
                    && (admin || owner == actor_id)
            }
            Resource::Project(id) => {
                let Some(p) = self.projects.get(&id) else {
                    return false;
                };
                let Some(org) = self.organizations.get(&p.organization_id) else {
                    return false;
                };
                if p.state != ResourceState::Active || org.state != OrganizationState::Active {
                    return false;
                }
                admin
                    || self.membership(actor_id, org.id).is_some_and(|m| {
                        self.org_admin(m, org) || self.project_mask(m, id).contains(check.action)
                    })
            }
            Resource::Principal(id) => {
                let Some(target) = self.principals.get(&id) else {
                    return false;
                };
                match check.action {
                    Action::CreateOwnToken | Action::ListOwnTokens | Action::RevokeOwnToken => {
                        id == actor_id
                    }
                    Action::SetPrincipalRole => admin,
                    Action::BlockPrincipal | Action::UnblockPrincipal => {
                        admin || (moderator && target.role == PlatformRole::User)
                    }
                    _ => false,
                }
            }
            Resource::Organization(id) => {
                let Some(org) = self.organizations.get(&id) else {
                    return false;
                };
                if check.action == Action::RestoreOrphanedOrganization {
                    return moderator
                        && org.state == OrganizationState::Deleted
                        && org.deletion_reason == Some(DeletionReason::LastMemberLeft);
                }
                if matches!(
                    check.action,
                    Action::BlockOrganization | Action::UnblockOrganization
                ) {
                    return moderator && org.state != OrganizationState::Deleted;
                }
                if org.state == OrganizationState::Deleted {
                    return false;
                }
                let m = self.membership(actor_id, id);
                match check.action {
                    Action::LeaveOrganization => m.is_some(),
                    Action::DeleteOrganization | Action::TransferOrganization => {
                        admin || m.is_some_and(|m| self.org_admin(m, org))
                    }
                    Action::InviteMember | Action::CreateProject => {
                        org.state == OrganizationState::Active
                            && (admin
                                || m.is_some_and(|m| {
                                    matches!(
                                        m.role,
                                        OrganizationRole::Admin | OrganizationRole::Operator
                                    )
                                }))
                    }
                    _ => false,
                }
            }
            Resource::Membership(id) => {
                let Some(target) = self.memberships.get(&id) else {
                    return false;
                };
                let Some(org) = self.organizations.get(&target.organization_id) else {
                    return false;
                };
                if target.state != MembershipState::Active || org.state != OrganizationState::Active
                {
                    return false;
                }
                if admin {
                    return true;
                }
                let Some(m) = self.membership(actor_id, org.id) else {
                    return false;
                };
                self.org_admin(m, org)
                    || (m.role == OrganizationRole::Operator
                        && target.role == OrganizationRole::Member
                        && check.action != Action::SetMemberRole)
            }
        }
    }

    pub fn ceiling_allows(&self, ceiling: &TokenCeiling, resource: Resource) -> bool {
        match ceiling {
            TokenCeiling::AllCurrent => true,
            TokenCeiling::Projects(ids) => {
                let project = match resource {
                    Resource::Project(id) => Some(id),
                    Resource::Initiative(id) => {
                        self.initiatives.get(&id).and_then(|i| i.project_id)
                    }
                    _ => None,
                };
                project.is_some_and(|id| ids.contains(&id))
            }
        }
    }
}
