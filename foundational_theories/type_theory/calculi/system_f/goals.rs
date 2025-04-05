use crate::foundational_theories::type_theory::{
    calculi::system_f::{term::Term, context::Context},
    context::Environment,
};
use serde::{Deserialize, Serialize};

/// Goal intention for System F operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemFGoalIntention {
    Nothing,

    // Type checking goals
    TypeCheck {
        term: Term,
        expected_type: Option<Term>,
        process: Option<TypeCheckProcess>,
    },
    TypeInference {
        term: Term,
        process: Option<TypeInferenceProcess>,
    },

    // Type abstraction goals
    AbstractType {
        term: Term,
        type_var: String,
        process: Option<AbstractionProcess>,
    },
    InstantiateType {
        term: Term,
        type_arg: Term,
        process: Option<InstantiationProcess>,
    },

    // Context and environment goals
    FindVariable {
        name: String,
        process: Option<LookupProcess>,
    },
    FindTypeVariable {
        name: String,
        process: Option<TypeLookupProcess>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemFGoal {
    pub id: String,
    pub intention: SystemFGoalIntention,
    pub context: Environment,
}

// Process types for each operation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeCheckProcess {
    Success(Term),
    Error(TypeCheckError),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeCheckError {
    TypeMismatch { expected: Term, found: Term },
    UnboundVariable { name: String },
    UnboundTypeVariable { name: String },
    KindError { message: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeInferenceProcess {
    Success { term: Term, inferred_type: Term },
    Error(TypeInferenceError),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeInferenceError {
    CannotInfer { term: Term },
    UnboundVariable { name: String },
    UnboundTypeVariable { name: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbstractionProcess {
    Success(Term),
    Error(AbstractionError),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbstractionError {
    TypeVariableInUse { name: String },
    InvalidTerm { message: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstantiationProcess {
    Success(Term),
    Error(InstantiationError),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstantiationError {
    NotATypeAbstraction { term: Term },
    InvalidTypeArgument { message: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LookupProcess {
    Found(Term),
    NotFound(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeLookupProcess {
    Found(Term),
    NotFound(String),
}

impl SystemFGoal {
    pub fn nothing() -> Self {
        SystemFGoal {
            id: String::new(),
            intention: SystemFGoalIntention::Nothing,
            context: Environment::new(),
        }
    }

    pub fn is_nothing(&self) -> bool {
        matches!(self.intention, SystemFGoalIntention::Nothing)
    }

    pub fn is_success(&self) -> bool {
        use SystemFGoalIntention::*;
        match &self.intention {
            Nothing => true,
            TypeCheck { process: Some(p), .. } => matches!(p, TypeCheckProcess::Success(_)),
            TypeInference { process: Some(p), .. } => matches!(p, TypeInferenceProcess::Success { .. }),
            AbstractType { process: Some(p), .. } => matches!(p, AbstractionProcess::Success(_)),
            InstantiateType { process: Some(p), .. } => matches!(p, InstantiationProcess::Success(_)),
            FindVariable { process: Some(p), .. } => matches!(p, LookupProcess::Found(_)),
            FindTypeVariable { process: Some(p), .. } => matches!(p, TypeLookupProcess::Found(_)),
            _ => false,
        }
    }

    pub fn process(&mut self) -> SystemFGoal {
        use SystemFGoalIntention::*;
        match &self.intention {
            TypeCheck { term, expected_type, process: None } => {
                // Implement type checking logic
                todo!()
            },
            TypeInference { term, process: None } => {
                // Implement type inference logic
                todo!()
            },
            // Implement other cases
            _ => self.clone(),
        }
    }
}

// Helper methods for creating goals
impl SystemFGoal {
    pub fn type_check(id: impl Into<String>, term: Term, expected_type: Option<Term>, context: Environment) -> Self {
        SystemFGoal {
            id: id.into(),
            intention: SystemFGoalIntention::TypeCheck {
                term,
                expected_type,
                process: None,
            },
            context,
        }
    }

    pub fn type_inference(id: impl Into<String>, term: Term, context: Environment) -> Self {
        SystemFGoal {
            id: id.into(),
            intention: SystemFGoalIntention::TypeInference {
                term,
                process: None,
            },
            context,
        }
    }

    pub fn abstract_type(id: impl Into<String>, term: Term, type_var: impl Into<String>, context: Environment) -> Self {
        SystemFGoal {
            id: id.into(),
            intention: SystemFGoalIntention::AbstractType {
                term,
                type_var: type_var.into(),
                process: None,
            },
            context,
        }
    }
}
