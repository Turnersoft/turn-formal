use serde::{Deserialize, Serialize};

use crate::turn_render::Identifier;

use super::super::super::super::math::theories::{VariantSet, VariantWrapper};

use super::super::super::formalism::extract::Parametrizable;

use super::axioms::{SatisfiesZFC, ZFCAxioms};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::sync::LazyLock;

/// Elements that can belong to sets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SetElement {
    /// A nested set (following ZFC's everything-is-a-set principle)
    Set(Box<Set>),

    /// Primitive numeric element (for efficient computation)
    Integer(i64),

    /// Named/symbolic element (for abstract sets)
    Symbol(String),

    /// Ordered pair (a,b) - fundamental for relations/functions
    Pair(Box<SetElement>, Box<SetElement>),

    /// Special marker for urelements (non-set objects in some set theories)
    Urelement(String),
}

/// A condition for set elements
/// Used in the Separation axiom to define subsets
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ElementCondition {
    /// Element is empty
    IsEmpty,
    /// Element contains a specific set element
    Contains(Box<SetElement>),
    /// Element is contained in a specific set
    ContainedIn(Box<Set>),
    /// Element is not contained in a specific set
    NotContainedIn(Box<Set>),
}

// Implement Hash manually for ElementCondition to be consistent with PartialEq
impl Hash for ElementCondition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            ElementCondition::IsEmpty => {
                // Use a unique number to identify this variant
                0.hash(state);
            }
            ElementCondition::Contains(s) => {
                // Use a unique number to identify this variant
                1.hash(state);
                s.hash(state);
            }
            ElementCondition::ContainedIn(s) => {
                // Use a unique number to identify this variant
                2.hash(state);
                s.hash(state);
            }
            ElementCondition::NotContainedIn(s) => {
                // Use a unique number to identify this variant
                3.hash(state);
                s.hash(state);
            }
        }
    }
}

/// Properties that can be applied to any set
/// These properties help track mathematical characteristics of sets
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum SetProperty {
    /// The cardinality (size) of the set
    /// Properties:
    /// - Finite sets have specific size n
    /// - ℵ₀ for countably infinite sets
    /// - 2^ℵ₀ for continuum size
    /// - Higher cardinals for larger sets
    Cardinality(CardinalityPropertyVariant),

    /// Whether the set is empty
    /// Properties:
    /// - Contains no elements
    /// - Is a subset of all sets
    /// - Unique up to extensionality
    /// Examples: ∅
    IsEmpty(bool),

    /// Whether the set is finite
    /// Properties:
    /// - Has finite number of elements
    /// - Can be put in bijection with {0,1,...,n-1}
    /// - Every subset has minimal element
    /// Examples: {∅}, {∅,{∅}}, finite powersets
    IsFinite(bool),

    /// Whether the set is countable
    /// Properties:
    /// - Can be put in bijection with ℕ
    /// - Has cardinality ℵ₀ if infinite
    /// - Union of countable sets is countable
    /// Examples: ℕ, ℤ, ℚ
    IsCountable(bool),

    /// Whether the set is well-ordered
    /// Properties:
    /// - Every non-empty subset has least element
    /// - Total ordering exists
    /// - Transfinite induction applies
    /// Examples: Ordinal numbers, ℕ with usual order
    IsWellOrdered(bool),

    /// Whether the set is transitive
    /// Properties:
    /// - If y ∈ x ∈ A then y ∈ A
    /// - Contains all elements of its elements
    /// - Important for ordinal numbers
    /// Examples: Ordinal numbers, von Neumann hierarchy
    IsTransitive(bool),

    /// Whether the set is an ordinal number
    /// Properties:
    /// - Transitive set
    /// - Well-ordered by ∈
    /// - Represents position/rank
    /// Examples: 0 = ∅, 1 = {∅}, 2 = {∅,{∅}}
    IsOrdinal(bool),

    /// Whether the set is a cardinal number
    /// Properties:
    /// - Initial ordinal of its cardinality class
    /// - Measures size of sets
    /// - Cannot be put in bijection with smaller ordinal
    /// Examples: ℵ₀, ℵ₁, finite cardinals
    IsCardinal(bool),

    /// Whether the relation is reflexive
    /// Properties:
    /// - For all x in domain, (x,x) is in the relation
    /// - Required for equivalence relations and partial orders
    /// Examples: =, ≤, ⊆
    IsReflexive(bool),

    /// Whether the relation is symmetric
    /// Properties:
    /// - If (x,y) is in relation then (y,x) is also in relation
    /// - Required for equivalence relations
    /// Examples: =, ≠, "is parallel to"
    IsSymmetric(bool),
}

/// Cardinality of a set
/// Represents the size of a set, including infinite cardinalities
#[derive(Clone, Hash, Serialize, Deserialize, PartialEq, Debug, Eq)]
pub enum CardinalityPropertyVariant {
    /// Finite sets with specific size
    Finite(usize),
    /// ℵ₀ (aleph-null) - countably infinite
    CountablyInfinite,
    /// 2^ℵ₀ - size of the continuum
    ContinuumSize,
    /// Higher cardinal numbers
    LargerCardinal(usize),
}

// impl Hash for VariantSet<SetProperty> {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         // Convert the HashSet to a Vec, sort it, and hash the sorted Vec
//         let mut elements: Vec<_> = self.inner.iter().collect();
//         elements.sort_by_key(|x| format!("{:?}", x));
//         elements.hash(state);
//     }
// }

// impl Hash for VariantSet<SetOpProperty> {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         // Convert the HashSet to a Vec, sort it, and hash the sorted Vec
//         let mut elements: Vec<_> = self.inner.iter().collect();
//         elements.sort_by_key(|x| format!("{:?}", x));
//         elements.hash(state);
//     }
// }

