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
pub struct AllImagesResponse {
    ids: Vec<String>,
}

pub async fn all_images(
    State(state): State<AppState>,
) -> Result<Json<AllImagesResponse>, (StatusCode, &'static str)> {
    let images = Image::get_all(&state.pool).await.unwrap();

    Ok(Json(AllImagesResponse {
        ids: images.into_iter().map(|image| image.id).collect(),
    }))
}
