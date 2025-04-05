use leptos::prelude::*;
use leptos::*;
use leptos_router::components::A;
use leptos_router::*;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav class="main-nav">
            <div class="logo">
                <A href="/">"Formalize V2"</A>
            </div>
            <div class="nav-links">
                <A href="/">"Home"</A>
                <A href="/theories">"Theories"</A>
                <A href="/about">"About"</A>
            </div>
        </nav>
    }
}
