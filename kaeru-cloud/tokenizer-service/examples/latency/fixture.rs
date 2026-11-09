use contracts::{
    AccessCheck, AccessMode, Action, ActionMask, AuthorizeRequest, CreateTokenRequest, Initiative,
    Membership, MembershipState, Organization, OrganizationRole, OrganizationState, PlatformRole,
    Principal, PrincipalState, Project, ProjectionSnapshot, Resource, ResourceState,
    SCHEMA_VERSION, SecretToken, TokenCeiling, TokenPurpose,
};
use uuid::Uuid;

use super::cli::Args;

pub(super) fn id(kind: u128, index: usize) -> Uuid {
    Uuid::from_u128((kind << 64) | (index as u128 + 1))
}

pub(super) fn fixture(args: &Args) -> ProjectionSnapshot {
    let owner = id(1, 0);
    let org = id(2, 0);
    let mut s = ProjectionSnapshot::default();
    for n in 0..=args.users {
        s.principals.push(Principal {
            id: id(1, n),
            role: PlatformRole::User,
            state: PrincipalState::Active,
        });
        s.memberships.push(Membership {
            id: id(3, n),
            principal_id: id(1, n),
            organization_id: org,
            role: if n == 0 {
                OrganizationRole::Admin
            } else {
                OrganizationRole::Member
            },
            state: MembershipState::Active,
            all_projects: n > 0,
            all_actions: if n > 0 {
                ActionMask::of(&[Action::Read])
            } else {
                ActionMask::EMPTY
            },
        });
    }
    s.organizations.push(Organization {
        id: org,
        owner_id: owner,
        state: OrganizationState::Active,
        deletion_reason: None,
    });
    for p in 0..args.projects {
        s.projects.push(Project {
            id: id(4, p),
            organization_id: org,
            state: ResourceState::Active,
        });
        for i in 0..args.per_project {
            s.initiatives.push(Initiative {
                id: id(5, p * args.per_project + i),
                personal_owner_id: None,
                project_id: Some(id(4, p)),
                state: ResourceState::Active,
                placement_revision: 1,
            });
        }
    }
    s
}

pub(super) fn create_request() -> CreateTokenRequest {
    CreateTokenRequest {
        schema_version: SCHEMA_VERSION,
        operation_id: Uuid::now_v7(),
        actor_id: id(1, 1),
        name: "Synthetic latency credential".into(),
        purpose: TokenPurpose::Memory,
        ceiling: TokenCeiling::AllCurrent,
        expires_at: None,
    }
}

pub(super) fn read_request(
    secret: &SecretToken,
    scenario: &str,
    index: usize,
    total: usize,
) -> AuthorizeRequest {
    let checks = match scenario {
        "authorize_batch16" => (0..16.min(total))
            .map(|n| AccessCheck {
                action: Action::Read,
                resource: Resource::Initiative(id(5, n)),
            })
            .collect(),
        "authorize_distinct" => vec![AccessCheck {
            action: Action::Read,
            resource: Resource::Initiative(id(5, index % total)),
        }],
        _ => vec![AccessCheck {
            action: if scenario == "authorize_deny" {
                Action::Write
            } else {
                Action::Read
            },
            resource: Resource::Initiative(id(5, 0)),
        }],
    };
    AuthorizeRequest {
        schema_version: SCHEMA_VERSION,
        request_id: Uuid::now_v7(),
        credential: secret.clone(),
        checks,
        mode: AccessMode::Normal,
    }
}
