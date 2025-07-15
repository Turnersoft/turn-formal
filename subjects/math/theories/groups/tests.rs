//! Tests for group theory definitions and operations.
//!
//! This file contains tests that verify the creation, manipulation, and properties
//! of all mathematical objects defined in the group theory module.

use super::super::VariantSet;
use super::definitions::*;
use crate::subjects::math::theories::groups::definitions::MetrizablePropertyVariant;
use crate::subjects::math::theories::groups::definitions::{
    LieGroupProperty, ReductivePropertyVariant, SemisimplePropertyVariant, TopologicalGroupProperty,
};
use crate::subjects::math::theories::topology::definitions::{TopologicalSpace, Topology};
use crate::subjects::math::theories::topology::{
    CompactnessPropertyVariant, ConnectednessPropertyVariant,
};
use crate::subjects::math::theories::zfc::definitions::CardinalityPropertyVariant;
use crate::subjects::math::theories::zfc::definitions::{Set, SetProperty};
use serde_json::{from_str, to_string};
use std::cmp::PartialEq;
use std::collections::HashMap;

// Helper function to create a test Set
fn create_test_set(name: &str, size: Option<u32>) -> Set {
    let mut properties = VariantSet::new();
    if let Some(size) = size {
        properties.insert(SetProperty::Cardinality(
            CardinalityPropertyVariant::Finite(size as usize),
        ));
    }
    Set::Parametric {
        parameters: HashMap::new(),
        description: name.to_string(),
        membership_condition: format!("Element of {}", name),
        properties,
    }
}

// Helper function to create a default Topology
fn default_topology() -> Topology {
    Topology {
        properties: VariantSet::new(),
    }
}

// Helper function to create a default TopologicalSpace
fn default_topological_space(set_name: &str) -> TopologicalSpace {
    TopologicalSpace {
        base_set: create_test_set(set_name, None),
        topology: default_topology(),
        properties: vec![],
    }
}

