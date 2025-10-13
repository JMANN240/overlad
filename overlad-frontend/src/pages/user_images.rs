use gloo::net::http::Request;
use overlad_api::{Image, User};
use yew::prelude::*;
use yew_nav::use_hide_nav_menu;
use yew_router::prelude::*;

use crate::{hooks::use_scroll_to_top, Route};

#[derive(Properties, PartialEq)]
pub struct UserImagesPageProps {
    pub id: i64,
}

#[function_component]
pub fn UserImagesPage(&UserImagesPageProps { id }: &UserImagesPageProps) -> Html {
    use_hide_nav_menu(());
    use_scroll_to_top();

    let user_state = use_state(Option::default);
    let images_state = use_state(Vec::<Image>::default);

    use_effect_with(id, {
        let user_state = user_state.clone();
        let images_state = images_state.clone();

        move |id| {
            let id = *id;
            let user_state = user_state.clone();
            let images_state = images_state.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let user_response = Request::get(&format!("/api/user/{id}")).send().await.unwrap();
                let user = user_response.json::<User>().await.unwrap();
                user_state.set(Some(user));

                let images_response = Request::get(&format!("/api/user/{id}/images")).send().await.unwrap();
                let images = images_response.json::<Vec<Image>>().await.unwrap();
                images_state.set(images);
            });
        }
    });

    html! {
        <main class="p-4 sm:p-8">
            <h1 class="text-4xl sm:text-6xl mb-4">{ format!("{}'s Images", user_state.as_ref().map(|user| user.username.as_str()).unwrap_or("User")) }</h1>
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
