use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::ApiError;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginParams {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "userPassword")]
    pub user_password: String,
}

pub struct AuthorizationController {
    pub router: Router,
}

impl AuthorizationController {
    pub fn new() -> Self {
        AuthorizationController {
            router: Router::new().route("/login", post(Self::login)),
        }
    }

    pub async fn login(body: Json<LoginParams>) -> Result<Json<Value>, ApiError> {
        Err(ApiError::AuthorizationFailed(format!(
            "Your account was incorrect {}:{}",
            body.user_name, body.user_password
        )))
    }

    pub async fn register() -> Result<Json<Value>, ApiError> {}
}
