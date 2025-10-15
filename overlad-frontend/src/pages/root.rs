use gloo::{net::http::Request, utils::window};
use image::RgbaImage;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys::encode_uri, wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;
use yew_nav::use_hide_nav_menu;
use yew_router::prelude::*;

use crate::{
    Route,
    components::{
        button::{Button, ButtonType},
        client_overlay::ClientOverlay,
    },
    hooks::use_scroll_to_top,
};

#[function_component]
pub fn RootPage() -> Html {
    use_hide_nav_menu(());
    use_scroll_to_top();

    let example_image_state = use_state(Option::<RgbaImage>::default);
    let text_state = use_state(|| String::from("Me When"));

    let example_image_id = "enXNxBF4Du7NQ8Ug96c3NnRcA1krmdQJWr_6IHKAy8Y";

    use_effect_with(
        (example_image_id, example_image_state.clone()),
        |(example_image_id, example_image_state)| {
            let example_image_id = *example_image_id;
            let example_image_state = example_image_state.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let image_response = Request::get(&format!(
                    "/api/overlay/{example_image_id}?resize_width=512&resize_height=512"
                ))
                .send()
                .await
                .unwrap();

                let image_bytes = image_response.binary().await.unwrap();

                let dynamic_image = image::load_from_memory(image_bytes.as_slice()).unwrap();

                example_image_state.set(Some(dynamic_image.into_rgba8()));
            });
        },
    );

    let on_text_input = {
        let text_state = text_state.clone();

        Callback::from(move |event: InputEvent| {
            if let Some(input_element) = event
                .target()
                .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
            {
                text_state.set(input_element.value());
            }
        })
    };

    let text_scale = 2.0;
    let outline_thickness = 1.0;

    let link = format!(
        "{}/api/overlay/{}?text={}&text_scale={}&outline_thickness={}",
        window().location().origin().unwrap(),
        example_image_id,
        &*text_state,
        text_scale,
        outline_thickness,
    );

    let copy_link = {
        let link = link.clone();

        Callback::from(move |_| {
            let link = link.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let url_encoded_link = encode_uri(&link).as_string().unwrap();
                JsFuture::from(window().navigator().clipboard().write_text(&url_encoded_link))
                    .await
                    .unwrap();
            });
        })
    };

    html! {
        <main class="flex flex-col px-4 sm:px-8 divide-y">
            <section class="min-h-128 py-8 flex justify-center">
                <div class="max-w-6xl w-full" >
                    <h1 class="flex text-6xl mb-2">
                        <span class="border-l border-t px-4 py-2 font-bold">
                            { "OverLad" }
                        </span>
                    </h1>
                    <h2 class="text-2xl mb-8">{ "Your personal overlay companion" }</h2>
                    <h2 class="text-2xl">
                        <Link<Route> to={Route::Register} classes="text-blue-500 hover:text-blue-600 underline duration-200">
                            { "Register" }
                        </Link<Route>>
                        { " or " }
                        <Link<Route> to={Route::Login} classes="text-blue-500 hover:text-blue-600 underline duration-200">
                            { "log in" }
                        </Link<Route>>
                        { " to start uploading your own images." }
                    </h2>
                </div>
            </section>
            <section class="py-8 flex justify-center">
                <div class="grid sm:grid-cols-2 max-w-6xl w-full gap-4" >
                    <div>
                        <h1 class="text-6xl">{ "Step 1" }</h1>
                        <h2 class="text-2xl">{ "Upload or Select and Image" }</h2>
                    </div>
                    <div class="flex justify-center items-center">
                        if let Some(example_image) = &*example_image_state {
                            <ClientOverlay image={example_image.clone()} classes="max-h-64 border" />
                        }
                    </div>
                </div>
            </section>
            <section class="py-8 flex justify-center">
                <div class="grid sm:grid-cols-2 max-w-6xl w-full gap-4" >
                    <div>
                        <h1 class="text-6xl">{ "Step 2" }</h1>
                        <h2 class="text-2xl">{ "Customize the Overlay" }</h2>
                    </div>
                    <div class="flex justify-center items-center">
                        <div class="flex flex-col gap-2">
                            if let Some(example_image) = &*example_image_state {
                                <ClientOverlay image={example_image.clone()} text={(*text_state).clone()} text_scale={text_scale} outline_thickness={outline_thickness} classes="max-h-64 border" />
                            }
                            <input class="bg-transparent text-gray-900 outline-blue-500 outline-offset-1 focus:outline-1 border p-1 rounded-sm" type="text" value={(*text_state).clone()} oninput={on_text_input} />
                        </div>
                    </div>
                </div>
            </section>
            <section class="py-8 flex justify-center">
                <div class="grid sm:grid-cols-2 max-w-6xl w-full gap-4" >
                    <div>
                        <h1 class="text-6xl">{ "Step 3" }</h1>
                        <h2 class="text-2xl">{ "Copy Link and Use" }</h2>
                    </div>
                    <div class="flex justify-center items-center">
                        <div class="flex flex-col gap-2">
                            if let Some(example_image) = &*example_image_state {
                                <ClientOverlay image={example_image.clone()} text={(*text_state).clone()} text_scale={text_scale} outline_thickness={outline_thickness} classes="max-h-64 border" />
                            }
                            <Button r#type={ButtonType::Button} onclick={copy_link}>{ "Copy" }</Button>
                        </div>
                    </div>
                </div>
            </section>
        </main>
    }
}
