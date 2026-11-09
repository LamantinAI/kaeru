use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};

/// Stable protocol IDs. IDs are not derived from declaration order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Action {
    Read = 0,
    Write = 1,
    CreateInitiative = 2,
    MoveOut = 3,
    AcceptInitiative = 4,
    DeleteInitiative = 5,
    CreateOrganization = 16,
    DeleteOrganization = 17,
    LeaveOrganization = 18,
    TransferOrganization = 19,
    InviteMember = 20,
    RemoveMember = 21,
    SetMemberGrants = 22,
    SetMemberRole = 23,
    CreateProject = 24,
    CreatePrincipal = 25,
    SetPrincipalRole = 26,
    BlockPrincipal = 27,
    UnblockPrincipal = 28,
    BlockOrganization = 29,
    UnblockOrganization = 30,
    InspectDeleted = 31,
    RestoreOrphanedOrganization = 32,
    CreateOwnToken = 33,
    ListOwnTokens = 34,
    RevokeOwnToken = 35,
}

impl Action {
    pub const ALL: [Self; 26] = [
        Self::Read,
        Self::Write,
        Self::CreateInitiative,
        Self::MoveOut,
        Self::AcceptInitiative,
        Self::DeleteInitiative,
        Self::CreateOrganization,
        Self::DeleteOrganization,
        Self::LeaveOrganization,
        Self::TransferOrganization,
        Self::InviteMember,
        Self::RemoveMember,
        Self::SetMemberGrants,
        Self::SetMemberRole,
        Self::CreateProject,
        Self::CreatePrincipal,
        Self::SetPrincipalRole,
        Self::BlockPrincipal,
        Self::UnblockPrincipal,
        Self::BlockOrganization,
        Self::UnblockOrganization,
        Self::InspectDeleted,
        Self::RestoreOrphanedOrganization,
        Self::CreateOwnToken,
        Self::ListOwnTokens,
        Self::RevokeOwnToken,
    ];

    pub fn from_id(id: u64) -> Option<Self> {
        Self::ALL.into_iter().find(|a| *a as u64 == id)
    }

    pub fn is_memory(self) -> bool {
        (self as u8) <= 5
    }

    pub fn is_session_only(self) -> bool {
        matches!(
            self,
            Self::CreateOwnToken | Self::ListOwnTokens | Self::RevokeOwnToken
        )
    }
}

impl Serialize for Action {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let id = u64::deserialize(deserializer)?;
        Self::from_id(id).ok_or_else(|| D::Error::custom("unknown action ID"))
    }
}

/// Known action bits only; safe to persist in a nonnegative PostgreSQL bigint.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u64", into = "u64")]
pub struct ActionMask(u64);

impl ActionMask {
    pub const EMPTY: Self = Self(0);
    pub const PROJECT_GRANT: Self = Self(0b01_1111);
    pub const INITIATIVE_GRANT: Self = Self(0b00_1011);
    pub const INITIATIVE_OWNER: Self = Self(0b10_1011);

    pub fn of(actions: &[Action]) -> Self {
        Self(
            actions
                .iter()
                .fold(0, |mask, a| mask | (1u64 << (*a as u8))),
        )
    }
    pub fn contains(self, action: Action) -> bool {
        self.0 & (1 << action as u8) != 0
    }
    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
    pub fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
    pub fn is_subset(self, other: Self) -> bool {
        self.0 & !other.0 == 0
    }
    pub fn bits(self) -> u64 {
        self.0
    }
}

impl TryFrom<u64> for ActionMask {
    type Error = &'static str;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let known = Action::ALL.iter().fold(0u64, |m, a| m | (1 << *a as u8));
        if value & !known != 0 {
            Err("unknown action mask bits")
        } else {
            Ok(Self(value))
        }
    }
}

impl From<ActionMask> for u64 {
    fn from(value: ActionMask) -> Self {
        value.0
    }
}
