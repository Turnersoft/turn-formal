//! Type constructors for type theory
//! Provides various type formers from simple to complex

pub mod base; // Base type operations
pub mod coinductive; // Coinductive types
pub mod cubical;
pub mod hits; // Higher inductive types
pub mod identity; // Identity types
pub mod inductive; // Inductive families
pub mod path; // Path algebra operations
pub mod product; // Dependent products (Π)
pub mod quotient; // Quotient types
pub mod sum; // Dependent sums (Σ)
pub mod wtypes; // Well-founded trees // Cubical type theory

use super::core::Term;

/// Common trait for all type constructors
pub trait TypeConstructor {
    /// Check if a term has this type
    fn check_term(&self, term: &Term) -> Result<()>;

    /// Get the type's universe level
    fn universe_level(&self) -> usize;
}

/// Common trait for type eliminators
pub trait TypeEliminator {
    /// Eliminate a term of this type
    fn eliminate(&self, term: &Term) -> Result<Term>;
}
