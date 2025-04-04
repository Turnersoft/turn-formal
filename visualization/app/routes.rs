//! Routes module for the formalize_v2 visualization app
//!
//! Defines all routes and their corresponding components for the application

#[cfg(feature = "theorem_visualizer")]
use leptos::*;
#[cfg(feature = "theorem_visualizer")]
use leptos_router::*;

// Import visualization components for different domains
#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::visualization::components::docs::DocumentationViewer;
#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::visualization::components::foundation::FoundationTheoryViewer;
#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::visualization::components::home::HomePage;
#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::visualization::components::logic::LogicViewer;
#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::visualization::components::math::MathDomainViewer;

/// The main routes component for the application
#[component]
#[cfg(feature = "theorem_visualizer")]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Route path="/" view=HomePage/>
        <Route path="/foundation" view=FoundationTheoryViewer/>
        <Route path="/math" view=MathDomainViewer/>
        <Route path="/logic" view=LogicViewer/>
        <Route path="/docs" view=DocumentationViewer/>
    }
}

/// 404 Not Found page component
#[component]
#[cfg(feature = "theorem_visualizer")]
pub fn NotFound() -> impl IntoView {
    // Set HTTP status code
    let status = use_context::<leptos_meta::MetaContext>().map(|meta| meta.response_codes());
    if let Some(status) = status {
        status.set(http::StatusCode::NOT_FOUND);
    }

    view! {
        <div class="not-found">
            <h1>"404"</h1>
            <h2>"Page Not Found"</h2>
            <p>"The page you're looking for doesn't exist or has been moved."</p>
            <a href="/">"Return to Home"</a>
        </div>
    }
}
