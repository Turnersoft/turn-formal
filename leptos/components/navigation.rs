use leptos::prelude::*;
use leptos::*;
use leptos_router::components::A;
use leptos_router::*;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav class="main-nav">
            <div class="logo">
                <A href="/">"Turn-Formal"</A>
            </div>
            <div class="nav-links">
                <A href="/theories">"Theories"</A>
                <A href="/about">"About"</A>
            </div>
        </nav>
    }
}
