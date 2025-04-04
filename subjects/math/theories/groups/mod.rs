//! Group theory implementation
//!
//! This module provides implementations for group theory concepts and operations.
//! It includes type-safe representations of groups, group elements, and operations,
//! along with macros for convenient manipulation of group expressions.

pub mod checker;
pub mod definitions;
pub mod helpers;
// pub mod macros;

#[cfg(test)]
pub mod tests;
pub mod theorems;

pub use checker::*;
pub use definitions::*;
pub use helpers::*;
// pub use macros::*;
pub use theorems::*;
