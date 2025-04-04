use crate::formalize_v2::subjects::math::theories::topology::definitions::CompactProperty;
use crate::formalize_v2::subjects::math::theories::zfc::set::Set;
use crate::formalize_v2::subjects::math::theories::{
    common::spaces::Space, linear_algebra::definitions::VectorSpace, VariantSet,
};
use serde::{Deserialize, Serialize};

/// A Banach space is a complete normed vector space.
/// Every Cauchy sequence in a Banach space converges to a point in the space.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BanachSpace {
    /// The underlying set
    pub set: Set,
    /// The norm
    pub norm: Norm,
    /// Properties of the Banach space
    pub properties: VariantSet<BanachSpaceProperty>,
}

/// Properties specific to Banach spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BanachSpaceProperty {
    /// Whether the space is separable
    Separable(bool),
    /// Whether the space is reflexive
    Reflexive(bool),
    /// Whether the space is uniformly convex
    UniformlyConvex(bool),
}

/// A Hilbert space is a complete inner product space.
/// The inner product induces a norm that makes the space a Banach space.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HilbertSpace {
    /// The underlying set
    pub set: Set,
    /// The inner product
    pub inner_product: InnerProduct,
    /// Properties of the Hilbert space
    pub properties: VariantSet<HilbertSpaceProperty>,
}

/// Properties specific to Hilbert spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HilbertSpaceProperty {
    /// Whether the space is separable
    Separable(bool),
    /// Whether the space has orthonormal basis
    HasOrthonormalBasis(bool),
    /// Whether the space is real or complex
    ScalarField(ScalarFieldType),
}

/// A norm on a vector space
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Norm {
    /// Name or description of the norm
    pub name: String,
    /// Type of the norm
    pub norm_type: NormType,
}

/// Types of norms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NormType {
    /// L^p norm
    Lp(f64),
    /// Supremum norm
    Supremum,
    /// Custom norm
    Custom(String),
}

/// An inner product on a vector space
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InnerProduct {
    /// Name or description of the inner product
    pub name: String,
    /// Type of the inner product
    pub inner_product_type: InnerProductType,
}

/// Types of inner products
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InnerProductType {
    /// Standard dot product
    DotProduct,
    /// L^2 inner product
    L2,
    /// Custom inner product
    Custom(String),
}

/// Types of scalar fields
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScalarFieldType {
    /// Real numbers
    Real,
    /// Complex numbers
    Complex,
}

/// A function space of maps between two spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionSpace {
    /// Domain space
    pub domain: Box<Space>,
    /// Codomain space
    pub codomain: Box<Space>,
    /// Properties of the function space
    pub properties: VariantSet<FunctionSpaceProperty>,
}

/// Properties specific to function spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FunctionSpaceProperty {
    /// Whether the space is complete
    Complete(bool),
    /// Whether the space is separable
    Separable(bool),
    /// Whether the space is reflexive
    Reflexive(bool),
}

/// A Sobolev space W^{k,p}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SobolevSpace {
    /// The underlying function space
    pub base_function_space: Box<Space>,
    /// Order of derivatives
    pub derivative_order: u32,
    /// Integrability index p
    pub integrability_index: Option<f64>,
    /// Properties of the Sobolev space
    pub properties: VariantSet<SobolevSpaceProperty>,
}

/// Properties specific to Sobolev spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SobolevSpaceProperty {
    /// Order of derivatives
    Order(u32),
    /// Integrability index
    Index(f64),
    /// Whether the space is complete
    Complete(bool),
    /// Whether the space is separable
    Separable(bool),
    /// Whether the space is reflexive
    Reflexive(bool),
}

/// A space of distributions (generalized functions)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DistributionSpace {
    /// The space of test functions
    pub test_function_space: Box<Space>,
    /// Order of the distribution
    pub order: Option<u32>,
    /// Properties of the distribution space
    pub properties: VariantSet<DistributionSpaceProperty>,
}

/// Properties specific to distribution spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DistributionSpaceProperty {
    /// Order of the distribution
    Order(Option<u32>),
    /// Whether the space is nuclear
    Nuclear(bool),
    /// Whether the space is reflexive
    Reflexive(bool),
}

/// A Fréchet space
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrechetSpace {
    /// The underlying vector space
    pub vector_space: VectorSpace,
    /// Properties specific to the Fréchet structure
    pub properties: VariantSet<FrechetSpaceProperty>,
}

/// Properties specific to Fréchet spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FrechetSpaceProperty {
    /// Whether the space is nuclear
    Nuclear(bool),
    /// Whether the space is Montel
    Montel(bool),
    /// Whether the space is separable
    Separable(bool),
}

/// A locally convex space
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocallyConvexSpace {
    /// The underlying vector space
    pub vector_space: VectorSpace,
    /// Properties specific to the locally convex structure
    pub properties: VariantSet<LocallyConvexSpaceProperty>,
}

/// Properties specific to locally convex spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LocallyConvexSpaceProperty {
    /// Whether the space is barreled
    Barreled(bool),
    /// Whether the space is bornological
    Bornological(bool),
    /// Whether the space is Montel
    Montel(bool),
}
