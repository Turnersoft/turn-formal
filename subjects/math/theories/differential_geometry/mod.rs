pub mod definitions;

// Re-export main types and traits
pub use definitions::{
    BundleSection, Connection, FiberBundle, RiemannianManifold, SmoothManifold, VectorBundle,
};

// Re-export property variants
pub use definitions::{
    BoundaryType, CompletenessType, ComplexStructureType, ConnectionProperty, CurvatureType,
    EinsteinType, FiberBundleProperty, KahlerType, OrientabilityType, ParallelizabilityType,
    RiemannianManifoldProperty, SectionType, SmoothManifoldProperty, VectorBundleProperty,
};
