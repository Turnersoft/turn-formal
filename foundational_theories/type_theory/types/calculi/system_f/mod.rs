use crate::formalize_v2::foundational_theories::type_theory::{
    core::{Result, Term, Context, Error},
    types::{TypeConstructor, calculi::CalculusType},
};
use serde::{Deserialize, Serialize};

/// System F type constructors
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemFType {
    // Base types
    Unit,
    Bool,
    Natural,
    
    // Type variables and abstraction
    TypeVariable(String),
    Universal {
        param_name: String,
        body_type: Box<Term>,
    },
    
    // Function types
    Function {
        param_type: Box<Term>,
        return_type: Box<Term>,
    },
    
    // Product and sum types
    Product {
        left_type: Box<Term>,
        right_type: Box<Term>,
    },
    Sum {
        left_type: Box<Term>,
        right_type: Box<Term>,
    },
    
    // Type application
    TypeApplication {
        type_function: Box<Term>,
        type_argument: Box<Term>,
    },
}

impl TypeConstructor for SystemFType {
    fn check_term(&self, term: &Term) -> Result<()> {
        match self {
            SystemFType::Universal { param_name, body_type } => {
                // Check that body_type is valid in extended context with type variable
                let mut context = Context::new();
                context.add_type_variable(param_name.clone())?;
                body_type.check_type_in_context(&context)
            },
            SystemFType::Function { param_type, return_type } => {
                // Check both parameter and return types are valid
                param_type.check_type()?;
                return_type.check_type()?;
                Ok(())
            },
            SystemFType::Product { left_type, right_type } => {
                // Check both component types are valid
                left_type.check_type()?;
                right_type.check_type()?;
                Ok(())
            },
            SystemFType::Sum { left_type, right_type } => {
                // Check both variant types are valid
                left_type.check_type()?;
                right_type.check_type()?;
                Ok(())
            },
            SystemFType::TypeVariable { name } => {
                // Type variables are valid if they're in context
                if !Context::current().contains_type_variable(name) {
                    Err(Error::UndefinedTypeVariable(name.clone()))
                } else {
                    Ok(())
                }
            },
            SystemFType::TypeApplication { type_function, type_argument } => {
                // Check type function is universal and argument is valid
                match type_function.as_ref() {
                    SystemFType::Universal { param_name, body_type } => {
                        type_argument.check_type()?;
                        Ok(())
                    },
                    _ => Err(Error::InvalidType("Type application requires universal type".into())),
                }
            },
            _ => Ok(()),
        }
    }

    fn universe_level(&self) -> usize {
        match self {
            SystemFType::Universal { body_type, .. } => {
                // Universal types live in same universe as body
                body_type.universe_level()
            },
            SystemFType::Function { param_type, return_type } |
            SystemFType::Product { left_type: param_type, right_type: return_type } |
            SystemFType::Sum { left_type: param_type, right_type: return_type } => {
                // Take max of component type levels
                param_type.universe_level().max(return_type.universe_level())
            },
            SystemFType::TypeVariable { .. } => 0, // Type variables are in base universe
            SystemFType::TypeApplication { type_function, type_argument } => {
                // Application lives in universe of function result
                type_function.universe_level()
            },
            _ => 0,
        }
    }
}

impl CalculusType for SystemFType {
    fn calculus_name() -> &'static str {
        "System F"
    }
    
    fn is_valid_in_calculus(&self, term: &Term) -> Result<()> {
        match self {
            SystemFType::Universal { param_name, body_type } => {
                // Check no dependent types in System F
                if body_type.has_free_term_variable(param_name) {
                    Err(Error::InvalidInCalculus {
                        calculus: Self::calculus_name(),
                        message: "System F does not support dependent types".into(),
                    })
                } else {
                    Ok(())
                }
            },
            SystemFType::TypeApplication { type_function, type_argument } => {
                // Ensure proper type application
                match type_function.as_ref() {
                    SystemFType::Universal { param_name, body_type } => {
                        // Check type argument is valid in System F
                        type_argument.is_valid_in_calculus(term)?;
                        
                        // Check substitution preserves well-formedness
                        let substituted = body_type.substitute_type(param_name, type_argument);
                        substituted.is_valid_in_calculus(term)
                    },
                    _ => Err(Error::InvalidInCalculus {
                        calculus: Self::calculus_name(),
                        message: "Type application requires universal type".into(),
                    }),
                }
            },
            _ => Ok(()),
        }
    }
    
    fn get_kind(&self) -> Result<Term> {
        match self {
            SystemFType::Universal { param_name, body_type } => {
                // Universal types are type constructors
                Ok(Term::Kind { 
                    level: body_type.universe_level() + 1 
                })
            },
            SystemFType::Function { .. } |
            SystemFType::Product { .. } |
            SystemFType::Sum { .. } |
            SystemFType::TypeVariable { .. } => {
                // Base types live in first universe
                Ok(Term::Universe { level: 0 })
            },
            SystemFType::TypeApplication { type_function, type_argument } => {
                // Result kind depends on function's kind
                match type_function.get_kind()? {
                    Term::Kind { level } => Ok(Term::Universe { level: level - 1 }),
                    _ => Err(Error::InvalidKind("Type application requires kind".into())),
                }
            },
            _ => Ok(Term::Universe { level: 0 }),
        }
    }
}

// Helper methods for type construction
impl SystemFType {
    pub fn universal(param_name: impl Into<String>, body_type: Term) -> Self {
        SystemFType::Universal {
            param_name: param_name.into(),
            body_type: Box::new(body_type),
        }
    }
    
    pub fn function(param_type: Term, return_type: Term) -> Self {
        SystemFType::Function {
            param_type: Box::new(param_type),
            return_type: Box::new(return_type),
        }
    }
    
    pub fn type_application(type_function: Term, type_argument: Term) -> Self {
        SystemFType::TypeApplication {
            type_function: Box::new(type_function),
            type_argument: Box::new(type_argument),
        }
    }
}
