use super::super::interpretation::TypeViewOperator;
use crate::subjects::math::theories::groups::definitions::Group;
use crate::turn_render::math_node::ToTurnMath;
use crate::turn_render::{MathNode, MathNodeContent};

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
                content: Box::new(MathNodeContent::Text(
                    "TypeView AsRingElement (TODO)".to_string(),
                )),
            },
            TypeViewOperator::AsFieldElement { field: _ } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text(
                    "TypeView AsFieldElement (TODO)".to_string(),
                )),
            },
            TypeViewOperator::AsGroup { operation: _ } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("TypeView AsGroup (TODO)".to_string())),
            },
            TypeViewOperator::AsRing { addition: _ } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("TypeView AsRing (TODO)".to_string())),
            },
            TypeViewOperator::AsTopologicalSpace { topology: _ } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text(
                    "TypeView AsTopologicalSpace (TODO)".to_string(),
                )),
            },
            TypeViewOperator::AsHomomorphism {
                source: _,
                target: _,
            } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text(
                    "TypeView AsHomomorphism (TODO)".to_string(),
                )),
            },
            TypeViewOperator::AsCyclicGroup => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text(
                    "TypeView AsCyclicGroup (TODO)".to_string(),
                )),
            },
            TypeViewOperator::AsPoint => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("TypeView AsPoint (TODO)".to_string())),
            },
            TypeViewOperator::AsFunction { domain: _ } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text(
                    "TypeView AsFunction (TODO)".to_string(),
                )),
            },
            TypeViewOperator::AsLinearTransformation => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text(
                    "TypeView AsLinearTransformation (TODO)".to_string(),
                )),
            },
            TypeViewOperator::Custom {
                name,
                source_type: _,
                target_type: _,
                parameters: _,
            } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text(format!("TypeView Custom: {}", name))),
            },
        }
    }
}
