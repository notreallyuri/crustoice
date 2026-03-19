use crate::state::SharedState;
use crate::{
    entities::{
        prelude::{Relationships, Users},
        presence_presets, relationships, user_settings,
    },
    extractors::auth::AuthedUser,
};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use shared::structures::prelude::*;
use shared::structures::user_settings::prelude::PresencePreset;

pub async fn get_guilds(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
) -> Result<Json<Vec<Guild>>, (StatusCode, String)> {
    let guilds = crate::services::guilds::fetch::get_user_guilds(&state, &UserId(user_id))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(guilds))
}

pub async fn get_full_user_settings(
    user_id: &str,
    db: &DatabaseConnection,
) -> Result<UserSettings, sea_orm::DbErr> {
    let settings_model = user_settings::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound(format!(
            "user_settings not found for user {}",
            user_id
        )))?;

    let preset_models = presence_presets::Entity::find()
        .filter(presence_presets::Column::UserId.eq(user_id))
        .all(db)
        .await?;

    let mut settings = UserSettings::from(settings_model);
    settings.presence_presets = preset_models
        .into_iter()
        .map(PresencePreset::from)
        .collect();

    Ok(settings)
}

pub async fn get_user_profile(
    db: &DatabaseConnection,
    user_id: &UserId,
) -> Result<Option<UserProfile>, DbErr> {
    let user = Users::find_by_id(&user_id.0).one(db).await?;

    Ok(user.map(|u| UserProfile {
        username: u.username.clone(),
        display_name: u.display_name.unwrap_or(u.username),
        avatar_url: u.avatar_url,
        banner_url: u.banner_url,
        bio: u.bio,
        profile_color: u.profile_color,
    }))
}

pub async fn get_user_relationships(
    db: &DatabaseConnection,
    user_id: &UserId,
) -> Result<Vec<UserRelationship>, DbErr> {
    let rows = Relationships::find()
        .filter(
            Condition::any()
                .add(relationships::Column::UserId.eq(&user_id.0))
                .add(relationships::Column::TargetId.eq(&user_id.0)),
        )
        .find_also_related(Users)
        .all(db)
        .await?;

    let mut result = vec![];

    for (rel, user_opt) in rows {
        let is_initiator = rel.user_id == user_id.0;

        let other_id = if is_initiator {
            rel.target_id
        } else {
            rel.user_id
        };

        let Some(other_user) = user_opt else { continue };

        if other_user.id != other_id {
            continue;
        }

        let status = match rel.status {
            1 => RelationshipStatus::Friend,
            2 => RelationshipStatus::Blocked,
            3 => {
                if is_initiator {
                    RelationshipStatus::PendingOutcoming
                } else {
                    RelationshipStatus::PendingIncoming
                }
            }
            _ => RelationshipStatus::None,
        };

        result.push(UserRelationship {
            id: UserId(other_id),
            user: UserPublic {
                id: UserId(other_user.id),
                profile: UserProfile {
                    username: other_user.username.clone(),
                    display_name: other_user.display_name.unwrap_or(other_user.username),
                    avatar_url: other_user.avatar_url,
                    banner_url: other_user.banner_url,
                    bio: other_user.bio,
                    profile_color: other_user.profile_color,
                },
                presence: UserPresence {
                    status: Status::Offline,
                    preset: None,
                },
            },
            status,
            since: rel.since.to_string(),
        });
    }

    Ok(result)
}

pub async fn get_me(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
) -> Result<Json<User>, (StatusCode, String)> {
    let user = Users::find_by_id(&user_id)
        .one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let display_name = match user.display_name {
        Some(name) if !name.trim().is_empty() => name,
        _ => user.username.clone(),
    };

    let presence = crate::services::ws::presence::get_presence(&state, &UserId(user_id.clone()))
        .await
        .unwrap_or(UserPresence {
            status: Status::Offline,
            preset: None,
        });

    let user = User {
        id: UserId(user_id.clone()),
        account: UserAccount {
            email: user.email,
            verified: true,
        },
        profile: UserProfile {
            username: user.username,
            display_name,
            avatar_url: user.avatar_url,
            banner_url: user.banner_url,
            bio: user.bio,
            profile_color: user.profile_color,
        },
        settings: get_full_user_settings(&user_id, &state.db)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        presence,
    };

    Ok(Json(user))
}
