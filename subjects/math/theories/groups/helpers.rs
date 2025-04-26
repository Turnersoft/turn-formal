//! Helper functions for group theory operations
//!
//! This module provides helper functions for creating common group types and elements.

use super::super::super::super::math::theories::VariantSet;
use super::super::super::super::math::theories::groups::definitions::{
    AbelianPropertyVariant, ElementValue, FinitePropertyVariant, Group, GroupIdentity,
    GroupInverse, GroupInverseApplication, GroupNotation, GroupOperation, GroupOperationProperty,
    GroupProperty, GroupSymbol,
};
use super::super::super::super::math::theories::zfc::Set;
use std::collections::HashMap;

use super::GroupOperationVariant;

/// Helper function to create a cyclic group Z_n
pub fn cyclic_group(n: i64) -> Group {
    // Create parameters for Z_n
    let mut parameters = HashMap::new();
    parameters.insert("n".to_string(), n.to_string());

    // Create the group set
    let base_set = Set::Parametric {
        parameters,
        description: format!("Z_{}", n),
        membership_condition: format!("x ∈ {{0, 1, ..., {} - 1}}", n),
        properties: VariantSet::new(),
    };

    // Define the group operation
    let operation = GroupOperation {
        operation_type: GroupOperationVariant::Addition,
        notation: GroupNotation::Infix(GroupSymbol::Plus),
        identity: GroupIdentity::Zero,
        inverse: GroupInverse::AdditiveInverse,
        inverse_application: GroupInverseApplication::TwoSided,
        properties: vec![
            GroupOperationProperty::Associative,
            GroupOperationProperty::Commutative(true),
            GroupOperationProperty::Closed,
        ],
    };

    Group {
        base_set,
        operation,
        properties: vec![
            GroupProperty::Abelian(AbelianPropertyVariant::Abelian),
            GroupProperty::Finite(FinitePropertyVariant::Finite(n as u32)),
        ],
    }
}

/// Helper function to create a symmetric group S_n
pub fn symmetric_group(degree: usize) -> Group {
    // Create parameters for S_n
    let mut parameters = HashMap::new();
    parameters.insert("degree".to_string(), degree.to_string());

    // Calculate the order of the symmetric group: n!
    let order = (1..=degree).fold(1, |acc, x| acc * x) as u32;

    // Create the group set
    let base_set = Set::Parametric {
        parameters,
        description: format!("S_{}", degree),
        membership_condition: format!("x is a permutation of {{1, 2, ..., {}}}", degree),
        properties: VariantSet::new(),
    };

    // Define the group operation
    let operation = GroupOperation {
        operation_type: GroupOperationVariant::Composition,
        notation: GroupNotation::Infix(GroupSymbol::Circle),
        identity: GroupIdentity::IdentityPermutation,
        inverse: GroupInverse::PermutationInverse,
        inverse_application: GroupInverseApplication::TwoSided,
        properties: vec![
            GroupOperationProperty::Associative,
            GroupOperationProperty::Closed,
        ],
    };

    // S_n is non-abelian for n > 2
    let abelian_property = if degree <= 2 {
        GroupProperty::Abelian(AbelianPropertyVariant::Abelian)
    } else {
        GroupProperty::Abelian(AbelianPropertyVariant::NonAbelian)
    };

    Group {
        base_set,
        operation,
        properties: vec![
            abelian_property,
            GroupProperty::Finite(FinitePropertyVariant::Finite(order)),
        ],
    }
}

/// Helper function to create integer element for group operations
pub fn int(value: i64) -> ElementValue {
    ElementValue::Integer(value)
}

/// Helper function to create permutation element for group operations
pub fn perm(values: Vec<usize>) -> ElementValue {
    ElementValue::Permutation(values)
}

/// Helper function to create a symbolic element for abstract groups
pub fn sym(name: &str) -> ElementValue {
    ElementValue::Symbol(name.to_string())
}

/// Helper function to create a matrix element for matrix groups
pub fn matrix(values: Vec<Vec<i64>>) -> ElementValue {
    ElementValue::Matrix(values)
}
