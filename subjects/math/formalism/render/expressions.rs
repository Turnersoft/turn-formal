use super::super::super::{
    formalism::expressions::{Identifier, MathExpression, TheoryExpression},
    theories::number_theory::definitions::Number,
};
use crate::turn_render::{BracketSize, BracketStyle, MathNode, MathNodeContent, ToTurnMath};
use std::string::String;

impl ToTurnMath for MathExpression {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            MathExpression::Var(id) => {
                // Convert variable identifier to MathNode
                match id {
                    Identifier::Name(name, _) => MathNode {
                        id: master_id,
                        content: Box::new(MathNodeContent::Identifier {
                            body: name.clone(),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: false,
                        }),
                    },
                    Identifier::O(id) => MathNode {
                        id: master_id,
                        content: Box::new(MathNodeContent::Identifier {
                            body: format!("O_{}", id),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: false,
                        }),
                    },
                    _ => {
                        // Handle other identifier types
                        MathNode {
                            id: master_id,
                            content: Box::new(MathNodeContent::Identifier {
                                body: format!("{:?}", id),
                                pre_script: None,
                                mid_script: None,
                                post_script: None,
                                primes: 0,
                                is_function: false,
                            }),
                        }
                    }
                }
            }
            MathExpression::Number(_num) => {
                // Number is a struct with no members, just render it as a generic number
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Quantity {
                        number: "0".to_string(), // Default representation
                        unit: None,
                    }),
                }
            }
            MathExpression::Object(obj) => {
                // For now, just display the name as text
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text(format!("{:?}", obj))),
                }
            }
            MathExpression::Expression(theory_expr) => {
                // For now, just display the expression as text
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text(format!("{:?}", theory_expr))),
                }
            }
            MathExpression::Relation(rel) => {
                // Delegate to relation's implementation
                rel.to_turn_math(master_id)
            }
            MathExpression::ViewAs { expression, view } => {
                // For now, just wrap the expression in brackets
                let inner = expression.to_turn_math(format!("{}_inner", master_id));

                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Bracketed {
                        inner: Box::new(inner),
                        style: BracketStyle::Round,
                        size: BracketSize::Normal,
                    }),
                }
            }
        }
    }
}
