use super::super::super::super::math::theories::{
    analysis::definition::functions::Function, zfc::set::Set, VariantSet,
};
use serde::{Deserialize, Serialize};

use super::{functional::NumericalOperator, space::NumericalFunctionSpace};

/// Convergence Analysis
///
/// Framework for analyzing convergence of numerical methods,
/// including rates, conditions, and acceleration techniques.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConvergenceAnalysis {
    /// The space where convergence is analyzed
    pub space: NumericalFunctionSpace,
    /// The sequence or method being analyzed
    pub sequence: Function,
    /// Properties of convergence
    pub properties: VariantSet<ConvergenceProperty>,
}

/// Properties of convergence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConvergenceProperty {
    /// Type and rate of convergence
    Rate(ConvergenceRate),

    /// Conditions for convergence
    Conditions(ConvergenceConditions),

    /// Acceleration techniques
    Acceleration(ConvergenceAcceleration),

    /// Stability aspects
    Stability(ConvergenceStability),
}

/// Types and rates of convergence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConvergenceRate {
    /// Linear convergence
    Linear {
        /// Contraction factor
        factor: f64,
        /// Error bound
        bound: Function,
    },

    /// Polynomial convergence
    Polynomial {
        /// Order of convergence
        order: f64,
        /// Leading coefficient
        coefficient: f64,
    },

    /// Exponential convergence
    Exponential {
        /// Base rate
        base: f64,
        /// Scaling factor
        scale: f64,
    },

    /// Asymptotic convergence
    Asymptotic {
        /// Leading term
        leading: Function,
        /// Error term
        error: Function,
    },
}

/// Conditions necessary for convergence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConvergenceConditions {
    /// Initial conditions
    Initial {
        /// Requirements on starting point
        requirements: Vec<String>,
        /// Size of neighborhood
        neighborhood: Option<f64>,
    },

    /// Regularity conditions
    Regularity {
        /// Smoothness requirements
        smoothness: String,
        /// Additional conditions
        conditions: Vec<String>,
    },

    /// Stability conditions
    Stability {
        /// CFL-like conditions
        cfl_condition: Option<String>,
        /// Other stability requirements
        requirements: Vec<String>,
    },
}

/// Convergence acceleration techniques
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConvergenceAcceleration {
    /// Extrapolation methods
    Extrapolation {
        /// Type of extrapolation
        method: String,
        /// Acceleration factor
        factor: f64,
    },

    /// Relaxation methods
    Relaxation {
        /// Relaxation parameter
        parameter: Function,
        /// Optimal value
        optimal: Option<f64>,
    },

    /// Preconditioning
    Preconditioning {
        /// Preconditioner
        operator: NumericalOperator,
        /// Effectiveness measure
        effectiveness: f64,
    },
}

/// Stability aspects of convergence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConvergenceStability {
    /// Numerical stability
    Numerical {
        /// Stability constant
        constant: f64,
        /// Growth bound
        growth: Function,
    },

    /// Perturbation effects
    Perturbation {
        /// Sensitivity measure
        sensitivity: f64,
        /// Error propagation
        propagation: String,
    },

    /// Conditioning aspects
    Conditioning {
        /// Condition number
        number: f64,
        /// Effect on convergence
        effect: String,
    },
}

/// Convergence Analysis for Sequences
///
/// Specialization for analyzing convergence of sequences
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SequenceConvergence {
    /// Base convergence analysis
    pub base: ConvergenceAnalysis,
    /// The limit (if known)
    pub limit: Option<Function>,
    /// Properties specific to sequences
    pub properties: VariantSet<SequenceConvergenceProperty>,
}

/// Properties specific to sequence convergence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SequenceConvergenceProperty {
    /// Monotonicity properties
    Monotonicity {
        /// Type of monotonicity
        monotone_type: String,
        /// From which term
        from_term: Option<usize>,
    },

    /// Oscillation properties
    Oscillation {
        /// Nature of oscillation
        oscillation_type: String,
        /// Bounds on oscillation
        bounds: Option<(f64, f64)>,
    },

    /// Subsequence properties
    Subsequence {
        /// Convergent subsequences
        convergent: bool,
        /// Rate for subsequences
        rate: Option<ConvergenceRate>,
    },
}

/// Convergence Analysis for Series
///
/// Specialization for analyzing convergence of series
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeriesConvergence {
    /// Base convergence analysis
    pub base: ConvergenceAnalysis,
    /// Partial sums
    pub partial_sums: Function,
    /// Properties specific to series
    pub properties: VariantSet<SeriesConvergenceProperty>,
}

/// Properties specific to series convergence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SeriesConvergenceProperty {
    /// Absolute convergence
    Absolute {
        /// Whether absolutely convergent
        is_absolute: bool,
        /// Related properties
        properties: Vec<String>,
    },

    /// Conditional convergence
    Conditional {
        /// Nature of conditions
        conditions: Vec<String>,
        /// Effect on sum
        effect: String,
    },

    /// Summability methods
    Summability {
        /// Method of summation
        method: String,
        /// Applicability conditions
        conditions: Vec<String>,
    },
}
