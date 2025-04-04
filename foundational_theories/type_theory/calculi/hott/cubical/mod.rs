//! Cubical Type Theory
//! Implements cubical type theory with:
//! - Face and cube types
//! - Kan operations
//! - Composition and coherence

use crate::formalize_v2::foundational_theories::type_theory::{calculi::Error, core::Result};

use super::Term;
use crate::formalize_v2::foundational_theories::type_theory::calculi::hott::homotopy::{
    coherence::CoherenceContext,
    composition::CompositionContext,
    higher_paths::HigherPathContext,
    HomotopyContext,
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cubical type theory calculus
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CubicalCalculus {
    /// Homotopy operations
    pub homotopy: HomotopyContext,
    /// Face constraints
    pub faces: HashMap<Term, Vec<Term>>,
}

impl CubicalCalculus {
    /// Create new cubical calculus
    pub fn new() -> Self {
        CubicalCalculus {
            homotopy: HomotopyContext::new(),
            faces: HashMap::new(),
        }
    }

    /// Add face constraint
    pub fn add_face(&mut self, cube: Term, faces: Vec<Term>) {
        self.faces.insert(cube, faces);
    }

    /// Get faces of a cube
    pub fn get_faces(&self, cube: &Term) -> Option<&Vec<Term>> {
        self.faces.get(cube)
    }

    /// Check if a term has the given type
    pub fn check_type(&self, term: &Term, expected_ty: &Term) -> Result<()> {
        match term {
            Term::Path {
                source,
                target,
                ty,
                level,
            } => {
                // Check path type matches expected type
                if ty != expected_ty {
                    return Err(Error::TypeError(format!(
                        "Path type mismatch: expected {:?}, got {:?}",
                        expected_ty, ty
                    )));
                }

                // Check source and target have the same type
                let source_ty = self.infer_type(source)?;
                let target_ty = self.infer_type(target)?;
                if source_ty != target_ty {
                    return Err(Error::TypeError("Path endpoints have different types".to_string()));
                }
                Ok(())
            }
            Term::HComp {
                ty: comp_ty,
                sides,
                cap,
            } => {
                // Check composition type matches
                if expected_ty != comp_ty {
                    return Err(Error::TypeError("Composition type mismatch".to_string()));
                }
                // Check sides and cap
                for side in sides {
                    self.check_type(side, expected_ty)?;
                }
                self.check_type(cap, expected_ty)?;
                Ok(())
            }
            Term::Face { ty: face_ty, face } => {
                // Check face type matches
                if expected_ty != face_ty {
                    return Err(Error::TypeError("Face type mismatch".to_string()));
                }
                self.check_type(face, expected_ty)
            }
            _ => Ok(()), // Other terms handled by main type checker
        }
    }

    /// Infer the type of a term
    pub fn infer_type(&self, term: &Term) -> Result<Term> {
        match term {
            Term::Path { source: _, target: _, ty, level: _ } => {
                // Path type is now explicitly stored
                Ok((*ty).clone())
            }
            Term::HComp { ty, sides: _, cap: _ } => {
                // Composition type is explicitly given
                Ok((*ty).clone())
            }
            Term::Face { ty, face: _ } => {
                // Face type is explicitly given
                Ok((*ty).clone())
            }
            Term::Var(_) => {
                Err(Error::TypeError("Cannot infer type of free variable".to_string()))
            }
            _ => Err(Error::TypeError("Cannot infer type".to_string())),
        }
    }

    /// Compose paths vertically
    pub fn vcompose(&self, p: &Term, q: &Term) -> Result<Term> {
        self.homotopy.paths.vcompose(p, q)
    }

    /// Compose paths horizontally
    pub fn hcompose(&self, p: &Term, q: &Term) -> Result<Term> {
        self.homotopy.paths.hcompose(p, q)
    }

    /// Perform path whiskering
    pub fn whisker(&self, p: &Term, q: &Term) -> Result<Term> {
        self.homotopy.paths.whisker(p, q)
    }

    /// Perform homogeneous composition
    pub fn hcomp(&self, ty: &Term, sides: Vec<Term>, cap: Term) -> Result<Term> {
        Ok(Term::HComp {
            ty: Box::new(ty.clone()),
            sides,
            cap: Box::new(cap),
        })
    }

    /// Perform heterogeneous composition
    pub fn comp(&self, ty: &Term, family: Term, sides: Vec<Term>, cap: Term) -> Result<Term> {
        Ok(Term::Comp {
            ty: Box::new(ty.clone()),
            family: Box::new(family),
            sides,
            cap: Box::new(cap),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubical_operations() {
        let mut calc = CubicalCalculus::new();

        // Create a path
        let x = Term::var("x");
        let y = Term::var("y");
        let ty = Term::var("A");
        let p = Term::path(x.clone(), y.clone(), ty.clone(), 0);

        // Create a face
        let face = Term::face(Term::var("A"), Term::var("b"));

        // Add face constraint
        calc.add_face(face.clone(), vec![x.clone(), y.clone()]);

        // Check face retrieval
        let faces = calc.get_faces(&face).unwrap();
        assert_eq!(faces.len(), 2);
    }

    #[test]
    fn test_type_checking() {
        let calc = CubicalCalculus::new();

        // Create a path and its type
        let x = Term::var("x");
        let y = Term::var("y");
        let ty = Term::var("A");
        let p = Term::path(x, y, ty.clone(), 0);

        // Type checking should succeed
        assert!(calc.check_type(&p, &ty).is_ok());
    }
}
