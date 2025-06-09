use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use crate::auth::{provide_auth_context, use_auth};
use crate::pages::{LoginPage, MainPage};

#[component]
pub fn App() -> impl IntoView {
    let _auth_context = provide_auth_context();
    
    // Auth temporarily disabled - always show main page
    view! {
        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <Route path=StaticSegment("") view=HomePage/>
                <Route path=StaticSegment("login") view=LoginPage/>
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    // Auth disabled - always show MainPage
    view! {
        <MainPage/>
    }
}
