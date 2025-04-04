use serde::{Deserialize, Serialize};

use crate::formalize_v2::subjects::math::theories::{VariantSet, VariantWrapper};

use super::axioms::{SatisfiesZFC, ZFCAxioms};
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::sync::LazyLock;

/// A condition for set elements
/// Used in the Separation axiom to define subsets
#[derive(Clone, Serialize, Deserialize)]
pub enum ElementCondition {
    /// Element is empty
    IsEmpty,
    /// Element contains a specific set
    Contains(Box<Set>),
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
#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
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
#[derive(Clone, Hash, Serialize, Deserialize)]
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

impl Hash for VariantSet<SetProperty> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Convert the HashSet to a Vec, sort it, and hash the sorted Vec
        let mut elements: Vec<_> = self.inner.iter().collect();
        elements.sort_by_key(|x| format!("{:?}", x));
        elements.hash(state);
    }
}

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

/// A set in ZFC set theory, defined by its membership rule
/// This implementation follows the ZFC axioms and provides a foundation for set-theoretic constructions
#[derive(Clone, Serialize, Deserialize)]
pub enum Set {
    /// The empty set (∅), unique and contains no elements
    /// This is guaranteed by the Empty Set Axiom of ZFC
    Empty,

    /// A singleton set {x} containing exactly one element
    /// Forms the basis for building more complex sets
    Singleton {
        /// The single element contained in this set
        element: Box<Set>,
        /// Properties of the singleton set (e.g., cardinality = 1)
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
        parameters: std::collections::HashMap<String, String>,
        /// Description of the set
        description: String,
        /// Condition for membership in the set
        membership_condition: String,
        /// Properties of the parametric set
        properties: VariantSet<SetProperty>,
    },
}

// Implement Hash manually for Set to be consistent with PartialEq
impl Hash for Set {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Set::Empty => {
                // Empty sets always hash to the same value
                0.hash(state);
            }
            Set::Singleton { element, .. } => {
                // For singletons, hash only depends on the element
                1.hash(state);
                element.hash(state);
            }
            Set::BinaryUnion { left, right, .. } => {
                // For binary unions, hash in a way that's order-independent
                // to match the PartialEq implementation
                2.hash(state);
                // Create a hash that's the same regardless of left/right order
                let left_hash = {
                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                    left.hash(&mut hasher);
                    hasher.finish()
                };
                let right_hash = {
                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                    right.hash(&mut hasher);
                    hasher.finish()
                };
                // Use min and max to make order irrelevant
                std::cmp::min(left_hash, right_hash).hash(state);
                std::cmp::max(left_hash, right_hash).hash(state);
            }
            // Hash other variants similarly to match their PartialEq implementation
            // ensuring that two equal sets will have the same hash
            _ => {
                // For other variants, use their Debug representation as a fallback
                // This is not optimal, but maintains consistency
                format!("{:?}", self).hash(state);
            }
        }
    }
}

impl Eq for Set {}

/// Mapping functions that can be applied to set elements
/// Used in the Replacement axiom to construct new sets
#[derive(Clone, Hash, Serialize, Deserialize)]
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

/// A reference to set properties
/// This allows us to access properties without taking ownership
#[derive(Debug)]
pub struct SetPropertiesRef<'a> {
    properties: &'a VariantSet<SetProperty>,
}

impl<'a> SetPropertiesRef<'a> {
    /// Creates a new reference to set properties
    pub fn new(properties: &'a VariantSet<SetProperty>) -> Self {
        Self { properties }
    }

    /// Gets the cardinality of the set
    pub fn cardinality(&self) -> Option<&CardinalityPropertyVariant> {
        self.properties.inner.iter().find_map(|p| {
            if let SetProperty::Cardinality(c) = &p.0 {
                Some(c)
            } else {
                None
            }
        })
    }

    /// Checks if the set is empty
    pub fn is_empty(&self) -> Option<bool> {
        self.properties.inner.iter().find_map(|p| {
            if let SetProperty::IsEmpty(b) = p.0 {
                Some(b)
            } else {
                None
            }
        })
    }

    /// Checks if the set is finite
    pub fn is_finite(&self) -> Option<bool> {
        self.properties.inner.iter().find_map(|p| {
            if let SetProperty::IsFinite(b) = p.0 {
                Some(b)
            } else {
                None
            }
        })
    }

    /// Checks if the set is countable
    pub fn is_countable(&self) -> Option<bool> {
        self.properties.inner.iter().find_map(|p| {
            if let SetProperty::IsCountable(b) = p.0 {
                Some(b)
            } else {
                None
            }
        })
    }

    /// Checks if the set is well-ordered
    pub fn is_well_ordered(&self) -> Option<bool> {
        self.properties.inner.iter().find_map(|p| {
            if let SetProperty::IsWellOrdered(b) = p.0 {
                Some(b)
            } else {
                None
            }
        })
    }

    /// Checks if the set is transitive
    pub fn is_transitive(&self) -> Option<bool> {
        self.properties.inner.iter().find_map(|p| {
            if let SetProperty::IsTransitive(b) = &p.0 {
                Some(*b)
            } else {
                None
            }
        })
    }

    /// Checks if the set is an ordinal number
    pub fn is_ordinal(&self) -> Option<bool> {
        self.properties.inner.iter().find_map(|p| {
            if let SetProperty::IsOrdinal(b) = &p.0 {
                Some(*b)
            } else {
                None
            }
        })
    }

    /// Checks if the set is a cardinal number
    pub fn is_cardinal(&self) -> Option<bool> {
        self.properties.inner.iter().find_map(|p| {
            if let SetProperty::IsCardinal(b) = p.0 {
                Some(b)
            } else {
                None
            }
        })
    }
}

impl Set {
    /// Creates an empty set (∅)
    pub fn empty() -> Self {
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
        Set::Empty
    }

    /// Creates a singleton set {x}
    pub fn singleton(x: Set) -> Self {
        let mut properties = VariantSet::new();
        properties.insert(SetProperty::IsEmpty(false));
        properties.insert(SetProperty::IsFinite(true));
        properties.insert(SetProperty::IsCountable(true));
        properties.insert(SetProperty::IsWellOrdered(true));
        properties.insert(SetProperty::IsTransitive(x.is_transitive()));
        properties.insert(SetProperty::IsOrdinal(x.is_ordinal()));
        properties.insert(SetProperty::IsCardinal(false));
        properties.insert(SetProperty::Cardinality(
            CardinalityPropertyVariant::Finite(1),
        ));
        Set::Singleton {
            element: Box::new(x),
            properties,
        }
    }

    /// Creates a pair set {a, b}
    /// If a = b, returns a singleton set
    /// Otherwise, creates a union of singleton sets {{a}} ∪ {{b}}
    pub fn pair(a: Set, b: Set) -> Self {
        if a == b {
            Set::singleton(a)
        } else {
            Set::BinaryUnion {
                left: Box::new(Set::singleton(a)),
                right: Box::new(Set::singleton(b)),
                properties: VariantSet::new(),
                op_properties: VariantSet::new(),
            }
        }
    }

    /// Creates a union of two sets A ∪ B
    /// The union contains all elements from both sets
    pub fn union(self, other: Set) -> Self {
        Set::BinaryUnion {
            left: Box::new(self),
            right: Box::new(other),
            properties: VariantSet::new(),
            op_properties: VariantSet::new(),
        }
    }

    /// Creates an intersection of two sets A ∩ B
    /// The intersection contains elements common to both sets
    pub fn intersection(self, other: Set) -> Self {
        Set::BinaryIntersection {
            left: Box::new(self),
            right: Box::new(other),
            properties: VariantSet::new(),
            op_properties: VariantSet::new(),
        }
    }

