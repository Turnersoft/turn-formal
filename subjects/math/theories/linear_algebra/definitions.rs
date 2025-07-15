use super::super::super::super::math::theories::VariantSet;
use super::super::super::super::math::theories::zfc::definitions::Set;
use serde::{Deserialize, Serialize};

/// A vector space is a set V equipped with vector addition and scalar multiplication
/// satisfying the vector space axioms.
///
/// Key concepts:
/// - Vector addition: V × V → V
/// - Scalar multiplication: K × V → V
/// - Linear combinations
/// - Linear independence
/// - Spanning sets
/// - Bases
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VectorSpace {
    /// The underlying set of vectors
    pub base_set: Set,
    /// Dimension of the space
    pub dimension: DimensionType,
    /// Field of scalars
    pub scalar_field: ScalarFieldType,
    /// Properties specific to the vector space
    pub properties: Vec<VectorSpaceProperty>,
}

/// A normed vector space is a vector space equipped with a norm function
/// that measures the "size" or "length" of vectors.
///
/// Key concepts:
/// - Norm axioms: ‖x‖ ≥ 0, ‖x‖ = 0 ⟺ x = 0, ‖λx‖ = |λ|‖x‖, ‖x+y‖ ≤ ‖x‖+‖y‖
/// - Induced metric: d(x,y) = ‖x-y‖
/// - Completeness: Cauchy sequences converge
/// - Banach spaces: Complete normed spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NormedSpace {
    /// The underlying vector space
    pub vector_space: VectorSpace,
    /// Type of norm
    pub norm_type: NormType,
    /// Properties specific to the normed structure
    pub properties: Vec<NormedSpaceProperty>,
}

/// An inner product space is a vector space equipped with an inner product
/// that allows measurement of angles and orthogonality.
///
/// Key concepts:
/// - Inner product axioms: ⟨x,y⟩ = ⟨y,x⟩*, ⟨λx,y⟩ = λ⟨x,y⟩, ⟨x+y,z⟩ = ⟨x,z⟩+⟨y,z⟩, ⟨x,x⟩ > 0
/// - Induced norm: ‖x‖ = √⟨x,x⟩
/// - Orthogonality: x ⊥ y ⟺ ⟨x,y⟩ = 0
/// - Hilbert spaces: Complete inner product spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InnerProductSpace {
    /// The underlying normed space
    pub normed_space: NormedSpace,
    /// Type of inner product
    pub inner_product_type: InnerProductType,
    /// Properties specific to the inner product structure
    pub properties: Vec<InnerProductSpaceProperty>,
}

/// Types of vector space dimension
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

/// Types of scalar fields
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ScalarFieldType {
    /// Real numbers ℝ
    Real,

    /// Complex numbers ℂ
    Complex,

    /// Rational numbers ℚ
    Rational,

    /// Finite field of prime order
    Prime(u32),
}

/// Properties specific to vector spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VectorSpaceProperty {
    /// Basis properties
    Basis(BasisType),

    /// Topology properties
    Topology(TopologyType),

    /// Duality properties
    Duality(DualityType),

    /// Decomposition properties
    Decomposition(DecompositionType),
}

/// Types of bases for vector spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BasisType {
    /// Standard basis
    Standard,

    /// Orthogonal basis
    Orthogonal,

    /// Orthonormal basis
    Orthonormal,

    /// Schauder basis
    Schauder,

    /// Hamel basis
    Hamel,
}

/// Types of topologies on vector spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TopologyType {
    /// Norm topology
    Norm,

    /// Weak topology
    Weak,

    /// Strong topology
    Strong,

    /// Discrete topology
    Discrete,
}

/// Types of duality for vector spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DualityType {
    /// Reflexive
    Reflexive,

    /// Non-reflexive
    NonReflexive,

    /// Bidual isomorphic
    BidualIsomorphic,
}

/// Types of decompositions for vector spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DecompositionType {
    /// Direct sum
    DirectSum,

    /// Tensor product
    TensorProduct,

    /// Quotient
    Quotient,
}

/// Types of norms on vector spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NormType {
    /// Euclidean norm
    Euclidean,

    /// Manhattan norm
    Manhattan,

    /// Maximum norm
    Maximum,

    /// p-norm
    Lp(i64), // todo: what is this, it used to be f64, but I change it avoid trait bound error introduced by Eq

    /// Operator norm
    Operator,
}

/// Properties specific to normed spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NormedSpaceProperty {
    /// Completeness properties
    Complete(CompletenessType),

    /// Convexity properties
    Convex(ConvexityType),

    /// Separability properties
    Separable(SeparabilityType),
}

/// Types of completeness for normed spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CompletenessType {
    /// Complete (Banach)
    Complete,

    /// Not complete
    Incomplete,
}

/// Types of convexity for normed spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConvexityType {
    /// Strictly convex
    Strictly,

    /// Uniformly convex
    Uniformly,

    /// Not convex
    NonConvex,
}

/// Types of separability for normed spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SeparabilityType {
    /// Separable
    Separable,

    /// Non-separable
    NonSeparable,
}

/// Types of inner products
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum InnerProductType {
    /// Real inner product
    Real,

    /// Complex inner product
    Complex,

    /// Indefinite inner product
    Indefinite,
}

