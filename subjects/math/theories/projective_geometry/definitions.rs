use crate::subjects::math::theories::{common::spaces::DimensionType, VariantSet};
use serde::{Deserialize, Serialize};

/// A projective space
/// A projective space P(V) is the set of lines through the origin in a vector space V.
/// Equivalently, it is the quotient of V\{0} by the scaling action of the multiplicative group.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectiveSpace {
    /// Dimension (one less than vector space dimension)
    pub dimension: DimensionType,
    /// Field over which space is defined
    pub scalar_field: ScalarFieldType,
    /// Properties of the projective space
    pub properties: VariantSet<ProjectiveSpaceProperty>,
}

/// Properties specific to projective spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectiveSpaceProperty {
    /// Whether the space is smooth
    Smooth(bool),
    /// Whether the space is complete
    Complete(bool),
    /// Whether the space has a preferred coordinate system
    HasCoordinates(bool),
}

/// Types of scalar fields for projective spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScalarFieldType {
    /// Real numbers
    Real,
    /// Complex numbers
    Complex,
    /// Rational numbers
    Rational,
    /// Finite field
    Finite(u32),
}