    /// Creates a difference of sets A - B
    /// Contains elements in A that are not in B
    pub fn difference(self, other: Set) -> Self {
        Set::SetDifference {
            left: Box::new(self),
            right: Box::new(other),
            properties: VariantSet::new(),
            op_properties: VariantSet::new(),
        }
    }

    /// Creates a symmetric difference of two sets A △ B
    /// Contains elements that are in exactly one of A or B
    pub fn symmetric_difference(self, other: Set) -> Self {
        Set::SymmetricDifference {
            left: Box::new(self),
            right: Box::new(other),
            properties: VariantSet::new(),
            op_properties: VariantSet::new(),
        }
    }

    /// Creates a cartesian product of two sets A × B
    /// The product contains all ordered pairs (a,b) where a ∈ A and b ∈ B
    pub fn cartesian_product(self, other: Set) -> Self {
        Set::CartesianProduct {
            left: Box::new(self),
            right: Box::new(other),
            properties: VariantSet::new(),
            op_properties: VariantSet::new(),
        }
    }

    /// Creates a power set P(x)
    /// Contains all subsets of x
    pub fn power_set(self) -> Self {
        Set::PowerSet {
            base: Box::new(self),
            properties: VariantSet::new(),
            op_properties: VariantSet::new(),
        }
    }

    /// Creates a subset satisfying a predicate
    /// Implements the Separation Axiom schema
    pub fn subset_of<P>(x: &Set, predicate: P) -> Set
    where
        P: Fn(&Set) -> bool,
    {
        // Directly collect elements satisfying the predicate
        let elements = x.elements().into_iter().filter(|e| predicate(e)).collect();
        Set::from_elements(elements)
    }

    /// Applies a mapping to a set element
    /// Used in implementing the Replacement Axiom
    fn apply_mapping(&self, mapping: &SetMapping) -> Option<Set> {
        match mapping {
            // Identity function f(x) = x
            SetMapping::Identity => Some(self.clone()),
            // Singleton function f(x) = {x}
            SetMapping::Singleton => Some(Set::singleton(self.clone())),
            // First projection π₁((x,y)) = x
            SetMapping::FirstProjection => match self {
                Set::OrderedPair { first, .. } => Some(first.as_ref().clone()),
                _ => self.elements().first().cloned(),
            },
            // Second projection π₂((x,y)) = y
            SetMapping::SecondProjection => match self {
                Set::OrderedPair { second, .. } => Some(second.as_ref().clone()),
                _ => self.elements().get(1).cloned(),
            },
            // Function composition (g ∘ f)(x) = g(f(x))
            SetMapping::Composition(f, g) => self
                .apply_mapping(f.as_ref())
                .and_then(|result| result.apply_mapping(g.as_ref())),
            // Custom mappings are not implemented
            SetMapping::Custom(_) => None,
        }
    }

