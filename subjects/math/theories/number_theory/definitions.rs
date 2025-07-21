// Module: src/formalize_v2/subjects/math/theories/number_theory/definitions.rs
// Defines structures for number theory domain

use crate::subjects::math::formalism::location::Located;

use super::super::super::super::math::formalism::expressions::MathExpression;
use super::super::super::super::math::formalism::extract::Parametrizable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Number {}

impl ToString for Number {
    fn to_string(&self) -> String {
        "Number".to_string()
    }
}

/// Relations specific to number theory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NumberTheoryRelation {
    /// One number is less than another
    LessThan {
        left: Located<MathExpression>,
        right: Located<MathExpression>,
    },

    /// One number is less than or equal to another
    LessThanOrEqual {
        left: Located<MathExpression>,
        right: Located<MathExpression>,
    },

    /// One number is greater than another
    GreaterThan {
        left: Located<MathExpression>,
        right: Located<MathExpression>,
    },

    /// One number is greater than or equal to another
    GreaterThanOrEqual {
        left: Located<MathExpression>,
        right: Located<MathExpression>,
    },

    /// One number divides another
    Divides {
        divisor: Located<MathExpression>,
        dividend: Located<MathExpression>,
    },

    /// Two numbers are congruent modulo a third
    Congruent {
        left: Located<MathExpression>,
        right: Located<MathExpression>,
        modulus: Located<MathExpression>,
    },

    /// A number is prime
    IsPrime { number: Located<MathExpression> },

    /// A number is composite
    IsComposite { number: Located<MathExpression> },

    /// Two numbers are coprime
    AreCoprime {
        first: Located<MathExpression>,
        second: Located<MathExpression>,
    },

    /// A number is a quadratic residue modulo another
    IsQuadraticResidue {
        residue: Located<MathExpression>,
        modulus: Located<MathExpression>,
    },

    /// Custom number theory relation
    Custom {
        name: String,
        parameters: Vec<Located<MathExpression>>,
    },
}

// Helper methods for constructor functions
impl NumberTheoryRelation {
    /// Create a new LessThan relation
    pub fn less_than(left: &MathExpression, right: &MathExpression) -> Self {
        NumberTheoryRelation::LessThan {
            left: Located::new_concrete(left.clone()),
            right: Located::new_concrete(right.clone()),
        }
    }

    /// Create a new LessThanOrEqual relation
    pub fn less_than_or_equal(left: &MathExpression, right: &MathExpression) -> Self {
        NumberTheoryRelation::LessThanOrEqual {
            left: Located::new_concrete(left.clone()),
            right: Located::new_concrete(right.clone()),
        }
    }

    /// Create a new GreaterThan relation
    pub fn greater_than(left: &MathExpression, right: &MathExpression) -> Self {
        NumberTheoryRelation::GreaterThan {
            left: Located::new_concrete(left.clone()),
            right: Located::new_concrete(right.clone()),
        }
    }

    /// Create a new GreaterThanOrEqual relation
    pub fn greater_than_or_equal(left: &MathExpression, right: &MathExpression) -> Self {
        NumberTheoryRelation::GreaterThanOrEqual {
            left: Located::new_concrete(left.clone()),
            right: Located::new_concrete(right.clone()),
        }
    }

    /// Create a new Divides relation
    pub fn divides(divisor: &MathExpression, dividend: &MathExpression) -> Self {
        NumberTheoryRelation::Divides {
            divisor: Located::new_concrete(divisor.clone()),
            dividend: Located::new_concrete(dividend.clone()),
        }
    }

    /// Create a new Congruent relation
    pub fn congruent(
        left: &MathExpression,
        right: &MathExpression,
        modulus: &MathExpression,
    ) -> Self {
        NumberTheoryRelation::Congruent {
            left: Located::new_concrete(left.clone()),
            right: Located::new_concrete(right.clone()),
            modulus: Located::new_concrete(modulus.clone()),
        }
    }

    /// Create a custom number theory relation with multiple parameters
    pub fn custom(name: String, parameters: Vec<MathExpression>) -> Self {
        NumberTheoryRelation::Custom {
            name,
            parameters: parameters
                .into_iter()
                .map(|p| Located::new_concrete(p))
                .collect(),
        }
    }
}
