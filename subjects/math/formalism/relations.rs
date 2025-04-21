// Module: src/formalize_v2/subjects/math/theorem/relations.rs
// Defines relationships between mathematical objects and expressions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::core::MathObjectType;
use super::expressions::{Identifier, MathExpression};

// Import domain-specific relations from their respective modules
use super::super::super::super::foundational_theories::category_theory::definitions::CategoryRelation;
use super::super::theories::groups::definitions::GroupRelation;
use super::super::theories::number_theory::definitions::NumberTheoryRelation;
use super::super::theories::rings::definitions::RingRelation;
use super::super::theories::topology::relations::TopologyRelation;
use super::super::theories::zfc::relations::SetTheoryRelation;

/// Quantification of a mathematical object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Quantification {
    /// Object is universally quantified (∀)
    Universal,

    /// Object exists (∃)
    Existential,

    /// Object exists uniquely (∃!)
    UniqueExistential,

    /// Object is defined in terms of other objects
    Defined(MathExpression),
}

/// Entity information for relation operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelationDetail {
    /// The expressions involved in the relation
    pub expressions: Vec<MathExpression>,

    /// Optional metadata for additional context
    pub metadata: HashMap<String, String>,

    /// Optional description of this relation instance
    pub description: Option<String>,
}

/// A mathematical relation between objects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MathRelation {
    // Core logical connectives only
    // Quantifier are encoded into theorem so that it is PNF directly.
    And(Vec<MathRelation>),
    Or(Vec<MathRelation>),
    Not(Box<MathRelation>),
    Implies(Box<MathRelation>, Box<MathRelation>),
    Equivalent(Box<MathRelation>, Box<MathRelation>),

    // Domain-specific relations organized by theory
    NumberTheory(NumberTheoryRelation),
    SetTheory(SetTheoryRelation),
    GroupTheory(GroupRelation),
    RingTheory(RingRelation),
    TopologyTheory(TopologyRelation),
    CategoryTheory(CategoryRelation),

    // For basic equality that crosses domains
    Equal {
        meta: RelationDetail,
        left: MathExpression,
        right: MathExpression,
    },

    // For custom relations that don't fit other categories
    Todo {
        name: String,
        expressions: Vec<MathExpression>,
    },
}

// Helper methods for MathRelation to maintain backward compatibility
impl MathRelation {
    /// Creates an Equal relation with entity information
    pub fn equal(left: MathExpression, right: MathExpression) -> Self {
        let entity = RelationDetail {
            expressions: vec![left.clone(), right.clone()],
            metadata: HashMap::new(),
            description: None,
        };
        MathRelation::Equal {
            meta: entity,
            left,
            right,
        }
    }

    /// Creates a number theory LessThan relation
    pub fn less_than(left: MathExpression, right: MathExpression) -> Self {
        MathRelation::NumberTheory(NumberTheoryRelation::less_than(&left, &right))
    }

    /// Creates a number theory GreaterThan relation
    pub fn greater_than(left: MathExpression, right: MathExpression) -> Self {
        MathRelation::NumberTheory(NumberTheoryRelation::greater_than(&left, &right))
    }

    /// Creates a number theory LessThanOrEqual relation
    pub fn less_than_or_equal(left: MathExpression, right: MathExpression) -> Self {
        MathRelation::NumberTheory(NumberTheoryRelation::less_than_or_equal(&left, &right))
    }

    /// Creates a number theory GreaterThanOrEqual relation
    pub fn greater_than_or_equal(left: MathExpression, right: MathExpression) -> Self {
        MathRelation::NumberTheory(NumberTheoryRelation::greater_than_or_equal(&left, &right))
    }

    /// Creates a set theory ElementOf relation
    pub fn element_of(element: MathExpression, set: MathExpression) -> Self {
        MathRelation::SetTheory(SetTheoryRelation::element_of(&element, &set))
    }

    /// Creates a set theory SubsetOf relation
    pub fn subset_of(subset: MathExpression, superset: MathExpression) -> Self {
        MathRelation::SetTheory(SetTheoryRelation::subset_of(&subset, &superset))
    }

    /// Creates an And relation
    pub fn and(relations: Vec<MathRelation>) -> Self {
        MathRelation::And(relations)
    }

    /// Creates an Implies relation
    pub fn implies(antecedent: MathRelation, consequent: MathRelation) -> Self {
        MathRelation::Implies(Box::new(antecedent), Box::new(consequent))
    }

    /// Creates a Custom relation
    pub fn custom(name: String, expressions: Vec<MathExpression>) -> Self {
        MathRelation::Todo { name, expressions }
    }

    /// Creates a category theory ObjectInCategory relation
    pub fn object_in_category(object: MathExpression, category: MathExpression) -> Self {
        MathRelation::CategoryTheory(CategoryRelation::object_in_category(&object, &category))
    }

    /// Creates a category theory MorphismBetween relation
    pub fn morphism_between(
        morphism: MathExpression,
        source: MathExpression,
        target: MathExpression,
        category: MathExpression,
    ) -> Self {
        MathRelation::CategoryTheory(CategoryRelation::morphism_between(
            &morphism, &source, &target, &category,
        ))
    }

    /// Creates a category theory IsIsomorphism relation
    pub fn is_isomorphism(morphism: MathExpression) -> Self {
        MathRelation::CategoryTheory(CategoryRelation::is_isomorphism(&morphism))
    }
}