#[cfg(test)]
mod group_operation_tests {
    use crate::variant_set;

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
        };

        let addition = GroupOperation {
            operation_type: GroupOperationVariant::Addition,
            notation: GroupNotation::Infix(GroupSymbol::Plus),
            identity: GroupIdentity::Zero,
            inverse: GroupInverse::AdditiveInverse,
            inverse_application: GroupInverseApplication::TwoSided,
        };

        let composition = GroupOperation {
            operation_type: GroupOperationVariant::Composition,
            notation: GroupNotation::Infix(GroupSymbol::Circle),
            identity: GroupIdentity::IdentityFunction,
            inverse: GroupInverse::FunctionInverse,
            inverse_application: GroupInverseApplication::TwoSided,
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
        };

        let right_inverse_op = GroupOperation {
            operation_type: GroupOperationVariant::Multiplication,
            notation: GroupNotation::Infix(GroupSymbol::Times),
            identity: GroupIdentity::One,
            inverse: GroupInverse::MultiplicativeInverse,
            inverse_application: GroupInverseApplication::Right,
        };

        let two_sided_inverse_op = GroupOperation {
            operation_type: GroupOperationVariant::Multiplication,
            notation: GroupNotation::Infix(GroupSymbol::Times),
            identity: GroupIdentity::One,
            inverse: GroupInverse::MultiplicativeInverse,
            inverse_application: GroupInverseApplication::TwoSided,
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
    use crate::variant_set;

    use super::*;

    fn create_test_group() -> Group {
        let base_set = create_test_set("Z", None);
        let operation = GroupOperation {
            operation_type: GroupOperationVariant::Addition,
            notation: GroupNotation::Infix(GroupSymbol::Plus),
            identity: GroupIdentity::Zero,
            inverse: GroupInverse::AdditiveInverse,
            inverse_application: GroupInverseApplication::TwoSided,
        };

        // Use VariantSet::new() and insert
        let mut props = VariantSet::new();
        props.insert(GroupProperty::Abelian(AbelianPropertyVariant::Abelian));
        props.insert(GroupProperty::Finite(FinitePropertyVariant::Infinite));
        props.insert(GroupProperty::Simple(SimplePropertyVariant::NonSimple));

        Group::Generic(GenericGroup {
            base_set,
            operation,
            props,
        })
    }

    #[test]
    fn test_group_creation() {
        let group = create_test_group();

        // Use get_core() to access GroupBasic fields
        let core = group.get_core();

        // Replace get_properties().inner.is_empty() with appropriate check
        // For example, if Set has is_parametric or if_parametric methods we could use that
        match &core.base_set {
            Set::Parametric { properties, .. } => assert!(properties.inner.is_empty()),
            _ => {} // No assertion needed for other set types
        }

        assert!(matches!(
            core.operation.operation_type,
            GroupOperationVariant::Addition
        ));

        // Access properties through core
        let is_abelian = core
            .props
            .iter()
            .any(|p| matches!(p, GroupProperty::Abelian(AbelianPropertyVariant::Abelian)));

        let is_infinite = core
            .props
            .iter()
            .any(|p| matches!(p, GroupProperty::Finite(FinitePropertyVariant::Infinite)));

        assert!(is_abelian);
        assert!(is_infinite);
    }

    #[test]
    fn test_group_serialization() {
        let group = create_test_group();
        let core = group.get_core(); // Get core for comparison

        // Test serialization
        let serialized = to_string(&group).expect("Failed to serialize group");
        assert!(!serialized.is_empty());

        // Test deserialization
        let deserialized: Group = from_str(&serialized).expect("Failed to deserialize group");
        let des_core = deserialized.get_core(); // Get core of deserialized

        // Verify deserialized core matches original core
        // Replace get_properties().inner.is_empty() with appropriate check for properties
        match (&des_core.base_set, &core.base_set) {
            (Set::Parametric { properties: p1, .. }, Set::Parametric { properties: p2, .. }) => {
                assert!(p1.inner.is_empty() == p2.inner.is_empty())
            }
            _ => {} // No assertion needed for other set types
        }

        assert!(matches!(
            des_core.operation.operation_type,
            GroupOperationVariant::Addition
        ));
        assert!(matches!(des_core.operation.identity, GroupIdentity::Zero));

        let is_abelian = des_core
            .props
            .iter()
            .any(|p| matches!(p, GroupProperty::Abelian(AbelianPropertyVariant::Abelian)));
        assert!(is_abelian);
    }

    #[test]
    fn test_various_group_types() {
        // Construct specific Group variants, e.g., Group::Symmetric
        let mut s3_props = VariantSet::new();
        s3_props.insert(GroupProperty::Abelian(AbelianPropertyVariant::NonAbelian));
        s3_props.insert(GroupProperty::Finite(FinitePropertyVariant::Finite(6)));
        s3_props.insert(GroupProperty::Simple(SimplePropertyVariant::NonSimple));
        let s3_core = GenericGroup {
            base_set: create_test_set("S3", Some(6)),
            operation: GroupOperation {
                operation_type: GroupOperationVariant::Composition,
                notation: GroupNotation::Infix(GroupSymbol::Circle),
                identity: GroupIdentity::IdentityPermutation,
                inverse: GroupInverse::PermutationInverse,
                inverse_application: GroupInverseApplication::TwoSided,
            },
            props: s3_props,
        };
        let symmetric_group = Group::Symmetric(SymmetricGroup {
            core: s3_core,
            degree: 3,
        });

        let mut z2_props = VariantSet::new();
        z2_props.insert(GroupProperty::Abelian(AbelianPropertyVariant::Abelian));
        z2_props.insert(GroupProperty::Finite(FinitePropertyVariant::Finite(2)));
        z2_props.insert(GroupProperty::Simple(SimplePropertyVariant::Simple));
        let z2_core = GenericGroup {
            base_set: create_test_set("Z/2Z", Some(2)),
            operation: GroupOperation {
                operation_type: GroupOperationVariant::Addition,
                notation: GroupNotation::Infix(GroupSymbol::Plus),
                identity: GroupIdentity::Zero,
                inverse: GroupInverse::AdditiveInverse,
                inverse_application: GroupInverseApplication::TwoSided,
            },
            props: z2_props,
        };
        let mut mod_props = VariantSet::new();
        mod_props.insert(ModularProperty::Modulus(2));
        mod_props.insert(ModularProperty::Representatives(
            RepresentativesVariant::Standard,
        ));

        let cyclic_group = Group::ModularAdditive(ModularAdditiveGroup {
            core: z2_core,
            modulus: 2,
            modular_props: mod_props,
        });

        // Verify S3 properties (accessing through get_core)
        assert!(matches!(
            symmetric_group.get_core().operation.operation_type,
            GroupOperationVariant::Composition
        ));
        assert!(symmetric_group.get_core().props.iter().any(|p| matches!(
            p,
            GroupProperty::Abelian(AbelianPropertyVariant::NonAbelian)
        )));
        assert!(
            symmetric_group
                .get_core()
                .props
                .iter()
                .any(|p| matches!(p, GroupProperty::Finite(FinitePropertyVariant::Finite(6))))
        );

        // Verify Z/2Z properties (accessing through get_core)
        assert!(matches!(
            cyclic_group.get_core().operation.operation_type,
            GroupOperationVariant::Addition
        ));
        assert!(
            cyclic_group
                .get_core()
                .props
                .iter()
                .any(|p| matches!(p, GroupProperty::Abelian(AbelianPropertyVariant::Abelian)))
        );
        assert!(
            cyclic_group
                .get_core()
                .props
                .iter()
                .any(|p| matches!(p, GroupProperty::Finite(FinitePropertyVariant::Finite(2))))
        );
        assert!(
            cyclic_group
                .get_core()
                .props
                .iter()
                .any(|p| matches!(p, GroupProperty::Simple(SimplePropertyVariant::Simple)))
        );
    }
}

