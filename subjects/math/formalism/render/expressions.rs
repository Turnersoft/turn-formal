use super::super::super::{
    formalism::expressions::{Identifier, MathExpression, TheoryExpression},
    theories::number_theory::definitions::Number,
};
use super::super::theorem::MathObject;
use crate::subjects::math::theories::groups::definitions::GroupExpression;
use crate::subjects::math::theories::rings::definitions::{FieldExpression, RingExpression};
use crate::turn_render::math_node::ToTurnMath;
use crate::turn_render::{BracketSize, BracketStyle, MathNode, MathNodeContent};
use serde::{Deserialize, Serialize};
use std::string::String;

// Add conversion trait for structured export
use crate::turn_render::section_node::{
    NumberType, OperationType, SetType, StructuredExpression, StructuredGroupElement,
    StructuredGroupExpression, StructuredGroupProperty, StructuredMathObject, StructuredStatement,
    structured_group_expr, structured_math_object, structured_num, structured_op,
    structured_placeholder, structured_var,
};

/// Trait for converting internal types to structured export format
pub trait ToStructuredFormat {
    type Output;
    fn to_structured(&self) -> Self::Output;
}

impl ToStructuredFormat for Identifier {
    type Output = StructuredExpression;

    fn to_structured(&self) -> Self::Output {
        match self {
            Identifier::Name(name, index) => {
                if *index == 0 {
                    structured_var(name)
                } else {
                    StructuredExpression::Variable {
                        name: name.clone(),
                        subscript: Some(index.to_string()),
                        superscript: None,
                    }
                }
            }
            Identifier::O(idx) => StructuredExpression::Variable {
                name: "o".to_string(),
                subscript: Some(idx.to_string()),
                superscript: None,
            },
            Identifier::M(idx) => StructuredExpression::Variable {
                name: "m".to_string(),
                subscript: Some(idx.to_string()),
                superscript: None,
            },
            Identifier::E(idx) => StructuredExpression::Variable {
                name: "e".to_string(),
                subscript: Some(idx.to_string()),
                superscript: None,
            },
            Identifier::N(idx) => StructuredExpression::Variable {
                name: "n".to_string(),
                subscript: Some(idx.to_string()),
                superscript: None,
            },
        }
    }
}

impl ToStructuredFormat for MathExpression {
    type Output = StructuredExpression;

    fn to_structured(&self) -> Self::Output {
        match self {
            MathExpression::Var(identifier) => identifier.to_structured(),
            MathExpression::Number(n) => {
                StructuredExpression::Number {
                    value: n.to_string(),
                    number_type: NumberType::Integer, // Could be enhanced with proper type detection
                }
            }
            MathExpression::Expression(theory_expr) => {
                match theory_expr {
                    TheoryExpression::Group(group_expr) => {
                        // Convert group expressions to structured form
                        structured_group_expr(group_expr.to_structured_group_expression())
                    }
                    _ => structured_placeholder("Complex theory expression"),
                }
            }
            MathExpression::Relation(relation) => structured_placeholder("Mathematical relation"),
            MathExpression::ViewAs { expression, view } => {
                // Recursively convert the inner expression
                expression.to_structured()
            }
            MathExpression::Object(obj) => structured_math_object(obj.to_structured_math_object()),
        }
    }
}

// Extension trait for group expressions
trait GroupExpressionExt {
    fn operation_description(&self) -> String;
    fn to_structured_group_expression(&self) -> StructuredGroupExpression;
}

impl GroupExpressionExt for crate::subjects::math::theories::groups::definitions::GroupExpression {
    fn operation_description(&self) -> String {
        use crate::subjects::math::theories::groups::definitions::GroupExpression;
        match self {
            GroupExpression::Operation { .. } => "group operation".to_string(),
            GroupExpression::Inverse { .. } => "group inverse".to_string(),
            GroupExpression::Power { .. } => "group power".to_string(),
            GroupExpression::Identity(_) => "group identity".to_string(),
            GroupExpression::Commutator { .. } => "commutator".to_string(),
            GroupExpression::Element { .. } => "group element".to_string(),
            GroupExpression::Coset { .. } => "coset".to_string(),
            GroupExpression::ActionOnElement { .. } => "group action on element".to_string(),
            GroupExpression::GroupOrder { .. } => "group order".to_string(),
            GroupExpression::ElementOrder { .. } => "element order".to_string(),
            GroupExpression::Homomorphism(_) => "group homomorphism".to_string(),
        }
    }

