//! Central visualization module for formalize_v2
//!
//! This module provides visualization tools for all components of the formalize_v2 system,
//! including foundational theories, documentation, and various subject domains.

pub mod app;
pub mod components;
pub mod core;
pub mod loaders;
pub mod themes;

#[cfg(feature = "theorem_visualizer")]
pub mod wasm;

/// Re-exports for convenient access
pub mod prelude {
    #[cfg(feature = "theorem_visualizer")]
    pub use crate::formalize_v2::visualization::app::VisualizeApp;
    #[cfg(feature = "theorem_visualizer")]
    pub use crate::formalize_v2::visualization::components::*;
}
