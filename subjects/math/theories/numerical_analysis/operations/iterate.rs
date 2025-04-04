use crate::formalize_v2::subjects::math::theories::{
    analysis::definition::functions::Function, zfc::set::Set, VariantSet,
};
use serde::{Deserialize, Serialize};

use super::super::definitions::{
    functional::NumericalOperator,
    iteration::{IterationMethod, IterationProperty},
    space::NumericalFunctionSpace,
};

use super::{Composable, NumericalOperation};

/// Iteration operation that applies iterative methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IterationOperation {
    /// The underlying iteration method
    pub method: IterationMethod,
    /// Additional operation properties
    pub properties: VariantSet<IterationOperationProperty>,
}

/// Properties specific to iteration operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IterationOperationProperty {
    /// Implementation details
    Implementation {
        /// Storage strategy
        storage: String,
        /// Update strategy
        update: String,
    },

    /// Performance characteristics
    Performance {
        /// Cost per iteration
        iteration_cost: usize,
        /// Memory per iteration
        iteration_memory: usize,
    },

    /// Monitoring characteristics
    Monitoring {
        /// Convergence test
        convergence_test: String,
        /// Progress metrics
        metrics: Vec<String>,
    },
}

impl Composable for IterationOperation {
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

impl NumericalOperation for IterationOperation {
    fn apply(&self, input: &Function) -> Result<Function, String> {
        // Validate input is in domain space
        if !self.method.space.contains(input) {
            return Err("Input function not in domain space".to_string());
        }

        // Apply iteration operator
        self.method.iteration_map.apply(input)
    }

    fn domain(&self) -> &NumericalFunctionSpace {
        &self.method.space
    }

    fn range(&self) -> &NumericalFunctionSpace {
        &self.method.space
    }
}

/// Extension trait for iteration methods
pub trait Iterate {
    /// Create an iteration operation from this method
    fn to_operation(&self) -> IterationOperation;
}

impl Iterate for IterationMethod {
    fn to_operation(&self) -> IterationOperation {
        IterationOperation {
            method: self.clone(),
            properties: VariantSet::new(),
        }
    }
}
