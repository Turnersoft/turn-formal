//! Tests for group theory definitions and operations.
//!
//! This file contains tests that verify the creation, manipulation, and properties
//! of all mathematical objects defined in the group theory module.

use crate::subjects::math::theories::groups::definitions::*;
use crate::subjects::math::theories::zfc::set::CardinalityPropertyVariant;
use crate::subjects::math::theories::zfc::set::{Set, SetProperty};
use crate::subjects::math::theories::VariantSet;
use serde_json::{from_str, to_string};
use std::cmp::PartialEq;

// Helper function to create a test Set
fn create_test_set(name: &str, size: Option<u32>) -> Set {
    let mut properties = VariantSet::new();
    if let Some(size) = size {
        properties.insert(SetProperty::Cardinality(
            CardinalityPropertyVariant::Finite(size as usize),
        ));
    }
    Set::Singleton {
        element: Box::new(Set::Empty),
        properties,
    }
}

#[cfg(test)]
mod group_operation_tests {
    use super::*;

    #[test]
    fn test_group_operation_creation() {
        // Test creating different group operations
        let multiplication = GroupOperation {
            operation_type: GroupOperationVariant::Multiplication,
            notation: GroupNotation::Infix(GroupSymbol::Times),
            identity: GroupIdentity::One,
            inverse: GroupInverse::MultiplicativeInverse,
            inverse_application: GroupInverseApplication::TwoSided,
            properties: vec![
                GroupOperationProperty::Associative,
                GroupOperationProperty::Closed,
                GroupOperationProperty::Commutative(true),
            ],
        };

        let addition = GroupOperation {
            operation_type: GroupOperationVariant::Addition,
            notation: GroupNotation::Infix(GroupSymbol::Plus),
            identity: GroupIdentity::Zero,
            inverse: GroupInverse::AdditiveInverse,
            inverse_application: GroupInverseApplication::TwoSided,
            properties: vec![
                GroupOperationProperty::Associative,
                GroupOperationProperty::Closed,
                GroupOperationProperty::Commutative(true),
            ],
        };

        let composition = GroupOperation {
            operation_type: GroupOperationVariant::Composition,
            notation: GroupNotation::Infix(GroupSymbol::Circle),
            identity: GroupIdentity::IdentityFunction,
            inverse: GroupInverse::FunctionInverse,
            inverse_application: GroupInverseApplication::TwoSided,
            properties: vec![
                GroupOperationProperty::Associative,
                GroupOperationProperty::Closed,
                GroupOperationProperty::Commutative(false),
            ],
        };

        // Verify operations have correct types
        assert!(matches!(
            multiplication.operation_type,
            GroupOperationVariant::Multiplication
        ));
        assert!(matches!(
            addition.operation_type,
            GroupOperationVariant::Addition
        ));
        assert!(matches!(
            composition.operation_type,
            GroupOperationVariant::Composition
        ));

        // Verify notations
        assert!(matches!(
            multiplication.notation,
            GroupNotation::Infix(GroupSymbol::Times)
        ));
        assert!(matches!(
            addition.notation,
            GroupNotation::Infix(GroupSymbol::Plus)
        ));
        assert!(matches!(
            composition.notation,
            GroupNotation::Infix(GroupSymbol::Circle)
        ));

        // Verify identities
        assert!(matches!(multiplication.identity, GroupIdentity::One));
        assert!(matches!(addition.identity, GroupIdentity::Zero));
        assert!(matches!(
            composition.identity,
            GroupIdentity::IdentityFunction
        ));

        // Verify inverses
        assert!(matches!(
            multiplication.inverse,
            GroupInverse::MultiplicativeInverse
        ));
        assert!(matches!(addition.inverse, GroupInverse::AdditiveInverse));
        assert!(matches!(composition.inverse, GroupInverse::FunctionInverse));

        // Verify commutativity
        let is_mult_commutative = multiplication.properties.iter().any(|p| {
            if let GroupOperationProperty::Commutative(commutative) = p {
                *commutative
            } else {
                false
            }
        });

        let is_comp_commutative = composition.properties.iter().any(|p| {
            if let GroupOperationProperty::Commutative(commutative) = p {
                *commutative
            } else {
                false
            }
        });

        assert!(is_mult_commutative);
        assert!(!is_comp_commutative);
    }

