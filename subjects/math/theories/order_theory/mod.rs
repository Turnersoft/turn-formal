pub mod definitions;
pub mod theorems;

pub use definitions::{
    BooleanAlgebra, BooleanAlgebraProperty, Lattice, LatticeProperty, OrderComparison,
    OrderProperty, PartiallyOrderedSet, Term, TotallyOrderedSet,
};
pub use theorems::{fundamental, rewrite_rules, OrderTheorem};