/// Properties specific to inner product spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum InnerProductSpaceProperty {
    /// Orthogonality properties
    Orthogonality(OrthogonalityType),

    /// Completeness properties
    Complete(CompletenessType),

    /// Separability properties
    Separable(SeparabilityType),
}

/// Types of orthogonality for inner product spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrthogonalityType {
    /// Has orthonormal basis
    OrthonormalBasis,

    /// Has orthogonal decomposition
    OrthogonalDecomposition,

    /// Not orthogonal
    NonOrthogonal,
}

/// A linear transformation T: V → W between vector spaces satisfies:
/// 1. T(u + v) = T(u) + T(v)  (additivity)
/// 2. T(av) = aT(v)           (homogeneity)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LinearTransformation {
    /// Domain vector space
    pub domain: VectorSpace,
    /// Codomain vector space
    pub codomain: VectorSpace,
    /// Properties of the transformation
    pub properties: VariantSet<LinearTransformationProperty>,
}

/// Properties of linear transformations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LinearTransformationProperty {
    /// Kernel: ker(T) = {v ∈ V : T(v) = 0}
    Kernel(KernelProperty),
    /// Image: im(T) = {T(v) : v ∈ V}
    Image(ImageProperty),
    /// Rank: dim(im(T))
    Rank(RankProperty),
    /// Nullity: dim(ker(T))
    Nullity(NullityProperty),
    /// Invertibility
    Invertible(InvertibilityProperty),
    /// Diagonalizability
    Diagonalizable(DiagonalizabilityProperty),
}

/// Properties for kernels of linear transformations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum KernelProperty {
    /// Trivial kernel
    Trivial,
    /// Non-trivial kernel
    NonTrivial,
    /// Finite dimensional
    FiniteDimensional(u32),
}

/// Properties for images of linear transformations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ImageProperty {
    /// Closed image
    Closed,
    /// Dense image
    Dense,
    /// Finite dimensional
    FiniteDimensional(u32),
}

/// Properties for rank of linear transformations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RankProperty {
    /// Finite rank
    Finite(u32),
    /// Infinite rank
    Infinite,
}

/// Properties for nullity of linear transformations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NullityProperty {
    /// Finite nullity
    Finite(u32),
    /// Infinite nullity
    Infinite,
}

/// Properties for invertibility of linear transformations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum InvertibilityProperty {
    /// Invertible
    Invertible,
    /// Not invertible
    NonInvertible,
}

/// Properties for diagonalizability of linear transformations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DiagonalizabilityProperty {
    /// Diagonalizable
    Diagonalizable,
    /// Not diagonalizable
    NonDiagonalizable,
    /// Diagonalizable over extension field
    DiagonalizableOverExtension,
}

/// A tensor space is a vector space constructed from the tensor product of vector spaces
/// Key concepts:
/// - Multilinearity
/// - Tensor product
/// - Symmetry and antisymmetry
/// - Contraction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TensorSpace {
    /// The underlying vector space
    pub vector_space: VectorSpace,
    /// Factor spaces in the tensor product
    pub factor_spaces: Vec<VectorSpace>,
    /// Tensor product structure
    pub product_structure: TensorProductStructure,
    /// Properties specific to the tensor space
    pub properties: VariantSet<TensorSpaceProperty>,
}

/// Structure of tensor products
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TensorProductStructure {
    /// Standard tensor product (no symmetry)
    Standard,
    /// Symmetric tensor product
    Symmetric {
        /// Symmetry group
        group: String,
    },
    /// Antisymmetric tensor product (exterior product)
    Antisymmetric,
    /// Mixed tensor product (combination of covariant and contravariant)
    Mixed {
        /// Covariant indices
        covariant: Vec<usize>,
        /// Contravariant indices
        contravariant: Vec<usize>,
    },
}

/// Properties specific to tensor spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TensorSpaceProperty {
    /// Algebraic properties
    Algebraic {
        /// Basis type (reusing BasisType)
        basis: BasisType,
        /// Multilinearity properties
        multilinear: MultilinearityType,
    },
    /// Symmetry properties
    Symmetry {
        /// Type of symmetry
        symmetry_type: String,
        /// Symmetry group
        group: String,
    },
    /// Decomposition properties (reusing DecompositionType)
    Decomposition(DecompositionType),
}

/// Types of multilinearity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MultilinearityType {
    /// Fully multilinear
    Full,
    /// Partially multilinear
    Partial {
        /// Which arguments are linear
        linear_args: Vec<bool>,
    },
    /// Alternating multilinear
    Alternating,
}

/// Operations on tensors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TensorOperation {
    /// Tensor product (outer product)
    Product,
    /// Contraction (trace on specified indices)
    Contract {
        /// Indices to contract (must be one covariant, one contravariant)
        indices: Vec<(usize, usize)>,
    },
    /// Symmetrization
    Symmetrize {
        /// Indices to symmetrize
        indices: Vec<usize>,
    },
    /// Antisymmetrization (exterior product)
    Antisymmetrize {
        /// Indices to antisymmetrize
        indices: Vec<usize>,
    },
    /// Linear transformation (reusing LinearTransformation)
    Transform {
        /// The transformation to apply
        transformation: LinearTransformation,
        /// Index to transform
        index: usize,
    },
}