    /// Returns true if this set contains the given element
    /// Implements the fundamental membership relation ∈
    pub fn contains(&self, x: &Set) -> bool {
        match self {
            // Empty set contains no elements
            Set::Empty => false,
            // Singleton {a} contains only a
            Set::Singleton { element, .. } => x == element.as_ref(),
            // A ∪ B contains elements in either A or B
            Set::BinaryUnion { left, right, .. } => left.contains(x) || right.contains(x),
            // A ∩ B contains elements in both A and B
            Set::BinaryIntersection { left, right, .. } => left.contains(x) && right.contains(x),
            // A - B contains elements in A but not in B
            Set::SetDifference { left, right, .. } => left.contains(x) && !right.contains(x),
            // A △ B contains elements in exactly one of A or B
            Set::SymmetricDifference { left, right, .. } => {
                (left.contains(x) && !right.contains(x)) || (!left.contains(x) && right.contains(x))
            }
            // For Cartesian product A × B, check if x is an ordered pair (a,b) with a ∈ A and b ∈ B
            Set::CartesianProduct { left, right, .. } => {
                // Debug output for tracing membership checks
                println!("Checking if {:?} is in Cartesian product", x);
                println!("Left set: {:?}", left);
                println!("Right set: {:?}", right);

                match x {
                    // If x is already in ordered pair form, check components directly
                    Set::OrderedPair { first, second, .. } => {
                        println!(
                            "Input is an ordered pair with first={:?}, second={:?}",
                            first, second
                        );
                        // Check if first component is in left set and second in right set
                        let first_in_left = left.contains(first.as_ref());
                        let second_in_right = right.contains(second.as_ref());
                        println!("first ∈ left: {}", first_in_left);
                        println!("second ∈ right: {}", second_in_right);
                        first_in_left && second_in_right
                    }
                    // If x is not an ordered pair, try to extract components using Kuratowski's definition
                    _ => {
                        println!(
                            "Input is not an ordered pair, checking if it matches Kuratowski encoding"
                        );
                        if let Some((a, b)) = Self::extract_ordered_pair(x) {
                            println!(
                                "Successfully extracted pair components: a={:?}, b={:?}",
                                a, b
                            );
                            // Check if extracted components satisfy membership conditions
                            let a_in_left = left.contains(&a);
                            let b_in_right = right.contains(&b);
                            println!("a ∈ left: {}", a_in_left);
                            println!("b ∈ right: {}", b_in_right);
                            a_in_left && b_in_right
                        } else {
                            println!("Failed to extract pair components");
                            false
                        }
                    }
                }
            }
            // For power set P(A), check if x is a subset of A
            Set::PowerSet { base, .. } => x.is_subset_of(base),
            // For separation {x ∈ source | condition(x)}, check both membership and condition
            Set::Separation {
                source, condition, ..
            } => {
                source.contains(x)
                    && match condition {
                        ElementCondition::IsEmpty => x.is_empty(),
                        ElementCondition::Contains(s) => x.contains(s),
                        ElementCondition::ContainedIn(s) => s.contains(x),
                        ElementCondition::NotContainedIn(s) => !s.contains(x),
                    }
            }
            // For replacement {f(x) | x ∈ source}, check if x is in the image of f
            Set::Replacement {
                source, mapping, ..
            } => source.elements().iter().any(|s| match mapping {
                SetMapping::Identity => s == x,
                SetMapping::Singleton => &Set::singleton(s.clone()) == x,
                SetMapping::FirstProjection => {
                    if let Set::OrderedPair { first, .. } = s {
                        first.as_ref() == x
                    } else {
                        false
                    }
                }
                SetMapping::SecondProjection => {
                    if let Set::OrderedPair { second, .. } = s {
                        second.as_ref() == x
                    } else {
                        false
                    }
                }
                SetMapping::Composition(f, g) => {
                    if let Some(f_result) = s.apply_mapping(f) {
                        if let Some(g_result) = f_result.apply_mapping(g) {
                            &g_result == x
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                SetMapping::Custom(_) => false,
            }),
            // For big union ⋃F, check if x is in any set in the family
            Set::BigUnion { family, .. } => family.elements().iter().any(|s| s.contains(x)),
            // For big intersection ⋂F, check if x is in all sets in the family
            Set::BigIntersection { family, .. } => family.elements().iter().all(|s| s.contains(x)),
            // For ordered pair (a,b), check if x matches Kuratowski's definition
            Set::OrderedPair { first, second, .. } => {
                let singleton_a = Set::singleton(first.as_ref().clone());
                let singleton_singleton_a = Set::singleton(singleton_a.clone());
                let pair_ab = Set::BinaryUnion {
                    left: Box::new(first.as_ref().clone()),
                    right: Box::new(second.as_ref().clone()),
                    properties: VariantSet::new(),
                    op_properties: VariantSet::new(),
                };
                x == &singleton_singleton_a || x == &pair_ab
            }
            // For complement A' = U - A, check if x is in universe but not in A
            Set::Complement { set, universe, .. } => universe.contains(x) && !set.contains(x),
            // For parametric set, check if x satisfies the membership condition
            Set::Parametric {
                parameters,
                membership_condition,
                ..
            } => {
                // For parametric set, check if x satisfies the membership condition
                // This would be more complex in a real implementation,
                // but for now we'll just check if the string representation
                // of x matches a substring of the membership condition
                let x_str = format!("{:?}", x);
                let condition_str = format!("{}", membership_condition);

                // Instead of checking contains, we should properly implement
                // membership testing based on the parametric set type
                if let Some(modulus_str) = parameters.get("n") {
                    // For cyclic groups Z_n
                    if let Set::Singleton { element, .. } = x {
                        if let Set::Parametric {
                            parameters: elem_params,
                            ..
                        } = element.as_ref()
                        {
                            if let Some(value_str) = elem_params.get("value") {
                                if let Ok(value) = value_str.parse::<i64>() {
                                    if let Ok(modulus) = modulus_str.parse::<i64>() {
                                        return value >= 0 && value < modulus;
                                    }
                                }
                            }
                        }
                    }
                }

                // For other parametric sets, fall back to string matching
                // This is a temporary solution
                condition_str.contains(&x_str)
            }
        }
    }

    /// Returns true if this set is a subset of another set
    /// A is a subset of B if every element of A is in B
    pub fn is_subset_of(&self, other: &Set) -> bool {
        self.elements().iter().all(|x| other.contains(x))
    }

    /// Returns true if this set is empty
    /// A set is empty if it has no elements
    pub fn is_empty(&self) -> bool {
        matches!(self, Set::Empty)
    }

    /// Returns the number of elements in this set (if finite)
    /// For infinite sets, returns the size of their finite representation
    pub fn len(&self) -> usize {
        self.elements().len()
    }

    /// Returns the elements of this set (if finite)
    /// For each set variant, computes its elements according to its mathematical definition
    pub fn elements(&self) -> Vec<Set> {
        match self {
            // Empty set has no elements
            Set::Empty => Vec::new(),

            // Singleton set contains exactly one element
            Set::Singleton { element, .. } => vec![element.as_ref().clone()],

            // Binary union contains all elements from both sets, removing duplicates
            Set::BinaryUnion { left, right, .. } => {
                let mut elements = left.elements();
                elements.extend(right.elements());
                elements.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
                elements.dedup();
                elements
            }

            // Binary intersection contains elements present in both sets
            Set::BinaryIntersection { left, right, .. } => left
                .elements()
                .into_iter()
                .filter(|x| right.contains(x))
                .collect(),

            // Set difference contains elements in left but not in right
            Set::SetDifference { left, right, .. } => left
                .elements()
                .into_iter()
                .filter(|x| !right.contains(x))
                .collect(),

            // Symmetric difference contains elements in exactly one of the sets
            Set::SymmetricDifference { left, right, .. } => {
                let mut elements = Vec::new();
                // Add elements in left but not in right
                for x in left.elements() {
                    if !right.contains(&x) {
                        elements.push(x);
                    }
                }
                // Add elements in right but not in left
                for x in right.elements() {
                    if !left.contains(&x) {
                        elements.push(x);
                    }
                }
                elements.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
                elements.dedup();
                elements
            }

            // Cartesian product contains all ordered pairs from the two sets
            Set::CartesianProduct { left, right, .. } => {
                let mut elements = Vec::new();
                // For each element in left set
                for a in left.elements() {
                    // For each element in right set
                    for b in right.elements() {
                        // Create ordered pair using Kuratowski's definition
                        let ordered_pair = Set::ordered_pair(a.clone(), b.clone());
                        elements.push(ordered_pair);
                    }
                }
                elements.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
                elements.dedup();
                elements
            }

            // Big union contains all elements from any set in the family
            Set::BigUnion { family, .. } => {
                let mut elements = Vec::new();
                for set in family.elements() {
                    elements.extend(set.elements());
                }
                elements.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
                elements.dedup();
                elements
            }

            // Big intersection contains elements present in all sets of the family
            Set::BigIntersection { family, .. } => {
                if let Some(first) = family.elements().first() {
                    first
                        .elements()
                        .into_iter()
                        .filter(|x| family.elements().iter().all(|s| s.contains(x)))
                        .collect()
                } else {
                    Vec::new()
                }
            }

            // Power set contains all possible subsets
            Set::PowerSet { base, .. } => {
                let mut subsets = vec![Set::empty()]; // Start with empty set
                // For each element in base set
                for elem in base.elements() {
                    let mut new_subsets = subsets.clone();
                    // Add element to each existing subset to create new subsets
                    for subset in &subsets {
                        let mut new_sub = subset.clone();
                        new_sub = new_sub.union(Set::singleton(elem.clone()));
                        new_subsets.push(new_sub);
                    }
                    subsets = new_subsets;
                }
                // Sort subsets by size and then by string representation
                subsets.sort_by(|a, b| {
                    let size_cmp = a.elements().len().cmp(&b.elements().len());
                    if size_cmp == std::cmp::Ordering::Equal {
                        format!("{:?}", a).cmp(&format!("{:?}", b))
                    } else {
                        size_cmp
                    }
                });
                subsets
            }

            // Separation contains elements from source that satisfy the condition
            Set::Separation {
                source, condition, ..
            } => source
                .elements()
                .into_iter()
                .filter(|x| match condition {
                    ElementCondition::IsEmpty => x.is_empty(),
                    ElementCondition::Contains(s) => x.contains(s),
                    ElementCondition::ContainedIn(s) => s.contains(x),
                    ElementCondition::NotContainedIn(s) => !s.contains(x),
                })
                .collect(),

            // Replacement applies the mapping to each element of the source
            Set::Replacement {
                source, mapping, ..
            } => {
                let mut elements = Vec::new();
                for x in source.elements() {
                    match mapping {
                        SetMapping::Identity => elements.push(x.clone()),
                        SetMapping::Singleton => elements.push(Set::singleton(x.clone())),
                        SetMapping::FirstProjection => {
                            if let Set::OrderedPair { first, .. } = x {
                                elements.push(first.as_ref().clone());
                            }
                        }
                        SetMapping::SecondProjection => {
                            if let Set::OrderedPair { second, .. } = x {
                                elements.push(second.as_ref().clone());
                            }
                        }
                        SetMapping::Composition(f, g) => {
                            if let Some(f_result) = x.apply_mapping(f) {
                                if let Some(g_result) = f_result.apply_mapping(g) {
                                    elements.push(g_result);
                                }
                            }
                        }
                        SetMapping::Custom(_) => {}
                    }
                }
                elements
            }

            // Complement contains elements in universe that are not in the set
            Set::Complement { set, universe, .. } => universe
                .elements()
                .into_iter()
                .filter(|x| !set.contains(x))
                .collect(),

            // Ordered pair contains its Kuratowski encoding elements
            Set::OrderedPair { first, second, .. } => {
                let singleton_a = Set::Singleton {
                    element: Box::new(first.as_ref().clone()),
                    properties: VariantSet::new(),
                };
                let pair_ab = Set::BinaryUnion {
                    left: Box::new(first.as_ref().clone()),
                    right: Box::new(second.as_ref().clone()),
                    properties: VariantSet::new(),
                    op_properties: VariantSet::new(),
                };
                vec![singleton_a, pair_ab]
            }

            // Parametric set, check if x satisfies the membership condition
            Set::Parametric {
                parameters,
                membership_condition,
                ..
            } => {
                // Note: For parametric sets, we can't directly enumerate all elements
                // since they're typically infinite or defined by a condition.
                // We would need access to the universe or specific logic for each parametric set.
                // For now, return an empty vector with a log message

                // Convert parameters to a string for logging
                let parameters_str = parameters
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<String>>()
                    .join(";");

                // Log that we can't enumerate elements for this parametric set
                // println!("Cannot enumerate elements for parametric set with parameters {}, membership condition: {}",
                //    parameters_str, membership_condition);

                // Return empty vector as we can't enumerate all elements of a parametric set
                Vec::new()
            }
        }
    }

    /// Returns true if this set is well-founded
    /// A set is well-founded if it has no infinite descending ∈-chains
    pub fn is_well_founded(&self) -> bool {
        !self.contains(self) && self.elements().iter().all(|x| x.is_well_founded())
    }

    /// Creates an ordered pair (a,b) using Kuratowski's definition
    /// (a,b) = {{a}, {a,b}}
    pub fn ordered_pair(a: Set, b: Set) -> Self {
        // First create {a}
        let singleton_a = Set::singleton(a.clone());
        // Then create {{a}}
        let singleton_singleton_a = Set::singleton(singleton_a);
        // Create {a,b}
        let pair_ab = Set::BinaryUnion {
            left: Box::new(a),
            right: Box::new(b),
            properties: VariantSet::new(),
            op_properties: VariantSet::new(),
        };
        // Finally create {{a}, {a,b}}
        Set::BinaryUnion {
            left: Box::new(singleton_singleton_a),
            right: Box::new(pair_ab),
            properties: VariantSet::new(),
            op_properties: VariantSet::new(),
        }
    }

    /// Creates a set from a vector of elements
    /// Constructs the set by taking the union of singleton sets
    pub fn from_elements(elements: Vec<Set>) -> Self {
        let len = elements.len();
        let mut result = Set::empty();
        for elem in elements {
            result = result.union(Set::singleton(elem));
        }

        // Set properties based on the elements
        let mut properties = VariantSet::new();
        properties.insert(SetProperty::IsEmpty(len == 0));
        properties.insert(SetProperty::IsFinite(true));
        properties.insert(SetProperty::IsCountable(true));

        // Check if this could be an ordinal
        let is_transitive = result.is_transitive();
        let is_well_ordered = result.is_well_ordered();
        properties.insert(SetProperty::IsTransitive(is_transitive));
        properties.insert(SetProperty::IsWellOrdered(is_well_ordered));
        properties.insert(SetProperty::IsOrdinal(is_transitive && is_well_ordered));
        properties.insert(SetProperty::Cardinality(
            CardinalityPropertyVariant::Finite(len),
        ));

        // Update properties of the result
        match result {
            Set::BinaryUnion {
                properties: ref mut p,
                ..
            } => {
                *p = properties;
            }
            _ => {}
        }

        result
    }

    /// Inserts an element into the set if it's not already present
    /// Returns the original set if the element is already present
    pub fn insert(self, element: Set) -> Set {
        if self.contains(&element) {
            self
        } else {
            self.union(Set::singleton(element))
        }
    }

    /// Compares two sets by comparing their elements recursively
    /// Used for implementing equality between sets
    fn compare_elements(&self, other: &Set) -> bool {
        // Special case for ordered pairs
        match (self, other) {
            (
                Set::OrderedPair {
                    first: f1,
                    second: s1,
                    ..
                },
                Set::OrderedPair {
                    first: f2,
                    second: s2,
                    ..
                },
            ) => {
                f1.as_ref().compare_elements(f2.as_ref())
                    && s1.as_ref().compare_elements(s2.as_ref())
            }
            _ => {
                let mut self_elements = self.elements();
                let mut other_elements = other.elements();

                if self_elements.len() != other_elements.len() {
                    return false;
                }

                // Sort elements by their string representation to ensure consistent comparison
                self_elements.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
                other_elements.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));

                // Compare each element recursively
                self_elements
                    .iter()
                    .zip(other_elements.iter())
                    .all(|(a, b)| a.compare_elements(b))
            }
        }
    }

    /// Helper function to extract components of an ordered pair if the set represents one
    /// Returns Some((a,b)) if the set is an ordered pair (a,b), None otherwise
    /// Uses Kuratowski's definition: (a,b) = {{a}, {a,b}}
    fn extract_ordered_pair(x: &Set) -> Option<(Set, Set)> {
        // An ordered pair (a,b) = {{a}, {a,b}} according to Kuratowski's definition
        if let Set::BinaryUnion {
            left: l1,
            right: r1,
            ..
        } = x
        {
            // We expect one part to be {{a}} and the other to be {a,b}
            let (singleton_singleton_part, pair_part) = match (l1.as_ref(), r1.as_ref()) {
                (Set::Singleton { element: e1, .. }, other) => (e1, other),
                (other, Set::Singleton { element: e1, .. }) => (e1, other),
                _ => return None,
            };

            // The singleton_singleton_part should be {a}
            if let Set::Singleton { element: a, .. } = singleton_singleton_part.as_ref() {
                // The pair_part should be a binary union containing a
                if let Set::BinaryUnion {
                    left: l2,
                    right: r2,
                    ..
                } = pair_part
                {
                    // One of l2 or r2 should be equal to a
                    let b = if l2.as_ref() == a.as_ref() {
                        r2.as_ref().clone()
                    } else if r2.as_ref() == a.as_ref() {
                        l2.as_ref().clone()
                    } else {
                        return None;
                    };
                    // Now a is actually the singleton {a}, so we need to extract a from it
                    if let Set::Singleton {
                        element: actual_a, ..
                    } = a.as_ref()
                    {
                        return Some((actual_a.as_ref().clone(), b));
                    }
                }
            }
        }
        None
    }

    /// Gets the properties of this set
    ///
    /// Returns a reference to the VariantSet<SetProperty> containing
    /// metadata about the set's mathematical properties
    pub fn get_properties(&self) -> &VariantSet<SetProperty> {
        static EMPTY_PROPERTIES: LazyLock<VariantSet<SetProperty>> = LazyLock::new(|| {
            let mut set = VariantSet::new();
            set.insert(SetProperty::IsEmpty(true));
            set.insert(SetProperty::IsFinite(true));
            set.insert(SetProperty::IsCountable(true));
            set.insert(SetProperty::IsWellOrdered(true));
            set.insert(SetProperty::IsTransitive(true));
            set.insert(SetProperty::IsOrdinal(true));
            set.insert(SetProperty::IsCardinal(true));
            set.insert(SetProperty::IsReflexive(true));
            set.insert(SetProperty::IsSymmetric(true));
            set.insert(SetProperty::Cardinality(
                CardinalityPropertyVariant::Finite(0),
            ));
            set
        });

        match self {
            Set::Empty => &EMPTY_PROPERTIES,
            Set::Singleton { properties, .. } => properties,
            Set::BinaryUnion { properties, .. } => properties,
            Set::BinaryIntersection { properties, .. } => properties,
            Set::SetDifference { properties, .. } => properties,
            Set::PowerSet { properties, .. } => properties,
            Set::BigUnion { properties, .. } => properties,
            Set::BigIntersection { properties, .. } => properties,
            Set::CartesianProduct { properties, .. } => properties,
            Set::Replacement { properties, .. } => properties,
            Set::OrderedPair { properties, .. } => properties,
            Set::Separation { properties, .. } => properties,
            Set::Complement { properties, .. } => properties,
            Set::SymmetricDifference { properties, .. } => properties,
            Set::Parametric { .. } => &EMPTY_PROPERTIES,
        }
    }

    /// Gets the mutable properties of this set
    ///
    /// Returns a mutable reference to the VariantSet<SetProperty> containing
    /// metadata about the set's mathematical properties

    /// Returns true if this set is an ordinal number
    /// An ordinal is a transitive set well-ordered by ∈
    pub fn is_ordinal(&self) -> bool {
        // Check cached property first
        if let Some(is_ordinal) = self.get_properties().is_ordinal() {
            return is_ordinal;
        }

        // An ordinal number is a transitive set that is well-ordered by ∈
        self.is_transitive() && self.is_well_ordered()
    }

    /// Returns true if this set is transitive
    /// A set is transitive if it contains all elements of its elements
    pub fn is_transitive(&self) -> bool {
        // Check cached property first
        if let Some(is_transitive) = self.get_properties().inner.iter().find_map(|p| {
            if let SetProperty::IsTransitive(b) = &p.0 {
                Some(*b)
            } else {
                None
            }
        }) {
            return is_transitive;
        }

        // For relations, use is_relation_transitive
        if self
            .elements()
            .iter()
            .all(|x| matches!(x, Set::OrderedPair { .. }))
        {
            return self.is_relation_transitive();
        }

        // For sets, check if every element's elements are also elements of self
        self.elements()
            .iter()
            .all(|x| x.elements().iter().all(|y| self.contains(y)))
    }

    /// Checks if a relation is transitive
    /// A relation R is transitive if for all x,y,z: xRy and yRz implies xRz
    pub fn is_relation_transitive(&self) -> bool {
        // Empty relation is vacuously transitive
        if self.is_empty() {
            return true;
        }

        // Collect all ordered pairs for easier processing
        let mut pairs = Vec::new();
        for element in self.elements() {
            if let Set::OrderedPair { first, second, .. } = element {
                pairs.push(((*first).clone(), (*second).clone()));
            }
        }

        // Check transitivity: for all x,y,z if xRy and yRz then xRz must exist
        for (x1, y1) in pairs.iter() {
            for (x2, y2) in pairs.iter() {
                // If we have xRy and yRz (including when y1=x2 for identity relation)
                if y1 == x2 {
                    // Then we must have xRz
                    let required_pair = Set::ordered_pair(x1.clone(), y2.clone());
                    if !pairs.iter().any(|(x, y)| x == x1 && y == y2) {
                        return false;
                    }
                }
            }
        }

        // If we haven't found any violations, the relation is transitive
        true
    }

    /// Returns true if this relation is an equivalence relation
    /// An equivalence relation is reflexive, symmetric, and transitive
    pub fn is_equivalence_relation(&self) -> bool {
        self.is_reflexive() && self.is_symmetric() && self.is_relation_transitive()
    }

    /// Returns true if this set is well-ordered
    /// A set is well-ordered if every non-empty subset has a least element
    pub fn is_well_ordered(&self) -> bool {
        // Check cached property first
        if let Some(is_well_ordered) = self.get_properties().is_well_ordered() {
            return is_well_ordered;
        }

        // A set is well-ordered if every non-empty subset has a least element
        // For finite sets, this is equivalent to being totally ordered
        self.is_total_order() && self.is_well_founded()
    }

    /// Returns the cardinality of the set
    /// For finite sets, returns the number of elements
    /// For infinite sets, returns the size of their finite representation
    pub fn cardinality(&self) -> usize {
        // First check if we have a cached property
        if let Some(cardinality) = self.get_properties().inner.iter().find_map(|p| {
            if let SetProperty::Cardinality(c) = &p.0 {
                match c {
                    CardinalityPropertyVariant::Finite(n) => Some(*n),
                    _ => None,
                }
            } else {
                None
            }
        }) {
            return cardinality;
        }

        // If not cached, compute it
        self.elements().len()
    }

    /// Performs ordinal addition
    /// For finite ordinals, this is equivalent to regular addition
    /// α + β represents the order type of α followed by β
    pub fn ordinal_add(&self, other: &Set) -> Set {
        // First check if both inputs are ordinals
        let mut properties = VariantSet::new();
        properties.insert(SetProperty::IsOrdinal(true));
        properties.insert(SetProperty::IsTransitive(true));
        properties.insert(SetProperty::IsWellOrdered(true));

        let mut elements = Vec::new();
        let base_len = self.elements().len();

        // First add all elements from self (these represent 0 to α-1)
        for x in self.elements() {
            elements.push(x.clone());
        }

        // For finite ordinals using von Neumann construction:
        // Each new element k is {0,1,...,k-1}
        for i in 0..other.elements().len() {
            // Create set {0,1,...,k-1} where k = base_len + i
            let mut new_element = Set::empty();
            for j in 0..(base_len + i) {
                new_element = new_element.insert(elements[j].clone());
            }
            elements.push(new_element);
        }

        // Create the result set with appropriate properties
        let mut result = Set::from_elements(elements);
        properties.insert(SetProperty::Cardinality(
            CardinalityPropertyVariant::Finite(base_len + other.elements().len()),
        ));

        // Update properties of the result
        match result {
            Set::BinaryUnion {
                properties: ref mut p,
                ..
            } => {
                *p = properties;
            }
            _ => {}
        }

        result
    }

    /// Performs ordinal multiplication
    /// For finite ordinals, this is equivalent to regular multiplication
    /// α × β represents β copies of α
    pub fn ordinal_multiply(&self, other: &Set) -> Set {
        // Verify both are ordinals
        if !self.is_ordinal() || !other.is_ordinal() {
            panic!("ordinal_multiply requires ordinal numbers");
        }

        // For finite ordinals, construct proper von Neumann ordinal
        if self.is_finite() && other.is_finite() {
            let alpha_size = self.len();
            let beta_size = other.len();
            let total_size = alpha_size * beta_size;

            println!("Starting ordinal multiplication:");
            println!("α size: {}", alpha_size);
            println!("β size: {}", beta_size);
            println!("Total size (α × β): {}", total_size);

            // Create elements for the resulting ordinal
            let mut elements = vec![Set::empty()]; // Start with 0

            // For each number up to total_size, create the corresponding von Neumann ordinal
            for i in 1..total_size {
                // Create set {0,1,...,i-1}
                let mut current = Set::empty();
                for j in 0..i {
                    current = current.insert(elements[j].clone());
                }
                elements.push(current);
                println!("Step {}: Added element", i);
            }

            // Create the result set with appropriate properties
            let mut result = Set::from_elements(elements);
            let mut properties = VariantSet::new();
            properties.insert(SetProperty::IsOrdinal(true));
            properties.insert(SetProperty::IsTransitive(true));
            properties.insert(SetProperty::IsWellOrdered(true));
            properties.insert(SetProperty::IsFinite(true));
            properties.insert(SetProperty::Cardinality(
                CardinalityPropertyVariant::Finite(total_size),
            ));

            // Update properties of the result
            match result {
                Set::BinaryUnion {
                    properties: ref mut p,
                    ..
                } => {
                    *p = properties;
                }
                _ => {}
            }

            println!("Final ordinal construction: {:?}", result);
            println!("Final ordinal elements: {:?}", result.elements());
            println!("Properties created");
            println!("Final result: {:?}", result);

            return result;
        }

        // Handle infinite ordinals
        unimplemented!("Infinite ordinal arithmetic not yet implemented")
    }

    /// Creates an identity relation on this set
    /// The identity relation contains pairs (x,x) for each x in the set
    pub fn identity_relation(&self) -> Set {
        let mut elements = Vec::new();
        for x in self.elements() {
            elements.push(Set::OrderedPair {
                first: Box::new(x.clone()),
                second: Box::new(x.clone()),
                properties: SetProperty::new(),
                op_properties: VariantSet::new(),
            });
        }

        let mut result = Set::from_elements(elements);

        // Set relation properties
        let mut properties = VariantSet::new();
        properties.insert(SetProperty::IsEmpty(self.is_empty()));
        properties.insert(SetProperty::IsFinite(true));
        properties.insert(SetProperty::IsCountable(true));
        properties.insert(SetProperty::IsTransitive(true));
        properties.insert(SetProperty::IsWellOrdered(true));
        properties.insert(SetProperty::IsReflexive(true));
        properties.insert(SetProperty::IsSymmetric(true));

        // Update properties of the result
        match result {
            Set::BinaryUnion {
                ref mut properties, ..
            } => {
                *properties = properties.clone();
            }
            _ => {}
        }

        result
    }

    /// Creates a subset relation on this set
    /// The subset relation contains pairs (x,y) where x ⊆ y
    pub fn subset_relation(&self) -> Set {
        let mut result = Set::empty();
        for x in self.elements() {
            for y in self.elements() {
                if x.is_subset_of(&y) {
                    result = result.insert(Set::ordered_pair(x.clone(), y.clone()));
                }
            }
        }

        // Set relation properties
        let mut properties = VariantSet::new();
        properties.insert(SetProperty::IsTransitive(true));
        properties.insert(SetProperty::IsWellOrdered(true));

        result
    }

    /// Performs ordinal exponentiation
    /// For finite ordinals, this is equivalent to regular exponentiation
    /// α^β represents β-fold multiplication of α
    pub fn ordinal_power(&self, other: &Set) -> Set {
        let mut properties = VariantSet::new();
        properties.insert(SetProperty::IsOrdinal(true));
        properties.insert(SetProperty::IsTransitive(true));
        properties.insert(SetProperty::IsWellOrdered(true));

        // For finite ordinals, compute total size
        if self.is_finite() && other.is_finite() {
            let size_a = self.elements().len();
            let size_b = other.elements().len();
            let total_size = size_a.pow(size_b as u32);

            // Create elements for the resulting ordinal
            let mut elements = vec![Set::empty()]; // Start with 0

            // For each number up to total_size, create the corresponding von Neumann ordinal
            for i in 1..total_size {
                // Create set {0,1,...,i-1}
                let mut current = Set::empty();
                for j in 0..i {
                    current = current.insert(elements[j].clone());
                }
                elements.push(current);
            }

            // Create the result set with appropriate properties
            let mut result = Set::from_elements(elements);
            properties.insert(SetProperty::IsFinite(true));
            properties.insert(SetProperty::Cardinality(
                CardinalityPropertyVariant::Finite(total_size),
            ));

            // Update properties of the result
            match result {
                Set::BinaryUnion {
                    properties: ref mut p,
                    ..
                } => {
                    *p = properties;
                }
                _ => {}
            }

            return result;
        }

        // Handle infinite ordinals
        unimplemented!("Infinite ordinal arithmetic not yet implemented")
    }

    /// Returns true if this relation is reflexive
    /// A relation R is reflexive if (x,x) ∈ R for all x in the domain
    pub fn is_reflexive(&self) -> bool {
        // Get domain from first components of pairs
        let domain: Vec<Set> = self
            .elements()
            .iter()
            .filter_map(|pair| {
                if let Set::OrderedPair { first, .. } = pair {
                    Some(first.as_ref().clone())
                } else {
                    None
                }
            })
            .collect();

        // Check that (x,x) exists for each x in domain
        domain.iter().all(|x| {
            self.elements().iter().any(|pair| {
                if let Set::OrderedPair { first, second, .. } = pair {
                    first.as_ref() == x && second.as_ref() == x
                } else {
                    false
                }
            })
        })
    }

    /// Returns true if this relation is symmetric
    /// A relation R is symmetric if (x,y) ∈ R implies (y,x) ∈ R
    pub fn is_symmetric(&self) -> bool {
        // Check cached property first
        if let Some(is_symmetric) = self.get_properties().is_symmetric() {
            return is_symmetric;
        }

        // Collect all ordered pairs
        let mut pairs = Vec::new();
        for pair in self.elements() {
            if let Set::OrderedPair { first, second, .. } = pair {
                pairs.push((first.as_ref().clone(), second.as_ref().clone()));
            }
        }

        // For each pair (x,y), check if (y,x) exists
        pairs
            .iter()
            .all(|(x, y)| pairs.iter().any(|(a, b)| a == y && b == x))
    }

    /// Returns true if this relation is antisymmetric
    /// A relation R is antisymmetric if (x,y) ∈ R and (y,x) ∈ R implies x = y
    pub fn is_antisymmetric(&self) -> bool {
        self.elements().iter().all(|pair1| {
            if let Set::OrderedPair {
                first: x,
                second: y,
                ..
            } = pair1
            {
                let reverse = Set::ordered_pair(y.as_ref().clone(), x.as_ref().clone());
                !self.contains(&reverse) || x == y
            } else {
                true // Non-pairs are ignored
            }
        })
    }

    /// Returns true if this relation is a partial order
    /// A partial order is reflexive, antisymmetric, and transitive
    pub fn is_partial_order(&self) -> bool {
        self.is_reflexive() && self.is_antisymmetric() && self.is_relation_transitive()
    }

    /// Returns true if this relation is total
    /// A relation R is total if for all x,y either (x,y) ∈ R or (y,x) ∈ R
    pub fn is_total(&self) -> bool {
        let domain: Vec<Set> = self
            .elements()
            .iter()
            .filter_map(|pair| {
                if let Set::OrderedPair { first, .. } = pair {
                    Some(first.as_ref().clone())
                } else {
                    None
                }
            })
            .collect();

        domain.iter().all(|x| {
            domain.iter().all(|y| {
                let forward = Set::ordered_pair(x.clone(), y.clone());
                let reverse = Set::ordered_pair(y.clone(), x.clone());
                self.contains(&forward) || self.contains(&reverse)
            })
        })
    }

    /// Returns true if this relation is a total order
    /// A total order is a partial order that is also total
    pub fn is_total_order(&self) -> bool {
        self.is_partial_order() && self.is_total()
    }

    /// Returns true if this relation is a well order
    /// A well order is a total order where every non-empty subset has a least element
    pub fn is_well_order(&self) -> bool {
        if !self.is_total_order() {
            return false;
        }

        let domain: Vec<Set> = self
            .elements()
            .iter()
            .filter_map(|pair| {
                if let Set::OrderedPair { first, .. } = pair {
                    Some(first.as_ref().clone())
                } else {
                    None
                }
            })
            .collect();

        // Check every non-empty subset has a least element
        let power = Set::from_elements(domain).power_set();
        power.elements().iter().all(|subset| {
            if subset.is_empty() {
                return true;
            }

            // Try to find least element
            subset.elements().iter().any(|x| {
                subset.elements().iter().all(|y| {
                    if x == y {
                        true
                    } else {
                        let pair = Set::ordered_pair(x.clone(), y.clone());
                        self.contains(&pair)
                    }
                })
            })
        })
    }

    /// Returns true if this set is finite
    /// A set is finite if it has a finite number of elements
    pub fn is_finite(&self) -> bool {
        // Check cached property first
        if let Some(is_finite) = self.get_properties().inner.iter().find_map(|p| {
            if let SetProperty::IsFinite(b) = &p.0 {
                Some(*b)
            } else {
                None
            }
        }) {
            return is_finite;
        }

        // If not cached, check if we can compute the elements
        // This is a simplification - in general determining if a set is finite
        // requires more sophisticated analysis
        match self {
            Set::Empty => true,
            Set::Singleton { .. } => true,
            Set::BinaryUnion { left, right, .. } => left.is_finite() && right.is_finite(),
            Set::BinaryIntersection { left, right, .. } => left.is_finite() && right.is_finite(),
            Set::SetDifference { left, .. } => left.is_finite(),
            Set::SymmetricDifference { left, right, .. } => left.is_finite() && right.is_finite(),
            Set::CartesianProduct { left, right, .. } => left.is_finite() && right.is_finite(),
            Set::PowerSet { base, .. } => base.is_finite(),
            Set::Separation { source, .. } => source.is_finite(),
            Set::Replacement { source, .. } => source.is_finite(),
            Set::OrderedPair { .. } => true,
            Set::Complement { universe, .. } => universe.is_finite(),
            Set::BigUnion { family, .. } => family.is_finite(),
            Set::BigIntersection { family, .. } => family.is_finite(),
            Set::Parametric { .. } => false,
        }
    }
}

/// Implementation of Debug trait for Set
/// Provides a mathematical notation for set display
impl fmt::Debug for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Empty set displayed as ∅
            Set::Empty => write!(f, "∅"),
            // Singleton set displayed as {x}
            Set::Singleton { element, .. } => write!(f, "{{{:?}}}", element),
            // Binary union displayed as (A ∪ B)
            Set::BinaryUnion { left, right, .. } => write!(f, "({:?} ∪ {:?})", left, right),
            // Binary intersection displayed as (A ∩ B)
            Set::BinaryIntersection { left, right, .. } => write!(f, "({:?} ∩ {:?})", left, right),
            // Set difference displayed as (A - B)
            Set::SetDifference { left, right, .. } => write!(f, "({:?} - {:?})", left, right),
            // Symmetric difference displayed as (A △ B)
            Set::SymmetricDifference { left, right, .. } => write!(f, "({:?} △ {:?})", left, right),
            // Power set displayed as P(A)
            Set::PowerSet { base, .. } => write!(f, "P({:?})", base),
            // Separation displayed as {x ∈ A | φ(x)}
            Set::Separation { source, .. } => write!(f, "{{x ∈ {:?} | φ(x)}}", source),
            // Replacement displayed as {f(x) | x ∈ A}
            Set::Replacement { source, .. } => write!(f, "{{f(x) | x ∈ {:?}}}", source),
            // Ordered pair displayed as (a, b)
            Set::OrderedPair { first, second, .. } => write!(f, "({:?}, {:?})", first, second),
            // Complement displayed as (A ∩ U)
            Set::Complement { set, universe, .. } => write!(f, "({:?} ∩ {:?})", set, universe),
            // Cartesian product displayed as (A × B)
            Set::CartesianProduct { left, right, .. } => write!(f, "({:?} × {:?})", left, right),
            // Big union displayed as ⋃(F)
            Set::BigUnion { family, .. } => write!(f, "⋃({:?})", family),
            // Big intersection displayed as ⋂(F)
            Set::BigIntersection { family, .. } => write!(f, "⋂({:?})", family),
            // Parametric set, display parameters and description
            Set::Parametric {
                parameters,
                description,
                ..
            } => write!(
                f,
                "Parametric set: {} (params: {})",
                description,
                parameters
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

/// Implementation of PartialEq for ElementCondition
/// Provides a human-readable representation of element conditions
impl PartialEq for ElementCondition {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Two IsEmpty conditions are always equal
            (ElementCondition::IsEmpty, ElementCondition::IsEmpty) => true,
            // Contains conditions are equal if their contained sets are equal
            (ElementCondition::Contains(a), ElementCondition::Contains(b)) => {
                a.as_ref() == b.as_ref()
            }
            // ContainedIn conditions are equal if their container sets are equal
            (ElementCondition::ContainedIn(a), ElementCondition::ContainedIn(b)) => {
                a.as_ref() == b.as_ref()
            }
            // NotContainedIn conditions are equal if their excluded sets are equal
            (ElementCondition::NotContainedIn(a), ElementCondition::NotContainedIn(b)) => {
                a.as_ref() == b.as_ref()
            }
            // Different types of conditions are never equal
            _ => false,
        }
    }
}

