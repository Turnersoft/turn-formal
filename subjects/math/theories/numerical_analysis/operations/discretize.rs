use crate::subjects::math::theories::{
    analysis::definition::functions::Function, zfc::set::Set, VariantSet,
};
use serde::{Deserialize, Serialize};

use super::super::definitions::{
    discretization::{DiscretizationMethod, DiscretizationProperty},
    functional::NumericalOperator,
    space::NumericalFunctionSpace,
};

use super::{Composable, NumericalOperation};

/// Discretization operation that converts continuous functions to discrete ones
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiscretizationOperation {
    /// The underlying discretization method
    pub method: DiscretizationMethod,
    /// Additional operation properties
    pub properties: VariantSet<DiscretizationOperationProperty>,
}

/// Properties specific to discretization operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiscretizationOperationProperty {
    /// Implementation details
    Implementation {
        /// Assembly strategy
        assembly: String,
        /// Storage format
        storage: String,
    },

    /// Performance characteristics
    Performance {
        /// Operation count
        operations: usize,
        /// Memory usage
        memory: usize,
    },

    /// Parallelization strategy
    Parallel {
        /// Decomposition method
        decomposition: String,
        /// Communication pattern
        communication: String,
    },
}

impl Composable for DiscretizationOperation {
    type Input = Function;
    type Output = Function;

    fn compose<T: Composable<Input = Self::Output>>(self, other: T) -> super::Composition<Self, T>
    where
        Self: Sized,
    {
        super::Composition {
            first: self,
            second: other,
        }
    }
}

impl NumericalOperation for DiscretizationOperation {
    fn apply(&self, input: &Function) -> Result<Function, String> {
        // Validate input is in domain space
        if !self.method.continuous_space.contains(input) {
            return Err("Input function not in domain space".to_string());
        }

        // Apply discretization operator
        self.method.operator.apply(input)
    }

    fn domain(&self) -> &NumericalFunctionSpace {
        &self.method.continuous_space
    }

    fn range(&self) -> &NumericalFunctionSpace {
        &self.method.discrete_space
    }
}

/// Extension trait for discretization methods
pub trait Discretize {
    /// Create a discretization operation from this method
    fn to_operation(&self) -> DiscretizationOperation;
}

impl Discretize for DiscretizationMethod {
    fn to_operation(&self) -> DiscretizationOperation {
        DiscretizationOperation {
            method: self.clone(),
            properties: VariantSet::new(),
        }
    }
}
