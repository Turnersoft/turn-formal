// Module: src/formalize_v2/subjects/math/theorem/properties.rs
// Defines mathematical properties and requirements for objects

use serde::{Deserialize, Serialize};

/// A mathematical property that objects can satisfy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MathProperty {
    // Group theory properties
    Associative,
    Commutative,
    HasIdentity,
    HasInverses,
    Finite,
    Infinite,
    Simple,
    Abelian,

    // Topology properties
    Connected,
    Compact,
    Hausdorff,

    // Properties for various types
    Custom(String),
}

/// A property requirement for a mathematical object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PropertyRequirement {
    /// The property that must be satisfied
    pub property: MathProperty,

    /// Whether the property must be satisfied (true) or not satisfied (false)
    pub polarity: bool,

    /// Optional explanation of why this property is required
    pub explanation: Option<String>,
}
