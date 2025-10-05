use std::io::Cursor;

use ab_glyph::FontRef;
use axum::{
    extract::{Path, Query},
    http::{HeaderMap, StatusCode, header::CONTENT_TYPE},
    response::IntoResponse,
};
use image::{ImageFormat, ImageReader, Rgba};
use overlad_lib::overlay;
use serde::Deserialize;

use crate::util::internal_server_error;

#[derive(Deserialize)]
pub struct OverlayQuery {
    text: Option<String>,
    text_color: Option<String>,
    text_scale: Option<f64>,
    outline_color: Option<String>,
    outline_thickness: Option<f64>,
}

pub async fn get_overlay(
    Path(id): Path<String>,
    Query(query): Query<OverlayQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let text = query.text.unwrap_or_default();
    let text_color = {
        let text_color_vec = hex::decode(query.text_color.unwrap_or(String::from("ffffffff"))).map_err(internal_server_error)?;
        Rgba::from([
            *text_color_vec.get(0).ok_or((StatusCode::BAD_REQUEST, String::from("bad text color")))?,
            *text_color_vec.get(1).ok_or((StatusCode::BAD_REQUEST, String::from("bad text color")))?,
            *text_color_vec.get(2).ok_or((StatusCode::BAD_REQUEST, String::from("bad text color")))?,
            *text_color_vec.get(3).ok_or((StatusCode::BAD_REQUEST, String::from("bad text color")))?,
        ])
    };
    let text_scale = query.text_scale.unwrap_or(1.0);
    let outline_color = {
        let outline_color_vec = hex::decode(query.outline_color.unwrap_or(String::from("000000ff"))).map_err(internal_server_error)?;
        Rgba::from([
            *outline_color_vec.get(0).ok_or((StatusCode::BAD_REQUEST, String::from("bad outline color")))?,
            *outline_color_vec.get(1).ok_or((StatusCode::BAD_REQUEST, String::from("bad outline color")))?,
            *outline_color_vec.get(2).ok_or((StatusCode::BAD_REQUEST, String::from("bad outline color")))?,
            *outline_color_vec.get(3).ok_or((StatusCode::BAD_REQUEST, String::from("bad outline color")))?,
        ])
    };
    let outline_thickness = query.outline_thickness.unwrap_or(0.0);

    let dynamic_image = ImageReader::open(format!("images/{id}.png"))
        .unwrap()
        .decode()
        .unwrap();
    let image = dynamic_image.into_rgba8();
    let font = FontRef::try_from_slice(include_bytes!("../../../roboto.ttf")).unwrap();

    let overlaid_image = overlay(image, text, text_color, outline_color, text_scale, outline_thickness, font);

    let mut buf = Cursor::new(Vec::new());
    overlaid_image.write_to(&mut buf, ImageFormat::Png).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "image/png".parse().unwrap());

    Ok((headers, buf.into_inner()))
}
