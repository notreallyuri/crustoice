use crate::{client_state::ClientState, structures::error::AppError, API_URL};
use shared::http::prelude::{ChangePasswordRequest, UpdateEmailRequest, UpdateUsernameRequest};
use tauri::State;

#[tauri::command]
pub async fn update_username(
    payload: UpdateUsernameRequest,
    state: State<'_, ClientState>,
) -> Result<(), AppError> {
    let token = {
        let store = state.store.lock().await;
        store.jwt_token.clone().ok_or(AppError::NoSession)?
    };

    let res = state
        .http
        .patch(format!("{}/users/@me/account/username", API_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        return Err(AppError::from_res(res, "Username").await);
    }

    Ok(())
}

#[tauri::command]
pub async fn update_email(
    payload: UpdateEmailRequest,
    state: State<'_, ClientState>,
) -> Result<(), AppError> {
    let token = {
        let store = state.store.lock().await;
        store.jwt_token.clone().ok_or(AppError::NoSession)?
    };

    let res = state
        .http
        .patch(format!("{}/users/@me/account/email", API_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        return Err(AppError::from_res(res, "Email").await);
    }

    Ok(())
}

#[tauri::command]
pub async fn change_password(
    payload: ChangePasswordRequest,
    state: State<'_, ClientState>,
) -> Result<(), AppError> {
    let token = {
        let store = state.store.lock().await;
        store.jwt_token.clone().ok_or(AppError::NoSession)?
    };

    let res = state
        .http
        .patch(format!("{}/users/@me/account/password", API_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        return Err(AppError::from_res(res, "Password").await);
    }

    Ok(())
}
