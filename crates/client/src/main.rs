mod app;
mod auth;
mod components;
mod pages;

use leptos::prelude::*;
use app::App;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
