pub mod abstraction_level;
pub mod collect_identifier;
pub mod complexity;
pub mod debug;
pub mod detag;
pub mod instantiable;
pub mod is_compatible;
pub use is_compatible::SameRole;
pub mod replace;
pub mod search;
pub mod substitutable;

// Re-export commonly used traits
pub use abstraction_level::{AbstractionLevel, GetAbstractionLevel};
pub use complexity::Complexity;
pub use debug::ShortDebug;
pub use detag::TryDetag;
pub use instantiable::Instantiable;
pub use is_compatible::IsCompatible;
pub use replace::Replace;
pub use search::Search;
pub use substitutable::Substitutable;
