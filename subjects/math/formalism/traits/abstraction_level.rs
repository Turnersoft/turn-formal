use std::sync::Arc;

use serde::{Deserialize, Serialize};

pub trait GetAbstractionLevel {
    fn level(&self) -> AbstractionLevel;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum AbstractionLevel {
    Level1, // Abstract schema
    Level2, // Specific type
    Level3, // Constructor
    Level4, // Concrete instance
}

impl<T: GetAbstractionLevel> GetAbstractionLevel for Arc<T> {
    fn level(&self) -> AbstractionLevel {
        self.as_ref().level()
    }
}

use crate::subjects::math::formalism::location::Located;
impl<T: GetAbstractionLevel> GetAbstractionLevel for Located<T> {
    fn level(&self) -> AbstractionLevel {
        self.data.level()
    }
}
