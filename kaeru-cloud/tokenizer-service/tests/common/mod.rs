#![allow(dead_code)]

pub mod database;

use contracts::{
    Action, ActionMask, Initiative, InitiativeGrant, Membership, MembershipState, Organization,
    OrganizationRole, OrganizationState, PlatformRole, Principal, PrincipalState, Project,
    ProjectGrant, ProjectionSnapshot, ResourceState,
};
use uuid::Uuid;

pub const OWNER: Uuid = Uuid::from_u128(1);
pub const USER: Uuid = Uuid::from_u128(2);
pub const ORG_OPERATOR: Uuid = Uuid::from_u128(3);
pub const PLATFORM_OPERATOR: Uuid = Uuid::from_u128(4);
pub const ADMIN: Uuid = Uuid::from_u128(5);
pub const ORG: Uuid = Uuid::from_u128(10);
pub const P1: Uuid = Uuid::from_u128(11);
pub const P2: Uuid = Uuid::from_u128(12);
pub const I1: Uuid = Uuid::from_u128(20);
pub const I2: Uuid = Uuid::from_u128(21);
pub const I3: Uuid = Uuid::from_u128(22);
pub const PERSONAL: Uuid = Uuid::from_u128(23);
pub const OWNER_MEMBERSHIP: Uuid = Uuid::from_u128(30);
pub const USER_MEMBERSHIP: Uuid = Uuid::from_u128(31);
pub const OP_MEMBERSHIP: Uuid = Uuid::from_u128(32);

pub fn snapshot() -> ProjectionSnapshot {
    ProjectionSnapshot {
        principals: vec![
            Principal {
                id: OWNER,
                role: PlatformRole::User,
                state: PrincipalState::Active,
            },
            Principal {
                id: USER,
                role: PlatformRole::User,
                state: PrincipalState::Active,
            },
            Principal {
                id: ORG_OPERATOR,
                role: PlatformRole::User,
                state: PrincipalState::Active,
            },
            Principal {
                id: PLATFORM_OPERATOR,
                role: PlatformRole::Operator,
                state: PrincipalState::Active,
            },
            Principal {
                id: ADMIN,
                role: PlatformRole::Admin,
                state: PrincipalState::Active,
            },
        ],
        organizations: vec![Organization {
            id: ORG,
            owner_id: OWNER,
            state: OrganizationState::Active,
            deletion_reason: None,
        }],
        memberships: vec![
            Membership {
                id: OWNER_MEMBERSHIP,
                principal_id: OWNER,
                organization_id: ORG,
                role: OrganizationRole::Admin,
                state: MembershipState::Active,
                all_projects: false,
                all_actions: ActionMask::EMPTY,
            },
            Membership {
                id: USER_MEMBERSHIP,
                principal_id: USER,
                organization_id: ORG,
                role: OrganizationRole::Member,
                state: MembershipState::Active,
                all_projects: false,
                all_actions: ActionMask::EMPTY,
            },
            Membership {
                id: OP_MEMBERSHIP,
                principal_id: ORG_OPERATOR,
                organization_id: ORG,
                role: OrganizationRole::Operator,
                state: MembershipState::Active,
                all_projects: false,
                all_actions: ActionMask::EMPTY,
            },
        ],
        projects: vec![
            Project {
                id: P1,
                organization_id: ORG,
                state: ResourceState::Active,
            },
            Project {
                id: P2,
                organization_id: ORG,
                state: ResourceState::Active,
            },
        ],
        initiatives: vec![
            Initiative {
                id: I1,
                project_id: Some(P1),
                personal_owner_id: None,
                state: ResourceState::Active,
                placement_revision: 1,
            },
            Initiative {
                id: I2,
                project_id: Some(P1),
                personal_owner_id: None,
                state: ResourceState::Active,
                placement_revision: 1,
            },
            Initiative {
                id: I3,
                project_id: Some(P2),
                personal_owner_id: None,
                state: ResourceState::Active,
                placement_revision: 1,
            },
            Initiative {
                id: PERSONAL,
                project_id: None,
                personal_owner_id: Some(USER),
                state: ResourceState::Active,
                placement_revision: 1,
            },
        ],
        project_grants: vec![ProjectGrant {
            membership_id: USER_MEMBERSHIP,
            project_id: P1,
            actions: ActionMask::of(&[Action::Read]),
        }],
        initiative_grants: vec![InitiativeGrant {
            membership_id: USER_MEMBERSHIP,
            initiative_id: I1,
            actions: ActionMask::of(&[Action::Write]),
            placement_revision: 1,
        }],
    }
}
