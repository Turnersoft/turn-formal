use crate::formalize_v2::subjects::math::theories::VariantSet;
use crate::formalize_v2::subjects::math::theories::common::spaces::*;
use crate::formalize_v2::subjects::math::theories::zfc::set::Set;
use serde::{Deserialize, Serialize};

/// Topology structure
/// A collection τ of open sets on a space X satisfying:
/// 1. ∅ and X are in τ
/// 2. τ is closed under arbitrary unions
/// 3. τ is closed under finite intersections
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Topology {
    /// Properties of the topology (compactness, connectedness, etc.)
    pub properties: VariantSet<TopologyProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopologyProperty {
    /// Compactness: every open cover has a finite subcover
    /// For any open cover {Uᵢ}ᵢ∈I, ∃ finite J⊆I: X = ⋃ᵢ∈J Uᵢ
    Compactness(CompactnessPropertyVariant),

    /// Connectedness: cannot be split into disjoint open sets
    /// If X = U∪V with U,V open and U∩V=∅, then U=∅ or V=∅
    Connectedness(ConnectednessPropertyVariant),

    /// Boundedness: contained in some "ball"
    /// ∃x₀,r: X ⊆ B(x₀,r)
    Boundedness(TopologicalBoundednessPropertyVariant),

    /// Separability: has countable dense subset
    /// ∃ countable D⊆X: D̄ = X
    Separable(SeparablePropertyVariant),

    /// Countability: cardinality property
    /// |X| ≤ ℵ₀ or |X| > ℵ₀
    Countable(CountablePropertyVariant),

    /// Paracompactness: locally finite refinements exist
    /// Every open cover has a locally finite refinement
    Paracompact(ParacompactPropertyVariant),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParacompactPropertyVariant {
    /// Every open cover has locally finite refinement
    Paracompact,
    /// Not paracompact
    NonParacompact,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CountablePropertyVariant {
    /// Has cardinality ≤ ℵ₀
    Countable,
    /// Has cardinality > ℵ₀
    Uncountable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SeparablePropertyVariant {
    /// Has countable dense subset
    /// ∃ countable D⊆X: D̄ = X
    Separable,
    /// Not separable
    NonSeparable,
}

/// Variants of connectedness for topological spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectednessPropertyVariant {
    /// Space cannot be disconnected by open sets
    /// If X = U∪V with U,V open and U∩V=∅, then U=∅ or V=∅
    Connected(VariantSet<ConnectedProperty>),

    /// Space can be split into disjoint open sets
    /// ∃U,V open: X = U∪V, U∩V=∅, U≠∅, V≠∅
    Disconnected,
}

/// Connectedness property of topological spaces
/// A space X is connected if it cannot be written as a union of two disjoint
/// non-empty open sets. Formally:
/// - If X = U∪V with U,V open and U∩V=∅, then U=∅ or V=∅
/// - This is equivalent to having no proper clopen subsets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectedProperty {
    /// Where the connectedness property holds
    Locality(TopologicalLocalityPropertyVariant),

    /// Extent of connectedness in the space
    Coverage(TopologicalCoveragePropertyVariant),

    /// Type of connectedness (path, arc, local)
    ConnectionType(ConnectionTypePropertyVariant),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionTypePropertyVariant {
    /// Connected by continuous paths
    /// ∀x,y ∃γ:[0,1]→X continuous: γ(0)=x, γ(1)=y
    PathConnected,
    /// Connected by injective continuous paths
    ArcConnected,
    /// Connected in neighborhoods
    LocallyConnected,
    /// Not connected in any way
    NotConnected,
}

/// Variants of compactness for topological spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompactnessPropertyVariant {
    /// Every open cover has a finite subcover
    /// For any open cover {Uᵢ}ᵢ∈I, ∃ finite J⊆I: X = ⋃ᵢ∈J Uᵢ
    Compact(VariantSet<CompactProperty>),

    /// Some open cover has no finite subcover
    /// ∃ open cover with no finite subcover
    NonCompact,
}

/// Compactness property of topological spaces
/// A space is compact if every open cover has a finite subcover.
/// Equivalent characterizations:
/// - Every open cover has a finite subcover
/// - Every net has a convergent subnet
/// - Every ultrafilter is convergent
/// - Every collection of closed sets with FIP has non-empty intersection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompactProperty {
    /// Where the compactness property holds
    Locality(TopologicalLocalityPropertyVariant),

    /// Extent of compactness in the space
    Coverage(TopologicalCoveragePropertyVariant),

    /// Type of compactness (sequential, countable)
    CompactnessType(CompactnessTypePropertyVariant),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompactnessTypePropertyVariant {
    /// Every sequence has convergent subsequence
    /// (xₙ) bounded ⟹ ∃(xₙₖ) convergent
    SequentiallyCompact,
    /// Every countable cover has finite subcover
    CountablyCompact,
    /// Not compact in any sense
    NotCompact,
}

/// A topological space (X,τ) consists of:
/// - A set X of points
/// - A topology τ (collection of open sets)
///
/// The topology τ determines:
/// - Continuity: f: X → Y is continuous iff f⁻¹(V) is open in X for all open V in Y
/// - Convergence: xₙ → x iff every open set containing x contains all but finitely many xₙ
/// - Connectedness: X cannot be written as union of two disjoint non-empty open sets
/// - Compactness: Every open cover has a finite subcover
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopologicalSpace {
    /// The underlying set X of points
    pub base_set: Set,
    /// The topology τ (collection of open sets)
    pub topology: Topology,
    /// Additional properties specific to the space as a whole
    pub properties: Vec<TopologicalSpaceProperty>,
}

/// Properties specific to topological spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopologicalSpaceProperty {
    /// Separation axioms (T0 through T4)
    Separation(SeparationAxiomLevel),

    /// Compactness properties
    Compact(CompactnessType),

    /// Connectedness properties
    Connected(ConnectednessType),

    /// Metrizability properties
    Metrizable(MetrizabilityType),
}

/// Separation axioms in topology form a hierarchy of increasingly strong conditions
/// on how points and closed sets can be separated by open sets.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SeparationAxiomLevel {
    /// T₀ (Kolmogorov): For any two distinct points x,y ∈ X, there exists an open set
    /// containing exactly one of them
    T0,

    /// T₁ (Fréchet): For any two distinct points x,y ∈ X, there exists an open set
    /// containing x but not y
    T1,

    /// T₂ (Hausdorff): Any two distinct points can be separated by disjoint open sets
    T2,

    /// T₂.₅ (Urysohn): Any two distinct points can be separated by a continuous function
    T2_5,

    /// T₃ (Regular Hausdorff): Any point and closed set can be separated by disjoint open sets
    T3,

    /// T₄ (Normal Hausdorff): Any two disjoint closed sets can be separated by disjoint open sets
    T4,
}

