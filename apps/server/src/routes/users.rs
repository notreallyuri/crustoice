use crate::{services::users, state::SharedState};
use axum::{
    Router,
    routing::{delete, get, patch, post},
};

pub fn router() -> Router<SharedState> {
    Router::new()
        .route(
            "/@me/guilds/leave/{guild_id}",
            delete(users::prelude::leave_guild),
        )
        .route("/@me/guilds/join", post(users::prelude::join_guild))
        .route("/@me/guilds", get(users::prelude::get_guilds))
        .route("/@me", get(users::prelude::get_me))
        .route("/@me/profile", patch(users::prelude::update_profile))
        .route(
            "/@me/account/username",
            patch(users::prelude::update_username),
        )
        .route("/@me/account/email", patch(users::prelude::update_email))
        .route(
            "/@me/account/password",
            patch(users::prelude::change_password),
        )
}
