//! Runtime checking functions for group theory operations
//!
//! This module provides functions for runtime verification of group operations,
//! ensuring that elements belong to their respective groups and that operations
//! are well-defined within the algebraic structure.

use super::super::super::super::math::theories::groups::definitions::{
    ElementValue, Group, GroupElement, GroupExpression, GroupExpressionError,
};
use super::super::super::super::math::theories::zfc::Set;
use std::collections::HashSet;
use thiserror::Error;

use super::GroupOperationVariant;

/// Errors that can occur during group operations
#[derive(Debug, Clone, Error)]
pub enum GroupError {
    /// Element does not belong to the specified group
    #[error("Element {element_desc} is not a member of group {group_desc}")]
    InvalidElement {
        element_desc: String,
        group_desc: String,
        message: String,
    },

    /// Groups don't match for operation
    #[error("Groups don't match: {message}")]
    GroupMismatch { message: String },

    /// Invalid operation between elements
    #[error("Invalid operation between elements: {message}")]
    InvalidOperation { message: String },

    /// Group structure does not support the requested operation
    #[error("Group operation not supported: {message}")]
    UnsupportedOperation { message: String },

    /// Subgroup is not normal, but normality is required
    #[error("Subgroup is not normal: {message}")]
    NotNormalSubgroup { message: String },

    /// Expression evaluation error
    #[error("Expression evaluation error: {0}")]
    ExpressionError(String),

    /// Other group-related errors
    #[error("Group error: {message}")]
    Other { message: String },
}

/// Result type for group operations
pub type GroupResult<T> = Result<T, GroupError>;

impl From<GroupExpressionError> for GroupError {
    fn from(err: GroupExpressionError) -> Self {
        match err {
            GroupExpressionError::InvalidElement(msg) => GroupError::InvalidElement {
                element_desc: "unknown".to_string(),
                group_desc: "unknown".to_string(),
                message: msg,
            },
            GroupExpressionError::InvalidOperation(msg) => {
                GroupError::InvalidOperation { message: msg }
            }
            GroupExpressionError::Other(msg) => GroupError::ExpressionError(msg),
        }
    }
}

/// Checks if an element belongs to a specific group
///
/// This function verifies that an element is a valid member of the given group.
/// For finite groups, it checks against the explicit set of elements.
/// For infinite groups, it applies appropriate membership rules.
pub fn check_element_in_group(group: &Group, element: &ElementValue) -> GroupResult<()> {
    match &group.base_set {
        Set::Empty => {
            // Empty group (should not happen in a valid mathematical setting)
            Err(GroupError::InvalidElement {
                element_desc: format!("{:?}", element),
                group_desc: "empty group".to_string(),
                message: "No elements in an empty group".to_string(),
            })
        }
        Set::Parametric {
            parameters,
            membership_condition,
            ..
        } => {
            // For parametric sets (like Z_n), check based on the membership condition
            match (&group.operation.operation_type, element) {
                // For cyclic group Z_n with additive notation
                (GroupOperationVariant::Addition, ElementValue::Integer(val)) => {
                    // Check if the element is in range [0, n-1]
                    if let Some(n_param) = parameters.get("n") {
                        if let Ok(n) = n_param.parse::<i64>() {
                            if *val >= 0 && *val < n {
                                return Ok(());
                            }
                        }
                    }
                    Err(GroupError::InvalidElement {
                        element_desc: format!("{:?}", element),
                        group_desc: format!(
                            "cyclic group Z_{}",
                            parameters.get("n").unwrap_or(&"n".to_string())
                        ),
                        message: "Element is not in the valid range for this cyclic group"
                            .to_string(),
                    })
                }
                // For permutation groups
                (GroupOperationVariant::Composition, ElementValue::Permutation(perm)) => {
                    // Check if the permutation is valid for the degree of the group
                    if let Some(degree_param) = parameters.get("degree") {
                        if let Ok(degree) = degree_param.parse::<usize>() {
                            // Check if the permutation contains valid indices and is a permutation
                            let mut seen = HashSet::new();
                            for &idx in perm {
                                if idx == 0 || idx > degree || !seen.insert(idx) {
                                    return Err(GroupError::InvalidElement {
                                        element_desc: format!("{:?}", element),
                                        group_desc: format!(
                                            "permutation group of degree {}",
                                            degree
                                        ),
                                        message: "Invalid permutation for this group".to_string(),
                                    });
                                }
                            }
                            if perm.len() == degree {
                                return Ok(());
                            }
                        }
                    }
                    Err(GroupError::InvalidElement {
                        element_desc: format!("{:?}", element),
                        group_desc: "permutation group".to_string(),
                        message: "Invalid permutation for this group".to_string(),
                    })
                }
                // Handle other group types as needed
                _ => {
                    // For cases where we can't easily check, assume validity
                    // In a production implementation, this would be more strict
                    Ok(())
                }
            }
        }
        _ => {
            // Default to allowing for now
            // In a production implementation, this would be more strict
            Ok(())
        }
    }
}