/// Properties specific to ordinal operations
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum OrdinalOpProperty {
    /// Whether operation preserves well-ordering
    PreservesWellOrdering(bool),
    /// Whether operation is associative
    Associative(bool),
    /// Whether operation is commutative
    Commutative(bool),
    /// Identity element for the operation
    Identity(Box<Set>),
    /// Operation this one distributes over
    DistributesOver(Box<Set>),
}

/// Properties specific to set-theoretic operations
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum SetOpProperty {
    /// Whether operation preserves cardinality
    PreservesCardinality(bool),
    /// Whether operation preserves transitivity
    PreservesTransitivity(bool),
    /// Whether operation satisfies extensionality
    SatisfiesExtensionality(bool),
    /// Whether operation is idempotent
    Idempotent(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct GenericSet {
    pub properties: VariantSet<SetProperty>,
}

impl GenericSet {
    pub fn new() -> Self {
        Self {
            properties: VariantSet::new(),
        }
    }
}

/// A set in ZFC set theory, defined by its membership rule
/// This implementation follows the ZFC axioms and provides a foundation for set-theoretic constructions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Set {
    /// A generic, abstract set (L1), or a set type defined by properties (L2).
    Generic(GenericSet),

    /// The empty set (∅), unique and contains no elements
    Empty,

    /// A singleton set {x} containing exactly one element
    /// Forms the basis for building more complex sets
    Singleton {
        /// The single element contained in this set
        element: SetElement,
        /// Properties of the singleton set (e.g., cardinality = 1)
        properties: VariantSet<SetProperty>,
    },

    /// A set defined by explicitly enumerating its elements
    /// {e₁, e₂, ..., eₙ}
    Enumeration {
        /// The elements in this set
        elements: Vec<SetElement>,
        /// Properties of the enumerated set
        properties: VariantSet<SetProperty>,
    },

    /// Union of two sets A ∪ B = {x | x ∈ A ∨ x ∈ B}
    /// Implements the Union Axiom of ZFC for the binary case
    BinaryUnion {
        /// First set in the union
        left: Box<Set>,
        /// Second set in the union
        right: Box<Set>,
        /// Properties of the resulting union
        properties: VariantSet<SetProperty>,
        op_properties: VariantSet<SetOpProperty>,
    },

    /// Intersection of two sets A ∩ B = {x | x ∈ A ∧ x ∈ B}
    BinaryIntersection {
        /// First set in the intersection
        left: Box<Set>,
        /// Second set in the intersection
        right: Box<Set>,
        /// Properties of the resulting intersection
        properties: VariantSet<SetProperty>,
        op_properties: VariantSet<SetOpProperty>,
    },

    /// Set difference A - B = {x | x ∈ A ∧ x ∉ B}
    SetDifference {
        /// Set to subtract from (minuend)
        left: Box<Set>,
        /// Set to subtract (subtrahend)
        right: Box<Set>,
        /// Properties of the resulting difference
        properties: VariantSet<SetProperty>,
        op_properties: VariantSet<SetOpProperty>,
    },

    /// Symmetric difference A △ B = (A - B) ∪ (B - A)
    SymmetricDifference {
        /// First set in the symmetric difference
        left: Box<Set>,
        /// Second set in the symmetric difference
        right: Box<Set>,
        /// Properties of the resulting symmetric difference
        properties: VariantSet<SetProperty>,
        op_properties: VariantSet<SetOpProperty>,
    },

    /// Cartesian product A × B = {(a,b) | a ∈ A ∧ b ∈ B}
    /// Uses Kuratowski's definition of ordered pairs
    CartesianProduct {
        /// First set in the product
        left: Box<Set>,
        /// Second set in the product
        right: Box<Set>,
        /// Properties of the resulting product
        properties: VariantSet<SetProperty>,
        op_properties: VariantSet<SetOpProperty>,
    },

    /// Union of a family of sets ⋃F = {x | ∃S ∈ F (x ∈ S)}
    /// Implements the general Union Axiom of ZFC
    BigUnion {
        /// Family of sets to take the union of
        family: Box<Set>,
        /// Properties of the resulting big union
        properties: VariantSet<SetProperty>,
        op_properties: VariantSet<SetOpProperty>,
    },

    /// Intersection of a family of sets ⋂F = {x | ∀S ∈ F (x ∈ S)}
    BigIntersection {
        /// Family of sets to take the intersection of
        family: Box<Set>,
        /// Properties of the resulting big intersection
        properties: VariantSet<SetProperty>,
        op_properties: VariantSet<SetOpProperty>,
    },

    /// Power set P(A) = {x | x ⊆ A}
    /// Implements the Power Set Axiom of ZFC
    PowerSet {
        /// Base set to take the power set of
        base: Box<Set>,
        /// Properties of the resulting power set
        properties: VariantSet<SetProperty>,
        op_properties: VariantSet<SetOpProperty>,
    },

    /// Separation {x ∈ source | condition(x)}
    /// Implements the Separation Axiom of ZFC
    Separation {
        /// Source set to separate elements from
        source: Box<Set>,
        /// Condition that elements must satisfy
        condition: ElementCondition,
        /// Properties of the resulting set
        properties: VariantSet<SetProperty>,
        op_properties: VariantSet<SetOpProperty>,
    },

    /// Replacement {f(x) | x ∈ source}
    /// Implements the Replacement Axiom of ZFC
    Replacement {
        /// Source set to apply the mapping to
        source: Box<Set>,
        /// Function to apply to elements
        mapping: SetMapping,
        /// Properties of the resulting set
        properties: VariantSet<SetProperty>,
        op_properties: VariantSet<SetOpProperty>,
    },

    /// Ordered pair (a,b) using Kuratowski's definition
    /// (a,b) = {{a}, {a,b}}
    OrderedPair {
        /// First element of the pair
        first: Box<Set>,
        /// Second element of the pair
        second: Box<Set>,
        /// Properties of the resulting ordered pair
        properties: VariantSet<SetProperty>,
        op_properties: VariantSet<SetOpProperty>,
    },

    /// Complement of a set relative to a universe
    /// A' = {x ∈ U | x ∉ A}
    Complement {
        /// Set to complement
        set: Box<Set>,
        /// Universe relative to which to take the complement
        universe: Box<Set>,
        /// Properties of the resulting complement
        properties: VariantSet<SetProperty>,
        op_properties: VariantSet<SetOpProperty>,
    },

    /// A parametric set defined by parameters and a membership condition
    /// Examples include Z_n, S_n, GL(n,F), etc.
    Parametric {
        /// Parameters that define the set (e.g., "n" in Z_n)
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        #[serde(default)]
        parameters: HashMap<String, String>,
        /// Description of the set
        description: String,
        /// Condition for membership in the set
        membership_condition: String,
        /// Properties of the parametric set
        properties: VariantSet<SetProperty>,
    },
}

/// SetExpression represents expressions involving sets that don't return sets
/// These include actions, morphisms, functions, and other operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SetExpression {
    /// A variable representing a set
    Variable(Identifier),

    /// The cardinality of a set: |A|
    Cardinality { set: Box<Parametrizable<Set>> },

    /// Selection of an element from a set (choice function)
    ElementSelection { set: Box<Parametrizable<Set>> },
}

