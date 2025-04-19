use super::components::navigation::Navigation;
use super::components::repository_layout::{RepositoryLayout, RepositoryLayoutStyles};
use super::components::repository_reader::RepositoryProvider;
use super::pages::about_page::AboutPage;
use super::pages::definition_page::DefinitionPage;
use super::pages::home_page::HomePage;
use super::pages::math_theorem_page::MathTheoremPage;
use super::pages::theorem_page::TheoremPage;
use super::pages::theory_detail_page::TheoryDetailPage;
use super::pages::theory_page::TheoryPage;
use leptos::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::{
    components::{A, Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Stylesheet id="main-stylesheet" href="/styles.css"/>
        <Script type_="text/javascript" src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/3.2.2/es5/tex-mml-chtml.min.js"/>
        <RepositoryProvider>
            <div class="app-container">
                <Router>
                    <Navigation />
                    <main>
                        <Routes fallback=|| view! { <NotFound/> }>
                            <Route path=path!("/") view=HomePage/>
                            <Route path=path!("/explorer") view=RepositoryLayout/>
                            <Route path=path!("/explorer/*any") view=RepositoryLayout/>
                            <Route path=path!("/theories") view=TheoryPage/>
                            <Route path=path!("/theory/:id") view=TheoryDetailPage/>
                            <Route path=path!("/theorems") view=MathTheoremPage/>
                            <Route path=path!("/theorem/:id") view=TheoremPage/>
                            <Route path=path!("/definition/:id") view=DefinitionPage/>
                            <Route path=path!("/about") view=AboutPage/>
                        </Routes>
                    </main>
                </Router>
            </div>

            <RepositoryLayoutStyles />
        </RepositoryProvider>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="not-found">
            <h1>"404 - Page Not Found"</h1>
            <p>"The page you are looking for does not exist."</p>
            <A href="/">"Return to Home"</A>
        </div>
    }
}
