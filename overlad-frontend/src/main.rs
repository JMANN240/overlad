use yew::prelude::*;
use yew_nav::NavMenuStateProvider;
use yew_router::prelude::*;

use crate::components::nav::NavBar;
use crate::components::token_provider::TokenProvider;
use crate::pages::image::ImagePage;
use crate::pages::images::ImagesPage;
use crate::pages::login::LoginPage;
use crate::pages::logout::LogoutPage;
use crate::pages::register::RegisterPage;
use crate::pages::root::RootPage;
use crate::pages::upload::UploadPage;
use crate::pages::user_images::UserImagesPage;

pub mod components;
pub mod hooks;
pub mod pages;
pub mod util;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/images")]
    Images,
    #[at("/images/:id")]
    Image { id: String },
    #[at("/user/:id/images")]
    UserImages { id: i64 },
    #[at("/upload")]
    Upload,
    #[at("/register")]
    Register,
    #[at("/login")]
    Login,
    #[at("/logout")]
    Logout,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Root => {
            html! { <RootPage /> }
        }
        Route::Images => {
            html! { <ImagesPage /> }
        }
        Route::Image { id } => {
            html! { <ImagePage id={id} /> }
        }
        Route::UserImages { id } => {
            html! { <UserImagesPage id={id} /> }
        }
        Route::Upload => {
            html! { <UploadPage /> }
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