/// Relations between sets, capturing the predicate structure of set theory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SetRelation {
    /// Element relation: x ∈ A
    ElementOf {
        element: Parametrizable<SetElement>,
        set: Parametrizable<Set>,
    },

    /// Subset relation: A ⊆ B
    SubsetOf {
        subset: Parametrizable<Set>,
        superset: Parametrizable<Set>,
    },

    /// Proper subset relation: A ⊂ B
    ProperSubsetOf {
        subset: Parametrizable<Set>,
        superset: Parametrizable<Set>,
    },

    /// Set equality: A = B
    Equals {
        left: Parametrizable<Set>,
        right: Parametrizable<Set>,
    },

    /// Disjoint sets: A ∩ B = ∅
    AreDisjoint {
        left: Parametrizable<Set>,
        right: Parametrizable<Set>,
    },

    /// Set has cardinality n: |A| = n
    HasCardinality {
        set: Parametrizable<Set>,
        cardinality: Parametrizable<CardinalityPropertyVariant>,
    },

    /// Set is countable: A is countable
    IsCountable { set: Parametrizable<Set> },

    /// Set is finite: A is finite
    IsFinite { set: Parametrizable<Set> },

    /// Set is empty: A = ∅
    IsEmpty { set: Parametrizable<Set> },

    /// Cardinality comparison: |A| < |B|
    CardinalityLessThan {
        left: Parametrizable<Set>,
        right: Parametrizable<Set>,
    },

    /// Cardinality comparison: |A| ≤ |B|
    CardinalityLessThanOrEqual {
        left: Parametrizable<Set>,
        right: Parametrizable<Set>,
    },

    /// Sets are equivalent/equinumerous: A ~ B (same cardinality)
    AreEquinumerous {
        left: Parametrizable<Set>,
        right: Parametrizable<Set>,
    },
}

// Helper methods for SetRelation
impl SetRelation {
    /// Creates an ElementOf relation with concrete expressions
    pub fn element_of(element: &SetElement, set: &Set) -> Self {
        SetRelation::ElementOf {
            element: Parametrizable::Concrete(element.clone()),
            set: Parametrizable::Concrete(set.clone()),
        }
    }

    /// Creates a SubsetOf relation with concrete expressions
    pub fn subset_of(subset: &Set, superset: &Set) -> Self {
        SetRelation::SubsetOf {
            subset: Parametrizable::Concrete(subset.clone()),
            superset: Parametrizable::Concrete(superset.clone()),
        }
    }

    /// Creates a ProperSubsetOf relation with concrete expressions
    pub fn proper_subset_of(subset: &Set, superset: &Set) -> Self {
        SetRelation::ProperSubsetOf {
            subset: Parametrizable::Concrete(subset.clone()),
            superset: Parametrizable::Concrete(superset.clone()),
        }
    }

    /// Creates an Equals relation with concrete expressions
    pub fn equals(left: &Set, right: &Set) -> Self {
        SetRelation::Equals {
            left: Parametrizable::Concrete(left.clone()),
            right: Parametrizable::Concrete(right.clone()),
        }
    }

    /// Creates an AreDisjoint relation with concrete expressions
    pub fn are_disjoint(left: &Set, right: &Set) -> Self {
        SetRelation::AreDisjoint {
            left: Parametrizable::Concrete(left.clone()),
            right: Parametrizable::Concrete(right.clone()),
        }
    }

    /// Creates a HasCardinality relation with concrete expressions
    pub fn has_cardinality(set: &Set, cardinality: CardinalityPropertyVariant) -> Self {
        SetRelation::HasCardinality {
            set: Parametrizable::Concrete(set.clone()),
            cardinality: Parametrizable::Concrete(cardinality),
        }
    }

    /// Creates an IsCountable relation with a concrete expression
    pub fn is_countable(set: &Set) -> Self {
        SetRelation::IsCountable {
            set: Parametrizable::Concrete(set.clone()),
        }
    }

    /// Creates an IsFinite relation with a concrete expression
    pub fn is_finite(set: &Set) -> Self {
        SetRelation::IsFinite {
            set: Parametrizable::Concrete(set.clone()),
        }
    }

    /// Creates an IsEmpty relation with a concrete expression
    pub fn is_empty(set: &Set) -> Self {
        SetRelation::IsEmpty {
            set: Parametrizable::Concrete(set.clone()),
        }
    }

    // Additional helper methods for other relations

