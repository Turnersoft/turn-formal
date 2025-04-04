use crate::formalize_v2::subjects::math::theories::{
    analysis::definition::functions::Function, zfc::set::Set, VariantSet,
};
use serde::{Deserialize, Serialize};

use super::{functional::NumericalOperator, space::NumericalFunctionSpace};

/// Approximation Method
///
/// Represents methods for approximating functions or solutions,
/// including interpolation, projection, and best approximation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApproximationMethod {
    /// The space containing the target function
    pub target_space: NumericalFunctionSpace,
    /// The space containing approximations
    pub approximation_space: NumericalFunctionSpace,
    /// The approximation operator
    pub operator: NumericalOperator,
    /// Properties of the approximation
    pub properties: VariantSet<ApproximationProperty>,
}

/// Properties of approximation methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApproximationProperty {
    /// Error estimates and bounds
    Error(ApproximationError),

    /// Stability properties
    Stability(ApproximationStability),

    /// Optimality properties
    Optimality(OptimalityProperty),

    /// Computational aspects
    Computation(ApproximationComputation),
}

/// Error properties for approximations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApproximationError {
    /// A priori error estimates
    APriori {
        /// Order of convergence
        order: f64,
        /// Constants in bounds
        constants: Vec<f64>,
        /// Requirements for order
        requirements: Vec<String>,
    },

    /// A posteriori error estimates
    APosteriori {
        /// Error indicators
        indicators: Vec<String>,
        /// Reliability constants
        reliability: f64,
        /// Efficiency constants
        efficiency: f64,
    },

    /// Best approximation error
    BestApproximation {
        /// Error functional
        error: Function,
        /// Characterization
        characterization: String,
    },
}

/// Stability properties for approximations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApproximationStability {
    /// Continuous dependence on data
    Continuous {
        /// Stability constant
        constant: f64,
        /// Modulus of continuity
        modulus: Function,
    },

    /// Discrete stability
    Discrete {
        /// Grid-dependent constant
        constant: Function,
        /// Mesh conditions
        conditions: Vec<String>,
    },

    /// Asymptotic stability
    Asymptotic {
        /// Limiting behavior
        behavior: String,
        /// Rate of convergence
        rate: f64,
    },
}

/// Optimality properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OptimalityProperty {
    /// Best approximation property
    BestApproximation {
        /// In what sense optimal
        criterion: String,
        /// Uniqueness
        unique: bool,
    },

    /// Quasi-optimality
    QuasiOptimal {
        /// Quasi-optimality constant
        constant: f64,
        /// Additional properties
        properties: Vec<String>,
    },

    /// Saturation property
    Saturation {
        /// Saturation order
        order: f64,
        /// Characterization
        characterization: String,
    },
}

/// Computational aspects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApproximationComputation {
    /// Construction cost
    Construction {
        /// Operations count
        operations: usize,
        /// Memory requirements
        memory: usize,
    },

    /// Evaluation cost
    Evaluation {
        /// Cost per point
        point_cost: usize,
        /// Preprocessing cost
        setup_cost: usize,
    },

    /// Error estimation cost
    ErrorEstimation {
        /// Cost of error estimate
        cost: usize,
        /// Reliability
        reliability: f64,
    },
}

/// Interpolation Method
///
/// Specialization of approximation methods for interpolation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InterpolationMethod {
    /// Base approximation method
    pub base: ApproximationMethod,
    /// Interpolation nodes/points
    pub nodes: Vec<Function>,
    /// Properties specific to interpolation
    pub properties: VariantSet<InterpolationProperty>,
}

/// Properties specific to interpolation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InterpolationProperty {
    /// Node properties
    Nodes {
        /// Distribution of nodes
        distribution: String,
        /// Optimality properties
        optimality: Option<String>,
    },

    /// Basis properties
    Basis {
        /// Type of basis
        basis_type: String,
        /// Conditioning
        condition_number: f64,
    },

    /// Unisolvence properties
    Unisolvence {
        /// Whether uniquely solvable
        unique: bool,
        /// Conditions for uniqueness
        conditions: Vec<String>,
    },
}

/// Projection Method
///
/// Specialization of approximation methods for projection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectionMethod {
    /// Base approximation method
    pub base: ApproximationMethod,
    /// The projection operator
    pub projector: NumericalOperator,
    /// Properties specific to projection
    pub properties: VariantSet<ProjectionProperty>,
}

/// Properties specific to projection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectionProperty {
    /// Orthogonality properties
    Orthogonality {
        /// What's orthogonal
        orthogonal_to: String,
        /// Inner product
        inner_product: Function,
    },

    /// Stability properties
    Stability {
        /// Stability in norm
        norm_bound: f64,
        /// Additional properties
        properties: Vec<String>,
    },

    /// Approximation properties
    Approximation {
        /// Best approximation?
        is_best: bool,
        /// Error bounds
        bounds: Vec<String>,
    },
}
