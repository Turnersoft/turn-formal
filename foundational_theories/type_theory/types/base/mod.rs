//! Base type operations
//! Provides fundamental type operations

use super::TypeConstructor;
use crate::foundational_theories::type_theory::core::{Result, Term};

/// Base type constructor
pub struct BaseType;

impl TypeConstructor for BaseType {
    fn check_term(&self, _term: &Term) -> Result<()> {
        Ok(())
    }

    fn universe_level(&self) -> usize {
        0
    }
}
