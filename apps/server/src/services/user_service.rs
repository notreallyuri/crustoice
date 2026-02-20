use crate::entities::{prelude::Users, users};
use sea_orm::*;
use shared::structures::{UserId, UserProfile};

pub async fn get_user_profile(
    db: &DatabaseConnection,
    user_id: &UserId,
) -> Result<Option<UserProfile>, DbErr> {
    let user = Users::find_by_id(user_id.0.clone()).one(db).await?;

    Ok(user.map(|u| UserProfile {
        username: u.username,
        display_name: u.username,
        avatar_url: u.avatar_url,
        bio: u.bio,
    }))
}
