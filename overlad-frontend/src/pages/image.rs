use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::components::token_provider::TokenContext;

#[derive(Properties, PartialEq)]
pub struct ImagePageProps {
    pub id: i64,
}

#[function_component]
pub fn ImagePage(&ImagePageProps { id }: &ImagePageProps) -> Html {
    let navigator = use_navigator().unwrap();
    let token_context = use_context::<TokenContext>().expect("no token context found");

    html! {
        <main>
            
        </main>
    }
}
