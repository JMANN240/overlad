use axum::{Json, body::Bytes, extract::State, http::StatusCode};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use base64::prelude::*;
use jwt::VerifyWithKey;
use overlad_api::{Image, TokenClaims};

use crate::{AppState, db::image::DbImage, util::internal_server_error};

#[derive(TryFromMultipart)]
pub struct UploadMultipart {
    image: Bytes,
}

pub async fn upload(
    State(state): State<AppState>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    multipart: TypedMultipart<UploadMultipart>,
) -> Result<Json<Image>, (StatusCode, String)> {
    let maybe_token_claims: Result<TokenClaims, jwt::Error> =
        authorization.token().verify_with_key(&state.key);

    if let Ok(token_claims) = maybe_token_claims {
        let image = image::load_from_memory(&multipart.image).unwrap();

        let mut id_bytes = [0u8; 32];
        rand::fill(&mut id_bytes);

        let id = BASE64_URL_SAFE_NO_PAD.encode(id_bytes);

        image.save(format!("images/{id}.png")).unwrap();

        let db_image = DbImage::insert(&state.pool, &id, token_claims.sub)
            .await
            .unwrap();

        Ok(Json(
            db_image
                .into_image(&state.pool)
                .await
                .map_err(internal_server_error)?,
        ))
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            String::from("could not verify token"),
        ))
    }
}
