use axum::{Json, extract::State, http::StatusCode};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jwt::VerifyWithKey;
use serde::Serialize;

use crate::{
    AppState,
    api::token::TokenClaims,
    db::{image::Image, user::User},
};

#[derive(Serialize)]
pub struct UserImagesResponse {
    ids: Vec<String>,
}

pub async fn user_images(
    State(state): State<AppState>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<UserImagesResponse>, (StatusCode, &'static str)> {
    let maybe_token_claims: Result<TokenClaims, jwt::Error> =
        authorization.token().verify_with_key(&state.key);

    if let Ok(token_claims) = maybe_token_claims {
        let user = User::get_by_username(&state.pool, &token_claims.sub)
            .await
            .unwrap()
            .unwrap();

        let images = Image::get_by_user_id(&state.pool, user.id).await.unwrap();

        Ok(Json(UserImagesResponse {
            ids: images.into_iter().map(|image| image.id).collect(),
        }))
    } else {
        Err((StatusCode::UNAUTHORIZED, "could not verify token"))
    }
}
