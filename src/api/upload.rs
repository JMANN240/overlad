use axum::{Json, body::Bytes, extract::State, http::StatusCode};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use base64::prelude::*;
use jwt::VerifyWithKey;
use serde::Serialize;

use crate::{
    AppState,
    api::token::TokenClaims,
    db::{image::Image, user::User},
};

#[derive(TryFromMultipart)]
pub struct UploadMultipart {
    image: Bytes,
}

#[derive(Serialize)]
pub struct UploadResponse {
    id: String,
}

pub async fn upload(
    State(state): State<AppState>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    multipart: TypedMultipart<UploadMultipart>,
) -> Result<Json<UploadResponse>, (StatusCode, &'static str)> {
    let maybe_token_claims: Result<TokenClaims, jwt::Error> =
        authorization.token().verify_with_key(&state.key);

    if let Ok(token_claims) = maybe_token_claims {
        let user = User::get_by_username(&state.pool, &token_claims.sub)
            .await
            .unwrap()
            .unwrap();

        let image = image::load_from_memory(&multipart.image).unwrap();

        let mut id_bytes = [0u8; 32];
        rand::fill(&mut id_bytes);

        let id = BASE64_URL_SAFE_NO_PAD.encode(id_bytes);

        image.save(format!("images/{id}.png")).unwrap();

        Image::insert(&state.pool, &id, user.id).await.unwrap();

        Ok(Json(UploadResponse { id }))
    } else {
        Err((StatusCode::UNAUTHORIZED, "could not verify token"))
    }
}
