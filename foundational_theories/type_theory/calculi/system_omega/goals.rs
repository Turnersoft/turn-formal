use crate::formalize_v2::foundational_theories::type_theory::{
    calculi::system_omega::{term::Term, context::Context},
    context::Environment,
};
use serde::{Deserialize, Serialize};

/// Goal intention for System Omega operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemOmegaGoalIntention {
    Nothing,

    // Type checking and kinding goals
    TypeCheck {
        term: Term,
        expected_type: Option<Term>,
        process: Option<TypeCheckProcess>,
    },
    KindCheck {
        type_term: Term,
        expected_kind: Option<Term>,
        process: Option<KindCheckProcess>,
    },
    TypeInference {
        term: Term,
        process: Option<TypeInferenceProcess>,
    },
    KindInference {
        type_term: Term,
        process: Option<KindInferenceProcess>,
    },

    // Type operator goals
    TypeOperatorApplication {
        operator: Term,
        argument: Term,
        process: Option<TypeOperatorProcess>,
    },
    TypeOperatorAbstraction {
        variable: String,
        kind: Term,
        body: Term,
        process: Option<TypeOperatorProcess>,
    },

    // Context and environment goals
    FindVariable {
        name: String,
        process: Option<LookupProcess>,
    },
    FindTypeOperator {
        name: String,
        process: Option<TypeOperatorLookupProcess>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemOmegaGoal {
    pub id: String,
    pub intention: SystemOmegaGoalIntention,
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
    KindError { message: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KindCheckProcess {
    Success(Term),
    Error(KindCheckError),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KindCheckError {
    KindMismatch { expected: Term, found: Term },
    InvalidTypeOperator { message: String },
    UnboundTypeOperator { name: String },
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KindInferenceProcess {
    Success { type_term: Term, inferred_kind: Term },
    Error(KindInferenceError),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KindInferenceError {
    CannotInfer { type_term: Term },
    UnboundTypeOperator { name: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeOperatorProcess {
    Success(Term),
    Error(TypeOperatorError),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeOperatorError {
    KindMismatch { expected: Term, found: Term },
    InvalidApplication { message: String },
    UnboundTypeOperator { name: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LookupProcess {
    Found(Term),
    NotFound(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeOperatorLookupProcess {
    Found { operator: Term, kind: Term },
    NotFound(String),
}

impl SystemOmegaGoal {
    pub fn nothing() -> Self {
        SystemOmegaGoal {
            id: String::new(),
            intention: SystemOmegaGoalIntention::Nothing,
            context: Environment::new(),
        }
    }

    pub fn is_nothing(&self) -> bool {
        matches!(self.intention, SystemOmegaGoalIntention::Nothing)
    }

    pub fn is_success(&self) -> bool {
        use SystemOmegaGoalIntention::*;
        match &self.intention {
            Nothing => true,
            TypeCheck { process: Some(p), .. } => matches!(p, TypeCheckProcess::Success(_)),
            KindCheck { process: Some(p), .. } => matches!(p, KindCheckProcess::Success(_)),
            TypeInference { process: Some(p), .. } => matches!(p, TypeInferenceProcess::Success { .. }),
            KindInference { process: Some(p), .. } => matches!(p, KindInferenceProcess::Success { .. }),
            TypeOperatorApplication { process: Some(p), .. } => matches!(p, TypeOperatorProcess::Success(_)),
            TypeOperatorAbstraction { process: Some(p), .. } => matches!(p, TypeOperatorProcess::Success(_)),
            FindVariable { process: Some(p), .. } => matches!(p, LookupProcess::Found(_)),
            FindTypeOperator { process: Some(p), .. } => matches!(p, TypeOperatorLookupProcess::Found { .. }),
            _ => false,
        }
    }

    pub fn process(&mut self) -> SystemOmegaGoal {
        use SystemOmegaGoalIntention::*;
        match &self.intention {
            TypeCheck { term, expected_type, process: None } => {
                // Implement type checking logic
                todo!()
            },
            KindCheck { type_term, expected_kind, process: None } => {
                // Implement kind checking logic
                todo!()
            },
            // Implement other cases
            _ => self.clone(),
        }
    }
}

// Helper methods for creating goals
impl SystemOmegaGoal {
    pub fn type_check(id: impl Into<String>, term: Term, expected_type: Option<Term>, context: Environment) -> Self {
        SystemOmegaGoal {
            id: id.into(),
            intention: SystemOmegaGoalIntention::TypeCheck {
                term,
                expected_type,
                process: None,
            },
            context,
        }
    }

    pub fn kind_check(id: impl Into<String>, type_term: Term, expected_kind: Option<Term>, context: Environment) -> Self {
        SystemOmegaGoal {
            id: id.into(),
            intention: SystemOmegaGoalIntention::KindCheck {
                type_term,
                expected_kind,
                process: None,
            },
            context,
        }
    }

    pub fn type_operator_application(
        id: impl Into<String>,
        operator: Term,
        argument: Term,
        context: Environment,
    ) -> Self {
        SystemOmegaGoal {
            id: id.into(),
            intention: SystemOmegaGoalIntention::TypeOperatorApplication {
                operator,
                argument,
                process: None,
            },
            context,
        }
    }
}
