use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

//--- Imports from crate::turn_render ---
use crate::turn_render::math_node::{
    BracketSize, BracketStyle, MathNode, MathNodeContent, ToTurnMath,
};

//--- Imports from this crate (subjects) ---
use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};

// Field definitions from the current theory
use super::definitions::{
    AlgebraicClosureField, Field, FieldBasic, FieldOperation, FieldProperty, FieldRelation,
    FiniteField, FunctionField, OrderedField, PAdicField, TopologicalField,
};

impl ToTurnMath for Field {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            Field::Basic(_field_basic) => MathNode {
                id: format!("{}-field-basic", master_id),
                content: Box::new(MathNodeContent::Identifier {
                    body: "𝔽".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                }),
            },
            Field::Finite(finite_field) => MathNode {
                id: format!("{}-field-finite", master_id),
                content: Box::new(MathNodeContent::Identifier {
                    body: format!("𝔽_{}", finite_field.order),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                }),
            },
            Field::PAdicNumbers(p_adic) => MathNode {
                id: format!("{}-field-padic", master_id),
                content: Box::new(MathNodeContent::Identifier {
                    body: format!("ℚ_{}", p_adic.prime),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                }),
            },
            Field::Function(_func_field) => MathNode {
                id: format!("{}-field-function", master_id),
                content: Box::new(MathNodeContent::FunctionCall {
                    name: Box::new(MathNode {
                        id: format!("{}-field-function-name", master_id),
                        content: Box::new(MathNodeContent::Identifier {
                            body: "K".to_string(),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: true,
                        }),
                    }),
                    parameters: vec![MathNode {
                        id: format!("{}-field-function-var", master_id),
                        content: Box::new(MathNodeContent::Identifier {
                            body: "X".to_string(),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: false,
                        }),
                    }],
                }),
            },
            Field::Topological(_topo_field) => MathNode {
                id: format!("{}-field-topological", master_id),
                content: Box::new(MathNodeContent::Identifier {
                    body: "𝔽_top".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                }),
            },
            Field::Ordered(_ordered_field) => MathNode {
                id: format!("{}-field-ordered", master_id),
                content: Box::new(MathNodeContent::Identifier {
                    body: "𝔽_ord".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                }),
            },
            Field::AlgebraicClosure(_alg_closure) => MathNode {
                id: format!("{}-field-algebraic-closure", master_id),
                content: Box::new(MathNodeContent::Identifier {
                    body: "𝔽̄".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                }),
            },
        }
    }
}

impl ToTurnMath for FieldProperty {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let property_str = match self {
            FieldProperty::Characteristic(char_var) => {
                format!("char = {:?}", char_var)
            }
            FieldProperty::AlgebraicClosure(alg_var) => {
                format!("alg_closed = {:?}", alg_var)
            }
            FieldProperty::Ordering(ord_var) => {
                format!("ordered = {:?}", ord_var)
            }
            FieldProperty::Completeness(comp_var) => {
                format!("complete = {:?}", comp_var)
            }
            FieldProperty::Perfect => "perfect".to_string(),
        };

        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::Text(property_str)),
        }
    }
}

impl ToTurnMath for FieldOperation {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::Identifier {
                body: self.symbol.clone(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            }),
        }
    }
}

impl ToTurnMath for FieldRelation {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let content = match self {
            FieldRelation::IsSubfieldOf { subfield, field } => MathNodeContent::Relationship {
                lhs: Box::new(subfield.to_turn_math(format!("{}-lhs", master_id))),
                rhs: Box::new(field.to_turn_math(format!("{}-rhs", master_id))),
                operator: crate::turn_render::math_node::RelationOperatorNode::SubsetOf,
            },
            FieldRelation::IsExtensionOf {
                extension,
                base_field,
                degree,
            } => {
                let ext_node = extension.to_turn_math(format!("{}-ext", master_id));
                let base_node = base_field.to_turn_math(format!("{}-base", master_id));

                if let Some(deg) = degree {
                    // Show as [E : F] = n
                    MathNodeContent::Relationship {
                        lhs: Box::new(MathNode {
                            id: format!("{}-extension-degree", master_id),
                            content: Box::new(MathNodeContent::Bracketed {
                                inner: Box::new(MathNode {
                                    id: format!("{}-degree-expr", master_id),
                                    content: Box::new(MathNodeContent::Text(format!(
                                        "{} : {}",
                                        "E", "F"
                                    ))),
                                }),
                                style: BracketStyle::Square,
                                size: BracketSize::Normal,
                            }),
                        }),
                        rhs: Box::new(MathNode {
                            id: format!("{}-degree-value", master_id),
                            content: Box::new(MathNodeContent::Quantity {
                                number: deg.to_string(),
                                unit: None,
                            }),
                        }),
                        operator: crate::turn_render::math_node::RelationOperatorNode::Equal,
                    }
                } else {
                    // Just show E/F
                    MathNodeContent::Division {
                        numerator: Box::new(ext_node),
                        denominator: Box::new(base_node),
                        style: crate::turn_render::math_node::DivisionStyle::Inline,
                    }
                }
            }
            FieldRelation::IsIsomorphicTo { first, second } => MathNodeContent::Relationship {
                lhs: Box::new(first.to_turn_math(format!("{}-first", master_id))),
                rhs: Box::new(second.to_turn_math(format!("{}-second", master_id))),
                operator: crate::turn_render::math_node::RelationOperatorNode::IsIsomorphicTo,
            },
            _ => MathNodeContent::Text(format!("Field Relation: {:?}", self)),
        };

        MathNode {
            id: master_id,
            content: Box::new(content),
        }
    }
}
