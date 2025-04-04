use crate::formalize_v2::subjects::math::theories::{
    analysis::definition::functions::Function, zfc::set::Set, VariantSet,
};
use serde::{Deserialize, Serialize};

use super::super::definitions::{functional::NumericalOperator, space::NumericalFunctionSpace};

use super::{Composable, NumericalOperation};

/// Composition operation that combines multiple operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompositionOperation<T: NumericalOperation, U: NumericalOperation> {
    /// First operation
    pub first: T,
    /// Second operation
    pub second: U,
    /// Additional operation properties
    pub properties: VariantSet<CompositionOperationProperty>,
}

/// Properties specific to composition operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompositionOperationProperty {
    /// Implementation details
    Implementation {
        /// Evaluation strategy
        strategy: String,
        /// Storage format
        storage: String,
    },

    /// Performance characteristics
    Performance {
        /// Combined cost
        total_cost: usize,
        /// Memory requirements
        memory: usize,
    },

    /// Optimization characteristics
    Optimization {
        /// Fusion opportunities
        fusion: Vec<String>,
        /// Reordering opportunities
        reordering: Vec<String>,
    },
}

impl<T: NumericalOperation, U: NumericalOperation> Composable for CompositionOperation<T, U> {
    type Input = T::Input;
    type Output = U::Output;

    fn compose<V: Composable<Input = Self::Output>>(self, other: V) -> super::Composition<Self, V>
    where
        Self: Sized,
    {
        super::Composition {
            first: self,
            second: other,
        }
    }
}

impl<T: NumericalOperation, U: NumericalOperation> NumericalOperation
    for CompositionOperation<T, U>
{
    fn apply(&self, input: &Function) -> Result<Function, String> {
        // Apply first operation
        let intermediate = self.first.apply(input)?;

        // Apply second operation
        self.second.apply(&intermediate)
    }

    fn domain(&self) -> &NumericalFunctionSpace {
        self.first.domain()
    }

    fn range(&self) -> &NumericalFunctionSpace {
        self.second.range()
    }
}

/// Extension trait for composing numerical operations
pub trait ComposeOperations: NumericalOperation + Sized {
    /// Compose this operation with another
    fn compose_with<T: NumericalOperation>(self, other: T) -> CompositionOperation<Self, T>;
}

impl<S: NumericalOperation> ComposeOperations for S {
    fn compose_with<T: NumericalOperation>(self, other: T) -> CompositionOperation<Self, T> {
        CompositionOperation {
            first: self,
            second: other,
            properties: VariantSet::new(),
        }
    }
}
