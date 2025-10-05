use yew::prelude::*;
use yew_nav::use_hide_nav_menu;
use yew_router::hooks::use_navigator;

use crate::{components::token_provider::TokenContext, hooks::use_scroll_to_top};

#[derive(Properties, PartialEq)]
pub struct UserImagesPageProps {
    pub id: i64,
}

#[function_component]
pub fn UserImagesPage(&UserImagesPageProps { id }: &UserImagesPageProps) -> Html {
    use_hide_nav_menu(());
    use_scroll_to_top();

    let navigator = use_navigator().unwrap();
    let token_context = use_context::<TokenContext>().expect("no token context found");

    html! {
        <main>
            
        </main>
    }
}