    #[test]
    fn test_custom_group_operation() {
        // Using standard variants as Custom variants were removed
        let custom_op = GroupOperation {
            operation_type: GroupOperationVariant::FreeProduct, // Was Custom("Star operation".to_string())
            notation: GroupNotation::Infix(GroupSymbol::DirectProduct), // Was Infix(GroupSymbol::Custom("★".to_string()))
            identity: GroupIdentity::IdentityFunction, // Was Custom("e".to_string())
            inverse: GroupInverse::FunctionInverse,    // Was Custom("star-inverse".to_string())
            inverse_application: GroupInverseApplication::TwoSided,
            properties: vec![
                GroupOperationProperty::Associative,
                GroupOperationProperty::Closed,
                GroupOperationProperty::Commutative(false),
            ],
        };

        // Verify using standard variants now
        assert!(matches!(
            custom_op.operation_type,
            GroupOperationVariant::FreeProduct // Was checking for Custom variant
        ));
        assert!(matches!(
            custom_op.notation,
            GroupNotation::Infix(GroupSymbol::DirectProduct) // Was checking for Custom variant
        ));
        assert!(matches!(
            custom_op.identity,
            GroupIdentity::IdentityFunction // Was checking for Custom variant
        ));
        assert!(matches!(
            custom_op.inverse,
            GroupInverse::FunctionInverse // Was checking for Custom variant
        ));
    }

    #[test]
    fn test_inverse_application_types() {
        // Create operations with different inverse applications
        let left_inverse_op = GroupOperation {
            operation_type: GroupOperationVariant::Multiplication,
            notation: GroupNotation::Infix(GroupSymbol::Times),
            identity: GroupIdentity::One,
            inverse: GroupInverse::MultiplicativeInverse,
            inverse_application: GroupInverseApplication::Left,
            properties: vec![
                GroupOperationProperty::Associative,
                GroupOperationProperty::Closed,
            ],
        };

        let right_inverse_op = GroupOperation {
            operation_type: GroupOperationVariant::Multiplication,
            notation: GroupNotation::Infix(GroupSymbol::Times),
            identity: GroupIdentity::One,
            inverse: GroupInverse::MultiplicativeInverse,
            inverse_application: GroupInverseApplication::Right,
            properties: vec![
                GroupOperationProperty::Associative,
                GroupOperationProperty::Closed,
            ],
        };

        let two_sided_inverse_op = GroupOperation {
            operation_type: GroupOperationVariant::Multiplication,
            notation: GroupNotation::Infix(GroupSymbol::Times),
            identity: GroupIdentity::One,
            inverse: GroupInverse::MultiplicativeInverse,
            inverse_application: GroupInverseApplication::TwoSided,
            properties: vec![
                GroupOperationProperty::Associative,
                GroupOperationProperty::Closed,
            ],
        };

        // Verify inverse application types
        assert!(matches!(
            left_inverse_op.inverse_application,
            GroupInverseApplication::Left
        ));
        assert!(matches!(
            right_inverse_op.inverse_application,
            GroupInverseApplication::Right
        ));
        assert!(matches!(
            two_sided_inverse_op.inverse_application,
            GroupInverseApplication::TwoSided
        ));
    }
}

#[cfg(test)]
mod group_tests {
    use super::*;

    fn create_test_group() -> Group {
        let base_set = create_test_set("Z", None);
        let operation = GroupOperation {
            operation_type: GroupOperationVariant::Addition,
            notation: GroupNotation::Infix(GroupSymbol::Plus),
            identity: GroupIdentity::Zero,
            inverse: GroupInverse::AdditiveInverse,
            inverse_application: GroupInverseApplication::TwoSided,
            properties: vec![
                GroupOperationProperty::Associative,
                GroupOperationProperty::Closed,
                GroupOperationProperty::Commutative(true),
            ],
        };

        Group {
            base_set,
            operation,
            properties: vec![
                GroupProperty::Abelian(AbelianPropertyVariant::Abelian),
                GroupProperty::Finite(FinitePropertyVariant::Infinite),
                GroupProperty::Simple(SimplePropertyVariant::NonSimple),
            ],
        }
    }