    /// Check if this relation matches a pattern
    pub fn matches_pattern(&self, pattern: &SetRelation) -> bool {
        match (self, pattern) {
            (
                SetRelation::ElementOf {
                    element: e1,
                    set: s1,
                },
                SetRelation::ElementOf {
                    element: e2,
                    set: s2,
                },
            ) => {
                // If pattern has variables, it's a wildcard match
                match (e2, s2) {
                    (Parametrizable::Variable(_), Parametrizable::Variable(_)) => true,
                    (Parametrizable::Variable(_), _) => true,
                    (_, Parametrizable::Variable(_)) => true,
                    // Otherwise, check if both are concrete and match
                    (
                        Parametrizable::Concrete(e2_concrete),
                        Parametrizable::Concrete(s2_concrete),
                    ) => match (e1, s1) {
                        (
                            Parametrizable::Concrete(e1_concrete),
                            Parametrizable::Concrete(s1_concrete),
                        ) => e1_concrete == e2_concrete && s1_concrete == s2_concrete,
                        _ => false,
                    },
                }
            }

            (
                SetRelation::SubsetOf {
                    subset: sub1,
                    superset: sup1,
                },
                SetRelation::SubsetOf {
                    subset: sub2,
                    superset: sup2,
                },
            ) => {
                // Similar pattern-matching logic using direct comparisons
                true
            }

            // Add more cases for other relation types
            _ => false, // Different relation types don't match
        }
    }
}

/// Mapping functions that can be applied to set elements
/// Used in the Replacement axiom to construct new sets
#[derive(Clone, Hash, Serialize, Deserialize, PartialEq, Debug, Eq)]
pub enum SetMapping {
    /// Identity function f(x) = x
    Identity,
    /// Singleton function f(x) = {x}
    Singleton,
    /// First projection π₁((x,y)) = x
    FirstProjection,
    /// Second projection π₂((x,y)) = y
    SecondProjection,
    /// Function composition (g ∘ f)(x) = g(f(x))
    Composition(Box<SetMapping>, Box<SetMapping>),
    /// Custom mapping defined by a string representation
    Custom(String),
}

// Helper function to calculate properties for an empty set
fn default_empty_properties() -> VariantSet<SetProperty> {
    let mut properties = VariantSet::new();
    properties.insert(SetProperty::IsEmpty(true));
    properties.insert(SetProperty::IsFinite(true));
    properties.insert(SetProperty::IsCountable(true));
    properties.insert(SetProperty::IsWellOrdered(true));
    properties.insert(SetProperty::IsTransitive(true));
    properties.insert(SetProperty::IsOrdinal(true));
    properties.insert(SetProperty::IsCardinal(true));
    properties.insert(SetProperty::Cardinality(
        CardinalityPropertyVariant::Finite(0),
    ));
    properties
}

// Helper function to calculate properties for an enumeration of known elements
fn calculate_properties_for_enumeration(elements: &Vec<SetElement>) -> VariantSet<SetProperty> {
    let mut properties = VariantSet::new();
    let len = elements.len();
    properties.insert(SetProperty::IsEmpty(len == 0));
    properties.insert(SetProperty::IsFinite(true));
    properties.insert(SetProperty::IsCountable(true));
    properties.insert(SetProperty::Cardinality(
        CardinalityPropertyVariant::Finite(len),
    ));
    if len == 0 {
        properties.insert(SetProperty::IsWellOrdered(true));
        properties.insert(SetProperty::IsTransitive(true));
        properties.insert(SetProperty::IsOrdinal(true));
        properties.insert(SetProperty::IsCardinal(true));
    } else {
        properties.insert(SetProperty::IsWellOrdered(false));
        properties.insert(SetProperty::IsTransitive(false));
        properties.insert(SetProperty::IsOrdinal(false));
        properties.insert(SetProperty::IsCardinal(false));
    }
    properties
}

// Custom Hash implementation for Set to handle the HashMap inside Parametric variant
impl Hash for Set {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Use discriminant to hash the variant
        std::mem::discriminant(self).hash(state);

        // For variants with fields, hash the relevant fields
        match self {
            Set::Generic(gs) => {
                (*gs).hash(state);
            }
            Set::Empty => {}
            Set::Singleton {
                element,
                properties,
            } => {
                element.hash(state);
                properties.hash(state);
            }
            Set::Enumeration {
                elements,
                properties,
            } => {
                elements.hash(state);
                properties.hash(state);
            }
            Set::BinaryUnion {
                left,
                right,
                properties,
                op_properties,
            } => {
                left.hash(state);
                right.hash(state);
                properties.hash(state);
                op_properties.hash(state);
            }
            Set::BinaryIntersection {
                left,
                right,
                properties,
                op_properties,
            } => {
                left.hash(state);
                right.hash(state);
                properties.hash(state);
                op_properties.hash(state);
            }
            Set::SetDifference {
                left,
                right,
                properties,
                op_properties,
            } => {
                left.hash(state);
                right.hash(state);
                properties.hash(state);
                op_properties.hash(state);
            }
            Set::SymmetricDifference {
                left,
                right,
                properties,
                op_properties,
            } => {
                left.hash(state);
                right.hash(state);
                properties.hash(state);
                op_properties.hash(state);
            }
            Set::CartesianProduct {
                left,
                right,
                properties,
                op_properties,
            } => {
                left.hash(state);
                right.hash(state);
                properties.hash(state);
                op_properties.hash(state);
            }
            Set::BigUnion {
                family,
                properties,
                op_properties,
            } => {
                family.hash(state);
                properties.hash(state);
                op_properties.hash(state);
            }
            Set::BigIntersection {
                family,
                properties,
                op_properties,
            } => {
                family.hash(state);
                properties.hash(state);
                op_properties.hash(state);
            }
            Set::PowerSet {
                base,
                properties,
                op_properties,
            } => {
                base.hash(state);
                properties.hash(state);
                op_properties.hash(state);
            }
            Set::Separation {
                source,
                condition,
                properties,
                op_properties,
            } => {
                source.hash(state);
                condition.hash(state);
                properties.hash(state);
                op_properties.hash(state);
            }
            Set::Replacement {
                source,
                mapping,
                properties,
                op_properties,
            } => {
                source.hash(state);
                mapping.hash(state);
                properties.hash(state);
                op_properties.hash(state);
            }
            Set::OrderedPair {
                first,
                second,
                properties,
                op_properties,
            } => {
                first.hash(state);
                second.hash(state);
                properties.hash(state);
                op_properties.hash(state);
            }
            Set::Complement {
                set,
                universe,
                properties,
                op_properties,
            } => {
                set.hash(state);
                universe.hash(state);
                properties.hash(state);
                op_properties.hash(state);
            }
            Set::Parametric {
                description,
                membership_condition,
                properties,
                parameters,
            } => {
                // For HashMap, hash each key-value pair after sorting
                let mut param_pairs: Vec<_> = parameters.iter().collect();
                param_pairs.sort_by(|a, b| a.0.cmp(b.0));
                for (k, v) in param_pairs {
                    k.hash(state);
                    v.hash(state);
                }

                description.hash(state);
                membership_condition.hash(state);
                properties.hash(state);
            }
        }
    }
}

