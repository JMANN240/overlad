use web_sys::window;
use yew::{hook, use_effect_with};

#[hook]
pub fn use_scroll_to_top() {
    use_effect_with((), move |_| {
        let window = window().expect("Could not get window");
        window.scroll_to_with_x_and_y(0.0, 0.0);
    });
}
