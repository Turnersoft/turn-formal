use std::fmt::Debug;
use std::{
    any::{Any, TypeId},
    hash::{Hash, Hasher},
    sync::Arc,
};

use serde::{Deserialize, Serialize};

use crate::turn_render::Identifier;

use super::detag::TryDetag;
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

                TryDetag::<T>::detag(math_expr).clone()
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
