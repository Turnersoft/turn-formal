use crate::leptos::components::theory::TheoryTree;
use leptos::prelude::*;
use leptos_router::{components::A, *};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home-page">
            <div class="hero">
                <div class="hero-content">
                    <h1>"Welcome to Turn-Formal"</h1>
                    <p>"A structured approach to mathematical theories and formal systems."</p>

                    <div class="cta-buttons">
                        <A href="/theories">"Explore Theories"</A>
                    </div>
                </div>
            </div>

            <div class="featured-content">
                <h2>"Featured Theories"</h2>
                <TheoryTree />
            </div>
        </div>
    }
}
