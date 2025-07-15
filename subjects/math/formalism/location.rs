use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Located<T> {
    pub id: String,
    pub data: T,
}

impl<T> Located<T> {
    pub fn new(data: T) -> Self {
        Located {
            id: Uuid::new_v4().to_string(),
            data,
        }
    }

    /// Access the wrapped value
    pub fn value(&self) -> &T {
        &self.data
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
