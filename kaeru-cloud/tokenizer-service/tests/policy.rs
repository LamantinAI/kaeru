mod common;

use contracts::{
    AccessCheck, AccessMode, Action, ActionMask, MembershipState, OrganizationState,
    ProjectionChange, Resource, ResourceState, TokenCeiling, TokenPurpose,
};
use contracts::{DeletionReason, PrincipalState};
use kaeru_tokenizer_service::policy::{
    Evaluator, purpose_allows, validate_snapshot, validate_transition,
};
use uuid::Uuid;

use common::{
    ADMIN, I1, I2, I3, OP_MEMBERSHIP, ORG, ORG_OPERATOR, OWNER, P1, P2, PERSONAL,
    PLATFORM_OPERATOR, USER, USER_MEMBERSHIP, snapshot,
};

fn check(action: Action, resource: Resource) -> AccessCheck {
    AccessCheck { action, resource }
}

#[test]
fn grants_are_additive_without_opening_siblings_or_parents() {
    let mut facts = snapshot();
    validate_snapshot(&facts).unwrap();
    for (action, id, expected) in [
        (Action::Read, I1, true),
        (Action::Write, I1, true),
        (Action::Read, I2, true),
        (Action::Write, I2, false),
        (Action::Read, I3, false),
    ] {
        assert_eq!(
            Evaluator::new(&facts).allows(
                USER,
                &check(action, Resource::Initiative(id)),
                AccessMode::Normal
            ),
            expected
        );
    }
    assert!(!Evaluator::new(&facts).allows(
        USER,
        &check(Action::CreateInitiative, Resource::Project(P1)),
        AccessMode::Normal
    ));
    facts.project_grants.clear();
    assert!(Evaluator::new(&facts).allows(
        USER,
        &check(Action::Write, Resource::Initiative(I1)),
        AccessMode::Normal
    ));
    assert!(!Evaluator::new(&facts).allows(
        USER,
        &check(Action::Read, Resource::Initiative(I1)),
        AccessMode::Normal
    ));
}

#[test]
fn all_projects_is_a_base_and_unchecking_it_preserves_direct_grants() {
    let mut facts = snapshot();
    let member = facts
        .memberships
        .iter_mut()
        .find(|m| m.id == USER_MEMBERSHIP)
        .unwrap();
    member.all_projects = true;
    member.all_actions = ActionMask::of(&[Action::Read]);
    let e = Evaluator::new(&facts);
    assert!(e.allows(
        USER,
        &check(Action::Read, Resource::Initiative(I3)),
        AccessMode::Normal
    ));
    assert!(e.allows(
        USER,
        &check(Action::Write, Resource::Initiative(I1)),
        AccessMode::Normal
    ));
    assert!(!e.allows(
        USER,
        &check(Action::Write, Resource::Initiative(I3)),
        AccessMode::Normal
    ));
    facts
        .memberships
        .iter_mut()
        .find(|m| m.id == USER_MEMBERSHIP)
        .unwrap()
        .all_projects = false;
    let e = Evaluator::new(&facts);
    assert!(!e.allows(
        USER,
        &check(Action::Read, Resource::Initiative(I3)),
        AccessMode::Normal
    ));
    assert!(e.allows(
        USER,
        &check(Action::Write, Resource::Initiative(I1)),
        AccessMode::Normal
    ));
}

#[test]
fn rejoining_does_not_resurrect_old_grants_or_reuse_an_incarnation() {
    let mut facts = snapshot();
    facts
        .memberships
        .iter_mut()
        .find(|m| m.id == USER_MEMBERSHIP)
        .unwrap()
        .state = MembershipState::Left;
    let left = facts.clone();
    let mut rejoined = facts
        .memberships
        .iter()
        .find(|m| m.id == USER_MEMBERSHIP)
        .unwrap()
        .clone();
    rejoined.id = Uuid::from_u128(90);
    rejoined.state = MembershipState::Active;
    facts.memberships.push(rejoined);
    validate_transition(&left, &facts).unwrap();
    assert!(!Evaluator::new(&facts).allows(
        USER,
        &check(Action::Read, Resource::Initiative(I1)),
        AccessMode::Normal
    ));
    let mut invalid = left.clone();
    invalid
        .memberships
        .iter_mut()
        .find(|m| m.id == USER_MEMBERSHIP)
        .unwrap()
        .state = MembershipState::Active;
    assert!(validate_transition(&left, &invalid).is_err());
}

