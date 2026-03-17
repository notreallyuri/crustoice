use crate::structures::ids::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SendFriendRequest {
    pub target_username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRelationshipRequest {
    pub target_id: UserId,
    pub action: RelationshipAction,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RelationshipAction {
    Accept,
    Decline,
    Block,
    Remove,
}
