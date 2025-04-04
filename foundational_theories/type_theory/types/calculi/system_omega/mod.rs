use crate::formalize_v2::foundational_theories::type_theory::{
    core::{Result, Term, Context, Error},
    types::{TypeConstructor, calculi::CalculusType},
};
use serde::{Deserialize, Serialize};

/// System Omega type constructors and kinds
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemOmegaType {
    // Base types
    Unit,
    Bool,
    Natural,
    
    // Type operators
    TypeOperator {
        param_name: String,
        param_kind: Box<Kind>,
        body_type: Box<Term>,
    },
    TypeApplication {
        operator: Box<Term>,
        argument: Box<Term>,
    },
    
    // Function types
    FunctionType {
        param_type: Box<Term>,
        return_type: Box<Term>,
    },
    
    // Kind-level constructs
    KindVariable(String),
    KindOperator {
        param_name: String,
        param_kind: Box<Kind>,
        body_kind: Box<Kind>,
    },
    
    // Universal types
    Universal {
        param_name: String,
        param_kind: Box<Kind>,
        body_type: Box<Term>,
    },
    
    // Type variables
    TypeVariable {
        name: String,
        kind: Box<Kind>,
    },
}

/// Kinds in System Omega
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Kind {
    // Base kind (type)
    Type,
    
    // Higher-order kinds
    Arrow {
        param_kind: Box<Kind>,
        return_kind: Box<Kind>,
    },
    
    // Dependent kinds
    DependentArrow {
        param_name: String,
        param_kind: Box<Kind>,
        return_kind: Box<Kind>,
    },
    
    // Universe kinds
    Universe { level: usize },
}

impl TypeConstructor for SystemOmegaType {
    fn check_term(&self, term: &Term) -> Result<()> {
        match self {
            SystemOmegaType::TypeOperator { param_name, param_kind, body_type } => {
                // Check param_kind is valid
                param_kind.check_kind()?;
                
                // Check body_type in extended context with type operator
                let mut context = Context::new();
                context.add_type_operator(param_name.clone(), param_kind.clone())?;
                body_type.check_type_in_context(&context)
            },
            SystemOmegaType::TypeApplication { operator, argument } => {
                // Check operator and argument are valid
                operator.check_type()?;
                argument.check_type()?;
                
                // Check kinds match for application
                let op_kind = operator.get_kind()?;
                let arg_kind = argument.get_kind()?;
                
                match op_kind {
                    Term::Kind { from_kind, to_kind } => {
                        if from_kind != arg_kind {
                            Err(Error::KindMismatch {
                                expected: from_kind,
                                found: arg_kind,
                            })
                        } else {
                            Ok(())
                        }
                    },
                    _ => Err(Error::InvalidKind("Type operator must have function kind".into())),
                }
            },
            SystemOmegaType::TypeVariable { name, kind } => {
                // Check kind is valid
                kind.check_kind()?;
                
                // Check variable is in context with matching kind
                if let Some(ctx_kind) = Context::current().get_type_variable_kind(name) {
                    if &ctx_kind != kind {
                        Err(Error::KindMismatch {
                            expected: ctx_kind,
                            found: kind.clone(),
                        })
                    } else {
                        Ok(())
                    }
                } else {
                    Err(Error::UndefinedTypeVariable(name.clone()))
                }
            },
            SystemOmegaType::Universal { param_name, param_kind, body_type } => {
                // Check param_kind is valid
                param_kind.check_kind()?;
                
                // Check body_type in extended context
                let mut context = Context::new();
                context.add_type_variable(param_name.clone())?;
                body_type.check_type_in_context(&context)
            },
            SystemOmegaType::FunctionType { param_type, return_type } => {
                // Check both types and ensure they have base kind *
                param_type.check_type()?;
                return_type.check_type()?;
                
                if !param_type.has_base_kind()? || !return_type.has_base_kind()? {
                    Err(Error::InvalidKind("Function types must have base kind *".into()))
                } else {
                    Ok(())
                }
            },
            _ => Ok(()),
        }
    }

