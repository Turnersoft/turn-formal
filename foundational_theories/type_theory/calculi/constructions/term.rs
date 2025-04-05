//! Calculus of Constructions Term Type
//! Defines the term type for the Calculus of Constructions (λC)

use serde::{Deserialize, Serialize};
use crate::foundational_theories::type_theory::calculi::{Error, Result};

/// Sort in Calculus of Constructions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sort {
    /// Prop universe (propositions)
    Prop,
    /// Type universe (types)
    Type,
}

/// Term in Calculus of Constructions
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

    /// Type abstraction (Λα:κ.t)
    TypeLambda {
        var: String,
        kind: Sort,
        body: Box<Term>,
    },

    /// Type application (t[T])
    TypeApply {
        term: Box<Term>,
        ty: Box<Type>,
    },

    /// Dependent product (Πx:A.B)
    Pi {
        var: String,
        domain: Box<Type>,
        codomain: Box<Type>,
    },

    /// Sort (Prop or Type)
    Sort(Sort),

    /// Type annotations (t : T)
    Annotated {
        term: Box<Term>,
        ty: Box<Type>,
    },
}

/// Type in Calculus of Constructions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
    /// Sort (Prop or Type)
    Sort(Sort),
    
    /// Type variable
    Var(String),
    
    /// Dependent product (Πx:A.B)
    Pi {
        var: String,
        domain: Box<Type>,
        codomain: Box<Type>,
    },
    
    /// Type-level abstraction (λx:A.B)
    Lambda {
        var: String,
        domain: Box<Type>,
        body: Box<Type>,
    },
    
    /// Type-level application (A B)
    Apply {
        func: Box<Type>,
        arg: Box<Type>,
    },
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

    /// Create a type lambda
    pub fn type_lambda(var: &str, kind: Sort, body: Term) -> Self {
        Term::TypeLambda {
            var: var.to_string(),
            kind,
            body: Box::new(body),
        }
    }

    /// Create a type application
    pub fn type_apply(term: Term, ty: Type) -> Self {
        Term::TypeApply {
            term: Box::new(term),
            ty: Box::new(ty),
        }
    }

    /// Create a dependent product type
    pub fn pi(var: &str, domain: Type, codomain: Type) -> Self {
        Term::Pi {
            var: var.to_string(),
            domain: Box::new(domain),
            codomain: Box::new(codomain),
        }
    }

    /// Create a sort term
    pub fn sort(sort: Sort) -> Self {
        Term::Sort(sort)
    }

    /// Create a type annotation
    pub fn annotated(term: Term, ty: Type) -> Self {
        Term::Annotated {
            term: Box::new(term),
            ty: Box::new(ty),
        }
    }
}

impl Type {
    /// Create a sort type
    pub fn sort(sort: Sort) -> Self {
        Type::Sort(sort)
    }

    /// Create a type variable
    pub fn var(name: &str) -> Self {
        Type::Var(name.to_string())
    }

    /// Create a dependent product type
    pub fn pi(var: &str, domain: Type, codomain: Type) -> Self {
        Type::Pi {
            var: var.to_string(),
            domain: Box::new(domain),
            codomain: Box::new(codomain),
        }
    }

    /// Create a type-level lambda
    pub fn lambda(var: &str, domain: Type, body: Type) -> Self {
        Type::Lambda {
            var: var.to_string(),
            domain: Box::new(domain),
            body: Box::new(body),
        }
    }

    /// Create a type-level application
    pub fn apply(func: Type, arg: Type) -> Self {
        Type::Apply {
            func: Box::new(func),
            arg: Box::new(arg),
        }
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
        let id = Term::lambda(
            "x",
            Type::sort(Sort::Type),
            Term::var("x"),
        );
        assert!(matches!(id, Term::Lambda { .. }));

        // Test type lambda
        let poly_id = Term::type_lambda(
            "α",
            Sort::Type,
            Term::lambda(
                "x",
                Type::var("α"),
                Term::var("x"),
            ),
        );
        assert!(matches!(poly_id, Term::TypeLambda { .. }));
    }

    #[test]
    fn test_type_creation() {
        // Test sort type
        let prop = Type::sort(Sort::Prop);
        assert!(matches!(prop, Type::Sort(Sort::Prop)));

        // Test dependent product
        let pi = Type::pi(
            "x",
            Type::sort(Sort::Type),
            Type::var("x"),
        );
        assert!(matches!(pi, Type::Pi { .. }));

        // Test type-level lambda
        let type_abs = Type::lambda(
            "x",
            Type::sort(Sort::Type),
            Type::var("x"),
        );
        assert!(matches!(type_abs, Type::Lambda { .. }));
    }
}