// Implementation for Set
impl Set {
    pub fn evaluate(&self) -> Self {
        match self {
            // L3 Constructors first
            Set::BinaryUnion {
                left,
                right,
                properties: _,
                op_properties,
            } => {
                let eval_left = left.evaluate();
                let eval_right = right.evaluate();
                match (&eval_left, &eval_right) {
                    (Set::Empty, _) => return eval_right.clone(),
                    (_, Set::Empty) => return eval_left.clone(),
                    (
                        Set::Enumeration {
                            elements: els_left, ..
                        },
                        Set::Enumeration {
                            elements: els_right,
                            ..
                        },
                    ) => {
                        let mut result_elements_set = HashSet::new();
                        els_left.iter().for_each(|el| {
                            result_elements_set.insert(el.clone());
                        });
                        els_right.iter().for_each(|el| {
                            result_elements_set.insert(el.clone());
                        });
                        let final_elements: Vec<SetElement> =
                            result_elements_set.into_iter().collect();
                        let new_props = calculate_properties_for_enumeration(&final_elements);
                        return Set::Enumeration {
                            elements: final_elements,
                            properties: new_props,
                        };
                    }
                    _ => {
                        let mut new_union_props = VariantSet::new();
                        if eval_left
                            .get_properties()
                            .map_or(false, |p| p.contains(&SetProperty::IsFinite(true)))
                            && eval_right
                                .get_properties()
                                .map_or(false, |p| p.contains(&SetProperty::IsFinite(true)))
                        {
                            new_union_props.insert(SetProperty::IsFinite(true));
                        }
                        if matches!(eval_left, Set::Empty) {
                            if let Some(rp) = eval_right.get_properties() {
                                if let Some(is_empty_prop) = rp
                                    .iter()
                                    .find(|p_ref| matches!(p_ref, SetProperty::IsEmpty(_)))
                                {
                                    new_union_props.insert(is_empty_prop.clone());
                                }
                            }
                        } else if matches!(eval_right, Set::Empty) {
                            if let Some(lp) = eval_left.get_properties() {
                                if let Some(is_empty_prop) = lp
                                    .iter()
                                    .find(|p_ref| matches!(p_ref, SetProperty::IsEmpty(_)))
                                {
                                    new_union_props.insert(is_empty_prop.clone());
                                }
                            }
                        } else {
                            new_union_props.insert(SetProperty::IsEmpty(false));
                        }
                        Set::BinaryUnion {
                            left: Box::new(eval_left),
                            right: Box::new(eval_right),
                            properties: new_union_props,
                            op_properties: op_properties.clone(),
                        }
                    }
                }
            }
            Set::BinaryIntersection {
                left,
                right,
                properties: _,
                op_properties,
            } => {
                let eval_left = left.evaluate();
                let eval_right = right.evaluate();
                match (&eval_left, &eval_right) {
                    (Set::Empty, _) | (_, Set::Empty) => return Set::empty(),
                    (
                        Set::Enumeration {
                            elements: els_left, ..
                        },
                        Set::Enumeration {
                            elements: els_right,
                            ..
                        },
                    ) => {
                        let els_right_set: HashSet<_> = els_right.iter().cloned().collect();
                        let final_elements: Vec<SetElement> = els_left
                            .iter()
                            .filter(|el| els_right_set.contains(el))
                            .cloned()
                            .collect();
                        let new_props = calculate_properties_for_enumeration(&final_elements);
                        return Set::Enumeration {
                            elements: final_elements,
                            properties: new_props,
                        };
                    }
                    _ => {
                        Set::BinaryIntersection {
                            left: Box::new(eval_left),
                            right: Box::new(eval_right),
                            properties: VariantSet::new(), // Placeholder for derived properties
                            op_properties: op_properties.clone(),
                        }
                    }
                }
            }
            Set::SetDifference {
                left,
                right,
                properties: _,
                op_properties,
            } => {
                let eval_left = left.evaluate();
                let eval_right = right.evaluate();
                match (&eval_left, &eval_right) {
                    (Set::Empty, _) => return Set::empty(),
                    (_, Set::Empty) => return eval_left.clone(),
                    (
                        Set::Enumeration {
                            elements: els_left, ..
                        },
                        Set::Enumeration {
                            elements: els_right,
                            ..
                        },
                    ) => {
                        let els_right_set: HashSet<_> = els_right.iter().cloned().collect();
                        let final_elements: Vec<SetElement> = els_left
                            .iter()
                            .filter(|el| !els_right_set.contains(el))
                            .cloned()
                            .collect();
                        let new_props = calculate_properties_for_enumeration(&final_elements);
                        return Set::Enumeration {
                            elements: final_elements,
                            properties: new_props,
                        };
                    }
                    _ => Set::SetDifference {
                        left: Box::new(eval_left),
                        right: Box::new(eval_right),
                        properties: VariantSet::new(),
                        op_properties: op_properties.clone(),
                    },
                }
            }
            Set::SymmetricDifference {
                left,
                right,
                properties: _,
                op_properties,
            } => {
                let eval_left = left.evaluate();
                let eval_right = right.evaluate();
                match (&eval_left, &eval_right) {
                    (
                        Set::Enumeration {
                            elements: els_left, ..
                        },
                        Set::Enumeration {
                            elements: els_right,
                            ..
                        },
                    ) => {
                        let mut result_elements_set = HashSet::new();
                        let set_left: HashSet<_> = els_left.iter().cloned().collect();
                        let set_right: HashSet<_> = els_right.iter().cloned().collect();
                        for el in &set_left {
                            if !set_right.contains(el) {
                                result_elements_set.insert(el.clone());
                            }
                        }
                        for el in &set_right {
                            if !set_left.contains(el) {
                                result_elements_set.insert(el.clone());
                            }
                        }
                        let final_elements: Vec<SetElement> =
                            result_elements_set.into_iter().collect();
                        let new_props = calculate_properties_for_enumeration(&final_elements);
                        return Set::Enumeration {
                            elements: final_elements,
                            properties: new_props,
                        };
                    }
                    _ => {
                        let diff_ab = Set::SetDifference {
                            left: Box::new(eval_left.clone()),
                            right: Box::new(eval_right.clone()),
                            properties: VariantSet::new(),
                            op_properties: op_properties.clone(),
                        };
                        let diff_ba = Set::SetDifference {
                            left: Box::new(eval_right),
                            right: Box::new(eval_left),
                            properties: VariantSet::new(),
                            op_properties: op_properties.clone(),
                        };
                        Set::BinaryUnion {
                            left: Box::new(diff_ab),
                            right: Box::new(diff_ba),
                            properties: VariantSet::new(),
                            op_properties: op_properties.clone(),
                        }
                        .evaluate()
                    }
                }
            }
            Set::CartesianProduct {
                left,
                right,
                properties: _,
                op_properties,
            } => {
                let eval_left = left.evaluate();
                let eval_right = right.evaluate();
                match (&eval_left, &eval_right) {
                    (Set::Empty, _) | (_, Set::Empty) => return Set::empty(),
                    (
                        Set::Enumeration {
                            elements: els_left, ..
                        },
                        Set::Enumeration {
                            elements: els_right,
                            ..
                        },
                    ) => {
                        let mut final_elements = Vec::new();
                        for el_l in els_left {
                            for el_r in els_right {
                                final_elements.push(SetElement::Pair(
                                    Box::new(el_l.clone()),
                                    Box::new(el_r.clone()),
                                ));
                            }
                        }
                        let new_props = calculate_properties_for_enumeration(&final_elements);
                        return Set::Enumeration {
                            elements: final_elements,
                            properties: new_props,
                        };
                    }
                    _ => Set::CartesianProduct {
                        left: Box::new(eval_left),
                        right: Box::new(eval_right),
                        properties: VariantSet::new(),
                        op_properties: op_properties.clone(),
                    },
                }
            }
            Set::PowerSet {
                base,
                properties: _,
                op_properties,
            } => {
                let eval_base = base.evaluate();
                match &eval_base {
                    Set::Empty => {
                        return Set::singleton(SetElement::Set(Box::new(Set::empty())));
                    }
                    Set::Enumeration {
                        elements: base_els, ..
                    } => {
                        if base_els.len() > 5 {
                            // TODO: More robust property calculation for P(LargeFiniteSet)
                            return Set::PowerSet {
                                base: Box::new(eval_base),
                                properties: VariantSet::new(),
                                op_properties: op_properties.clone(),
                            };
                        }
                        let mut subsets_elements = Vec::new();
                        let num_subsets = 1 << base_els.len();
                        for i in 0..num_subsets {
                            let mut current_subset_els = Vec::new();
                            for j in 0..base_els.len() {
                                if (i >> j) & 1 == 1 {
                                    current_subset_els.push(base_els[j].clone());
                                }
                            }
                            subsets_elements.push(SetElement::Set(Box::new(Set::from_elements(
                                current_subset_els,
                            ))));
                        }
                        let new_props = calculate_properties_for_enumeration(&subsets_elements);
                        return Set::Enumeration {
                            elements: subsets_elements,
                            properties: new_props,
                        };
                    }
                    _ => {
                        let mut new_props = VariantSet::new();
                        if eval_base
                            .get_properties()
                            .map_or(false, |p| p.contains(&SetProperty::IsFinite(true)))
                        {
                            new_props.insert(SetProperty::IsFinite(true));
                            if let Some(card_prop) = eval_base.get_properties().and_then(|props| {
                                props.iter().find(|p_ref| {
                                    matches!(
                                        p_ref,
                                        SetProperty::Cardinality(
                                            CardinalityPropertyVariant::Finite(_)
                                        )
                                    )
                                })
                            }) {
                                if let SetProperty::Cardinality(
                                    CardinalityPropertyVariant::Finite(n_base),
                                ) = card_prop
                                {
                                    new_props.insert(SetProperty::Cardinality(
                                        CardinalityPropertyVariant::Finite(1 << n_base),
                                    ));
                                }
                            }
                        } else if eval_base
                            .get_properties()
                            .map_or(false, |p| p.contains(&SetProperty::IsCountable(true)))
                        {
                            new_props.insert(SetProperty::IsCountable(false));
                            new_props.insert(SetProperty::Cardinality(
                                CardinalityPropertyVariant::ContinuumSize,
                            ));
                        }
                        Set::PowerSet {
                            base: Box::new(eval_base),
                            properties: new_props,
                            op_properties: op_properties.clone(),
                        }
                    }
                }
            }
            // For these more complex constructors, just evaluate components for now.
            // Full evaluation would require interpreting conditions/mappings.
            Set::BigUnion {
                family,
                properties,
                op_properties,
            } => Set::BigUnion {
                family: Box::new(family.evaluate()),
                properties: properties.clone(),
                op_properties: op_properties.clone(),
            },
            Set::BigIntersection {
                family,
                properties,
                op_properties,
            } => Set::BigIntersection {
                family: Box::new(family.evaluate()),
                properties: properties.clone(),
                op_properties: op_properties.clone(),
            },
            Set::Separation {
                source,
                condition,
                properties,
                op_properties,
            } => Set::Separation {
                source: Box::new(source.evaluate()),
                condition: condition.clone(),
                properties: properties.clone(),
                op_properties: op_properties.clone(),
            },
            Set::Replacement {
                source,
                mapping,
                properties,
                op_properties,
            } => Set::Replacement {
                source: Box::new(source.evaluate()),
                mapping: mapping.clone(),
                properties: properties.clone(),
                op_properties: op_properties.clone(),
            },
            Set::OrderedPair {
                first,
                second,
                properties,
                op_properties,
            } => Set::OrderedPair {
                first: Box::new(first.evaluate()),
                second: Box::new(second.evaluate()),
                properties: properties.clone(),
                op_properties: op_properties.clone(),
            },
            Set::Complement {
                set,
                universe,
                properties,
                op_properties,
            } => Set::Complement {
                set: Box::new(set.evaluate()),
                universe: Box::new(universe.evaluate()),
                properties: properties.clone(),
                op_properties: op_properties.clone(),
            },

            // Base cases: L1, L2, or L4 non-reducible forms
            Set::Generic { .. } => self.clone(),
            Set::Empty => self.clone(),
            Set::Singleton {
                element,
                properties,
            } => {
                let eval_element = match element {
                    SetElement::Set(s_box) => SetElement::Set(Box::new(s_box.evaluate())),
                    _ => element.clone(),
                };
                // Properties of a singleton are fairly fixed or depend on the element's nature.
                // Re-calculating them fully here could be complex. Assume they are okay or re-derive if needed.
                // For now, just ensure element is evaluated.
                let mut current_props = properties.clone();
                // Ensure basic singleton props are there if we were to rebuild
                current_props.insert(SetProperty::IsEmpty(false));
                current_props.insert(SetProperty::IsFinite(true));
                current_props.insert(SetProperty::IsCountable(true));
                current_props.insert(SetProperty::Cardinality(
                    CardinalityPropertyVariant::Finite(1),
                ));
                Set::Singleton {
                    element: eval_element,
                    properties: current_props,
                }
            }
            Set::Enumeration {
                elements,
                properties: _,
            } => {
                // Original properties ignored, re-calculate
                let eval_elements: Vec<SetElement> = elements
                    .iter()
                    .map(|el| match el {
                        SetElement::Set(s_box) => SetElement::Set(Box::new(s_box.evaluate())),
                        _ => el.clone(),
                    })
                    .collect();
                let new_props = calculate_properties_for_enumeration(&eval_elements);
                Set::Enumeration {
                    elements: eval_elements,
                    properties: new_props,
                }
            }
            Set::Parametric {
                parameters,
                description,
                membership_condition,
                properties,
            } => self.clone(),
        }
    }

