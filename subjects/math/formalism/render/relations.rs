use super::super::relations::MathRelation;
use crate::turn_render::{
    MathNode, MathNodeContent, RefinedMulOrDivOperation, RelationOperatorNode, ToTurnMath,
    UnaryRelationOperatorNode,
};
use std::string::String;

// Add structured conversion trait
use super::expressions::ToStructuredFormat;
use crate::turn_render::section_node::{
    InequalityType, StructuredExpression, StructuredGroupRelation, StructuredStatement,
    structured_group_relation, structured_placeholder, structured_todo,
};

impl ToTurnMath for MathRelation {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            MathRelation::Equal { left, right, .. } => {
                let lhs = left.to_turn_math(format!("{}_left", master_id));
                let rhs = right.to_turn_math(format!("{}_right", master_id));

                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Relationship {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        operator: RelationOperatorNode::Equal,
                    }),
                }
            }
            MathRelation::And(relations) => {
                if relations.is_empty() {
                    // Handle empty AND (logical true) - use proper mathematical constant
                    return MathNode {
                        id: master_id,
                        content: Box::new(MathNodeContent::Identifier {
                            body: "⊤".to_string(), // Top symbol for logical true
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: false,
                        }),
                    };
                }

                if relations.len() == 1 {
                    // Single relation, just return it directly
                    return relations[0].to_turn_math(master_id);
                }

                // Create a proper conjunction using mathematical AND symbol
                let mut terms = Vec::new();
                for (i, relation) in relations.iter().enumerate() {
                    let rel_node = relation.to_turn_math(format!("{}_and_{}", master_id, i));

                    if i == 0 {
                        terms.push((RefinedMulOrDivOperation::None, rel_node));
                    } else {
                        // Add AND symbol between relations
                        let and_symbol = MathNode {
                            id: format!("{}_and_symbol_{}", master_id, i),
                            content: Box::new(MathNodeContent::Identifier {
                                body: "∧".to_string(), // Logical AND symbol
                                pre_script: None,
                                mid_script: None,
                                post_script: None,
                                primes: 0,
                                is_function: false,
                            }),
                        };
                        terms.push((RefinedMulOrDivOperation::None, and_symbol));
                        terms.push((RefinedMulOrDivOperation::None, rel_node));
                    }
                }

                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Multiplications { terms }),
                }
            }
            MathRelation::Or(relations) => {
                if relations.is_empty() {
                    // Handle empty OR (logical false) - use proper mathematical constant
                    return MathNode {
                        id: master_id,
                        content: Box::new(MathNodeContent::Identifier {
                            body: "⊥".to_string(), // Bottom symbol for logical false
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: false,
                        }),
                    };
                }

                if relations.len() == 1 {
                    // Single relation, just return it directly
                    return relations[0].to_turn_math(master_id);
                }

                // Create a proper disjunction using mathematical OR symbol
                let mut terms = Vec::new();
                for (i, relation) in relations.iter().enumerate() {
                    let rel_node = relation.to_turn_math(format!("{}_or_{}", master_id, i));

                    if i == 0 {
                        terms.push((RefinedMulOrDivOperation::None, rel_node));
                    } else {
                        // Add OR symbol between relations
                        let or_symbol = MathNode {
                            id: format!("{}_or_symbol_{}", master_id, i),
                            content: Box::new(MathNodeContent::Identifier {
                                body: "∨".to_string(), // Logical OR symbol
                                pre_script: None,
                                mid_script: None,
                                post_script: None,
                                primes: 0,
                                is_function: false,
                            }),
                        };
                        terms.push((RefinedMulOrDivOperation::None, or_symbol));
                        terms.push((RefinedMulOrDivOperation::None, rel_node));
                    }
                }

                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Multiplications { terms }),
                }
            }
            MathRelation::Not(relation) => {
                let inner = relation.to_turn_math(format!("{}_inner", master_id));

                // Use proper prefix notation with negation symbol
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::UnaryPrefixOperation {
                        parameter: Box::new(inner),
                        operator: "¬".to_string(), // Logical NOT symbol
                    }),
                }
            }
            MathRelation::Implies(premise, conclusion) => {
                let lhs = premise.to_turn_math(format!("{}_premise", master_id));
                let rhs = conclusion.to_turn_math(format!("{}_conclusion", master_id));

                // Use proper relationship structure with implies operator
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Relationship {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        operator: RelationOperatorNode::Implies,
                    }),
                }
            }
            MathRelation::Equivalent(left, right) => {
                let lhs = left.to_turn_math(format!("{}_left", master_id));
                let rhs = right.to_turn_math(format!("{}_right", master_id));

                // Use Relationship with equivalence operator
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Relationship {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        operator: RelationOperatorNode::Equivalent,
                    }),
                }
            }
            MathRelation::NumberTheory(relation) => relation.to_turn_math(master_id),
            MathRelation::SetTheory(relation) => {
                // Create proper mathematical representation for set theory relations
                MathNode {
                    id: master_id.clone(),
                    content: Box::new(MathNodeContent::UnaryRelationship {
                        subject: Box::new(MathNode {
                            id: format!("{}_set_subject", master_id),
                            content: Box::new(MathNodeContent::Identifier {
                                body: "S".to_string(),
                                pre_script: None,
                                mid_script: None,
                                post_script: None,
                                primes: 0,
                                is_function: false,
                            }),
                        }),
                        predicate: UnaryRelationOperatorNode::Custom("set_property".to_string()),
                    }),
                }
            }
            MathRelation::GroupTheory(relation) => relation.to_turn_math(master_id),
            MathRelation::RingTheory(relation) => {
                // Create proper mathematical representation for ring theory relations
                MathNode {
                    id: master_id.clone(),
                    content: Box::new(MathNodeContent::UnaryRelationship {
                        subject: Box::new(MathNode {
                            id: format!("{}_ring_subject", master_id),
                            content: Box::new(MathNodeContent::Identifier {
                                body: "R".to_string(),
                                pre_script: None,
                                mid_script: None,
                                post_script: None,
                                primes: 0,
                                is_function: false,
                            }),
                        }),
                        predicate: UnaryRelationOperatorNode::Custom("ring_property".to_string()),
                    }),
                }
            }
            MathRelation::TopologyTheory(relation) => {
                // Create proper mathematical representation for topology relations
                MathNode {
                    id: master_id.clone(),
                    content: Box::new(MathNodeContent::UnaryRelationship {
                        subject: Box::new(MathNode {
                            id: format!("{}_topology_subject", master_id),
                            content: Box::new(MathNodeContent::Identifier {
                                body: "X".to_string(),
                                pre_script: None,
                                mid_script: None,
                                post_script: None,
                                primes: 0,
                                is_function: false,
                            }),
                        }),
                        predicate: UnaryRelationOperatorNode::Custom(
                            "topology_property".to_string(),
                        ),
                    }),
                }
            }
            MathRelation::CategoryTheory(relation) => {
                // Create proper mathematical representation for category theory relations
                MathNode {
                    id: master_id.clone(),
                    content: Box::new(MathNodeContent::UnaryRelationship {
                        subject: Box::new(MathNode {
                            id: format!("{}_category_subject", master_id),
                            content: Box::new(MathNodeContent::Identifier {
                                body: "𝒞".to_string(), // Script C for category
                                pre_script: None,
                                mid_script: None,
                                post_script: None,
                                primes: 0,
                                is_function: false,
                            }),
                        }),
                        predicate: UnaryRelationOperatorNode::Custom(
                            "category_property".to_string(),
                        ),
                    }),
                }
            }
            MathRelation::Todo { name, expressions } => {
                // Render TODO relations as structured math statements
                if expressions.is_empty() {
                    // Simple TODO without expressions
                    MathNode {
                        id: master_id,
                        content: Box::new(MathNodeContent::Text(format!("Assumption: {}", name))),
                    }
                } else if expressions.len() == 1 {
                    // Single expression TODO - render the expression with a label
                    let expr_node = expressions[0].to_turn_math(format!("{}_expr", master_id));
                    MathNode {
                        id: master_id.clone(),
                        content: Box::new(MathNodeContent::EmbeddedSentence {
                            subject: Box::new(MathNode {
                                id: format!("{}_label", master_id),
                                content: Box::new(MathNodeContent::Text(format!(
                                    "Assume: {}",
                                    name
                                ))),
                            }),
                            verb: "states".to_string(),
                            object: Box::new(expr_node),
                        }),
                    }
                } else {
                    // Multiple expressions - create a conjunction
                    let mut terms = Vec::new();
                    for (i, expr) in expressions.iter().enumerate() {
                        let expr_node = expr.to_turn_math(format!("{}_expr_{}", master_id, i));
                        if i > 0 {
                            // Add AND symbol between expressions
                            let and_symbol = MathNode {
                                id: format!("{}_and_{}", master_id, i),
                                content: Box::new(MathNodeContent::Identifier {
                                    body: "∧".to_string(),
                                    pre_script: None,
                                    mid_script: None,
                                    post_script: None,
                                    primes: 0,
                                    is_function: false,
                                }),
                            };
                            terms.push((RefinedMulOrDivOperation::None, and_symbol));
                        }
                        terms.push((RefinedMulOrDivOperation::None, expr_node));
                    }

                    MathNode {
                        id: master_id.clone(),
                        content: Box::new(MathNodeContent::EmbeddedSentence {
                            subject: Box::new(MathNode {
                                id: format!("{}_label", master_id),
                                content: Box::new(MathNodeContent::Text(format!(
                                    "Assume: {}",
                                    name
                                ))),
                            }),
                            verb: "states".to_string(),
                            object: Box::new(MathNode {
                                id: format!("{}_conjunction", master_id),
                                content: Box::new(MathNodeContent::Multiplications { terms }),
                            }),
                        }),
                    }
                }
            }
        }
    }
}

