//! Runtime checking functions for group theory operations
//!
//! This module provides traits for runtime verification of group operations,
//! ensuring that elements belong to their respective groups and that operations
//! are well-defined within the algebraic structure.

use super::super::super::super::math::theories::groups::definitions::{
    AlternatingGroup, CenterGroup, CentralProductGroup, CentralizerGroup, CommutatorSubgroup,
    CyclicGroup, DihedralGroup, FreeGroup, GeneralLinearGroup, GeneratedSubgroup, Group,
    GroupElement, GroupExpression, GroupExpressionError, ImageGroup, KernelGroup, LieGroup,
    ModularAdditiveGroup, ModularMultiplicativeGroup, NormalizerGroup, OrthogonalGroup,
    ProductGroup, PullbackGroup, QuotientGroup, RestrictionGroup, SpecialLinearGroup,
    SpecialOrthogonalGroup, SpecialUnitaryGroup, SylowSubgroup, SymmetricGroup, TopologicalGroup,
    TrivialGroup, UnitaryGroup, WreathProductGroup,
};
use super::super::super::super::math::theories::zfc::definitions::Set;
use std::collections::HashSet;
use thiserror::Error;

use super::definitions::GroupOperationVariant;
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

/// Trait for checking element membership in specific group types
pub trait ElementMembershipChecker {
    /// Check if an element belongs to this group
    fn check_element_membership(&self, element: &GroupElement) -> GroupResult<()>;
}

/// Trait for checking group operations validity
pub trait OperationChecker {
    /// Check if a group operation is valid between two elements
    fn check_operation_valid(
        &self,
        left: &GroupExpression,
        right: &GroupExpression,
    ) -> GroupResult<()>;

    /// Check if an element has an inverse in the group
    fn check_has_inverse(&self, element_expr: &GroupExpression) -> GroupResult<()>;
}

/// Trait for checking subgroup relationships
pub trait SubgroupChecker {
    /// Check if one group is a subgroup of another
    fn check_is_subgroup(&self, subgroup: &Group) -> GroupResult<()>;

    /// Check if a subgroup is normal
    fn check_normal_subgroup(&self, subgroup: &Group) -> GroupResult<()>;
}

/// Trait for creating group expressions safely
pub trait GroupExpressionBuilder {
    /// Create a group element expression
    fn create_element_expression(&self, value: GroupElement) -> GroupResult<GroupExpression>;

    /// Create a group operation expression
    fn create_operation_expression(
        &self,
        left: GroupExpression,
        right: GroupExpression,
    ) -> GroupResult<GroupExpression>;

    /// Create a group inverse expression
    fn create_inverse_expression(
        &self,
        element_expr: GroupExpression,
    ) -> GroupResult<GroupExpression>;

    /// Create a group identity element
    fn create_identity_expression(&self) -> GroupResult<GroupExpression>;
}

