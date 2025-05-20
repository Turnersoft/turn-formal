use crate::subjects::math::formalism::complexity::Complexity;
use crate::subjects::math::theories::zfc::set::{Set, SetElement};

impl Complexity for Set {
    fn complexity(&self) -> usize {
        match self {
            Set::Generic { properties, .. } => 1 + properties.inner.len(),
            Set::Empty => 1,
            Set::Singleton { element, .. } => 1 + element.complexity(),
            Set::Enumeration { elements, .. } => {
                1 + elements.iter().map(|e| e.complexity()).sum::<usize>()
            }
            Set::BinaryUnion { left, right, .. } => 1 + left.complexity() + right.complexity(),
            Set::BinaryIntersection { left, right, .. } => {
                1 + left.complexity() + right.complexity()
            }
            Set::SetDifference { left, right, .. } => 1 + left.complexity() + right.complexity(),
            Set::SymmetricDifference { left, right, .. } => {
                1 + left.complexity() + right.complexity()
            }
            Set::CartesianProduct { left, right, .. } => 1 + left.complexity() + right.complexity(),
            Set::BigUnion { family, .. } => 1 + family.complexity(),
            Set::BigIntersection { family, .. } => 1 + family.complexity(),
            Set::PowerSet { base, .. } => 1 + base.complexity(),
            Set::Separation {
                source, condition, ..
            } => 1 + source.complexity(),
            Set::Replacement {
                source, mapping, ..
            } => 1 + source.complexity(),
            Set::OrderedPair { first, second, .. } => 1 + first.complexity() + second.complexity(),
            Set::Complement { set, universe, .. } => 1 + set.complexity() + universe.complexity(),
            Set::Parametric {
                parameters,
                description,
                membership_condition,
                ..
            } => 1 + parameters.len() + description.len() + membership_condition.len(),
        }
    }
}

impl Complexity for SetElement {
    fn complexity(&self) -> usize {
        match self {
            SetElement::Set(set) => 1 + set.complexity(),
            SetElement::Integer(_) => 1,
            SetElement::Symbol(_) => 1,
            SetElement::Pair(first, second) => 1 + first.complexity() + second.complexity(),
            SetElement::Urelement(_) => 1,
        }
    }
}
