use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::auth::use_auth;
use crate::components::{TaskHistory, ChatInterface};

#[component]
pub fn MainPage() -> impl IntoView {
    let auth = use_auth();
    
    let (current_task, set_current_task) = signal(None::<String>);
    let (messages, set_messages) = signal(Vec::<ChatMessage>::new());

    // Mock data for demonstration
    let tasks = vec![
        "Build a todo app".to_string(),
        "Create user authentication".to_string(),
        "Design database schema".to_string(),
        "Implement chat interface".to_string(),
    ];

    let on_logout = move |_| {
        // Auth disabled - logout does nothing for now
    };

    let on_task_select = move |task: String| {
        set_current_task.set(Some(task.clone()));
        // In a real app, you'd load messages for this task
        set_messages.set(vec![
            ChatMessage {
                id: 1,
                content: format!("Starting work on: {}", task),
                is_user: false,
                timestamp: "Just now".to_string(),
            }
        ]);
    };

    let on_new_chat = move |_| {
        set_current_task.set(Some("New Chat".to_string()));
        set_messages.set(Vec::new());
    };

    let on_send_message = move |content: String| {
        let mut current_messages = messages.get();
        current_messages.push(ChatMessage {
            id: current_messages.len() + 1,
            content: content.clone(),
            is_user: true,
            timestamp: "Just now".to_string(),
        });
        
        // Mock AI response
        current_messages.push(ChatMessage {
            id: current_messages.len() + 1,
            content: "I understand. Let me help you with that.".to_string(),
            is_user: false,
            timestamp: "Just now".to_string(),
        });
        
        set_messages.set(current_messages);
    };

    view! {
        <div class="h-screen flex bg-gray-100">
            // Left sidebar - Task History
            <div class="w-80 bg-white border-r border-gray-200 flex flex-col">
                // Header
                <div class="p-4 border-b border-gray-200">
                    <div class="flex items-center justify-between">
                        <h1 class="text-lg font-semibold text-gray-900">"Zero Chat"</h1>
                        <button
                            on:click=on_logout
                            class="text-sm text-gray-500 hover:text-gray-700"
                        >
                            "Logout"
                        </button>
                    </div>
                    <button
                        on:click=on_new_chat
                        class="mt-3 w-full bg-blue-600 text-white px-4 py-2 rounded-md text-sm font-medium hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
                    >
                        "New Chat"
                    </button>
                </div>

                // Task History
                <TaskHistory 
                    tasks=tasks
                    current_task=current_task
                    on_select=on_task_select
                />
            </div>

            // Right side - Chat Interface
            <div class="flex-1 flex flex-col">
                <ChatInterface
                    current_task=current_task
                    messages=messages
                    on_send=on_send_message
                />
            </div>
        </div>
    }
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub id: usize,
    pub content: String,
    pub is_user: bool,
    pub timestamp: String,
}