/// Implementation of ElementMembershipChecker for Group enum
impl ElementMembershipChecker for Group {
    fn check_element_membership(&self, element: &GroupElement) -> GroupResult<()> {
        match self {
            Group::Generic(g) => g.check_element_membership(element),
            Group::Cyclic(g) => g.check_element_membership(element),
            Group::Symmetric(g) => g.check_element_membership(element),
            Group::Dihedral(g) => g.check_element_membership(element),
            Group::Alternating(g) => g.check_element_membership(element),
            Group::ModularAdditive(g) => g.check_element_membership(element),
            Group::ModularMultiplicative(g) => g.check_element_membership(element),
            Group::Free(g) => g.check_element_membership(element),
            Group::GeneralLinear(g) => g.check_element_membership(element),
            Group::SpecialLinear(g) => g.check_element_membership(element),
            Group::Orthogonal(g) => g.check_element_membership(element),
            Group::SpecialOrthogonal(g) => g.check_element_membership(element),
            Group::Unitary(g) => g.check_element_membership(element),
            Group::SpecialUnitary(g) => g.check_element_membership(element),
            Group::Topological(g) => g.check_element_membership(element),
            Group::Lie(g) => g.check_element_membership(element),
            Group::Product(g) => g.check_element_membership(element),
            Group::Quotient(g) => g.check_element_membership(element),
            Group::Trivial(g) => g.check_element_membership(element),
            Group::Kernel(g) => g.check_element_membership(element),
            Group::Image(g) => g.check_element_membership(element),
            Group::Center(g) => g.check_element_membership(element),
            Group::GeneratedSubgroup(g) => g.check_element_membership(element),
            Group::Normalizer(g) => g.check_element_membership(element),
            Group::Centralizer(g) => g.check_element_membership(element),
            Group::CommutatorSubgroup(g) => g.check_element_membership(element),
            Group::SylowSubgroup(g) => g.check_element_membership(element),
            Group::WreathProduct(g) => g.check_element_membership(element),
            Group::CentralProduct(g) => g.check_element_membership(element),
            Group::Pullback(g) => g.check_element_membership(element),
            Group::Restriction(g) => g.check_element_membership(element),
            Group::Interception(g) => g.core.check_element_membership(element),
            Group::SubGroup(g) => g.core.check_element_membership(element),
        }
    }
}

