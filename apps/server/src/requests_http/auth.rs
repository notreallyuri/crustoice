use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use shared::{
    requests::CreateUserRequest,
    structures::{UserId, UserProfile},
};
use uuid::Uuid;

use crate::state::SharedState;

pub async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let mut state_guard = state.lock().await;

    let existing_user = state_guard
        .users
        .values()
        .find(|u| u.username == payload.username)
        .cloned();

    if let Some(user) = existing_user {
        println!("[SERVER] User '{}' logged in.", user.username);
        return Json(user).into_response();
    }

    let user_id = UserId(Uuid::new_v4().to_string());

    let new_user = UserProfile {
        id: user_id.clone(),
        username: payload.username.clone(),
        display_name: payload.username,
        pfp: None,
        description: None,
    };

    state_guard.users.insert(user_id, new_user.clone());
    state_guard.save_users();

    println!("[SERVER] New user registered: {}", new_user.username);
    (StatusCode::CREATED, Json(new_user)).into_response()
}
