use std::io::Cursor;

use ab_glyph::FontRef;
use axum::{
    extract::Query,
    http::{HeaderMap, header::CONTENT_TYPE},
    response::IntoResponse,
};
use current_previous::CurrentPrevious;
use image::{ImageFormat, ImageReader, Rgb};
use imageproc::drawing::text_size;
use serde::Deserialize;

use crate::util::draw_text_outline_mut;

#[derive(Deserialize)]
pub struct OverlayQuery {
    id: String,
    text: String,
    scale: Option<f64>,
    thickness: Option<f64>,
}

pub async fn overlay(Query(query): Query<OverlayQuery>) -> impl IntoResponse {
    let scale = query.scale.unwrap_or(1.0);
    let thickness = query.thickness.unwrap_or(0.0);

    let dynamic_image = ImageReader::open(format!("images/{}.png", query.id))
        .unwrap()
        .decode()
        .unwrap();
    let mut image = dynamic_image.into_rgb8();

    let image_min = image.width().min(image.height());
    let margin = image_min as f64 * 0.05;

    let words = query.text.split(" ").collect::<Vec<&str>>();
    let font = FontRef::try_from_slice(include_bytes!("../../../roboto.ttf")).unwrap();
    let font_scale = scale as f32 * image_min as f32 * 0.1;

    let max_width = image.width() as f64 * 0.75 - 2.0 * margin;

    let thickness = thickness * image_min as f64 * 0.001;
    let mut line_words = CurrentPrevious::new(Vec::new());
    let mut y_offset = 0;
    for word in words {
        let mut new_line_words = line_words.current().clone();
        new_line_words.push(word);

        line_words.update(new_line_words);

        let current_line = line_words.current().join(" ");
        let current_measurement = text_size(font_scale, &font, &current_line);

        if let Some(previous_line_words) = line_words.previous() {
            let previous_line = previous_line_words.join(" ");

            if (current_measurement.0 as f64) > max_width
            {
                draw_text_outline_mut(
                    &mut image,
                    Rgb([255, 255, 255]),
                    Rgb([0, 0, 0]),
                    thickness,
                    margin as i32,
                    margin as i32 + y_offset,
                    font_scale,
                    &font,
                    &previous_line,
                );

                line_words.update(vec![line_words.current().last().unwrap()]);
                y_offset += font_scale as i32;
            }
        }
    }

    let current_line = line_words.current().join(" ");
    draw_text_outline_mut(
        &mut image,
        Rgb([255, 255, 255]),
        Rgb([0, 0, 0]),
        thickness,
        margin as i32,
        margin as i32 + y_offset,
        font_scale,
        &font,
        &current_line,
    );

    let mut buf = Cursor::new(Vec::new());
    image.write_to(&mut buf, ImageFormat::Png).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "image/png".parse().unwrap());

    (headers, buf.into_inner())
}