/// Types of compactness in topology
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompactnessType {
    /// Every open cover has a finite subcover
    Compact,

    /// Every sequence has a convergent subsequence
    Sequentially,

    /// Every closed and bounded set is compact
    LocallyCompact,

    /// Compact after adding a single point
    OnePointCompactification,
}

/// Types of connectedness in topology
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectednessType {
    /// Cannot be written as union of two disjoint non-empty open sets
    Connected,

    /// Every two points can be joined by a continuous path
    PathConnected,

    /// Every component is a singleton
    TotallyDisconnected,

    /// Connected and compact
    Continuum,
}

/// Types of metrizability in topology
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MetrizabilityType {
    /// Space admits a metric inducing its topology
    Metrizable,

    /// Space admits a complete metric
    CompletelyMetrizable,

    /// Space admits a metric but is not complete
    NonCompletelyMetrizable,
}

/// Properties specific to metric spaces in topology
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopologicalMetricSpaceProperty {
    /// Whether the space is complete
    Complete(bool),
    /// Whether the space is compact
    Compact(bool),
    /// Whether the space is separable
    Separable(bool),
    /// Whether the space is proper (closed balls are compact)
    Proper(bool),
    /// Whether the space is bounded
    Bounded(bool),
    /// Whether the space is totally bounded
    TotallyBounded(bool),
    /// Whether the space is locally compact
    LocallyCompact(bool),
}

