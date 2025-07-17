use std::fmt::Debug;
use std::{
    any::{Any, TypeId},
    hash::{Hash, Hasher},
    sync::Arc,
};

use serde::{Deserialize, Serialize};

use crate::turn_render::Identifier;

use super::{
    expressions::{MathExpression, TheoryExpression},
    location::Located,
    objects::MathObject,
    proof::ContextEntry,
    relations::MathRelation,
};

/// Generic wrapper to allow a field to hold either a concrete value
/// or a reference to a variable defined in the context.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Parametrizable<T> {
    Concrete(T),
    Variable(Identifier),
}

impl<T: 'static + Clone + Debug> Parametrizable<T> {
    pub fn unwrap(&self, context: &Vec<ContextEntry>) -> T {
        match self {
            Parametrizable::Concrete(t) => t.clone(),
            Parametrizable::Variable(id) => {
                let math_expr = &context
                    .iter()
                    .find(|entry| entry.name == *id)
                    .unwrap_or_else(|| panic!("Variable with id {:?} not found in context", id))
                    .ty
                    .data;

                math_expr.extract::<T>()
            }
        }
    }
}

impl<T: Hash> Hash for Parametrizable<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Parametrizable::Concrete(t) => t.hash(state),
            Parametrizable::Variable(id) => id.hash(state),
        }
    }
}

pub trait Extractable: Debug {
    fn try_extract<T: 'static + Clone>(&self) -> Option<T>;
    fn extract<T: 'static + Clone>(&self) -> T
    where
        Self: Sized + 'static,
    {
        // check if the current type is the same as the target type
        if let Some(val) = (self as &dyn Any).downcast_ref::<T>() {
            return val.clone();
        }
        // if not, try to extract the target type from the current type
        self.try_extract().unwrap_or_else(|| {
            panic!(
                "Could not extract type {} from expression {:?}",
                std::any::type_name::<T>(),
                self
            )
        })
    }
}

impl<T: Extractable> Extractable for Located<T> {
    fn try_extract<U: 'static + Clone>(&self) -> Option<U> {
        self.data.try_extract::<U>()
    }
}

impl Extractable for MathExpression {
    fn try_extract<T: 'static + Clone>(&self) -> Option<T> {
        // Special case: if T is Arc<MathExpression>, wrap self in Arc
        if std::any::TypeId::of::<T>() == std::any::TypeId::of::<Arc<MathExpression>>() {
            let arc_expr = Arc::new(self.clone());
            return ((&arc_expr as &dyn Any).downcast_ref::<T>()).cloned();
        }

        match self {
            MathExpression::Object(math_object) => math_object.try_extract::<T>(),
            MathExpression::Expression(theory_expression) => theory_expression.try_extract::<T>(),
            _ => (self as &dyn Any).downcast_ref::<T>().cloned(),
        }
    }
}

// Generic implementation for Arc<T> where T: Extractable
impl<T: Extractable> Extractable for Arc<T> {
    fn try_extract<U: 'static + Clone>(&self) -> Option<U> {
        (**self).try_extract::<U>()
    }
}

impl Extractable for MathObject {
    fn try_extract<T: 'static + Clone>(&self) -> Option<T> {
        macro_rules! extract_variant {
            ($val:expr) => {
                ($val as &dyn Any).downcast_ref::<T>().cloned()
            };
        }
        match self {
            MathObject::Group(g) => extract_variant!(g),
            MathObject::Ring(r) => extract_variant!(r),
            MathObject::Field(f) => extract_variant!(f),
            MathObject::Module(m) => extract_variant!(m),
            MathObject::Algebra(a) => extract_variant!(a),
            MathObject::TopologicalSpace(ts) => extract_variant!(ts),
            MathObject::VectorSpace(vs) => extract_variant!(vs),
            MathObject::Set(s) => extract_variant!(s),
            MathObject::Function(func) => extract_variant!(func),
        }
    }
}

impl Extractable for MathRelation {
    fn try_extract<T: 'static + Clone>(&self) -> Option<T> {
        macro_rules! extract_variant {
            ($val:expr) => {
                ($val as &dyn Any).downcast_ref::<T>().cloned()
            };
        }
        match self {
            MathRelation::And(locateds) => todo!(),
            MathRelation::Or(locateds) => todo!(),
            MathRelation::Not(located) => todo!(),
            MathRelation::Implies(located, located1) => todo!(),
            MathRelation::Equivalent(located, located1) => todo!(),
            MathRelation::True => todo!(),
            MathRelation::False => todo!(),
            MathRelation::NumberTheory(number_theory_relation) => todo!(),
            MathRelation::SetTheory(set_relation) => todo!(),
            MathRelation::GroupTheory(group_relation) => todo!(),
            MathRelation::RingTheory(ring_relation) => todo!(),
            MathRelation::TopologyTheory(topology_relation) => todo!(),
            MathRelation::CategoryTheory(category_relation) => todo!(),
            MathRelation::ProbabilityTheory(probability_relation) => todo!(),
            MathRelation::Equal { left, right } => todo!(),
        }
    }
}

impl Extractable for TheoryExpression {
    fn try_extract<T: 'static + Clone>(&self) -> Option<T> {
        macro_rules! extract_variant {
            ($val:expr) => {
                ($val as &dyn Any).downcast_ref::<T>().cloned()
            };
        }
        match self {
            TheoryExpression::Group(group) => extract_variant!(group),
            TheoryExpression::Ring(ring) => extract_variant!(ring),
            TheoryExpression::Field(field) => extract_variant!(field),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::subjects::math::theories::groups::definitions::GenericGroup;
    use crate::subjects::math::theories::groups::definitions::Group;
    use crate::subjects::math::theories::rings::definitions::Ring;

    use super::*;

    #[test]
    fn test_extract_from_math_expression() {
        let group = Group::Generic(GenericGroup::default());
        let math_object = MathObject::Group(group.clone());
        let expr = MathExpression::Object(Arc::new(math_object));
        let extracted = expr.extract::<Group>();
        assert_eq!(extracted, group);
    }

    #[test]
    #[should_panic]
    fn test_extract_panic() {
        let group = Group::Generic(GenericGroup::default());
        let math_object = MathObject::Group(group.clone());
        let expr = MathExpression::Object(Arc::new(math_object));
        expr.extract::<Ring>();
    }
}
