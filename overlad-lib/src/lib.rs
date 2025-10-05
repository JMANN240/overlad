use std::f64::consts::TAU;

use ab_glyph::{point, Font, GlyphId, OutlinedGlyph, PxScale, Rect, ScaleFont};
use current_previous::CurrentPrevious;
use image::{Pixel, Rgba, RgbaImage};
use imageproc::{definitions::Clamp, drawing::{text_size, Canvas}, pixelops::weighted_sum};

pub fn overlay(mut image: RgbaImage, text: String, scale: f64, thickness: f64, font: impl Font) -> RgbaImage {
    let image_min = image.width().min(image.height());
    let margin = image_min as f64 * 0.05;

    let words = text.split(" ").collect::<Vec<&str>>();
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
                    Rgba([255, 255, 255, 255]),
                    Rgba([0, 0, 0, 255]),
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
        Rgba([255, 255, 255, 255]),
        Rgba([0, 0, 0, 255]),
        thickness,
        margin as i32,
        margin as i32 + y_offset,
        font_scale,
        &font,
        &current_line,
    );

    image
}

pub fn draw_text_mut<C>(
    canvas: &mut C,
    color: C::Pixel,
    x: i32,
    y: i32,
    scale: impl Into<PxScale> + Copy,
    font: &impl Font,
    text: &str,
) where
    C: Canvas,
    <C::Pixel as Pixel>::Subpixel: Into<f32> + Clamp<f32>,
{
    let image_width = canvas.width() as i32;
    let image_height = canvas.height() as i32;

    layout_glyphs(scale, font, text, |g, bb| {
        g.draw(|gx, gy, gv| {
            let gv = (gv * 100.0).floor() / 100.0;

            let image_x = gx as i32 + x + bb.min.x.round() as i32;
            let image_y = gy as i32 + y + bb.min.y.round() as i32;
            let gv = gv.clamp(0.0, 1.0);

            if (0..image_width).contains(&image_x) && (0..image_height).contains(&image_y) {
                let image_x = image_x as u32;
                let image_y = image_y as u32;
                let pixel = canvas.get_pixel(image_x, image_y);
                let weighted_color = weighted_sum(pixel, color, 1.0 - gv, gv);
                canvas.draw_pixel(image_x, image_y, weighted_color);
            }
        })
    });
}

pub fn draw_text_outline_mut<C>(
    canvas: &mut C,
    color: C::Pixel,
    outline_color: C::Pixel,
    thickness: f64,
    x: i32,
    y: i32,
    scale: impl Into<PxScale> + Copy,
    font: &impl Font,
    text: &str,
) where
    C: Canvas,
    <C::Pixel as Pixel>::Subpixel: Into<f32> + Clamp<f32>,
{
    if thickness > 0.0 {
        let steps = 32;
    
        for i in 0..steps {
            let theta = TAU * i as f64 / steps as f64;
            let x_offset = theta.cos() * thickness;
            let y_offset = theta.sin() * thickness;
    
            draw_text_mut(
                canvas,
                outline_color,
                x + x_offset.round() as i32,
                y + y_offset.round() as i32,
                scale,
                font,
                text,
            );
        }
    }

    draw_text_mut(canvas, color, x, y, scale, font, text);
}

fn layout_glyphs(
    scale: impl Into<PxScale> + Copy,
    font: &impl Font,
    text: &str,
    mut f: impl FnMut(OutlinedGlyph, Rect),
) -> (u32, u32) {
    let (mut w, mut h) = (0f32, 0f32);

    let font = font.as_scaled(scale);
    let mut last: Option<GlyphId> = None;

    for c in text.chars() {
        let glyph_id = font.glyph_id(c);
        let glyph = glyph_id.with_scale_and_position(scale, point(w, font.ascent()));
        w += font.h_advance(glyph_id);
        if let Some(g) = font.outline_glyph(glyph) {
            if let Some(last) = last {
                w += font.kern(glyph_id, last);
            }
            last = Some(glyph_id);
            let bb = g.px_bounds();
            h = h.max(bb.height());
            f(g, bb);
        }
    }

    (w as u32, h as u32)
}
