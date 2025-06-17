use super::super::objects::MathObject;
use crate::subjects::math::theories::groups::definitions::Group;
use crate::turn_render::math_node::ToTurnMath;
use crate::turn_render::{BracketStyle, MathNode, MathNodeContent, RelationOperatorNode};

impl ToTurnMath for MathObject {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let content = match self {
            MathObject::Group(group) => return group.to_turn_math(master_id),
            _ => todo!(),
        };
        MathNode {
            id: master_id,
            content: Box::new(content),
        }
    }
}
