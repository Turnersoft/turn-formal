use std::any::Any;

use serde::{Deserialize, Serialize};

use super::{
    expressions::{Identifier, MathExpression},
    theorem::MathObject,
};

/// Generic wrapper to allow a field to hold either a concrete value
/// or a reference to a variable defined in the context.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Parametrizable<T> {
    Concrete(T),
    Variable(Identifier),
}

pub trait Extractable {
    fn extract<T: 'static + Clone>(&self) -> Option<T>;
}

impl<T: Clone> Parametrizable<T> {
    pub fn unwrap(&self) -> T {
        match self {
            Parametrizable::Concrete(t) => t.clone(),
            Parametrizable::Variable(_) => panic!("Cannot unwrap a variable"),
        }
    }
}

/// Macro for implementing type extraction with pattern matching
#[macro_export]
macro_rules! extract_as {
    // Extract directly from a value
    ($val:expr, $target_type:ty) => {
        ($val as &dyn std::any::Any).downcast_ref::<$target_type>().cloned()
    };

    // Try extracting directly, then from inner structure
    ($val:expr, $target_type:ty, try_extract) => {
        ($val as &dyn std::any::Any)
            .downcast_ref::<$target_type>()
            .cloned()
            .or_else(|| $val.extract::<$target_type>())
    };

    // Try multiple structures with OR (assuming T is in scope from calling fn)
    ($val:expr, $($rest:expr),+) => {
        $val.extract::<T>()$(
            .or_else(|| $rest.extract::<T>())
        )+
    };
}

/// Macro for implementing Extractable trait for expression types
/// Automatically implements check for self as the target type and field extractions
macro_rules! impl_extractable {
    // Basic implementation with just fields to extract from
    ($type:ty, $($field:ident),* $(,)?) => {
        impl Extractable for $type {
            fn extract<T: 'static + Any + Clone>(&self) -> Option<T> {
                if let Some(t) = (self as &dyn Any).downcast_ref::<T>() {
                    return Some(t.clone());
                }
                $(
                    if let Some(extracted) = self.$field.extract::<T>() {
                        return Some(extracted);
                    }
                )*
                None
            }
        }
    };

    // Implementation with match expression
    ($type:ty, match $self:ident {
        $($variant:pat => $extract_expr:expr),* $(,)?
    }) => {
        impl Extractable for $type {
            fn extract<T: 'static + Any + Clone>(&self) -> Option<T> {
                if let Some(t) = (self as &dyn Any).downcast_ref::<T>() {
                    return Some(t.clone());
                }

                let $self = self;
                match $self {
                    $($variant => $extract_expr),*
                }
            }
        }
    };
}

impl Extractable for MathExpression {
    fn extract<T: 'static + Clone>(&self) -> Option<T> {
        match self {
            MathExpression::Var(_) => None,
            MathExpression::Object(math_object) => math_object.extract::<T>(),
            MathExpression::Expression(theory_expression) => todo!(),
            MathExpression::Relation(_) => None,
            MathExpression::Number(number) => extract_as!(number, T),
            MathExpression::ViewAs { expression, .. } => expression.extract::<T>(),
        }
    }
}

impl Extractable for MathObject {
    fn extract<T: 'static + Clone>(&self) -> Option<T> {
        match self {
            MathObject::Group(g) => g.extract(),
            MathObject::Ring(r) => extract_as!(r, T),
            MathObject::Field(f) => extract_as!(f, T),
            MathObject::Module(m) => extract_as!(m, T),
            MathObject::Algebra(a) => extract_as!(a, T),
            MathObject::TopologicalSpace(ts) => extract_as!(ts, T),
            MathObject::VectorSpace(vs) => extract_as!(vs, T),
            MathObject::Set(s) => extract_as!(s, T),
            MathObject::Function(func) => extract_as!(func, T),
            MathObject::Element(inner_obj) => inner_obj.extract::<T>(),
            MathObject::Morphism(dom, cod) => dom.extract::<T>().or_else(|| cod.extract::<T>()),
            MathObject::Product(objs) | MathObject::Coproduct(objs) => {
                objs.iter().find_map(|o| o.extract::<T>())
            }
            MathObject::Integer
            | MathObject::Rational
            | MathObject::Irrational
            | MathObject::Real
            | MathObject::Complex
            | MathObject::Todo(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::subjects::math::theories::groups::definitions::Group;
    use crate::subjects::math::theories::groups::definitions::GroupBasic;
    use crate::subjects::math::theories::rings::definitions::Ring;

    use super::*;

    #[test]
    fn test_extract_math_object() {
        let group = Group::Basic(GroupBasic::default());
        let math_object = MathObject::Group(group.clone());
        let extracted = math_object.extract::<Group>();
        assert_eq!(extracted, Some(group));
    }

    #[test]
    fn test_extract_with_macro() {
        let group = Group::Basic(GroupBasic::default());
        let math_object = MathObject::Group(group.clone());

        // Test with the extract_as macro (via its usage in MathObject impl)
        let extracted_group: Option<Group> = math_object.extract();
        assert_eq!(extracted_group, Some(group));

        // Test extracting a type that's not present
        let extracted_ring: Option<Ring> = math_object.extract();
        assert_eq!(extracted_ring, None);
    }
}
