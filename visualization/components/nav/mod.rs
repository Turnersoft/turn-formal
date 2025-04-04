//! Navigation components for the formalize_v2 visualization system
//!
//! Provides components for site navigation, menus, and navigation-related UI elements

#[cfg(feature = "theorem_visualizer")]
use leptos::*;
#[cfg(feature = "theorem_visualizer")]
use leptos_router::*;

#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::visualization::components::theme_switch::ThemeToggle;

/// Main navigation bar component
#[component]
#[cfg(feature = "theorem_visualizer")]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="navbar">
            <div class="navbar-brand">
                <A href="/" class="logo">
                    "Formalize v2"
                </A>
            </div>

            <div class="navbar-menu">
                <A href="/" exact=true class="nav-item" active_class="active">
                    "Home"
                </A>
                <A href="/foundation" class="nav-item" active_class="active">
                    "Foundations"
                </A>
                <A href="/math" class="nav-item" active_class="active">
                    "Mathematics"
                </A>
                <A href="/logic" class="nav-item" active_class="active">
                    "Logic"
                </A>
                <A href="/docs" class="nav-item" active_class="active">
                    "Documentation"
                </A>
            </div>

            <div class="navbar-end">
                <ThemeToggle/>
            </div>
        </nav>
    }
}

/// Mobile menu component for responsive navigation
#[component]
#[cfg(feature = "theorem_visualizer")]
pub fn MobileMenu() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);

    let toggle_menu = move |_| {
        set_is_open.update(|open| *open = !*open);
    };

    view! {
        <div class="mobile-menu-container">
            <button
                class="mobile-menu-toggle"
                on:click=toggle_menu
                aria-label="Toggle menu"
            >
                <span class="hamburger"></span>
            </button>

            <div class=move || format!("mobile-menu {}", if is_open.get() { "open" } else { "" })>
                <div class="mobile-menu-items">
                    <A href="/" exact=true class="mobile-nav-item" active_class="active" on:click=toggle_menu>
                        "Home"
                    </A>
                    <A href="/foundation" class="mobile-nav-item" active_class="active" on:click=toggle_menu>
                        "Foundations"
                    </A>
                    <A href="/math" class="mobile-nav-item" active_class="active" on:click=toggle_menu>
                        "Mathematics"
                    </A>
                    <A href="/logic" class="mobile-nav-item" active_class="active" on:click=toggle_menu>
                        "Logic"
                    </A>
                    <A href="/docs" class="mobile-nav-item" active_class="active" on:click=toggle_menu>
                        "Documentation"
                    </A>

                    <div class="mobile-theme-toggle">
                        <ThemeToggle/>
                    </div>
                </div>
            </div>
        </div>
    }
}