#[test]
fn moving_back_does_not_revive_direct_grants_and_ceilings_follow_current_placement() {
    let mut facts = snapshot();
    let old = facts.clone();
    let i = facts.initiatives.iter_mut().find(|i| i.id == I1).unwrap();
    i.project_id = Some(P2);
    i.placement_revision = 2;
    validate_transition(&old, &facts).unwrap();
    let e = Evaluator::new(&facts);
    assert!(!e.ceiling_allows(&TokenCeiling::Projects(vec![P1]), Resource::Initiative(I1)));
    assert!(!e.allows(
        USER,
        &check(Action::Write, Resource::Initiative(I1)),
        AccessMode::Normal
    ));
    let i = facts.initiatives.iter_mut().find(|i| i.id == I1).unwrap();
    i.project_id = Some(P1);
    i.placement_revision = 3;
    assert!(Evaluator::new(&facts).allows(
        USER,
        &check(Action::Read, Resource::Initiative(I1)),
        AccessMode::Normal
    ));
    assert!(!Evaluator::new(&facts).allows(
        USER,
        &check(Action::Write, Resource::Initiative(I1)),
        AccessMode::Normal
    ));
}

#[test]
fn organizational_management_does_not_grant_data_access_or_peer_control() {
    let facts = snapshot();
    let e = Evaluator::new(&facts);
    assert!(!e.allows(
        ORG_OPERATOR,
        &check(Action::Read, Resource::Initiative(I1)),
        AccessMode::Normal
    ));
    assert!(e.allows(
        ORG_OPERATOR,
        &check(Action::DeleteInitiative, Resource::Initiative(I1)),
        AccessMode::Normal
    ));
    assert!(e.allows(
        ORG_OPERATOR,
        &check(
            Action::SetMemberGrants,
            Resource::Membership(USER_MEMBERSHIP)
        ),
        AccessMode::Normal
    ));
    assert!(!e.allows(
        ORG_OPERATOR,
        &check(Action::SetMemberGrants, Resource::Membership(OP_MEMBERSHIP)),
        AccessMode::Normal
    ));
    assert!(!e.allows(
        ORG_OPERATOR,
        &check(Action::SetMemberRole, Resource::Membership(USER_MEMBERSHIP)),
        AccessMode::Normal
    ));
    assert!(e.allows(
        OWNER,
        &check(Action::SetMemberRole, Resource::Membership(OP_MEMBERSHIP)),
        AccessMode::Normal
    ));
    assert!(e.allows(
        PLATFORM_OPERATOR,
        &check(Action::BlockPrincipal, Resource::Principal(ORG_OPERATOR)),
        AccessMode::Normal
    ));
    assert!(!e.allows(
        PLATFORM_OPERATOR,
        &check(Action::BlockPrincipal, Resource::Principal(ADMIN)),
        AccessMode::Normal
    ));
    assert!(!e.allows(
        PLATFORM_OPERATOR,
        &check(Action::Read, Resource::Initiative(PERSONAL)),
        AccessMode::Normal
    ));
}

