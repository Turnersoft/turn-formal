use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::theories::VariantSet;
use crate::subjects::math::theories::zfc::definitions::{
    CardinalityPropertyVariant, Set, SetElement, SetExpression, SetProperty, SetRelation,
};

/// Implementation of abstraction levels for Set Theory objects

impl GetAbstractionLevel for Set {
    fn level(&self) -> AbstractionLevel {
        match self {
            Set::Generic(gs) => {
                // L1: Abstract schema (e.g., "Set S" with no constraining properties)
                // L2: A specific "type" of set defined by properties (e.g., "Finite Set F")
                if gs.properties.inner.is_empty() {
                    AbstractionLevel::Level1
                } else {
                    // Any property makes it a more specific type of set.
                    AbstractionLevel::Level2
                }
            }
            Set::Empty => AbstractionLevel::Level4,
            // The WellKnown variant has been removed from the Set enum
            // Set::WellKnown(..) => AbstractionLevel::Level4,
            Set::Singleton { element, .. } => {
                match element.level() {
                    AbstractionLevel::Level4 => AbstractionLevel::Level4, // e.g. {1}
                    // If element is L1/L2/L3, the singleton represents a more abstract concept
                    // like "{abstract_x}" or "{result_of_L3_construction}". This is an L2 "type" of set.
                    _ => AbstractionLevel::Level2,
                }
            }
            Set::Enumeration { elements, .. } => {
                if elements.is_empty() {
                    return AbstractionLevel::Level4; // = Set::Empty
                }
                let mut overall_level = AbstractionLevel::Level4;
                for el in elements {
                    let el_level = el.level();
                    if el_level < overall_level {
                        overall_level = el_level;
                    }
                }
                overall_level // Level is the most abstract of its elements
            }

            // L3 Constructor Variants: These define a rule or template.
            // Their `properties` field describes the L2 nature of the set they *define*.
            Set::BinaryUnion { .. }
            | Set::BinaryIntersection { .. }
            | Set::SetDifference { .. }
            | Set::SymmetricDifference { .. }
            | Set::CartesianProduct { .. }
            | Set::BigUnion { .. }
            | Set::BigIntersection { .. }
            | Set::PowerSet { .. }
            | Set::Separation { .. }
            | Set::Replacement { .. }
            | Set::OrderedPair { .. }
            | Set::Complement { .. } => AbstractionLevel::Level3,

            // **FIXED**: Parametric sets can be L1 schemas when they have no specific parameters
            Set::Parametric { parameters, .. } => {
                if parameters.is_empty() {
                    AbstractionLevel::Level1 // Abstract schema like "x ∈ G"
                } else {
                    AbstractionLevel::Level3 // Parametrized construction
                }
            }
            // Note: Parametric, NaturalNumbers, Integers, etc., variants were removed from Set enum.
            // If any new variants are added, they need to be handled here.
        }
    }
}

impl GetAbstractionLevel for SetElement {
    fn level(&self) -> AbstractionLevel {
        match self {
            SetElement::Set(s_box) => s_box.level(),
            SetElement::Integer(_) => AbstractionLevel::Level4,
            SetElement::Symbol(_) => AbstractionLevel::Level4,
            SetElement::Pair(e1, e2) => {
                let l1 = e1.level();
                let l2 = e2.level();
                if l1 < l2 { l1 } else { l2 } // Assumes Ord for AbstractionLevel (L1 < L2 < L3 < L4)
            }
            SetElement::Urelement(_) => AbstractionLevel::Level4,
        }
    }
}

impl GetAbstractionLevel for SetExpression {
    fn level(&self) -> AbstractionLevel {
        match self {
            // Variables or parameters in expressions are Level 2 (generic but with constraints)
            SetExpression::Variable(_) => AbstractionLevel::Level2,

            // Expressions that compute properties rather than create new sets
            SetExpression::Cardinality { set: _ } => AbstractionLevel::Level4,
            SetExpression::ElementSelection { set: _ } => AbstractionLevel::Level4,

            // Default for other expressions
            _ => AbstractionLevel::Level4,
        }
    }
}

impl GetAbstractionLevel for SetRelation {
    fn level(&self) -> AbstractionLevel {
        match self {
            // Generic relations that define sets are Level 2
            SetRelation::Equals { .. } => AbstractionLevel::Level2,
            SetRelation::ElementOf { .. } => AbstractionLevel::Level2,
            SetRelation::SubsetOf { .. } => AbstractionLevel::Level2,

            // Constraints on sets that still allow for generic sets are Level 2
            SetRelation::HasCardinality { .. } => AbstractionLevel::Level2,

            // Relations that give rise to specific set instances are Level 3
            SetRelation::IsEmpty { .. } => AbstractionLevel::Level3,
            SetRelation::IsFinite { .. } => AbstractionLevel::Level3,
            SetRelation::IsCountable { .. } => AbstractionLevel::Level3,

            // Default for other relations
            _ => AbstractionLevel::Level2,
        }
    }
}
