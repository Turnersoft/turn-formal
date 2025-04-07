//! Dependent Type Theory
//! Implementation of a basic dependent type system

use serde::{Deserialize, Serialize};
use crate::foundational_theories::type_theory::calculi::{Error, Result};
use std::collections::HashMap;

pub mod term;
use term::Term;

/// Type environment for dependent types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeEnv {
    /// Term variable types
    term_types: HashMap<String, Term>,
}

impl TypeEnv {
    /// Create new type environment
    pub fn new() -> Self {
        TypeEnv {
            term_types: HashMap::new(),
        }
    }

    /// Add term type binding
    pub fn add_term(&mut self, var: String, ty: Term) {
        self.term_types.insert(var, ty);
    }

    /// Get type of term variable
    pub fn get_term(&self, var: &str) -> Option<&Term> {
        self.term_types.get(var)
    }
}

/// Dependent type system implementation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependentType {
    /// Type environment
    env: TypeEnv,
}

impl DependentType {
    /// Create new dependent type system
    pub fn new() -> Self {
        DependentType {
            env: TypeEnv::new(),
        }
    }

    /// Type check and return the type of a term
    pub fn type_check(&self, term: &Term) -> Result<Term> {
        match term {
            Term::Var(x) => self.env.get_term(x)
                .cloned()
                .ok_or_else(|| Error::TypeError(format!("Unbound variable: {}", x))),

            Term::Lambda { var, ty, body } => {
                let mut env = self.env.clone();
                env.add_term(var.to_string(), *ty.clone());
                let body_ty = DependentType { env }.type_check(body)?;
                Ok(Term::pi(var, *ty.clone(), body_ty))
            }

            Term::Apply { func, arg } => {
                let func_ty = self.type_check(func)?;
                let arg_ty = self.type_check(arg)?;
                match func_ty {
                    Term::Pi { var, domain, codomain } => {
                        if self.type_check(&domain)? == arg_ty {
                            Ok(codomain.substitute(&var, arg))
                        } else {
                            Err(Error::TypeError("Function argument type mismatch".to_string()))
                        }
                    }
                    _ => Err(Error::TypeError("Expected Pi type".to_string())),
                }
            }

            Term::Pi { var: _, domain, codomain } => {
                let domain_ty = self.type_check(domain)?;
                let codomain_ty = self.type_check(codomain)?;
                match (domain_ty, codomain_ty) {
                    (Term::Universe(n), Term::Universe(m)) => Ok(Term::universe(n.max(m))),
                    _ => Err(Error::TypeError("Invalid Pi type".to_string())),
                }
            }

            Term::Universe(n) => Ok(Term::universe(n + 1)),

            Term::Annotated { term, ty } => {
                let inferred_ty = self.type_check(term)?;
                let expected_ty = self.type_check(ty)?;
                if inferred_ty == expected_ty {
                    Ok(*ty.clone())
                } else {
                    Err(Error::TypeError("Type annotation mismatch".to_string()))
                }
            }

            Term::Pair { first, second } => {
                let first_ty = self.type_check(first)?;
                let second_ty = self.type_check(second)?;
                Ok(Term::sigma("_", first_ty, second_ty))
            }

            Term::First(pair) => {
                match self.type_check(pair)? {
                    Term::Sigma { domain, .. } => Ok(*domain),
                    _ => Err(Error::TypeError("Expected Sigma type".to_string())),
                }
            }

            Term::Second(pair) => {
                match self.type_check(pair)? {
                    Term::Sigma { var, domain: _, codomain } => {
                        Ok(*codomain.substitute(&var, &Term::first(pair.clone())))
                    }
                    _ => Err(Error::TypeError("Expected Sigma type".to_string())),
                }
            }

            Term::Sigma { var: _, domain, codomain } => {
                let domain_ty = self.type_check(domain)?;
                let codomain_ty = self.type_check(codomain)?;
                match (domain_ty, codomain_ty) {
                    (Term::Universe(n), Term::Universe(m)) => Ok(Term::universe(n.max(m))),
                    _ => Err(Error::TypeError("Invalid Sigma type".to_string())),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependent_function() {
        let system = DependentType::new();

        // Π(A:Type₀).(A → A)
        let term = Term::pi(
            "A",
            Term::universe(0),
            Term::pi(
                "x",
                Term::var("A"),
                Term::var("A"),
            ),
        );

        let ty = system.type_check(&term).unwrap();
        assert!(matches!(ty, Term::Universe(1)));
    }

    #[test]
    fn test_dependent_pair() {
        let system = DependentType::new();

        // Σ(A:Type₀).(A → A)
        let term = Term::sigma(
            "A",
            Term::universe(0),
            Term::pi(
                "x",
                Term::var("A"),
                Term::var("A"),
            ),
        );

        let ty = system.type_check(&term).unwrap();
        assert!(matches!(ty, Term::Universe(1)));
    }

    #[test]
    fn test_polymorphic_identity() {
        let system = DependentType::new();

        // λ(A:Type₀).λ(x:A).x
        let id = Term::lambda(
            "A",
            Term::universe(0),
            Term::lambda(
                "x",
                Term::var("A"),
                Term::var("x"),
            ),
        );

        let ty = system.type_check(&id).unwrap();
        assert!(matches!(ty,
            Term::Pi { var: a, domain, codomain } if a == "A"
                && matches!(*domain, Term::Universe(0))
                && matches!(*codomain,
                    Term::Pi { var: x, domain: d, codomain: c } if x == "x"
                        && matches!(*d, Term::Var(v) if v == "A")
                        && matches!(*c, Term::Var(v) if v == "A")
                )
        ));
    }
}
