extern crate console_error_panic_hook;
use leptos::*;

mod camera;
use crate::camera::Photo;

#[component]
pub fn App() -> impl IntoView {

    view! { <Photo/> }
}

pub fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(move || {
        view! { <App/> }
    });
}