    #[test]
    fn test_group_creation() {
        let group = create_test_group();

        // Verify group components
        assert!(group.base_set.get_properties().inner.is_empty());
        assert!(matches!(
            group.operation.operation_type,
            GroupOperationVariant::Addition
        ));

        // Check group properties
        let is_abelian = group
            .properties
            .iter()
            .any(|p| matches!(p, GroupProperty::Abelian(AbelianPropertyVariant::Abelian)));

        let is_infinite = group
            .properties
            .iter()
            .any(|p| matches!(p, GroupProperty::Finite(FinitePropertyVariant::Infinite)));

        assert!(is_abelian);
        assert!(is_infinite);
    }

    #[test]
    fn test_group_serialization() {
        let group = create_test_group();

        // Test serialization
        let serialized = to_string(&group).expect("Failed to serialize group");
        assert!(!serialized.is_empty());

        // Test deserialization
        let deserialized: Group = from_str(&serialized).expect("Failed to deserialize group");

        // Verify that deserialized group matches original
        assert!(
            deserialized.base_set.get_properties().inner.is_empty()
                == group.base_set.get_properties().inner.is_empty()
        );
        assert!(matches!(
            deserialized.operation.operation_type,
            GroupOperationVariant::Addition
        ));
        assert!(matches!(
            deserialized.operation.identity,
            GroupIdentity::Zero
        ));

        // Check that properties are preserved
        let is_abelian = deserialized
            .properties
            .iter()
            .any(|p| matches!(p, GroupProperty::Abelian(AbelianPropertyVariant::Abelian)));
        assert!(is_abelian);
    }

    #[test]
    fn test_various_group_types() {
        // Symmetric group S3
        let s3_set = create_test_set("S3", Some(6));
        let s3_operation = GroupOperation {
            operation_type: GroupOperationVariant::Composition,
            notation: GroupNotation::Infix(GroupSymbol::Circle),
            identity: GroupIdentity::IdentityPermutation,
            inverse: GroupInverse::PermutationInverse,
            inverse_application: GroupInverseApplication::TwoSided,
            properties: vec![
                GroupOperationProperty::Associative,
                GroupOperationProperty::Closed,
                GroupOperationProperty::Commutative(false),
            ],
        };

        let symmetric_group = Group {
            base_set: s3_set,
            operation: s3_operation,
            properties: vec![
                GroupProperty::Abelian(AbelianPropertyVariant::NonAbelian),
                GroupProperty::Finite(FinitePropertyVariant::Finite(6)),
                GroupProperty::Simple(SimplePropertyVariant::NonSimple),
            ],
        };

        // Cyclic group Z/2Z
        let z2_set = create_test_set("Z/2Z", Some(2));
        let z2_operation = GroupOperation {
            operation_type: GroupOperationVariant::Addition,
            notation: GroupNotation::Infix(GroupSymbol::Plus),
            identity: GroupIdentity::Zero,
            inverse: GroupInverse::AdditiveInverse,
            inverse_application: GroupInverseApplication::TwoSided,
            properties: vec![
                GroupOperationProperty::Associative,
                GroupOperationProperty::Closed,
                GroupOperationProperty::Commutative(true),
            ],
        };

        let cyclic_group = Group {
            base_set: z2_set,
            operation: z2_operation,
            properties: vec![
                GroupProperty::Abelian(AbelianPropertyVariant::Abelian),
                GroupProperty::Finite(FinitePropertyVariant::Finite(2)),
                GroupProperty::Simple(SimplePropertyVariant::Simple),
            ],
        };

        // Verify S3 properties
        assert!(matches!(
            symmetric_group.operation.operation_type,
            GroupOperationVariant::Composition
        ));
        assert!(symmetric_group.properties.iter().any(|p| matches!(
            p,
            GroupProperty::Abelian(AbelianPropertyVariant::NonAbelian)
        )));
        assert!(symmetric_group
            .properties
            .iter()
            .any(|p| matches!(p, GroupProperty::Finite(FinitePropertyVariant::Finite(6)))));

        // Verify Z/2Z properties
        assert!(matches!(
            cyclic_group.operation.operation_type,
            GroupOperationVariant::Addition
        ));
        assert!(cyclic_group
            .properties
            .iter()
            .any(|p| matches!(p, GroupProperty::Abelian(AbelianPropertyVariant::Abelian))));
        assert!(cyclic_group
            .properties
            .iter()
            .any(|p| matches!(p, GroupProperty::Finite(FinitePropertyVariant::Finite(2)))));
        assert!(cyclic_group
            .properties
            .iter()
            .any(|p| matches!(p, GroupProperty::Simple(SimplePropertyVariant::Simple))));
    }
}

