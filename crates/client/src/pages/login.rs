use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use crate::auth::use_auth;

#[component]
pub fn LoginPage() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (error, set_error) = signal(None::<String>);

    let on_submit = {
        let auth = auth.clone();
        let navigate = navigate.clone();
        move |ev: web_sys::SubmitEvent| {
            ev.prevent_default();
            
            let email_val = email.get();
            let password_val = password.get();
            
            if email_val.is_empty() || password_val.is_empty() {
                set_error.set(Some("Please fill in all fields".to_string()));
                return;
            }

            let auth_clone = auth.clone();
            let navigate_clone = navigate.clone();
            
            spawn_local(async move {
                match auth_clone.login(email_val, password_val).await {
                    Ok(_) => {
                        set_error.set(None);
                        navigate_clone("/", Default::default());
                    }
                    Err(e) => {
                        set_error.set(Some(e));
                    }
                }
            });
        }
    };

    view! {
        <div class="min-h-screen bg-gray-50 flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-bold text-gray-900">
                        "Sign in to Zero Chat"
                    </h2>
                </div>
                <form class="mt-8 space-y-6" on:submit=on_submit>
                    <div class="space-y-4">
                        <div>
                            <label for="email" class="block text-sm font-medium text-gray-700">
                                "Email address"
                            </label>
                            <input
                                id="email"
                                name="email"
                                type="email"
                                required
                                class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm"
                                placeholder="Enter your email"
                                prop:value=email
                                on:input=move |ev| {
                                    set_email.set(event_target_value(&ev));
                                }
                            />
                        </div>
                        <div>
                            <label for="password" class="block text-sm font-medium text-gray-700">
                                "Password"
                            </label>
                            <input
                                id="password"
                                name="password"
                                type="password"
                                required
                                class="mt-1 appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm"
                                placeholder="Enter your password"
                                prop:value=password
                                on:input=move |ev| {
                                    set_password.set(event_target_value(&ev));
                                }
                            />
                        </div>
                    </div>

                    {move || error.get().map(|err| view! {
                        <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
                            {err}
                        </div>
                    })}

                    <div>
                        <button
                            type="submit"
                            disabled=move || auth.is_loading.get()
                            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            {move || if auth.is_loading.get() {
                                "Signing in..."
                            } else {
                                "Sign in"
                            }}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
