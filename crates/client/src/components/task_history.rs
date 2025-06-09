use leptos::prelude::*;

#[component]
pub fn TaskHistory(
    tasks: Vec<String>,
    current_task: ReadSignal<Option<String>>,
    on_select: impl Fn(String) + 'static + Copy + Send + Sync,
) -> impl IntoView {
    let (search_term, set_search_term) = signal(String::new());
    
    let filtered_tasks = move || {
        let search = search_term.get().to_lowercase();
        if search.is_empty() {
            tasks.clone()
        } else {
            tasks.iter()
                .filter(|task| task.to_lowercase().contains(&search))
                .cloned()
                .collect()
        }
    };

    view! {
        <div class="flex-1 flex flex-col">
            // Search
            <div class="p-4 border-b border-gray-200">
                <input
                    type="text"
                    placeholder="Search tasks..."
                    class="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                    prop:value=search_term
                    on:input=move |ev| {
                        set_search_term.set(event_target_value(&ev));
                    }
                />
            </div>

            // Task List
            <div class="flex-1 overflow-y-auto">
                <div class="p-2">
                    {move || {
                        let tasks = filtered_tasks();
                        let current = current_task.get();
                        
                        tasks.into_iter().map(|task| {
                            let task_clone = task.clone();
                            let is_current = current.as_ref() == Some(&task);
                            
                            view! {
                                <button
                                    class=move || format!(
                                        "w-full text-left p-3 rounded-lg mb-2 transition-colors {}",
                                        if is_current {
                                            "bg-blue-50 border border-blue-200 text-blue-900"
                                        } else {
                                            "hover:bg-gray-50 border border-transparent"
                                        }
                                    )
                                    on:click=move |_| on_select(task_clone.clone())
                                >
                                    <div class="font-medium text-sm truncate">
                                        {task.clone()}
                                    </div>
                                    <div class="text-xs text-gray-500 mt-1">
                                        "2 hours ago"
                                    </div>
                                </button>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </div>
    }
}
