use super::super::super::{
    formalism::expressions::{MathExpression, TheoryExpression},
    theories::number_theory::definitions::Number,
};
use super::super::objects::MathObject;
use super::super::relations::MathRelation;
use crate::turn_render::math_node::ToTurnMath;
use crate::turn_render::{BracketSize, BracketStyle, MathNode, MathNodeContent};
use crate::turn_render::{RichText, RichTextSegment, ToRichText};
use crate::{
    subjects::math::theories::groups::definitions::GroupExpression, turn_render::ScriptNode,
};
use crate::{
    subjects::math::theories::rings::definitions::{FieldExpression, RingExpression},
    turn_render::Identifier,
};
use serde::{Deserialize, Serialize};
use std::{string::String, sync::Arc};

impl ToTurnMath for Identifier {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id,
            content: Arc::new(MathNodeContent::Identifier(self.clone())),
        }
    }
}

impl ToTurnMath for MathExpression {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            // MathExpression::Var(id) => id.to_turn_math(master_id),
            MathExpression::Number(_num) => {
                // Number is a struct with no members, just render it as a generic number
                MathNode {
                    id: master_id,
                    content: Arc::new(MathNodeContent::Quantity {
                        number: "0".to_string(), // Default representation
                        scientific_notation: None,
                        unit: None,
                    }),
                }
            }
            MathExpression::Object(obj) => obj.to_turn_math(master_id),
            MathExpression::Expression(theory_expr) => {
                // For now, just display the expression as text
                theory_expr.to_turn_math(master_id)
            }
            MathExpression::Relation(rel) => rel.to_turn_math(master_id),
            MathExpression::ViewAs { expression, view } => {
                // Avoid infinite recursion by just returning a simple text representation
                MathNode {
                    id: master_id,
                    content: Arc::new(MathNodeContent::Text("⟨view⟩".to_string())),
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
                    content: Arc::new(MathNodeContent::Text("RingExpression (TODO)".to_string())),
                }
            }
            TheoryExpression::Field(_field_expr) => {
                // TODO: Implement ToTurnMath for FieldExpression or provide better placeholder
                MathNode {
                    id: master_id,
                    content: Arc::new(MathNodeContent::Text("FieldExpression (TODO)".to_string())),
                }
            }
        }
    }
}

// Generic implementation for Arc<T> where T: ToTurnMath
impl<T: ToTurnMath> ToTurnMath for Arc<T> {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        (**self).to_turn_math(master_id)
    }
}

impl ToRichText for MathExpression {
    fn to_rich_text(&self) -> RichText {
        match self {
            MathExpression::Object(obj) => match obj.as_ref() {
                super::super::objects::MathObject::Group(_) => RichText {
                    segments: vec![RichTextSegment::Text("Group".to_string())],
                    alignment: None,
                },
                super::super::objects::MathObject::Ring(_) => RichText {
                    segments: vec![RichTextSegment::Text("Ring".to_string())],
                    alignment: None,
                },
                super::super::objects::MathObject::Field(_) => RichText {
                    segments: vec![RichTextSegment::Text("Field".to_string())],
                    alignment: None,
                },
                super::super::objects::MathObject::Set(_) => RichText {
                    segments: vec![RichTextSegment::Text("Set".to_string())],
                    alignment: None,
                },
                _ => RichText {
                    segments: vec![RichTextSegment::Text("Object".to_string())],
                    alignment: None,
                },
            },
            MathExpression::Expression(theory_expr) => {
                // Delegate to theory-specific implementations
                match theory_expr {
                    TheoryExpression::Group(group_expr) => group_expr.to_rich_text(),
                    TheoryExpression::Ring(_ring_expr) => RichText {
                        segments: vec![RichTextSegment::Text("ring element".to_string())],
                        alignment: None,
                    },
                    TheoryExpression::Field(_field_expr) => RichText {
                        segments: vec![RichTextSegment::Text("field element".to_string())],
                        alignment: None,
                    },
                }
            }
            MathExpression::Expression(TheoryExpression::Ring(ring_expr)) => RichText {
                segments: vec![RichTextSegment::Text("ring element".to_string())],
                alignment: None,
            },
            MathExpression::Expression(TheoryExpression::Field(field_expr)) => RichText {
                segments: vec![RichTextSegment::Text("field element".to_string())],
                alignment: None,
            },
            MathExpression::Relation(_) => RichText {
                segments: vec![RichTextSegment::Text("relation".to_string())],
                alignment: None,
            },
            MathExpression::Number(_) => RichText {
                segments: vec![RichTextSegment::Text("number".to_string())],
                alignment: None,
            },
            MathExpression::ViewAs { .. } => RichText {
                segments: vec![RichTextSegment::Text("view".to_string())],
                alignment: None,
            },
        }
    }
}
