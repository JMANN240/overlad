use std::f64::consts::TAU;

use ab_glyph::{Font, PxScale};
use image::Pixel;
use imageproc::{
    definitions::Clamp,
    drawing::{Canvas, draw_text_mut},
};

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
    let steps = 128;

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

    draw_text_mut(canvas, color, x, y, scale, font, text);
}
