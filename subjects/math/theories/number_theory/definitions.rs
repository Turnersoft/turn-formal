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
        left: Located<Parametrizable<MathExpression>>,
        right: Located<Parametrizable<MathExpression>>,
    },

    /// One number is less than or equal to another
    LessThanOrEqual {
        left: Located<Parametrizable<MathExpression>>,
        right: Located<Parametrizable<MathExpression>>,
    },

    /// One number is greater than another
    GreaterThan {
        left: Located<Parametrizable<MathExpression>>,
        right: Located<Parametrizable<MathExpression>>,
    },

    /// One number is greater than or equal to another
    GreaterThanOrEqual {
        left: Located<Parametrizable<MathExpression>>,
        right: Located<Parametrizable<MathExpression>>,
    },

    /// One number divides another
    Divides {
        divisor: Located<Parametrizable<MathExpression>>,
        dividend: Located<Parametrizable<MathExpression>>,
    },

    /// Two numbers are congruent modulo a third
    Congruent {
        left: Located<Parametrizable<MathExpression>>,
        right: Located<Parametrizable<MathExpression>>,
        modulus: Located<Parametrizable<MathExpression>>,
    },

    /// A number is prime
    IsPrime {
        number: Located<Parametrizable<MathExpression>>,
    },

    /// A number is composite
    IsComposite {
        number: Located<Parametrizable<MathExpression>>,
    },

    /// Two numbers are coprime
    AreCoprime {
        first: Located<Parametrizable<MathExpression>>,
        second: Located<Parametrizable<MathExpression>>,
    },

    /// A number is a quadratic residue modulo another
    IsQuadraticResidue {
        residue: Located<Parametrizable<MathExpression>>,
        modulus: Located<Parametrizable<MathExpression>>,
    },

    /// Custom number theory relation
    Custom {
        name: String,
        parameters: Vec<Located<Parametrizable<MathExpression>>>,
    },
}

// Helper methods for constructor functions
impl NumberTheoryRelation {
    /// Create a new LessThan relation
    pub fn less_than(left: &MathExpression, right: &MathExpression) -> Self {
        NumberTheoryRelation::LessThan {
            left: Located::new(Parametrizable::Concrete(left.clone())),
            right: Located::new(Parametrizable::Concrete(right.clone())),
        }
    }

    /// Create a new LessThanOrEqual relation
    pub fn less_than_or_equal(left: &MathExpression, right: &MathExpression) -> Self {
        NumberTheoryRelation::LessThanOrEqual {
            left: Located::new(Parametrizable::Concrete(left.clone())),
            right: Located::new(Parametrizable::Concrete(right.clone())),
        }
    }

    /// Create a new GreaterThan relation
    pub fn greater_than(left: &MathExpression, right: &MathExpression) -> Self {
        NumberTheoryRelation::GreaterThan {
            left: Located::new(Parametrizable::Concrete(left.clone())),
            right: Located::new(Parametrizable::Concrete(right.clone())),
        }
    }

    /// Create a new GreaterThanOrEqual relation
    pub fn greater_than_or_equal(left: &MathExpression, right: &MathExpression) -> Self {
        NumberTheoryRelation::GreaterThanOrEqual {
            left: Located::new(Parametrizable::Concrete(left.clone())),
            right: Located::new(Parametrizable::Concrete(right.clone())),
        }
    }

    /// Create a new Divides relation
    pub fn divides(divisor: &MathExpression, dividend: &MathExpression) -> Self {
        NumberTheoryRelation::Divides {
            divisor: Located::new(Parametrizable::Concrete(divisor.clone())),
            dividend: Located::new(Parametrizable::Concrete(dividend.clone())),
        }
    }

    /// Create a new Congruent relation
    pub fn congruent(
        left: &MathExpression,
        right: &MathExpression,
        modulus: &MathExpression,
    ) -> Self {
        NumberTheoryRelation::Congruent {
            left: Located::new(Parametrizable::Concrete(left.clone())),
            right: Located::new(Parametrizable::Concrete(right.clone())),
            modulus: Located::new(Parametrizable::Concrete(modulus.clone())),
        }
    }

    /// Create a custom relation
    pub fn custom(name: &str, parameters: Vec<MathExpression>) -> Self {
        NumberTheoryRelation::Custom {
            name: name.to_string(),
            parameters: parameters
                .into_iter()
                .map(|p| Located::new(Parametrizable::Concrete(p)))
                .collect(),
        }
    }
}
