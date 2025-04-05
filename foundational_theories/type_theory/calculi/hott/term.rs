//! HoTT Term Type
//! Defines the term type for Homotopy Type Theory

use serde::{Deserialize, Serialize};
use crate::foundational_theories::type_theory::calculi::{Error, Result};

/// Term in Homotopy Type Theory
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Term {
    /// Variables (x, y, z)
    Var(String),

    /// Lambda abstraction (λx:T.t)
    Lambda {
        var: String,
        ty: Box<Type>,
        body: Box<Term>,
    },

    /// Application (t u)
    Apply {
        func: Box<Term>,
        arg: Box<Term>,
    },

    /// Type annotations (t : T)
    Annotated {
        term: Box<Term>,
        ty: Box<Type>,
    },

    /// Unit type value
    Unit,

    /// Path terms (p : x = y)
    Path {
        source: Box<Term>,
        target: Box<Term>,
        ty: Box<Term>,
        level: usize,
    },

    /// Path composition (p • q)
    Compose {
        left: Box<Term>,
        right: Box<Term>,
    },

    /// Transport along path (transport(P, p, x))
    Transport {
        ty: Box<Term>,
        path: Box<Term>,
        term: Box<Term>,
    },

    /// Higher inductive type constructor
    HIT {
        name: String,
        args: Vec<Term>,
    },

    /// Homogeneous composition (hcomp)
    HComp {
        ty: Box<Term>,
        sides: Vec<Term>,
        cap: Box<Term>,
    },

    /// Heterogeneous composition (comp)
    Comp {
        ty: Box<Term>,
        family: Box<Term>,
        sides: Vec<Term>,
        cap: Box<Term>,
    },

    /// Face term (face)
    Face {
        ty: Box<Term>,
        face: Box<Term>,
    },

    /// Universe (Type_n)
    Universe(usize),
}

/// Type in Homotopy Type Theory
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
    /// Base type
    Base(String),

    /// Function type (A → B)
    Function {
        domain: Box<Type>,
        codomain: Box<Type>,
    },

    /// Identity type (x = y)
    Identity {
        left: Box<Term>,
        right: Box<Term>,
    },

    /// Universe type (Type_n)
    Universe(usize),
}

impl Term {
    /// Create a variable term
    pub fn var(name: &str) -> Self {
        Term::Var(name.to_string())
    }

    /// Create a lambda abstraction
    pub fn lambda(var: &str, ty: Type, body: Term) -> Self {
        Term::Lambda {
            var: var.to_string(),
            ty: Box::new(ty),
            body: Box::new(body),
        }
    }

    /// Create an application
    pub fn apply(func: Term, arg: Term) -> Self {
        Term::Apply {
            func: Box::new(func),
            arg: Box::new(arg),
        }
    }

    /// Create a type annotation
    pub fn annotated(term: Term, ty: Type) -> Self {
        Term::Annotated {
            term: Box::new(term),
            ty: Box::new(ty),
        }
    }

    /// Create a path term
    pub fn path(source: Term, target: Term, ty: Term, level: usize) -> Self {
        Term::Path {
            source: Box::new(source),
            target: Box::new(target),
            ty: Box::new(ty),
            level,
        }
    }

    /// Create a composition term
    pub fn compose(left: Term, right: Term) -> Self {
        Term::Compose {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create a transport term
    pub fn transport(ty: Term, path: Term, term: Term) -> Self {
        Term::Transport {
            ty: Box::new(ty),
            path: Box::new(path),
            term: Box::new(term),
        }
    }

    /// Create a higher inductive type constructor
    pub fn hit(name: &str, args: Vec<Term>) -> Self {
        Term::HIT {
            name: name.to_string(),
            args,
        }
    }

    /// Create a homogeneous composition term
    pub fn hcomp(ty: Term, sides: Vec<Term>, cap: Term) -> Self {
        Term::HComp {
            ty: Box::new(ty),
            sides,
            cap: Box::new(cap),
        }
    }

    /// Create a heterogeneous composition term
    pub fn comp(ty: Term, family: Term, sides: Vec<Term>, cap: Term) -> Self {
        Term::Comp {
            ty: Box::new(ty),
            family: Box::new(family),
            sides,
            cap: Box::new(cap),
        }
    }

    /// Create a face term
    pub fn face(ty: Term, face: Term) -> Self {
        Term::Face {
            ty: Box::new(ty),
            face: Box::new(face),
        }
    }

    /// Create a universe term
    pub fn universe(level: usize) -> Self {
        Term::Universe(level)
    }

    /// Create a unit term
    pub fn unit() -> Self {
        Term::Unit
    }
}

impl Type {
    /// Create a base type
    pub fn base(name: &str) -> Self {
        Type::Base(name.to_string())
    }

    /// Create a function type
    pub fn function(domain: Type, codomain: Type) -> Self {
        Type::Function {
            domain: Box::new(domain),
            codomain: Box::new(codomain),
        }
    }

    /// Create an identity type
    pub fn identity(left: Term, right: Term) -> Self {
        Type::Identity {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create a universe type
    pub fn universe(level: usize) -> Self {
        Type::Universe(level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term_creation() {
        // Test variable creation
        let x = Term::var("x");
        assert!(matches!(x, Term::Var(s) if s == "x"));

        // Test lambda abstraction
        let ty = Type::base("Int");
        let body = Term::var("x");
        let lambda = Term::lambda("x", ty.clone(), body);
        assert!(matches!(lambda, Term::Lambda { .. }));

        // Test path creation
        let x = Term::var("x");
        let y = Term::var("y");
        let ty = Term::var("ty");
        let path = Term::path(x, y, ty, 0);
        assert!(matches!(path, Term::Path { .. }));

        // Test higher inductive type constructor
        let hit = Term::hit("MyHIT", vec![]);
        assert!(matches!(hit, Term::HIT { .. }));

        // Test universe creation
        let universe = Term::universe(0);
        assert!(matches!(universe, Term::Universe(_)));

        // Test unit creation
        let unit = Term::unit();
        assert!(matches!(unit, Term::Unit));
    }

    #[test]
    fn test_type_creation() {
        // Test base type
        let int = Type::base("Int");
        assert!(matches!(int, Type::Base(s) if s == "Int"));

        // Test function type
        let bool = Type::base("Bool");
        let func = Type::function(int, bool);
        assert!(matches!(func, Type::Function { .. }));

        // Test identity type
        let x = Term::var("x");
        let y = Term::var("y");
        let eq = Type::identity(x, y);
        assert!(matches!(eq, Type::Identity { .. }));
    }
}
