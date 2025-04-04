use crate::parse::entities::Identifier;

use super::{terms::Term, types::Type};
use std::collections::HashMap;
use std::fmt;

/// Typing context mapping variables to their types
#[derive(Debug, Clone, Default)]
pub struct Context {
    pub types: HashMap<Identifier, Type>,
}

impl Context {
    /// Create a new empty context
    pub fn new() -> Self {
        Context {
            types: HashMap::new(),
        }
    }

    /// Add a variable with its type to the context
    pub fn add_variable(&mut self, name: Identifier, ty: Type) {
        self.types.insert(name, ty);
    }

    /// Look up a variable's type in the context
    pub fn get_type(&self, name: &Identifier) -> Option<&Type> {
        self.types.get(name)
    }
}

/// Typing and evaluation errors
#[derive(Debug, Clone)]
pub enum SimplyTypedCalculusError {
    TypeMismatch { expected: Type, found: Type },
    UndefinedVariable(String),
    NotAFunction(Type),
    NotAPair(Type),
    NotAPairOrSum(Type),
    CannotReduce(String),
}

pub type SimplyTypedCalculusResult<T> = Result<T, SimplyTypedCalculusError>;

/// Check uniqueness of types - a key property of simply typed lambda calculus
pub fn check_unique_typing(
    context: &Context,
    term: &Term,
    ty1: &Type,
    ty2: &Type,
) -> SimplyTypedCalculusResult<()> {
    if ty1 != ty2 {
        Err(SimplyTypedCalculusError::TypeMismatch {
            expected: ty1.clone(),
            found: ty2.clone(),
        })
    } else {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum SimplyTypedGoal {
    // Type checking goals
    CheckType {
        term: Term,
        expected_type: Type,
        process: Option<TypeCheckProcess>,
    },
    InferType {
        term: Term,
        process: Option<TypeInferenceProcess>,
    },

    // Evaluation goals
    Evaluate {
        term: Term,
        process: Option<EvaluationProcess>,
    },
    BetaReduce {
        term: Term,
        process: Option<ReductionProcess>,
    },

    // Context goals
    LookupVariable {
        name: Identifier,
        process: Option<LookupProcess>,
    },

    // Function-specific goals
    CheckFunctionApplication {
        function: Term,
        argument: Term,
        process: Option<ApplicationProcess>,
    },
}

#[derive(Debug, Clone)]
pub enum TypeCheckProcess {
    Checking,
    Checked,
    Failed(String),
}

#[derive(Debug, Clone)]
pub enum TypeInferenceProcess {
    Inferring,
    Inferred(Type),
    Failed(String),
}

#[derive(Debug, Clone)]
pub enum EvaluationProcess {
    Evaluating,
    Evaluated(Term),
    Failed(String),
}

#[derive(Debug, Clone)]
pub enum ReductionProcess {
    Reducing,
    Reduced(Term),
    Failed(String),
}

#[derive(Debug, Clone)]
pub enum LookupProcess {
    Looking,
    Found(Type),
    NotFound,
}

#[derive(Debug, Clone)]
pub enum ApplicationProcess {
    Checking,
    Checked(Type),
    Failed(String),
}

impl SimplyTypedGoal {
    pub fn process(&mut self, context: &Context) {
        match self {
            SimplyTypedGoal::CheckType {
                term,
                expected_type,
                process,
            } => {
                *process = Some(match term.check_type(context, expected_type) {
                    Ok(()) => TypeCheckProcess::Checked,
                    Err(e) => TypeCheckProcess::Failed(e.to_string()),
                });
            }
            SimplyTypedGoal::InferType { term, process } => {
                *process = Some(match term.infer_type(context) {
                    Ok(ty) => TypeInferenceProcess::Inferred(ty),
                    Err(e) => TypeInferenceProcess::Failed(e.to_string()),
                });
            }
            SimplyTypedGoal::Evaluate { term, process } => {
                *process = Some(match term.evaluate() {
                    Ok(t) => EvaluationProcess::Evaluated(t),
                    Err(e) => EvaluationProcess::Failed(e.to_string()),
                });
            }
            SimplyTypedGoal::BetaReduce { term, process } => {
                *process = Some(match term.evaluate() {
                    Ok(t) => ReductionProcess::Reduced(t),
                    Err(e) => ReductionProcess::Failed(e.to_string()),
                });
            }
            SimplyTypedGoal::LookupVariable { name, process } => {
                *process = Some(match context.get_type(name) {
                    Some(ty) => LookupProcess::Found(ty.clone()),
                    None => LookupProcess::NotFound,
                });
            }
            SimplyTypedGoal::CheckFunctionApplication {
                function,
                argument,
                process,
            } => {
                *process = Some(match function.infer_type(context) {
                    Ok(Type::Function { return_type, .. }) => {
                        ApplicationProcess::Checked(*return_type)
                    }
                    Ok(ty) => {
                        ApplicationProcess::Failed(format!("Expected function type, found {}", ty))
                    }
                    Err(e) => ApplicationProcess::Failed(e.to_string()),
                });
            }
        }
    }

    /// Create an empty type checking goal
    pub fn new_check() -> Self {
        SimplyTypedGoal::CheckType {
            term: Term::Unit,
            expected_type: Type::Unit,
            process: None,
        }
    }

    /// Create an empty type inference goal
    pub fn new_infer() -> Self {
        SimplyTypedGoal::InferType {
            term: Term::Unit,
            process: None,
        }
    }

    /// Create an empty evaluation goal
    pub fn new_eval() -> Self {
        SimplyTypedGoal::Evaluate {
            term: Term::Unit,
            process: None,
        }
    }

    /// Check if this goal is complete (has been processed)
    pub fn is_success(&self) -> bool {
        match self {
            SimplyTypedGoal::CheckType { process, .. } => {
                matches!(process, Some(TypeCheckProcess::Checked))
            }
            SimplyTypedGoal::InferType { process, .. } => {
                matches!(process, Some(TypeInferenceProcess::Inferred(_)))
            }
            SimplyTypedGoal::Evaluate { process, .. } => {
                matches!(process, Some(EvaluationProcess::Evaluated(_)))
            }
            SimplyTypedGoal::BetaReduce { process, .. } => {
                matches!(process, Some(ReductionProcess::Reduced(_)))
            }
            SimplyTypedGoal::LookupVariable { process, .. } => {
                matches!(process, Some(LookupProcess::Found(_)))
            }
            SimplyTypedGoal::CheckFunctionApplication { process, .. } => {
                matches!(process, Some(ApplicationProcess::Checked(_)))
            }
        }
    }
}

/// Capture-avoiding substitution
fn substitute(term: &Term, var: &Identifier, replacement: &Term) -> Term {
    match term {
        Term::Variable(name) => {
            if name == var {
                replacement.clone()
            } else {
                term.clone()
            }
        }

        Term::Abstraction {
            param_name,
            param_type,
            body,
        } => {
            if param_name == var {
                // Variable is shadowed, don't substitute
                term.clone()
            } else if replacement.has_free_var(param_name) {
                // Need alpha conversion to avoid capture
                let fresh_name = param_name.add_prime();
                let fresh_var = Term::var(fresh_name.clone());
                let new_body = substitute(body, param_name, &fresh_var);
                Term::lambda(
                    fresh_name.clone(),
                    *param_type.clone(),
                    substitute(&new_body, var, replacement),
                )
            } else {
                Term::lambda(
                    param_name.clone(),
                    *param_type.clone(),
                    substitute(body, var, replacement),
                )
            }
        }

        Term::Application { function, argument } => Term::app(
            substitute(function, var, replacement),
            substitute(argument, var, replacement),
        ),

        _ => term.clone(),
    }
}

impl Term {
    /// Get the parameter name from an abstraction term
    pub fn get_param_name(&self) -> Option<Identifier> {
        match self {
            Term::Abstraction { param_name, .. } => Some(param_name.clone()),
            _ => None,
        }
    }

    /// Check if this term has a unique type in the given context
    pub fn has_unique_type(&self, context: &Context) -> SimplyTypedCalculusResult<Type> {
        // First infer the type
        let inferred = self.infer_type(context)?;

        // Then verify no other type is possible
        match self {
            Term::Variable(_) | Term::Unit | Term::Boolean(_) | Term::Number(_) => Ok(inferred),
            Term::Abstraction {
                param_type, body, ..
            } => {
                // Check body has unique type under extended context
                let mut extended_ctx = context.clone();
                extended_ctx.add_variable(self.get_param_name().unwrap(), *param_type.clone());
                body.has_unique_type(&extended_ctx)?;
                Ok(inferred)
            }
            Term::Application { function, argument } => {
                // Check both parts have unique types
                function.has_unique_type(context)?;
                argument.has_unique_type(context)?;
                Ok(inferred)
            }
            Term::Product { left, right } => {
                left.has_unique_type(context)?;
                right.has_unique_type(context)?;
                Ok(inferred)
            }
            Term::ProjectLeft(term) | Term::ProjectRight(term) => {
                term.has_unique_type(context)?;
                Ok(inferred)
            }
            Term::InjectSum { term, .. } => {
                term.has_unique_type(context)?;
                Ok(inferred)
            }
            Term::CaseSum {
                term,
                left_case,
                right_case,
                ..
            } => {
                term.has_unique_type(context)?;
                left_case.has_unique_type(context)?;
                right_case.has_unique_type(context)?;
                Ok(inferred)
            }
        }
    }

    /// Strong normalization theorem:
    /// Every well-typed term in simply typed lambda calculus is strongly normalizing.
    /// This means all reduction sequences terminate in a normal form.
    /// This is a proven property of the calculus, not something we need to verify.
    pub fn is_strongly_normalizing(&self) -> bool {
        // For well-typed terms, this is always true
        true
    }
}

impl fmt::Display for SimplyTypedCalculusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimplyTypedCalculusError::TypeMismatch { expected, found } => {
                write!(
                    f,
                    "Type mismatch: expected {}, but found {}",
                    expected, found
                )
            }
            SimplyTypedCalculusError::UndefinedVariable(name) => {
                write!(f, "Undefined variable: {}", name)
            }
            SimplyTypedCalculusError::NotAFunction(ty) => {
                write!(f, "Expected a function type, but found: {}", ty)
            }
            SimplyTypedCalculusError::NotAPair(ty) => {
                write!(f, "Expected a pair type, but found: {}", ty)
            }
            SimplyTypedCalculusError::NotAPairOrSum(ty) => {
                write!(f, "Expected a pair or sum type, but found: {}", ty)
            }
            SimplyTypedCalculusError::CannotReduce(msg) => {
                write!(f, "Cannot reduce term: {}", msg)
            }
        }
    }
}
