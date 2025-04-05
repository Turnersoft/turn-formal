use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::subjects::math::theories::number_theory::definitions::Number;

use super::{
    goals::{Context, SimplyTypedCalculusError, SimplyTypedCalculusResult},
    types::Type,
};

/// Side of a sum type injection
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SumSide {
    Left,
    Right,
}

impl fmt::Display for SumSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SumSide::Left => write!(f, "left"),
            SumSide::Right => write!(f, "right"),
        }
    }
}

/// Terms in Simply Typed Lambda Calculus
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Term {
    /// Variables
    Variable(String),

    /// Lambda abstraction (λx:T.M)
    Abstraction {
        /// Parameter name
        param_name: String,
        /// Parameter type
        param_type: Box<Type>,
        /// Function body
        body: Box<Term>,
    },

    /// Function application (M N)
    Application {
        /// Function term
        function: Box<Term>,
        /// Argument term
        argument: Box<Term>,
    },

    /// Product introduction (M, N)
    Product {
        left: Box<Term>,
        right: Box<Term>,
    },

    /// Product elimination - left projection
    ProjectLeft(Box<Term>),

    /// Product elimination - right projection
    ProjectRight(Box<Term>),

    /// Sum type injection
    InjectSum {
        /// Term to inject
        term: Box<Term>,
        /// Target sum type to inject into
        target_type: Box<Type>,
        /// Which side to inject into
        side: SumSide,
    },

    /// Sum type elimination via case analysis
    CaseSum {
        /// Term to analyze
        term: Box<Term>,
        /// Left case branch
        left_case: Box<Term>,
        /// Right case branch
        right_case: Box<Term>,
        /// Result type annotation
        result_type: Box<Type>,
    },

    /// Base type terms
    Boolean(bool),
    Number(Number),
    Unit,
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Variable(name) => write!(f, "{}", name.to_string()),
            Term::Abstraction {
                param_name,
                param_type,
                body,
            } => write!(f, "λ{}:{}. {}", param_name.to_string(), param_type, body),
            Term::Application { function, argument } => write!(f, "({} {})", function, argument),
            Term::Product { left, right } => write!(f, "({}, {})", left, right),
            Term::ProjectLeft(term) => write!(f, "π₁({})", term),
            Term::ProjectRight(term) => write!(f, "π₂({})", term),
            Term::InjectSum {
                term,
                target_type,
                side,
            } => write!(f, "inj_{}<{}>({})", side, target_type, term),
            Term::CaseSum {
                term,
                left_case,
                right_case,
                result_type,
            } => write!(
                f,
                "case {} of left => {} | right => {} : {}",
                term, left_case, right_case, result_type
            ),
            Term::Boolean(b) => write!(f, "{}", b),
            Term::Number(n) => write!(f, "{}", n.to_string()),
            Term::Unit => write!(f, "()"),
        }
    }
}

impl Term {
    /// Create a new variable term
    pub fn var(name: String) -> Self {
        Term::Variable(name)
    }

    /// Create a new application
    pub fn app(function: Term, argument: Term) -> Self {
        Term::Application {
            function: Box::new(function),
            argument: Box::new(argument),
        }
    }

    /// Get the parameter name if this is an abstraction
    pub fn param_name(&self) -> Option<&String> {
        match self {
            Term::Abstraction { param_name, .. } => Some(param_name),
            _ => None,
        }
    }

    /// Beta reduce a term once
    pub fn beta_reduce(&mut self) -> SimplyTypedCalculusResult<bool> {
        match self {
            Term::Application { function, argument } => {
                if let Term::Abstraction {
                    param_name, body, ..
                } = &**function
                {
                    let mut new_body = (**body).clone();
                    new_body.substitute(param_name, argument);
                    *self = new_body;
                    Ok(true)
                } else {
                    let mut reduced = false;
                    if function.beta_reduce()? {
                        reduced = true;
                    }
                    if argument.beta_reduce()? {
                        reduced = true;
                    }
                    Ok(reduced)
                }
            }
            Term::Abstraction { body, .. } => body.beta_reduce(),
            Term::Product { left, right } => {
                let mut reduced = false;
                if left.beta_reduce()? {
                    reduced = true;
                }
                if right.beta_reduce()? {
                    reduced = true;
                }
                Ok(reduced)
            }
            Term::ProjectLeft(term) | Term::ProjectRight(term) => term.beta_reduce(),
            Term::InjectSum { term, .. } => term.beta_reduce(),
            Term::CaseSum {
                term,
                left_case,
                right_case,
                ..
            } => {
                let mut reduced = false;
                if term.beta_reduce()? {
                    reduced = true;
                }
                if left_case.beta_reduce()? {
                    reduced = true;
                }
                if right_case.beta_reduce()? {
                    reduced = true;
                }
                Ok(reduced)
            }
            _ => Ok(false),
        }
    }

