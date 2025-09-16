use yew::prelude::*;
use yew_nav::{NavBar, NavLink};
use yew_router::{components::Link};

use crate::{Route, components::token_provider::TokenContext};

#[function_component]
pub fn Navbar() -> Html {
    let token_context = use_context::<TokenContext>().expect("no token found");

    let left_nav_links = html! {
        <Link<Route> to={Route::Root}>
            <h1 class="text-xl">{ "OverLad" }</h1>
        </Link<Route>>
    };

    let right_nav_links = html! {
        if token_context.0.is_some() {
            <NavLink<Route>
                to={Route::Upload}
                classes={classes!("duration-200")}
                active_classes={classes!("border-b", "border-white")}
            >
                <h4 style="color: #FFFF00;">{ "Upload" }</h4>
            </NavLink<Route>>
            <NavLink<Route>
                to={Route::Logout}
                classes={classes!("duration-200")}
                active_classes={classes!("border-b", "border-white")}
            >
                <h4 style="color: #00FF00;">{ "Logout" }</h4>
            </NavLink<Route>>
        } else {
            <NavLink<Route>
                to={Route::Login}
                classes={classes!("duration-200")}
                active_classes={classes!("border-b", "border-white")}
            >
                <h4 style="color: #FFFF00;">{ "Login" }</h4>
            </NavLink<Route>>
            <NavLink<Route>
                to={Route::Register}
                classes={classes!("duration-200")}
                active_classes={classes!("border-b", "border-white")}
            >
                <h4 style="color: #00FF00;">{ "Register" }</h4>
            </NavLink<Route>>
        }
    };

    html! {
        <NavBar
            container_classes={classes!("flex", "items-center", "gap-2")}
            left_nav_links={left_nav_links}
            right_nav_links={right_nav_links}
        />
    }
}
