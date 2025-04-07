//! Calculus of Inductive Constructions (CIC) Implementation
//!
//! This module provides a complete implementation of CIC, which is a dependent type theory
//! that combines:
//!
//! - Dependent types (Pi types)
//! - Universe hierarchy (Type₀, Type₁, ...)
//! - Inductive types
//! - Pattern matching
//!
//! The implementation follows the standard presentation of CIC as used in proof assistants
//! like Coq and Lean, with the following key features:
//!
//! - Type safety through static type checking
//! - Universe consistency checking
//! - Support for inductive definitions
//! - Pattern matching with dependent elimination
//!
//! # Example
//! ```rust
//! use cic::{Term, Type, Context};
//!
//! // Define identity function: λx:Type₀. x
//! let id = Term::Lambda(
//!     "x".to_string(),
//!     Rc::new(Type::Type(Level::new(0))),
//!     Rc::new(Term::Var("x".to_string()))
//! );
//! ```

pub mod context;
pub mod reduction;
pub mod term;
pub mod tests;
pub mod tests_old;
pub mod type_;
pub mod typing;
pub mod universe;

pub use context::Context;
pub use reduction::Reduction;
pub use term::Term;
pub use type_::Type;
pub use typing::{TypeChecker, TypeError};
pub use universe::{Level, Universe};
