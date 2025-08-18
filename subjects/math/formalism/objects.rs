use crate::subjects::math::theories::fields::Field;
use crate::subjects::math::theories::topology::definitions::TopologicalSpace;

use super::super::theories::analysis::definition::functions::Function;
use super::super::theories::groups::definitions::{
    Group, GroupOperation, GroupProperty, GroupRelation, LieGroup, TopologicalGroup,
};
use super::super::theories::linear_algebra::definitions::VectorSpace;
use super::super::theories::rings::definitions::{
    Algebra, Module, Ring, RingExpression, RingProperty,
};

use super::super::theories::zfc::definitions::Set;
use crate::subjects::math::formalism::traits::is_compatible::SameRole;
use serde::{Deserialize, Serialize};
/// A unified wrapper for all mathematical objects across theories
/// This is just a reference to objects defined in their respective theory modules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MathObject {
    // Group theory objects
    Group(Group),

    // Ring theory objects
    Ring(Ring),
    Field(Field),
    Module(Module),
    Algebra(Algebra),

    // Topology objects
    TopologicalSpace(TopologicalSpace),

    // Linear algebra objects
    VectorSpace(VectorSpace),

    // Set theory objects
    Set(Set),

    // Analysis objects
    Function(Function),
    // Other
    // The standard way to address this in systems aiming for
    // HOL/HoTT compatibility is not typically by changing
    // the quantifier structure to directly take a MathRelation or MathExpression.
    // Instead, the MathObject (or a parallel "Type" system) is extended to include variants representing these higher-order concepts:
    // Prop,
    // Type(UnverseLevel),
    // FunctionType(FunctionType),
    // Todo(String),
}

impl SameRole for MathObject {
    fn same_role(
        &self,
        target_context: &Vec<crate::subjects::math::formalism::proof::ContextEntry>,
        candidate: &Self,
        candidate_context: &Vec<crate::subjects::math::formalism::proof::ContextEntry>,
    ) -> bool {
        match (self, candidate) {
            (MathObject::Group(a), MathObject::Group(b)) => {
                a.same_role(target_context, b, candidate_context)
            }
            _ => false,
        }
    }
}
