// Module: src/formalize_v2/subjects/math/theories/zfc/relations.rs
// Defines relations specific to set theory

use crate::subjects::math::formalism::expressions::MathExpression;
use crate::subjects::math::formalism::relations::RelationDetail;
use serde::{Deserialize, Serialize};

/// Entity information for set theory relation operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetTheoryRelationEntity {
    /// Optional ID for referencing this relation
    pub id: Option<String>,

    /// Optional description explaining this relation instance
    pub description: Option<String>,

    /// Optional key-value pairs for additional context
    pub tags: Vec<(String, String)>,
}

/// Relations specific to set theory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SetTheoryRelation {
    /// Element is in a set: x ∈ A
    ElementOf {
        entity: SetTheoryRelationEntity,
        element: MathExpression,
        set: MathExpression,
    },

    /// Element is not in a set: x ∉ A
    NotElementOf {
        entity: SetTheoryRelationEntity,
        element: MathExpression,
        set: MathExpression,
    },

    /// One set is a subset of another: A ⊆ B
    SubsetOf {
        entity: SetTheoryRelationEntity,
        subset: MathExpression,
        superset: MathExpression,
    },

    /// One set is a proper subset of another: A ⊊ B
    ProperSubsetOf {
        entity: SetTheoryRelationEntity,
        subset: MathExpression,
        superset: MathExpression,
    },

    /// Two sets are equal: A = B
    Equal {
        entity: SetTheoryRelationEntity,
        left: MathExpression,
        right: MathExpression,
    },

    /// Two sets are disjoint: A ∩ B = ∅
    Disjoint {
        entity: SetTheoryRelationEntity,
        first: MathExpression,
        second: MathExpression,
    },

    /// A set is the union of two others: C = A ∪ B
    Union {
        entity: SetTheoryRelationEntity,
        result: MathExpression,
        first: MathExpression,
        second: MathExpression,
    },

    /// A set is the intersection of two others: C = A ∩ B
    Intersection {
        entity: SetTheoryRelationEntity,
        result: MathExpression,
        first: MathExpression,
        second: MathExpression,
    },

    /// A set is the complement of another: A = B^c
    Complement {
        entity: SetTheoryRelationEntity,
        complement: MathExpression,
        original: MathExpression,
        universe: Option<MathExpression>,
    },

    /// A set is the power set of another: A = P(B)
    PowerSet {
        entity: SetTheoryRelationEntity,
        power_set: MathExpression,
        original: MathExpression,
    },

    /// A set has a specific cardinality: |A| = n
    HasCardinality {
        entity: SetTheoryRelationEntity,
        set: MathExpression,
        cardinality: MathExpression,
    },

    /// Two sets have the same cardinality: |A| = |B|
    SameCardinality {
        entity: SetTheoryRelationEntity,
        first: MathExpression,
        second: MathExpression,
    },

    /// A set is countable
    IsCountable {
        entity: SetTheoryRelationEntity,
        set: MathExpression,
    },

    /// A set is uncountable
    IsUncountable {
        entity: SetTheoryRelationEntity,
        set: MathExpression,
    },

    /// A set is finite
    IsFinite {
        entity: SetTheoryRelationEntity,
        set: MathExpression,
    },

    /// A set is infinite
    IsInfinite {
        entity: SetTheoryRelationEntity,
        set: MathExpression,
    },

    /// A set is empty
    IsEmpty {
        entity: SetTheoryRelationEntity,
        set: MathExpression,
    },

    /// Custom set theory relation
    Custom {
        entity: SetTheoryRelationEntity,
        name: String,
        parameters: Vec<MathExpression>,
    },
}

// Helper methods for constructor functions
impl SetTheoryRelation {
    /// Create a new ElementOf relation
    pub fn element_of(element: &MathExpression, set: &MathExpression) -> Self {
        let entity = SetTheoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        SetTheoryRelation::ElementOf {
            entity,
            element: element.clone(),
            set: set.clone(),
        }
    }

    /// Create a new SubsetOf relation
    pub fn subset_of(subset: &MathExpression, superset: &MathExpression) -> Self {
        let entity = SetTheoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        SetTheoryRelation::SubsetOf {
            entity,
            subset: subset.clone(),
            superset: superset.clone(),
        }
    }

    /// Create a new ProperSubsetOf relation
    pub fn proper_subset_of(subset: &MathExpression, superset: &MathExpression) -> Self {
        let entity = SetTheoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        SetTheoryRelation::ProperSubsetOf {
            entity,
            subset: subset.clone(),
            superset: superset.clone(),
        }
    }

    /// Create a new Equal relation
    pub fn equal(left: &MathExpression, right: &MathExpression) -> Self {
        let entity = SetTheoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        SetTheoryRelation::Equal {
            entity,
            left: left.clone(),
            right: right.clone(),
        }
    }

    /// Create a new Disjoint relation
    pub fn disjoint(first: &MathExpression, second: &MathExpression) -> Self {
        let entity = SetTheoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        SetTheoryRelation::Disjoint {
            entity,
            first: first.clone(),
            second: second.clone(),
        }
    }

    /// Create a custom relation
    pub fn custom(name: &str, parameters: Vec<MathExpression>) -> Self {
        let entity = SetTheoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        SetTheoryRelation::Custom {
            entity,
            name: name.to_string(),
            parameters,
        }
    }
}
