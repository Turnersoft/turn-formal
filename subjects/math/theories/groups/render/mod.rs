use crate::turn_render::{
    MathNode, MathNodeContent, MulSymbol, RefinedMulOrDivOperation, RelationOperatorNode,
    ToTurnMath, UnaryRelationOperatorNode,
};

use super::GroupRelation;
use super::definitions::{
    AbelianPropertyVariant, FinitePropertyVariant, Group, GroupIdentity, GroupInverse,
    GroupOperationProperty, GroupOperationVariant, GroupProperty, NilpotentPropertyVariant,
    SimplePropertyVariant, SolvablePropertyVariant,
};

impl ToTurnMath for Group {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        // 1. Create a simplified representation of the group

        // Get the base set representation
        let set_name = match &self.base_set {
            // For parametric sets like Z_n, use their description
            crate::subjects::math::theories::zfc::Set::Parametric { description, .. } => {
                description.clone()
            }
            // For other sets, use a generic representation
            _ => format!("{:?}", self.base_set),
        };

        // Get the operation symbol
        let op_symbol = match self.operation.operation_type {
            GroupOperationVariant::Addition => "+",
            GroupOperationVariant::Multiplication => "·",
            GroupOperationVariant::MatrixMultiplication => "×",
            GroupOperationVariant::Composition => "∘",
            GroupOperationVariant::DirectProduct => "×",
            GroupOperationVariant::SemidirectProduct => "⋊",
            GroupOperationVariant::FreeProduct => "*",
        };

        // 2. Format key properties into a structured description

        // Create a basic description
        let name = if set_name.contains("Z_") || set_name.contains("ℤ_") {
            format!("Cyclic group {}", set_name)
        } else if set_name.contains("S_") || set_name.contains("Sym") {
            format!("Symmetric group {}", set_name)
        } else if set_name.contains("GL") || set_name.contains("SL") {
            format!("Matrix group {}", set_name)
        } else {
            format!("Group {}", set_name)
        };

        // Build a structured description with key information first
        let mut description = format!("{} with operation {}", name, op_symbol);

        // Add identity element info
        let identity_str = match self.operation.identity {
            GroupIdentity::One => "1",
            GroupIdentity::Zero => "0",
            GroupIdentity::IdentityMatrix => "I",
            GroupIdentity::IdentityPermutation => "id",
            GroupIdentity::IdentityFunction => "idₑ",
        };

        // Add inverse operation info
        let inverse_str = match self.operation.inverse {
            GroupInverse::MultiplicativeInverse => "x⁻¹",
            GroupInverse::AdditiveInverse => "-x",
            GroupInverse::MatrixInverse => "A⁻¹",
            GroupInverse::PermutationInverse => "σ⁻¹",
            GroupInverse::FunctionInverse => "f⁻¹",
        };

        // 3. Format critical properties in a human-readable way

        // Extract key properties for the short description
        let mut props = Vec::<String>::new();

        // Check if abelian
        let is_abelian = self
            .properties
            .iter()
            .any(|p| matches!(p, GroupProperty::Abelian(AbelianPropertyVariant::Abelian)));

        // Check if finite
        let is_finite = self.properties.iter().any(|p| {
            matches!(p, GroupProperty::Finite(FinitePropertyVariant::Finite(_)))
                || matches!(p, GroupProperty::FiniteGroup(true))
        });

        // Get the order if it's finite
        let order = self.properties.iter().find_map(|p| {
            if let GroupProperty::Finite(FinitePropertyVariant::Finite(n)) = p {
                Some(*n)
            } else {
                None
            }
        });

        // Add key properties to the list
        if is_abelian {
            props.push("abelian".to_string());
        }

        if is_finite {
            if let Some(n) = order {
                props.push(format!("finite (order {})", n));
            } else {
                props.push("finite".to_string());
            }
        } else {
            // Check if infinite is explicitly specified
            let is_infinite = self.properties.iter().any(|p| {
                matches!(p, GroupProperty::Finite(FinitePropertyVariant::Infinite))
                    || matches!(p, GroupProperty::FiniteGroup(false))
            });

            if is_infinite {
                props.push("infinite".to_string());
            }
        }

