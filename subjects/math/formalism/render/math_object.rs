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

// Add conversion trait for structured export
use crate::turn_render::section_node::{
    StructuredExpression, StructuredGroupProperty, StructuredMathObject, StructuredNumberType,
    StructuredSetType, StructuredSpaceType, structured_placeholder,
};

use super::expressions::ToStructuredFormat;

impl ToStructuredFormat for MathObject {
    type Output = String;

    fn to_structured(&self) -> Self::Output {
        match self {
            MathObject::Set(_) => "Set".to_string(),
            MathObject::Element(element) => "Element".to_string(),
            MathObject::Group(_) => "Group".to_string(),
            MathObject::Ring(_) => "Ring".to_string(),
            MathObject::Field(_) => "Field".to_string(),
            MathObject::Module(_) => "Module".to_string(),
            MathObject::Algebra(_) => "Algebra".to_string(),
            MathObject::TopologicalSpace(_) => "Topological Space".to_string(),
            MathObject::VectorSpace(_) => "Vector Space".to_string(),
            MathObject::Function(_) => "Function".to_string(),
            MathObject::Integer => "Integer".to_string(),
            MathObject::Rational => "Rational".to_string(),
            MathObject::Irrational => "Irrational".to_string(),
            MathObject::Real => "Real".to_string(),
            MathObject::Complex => "Complex".to_string(),
            MathObject::Morphism(from, to) => "Morphism".to_string(),
            MathObject::Product(_) => "Product".to_string(),
            MathObject::Coproduct(_) => "Coproduct".to_string(),
            MathObject::Todo(description) => format!("Todo: {}", description),
        }
    }
}

impl MathObject {
    pub fn to_structured_math_object(&self) -> StructuredMathObject {
        match self {
            MathObject::Group(group) => StructuredMathObject::Group {
                structure: Box::new(structured_placeholder("Group structure")),
                properties: vec![], // Could be enhanced with actual group properties
            },
            MathObject::Ring(ring) => StructuredMathObject::Ring {
                structure: Box::new(structured_placeholder("Ring structure")),
                properties: vec![],
            },
            MathObject::Field(field) => StructuredMathObject::Field {
                structure: Box::new(structured_placeholder("Field structure")),
                properties: vec![],
            },
            MathObject::Set(set) => StructuredMathObject::Set {
                elements: vec![],                    // Could be enhanced with actual set elements
                set_type: StructuredSetType::Finite, // Default assumption
            },
            MathObject::Element(element) => StructuredMathObject::Element {
                value: Box::new(structured_placeholder("Element value")),
                parent_object: Box::new(element.to_structured_math_object()),
            },
            MathObject::Function(func) => StructuredMathObject::Function {
                domain: Box::new(StructuredMathObject::Set {
                    elements: vec![],
                    set_type: StructuredSetType::Finite,
                }),
                codomain: Box::new(StructuredMathObject::Set {
                    elements: vec![],
                    set_type: StructuredSetType::Finite,
                }),
                definition: None,
            },
            MathObject::TopologicalSpace(space) => StructuredMathObject::Space {
                structure: Box::new(structured_placeholder("Topological space structure")),
                space_type: StructuredSpaceType::Topological,
            },
            MathObject::VectorSpace(space) => StructuredMathObject::Space {
                structure: Box::new(structured_placeholder("Vector space structure")),
                space_type: StructuredSpaceType::Vector,
            },
            MathObject::Integer => StructuredMathObject::NumberType(StructuredNumberType::Integer),
            MathObject::Rational => {
                StructuredMathObject::NumberType(StructuredNumberType::Rational)
            }
            MathObject::Irrational => {
                StructuredMathObject::NumberType(StructuredNumberType::Irrational)
            }
            MathObject::Real => StructuredMathObject::NumberType(StructuredNumberType::Real),
            MathObject::Complex => StructuredMathObject::NumberType(StructuredNumberType::Complex),
            MathObject::Morphism(from, to) => StructuredMathObject::Function {
                domain: Box::new(from.to_structured_math_object()),
                codomain: Box::new(to.to_structured_math_object()),
                definition: None,
            },
            MathObject::Product(objects) => StructuredMathObject::Set {
                elements: objects
                    .iter()
                    .map(|obj| structured_placeholder(&obj.to_structured()))
                    .collect(),
                set_type: StructuredSetType::Finite,
            },
            MathObject::Coproduct(objects) => StructuredMathObject::Set {
                elements: objects
                    .iter()
                    .map(|obj| structured_placeholder(&obj.to_structured()))
                    .collect(),
                set_type: StructuredSetType::Finite,
            },
            MathObject::Module(module) => StructuredMathObject::Set {
                elements: vec![],
                set_type: StructuredSetType::Finite,
            },
            MathObject::Algebra(algebra) => StructuredMathObject::Ring {
                structure: Box::new(structured_placeholder("Algebra structure")),
                properties: vec![],
            },
            MathObject::Todo(description) => StructuredMathObject::Set {
                elements: vec![structured_placeholder(description)],
                set_type: StructuredSetType::Finite,
            },
        }
    }
}
