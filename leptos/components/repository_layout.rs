use leptos::prelude::*;

/// Main layout component for the repository viewer
#[component]
pub fn RepositoryLayout() -> impl IntoView {
    let current_path = create_rw_signal(String::new());

    view! {
        <div class="repository-layout">
            <div class="repository-sidebar">
                <div class="sidebar-header">
                    <h2>"Repository Explorer"</h2>
                </div>
                <div class="sidebar-content">
                    <p>"Sidebar content will go here"</p>
                </div>
            </div>
            <div class="repository-content">
                <div class="content-viewer">
                    <p>"Content viewer will display: " {current_path.get_untracked()}</p>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn RepositoryLayoutStyles() -> impl IntoView {
    view! {
        <style>
            ".repository-layout {
                width: 100%;
                height: 100vh;
                overflow: hidden;
                display: flex;
            }
            
            .repository-sidebar {
                width: 280px;
                height: 100%;
                background-color: #f5f7fa;
                border-right: 1px solid #e0e0e0;
                overflow-y: auto;
            }
            
            .sidebar-header {
                padding: 1rem;
                border-bottom: 1px solid #e0e0e0;
            }
            
            .sidebar-header h2 {
                margin: 0;
                font-size: 1.2rem;
                color: #2c3e50;
            }
            
            .sidebar-content {
                padding: 1rem;
            }
            
            .repository-content {
                flex: 1;
                overflow: auto;
                padding: 1rem;
            }"
        </style>
    }
}
