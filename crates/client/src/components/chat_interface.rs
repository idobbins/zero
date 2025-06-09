use leptos::prelude::*;
use crate::pages::main::ChatMessage;

#[component]
pub fn ChatInterface(
    current_task: ReadSignal<Option<String>>,
    messages: ReadSignal<Vec<ChatMessage>>,
    on_send: impl Fn(String) + 'static + Copy,
) -> impl IntoView {
    let (input_value, set_input_value) = signal(String::new());
    
    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let content = input_value.get().trim().to_string();
        if !content.is_empty() {
            on_send(content);
            set_input_value.set(String::new());
        }
    };

    let on_key_down = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" && !ev.shift_key() {
            ev.prevent_default();
            let content = input_value.get().trim().to_string();
            if !content.is_empty() {
                on_send(content);
                set_input_value.set(String::new());
            }
        }
    };

    view! {
        <div class="flex flex-col h-full">
            // Header
            <div class="bg-white border-b border-gray-200 px-6 py-4">
                <h2 class="text-lg font-semibold text-gray-900">
                    {move || current_task.get().unwrap_or_else(|| "Select a task or start a new chat".to_string())}
                </h2>
            </div>

            // Messages Area
            <div class="flex-1 overflow-y-auto bg-gray-50 p-6">
                {move || {
                    let msgs = messages.get();
                    if msgs.is_empty() {
                        view! {
                            <div class="flex items-center justify-center h-full">
                                <div class="text-center text-gray-500">
                                    <div class="text-lg mb-2">"💬"</div>
                                    <p>"Start a conversation or select a task from the sidebar"</p>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="space-y-4">
                                {msgs.into_iter().map(|msg| {
                                    view! {
                                        <div class=format!(
                                            "flex {}",
                                            if msg.is_user { "justify-end" } else { "justify-start" }
                                        )>
                                            <div class=format!(
                                                "max-w-xs lg:max-w-md px-4 py-2 rounded-lg {}",
                                                if msg.is_user {
                                                    "bg-blue-600 text-white"
                                                } else {
                                                    "bg-white text-gray-900 border border-gray-200"
                                                }
                                            )>
                                                <p class="text-sm">{msg.content}</p>
                                                <p class=format!(
                                                    "text-xs mt-1 {}",
                                                    if msg.is_user { "text-blue-100" } else { "text-gray-500" }
                                                )>
                                                    {msg.timestamp}
                                                </p>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    }
                }}
            </div>

            // Input Area
            <div class="bg-white border-t border-gray-200 p-4">
                <form on:submit=on_submit class="flex space-x-4">
                    <div class="flex-1">
                        <textarea
                            placeholder="Type your message... (Press Enter to send, Shift+Enter for new line)"
                            class="w-full px-3 py-2 border border-gray-300 rounded-md resize-none focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                            rows="2"
                            prop:value=input_value
                            on:input=move |ev| {
                                set_input_value.set(event_target_value(&ev));
                            }
                            on:keydown=on_key_down
                        ></textarea>
                    </div>
                    <button
                        type="submit"
                        disabled=move || input_value.get().trim().is_empty()
                        class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        "Send"
                    </button>
                </form>
            </div>
        </div>
    }
}
