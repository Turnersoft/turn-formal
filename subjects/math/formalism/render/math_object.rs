use super::super::theorem::MathObject;
use crate::turn_render::{BracketStyle, MathNode, MathNodeContent, ToTurnMath};

impl ToTurnMath for MathObject {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let content = match self {
            MathObject::Group(group) => return group.to_turn_math(master_id),
            MathObject::Ring(ring) => todo!(),
            MathObject::Field(field) => todo!(),
            MathObject::Module(module) => todo!(),
            MathObject::Algebra(algebra) => todo!(),
            MathObject::TopologicalSpace(topological_space) => todo!(),
            MathObject::VectorSpace(vector_space) => todo!(),
            MathObject::Set(set) => todo!(),
            MathObject::Function(function) => todo!(),
            MathObject::Integer => todo!(),
            MathObject::Rational => todo!(),
            MathObject::Irrational => todo!(),
            MathObject::Real => todo!(),
            MathObject::Complex => todo!(),
            MathObject::Element(math_object) => {
                // element/member of a math object
                let object_node = math_object.to_turn_math(master_id.clone());
                MathNodeContent::ElementOf {
                    target: Box::new(object_node),
                }
            }
            MathObject::Morphism(math_object, math_object1) => todo!(),
            MathObject::Product(math_objects) => todo!(),
            MathObject::Coproduct(math_objects) => todo!(),
            MathObject::Todo(_) => todo!(),
        };
        MathNode {
            id: master_id,
            content: Box::new(content),
        }
    }
}
