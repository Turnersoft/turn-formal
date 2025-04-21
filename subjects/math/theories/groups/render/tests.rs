#[cfg(test)]
mod tests {
    use crate::subjects::math::theories::groups::definitions::{
        AbelianPropertyVariant, FinitePropertyVariant, Group, GroupIdentity, GroupInverse,
        GroupInverseApplication, GroupNotation, GroupOperation, GroupOperationProperty,
        GroupOperationVariant, GroupProperty, GroupSymbol,
    };
    use crate::subjects::math::theories::groups::helpers::cyclic_group;
    use crate::subjects::math::theories::zfc::Set;
    use crate::turn_render::{MathNodeContent, ToTurnMath};

    #[test]
    fn test_group_to_turn_math() {
        // Create a simple cyclic group Z_5
        let group = cyclic_group(5);

        // Convert to MathNode
        let math_node = group.to_turn_math("test_id".to_string());

        // Extract the content
        if let MathNodeContent::Text(text) = *math_node.content {
            // Verify the text contains the expected representation
            assert!(text.contains("Z_5"), "Group text should contain Z_5");
            assert!(text.contains("+"), "Cyclic group operation should be +");

            // Verify it includes identity element
            assert!(
                text.contains("Identity element: 0"),
                "Should include identity element"
            );

            // Verify it includes inverse operation
            assert!(
                text.contains("Inverse operation: -x"),
                "Should include inverse operation"
            );

            // Verify classification
            assert!(
                text.contains("cyclic group"),
                "Should be classified as cyclic group"
            );

            // Print representation regardless of test mode
            eprintln!(
                "\n=== Z_5 GROUP REPRESENTATION ===\n{}\n===========================\n",
                text
            );
        } else {
            panic!("Expected Text content, got something else");
        }

        // Create a custom group with multiplication and more properties
        let mut custom_group = Group::default();
        custom_group.operation.operation_type = GroupOperationVariant::Multiplication;

        // Add some properties to test
        custom_group
            .properties
            .push(GroupProperty::Abelian(AbelianPropertyVariant::Abelian));
        custom_group
            .properties
            .push(GroupProperty::Finite(FinitePropertyVariant::Finite(6)));

        // Convert to MathNode
        let math_node = custom_group.to_turn_math("test_id_2".to_string());

        // Extract the content
        if let MathNodeContent::Text(text) = *math_node.content {
            // Verify the text contains the expected representation
            assert!(
                text.contains("·"),
                "Multiplicative group should use · symbol"
            );

            // Verify it includes properties
            assert!(text.contains("Abelian"), "Should include abelian property");
            assert!(
                text.contains("Finite (order 6)"),
                "Should include finite property with order"
            );

            // Verify it includes identity element
            assert!(
                text.contains("Identity element: 1"),
                "Should include identity element"
            );

            // Print representation regardless of test mode
            eprintln!(
                "\n=== CUSTOM GROUP REPRESENTATION ===\n{}\n===========================\n",
                text
            );
        } else {
            panic!("Expected Text content, got something else");
        }
    }
}
