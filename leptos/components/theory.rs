use crate::leptos::components::repository_reader::{MathItem, RepositoryData};
use leptos::prelude::*;
use leptos_router::{components::A, *};

/// Component to display a tree of theories
#[component]
pub fn TheoryTree() -> impl IntoView {
    let repo_data = use_context::<RepositoryData>().expect("Repository data should be provided");

    let theories = move || {
        let data = repo_data.0.get();
        data.values().cloned().collect::<Vec<_>>()
    };

    // Convert the entire tree to HTML
    let tree_html = create_memo(move |_| {
        let items = theories();
        let mut html = String::new();

        for item in items {
            html.push_str(&theory_item(item));
        }

        html
    });

    view! {
        <div class="theory-tree" inner_html=move || tree_html.get()></div>
    }
}

/// Basic function to create HTML for a theory item
fn theory_item(item: MathItem) -> String {
    match item {
        MathItem::Theory {
            name,
            path,
            children,
        } => {
            let url = format!("/theory/{}", path);

            let children_html = if !children.is_empty() {
                let mut kids_html = String::new();
                for child in children {
                    kids_html.push_str(&theory_item(child));
                }

                format!(
                    r#"<div class="theory-children">
                        {kids_html}
                    </div>"#
                )
            } else {
                String::new()
            };

            format!(
                r#"<div class="theory-item-container">
                    <div class="theory-item">
                        <div class="theory-header">
                            <span class="expander">▶</span>
                            <a href="{url}">{name}</a>
                        </div>
                    </div>
                    {children_html}
                </div>"#
            )
        }
        MathItem::Definition { name, path } => {
            let url = format!("/definition/{}", path);

            format!(
                r#"<div class="theory-item-container">
                    <div class="theory-item">
                        <div class="theory-header">
                            <span class="expander"></span>
                            <a href="{url}">{name}</a>
                        </div>
                    </div>
                </div>"#
            )
        }
        MathItem::Theorem { name, path } => {
            let url = format!("/theorem/{}", path);

            format!(
                r#"<div class="theory-item-container">
                    <div class="theory-item">
                        <div class="theory-header">
                            <span class="expander"></span>
                            <a href="{url}">{name}</a>
                        </div>
                    </div>
                </div>"#
            )
        }
    }
}

/// Component to view a specific theory
#[component]
pub fn TheoryViewer<F>(id: F) -> impl IntoView
where
    F: Fn() -> String + 'static,
{
    let repo_data = use_context::<RepositoryData>().expect("Repository data should be provided");

    // Create a signal to store the found theory
    let (theory_data, set_theory_data) = create_signal(None::<MathItem>);
    let (is_loading, set_is_loading) = create_signal(true);

    // Effect to find the theory based on the id
    create_effect(move |_| {
        set_is_loading.set(true);
        let data = repo_data.0.get();
        let path = id();

        // Define a recursive function to find a theory by path
        fn find_item(item: &MathItem, target_path: &str) -> Option<MathItem> {
            match item {
                MathItem::Theory { path, .. } if path == target_path => Some(item.clone()),
                MathItem::Theory { children, .. } => {
                    for child in children {
                        if let Some(found) = find_item(child, target_path) {
                            return Some(found);
                        }
                    }
                    None
                }
                _ => None,
            }
        }

        let mut found_theory = None;
        for item in data.values() {
            if let Some(found) = find_item(item, &path) {
                found_theory = Some(found);
                break;
            }
        }

        set_theory_data.set(found_theory);
        set_is_loading.set(false);
    });

    let name_display = create_memo(move |_| {
        if let Some(MathItem::Theory { name, .. }) = theory_data.get() {
            name.clone()
        } else {
            "Theory not found".to_string()
        }
    });

    // Extract and format definitions as HTML
    let definitions_html = create_memo(move |_| {
        if let Some(MathItem::Theory { children, .. }) = theory_data.get() {
            let definitions = children
                .iter()
                .filter(|child| matches!(child, MathItem::Definition { .. }))
                .collect::<Vec<_>>();

            if definitions.is_empty() {
                "<p class=\"empty-message\">No definitions available.</p>".to_string()
            } else {
                let mut html = String::from("<div class=\"items-container\">");

                for def in definitions {
                    if let MathItem::Definition { name, path } = def {
                        html.push_str(&format!(
                            r#"<div class="definition-item">
                                <a href="/definition/{}">{}</a>
                            </div>"#,
                            path, name
                        ));
                    }
                }

                html.push_str("</div>");
                html
            }
        } else {
            "<p class=\"empty-message\">No definitions available.</p>".to_string()
        }
    });

    // Extract and format theorems as HTML
    let theorems_html = create_memo(move |_| {
        if let Some(MathItem::Theory { children, .. }) = theory_data.get() {
            let theorems = children
                .iter()
                .filter(|child| matches!(child, MathItem::Theorem { .. }))
                .collect::<Vec<_>>();

            if theorems.is_empty() {
                "<p class=\"empty-message\">No theorems available.</p>".to_string()
            } else {
                let mut html = String::from("<div class=\"items-container\">");

                for thm in theorems {
                    if let MathItem::Theorem { name, path } = thm {
                        html.push_str(&format!(
                            r#"<div class="theorem-item">
                                <a href="/theorem/{}">{}</a>
                            </div>"#,
                            path, name
                        ));
                    }
                }

                html.push_str("</div>");
                html
            }
        } else {
            "<p class=\"empty-message\">No theorems available.</p>".to_string()
        }
    });

    // Extract and format subtheories as HTML
    let subtheories_html = create_memo(move |_| {
        if let Some(MathItem::Theory { children, .. }) = theory_data.get() {
            let subtheories = children
                .iter()
                .filter(|child| matches!(child, MathItem::Theory { .. }))
                .collect::<Vec<_>>();

            if subtheories.is_empty() {
                "<p class=\"empty-message\">No subtheories available.</p>".to_string()
            } else {
                let mut html = String::from("<div class=\"items-container\">");

                for theory in subtheories {
                    if let MathItem::Theory { name, path, .. } = theory {
                        html.push_str(&format!(
                            r#"<div class="subtheory-item">
                                <a href="/theory/{}">{}</a>
                            </div>"#,
                            path, name
                        ));
                    }
                }

                html.push_str("</div>");
                html
            }
        } else {
            "<p class=\"empty-message\">No subtheories available.</p>".to_string()
        }
    });

    view! {
        <div class="theory-viewer">
            <div class="theory-container">
                <h2>{move || name_display.get()}</h2>
                <div class="theory-contents">
                    <h3>"Definitions"</h3>
                    <div class="definition-list" inner_html=move || definitions_html.get()></div>

                    <h3>"Theorems"</h3>
                    <div class="theorem-list" inner_html=move || theorems_html.get()></div>

                    <h3>"Subtheories"</h3>
                    <div class="subtheory-list" inner_html=move || subtheories_html.get()></div>
                </div>
            </div>
        </div>
    }
}
