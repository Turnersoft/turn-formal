//! Simply Typed Lambda Calculus Terms
//! Defines the term type for Simply Typed Lambda Calculus (STLC)

use serde::{Deserialize, Serialize};
use crate::foundational_theories::type_theory::calculi::{Error, Result};

/// Term in Simply Typed Lambda Calculus
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
}

/// Type in Simply Typed Lambda Calculus
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    /// Base type
    Base(String),

    /// Function type (T → U)
    Function {
        domain: Box<Type>,
        codomain: Box<Type>,
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

    /// Create a type annotation
    pub fn annotated(term: Term, ty: Type) -> Self {
        Term::Annotated {
            term: Box::new(term),
            ty: Box::new(ty),
        }
    }

    /// Get free variables in a term
    pub fn free_vars(&self) -> Vec<String> {
        match self {
            Term::Var(x) => vec![x.clone()],
            Term::Lambda { var, body, .. } => {
                let mut vars = body.free_vars();
                vars.retain(|x| x != var);
                vars
            }
            Term::Apply { func, arg } => {
                let mut vars = func.free_vars();
                vars.extend(arg.free_vars());
                vars.sort();
                vars.dedup();
                vars
            }
            Term::Annotated { term, .. } => term.free_vars(),
        }
    }

    /// Check if a term contains a variable
    pub fn contains_var(&self, var: &str) -> bool {
        match self {
            Term::Var(x) => x == var,
            Term::Lambda { var: v, body, .. } => v == var || body.contains_var(var),
            Term::Apply { func, arg } => func.contains_var(var) || arg.contains_var(var),
            Term::Annotated { term, .. } => term.contains_var(var),
        }
    }

    /// Substitute a term for a variable
    pub fn substitute(&self, var: &str, replacement: &Term) -> Term {
        match self {
            Term::Var(x) if x == var => replacement.clone(),
            Term::Var(_) => self.clone(),
            Term::Lambda { var: v, ty, body } if v != var => Term::lambda(
                v,
                *ty.clone(),
                body.substitute(var, replacement),
            ),
            Term::Lambda { .. } => self.clone(),
            Term::Apply { func, arg } => Term::apply(
                func.substitute(var, replacement),
                arg.substitute(var, replacement),
            ),
            Term::Annotated { term, ty } => Term::annotated(
                term.substitute(var, replacement),
                *ty.clone(),
            ),
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term_creation() {
        // λx:Bool.x : Bool → Bool
        let term = Term::lambda(
            "x",
            Type::base("Bool"),
            Term::var("x"),
        );

        match term {
            Term::Lambda { var, ty, body } => {
                assert_eq!(var, "x");
                assert_eq!(*ty, Type::base("Bool"));
                assert_eq!(*body, Term::var("x"));
            }
            _ => panic!("Expected lambda term"),
        }
    }

    #[test]
    fn test_type_creation() {
        // Bool → Bool
        let ty = Type::function(
            Type::base("Bool"),
            Type::base("Bool"),
        );

        match ty {
            Type::Function { domain, codomain } => {
                assert_eq!(*domain, Type::base("Bool"));
                assert_eq!(*codomain, Type::base("Bool"));
            }
            _ => panic!("Expected function type"),
        }
    }

    #[test]
    fn test_free_vars() {
        // λx.y x
        let term = Term::lambda(
            "x",
            Type::base("Bool"),
            Term::apply(Term::var("y"), Term::var("x")),
        );

        let free_vars = term.free_vars();
        assert_eq!(free_vars, vec!["y"]);
    }

    #[test]
    fn test_substitution() {
        // (λx.y)[y := z] = λx.z
        let term = Term::lambda(
            "x",
            Type::base("Bool"),
            Term::var("y"),
        );
        let result = term.substitute("y", &Term::var("z"));

        match result {
            Term::Lambda { var, body, .. } => {
                assert_eq!(var, "x");
                assert_eq!(*body, Term::var("z"));
            }
            _ => panic!("Expected lambda term"),
        }
    }
}