/// A metric space is a set M together with a distance function (metric) that defines
/// a notion of distance between elements of the set.
/// The metric must satisfy:
/// 1. d(x,y) ≥ 0 (non-negativity)
/// 2. d(x,y) = 0 iff x = y (identity of indiscernibles)
/// 3. d(x,y) = d(y,x) (symmetry)
/// 4. d(x,z) ≤ d(x,y) + d(y,z) (triangle inequality)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetricSpace {
    /// The underlying set
    pub set: Set,
    /// The metric/distance function
    pub metric: Metric,
    /// Properties of the metric space
    pub properties: VariantSet<TopologicalMetricSpaceProperty>,
}

/// Properties specific to affine spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AffineSpaceProperty {
    /// Dimension of the affine space
    Dimension(u32),
    /// Whether the space is real or complex
    ScalarField(ScalarFieldType),
    /// Whether the space is complete
    Complete(bool),
}

/// Properties specific to fiber bundles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FiberBundleProperty {
    /// Whether the bundle is trivial
    Trivial(bool),
    /// Whether the bundle is locally trivial
    LocallyTrivial(bool),
    /// Whether the bundle is principal
    Principal(bool),
    /// Whether the bundle is vector bundle
    VectorBundle(bool),
}

/// Properties specific to projective spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectiveSpaceProperty {
    /// Dimension of the projective space
    Dimension(u32),
    /// Whether the space is real or complex
    ScalarField(ScalarFieldType),
    /// Whether the space is smooth
    Smooth(bool),
}

/// Types of scalar fields
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScalarFieldType {
    /// Real numbers
    Real,
    /// Complex numbers
    Complex,
    /// Rational numbers
    Rational,
    /// Finite field
    Finite(u32),
}

/// Variants of completeness properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompletenessPropertyVariant {
    /// Complete metric space
    Complete,
    /// Not complete
    Incomplete,
    /// Completable
    Completable,
}

/// Variants of separability properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SeparabilityPropertyVariant {
    /// Separable metric space
    Separable,
    /// Not separable
    NonSeparable,
    /// Second countable
    SecondCountable,
}

/// A metric space completion is a complete metric space that contains the original space as a dense subset
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetricCompletion {
    /// The original metric space
    pub original_space: MetricSpace,
    /// The completion (complete metric space)
    pub completion: MetricSpace,
    /// The inclusion map from original space to completion
    pub inclusion_map: String,
    /// Properties of the completion
    pub properties: VariantSet<CompletionProperty>,
}

/// Properties specific to metric space completions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompletionProperty {
    /// Whether the completion is unique up to isometry
    Unique(bool),
    /// Whether the original space is dense in the completion
    DenseEmbedding(bool),
    /// Whether the completion preserves additional structure (e.g. group structure)
    PreservesStructure(bool),
}

