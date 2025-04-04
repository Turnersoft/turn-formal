use serde::{Deserialize, Serialize};
use crate::formalize_v2::foundational_theories::type_theory::calculi::{Error, Result};

/// Term in System F (Polymorphic Lambda Calculus)
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

    /// Type abstraction (Λα.t)
    TypeLambda {
        type_var: String,
        body: Box<Term>,
    },

    /// Type application (t[T])
    TypeApply {
        term: Box<Term>,
        ty: Box<Type>,
    },

    /// Type annotations (t : T)
    Annotated {
        term: Box<Term>,
        ty: Box<Type>,
    },
}

/// Type in System F
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
    /// Type variables (α, β)
    Var(String),

    /// Base types
    Base(String),

    /// Function types (T → U)
    Function {
        domain: Box<Type>,
        codomain: Box<Type>,
    },

    /// Universal quantification (∀α.T)
    Universal {
        var: String,
        body: Box<Type>,
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

    /// Create a type lambda abstraction
    pub fn type_lambda(type_var: &str, body: Term) -> Self {
        Term::TypeLambda {
            type_var: type_var.to_string(),
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

    /// Create a type annotation
    pub fn annotated(term: Term, ty: Type) -> Self {
        Term::Annotated {
            term: Box::new(term),
            ty: Box::new(ty),
        }
    }
}

impl Type {
    /// Create a type variable
    pub fn var(name: &str) -> Self {
        Type::Var(name.to_string())
    }

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

    /// Create a universal type
    pub fn universal(var: &str, body: Type) -> Self {
        Type::Universal {
            var: var.to_string(),
            body: Box::new(body),
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
        let ty = Type::base("Int");
        let body = Term::var("x");
        let lambda = Term::lambda("x", ty.clone(), body);
        assert!(matches!(lambda, Term::Lambda { .. }));

        // Test type abstraction
        let type_lambda = Term::type_lambda("T", Term::var("x"));
        assert!(matches!(type_lambda, Term::TypeLambda { .. }));
    }

    #[test]
    fn test_type_creation() {
        // Test type variable
        let alpha = Type::var("α");
        assert!(matches!(alpha, Type::Var(s) if s == "α"));

        // Test function type
        let int = Type::base("Int");
        let bool = Type::base("Bool");
        let func = Type::function(int, bool);
        assert!(matches!(func, Type::Function { .. }));

        // Test universal type
        let univ = Type::universal("T", Type::var("T"));
        assert!(matches!(univ, Type::Universal { .. }));
    }
}