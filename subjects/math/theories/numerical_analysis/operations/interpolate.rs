use crate::subjects::math::theories::{
    analysis::definition::functions::Function, zfc::set::Set, VariantSet,
};
use serde::{Deserialize, Serialize};

use super::super::definitions::{
    approximation::{InterpolationMethod, InterpolationProperty},
    functional::NumericalOperator,
    space::NumericalFunctionSpace,
};

use super::{Composable, NumericalOperation};

/// Interpolation operation that constructs interpolants from data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InterpolationOperation {
    /// The underlying interpolation method
    pub method: InterpolationMethod,
    /// Additional operation properties
    pub properties: VariantSet<InterpolationOperationProperty>,
}

/// Properties specific to interpolation operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InterpolationOperationProperty {
    /// Implementation details
    Implementation {
        /// Evaluation strategy
        evaluation: String,
        /// Storage format
        storage: String,
    },

    /// Performance characteristics
    Performance {
        /// Setup cost
        setup: usize,
        /// Evaluation cost
        evaluation: usize,
    },

    /// Stability characteristics
    Stability {
        /// Lebesgue constant
        lebesgue: f64,
        /// Condition number
        condition: f64,
    },
}

impl Composable for InterpolationOperation {
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

impl NumericalOperation for InterpolationOperation {
    fn apply(&self, input: &Function) -> Result<Function, String> {
        // Validate input is in domain space
        if !self.method.base.target_space.contains(input) {
            return Err("Input function not in domain space".to_string());
        }

        // Apply interpolation operator
        self.method.base.operator.apply(input)
    }

    fn domain(&self) -> &NumericalFunctionSpace {
        &self.method.base.target_space
    }

    fn range(&self) -> &NumericalFunctionSpace {
        &self.method.base.approximation_space
    }
}

/// Extension trait for interpolation methods
pub trait Interpolate {
    /// Create an interpolation operation from this method
    fn to_operation(&self) -> InterpolationOperation;
}

impl Interpolate for InterpolationMethod {
    fn to_operation(&self) -> InterpolationOperation {
        InterpolationOperation {
            method: self.clone(),
            properties: VariantSet::new(),
        }
    }
}
