use crate::{
    client_state::ClientState,
    general::upload::upload_internal,
    structures::{
        crop::{CropData, TempCroppedImage},
        error::AppError,
    },
    API_URL,
};
use shared::http::prelude::UpdateProfileRequest;
use tauri::State;

#[tauri::command]
pub async fn update_profile(
    payload: UpdateProfileRequest,
    pfp_crop: Option<CropData>,
    banner_crop: Option<CropData>,
    state: State<'_, ClientState>,
) -> Result<(), AppError> {
    let (token, user_id) = {
        let store = state.store.lock().await;
        let token = store.jwt_token.clone().ok_or(AppError::NoSession)?;
        let user_id = store.user_id.clone().ok_or(AppError::NoSession)?;

        (token, user_id)
    };

    if let Some(original_path) = payload.avatar_url {
        let ext = std::path::Path::new(&original_path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("png")
            .to_string();

        let mut final_upload_path = original_path.clone();
        let mut _temp_crop: Option<TempCroppedImage> = None;

        if let Some(crop_math) = pfp_crop {
            let processed = TempCroppedImage::process(&original_path, &crop_math)?;
            final_upload_path = processed.path.to_string_lossy().to_string();
            _temp_crop = Some(processed);
        }

        upload_internal(&state, "avatar", &user_id, &ext, &final_upload_path).await?;
    }

    if let Some(original_path) = payload.banner_url {
        let ext = std::path::Path::new(&original_path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("png")
            .to_string();

        let mut final_upload_path = original_path.clone();
        let mut _temp_crop: Option<TempCroppedImage> = None;

        if let Some(crop_math) = banner_crop {
            let processed = TempCroppedImage::process(&original_path, &crop_math)?;
            final_upload_path = processed.path.to_string_lossy().to_string();
            _temp_crop = Some(processed);
        }

        upload_internal(&state, "banner", &user_id, &ext, &final_upload_path).await?;
    }

    let request = UpdateProfileRequest {
        display_name: payload.display_name,
        bio: payload.bio,
        profile_color: payload.profile_color,
        avatar_url: None,
        banner_url: None,
    };

    let res = state
        .http
        .patch(format!("{}/users/@me/profile", API_URL))
        .header("Authorization", format!("Bearer {}", token))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if !res.status().is_success() {
        return Err(AppError::Internal(format!("{}", res.status())));
    }

    Ok(())
}