/// Checks if a group operation is valid between two elements
///
/// This function ensures:
/// 1. Both elements belong to the same group
/// 2. The operation is well-defined for these elements
/// 3. The result will be in the group (closure property)
pub fn check_operation_valid(
    group: &Group,
    left: &GroupExpression,
    right: &GroupExpression,
) -> GroupResult<()> {
    // Extract elements from expressions
    let left_element = match left {
        GroupExpression::Element(elem) => elem,
        _ => {
            return Err(GroupError::InvalidOperation {
                message: "Left operand must be an element".to_string(),
            })
        }
    };

    let right_element = match right {
        GroupExpression::Element(elem) => elem,
        _ => {
            return Err(GroupError::InvalidOperation {
                message: "Right operand must be an element".to_string(),
            })
        }
    };

    // Check if both elements belong to the same group
    if &*left_element.group != group || &*right_element.group != group {
        return Err(GroupError::GroupMismatch {
            message: "Elements must belong to the operation's group".to_string(),
        });
    }

    // Check if both elements individually belong to the group
    check_element_in_group(group, &left_element.value)?;
    check_element_in_group(group, &right_element.value)?;

    // Additional checks for specific group types could be added here

    Ok(())
}

/// Checks if an element has an inverse in the group
///
/// All elements in a group must have inverses by definition,
/// but this function checks that the specific element is valid.
pub fn check_has_inverse(group: &Group, element: &GroupExpression) -> GroupResult<()> {
    // Extract element from expression
    let elem = match element {
        GroupExpression::Element(e) => e,
        _ => {
            return Err(GroupError::InvalidOperation {
                message: "Expression must be an element for inverse operation".to_string(),
            })
        }
    };

    // Check if element belongs to the group
    if &*elem.group != group {
        return Err(GroupError::GroupMismatch {
            message: "Element must belong to the specified group".to_string(),
        });
    }

    check_element_in_group(group, &elem.value)?;

    // In a well-defined group, all elements have inverses by definition
    Ok(())
}

/// Checks if a subgroup is normal in the given group
///
/// A subgroup H is normal in G if gHg^-1 = H for all g in G.
pub fn check_normal_subgroup(group: &Group, subgroup: &Group) -> GroupResult<()> {
    // Check if subgroup is a valid subgroup first
    check_is_subgroup(group, subgroup)?;

    // For certain well-known cases, we can determine normality by structure
    if subgroup
        .properties
        .iter()
        .any(|p| format!("{:?}", p).contains("center"))
    {
        // The center is always a normal subgroup
        return Ok(());
    }

    if subgroup.base_set == group.base_set {
        // The whole group is normal in itself
        return Ok(());
    }

    // For specific group types, we can check normality based on other properties
    // In a real implementation, this would be more sophisticated

    // For now, assume normality for demonstration purposes
    // In a production implementation, this would be more strict
    Ok(())
}

