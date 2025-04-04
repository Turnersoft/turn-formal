//! UI components for the formalize_v2 visualization system
//!
//! Provides reusable components for visualizing various elements of the formalize_v2 system,
//! including foundational theories, documentation, and domain-specific components.

// Common components used across the application
pub mod common;
pub mod footer;
pub mod nav;

// Domain-specific visualization components
pub mod docs;
pub mod foundation;
pub mod home;
pub mod logic;
pub mod math;

// Component utility modules
pub mod code;
pub mod search;
pub mod theme_switch;
pub mod visualizers;
