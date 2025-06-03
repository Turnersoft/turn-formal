use super::super::interpretation::TypeViewOperator;
use crate::subjects::math::theories::groups::definitions::Group;
use crate::turn_render::*;

// Commented out due to removal of ToTurnMath trait from Group Theory
// impl ToTurnMath for TypeViewOperator {
//     fn to_turn_math(&self, master_id: String) -> MathNode {
//         match self {
//             TypeViewOperator::AsGroupElement { group } => group.to_turn_math(master_id),
//             TypeViewOperator::AsRingElement { ring } => ring.to_turn_math(master_id),
//             TypeViewOperator::AsFieldElement { field } => field.to_turn_math(master_id),
//             TypeViewOperator::AsVector { vector_space } => vector_space.to_turn_math(master_id),
//             TypeViewOperator::AsModuleElement { module } => module.to_turn_math(master_id),
//             TypeViewOperator::AsTopologicalSpaceElement { topological_space } => {
//                 topological_space.to_turn_math(master_id)
//             }
//             TypeViewOperator::AsZFCSetElement { zfc_set } => zfc_set.to_turn_math(master_id),
//         }
//     }
// }

impl ToTurnMath for TypeViewOperator {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            TypeViewOperator::AsGroupElement { group } => group.to_turn_math(master_id),
            TypeViewOperator::AsRingElement { ring: _ } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Identifier {
                    body: "r ∈ R".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                }),
            },
            TypeViewOperator::AsFieldElement { field: _ } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Identifier {
                    body: "f ∈ 𝔽".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                }),
            },
            TypeViewOperator::AsGroup { operation: _ } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Identifier {
                    body: "(G, ∘)".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                }),
            },
            TypeViewOperator::AsRing { addition: _ } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Identifier {
                    body: "(R, +, ·)".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                }),
            },
            TypeViewOperator::AsTopologicalSpace { topology: _ } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Identifier {
                    body: "(X, τ)".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                }),
            },
            TypeViewOperator::AsHomomorphism {
                source: _,
                target: _,
            } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Identifier {
                    body: "φ: G → H".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                }),
            },
            TypeViewOperator::AsCyclicGroup => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Identifier {
                    body: "⟨g⟩".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                }),
            },
            TypeViewOperator::AsPoint => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Identifier {
                    body: "p ∈ X".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                }),
            },
            TypeViewOperator::AsFunction { domain: _ } => {
                let arg_id = format!("{}_arg", master_id.clone());
                MathNode {
                    id: master_id.clone(),
                    content: Box::new(MathNodeContent::FunctionCall {
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
                            id: arg_id,
                            content: Box::new(MathNodeContent::Identifier {
                                body: "x".to_string(),
                                pre_script: None,
                                mid_script: None,
                                post_script: None,
                                primes: 0,
                                is_function: false,
                            }),
                        }],
                    }),
                }
            }
            TypeViewOperator::AsLinearTransformation => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Identifier {
                    body: "T: V → W".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                }),
            },
            TypeViewOperator::Custom {
                name,
                source_type: _,
                target_type: _,
                parameters: _,
            } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Identifier {
                    body: format!("Custom: {}", name),
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
