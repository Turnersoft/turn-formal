use std::hash::{Hash, Hasher};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::extract::Parametrizable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Located<T> {
    pub id: String,
    pub data: Parametrizable<Arc<T>>,
}

impl<T> Located<T> {
    pub fn new(data: Parametrizable<Arc<T>>) -> Self {
        Located {
            id: Uuid::new_v4().to_string(),
            data,
        }
    }

    /// Create a new Located with concrete data
    pub fn new_concrete(data: T) -> Self {
        Located {
            id: Uuid::new_v4().to_string(),
            data: Parametrizable::Concrete(Arc::new(data)),
        }
    }

    /// Create a new Located with variable reference
    pub fn new_variable(id: crate::turn_render::Identifier) -> Self {
        Located {
            id: Uuid::new_v4().to_string(),
            data: Parametrizable::Variable(id),
        }
    }

    /// Create a new Located from an Arc (for backward compatibility)
    pub fn from_arc(arc_data: Arc<T>) -> Self {
        Located {
            id: Uuid::new_v4().to_string(),
            data: Parametrizable::Concrete(arc_data),
        }
    }

    /// Access the wrapped parametrizable value
    pub fn value(&self) -> &Parametrizable<Arc<T>> {
        &self.data
    }

    /// Get the concrete value if it exists, otherwise None
    pub fn concrete_value(&self) -> Option<&Arc<T>> {
        match &self.data {
            Parametrizable::Concrete(arc) => Some(arc),
            Parametrizable::Variable(_) => None,
        }
    }

    /// Get the variable identifier if it exists, otherwise None
    pub fn variable_id(&self) -> Option<&crate::turn_render::Identifier> {
        match &self.data {
            Parametrizable::Concrete(_) => None,
            Parametrizable::Variable(id) => Some(id),
        }
    }
}

impl<T> Eq for Located<T> where T: Eq {}

impl<T> PartialEq for Located<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Hash for Located<T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
