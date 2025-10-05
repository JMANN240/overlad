use gloo::net::http::Request;
use gloo::storage::{LocalStorage, Storage};
use overlad_api::TokenRequest;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_nav::use_hide_nav_menu;
use yew_router::hooks::use_navigator;

use crate::hooks::use_scroll_to_top;
use crate::{
    Route,
    components::{
        button::{Button, ButtonType},
        token_provider::{TokenAction, TokenContext},
    },
};

#[function_component]
pub fn LoginPage() -> Html {
    use_hide_nav_menu(());
    use_scroll_to_top();

    let navigator = use_navigator().unwrap();
    let token_context = use_context::<TokenContext>().expect("no token found");

    let username_input_node_ref = use_node_ref();
    let password_input_node_ref = use_node_ref();

    let error_text = use_state::<Option<String>, _>(|| None);

    let username = use_state(String::default);
    let password = use_state(String::default);

    let handle_username_change = {
        let username = username.clone();
        let username_input_node_ref = username_input_node_ref.clone();

        Callback::from(move |_| {
            let username_input = username_input_node_ref.cast::<HtmlInputElement>();

            if let Some(username_input) = username_input {
                username.set(username_input.value());
            }
        })
    };

    let handle_password_change = {
        let password = password.clone();
        let password_input_node_ref = password_input_node_ref.clone();

        Callback::from(move |_| {
            let password_input = password_input_node_ref.cast::<HtmlInputElement>();

            if let Some(password_input) = password_input {
                password.set(password_input.value());
            }
        })
    };

    let handle_submit = {
        let error_text = error_text.clone();

        let username = username.clone();
        let password = password.clone();

        Callback::from(move |event: SubmitEvent| {
            let navigator = navigator.clone();
            let token_context = token_context.clone();

            let error_text = error_text.clone();

            let username = username.clone();
            let password = password.clone();

            event.prevent_default();

            wasm_bindgen_futures::spawn_local(async move {
                let token_request = TokenRequest {
                    username: (*username).clone(),
                    password: (*password).clone(),
                };

                let token_response = Request::post("/api/token")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&token_request).unwrap())
                    .unwrap()
                    .send()
                    .await
                    .unwrap();

                let token_response_text = token_response.text().await.unwrap();

                if token_response.ok() {
                    LocalStorage::set("token", &token_response_text).unwrap();

                    token_context.dispatch(TokenAction::Set(token_response_text));
                    navigator.push(&Route::Root);
                } else {
                    error_text.set(Some(token_response_text));
                }
            });
        })
    };

    html! {
        <main class="flex flex-col items-center p-8">
            <form class="flex flex-col gap-2 w-64" onsubmit={handle_submit}>
                if let Some(error_text) = &(*error_text) {
                    <p class="text-red-500">{error_text}</p>
                }
                <input
                    ref={username_input_node_ref}
                    class="bg-transparent text-gray-900 outline-blue-500 outline-offset-1 focus:outline-1 border p-1 rounded-sm"
                    value={(*username).clone()}
                    onchange={handle_username_change}
                    type="text"
                    placeholder="Username"
                    required=true
                />
                <input
                    ref={password_input_node_ref}
                    class="bg-transparent text-gray-900 outline-blue-500 outline-offset-1 focus:outline-1 border p-1 rounded-sm"
                    value={(*password).clone()}
                    onchange={handle_password_change}
                    type="password"
                    placeholder="Password"
                    required=true
                />
                <Button r#type={ButtonType::Submit}>
                    { "Login" }
                </Button>
            </form>
        </main>
    }
}
