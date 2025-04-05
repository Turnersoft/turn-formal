use crate::subjects::math::theories::{
    analysis::definition::functions::Function, zfc::set::Set, VariantSet,
};
use serde::{Deserialize, Serialize};

use super::{functional::NumericalOperator, space::NumericalFunctionSpace};

/// Error Analysis
///
/// Framework for analyzing errors in numerical methods,
/// including a priori and a posteriori estimates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorAnalysis {
    /// The space where error is measured
    pub space: NumericalFunctionSpace,
    /// The error functional
    pub error: Function,
    /// Properties of the error
    pub properties: VariantSet<ErrorProperty>,
}

/// Properties of numerical errors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ErrorProperty {
    /// A priori estimates
    APriori(APrioriError),

    /// A posteriori estimates
    APosteriori(APosterioriError),

    /// Stability aspects
    Stability(ErrorStability),

    /// Propagation aspects
    Propagation(ErrorPropagation),
}

/// A priori error estimates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum APrioriError {
    /// Truncation error
    Truncation {
        /// Order of accuracy
        order: usize,
        /// Constants in bounds
        constants: Vec<f64>,
        /// Requirements
        requirements: Vec<String>,
    },

    /// Approximation error
    Approximation {
        /// Best approximation
        best: Function,
        /// Error bounds
        bounds: Vec<String>,
    },

    /// Discretization error
    Discretization {
        /// Mesh dependent terms
        mesh_terms: Vec<String>,
        /// Method dependent terms
        method_terms: Vec<String>,
    },
}

/// A posteriori error estimates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum APosterioriError {
    /// Residual based
    Residual {
        /// Error indicators
        indicators: Vec<Function>,
        /// Reliability constant
        reliability: f64,
        /// Efficiency constant
        efficiency: f64,
    },

    /// Recovery based
    Recovery {
        /// Recovery operator
        operator: NumericalOperator,
        /// Superconvergence
        superconvergence: bool,
    },

    /// Hierarchical based
    Hierarchical {
        /// Refinement levels
        levels: usize,
        /// Saturation assumption
        saturation: f64,
    },
}

/// Error stability properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ErrorStability {
    /// Conditioning
    Conditioning {
        /// Condition number
        number: f64,
        /// Growth factors
        growth: Vec<f64>,
    },

    /// Perturbation effects
    Perturbation {
        /// Sensitivity
        sensitivity: f64,
        /// Robustness
        robustness: String,
    },

    /// Asymptotic behavior
    Asymptotic {
        /// Leading term
        leading: Function,
        /// Higher order terms
        higher_order: Vec<String>,
    },
}

/// Error propagation properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ErrorPropagation {
    /// Local propagation
    Local {
        /// Propagation rate
        rate: f64,
        /// Damping factors
        damping: Vec<f64>,
    },

    /// Global propagation
    Global {
        /// Long-term behavior
        behavior: String,
        /// Stability regions
        stability: Vec<String>,
    },

    /// Accumulation effects
    Accumulation {
        /// Growth pattern
        pattern: String,
        /// Bounds
        bounds: Vec<String>,
    },
}

/// Round-off Error Analysis
///
/// Specialization for analyzing floating-point errors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoundoffError {
    /// Base error analysis
    pub base: ErrorAnalysis,
    /// Floating point format
    pub format: FloatingPointFormat,
    /// Properties specific to roundoff
    pub properties: VariantSet<RoundoffProperty>,
}

/// Floating point format specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FloatingPointFormat {
    /// Number of bits in mantissa
    pub precision: usize,
    /// Exponent range
    pub exponent_range: (i32, i32),
    /// Special values handling
    pub special_values: Vec<String>,
}

/// Properties specific to roundoff error
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoundoffProperty {
    /// Unit roundoff properties
    UnitRoundoff {
        /// Machine epsilon
        epsilon: f64,
        /// Rounding mode
        mode: String,
    },

    /// Operation properties
    Operation {
        /// Error per operation
        error: f64,
        /// Worst case scenario
        worst_case: String,
    },

    /// Accumulation properties
    Accumulation {
        /// Growth rate
        rate: String,
        /// Compensated summation
        compensation: bool,
    },
}

/// Discretization Error Analysis
///
/// Specialization for analyzing discretization errors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiscretizationError {
    /// Base error analysis
    pub base: ErrorAnalysis,
    /// Discretization method
    pub method: String,
    /// Properties specific to discretization
    pub properties: VariantSet<DiscretizationErrorProperty>,
}

/// Properties specific to discretization error
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiscretizationErrorProperty {
    /// Consistency properties
    Consistency {
        /// Order of consistency
        order: usize,
        /// Requirements
        requirements: Vec<String>,
    },

    /// Stability properties
    Stability {
        /// CFL condition
        cfl: Option<String>,
        /// Energy estimates
        energy: Vec<String>,
    },

    /// Convergence properties
    Convergence {
        /// Order of convergence
        order: usize,
        /// Rate in different norms
        rates: Vec<(String, f64)>,
    },
}
