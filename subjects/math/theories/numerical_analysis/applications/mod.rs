pub mod differentiation;
pub mod integration;
pub mod optimization;
pub mod solvers;

use crate::subjects::math::theories::{
    analysis::definition::functions::Function, zfc::set::Set, VariantSet,
};
use serde::{Deserialize, Serialize};

// Re-export common types
pub use differentiation::*;
pub use integration::*;
pub use optimization::*;
pub use solvers::*;

/// Common trait for all numerical methods
pub trait NumericalMethod {
    /// The input type for the method
    type Input;
    /// The output type for the method
    type Output;
    /// The parameters configuring the method
    type Parameters;

    /// Apply the numerical method
    fn apply(&self, input: &Self::Input) -> Result<Self::Output, String>;

    /// Get the method's parameters
    fn parameters(&self) -> &Self::Parameters;

    /// Get the method's properties
    fn properties(&self) -> &VariantSet<MethodProperty>;
}

/// Properties common to numerical methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MethodProperty {
    /// Convergence properties
    Convergence {
        /// Order of convergence
        order: f64,
        /// Convergence conditions
        conditions: Vec<String>,
    },

    /// Stability properties
    Stability {
        /// Stability type
        stability_type: String,
        /// Stability conditions
        conditions: Vec<String>,
    },

    /// Computational properties
    Computation {
        /// Operation count
        operations: usize,
        /// Memory requirements
        memory: usize,
    },

    /// Error properties
    Error {
        /// Error type
        error_type: String,
        /// Error bounds
        bounds: Vec<String>,
    },
}

/// Common parameters for numerical methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MethodParameters {
    /// Tolerance for convergence
    pub tolerance: f64,
    /// Maximum iterations
    pub max_iterations: usize,
    /// Whether to use adaptive strategies
    pub adaptive: bool,
    /// Additional method-specific parameters
    pub additional: VariantSet<AdditionalParameter>,
}

/// Additional parameters for specific methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AdditionalParameter {
    /// Time stepping parameters
    TimeStep {
        /// Initial step size
        initial: f64,
        /// Minimum step size
        min: f64,
        /// Maximum step size
        max: f64,
    },

    /// Mesh parameters
    Mesh {
        /// Initial mesh size
        initial: f64,
        /// Refinement strategy
        refinement: String,
    },

    /// Solver parameters
    Solver {
        /// Preconditioner type
        preconditioner: String,
        /// Solver specific options
        options: Vec<String>,
    },

    /// Integration parameters
    Integration {
        /// Quadrature points
        points: usize,
        /// Weight function
        weight: Option<Function>,
    },
}
