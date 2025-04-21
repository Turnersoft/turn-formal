use crate::{
    subjects::math::formalism::core::MathObject,
    turn_render::{MathNode, ToTurnMath},
};

impl ToTurnMath for MathObject {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            MathObject::Group(group) => group.to_turn_math(master_id),
            MathObject::TopologicalGroup(topological_group) => todo!(),
            MathObject::LieGroup(lie_group) => todo!(),
            MathObject::Ring(ring) => todo!(),
            MathObject::Field(field) => todo!(),
            MathObject::Module(module) => todo!(),
            MathObject::Algebra(algebra) => todo!(),
            MathObject::TopologicalSpace(topological_space) => todo!(),
            MathObject::VectorSpace(vector_space) => todo!(),
            MathObject::Set(set) => todo!(),
            MathObject::Function(function) => todo!(),
        }
    }
}
