use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| {
        view! {
            <main>
              <h1 class="text-3xl font-bold underline bg-red-500">
                "Hello world Zero Chat!"
              </h1>
            </main>
          }
    })
}
