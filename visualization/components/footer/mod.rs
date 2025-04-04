//! Footer component for the formalize_v2 visualization system
//!
//! Provides the site footer component with links and copyright information

#[cfg(feature = "theorem_visualizer")]
use leptos::*;
#[cfg(feature = "theorem_visualizer")]
use leptos_router::*;

/// Main footer component
#[component]
#[cfg(feature = "theorem_visualizer")]
pub fn Footer() -> impl IntoView {
    // Get the current year for copyright notice
    let current_year = chrono::Utc::now().year();

    view! {
        <footer class="site-footer">
            <div class="footer-content">
                <div class="footer-section">
                    <h3>"About Formalize v2"</h3>
                    <p>
                        "Formalize v2 is a project aimed at providing tools for rigorous formal mathematics,
                        foundational theory exploration, and proof visualization."
                    </p>
                </div>

                <div class="footer-section">
                    <h3>"Quick Links"</h3>
                    <ul class="footer-links">
                        <li><A href="/">"Home"</A></li>
                        <li><A href="/foundation">"Foundations"</A></li>
                        <li><A href="/math">"Mathematics"</A></li>
                        <li><A href="/logic">"Logic"</A></li>
                        <li><A href="/docs">"Documentation"</A></li>
                    </ul>
                </div>

                <div class="footer-section">
                    <h3>"Resources"</h3>
                    <ul class="footer-links">
                        <li><a href="https://github.com/username/formalize-v2" target="_blank">"GitHub"</a></li>
                        <li><a href="#" target="_blank">"Documentation"</a></li>
                        <li><a href="#" target="_blank">"Examples"</a></li>
                    </ul>
                </div>
            </div>

            <div class="footer-bottom">
                <p class="copyright">
                    {format!("© {} Formalize v2. All rights reserved.", current_year)}
                </p>
            </div>
        </footer>
    }
}
