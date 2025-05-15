// Module: src/formalize_v2/subjects/math/theorem/mod.rs
// Acts as a central hub for the theorem system in the project

// Note: counter_example is intentionally commented out to prevent compile errors
// Uncomment to see various examples of compile-time errors in action
// pub mod counter_example;

pub mod complexity;
pub mod expressions;
pub mod interpretation;
pub mod proof;
pub mod registry;
pub mod theorem;
#[macro_use]
pub mod extract;
pub mod relations;
pub mod render;
pub mod test;