    pub fn empty() -> Self {
        Set::Empty
    }

    pub fn singleton(element: SetElement) -> Self {
        let mut properties = VariantSet::new();
        properties.insert(SetProperty::IsEmpty(false));
        properties.insert(SetProperty::IsFinite(true));
        properties.insert(SetProperty::IsCountable(true));
        properties.insert(SetProperty::Cardinality(
            CardinalityPropertyVariant::Finite(1),
        ));
        properties.insert(SetProperty::IsWellOrdered(true));
        let is_trans_ord_card = match &element {
            SetElement::Set(s_box) => matches!(**s_box, Set::Empty),
            _ => false,
        };
        properties.insert(SetProperty::IsTransitive(is_trans_ord_card));
        properties.insert(SetProperty::IsOrdinal(is_trans_ord_card));
        properties.insert(SetProperty::IsCardinal(is_trans_ord_card));
        Set::Singleton {
            element,
            properties,
        }
    }

    pub fn from_elements(elements: Vec<SetElement>) -> Self {
        let unique_elements: Vec<SetElement> = {
            let mut seen = HashSet::new();
            elements
                .into_iter()
                .filter(|el| seen.insert(el.clone()))
                .collect()
        };
        if unique_elements.is_empty() {
            return Set::empty();
        }
        let properties = calculate_properties_for_enumeration(&unique_elements);
        Set::Enumeration {
            elements: unique_elements,
            properties,
        }
    }

