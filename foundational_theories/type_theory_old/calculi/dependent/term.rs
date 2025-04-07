use serde::{Deserialize, Serialize};
use crate::foundational_theories::type_theory::calculi::{Error, Result};

/// Term in Dependent Type Theory
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Term {
    /// Variables (x, y, z)
    Var(String),

    /// Lambda abstraction (λx:T.t)
    Lambda {
        var: String,
        ty: Box<Term>,
        body: Box<Term>,
    },

    /// Application (t u)
    Apply {
        func: Box<Term>,
        arg: Box<Term>,
    },

    /// Pi type (Πx:A.B)
    Pi {
        var: String,
        domain: Box<Term>,
        codomain: Box<Term>,
    },

    /// Type universe (Type_n)
    Universe(usize),

    /// Type annotations (t : T)
    Annotated {
        term: Box<Term>,
        ty: Box<Term>,
    },

    /// Pair (a, b)
    Pair {
        first: Box<Term>,
        second: Box<Term>,
    },

    /// First projection (fst p)
    First(Box<Term>),

    /// Second projection (snd p)
    Second(Box<Term>),

    /// Sigma type (Σx:A.B)
    Sigma {
        var: String,
        domain: Box<Term>,
        codomain: Box<Term>,
    },
}

impl Term {
    /// Create a variable term
    pub fn var(name: &str) -> Self {
        Term::Var(name.to_string())
    }

    /// Create a lambda abstraction
    pub fn lambda(var: &str, ty: Term, body: Term) -> Self {
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

    /// Create a Pi type
    pub fn pi(var: &str, domain: Term, codomain: Term) -> Self {
        Term::Pi {
            var: var.to_string(),
            domain: Box::new(domain),
            codomain: Box::new(codomain),
        }
    }

    /// Create a universe
    pub fn universe(level: usize) -> Self {
        Term::Universe(level)
    }

    /// Create a type annotation
    pub fn annotated(term: Term, ty: Term) -> Self {
        Term::Annotated {
            term: Box::new(term),
            ty: Box::new(ty),
        }
    }

    /// Create a pair
    pub fn pair(first: Term, second: Term) -> Self {
        Term::Pair {
            first: Box::new(first),
            second: Box::new(second),
        }
    }

    /// Create a first projection
    pub fn first(pair: Term) -> Self {
        Term::First(Box::new(pair))
    }

    /// Create a second projection
    pub fn second(pair: Term) -> Self {
        Term::Second(Box::new(pair))
    }

    /// Create a Sigma type
    pub fn sigma(var: &str, domain: Term, codomain: Term) -> Self {
        Term::Sigma {
            var: var.to_string(),
            domain: Box::new(domain),
            codomain: Box::new(codomain),
        }
    }

    /// Substitute a term for a variable
    pub fn substitute(&self, var: &str, replacement: &Term) -> Term {
        match self {
            Term::Var(x) if x == var => replacement.clone(),
            Term::Var(_) => self.clone(),
            Term::Lambda { var: v, ty, body } if v != var => Term::lambda(
                v,
                ty.substitute(var, replacement),
                body.substitute(var, replacement),
            ),
            Term::Lambda { .. } => self.clone(),
            Term::Apply { func, arg } => Term::apply(
                func.substitute(var, replacement),
                arg.substitute(var, replacement),
            ),
            Term::Pi { var: v, domain, codomain } if v != var => Term::pi(
                v,
                domain.substitute(var, replacement),
                codomain.substitute(var, replacement),
            ),
            Term::Pi { .. } => self.clone(),
            Term::Universe(_) => self.clone(),
            Term::Annotated { term, ty } => Term::annotated(
                term.substitute(var, replacement),
                ty.substitute(var, replacement),
            ),
            Term::Pair { first, second } => Term::pair(
                first.substitute(var, replacement),
                second.substitute(var, replacement),
            ),
            Term::First(pair) => Term::first(pair.substitute(var, replacement)),
            Term::Second(pair) => Term::second(pair.substitute(var, replacement)),
            Term::Sigma { var: v, domain, codomain } if v != var => Term::sigma(
                v,
                domain.substitute(var, replacement),
                codomain.substitute(var, replacement),
            ),
            Term::Sigma { .. } => self.clone(),
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
        let ty = Term::universe(0);
        let body = Term::var("x");
        let lambda = Term::lambda("x", ty.clone(), body);
        assert!(matches!(lambda, Term::Lambda { .. }));

        // Test Pi type
        let domain = Term::universe(0);
        let codomain = Term::var("x");
        let pi = Term::pi("x", domain, codomain);
        assert!(matches!(pi, Term::Pi { .. }));
    }

    #[test]
    fn test_substitution() {
        // Test variable substitution
        let x = Term::var("x");
        let y = Term::var("y");
        assert_eq!(x.substitute("x", &y), y);

        // Test lambda substitution
        let lambda = Term::lambda("x", Term::universe(0), Term::var("y"));
        let result = lambda.substitute("y", &Term::var("z"));
        assert!(matches!(result,
            Term::Lambda { var, ty, body } if var == "x"
                && matches!(*ty, Term::Universe(0))
                && matches!(*body, Term::Var(s) if s == "z")
        ));
    }

    #[test]
    fn test_pair_operations() {
        // Test pair creation
        let pair = Term::pair(Term::var("x"), Term::var("y"));
        assert!(matches!(pair, Term::Pair { .. }));

        // Test projections
        let first = Term::first(pair.clone());
        let second = Term::second(pair);
        assert!(matches!(first, Term::First(_)));
        assert!(matches!(second, Term::Second(_)));
    }
}
