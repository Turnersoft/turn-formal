//! ZFC Set Theory Implementation
//!
//! This module provides a foundational implementation of Zermelo-Fraenkel Set Theory with Choice (ZFC).
//! It serves as the basis for building other mathematical theories in a formally verified way.

pub mod axioms;
pub mod collect;
pub mod render;

pub mod abstraction_level;
pub mod complexity;
pub mod definitions;
pub mod verifier;

pub use axioms::{SatisfiesZFC, ZFCAxioms};
// Keep the legacy relation export, but also export the new one
pub use definitions::{CardinalityPropertyVariant, Set, SetElement, SetExpression, SetRelation};
pub use verifier::ZFCVerifier;

// Import VariantSet directly from the theories module
use super::super::theories::VariantSet;

// Core set operations that guarantee ZFC compliance by construction

/// Creates an empty set (∅)
#[inline]
pub fn empty_set() -> Set {
    Set::empty()
}

/// Creates a singleton set ({x})
#[inline]
pub fn singleton_set(element: SetElement) -> Set {
    Set::singleton(element)
}

/// Creates a pair set ({a, b})
#[inline]
pub fn pair_set(a: SetElement, b: SetElement) -> Set {
    Set::pair(a, b)
}

/// Creates a union of sets (⋃ A)
#[inline]
pub fn union_set(sets: &[Set]) -> Set {
    let mut result = empty_set();
    for set in sets {
        result = result.union(set.clone());
    }
    result
}

/// Creates a power set (𝒫(A))
#[inline]
pub fn power_set(set: &Set) -> Set {
    Set::PowerSet {
        base: Box::new(set.clone()),
        properties: VariantSet::new(),
        op_properties: VariantSet::new(),
    }
}

/// Creates an intersection of sets (⋂ A)
#[inline]
pub fn intersection_set(sets: &[Set]) -> Set {
    if sets.is_empty() {
        return Set::empty();
    }

    // For binary intersection
    if sets.len() == 2 {
        return Set::BinaryIntersection {
            left: Box::new(sets[0].clone()),
            right: Box::new(sets[1].clone()),
            properties: VariantSet::new(),
            op_properties: VariantSet::new(),
        };
    }

    // For multiple sets, we could implement a recursive approach
    // This is a simplified implementation
    let mut result = sets[0].clone();
    for i in 1..sets.len() {
        result = Set::BinaryIntersection {
            left: Box::new(result),
            right: Box::new(sets[i].clone()),
            properties: VariantSet::new(),
            op_properties: VariantSet::new(),
        };
    }
    result
}

/// Creates a difference of sets (A \ B)
#[inline]
pub fn difference_set(a: &Set, b: &Set) -> Set {
    Set::SetDifference {
        left: Box::new(a.clone()),
        right: Box::new(b.clone()),
        properties: VariantSet::new(),
        op_properties: VariantSet::new(),
    }
}

/// Creates a symmetric difference of sets (A △ B)
#[inline]
pub fn symmetric_difference_set(a: &Set, b: &Set) -> Set {
    Set::SymmetricDifference {
        left: Box::new(a.clone()),
        right: Box::new(b.clone()),
        properties: VariantSet::new(),
        op_properties: VariantSet::new(),
    }
}

/// Creates a cartesian product of sets (A × B)
#[inline]
pub fn cartesian_product(a: &Set, b: &Set) -> Set {
    Set::CartesianProduct {
        left: Box::new(a.clone()),
        right: Box::new(b.clone()),
        properties: VariantSet::new(),
        op_properties: VariantSet::new(),
    }
}

/// Creates an ordered pair ((a, b))
#[inline]
pub fn ordered_pair(a: Set, b: Set) -> Set {
    Set::OrderedPair {
        first: Box::new(a),
        second: Box::new(b),
        properties: VariantSet::new(),
        op_properties: VariantSet::new(),
    }
}

/// Verifies if a set is well-founded
#[inline]
pub fn is_well_founded(set: &Set) -> bool {
    ZFCVerifier::verify_foundation(set)
}

