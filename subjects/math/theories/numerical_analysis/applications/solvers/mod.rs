pub mod linear;
pub mod nonlinear;
pub mod time_integration;

use super::super::super::super::math::theories::{
    analysis::definition::functions::Function, zfc::set::Set, VariantSet,
};
use serde::{Deserialize, Serialize};

use super::super::definitions::{
    approximation::ApproximationMethod, functional::NumericalOperator,
    space::NumericalFunctionSpace,
};

use super::{MethodParameters, MethodProperty, NumericalMethod};

/// Common solver types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SolverType {
    /// Direct solvers
    Direct {
        /// Factorization type
        factorization: String,
        /// Pivoting strategy
        pivoting: String,
    },

    /// Iterative solvers
    Iterative {
        /// Base method
        method: String,
        /// Preconditioner
        preconditioner: String,
    },

    /// Time integration solvers
    TimeIntegration {
        /// Integration scheme
        scheme: String,
        /// Order of accuracy
        order: usize,
    },
}

/// Properties specific to solvers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SolverProperty {
    /// Matrix properties
    Matrix {
        /// Condition number
        condition: f64,
        /// Sparsity pattern
        sparsity: String,
    },

    /// Preconditioner properties
    Preconditioner {
        /// Quality measure
        quality: f64,
        /// Setup cost
        setup_cost: usize,
    },

    /// Time stepping properties
    TimeStepping {
        /// Stability region
        stability_region: String,
        /// Local truncation error
        truncation_error: f64,
    },
}

/// Parameters for solvers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SolverParameters {
    /// Base parameters
    pub base: MethodParameters,
    /// Solver type
    pub solver_type: SolverType,
    /// Additional solver-specific parameters
    pub solver_specific: VariantSet<SolverSpecificParameter>,
}

/// Additional parameters specific to solvers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SolverSpecificParameter {
    /// Direct solver parameters
    Direct {
        /// Blocking size
        block_size: usize,
        /// Threading threshold
        thread_threshold: usize,
    },

    /// Iterative solver parameters
    Iterative {
        /// Restart parameter
        restart: usize,
        /// Orthogonalization method
        orthog: String,
    },

    /// Time integration parameters
    TimeIntegration {
        /// Stage order
        stage_order: usize,
        /// Error estimator
        error_estimator: String,
    },
}

/// Common trait for numerical solvers
pub trait NumericalSolver: NumericalMethod {
    /// Get the solver type
    fn solver_type(&self) -> &SolverType;

    /// Get solver-specific properties
    fn solver_properties(&self) -> &VariantSet<SolverProperty>;
}