    pub fn pair(a: SetElement, b: SetElement) -> Self {
        if a == b {
            Set::singleton(a)
        } else {
            Set::from_elements(vec![a, b])
        }
    }

    pub fn union(self, other: Set) -> Self {
        let mut initial_props = VariantSet::new();
        let self_props = self.get_properties();
        let other_props = other.get_properties();

        // Assuming VariantSet has a .contains(&SetProperty) method
        let self_is_empty = self_props.map_or(false, |p| p.contains(&SetProperty::IsEmpty(true)));
        let other_is_empty = other_props.map_or(false, |p| p.contains(&SetProperty::IsEmpty(true)));
        initial_props.insert(SetProperty::IsEmpty(self_is_empty && other_is_empty));

        let self_is_finite = self_props.map_or(false, |p| p.contains(&SetProperty::IsFinite(true)));
        let other_is_finite =
            other_props.map_or(false, |p| p.contains(&SetProperty::IsFinite(true)));
        if self_is_finite && other_is_finite {
            initial_props.insert(SetProperty::IsFinite(true));
            initial_props.insert(SetProperty::IsCountable(true));
        } else {
            initial_props.insert(SetProperty::IsFinite(false));
            if self_props.map_or(false, |p| p.contains(&SetProperty::IsCountable(true)))
                && other_props.map_or(false, |p| p.contains(&SetProperty::IsCountable(true)))
            {
                initial_props.insert(SetProperty::IsCountable(true));
            } else {
                initial_props.insert(SetProperty::IsCountable(false));
            }
        }
        Set::BinaryUnion {
            left: Box::new(self),
            right: Box::new(other),
            properties: initial_props,
            op_properties: VariantSet::new(),
        }
    }

