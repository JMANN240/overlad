use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    Route,
    components::token_provider::{TokenAction, TokenContext},
};

#[function_component]
pub fn LogoutPage() -> Html {
    let navigator = use_navigator().unwrap();
    let token_context = use_context::<TokenContext>().expect("no token found");

    token_context.dispatch(TokenAction::Clear);
    LocalStorage::delete("token");
    navigator.replace(&Route::Root);

    html! {}
}
