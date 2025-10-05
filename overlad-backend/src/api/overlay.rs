use std::io::Cursor;

use ab_glyph::FontRef;
use axum::{
    extract::{Path, Query},
    http::{header::CONTENT_TYPE, HeaderMap},
    response::IntoResponse,
};
use image::{ImageFormat, ImageReader};
use overlad_lib::overlay;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OverlayQuery {
    text: Option<String>,
    scale: Option<f64>,
    thickness: Option<f64>,
}

pub async fn get_overlay(
    Path(id): Path<String>,
    Query(query): Query<OverlayQuery>
) -> impl IntoResponse {
    let text = query.text.unwrap_or_default();
    let scale = query.scale.unwrap_or(1.0);
    let thickness = query.thickness.unwrap_or(0.0);

    let dynamic_image = ImageReader::open(format!("images/{id}.webp"))
        .unwrap()
        .decode()
        .unwrap();
    let image = dynamic_image.into_rgba8();
    let font = FontRef::try_from_slice(include_bytes!("../../../roboto.ttf")).unwrap();

    let overlaid_image = overlay(image, text, scale, thickness, font);

    let mut buf = Cursor::new(Vec::new());
    overlaid_image.write_to(&mut buf, ImageFormat::WebP).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "image/webp".parse().unwrap());

    (headers, buf.into_inner())
}
