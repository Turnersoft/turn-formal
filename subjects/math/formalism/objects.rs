use super::super::theories::analysis::definition::functions::Function;
use super::super::theories::groups::definitions::{
    Group, GroupOperation, GroupProperty, GroupRelation, LieGroup, TopologicalGroup,
};
use super::super::theories::linear_algebra::definitions::VectorSpace;
use super::super::theories::rings::definitions::{
    Algebra, Field, Module, Ring, RingExpression, RingProperty,
};
use super::super::theories::topology::TopologicalSpace;
use super::super::theories::zfc::Set;
use serde::{Deserialize, Serialize};
/// A unified wrapper for all mathematical objects across theories
/// This is just a reference to objects defined in their respective theory modules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

    // Basic number types
    Integer,
    Rational,
    Irrational,
    Real,
    Complex,

    // General types
    Element(Box<MathObject>),                   // Element of a given type
    Morphism(Box<MathObject>, Box<MathObject>), // Morphism between types

    // Type constructors
    Product(Vec<MathObject>),
    Coproduct(Vec<MathObject>),

    // Other
    // The standard way to address this in systems aiming for
    // HOL/HoTT compatibility is not typically by changing
    // the quantifier structure to directly take a MathRelation or MathExpression.
    // Instead, the MathObject (or a parallel "Type" system) is extended to include variants representing these higher-order concepts:
    // Prop,
    // Type(UnverseLevel),
    // FunctionType(FunctionType),
    Todo(String),
}
