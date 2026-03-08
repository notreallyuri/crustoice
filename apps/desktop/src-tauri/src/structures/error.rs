use serde::{Serialize, Serializer};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NoSession,
    Unauthorized,
    NotFound(String),
    InvalidPassword,
    Conflict(String),
    Internal(String),
}

impl AppError {
    pub async fn from_res(res: reqwest::Response, resource_context: &str) -> Self {
        let status = res.status();

        let backend_error = res
            .text()
            .await
            .unwrap_or_else(|_| "Unknown API error".to_string());

        match status.as_u16() {
            400 => AppError::Internal(format!("Bad Request: {}", backend_error)),
            401 => AppError::Unauthorized,
            403 => AppError::Unauthorized,
            404 => AppError::NotFound(resource_context.to_string()),
            409 => AppError::Conflict(resource_context.to_string()),
            _ => AppError::Internal(format!("API Error ({}): {}", status, backend_error)),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSession => write!(f, "No active session. Please log in again."),
            Self::Unauthorized => write!(f, "You do not have permission to perform this action."),
            Self::NotFound(item) => write!(f, "{} could not be found.", item),
            Self::InvalidPassword => write!(f, "The password you entered is incorrect."),
            Self::Conflict(item) => write!(f, "That {} is already taken.", item),
            Self::Internal(msg) => write!(f, "{}", msg),
        }
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let error_message = self.to_string();
        eprintln!("❌ [Command Error]: {}", error_message);
        serializer.serialize_str(&error_message)
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::Internal(err)
    }
}

impl From<&str> for AppError {
    fn from(err: &str) -> Self {
        AppError::Internal(err.to_string())
    }
}
