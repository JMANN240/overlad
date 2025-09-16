use axum::{Json, extract::State, http::StatusCode};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jwt::VerifyWithKey;
use overlad_api::TokenClaims;
use serde::Serialize;

use crate::{
    AppState,
    db::image::Image,
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
        let images = Image::get_by_user_id(&state.pool, token_claims.sub).await.unwrap();

        Ok(Json(UserImagesResponse {
            ids: images.into_iter().map(|image| image.id).collect(),
        }))
    } else {
        Err((StatusCode::UNAUTHORIZED, "could not verify token"))
    }
}
