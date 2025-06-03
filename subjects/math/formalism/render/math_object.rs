use super::super::theorem::MathObject;
use super::expressions::ToStructuredFormat;
use crate::subjects::math::theories::groups::definitions::Group;
use crate::turn_render::math_node::ToTurnMath;
use crate::turn_render::{BracketStyle, MathNode, MathNodeContent, RelationOperatorNode};

impl ToTurnMath for MathObject {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let content = match self {
            MathObject::Group(group) => return group.to_turn_math(master_id),
            MathObject::Ring(_ring) => MathNodeContent::Identifier {
                body: "(R, +, ·)".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            },
            MathObject::Field(_field) => MathNodeContent::Identifier {
                body: "(𝔽, +, ·)".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            },
            MathObject::Module(_module) => MathNodeContent::Identifier {
                body: "M_R".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            },
            MathObject::Algebra(_algebra) => MathNodeContent::Identifier {
                body: "𝒜".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            },
            MathObject::TopologicalSpace(_ts) => MathNodeContent::Identifier {
                body: "(X, τ)".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            },
            MathObject::VectorSpace(_vs) => MathNodeContent::Identifier {
                body: "V".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            },
            MathObject::Set(_set) => MathNodeContent::Identifier {
                body: "S".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            },
            MathObject::Function(_func) => MathNodeContent::FunctionCall {
                name: Box::new(MathNode {
                    id: format!("{}_func", master_id),
                    content: Box::new(MathNodeContent::Identifier {
                        body: "f".to_string(),
                        pre_script: None,
                        mid_script: None,
                        post_script: None,
                        primes: 0,
                        is_function: true,
                    }),
                }),
                parameters: vec![MathNode {
                    id: format!("{}_domain", master_id),
                    content: Box::new(MathNodeContent::Identifier {
                        body: "X".to_string(),
                        pre_script: None,
                        mid_script: None,
                        post_script: None,
                        primes: 0,
                        is_function: false,
                    }),
                }],
            },
            MathObject::Integer => MathNodeContent::Identifier {
                body: "ℤ".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            },
            MathObject::Rational => MathNodeContent::Identifier {
                body: "ℚ".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            },
            MathObject::Irrational => MathNodeContent::Identifier {
                body: "ℝ\\ℚ".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            },
            MathObject::Real => MathNodeContent::Identifier {
                body: "ℝ".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            },
            MathObject::Complex => MathNodeContent::Identifier {
                body: "ℂ".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            },
            MathObject::Element(math_object) => {
                let object_node = math_object.to_turn_math(format!("{}_set", master_id));
                MathNodeContent::Relationship {
                    lhs: Box::new(MathNode {
                        id: format!("{}_elem", master_id),
                        content: Box::new(MathNodeContent::Identifier {
                            body: "x".to_string(),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: false,
                        }),
                    }),
                    rhs: Box::new(object_node),
                    operator: RelationOperatorNode::ElementOf,
                }
            }
            MathObject::Morphism(_from, _to) => MathNodeContent::Identifier {
                body: "φ".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            },
            MathObject::Product(_objs) => MathNodeContent::Identifier {
                body: "∏".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            },
            MathObject::Coproduct(_objs) => MathNodeContent::Identifier {
                body: "∐".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            },
            MathObject::Todo(s) => MathNodeContent::Identifier {
                body: "?".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            },
        };
        MathNode {
            id: master_id,
            content: Box::new(content),
        }
    }
}

impl ToStructuredFormat for MathObject {
    type Output = String;

    fn to_structured(&self) -> Self::Output {
        match self {
            MathObject::Set(_) => "Set".to_string(),
            MathObject::Element(element) => "Element".to_string(),
            MathObject::Group(_) => "Group".to_string(),
            MathObject::Ring(_) => "Ring".to_string(),
            MathObject::Field(_) => "Field".to_string(),
            MathObject::Module(_) => "Module".to_string(),
            MathObject::Algebra(_) => "Algebra".to_string(),
            MathObject::TopologicalSpace(_) => "Topological Space".to_string(),
            MathObject::VectorSpace(_) => "Vector Space".to_string(),
            MathObject::Function(_) => "Function".to_string(),
            MathObject::Integer => "Integer".to_string(),
            MathObject::Rational => "Rational".to_string(),
            MathObject::Irrational => "Irrational".to_string(),
            MathObject::Real => "Real".to_string(),
            MathObject::Complex => "Complex".to_string(),
            MathObject::Morphism(from, to) => "Morphism".to_string(),
            MathObject::Product(_) => "Product".to_string(),
            MathObject::Coproduct(_) => "Coproduct".to_string(),
            MathObject::Todo(description) => format!("Todo: {}", description),
        }
    }
}
