use std::sync::Arc;

use crate::{
    subjects::math::formalism::{extract::Parametrizable, location::Located},
    turn_render::{
        LogicalNode, MathNode, MathNodeContent, RichText, RichTextSegment, ToLogicalNode,
        ToRichText, ToTurnMath,
    },
};

impl<T: ToTurnMath> ToTurnMath for Located<T> {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        self.data.to_turn_math(master_id)
    }
}

impl<T: ToRichText> ToRichText for Located<T> {
    fn to_rich_text(&self) -> RichText {
        self.data.to_rich_text()
    }
}

impl<T: ToRichText> ToRichText for Parametrizable<Arc<T>> {
    fn to_rich_text(&self) -> RichText {
        match &self {
            Parametrizable::Concrete(arc) => {
                let concrete_value = &**arc;
                concrete_value.to_rich_text()
            }
            Parametrizable::Variable(id) => RichText {
                segments: vec![RichTextSegment::Math(id.to_turn_math("".to_string()))],
                alignment: None,
            },
        }
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
