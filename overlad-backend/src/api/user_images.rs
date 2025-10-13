use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use overlad_api::Image;

use crate::{AppState, db::image::DbImage, util::internal_server_error};

pub async fn user_images(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<Image>>, (StatusCode, String)> {
    let db_images = DbImage::get_by_user_id(&state.pool, user_id)
        .await
        .map_err(internal_server_error)?;

    let image_futures = db_images
        .into_iter()
        .map(|db_image| db_image.into_image(&state.pool));

    let images = futures::future::try_join_all(image_futures)
        .await
        .map_err(internal_server_error)?;

    Ok(Json(images))
}
