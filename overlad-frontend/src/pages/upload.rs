use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::components::token_provider::TokenContext;


#[function_component]
pub fn UploadPage() -> Html {
    let navigator = use_navigator().unwrap();
    let token_context = use_context::<TokenContext>().expect("no token context found");

    html! {
        <main class="flex flex-col items-center p-8">
            
        </main>
    }
}