    fn to_structured_group_expression(&self) -> StructuredGroupExpression {
        use crate::subjects::math::formalism::extract::Parametrizable;
        use crate::subjects::math::theories::groups::definitions::{GroupElement, GroupExpression};

        match self {
            GroupExpression::Element { group, element } => StructuredGroupExpression::Element {
                group: Box::new(structured_placeholder("G")),
                element: Box::new(match element {
                    Parametrizable::Concrete(elem) => match elem {
                        GroupElement::Integer(n) => StructuredGroupElement::Integer(*n),
                        GroupElement::Permutation(p) => {
                            StructuredGroupElement::Permutation(p.clone())
                        }
                        GroupElement::Matrix(m) => StructuredGroupElement::Matrix(m.clone()),
                        GroupElement::Symbol(s) => StructuredGroupElement::Symbol(s.clone()),
                    },
                    Parametrizable::Variable(id) => {
                        // Convert identifier to structured element
                        match id {
                            Identifier::Name(name, _) => {
                                StructuredGroupElement::Symbol(name.clone())
                            }
                            _ => StructuredGroupElement::Symbol("var".to_string()),
                        }
                    }
                }),
            },
            GroupExpression::Identity(group) => StructuredGroupExpression::Identity {
                group: Box::new(structured_placeholder("G")),
            },
            GroupExpression::Operation { group, left, right } => {
                // Recursively convert operands
                let left_expr = match left.as_ref() {
                    Parametrizable::Concrete(expr) => expr.to_structured_group_expression(),
                    Parametrizable::Variable(id) => {
                        // Create element expression for variable
                        StructuredGroupExpression::Element {
                            group: Box::new(structured_placeholder("G")),
                            element: Box::new(match id {
                                Identifier::Name(name, _) => {
                                    StructuredGroupElement::Symbol(name.clone())
                                }
                                _ => StructuredGroupElement::Symbol("var".to_string()),
                            }),
                        }
                    }
                };

                let right_expr = match right.as_ref() {
                    Parametrizable::Concrete(expr) => expr.to_structured_group_expression(),
                    Parametrizable::Variable(id) => {
                        // Create element expression for variable
                        StructuredGroupExpression::Element {
                            group: Box::new(structured_placeholder("G")),
                            element: Box::new(match id {
                                Identifier::Name(name, _) => {
                                    StructuredGroupElement::Symbol(name.clone())
                                }
                                _ => StructuredGroupElement::Symbol("var".to_string()),
                            }),
                        }
                    }
                };

                StructuredGroupExpression::Operation {
                    group: Box::new(structured_placeholder("G")),
                    left: Box::new(structured_group_expr(left_expr)),
                    right: Box::new(structured_group_expr(right_expr)),
                    operation_type:
                        crate::turn_render::section_node::GroupOperationType::Multiplication,
                }
            }
            GroupExpression::Inverse { group, element } => {
                let elem_expr = match element.as_ref() {
                    Parametrizable::Concrete(expr) => expr.to_structured_group_expression(),
                    Parametrizable::Variable(id) => StructuredGroupExpression::Element {
                        group: Box::new(structured_placeholder("G")),
                        element: Box::new(match id {
                            Identifier::Name(name, _) => {
                                StructuredGroupElement::Symbol(name.clone())
                            }
                            _ => StructuredGroupElement::Symbol("var".to_string()),
                        }),
                    },
                };

                StructuredGroupExpression::Inverse {
                    group: Box::new(structured_placeholder("G")),
                    element: Box::new(structured_group_expr(elem_expr)),
                }
            }
            GroupExpression::Power {
                group,
                base,
                exponent,
            } => {
                let base_expr = match base.as_ref() {
                    Parametrizable::Concrete(expr) => expr.to_structured_group_expression(),
                    Parametrizable::Variable(id) => StructuredGroupExpression::Element {
                        group: Box::new(structured_placeholder("G")),
                        element: Box::new(match id {
                            Identifier::Name(name, _) => {
                                StructuredGroupElement::Symbol(name.clone())
                            }
                            _ => StructuredGroupElement::Symbol("var".to_string()),
                        }),
                    },
                };

                let exp_expr = match exponent {
                    Parametrizable::Concrete(n) => {
                        structured_num(&n.to_string(), NumberType::Integer)
                    }
                    Parametrizable::Variable(id) => id.to_structured(),
                };

                StructuredGroupExpression::Power {
                    group: Box::new(structured_placeholder("G")),
                    base: Box::new(structured_group_expr(base_expr)),
                    exponent: Box::new(exp_expr),
                }
            }
            GroupExpression::Commutator { group, a, b } => {
                let a_expr = match a.as_ref() {
                    Parametrizable::Concrete(expr) => expr.to_structured_group_expression(),
                    Parametrizable::Variable(id) => StructuredGroupExpression::Element {
                        group: Box::new(structured_placeholder("G")),
                        element: Box::new(match id {
                            Identifier::Name(name, _) => {
                                StructuredGroupElement::Symbol(name.clone())
                            }
                            _ => StructuredGroupElement::Symbol("var".to_string()),
                        }),
                    },
                };

                let b_expr = match b.as_ref() {
                    Parametrizable::Concrete(expr) => expr.to_structured_group_expression(),
                    Parametrizable::Variable(id) => StructuredGroupExpression::Element {
                        group: Box::new(structured_placeholder("G")),
                        element: Box::new(match id {
                            Identifier::Name(name, _) => {
                                StructuredGroupElement::Symbol(name.clone())
                            }
                            _ => StructuredGroupElement::Symbol("var".to_string()),
                        }),
                    },
                };

                StructuredGroupExpression::Commutator {
                    group: Box::new(structured_placeholder("G")),
                    first: Box::new(structured_group_expr(a_expr)),
                    second: Box::new(structured_group_expr(b_expr)),
                }
            }
            GroupExpression::Coset {
                group,
                element,
                subgroup,
                is_left,
            } => StructuredGroupExpression::Coset {
                group: Box::new(structured_placeholder("G")),
                element: Box::new(structured_placeholder("Element")),
                subgroup: Box::new(structured_placeholder("H")),
                is_left: *is_left,
            },
            GroupExpression::GroupOrder { group } => StructuredGroupExpression::GroupOrder {
                group: Box::new(structured_placeholder("G")),
            },
            GroupExpression::ElementOrder { element, group } => {
                StructuredGroupExpression::ElementOrder {
                    element: Box::new(structured_placeholder("Element")),
                    group: Box::new(structured_placeholder("G")),
                }
            }
            GroupExpression::Homomorphism(hom) => StructuredGroupExpression::Homomorphism {
                domain: Box::new(structured_placeholder("Domain")),
                codomain: Box::new(structured_placeholder("Codomain")),
            },
            _ => StructuredGroupExpression::Identity {
                group: Box::new(structured_placeholder("G")),
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
