//! Macros for type-safe group theory operations
//!
//! This module provides macros for performing type-safe operations in group theory.
//! The macros ensure that elements belong to their respective groups before operations
//! are performed, providing both compile-time and runtime checks.

use crate::formalize_v2::subjects::math::theories::groups::checker::{
    check_element_in_group, check_has_inverse, check_normal_subgroup, check_operation_valid,
    create_group_commutator, create_group_coset, create_group_element, create_group_identity,
    create_group_inverse, create_group_operation, evaluate_group_expression, GroupError,
    GroupResult,
};
use crate::formalize_v2::subjects::math::theories::groups::definitions::{
    AbelianPropertyVariant, ElementValue, FinitePropertyVariant, GroupElement, GroupExpression,
    GroupOperationVariant, GroupProperty,
};
use crate::formalize_v2::subjects::math::theories::groups::helpers::{
    cyclic_group, int, matrix, perm, sym, symmetric_group,
};

/// Macro for type-safe group element creation
///
/// Ensures that the element is a valid member of the group before creating it.
///
/// # Example
///
/// ```
/// let z5 = cyclic_group(5); // Z/5Z
/// let a = group_element!(z5, int(2))?; // Element 2 in Z/5Z
/// ```
#[macro_export]
macro_rules! group_element {
    ($group:expr, $value:expr) => {{
        crate::formalize_v2::subjects::math::theories::groups::checker::create_group_element(
            (*$group).clone(),
            $value,
        )
    }};
}

/// Macro for type-safe group binary operation
///
/// Ensures that both elements belong to the specified group before performing the operation.
///
/// # Example
///
/// ```
/// let z5 = cyclic_group(5); // Z/5Z
/// let a = group_element!(z5, int(2))?; // 2
/// let b = group_element!(z5, int(3))?; // 3
///
/// let product = group_op!(z5, a, b)?; // 2*3 = 0 in Z/5Z
/// ```
#[macro_export]
macro_rules! group_op {
    ($group:expr, $left:expr, $right:expr) => {{
        crate::formalize_v2::subjects::math::theories::groups::checker::create_group_operation(
            (*$group).clone(),
            $left,
            $right,
        )
    }};
}

/// Macro for type-safe group inverse
///
/// Ensures that the element belongs to the specified group before taking its inverse.
///
/// # Example
///
/// ```
/// let z5 = cyclic_group(5); // Z/5Z
/// let a = group_element!(z5, int(2))?; // 2
///
/// let a_inv = group_inverse!(z5, a)?; // 3 (inverse of 2 in Z/5Z)
/// ```
#[macro_export]
macro_rules! group_inverse {
    ($group:expr, $element:expr) => {{
        crate::formalize_v2::subjects::math::theories::groups::checker::create_group_inverse(
            (*$group).clone(),
            $element,
        )
    }};
}

/// Macro for getting the identity element of a group
///
/// # Example
///
/// ```
/// let z5 = cyclic_group(5); // Z/5Z
/// let e = group_identity!(z5)?; // 0 in Z/5Z
/// ```
#[macro_export]
macro_rules! group_identity {
    ($group:expr) => {{
        crate::formalize_v2::subjects::math::theories::groups::checker::create_group_identity(
            (*$group).clone(),
        )
    }};
}

/// Macro for creating a commutator [a,b] = a*b*a^(-1)*b^(-1)
///
/// Ensures that both elements belong to the specified group.
///
/// # Example
///
/// ```
/// let s3 = symmetric_group(3); // S₃
/// let a = group_element!(s3, perm([2, 1, 3]))?; // (1,2)
/// let b = group_element!(s3, perm([1, 3, 2]))?; // (2,3)
///
/// let comm = group_commutator!(s3, a, b)?; // [a,b]
/// ```
#[macro_export]
macro_rules! group_commutator {
    ($group:expr, $a:expr, $b:expr) => {{
        crate::formalize_v2::subjects::math::theories::groups::checker::create_group_commutator(
            (*$group).clone(),
            $a,
            $b,
        )
    }};
}

/// Macro for creating a coset g*H or H*g
///
/// Ensures that the element belongs to the parent group and that H is a subgroup.
///
/// # Example
///
/// ```
/// let s3 = symmetric_group(3); // S₃
/// let a2 = alternating_group(3); // A₃ (subgroup of S₃)
/// let g = group_element!(s3, perm([2, 1, 3]))?; // (1,2)
///
/// let left_coset = group_coset!(s3, a2, g, true)?; // g*A₃
/// ```
#[macro_export]
macro_rules! group_coset {
    ($group:expr, $subgroup:expr, $element:expr, $is_left:expr) => {{
        crate::formalize_v2::subjects::math::theories::groups::checker::create_group_coset(
            (*$group).clone(),
            (*$subgroup).clone(),
            $element,
            $is_left,
        )
    }};
}

/// Macro for checking if an element is in a group
///
/// # Example
///
/// ```
/// let z5 = cyclic_group(5); // Z/5Z
/// let a = group_element!(z5, int(2))?; // 2
/// assert!(is_in_group!(z5, a));
/// ```
#[macro_export]
macro_rules! is_in_group {
    ($group:expr, $element:expr) => {{
        crate::formalize_v2::subjects::math::theories::groups::checker::check_element_in_group(
            $group, $element,
        )
    }};
}

