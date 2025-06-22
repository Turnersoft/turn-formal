// Module: src/formalize_v2/subjects/math/theories/number_theory/definitions.rs
// Defines structures for number theory domain

use super::super::super::super::math::formalism::expressions::MathExpression;
use super::super::super::super::math::formalism::relations::RelationDetail;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Number {}

impl ToString for Number {
    fn to_string(&self) -> String {
        "Number".to_string()
    }
}

/// Relations specific to number theory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NumberTheoryRelation {
    /// One number is less than another
    LessThan {
        left: MathExpression,
        right: MathExpression,
    },

    /// One number is less than or equal to another
    LessThanOrEqual {
        left: MathExpression,
        right: MathExpression,
    },

    /// One number is greater than another
    GreaterThan {
        left: MathExpression,
        right: MathExpression,
    },

    /// One number is greater than or equal to another
    GreaterThanOrEqual {
        left: MathExpression,
        right: MathExpression,
    },

    /// One number divides another
    Divides {
        divisor: MathExpression,
        dividend: MathExpression,
    },

    /// Two numbers are congruent modulo a third
    Congruent {
        left: MathExpression,
        right: MathExpression,
        modulus: MathExpression,
    },

    /// A number is prime
    IsPrime { number: MathExpression },

    /// A number is composite
    IsComposite { number: MathExpression },

    /// Two numbers are coprime
    AreCoprime {
        first: MathExpression,
        second: MathExpression,
    },

    /// A number is a quadratic residue modulo another
    IsQuadraticResidue {
        residue: MathExpression,
        modulus: MathExpression,
    },

    /// Custom number theory relation
    Custom {
        name: String,
        parameters: Vec<MathExpression>,
    },
}

// Helper methods for constructor functions
impl NumberTheoryRelation {
    /// Create a new LessThan relation
    pub fn less_than(left: &MathExpression, right: &MathExpression) -> Self {
        NumberTheoryRelation::LessThan {
            left: left.clone(),
            right: right.clone(),
        }
    }

    /// Create a new LessThanOrEqual relation
    pub fn less_than_or_equal(left: &MathExpression, right: &MathExpression) -> Self {
        NumberTheoryRelation::LessThanOrEqual {
            left: left.clone(),
            right: right.clone(),
        }
    }

    /// Create a new GreaterThan relation
    pub fn greater_than(left: &MathExpression, right: &MathExpression) -> Self {
        NumberTheoryRelation::GreaterThan {
            left: left.clone(),
            right: right.clone(),
        }
    }

    /// Create a new GreaterThanOrEqual relation
    pub fn greater_than_or_equal(left: &MathExpression, right: &MathExpression) -> Self {
        NumberTheoryRelation::GreaterThanOrEqual {
            left: left.clone(),
            right: right.clone(),
        }
    }

    /// Create a new Divides relation
    pub fn divides(divisor: &MathExpression, dividend: &MathExpression) -> Self {
        NumberTheoryRelation::Divides {
            divisor: divisor.clone(),
            dividend: dividend.clone(),
        }
    }

    /// Create a new Congruent relation
    pub fn congruent(
        left: &MathExpression,
        right: &MathExpression,
        modulus: &MathExpression,
    ) -> Self {
        NumberTheoryRelation::Congruent {
            left: left.clone(),
            right: right.clone(),
            modulus: modulus.clone(),
        }
    }

    /// Create a custom relation
    pub fn custom(name: &str, parameters: Vec<MathExpression>) -> Self {
        NumberTheoryRelation::Custom {
            name: name.to_string(),
            parameters,
        }
    }
}
