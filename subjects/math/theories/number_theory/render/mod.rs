use std::sync::Arc;

use crate::turn_render::*;

use super::NumberTheoryRelation;

impl ToTurnMath for NumberTheoryRelation {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            NumberTheoryRelation::LessThan { left, right } => MathNode {
                id: master_id.clone(),
                content: Arc::new(MathNodeContent::Relationship {
                    lhs: Arc::new(left.to_turn_math(format!("{}:lhs", master_id))),
                    rhs: Arc::new(right.to_turn_math(format!("{}:rhs", master_id))),
                    operator: RelationOperatorNode::Less,
                }),
            },
            NumberTheoryRelation::LessThanOrEqual { left, right } => MathNode {
                id: master_id.clone(),
                content: Arc::new(MathNodeContent::Relationship {
                    lhs: Arc::new(left.to_turn_math(format!("{}:lhs", master_id))),
                    rhs: Arc::new(right.to_turn_math(format!("{}:rhs", master_id))),
                    operator: RelationOperatorNode::LessEqual,
                }),
            },
            NumberTheoryRelation::GreaterThan { left, right } => MathNode {
                id: master_id.clone(),
                content: Arc::new(MathNodeContent::Relationship {
                    lhs: Arc::new(left.to_turn_math(format!("{}:lhs", master_id))),
                    rhs: Arc::new(right.to_turn_math(format!("{}:rhs", master_id))),
                    operator: RelationOperatorNode::Greater,
                }),
            },
            NumberTheoryRelation::GreaterThanOrEqual { left, right } => MathNode {
                id: master_id.clone(),
                content: Arc::new(MathNodeContent::Relationship {
                    lhs: Arc::new(left.to_turn_math(format!("{}:lhs", master_id))),
                    rhs: Arc::new(right.to_turn_math(format!("{}:rhs", master_id))),
                    operator: RelationOperatorNode::GreaterEqual,
                }),
            },
            NumberTheoryRelation::Divides { divisor, dividend } => MathNode {
                id: master_id.clone(),
                content: Arc::new(MathNodeContent::Relationship {
                    lhs: Arc::new(divisor.to_turn_math(format!("{}:divisor", master_id))),
                    rhs: Arc::new(dividend.to_turn_math(format!("{}:dividend", master_id))),
                    operator: RelationOperatorNode::Divides,
                }),
            },
            NumberTheoryRelation::Congruent {
                left,
                right,
                modulus,
            } => {
                // For congruent modulo, we need to create a special representation
                // left ≡ right (mod modulus)
                let base_relation = MathNode {
                    id: format!("{}:base", master_id.clone()),
                    content: Arc::new(MathNodeContent::Relationship {
                        lhs: Arc::new(left.to_turn_math(format!("{}:lhs", master_id))),
                        rhs: Arc::new(right.to_turn_math(format!("{}:rhs", master_id))),
                        operator: RelationOperatorNode::CongruentMod,
                    }),
                };

                // We need to add the modulus in a note or parenthetical
                let modulus_note = MathNode {
                    id: format!("{}:mod", master_id.clone()),
                    content: Arc::new(MathNodeContent::Text(format!(" (mod {:?})", modulus))),
                };

                // Combine them
                MathNode {
                    id: master_id.clone(),
                    content: Arc::new(MathNodeContent::Multiplications {
                        terms: vec![
                            (RefinedMulOrDivOperation::None, base_relation),
                            (RefinedMulOrDivOperation::None, modulus_note),
                        ],
                    }),
                }
            }
            NumberTheoryRelation::IsPrime { number } => MathNode {
                id: master_id.clone(),
                content: Arc::new(MathNodeContent::UnaryRelationship {
                    subject: Arc::new(number.to_turn_math(format!("{}:number", master_id))),
                    predicate: UnaryRelationOperatorNode::IsPrime,
                }),
            },
            NumberTheoryRelation::IsComposite { number } => MathNode {
                id: master_id.clone(),
                content: Arc::new(MathNodeContent::UnaryRelationship {
                    subject: Arc::new(number.to_turn_math(format!("{}:number", master_id))),
                    predicate: UnaryRelationOperatorNode::IsComposite,
                }),
            },
            NumberTheoryRelation::AreCoprime { first, second } => MathNode {
                id: master_id.clone(),
                content: Arc::new(MathNodeContent::Relationship {
                    lhs: Arc::new(first.to_turn_math(format!("{}:first", master_id))),
                    rhs: Arc::new(second.to_turn_math(format!("{}:second", master_id))),
                    operator: RelationOperatorNode::AreCoprime,
                }),
            },
            NumberTheoryRelation::IsQuadraticResidue { residue, modulus } => {
                // This is a custom relation with specific parameters
                // We can use a binary relation: residue is QR mod modulus
                let math_text = format!("QR(mod {:?})", modulus);

                MathNode {
                    id: master_id.clone(),
                    content: Arc::new(MathNodeContent::Relationship {
                        lhs: Arc::new(residue.to_turn_math(format!("{}:residue", master_id))),
                        rhs: Arc::new(MathNode {
                            id: format!("{}:mod_text", master_id),
                            content: Arc::new(MathNodeContent::Text(math_text)),
                        }),
                        operator: RelationOperatorNode::IsEqual,
                    }),
                }
            }
            NumberTheoryRelation::Custom { name, parameters } => {
                if parameters.len() == 1 {
                    // Unary relation
                    MathNode {
                        id: master_id.clone(),
                        content: Arc::new(MathNodeContent::UnaryRelationship {
                            subject: Arc::new(
                                parameters[0].to_turn_math(format!("{}:param0", master_id)),
                            ),
                            predicate: UnaryRelationOperatorNode::Custom(name.clone()),
                        }),
                    }
                } else if parameters.len() == 2 {
                    // Binary relation
                    MathNode {
                        id: master_id.clone(),
                        content: Arc::new(MathNodeContent::Relationship {
                            lhs: Arc::new(
                                parameters[0].to_turn_math(format!("{}:param0", master_id)),
                            ),
                            rhs: Arc::new(
                                parameters[1].to_turn_math(format!("{}:param1", master_id)),
                            ),
                            operator: RelationOperatorNode::Custom(name.clone()),
                        }),
                    }
                } else {
                    // For more than 2 parameters, create a custom function
                    MathNode {
                        id: master_id.clone(),
                        content: Arc::new(MathNodeContent::FunctionCall {
                            name: Arc::new(MathNode::identifier(Identifier::new_simple(
                                name.clone(),
                            ))),
                            parameters: parameters
                                .iter()
                                .enumerate()
                                .map(|(i, param)| {
                                    param.to_turn_math(format!("{}:param{}", master_id, i))
                                })
                                .collect(),
                        }),
                    }
                }
            }
        }
    }
}
