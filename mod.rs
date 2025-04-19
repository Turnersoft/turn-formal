// Main library entry point for formalize_v2
// This file acts as the main entry point for the crate when used as a library

// Include the leptos module
pub mod foundational_theories;
pub mod leptos;
pub mod subjects;

#[path = "./frontend/src/pages/MathPage/components/turn-canvas/mod.rs"]
pub mod turn_math;
