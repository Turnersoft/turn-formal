//! Home page for the formalize_v2 visualization system
//!
//! Provides the main landing page component that showcases different visualization domains

#[cfg(feature = "theorem_visualizer")]
use leptos::*;
#[cfg(feature = "theorem_visualizer")]
use leptos_router::*;

/// Main homepage component
#[component]
#[cfg(feature = "theorem_visualizer")]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home-page">
            <section class="hero">
                <div class="hero-content">
                    <h1>"Formalize v2 Visualization"</h1>
                    <p class="hero-description">
                        "Explore foundational theories, mathematics, and logic through interactive visualizations."
                    </p>
                </div>
            </section>

            <section class="domain-cards">
                <h2>"Explore Domains"</h2>
                <div class="card-grid">
                    <div class="domain-card">
                        <h3>"Foundational Theories"</h3>
                        <p>
                            "Explore type theory, category theory, and other foundations of mathematics."
                        </p>
                        <A href="/foundation" class="card-link">
                            "Explore Foundations →"
                        </A>
                    </div>

                    <div class="domain-card">
                        <h3>"Mathematics"</h3>
                        <p>
                            "Visualize mathematical concepts, theorems, and their proofs."
                        </p>
                        <A href="/math" class="card-link">
                            "Explore Mathematics →"
                        </A>
                    </div>

                    <div class="domain-card">
                        <h3>"Logic"</h3>
                        <p>
                            "Understand formal logic systems, inference rules, and proof methods."
                        </p>
                        <A href="/logic" class="card-link">
                            "Explore Logic →"
                        </A>
                    </div>

                    <div class="domain-card">
                        <h3>"Documentation"</h3>
                        <p>
                            "Read comprehensive documentation on formalize_v2 concepts and tools."
                        </p>
                        <A href="/docs" class="card-link">
                            "Read Documentation →"
                        </A>
                    </div>
                </div>
            </section>

            <section class="features">
                <h2>"Key Features"</h2>
                <div class="feature-list">
                    <div class="feature-item">
                        <h3>"Interactive Visualizations"</h3>
                        <p>
                            "Interact with mathematical concepts and proofs through intuitive visualizations."
                        </p>
                    </div>

                    <div class="feature-item">
                        <h3>"Formal Verification"</h3>
                        <p>
                            "Ensure mathematical correctness with rigorous formal verification."
                        </p>
                    </div>

                    <div class="feature-item">
                        <h3>"Cross-Domain Integration"</h3>
                        <p>
                            "Explore connections between different mathematical domains and theories."
                        </p>
                    </div>

                    <div class="feature-item">
                        <h3>"Intuitive Interface"</h3>
                        <p>
                            "Navigate complex mathematical concepts through a user-friendly interface."
                        </p>
                    </div>
                </div>
            </section>
        </div>
    }
}
