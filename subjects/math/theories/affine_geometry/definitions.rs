use super::super::super::super::math::theories::{
    VariantSet,
    common::spaces::{DimensionType, Space},
};
use serde::{Deserialize, Serialize};

/// An affine space modeled on a vector space
/// An affine space is a set A together with a vector space V and a transitive
/// and free action of V on A by translation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AffineSpace {
    /// Dimension of the space
    pub dimension: DimensionType,
    /// The modeling vector space
    pub vector_space: Box<Space>,
    /// Properties of the affine space
    pub properties: VariantSet<AffineSpaceProperty>,
}

/// Properties specific to affine spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AffineSpaceProperty {
    /// Whether the space is real or complex
    ScalarField(ScalarFieldType),
    /// Whether the space has a preferred origin
    HasOrigin(bool),
    /// Whether the space is complete
    Complete(bool),
}

/// Types of scalar fields for affine spaces
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