#[cfg(test)]
mod topological_group_tests {
    use super::*;

    #[test]
    fn test_topological_group_creation() {
        // Create a base group
        let base_set = create_test_set("R", None);
        let operation = GroupOperation {
            operation_type: GroupOperationVariant::Addition,
            notation: GroupNotation::Infix(GroupSymbol::Plus),
            identity: GroupIdentity::Zero,
            inverse: GroupInverse::AdditiveInverse,
            inverse_application: GroupInverseApplication::TwoSided,
            properties: vec![
                GroupOperationProperty::Associative,
                GroupOperationProperty::Closed,
                GroupOperationProperty::Commutative(true),
            ],
        };

        let group = Group {
            base_set,
            operation,
            properties: vec![
                GroupProperty::Abelian(AbelianPropertyVariant::Abelian),
                GroupProperty::Finite(FinitePropertyVariant::Infinite),
            ],
        };

        // Create a topological group
        let topological_group = TopologicalGroup {
            group,
            properties: vec![
                TopologicalGroupProperty::Connected(ConnectedPropertyVariant::Connected),
                TopologicalGroupProperty::Connected(ConnectedPropertyVariant::LocallyConnected),
            ],
        };

        // Verify topological group properties
        assert!(topological_group.properties.iter().any(|p| matches!(
            p,
            TopologicalGroupProperty::Connected(ConnectedPropertyVariant::Connected)
        )));

        assert!(topological_group.properties.iter().any(|p| matches!(
            p,
            TopologicalGroupProperty::Connected(ConnectedPropertyVariant::LocallyConnected)
        )));

        // Verify that the underlying group is abelian and infinite
        assert!(topological_group
            .group
            .properties
            .iter()
            .any(|p| matches!(p, GroupProperty::Abelian(AbelianPropertyVariant::Abelian))));

        assert!(topological_group
            .group
            .properties
            .iter()
            .any(|p| matches!(p, GroupProperty::Finite(FinitePropertyVariant::Infinite))));
    }

