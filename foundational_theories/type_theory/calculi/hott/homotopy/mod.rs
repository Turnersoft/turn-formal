//! Homotopy Operations
//! Common infrastructure for homotopy-based type theories (HoTT and Cubical)

use crate::formalize_v2::foundational_theories::type_theory::{
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod higher_paths;
pub mod coherence;
pub mod composition;
pub use higher_paths::{HigherPath, HigherPathContext};

use super::Term;

/// Homotopy type theory context
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HomotopyContext {
    /// Type equivalences
    pub equivalences: HashMap<(Term, Term), super::Equivalence>,
    /// Type h-levels
    pub h_levels: HashMap<Term, usize>,
    /// Path operations
    pub paths: HigherPathContext,
}

impl HomotopyContext {
    /// Create new homotopy context
    pub fn new() -> Self {
        HomotopyContext {
            equivalences: HashMap::new(),
            h_levels: HashMap::new(),
            paths: HigherPathContext::new(),
        }
    }

    /// Add type equivalence
    pub fn add_equivalence(&mut self, source: Term, target: Term, equiv: super::Equivalence) {
        self.equivalences.insert((source, target), equiv);
    }

    /// Get type equivalence
    pub fn get_equivalence(&self, source: &Term, target: &Term) -> Option<&super::Equivalence> {
        self.equivalences.get(&(source.clone(), target.clone()))
    }

    /// Set type h-level
    pub fn set_h_level(&mut self, ty: Term, level: usize) {
        self.h_levels.insert(ty, level);
    }

    /// Get type h-level
    pub fn get_h_level(&self, ty: &Term) -> Option<usize> {
        self.h_levels.get(ty).cloned()
    }

    /// Check if type has given h-level
    pub fn check_h_level(&self, ty: &Term, level: usize) -> Result<bool> {
        match self.get_h_level(ty) {
            Some(actual) => Ok(actual >= level),
            None => Err(super::Error::TypeError("Unknown h-level".to_string())),
        }
    }

    /// Check if type is a proposition (h-level 1)
    pub fn is_prop(&self, ty: &Term) -> Result<bool> {
        self.check_h_level(ty, 1)
    }

    /// Check if type is a set (h-level 2)
    pub fn is_set(&self, ty: &Term) -> Result<bool> {
        self.check_h_level(ty, 2)
    }

    /// Type check a path term
    pub fn check_path(&self, term: &Term, expected_ty: &Term) -> Result<()> {
        match term {
            Term::Path { source, target, ty, level } => {
                // Check that the path's type matches the expected type
                if ty != expected_ty {
                    return Err(super::Error::TypeError(format!(
                        "Path type mismatch: expected {:?}, got {:?}",
                        expected_ty, ty
                    )));
                }

                // Check source and target have the same type
                let source_ty = self.infer_type(source)?;
                let target_ty = self.infer_type(target)?;
                if source_ty != target_ty {
                    return Err(super::Error::TypeError("Path endpoints have different types".to_string()));
                }

                Ok(())
            }
            _ => Err(super::Error::TypeError("Expected path term".to_string())),
        }
    }

    /// Infer type of a path term
    pub fn infer_type(&self, term: &Term) -> Result<Term> {
        match term {
            Term::Path { source: _, target: _, ty, level: _ } => {
                // Path type is now explicitly stored
                Ok((*ty).clone())
            }
            Term::Var(_) => {
                // Variables need to be looked up in environment
                Err(super::Error::TypeError("Cannot infer type of free variable".to_string()))
            }
            _ => Err(super::Error::TypeError("Cannot infer type".to_string())),
        }
    }
}

/// Higher path operations
pub trait HigherPathOps {
    /// Compose paths vertically
    fn vcompose(&self, p: &Term, q: &Term) -> Result<Term>;
    /// Compose paths horizontally
    fn hcompose(&self, p: &Term, q: &Term) -> Result<Term>;
    /// Perform path whiskering
    fn whisker(&self, p: &Term, q: &Term) -> Result<Term>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_levels() {
        let mut ctx = HomotopyContext::new();

        // Create some types
        let bool_ty = Term::var("Bool");
        let nat_ty = Term::var("Nat");

        // Set h-levels
        ctx.set_h_level(bool_ty.clone(), 1); // Bool is a prop
        ctx.set_h_level(nat_ty.clone(), 2);  // Nat is a set

        // Check h-levels
        assert!(ctx.is_prop(&bool_ty).unwrap());
        assert!(ctx.is_set(&nat_ty).unwrap());
        assert!(!ctx.is_prop(&nat_ty).unwrap());
    }

    #[test]
    fn test_path_type_checking() {
        let ctx = HomotopyContext::new();

        // Create a path
        let x = Term::var("x");
        let y = Term::var("y");
        let ty = Term::var("A");
        let p = Term::path(x, y, ty.clone(), 0);

        // Type checking should succeed
        assert!(ctx.check_path(&p, &ty).is_ok());
    }
}
