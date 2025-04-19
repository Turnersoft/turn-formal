mod decompose;
mod spectral;
mod wavelet;

pub use decompose::*;
pub use spectral::*;
pub use wavelet::*;

/// Common traits and utilities for numerical transformations
use super::super::super::super::math::theories::{
    analysis::definition::functions::Function, zfc::set::Set, VariantSet,
};
use serde::{Deserialize, Serialize};

use super::definitions::{functional::NumericalOperator, space::NumericalFunctionSpace};

/// Trait for numerical transformations
pub trait NumericalTransformation {
    /// Transform a function
    fn transform(&self, input: &Function) -> Result<Function, String>;

    /// Inverse transform a function
    fn inverse_transform(&self, input: &Function) -> Result<Function, String>;

    /// Get the domain space of the transformation
    fn domain(&self) -> &NumericalFunctionSpace;

    /// Get the range space of the transformation
    fn range(&self) -> &NumericalFunctionSpace;
}

/// Base transformation properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransformationProperty {
    /// Orthogonality properties
    Orthogonality {
        /// Inner product
        inner_product: Function,
        /// Orthogonality type
        orthogonality_type: String,
    },

    /// Stability properties
    Stability {
        /// Stability constant
        constant: f64,
        /// Stability type
        stability_type: String,
    },

    /// Computational properties
    Computation {
        /// Forward transform cost
        forward_cost: usize,
        /// Inverse transform cost
        inverse_cost: usize,
    },
}
