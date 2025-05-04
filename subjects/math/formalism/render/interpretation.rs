use super::super::interpretation::TypeViewOperator;
use crate::turn_render::{MathNode, ToTurnMath};

impl ToTurnMath for TypeViewOperator {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            TypeViewOperator::AsGroupElement { group } => group.to_turn_math(master_id),
            TypeViewOperator::AsRingElement { ring } => todo!(),
            TypeViewOperator::AsFieldElement { field } => todo!(),
            TypeViewOperator::AsGroup { operation } => todo!(),
            TypeViewOperator::AsRing { addition } => todo!(),
            TypeViewOperator::AsTopologicalSpace { topology } => todo!(),
            TypeViewOperator::AsHomomorphism { source, target } => todo!(),
            TypeViewOperator::AsCyclicGroup => todo!(),
            TypeViewOperator::AsPoint => todo!(),
            TypeViewOperator::AsFunction { domain } => todo!(),
            TypeViewOperator::AsLinearTransformation => todo!(),
            TypeViewOperator::Custom {
                name,
                source_type,
                target_type,
                parameters,
            } => todo!(),
        }
    }
}