/// Metric structure
/// A function d: X × X → ℝ satisfying:
/// 1. d(x,y) ≥ 0 and d(x,y) = 0 iff x = y (positive definiteness)
/// 2. d(x,y) = d(y,x) (symmetry)
/// 3. d(x,z) ≤ d(x,y) + d(y,z) (triangle inequality)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Metric {
    /// Properties of the metric (completeness, boundedness, etc.)
    pub properties: VariantSet<MetricProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MetricProperty {
    /// Continuity of the metric function
    /// (x,y) ↦ d(x,y) is continuous from X×X to ℝ
    Continuity(TopologicalContinuityPropertyVariant),

    /// Boundedness of distances
    /// ∃M>0: d(x,y) ≤ M for all x,y
    Boundedness(TopologicalBoundednessPropertyVariant),

    /// Geodesic property: existence of length-minimizing paths
    /// ∃ path γ: d(x,y) = length(γ) connecting x to y
    Geodesic(GeodesicPropertyVariant),

    /// Length space property: distance as infimum of path lengths
    /// d(x,y) = inf{length(γ): γ connects x to y}
    LengthSpace(LengthSpacePropertyVariant),

    /// Isometry group: symmetries preserving distance
    /// Group of maps T: d(Tx,Ty) = d(x,y)
    IsometryGroup(IsometryGroupPropertyVariant),

    /// Uniform structure: compatible uniformity
    /// Topology induced by d agrees with given uniform structure
    UniformStructure(UniformStructurePropertyVariant),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UniformStructurePropertyVariant {
    /// Has compatible uniform structure
    /// Topology from uniformity agrees with given topology
    Uniform,
    /// No compatible uniform structure
    NonUniform,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IsometryGroupPropertyVariant {
    /// Has non-trivial isometry group
    /// ∃T≠id: d(Tx,Ty) = d(x,y) for all x,y
    HasIsometryGroup,
    /// No non-trivial isometries
    NoIsometryGroup,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LengthSpacePropertyVariant {
    /// Distance is infimum of path lengths
    /// d(x,y) = inf{length(γ): γ connects x to y}
    LengthSpace,
    /// Not a length space
    NonLengthSpace,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GeodesicPropertyVariant {
    /// Distance realized by length of path
    /// ∃ path γ: d(x,y) = length(γ)
    Geodesic,
    /// Not geodesic
    NonGeodesic,
}

/// Variants of boundedness for topological spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopologicalBoundednessPropertyVariant {
    /// Space is bounded in some metric
    /// ∃ metric d, x₀,r: X ⊆ B(x₀,r)
    Bounded(VariantSet<TopologicalBoundedProperty>),

    /// Space is not bounded in any metric
    /// ∀ metric d, x₀,r: X ⊈ B(x₀,r)
    Unbounded,
}

/// Properties modifying bounded topological spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopologicalBoundedProperty {
    /// Where the boundedness holds
    Locality(TopologicalLocalityPropertyVariant),

    /// Extent of boundedness
    Coverage(TopologicalCoveragePropertyVariant),

    /// Type of boundedness
    BoundednessType(TopologicalBoundednessTypeVariant),
}

/// Types of boundedness in topology
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopologicalBoundednessTypeVariant {
    /// Bounded in metric sense
    MetricBounded,
    /// Bounded in measure sense
    MeasureBounded,
    /// Bounded in order sense
    OrderBounded,
}

/// Variants of continuity for topological spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopologicalContinuityPropertyVariant {
    /// Function preserves topology
    /// Inverse images of open sets are open
    Continuous(VariantSet<TopologicalContinuousProperty>),

    /// Function does not preserve topology
    /// Some inverse image of open set is not open
    Discontinuous,
}

/// Properties modifying continuous functions in topology
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopologicalContinuousProperty {
    /// Where the continuity holds
    Locality(TopologicalLocalityPropertyVariant),

    /// Extent of continuity
    Coverage(TopologicalCoveragePropertyVariant),

    /// Type of continuity
    ContinuityType(TopologicalContinuityTypeVariant),
}

/// Types of continuity in topology
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopologicalContinuityTypeVariant {
    /// Continuous everywhere
    Continuous,
    /// Continuous at a point
    ContinuousAtPoint,
    /// Uniformly continuous
    UniformlyContinuous,
}

/// Locality variants for topological properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopologicalLocalityPropertyVariant {
    /// Property holds on entire space
    Global,
    /// Property holds on open neighborhoods
    Local,
    /// Property holds at points
    PointWise,
    /// Property holds on compact sets
    OnCompact,
}

/// Coverage variants for topological properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopologicalCoveragePropertyVariant {
    /// Property holds everywhere
    Complete,
    /// Property holds on dense subset
    Dense,
    /// Property holds on open subset
    Open,
    /// Property holds on closed subset
    Closed,
}