    /// Apply this term to an argument
    pub fn apply(&mut self, argument: Term) -> SimplyTypedCalculusResult<()> {
        *self = Term::Application {
            function: Box::new(self.clone()),
            argument: Box::new(argument),
        };
        Ok(())
    }

    /// Create a new lambda abstraction
    pub fn lambda(param_name: String, param_type: Type, body: Term) -> Self {
        Term::Abstraction {
            param_name,
            param_type: Box::new(param_type),
            body: Box::new(body),
        }
    }

    /// Create a new product term
    pub fn product(left: Term, right: Term) -> Self {
        Term::Product {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create a new sum injection
    pub fn inject_sum(term: Term, target_type: Type, side: SumSide) -> Self {
        Term::InjectSum {
            term: Box::new(term),
            target_type: Box::new(target_type),
            side,
        }
    }

    /// Create a new case analysis
    pub fn case_sum(term: Term, left_case: Term, right_case: Term, result_type: Type) -> Self {
        Term::CaseSum {
            term: Box::new(term),
            left_case: Box::new(left_case),
            right_case: Box::new(right_case),
            result_type: Box::new(result_type),
        }
    }

    /// Check if a term has a free variable
    pub fn has_free_var(&self, var_name: &String) -> bool {
        match self {
            Term::Variable(name) => name == var_name,
            Term::Abstraction {
                param_name, body, ..
            } => param_name != var_name && body.has_free_var(var_name),
            Term::Application { function, argument } => {
                function.has_free_var(var_name) || argument.has_free_var(var_name)
            }
            Term::Product { left, right } => {
                left.has_free_var(var_name) || right.has_free_var(var_name)
            }
            Term::ProjectLeft(term) | Term::ProjectRight(term) => term.has_free_var(var_name),
            Term::InjectSum { term, .. } => term.has_free_var(var_name),
            Term::CaseSum {
                term,
                left_case,
                right_case,
                ..
            } => {
                term.has_free_var(var_name)
                    || left_case.has_free_var(var_name)
                    || right_case.has_free_var(var_name)
            }
            Term::Boolean(_) | Term::Number(_) | Term::Unit => false,
        }
    }

    /// Substitute a variable with a term
    pub fn substitute(&mut self, var: &String, replacement: &Term) {
        match self {
            Term::Variable(name) => {
                if name == var {
                    *self = replacement.clone();
                }
            }
            Term::Abstraction {
                param_name,
                param_type: _,
                body,
            } => {
                if param_name != var {
                    body.substitute(var, replacement);
                }
            }
            Term::Application { function, argument } => {
                function.substitute(var, replacement);
                argument.substitute(var, replacement);
            }
            Term::Product { left, right } => {
                left.substitute(var, replacement);
                right.substitute(var, replacement);
            }
            Term::ProjectLeft(term) | Term::ProjectRight(term) => {
                term.substitute(var, replacement);
            }
            Term::InjectSum {
                term,
                target_type: _,
                side: _,
            } => {
                term.substitute(var, replacement);
            }
            Term::CaseSum {
                term,
                left_case,
                right_case,
                result_type: _,
            } => {
                term.substitute(var, replacement);
                left_case.substitute(var, replacement);
                right_case.substitute(var, replacement);
            }
            Term::Boolean(_) | Term::Number(_) | Term::Unit => {}
        }
    }

    /// Evaluate a term to normal form
    pub fn evaluate(&self) -> SimplyTypedCalculusResult<Term> {
        match self {
            Term::Variable(_) => Ok(self.clone()),
            Term::Abstraction { .. } => Ok(self.clone()),
            Term::Application { function, argument } => {
                let eval_func = function.evaluate()?;
                let eval_arg = argument.evaluate()?;
                match eval_func {
                    Term::Abstraction {
                        param_name, body, ..
                    } => {
                        let mut new_body = (*body).clone();
                        new_body.substitute(&param_name, &eval_arg);
                        new_body.evaluate()
                    }
                    _ => Ok(Term::Application {
                        function: Box::new(eval_func),
                        argument: Box::new(eval_arg),
                    }),
                }
            }
            Term::Product { left, right } => {
                let eval_left = left.evaluate()?;
                let eval_right = right.evaluate()?;
                Ok(Term::Product {
                    left: Box::new(eval_left),
                    right: Box::new(eval_right),
                })
            }
            Term::ProjectLeft(term) => {
                let eval_term = term.evaluate()?;
                match eval_term {
                    Term::Product { left, .. } => Ok(*left),
                    _ => Ok(Term::ProjectLeft(Box::new(eval_term))),
                }
            }
            Term::ProjectRight(term) => {
                let eval_term = term.evaluate()?;
                match eval_term {
                    Term::Product { right, .. } => Ok(*right),
                    _ => Ok(Term::ProjectRight(Box::new(eval_term))),
                }
            }
            Term::InjectSum {
                term,
                target_type,
                side,
            } => {
                let eval_term = term.evaluate()?;
                Ok(Term::InjectSum {
                    term: Box::new(eval_term),
                    target_type: target_type.clone(),
                    side: side.clone(),
                })
            }
            Term::CaseSum {
                term,
                left_case,
                right_case,
                result_type,
            } => {
                let eval_term = term.evaluate()?;
                match eval_term {
                    Term::InjectSum { term, side, .. } => match side {
                        SumSide::Left => {
                            let mut new_case = left_case.clone();
                            new_case.substitute(&String::from("x"), &term);
                            new_case.evaluate()
                        }
                        SumSide::Right => {
                            let mut new_case = right_case.clone();
                            new_case.substitute(&String::from("x"), &term);
                            new_case.evaluate()
                        }
                    },
                    _ => Ok(Term::CaseSum {
                        term: Box::new(eval_term),
                        left_case: left_case.clone(),
                        right_case: right_case.clone(),
                        result_type: result_type.clone(),
                    }),
                }
            }
            Term::Boolean(_) | Term::Number(_) | Term::Unit => Ok(self.clone()),
        }
    }

    /// Type check a term against a given type
    pub fn check_type(&self, context: &Context, expected: &Type) -> SimplyTypedCalculusResult<()> {
        let inferred = self.infer_type(context)?;
        if inferred == *expected {
            Ok(())
        } else {
            Err(SimplyTypedCalculusError::TypeMismatch {
                expected: expected.clone(),
                found: inferred,
            })
        }
    }

    /// Infer the type of a term
    pub fn infer_type(&self, context: &Context) -> SimplyTypedCalculusResult<Type> {
        match self {
            Term::Variable(name) => context
                .get_type(name)
                .cloned()
                .ok_or_else(|| SimplyTypedCalculusError::UndefinedVariable(name.to_string())),
            Term::Abstraction {
                param_type,
                param_name,
                body,
            } => {
                let mut ctx = context.clone();
                ctx.add_variable(param_name.clone(), *param_type.clone());
                let return_type = body.infer_type(&ctx)?;
                Ok(Type::Function {
                    param_type: param_type.clone(),
                    return_type: Box::new(return_type),
                })
            }
            Term::Application { function, argument } => {
                let func_type = function.infer_type(context)?;
                if let Type::Function {
                    param_type,
                    return_type,
                } = func_type
                {
                    argument.check_type(context, &param_type)?;
                    Ok(*return_type)
                } else {
                    Err(SimplyTypedCalculusError::NotAFunction(func_type))
                }
            }
            Term::Product { left, right } => {
                let left_type = left.infer_type(context)?;
                let right_type = right.infer_type(context)?;
                Ok(Type::Product {
                    left: Box::new(left_type),
                    right: Box::new(right_type),
                })
            }
            Term::ProjectLeft(term) => {
                let term_type = term.infer_type(context)?;
                if let Type::Product { left, .. } = term_type {
                    Ok(*left)
                } else {
                    Err(SimplyTypedCalculusError::NotAPairOrSum(term_type))
                }
            }
            Term::ProjectRight(term) => {
                let term_type = term.infer_type(context)?;
                if let Type::Product { right, .. } = term_type {
                    Ok(*right)
                } else {
                    Err(SimplyTypedCalculusError::NotAPairOrSum(term_type))
                }
            }
            Term::InjectSum {
                term,
                target_type,
                side,
            } => {
                let _term_type = term.infer_type(context)?;
                match target_type.as_ref() {
                    Type::Sum { left, right } => match side {
                        SumSide::Left => {
                            term.check_type(context, left)?;
                            Ok(*target_type.clone())
                        }
                        SumSide::Right => {
                            term.check_type(context, right)?;
                            Ok(*target_type.clone())
                        }
                    },
                    _ => Err(SimplyTypedCalculusError::NotAPairOrSum(
                        *target_type.clone(),
                    )),
                }
            }
            Term::CaseSum {
                term,
                left_case,
                right_case,
                result_type,
            } => {
                let term_type = term.as_ref().infer_type(context)?;
                if let Type::Sum { left, right } = term_type {
                    left_case.as_ref().check_type(context, &left)?;
                    right_case.as_ref().check_type(context, &right)?;
                    Ok(*result_type.clone())
                } else {
                    Err(SimplyTypedCalculusError::NotAPairOrSum(term_type))
                }
            }
            Term::Boolean(_) => Ok(Type::Bool),
            Term::Number(_) => Ok(Type::Number),
            Term::Unit => Ok(Type::Unit),
        }
    }
}
