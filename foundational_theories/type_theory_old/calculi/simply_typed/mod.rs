//! Simply Typed Lambda Calculus (λ→)
//! The simplest typed lambda calculus with only function types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Error, Result, TypeChecker, Reducer};
pub mod term;
pub use term::{Term, Type};

/// Type environment for Simply Typed Lambda Calculus
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeEnv {
    /// Term variable types
    types: HashMap<String, Type>,
}

impl TypeEnv {
    /// Create new type environment
    pub fn new() -> Self {
        TypeEnv {
            types: HashMap::new(),
        }
    }

    /// Add type binding
    pub fn add(&mut self, var: String, ty: Type) {
        self.types.insert(var, ty);
    }

    /// Get type of variable
    pub fn get(&self, var: &str) -> Option<&Type> {
        self.types.get(var)
    }

    /// Create environment with single binding
    pub fn single(var: String, ty: Type) -> Self {
        let mut env = TypeEnv::new();
        env.add(var, ty);
        env
    }

    /// Extend environment with new binding
    pub fn extend(&self, var: String, ty: Type) -> Self {
        let mut new_env = self.clone();
        new_env.add(var, ty);
        new_env
    }
}

/// Simply typed lambda calculus implementation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimplyTyped {
    /// Type environment
    env: TypeEnv,
}

impl SimplyTyped {
    /// Create new empty calculus
    pub fn new() -> Self {
        SimplyTyped {
            env: TypeEnv::new(),
        }
    }

    /// Create calculus with initial environment
    pub fn with_env(env: TypeEnv) -> Self {
        SimplyTyped { env }
    }
}

impl TypeChecker for SimplyTyped {
    type Term = Term;
    type Type = Type;

    fn type_check(&self, term: &Term) -> Result<Type> {
        match term {
            Term::Var(name) => {
                self.env
                    .get(name)
                    .cloned()
                    .ok_or_else(|| Error::UnboundVariable(name.clone()))
            }
            Term::Lambda { var, ty, body } => {
                let new_env = self.env.extend(var.clone(), *ty.clone());
                let body_ty = SimplyTyped::with_env(new_env).type_check(body)?;
                Ok(Type::function(*ty.clone(), body_ty))
            }
            Term::Apply { func, arg } => {
                match self.type_check(func)? {
                    Type::Function { domain, codomain } => {
                        let arg_ty = self.type_check(arg)?;
                        if arg_ty == *domain {
                            Ok(*codomain)
                        } else {
                            Err(Error::TypeError(format!(
                                "Type mismatch in application: expected {domain:?}, got {arg_ty:?}"
                            )))
                        }
                    }
                    ty => Err(Error::TypeError(format!(
                        "Expected function type, got {:?}",
                        ty
                    ))),
                }
            }
            Term::Annotated { term, ty } => {
                let inferred = self.type_check(term)?;
                if inferred == **ty {
                    Ok(*ty.clone())
                } else {
                    Err(Error::TypeError(format!(
                        "Type annotation mismatch: expected {:?}, got {:?}",
                        ty, inferred
                    )))
                }
            }
        }
    }
}

impl Reducer for SimplyTyped {
    type Term = Term;

    fn reduce(&self, term: &Term) -> Result<Term> {
        match term {
            Term::Apply { func, arg } => {
                let func = self.reduce(func)?;
                let arg = self.reduce(arg)?;
                match func {
                    Term::Lambda { var, body, .. } => {
                        // Beta reduction
                        Ok(body.substitute(&var, &arg))
                    }
                    _ => Ok(Term::apply(func, arg)),
                }
            }
            Term::Lambda { var, ty, body } => {
                let body = self.reduce(body)?;
                Ok(Term::lambda(var, *ty.clone(), body))
            }
            Term::Annotated { term, ty } => {
                let term = self.reduce(term)?;
                Ok(Term::annotated(term, *ty.clone()))
            }
            _ => Ok(term.clone()),
        }
    }

    fn is_normal_form(&self, term: &Term) -> bool {
        match term {
            Term::Apply { func, arg } => {
                // An application is in normal form if its function part is not a lambda
                // and both parts are in normal form
                !matches!(**func, Term::Lambda { .. })
                    && self.is_normal_form(func)
                    && self.is_normal_form(arg)
            }
            Term::Lambda { body, .. } => self.is_normal_form(body),
            Term::Annotated { term, .. } => self.is_normal_form(term),
            Term::Var(_) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_check_identity() {
        let stlc = SimplyTyped::new();
        
        // λx:Bool.x : Bool → Bool
        let id = Term::lambda(
            "x",
            Type::base("Bool"),
            Term::var("x"),
        );

        let ty = stlc.type_check(&id).unwrap();
        match ty {
            Type::Function { domain, codomain } => {
                assert_eq!(*domain, Type::base("Bool"));
                assert_eq!(*codomain, Type::base("Bool"));
            }
            _ => panic!("Expected function type"),
        }
    }

    #[test]
    fn test_type_check_application() {
        let stlc = SimplyTyped::new();

        // (λx:Bool.x) true
        let id = Term::lambda(
            "x",
            Type::base("Bool"),
            Term::var("x"),
        );
        let app = Term::apply(
            id,
            Term::annotated(Term::var("true"), Type::base("Bool")),
        );

        let ty = stlc.type_check(&app).unwrap();
        assert_eq!(ty, Type::base("Bool"));
    }

    #[test]
    fn test_type_check_error() {
        let stlc = SimplyTyped::new();

        // (λx:Bool.x) 42
        let id = Term::lambda(
            "x",
            Type::base("Bool"),
            Term::var("x"),
        );
        let app = Term::apply(
            id,
            Term::annotated(Term::var("42"), Type::base("Int")),
        );

        assert!(matches!(
            stlc.type_check(&app),
            Err(Error::TypeError(_))
        ));
    }

    #[test]
    fn test_reduction() {
        let stlc = SimplyTyped::new();

        // (λx:Bool.x) true → true
        let id = Term::lambda(
            "x",
            Type::base("Bool"),
            Term::var("x"),
        );
        let app = Term::apply(
            id,
            Term::annotated(Term::var("true"), Type::base("Bool")),
        );

        let reduced = stlc.reduce(&app).unwrap();
        assert_eq!(reduced, Term::annotated(Term::var("true"), Type::base("Bool")));
        assert!(stlc.is_normal_form(&reduced));
    }
}
