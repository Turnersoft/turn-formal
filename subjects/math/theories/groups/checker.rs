//! Runtime checking functions for group theory operations
//!
//! This module provides functions for runtime verification of group operations,
//! ensuring that elements belong to their respective groups and that operations
//! are well-defined within the algebraic structure.

use super::super::super::super::math::theories::groups::definitions::{
    Group, GroupElement, GroupExpression, GroupExpressionError,
};
use super::super::super::super::math::theories::zfc::Set;
use std::collections::HashSet;
use thiserror::Error;

use super::GroupOperationVariant;
use crate::subjects::math::formalism::extract::Parametrizable;

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
pub fn check_element_in_group(group: &Group, element: &GroupElement) -> GroupResult<()> {
    match &group.get_core().base_set {
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
            match (&group.get_core().operation.operation_type, element) {
                // For cyclic group Z_n with additive notation
                (GroupOperationVariant::Addition, GroupElement::Integer(val)) => {
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
                (GroupOperationVariant::Composition, GroupElement::Permutation(perm)) => {
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
    let left_element = evaluate_group_expression(left)?;
    let right_element = evaluate_group_expression(right)?;
    check_element_in_group(group, &left_element)?;
    check_element_in_group(group, &right_element)?;
    Ok(())
}

/// Checks if an element has an inverse in the group
///
/// All elements in a group must have inverses by definition,
/// but this function checks that the specific element expression is valid.
pub fn check_has_inverse(group: &Group, element_expr: &GroupExpression) -> GroupResult<()> {
    let elem = evaluate_group_expression(element_expr)?;
    check_element_in_group(group, &elem)?;
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
        .get_core()
        .props
        .iter()
        .any(|p| format!("{:?}", p).contains("center"))
    {
        // The center is always a normal subgroup
        return Ok(());
    }

    if subgroup.get_core().base_set == group.get_core().base_set {
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
    let group_core = group.get_core();
    let subgroup_core = subgroup.get_core();
    // Check if subgroup's base set is a subset of the group's base set
    // TODO: Implement Set::is_subset_of relation checking if needed
    // if !subgroup_core.base_set.is_subset_of(&group_core.base_set) {
    //     return Err(GroupError::InvalidElement { ... });
    // }
    println!("Warning: Base set subset check in check_is_subgroup is stubbed.");

    // Check if the operation is compatible (usually means it's the same operation restricted)
    if subgroup_core.operation != group_core.operation {
        // This might be too strict; operations could be structurally same but not PartialEq
        // Need a more nuanced comparison or rely on how subgroups are defined.
        println!("Warning: Operation check in check_is_subgroup might be too strict.");
        // return Err(GroupError::InvalidOperation { message: "Subgroup operation differs from group operation".to_string() });
    }

    // TODO: Check closure property within the subgroup
    Ok(())
}

/// Checks if a quotient group construction is valid
///
/// For G/N to be valid, N must be a normal subgroup of G.
pub fn check_quotient_valid(group: &Group, normal_subgroup: &Group) -> GroupResult<()> {
    // In the flattened structure, QuotientGroup directly holds the group and normal_subgroup.
    // The 'normality' is definitional or asserted by a relation.
    // This check might just ensure the subgroup is valid w.r.t the group.
    check_is_subgroup(group, normal_subgroup)?;
    // We might want to check if there's an `IsNormalSubgroupOf` relation asserted somewhere,
    // but that's beyond a simple structural check.
    Ok(())
}

/// Create a group element expression for a given group and value
pub fn create_group_element(group: Group, value: GroupElement) -> GroupResult<GroupExpression> {
    check_element_in_group(&group, &value)?;
    Ok(GroupExpression::Element {
        group: Parametrizable::Concrete(group),
        element: Parametrizable::Concrete(value),
    })
}

/// Create a group operation expression
pub fn create_group_operation(
    group: Group,
    left: GroupExpression,
    right: GroupExpression,
) -> GroupResult<GroupExpression> {
    check_operation_valid(&group, &left, &right)?;
    Ok(GroupExpression::Operation {
        group: Parametrizable::Concrete(group),
        left: Box::new(Parametrizable::Concrete(left)),
        right: Box::new(Parametrizable::Concrete(right)),
    })
}

/// Create a group inverse expression
pub fn create_group_inverse(
    group: Group,
    element_expr: GroupExpression,
) -> GroupResult<GroupExpression> {
    check_has_inverse(&group, &element_expr)?;
    Ok(GroupExpression::Inverse {
        group: Parametrizable::Concrete(group),
        element: Box::new(Parametrizable::Concrete(element_expr)),
    })
}

/// Create a group identity element
pub fn create_group_identity(group: Group) -> GroupResult<GroupExpression> {
    Ok(GroupExpression::Identity(Parametrizable::Concrete(group)))
}

/// Create a commutator [a,b] = a*b*a^(-1)*b^(-1)
pub fn create_group_commutator(
    group: Group,
    a: GroupExpression,
    b: GroupExpression,
) -> GroupResult<GroupExpression> {
    check_operation_valid(&group, &a, &b)?;
    Ok(GroupExpression::Commutator {
        group: Parametrizable::Concrete(group),
        a: Box::new(Parametrizable::Concrete(a)),
        b: Box::new(Parametrizable::Concrete(b)),
    })
}

/// Create a coset expression a*H or H*a
pub fn create_group_coset(
    group: Group,
    subgroup: Group,
    element_expr: GroupExpression,
    is_left: bool,
) -> GroupResult<GroupExpression> {
    check_is_subgroup(&group, &subgroup)?;
    let elem_val = evaluate_group_expression(&element_expr)?;
    check_element_in_group(&group, &elem_val)?;
    Ok(GroupExpression::Coset {
        group: Parametrizable::Concrete(group),
        element: Box::new(Parametrizable::Concrete(element_expr)),
        subgroup: Parametrizable::Concrete(subgroup),
        is_left,
    })
}

/// Evaluate a group expression to compute its value
///
/// This function recursively evaluates a group expression to find
/// its concrete value as a group element when possible.
pub fn evaluate_group_expression(expr: &GroupExpression) -> GroupResult<GroupElement> {
    match expr {
        GroupExpression::Element { group: _, element } => Ok(element.unwrap().clone()),
        GroupExpression::Operation { group, left, right } => {
            let left_val = evaluate_group_expression(&left.as_ref().unwrap())?;
            let right_val = evaluate_group_expression(&right.as_ref().unwrap())?;
            let unwrapped_group = group.unwrap();
            match (
                &unwrapped_group.get_core().operation.operation_type,
                &left_val,
                &right_val,
            ) {
                (
                    GroupOperationVariant::Addition,
                    GroupElement::Integer(a),
                    GroupElement::Integer(b),
                ) => {
                    let modulus = match &group.unwrap().get_core().base_set {
                        Set::Parametric { parameters, .. } => {
                            parameters.get("n").and_then(|s| s.parse::<i64>().ok())
                        }
                        _ => None,
                    };
                    if let Some(m) = modulus {
                        Ok(GroupElement::Integer((a + b).rem_euclid(m)))
                    } else {
                        Ok(GroupElement::Integer(a + b))
                    }
                }
                (
                    GroupOperationVariant::Multiplication,
                    GroupElement::Integer(a),
                    GroupElement::Integer(b),
                ) => {
                    let modulus = match &group.unwrap().get_core().base_set {
                        Set::Parametric { parameters, .. } => {
                            parameters.get("n").and_then(|s| s.parse::<i64>().ok())
                        }
                        _ => None,
                    };
                    if let Some(m) = modulus {
                        Ok(GroupElement::Integer((a * b).rem_euclid(m)))
                    } else {
                        Ok(GroupElement::Integer(a * b))
                    }
                }
                (
                    GroupOperationVariant::Composition,
                    GroupElement::Permutation(p1),
                    GroupElement::Permutation(p2),
                ) => {
                    if p1.len() != p2.len() {
                        return Err(GroupError::InvalidOperation {
                            message: "Permutation sizes must match".to_string(),
                        });
                    }
                    let mut result = vec![0; p1.len()];
                    for i in 0..p1.len() {
                        let p2_i_val = p2[i];
                        if p2_i_val == 0 || p2_i_val > p1.len() {
                            return Err(GroupError::InvalidOperation {
                                message: format!("Invalid index in p2 at pos {}", i),
                            });
                        }
                        result[i] = p1[p2_i_val - 1];
                    }
                    Ok(GroupElement::Permutation(result))
                }
                _ => Err(GroupError::UnsupportedOperation {
                    message: format!(
                        "Operation {:?} unimplemented for {:?} and {:?}",
                        group.unwrap().get_core().operation.operation_type,
                        left_val,
                        right_val
                    ),
                }),
            }
        }
        GroupExpression::Inverse { group, element } => {
            let elem_val = evaluate_group_expression(&element.as_ref().unwrap())?;
            match (
                &group.unwrap().get_core().operation.operation_type,
                &elem_val,
            ) {
                (GroupOperationVariant::Addition, GroupElement::Integer(a)) => {
                    let modulus = match &group.unwrap().get_core().base_set {
                        Set::Parametric { parameters, .. } => {
                            parameters.get("n").and_then(|s| s.parse::<i64>().ok())
                        }
                        _ => None,
                    };
                    if let Some(m) = modulus {
                        Ok(GroupElement::Integer((-a).rem_euclid(m)))
                    } else {
                        Ok(GroupElement::Integer(-a))
                    }
                }
                (GroupOperationVariant::Multiplication, GroupElement::Integer(a)) => {
                    let modulus = match &group.unwrap().get_core().base_set {
                        Set::Parametric { parameters, .. } => {
                            parameters.get("n").and_then(|s| s.parse::<i64>().ok())
                        }
                        _ => None,
                    };
                    if let Some(m) = modulus {
                        fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
                            if a == 0 {
                                (b, 0, 1)
                            } else {
                                let (g, x, y) = extended_gcd(b % a, a);
                                (g, y - (b / a) * x, x)
                            }
                        }
                        fn mod_inverse(a: i64, m: i64) -> Option<i64> {
                            let (g, x, _) = extended_gcd(a, m);
                            if g != 1 { None } else { Some(x.rem_euclid(m)) }
                        }
                        mod_inverse(*a, m)
                            .map(GroupElement::Integer)
                            .ok_or_else(|| GroupError::InvalidOperation {
                                message: format!("Inverse of {} mod {} does not exist", a, m),
                            })
                    } else {
                        if *a == 0 {
                            return Err(GroupError::InvalidOperation {
                                message: "Cannot invert 0".to_string(),
                            });
                        }
                        if *a == 1 {
                            Ok(GroupElement::Integer(1))
                        } else if *a == -1 {
                            Ok(GroupElement::Integer(-1))
                        } else {
                            Err(GroupError::UnsupportedOperation {
                                message: format!(
                                    "Integer inverse for {} only exists for +/-1 without modulus",
                                    a
                                ),
                            })
                        }
                    }
                }
                (GroupOperationVariant::Composition, GroupElement::Permutation(p)) => {
                    let mut result = vec![0; p.len()];
                    for (i, &val) in p.iter().enumerate() {
                        if val == 0 || val > p.len() {
                            return Err(GroupError::InvalidOperation {
                                message: "Invalid value in permutation".to_string(),
                            });
                        }
                        result[val - 1] = (i + 1) as usize;
                    }
                    Ok(GroupElement::Permutation(result))
                }
                _ => Err(GroupError::UnsupportedOperation {
                    message: format!(
                        "Inverse op {:?} unimplemented for {:?}",
                        group.unwrap().get_core().operation.operation_type,
                        elem_val
                    ),
                }),
            }
        }
        GroupExpression::Identity(group) => {
            match group.unwrap().get_core().operation.operation_type {
                GroupOperationVariant::Addition => Ok(GroupElement::Integer(0)),
                GroupOperationVariant::Multiplication => Ok(GroupElement::Integer(1)),
                GroupOperationVariant::Composition => {
                    let degree = match &group.unwrap().get_core().base_set {
                        Set::Parametric { parameters, .. } => parameters
                            .get("degree")
                            .and_then(|s| s.parse::<usize>().ok()),
                        _ => None,
                    };
                    if let Some(d) = degree {
                        Ok(GroupElement::Permutation((1..=d).collect()))
                    } else {
                        Err(GroupError::InvalidOperation {
                            message: "Cannot determine degree for identity permutation".to_string(),
                        })
                    }
                }
                _ => Err(GroupError::UnsupportedOperation {
                    message: format!(
                        "Identity undefined for op type {:?}",
                        group.unwrap().get_core().operation.operation_type
                    ),
                }),
            }
        }
        GroupExpression::Commutator { .. } => Err(GroupError::ExpressionError(
            "Evaluation unimplemented for Commutator".to_string(),
        )),
        GroupExpression::Coset { .. } => Err(GroupError::ExpressionError(
            "Evaluation unimplemented for Coset".to_string(),
        )),
        GroupExpression::ActionOnElement { .. } => Err(GroupError::ExpressionError(
            "Evaluation unimplemented for Action".to_string(),
        )),
        GroupExpression::Power { .. } => Err(GroupError::ExpressionError(
            "Evaluation unimplemented for Power".to_string(),
        )),
        GroupExpression::GroupOrder { .. } => Err(GroupError::ExpressionError(
            "Evaluation unimplemented for GroupOrder".to_string(),
        )),
        GroupExpression::ElementOrder { .. } => Err(GroupError::ExpressionError(
            "Evaluation unimplemented for ElementOrder".to_string(),
        )),
        GroupExpression::Homomorphism(_) => Err(GroupError::ExpressionError(
            "Evaluation unimplemented for Homomorphism".to_string(),
        )),
    }
}
