use axum::{extract::State, http::StatusCode, Json};
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};

use crate::{AppState, db::user::User};

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
}

#[derive(Deserialize)]
pub struct TokenRequest {
    username: String,
    password: String,
}

pub async fn token(
    State(state): State<AppState>,
    Json(token_request): Json<TokenRequest>,
) -> Result<(StatusCode, String), (StatusCode, &'static str)> {
    let maybe_user = User::get_by_username(&state.pool, &token_request.username)
        .await
        .unwrap();

    if let Some(user) = maybe_user {
        if user.verify_password(&token_request.password) {
            let token_claims = TokenClaims { sub: user.username };

            let token = token_claims.sign_with_key(&state.key).unwrap();

            return Ok((StatusCode::OK, token));
        }
    }
    Err((StatusCode::UNAUTHORIZED, "username or password not found"))
}
