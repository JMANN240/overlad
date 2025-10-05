use gloo::net::http::Request;
use overlad_api::Image;
use web_sys::{File, FormData, HtmlInputElement, Url, wasm_bindgen::JsCast};
use yew::prelude::*;
use yew_nav::use_hide_nav_menu;
use yew_router::hooks::use_navigator;

use crate::{
    components::{
        button::{Button, ButtonType},
        token_provider::TokenContext,
    },
    hooks::use_scroll_to_top,
    util::WithToken, Route,
};

#[function_component]
pub fn UploadPage() -> Html {
    use_hide_nav_menu(());
    use_scroll_to_top();

    let navigator = use_navigator().unwrap();
    let token_context = use_context::<TokenContext>().expect("no token context found");

    let error_text_state = use_state(Option::<String>::default);
    let file_state = use_state(Option::<File>::default);

    let preview_url_memo = use_memo(file_state.clone(), |file_state| {
        file_state
            .as_ref()
            .and_then(|file| Url::create_object_url_with_blob(file).ok())
    });

    let handle_file_change = {
        let error_text_state = error_text_state.clone();
        let file_state = file_state.clone();

        Callback::from(move |event: Event| {
            let error_text_state = error_text_state.clone();
            let file_state = file_state.clone();

            if let Some(file_input) = event
                .target()
                .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
            {
                if let Some(files) = file_input.files() {
                    if let Some(file) = files.item(0) {
                        if file.type_().starts_with("image/") {
                            error_text_state.set(None);
                            file_state.set(Some(file));
                        } else {
                            error_text_state
                                .set(Some(String::from("Selecte file is not an image!")));
                            file_state.set(None);
                            file_input.set_value("");
                        }
                    }
                }
            }
        })
    };

    let handle_submit = {
        let navigator = navigator.clone();
        let token_context = token_context.clone();
        let error_text_state = error_text_state.clone();

        Callback::from(move |event: SubmitEvent| {
            let navigator = navigator.clone();
            let error_text_state = error_text_state.clone();

            event.prevent_default();

            if let Some(token) = token_context.0.clone()
                && let Some(file) = (*file_state).clone()
            {
                wasm_bindgen_futures::spawn_local(async move {
                    let form = FormData::new().unwrap();
                    form.append_with_blob("image", &file).unwrap();

                    let image_response = Request::post("/api/upload")
                        .with_token(token)
                        .body(form)
                        .unwrap()
                        .send()
                        .await
                        .unwrap();

                    if image_response.ok() {
                        let image = image_response.json::<Image>().await.unwrap();

                        navigator.push(&Route::Image { id: image.id });
                    } else {
                        let image_response_text = image_response.text().await.unwrap();

                        error_text_state.set(Some(image_response_text));
                    }
                });
            }
        })
    };

    html! {
        <main class="flex flex-col items-center p-4 sm:p-8">
            <form class="flex flex-col gap-2" enctype="multipart/form-data" onsubmit={handle_submit}>
                <input
                    onchange={handle_file_change}
                    class="bg-transparent text-gray-900 outline-blue-500 autofill:bg-blue-200 autofill:filter-none outline-offset-1 focus:outline-1 border p-1 rounded-sm"
                    type="file"
                    accept="image/*"
                    required=true
                />
                if let Some(error_text) = &*error_text_state {
                    <p class="text-red-500">{error_text}</p>
                }
                if let Some(preview_url) = &*preview_url_memo {
                    <img src={preview_url.clone()} class="border w-128" />
                }
                <Button r#type={ButtonType::Submit} disabled={preview_url_memo.is_none()}>{"Upload"}</Button>
            </form>
        </main>
    }
}
