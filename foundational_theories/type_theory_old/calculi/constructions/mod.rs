//! Calculus of Constructions (λC)
//! The most powerful calculus, combining System F and dependent types

use serde::{Deserialize, Serialize};
use crate::foundational_theories::type_theory::calculi::{Error, Result};
use std::collections::HashMap;

pub mod term;
use term::{Term, Type, Sort};

/// Type environment for Calculus of Constructions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeEnv {
    /// Term-level bindings
    term_types: HashMap<String, Type>,
    /// Type-level bindings
    type_sorts: HashMap<String, Sort>,
}

impl TypeEnv {
    /// Create new type environment
    pub fn new() -> Self {
        TypeEnv {
            term_types: HashMap::new(),
            type_sorts: HashMap::new(),
        }
    }

    /// Add term binding
    pub fn add_term(&mut self, var: String, ty: Type) {
        self.term_types.insert(var, ty);
    }

    /// Add type binding
    pub fn add_type(&mut self, var: String, sort: Sort) {
        self.type_sorts.insert(var, sort);
    }

    /// Get term type
    pub fn get_term(&self, var: &str) -> Option<&Type> {
        self.term_types.get(var)
    }

    /// Get type sort
    pub fn get_type(&self, var: &str) -> Option<&Sort> {
        self.type_sorts.get(var)
    }
}

/// Calculus of Constructions implementation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Constructions {
    /// Type environment
    env: TypeEnv,
}

impl Constructions {
    /// Create new empty calculus
    pub fn new() -> Self {
        Constructions {
            env: TypeEnv::new(),
        }
    }

    /// Type check a term
    pub fn type_check(&self, term: &Term) -> Result<Type> {
        match term {
            Term::Var(name) => self.env.get_term(name)
                .cloned()
                .ok_or_else(|| Error::TypeError(format!("Unbound variable: {}", name))),

            Term::Lambda { var, ty, body } => {
                let mut env = self.env.clone();
                env.add_term(var.clone(), *ty.clone());
                let body_ty = Constructions { env }.type_check(body)?;
                Ok(Type::Pi {
                    var: var.clone(),
                    domain: ty.clone(),
                    codomain: Box::new(body_ty),
                })
            }

            Term::Apply { func, arg } => {
                let func_ty = self.type_check(func)?;
                let arg_ty = self.type_check(arg)?;
                match func_ty {
                    Type::Pi { var, domain, codomain } => {
                        if *domain == arg_ty {
                            Ok(*codomain)
                        } else {
                            Err(Error::TypeError("Function argument type mismatch".to_string()))
                        }
                    }
                    _ => Err(Error::TypeError("Expected dependent function type".to_string())),
                }
            }

            Term::TypeLambda { var, kind, body } => {
                let mut env = self.env.clone();
                env.add_type(var.clone(), kind.clone());
                let body_ty = Constructions { env }.type_check(body)?;
                Ok(body_ty)
            }

            Term::TypeApply { term, ty } => {
                let term_ty = self.type_check(term)?;
                self.substitute_type(&term_ty, ty)
            }

            Term::Pi { var, domain, codomain } => {
                let domain_sort = self.type_check_type(domain)?;
                let mut env = self.env.clone();
                env.add_term(var.clone(), *domain.clone());
                let codomain_sort = Constructions { env }.type_check_type(codomain)?;
                Ok(Type::Sort(Sort::Type))
            }

            Term::Sort(sort) => Ok(Type::Sort(sort.clone())),

            Term::Annotated { term, ty } => {
                let inferred_ty = self.type_check(term)?;
                if inferred_ty == *ty {
                    Ok(*ty.clone())
                } else {
                    Err(Error::TypeError("Type annotation mismatch".to_string()))
                }
            }
        }
    }

    /// Type check a type
    fn type_check_type(&self, ty: &Type) -> Result<Sort> {
        match ty {
            Type::Sort(sort) => Ok(sort.clone()),

            Type::Var(name) => self.env.get_type(name)
                .cloned()
                .ok_or_else(|| Error::TypeError(format!("Unbound type variable: {}", name))),

            Type::Pi { var, domain, codomain } => {
                let domain_sort = self.type_check_type(domain)?;
                let mut env = self.env.clone();
                env.add_term(var.clone(), *domain.clone());
                let codomain_sort = Constructions { env }.type_check_type(codomain)?;
                Ok(Sort::Type)
            }

            Type::Lambda { var, domain, body } => {
                let domain_sort = self.type_check_type(domain)?;
                let mut env = self.env.clone();
                env.add_term(var.clone(), *domain.clone());
                Constructions { env }.type_check_type(body)
            }

            Type::Apply { func, arg } => {
                let func_sort = self.type_check_type(func)?;
                let arg_sort = self.type_check_type(arg)?;
                Ok(Sort::Type)
            }
        }
    }

    /// Substitute a type in another type
    fn substitute_type(&self, ty: &Type, replacement: &Type) -> Result<Type> {
        Ok(ty.clone()) // TODO: Implement proper type substitution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_checking() {
        let calc = Constructions::new();

        // λx:Type.x
        let id = Term::lambda(
            "x",
            Type::sort(Sort::Type),
            Term::var("x"),
        );

        let ty = calc.type_check(&id).unwrap();
        assert!(matches!(ty, Type::Pi { .. }));
    }

    #[test]
    fn test_dependent_function() {
        let calc = Constructions::new();

        // Πx:Type.x→x
        let ty = Term::pi(
            "x",
            Type::sort(Sort::Type),
            Type::pi(
                "y",
                Type::var("x"),
                Type::var("x"),
            ),
        );

        let sort = calc.type_check(&ty).unwrap();
        assert!(matches!(sort, Type::Sort(Sort::Type)));
    }
}
