use crate::foundational_theories::type_theory::{
    core::{Result, Term},
    types::TypeConstructor,
};

pub mod hott;
pub mod system_f;
pub mod system_omega;
pub mod dependent;
pub mod simply_typed;

/// Trait for calculus-specific type operations
pub trait CalculusType: TypeConstructor {
    /// Name of the calculus
    fn calculus_name() -> &'static str;
    
    /// Check if this type is valid in the calculus
    fn is_valid_in_calculus(&self, term: &Term) -> Result<()>;
    
    /// Get the kind/sort of this type in the calculus
    fn get_kind(&self) -> Result<Term>;
}
