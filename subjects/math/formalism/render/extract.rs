use crate::{
    subjects::math::formalism::{extract::Parametrizable, location::Located},
    turn_render::{LogicalNode, MathNode, ToLogicalNode, ToTurnMath},
};

impl<T: ToTurnMath> ToTurnMath for Parametrizable<T> {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            Parametrizable::Concrete(t) => t.to_turn_math(master_id),
            Parametrizable::Variable(id) => id.to_turn_math(master_id),
        }
    }
}

impl<T: ToLogicalNode> ToLogicalNode for Parametrizable<T> {
    fn to_logical_node(&self) -> LogicalNode {
        match self {
            Parametrizable::Concrete(t) => t.to_logical_node(),
            Parametrizable::Variable(id) => LogicalNode::Atomic(id.to_turn_math("".to_string())),
        }
    }
}
