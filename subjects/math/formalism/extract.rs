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

impl<T: Hash> Hash for Parametrizable<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Parametrizable::Concrete(t) => t.hash(state),
            Parametrizable::Variable(id) => id.hash(state),
        }
    }
}

impl<T: 'static + Clone + Debug> Parametrizable<Arc<T>> {
    pub fn unwrap_arc(&self, context: &Vec<ContextEntry>) -> Arc<T> {
        match self {
            Parametrizable::Concrete(arc_t) => arc_t.clone(),
            Parametrizable::Variable(id) => {
                let math_expr = &context
                    .iter()
                    .find(|entry| entry.name == *id)
                    .unwrap_or_else(|| panic!("Variable with id {:?} not found in context", id))
                    .ty
                    .data
                    .unwrap_arc(context);

                // Try to get Arc<T> directly
                match TryDetag::<Arc<T>>::try_detag(math_expr) {
                    Ok(result) => result.clone(),
                    Err(_) => {
                        // If that fails, try to get T and wrap it in Arc
                        match TryDetag::<T>::try_detag(math_expr) {
                            Ok(inner) => Arc::new(inner.clone()),
                            Err(e) => panic!(
                                "Could not extract {} or Arc<{}> from context: {}",
                                std::any::type_name::<T>(),
                                std::any::type_name::<T>(),
                                e
                            ),
                        }
                    }
                }
            }
        }
    }

    pub fn unwrap(&self, context: &Vec<ContextEntry>) -> T {
        self.unwrap_arc(context).as_ref().clone()
    }

    /// Create a concrete parametrizable from a value
    pub fn concrete(value: T) -> Self {
        Parametrizable::Concrete(Arc::new(value))
    }

    /// Create a variable parametrizable from an identifier
    pub fn variable(id: Identifier) -> Self {
        Parametrizable::Variable(id)
    }
}