/// Convert MathRelation to structured statements  
impl ToStructuredFormat for MathRelation {
    type Output = StructuredStatement;

    fn to_structured(&self) -> Self::Output {
        match self {
            MathRelation::Equal { left, right, .. } => StructuredStatement::Equality {
                left: left.to_structured(),
                right: right.to_structured(),
                justification: None,
            },
            MathRelation::And(relations) => StructuredStatement::Conjunction {
                statements: relations.iter().map(|r| r.to_structured()).collect(),
            },
            MathRelation::Or(relations) => StructuredStatement::Disjunction {
                statements: relations.iter().map(|r| r.to_structured()).collect(),
            },
            MathRelation::Not(relation) => StructuredStatement::Negation {
                statement: Box::new(relation.to_structured()),
            },
            MathRelation::Implies(premise, conclusion) => StructuredStatement::Implication {
                premise: Box::new(premise.to_structured()),
                conclusion: Box::new(conclusion.to_structured()),
            },
            MathRelation::Equivalent(left, right) => StructuredStatement::Equivalence {
                left: Box::new(left.to_structured()),
                right: Box::new(right.to_structured()),
            },
            MathRelation::GroupTheory(group_rel) => {
                // Convert group theory relations to structured format
                StructuredStatement::PropertyAssertion {
                    object: StructuredExpression::Placeholder {
                        description: "Group theory object".to_string(),
                    },
                    property: "Group theory property".to_string(),
                    property_args: vec![],
                }
            }
            MathRelation::NumberTheory(num_rel) => StructuredStatement::PropertyAssertion {
                object: StructuredExpression::Placeholder {
                    description: "Number theory object".to_string(),
                },
                property: "Number theory property".to_string(),
                property_args: vec![],
            },
            MathRelation::SetTheory(set_rel) => StructuredStatement::PropertyAssertion {
                object: StructuredExpression::Placeholder {
                    description: "Set theory object".to_string(),
                },
                property: "Set theory property".to_string(),
                property_args: vec![],
            },
            MathRelation::RingTheory(ring_rel) => StructuredStatement::PropertyAssertion {
                object: StructuredExpression::Placeholder {
                    description: "Ring theory object".to_string(),
                },
                property: "Ring theory property".to_string(),
                property_args: vec![],
            },
            MathRelation::TopologyTheory(topo_rel) => StructuredStatement::PropertyAssertion {
                object: StructuredExpression::Placeholder {
                    description: "Topology theory object".to_string(),
                },
                property: "Topology theory property".to_string(),
                property_args: vec![],
            },
            MathRelation::CategoryTheory(cat_rel) => StructuredStatement::PropertyAssertion {
                object: StructuredExpression::Placeholder {
                    description: "Category theory object".to_string(),
                },
                property: "Category theory property".to_string(),
                property_args: vec![],
            },
            MathRelation::Todo { name, expressions } => StructuredStatement::Todo {
                description: format!("Todo relation: {}", name),
                context: expressions
                    .iter()
                    .map(|_| "expression".to_string())
                    .collect(),
            },
        }
    }
}
