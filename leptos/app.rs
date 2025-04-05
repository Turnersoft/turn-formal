use crate::leptos::pages::about_page::AboutPage;
use crate::leptos::pages::home_page::HomePage;
use crate::leptos::pages::theory_page::TheoryPage;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes, A},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app-container">
            <Router>
                <div class="main-nav">
                    <div class="logo">
                        <A href="/">"Formalize V2"</A>
                    </div>
                    <div class="nav-links">
                        <A href="/">"Home"</A>
                        <A href="/theories">"Theories"</A>
                        <A href="/about">"About"</A>
                    </div>
                </div>
                <main>
                    <Routes fallback=|| view! { <NotFound/> }>
                        <Route path=path!("/") view=HomePage/>
                        <Route path=path!("/theories") view=TheoryPage/>
                        <Route path=path!("/about") view=AboutPage/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="not-found">
            <h1>"404 - Page Not Found"</h1>
            <p>"The page you are looking for does not exist."</p>
            <A href="/">"Return to Home"</A>
        </div>
    }
}
