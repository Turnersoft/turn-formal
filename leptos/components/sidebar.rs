use crate::leptos::components::repository_reader::{MathItem, RepositoryData};
use leptos::ev::MouseEvent;
use leptos::html::{self, Li};
use leptos::prelude::*;
use leptos::web_sys;
use leptos::*;
use leptos::{IntoView, prelude::*};
use leptos_router::components::A;
use std::collections::HashSet;

/// Sidebar navigation item
#[derive(Clone, Debug)]
pub struct NavItem {
    /// Item identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// URL or route to navigate to
    pub route: String,
    /// Icon class (if any)
    pub icon: Option<String>,
    /// Child navigation items (for hierarchical navigation)
    pub children: Vec<NavItem>,
}

/// Group of navigation items
#[derive(Clone)]
pub struct NavGroup {
    /// Group title
    pub title: String,
    /// Navigation items in this group
    pub items: Vec<NavItem>,
}

/// Simple component to render a single navigation item
#[component]
fn NavItemView(#[prop(into)] item: NavItem, #[prop(into)] is_active: bool) -> impl IntoView {
    let active_class = if is_active { "active" } else { "" };
    let item_clone = item.clone();

    let has_children = !item.children.is_empty();
    let (is_expanded, set_is_expanded) = create_signal(false);

    let toggle_expand = move |ev: web_sys::MouseEvent| {
        if has_children {
            ev.prevent_default();
            set_is_expanded.update(|val| {
                *val = !*val;
            });
        }
    };

    view! {
        <li class={format!("sidebar-nav-item {}", active_class)}>
            <a
                href={item.route.clone()}
                class="nav-link"
                on:click={toggle_expand}
            >
                {item.icon.map(|icon| {
                    view! { <span class={format!("nav-icon {}", icon)}></span> }
                })}
                <span class="nav-text">{item.name.clone()}</span>
                {has_children.then(|| {
                    let icon_class = move || {
                        if is_expanded.get() {
                            "chevron-down"
                        } else {
                            "chevron-right"
                        }
                    };
                    view! { <span class={move || format!("expand-icon {}", icon_class())}></span> }
                })}
            </a>

            // Render different ULs based on expanded state but always with the same structure
            {move || {
                let children_to_render = if has_children && is_expanded.get() {
                    item_clone.children.clone()
                } else {
                    Vec::new() // Use an empty vector for the else case
                };

                let display_style = if is_expanded.get() && has_children {
                    "display: block;"
                } else {
                    "display: none;"
                };

                view! {
                    <ul class="sidebar-subnav" style=display_style>
                        {children_to_render.iter().map(|child| {
                            let child_item = child.clone();
                            view! {
                                <li>
                                    <a href={child_item.route} class="subnav-link">
                                        {child_item.name}
                                    </a>
                                </li>
                            }
                        }).collect::<Vec<_>>()}
                    </ul>
                }
            }}
        </li>
    }
}

/// Component to render a navigation group and its items
#[component]
fn NavGroupView(
    #[prop()] group: NavGroup,
    #[prop(into, optional)] active_route: String,
) -> impl IntoView {
    // Clone items immediately to avoid moving the original group.items
    let group_items = group.items.clone();
    let (content, set_content) = create_signal(group_items);

    // Helper function to check if an item is active
    let is_active_fn = move |item: NavItem| item.route == active_route;

    create_effect(move |_| {
        // Use the cloned items here as well
        let updated_items = group.items.clone();
        set_content.set(updated_items);
    });

    view! {
        <div class="sidebar-nav-group">
            <h3 class="sidebar-group-title">{group.title}</h3>
            <ul class="sidebar-nav-list">
                {move || {
                    content.get().iter().map(|item| {
                        let item_clone = item.clone();
                        let is_active = is_active_fn(item_clone.clone());
                        view! {
                            <NavItemView
                                item={item_clone}
                                is_active={is_active}
                            />
                        }
                    }).collect::<Vec<_>>()
                }}
            </ul>
        </div>
    }
}

/// Sidebar/navigation panel component
#[component]
pub fn Sidebar(
    #[prop(into, optional)] title: MaybeSignal<String>,
    #[prop(into, optional)] active_route: String,
) -> impl IntoView {
    view! {
        <div class="sidebar">
            <div class="sidebar-header">
                <h2 class="sidebar-title">{title}</h2>
            </div>
            <div class="sidebar-content">
                <p>"Default sidebar content. Use RepositorySidebar for repository navigation."</p>
            </div>
        </div>
    }
}

/// Repository Sidebar component
#[component]
pub fn RepositorySidebar() -> impl IntoView {
    let repo_data = use_context::<RepositoryData>().expect("Repository data should be provided");

    view! {
        <aside class="repository-sidebar">
            <header class="sidebar-header">
                <h2>"Repository"</h2>
            </header>
            <div class="sidebar-content">
                <ul class="repository-tree">
                    {move || {
                        let data = repo_data.0.get();
                        data.values().map(|item| {
                            match item {
                                MathItem::Theory { name, path, .. } => {
                                    let name = name.clone();
                                    let path = path.clone();
                                    view! {
                                        <li class="repo-item">
                                            <div class="theory-item">
                                                <a href={format!("/theory/{}", path)}>
                                                    {"📚 "}{name}
                                                </a>
                                            </div>
                                        </li>
                                    }
                                },
                                MathItem::Definition { name, path, .. } => {
                                    let name = name.clone();
                                    let path = path.clone();
                                    view! {
                                        <li class="repo-item">
                                            <div class="definition-item">
                                                <a href={format!("/definition/{}", path)}>
                                                    {"📄 "}{name}
                                                </a>
                                            </div>
                                        </li>
                                    }
                                },
                                MathItem::Theorem { name, path, .. } => {
                                    let name = name.clone();
                                    let path = path.clone();
                                    view! {
                                        <li class="repo-item">
                                            <div class="theorem-item">
                                                <a href={format!("/theorem/{}", path)}>
                                                    {"📜 "}{name}
                                                </a>
                                            </div>
                                        </li>
                                    }
                                }
                            }
                        }).collect::<Vec<_>>()
                    }}
                </ul>
            </div>
        </aside>
    }
}
