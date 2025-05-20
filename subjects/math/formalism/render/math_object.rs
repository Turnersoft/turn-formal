use super::super::theorem::MathObject;
use crate::subjects::math::theories::groups::definitions::Group;
use crate::turn_render::math_node::ToTurnMath;
use crate::turn_render::{BracketStyle, MathNode, MathNodeContent};

// Commented out due to removal of ToTurnMath trait from Group Theory
// impl ToTurnMath for MathObject {
//     fn to_turn_math(&self, master_id: String) -> MathNode {
//         match self {
//             MathObject::Group(group) => return group.to_turn_math(master_id),
//             MathObject::Ring(ring) => return ring.to_turn_math(master_id),
//             MathObject::Field(field) => return field.to_turn_math(master_id),
//             MathObject::VectorSpace(vector_space) => return vector_space.to_turn_math(master_id),
//             MathObject::Module(module) => return module.to_turn_math(master_id),
//             MathObject::TopologicalSpace(topological_space) => {
//                 return topological_space.to_turn_math(master_id)
//             }
//             MathObject::ZFCSet(zfc_set) => return zfc_set.to_turn_math(master_id),
//         }
//     }
// }

impl ToTurnMath for MathObject {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let content = match self {
            MathObject::Group(group) => return group.to_turn_math(master_id),
            MathObject::Ring(_ring) => MathNodeContent::Text("Ring (TODO)".to_string()),
            MathObject::Field(_field) => MathNodeContent::Text("Field (TODO)".to_string()),
            MathObject::Module(_module) => MathNodeContent::Text("Module (TODO)".to_string()),
            MathObject::Algebra(_algebra) => MathNodeContent::Text("Algebra (TODO)".to_string()),
            MathObject::TopologicalSpace(_ts) => {
                MathNodeContent::Text("TopologicalSpace (TODO)".to_string())
            }
            MathObject::VectorSpace(_vs) => MathNodeContent::Text("VectorSpace (TODO)".to_string()),
            MathObject::Set(_set) => MathNodeContent::Text("Set (TODO)".to_string()),
            MathObject::Function(_func) => MathNodeContent::Text("Function (TODO)".to_string()),
            MathObject::Integer => MathNodeContent::Text("Integer (Type)".to_string()),
            MathObject::Rational => MathNodeContent::Text("Rational (Type)".to_string()),
            MathObject::Irrational => MathNodeContent::Text("Irrational (Type)".to_string()),
            MathObject::Real => MathNodeContent::Text("Real (Type)".to_string()),
            MathObject::Complex => MathNodeContent::Text("Complex (Type)".to_string()),
            MathObject::Element(math_object) => {
                let object_node = math_object.to_turn_math(format!("{}_elem_of", master_id));
                MathNodeContent::Text(format!("element of {:?}", object_node.id))
            }
            MathObject::Morphism(_from, _to) => {
                MathNodeContent::Text("Morphism (TODO)".to_string())
            }
            MathObject::Product(_objs) => MathNodeContent::Text("Product (TODO)".to_string()),
            MathObject::Coproduct(_objs) => MathNodeContent::Text("Coproduct (TODO)".to_string()),
            MathObject::Todo(s) => MathNodeContent::Text(format!("MathObject TODO: {}", s)),
        };
        MathNode {
            id: master_id,
            content: Box::new(content),
        }
    }
}
