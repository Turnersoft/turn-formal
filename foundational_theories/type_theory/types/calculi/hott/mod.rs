use crate::formalize_v2::foundational_theories::type_theory::{
    core::{Result, Term, Context, Error},
    types::{TypeConstructor, calculi::CalculusType},
};
use serde::{Deserialize, Serialize};

/// HoTT type constructors
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HottType {
    // Base types
    Unit,
    Bool,
    Natural,
    
    // Higher-order types
    Path {
        base_type: Box<Term>,
        start: Box<Term>,
        end: Box<Term>,
    },
    Identity {
        base_type: Box<Term>,
        left: Box<Term>,
        right: Box<Term>,
    },
    
    // Dependent types
    DependentProduct {
        param_name: String,
        param_type: Box<Term>,
        body_type: Box<Term>,
    },
    DependentSum {
        param_name: String,
        param_type: Box<Term>,
        body_type: Box<Term>,
    },
    
    // Higher inductive types
    Circle,
    Sphere { dimension: usize },
    Torus,
    
    // Universe types
    Universe { level: usize },
}

impl TypeConstructor for HottType {
    fn check_term(&self, term: &Term) -> Result<()> {
        match self {
            HottType::Path { base_type, start, end } => {
                // Check that base_type is a valid type
                base_type.check_type()?;
                
                // Check that start and end points have the base type
                start.check_type_against(base_type)?;
                end.check_type_against(base_type)?;
                
                Ok(())
            },
            HottType::Identity { base_type, left, right } => {
                // Check that base_type is a valid type
                base_type.check_type()?;
                
                // Check that left and right terms have the base type
                left.check_type_against(base_type)?;
                right.check_type_against(base_type)?;
                
                Ok(())
            },
            HottType::DependentProduct { param_name, param_type, body_type } => {
                // Check param_type is a valid type
                param_type.check_type()?;
                
                // Check body_type in extended context
                let mut context = Context::new();
                context.add_variable(param_name.clone(), param_type.clone())?;
                body_type.check_type_in_context(&context)
            },
            HottType::DependentSum { param_name, param_type, body_type } => {
                // Similar to DependentProduct
                param_type.check_type()?;
                
                let mut context = Context::new();
                context.add_variable(param_name.clone(), param_type.clone())?;
                body_type.check_type_in_context(&context)
            },
            HottType::Circle => Ok(()), // Base HIT
            HottType::Sphere { dimension } => {
                if *dimension == 0 {
                    Err(Error::InvalidType("Sphere dimension must be positive".into()))
                } else {
                    Ok(())
                }
            },
            HottType::Torus => Ok(()), // Base HIT
            HottType::Universe { level } => Ok(()),
            _ => Ok(()),
        }
    }

    fn universe_level(&self) -> usize {
        match self {
            HottType::Universe { level } => level + 1,
            HottType::Path { base_type, .. } => {
                // Path types live in same universe as base type
                base_type.universe_level()
            },
            HottType::Identity { base_type, .. } => {
                // Identity types live in same universe as base type
                base_type.universe_level()
            },
            HottType::DependentProduct { param_type, body_type } |
            HottType::DependentSum { param_type, body_type } => {
                // Take max of parameter and body type levels
                param_type.universe_level().max(body_type.universe_level())
            },
            HottType::Circle | HottType::Torus => 0, // Base HITs live in first universe
            HottType::Sphere { dimension } => *dimension, // n-sphere lives in n-th universe
            _ => 0,
        }
    }
}

impl CalculusType for HottType {
    fn calculus_name() -> &'static str {
        "Homotopy Type Theory"
    }
    
    fn is_valid_in_calculus(&self, term: &Term) -> Result<()> {
        match self {
            HottType::Path { base_type, start, end } => {
                // Check path endpoints are equal as types
                if !start.is_definitionally_equal(end)? {
                    Err(Error::InvalidInCalculus {
                        calculus: Self::calculus_name(),
                        message: "Path endpoints must be definitionally equal".into(),
                    })
                } else {
                    Ok(())
                }
            },
            HottType::Identity { base_type, left, right } => {
                // Check terms are in the same connected component
                if !left.is_path_connected_to(right)? {
                    Err(Error::InvalidInCalculus {
                        calculus: Self::calculus_name(),
                        message: "Identity type terms must be path-connected".into(),
                    })
                } else {
                    Ok(())
                }
            },
            HottType::Sphere { dimension } => {
                // Check dimension is valid for spheres
                if *dimension == 0 {
                    Err(Error::InvalidInCalculus {
                        calculus: Self::calculus_name(),
                        message: "0-sphere is defined as Bool type".into(),
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
            HottType::Universe { level } => {
                // Universe n has type Universe (n+1)
                Ok(Term::Universe { level: level + 1 })
            },
            HottType::Path { .. } |
            HottType::Identity { .. } |
            HottType::DependentProduct { .. } |
            HottType::DependentSum { .. } |
            HottType::Circle |
            HottType::Sphere { .. } |
            HottType::Torus => {
                // All other types live in some universe
                Ok(Term::Universe { level: self.universe_level() })
            },
            _ => Ok(Term::Universe { level: 0 }),
        }
    }
}

// Helper methods for type construction
impl HottType {
    pub fn path(base_type: Term, start: Term, end: Term) -> Self {
        HottType::Path {
            base_type: Box::new(base_type),
            start: Box::new(start),
            end: Box::new(end),
        }
    }
    
    pub fn identity(base_type: Term, left: Term, right: Term) -> Self {
        HottType::Identity {
            base_type: Box::new(base_type),
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    
    pub fn dependent_product(param_name: impl Into<String>, param_type: Term, body_type: Term) -> Self {
        HottType::DependentProduct {
            param_name: param_name.into(),
            param_type: Box::new(param_type),
            body_type: Box::new(body_type),
        }
    }
}