    #[test]
    fn test_various_topological_groups() {
        // Create circle group S¹
        let circle_set = create_test_set("S¹", None);
        let circle_operation = GroupOperation {
            operation_type: GroupOperationVariant::Multiplication,
            notation: GroupNotation::Infix(GroupSymbol::Times),
            identity: GroupIdentity::One,
            inverse: GroupInverse::MultiplicativeInverse,
            inverse_application: GroupInverseApplication::TwoSided,
            properties: vec![
                GroupOperationProperty::Associative,
                GroupOperationProperty::Closed,
                GroupOperationProperty::Commutative(true),
            ],
        };

        let circle_group = Group {
            base_set: circle_set,
            operation: circle_operation,
            properties: vec![
                GroupProperty::Abelian(AbelianPropertyVariant::Abelian),
                GroupProperty::Finite(FinitePropertyVariant::Infinite),
                GroupProperty::Simple(SimplePropertyVariant::NonSimple),
            ],
        };

        let circle_topological_group = TopologicalGroup {
            group: circle_group,
            properties: vec![
                TopologicalGroupProperty::Compact(CompactPropertyVariant::Compact),
                TopologicalGroupProperty::Connected(ConnectedPropertyVariant::Connected),
            ],
        };

        // Create discrete group Z with discrete topology
        let integer_set = create_test_set("Z", None);
        let integer_operation = GroupOperation {
            operation_type: GroupOperationVariant::Addition,
            notation: GroupNotation::Infix(GroupSymbol::Plus),
            identity: GroupIdentity::Zero,
            inverse: GroupInverse::AdditiveInverse,
            inverse_application: GroupInverseApplication::TwoSided,
            properties: vec![
                GroupOperationProperty::Associative,
                GroupOperationProperty::Closed,
                GroupOperationProperty::Commutative(true),
            ],
        };

        let integer_group = Group {
            base_set: integer_set,
            operation: integer_operation,
            properties: vec![
                GroupProperty::Abelian(AbelianPropertyVariant::Abelian),
                GroupProperty::Finite(FinitePropertyVariant::Infinite),
            ],
        };

        let discrete_topological_group = TopologicalGroup {
            group: integer_group,
            properties: vec![
                TopologicalGroupProperty::Connected(ConnectedPropertyVariant::TotallyDisconnected),
                TopologicalGroupProperty::Metrizable(MetrizablePropertyVariant::Metrizable),
            ],
        };

        // Verify circle group properties
        assert!(circle_topological_group.properties.iter().any(|p| matches!(
            p,
            TopologicalGroupProperty::Compact(CompactPropertyVariant::Compact)
        )));

        assert!(circle_topological_group.properties.iter().any(|p| matches!(
            p,
            TopologicalGroupProperty::Connected(ConnectedPropertyVariant::Connected)
        )));

        // Verify discrete group properties
        assert!(discrete_topological_group
            .properties
            .iter()
            .any(|p| matches!(
                p,
                TopologicalGroupProperty::Connected(ConnectedPropertyVariant::TotallyDisconnected)
            )));

        assert!(discrete_topological_group
            .properties
            .iter()
            .any(|p| matches!(
                p,
                TopologicalGroupProperty::Metrizable(MetrizablePropertyVariant::Metrizable)
            )));
    }
}

#[cfg(test)]
mod lie_group_tests {
    use super::*;

    #[test]
    fn test_lie_group_creation() {
        // First create a base group
        let base_set = create_test_set("GL(n,R)", None);
        let operation = GroupOperation {
            operation_type: GroupOperationVariant::MatrixMultiplication,
            notation: GroupNotation::Infix(GroupSymbol::Times),
            identity: GroupIdentity::IdentityMatrix,
            inverse: GroupInverse::MatrixInverse,
            inverse_application: GroupInverseApplication::TwoSided,
            properties: vec![
                GroupOperationProperty::Associative,
                GroupOperationProperty::Closed,
                GroupOperationProperty::Commutative(false),
            ],
        };

        let group = Group {
            base_set,
            operation,
            properties: vec![
                GroupProperty::Abelian(AbelianPropertyVariant::NonAbelian),
                GroupProperty::Finite(FinitePropertyVariant::Infinite),
            ],
        };

        // Create a topological group
        let topological_group = TopologicalGroup {
            group,
            properties: vec![
                TopologicalGroupProperty::Connected(ConnectedPropertyVariant::Connected),
                TopologicalGroupProperty::Connected(ConnectedPropertyVariant::LocallyConnected),
            ],
        };

        // Create a Lie group
        let lie_group = LieGroup {
            topological_group,
            properties: vec![
                LieGroupProperty::Semisimple(SemisimplePropertyVariant::Semisimple),
                LieGroupProperty::Reductive(ReductivePropertyVariant::Reductive),
            ],
        };

        // Verify Lie group properties
        assert!(lie_group.properties.iter().any(|p| matches!(
            p,
            LieGroupProperty::Semisimple(SemisimplePropertyVariant::Semisimple)
        )));

        assert!(lie_group.properties.iter().any(|p| matches!(
            p,
            LieGroupProperty::Reductive(ReductivePropertyVariant::Reductive)
        )));

        // Verify topological properties
        assert!(lie_group
            .topological_group
            .properties
            .iter()
            .any(|p| matches!(
                p,
                TopologicalGroupProperty::Connected(ConnectedPropertyVariant::Connected)
            )));

        // Verify group properties
        assert!(lie_group
            .topological_group
            .group
            .properties
            .iter()
            .any(|p| matches!(
                p,
                GroupProperty::Abelian(AbelianPropertyVariant::NonAbelian)
            )));

        assert!(matches!(
            lie_group.topological_group.group.operation.operation_type,
            GroupOperationVariant::MatrixMultiplication
        ));
    }

