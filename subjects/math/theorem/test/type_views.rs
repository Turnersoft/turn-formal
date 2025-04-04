// Module: src/formalize_v2/subjects/math/theorem/test/type_views.rs
// Tests for type view conversions

use crate::formalize_v2::subjects::math::theorem::core::MathObjectType;
use crate::formalize_v2::subjects::math::theorem::expressions::{
    MathExpression, TypeViewError, TypeViewOperator, Variable,
};
use crate::formalize_v2::subjects::math::theories::groups::definitions::Group;
use crate::formalize_v2::subjects::math::theories::rings::definitions::{Field, Ring};

/// Helper to create default Group
fn default_group() -> Group {
    Group::default()
}

/// Helper to create default Ring
fn default_ring() -> Ring {
    Ring::default()
}

/// Helper to create default Field
fn default_field() -> Field {
    Field::default()
}

#[test]
fn test_basic_type_views() {
    let num = MathExpression::Number(42.0);

    // Test viewing a number as a field element
    let field_view = num.to_view(TypeViewOperator::AsFieldElement {
        field: Some(Field::default()),
    });
    assert!(field_view.is_ok());

    if let Ok(MathObjectType::Element(boxed_type)) = field_view {
        if let MathObjectType::Field(_) = *boxed_type {
            // Success
        } else {
            panic!("Expected Field type, got something else");
        }
    } else {
        panic!("Expected Element type, got something else");
    }

    // Test viewing a number as a group element
    let group_view = num.to_view(TypeViewOperator::AsGroupElement {
        group: Some(Group::default()),
    });
    assert!(group_view.is_ok());

    // Check the resulting type is correct
    match group_view {
        Ok(MathObjectType::Element(boxed_type)) => {
            match *boxed_type {
                MathObjectType::Group(_) => {
                    // Success - group element type is correct
                }
                _ => panic!("Expected Group inner type, got something else"),
            }
        }
        _ => panic!("Expected Element type, got something else"),
    }
}

#[test]
fn test_variable_type_views() {
    let var_expr = MathExpression::Var(Variable::O(1)); // Object variable

    // Test viewing object variable as a group
    let group_view = var_expr.to_view(TypeViewOperator::AsGroup {});
    assert!(group_view.is_ok());

    // Check the resulting type
    match group_view {
        Ok(MathObjectType::Group(_)) => {
            // Success
        }
        _ => panic!("Expected Group type, got something else"),
    }
}

#[test]
fn test_custom_type_views() {
    let var_expr = MathExpression::Var(Variable::O(1)); // Object variable

    // Test viewing with custom type
    let custom_view = var_expr.to_view(TypeViewOperator::AsArbitraryType {
        name: Some("VectorSpace".to_string()),
        predicate: None,
    });

    assert!(custom_view.is_ok());
    match custom_view {
        Ok(MathObjectType::Custom(name)) => {
            assert_eq!(name, "VectorSpace");
        }
        _ => panic!("Expected Custom type, got something else"),
    }
}

#[test]
fn test_predicate_type_views() {
    let var_expr = MathExpression::Var(Variable::O(1)); // Object variable

    // Test predicate that always returns false
    let failing_predicate = |_: &MathExpression| -> bool { false };
    let failing_view = var_expr.to_view(TypeViewOperator::AsGeneric {
        name: Some("FailingTest".to_string()),
        predicate: Some(Box::new(failing_predicate)),
    });

    assert!(failing_view.is_err());

    // Test predicate that always returns true
    let passing_predicate = |_: &MathExpression| -> bool { true };
    let passing_view = var_expr.to_view(TypeViewOperator::AsGeneric {
        name: Some("PassingTest".to_string()),
        predicate: Some(Box::new(passing_predicate)),
    });

    assert!(passing_view.is_ok());
    match passing_view {
        Ok(MathObjectType::Custom(name)) => {
            assert_eq!(name, "PassingTest");
        }
        _ => panic!("Expected Custom type, got something else"),
    }
}

#[test]
fn test_type_compatibility() {
    let o_var = MathExpression::Var(Variable::O(0)); // Object variable

    // Check that we can view the same expression in multiple ways
    let as_group = o_var.to_view(TypeViewOperator::AsGroup {});
    let as_ring = o_var.to_view(TypeViewOperator::AsRing {});

    assert!(as_group.is_ok());
    assert!(as_ring.is_ok());

    // Ensure the types match what we expect
    match (as_group, as_ring) {
        (Ok(MathObjectType::Group(_)), Ok(MathObjectType::Ring(_))) => {
            // Success - types are as expected
        }
        _ => panic!("Expected Group and Ring types"),
    }
}
