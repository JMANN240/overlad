use gloo::net::http::Request;
use overlad_api::Image;
use yew::prelude::*;
use yew_nav::use_hide_nav_menu;
use yew_router::prelude::*;

use crate::{Route, hooks::use_scroll_to_top};

#[function_component]
pub fn ImagesPage() -> Html {
    use_hide_nav_menu(());
    use_scroll_to_top();

    let images_state = use_state(Vec::<Image>::default);

    use_effect_with((), {
        let images_state = images_state.clone();

        move |_| {
            let images_state = images_state.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let images_response = Request::get("/api/all_images").send().await.unwrap();

                let images = images_response.json::<Vec<Image>>().await.unwrap();

                images_state.set(images);
            });
        }
    });

    html! {
        <main class="p-4 sm:p-8">
            <h1 class="text-4xl sm:text-6xl mb-4">{ "All Images" }</h1>
            <section class="flex flex-col flex-wrap sm:flex-row gap-2">
                {
                    images_state.iter().map(|image| {
                        html! {
                            <Link<Route> to={Route::Image { id: image.id.clone() }} classes="border">
                                <img src={format!("/api/overlay/{}", image.id)} class="sm:h-64" />
                            </Link<Route>>
                        }
                    }).collect::<Html>()
                }
            </section>
        </main>
    }
}