    pub fn get_properties(&self) -> Option<&VariantSet<SetProperty>> {
        match self {
            Set::Generic(gs) => Some(&gs.properties),
            Set::Singleton { properties, .. }
            | Set::Enumeration { properties, .. }
            | Set::BinaryUnion { properties, .. }
            | Set::BinaryIntersection { properties, .. }
            | Set::SetDifference { properties, .. }
            | Set::SymmetricDifference { properties, .. }
            | Set::CartesianProduct { properties, .. }
            | Set::BigUnion { properties, .. }
            | Set::BigIntersection { properties, .. }
            | Set::PowerSet { properties, .. }
            | Set::Separation { properties, .. }
            | Set::Replacement { properties, .. }
            | Set::OrderedPair { properties, .. }
            | Set::Complement { properties, .. }
            | Set::Parametric { properties, .. } => Some(properties),
            Set::Empty => Some(&LAZY_EMPTY_PROPERTIES),
        }
    }

    /// Checks if this set contains the given element
    pub fn contains(&self, element: &SetElement) -> bool {
        match self {
            Set::Empty => false,
            Set::Singleton {
                element: set_element,
                ..
            } => set_element == element,
            Set::Enumeration { elements, .. } => elements.contains(element),
            Set::BinaryUnion { left, right, .. } => {
                left.contains(element) || right.contains(element)
            }
            Set::BinaryIntersection { left, right, .. } => {
                left.contains(element) && right.contains(element)
            }
            Set::SetDifference { left, right, .. } => {
                left.contains(element) && !right.contains(element)
            }
            Set::SymmetricDifference { left, right, .. } => {
                (left.contains(element) && !right.contains(element))
                    || (!left.contains(element) && right.contains(element))
            }
            // For more complex set constructions, we would need to evaluate the membership condition
            // This is a simplified implementation
            Set::CartesianProduct { .. } => {
                if let SetElement::Pair(first, second) = element {
                    // Check if first part is in left set and second part is in right set
                    // This is a simplification; a full implementation would require recursion
                    true
                } else {
                    false
                }
            }
            Set::BigUnion { family, .. } => {
                // If this is a set of sets, check if the element is in any of the member sets
                match family.as_ref() {
                    Set::Enumeration { elements, .. } => {
                        for set_elem in elements {
                            if let SetElement::Set(inner_set) = set_elem {
                                if inner_set.contains(element) {
                                    return true;
                                }
                            }
                        }
                        false
                    }
                    _ => false, // Simplified implementation
                }
            }
            Set::BigIntersection { family, .. } => {
                // If this is a set of sets, check if the element is in all of the member sets
                match family.as_ref() {
                    Set::Enumeration { elements, .. } => {
                        if elements.is_empty() {
                            return true; // Intersection of empty family is the universe
                        }
                        for set_elem in elements {
                            if let SetElement::Set(inner_set) = set_elem {
                                if !inner_set.contains(element) {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        }
                        true
                    }
                    _ => false, // Simplified implementation
                }
            }
            Set::PowerSet { base, .. } => {
                // Check if element is a set and is a subset of the base set
                if let SetElement::Set(elem_set) = element {
                    // A set is in the power set if it's a subset of the base set
                    match elem_set.as_ref() {
                        Set::Enumeration { elements, .. } => {
                            for e in elements {
                                if !base.contains(e) {
                                    return false;
                                }
                            }
                            true
                        }
                        Set::Empty => true, // Empty set is a subset of any set
                        _ => false,         // Simplified implementation
                    }
                } else {
                    false
                }
            }
            Set::Separation {
                source, condition, ..
            } => {
                source.contains(element)
                    && match condition {
                        ElementCondition::IsEmpty => false, // Simplified: no element is empty
                        ElementCondition::Contains(contained) => {
                            if let SetElement::Set(elem_set) = element {
                                elem_set.contains(contained)
                            } else {
                                false
                            }
                        }
                        ElementCondition::ContainedIn(container) => container.contains(element),
                        ElementCondition::NotContainedIn(container) => !container.contains(element),
                    }
            }
            Set::Replacement {
                source, mapping, ..
            } => {
                // This is a complex case - for a simplified version, we'll just return false
                false
            }
            Set::OrderedPair { first, second, .. } => {
                // Kuratowski pair: {{a}, {a,b}}
                if let SetElement::Set(inner_set) = element {
                    match inner_set.as_ref() {
                        Set::Enumeration { elements, .. } => {
                            if elements.len() == 1 {
                                if let SetElement::Set(single_elem) = &elements[0] {
                                    return single_elem.as_ref() == first.as_ref();
                                }
                            } else if elements.len() == 2 {
                                let contains_first = elements.iter().any(|e| {
                                    if let SetElement::Set(s) = e {
                                        s.as_ref() == first.as_ref()
                                    } else {
                                        false
                                    }
                                });
                                let contains_second = elements.iter().any(|e| {
                                    if let SetElement::Set(s) = e {
                                        s.as_ref() == second.as_ref()
                                    } else {
                                        false
                                    }
                                });
                                return contains_first && contains_second;
                            }
                        }
                        _ => {}
                    }
                }
                false
            }
            Set::Complement { set, universe, .. } => {
                universe.contains(element) && !set.contains(element)
            }
            Set::Generic { .. } => false, // Abstract sets don't have concrete elements
            Set::Parametric { .. } => false, // Parametric sets need specific membership logic
        }
    }
}

static LAZY_EMPTY_PROPERTIES: LazyLock<VariantSet<SetProperty>> =
    LazyLock::new(|| default_empty_properties());

// Implement From<Set> for SetElement to wrap a Set as a SetElement
impl From<Set> for SetElement {
    fn from(set: Set) -> Self {
        SetElement::Set(Box::new(set))
    }
}