#[cfg(test)]
mod topological_group_tests {
    use super::*;
    use crate::{subjects::math::theories::topology::definitions::TopologicalSpace, variant_set};

    #[test]
    fn test_topological_group_creation() {
        // Create a base GroupBasic
        let mut group_props = VariantSet::new();
        group_props.insert(GroupProperty::Abelian(AbelianPropertyVariant::Abelian));
        group_props.insert(GroupProperty::Finite(FinitePropertyVariant::Infinite));
        let group_core = GenericGroup {
            base_set: create_test_set("R", None),
            operation: GroupOperation {
                operation_type: GroupOperationVariant::Addition,
                notation: GroupNotation::Infix(GroupSymbol::Plus),
                identity: GroupIdentity::Zero,
                inverse: GroupInverse::AdditiveInverse,
                inverse_application: GroupInverseApplication::TwoSided,
            },
            props: group_props,
        };

        // Use default_topological_space helper
        let topology = default_topological_space("R_topology");

        // Create a topological group variant
        let mut topo_props = VariantSet::new();
        topo_props.insert(TopologicalGroupProperty::Connected(
            ConnectedPropertyVariant::Connected,
        ));
        topo_props.insert(TopologicalGroupProperty::Connected(
            ConnectedPropertyVariant::LocallyConnected,
        ));
        let topological_group = Group::Topological(TopologicalGroup {
            core: group_core,
            topology,
            props: topo_props,
        });

        // Match on the Group enum to test
        if let Group::Topological(tg) = &topological_group {
            // Verify topological group properties
            assert!(tg.props.iter().any(|p| matches!(
                p,
                TopologicalGroupProperty::Connected(ConnectedPropertyVariant::Connected)
            )));
            assert!(tg.props.iter().any(|p| matches!(
                p,
                TopologicalGroupProperty::Connected(ConnectedPropertyVariant::LocallyConnected)
            )));

            // Verify underlying group properties
            assert!(
                tg.core
                    .props
                    .iter()
                    .any(|p| matches!(p, GroupProperty::Abelian(AbelianPropertyVariant::Abelian)))
            );
            assert!(
                tg.core
                    .props
                    .iter()
                    .any(|p| matches!(p, GroupProperty::Finite(FinitePropertyVariant::Infinite)))
            );
        } else {
            panic!("Expected Group::Topological variant");
        }
    }