/// Macro for evaluating a group expression
///
/// # Example
///
/// ```
/// let z5 = cyclic_group(5); // Z/5Z
/// let a = group_element!(z5, int(2))?; // 2
/// let b = group_element!(z5, int(3))?; // 3
/// let expr = group_op!(z5, a, b)?; // 2*3
/// let result = eval_group_expr!(expr)?; // 0
/// ```
#[macro_export]
macro_rules! eval_group_expr {
    ($expr:expr) => {{
        crate::formalize_v2::subjects::math::theories::groups::checker::evaluate_group_expression(
            $expr,
        )
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_element() {
        let z5 = cyclic_group(5);
        let g = group_element!(&z5, int(1)).unwrap();
        assert_eq!(
            g,
            GroupExpression::Element(GroupElement {
                group: Box::new(z5.clone()),
                value: ElementValue::Integer(1),
            })
        );
    }

    #[test]
    fn test_group_op() {
        let z5 = cyclic_group(5);
        let g1 = group_element!(&z5, int(1)).unwrap();
        let g2 = group_element!(&z5, int(2)).unwrap();
        let op = group_op!(&z5, g1.clone(), g2.clone()).unwrap();
        assert_eq!(
            op,
            GroupExpression::Operation {
                group: Box::new(z5.clone()),
                left: Box::new(g1),
                right: Box::new(g2),
            }
        );
    }

    #[test]
    fn test_group_inverse() {
        let z5 = cyclic_group(5);
        let g = group_element!(&z5, int(1)).unwrap();
        let inv = group_inverse!(&z5, g.clone()).unwrap();
        assert_eq!(
            inv,
            GroupExpression::Inverse {
                group: Box::new(z5.clone()),
                element: Box::new(g),
            }
        );
    }

    #[test]
    fn test_group_identity() {
        let z5 = cyclic_group(5);
        let id = group_identity!(&z5).unwrap();
        assert_eq!(id, GroupExpression::Identity(Box::new(z5.clone())));
    }

    #[test]
    fn test_group_commutator() {
        let z5 = cyclic_group(5);
        let g1 = group_element!(&z5, int(1)).unwrap();
        let g2 = group_element!(&z5, int(2)).unwrap();
        let comm = group_commutator!(&z5, g1.clone(), g2.clone()).unwrap();
        assert_eq!(
            comm,
            GroupExpression::Commutator {
                group: Box::new(z5.clone()),
                a: Box::new(g1),
                b: Box::new(g2),
            }
        );
    }

    #[test]
    fn test_group_coset() {
        let z5 = cyclic_group(5);
        let g = group_element!(&z5, int(1)).unwrap();
        let coset = group_coset!(&z5, &z5.clone(), g.clone(), true).unwrap();
        assert_eq!(
            coset,
            GroupExpression::Coset {
                group: Box::new(z5.clone()),
                subgroup: Box::new(z5),
                element: Box::new(g),
                is_left: true,
            }
        );
    }

    #[test]
    fn test_cyclic_group() {
        let z5 = cyclic_group(5);
        assert_eq!(z5.operation.operation_type, GroupOperationVariant::Addition);
        assert!(z5
            .properties
            .iter()
            .any(|p| matches!(p, GroupProperty::Finite(FinitePropertyVariant::Finite(5)))));
        assert!(z5
            .properties
            .iter()
            .any(|p| matches!(p, GroupProperty::Abelian(AbelianPropertyVariant::Abelian))));
    }

    #[test]
    fn test_symmetric_group() {
        let s3 = symmetric_group(3);
        assert_eq!(
            s3.operation.operation_type,
            GroupOperationVariant::Composition
        );
        assert!(s3
            .properties
            .iter()
            .any(|p| matches!(p, GroupProperty::Finite(FinitePropertyVariant::Finite(6)))));
        assert!(s3.properties.iter().any(|p| matches!(
            p,
            GroupProperty::Abelian(AbelianPropertyVariant::NonAbelian)
        )));
    }

    #[test]
    fn test_int() {
        let n = int(5);
        assert_eq!(n, ElementValue::Integer(5));
    }

    #[test]
    fn test_perm() {
        let p = perm(vec![1, 2, 3]);
        assert_eq!(p, ElementValue::Permutation(vec![1, 2, 3]));
    }

    #[test]
    fn test_sym() {
        let s = sym("x");
        assert_eq!(s, ElementValue::Symbol("x".to_string()));
    }

    #[test]
    fn test_matrix() {
        let m = matrix(vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(m, ElementValue::Matrix(vec![vec![1, 2], vec![3, 4]]));
    }

    #[test]
    fn test_is_in_group() {
        let z5 = cyclic_group(5);
        let g = group_element!(&z5, int(3)).unwrap();
        match &g {
            GroupExpression::Element(element) => {
                assert!(is_in_group!(&z5, &element.value).is_ok());
            }
            _ => panic!("Expected Element variant"),
        }
    }

    #[test]
    fn test_eval_group_expr() {
        let z5 = cyclic_group(5);
        let g1 = group_element!(&z5, int(1)).unwrap();
        let g2 = group_element!(&z5, int(2)).unwrap();
        let expr = group_op!(&z5, g1, g2).unwrap();
        let result = eval_group_expr!(&expr).unwrap();
        assert_eq!(result.value, ElementValue::Integer(3));
    }
}