    fn universe_level(&self) -> usize {
        match self {
            SystemOmegaType::TypeOperator { body_type, .. } => {
                // Type operators live one level above their body
                body_type.universe_level() + 1
            },
            SystemOmegaType::TypeApplication { operator, argument } => {
                // Take max of operator and argument levels
                operator.universe_level().max(argument.universe_level())
            },
            SystemOmegaType::TypeVariable { kind, .. } => {
                // Variables inherit level from their kind
                kind.universe_level()
            },
            SystemOmegaType::Universal { body_type, .. } => {
                // Universal types live in same universe as body
                body_type.universe_level()
            },
            SystemOmegaType::FunctionType { param_type, return_type } => {
                // Take max of component type levels
                param_type.universe_level().max(return_type.universe_level())
            },
            _ => 0,
        }
    }
}

impl CalculusType for SystemOmegaType {
    fn calculus_name() -> &'static str {
        "System Omega"
    }
    
    fn is_valid_in_calculus(&self, term: &Term) -> Result<()> {
        match self {
            SystemOmegaType::TypeOperator { param_kind, body_type, .. } => {
                // Check param_kind is well-formed in System Omega
                if param_kind.universe_level() >= self.max_kind_level() {
                    Err(Error::InvalidInCalculus {
                        calculus: Self::calculus_name(),
                        message: "Kind level exceeds System Omega limit".into(),
                    })
                } else {
                    // Check body_type is valid in System Omega
                    body_type.is_valid_in_calculus(term)
                }
            },
            SystemOmegaType::TypeApplication { operator, argument } => {
                // Check both parts are valid in System Omega
                operator.is_valid_in_calculus(term)?;
                argument.is_valid_in_calculus(term)?;
                
                // Check application preserves stratification
                if operator.universe_level() <= argument.universe_level() {
                    Err(Error::InvalidInCalculus {
                        calculus: Self::calculus_name(),
                        message: "Type application violates stratification".into(),
                    })
                } else {
                    Ok(())
                }
            },
            _ => Ok(()),
        }
    }
    
    fn get_kind(&self) -> Result<Term> {
        match self {
            SystemOmegaType::TypeOperator { param_kind, body_type } => {
                // Type operators have function kind
                let result_kind = body_type.get_kind()?;
                Ok(Term::Kind {
                    from_kind: Box::new(param_kind.clone()),
                    to_kind: Box::new(result_kind),
                })
            },
            SystemOmegaType::TypeApplication { operator, argument } => {
                // Get result kind from operator's function kind
                match operator.get_kind()? {
                    Term::Kind { to_kind, .. } => Ok(*to_kind),
                    _ => Err(Error::InvalidKind("Expected function kind".into())),
                }
            },
            SystemOmegaType::TypeVariable { kind, .. } => {
                // Variables have their declared kind
                Ok(kind.clone())
            },
            SystemOmegaType::Universal { body_type, .. } |
            SystemOmegaType::FunctionType { .. } => {
                // These construct proper types of kind *
                Ok(Term::BaseKind)
            },
            _ => Ok(Term::BaseKind),
        }
    }
}

// Helper methods
impl SystemOmegaType {
    fn max_kind_level() -> usize {
        // Maximum kind level in System Omega (implementation specific)
        3
    }
    
    fn has_base_kind(&self) -> Result<bool> {
        match self.get_kind()? {
            Term::BaseKind => Ok(true),
            _ => Ok(false),
        }
    }
}

// Helper methods for kind construction
impl Kind {
    pub fn arrow(param_kind: Kind, return_kind: Kind) -> Self {
        Kind::Arrow {
            param_kind: Box::new(param_kind),
            return_kind: Box::new(return_kind),
        }
    }
    
    pub fn dependent_arrow(param_name: impl Into<String>, param_kind: Kind, return_kind: Kind) -> Self {
        Kind::DependentArrow {
            param_name: param_name.into(),
            param_kind: Box::new(param_kind),
            return_kind: Box::new(return_kind),
        }
    }
}

// Helper methods for type construction
impl SystemOmegaType {
    pub fn type_operator(param_name: impl Into<String>, param_kind: Kind, body_type: Term) -> Self {
        SystemOmegaType::TypeOperator {
            param_name: param_name.into(),
            param_kind: Box::new(param_kind),
            body_type: Box::new(body_type),
        }
    }
    
    pub fn type_application(operator: Term, argument: Term) -> Self {
        SystemOmegaType::TypeApplication {
            operator: Box::new(operator),
            argument: Box::new(argument),
        }
    }
    
    pub fn kind_operator(param_name: impl Into<String>, param_kind: Kind, body_kind: Kind) -> Self {
        SystemOmegaType::KindOperator {
            param_name: param_name.into(),
            param_kind: Box::new(param_kind),
            body_kind: Box::new(body_kind),
        }
    }
}