/// Verifies if one set is a subset of another (A ⊆ B)
#[inline]
pub fn is_subset(subset: &Set, superset: &Set) -> bool {
    is_subset_of(subset, superset)
}

/// Helper function to check if one set is a subset of another
pub fn is_subset_of(subset: &Set, superset: &Set) -> bool {
    match subset {
        Set::Empty => true, // Empty set is a subset of everything
        Set::Singleton { element, .. } => superset.contains(element),
        Set::Enumeration { elements, .. } => elements.iter().all(|e| superset.contains(e)),
        _ => false, // Simplified implementation
    }
}

/// Verifies if one set is a proper subset of another (A ⊊ B)
#[inline]
pub fn is_proper_subset(subset: &Set, superset: &Set) -> bool {
    is_subset_of(subset, superset) && subset != superset
}

/// Error type for set operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetError {
    EmptyIntersection,
    InvalidOperation,
}

impl Set {
    /// Returns true if this set is a proper subset of another set
    pub fn is_proper_subset_of(&self, other: &Set) -> bool {
        is_subset_of(self, other) && self != other
    }

    /// Returns an iterator over the elements of this set
    pub fn elements(&self) -> Vec<SetElement> {
        match self {
            Set::Empty => Vec::new(),
            Set::Singleton { element, .. } => vec![element.clone()],
            Set::Enumeration { elements, .. } => elements.clone(),
            // For other variants, we would need to implement element extraction
            _ => Vec::new(), // Simplified implementation
        }
    }

    /// Returns the number of elements in the set
    pub fn len(&self) -> usize {
        self.elements().len()
    }

    /// Returns true if the set is empty
    pub fn is_empty(&self) -> bool {
        match self {
            Set::Empty => true,
            _ => self.elements().is_empty(),
        }
    }

    /// Returns true if this set is well-founded
    pub fn is_well_founded(&self) -> bool {
        is_well_founded(self)
    }

    /// Returns the cardinality of this set
    pub fn cardinality(&self) -> usize {
        self.len()
    }

    /// Returns true if this set is a subset of another set
    pub fn is_subset_of(&self, other: &Set) -> bool {
        is_subset_of(self, other)
    }

    /// Returns true if this set is an ordinal number
    pub fn is_ordinal(&self) -> bool {
        match self {
            Set::Empty => true, // The empty set is the ordinal 0
            // For other variants, we would need to check ordinality properties
            _ => false, // Simplified implementation
        }
    }

    /// Add two ordinals
    pub fn ordinal_add(&self, other: &Set) -> Set {
        // Simplified implementation
        Set::BinaryUnion {
            left: Box::new(self.clone()),
            right: Box::new(other.clone()),
            properties: VariantSet::new(),
            op_properties: VariantSet::new(),
        }
    }

    /// Multiply two ordinals
    pub fn ordinal_multiply(&self, other: &Set) -> Set {
        // Simplified implementation
        Set::CartesianProduct {
            left: Box::new(self.clone()),
            right: Box::new(other.clone()),
            properties: VariantSet::new(),
            op_properties: VariantSet::new(),
        }
    }

    /// Raise one ordinal to the power of another
    pub fn ordinal_power(&self, other: &Set) -> Set {
        // Simplified implementation
        Set::PowerSet {
            base: Box::new(self.clone()),
            properties: VariantSet::new(),
            op_properties: VariantSet::new(),
        }
    }

    /// Create a symmetric difference of two sets
    pub fn symmetric_difference(&self, other: Set) -> Set {
        symmetric_difference_set(self, &other)
    }

    /// Create an identity relation on this set
    pub fn identity_relation(&self) -> Set {
        // Simplified implementation
        Set::Empty // Placeholder
    }

    /// Create a subset relation on this set
    pub fn subset_relation(&self) -> Set {
        // Simplified implementation
        Set::Empty // Placeholder
    }
}

#[cfg(test)]
mod tests;
