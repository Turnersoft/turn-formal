//! Group theory implementation
//!
//! This module provides implementations for group theory concepts and operations.
//! It includes type-safe representations of groups, group elements, and operations,
//! along with macros for convenient manipulation of group expressions.

pub mod abstraction_level;
pub mod axioms;
pub mod case_generator;
pub mod checker;
pub mod complexity;
pub mod definitions;
pub mod detag;
pub mod render;
pub mod replace;

pub mod collect_identifier;
pub mod search;
pub mod tests;
pub mod theorems;