    #[test]
    fn test_various_topological_groups() {
        // Create circle group S¹
        let mut circle_props = VariantSet::new();
        circle_props.insert(GroupProperty::Abelian(AbelianPropertyVariant::Abelian));
        circle_props.insert(GroupProperty::Finite(FinitePropertyVariant::Infinite));
        circle_props.insert(GroupProperty::Simple(SimplePropertyVariant::NonSimple));
        let circle_set = create_test_set("S¹", None);
        let circle_operation = GroupOperation {
            operation_type: GroupOperationVariant::Multiplication,
            notation: GroupNotation::Infix(GroupSymbol::Times),
            identity: GroupIdentity::One,
            inverse: GroupInverse::MultiplicativeInverse,
            inverse_application: GroupInverseApplication::TwoSided,
        };

        let circle_core = GenericGroup {
            base_set: circle_set,
            operation: circle_operation,
            props: circle_props,
        };

        // Use default_topological_space helper
        let circle_topology = default_topological_space("S1_topology");

        let mut circle_topo_props = VariantSet::new();
        circle_topo_props.insert(TopologicalGroupProperty::Compact(
            CompactPropertyVariant::Compact,
        ));
        circle_topo_props.insert(TopologicalGroupProperty::Connected(
            ConnectedPropertyVariant::Connected,
        ));

        let circle_topological_group = Group::Topological(TopologicalGroup {
            core: circle_core,
            topology: circle_topology,
            props: circle_topo_props,
        });

        // Create discrete group Z with discrete topology
        let mut integer_props = VariantSet::new();
        integer_props.insert(GroupProperty::Abelian(AbelianPropertyVariant::Abelian));
        integer_props.insert(GroupProperty::Finite(FinitePropertyVariant::Infinite));
        let integer_set = create_test_set("Z", None);
        let integer_operation = GroupOperation {
            operation_type: GroupOperationVariant::Addition,
            notation: GroupNotation::Infix(GroupSymbol::Plus),
            identity: GroupIdentity::Zero,
            inverse: GroupInverse::AdditiveInverse,
            inverse_application: GroupInverseApplication::TwoSided,
        };

        let integer_core = GenericGroup {
            base_set: integer_set,
            operation: integer_operation,
            props: integer_props,
        };

        // Use default_topological_space helper (representing discrete implicitly)
        let discrete_topology = default_topological_space("Z_topology");

        let mut discrete_topo_props = VariantSet::new();
        discrete_topo_props.insert(TopologicalGroupProperty::Connected(
            ConnectedPropertyVariant::TotallyDisconnected,
        ));
        discrete_topo_props.insert(TopologicalGroupProperty::Metrizable(
            MetrizablePropertyVariant::Metrizable,
        ));

        let discrete_topological_group = Group::Topological(TopologicalGroup {
            core: integer_core,
            topology: discrete_topology,
            props: discrete_topo_props,
        });

        // Verify circle group properties
        if let Group::Topological(tg) = &circle_topological_group {
            assert!(
                tg.props
                    .contains_variant(&TopologicalGroupProperty::Compact(
                        CompactPropertyVariant::Compact
                    ))
            );
            assert!(
                tg.props
                    .contains_variant(&TopologicalGroupProperty::Connected(
                        ConnectedPropertyVariant::Connected
                    ))
            );
        } else {
            panic!("Expected Topological Group");
        }

        // Verify discrete group properties
        if let Group::Topological(tg) = &discrete_topological_group {
            assert!(
                tg.props
                    .contains_variant(&TopologicalGroupProperty::Connected(
                        ConnectedPropertyVariant::TotallyDisconnected
                    ))
            );
            assert!(
                tg.props
                    .contains_variant(&TopologicalGroupProperty::Metrizable(
                        MetrizablePropertyVariant::Metrizable
                    ))
            );
        } else {
            panic!("Expected Topological Group");
        }
    }
}

#[cfg(test)]
mod lie_group_tests {
    use super::*;
    use crate::{subjects::math::theories::topology::definitions::TopologicalSpace, variant_set};

