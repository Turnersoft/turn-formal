use super::super::super::{
    formalism::expressions::{Identifier, MathExpression, TheoryExpression},
    theories::number_theory::definitions::Number,
};
use super::super::relations::MathRelation;
use super::super::theorem::MathObject;
use crate::subjects::math::theories::rings::definitions::{FieldExpression, RingExpression};
use crate::turn_render::math_node::ToTurnMath;
use crate::turn_render::{BracketSize, BracketStyle, MathNode, MathNodeContent};
use crate::{
    subjects::math::theories::groups::definitions::GroupExpression, turn_render::ScriptNode,
};
use serde::{Deserialize, Serialize};
use std::string::String;

/// Trait for converting internal types to structured export format
pub trait ToStructuredFormat {
    type Output;
    fn to_structured(&self) -> Self::Output;
}

impl ToStructuredFormat for Identifier {
    type Output = MathNode;

    fn to_structured(&self) -> Self::Output {
        match self {
            Identifier::Name(name, index) => {
                if *index == 0 {
                    MathNode::identifier(name.clone())
                } else {
                    MathNode {
                        id: format!("{}-{}", name, index),
                        content: Box::new(MathNodeContent::String(format!("{}_{}", name, index))),
                    }
                }
            }
            Identifier::O(idx) => MathNode {
                id: format!("o-{}", idx),
                content: Box::new(MathNodeContent::String(format!("o_{}", idx))),
            },
            Identifier::M(idx) => MathNode {
                id: format!("m-{}", idx),
                content: Box::new(MathNodeContent::String(format!("m_{}", idx))),
            },
            Identifier::E(idx) => MathNode {
                id: format!("e-{}", idx),
                content: Box::new(MathNodeContent::String(format!("e_{}", idx))),
            },
            Identifier::N(idx) => MathNode {
                id: format!("n-{}", idx),
                content: Box::new(MathNodeContent::String(format!("n_{}", idx))),
            },
        }
    }
}

impl ToTurnMath for Identifier {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            Identifier::Name(name, n) => {
                MathNode::identifier_with_simple_sub_scripts(name.clone(), vec![n.to_string()])
            }
            Identifier::O(n) => {
                MathNode::identifier_with_simple_sub_scripts("o".to_string(), vec![n.to_string()])
            }
            Identifier::M(n) => {
                MathNode::identifier_with_simple_sub_scripts("m".to_string(), vec![n.to_string()])
            }
            Identifier::E(n) => {
                MathNode::identifier_with_simple_sub_scripts("e".to_string(), vec![n.to_string()])
            }
            Identifier::N(n) => {
                MathNode::identifier_with_simple_sub_scripts("n".to_string(), vec![n.to_string()])
            }
        }
    }
}

impl ToStructuredFormat for MathExpression {
    type Output = MathNode;

    fn to_structured(&self) -> Self::Output {
        match self {
            MathExpression::Var(identifier) => identifier.to_structured(),
            MathExpression::Number(n) => MathNode {
                id: "number".to_string(),
                content: Box::new(MathNodeContent::String(n.to_string())),
            },
            MathExpression::Expression(theory_expr) => MathNode {
                id: "theory-expr".to_string(),
                content: Box::new(MathNodeContent::Text("Theory Expression".to_string())),
            },
            MathExpression::Relation(relation) => MathNode {
                id: "math-relation".to_string(),
                content: Box::new(MathNodeContent::Text("Mathematical relation".to_string())),
            },
            MathExpression::ViewAs { expression, view } => {
                // Recursively convert the inner expression
                expression.to_structured()
            }
            MathExpression::Object(obj) => MathNode {
                id: "math-object".to_string(),
                content: Box::new(MathNodeContent::Text("Mathematical object".to_string())),
            },
        }
    }
}

impl ToTurnMath for MathExpression {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            MathExpression::Var(id) => {
                // Convert variable identifier to MathNode
                match id {
                    Identifier::Name(name, _) => MathNode::identifier(name.clone()),
                    Identifier::O(id) => MathNode {
                        id: master_id,
                        content: Box::new(MathNodeContent::String(format!("O_{}", id))),
                    },
                    Identifier::E(id) => MathNode {
                        id: master_id,
                        content: Box::new(MathNodeContent::String(format!("E_{}", id))),
                    },
                    Identifier::M(id) => MathNode {
                        id: master_id,
                        content: Box::new(MathNodeContent::String(format!("M_{}", id))),
                    },
                    Identifier::N(id) => MathNode {
                        id: master_id,
                        content: Box::new(MathNodeContent::String(format!("N_{}", id))),
                    },
                }
            }
            MathExpression::Number(_num) => {
                // Number is a struct with no members, just render it as a generic number
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Quantity {
                        number: "0".to_string(), // Default representation
                        scientific_notation: None,
                        unit: None,
                    }),
                }
            }
            MathExpression::Object(obj) => {
                // For now, just display the name as text
                (**obj).to_turn_math(master_id)
            }
            MathExpression::Expression(theory_expr) => {
                // For now, just display the expression as text
                theory_expr.to_turn_math(master_id)
            }
            MathExpression::Relation(rel) => {
                // Delegate to relation's implementation
                (**rel).to_turn_math(master_id)
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

impl ToTurnMath for TheoryExpression {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            TheoryExpression::Group(group_expr) => group_expr.to_turn_math(master_id),
            TheoryExpression::Ring(_ring_expr) => {
                // TODO: Implement ToTurnMath for RingExpression or provide better placeholder
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text("RingExpression (TODO)".to_string())),
                }
            }
            TheoryExpression::Field(_field_expr) => {
                // TODO: Implement ToTurnMath for FieldExpression or provide better placeholder
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text("FieldExpression (TODO)".to_string())),
                }
            }
        }
    }
}
