//! Layout module for the formalize_v2 visualization app
//!
//! Defines the main layout components for the application

#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::visualization::components::footer::Footer;
#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::visualization::components::nav::Navbar;
#[cfg(feature = "theorem_visualizer")]
use leptos::*;

/// Main layout component that wraps the entire application
#[component]
#[cfg(feature = "theorem_visualizer")]
pub fn MainLayout(children: Children) -> impl IntoView {
    view! {
        <div class="app-container">
            <Navbar />
            <main class="main-content">
                {children()}
            </main>
            <Footer />
        </div>
    }
}