        // Check for simple property
        let is_simple = self
            .properties
            .iter()
            .any(|p| matches!(p, GroupProperty::Simple(SimplePropertyVariant::Simple)));

        if is_simple {
            props.push("simple".to_string());
        }

        // Complete the description with properties
        if !props.is_empty() {
            description.push_str(&format!(" ({})", props.join(", ")));
        }

        // Add identity and inverse information
        description.push_str(&format!("\n\nIdentity element: {}", identity_str));
        description.push_str(&format!("\nInverse operation: {}", inverse_str));

        // Add operation properties
        let mut op_props = Vec::<String>::new();

        if self
            .operation
            .properties
            .iter()
            .any(|p| matches!(p, GroupOperationProperty::Associative))
        {
            op_props.push("associative".to_string());
        }

        if self
            .operation
            .properties
            .iter()
            .any(|p| matches!(p, GroupOperationProperty::Commutative(true)))
        {
            op_props.push("commutative".to_string());
        }

        if !op_props.is_empty() {
            description.push_str(&format!("\nOperation is {}", op_props.join(", ")));
        }

        // 4. Create a MathNode with the comprehensive description
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::Text(description)),
        }
    }
}

impl ToTurnMath for GroupRelation {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            GroupRelation::IsSubgroupOf {
                entity,
                subgroup,
                group,
            } => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Relationship {
                    lhs: Box::new(subgroup.to_turn_math(format!("{}:subgroup", master_id))),
                    rhs: Box::new(group.to_turn_math(format!("{}:group", master_id))),
                    operator: RelationOperatorNode::IsSubgroupOf,
                }),
            },

            GroupRelation::IsNormalSubgroupOf {
                entity,
                subgroup,
                group,
            } => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Relationship {
                    lhs: Box::new(subgroup.to_turn_math(format!("{}:subgroup", master_id))),
                    rhs: Box::new(group.to_turn_math(format!("{}:group", master_id))),
                    operator: RelationOperatorNode::IsNormalSubgroupOf,
                }),
            },

            GroupRelation::IsIsomorphicTo {
                entity,
                first,
                second,
            } => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Relationship {
                    lhs: Box::new(first.to_turn_math(format!("{}:first", master_id))),
                    rhs: Box::new(second.to_turn_math(format!("{}:second", master_id))),
                    operator: RelationOperatorNode::IsIsomorphicTo,
                }),
            },

            GroupRelation::IsQuotientOf {
                entity,
                quotient,
                group,
                normal_subgroup,
            } => {
                // For complex relations with multiple components, we may want to
                // create a special representation that shows the relationship
                let quotient_text = format!("{:?}/{:?}", group, normal_subgroup);

                MathNode {
                    id: master_id.clone(),
                    content: Box::new(MathNodeContent::Relationship {
                        lhs: Box::new(quotient.to_turn_math(format!("{}:quotient", master_id))),
                        rhs: Box::new(MathNode {
                            id: format!("{}:quotient_expr", master_id),
                            content: Box::new(MathNodeContent::Text(quotient_text)),
                        }),
                        operator: RelationOperatorNode::IsEqual,
                    }),
                }
            }

            GroupRelation::IsInCenterOf {
                entity,
                element,
                group,
            } => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Relationship {
                    lhs: Box::new(element.to_turn_math(format!("{}:element", master_id))),
                    rhs: Box::new(group.to_turn_math(format!("{}:group", master_id))),
                    operator: RelationOperatorNode::IsInCenterOf,
                }),
            },

            GroupRelation::AreConjugateIn {
                entity,
                element1,
                element2,
                group,
            } => {
                // For relations with more than two parts, we need to be creative
                // g ~ h in G
                let left_expr = element1.to_turn_math(format!("{}:element1", master_id));
                let right_expr = element2.to_turn_math(format!("{}:element2", master_id));
                let group_expr = group.to_turn_math(format!("{}:group", master_id));

                // Create a relationship node for the conjugacy
                let base_relation = MathNode {
                    id: format!("{}:conj_base", master_id),
                    content: Box::new(MathNodeContent::Relationship {
                        lhs: Box::new(left_expr),
                        rhs: Box::new(right_expr),
                        operator: RelationOperatorNode::AreConjugateIn,
                    }),
                };

                // Add the group information
                let group_info = MathNode {
                    id: format!("{}:in_group", master_id),
                    content: Box::new(MathNodeContent::Text(format!(" in {:?}", group))),
                };

                // Combine them
                MathNode {
                    id: master_id.clone(),
                    content: Box::new(MathNodeContent::Multiplications {
                        terms: vec![
                            (
                                RefinedMulOrDivOperation::Multiplication(MulSymbol::None),
                                base_relation,
                            ),
                            (
                                RefinedMulOrDivOperation::Multiplication(MulSymbol::None),
                                group_info,
                            ),
                        ],
                    }),
                }
            }

            GroupRelation::HasOrderInGroup {
                entity,
                element,
                group,
                order,
            } => {
                // This is a unary relation with context - "element has order n in group"
                // We'll represent this as |g| = n in G
                let element_expr = element.to_turn_math(format!("{}:element", master_id));

                // Create the order notation
                let order_expr = MathNode {
                    id: format!("{}:order_expr", master_id),
                    content: Box::new(MathNodeContent::Text(format!(
                        "|{:?}| = {}",
                        element, order
                    ))),
                };

                // Create the full expression with group context
                let group_info = MathNode {
                    id: format!("{}:in_group", master_id),
                    content: Box::new(MathNodeContent::Text(format!(" in {:?}", group))),
                };

                // Combine them
                MathNode {
                    id: master_id.clone(),
                    content: Box::new(MathNodeContent::Multiplications {
                        terms: vec![
                            (
                                RefinedMulOrDivOperation::Multiplication(MulSymbol::None),
                                order_expr,
                            ),
                            (
                                RefinedMulOrDivOperation::Multiplication(MulSymbol::None),
                                group_info,
                            ),
                        ],
                    }),
                }
            }

            GroupRelation::HasIndexInGroup {
                entity,
                subgroup,
                group,
                index,
            } => {
                // This is a specialized relation - [G:H] = n
                let index_text = format!("[{:?}:{:?}] = {}", group, subgroup, index).to_string();

                MathNode {
                    id: master_id.clone(),
                    content: Box::new(MathNodeContent::Text(index_text)),
                }
            }

            GroupRelation::HasOrder {
                entity,
                group,
                order,
            } => {
                // Represent as |G| = n
                MathNode {
                    id: master_id.clone(),
                    content: Box::new(MathNodeContent::Text(format!("|{:?}| = {}", group, order))),
                }
            }

            GroupRelation::IsCyclicWithGenerator {
                entity,
                group,
                generator,
            } => {
                // "G = <g>" representation
                let base_relation = MathNode {
                    id: format!("{}:cyclic_base", master_id),
                    content: Box::new(MathNodeContent::Relationship {
                        lhs: Box::new(group.to_turn_math(format!("{}:group", master_id))),
                        rhs: Box::new(MathNode {
                            id: format!("{}:generator_expr", master_id),
                            content: Box::new(MathNodeContent::Text(format!("<{:?}>", generator))),
                        }),
                        operator: RelationOperatorNode::IsEqual,
                    }),
                };

                // Additional info that it's cyclic
                let cyclic_info = MathNode {
                    id: format!("{}:cyclic_info", master_id),
                    content: Box::new(MathNodeContent::Text(" (cyclic)".to_string())),
                };

                // Combine them
                MathNode {
                    id: master_id.clone(),
                    content: Box::new(MathNodeContent::Multiplications {
                        terms: vec![
                            (
                                RefinedMulOrDivOperation::Multiplication(MulSymbol::None),
                                base_relation,
                            ),
                            (
                                RefinedMulOrDivOperation::Multiplication(MulSymbol::None),
                                cyclic_info,
                            ),
                        ],
                    }),
                }
            }

            GroupRelation::NormalizesSubgroup {
                entity,
                element,
                subgroup,
                group,
            } => {
                // Similar to conjugacy, but with a subgroup
                let element_expr = element.to_turn_math(format!("{}:element", master_id));
                let subgroup_expr = subgroup.to_turn_math(format!("{}:subgroup", master_id));

                // Create base relation - "g normalizes H in G"
                let relation_text =
                    format!("{:?} normalizes {:?} in {:?}", element, subgroup, group).to_string();

                // Return the text representation
                MathNode {
                    id: master_id.clone(),
                    content: Box::new(MathNodeContent::Text(relation_text)),
                }
            }

            GroupRelation::CentralizesSubgroup {
                entity,
                element,
                subgroup,
                group,
            } => {
                // Similar to normalizes, but for centralizing
                let relation_text =
                    format!("{:?} centralizes {:?} in {:?}", element, subgroup, group).to_string();

                // Return the text representation
                MathNode {
                    id: master_id.clone(),
                    content: Box::new(MathNodeContent::Text(relation_text)),
                }
            }

            GroupRelation::IsCharacteristicSubgroupOf {
                entity,
                subgroup,
                group,
            } => {
                // "H char G" representation
                let relation_text = format!("{:?} char {:?}", subgroup, group).to_string();

                // Return the text representation
                MathNode {
                    id: master_id.clone(),
                    content: Box::new(MathNodeContent::Text(relation_text)),
                }
            }

            GroupRelation::OrderDivides {
                entity,
                group1,
                group2,
            } => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Relationship {
                    lhs: Box::new(MathNode {
                        id: format!("{}:order_g1", master_id),
                        content: Box::new(MathNodeContent::Text(format!("|{:?}|", group1))),
                    }),
                    rhs: Box::new(MathNode {
                        id: format!("{}:order_g2", master_id),
                        content: Box::new(MathNodeContent::Text(format!("|{:?}|", group2))),
                    }),
                    operator: RelationOperatorNode::Divides,
                }),
            },

            GroupRelation::HasUniqueInverse {
                entity,
                element,
                group,
            } => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::UnaryRelationship {
                    subject: Box::new(element.to_turn_math(format!("{}:element", master_id))),
                    predicate: UnaryRelationOperatorNode::HasUniqueInverse,
                }),
            },

            GroupRelation::SylowSubgroupProperties {
                entity,
                prime,
                group,
            } => {
                // This is a complex relation with Sylow subgroup properties
                // We'll represent it as text
                let relation_text =
                    format!("Sylow {:?}-subgroup properties of {:?}", prime, group).to_string();

                // Return the text representation
                MathNode {
                    id: master_id.clone(),
                    content: Box::new(MathNodeContent::Text(relation_text)),
                }
            }

            GroupRelation::IsInverseOf {
                entity,
                element,
                inverse,
                group,
            } => {
                // g^(-1) = h in G
                let base_relation = MathNode {
                    id: format!("{}:inverse_base", master_id),
                    content: Box::new(MathNodeContent::Relationship {
                        lhs: Box::new(MathNode {
                            id: format!("{}:element_inv", master_id),
                            content: Box::new(MathNodeContent::Text(format!(
                                "({:?})^(-1)",
                                element
                            ))),
                        }),
                        rhs: Box::new(inverse.to_turn_math(format!("{}:inverse", master_id))),
                        operator: RelationOperatorNode::IsEqual,
                    }),
                };

                // Add group context
                let group_info = MathNode {
                    id: format!("{}:in_group", master_id),
                    content: Box::new(MathNodeContent::Text(format!(" in {:?}", group))),
                };

                // Combine them
                MathNode {
                    id: master_id.clone(),
                    content: Box::new(MathNodeContent::Multiplications {
                        terms: vec![
                            (
                                RefinedMulOrDivOperation::Multiplication(MulSymbol::None),
                                base_relation,
                            ),
                            (
                                RefinedMulOrDivOperation::Multiplication(MulSymbol::None),
                                group_info,
                            ),
                        ],
                    }),
                }
            }

            GroupRelation::IsHomomorphism {
                entity,
                homomorphism,
                domain,
                codomain,
            } => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Relationship {
                    lhs: Box::new(homomorphism.to_turn_math(format!("{}:homomorphism", master_id))),
                    rhs: Box::new(MathNode {
                        id: format!("{}:domain_codomain", master_id),
                        content: Box::new(MathNodeContent::Text(format!(
                            "{:?} → {:?}",
                            domain, codomain
                        ))),
                    }),
                    operator: RelationOperatorNode::IsHomomorphicTo,
                }),
            },

            GroupRelation::IsomorphicEmbedding {
                entity,
                source,
                target,
            } => {
                // Similar to isomorphism, but as an embedding
                let relation_text =
                    format!("{:?} embeds isomorphically into {:?}", source, target).to_string();

                // Return the text representation
                MathNode {
                    id: master_id.clone(),
                    content: Box::new(MathNodeContent::Text(relation_text)),
                }
            }
        }
    }
}

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
        let mut group = cyclic_group(5);

        // Add the finite property explicitly since the helper doesn't include it
        group
            .properties
            .push(GroupProperty::Finite(FinitePropertyVariant::Finite(5)));

        // Convert to MathNode
        let math_node = group.to_turn_math("test_id".to_string());

        println!("Cyclic group Z_5 output: {:?}", math_node);

        // Extract the content
        if let MathNodeContent::Text(text) = *math_node.content {
            // Verify the text contains the expected representation
            assert!(
                text.contains("Cyclic group Z_5"),
                "Group text should identify it as a cyclic group"
            );
            assert!(text.contains("+"), "Cyclic group operation should be +");
            assert!(
                text.contains("Identity element: 0"),
                "Should show identity element"
            );
            assert!(
                text.contains("Inverse operation: -x"),
                "Should show inverse operation"
            );
            assert!(
                text.contains("Operation is associative"),
                "Should identify associative property"
            );
            assert!(
                text.contains("finite (order 5)"),
                "Should identify it as finite with order 5"
            );
        } else {
            panic!("Expected Text content, got something else");
        }

        // Create a custom group with multiplication and additional properties
        let mut custom_group = Group::default();
        custom_group.operation.operation_type = GroupOperationVariant::Multiplication;

        // Add abelian property
        custom_group
            .properties
            .push(GroupProperty::Abelian(AbelianPropertyVariant::Abelian));

        // Add finite property
        custom_group
            .properties
            .push(GroupProperty::Finite(FinitePropertyVariant::Finite(6)));

        // Make sure operation is marked as commutative
        custom_group
            .operation
            .properties
            .push(GroupOperationProperty::Commutative(true));

        // Convert to MathNode
        let math_node = custom_group.to_turn_math("test_id_2".to_string());

        println!("Custom group output: {:?}", math_node);

        // Extract the content
        if let MathNodeContent::Text(text) = *math_node.content {
            // Verify the text contains the expected representation
            assert!(
                text.contains("·"),
                "Multiplicative group should use · symbol"
            );
            assert!(text.contains("abelian"), "Should identify as abelian");

            // Create a let binding for the formatted string to fix the linter error
            let order_string = "finite (order 6)";
            assert!(text.contains(order_string), "Should show order 6");

            assert!(
                text.contains("Identity element: 1"),
                "Should show identity element"
            );
            assert!(
                text.contains("Operation is associative, commutative"),
                "Should identify operation properties"
            );
        } else {
            panic!("Expected Text content, got something else");
        }
    }
}
