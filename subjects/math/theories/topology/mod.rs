pub mod collect;
pub mod definitions;
pub mod relations;

// Re-export main types and traits
pub use definitions::{Metric, MetricCompletion, MetricSpace, TopologicalSpace, Topology};

// Re-export property variants
pub use definitions::*;

// Export the public modules and types
pub use relations::TopologyRelation;
