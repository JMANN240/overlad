use std::io::Cursor;

use ab_glyph::FontRef;
use base64::{Engine, prelude::BASE64_STANDARD};
use gloo::net::http::Request;
use image::{imageops::FilterType, ImageFormat, RgbaImage};
use overlad_lib::overlay;
use wasm_bindgen_futures::JsFuture;
use web_sys::{wasm_bindgen::JsCast, window, HtmlInputElement};
use yew::prelude::*;
use yew_nav::use_hide_nav_menu;

use crate::{components::button::{Button, ButtonType}, hooks::use_scroll_to_top};

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

    use_effect_with((), {
        let id = id.clone();
        let image_state = image_state.clone();

        move |_| {
            let id = id.clone();
            let image_state = image_state.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let image_response = Request::get(&format!("/api/overlay/{id}")).send().await.unwrap();

                let image_bytes = image_response.binary().await.unwrap();

                let dynamic_image = image::load_from_memory(image_bytes.as_slice()).unwrap();

                let resized_dynamic_image = dynamic_image.resize(512, 512, FilterType::Lanczos3);

                image_state.set(Some(resized_dynamic_image.into_rgba8()));
            });
        }
    });

    let font = FontRef::try_from_slice(include_bytes!("../../../roboto.ttf")).unwrap();

    let overlaid_image_memo = use_memo((image_state.clone(), text_state.clone()), |(image_state, text_state)| {
        image_state.as_ref().map(|image| {
            overlay(image.clone(), (**text_state).clone(), 1.0, 0.0, font)
        })
    });

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
            if let Some(input) = event.target().and_then(|target| target.dyn_into::<HtmlInputElement>().ok()) {
                text_state.set(input.value());
            }
        })
    };

    let link = format!("{}/api/overlay/{id}?text={}", window().unwrap().location().origin().unwrap(), &*text_state);

    let on_copy_link = {
        let link = link.clone();

        Callback::from(move |_| {
            let link = link.clone();

            wasm_bindgen_futures::spawn_local(async move {
                JsFuture::from(window().unwrap().navigator().clipboard().write_text(&link)).await.unwrap();
            });
        })
    };

    html! {
        <main class="flex flex-col items-center p-4 sm:p-8 gap-2">
            if let Some(overlaid_image_base64) = &*overlaid_image_base64_memo {
                <img src={format!("data:image/webp;base64,{overlaid_image_base64}")} class="border max-w-128 max-h-128" />
            }
            <input value={(*text_state).clone()} oninput={on_text_input} class="w-128 bg-transparent text-gray-900 outline-blue-500 autofill:bg-blue-200 autofill:filter-none outline-offset-1 focus:outline-1 border p-1 rounded-sm" />
            <Button r#type={ButtonType::Button} onclick={on_copy_link}>{ "Copy Link" }</Button>
        </main>
    }
}
