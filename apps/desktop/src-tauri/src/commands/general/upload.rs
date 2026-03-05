use std::{collections::HashMap, fs};

use tauri::State;

use crate::{client_state::ClientState, API_URL};

#[tauri::command]
pub async fn get_upload_url(
    state: State<'_, ClientState>,
    resource: String,
    id: String,
    extension: String,
) -> Result<String, String> {
    let token = {
        let store = state.store.lock().await;
        store.jwt_token.clone().ok_or("No token")?
    };

    let mut params = HashMap::new();
    params.insert("resource", resource);
    params.insert("id", id);
    params.insert("ext", extension);

    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/upload/url", API_URL))
        .query(&params)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Server error: {}", res.status()));
    }

    res.json::<String>().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn confirm_upload(
    state: tauri::State<'_, ClientState>,
    resource: String,
    id: String,
    extension: String,
) -> Result<(), String> {
    let token = {
        let store = state.store.lock().await;
        store.jwt_token.clone().ok_or("No token")?
    };

    let mut params = HashMap::new();
    params.insert("resource", resource);
    params.insert("id", id);
    params.insert("ext", extension);

    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/upload/confirm", API_URL))
        .query(&params)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Confirmation failed: {}", res.status()));
    }

    Ok(())
}

pub async fn upload_internal(
    state: &State<'_, ClientState>,
    resource: &str,
    id: &str,
    extension: &str,
    file_path: &str,
) -> Result<(), String> {
    let token = {
        let store = state.store.lock().await;
        store
            .jwt_token
            .clone()
            .ok_or("No token available for upload")?
    };

    let mut params = HashMap::new();
    params.insert("resource", resource);
    params.insert("id", id);
    params.insert("ext", extension);

    let client = reqwest::Client::new();
    let url_res = client
        .get(format!("{}/upload/url", API_URL))
        .query(&params)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Failed to get upload URL: {}", e))?;

    if !url_res.status().is_success() {
        return Err(format!("Server refused upload URL: {}", url_res.status()));
    }

    let presigned_url = url_res.json::<String>().await.map_err(|e| e.to_string())?;

    let file_bytes =
        fs::read(file_path).map_err(|e| format!("Failed to read file from disk: {}", e))?;

    let mime_type = match extension.to_lowercase().as_str() {
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        _ => "image/jpeg",
    };

    let put_res = client
        .put(presigned_url)
        .body(file_bytes)
        .header("Content-Type", mime_type)
        .send()
        .await
        .map_err(|e| format!("Failed to upload to R2: {}", e))?;

    if !put_res.status().is_success() {
        return Err(format!("R2 upload failed: {}", put_res.status()));
    }

    let confirm_res = client
        .post(format!("{}/upload/confirm", API_URL))
        .query(&params)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Failed to confirm upload: {}", e))?;

    if !confirm_res.status().is_success() {
        return Err(format!(
            "Database confirmation failed: {}",
            confirm_res.status()
        ));
    }

    Ok(())
}
