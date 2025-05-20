use super::super::super::{
    formalism::expressions::{Identifier, MathExpression, TheoryExpression},
    theories::number_theory::definitions::Number,
};
use super::super::theorem::MathObject;
use crate::subjects::math::theories::groups::definitions::GroupExpression;
use crate::subjects::math::theories::rings::definitions::{FieldExpression, RingExpression};
use crate::turn_render::math_node::ToTurnMath;
use crate::turn_render::{BracketSize, BracketStyle, MathNode, MathNodeContent};
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
                    Identifier::E(id) => MathNode {
                        id: master_id,
                        content: Box::new(MathNodeContent::Identifier {
                            body: format!("E_{}", id),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: false,
                        }),
                    },
                    Identifier::M(id) => MathNode {
                        id: master_id,
                        content: Box::new(MathNodeContent::Identifier {
                            body: format!("M_{}", id),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: false,
                        }),
                    },
                    Identifier::N(id) => MathNode {
                        id: master_id,
                        content: Box::new(MathNodeContent::Identifier {
                            body: format!("N_{}", id),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: false,
                        }),
                    },
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
                obj.to_turn_math(master_id)
            }
            MathExpression::Expression(theory_expr) => {
                // For now, just display the expression as text
                theory_expr.to_turn_math(master_id)
            }
            MathExpression::Relation(rel) => {
                // Delegate to relation's implementation
                rel.to_turn_math(master_id)
            }
            MathExpression::ViewAs { expression, view } => {
                // For now, just wrap the expression in brackets
                let inner = expression.to_turn_math(format!("{}_inner", master_id));
                let view = view.to_turn_math(format!("{}_view", master_id));

                // MathNode {
                //     id: master_id,
                //     content: Box::new(MathNodeContent::Bracketed {
                //         inner: Box::new(inner),
                //         style: BracketStyle::Round,
                //         size: BracketSize::Normal,
                //     }),
                // }
                todo!()
            }
        }
    }
}

// Commented out due to removal of ToTurnMath trait from Group Theory
// This entire block below will be replaced by the new implementation
// impl ToTurnMath for TheoryExpression {
//     fn to_turn_math(&self, master_id: String) -> MathNode {
//         match self {
//             TheoryExpression::Group(group) => group.to_turn_math(master_id),
//             TheoryExpression::Ring(ring) => ring.to_turn_math(master_id),
//             TheoryExpression::Field(field) => field.to_turn_math(master_id),
//             TheoryExpression::Topology(topology) => topology.to_turn_math(master_id),
//             TheoryExpression::VectorSpace(vector_space) => {
//                 vector_space.to_turn_math(master_id)
//             }
//             TheoryExpression::Module(module) => module.to_turn_math(master_id),
//             TheoryExpression::ZFCSet(set) => set.to_turn_math(master_id),
//         }
//     }
// }

// New and complete implementation:
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
            } // Removed non-existent variants: Topology, VectorSpace, Module, ZFCSet
        }
    }
}