#[test]
fn hidden_state_and_purpose_require_explicit_inspection() {
    let mut facts = snapshot();
    facts
        .initiatives
        .iter_mut()
        .find(|i| i.id == I1)
        .unwrap()
        .state = ResourceState::Deleted;
    let e = Evaluator::new(&facts);
    assert!(!e.allows(
        ADMIN,
        &check(Action::Read, Resource::Initiative(I1)),
        AccessMode::Normal
    ));
    assert!(e.allows(
        PLATFORM_OPERATOR,
        &check(Action::InspectDeleted, Resource::Initiative(I1)),
        AccessMode::InspectDeleted
    ));
    assert!(!e.allows(
        OWNER,
        &check(Action::InspectDeleted, Resource::Initiative(I1)),
        AccessMode::InspectDeleted
    ));
    assert!(!purpose_allows(
        TokenPurpose::Memory,
        Action::InspectDeleted
    ));
    assert!(!purpose_allows(TokenPurpose::Control, Action::Read));
    assert!(!purpose_allows(
        TokenPurpose::Control,
        Action::CreateOwnToken
    ));
    facts.organizations[0].state = OrganizationState::Blocked;
    assert!(Evaluator::new(&facts).allows(
        PLATFORM_OPERATOR,
        &check(Action::UnblockOrganization, Resource::Organization(ORG)),
        AccessMode::Normal
    ));
}

#[test]
fn invalid_projection_cannot_smuggle_privileged_grants() {
    let mut facts = snapshot();
    facts.project_grants[0].actions = ActionMask::of(&[Action::DeleteInitiative]);
    assert!(validate_snapshot(&facts).is_err());
    let mut facts = snapshot();
    facts.initiative_grants[0].actions = ActionMask::of(&[Action::CreateInitiative]);
    assert!(validate_snapshot(&facts).is_err());
    let mut facts = snapshot();
    facts.initiatives[0].personal_owner_id = Some(USER);
    assert!(validate_snapshot(&facts).is_err());
    let mut changes = vec![
        ProjectionChange::ProjectGrant(snapshot().project_grants[0].clone()),
        ProjectionChange::RemoveProjectGrant {
            membership_id: USER_MEMBERSHIP,
            project_id: P1,
        },
    ];
    assert!(kaeru_tokenizer_service::policy::normalize_changes(&mut changes).is_err());
}

#[test]
fn orphan_recovery_does_not_grant_ordinary_members_restore_power() {
    let mut facts = snapshot();
    facts.organizations[0].state = OrganizationState::Deleted;
    facts.organizations[0].deletion_reason = Some(DeletionReason::LastMemberLeft);
    for m in &mut facts.memberships {
        m.state = MembershipState::Left;
    }
    validate_snapshot(&facts).unwrap();
    let e = Evaluator::new(&facts);
    let restore = check(
        Action::RestoreOrphanedOrganization,
        Resource::Organization(ORG),
    );
    assert!(e.allows(PLATFORM_OPERATOR, &restore, AccessMode::Normal));
    assert!(e.allows(ADMIN, &restore, AccessMode::Normal));
    assert!(!e.allows(OWNER, &restore, AccessMode::Normal));
    facts.organizations[0].deletion_reason = Some(DeletionReason::Administrative);
    assert!(!Evaluator::new(&facts).allows(PLATFORM_OPERATOR, &restore, AccessMode::Normal));
}

#[test]
fn privileged_roles_do_not_create_resources_or_active_accounts_from_missing_facts() {
    let mut facts = snapshot();
    let missing = check(Action::Read, Resource::Initiative(Uuid::from_u128(999)));
    assert!(!Evaluator::new(&facts).allows(ADMIN, &missing, AccessMode::Normal));
    assert!(!Evaluator::new(&facts).ceiling_allows(
        &TokenCeiling::Projects(vec![P1]),
        Resource::PersonalSpace(USER)
    ));
    facts
        .principals
        .iter_mut()
        .find(|p| p.id == ADMIN)
        .unwrap()
        .state = PrincipalState::Blocked;
    assert!(!Evaluator::new(&facts).allows(
        ADMIN,
        &check(Action::Read, Resource::Initiative(I1)),
        AccessMode::Normal
    ));
}
