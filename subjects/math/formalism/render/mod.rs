// Located in: subjects/math/formalism/render/mod.rs
// This file should only declare its submodules.

pub mod expressions;
pub mod extract;
pub mod interpretation;
pub mod math_object;
pub mod relations;
pub mod theorem;

// All other content, including use statements for serde/HashMap/TS (unless a submodule *needs* them re-exported from here),
// helper function definitions (p_text, link_to_definition),
// and local enum definitions (FunctionDisplayStyle, OperatorType) are REMOVED.

// Diagnostic: Explicitly bring the module containing ToTurnMath impls into scope here.
// This shouldn't typically be necessary for trait impls to be found if types/traits are in scope.
pub use crate::subjects::math::theories::groups::render as _; // Underscore to avoid unused warning if not directly used
