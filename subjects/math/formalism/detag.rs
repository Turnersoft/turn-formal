use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

use super::expressions::{MathExpression, TheoryExpression};
use super::extract::Parametrizable;
use super::location::Located;
use super::objects::MathObject;
use super::relations::MathRelation;
use crate::subjects::math::theories::fields::definitions::Field;
use crate::subjects::math::theories::groups::definitions::{
    Group, GroupElement, GroupExpression, GroupHomomorphism,
};
use crate::subjects::math::theories::number_theory::definitions::Number;
use crate::subjects::math::theories::rings::definitions::{FieldExpression, Ring, RingExpression};
use crate::subjects::math::theories::zfc::definitions::Set;

// --- Generic Getter Trait & Ad-Hoc Checker ---

/// A utility macro for ad-hoc, inline type checking.
#[macro_export]
macro_rules! try_detag_as {
    ($val:expr, $T:ty) => {
        ($val as &dyn Any).downcast_ref::<$T>().ok_or_else(|| {
            format!(
                "Could not get type `{}` from an expression containing `{}`",
                std::any::type_name::<$T>(),
                std::any::type_name_of_val($val)
            )
        })
    };
}

// --- Implementations for Terminal Types (that don't recurse further) ---
#[macro_export]
macro_rules! impl_try_get_for_terminal_type {
    ($type:ty) => {
        impl<T: 'static + Debug> TryDetag<T> for $type {
            fn try_detag(&self) -> Result<&T, String> {
                try_detag_as!(self, T)
            }
        }
    };
}

/// A generic trait for recursively digging into nested expression types to find a value of a specific type `T`.
pub trait TryDetag<T: 'static + Debug> {
    fn try_detag(&self) -> Result<&T, String>;
}

// --- Manual Implementations ---

// Implement for wrapper types that just pass the call down.
impl<T, U> TryDetag<U> for Arc<T>
where
    T: TryDetag<U> + 'static,
    U: 'static + Debug,
{
    fn try_detag(&self) -> Result<&U, String> {
        use std::any::{Any, TypeId};

        // Case 1: The caller wants the Arc<T> itself.
        if TypeId::of::<U>() == TypeId::of::<Arc<T>>() {
            let any_self: &dyn Any = self;
            // This downcast should always succeed.
            return any_self
                .downcast_ref::<U>()
                .ok_or_else(|| "downcast failed".to_string());
        }

        // Case 2: The caller wants the inner T.
        if TypeId::of::<U>() == TypeId::of::<T>() {
            let any_inner: &dyn Any = self.as_ref();
            // This downcast should always succeed.
            return any_inner
                .downcast_ref::<U>()
                .ok_or_else(|| "downcast failed".to_string());
        }

        // Case 3: The caller wants something inside T. Delegate.
        self.as_ref().try_detag()
    }
}

impl<T, U> TryDetag<U> for Box<T>
where
    T: TryDetag<U>,
    U: 'static + Debug,
{
    fn try_detag(&self) -> Result<&U, String> {
        self.as_ref().try_detag()
    }
}

impl<T, U> TryDetag<U> for Parametrizable<T>
where
    T: TryDetag<U> + 'static + Debug,
    U: 'static + Debug,
{
    fn try_detag(&self) -> Result<&U, String> {
        match self {
            Parametrizable::Concrete(concrete) => concrete.try_detag(),
            Parametrizable::Variable(id) => try_detag_as!(id, U),
        }
    }
}

impl<T, U> TryDetag<U> for Located<T>
where
    T: 'static + Debug,
    U: 'static + Debug,
{
    fn try_detag(&self) -> Result<&U, String> {
        use std::any::{Any, TypeId};

        // Case 1: The caller wants the Located<T> itself.
        if TypeId::of::<U>() == TypeId::of::<Located<T>>() {
            let any_self: &dyn Any = self;
            return any_self
                .downcast_ref::<U>()
                .ok_or_else(|| "downcast failed for Located<T>".to_string());
        }

        // Case 2: The caller wants the inner T (if it's concrete).
        if TypeId::of::<U>() == TypeId::of::<T>() {
            match &self.data {
                Parametrizable::Concrete(arc_concrete) => {
                    let any_inner: &dyn Any = arc_concrete.as_ref();
                    return any_inner
                        .downcast_ref::<U>()
                        .ok_or_else(|| "downcast failed for concrete data".to_string());
                }
                Parametrizable::Variable(_) => {
                    return Err("Cannot extract concrete type from variable".to_string());
                }
            }
        }

        // Case 3: Try direct conversion from Arc<T> content to U
        match &self.data {
            Parametrizable::Concrete(arc_concrete) => {
                let any_arc_content: &dyn Any = arc_concrete.as_ref();
                any_arc_content.downcast_ref::<U>().ok_or_else(|| {
                    format!(
                        "Cannot extract {} from Located<{}>",
                        std::any::type_name::<U>(),
                        std::any::type_name::<T>()
                    )
                })
            }
            Parametrizable::Variable(id) => try_detag_as!(id, U),
        }
    }
}

// --- Implementations for Core Recursive Enums ---

impl<T: 'static + Debug> TryDetag<T> for MathObject {
    fn try_detag(&self) -> Result<&T, String> {
        match self {
            MathObject::Group(g) => try_detag_as!(g, T),
            MathObject::Ring(r) => try_detag_as!(r, T),
            MathObject::Field(f) => try_detag_as!(f, T),
            // The other variants from objects.rs need to be added here if they are used.
            _ => Err(format!(
                "TryGet not implemented for this MathObject variant to find {}",
                std::any::type_name::<T>()
            )),
        }
    }
}

impl<T: 'static + Debug> TryDetag<T> for MathExpression {
    fn try_detag(&self) -> Result<&T, String> {
        if let Ok(res) = try_detag_as!(self, T) {
            return Ok(res);
        }
        match self {
            MathExpression::Object(obj) => obj.try_detag(),
            MathExpression::Expression(expr) => expr.try_detag(),
            MathExpression::Relation(rel) => rel.try_detag(),
            MathExpression::Number(num) => try_detag_as!(num, T),
            MathExpression::ViewAs { expression, .. } => expression.data.try_detag(),
        }
    }
}

impl<T: 'static + Debug> TryDetag<T> for MathRelation {
    fn try_detag(&self) -> Result<&T, String> {
        if let Ok(res) = try_detag_as!(self, T) {
            return Ok(res);
        }
        match self {
            MathRelation::GroupTheory(rel) => try_detag_as!(rel, T),
            MathRelation::NumberTheory(rel) => todo!(),
            MathRelation::SetTheory(rel) => todo!(),
            MathRelation::RingTheory(ring_relation) => todo!(),
            MathRelation::TopologyTheory(topology_relation) => todo!(),
            MathRelation::CategoryTheory(category_relation) => todo!(),
            MathRelation::ProbabilityTheory(probability_relation) => todo!(),
            _ => Err(format!(
                "TryGet not implemented for this MathRelation variant to find {}",
                std::any::type_name::<T>()
            )),
        }
    }
}

impl<T: 'static + Debug> TryDetag<T> for TheoryExpression {
    fn try_detag(&self) -> Result<&T, String> {
        if let Ok(res) = try_detag_as!(self, T) {
            return Ok(res);
        }
        match self {
            TheoryExpression::Group(g) => try_detag_as!(g, T),
            TheoryExpression::Ring(r) => todo!(),
            TheoryExpression::Field(f) => todo!(),
            _ => Err(format!(
                "TryGet not implemented for this TheoryExpression variant to find {}",
                std::any::type_name::<T>()
            )),
        }
    }
}
