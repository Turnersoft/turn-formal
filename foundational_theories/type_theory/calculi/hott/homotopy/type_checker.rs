//! Type checking for HoTT terms
//! Defines the type checking interface and common functionality

use crate::foundational_theories::type_theory::{
    core::{Error, Result},
    calculi::hott::Term,
};

/// Type checker trait
pub trait TypeChecker {
    /// Check if a term has the given type
    fn check(&self, term: &Term, expected_ty: &Term) -> Result<()>;

    /// Infer the type of a term
    fn infer(&self, term: &Term) -> Result<Term>;

    /// Check if two terms are convertible
    fn convertible(&self, t1: &Term, t2: &Term) -> Result<bool> {
        // Default implementation - structural equality
        Ok(t1 == t2)
    }

    /// Check if two types are equivalent
    fn equivalent(&self, ty1: &Term, ty2: &Term) -> Result<bool> {
        // Default implementation - structural equality
        Ok(ty1 == ty2)
    }
}

/// Common type checking functionality
pub trait TypeCheckerExt: TypeChecker {
    /// Check if a term has a type that is equivalent to the expected type
    fn check_equivalent(&self, term: &Term, expected_ty: &Term) -> Result<()> {
        let inferred_ty = self.infer(term)?;
        if !self.equivalent(&inferred_ty, expected_ty)? {
            return Err(Error::TypeError(format!(
                "Type mismatch: expected {:?}, got {:?}",
                expected_ty, inferred_ty
            )));
        }
        Ok(())
    }

    /// Check if a term is well-typed
    fn well_typed(&self, term: &Term) -> Result<bool> {
        match self.infer(term) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