    #[test]
    fn test_various_lie_groups() {
        // Create SO(3) - Special Orthogonal Group
        let so3_set = create_test_set("SO(3)", None);
        let so3_operation = GroupOperation {
            operation_type: GroupOperationVariant::MatrixMultiplication,
            notation: GroupNotation::Infix(GroupSymbol::Times),
            identity: GroupIdentity::IdentityMatrix,
            inverse: GroupInverse::MatrixInverse,
            inverse_application: GroupInverseApplication::TwoSided,
            properties: vec![
                GroupOperationProperty::Associative,
                GroupOperationProperty::Closed,
                GroupOperationProperty::Commutative(false),
            ],
        };

        let so3_group = Group {
            base_set: so3_set,
            operation: so3_operation,
            properties: vec![
                GroupProperty::Abelian(AbelianPropertyVariant::NonAbelian),
                GroupProperty::Finite(FinitePropertyVariant::Infinite),
                GroupProperty::Simple(SimplePropertyVariant::Simple),
            ],
        };

        let so3_topological_group = TopologicalGroup {
            group: so3_group,
            properties: vec![
                TopologicalGroupProperty::Compact(CompactPropertyVariant::Compact),
                TopologicalGroupProperty::Connected(ConnectedPropertyVariant::Connected),
            ],
        };

        let so3_lie_group = LieGroup {
            topological_group: so3_topological_group,
            properties: vec![
                LieGroupProperty::Semisimple(SemisimplePropertyVariant::Semisimple),
                LieGroupProperty::Reductive(ReductivePropertyVariant::Reductive),
            ],
        };

        // Create SL(2,R) - Special Linear Group
        let sl2r_set = create_test_set("SL(2,R)", None);
        let sl2r_operation = GroupOperation {
            operation_type: GroupOperationVariant::MatrixMultiplication,
            notation: GroupNotation::Infix(GroupSymbol::Times),
            identity: GroupIdentity::IdentityMatrix,
            inverse: GroupInverse::MatrixInverse,
            inverse_application: GroupInverseApplication::TwoSided,
            properties: vec![
                GroupOperationProperty::Associative,
                GroupOperationProperty::Closed,
                GroupOperationProperty::Commutative(false),
            ],
        };

        let sl2r_group = Group {
            base_set: sl2r_set,
            operation: sl2r_operation,
            properties: vec![
                GroupProperty::Abelian(AbelianPropertyVariant::NonAbelian),
                GroupProperty::Finite(FinitePropertyVariant::Infinite),
                GroupProperty::Simple(SimplePropertyVariant::Simple),
            ],
        };

        let sl2r_topological_group = TopologicalGroup {
            group: sl2r_group,
            properties: vec![
                TopologicalGroupProperty::Connected(ConnectedPropertyVariant::Connected),
                TopologicalGroupProperty::Compact(CompactPropertyVariant::NonCompact),
            ],
        };

        let sl2r_lie_group = LieGroup {
            topological_group: sl2r_topological_group,
            properties: vec![
                LieGroupProperty::Semisimple(SemisimplePropertyVariant::Semisimple),
                LieGroupProperty::Reductive(ReductivePropertyVariant::Reductive),
            ],
        };

        // Verify SO(3) properties
        assert!(so3_lie_group.properties.iter().any(|p| matches!(
            p,
            LieGroupProperty::Semisimple(SemisimplePropertyVariant::Semisimple)
        )));

        assert!(so3_lie_group
            .topological_group
            .properties
            .iter()
            .any(|p| matches!(
                p,
                TopologicalGroupProperty::Compact(CompactPropertyVariant::Compact)
            )));

        // Verify SL(2,R) properties
        assert!(sl2r_lie_group.properties.iter().any(|p| matches!(
            p,
            LieGroupProperty::Semisimple(SemisimplePropertyVariant::Semisimple)
        )));

        assert!(sl2r_lie_group
            .topological_group
            .properties
            .iter()
            .any(|p| matches!(
                p,
                TopologicalGroupProperty::Compact(CompactPropertyVariant::NonCompact)
            )));

        assert!(sl2r_lie_group
            .topological_group
            .group
            .properties
            .iter()
            .any(|p| matches!(p, GroupProperty::Simple(SimplePropertyVariant::Simple))));
    }
}

