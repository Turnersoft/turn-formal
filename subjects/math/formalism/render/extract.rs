use crate::{
    subjects::math::formalism::{expressions::Identifier, extract::Parametrizable},
    turn_render::{MathNode, MathNodeContent, ToTurnMath},
};

impl<T: ToTurnMath> ToTurnMath for Parametrizable<T> {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            Parametrizable::Concrete(object) => object.to_turn_math(master_id),
            Parametrizable::Variable(identifier) => identifier.to_turn_math(master_id),
        }
    }
}

impl ToTurnMath for Identifier {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            Identifier::O(o) => MathNode::identifier(format!("O_{}", o)),
            Identifier::M(m) => MathNode::identifier(format!("M_{}", m)),
            Identifier::E(e) => MathNode::identifier(format!("E_{}", e)),
            Identifier::N(n) => MathNode::identifier(format!("N_{}", n)),
            Identifier::Name(name, index) => {
                if *index == 0 {
                    MathNode::identifier(name.clone())
                } else {
                    MathNode::identifier(format!("{}_{}", name, index))
                }
            }
        }
    }
}
