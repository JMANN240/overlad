use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use overlad_api::Image;

use crate::{AppState, db::image::DbImage, util::internal_server_error};

pub async fn get_image(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Image>, (StatusCode, String)> {
    let maybe_db_image = DbImage::get_by_id(&state.pool, &id)
        .await
        .map_err(internal_server_error)?;

    let db_image =
        maybe_db_image.ok_or((StatusCode::NOT_FOUND, format!("image {id} not found")))?;

    let image = db_image
        .into_image(&state.pool)
        .await
        .map_err(internal_server_error)?;

    Ok(Json(image))
}
