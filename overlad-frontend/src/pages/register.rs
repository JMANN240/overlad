use gloo::net::http::Request;
use overlad_api::{RegisterRequest, TokenRequest};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    Route,
    components::{button::{Button, ButtonType}, token_provider::{TokenAction, TokenContext}},
    pages::root::RootLayout,
};

#[function_component]
pub fn RegisterPage() -> Html {
    let navigator = use_navigator().unwrap();
    let token_context = use_context::<TokenContext>().expect("no token found");

    let username_input_node_ref = use_node_ref();
    let password_input_node_ref = use_node_ref();
    let confirm_password_input_node_ref = use_node_ref();

    let error_text = use_state::<Option<String>, _>(|| None);

    let username = use_state(String::default);
    let password = use_state(String::default);
    let confirm_password = use_state(String::default);

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

    let handle_confirm_password_change = {
        let confirm_password = confirm_password.clone();
        let confirm_password_input_node_ref = confirm_password_input_node_ref.clone();

        Callback::from(move |_| {
            let confirm_password_input = confirm_password_input_node_ref.cast::<HtmlInputElement>();

            if let Some(confirm_password_input) = confirm_password_input {
                confirm_password.set(confirm_password_input.value());
            }
        })
    };

    let handle_submit = {
        let error_text = error_text.clone();

        let username = username.clone();
        let password = password.clone();
        let confirm_password = confirm_password.clone();

        Callback::from(move |event: SubmitEvent| {
            let navigator = navigator.clone();
            let token_context = token_context.clone();

            let error_text = error_text.clone();

            let username = username.clone();
            let password = password.clone();
            let confirm_password = confirm_password.clone();

            event.prevent_default();

            wasm_bindgen_futures::spawn_local(async move {
                let register_request = RegisterRequest {
                    username: (*username).clone(),
                    password: (*password).clone(),
                    confirm_password: (*confirm_password).clone(),
                };

                let register_response = Request::post("/api/register")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&register_request).unwrap())
                    .unwrap()
                    .send()
                    .await
                    .unwrap();

                let register_response_status = register_response.status();

                match register_response_status {
                    201 => {
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
    
                        let token_response_status = token_response.status();
                        let token_response_text = token_response.text().await.unwrap();
    
                        match token_response_status {
                            200 => {
                                token_context.dispatch(TokenAction::Set(token_response_text));
                                navigator.push(&Route::Root);
                            },
                            _ => {
                                error_text.set(Some(token_response_text));
                            }
                        }
                    },
                    _ => {
                        let register_response_text = register_response.text().await.unwrap();
                        error_text.set(Some(register_response_text));
                    }
                }
            });
        })
    };

    html! {
        <RootLayout>
            <main class="flex flex-col items-center p-8">
                <div class="border p-4">
                    <form class="flex flex-col gap-2" onsubmit={handle_submit}>
                        if let Some(error_text) = &(*error_text) {
                            <p class="text-red-500">{error_text}</p>
                        }
                        <input ref={username_input_node_ref} class="outline-offset-1 focus:outline-1 border p-1" value={(*username).clone()} onchange={handle_username_change} type="text" name="username" placeholder="Username" required=true />
                        <input ref={password_input_node_ref} class="outline-offset-1 focus:outline-1 border p-1" value={(*password).clone()} onchange={handle_password_change} type="password" name="password" placeholder="Password" required=true />
                        <input ref={confirm_password_input_node_ref} class="outline-offset-1 focus:outline-1 border p-1" value={(*confirm_password).clone()} onchange={handle_confirm_password_change} type="password" name="confirm-password" placeholder="Confirm Password" required=true />
                        <Button r#type={ButtonType::Submit}>{ "Register" }</Button>
                    </form>
                </div>
            </main>
        </RootLayout>
    }
}
