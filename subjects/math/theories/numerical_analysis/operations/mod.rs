pub mod approximate;
pub mod compose;
pub mod discretize;
pub mod interpolate;
pub mod iterate;
pub mod tensor;

// Re-export common types
pub use tensor::*;

/// Common traits and utilities for numerical operations
use super::super::super::super::math::theories::{
    analysis::definition::functions::Function, zfc::set::Set, VariantSet,
};
use serde::{Deserialize, Serialize};

use super::definitions::{functional::NumericalOperator, space::NumericalFunctionSpace};

/// Trait for numerical operations that can be composed
pub trait Composable {
    /// The input type of the operation
    type Input;
    /// The output type of the operation
    type Output;

    /// Compose this operation with another
    fn compose<T: Composable<Input = Self::Output>>(self, other: T) -> Composition<Self, T>
    where
        Self: Sized;
}

/// Trait for operations that can be applied to numerical functions
pub trait NumericalOperation: Composable {
    /// Apply the operation to a function
    fn apply(&self, input: &Function) -> Result<Function, String>;

    /// Get the domain space of the operation
    fn domain(&self) -> &NumericalFunctionSpace;

    /// Get the range space of the operation
    fn range(&self) -> &NumericalFunctionSpace;
}

/// Composition of two numerical operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Composition<T: Composable, U: Composable<Input = T::Output>> {
    /// First operation
    pub first: T,
    /// Second operation
    pub second: U,
}

impl<T: Composable, U: Composable<Input = T::Output>> Composable for Composition<T, U> {
    type Input = T::Input;
    type Output = U::Output;

    fn compose<V: Composable<Input = Self::Output>>(self, other: V) -> Composition<Self, V>
    where
        Self: Sized,
    {
        Composition {
            first: self,
            second: other,
        }
    }
}
