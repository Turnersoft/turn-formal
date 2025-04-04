// Module: src/formalize_v2/subjects/math/visualization/mod.rs
// Visualization tools for the mathematical structures and theorems

pub mod app;
pub mod components;
pub mod loader;
pub mod models;

#[cfg(feature = "theorem_visualizer")]
pub mod wasm;

#[cfg(test)]
mod tests;