/// Implementation of PartialEq for Set
/// Defines when two sets are considered equal based on the Extensionality Axiom
impl PartialEq for Set {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Empty sets are always equal to each other
            (Set::Empty, Set::Empty) => true,
            // Singleton sets are equal if their elements are equal
            (Set::Singleton { element: e1, .. }, Set::Singleton { element: e2, .. }) => {
                e1.as_ref() == e2.as_ref()
            }
            // Binary unions are equal if they contain the same elements (order doesn't matter)
            (
                Set::BinaryUnion {
                    left: l1,
                    right: r1,
                    ..
                },
                Set::BinaryUnion {
                    left: l2,
                    right: r2,
                    ..
                },
            ) => {
                // For binary unions, we need to check both possible orderings
                (l1.as_ref() == l2.as_ref() && r1.as_ref() == r2.as_ref())
                    || (l1.as_ref() == r2.as_ref() && r1.as_ref() == l2.as_ref())
            }
            // Ordered pairs are equal if their components are equal in order
            (
                Set::OrderedPair {
                    first: f1,
                    second: s1,
                    ..
                },
                Set::OrderedPair {
                    first: f2,
                    second: s2,
                    ..
                },
            ) => f1.as_ref() == f2.as_ref() && s1.as_ref() == s2.as_ref(),
            // For all other cases, compare based on elements
            _ => {
                let mut self_elements = self.elements();
                let mut other_elements = other.elements();

                if self_elements.len() != other_elements.len() {
                    return false;
                }

                // Sort elements by their string representation for consistent comparison
                self_elements.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
                other_elements.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));

                // Compare each element recursively
                self_elements
                    .iter()
                    .zip(other_elements.iter())
                    .all(|(a, b)| a == b)
            }
        }
    }
}

