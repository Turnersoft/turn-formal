use super::context::ConstraintKind;
use super::context::Context;
use super::term::Term;
use super::type_::Type;
use super::universe::{Level, Universe};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub type TypeResult = Result<Rc<Type>, TypeError>;

#[derive(Debug)]
pub enum TypeError {
    UnboundVariable(String),
    TypeMismatch { expected: Rc<Type>, got: Rc<Type> },
    ConstructorError(String),
    UniverseError(String),
    WrongNumberOfArguments { expected: usize, got: usize },
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeError::UnboundVariable(var) => write!(f, "Unbound variable: {}", var),
            TypeError::TypeMismatch { expected, got } => {
                write!(f, "Type mismatch: expected {}, but got {}", expected, got)
            }
            TypeError::ConstructorError(msg) => write!(f, "Constructor error: {}", msg),
            TypeError::UniverseError(msg) => write!(f, "Universe error: {}", msg),
            TypeError::WrongNumberOfArguments { expected, got } => {
                write!(
                    f,
                    "Wrong number of arguments: expected {}, but got {}",
                    expected, got
                )
            }
        }
    }
}

pub trait TypeChecker {
    fn type_check(&self, ctx: &Context) -> TypeResult;
}

impl TypeChecker for Term {
    fn type_check(&self, ctx: &Context) -> TypeResult {
        match self {
            Term::Var(name) => {
                if let Some(ty) = ctx.lookup_type(name) {
                    // If it's a variable that refers to an inductive type, return just the base name
                    for (ind_name, _) in &ctx.inductives {
                        if ind_name == name {
                            return Ok(Rc::new(Type::Named(name.clone())));
                        }
                    }
                    return Ok(ty.clone());
                }
                Err(TypeError::UnboundVariable(name.clone()))
            }
            Term::App(f, arg) => {
                let f_type = f.type_check(ctx)?;
                let arg_type = arg.type_check(ctx)?;

                match &*f_type {
                    Type::Pi(x, param_type, return_type) => {
                        // Check universe constraints
                        ctx.check_type(param_type)?;
                        ctx.check_type(&arg_type)?;

                        // Check if the argument type is convertible with the parameter type
                        // considering universe constraints
                        if !arg_type.is_convertible_with_ctx(param_type, ctx)? {
                            // For universe polymorphism, we need to check if there's a valid constraint
                            match (&**param_type, &*arg_type) {
                                (Type::Type(param_level), Type::Type(arg_level)) => {
                                    let mut valid = param_level.0 == arg_level.0;
                                    // Check if there's a constraint that allows this application
                                    for constraint in &ctx.constraints {
                                        if constraint.left.0 == param_level.0
                                            && constraint.right.0 == arg_level.0
                                        {
                                            valid = true;
                                            break;
                                        }
                                    }
                                    if !valid {
                                        return Err(TypeError::TypeMismatch {
                                            expected: param_type.clone(),
                                            got: arg_type.clone(),
                                        });
                                    }
                                }
                                _ => {
                                    return Err(TypeError::TypeMismatch {
                                        expected: param_type.clone(),
                                        got: arg_type.clone(),
                                    });
                                }
                            }
                        }

                        // For type-level applications, we need to substitute the type argument
                        // and maintain the correct universe level
                        match (&**param_type, &*arg_type) {
                            (Type::Type(_), Type::Type(arg_level)) => {
                                let mut inner_ctx = ctx.clone();
                                inner_ctx.add_var(x.clone(), arg_type.clone());
                                let result = return_type.substitute_with_ctx(x, arg, &inner_ctx)?;
                                // For Type₀, stay at level 1
                                if arg_level.is_zero() {
                                    Ok(Rc::new(Type::Type(Level::new(1))))
                                } else {
                                    // Ensure the result type has the same universe level as the argument
                                    match &*result {
                                        Type::Type(_) => Ok(Rc::new(Type::Type(arg_level.clone()))),
                                        _ => Ok(result),
                                    }
                                }
                            }
                            _ => Ok(return_type.substitute_with_ctx(x, arg, ctx)?),
                        }
                    }
                    Type::Function(param_type, return_type) => {
                        // Check universe constraints
                        ctx.check_type(param_type)?;
                        ctx.check_type(&arg_type)?;

                        if !arg_type.is_convertible_with_ctx(param_type, ctx)? {
                            return Err(TypeError::TypeMismatch {
                                expected: param_type.clone(),
                                got: arg_type.clone(),
                            });
                        }
                        Ok(return_type.clone())
                    }
                    _ => Err(TypeError::ConstructorError(format!(
                        "Cannot apply non-function type {}",
                        f_type
                    ))),
                }
            }
            Term::Lambda(x, param_type, body) => {
                // Check that the parameter type is well-formed
                ctx.check_type(param_type)?;

                let mut inner_ctx = ctx.clone();
                inner_ctx.add_var(x.clone(), param_type.clone());
                let body_type = body.type_check(&inner_ctx)?;
                Ok(Rc::new(Type::Pi(x.clone(), param_type.clone(), body_type)))
            }
            Term::Pi(x, param_type, return_type) => {
                // Check that both types are well-formed
                ctx.check_type(param_type)?;
                let param_type_type = param_type.type_check(ctx)?;

                let mut inner_ctx = ctx.clone();
                inner_ctx.add_var(x.clone(), param_type.clone());
                let return_type_type = return_type.type_check(&inner_ctx)?;

                match (&*param_type_type, &*return_type_type) {
                    // Handle all Prop cases first - impredicative
                    (Type::Prop, _) | (_, Type::Prop) => {
                        // For impredicative Prop, always stay at level 1
                        Ok(Rc::new(Type::Type(Level::new(1))))
                    }
                    (Type::Type(l1), Type::Type(l2)) => {
                        // Check universe constraints
                        let mut valid = l1.0 <= l2.0; // By default, parameter level must be <= return level
                        for constraint in &ctx.constraints {
                            match constraint.kind {
                                ConstraintKind::Equal => {
                                    if (constraint.left.0 == l1.0 && constraint.right.0 == l2.0)
                                        || (constraint.left.0 == l2.0 && constraint.right.0 == l1.0)
                                    {
                                        valid = true;
                                        break;
                                    }
                                }
                                ConstraintKind::LessThan => {
                                    if constraint.left.0 == l1.0 && constraint.right.0 >= l2.0 {
                                        valid = true;
                                        break;
                                    }
                                }
                            }
                        }

                        if !valid {
                            return Err(TypeError::UniverseError(format!(
                                "Parameter type universe level {} must be less than or equal to return type universe level {}",
                                l1.0, l2.0
                            )));
                        }

                        // For template polymorphism (Type₀ -> Type₀), stay at level 1
                        if l1.0 == 0 && l2.0 == 0 {
                            Ok(Rc::new(Type::Type(Level::new(1))))
                        } else {
                            // Otherwise, use max(l1,l2) + 1 to maintain predicativity
                            Ok(Rc::new(Type::Type((*l1).max(*l2).succ())))
                        }
                    }
                    (Type::Type(l1), Type::Named(_)) => {
                        if l1.0 == 0 {
                            Ok(Rc::new(Type::Type(Level::new(1))))
                        } else {
                            // For polymorphic recursion, increment level
                            Ok(Rc::new(Type::Type((*l1).succ())))
                        }
                    }
                    (Type::Named(_), Type::Type(l2)) => {
                        if l2.0 == 0 {
                            Ok(Rc::new(Type::Type(Level::new(1))))
                        } else {
                            // For polymorphic functions with named parameter type,
                            // increment level
                            Ok(Rc::new(Type::Type((*l2).succ())))
                        }
                    }
                    (Type::Named(_), Type::Named(_)) => {
                        // Both types are variables, stay at level 1
                        Ok(Rc::new(Type::Type(Level::new(1))))
                    }
                    _ => Ok(Rc::new(Type::Type(Level::new(1)))), // Default to Type₁
                }
            }
            Term::Sort(universe) => match universe {
                Universe::Prop => Ok(Rc::new(Type::Type(Level::new(1)))),
                Universe::Type(level) => {
                    // For Type₀, stay at level 1
                    if level.is_zero() {
                        Ok(Rc::new(Type::Type(Level::new(1))))
                    } else {
                        Ok(Rc::new(Type::Type(level.succ())))
                    }
                }
            },
            Term::Number(_) => Ok(Rc::new(Type::Number)),
            Term::Bool(_) => Ok(Rc::new(Type::Bool)),
            Term::Unit => Ok(Rc::new(Type::Unit)),
            Term::Pair(t1, t2) => {
                let ty1 = t1.type_check(ctx)?;
                let ty2 = t2.type_check(ctx)?;
                Ok(Rc::new(Type::Product(ty1, ty2)))
            }
            Term::Left(t, right_ty) => {
                let left_ty = t.type_check(ctx)?;
                Ok(Rc::new(Type::Sum(left_ty, right_ty.clone())))
            }
            Term::Right(t, left_ty) => {
                let right_ty = t.type_check(ctx)?;
                Ok(Rc::new(Type::Sum(left_ty.clone(), right_ty)))
            }
            Term::Constructor(name, _) => {
                // For constructors of parameterized inductive types, return the base type name
                for (ind_name, ind_type) in ctx.inductives.iter() {
                    for ctor in &ind_type.constructors {
                        if &ctor.name == name {
                            return Ok(Rc::new(Type::Named(ind_name.clone())));
                        }
                    }
                }

                // Finally check if it's a definition
                if let Some(def) = ctx.definitions.get(name) {
                    return Ok(def.ty.clone());
                }

                // Return error for unbound variable
                Err(TypeError::UnboundVariable(name.clone()))
            }
            Term::Match(scrutinee, branches) => {
                let scrutinee_type = scrutinee.type_check(ctx)?;
                let mut result_type: Option<Rc<Type>> = None;

                for branch in branches {
                    let mut branch_ctx = ctx.clone();
                    // Add pattern variables to context
                    for var in &branch.pattern.bound_vars {
                        branch_ctx.add_var(var.clone(), scrutinee_type.clone());
                    }
                    let branch_type = branch.body.type_check(&branch_ctx)?;

                    if let Some(ref expected_type) = result_type {
                        if !branch_type.is_convertible_with_ctx(expected_type, ctx)? {
                            return Err(TypeError::TypeMismatch {
                                expected: expected_type.clone(),
                                got: branch_type,
                            });
                        }
                    } else {
                        result_type = Some(branch_type);
                    }
                }

                result_type.ok_or_else(|| {
                    TypeError::ConstructorError("Empty match expression".to_string())
                })
            }
        }
    }
}
