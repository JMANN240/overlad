use std::io::Cursor;

use ab_glyph::FontRef;
use base64::{Engine, prelude::BASE64_STANDARD};
use gloo::net::http::Request;
use image::{ImageFormat, Rgba, RgbaImage, imageops::FilterType};
use overlad_lib::overlay;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlInputElement, wasm_bindgen::JsCast, window};
use yew::prelude::*;
use yew_nav::use_hide_nav_menu;

use crate::{
    components::button::{Button, ButtonType},
    hooks::use_scroll_to_top,
};

#[derive(Properties, PartialEq)]
pub struct ImagePageProps {
    pub id: String,
}

#[function_component]
pub fn ImagePage(ImagePageProps { id }: &ImagePageProps) -> Html {
    use_hide_nav_menu(());
    use_scroll_to_top();

    let image_state = use_state(Option::<RgbaImage>::default);
    let text_state = use_state(String::default);
    let text_color_state = use_state(|| Rgba([255, 255, 255, 255]));
    let text_scale_state = use_state(|| 1.0f64);
    let outline_color_state = use_state(|| Rgba([0, 0, 0, 255]));
    let outline_thickness_state = use_state(|| 0.0f64);

    use_effect_with((), {
        let id = id.clone();
        let image_state = image_state.clone();

        move |_| {
            let id = id.clone();
            let image_state = image_state.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let image_response = Request::get(&format!("/api/overlay/{id}"))
                    .send()
                    .await
                    .unwrap();

                let image_bytes = image_response.binary().await.unwrap();

                let dynamic_image = image::load_from_memory(image_bytes.as_slice()).unwrap();

                let resized_dynamic_image = dynamic_image.resize(512, 512, FilterType::Lanczos3);

                image_state.set(Some(resized_dynamic_image.into_rgba8()));
            });
        }
    });

    let font = FontRef::try_from_slice(include_bytes!("../../../roboto.ttf")).unwrap();

    let overlaid_image_memo = use_memo(
        (image_state.clone(), text_state.clone(), text_color_state.clone(), text_scale_state.clone(), outline_color_state.clone(), outline_thickness_state.clone()),
        |(image_state, text_state, text_color_state, text_scale_state, outline_color_state, outline_thickness_state)| {
            image_state.as_ref().map(|image| {
                overlay(
                    image.clone(),
                    (**text_state).clone(),
                    **text_color_state,
                    **outline_color_state,
                    **text_scale_state,
                    **outline_thickness_state,
                    font,
                )
            })
        },
    );

    let overlaid_image_base64_memo = use_memo(overlaid_image_memo, |overlaid_image_memo| {
        (**overlaid_image_memo).as_ref().map(|overlaid_image| {
            let mut buffer = vec![];

            overlaid_image
                .write_to(&mut Cursor::new(&mut buffer), ImageFormat::WebP)
                .unwrap();

            BASE64_STANDARD.encode(buffer)
        })
    });

    let on_text_input = {
        let text_state = text_state.clone();

        Callback::from(move |event: InputEvent| {
            if let Some(input) = event
                .target()
                .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
            {
                text_state.set(input.value());
            }
        })
    };

    let on_text_color_input = {
        let text_color_state = text_color_state.clone();

        Callback::from(move |event: InputEvent| {
            if let Some(input) = event
                .target()
                .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
            {
                let value = hex::decode(&input.value()[1..]).unwrap();
                let r = value[0];
                let g = value[1];
                let b = value[2];

                text_color_state.set(Rgba([r, g, b, 255]));
            }
        })
    };

    let on_text_scale_input = {
        let text_scale_state = text_scale_state.clone();

        Callback::from(move |event: InputEvent| {
            if let Some(input) = event
                .target()
                .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
            {
                text_scale_state.set(input.value_as_number());
            }
        })
    };

    let on_outline_color_input = {
        let outline_color_state = outline_color_state.clone();

        Callback::from(move |event: InputEvent| {
            if let Some(input) = event
                .target()
                .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
            {
                let value = hex::decode(&input.value()[1..]).unwrap();
                let r = value[0];
                let g = value[1];
                let b = value[2];

                outline_color_state.set(Rgba([r, g, b, 255]));
            }
        })
    };

    let on_outline_thickness_input = {
        let outline_thickness_state = outline_thickness_state.clone();

        Callback::from(move |event: InputEvent| {
            if let Some(input) = event
                .target()
                .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
            {
                outline_thickness_state.set(input.value_as_number());
            }
        })
    };

    let link = format!(
        "{}/api/overlay/{id}?text={}&text_color={}&text_scale={}&outline_color={}&outline_thickness={}",
        window().unwrap().location().origin().unwrap(),
        &*text_state,
        hex::encode(text_color_state.0),
        *text_scale_state,
        hex::encode(outline_color_state.0),
        *outline_thickness_state
    );

    let on_copy_link = {
        let link = link.clone();

        Callback::from(move |_| {
            let link = link.clone();

            wasm_bindgen_futures::spawn_local(async move {
                JsFuture::from(window().unwrap().navigator().clipboard().write_text(&link))
                    .await
                    .unwrap();
            });
        })
    };

    html! {
        <main class="flex flex-col items-center p-4 sm:p-8">
            <div class="max-w-full w-128 flex flex-col gap-2">
                if let Some(overlaid_image_base64) = &*overlaid_image_base64_memo {
                    <img src={format!("data:image/webp;base64,{overlaid_image_base64}")} class="border max-w-128 max-h-128" />
                }
                <input value={(*text_state).clone()} oninput={on_text_input} class="bg-transparent text-gray-900 outline-blue-500 autofill:bg-blue-200 autofill:filter-none outline-offset-1 focus:outline-1 border p-1 rounded-sm" />
                <div class="flex items-center">
                    <label class="px-2 grow-0">{ "Text Color" }</label>
                    <input type="color" value={format!("#{}", hex::encode(&text_color_state.0[0..3]))} oninput={on_text_color_input} class="grow outline-offset-1 focus:outline-1 border rounded-sm" />
                </div>
                <div class="flex items-center">
                    <label class="px-2 grow-0">{ "Text Size" }</label>
                    <input type="range" min="0.2" step="0.01" max="5" value={text_scale_state.to_string()} oninput={on_text_scale_input} class="grow" />
                </div>
                <div class="flex items-center">
                    <label class="px-2 grow-0">{ "Outline Color" }</label>
                    <input type="color" value={format!("#{}", hex::encode(&outline_color_state.0[0..3]))} oninput={on_outline_color_input} class="grow outline-offset-1 focus:outline-1 border rounded-sm" />
                </div>
                <div class="flex items-center">
                    <label class="px-2 grow-0">{ "Outline Thickness" }</label>
                    <input type="range" min="0" step="1" max="10" value={outline_thickness_state.to_string()} oninput={on_outline_thickness_input} class="grow" />
                </div>
                <Button r#type={ButtonType::Button} onclick={on_copy_link}>{ "Copy Link" }</Button>
            </div>
        </main>
    }
}
