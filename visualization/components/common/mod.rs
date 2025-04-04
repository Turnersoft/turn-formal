//! Common components for the formalize_v2 visualization system
//!
//! Provides basic UI components that are used throughout the application

#[cfg(feature = "theorem_visualizer")]
use leptos::*;

/// Re-export the log macro from leptos for convenience
#[cfg(feature = "theorem_visualizer")]
pub use leptos::log;

/// Components for displaying cards
pub mod card;

/// Components for displaying data tables
pub mod table;

/// Components for pagination
pub mod pagination;

/// Components for alerts and notifications
pub mod alert;

/// Components for modals and dialogs
pub mod modal;

/// Button and action components
pub mod button;

/// Form input components
pub mod input;

/// Typography components
pub mod typography;

/// Layout utility components
pub mod layout;
