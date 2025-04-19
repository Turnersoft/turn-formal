use super::super::super::super::math::theories::{zfc::set::Set, VariantSet};
use serde::{Deserialize, Serialize};

/// A term that can be compared
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Term {
    /// A variable like x, y, z
    Variable(String),
    /// A constant value
    Constant(f64),
    /// A sequence element like xₙ
    SequenceElement(String, String), // (sequence_name, index)
}

/// Order comparison operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderComparison {
    /// Greater than: x > y
    GreaterThan(Box<Term>, Box<Term>),
    /// Greater than or equal: x ≥ y
    GreaterOrEqual(Box<Term>, Box<Term>),
    /// Less than: x < y
    LessThan(Box<Term>, Box<Term>),
    /// Less than or equal: x ≤ y
    LessOrEqual(Box<Term>, Box<Term>),
    /// Equal: x = y
    Equal(Box<Term>, Box<Term>),
}

/// A partially ordered set (poset) is a set with a binary relation ≤ that is:
/// 1. Reflexive: x ≤ x
/// 2. Antisymmetric: x ≤ y and y ≤ x implies x = y
/// 3. Transitive: x ≤ y and y ≤ z implies x ≤ z
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PartiallyOrderedSet {
    /// The underlying set
    pub base_set: Set,
    /// Properties of the partial order
    pub properties: VariantSet<OrderProperty>,
}

/// A totally ordered set (chain) is a partially ordered set where any two elements
/// are comparable: for any x,y either x ≤ y or y ≤ x
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotallyOrderedSet {
    /// The underlying set
    pub base_set: Set,
    /// Properties of the total order
    pub properties: VariantSet<OrderProperty>,
}

/// Properties of ordered sets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderProperty {
    /// Whether the order is complete (every bounded subset has a supremum)
    Complete(bool),
    /// Whether the order has a minimum element
    HasMinimum(bool),
    /// Whether the order has a maximum element
    HasMaximum(bool),
    /// Whether the order is dense (between any two elements there is another)
    Dense(bool),
    /// Whether the order is discrete (every element has immediate neighbors)
    Discrete(bool),
    /// Whether the order is well-founded (no infinite descending chains)
    WellFounded(bool),
}

/// A lattice is a partially ordered set where any two elements have:
/// 1. A least upper bound (join, ∨)
/// 2. A greatest lower bound (meet, ∧)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Lattice {
    /// The underlying partially ordered set
    pub poset: PartiallyOrderedSet,
    /// Properties of the lattice
    pub properties: VariantSet<LatticeProperty>,
}

/// Properties of lattices
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LatticeProperty {
    /// Whether the lattice is distributive: x∧(y∨z) = (x∧y)∨(x∧z)
    Distributive(bool),
    /// Whether the lattice is modular: x≤z implies x∨(y∧z) = (x∨y)∧z
    Modular(bool),
    /// Whether the lattice is complemented: every element has a complement
    Complemented(bool),
    /// Whether the lattice is bounded: has top and bottom elements
    Bounded(bool),
    /// Whether the lattice is complete: every subset has sup and inf
    Complete(bool),
}

/// A Boolean algebra is a complemented distributive lattice
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BooleanAlgebra {
    /// The underlying lattice
    pub lattice: Lattice,
    /// Properties of the Boolean algebra
    pub properties: VariantSet<BooleanAlgebraProperty>,
}

/// Properties of Boolean algebras
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BooleanAlgebraProperty {
    /// Whether the algebra is complete (every subset has sup and inf)
    Complete(bool),
    /// Whether the algebra is atomic (every element is sup of atoms below it)
    Atomic(bool),
    /// Whether the algebra is finite
    Finite(bool),
}
