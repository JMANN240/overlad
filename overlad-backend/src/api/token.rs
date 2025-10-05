use axum::{Json, extract::State, http::StatusCode};
use jwt::SignWithKey;
use overlad_api::{TokenClaims, TokenRequest};

use crate::{AppState, db::user::DbUser, util::internal_server_error};

pub async fn token(
    State(state): State<AppState>,
    Json(token_request): Json<TokenRequest>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let maybe_user = DbUser::get_by_username(&state.pool, &token_request.username)
        .await
        .map_err(internal_server_error)?;

    if let Some(user) = maybe_user
        && user.verify_password(&token_request.password)
    {
        let token_claims = TokenClaims { sub: user.id };

        let token = token_claims
            .sign_with_key(&state.key)
            .map_err(internal_server_error)?;

        Ok((StatusCode::OK, token))
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            String::from("username or password not found"),
        ))
    }
}
