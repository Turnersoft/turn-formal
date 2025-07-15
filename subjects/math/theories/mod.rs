// src/formalize_v2/subjects/math/theories/mod.rs

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::mem::discriminant;

// Core mathematical theories
pub mod algebraic_geometry;
pub mod analysis;
pub mod common;
pub mod differential_geometry;
pub mod groups;
pub mod order_theory;
pub mod rings;
pub mod symplectic_geometry;
pub mod topology;
pub mod zfc;

// Additional mathematical theories
pub mod affine_geometry;
pub mod algebra;
pub mod fields;
pub mod homology;
pub mod lie_theory;
pub mod linear_algebra;
pub mod measure;
pub mod number_theory;
pub mod probability;
pub mod projective_geometry;
pub mod representation;
pub mod riemannian_geometry;

// VariantSet implementation for property collections
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariantSet<T> {
    inner: HashSet<VariantWrapper<T>>,
}

impl<T> Hash for VariantSet<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut elements: Vec<_> = self.inner.iter().collect();
        // sort elements to get a consistent order for hashing
        elements.sort();
        elements.hash(state);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VariantWrapper<T>(T);

impl<T> PartialOrd for VariantWrapper<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for VariantWrapper<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // We can't compare discriminants directly, but we can compare their hashes.
        // This is stable enough for a consistent sort order.
        let mut s = std::collections::hash_map::DefaultHasher::new();
        discriminant(&self.0).hash(&mut s);
        let self_hash = s.finish();

        let mut s = std::collections::hash_map::DefaultHasher::new();
        discriminant(&other.0).hash(&mut s);
        let other_hash = s.finish();

        self_hash.cmp(&other_hash)
    }
}

impl<T> Hash for VariantWrapper<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        discriminant(&self.0).hash(state);
    }
}

impl<T> PartialEq for VariantWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        discriminant(&self.0) == discriminant(&other.0)
    }
}

impl<T> Eq for VariantWrapper<T> {}

impl<T: Clone> VariantSet<T> {
    pub fn new() -> Self {
        Self {
            inner: std::collections::HashSet::new(),
        }
    }

    pub fn insert(&mut self, value: T) -> Option<T> {
        if let Some(old) = self.inner.take(&VariantWrapper(value.clone())) {
            self.inner.insert(VariantWrapper(value));
            Some(old.0)
        } else {
            self.inner.insert(VariantWrapper(value));
            None
        }
    }

    pub fn contains_variant(&self, value: &T) -> bool {
        self.inner.contains(&VariantWrapper(value.clone()))
    }

    pub fn contains(&self, value: &T) -> bool {
        self.inner.contains(&VariantWrapper(value.clone()))
    }

    pub fn get(&self, value: &T) -> Option<&T> {
        self.inner
            .get(&VariantWrapper(value.clone()))
            .map(|wrapper| &wrapper.0)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter().map(|wrapper| &wrapper.0)
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.inner.is_subset(&other.inner)
    }
}

pub trait HasProperties<T> {
    fn get_properties(&self) -> &VariantSet<T>;
    fn get_properties_mut(&mut self) -> &mut VariantSet<T>;
}

/// A macro to create a `VariantSet` with initial values.
///
/// Example:
/// ```
/// let set = variant_set![MyEnum::A, MyEnum::B("hello")];
/// ```
#[macro_export]
macro_rules! variant_set {
    // Case for an empty set
    () => {
        $crate::subjects::math::theories::VariantSet::new()
    };
    // Case for a set with elements, allowing a trailing comma
    ( $( $x:expr ),* $(,)? ) => {
        {
            let mut set = $crate::subjects::math::theories::VariantSet::new();
            $(
                set.insert($x);
            )*
            set
        }
    };
}
