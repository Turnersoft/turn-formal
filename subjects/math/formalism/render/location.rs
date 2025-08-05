use std::sync::Arc;

use crate::{
    subjects::math::formalism::{extract::Parametrizable, location::Located},
    turn_render::{LogicalNode, MathNode, MathNodeContent, ToLogicalNode, ToTurnMath},
};

impl<T: ToTurnMath + ToLogicalNode> ToTurnMath for Located<T> {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        self.data.to_turn_math(master_id)
    }
}

impl<T: ToLogicalNode> ToLogicalNode for Located<T> {
    fn to_logical_node(&self) -> LogicalNode {
        match &self.data {
            Parametrizable::Concrete(arc) => {
                // Dereference the Arc to get the concrete value
                let concrete_value = &**arc;
                concrete_value.to_logical_node()
            }
            Parametrizable::Variable(id) => LogicalNode::Atomic(MathNode {
                id: self.id.clone(),
                content: Arc::new(MathNodeContent::Identifier(id.clone())),
            }),
        }
    }
}

impl<T: ToLogicalNode> ToLogicalNode for Arc<T> {
    fn to_logical_node(&self) -> LogicalNode {
        self.to_logical_node()
    }
}
