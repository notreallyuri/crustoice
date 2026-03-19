use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub profile_color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUsernameRequest {
    pub current_password: String,
    pub new_username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEmailRequest {
    pub current_password: String,
    pub new_email: String,
}
