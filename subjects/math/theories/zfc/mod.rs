//! ZFC Set Theory Implementation
//!
//! This module provides a foundational implementation of Zermelo-Fraenkel Set Theory with Choice (ZFC).
//! It serves as the basis for building other mathematical theories in a formally verified way.

pub mod axioms;
pub mod collect;
pub mod relations;
pub mod set;
pub mod verifier;

pub use axioms::{SatisfiesZFC, ZFCAxioms};
pub use relations::SetTheoryRelation;
pub use set::Set;
pub use verifier::ZFCVerifier;

// Core set operations that guarantee ZFC compliance by construction

/// Creates an empty set (∅)
#[inline]
pub fn empty_set() -> Set {
    Set::empty()
}

/// Creates a singleton set ({x})
#[inline]
pub fn singleton_set(element: Set) -> Set {
    Set::singleton(element)
}

/// Creates a pair set ({a, b})
#[inline]
pub fn pair_set(a: Set, b: Set) -> Set {
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
    set.clone().power_set()
}

/// Creates an intersection of sets (⋂ A)
#[inline]
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

/// Creates a difference of sets (A \ B)
#[inline]
pub fn difference_set(a: &Set, b: &Set) -> Set {
    a.clone().difference(b.clone())
}

/// Creates a symmetric difference of sets (A △ B)
#[inline]
pub fn symmetric_difference_set(a: &Set, b: &Set) -> Set {
    union_set(&[difference_set(a, b), difference_set(b, a)])
}

/// Creates a cartesian product of sets (A × B)
#[inline]
pub fn cartesian_product(a: &Set, b: &Set) -> Set {
    a.clone().cartesian_product(b.clone())
}

/// Creates an ordered pair ((a, b))
#[inline]
pub fn ordered_pair(a: Set, b: Set) -> Set {
    Set::ordered_pair(a, b)
}

/// Verifies if a set is well-founded
#[inline]
pub fn is_well_founded(set: &Set) -> bool {
    ZFCVerifier::verify_foundation(set)
}

/// Verifies if one set is a subset of another (A ⊆ B)
#[inline]
pub fn is_subset(subset: &Set, superset: &Set) -> bool {
    subset.is_subset_of(superset)
}

/// Verifies if one set is a proper subset of another (A ⊊ B)
#[inline]
pub fn is_proper_subset(subset: &Set, superset: &Set) -> bool {
    subset.is_proper_subset_of(superset)
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
        self.is_subset_of(other) && self != other
    }

    /// Returns an iterator over the elements of this set
    pub fn iter(&self) -> std::vec::IntoIter<Set> {
        self.elements().into_iter()
    }
}

#[cfg(test)]
mod tests;
