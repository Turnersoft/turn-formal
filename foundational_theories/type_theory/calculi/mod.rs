//! Different logical calculi implementations
//! Each calculus is self-contained and independent:
//! - Simply Typed Lambda Calculus (λ→)
//! - System F (λ2)
//! - System Omega (λω)
//! - Dependent Types (λP)
//! - Calculus of Constructions (λC)
//! - Homotopy Type Theory

use serde::{Deserialize, Serialize};

pub mod simply_typed;    // λ→: Simply typed lambda calculus
pub mod system_f;       // λ2: Second-order lambda calculus
pub mod system_omega;   // λω: Higher-order lambda calculus
pub mod dependent;      // λP: Dependent types
pub mod constructions;  // λC: Calculus of Constructions
pub mod hott;          // Homotopy Type Theory (includes Cubical and Homotopy variants)

/// Result type for calculi operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for calculi operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Type error with message
    TypeError(String),
    /// Reduction error with message
    ReductionError(String),
    /// Variable not found
    UnboundVariable(String),
}

/// Common functionality for type checking
pub trait TypeChecker {
    /// The term type for this type checker
    type Term;
    /// The type representation for this type checker
    type Type;

    /// Type check a term in the current environment
    fn type_check(&self, term: &Self::Term) -> Result<Self::Type>;

    /// Check if two types are equal
    fn types_equal(&self, ty1: &Self::Type, ty2: &Self::Type) -> bool {
        ty1 == ty2
    }
}

/// Common functionality for term reduction
pub trait Reducer {
    /// The term type for this reducer
    type Term;

    /// Reduce a term to normal form
    fn reduce(&self, term: &Self::Term) -> Result<Self::Term>;

    /// Check if a term is in normal form
    fn is_normal_form(&self, term: &Self::Term) -> bool;
}