#[cfg(test)]
mod group_action_tests {
    use crate::subjects::math::theories::VariantSet;

    use super::*;

    #[test]
    fn test_group_action_creation() {
        // Create a group
        let group_set = create_test_set("S4", None);
        let group_operation = GroupOperation {
            operation_type: GroupOperationVariant::Composition,
            notation: GroupNotation::Infix(GroupSymbol::Circle),
            identity: GroupIdentity::IdentityPermutation,
            inverse: GroupInverse::PermutationInverse,
            inverse_application: GroupInverseApplication::TwoSided,
            properties: vec![
                GroupOperationProperty::Associative,
                GroupOperationProperty::Closed,
                GroupOperationProperty::Commutative(false),
            ],
        };

        let group = Group {
            base_set: group_set,
            operation: group_operation,
            properties: vec![
                GroupProperty::Abelian(AbelianPropertyVariant::NonAbelian),
                GroupProperty::Finite(FinitePropertyVariant::Finite(24)),
            ],
        };

        // Create a set to act on
        let space = create_test_set("X", Some(4));

        // Create a group action properties
        let mut properties = VariantSet::new();
        properties.insert(GroupActionProperty::Transitive(
            TransitivityPropertyVariant::Transitive,
        ));
        properties.insert(GroupActionProperty::Faithful(
            FaithfulnessPropertyVariant::Faithful,
        ));

        // Use the factory method instead of struct initialization
        let action = GroupAction::set_action(group, space, properties.clone());

        // Verify group action properties
        assert!(
            properties.contains_variant(&GroupActionProperty::Transitive(
                TransitivityPropertyVariant::Transitive
            ))
        );
        assert!(properties.contains_variant(&GroupActionProperty::Faithful(
            FaithfulnessPropertyVariant::Faithful
        )));

        // Verify the acting group
        assert!(action
            .get_group()
            .base_set
            .get_properties()
            .inner
            .is_empty());
    }