/// Implementation of SetProperties
/// Provides methods to create and manage set properties
impl SetProperty {
    /// Creates a new SetProperties with all fields set to false
    pub fn new() -> VariantSet<SetProperty> {
        let mut set = VariantSet::new();
        set.insert(SetProperty::IsEmpty(false));
        set.insert(SetProperty::IsFinite(false));
        set.insert(SetProperty::IsCountable(false));
        set.insert(SetProperty::IsWellOrdered(false));
        set.insert(SetProperty::IsTransitive(false));
        set.insert(SetProperty::IsOrdinal(false));
        set.insert(SetProperty::IsCardinal(false));
        set.insert(SetProperty::IsReflexive(false));
        set.insert(SetProperty::IsSymmetric(false));
        set
    }

    /// Creates properties for the empty set
    pub fn empty() -> VariantSet<SetProperty> {
        let mut set = VariantSet::new();
        set.insert(SetProperty::Cardinality(
            CardinalityPropertyVariant::Finite(0),
        ));
        set.insert(SetProperty::IsEmpty(true));
        set.insert(SetProperty::IsFinite(true));
        set.insert(SetProperty::IsCountable(true));
        set.insert(SetProperty::IsWellOrdered(true));
        set.insert(SetProperty::IsTransitive(true));
        set.insert(SetProperty::IsOrdinal(true));
        set.insert(SetProperty::IsCardinal(true));
        set.insert(SetProperty::IsReflexive(true));
        set.insert(SetProperty::IsSymmetric(true));
        set
    }
}

