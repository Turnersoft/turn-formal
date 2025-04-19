use super::super::super::subjects::math::theories::groups::leptos::{GroupStyles, GroupTheorems};
use leptos::prelude::*;

#[component]
pub fn MathTheoremPage() -> impl IntoView {
    let is_feature_enabled = cfg!(feature = "visualization");

    view! {
        <div class="theorem-library">
            <div class="theorem-library-header">
                <div class="theorem-library-title">
                    "Mathlib.GroupTheory"
                </div>
                <div class="theorem-library-search">
                    <input type="text" placeholder="Search" />
                </div>
            </div>

            <div class="theorem-library-imports">
                <h2>"Imports"</h2>
                <ul>
                    <li>"Mathlib.Algebra.Group.Basic"</li>
                    <li>"Mathlib.GroupTheory.Torsion"</li>
                    <li>"Mathlib.Data.Nat.Basic"</li>
                </ul>
            </div>

            <div class="debug-info" style="background-color: #f8f8f8; padding: 10px; margin: 10px 0; border: 1px solid #ddd; display: none;">
                <h3>"Debug Information"</h3>
                <p>"Visualization feature enabled: " {if is_feature_enabled { "Yes" } else { "No" }}</p>
                <p>"Attempting to load: GroupTheorems component"</p>
            </div>

            // Main content area
            <div class="theorem-sections">
                <section id="group-theory" class="theorem-section">
                    <h1>"Group Theory Theorems"</h1>
                    <p class="section-description">
                        "This file defines fundamental theorems of group theory including uniqueness of identity,
                        uniqueness of inverses, and properties of group operations."
                    </p>

                    <div class="theorem-doc">
                        <div class="theorem-header">
                            <div class="theorem-name">"Group.Inverse.Uniqueness"</div>
                            <a href="#" class="theorem-source-link">"source"</a>
                        </div>
                        <div class="theorem-declaration">
                            <pre>"theorem Group.Inverse.Uniqueness {G : Type u} [Group G] {g : G} {h₁ h₂ : G}
    (h₁_eq : g * h₁ = 1) (h₂_eq : g * h₂ = 1) : h₁ = h₂"</pre>
                        </div>
                        <div class="theorem-description">
                            "If an element " <span class="code">"g"</span> " has two right inverses " <span class="code">"h₁"</span> " and " <span class="code">"h₂"</span> ", then " <span class="code">"h₁ = h₂"</span>"."
                        </div>
                    </div>

                    <div class="theorem-doc">
                        <div class="theorem-header">
                            <div class="theorem-name">"Group.Identity.Uniqueness"</div>
                            <a href="#" class="theorem-source-link">"source"</a>
                        </div>
                        <div class="theorem-declaration">
                            <pre>"theorem Group.Identity.Uniqueness {G : Type u} [Group G] {e₁ e₂ : G}
    (h₁ : ∀ g, e₁ * g = g) (h₂ : ∀ g, g * e₂ = g) : e₁ = e₂"</pre>
                        </div>
                        <div class="theorem-description">
                            "If " <span class="code">"e₁"</span> " is a left identity and " <span class="code">"e₂"</span> " is a right identity, then " <span class="code">"e₁ = e₂"</span>"."
                        </div>
                    </div>

                    <div class="theorem-doc">
                        <div class="theorem-header">
                            <div class="theorem-name">"Group.Inverse.Product"</div>
                            <a href="#" class="theorem-source-link">"source"</a>
                        </div>
                        <div class="theorem-declaration">
                            <pre>"theorem Group.Inverse.Product {G : Type u} [Group G] (a b : G) :
    (a * b)⁻¹ = b⁻¹ * a⁻¹"</pre>
                        </div>
                        <div class="theorem-description">
                            "The inverse of a product is the product of the inverses in reverse order."
                        </div>
                    </div>

                    <div class="theorem-doc">
                        <div class="theorem-header">
                            <div class="theorem-name">"Group.Abelian.Squared.Criterion"</div>
                            <a href="#" class="theorem-source-link">"source"</a>
                        </div>
                        <div class="theorem-declaration">
                            <pre>"theorem Group.Abelian.Squared.Criterion {G : Type u} [Group G] :
    (∀ g : G, g * g = g * g) → IsAbelian G"</pre>
                        </div>
                        <div class="theorem-description">
                            "A group is abelian if and only if $(ab)^2 = a^2b^2$ for all elements $a, b$."
                        </div>
                        <div class="equation">
                            {r#"(ab)^2 = a^2b^2 \iff ab = ba"#}
                        </div>
                    </div>

                    <div class="theorem-doc">
                        <div class="theorem-header">
                            <div class="theorem-name">"Group.Lagrange.Theorem"</div>
                            <a href="#" class="theorem-source-link">"source"</a>
                        </div>
                        <div class="theorem-declaration">
                            <pre>"theorem Group.Lagrange {G : Type u} [Group G] [Fintype G] (H : Subgroup G) :
    card H ∣ card G"</pre>
                        </div>
                        <div class="theorem-description">
                            "The order of a subgroup divides the order of the group (Lagrange's theorem)."
                        </div>
                    </div>

                    // Interactive component with modern UI
                    <div class="interactive-theorems">
                        <h2>"Interactive Theorem Viewer"</h2>
                        <p>"Expand the theorems below to see the detailed proofs and statements."</p>

                        // Include both the component and its styles
                        <GroupTheorems />
                        <GroupStyles />
                    </div>
                </section>

                <section id="related-topics" class="related-topics">
                    <h2>"Related Topics"</h2>
                    <div class="topic-links">
                        <a href="#" class="topic-link">
                            <div class="topic-name">"Ring Theory"</div>
                            <div class="topic-description">"Theorems about rings, ideals, and ring homomorphisms"</div>
                        </a>
                        <a href="#" class="topic-link">
                            <div class="topic-name">"Field Theory"</div>
                            <div class="topic-description">"Algebraic and transcendental extensions, Galois theory"</div>
                        </a>
                        <a href="#" class="topic-link">
                            <div class="topic-name">"Linear Algebra"</div>
                            <div class="topic-description">"Vector spaces, linear transformations, and determinants"</div>
                        </a>
                        <a href="#" class="topic-link">
                            <div class="topic-name">"Topology"</div>
                            <div class="topic-description">"Continuous functions, compactness, and connectedness"</div>
                        </a>
                        <a href="#" class="topic-link">
                            <div class="topic-name">"Analysis"</div>
                            <div class="topic-description">"Calculus, sequences, series, and differential equations"</div>
                        </a>
                    </div>
                </section>
            </div>
        </div>
    }
}
