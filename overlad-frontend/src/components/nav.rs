use jwt::{Header, Token, Unverified};
use overlad_api::TokenClaims;
use yew::prelude::*;
use yew_nav::{NavLink, NavMenuButton, NavMenuStateContext};
use yew_router::{Routable, components::Link};

use crate::{Route, components::token_provider::TokenContext};

#[function_component]
pub fn NavBar() -> Html {
    let token_reducer = use_context::<TokenContext>().expect("no token context found");
    let nav_menu_state_reducer =
        use_context::<NavMenuStateContext>().expect("no nav menu state context found");

    let maybe_user_id = token_reducer.0.as_ref().map(|token_string |{
        let token = Token::<Header, TokenClaims, Unverified<'_>>::parse_unverified(token_string).unwrap();
        let claims: &TokenClaims = token.claims();
        claims.sub
    });

    html! {
        <nav class="flex justify-between items-center relative px-4 py-2 bg-inherit">
            <div class="flex items-center gap-4">
                <Link<Route> to={Route::Root}>
                    <h1 class="text-xl border-l border-t px-2 py-1 font-bold">
                        { "OverLad" }
                    </h1>
                </Link<Route>>
                <OverLadNavLink<Route> to={Route::Images} classes="max-sm:hidden px-1">
                    <h2>{ "Images" }</h2>
                </OverLadNavLink<Route>>
            </div>
            <div class={classes!("flex", "items-center", "gap-4", "max-sm:hidden")}>
                if let Some(user_id) = maybe_user_id {
                    <OverLadNavLink<Route> to={Route::UserImages { id: user_id }}>
                        <h2>{ "Your Images" }</h2>
                    </OverLadNavLink<Route>>
                    <OverLadNavLink<Route> to={Route::Upload}>
                        <h2>{ "Upload" }</h2>
                    </OverLadNavLink<Route>>
                    <OverLadNavLink<Route> to={Route::Logout}>
                        <h2>{ "Logout" }</h2>
                    </OverLadNavLink<Route>>
                } else {
                    <OverLadNavLink<Route> to={Route::Login}>
                        <h2>{ "Login" }</h2>
                    </OverLadNavLink<Route>>
                    <OverLadNavLink<Route> to={Route::Register}>
                        <h2>{ "Register" }</h2>
                    </OverLadNavLink<Route>>
                }
            </div>
            <NavMenuButton classes="sm:hidden text-lg">
                { "Menu" }
            </NavMenuButton>
            <div class={classes!("absolute", "top-[calc(100%+1px)]", "left-0", "right-0", "bg-inherit", "sm:hidden", "overflow-y-hidden", "duration-500", if nav_menu_state_reducer.shown { "h-64 border-b" } else { "h-0" })}>
                <div class={classes!("flex", "flex-col", "p-2", "gap-2")}>
                    <OverLadNavLink<Route> to={Route::Images}>
                        <h2>{ "Images" }</h2>
                    </OverLadNavLink<Route>>
                if let Some(user_id) = maybe_user_id {
                    <OverLadNavLink<Route> to={Route::UserImages { id: user_id }}>
                        <h2>{ "Your Images" }</h2>
                    </OverLadNavLink<Route>>
                    <OverLadNavLink<Route> to={Route::Upload}>
                        <h2>{ "Upload" }</h2>
                    </OverLadNavLink<Route>>
                    <OverLadNavLink<Route> to={Route::Logout}>
                        <h2>{ "Logout" }</h2>
                    </OverLadNavLink<Route>>
                } else {
                    <OverLadNavLink<Route> to={Route::Login}>
                        <h2>{ "Login" }</h2>
                    </OverLadNavLink<Route>>
                    <OverLadNavLink<Route> to={Route::Register}>
                        <h2>{ "Register" }</h2>
                    </OverLadNavLink<Route>>
                }
                </div>
            </div>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub struct OverLadNavLinkProps<R: PartialEq> {
    pub to: R,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn OverLadNavLink<R: Routable + 'static>(
    OverLadNavLinkProps {
        to,
        classes,
        children,
    }: &OverLadNavLinkProps<R>,
) -> Html {
    html! {
        <NavLink<R>
            classes={classes!("border-y", "border-t-transparent", classes.clone())}
            inactive_classes={classes!("border-b-transparent")}
            active_classes={classes!("border-b")}
            to={to.clone()}
        >
            { children.clone() }
        </NavLink<R>>
    }
}