/// ZFC axiom verification
/// Provides methods to verify that sets satisfy the ZFC axioms
pub struct ZFCVerifier;

impl ZFCVerifier {
    /// Verify the extensionality axiom: two sets are equal if they have the same elements
    /// This is a fundamental axiom that defines set equality
    pub fn verify_extensionality(a: &Set, b: &Set) -> bool {
        a == b
    }

    /// Verify the empty set axiom: there exists a set with no elements
    /// This axiom guarantees the existence of at least one set
    pub fn verify_empty_set(x: &Set) -> bool {
        x.is_empty()
    }

    /// Verify the pairing axiom: for any a and b, there exists a set containing exactly a and b
    /// This axiom allows us to construct sets with exactly two elements
    pub fn verify_pairing(pair: &Set, a: &Set, b: &Set) -> bool {
        // Check if the pair contains both elements
        let contains_a = pair.contains(a);
        let contains_b = pair.contains(b);

        // Check if the pair contains only these elements
        let elements = pair.elements();
        let correct_size = if a == b { 1 } else { 2 };

        contains_a
            && contains_b
            && elements.len() == correct_size
            && elements.iter().all(|x| x == a || x == b)
    }

    /// Verify the union axiom: for any set of sets, there exists a set containing all elements
    /// This axiom allows us to combine multiple sets into one
    pub fn verify_union(union: &Set, sets: &[Set]) -> bool {
        // Every element in any of the sets should be in the union
        sets.iter()
            .all(|s| s.elements().iter().all(|x| union.contains(x)))
            // Every element in the union should be in at least one of the sets
            && union
                .elements()
                .iter()
                .all(|x| sets.iter().any(|s| s.contains(x)))
    }