    #[test]
    fn test_various_group_actions() {
        // Create Z/2Z group
        let z2_set = create_test_set("Z/2Z", Some(2));
        let z2_operation = GroupOperation {
            operation_type: GroupOperationVariant::Addition,
            notation: GroupNotation::Infix(GroupSymbol::Plus),
            identity: GroupIdentity::Zero,
            inverse: GroupInverse::AdditiveInverse,
            inverse_application: GroupInverseApplication::TwoSided,
            properties: vec![
                GroupOperationProperty::Associative,
                GroupOperationProperty::Closed,
                GroupOperationProperty::Commutative(true),
            ],
        };

        let z2_group = Group {
            base_set: z2_set,
            operation: z2_operation,
            properties: vec![
                GroupProperty::Abelian(AbelianPropertyVariant::Abelian),
                GroupProperty::Finite(FinitePropertyVariant::Finite(2)),
            ],
        };

        // Create a space to act on
        let space = create_test_set("R², {0}", None);

        // Create Z/2Z action (reflection)
        let mut reflection_properties = VariantSet::new();
        reflection_properties.insert(GroupActionProperty::Free(FreenessPropertyVariant::NonFree));
        reflection_properties.insert(GroupActionProperty::Faithful(
            FaithfulnessPropertyVariant::Faithful,
        ));

        // Use the factory method instead of struct initialization
        let reflection_action =
            GroupAction::set_action(z2_group.clone(), space.clone(), reflection_properties);

        // Create Z/2Z action (rotation)
        let mut rotation_properties = VariantSet::new();
        rotation_properties.insert(GroupActionProperty::Free(FreenessPropertyVariant::Free));
        rotation_properties.insert(GroupActionProperty::Faithful(
            FaithfulnessPropertyVariant::Faithful,
        ));

        // Use the factory method instead of struct initialization
        let rotation_action = GroupAction::set_action(z2_group, space, rotation_properties);

        // Verify reflection action properties
        assert!(reflection_action
            .get_properties()
            .contains_variant(&GroupActionProperty::Free(FreenessPropertyVariant::NonFree)));
        assert!(reflection_action.get_properties().contains_variant(
            &GroupActionProperty::Faithful(FaithfulnessPropertyVariant::Faithful)
        ));

        // Verify rotation action properties
        assert!(rotation_action
            .get_properties()
            .contains_variant(&GroupActionProperty::Free(FreenessPropertyVariant::Free)));
        assert!(rotation_action
            .get_properties()
            .contains_variant(&GroupActionProperty::Faithful(
                FaithfulnessPropertyVariant::Faithful
            )));
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;

    #[test]
    fn test_abelian_type() {
        // Create AbelianType variants
        let abelian = AbelianPropertyVariant::Abelian;
        let non_abelian = AbelianPropertyVariant::NonAbelian;

        // Test variant matching
        assert!(matches!(abelian, AbelianPropertyVariant::Abelian));
        assert!(matches!(non_abelian, AbelianPropertyVariant::NonAbelian));
        assert!(!matches!(abelian, AbelianPropertyVariant::NonAbelian));

        // Test cloning
        let abelian_clone = abelian.clone();
        assert!(matches!(abelian_clone, AbelianPropertyVariant::Abelian));
    }

    #[test]
    fn test_finite_type() {
        // Create FiniteType variants
        let finite = FinitePropertyVariant::Finite(10);
        let infinite = FinitePropertyVariant::Infinite;
        let locally_finite = FinitePropertyVariant::LocallyFinite;

        // Test variant matching
        assert!(matches!(finite, FinitePropertyVariant::Finite(10)));
        assert!(!matches!(finite, FinitePropertyVariant::Finite(11)));
        assert!(matches!(infinite, FinitePropertyVariant::Infinite));
        assert!(matches!(
            locally_finite,
            FinitePropertyVariant::LocallyFinite
        ));

        // Test cloning
        let finite_clone = finite.clone();
        assert!(matches!(finite_clone, FinitePropertyVariant::Finite(10)));
    }

    #[test]
    fn test_simple_type() {
        // Create SimpleType variants
        let simple = SimplePropertyVariant::Simple;
        let non_simple = SimplePropertyVariant::NonSimple;
        let quasi_simple = SimplePropertyVariant::QuasiSimple;

        // Test variant matching
        assert!(matches!(simple, SimplePropertyVariant::Simple));
        assert!(matches!(non_simple, SimplePropertyVariant::NonSimple));
        assert!(matches!(quasi_simple, SimplePropertyVariant::QuasiSimple));

        // Test cloning
        let simple_clone = simple.clone();
        assert!(matches!(simple_clone, SimplePropertyVariant::Simple));
    }

    #[test]
    fn test_solvable_type() {
        // Create SolvableType variants
        let solvable = SolvablePropertyVariant::Solvable;
        let non_solvable = SolvablePropertyVariant::NonSolvable;
        let polysolvable = SolvablePropertyVariant::Polysolvable;

        // Test variant matching
        assert!(matches!(solvable, SolvablePropertyVariant::Solvable));
        assert!(matches!(non_solvable, SolvablePropertyVariant::NonSolvable));
        assert!(matches!(
            polysolvable,
            SolvablePropertyVariant::Polysolvable
        ));

        // Test cloning
        let solvable_clone = solvable.clone();
        assert!(matches!(solvable_clone, SolvablePropertyVariant::Solvable));
    }

    #[test]
    fn test_nilpotent_type() {
        // Create NilpotentType variants
        let nilpotent = NilpotentPropertyVariant::Nilpotent(2);
        let non_nilpotent = NilpotentPropertyVariant::NonNilpotent;

        // Test variant matching
        assert!(matches!(nilpotent, NilpotentPropertyVariant::Nilpotent(2)));
        assert!(!matches!(nilpotent, NilpotentPropertyVariant::Nilpotent(3)));
        assert!(matches!(
            non_nilpotent,
            NilpotentPropertyVariant::NonNilpotent
        ));

        // Test cloning
        let nilpotent_clone = nilpotent.clone();
        assert!(matches!(
            nilpotent_clone,
            NilpotentPropertyVariant::Nilpotent(2)
        ));
    }
}