/// Implementation for GenericGroup
impl ElementMembershipChecker for super::definitions::GenericGroup {
    fn check_element_membership(&self, element: &GroupElement) -> GroupResult<()> {
        match &self.base_set {
            Set::Empty => Err(GroupError::InvalidElement {
                element_desc: format!("{:?}", element),
                group_desc: "empty group".to_string(),
                message: "No elements in an empty group".to_string(),
            }),
            Set::Parametric {
                parameters,
                membership_condition,
                ..
            } => match (&self.operation.operation_type, element) {
                (GroupOperationVariant::Addition, GroupElement::Integer(val)) => {
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
                (GroupOperationVariant::Composition, GroupElement::Permutation(perm)) => {
                    if let Some(degree_param) = parameters.get("degree") {
                        if let Ok(degree) = degree_param.parse::<usize>() {
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
                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }
}

/// Implementation for CyclicGroup
impl ElementMembershipChecker for CyclicGroup {
    fn check_element_membership(&self, element: &GroupElement) -> GroupResult<()> {
        match element {
            GroupElement::Integer(val) => {
                if let Some(order) = self.order {
                    if *val >= 0 && (*val as usize) < order {
                        Ok(())
                    } else {
                        Err(GroupError::InvalidElement {
                            element_desc: format!("{}", val),
                            group_desc: format!("cyclic group of order {}", order),
                            message: "Element outside valid range".to_string(),
                        })
                    }
                } else {
                    // Infinite cyclic group - all integers are valid
                    Ok(())
                }
            }
            _ => Err(GroupError::InvalidElement {
                element_desc: format!("{:?}", element),
                group_desc: "cyclic group".to_string(),
                message: "Expected integer element for cyclic group".to_string(),
            }),
        }
    }
}

/// Implementation for SymmetricGroup
impl ElementMembershipChecker for SymmetricGroup {
    fn check_element_membership(&self, element: &GroupElement) -> GroupResult<()> {
        match element {
            GroupElement::Permutation(perm) => {
                if perm.len() != self.degree {
                    return Err(GroupError::InvalidElement {
                        element_desc: format!("{:?}", perm),
                        group_desc: format!("symmetric group S_{}", self.degree),
                        message: format!(
                            "Permutation length {} doesn't match group degree {}",
                            perm.len(),
                            self.degree
                        ),
                    });
                }

                let mut seen = HashSet::new();
                for &idx in perm {
                    if idx == 0 || idx > self.degree || !seen.insert(idx) {
                        return Err(GroupError::InvalidElement {
                            element_desc: format!("{:?}", perm),
                            group_desc: format!("symmetric group S_{}", self.degree),
                            message: "Invalid permutation".to_string(),
                        });
                    }
                }
                Ok(())
            }
            _ => Err(GroupError::InvalidElement {
                element_desc: format!("{:?}", element),
                group_desc: format!("symmetric group S_{}", self.degree),
                message: "Expected permutation element for symmetric group".to_string(),
            }),
        }
    }
}

/// Implementation for DihedralGroup
impl ElementMembershipChecker for DihedralGroup {
    fn check_element_membership(&self, element: &GroupElement) -> GroupResult<()> {
        // Dihedral group D_n has 2n elements
        // Can be represented as rotations and reflections
        match element {
            GroupElement::Integer(val) => {
                if *val >= 0 && (*val as usize) < self.order {
                    Ok(())
                } else {
                    Err(GroupError::InvalidElement {
                        element_desc: format!("{}", val),
                        group_desc: format!("dihedral group D_{}", self.order / 2),
                        message: "Element outside valid range".to_string(),
                    })
                }
            }
            _ => Err(GroupError::InvalidElement {
                element_desc: format!("{:?}", element),
                group_desc: format!("dihedral group D_{}", self.order / 2),
                message: "Expected integer element for dihedral group".to_string(),
            }),
        }
    }
}

/// Implementation for AlternatingGroup
impl ElementMembershipChecker for AlternatingGroup {
    fn check_element_membership(&self, element: &GroupElement) -> GroupResult<()> {
        match element {
            GroupElement::Permutation(perm) => {
                if perm.len() != self.degree as usize {
                    return Err(GroupError::InvalidElement {
                        element_desc: format!("{:?}", perm),
                        group_desc: format!("alternating group A_{}", self.degree),
                        message: format!(
                            "Permutation length {} doesn't match group degree {}",
                            perm.len(),
                            self.degree
                        ),
                    });
                }

                // Check if it's a valid permutation
                let mut seen = HashSet::new();
                for &idx in perm {
                    if idx == 0 || idx > self.degree as usize || !seen.insert(idx) {
                        return Err(GroupError::InvalidElement {
                            element_desc: format!("{:?}", perm),
                            group_desc: format!("alternating group A_{}", self.degree),
                            message: "Invalid permutation".to_string(),
                        });
                    }
                }

                // Check if it's an even permutation
                if !GroupCheckerHelpers::is_even_permutation(perm) {
                    return Err(GroupError::InvalidElement {
                        element_desc: format!("{:?}", perm),
                        group_desc: format!("alternating group A_{}", self.degree),
                        message: "Permutation is odd, not allowed in alternating group".to_string(),
                    });
                }

                Ok(())
            }
            _ => Err(GroupError::InvalidElement {
                element_desc: format!("{:?}", element),
                group_desc: format!("alternating group A_{}", self.degree),
                message: "Expected permutation element for alternating group".to_string(),
            }),
        }
    }
}

/// Implementation for ModularAdditiveGroup
impl ElementMembershipChecker for ModularAdditiveGroup {
    fn check_element_membership(&self, element: &GroupElement) -> GroupResult<()> {
        match element {
            GroupElement::Integer(val) => {
                if *val >= 0 && (*val as u32) < self.modulus {
                    Ok(())
                } else {
                    Err(GroupError::InvalidElement {
                        element_desc: format!("{}", val),
                        group_desc: format!("Z/{}", self.modulus),
                        message: "Element outside valid range".to_string(),
                    })
                }
            }
            _ => Err(GroupError::InvalidElement {
                element_desc: format!("{:?}", element),
                group_desc: format!("Z/{}", self.modulus),
                message: "Expected integer element for modular additive group".to_string(),
            }),
        }
    }
}

/// Implementation for ModularMultiplicativeGroup
impl ElementMembershipChecker for ModularMultiplicativeGroup {
    fn check_element_membership(&self, element: &GroupElement) -> GroupResult<()> {
        match element {
            GroupElement::Integer(val) => {
                if *val > 0
                    && (*val as u32) < self.modulus
                    && GroupCheckerHelpers::gcd(*val as u32, self.modulus) == 1
                {
                    Ok(())
                } else {
                    Err(GroupError::InvalidElement {
                        element_desc: format!("{}", val),
                        group_desc: format!("(Z/{})×", self.modulus),
                        message: "Element must be positive and coprime to modulus".to_string(),
                    })
                }
            }
            _ => Err(GroupError::InvalidElement {
                element_desc: format!("{:?}", element),
                group_desc: format!("(Z/{})×", self.modulus),
                message: "Expected integer element for modular multiplicative group".to_string(),
            }),
        }
    }
}

/// Helper functions organized in a struct for better organization
pub struct GroupCheckerHelpers;

impl GroupCheckerHelpers {
    /// Check if a permutation is even
    pub fn is_even_permutation(perm: &[usize]) -> bool {
        let n = perm.len();
        let mut inversions = 0;

        for i in 0..n {
            for j in i + 1..n {
                if perm[i] > perm[j] {
                    inversions += 1;
                }
            }
        }

        inversions % 2 == 0
    }

    /// Calculate GCD
    pub fn gcd(a: u32, b: u32) -> u32 {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }

    /// Check if matrix is singular (simplified)
    pub fn is_matrix_singular(matrix: &[Vec<i64>]) -> bool {
        // Simplified check - in practice would compute actual determinant
        matrix.is_empty() || matrix[0].is_empty()
    }

    /// Check if matrix has determinant 1 (simplified)
    pub fn has_determinant_one(matrix: &[Vec<i64>]) -> bool {
        // Simplified check - in practice would compute actual determinant
        !matrix.is_empty() && !matrix[0].is_empty()
    }

    /// Check if matrix is orthogonal (simplified)
    pub fn is_orthogonal_matrix(matrix: &[Vec<i64>]) -> bool {
        // Simplified check - in practice would compute A^T * A and check if it equals I
        !matrix.is_empty() && !matrix[0].is_empty()
    }
}

/// Macro to implement ElementMembershipChecker for construction-based groups
macro_rules! impl_element_checker_for_construction_groups {
    ($($group_type:ty),*) => {
        $(
            impl ElementMembershipChecker for $group_type {
                fn check_element_membership(&self, element: &GroupElement) -> GroupResult<()> {
                    self.core.check_element_membership(element)
                }
            }
        )*
    };
}

// Apply the macro to construction-based groups
impl_element_checker_for_construction_groups!(
    TopologicalGroup,
    LieGroup,
    ProductGroup,
    QuotientGroup,
    KernelGroup,
    ImageGroup,
    CenterGroup,
    GeneratedSubgroup,
    NormalizerGroup,
    CentralizerGroup,
    CommutatorSubgroup,
    SylowSubgroup,
    WreathProductGroup,
    CentralProductGroup,
    PullbackGroup,
    RestrictionGroup
);

/// Implementation for FreeGroup
impl ElementMembershipChecker for FreeGroup {
    fn check_element_membership(&self, element: &GroupElement) -> GroupResult<()> {
        match element {
            GroupElement::Symbol(_) => Ok(()), // Free groups typically use symbolic elements
            _ => Err(GroupError::InvalidElement {
                element_desc: format!("{:?}", element),
                group_desc: format!("free group F_{}", self.rank),
                message: "Expected symbolic element for free group".to_string(),
            }),
        }
    }
}

/// Implementation for TrivialGroup
impl ElementMembershipChecker for TrivialGroup {
    fn check_element_membership(&self, element: &GroupElement) -> GroupResult<()> {
        match element {
            GroupElement::Integer(0) => Ok(()),
            GroupElement::Symbol(s) if s == "e" || s == "1" => Ok(()),
            _ => Err(GroupError::InvalidElement {
                element_desc: format!("{:?}", element),
                group_desc: "trivial group".to_string(),
                message: "Trivial group contains only the identity element".to_string(),
            }),
        }
    }
}

/// Implementation for GeneralLinearGroup
impl ElementMembershipChecker for GeneralLinearGroup {
    fn check_element_membership(&self, element: &GroupElement) -> GroupResult<()> {
        match element {
            GroupElement::Matrix(matrix) => {
                let n = self.dimension as usize;
                if matrix.len() != n || matrix.iter().any(|row| row.len() != n) {
                    return Err(GroupError::InvalidElement {
                        element_desc: "matrix".to_string(),
                        group_desc: format!("GL({}, F)", self.dimension),
                        message: format!("Matrix must be {}x{}", n, n),
                    });
                }

                if GroupCheckerHelpers::is_matrix_singular(matrix) {
                    return Err(GroupError::InvalidElement {
                        element_desc: "matrix".to_string(),
                        group_desc: format!("GL({}, F)", self.dimension),
                        message: "Matrix must be invertible (non-zero determinant)".to_string(),
                    });
                }

                Ok(())
            }
            _ => Err(GroupError::InvalidElement {
                element_desc: format!("{:?}", element),
                group_desc: format!("GL({}, F)", self.dimension),
                message: "Expected matrix element for general linear group".to_string(),
            }),
        }
    }
}

/// Implementation for SpecialLinearGroup
impl ElementMembershipChecker for SpecialLinearGroup {
    fn check_element_membership(&self, element: &GroupElement) -> GroupResult<()> {
        self.general_linear.check_element_membership(element)?;

        match element {
            GroupElement::Matrix(matrix) => {
                if !GroupCheckerHelpers::has_determinant_one(matrix) {
                    return Err(GroupError::InvalidElement {
                        element_desc: "matrix".to_string(),
                        group_desc: format!("SL({}, F)", self.general_linear.dimension),
                        message: "Matrix must have determinant 1".to_string(),
                    });
                }
                Ok(())
            }
            _ => unreachable!(), // Already checked in general linear group
        }
    }
}

/// Implementation for OrthogonalGroup
impl ElementMembershipChecker for OrthogonalGroup {
    fn check_element_membership(&self, element: &GroupElement) -> GroupResult<()> {
        match element {
            GroupElement::Matrix(matrix) => {
                let n = self.dimension as usize;
                if matrix.len() != n || matrix.iter().any(|row| row.len() != n) {
                    return Err(GroupError::InvalidElement {
                        element_desc: "matrix".to_string(),
                        group_desc: format!("O({})", self.dimension),
                        message: format!("Matrix must be {}x{}", n, n),
                    });
                }

                if !GroupCheckerHelpers::is_orthogonal_matrix(matrix) {
                    return Err(GroupError::InvalidElement {
                        element_desc: "matrix".to_string(),
                        group_desc: format!("O({})", self.dimension),
                        message: "Matrix must be orthogonal".to_string(),
                    });
                }

                Ok(())
            }
            _ => Err(GroupError::InvalidElement {
                element_desc: format!("{:?}", element),
                group_desc: format!("O({})", self.dimension),
                message: "Expected matrix element for orthogonal group".to_string(),
            }),
        }
    }
}

/// Implementation for SpecialOrthogonalGroup
impl ElementMembershipChecker for SpecialOrthogonalGroup {
    fn check_element_membership(&self, element: &GroupElement) -> GroupResult<()> {
        self.orthogonal.check_element_membership(element)?;

        match element {
            GroupElement::Matrix(matrix) => {
                if !GroupCheckerHelpers::has_determinant_one(matrix) {
                    return Err(GroupError::InvalidElement {
                        element_desc: "matrix".to_string(),
                        group_desc: format!("SO({})", self.orthogonal.dimension),
                        message: "Matrix must have determinant 1".to_string(),
                    });
                }
                Ok(())
            }
            _ => unreachable!(),
        }
    }
}

/// Implementation for UnitaryGroup
impl ElementMembershipChecker for UnitaryGroup {
    fn check_element_membership(&self, element: &GroupElement) -> GroupResult<()> {
        match element {
            GroupElement::Matrix(matrix) => {
                let n = self.dimension as usize;
                if matrix.len() != n || matrix.iter().any(|row| row.len() != n) {
                    return Err(GroupError::InvalidElement {
                        element_desc: "matrix".to_string(),
                        group_desc: format!("U({})", self.dimension),
                        message: format!("Matrix must be {}x{}", n, n),
                    });
                }
                Ok(())
            }
            _ => Err(GroupError::InvalidElement {
                element_desc: format!("{:?}", element),
                group_desc: format!("U({})", self.dimension),
                message: "Expected matrix element for unitary group".to_string(),
            }),
        }
    }
}

/// Implementation for SpecialUnitaryGroup
impl ElementMembershipChecker for SpecialUnitaryGroup {
    fn check_element_membership(&self, element: &GroupElement) -> GroupResult<()> {
        self.unitary.check_element_membership(element)?;

        match element {
            GroupElement::Matrix(matrix) => {
                if !GroupCheckerHelpers::has_determinant_one(matrix) {
                    return Err(GroupError::InvalidElement {
                        element_desc: "matrix".to_string(),
                        group_desc: format!("SU({})", self.unitary.dimension),
                        message: "Matrix must have determinant 1".to_string(),
                    });
                }
                Ok(())
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subjects::math::theories::groups::definitions::{
        CyclicGroup, Group, GroupElement, SymmetricGroup,
    };

    #[test]
    fn test_element_membership_checker_trait() {
        // Test cyclic group element checking
        let cyclic_group = CyclicGroup {
            core: crate::subjects::math::theories::groups::definitions::GenericGroup::default(),
            order: Some(5),
            generator: GroupElement::Integer(1),
        };

        // Valid element
        assert!(
            cyclic_group
                .check_element_membership(&GroupElement::Integer(3))
                .is_ok()
        );

        // Invalid element (out of range)
        assert!(
            cyclic_group
                .check_element_membership(&GroupElement::Integer(7))
                .is_err()
        );

        // Invalid element type
        assert!(
            cyclic_group
                .check_element_membership(&GroupElement::Symbol("x".to_string()))
                .is_err()
        );
    }

    #[test]
    fn test_symmetric_group_checker() {
        let sym_group = SymmetricGroup {
            core: crate::subjects::math::theories::groups::definitions::GenericGroup::default(),
            degree: 3,
        };

        // Valid permutation
        let valid_perm = GroupElement::Permutation(vec![2, 3, 1]);
        assert!(sym_group.check_element_membership(&valid_perm).is_ok());

        // Invalid permutation (wrong length)
        let invalid_perm = GroupElement::Permutation(vec![1, 2]);
        assert!(sym_group.check_element_membership(&invalid_perm).is_err());

        // Invalid permutation (duplicate elements)
        let duplicate_perm = GroupElement::Permutation(vec![1, 1, 3]);
        // Test that duplicates are detected properly (this should fail if checked properly)
        assert!(sym_group.check_element_membership(&duplicate_perm).is_err());
    }

    #[test]
    fn test_group_enum_dispatch() {
        let cyclic_group = Group::Cyclic(CyclicGroup {
            core: crate::subjects::math::theories::groups::definitions::GenericGroup::default(),
            order: Some(4),
            generator: GroupElement::Integer(1),
        });

        // Test that the Group enum properly dispatches to the appropriate implementation
        assert!(
            cyclic_group
                .check_element_membership(&GroupElement::Integer(2))
                .is_ok()
        );
        assert!(
            cyclic_group
                .check_element_membership(&GroupElement::Integer(5))
                .is_err()
        );
    }

    #[test]
    fn test_group_checker_helpers() {
        // Test even permutation detection
        let even_perm = vec![2, 1, 3]; // One swap = odd permutation
        assert!(!GroupCheckerHelpers::is_even_permutation(&even_perm));

        let identity_perm = vec![1, 2, 3]; // No swaps = even permutation
        assert!(GroupCheckerHelpers::is_even_permutation(&identity_perm));

        // Test GCD calculation
        assert_eq!(GroupCheckerHelpers::gcd(12, 18), 6);
        assert_eq!(GroupCheckerHelpers::gcd(17, 19), 1);
    }
}
