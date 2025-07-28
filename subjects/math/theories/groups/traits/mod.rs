pub mod abstraction_level;
pub mod collect_identifier;
pub mod complexity;
pub mod detag;
pub mod replace;
pub mod search;

// Re-export commonly used traits from formalism
pub use crate::subjects::math::formalism::traits::{
    AbstractionLevel, Complexity, GetAbstractionLevel, IsCompatible, Replace, Search,
    Substitutable, TryDetag,
};