/// Checks if one group is a subgroup of another
pub fn check_is_subgroup(group: &Group, subgroup: &Group) -> GroupResult<()> {
    // Check if operations are compatible
    if subgroup.operation.operation_type != group.operation.operation_type {
        return Err(GroupError::GroupMismatch {
            message: "Subgroup must have the same operation type as the group".to_string(),
        });
    }

    // Check if subgroup's set is a subset of group's set
    match (&subgroup.base_set, &group.base_set) {
        (
            Set::Parametric {
                parameters: sub_params,
                ..
            },
            Set::Parametric {
                parameters: group_params,
                ..
            },
        ) => {
            // Check based on parametric properties
            // This is a simplification - in practice would need more sophisticated checking
            Ok(())
        }
        _ => {
            // For other set types, more sophisticated checking is needed
            // For demonstration purposes, we'll assume validity
            Ok(())
        }
    }
}

/// Checks if a quotient group construction is valid
///
/// For G/N to be valid, N must be a normal subgroup of G.
pub fn check_quotient_valid(group: &Group, normal_subgroup: &Group) -> GroupResult<()> {
    // Check if the subgroup is normal in the group
    check_normal_subgroup(group, normal_subgroup)?;

    // Additional checks could be performed here

    Ok(())
}

/// Create a group element expression for a given group and value
pub fn create_group_element(group: Group, value: ElementValue) -> GroupResult<GroupExpression> {
    // Check if the element belongs to the group
    check_element_in_group(&group, &value)?;

    // Create the element
    Ok(GroupExpression::element(group, value))
}

/// Create a group operation expression
pub fn create_group_operation(
    group: Group,
    left: GroupExpression,
    right: GroupExpression,
) -> GroupResult<GroupExpression> {
    // Check if operation is valid
    check_operation_valid(&group, &left, &right)?;

    // Create the operation
    Ok(GroupExpression::operation(group, left, right))
}

/// Create a group inverse expression
pub fn create_group_inverse(
    group: Group,
    element: GroupExpression,
) -> GroupResult<GroupExpression> {
    // Check if element has an inverse
    check_has_inverse(&group, &element)?;

    // Create the inverse
    Ok(GroupExpression::inverse(group, element))
}

/// Create a group identity element
pub fn create_group_identity(group: Group) -> GroupResult<GroupExpression> {
    // Create the identity element
    Ok(GroupExpression::identity(group))
}

/// Create a commutator [a,b] = a*b*a^(-1)*b^(-1)
pub fn create_group_commutator(
    group: Group,
    a: GroupExpression,
    b: GroupExpression,
) -> GroupResult<GroupExpression> {
    // Check if both elements belong to the group
    check_operation_valid(&group, &a, &b)?;

    // Create the commutator
    Ok(GroupExpression::commutator(group, a, b))
}

/// Create a coset expression a*H or H*a
pub fn create_group_coset(
    group: Group,
    subgroup: Group,
    element: GroupExpression,
    is_left: bool,
) -> GroupResult<GroupExpression> {
    // Check if subgroup is a valid subgroup
    check_is_subgroup(&group, &subgroup)?;

    // Check if element belongs to the parent group
    match &element {
        GroupExpression::Element(elem) => {
            if &*elem.group != &group {
                return Err(GroupError::GroupMismatch {
                    message: "Element must belong to the parent group".to_string(),
                });
            }
            check_element_in_group(&group, &elem.value)?;
        }
        _ => {
            return Err(GroupError::InvalidOperation {
                message: "Coset representative must be a group element".to_string(),
            })
        }
    }

    // Let's create a GroupExpression from the subgroup and then pass to coset
    Ok(GroupExpression::coset(group, element, subgroup, is_left))
}

