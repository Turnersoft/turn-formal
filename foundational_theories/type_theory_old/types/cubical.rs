//! Cubical Type Theory
//! Implements cubical types with composition operations

use crate::foundational_theories::type_theory::{
    core::{Error, Result, Term},
    types::{
        path::{PathGroupoid, PathOperations},
        TypeConstructor,
    },
};
use std::collections::{HashMap, HashSet};

use super::path::PathContext;

/// Dimension variable
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct DimVar(String);

impl DimVar {
    /// Create new dimension variable
    pub fn new(name: impl Into<String>) -> Self {
        DimVar(name.into())
    }
}

/// Face lattice element
#[derive(Debug, Clone)]
pub enum Face {
    /// Dimension variable
    Var(DimVar),
    /// Zero endpoint
    Zero,
    /// One endpoint
    One,
    /// Meet (intersection)
    Meet(Box<Face>, Box<Face>),
    /// Join (union)
    Join(Box<Face>, Box<Face>),
}

impl Face {
    /// Check if face is valid
    pub fn is_valid(&self) -> bool {
        match self {
            Face::Var(_) | Face::Zero | Face::One => true,
            Face::Meet(f1, f2) | Face::Join(f1, f2) => f1.is_valid() && f2.is_valid(),
        }
    }
}

/// Cubical type
#[derive(Debug, Clone)]
pub struct CubicalType {
    /// Base type
    base: Term,
    /// Dimension context
    dimensions: HashSet<DimVar>,
}

impl CubicalType {
    /// Create new cubical type
    pub fn new(base: Term) -> Self {
        CubicalType {
            base,
            dimensions: HashSet::new(),
        }
    }

    /// Add dimension variable
    pub fn add_dimension(&mut self, dim: DimVar) {
        self.dimensions.insert(dim);
    }

    /// Get type with given face
    pub fn at_face(&self, face: Face) -> Result<Term> {
        if !face.is_valid() {
            return Err(Error::TypeError("Invalid face".to_string()));
        }

        Ok(Term::Face {
            ty: Box::new(self.base.clone()),
            face: Box::new(Term::from(face)),
        })
    }
}

/// Kan composition operation
#[derive(Debug)]
pub struct KanComposition {
    /// Source face
    source: Face,
    /// Target face
    target: Face,
    /// Filling operation
    filling: Term,
}

impl KanComposition {
    /// Create new Kan composition
    pub fn new(source: Face, target: Face, filling: Term) -> Self {
        KanComposition {
            source,
            target,
            filling,
        }
    }

    /// Apply composition
    pub fn compose(&self) -> Term {
        Term::Compose {
            left: Box::new(Term::from(self.source.clone())),
            right: Box::new(Term::from(self.target.clone())),
        }
    }
}

/// Cubical path operations
pub trait CubicalPath: PathGroupoid {
    /// Path with face constraint
    fn face_path(&self, p: Term, face: Face) -> Result<Term>;

    /// Kan composition
    fn kan_compose(&self, comp: KanComposition) -> Term;

    /// Transport along path
    fn transport(&self, ty: Term, p: Term) -> Result<Term>;
}

impl CubicalPath for PathContext {
    fn face_path(&self, p: Term, face: Face) -> Result<Term> {
        Ok(Term::Face {
            ty: Box::new(p),
            face: Box::new(Term::from(face)),
        })
    }

    fn kan_compose(&self, comp: KanComposition) -> Term {
        comp.compose()
    }

    fn transport(&self, ty: Term, p: Term) -> Result<Term> {
        Ok(Term::Transport {
            ty: Box::new(ty),
            path: Box::new(p),
        })
    }
}

/// Examples of cubical types
pub mod examples {
    use super::*;

    /// Create interval type
    pub fn interval() -> CubicalType {
        let mut interval = CubicalType::new(Term::Unit);
        interval.add_dimension(DimVar::new("i"));
        interval
    }

    /// Create square type
    pub fn square() -> CubicalType {
        let mut square = CubicalType::new(Term::Unit);
        square.add_dimension(DimVar::new("i"));
        square.add_dimension(DimVar::new("j"));
        square
    }

    /// Create cube type
    pub fn cube() -> CubicalType {
        let mut cube = CubicalType::new(Term::Unit);
        cube.add_dimension(DimVar::new("i"));
        cube.add_dimension(DimVar::new("j"));
        cube.add_dimension(DimVar::new("k"));
        cube
    }
}

/// Conversion from Face to Term
impl From<Face> for Term {
    fn from(face: Face) -> Self {
        match face {
            Face::Var(DimVar(name)) => Term::Var(name),
            Face::Zero => Term::Zero,
            Face::One => Term::One,
            Face::Meet(f1, f2) => Term::Meet {
                left: Box::new(Term::from(*f1)),
                right: Box::new(Term::from(*f2)),
            },
            Face::Join(f1, f2) => Term::Join {
                left: Box::new(Term::from(*f1)),
                right: Box::new(Term::from(*f2)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::examples::*;
    use super::*;

    #[test]
    fn test_interval() {
        let interval = interval();
        assert_eq!(interval.dimensions.len(), 1);

        let i = DimVar::new("i");
        let face = Face::Var(i);
        assert!(interval.at_face(face).is_ok());
    }

    #[test]
    fn test_square() {
        let square = square();
        assert_eq!(square.dimensions.len(), 2);

        let i = DimVar::new("i");
        let j = DimVar::new("j");
        let face = Face::Meet(Box::new(Face::Var(i)), Box::new(Face::Var(j)));
        assert!(square.at_face(face).is_ok());
    }

    #[test]
    fn test_kan_composition() {
        let ops = PathOperations::new();

        let source = Face::Zero;
        let target = Face::One;
        let filling = Term::Lambda {
            var: "x".to_string(),
            body: Box::new(Term::Var("x".to_string())),
        };

        let comp = KanComposition::new(source, target, filling);
        let result = ops.kan_compose(comp);

        assert!(matches!(result, Term::Compose { .. }));
    }
}