    #[test]
    fn test_lie_group_creation() {
        // Create a base GroupBasic for GL(n,R)
        let mut glnr_props = VariantSet::new();
        glnr_props.insert(GroupProperty::Abelian(AbelianPropertyVariant::NonAbelian));
        glnr_props.insert(GroupProperty::Finite(FinitePropertyVariant::Infinite));
        let glnr_core = GenericGroup {
            base_set: create_test_set("GL(n,R)", None),
            operation: GroupOperation {
                operation_type: GroupOperationVariant::MatrixMultiplication,
                notation: GroupNotation::Infix(GroupSymbol::Times),
                identity: GroupIdentity::IdentityMatrix,
                inverse: GroupInverse::MatrixInverse,
                inverse_application: GroupInverseApplication::TwoSided,
            },
            props: glnr_props,
        };

        // Use default_topological_space helper
        let topology = default_topological_space("GLnR_topology");

        // Create a Lie group variant
        let mut lie_props = VariantSet::new();
        lie_props.insert(LieGroupProperty::Semisimple(
            SemisimplePropertyVariant::Semisimple,
        ));
        lie_props.insert(LieGroupProperty::Reductive(
            ReductivePropertyVariant::Reductive,
        ));
        let lie_group = Group::Lie(LieGroup {
            core: glnr_core,
            topology,
            charts: vec!["chart1".to_string()],
            props: lie_props,
        });

        // Match on the Group enum to test
        if let Group::Lie(lg) = &lie_group {
            // Verify Lie group properties
            assert!(lg.props.iter().any(|p| matches!(
                p,
                LieGroupProperty::Semisimple(SemisimplePropertyVariant::Semisimple)
            )));
            assert!(lg.props.iter().any(|p| matches!(
                p,
                LieGroupProperty::Reductive(ReductivePropertyVariant::Reductive)
            )));

            // Verify underlying group properties
            assert!(lg.core.props.iter().any(|p| matches!(
                p,
                GroupProperty::Abelian(AbelianPropertyVariant::NonAbelian)
            )));
            assert!(matches!(
                lg.core.operation.operation_type,
                GroupOperationVariant::MatrixMultiplication
            ));
        } else {
            panic!("Expected Group::Lie variant");
        }
    }

    #[test]
    fn test_various_lie_groups() {
        // Create SO(3) - Special Orthogonal Group
        let mut so3_group_props = VariantSet::new();
        so3_group_props.insert(GroupProperty::Abelian(AbelianPropertyVariant::NonAbelian));
        so3_group_props.insert(GroupProperty::Finite(FinitePropertyVariant::Infinite));
        so3_group_props.insert(GroupProperty::Simple(SimplePropertyVariant::Simple));
        let so3_set = create_test_set("SO(3)", None);
        let so3_operation = GroupOperation {
            operation_type: GroupOperationVariant::MatrixMultiplication,
            notation: GroupNotation::Infix(GroupSymbol::Times),
            identity: GroupIdentity::IdentityMatrix,
            inverse: GroupInverse::MatrixInverse,
            inverse_application: GroupInverseApplication::TwoSided,
        };

        let so3_core = GenericGroup {
            base_set: so3_set,
            operation: so3_operation,
            props: so3_group_props,
        };

        // Use default_topological_space helper
        let so3_topology = default_topological_space("SO3_topology");

        let mut so3_lie_props = VariantSet::new();
        so3_lie_props.insert(LieGroupProperty::Semisimple(
            SemisimplePropertyVariant::Semisimple,
        ));
        so3_lie_props.insert(LieGroupProperty::Reductive(
            ReductivePropertyVariant::Reductive,
        ));

        let so3_lie_group = Group::Lie(LieGroup {
            core: so3_core,
            topology: so3_topology,
            charts: vec![],
            props: so3_lie_props,
        });

        // Create SL(2,R) - Special Linear Group
        let mut sl2r_group_props = VariantSet::new();
        sl2r_group_props.insert(GroupProperty::Abelian(AbelianPropertyVariant::NonAbelian));
        sl2r_group_props.insert(GroupProperty::Finite(FinitePropertyVariant::Infinite));
        sl2r_group_props.insert(GroupProperty::Simple(SimplePropertyVariant::Simple));
        let sl2r_set = create_test_set("SL(2,R)", None);
        let sl2r_operation = GroupOperation {
            operation_type: GroupOperationVariant::MatrixMultiplication,
            notation: GroupNotation::Infix(GroupSymbol::Times),
            identity: GroupIdentity::IdentityMatrix,
            inverse: GroupInverse::MatrixInverse,
            inverse_application: GroupInverseApplication::TwoSided,
        };

        let sl2r_core = GenericGroup {
            base_set: sl2r_set,
            operation: sl2r_operation,
            props: sl2r_group_props,
        };

        // Use default_topological_space helper
        let sl2r_topology = default_topological_space("SL2R_topology");

        let mut sl2r_lie_props = VariantSet::new();
        sl2r_lie_props.insert(LieGroupProperty::Semisimple(
            SemisimplePropertyVariant::Semisimple,
        ));
        sl2r_lie_props.insert(LieGroupProperty::Reductive(
            ReductivePropertyVariant::Reductive,
        ));

        let sl2r_lie_group = Group::Lie(LieGroup {
            core: sl2r_core,
            topology: sl2r_topology,
            charts: vec![],
            props: sl2r_lie_props,
        });

        // Verify SO(3) properties (match on Group::Lie)
        if let Group::Lie(lg) = &so3_lie_group {
            assert!(lg.props.contains_variant(&LieGroupProperty::Semisimple(
                SemisimplePropertyVariant::Semisimple
            )));
            assert!(
                lg.core
                    .props
                    .contains_variant(&GroupProperty::Simple(SimplePropertyVariant::Simple))
            );
        } else {
            panic!("Expected Lie Group");
        }

        // Verify SL(2,R) properties (match on Group::Lie)
        if let Group::Lie(lg) = &sl2r_lie_group {
            assert!(lg.props.contains_variant(&LieGroupProperty::Semisimple(
                SemisimplePropertyVariant::Semisimple
            )));
            assert!(
                lg.core
                    .props
                    .contains_variant(&GroupProperty::Simple(SimplePropertyVariant::Simple))
            );
        } else {
            panic!("Expected Lie Group");
        }
    }
}

