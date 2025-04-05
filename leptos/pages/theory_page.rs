use leptos::prelude::*;
use leptos::*;

#[component]
pub fn TheoryPage() -> impl IntoView {
    view! {
        <div class="theory-page">
            <h1>"Mathematical Theories"</h1>
            <div class="content">
                <div class="theory-card">
                    <h2>"Group Theory"</h2>
                    <p>"Explore the fundamental concepts of group theory, including symmetry, permutations, and algebraic structures."</p>
                </div>
                <div class="theory-card">
                    <h2>"Category Theory"</h2>
                    <p>"Discover the abstractions and connections in mathematics through categories, functors, and natural transformations."</p>
                </div>
            </div>
        </div>
    }
}
