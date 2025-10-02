use yew::prelude::*;
use yew_nav::NavMenuStateProvider;
use yew_router::prelude::*;

use crate::components::nav::NavBar;
use crate::components::token_provider::TokenProvider;
use crate::pages::image::ImagePage;
use crate::pages::login::LoginPage;
use crate::pages::logout::LogoutPage;
use crate::pages::register::RegisterPage;
use crate::pages::root::RootPage;
use crate::pages::upload::UploadPage;

pub mod components;
pub mod hooks;
pub mod pages;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/images")]
    Images,
    #[at("/images/:id")]
    Image { id: i64 },
    #[at("/register")]
    Register,
    #[at("/login")]
    Login,
    #[at("/logout")]
    Logout,
    #[at("/upload")]
    Upload,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Root => {
            html! { <RootPage /> }
        }
        Route::Images => {
            html! { <RootPage /> }
        }
        Route::Image { id } => {
            html! { <ImagePage id={id} /> }
        }
        Route::Register => {
            html! { <RegisterPage /> }
        }
        Route::Login => {
            html! { <LoginPage /> }
        }
        Route::Logout => {
            html! { <LogoutPage /> }
        }
        Route::Upload => {
            html! { <UploadPage /> }
        }
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <TokenProvider>
            <NavMenuStateProvider>
                <BrowserRouter>
                    <header class="sticky top-0 bg-inherit z-10 border-b">
                        <NavBar />
                    </header>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </NavMenuStateProvider>
        </TokenProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
