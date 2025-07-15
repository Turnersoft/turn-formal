use super::super::super::super::math::theories::{
    affine_geometry::definitions::AffineSpace,
    algebraic_geometry::definitions::{Scheme, Variety},
    analysis::definition::spaces::{
        BanachSpace, DistributionSpace, FrechetSpace, FunctionSpace, HilbertSpace,
        LocallyConvexSpace, SobolevSpace,
    },
    differential_geometry::definitions::{FiberBundle, RiemannianManifold, SmoothManifold},
    groups::definitions::{Group, LieGroup, TopologicalGroup},
    linear_algebra::definitions::{InnerProductSpace, NormedSpace, VectorSpace},
    measure::definitions::{LpSpace, MeasurableSpace, MeasureSpace},
    projective_geometry::definitions::ProjectiveSpace,
    rings::definitions::{Field, Ring},
    symplectic_geometry::definitions::SymplecticManifold,
    topology::definitions::{MetricSpace, TopologicalSpace},
    zfc::definitions::Set,
};

use serde::{Deserialize, Serialize};

/// Registry of all mathematical spaces
/// Each space type is defined in its respective theory folder
/// This enum serves as a unified type for referencing any space
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Space {
    /// Basic set without additional structure
    Set(Set),

    /// Topological space (topology/definitions.rs)
    TopologicalSpace(TopologicalSpace),

    /// Metric space (topology/definitions.rs)
    MetricSpace(MetricSpace),

    /// Smooth manifold (differential_geometry/definitions.rs)
    SmoothManifold(SmoothManifold),

    /// Riemannian manifold (differential_geometry/definitions.rs)
    RiemannianManifold(RiemannianManifold),

    /// Fiber bundle (differential_geometry/definitions.rs)
    FiberBundle(FiberBundle),

    /// Symplectic manifold (symplectic_geometry/definitions.rs)
    SymplecticManifold(SymplecticManifold),

    /// Vector space (linear_algebra/definitions.rs)
    VectorSpace(VectorSpace),

    /// Normed vector space (linear_algebra/definitions.rs)
    NormedSpace(NormedSpace),

    /// Inner product space (linear_algebra/definitions.rs)
    InnerProductSpace(InnerProductSpace),

    /// Function space (analysis/definitions.rs)
    FunctionSpace(FunctionSpace),

    /// Measurable space (measure/definitions.rs)
    MeasurableSpace(MeasurableSpace),

    /// Measure space (measure/definitions.rs)
    MeasureSpace(MeasureSpace),

    /// Lp space (measure/definitions.rs)
    LpSpace(LpSpace),

    /// Sobolev space (analysis/definitions.rs)
    SobolevSpace(SobolevSpace),

    /// Distribution space (analysis/definitions.rs)
    DistributionSpace(DistributionSpace),

    /// Affine space (affine_geometry/definitions.rs)
    AffineSpace(AffineSpace),

    /// Projective space (projective_geometry/definitions.rs)
    ProjectiveSpace(ProjectiveSpace),

    /// Group (groups/definitions.rs)
    Group(Group),

    /// Topological group (groups/definitions.rs)
    TopologicalGroup(TopologicalGroup),

    /// Lie group (groups/definitions.rs)
    LieGroup(LieGroup),

    /// Ring (rings/definitions.rs)
    Ring(Ring),

    /// Field (rings/definitions.rs)
    Field(Field),

    /// Scheme (algebraic_geometry/definitions.rs)
    Scheme(Scheme),

    /// Variety (algebraic_geometry/definitions.rs)
    Variety(Variety),

    /// Banach space (analysis/definitions.rs)
    BanachSpace(BanachSpace),

    /// Hilbert space (analysis/definitions.rs)
    HilbertSpace(HilbertSpace),

    /// Fréchet space (analysis/definitions.rs)
    FrechetSpace(FrechetSpace),

    /// Locally convex space (analysis/definitions.rs)
    LocallyConvexSpace(LocallyConvexSpace),
}

/// Types of dimensions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DimensionType {
    /// Zero dimensional
    Zero,
    /// Finite dimensional
    Finite(u32),
    /// Countably infinite dimensional
    CountablyInfinite,
    /// Uncountably infinite dimensional
    Uncountable,
}
