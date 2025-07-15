pub mod definitions;
pub mod search;

// Re-export property variants
pub use definitions::{
    CompactnessPropertyVariant, ConnectednessPropertyVariant, CountablePropertyVariant,
    ParacompactPropertyVariant, SeparablePropertyVariant, TopologicalBoundednessPropertyVariant,
};