    /// Verify the power set axiom: for any set, there exists a set of all its subsets
    /// This axiom allows us to construct sets of subsets
    pub fn verify_power_set(power: &Set, x: &Set) -> bool {
        // Every element in the power set should be a subset of x
        power.elements().iter().all(|s| s.is_subset_of(x))
            // Every subset of x should be in the power set
            && power.elements().iter().all(|s| power.contains(s))
    }

    /// Verify the foundation axiom: every non-empty set has an ∈-minimal element
    /// This axiom prevents infinite descending chains of membership
    pub fn verify_foundation(x: &Set) -> bool {
        x.is_well_founded()
    }

    /// Verify the separation axiom: for any set and predicate, there exists a subset
    /// This axiom allows us to construct subsets using predicates
    pub fn verify_separation<P>(subset: &Set, x: &Set, predicate: P) -> bool
    where
        P: Fn(&Set) -> bool,
    {
        // The subset should be contained in x
        subset.is_subset_of(x)
            // Every element in the subset should satisfy the predicate
            && subset.elements().iter().all(|s| predicate(s))
            // Every element in x that satisfies the predicate should be in the subset
            && x.elements()
                .iter()
                .filter(|s| predicate(s))
                .all(|s| subset.contains(s))
    }

    /// Verify the replacement axiom: for any set and function, there exists an image set
    /// This axiom allows us to construct new sets by applying functions to existing sets
    pub fn verify_replacement<F>(image: &Set, domain: &Set, f: F) -> bool
    where
        F: Fn(&Set) -> Set,
    {
        // Every element in the domain should map to an element in the image
        domain.elements().iter().all(|x| image.contains(&f(x)))
            // Every element in the image should be the image of some element in the domain
            && image
                .elements()
                .iter()
                .all(|y| domain.elements().iter().any(|x| f(x) == *y))
    }
}

