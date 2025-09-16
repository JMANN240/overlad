use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;

use crate::{components::navbar::Navbar, hooks::{use_hide_navmenu, use_scroll_to_top}};

#[derive(Properties, PartialEq)]
pub struct RootLayoutProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn RootLayout(RootLayoutProps { children }: &RootLayoutProps) -> Html {
	use_hide_navmenu();
	use_scroll_to_top();

    html! {
        <>
            <header class="sticky top-0 bg-black z-10 border-b border-white px-2 py-1">
                <Navbar />
            </header>
            { children.clone() }
        </>
    }
}

#[function_component]
pub fn RootPage() -> Html {
    let text_state = use_state(|| String::from("Me When"));

    let oninput = {
        let text_state = text_state.clone();

        Callback::from(move |event: InputEvent| {
            let maybe_target = event.target();

            if let Some(input_element) = maybe_target.and_then(|target| target.dyn_into::<HtmlInputElement>().ok()) {
                text_state.set(input_element.value());
            }
        })
    };

    let example_image_id = std::env!("EXAMPLE_IMAGE_ID");

    let link = format!("/api/overlay?id={}&text={}&thickness=1&scale=2", example_image_id, &*text_state);

    html! {
        <RootLayout>
            <main class="flex flex-col p-8 gap-4">
                <section class="grid lg:grid-cols-2 gap-4 min-h-128">
                    <div>
                        <h1 class="text-6xl">{ "OverLad" }</h1>
                        <h2 class="text-2xl mb-8">{ "Your personal overlay companion" }</h2>
                        <h2 class="text-2xl">{ "Log in to start uploading images" }</h2>
                    </div>
                    <div class="flex flex-wrap gap-2 content-start">
                        // <ImageWall image_ids={image_ids} image_class="h-32 border" />
                    </div>
                </section>
                <hr />
                <section class="grid lg:grid-cols-2 gap-4 min-h-96">
                    <div>
                        <h1 class="text-6xl">{ "Step 1" }</h1>
                        <h2 class="text-2xl">{ "Upload or Select and Image" }</h2>
                    </div>
                    <div class="flex justify-center items-center">
                        <img class="max-h-64 border" src={format!("/api/overlay?id={}&text=&thickness=0&scale=1", example_image_id)} />
                    </div>
                </section>
                <hr />
                <section class="grid lg:grid-cols-2 gap-4 min-h-96">
                    <div>
                        <h1 class="text-6xl">{ "Step 2" }</h1>
                        <h2 class="text-2xl">{ "Customize the Overlay" }</h2>
                    </div>
                    <div class="flex flex-col justify-center items-center gap-2">
                        <img class="max-h-64 border" src={link.clone()} />
                        <input class="outline-offset-1 focus:outline-1 border p-1 w-64" type="text" value={(*text_state).clone()} oninput={oninput} />
                    </div>
                </section>
                <hr />
                <section class="grid lg:grid-cols-2 gap-4 min-h-96">
                    <div>
                        <h1 class="text-6xl">{ "Step 3" }</h1>
                        <h2 class="text-2xl">{ "Copy Link and Use" }</h2>
                    </div>
                    <div class="flex flex-col justify-center items-center gap-2">
                        <img class="max-h-64 border" src={link} />
                        // <Button type="button" onClick={copyLink}>{buttonText}</Button>
                    </div>
                </section>
            </main>
        </RootLayout>
    }
}
