use web_sys::window;
use yew::{hook, use_context, use_effect_with};
use yew_nav::{NavStateAction, NavStateContext};


#[hook]
pub fn use_scroll_to_top() {
	use_effect_with((), move |_| {
		let window = window().expect("Could not get window");
		window.scroll_to_with_x_and_y(0.0, 0.0);
	});
}

#[hook]
pub fn use_hide_navmenu() {
	let navmenu_state = use_context::<NavStateContext>().expect("No NavmenuState found!");

	use_effect_with((), move |_| {
		navmenu_state.dispatch(NavStateAction::Close);
	});
}
