use gloo::utils::window;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};
use yew::prelude::*;
use yew_nav::use_hide_nav_menu;
use yew_router::prelude::*;

use crate::{Route, hooks::use_scroll_to_top};

#[function_component]
pub fn RootPage() -> Html {
    use_hide_nav_menu(());
    use_scroll_to_top();

    let text_state = use_state(|| String::from("Me When"));

    let oninput = {
        let text_state = text_state.clone();

        Callback::from(move |event: InputEvent| {
            let maybe_target = event.target();

            if let Some(input_element) =
                maybe_target.and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
            {
                text_state.set(input_element.value());
            }
        })
    };

    let example_image_id = "L5DL15OVCRnye-VryMr19KBVxkC3e9_r4WLx20Or-gg";

    let link = format!(
        "http://localhost:3000/api/overlay?id={}&text={}&thickness=1&scale=2",
        example_image_id, &*text_state
    );

    let copy_link = {
        let link = link.clone();

        Callback::from(move |_| {
            let link = link.clone();

            wasm_bindgen_futures::spawn_local(async move {
                JsFuture::from(window().navigator().clipboard().write_text(&link))
                    .await
                    .unwrap();
            });
        })
    };

    html! {
        <main class="flex flex-col p-8 gap-4 divide-y">
            <section class="grid sm:grid-cols-2 gap-4 min-h-128">
                <div>
                    <h1 class="flex text-6xl mb-2">
                        <span class="border-l border-t px-4 py-2 font-bold">
                            { "OverLad" }
                        </span>
                    </h1>
                    <h2 class="text-2xl mb-8">{ "Your personal overlay companion" }</h2>
                    <h2 class="text-2xl">
                        <Link<Route> to={Route::Register} classes="text-blue-500 underline">
                            { "Register" }
                        </Link<Route>>
                        { " or " }
                        <Link<Route> to={Route::Login} classes="text-blue-500 underline">
                            { "log in" }
                        </Link<Route>>
                        { " to start uploading your own images." }
                    </h2>
                </div>
                <div class="flex flex-wrap gap-2 content-start">
                    // <ImageWall image_ids={image_ids} image_class="h-32 border" />
                </div>
            </section>
            <section class="grid sm:grid-cols-2 gap-4 min-h-96">
                <div>
                    <h1 class="text-6xl">{ "Step 1" }</h1>
                    <h2 class="text-2xl">{ "Upload or Select and Image" }</h2>
                </div>
                <div class="flex justify-center items-center">
                    <img class="max-h-64 border" src={format!("/api/overlay?id={}&text=&thickness=0&scale=1", example_image_id)} />
                </div>
            </section>
            <section class="grid sm:grid-cols-2 gap-4 min-h-96">
                <div>
                    <h1 class="text-6xl">{ "Step 2" }</h1>
                    <h2 class="text-2xl">{ "Customize the Overlay" }</h2>
                </div>
                <div class="flex flex-col justify-center items-center gap-2">
                    <img class="max-h-64 border" src={link.clone()} />
                    <input class="outline-offset-1 focus:outline-1 border p-1 w-64" type="text" value={(*text_state).clone()} oninput={oninput} />
                </div>
            </section>
            <section class="grid sm:grid-cols-2 gap-4 min-h-96">
                <div>
                    <h1 class="text-6xl">{ "Step 3" }</h1>
                    <h2 class="text-2xl">{ "Copy Link and Use" }</h2>
                </div>
                <div class="flex flex-col justify-center items-center gap-2">
                    <img class="max-h-64 border" src={link} />
                    <button type="button" onclick={copy_link}>{ "Copy" }</button>
                </div>
            </section>
        </main>
    }
}