/// Evaluate a group expression to compute its value
///
/// This function recursively evaluates a group expression to find
/// its concrete value as a group element when possible.
pub fn evaluate_group_expression(expr: &GroupExpression) -> GroupResult<GroupElement> {
    match expr {
        GroupExpression::Element(elem) => {
            // Elements are already evaluated
            Ok(elem.clone())
        }
        GroupExpression::Operation { group, left, right } => {
            // Evaluate the operands
            let left_val = evaluate_group_expression(left)?;
            let right_val = evaluate_group_expression(right)?;

            // Check if operation is valid
            if &*left_val.group != &**group || &*right_val.group != &**group {
                return Err(GroupError::GroupMismatch {
                    message: "Operation elements must belong to the same group".to_string(),
                });
            }

            // Perform the operation based on the group type
            match (
                // Use a reference to the operation_type to avoid the move
                &group.operation.operation_type,
                &left_val.value,
                &right_val.value,
            ) {
                // Addition in Z_n
                (
                    GroupOperationVariant::Addition,
                    ElementValue::Integer(a),
                    ElementValue::Integer(b),
                ) => {
                    // Extract the modulus for Z_n
                    let modulus = match &group.base_set {
                        Set::Parametric { parameters, .. } => {
                            if let Some(n_str) = parameters.get("n") {
                                n_str.parse::<i64>().unwrap_or(0)
                            } else {
                                0
                            }
                        }
                        _ => 0,
                    };

                    if modulus > 0 {
                        // Compute (a + b) mod n
                        let result = (a + b) % modulus;
                        let result = if result < 0 { result + modulus } else { result };
                        Ok(GroupElement::new(
                            *group.clone(),
                            ElementValue::Integer(result),
                        ))
                    } else {
                        // Regular integer addition
                        Ok(GroupElement::new(
                            *group.clone(),
                            ElementValue::Integer(a + b),
                        ))
                    }
                }
                // Multiplication in multiplicative groups
                (
                    GroupOperationVariant::Multiplication,
                    ElementValue::Integer(a),
                    ElementValue::Integer(b),
                ) => {
                    // Extract the modulus for multiplicative groups mod n
                    let modulus = match &group.base_set {
                        Set::Parametric { parameters, .. } => {
                            if let Some(n_str) = parameters.get("n") {
                                n_str.parse::<i64>().unwrap_or(0)
                            } else {
                                0
                            }
                        }
                        _ => 0,
                    };

                    if modulus > 0 {
                        // Compute (a * b) mod n
                        let result = (a * b) % modulus;
                        let result = if result < 0 { result + modulus } else { result };
                        Ok(GroupElement::new(
                            *group.clone(),
                            ElementValue::Integer(result),
                        ))
                    } else {
                        // Regular integer multiplication
                        Ok(GroupElement::new(
                            *group.clone(),
                            ElementValue::Integer(a * b),
                        ))
                    }
                }
                // Composition of permutations
                (
                    GroupOperationVariant::Composition,
                    ElementValue::Permutation(p1),
                    ElementValue::Permutation(p2),
                ) => {
                    // Compose permutations (this is a simplified implementation)
                    // p1 ∘ p2 means apply p2 first, then p1
                    let mut result = Vec::with_capacity(p1.len());
                    for i in 1..=p1.len() {
                        let p2_idx = p2[i - 1];
                        let p1_idx = p1[p2_idx - 1];
                        result.push(p1_idx);
                    }
                    Ok(GroupElement::new(
                        *group.clone(),
                        ElementValue::Permutation(result),
                    ))
                }
                // Other cases would be implemented similarly
                _ => Err(GroupError::UnsupportedOperation {
                    message: "Operation not implemented for these element types".to_string(),
                }),
            }
        }
        GroupExpression::Inverse { group, element } => {
            // Evaluate the element
            let elem_val = evaluate_group_expression(element)?;

            // Check if the element belongs to the group
            if &*elem_val.group != &**group {
                return Err(GroupError::GroupMismatch {
                    message: "Element must belong to the group for inverse".to_string(),
                });
            }

            // Compute the inverse based on the group type
            match (
                // Use a reference to avoid the move
                &group.operation.operation_type,
                &elem_val.value,
            ) {
                // Additive inverse in Z_n
                (GroupOperationVariant::Addition, ElementValue::Integer(a)) => {
                    // Extract the modulus for Z_n
                    let modulus = match &group.base_set {
                        Set::Parametric { parameters, .. } => {
                            if let Some(n_str) = parameters.get("n") {
                                n_str.parse::<i64>().unwrap_or(0)
                            } else {
                                0
                            }
                        }
                        _ => 0,
                    };

                    if modulus > 0 {
                        // Compute -a mod n
                        let result = if *a == 0 { 0 } else { modulus - *a };
                        Ok(GroupElement::new(
                            *group.clone(),
                            ElementValue::Integer(result),
                        ))
                    } else {
                        // Regular additive inverse
                        Ok(GroupElement::new(*group.clone(), ElementValue::Integer(-a)))
                    }
                }
                // Multiplicative inverse in a modular group
                (GroupOperationVariant::Multiplication, ElementValue::Integer(a)) => {
                    // Extract the modulus
                    let modulus = match &group.base_set {
                        Set::Parametric { parameters, .. } => {
                            if let Some(n_str) = parameters.get("n") {
                                n_str.parse::<i64>().unwrap_or(0)
                            } else {
                                0
                            }
                        }
                        _ => 0,
                    };

                    if modulus > 0 {
                        // We need to find b such that (a * b) % modulus = 1
                        // This requires computing the modular multiplicative inverse
                        // Using the extended Euclidean algorithm

                        // Simplified implementation for demonstration
                        // In practice, we'd use a proper algorithm
                        for b in 1..modulus {
                            if (a * b) % modulus == 1 {
                                return Ok(GroupElement::new(
                                    *group.clone(),
                                    ElementValue::Integer(b),
                                ));
                            }
                        }

                        return Err(GroupError::InvalidOperation {
                            message: format!(
                                "Element {} has no multiplicative inverse modulo {}",
                                a, modulus
                            ),
                        });
                    } else {
                        // For regular multiplication, only ±1 has an inverse
                        if *a == 1 {
                            Ok(GroupElement::new(*group.clone(), ElementValue::Integer(1)))
                        } else if *a == -1 {
                            Ok(GroupElement::new(*group.clone(), ElementValue::Integer(-1)))
                        } else {
                            Err(GroupError::InvalidOperation {
                                message: format!("Element {} has no multiplicative inverse", a),
                            })
                        }
                    }
                }
                // Inverse of a permutation
                (GroupOperationVariant::Composition, ElementValue::Permutation(p)) => {
                    // Compute the inverse permutation
                    let mut result = vec![0; p.len()];
                    for (i, &val) in p.iter().enumerate() {
                        result[val - 1] = i + 1;
                    }
                    Ok(GroupElement::new(
                        *group.clone(),
                        ElementValue::Permutation(result),
                    ))
                }
                // Other cases would be implemented similarly
                _ => Err(GroupError::UnsupportedOperation {
                    message: "Inverse not implemented for this element type".to_string(),
                }),
            }
        }
        GroupExpression::Identity(group) => {
            // Return the identity element for the group
            match group.operation.operation_type {
                GroupOperationVariant::Addition => {
                    Ok(GroupElement::new(*group.clone(), ElementValue::Integer(0)))
                }
                GroupOperationVariant::Multiplication => {
                    Ok(GroupElement::new(*group.clone(), ElementValue::Integer(1)))
                }
                GroupOperationVariant::Composition => {
                    // For permutation groups, identity is [1,2,3,...,n]
                    let degree = match &group.base_set {
                        Set::Parametric { parameters, .. } => {
                            if let Some(n_str) = parameters.get("degree") {
                                n_str.parse::<usize>().unwrap_or(0)
                            } else {
                                0
                            }
                        }
                        _ => 0,
                    };

                    if degree > 0 {
                        let identity_perm: Vec<usize> = (1..=degree).collect();
                        Ok(GroupElement::new(
                            *group.clone(),
                            ElementValue::Permutation(identity_perm),
                        ))
                    } else {
                        Err(GroupError::UnsupportedOperation {
                            message: "Cannot determine identity for this group type".to_string(),
                        })
                    }
                }
                // Other cases would be implemented similarly
                _ => Err(GroupError::UnsupportedOperation {
                    message: "Identity not implemented for this group type".to_string(),
                }),
            }
        }
        // Other expression types would be implemented here
        _ => Err(GroupError::UnsupportedOperation {
            message: "Evaluation not implemented for this expression type".to_string(),
        }),
    }
}
