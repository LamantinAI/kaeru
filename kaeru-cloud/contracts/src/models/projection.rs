use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ActionMask;

macro_rules! state_enum {
    ($name:ident { $($variant:ident => $text:literal),+ }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name { $(#[serde(rename = $text)] $variant),+ }
        impl $name {
            pub fn as_str(self) -> &'static str { match self { $(Self::$variant => $text),+ } }
            pub fn parse(value: &str) -> Option<Self> { match value { $($text => Some(Self::$variant),)+ _ => None } }
        }
    };
}

state_enum!(PlatformRole { User => "user", Operator => "operator", Admin => "admin" });
state_enum!(OrganizationRole { Member => "member", Operator => "operator", Admin => "admin" });
state_enum!(PrincipalState { Active => "active", Blocked => "blocked", Deleted => "deleted" });
state_enum!(OrganizationState { Active => "active", Blocked => "blocked", Deleted => "deleted" });
state_enum!(ResourceState { Active => "active", Pending => "pending", Moving => "moving", Deleted => "deleted" });
state_enum!(MembershipState { Active => "active", Left => "left" });
state_enum!(DeletionReason { Administrative => "administrative", LastMemberLeft => "last_member_left" });

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Principal {
    pub id: Uuid,
    pub role: PlatformRole,
    pub state: PrincipalState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Organization {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub state: OrganizationState,
    pub deletion_reason: Option<DeletionReason>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Membership {
    pub id: Uuid,
    pub principal_id: Uuid,
    pub organization_id: Uuid,
    pub role: OrganizationRole,
    pub state: MembershipState,
    pub all_projects: bool,
    pub all_actions: ActionMask,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub state: ResourceState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Initiative {
    pub id: Uuid,
    pub personal_owner_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub state: ResourceState,
    pub placement_revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectGrant {
    pub membership_id: Uuid,
    pub project_id: Uuid,
    pub actions: ActionMask,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InitiativeGrant {
    pub membership_id: Uuid,
    pub initiative_id: Uuid,
    pub actions: ActionMask,
    pub placement_revision: u64,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectionSnapshot {
    pub principals: Vec<Principal>,
    pub organizations: Vec<Organization>,
    pub memberships: Vec<Membership>,
    pub projects: Vec<Project>,
    pub initiatives: Vec<Initiative>,
    pub project_grants: Vec<ProjectGrant>,
    pub initiative_grants: Vec<InitiativeGrant>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "record",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ProjectionChange {
    Principal(Principal),
    Organization(Organization),
    Membership(Membership),
    Project(Project),
    Initiative(Initiative),
    ProjectGrant(ProjectGrant),
    InitiativeGrant(InitiativeGrant),
    RemoveProjectGrant {
        membership_id: Uuid,
        project_id: Uuid,
    },
    RemoveInitiativeGrant {
        membership_id: Uuid,
        initiative_id: Uuid,
    },
}

impl ProjectionSnapshot {
    pub fn normalize(&mut self) {
        self.principals.sort_by_key(|r| r.id);
        self.organizations.sort_by_key(|r| r.id);
        self.memberships.sort_by_key(|r| r.id);
        self.projects.sort_by_key(|r| r.id);
        self.initiatives.sort_by_key(|r| r.id);
        self.project_grants
            .sort_by_key(|r| (r.membership_id, r.project_id));
        self.initiative_grants
            .sort_by_key(|r| (r.membership_id, r.initiative_id));
    }
    pub fn len(&self) -> usize {
        self.principals.len()
            + self.organizations.len()
            + self.memberships.len()
            + self.projects.len()
            + self.initiatives.len()
            + self.project_grants.len()
            + self.initiative_grants.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
