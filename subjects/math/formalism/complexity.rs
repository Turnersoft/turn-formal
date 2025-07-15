use crate::turn_render::Identifier;

use super::{
    expressions::{MathExpression, TheoryExpression},
    location::Located,
    objects::MathObject,
    relations::MathRelation,
};

pub trait Complexity {
    fn complexity(&self) -> usize;
}

// Implement Complexity for basic types that might be parameters
impl Complexity for String {
    fn complexity(&self) -> usize {
        1 + self.len() / 4 // Simple complexity for strings
    }
}

impl Complexity for usize {
    fn complexity(&self) -> usize {
        1
    }
}

impl Complexity for i32 {
    fn complexity(&self) -> usize {
        1
    }
}

impl Complexity for u32 {
    fn complexity(&self) -> usize {
        1
    }
}

impl Complexity for bool {
    fn complexity(&self) -> usize {
        1
    }
}

// MathObject complexity (can be detailed based on variants)
impl Complexity for MathObject {
    fn complexity(&self) -> usize {
        match self {
            MathObject::Group(g) => g.complexity(), // Assuming Group implements Complexity
            MathObject::Ring(r) => 1,               // Placeholder
            MathObject::Field(f) => 1,              // Placeholder
            MathObject::Module(m) => 1,             // Placeholder
            MathObject::Algebra(a) => 1,            // Placeholder
            MathObject::TopologicalSpace(ts) => 1,  // Placeholder
            MathObject::VectorSpace(vs) => 1,       // Placeholder
            MathObject::Set(s) => 1,                // Placeholder
            MathObject::Function(f) => 1,           // Placeholder
        }
    }
}

impl Complexity for Identifier {
    fn complexity(&self) -> usize {
        todo!()
    }
}

impl Complexity for TheoryExpression {
    fn complexity(&self) -> usize {
        match self {
            TheoryExpression::Group(ge) => 1 + ge.complexity(),
            TheoryExpression::Ring(re) => 1 + 0, // Assuming RingExpression complexity will be added
            TheoryExpression::Field(fe) => 1 + 0, // Assuming FieldExpression complexity will be added
        }
    }
}

impl Complexity for MathExpression {
    fn complexity(&self) -> usize {
        match self {
            // MathExpression::Var(id) => id.complexity(),
            MathExpression::Object(obj) => 1 + 0, // Assuming MathObject complexity will be added
            MathExpression::Expression(te) => 1 + te.complexity(),
            MathExpression::Relation(rel) => 1 + rel.complexity(),
            MathExpression::Number(_) => 1, // Simple number
            MathExpression::ViewAs { expression, view } => {
                1 + expression.data.complexity() + 0 // Assuming TypeViewOperator complexity
            }
        }
    }
}

impl Complexity for MathRelation {
    fn complexity(&self) -> usize {
        match self {
            MathRelation::Equal { left, right, .. } => {
                1 + left.data.complexity() + right.data.complexity()
            }
            MathRelation::And(relations) => {
                1 + relations.iter().map(|r| r.data.complexity()).sum::<usize>()
            }
            MathRelation::Or(relations) => {
                1 + relations.iter().map(|r| r.data.complexity()).sum::<usize>()
            }
            MathRelation::Implies(a, b) => 1 + a.data.complexity() + b.data.complexity(),
            MathRelation::Equivalent(a, b) => 1 + a.data.complexity() + b.data.complexity(),
            MathRelation::Not(r) => 1 + r.data.complexity(),
            MathRelation::True => 1,
            MathRelation::False => 1,
            MathRelation::NumberTheory(_) => 1,
            MathRelation::SetTheory(_) => 1,
            MathRelation::GroupTheory(_) => 1,
            MathRelation::RingTheory(_) => 1,
            MathRelation::TopologyTheory(_) => 1,
            MathRelation::CategoryTheory(_) => 1,
            MathRelation::ProbabilityTheory(_) => 1,
            _ => 1,
        }
    }
}

// impl<T: Complexity> Complexity for Located<T> {
//     fn complexity(&self) -> usize {
//         self.data.complexity()
//     }
// }
