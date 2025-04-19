use super::super::super::super::math::theories::{
    analysis::definition::functions::Function, linear_algebra::definitions::VectorSpace,
    topology::definitions::TopologicalSpace, zfc::set::Set, VariantSet,
};
use serde::{Deserialize, Serialize};

use super::space::NumericalFunctionSpace;

/// Numerical Operator
///
/// An operator between numerical function spaces, equipped with
/// computational structure for numerical methods.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NumericalOperator {
    /// Domain space
    pub domain: NumericalFunctionSpace,
    /// Range space
    pub range: NumericalFunctionSpace,
    /// The operator mapping
    pub mapping: Function,
    /// Properties of the operator
    pub properties: VariantSet<OperatorProperty>,
}

/// Properties of numerical operators
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OperatorProperty {
    /// Linearity properties
    Linearity(LinearityProperty),

    /// Continuity properties
    Continuity(ContinuityProperty),

    /// Boundedness properties
    Boundedness(BoundednessProperty),

    /// Spectral properties
    Spectral(SpectralProperty),

    /// Computational properties
    Computation(ComputationProperty),
}

/// Linearity properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LinearityProperty {
    /// Linear operator
    Linear {
        /// Matrix representation (if finite dim)
        matrix: Option<Vec<Vec<f64>>>,
    },

    /// Affine operator
    Affine {
        /// Linear part
        linear: Box<NumericalOperator>,
        /// Translation
        translation: Function,
    },

    /// Nonlinear operator
    Nonlinear {
        /// Derivative operator
        derivative: Option<Box<NumericalOperator>>,
        /// Linearization point
        linearization_point: Option<Function>,
    },
}

/// Continuity properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContinuityProperty {
    /// Continuous operator
    Continuous {
        /// Modulus of continuity
        modulus: Function,
    },

    /// Hölder continuous
    Holder {
        /// Hölder exponent
        exponent: f64,
        /// Hölder constant
        constant: f64,
    },

    /// Lipschitz continuous
    Lipschitz {
        /// Lipschitz constant
        constant: f64,
    },
}

/// Boundedness properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BoundednessProperty {
    /// Bounded operator
    Bounded {
        /// Operator norm
        norm: f64,
    },

    /// Coercive operator
    Coercive {
        /// Coercivity constant
        constant: f64,
    },

    /// Monotone operator
    Monotone {
        /// Monotonicity constant
        constant: f64,
    },
}

/// Spectral properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpectralProperty {
    /// Eigenvalue properties
    Eigenvalues {
        /// Known eigenvalues
        values: Vec<f64>,
        /// Spectral bounds
        bounds: (f64, f64),
    },

    /// Resolvent properties
    Resolvent {
        /// Resolvent bounds
        bounds: Vec<(f64, f64)>,
    },

    /// Spectral radius
    SpectralRadius {
        /// Value or bound
        value: f64,
    },
}

/// Computational properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComputationProperty {
    /// Matrix properties
    Matrix {
        /// Condition number
        condition: f64,
        /// Sparsity pattern
        sparsity: String,
    },

    /// Iteration properties
    Iteration {
        /// Convergence rate
        rate: f64,
        /// Cost per iteration
        cost: usize,
    },

    /// Preconditioning
    Preconditioning {
        /// Preconditioner type
        type_: String,
        /// Effectiveness
        quality: f64,
    },
}

/// Numerical Functional
///
/// A functional on a numerical function space, equipped with
/// computational structure for numerical methods.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NumericalFunctional {
    /// Domain space
    pub domain: NumericalFunctionSpace,
    /// The functional mapping
    pub mapping: Function,
    /// Properties of the functional
    pub properties: VariantSet<FunctionalProperty>,
}

/// Properties of numerical functionals
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FunctionalProperty {
    /// Linearity properties
    Linearity(LinearityProperty),

    /// Convexity properties
    Convexity(ConvexityProperty),

    /// Differentiability properties
    Differentiability(DifferentiabilityProperty),

    /// Computational properties
    Computation(ComputationProperty),
}

/// Convexity properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConvexityProperty {
    /// Convex functional
    Convex {
        /// Modulus of convexity
        modulus: Option<Function>,
    },

    /// Strongly convex
    StronglyConvex {
        /// Strong convexity constant
        constant: f64,
    },

    /// Uniformly convex
    UniformlyConvex {
        /// Convexity modulus
        modulus: Function,
    },
}

/// Differentiability properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DifferentiabilityProperty {
    /// Gâteaux differentiable
    Gateaux {
        /// Derivative operator
        derivative: NumericalOperator,
    },

    /// Fréchet differentiable
    Frechet {
        /// Derivative operator
        derivative: NumericalOperator,
        /// Modulus of differentiability
        modulus: Function,
    },

    /// Twice differentiable
    TwiceDifferentiable {
        /// Second derivative
        second_derivative: NumericalOperator,
    },
}

/// Variational Derivative
///
/// The variational derivative of a functional, used in optimization
/// and variational problems.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariationalDerivative {
    /// The functional being differentiated
    pub functional: NumericalFunctional,
    /// The derivative operator
    pub derivative: NumericalOperator,
    /// Properties of the derivative
    pub properties: VariantSet<VariationalProperty>,
}

/// Properties of variational derivatives
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VariationalProperty {
    /// Existence and uniqueness
    Existence {
        /// Whether derivative exists
        exists: bool,
        /// Whether unique
        unique: bool,
    },

    /// Regularity properties
    Regularity {
        /// Smoothness
        smoothness: String,
        /// Estimates
        estimates: Vec<String>,
    },

    /// Computational aspects
    Computation {
        /// How to compute
        method: String,
        /// Computational cost
        cost: usize,
    },
}
