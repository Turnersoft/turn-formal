//! Module: src/formalize_v2/subjects/math/visualization/app.rs
//! Main application component for the theorem visualizer

#[cfg(feature = "theorem_visualizer")]
use leptos::*;
#[cfg(feature = "theorem_visualizer")]
use leptos_meta::*;
#[cfg(feature = "theorem_visualizer")]
use leptos_router::*;

#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::subjects::math::theorem::core::Theorem;
#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::subjects::math::visualization::components::common::*;
#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::subjects::math::visualization::components::theorem_detail::TheoremDetail;
#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::subjects::math::visualization::components::theorem_list::TheoremList;
#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::subjects::math::visualization::components::theory_selector::TheorySelector;
#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::subjects::math::visualization::loader;
#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::subjects::math::visualization::models::{
    MathLibrary, TheoryVisualization,
};

/// The main application component for the Theorem Visualizer
#[component]
#[cfg(feature = "theorem_visualizer")]
pub fn TheoremVisualizerApp() -> impl IntoView {
    // State signals for the app
    let (selected_theory, set_selected_theory) = create_signal(None::<TheoryVisualization>);
    let (selected_theorem, set_selected_theorem) = create_signal(None::<Theorem>);

    // Load the math library (resource or signal)
    let math_library = create_resource(|| (), |_| async { loader::load_math_library().await });

    // Memoize the theories for the selector
    let theories = create_memo(move |_| match math_library.get() {
        Some(lib) => lib.theories.clone(),
        None => std::collections::HashMap::new(),
    });

    // Callbacks for selection
    let on_theory_select = create_callback(move |theory: TheoryVisualization| {
        set_selected_theory.set(Some(theory));
        set_selected_theorem.set(None);
    });

    let on_theorem_select = create_callback(move |theorem: Theorem| {
        set_selected_theorem.set(Some(theorem));
    });

    view! {
        <div class="theorem-visualizer-app">
            <header>
                <h1>"Theorem Visualizer"</h1>
            </header>

            <main>
                <div class="app-container">
                    <div class="sidebar">
                        <TheorySelector
                            theories={theories}
                            on_select={on_theory_select}
                        />

                        <TheoremList
                            theory={selected_theory}
                            on_select={on_theorem_select}
                        />
                    </div>

                    <div class="content">
                        <TheoremDetail theorem={selected_theorem} />
                    </div>
                </div>
            </main>

            <footer>
                <p>"© 2023 Math Theorem Visualizer"</p>
            </footer>
        </div>
    }
}
