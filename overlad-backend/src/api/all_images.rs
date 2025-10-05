use axum::{Json, extract::State, http::StatusCode};
use overlad_api::Image;

use crate::{AppState, db::image::DbImage, util::internal_server_error};

pub async fn all_images(
    State(state): State<AppState>,
) -> Result<Json<Vec<Image>>, (StatusCode, String)> {
    let db_images = DbImage::get_all(&state.pool).await.unwrap();

    let image_futures = db_images
        .into_iter()
        .map(|db_image| db_image.into_image(&state.pool));

    let images = futures::future::try_join_all(image_futures)
        .await
        .map_err(internal_server_error)?;

    Ok(Json(images))
}
