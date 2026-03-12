use crate::structures::{ids::UserId, user::UserPublic};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RelationshipStatus {
    None,
    Friend,
    Blocked,
    PendingIncoming,
    PendingOutcoming,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRelationship {
    pub id: UserId,
    pub user: UserPublic,
    pub status: RelationshipStatus,
    pub since: String,
}
