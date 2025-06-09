use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="h-screen w-screen flex bg-gray-50">
            // Avatar
            // <div class="h-8 w-8 absolute top-0 right-0 border border-black"></div>

            // Chat History
            <div class="w-72 h-full flex flex-col">
                <div class="w-full flex m-4 items-center gap-2">
                    <div class="bg-black text-white px-2 py-1 flex-1 text-center">
                        "Log In"
                    </div>
                    <div class="bg-black text-white px-2 py-1 flex-1 text-center">
                        "New Chat"
                    </div>
                </div>

                <div class="w-full hover:bg-gray-200 p-4">
                    <div class="text-md font-semibold">"Title of a chat"</div>
                    <div class="text-sm font-semibold text-gray-400">"Summary of a chat a little longer"</div>
                </div>
                <div class="w-full hover:bg-gray-200 p-4">
                    <div class="text-md font-semibold">"Title of a chat"</div>
                    <div class="text-sm font-semibold text-gray-400">"Summary of a chat"</div>
                </div>
                <div class="w-full hover:bg-gray-200 p-4">
                    <div class="text-md font-semibold">"Title of a chat"</div>
                    <div class="text-sm font-semibold text-gray-400">"Summary of a chat"</div>
                </div>

            </div>

            // Chat 
            <div class="w-1/2 h-full flex items-center justify-end mx-auto flex-col">

                // Active chat area
                <div class="w-3/4 h-full flex flex-col gap-2 justify-end mb-4 mx-auto">
                    <div class="ml-auto">"I would like to know"</div>
                    <div class="mr-auto">"I bet you would, peasant!"</div>
                    <div class="ml-auto">"rude"</div>
                    <div class="mr-auto">"I am aware"</div>
                    <div class="ml-auto">"Who dis?"</div>
                </div>

                // Model switch
                <div class="px-2 py-1 flex mr-auto items-center gap-1 text-white bg-black">
                    <svg class="w-[18px] h-[18px] text-white dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m9 5 7 7-7 7"/>
                    </svg>
                    "Gemini 2.5"
                </div>

                <div class="w-full h-16 flex border-2 border-b-0 border-black bg-white">
                    <textarea class="flex-1 border-none outline-none focus:ring-0 resize-none p-2" name="Text1" cols="40" rows="5"></textarea>
                    <button class="w-32 bg-black text-white font-semibold text-xl">"Send"</button>
                </div>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
