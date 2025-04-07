use crate::foundational_theories::type_theory::calculi::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub mod term;
use term::{Term, Type};

/// Type environment for System F
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeEnv {
    /// Term variable types
    term_types: HashMap<String, Type>,
    /// Type variables in scope
    type_vars: HashSet<String>,
}

impl TypeEnv {
    /// Create new type environment
    pub fn new() -> Self {
        TypeEnv {
            term_types: HashMap::new(),
            type_vars: HashSet::new(),
        }
    }

    /// Add term type binding
    pub fn add_term(&mut self, var: String, ty: Type) {
        self.term_types.insert(var, ty);
    }

    /// Add type variable
    pub fn add_type_var(&mut self, var: String) {
        self.type_vars.insert(var);
    }

    /// Get type of term variable
    pub fn get_term(&self, var: &str) -> Option<&Type> {
        self.term_types.get(var)
    }

    /// Check if type variable is in scope
    pub fn has_type_var(&self, var: &str) -> bool {
        self.type_vars.contains(var)
    }
}

/// System F implementation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemF {
    /// Type environment
    env: TypeEnv,
}

impl SystemF {
    /// Create new System F instance
    pub fn new() -> Self {
        SystemF {
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
                let body_ty = SystemF { env }.type_check(body)?;
                Ok(Type::Function {
                    domain: ty.clone(),
                    codomain: Box::new(body_ty),
                })
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

            Term::TypeLambda { type_var, body } => {
                let mut env = self.env.clone();
                env.add_type_var(type_var.clone());
                let body_ty = SystemF { env }.type_check(body)?;
                Ok(Type::Universal {
                    var: type_var.clone(),
                    body: Box::new(body_ty),
                })
            }

            Term::TypeApply { term, ty } => {
                let term_ty = self.type_check(term)?;
                match term_ty {
                    Type::Universal { var, body } => {
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
            Type::Function { domain, codomain } => Type::Function {
                domain: Box::new(self.substitute_type(domain, var, replacement)),
                codomain: Box::new(self.substitute_type(codomain, var, replacement)),
            },
            Type::Universal { var: v, body } if v != var => Type::Universal {
                var: v.clone(),
                body: Box::new(self.substitute_type(body, var, replacement)),
            },
            Type::Universal { .. } => ty.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_check_identity() {
        let system_f = SystemF::new();
        
        // Polymorphic identity function: Λα.λx:α.x
        let id_term = Term::type_lambda(
            "α",
            Term::lambda(
                "x",
                Type::var("α"),
                Term::var("x"),
            ),
        );

        // Should type check to ∀α.α→α
        let ty = system_f.type_check(&id_term).unwrap();
        assert!(matches!(ty,
            Type::Universal { var, body } if var == "α" && matches!(*body,
                Type::Function { domain, codomain } if matches!(*domain, Type::Var(v) if v == "α")
                                                     && matches!(*codomain, Type::Var(v) if v == "α")
            )
        ));
    }

    #[test]
    fn test_type_check_application() {
        let system_f = SystemF::new();
        
        // (Λα.λx:α.x)[Int]
        let term = Term::type_apply(
            Term::type_lambda(
                "α",
                Term::lambda(
                    "x",
                    Type::var("α"),
                    Term::var("x"),
                ),
            ),
            Type::base("Int"),
        );

        // Should type check to Int→Int
        let ty = system_f.type_check(&term).unwrap();
        assert!(matches!(ty,
            Type::Function { domain, codomain } if matches!(*domain, Type::Base(s) if s == "Int")
                                                 && matches!(*codomain, Type::Base(s) if s == "Int")
        ));
    }
}