#[cfg(test)]
mod group_action_tests {
    use crate::variant_set;

    use super::super::super::super::super::math::theories::VariantSet;

    use super::*;

    #[test]
    fn test_group_action_creation() {
        // Create a group
        let mut s4_group_props = VariantSet::new();
        s4_group_props.insert(GroupProperty::Abelian(AbelianPropertyVariant::NonAbelian));
        s4_group_props.insert(GroupProperty::Finite(FinitePropertyVariant::Finite(24)));
        let group_set = create_test_set("S4", None);
        let group_operation = GroupOperation {
            operation_type: GroupOperationVariant::Composition,
            notation: GroupNotation::Infix(GroupSymbol::Circle),
            identity: GroupIdentity::IdentityPermutation,
            inverse: GroupInverse::PermutationInverse,
            inverse_application: GroupInverseApplication::TwoSided,
        };

        let group = Group::Symmetric(SymmetricGroup {
            core: GenericGroup {
                base_set: group_set,
                operation: group_operation,
                props: s4_group_props,
            },
            degree: 4,
        });

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
        // Replace get_properties().inner.is_empty() with appropriate check
        match &action.get_group().get_core().base_set {
            Set::Parametric { properties, .. } => assert!(properties.inner.is_empty()),
            _ => {} // No assertion needed for other set types
        }
    }

    #[test]
    fn test_various_group_actions() {
        // Create Z/2Z group
        let mut z2_group_props = VariantSet::new();
        z2_group_props.insert(GroupProperty::Abelian(AbelianPropertyVariant::Abelian));
        z2_group_props.insert(GroupProperty::Finite(FinitePropertyVariant::Finite(2)));
        let z2_set = create_test_set("Z/2Z", Some(2));
        let z2_operation = GroupOperation {
            operation_type: GroupOperationVariant::Addition,
            notation: GroupNotation::Infix(GroupSymbol::Plus),
            identity: GroupIdentity::Zero,
            inverse: GroupInverse::AdditiveInverse,
            inverse_application: GroupInverseApplication::TwoSided,
        };

        let z2_group = Group::ModularAdditive(ModularAdditiveGroup {
            core: GenericGroup {
                base_set: z2_set,
                operation: z2_operation,
                props: z2_group_props,
            },
            modulus: 2,
            modular_props: variant_set![ModularProperty::Modulus(2)],
        });

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
        assert!(
            reflection_action
                .get_properties()
                .contains_variant(&GroupActionProperty::Free(FreenessPropertyVariant::NonFree))
        );
        assert!(reflection_action.get_properties().contains_variant(
            &GroupActionProperty::Faithful(FaithfulnessPropertyVariant::Faithful)
        ));

        // Verify rotation action properties
        assert!(
            rotation_action
                .get_properties()
                .contains_variant(&GroupActionProperty::Free(FreenessPropertyVariant::Free))
        );
        assert!(
            rotation_action
                .get_properties()
                .contains_variant(&GroupActionProperty::Faithful(
                    FaithfulnessPropertyVariant::Faithful
                ))
        );
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
