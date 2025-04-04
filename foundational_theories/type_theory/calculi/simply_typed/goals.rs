use crate::formalize_v2::foundational_theories::type_theory::{
    calculi::simply_typed::{term::Term, context::Context},
    context::Environment,
};
use serde::{Deserialize, Serialize};

/// Goal intention for Simply Typed Lambda Calculus operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimplyTypedGoalIntention {
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

    // Beta reduction goals
    BetaReduce {
        term: Term,
        process: Option<ReductionProcess>,
    },
    NormalForm {
        term: Term,
        process: Option<NormalizationProcess>,
    },

    // Context and environment goals
    FindVariable {
        name: String,
        process: Option<LookupProcess>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimplyTypedGoal {
    pub id: String,
    pub intention: SimplyTypedGoalIntention,
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
    ApplicationError { message: String },
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
pub enum ReductionProcess {
    Success { reduced_term: Term, steps: Vec<Term> },
    Error(ReductionError),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReductionError {
    StuckTerm { term: Term },
    MaxStepsExceeded { last_term: Term },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizationProcess {
    Success(Term),
    Error(NormalizationError),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizationError {
    NoNormalForm { term: Term },
    DivergentReduction { message: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LookupProcess {
    Found(Term),
    NotFound(String),
}

impl SimplyTypedGoal {
    pub fn nothing() -> Self {
        SimplyTypedGoal {
            id: String::new(),
            intention: SimplyTypedGoalIntention::Nothing,
            context: Environment::new(),
        }
    }

    pub fn is_nothing(&self) -> bool {
        matches!(self.intention, SimplyTypedGoalIntention::Nothing)
    }

    pub fn is_success(&self) -> bool {
        use SimplyTypedGoalIntention::*;
        match &self.intention {
            Nothing => true,
            TypeCheck { process: Some(p), .. } => matches!(p, TypeCheckProcess::Success(_)),
            TypeInference { process: Some(p), .. } => matches!(p, TypeInferenceProcess::Success { .. }),
            BetaReduce { process: Some(p), .. } => matches!(p, ReductionProcess::Success { .. }),
            NormalForm { process: Some(p), .. } => matches!(p, NormalizationProcess::Success(_)),
            FindVariable { process: Some(p), .. } => matches!(p, LookupProcess::Found(_)),
            _ => false,
        }
    }

    pub fn process(&mut self) -> SimplyTypedGoal {
        use SimplyTypedGoalIntention::*;
        match &self.intention {
            TypeCheck { term, expected_type, process: None } => {
                // Implement type checking logic
                todo!()
            },
            BetaReduce { term, process: None } => {
                // Implement beta reduction logic
                todo!()
            },
            // Implement other cases
            _ => self.clone(),
        }
    }
}

// Helper methods for creating goals
impl SimplyTypedGoal {
    pub fn type_check(id: impl Into<String>, term: Term, expected_type: Option<Term>, context: Environment) -> Self {
        SimplyTypedGoal {
            id: id.into(),
            intention: SimplyTypedGoalIntention::TypeCheck {
                term,
                expected_type,
                process: None,
            },
            context,
        }
    }

    pub fn beta_reduce(id: impl Into<String>, term: Term, context: Environment) -> Self {
        SimplyTypedGoal {
            id: id.into(),
            intention: SimplyTypedGoalIntention::BetaReduce {
                term,
                process: None,
            },
            context,
        }
    }

    pub fn normal_form(id: impl Into<String>, term: Term, context: Environment) -> Self {
        SimplyTypedGoal {
            id: id.into(),
            intention: SimplyTypedGoalIntention::NormalForm {
                term,
                process: None,
            },
            context,
        }
    }
}
