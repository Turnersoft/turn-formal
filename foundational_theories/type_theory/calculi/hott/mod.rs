//! Homotopy Type Theory
//! Implementation of Homotopy Type Theory (HoTT)

use crate::foundational_theories::type_theory::{
    core::{
        equivalence::{Equivalence, Transport, Univalence},
        Error, Result,
    },
    types::hits::Hit,
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod cubical;
pub mod homotopy;
pub mod term;
pub mod goals;

use homotopy::{
    coherence::CoherenceContext, composition::CompositionContext, higher_paths::HigherPathContext,
    type_checker::TypeChecker, HomotopyContext,
};
pub use term::Term;

/// HoTT calculus
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HottCalculus {
    /// Homotopy operations
    homotopy: HomotopyContext,
    /// Higher inductive types
    hits: HashMap<String, Hit>,
    /// Univalence implementation
    univalence: Univalence,
}

impl HottCalculus {
    /// Create new HoTT calculus
    pub fn new() -> Self {
        HottCalculus {
            homotopy: HomotopyContext::new(),
            hits: HashMap::new(),
            univalence: Univalence::new(),
        }
    }

    /// Add higher inductive type
    pub fn add_hit(&mut self, hit: Hit) {
        self.hits.insert(hit.name().to_string(), hit);
    }

    /// Add type equivalence
    pub fn add_equivalence(
        &mut self,
        source: Term,
        target: Term,
        equiv: Equivalence,
    ) -> Result<()> {
        self.univalence.add_equivalence(source, target, equiv)
    }

    /// Transport along path
    pub fn transport(&self, source: Term, target: Term, path: Term, term: Term) -> Result<Term> {
        let transport = Transport::new(source, target, path);
        transport.apply(&term)
    }

    /// Check if type is a proposition (h-level 1)
    pub fn is_prop(&self, ty: &Term) -> Result<bool> {
        self.homotopy.is_prop(ty)
    }

    /// Check if type is a set (h-level 2)
    pub fn is_set(&self, ty: &Term) -> Result<bool> {
        self.homotopy.is_set(ty)
    }

    /// Compute h-level of type
    pub fn h_level(&self, ty: &Term) -> Result<usize> {
        match self.homotopy.get_h_level(ty) {
            Some(level) => Ok(level),
            None => {
                // Infer h-level based on type structure
                if self.is_prop(ty)? {
                    Ok(1)
                } else if self.is_set(ty)? {
                    Ok(2)
                } else {
                    Ok(0) // Default to h-level 0 (general type)
                }
            }
        }
    }

    /// Type check a term
    pub fn type_check(&self, term: &Term, expected_ty: Option<&Term>) -> Result<()> {
        let inferred_ty = self.infer_type(term)?;
        if let Some(expected) = expected_ty {
            if &inferred_ty != expected {
                return Err(Error::TypeError(format!(
                    "Type mismatch: expected {:?}, got {:?}",
                    expected, inferred_ty
                )));
            }
        }
        Ok(())
    }

    /// Infer type of a term
    pub fn infer_type(&self, term: &Term) -> Result<Term> {
        match term {
            Term::Path { .. } => self.homotopy.infer_type(term),
            Term::HIT { name, .. } => {
                if let Some(hit) = self.hits.get(name) {
                    Ok(hit.get_type())
                } else {
                    Err(Error::TypeError(format!("Unknown HIT: {}", name)))
                }
            }
            Term::Var(_) => Err(Error::TypeError(
                "Cannot infer type of free variable".to_string(),
            )),
            _ => Ok(Term::Type),
        }
    }

    /// Reduce a term to normal form
    pub fn reduce(&self, term: &Term) -> Result<Term> {
        match term {
            Term::Path { .. } => {
                // Path reduction rules
                Ok(term.clone())
            }
            Term::HIT { name, args } => {
                if let Some(hit) = self.hits.get(name) {
                    // Reduce HIT constructor arguments
                    let reduced_args = args
                        .iter()
                        .map(|arg| self.reduce(arg))
                        .collect::<Result<Vec<_>>>()?;
                    Ok(Term::HIT {
                        name: name.clone(),
                        args: reduced_args,
                    })
                } else {
                    Err(Error::TypeError(format!("Unknown HIT: {}", name)))
                }
            }
            _ => Ok(term.clone()),
        }
    }
}

impl TypeChecker for HottCalculus {
    fn check(&self, term: &Term, ty: &Term) -> Result<()> {
        self.type_check(term, Some(ty))
    }

    fn infer(&self, term: &Term) -> Result<Term> {
        self.infer_type(term)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_type_checking() {
        let mut hott = HottCalculus::new();

        // Create a path
        let x = Term::var("x");
        let y = Term::var("y");
        let ty = Term::var("A");
        let p = Term::path(x.clone(), y.clone(), ty.clone(), 0);

        // Type checking should succeed
        assert!(hott.type_check(&p, Some(&ty)).is_ok());
    }

    #[test]
    fn test_hit_type_checking() {
        let mut hott = HottCalculus::new();

        // Create a HIT
        let circle = Hit::new("S1".to_string(), vec![], vec![], vec![]);
        hott.add_hit(circle);

        // Create a point on the circle
        let base = Term::HIT {
            name: "S1".to_string(),
            args: vec![],
        };

        // Type checking should succeed
        assert!(hott.type_check(&base, None).is_ok());
    }

    #[test]
    fn test_h_levels() {
        let mut hott = HottCalculus::new();

        // Create types
        let prop_ty = Term::var("Prop");
        let set_ty = Term::var("Set");

        // Set h-levels
        hott.homotopy.set_h_level(prop_ty.clone(), 1);
        hott.homotopy.set_h_level(set_ty.clone(), 2);

        // Check h-levels
        assert!(hott.is_prop(&prop_ty).unwrap());
        assert!(hott.is_set(&set_ty).unwrap());
        assert!(!hott.is_prop(&set_ty).unwrap());
    }
}
