use std::io::Cursor;

use ab_glyph::FontRef;
use base64::{Engine, prelude::BASE64_STANDARD};
use image::{ImageFormat, Rgba, RgbaImage};
use overlad_lib::overlay;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ClientOverlayProps {
    pub image: RgbaImage,

    #[prop_or_default]
    pub text: String,

    #[prop_or(Rgba([255, 255, 255, 255]))]
    pub text_color: Rgba<u8>,

    #[prop_or(1.0)]
    pub text_scale: f64,

    #[prop_or(Rgba([0, 0, 0, 255]))]
    pub outline_color: Rgba<u8>,

    #[prop_or(0.0)]
    pub outline_thickness: f64,

    #[prop_or_default]
    pub classes: Classes,
}

#[function_component]
pub fn ClientOverlay(
    ClientOverlayProps {
        image,
        text,
        text_color,
        text_scale,
        outline_color,
        outline_thickness,
        classes,
    }: &ClientOverlayProps,
) -> Html {
    let font = FontRef::try_from_slice(include_bytes!("../../../roboto.ttf")).unwrap();

    let overlaid_image_memo = use_memo(
        (
            image.clone(),
            text.clone(),
            text_color.clone(),
            text_scale.clone(),
            outline_color.clone(),
            outline_thickness.clone(),
        ),
        |(
            image,
            text,
            text_color,
            text_scale,
            outline_color,
            outline_thickness,
        )| {
            overlay(
                image.clone(),
                text.clone(),
                *text_color,
                *outline_color,
                *text_scale,
                *outline_thickness,
                font,
            )
        }
    );

    let overlaid_image_base64_memo = use_memo(overlaid_image_memo, |overlaid_image_memo| {
        let mut buffer = vec![];

        overlaid_image_memo
            .write_to(&mut Cursor::new(&mut buffer), ImageFormat::WebP)
            .unwrap();

        BASE64_STANDARD.encode(buffer)
    });

    html! {
        <img src={format!("data:image/webp;base64,{overlaid_image_base64_memo}")} class={classes.clone()} />
    }
}
