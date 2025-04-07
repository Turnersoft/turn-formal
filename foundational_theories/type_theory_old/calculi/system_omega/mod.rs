//! System Omega (λω) - Higher-Order Lambda Calculus
//! Extends System F with type-level computation

use serde::{Deserialize, Serialize};
use crate::foundational_theories::type_theory::calculi::{Error, Result};
use std::collections::{HashMap, HashSet};

pub mod term;
use term::{Term, Type, Kind};

/// Type environment for System Omega
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeEnv {
    /// Term variable types
    term_types: HashMap<String, Type>,
    /// Type variables and their kinds
    type_kinds: HashMap<String, Kind>,
}

impl TypeEnv {
    /// Create new type environment
    pub fn new() -> Self {
        TypeEnv {
            term_types: HashMap::new(),
            type_kinds: HashMap::new(),
        }
    }

    /// Add term type binding
    pub fn add_term(&mut self, var: String, ty: Type) {
        self.term_types.insert(var, ty);
    }

    /// Add type kind binding
    pub fn add_type(&mut self, var: String, kind: Kind) {
        self.type_kinds.insert(var, kind);
    }

    /// Get type of term variable
    pub fn get_term(&self, var: &str) -> Option<&Type> {
        self.term_types.get(var)
    }

    /// Get kind of type variable
    pub fn get_type(&self, var: &str) -> Option<&Kind> {
        self.type_kinds.get(var)
    }
}

/// System Omega implementation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemOmega {
    /// Type environment
    env: TypeEnv,
}

impl SystemOmega {
    /// Create new System Omega instance
    pub fn new() -> Self {
        SystemOmega {
            env: TypeEnv::new(),
        }
    }

    /// Type check a term
    pub fn type_check(&self, term: &Term) -> Result<Type> {
        match term {
            Term::Var(x) => self.env.get_term(x)
                .cloned()
                .ok_or_else(|| Error::TypeError(format!("Unbound variable: {}", x))),

            Term::Lambda { var, ty, body } => {
                let mut env = self.env.clone();
                env.add_term(var.clone(), *ty.clone());
                let body_ty = SystemOmega { env }.type_check(body)?;
                Ok(Type::function(*ty.clone(), body_ty))
            }

            Term::Apply { func, arg } => {
                let func_ty = self.type_check(func)?;
                let arg_ty = self.type_check(arg)?;
                match func_ty {
                    Type::Function { domain, codomain } => {
                        if *domain == arg_ty {
                            Ok(*codomain)
                        } else {
                            Err(Error::TypeError("Function argument type mismatch".to_string()))
                        }
                    }
                    _ => Err(Error::TypeError("Expected function type".to_string())),
                }
            }

            Term::TypeLambda { type_var, kind, body } => {
                let mut env = self.env.clone();
                env.add_type(type_var.clone(), *kind.clone());
                let body_ty = SystemOmega { env }.type_check(body)?;
                Ok(Type::universal(type_var, *kind.clone(), body_ty))
            }

            Term::TypeApply { term, ty } => {
                let term_ty = self.type_check(term)?;
                match term_ty {
                    Type::Universal { var, kind: _, body } => {
                        Ok(self.substitute_type(&body, &var, ty))
                    }
                    _ => Err(Error::TypeError("Expected universal type".to_string())),
                }
            }

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

    /// Substitute type variable in type
    pub fn substitute_type(&self, ty: &Type, var: &str, replacement: &Type) -> Type {
        match ty {
            Type::Var(x) if x == var => replacement.clone(),
            Type::Var(_) => ty.clone(),
            Type::Base(_) => ty.clone(),
            Type::Function { domain, codomain } => Type::function(
                self.substitute_type(domain, var, replacement),
                self.substitute_type(codomain, var, replacement),
            ),
            Type::Universal { var: v, kind, body } if v != var => Type::universal(
                v,
                *kind.clone(),
                self.substitute_type(body, var, replacement),
            ),
            Type::Universal { .. } => ty.clone(),
            Type::TypeLambda { var: v, kind, body } if v != var => Type::type_lambda(
                v,
                *kind.clone(),
                self.substitute_type(body, var, replacement),
            ),
            Type::TypeLambda { .. } => ty.clone(),
            Type::TypeApply { func, arg } => Type::type_apply(
                self.substitute_type(func, var, replacement),
                self.substitute_type(arg, var, replacement),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_check_identity() {
        let system = SystemOmega::new();
        
        // Polymorphic identity function: Λα:*.λx:α.x
        let id_term = Term::type_lambda(
            "α",
            Kind::type_kind(),
            Term::lambda(
                "x",
                Type::var("α"),
                Term::var("x"),
            ),
        );

        let ty = system.type_check(&id_term).unwrap();
        assert!(matches!(ty,
            Type::Universal { var, kind, body } if var == "α" 
                && matches!(*kind, Kind::Type)
                && matches!(*body,
                    Type::Function { domain, codomain } if matches!(*domain, Type::Var(v) if v == "α")
                                                         && matches!(*codomain, Type::Var(v) if v == "α")
                )
        ));
    }

    #[test]
    fn test_type_level_computation() {
        let system = SystemOmega::new();

        // Type-level identity function: λα:*.α
        let type_id = Type::type_lambda(
            "α",
            Kind::type_kind(),
            Type::var("α"),
        );

        // Application: (λα:*.α) Int
        let applied = Type::type_apply(
            type_id,
            Type::base("Int"),
        );

        // Create a term using this type
        let term = Term::lambda(
            "x",
            applied.clone(),
            Term::var("x"),
        );

        let ty = system.type_check(&term).unwrap();
        assert!(matches!(ty,
            Type::Function { domain, codomain } if *domain == Type::base("Int")
                                                 && *codomain == Type::base("Int")
        ));
    }
}
