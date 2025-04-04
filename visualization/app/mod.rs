//! Main application module for the formalize_v2 visualization system
//!
//! This module contains the main app component that integrates all visualization
//! elements for different domains in the formalize_v2 system.

mod layout;
mod routes;

#[cfg(feature = "theorem_visualizer")]
mod app_impl {
    use leptos::*;
    use leptos_meta::*;
    use leptos_router::*;

    use super::layout::MainLayout;
    use super::routes::{AppRoutes, NotFound};
    use crate::formalize_v2::visualization::components::common::*;
    use crate::formalize_v2::visualization::components::nav::Navbar;
    use crate::formalize_v2::visualization::themes::ThemeProvider;

    /// The main application component for visualizing formalize_v2 concepts
    #[component]
    pub fn VisualizeApp() -> impl IntoView {
        // Set up the application metadata
        provide_meta_context();

        view! {
            <Stylesheet id="leptos" href="/pkg/formalize_v2.css"/>
            <Title text="Formalize v2 - Visualization"/>
            <Meta name="description" content="Visualization tool for formal mathematics and logic"/>

            <ThemeProvider>
                <Router>
                    <MainLayout>
                        <Routes>
                            <AppRoutes/>
                            <Route path="/*any" view=NotFound/>
                        </Routes>
                    </MainLayout>
                </Router>
            </ThemeProvider>
        }
    }
}

#[cfg(feature = "theorem_visualizer")]
pub use app_impl::VisualizeApp;
