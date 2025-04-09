// Module: src/formalize_v2/subjects/math/theories/number_theory/definitions.rs
// Defines structures for number theory domain

use crate::subjects::math::formalism::expressions::MathExpression;
use crate::subjects::math::formalism::relations::RelationDetail;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Number {}

impl ToString for Number {
    fn to_string(&self) -> String {
        "Number".to_string()
    }
}

/// Entity information for number theory relation operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NumberTheoryRelationEntity {
    /// Optional ID for referencing this relation
    pub id: Option<String>,

    /// Optional description explaining this relation instance
    pub description: Option<String>,

    /// Optional key-value pairs for additional context
    pub tags: Vec<(String, String)>,
}

/// Relations specific to number theory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NumberTheoryRelation {
    /// One number is less than another
    LessThan {
        entity: NumberTheoryRelationEntity,
        left: MathExpression,
        right: MathExpression,
    },

    /// One number is less than or equal to another
    LessThanOrEqual {
        entity: NumberTheoryRelationEntity,
        left: MathExpression,
        right: MathExpression,
    },

    /// One number is greater than another
    GreaterThan {
        entity: NumberTheoryRelationEntity,
        left: MathExpression,
        right: MathExpression,
    },

    /// One number is greater than or equal to another
    GreaterThanOrEqual {
        entity: NumberTheoryRelationEntity,
        left: MathExpression,
        right: MathExpression,
    },

    /// One number divides another
    Divides {
        entity: NumberTheoryRelationEntity,
        divisor: MathExpression,
        dividend: MathExpression,
    },

    /// Two numbers are congruent modulo a third
    Congruent {
        entity: NumberTheoryRelationEntity,
        left: MathExpression,
        right: MathExpression,
        modulus: MathExpression,
    },

    /// A number is prime
    IsPrime {
        entity: NumberTheoryRelationEntity,
        number: MathExpression,
    },

    /// A number is composite
    IsComposite {
        entity: NumberTheoryRelationEntity,
        number: MathExpression,
    },

    /// Two numbers are coprime
    AreCoprime {
        entity: NumberTheoryRelationEntity,
        first: MathExpression,
        second: MathExpression,
    },

    /// A number is a quadratic residue modulo another
    IsQuadraticResidue {
        entity: NumberTheoryRelationEntity,
        residue: MathExpression,
        modulus: MathExpression,
    },

    /// Custom number theory relation
    Custom {
        entity: NumberTheoryRelationEntity,
        name: String,
        parameters: Vec<MathExpression>,
    },
}

// Helper methods for constructor functions
impl NumberTheoryRelation {
    /// Create a new LessThan relation
    pub fn less_than(left: &MathExpression, right: &MathExpression) -> Self {
        let entity = NumberTheoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        NumberTheoryRelation::LessThan {
            entity,
            left: left.clone(),
            right: right.clone(),
        }
    }

    /// Create a new LessThanOrEqual relation
    pub fn less_than_or_equal(left: &MathExpression, right: &MathExpression) -> Self {
        let entity = NumberTheoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        NumberTheoryRelation::LessThanOrEqual {
            entity,
            left: left.clone(),
            right: right.clone(),
        }
    }

    /// Create a new GreaterThan relation
    pub fn greater_than(left: &MathExpression, right: &MathExpression) -> Self {
        let entity = NumberTheoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        NumberTheoryRelation::GreaterThan {
            entity,
            left: left.clone(),
            right: right.clone(),
        }
    }

    /// Create a new GreaterThanOrEqual relation
    pub fn greater_than_or_equal(left: &MathExpression, right: &MathExpression) -> Self {
        let entity = NumberTheoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        NumberTheoryRelation::GreaterThanOrEqual {
            entity,
            left: left.clone(),
            right: right.clone(),
        }
    }

    /// Create a new Divides relation
    pub fn divides(divisor: &MathExpression, dividend: &MathExpression) -> Self {
        let entity = NumberTheoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        NumberTheoryRelation::Divides {
            entity,
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
        let entity = NumberTheoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        NumberTheoryRelation::Congruent {
            entity,
            left: left.clone(),
            right: right.clone(),
            modulus: modulus.clone(),
        }
    }

    /// Create a custom relation
    pub fn custom(name: &str, parameters: Vec<MathExpression>) -> Self {
        let entity = NumberTheoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        NumberTheoryRelation::Custom {
            entity,
            name: name.to_string(),
            parameters,
        }
    }
}
