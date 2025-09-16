use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

use crate::{AppState, db::user::User};

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
    confirm_password: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(register_request): Json<RegisterRequest>,
) -> Result<(StatusCode, String), (StatusCode, &'static str)> {
    let maybe_user = User::get_by_username(&state.pool, &register_request.username)
        .await
        .unwrap();

    if maybe_user.is_some() {
        return Err((StatusCode::CONFLICT, "username is taken"));
    }

    if register_request.password != register_request.confirm_password {
        return Err((StatusCode::BAD_REQUEST, "passwords do not match"));
    }

    let result = User::insert(
        &state.pool,
        &register_request.username,
        &register_request.password,
    )
    .await
    .unwrap();

    Ok((StatusCode::CREATED, result.last_insert_rowid().to_string()))
}