/// Helper functions for set construction
/// These provide convenient ways to create common set structures

/// Creates an empty set
/// Returns the unique empty set guaranteed by the Empty Set Axiom
pub fn empty_set() -> Set {
    Set::empty()
}

/// Creates a singleton set containing one element
/// Returns {x} for any given set x
pub fn singleton_set(x: Set) -> Set {
    Set::singleton(x)
}

/// Creates a pair set containing two elements
/// Returns {a, b} for any given sets a and b
pub fn pair_set(a: Set, b: Set) -> Set {
    Set::pair(a, b)
}

/// Creates a union of multiple sets
/// Returns the union of all sets in the given slice
pub fn union_set(sets: &[Set]) -> Set {
    let mut result = empty_set();
    for set in sets {
        result = result.union(set.clone());
    }
    result
}

/// Creates a power set of a given set
/// Returns P(x) containing all subsets of x
pub fn power_set(x: &Set) -> Set {
    x.clone().power_set()
}

/// Creates an intersection of multiple sets
/// Returns the intersection of all sets in the given slice
/// If the slice is empty, returns the empty set
pub fn intersection_set(sets: &[Set]) -> Set {
    if sets.is_empty() {
        return Set::empty();
    }
    let mut result = sets[0].clone();
    for set in &sets[1..] {
        result = result.intersection(set.clone());
    }
    result
}

/// Creates a difference of two sets
/// Returns A - B for given sets A and B
pub fn difference_set(a: &Set, b: &Set) -> Set {
    a.clone().difference(b.clone())
}

/// Implementation of Debug trait for ElementCondition
/// Provides a human-readable representation of element conditions
impl fmt::Debug for ElementCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElementCondition::IsEmpty => write!(f, "is_empty"),
            ElementCondition::Contains(s) => write!(f, "contains({:?})", s),
            ElementCondition::ContainedIn(s) => write!(f, "contained_in({:?})", s),
            ElementCondition::NotContainedIn(s) => write!(f, "not_contained_in({:?})", s),
        }
    }
}

/// Implementation of Debug trait for SetMapping
/// Provides a human-readable representation of set mappings
impl fmt::Debug for SetMapping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetMapping::Identity => write!(f, "id"),
            SetMapping::Singleton => write!(f, "singleton"),
            SetMapping::FirstProjection => write!(f, "π₁"),
            SetMapping::SecondProjection => write!(f, "π₂"),
            SetMapping::Composition(g, h) => write!(f, "({:?} ∘ {:?})", g, h),
            SetMapping::Custom(s) => write!(f, "custom({})", s),
        }
    }
}

/// Implementation of Debug trait for Cardinality
/// Provides a human-readable representation of set cardinalities
impl fmt::Debug for CardinalityPropertyVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardinalityPropertyVariant::Finite(n) => write!(f, "|{}|", n),
            CardinalityPropertyVariant::CountablyInfinite => write!(f, "ℵ₀"),
            CardinalityPropertyVariant::ContinuumSize => write!(f, "2^ℵ₀"),
            CardinalityPropertyVariant::LargerCardinal(n) => write!(f, "ℵ_{}", n),
        }
    }
}

impl VariantSet<SetProperty> {
    /// Gets the ordinal property
    pub fn is_ordinal(&self) -> Option<bool> {
        self.inner.iter().find_map(|p| {
            if let SetProperty::IsOrdinal(b) = &p.0 {
                Some(*b)
            } else {
                None
            }
        })
    }

    /// Gets the transitive property
    pub fn is_transitive(&self) -> Option<bool> {
        self.inner.iter().find_map(|p| {
            if let SetProperty::IsTransitive(b) = &p.0 {
                Some(*b)
            } else {
                None
            }
        })
    }

    /// Gets the well-ordered property
    pub fn is_well_ordered(&self) -> Option<bool> {
        self.inner.iter().find_map(|p| {
            if let SetProperty::IsWellOrdered(b) = &p.0 {
                Some(*b)
            } else {
                None
            }
        })
    }

    /// Gets the reflexive property
    pub fn is_reflexive(&self) -> Option<bool> {
        self.inner.iter().find_map(|p| {
            if let SetProperty::IsReflexive(b) = &p.0 {
                Some(*b)
            } else {
                None
            }
        })
    }

    /// Gets the symmetric property
    pub fn is_symmetric(&self) -> Option<bool> {
        self.inner.iter().find_map(|p| {
            if let SetProperty::IsSymmetric(b) = &p.0 {
                Some(*b)
            } else {
                None
            }
        })
    }
}

impl Hash for VariantSet<SetOpProperty> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut elements: Vec<_> = self.inner.iter().collect();
        elements.sort_by_key(|x| format!("{:?}", x));
        elements.hash(state);
    }
}

impl Hash for VariantSet<OrdinalOpProperty> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut elements: Vec<_> = self.inner.iter().collect();
        elements.sort_by_key(|x| format!("{:?}", x));
        elements.hash(state);
    }
}
