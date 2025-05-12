use crate::subjects::math::formalism::expressions::Identifier;
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::{complexity::Complexity, theorem::MathObject};

use super::super::super::formalism::expressions::{MathExpression, TheoryExpression};
use super::super::super::formalism::relations::MathRelation;
use super::super::VariantSet;
use super::super::fields::definitions::Field;
use super::super::topology::definitions::TopologicalSpace;
use super::super::zfc::set::{Set, SetProperty};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use thiserror::Error;

//==== GROUP-SPECIFIC OPERATION TYPES ====//

/// Types of operations specific to group theory
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum GroupOperationVariant {
    /// Standard multiplication (used in most abstract groups)
    Multiplication,
    /// Addition (used in additive groups)
    Addition,
    /// Composition (used in transformation groups)
    Composition,
    /// Matrix multiplication (for matrix groups)
    MatrixMultiplication,
    /// Direct product of groups
    DirectProduct,
    /// Semidirect product of groups
    SemidirectProduct,
    /// Free product (used in combinatorial group theory)
    FreeProduct,
}

/// Notation used for group operations
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum GroupNotation {
    /// Infix notation: a * b
    Infix(GroupSymbol),
    /// Function notation: f(a, b)
    Function(String),
    /// Juxtaposition: ab (for multiplication)
    Juxtaposition,
}

/// Common symbols used in group theory
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum GroupSymbol {
    /// Multiplication: ×
    Times,
    /// Multiplication: ·
    Dot,
    /// Multiplication: *
    Asterisk,
    /// Addition: +
    Plus,
    /// Circle: ∘
    Circle,
    /// Semidirect product: ⋊
    SemiDirectLeft,
    /// Semidirect product: ⋉
    SemiDirectRight,
    /// Direct product: ×
    DirectProduct,
}

/// Identity element for group operations
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum GroupIdentity {
    /// Multiplicative identity: 1
    One,
    /// Additive identity: 0
    Zero,
    /// Identity matrix
    IdentityMatrix,
    /// Identity permutation
    IdentityPermutation,
    /// Identity function
    IdentityFunction,
}

/// Inverse operation types in group theory
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum GroupInverse {
    /// Multiplicative inverse: x⁻¹
    MultiplicativeInverse,
    /// Additive inverse: -x
    AdditiveInverse,
    /// Matrix inverse
    MatrixInverse,
    /// Permutation inverse
    PermutationInverse,
    /// Function inverse (for function composition groups)
    FunctionInverse,
}

/// How inverses are applied in groups
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum GroupInverseApplication {
    /// Left inverse: b*a = e
    Left,
    /// Right inverse: a*b = e
    Right,
    /// Two-sided inverse: a*b = b*a = e (standard for groups)
    TwoSided,
}

/// Properties specific to group operations
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum GroupOperationProperty {
    /// Whether the operation is associative (required for groups)
    Associative,
    /// Whether the operation is commutative
    Commutative(bool),
    /// Whether the operation is closed (required for groups)
    Closed,
}

/// Complete binary operation structure specific to group theory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupOperation {
    /// Type of operation
    pub operation_type: GroupOperationVariant,

    /// Notation/symbol used for this operation
    pub notation: GroupNotation,

    /// Identity element (required for groups)
    pub identity: GroupIdentity,

    /// Inverse operation (required for groups)
    pub inverse: GroupInverse,

    /// How inverses are applied (usually two-sided in groups)
    pub inverse_application: GroupInverseApplication,

    /// Properties of this operation
    pub properties: Vec<GroupOperationProperty>,

    /// For product operations, contains information about the product structure
    pub product_info: Option<ProductInfo>,
}

/// Information about product operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductInfo {
    /// The type of product operation
    pub operation: ProductOperation,

    /// For semidirect products, the action mapping
    pub action: Option<String>,

    /// For fibered products, the homomorphism
    pub homomorphism: Option<String>,

    /// Properties specific to this product operation
    pub properties: VariantSet<ProductProperty>,
}

impl Default for GroupOperation {
    fn default() -> Self {
        GroupOperation {
            operation_type: GroupOperationVariant::Multiplication,
            notation: GroupNotation::Infix(GroupSymbol::Times),
            identity: GroupIdentity::One,
            inverse: GroupInverse::MultiplicativeInverse,
            inverse_application: GroupInverseApplication::TwoSided,
            properties: vec![GroupOperationProperty::Associative],
            product_info: None,
        }
    }
}

/// Core algebraic structure of a group, containing the minimal data needed to satisfy group axioms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupBasic {
    /// The underlying set
    pub base_set: Set,
    /// The binary operation with its properties
    pub operation: GroupOperation,
    /// Properties specific to the group structure
    pub props: VariantSet<GroupProperty>,
}

impl Default for GroupBasic {
    fn default() -> Self {
        GroupBasic {
            base_set: Set::empty(),
            operation: GroupOperation::default(),
            props: VariantSet::new(),
        }
    }
}

/// Type of product operation used to form a product group
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProductOperation {
    /// Direct product (×): Cartesian product with componentwise operation
    Direct,

    /// Semidirect product (⋊): Normal subgroup with an action
    Semidirect {
        /// The action defining the semidirect product
        action: Box<GroupAction>,
    },

    /// Free product (*): No relations between the groups
    Free,

    /// Wreath product (≀): Special semidirect product with permutation action
    Wreath,

    /// Central product: Quotient of direct product
    Central,

    /// Fibered product: Pullback along a homomorphism
    Fibered {
        /// The homomorphism defining the fibered product
        homomorphism: Box<GroupHomomorphism>,
    },
}

/// A product group combining two or more groups with a specific operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductGroup {
    /// The core algebraic group structure
    pub core: GroupBasic,

    /// The type of product operation used
    pub operation: ProductOperation,

    /// The component groups
    pub components: Vec<Box<Group>>,

    /// For semidirect products, identifies which component is normal
    pub normal_component: Option<usize>,

    /// Product specific properties
    pub product_props: VariantSet<ProductProperty>,
}

/// A unified wrapper for all group-like structures
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Group {
    /// Basic abstract group
    Basic(GroupBasic),

    // Groups with additional structure (axiomatically defined)
    Topological(TopologicalGroup),
    Lie(LieGroup),
    Cyclic(CyclicGroup),
    Symmetric(SymmetricGroup),
    Dihedral(DihedralGroup),
    GeneralLinear(GeneralLinearGroup),
    SpecialLinear(SpecialLinearGroup),
    Orthogonal(OrthogonalGroup),
    SpecialOrthogonal(SpecialOrthogonalGroup),
    Unitary(UnitaryGroup),
    SpecialUnitary(SpecialUnitaryGroup),
    Alternating(AlternatingGroup),
    ModularAdditive(ModularAdditiveGroup),
    ModularMultiplicative(ModularMultiplicativeGroup),
    Free(FreeGroup),
    Trivial(TrivialGroup),

    // Groups defined by operations on other groups
    Product(ProductGroup),
    Quotient(QuotientGroup),

    // Groups defined by other explicit constructions (flattened)
    Kernel(KernelGroup),
    Image(ImageGroup),
    Center(CenterGroup),
    GeneratedSubgroup(GeneratedSubgroup),
    Normalizer(NormalizerGroup),
    Centralizer(CentralizerGroup),
    CommutatorSubgroup(CommutatorSubgroup),
    SylowSubgroup(SylowSubgroup),
    WreathProduct(WreathProductGroup),
    CentralProduct(CentralProductGroup),
    Pullback(PullbackGroup),
    Restriction(RestrictionGroup),
}

/// Variants for property preservation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PreservationVariant {
    /// Preserves finiteness
    PreservesFiniteness,
    /// Preserves commutativity
    PreservesCommutativity,
    /// Preserves other properties
    PreservesOther(String),
}

/// Variants for construction complexity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComplexityVariant {
    /// Simple construction
    Simple,
    /// Moderate complexity
    Moderate,
    /// Complex construction
    Complex,
}

/// Variants for construction canonicity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CanonicityVariant {
    /// The construction method yields a canonically defined object,
    /// independent of specific choices within the variant data.
    Canonical,
    /// The construction method is either inherently non-canonical, or its
    /// resulting object's canonicity depends on the specific choices
    /// (e.g., generators, homomorphisms) provided in the variant's data fields.
    NonCanonical,
}

/// Properties specific to groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
pub enum GroupProperty {
    /// Commutativity properties
    Abelian(AbelianPropertyVariant),

    /// Finiteness properties
    Finite(FinitePropertyVariant),

    /// Simplicity properties
    Simple(SimplePropertyVariant),

    /// Solvability properties
    Solvable(SolvablePropertyVariant),

    /// Nilpotency properties
    Nilpotent(NilpotentPropertyVariant),
}

/// Properties specific to topological groups
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum TopologicalGroupProperty {
    /// Compactness properties
    Compact(CompactPropertyVariant),

    /// Connectedness properties
    Connected(ConnectedPropertyVariant),

    /// Metrizability properties
    Metrizable(MetrizablePropertyVariant),
}

/// Properties specific to Lie groups
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum LieGroupProperty {
    /// Semisimplicity properties
    Semisimple(SemisimplePropertyVariant),

    /// Reductivity properties
    Reductive(ReductivePropertyVariant),
}

/// Types of abelian groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
pub enum AbelianPropertyVariant {
    /// Commutative
    Abelian,

    /// Non-commutative
    NonAbelian,
}

/// Types of finite groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
pub enum FinitePropertyVariant {
    /// Finite order
    Finite(u32),

    /// Infinite order
    Infinite,

    /// Locally finite (every finitely generated subgroup is finite)
    LocallyFinite,
}

/// Types of simple groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
pub enum SimplePropertyVariant {
    /// No proper normal subgroups
    Simple,

    /// Not simple
    NonSimple,

    /// Quasi-simple
    QuasiSimple,
}

/// Types of solvable groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
pub enum SolvablePropertyVariant {
    /// Has solvable series
    Solvable,

    /// Not solvable
    NonSolvable,

    /// Polysolvable
    Polysolvable,
}

/// Types of nilpotent groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
pub enum NilpotentPropertyVariant {
    /// Has nilpotent series
    Nilpotent(u32),

    /// Not nilpotent
    NonNilpotent,
}

/// Types of compact groups
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum CompactPropertyVariant {
    /// Compact
    Compact,

    /// Non-compact
    NonCompact,

    /// Locally compact
    LocallyCompact,
}

/// Types of connected groups
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum ConnectedPropertyVariant {
    /// Connected
    Connected,

    /// Simply connected (stronger than just connected)
    SimplyConnected,

    /// Totally disconnected
    TotallyDisconnected,

    /// Locally connected
    LocallyConnected,

    /// Locally simply connected
    LocallySimplyConnected,
}

/// Types of metrizable groups
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum MetrizablePropertyVariant {
    /// Admits compatible metric
    Metrizable,

    /// Not metrizable
    NonMetrizable,
}

/// Types of semisimple Lie groups
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum SemisimplePropertyVariant {
    /// No abelian ideals
    Semisimple,

    /// Not semisimple
    NonSemisimple,

    /// Split semisimple
    Split,
}

/// Types of reductive Lie groups
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum ReductivePropertyVariant {
    /// Reductive
    Reductive,

    /// Not reductive
    NonReductive,
}

/// A group action of G on X is a homomorphism:
/// φ: G → Aut(X)
/// This combines both the action definition and target information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GroupAction {
    /// Action on a set
    SetAction {
        /// The acting group
        group: Group,
        /// The space being acted on
        space: Set,
        /// The specific point in the space (if any)
        point: Option<Box<GroupExpression>>,
        /// Properties of the action
        properties: VariantSet<GroupActionProperty>,
    },
    /// Action on a vector space
    VectorSpaceAction {
        /// The acting group
        group: Group,
        /// The vector space being acted on
        space: String,
        /// Specific vector in the space (if any)
        vector: Option<Vec<f64>>,
        /// Properties of the action
        properties: VariantSet<GroupActionProperty>,
    },
    /// Action on a topological space
    TopologicalSpaceAction {
        /// The acting group
        group: Group,
        /// The space
        space: String,
        /// Specific point in the space (if any)
        point: Option<String>,
        /// Properties of the action
        properties: VariantSet<GroupActionProperty>,
    },
}

/// Properties specific to group actions
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum GroupActionProperty {
    /// Transitive: Single orbit
    Transitive(TransitivityPropertyVariant),
    /// Free: Stabilizers are trivial
    Free(FreenessPropertyVariant),
    /// Proper: Preimages of compacts are compact
    Proper(PropernessPropertyVariant),
    /// Faithful: Kernel is trivial
    Faithful(FaithfulnessPropertyVariant),
}

/// Properties for transitivity of group actions
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum TransitivityPropertyVariant {
    /// Single orbit
    Transitive,
    /// Finitely many orbits
    FinitelyTransitive,
    /// Infinitely many orbits
    NonTransitive,
}

/// Properties for properness of group actions
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum PropernessPropertyVariant {
    /// Proper action
    Proper,

    /// Non-proper
    NonProper,

    /// Locally proper
    LocallyProper,
}

/// Properties for faithfulness of group actions
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum FaithfulnessPropertyVariant {
    /// Trivial kernel
    Faithful,

    /// Non-faithful
    NonFaithful,

    /// Locally faithful (finite kernel)
    LocallyFaithful,
}

/// Properties for freeness of group actions
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum FreenessPropertyVariant {
    /// Trivial stabilizers
    Free,

    /// Non-free
    NonFree,

    /// Locally free (finite stabilizers)
    LocallyFree,
}

/// Relations specific to group theory
/// these are the verbs in the language of group theory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GroupRelation {
    /// One group is a subgroup of another
    IsSubgroupOf {
        subgroup: Parametrizable<Group>,
        group: Parametrizable<Group>,
    },

    /// One group is a normal subgroup of another
    IsNormalSubgroupOf {
        subgroup: Parametrizable<Group>,
        group: Parametrizable<Group>,
    },

    /// Two groups are isomorphic
    IsIsomorphicTo {
        first: Parametrizable<Group>,
        second: Parametrizable<Group>,
    },

    /// One group is a quotient of another
    IsQuotientOf {
        quotient: Parametrizable<Group>,
        group: Parametrizable<Group>,
        normal_subgroup: Parametrizable<Group>,
    },

    /// Element is in the center of a group
    IsInCenterOf {
        element: Parametrizable<GroupExpression>,
        group: Parametrizable<Group>,
    },

    /// Two elements are conjugate in a group
    AreConjugateIn {
        element1: Parametrizable<GroupElement>,
        element2: Parametrizable<GroupElement>,
        group: Parametrizable<Group>,
    },

    /// An element has a specified order in a group
    HasOrderInGroup {
        element: Parametrizable<GroupExpression>,
        group: Parametrizable<Group>,
        order: Parametrizable<usize>,
    },

    /// A subgroup is of a specific index in a group
    HasIndexInGroup {
        subgroup: Parametrizable<Group>,
        group: Parametrizable<Group>,
        index: Parametrizable<usize>,
    },

    /// A group has a specific number of elements
    HasOrder {
        group: Parametrizable<Group>,
        order: Parametrizable<usize>,
    },

    /// A group is cyclic with a specific generator
    IsCyclicWithGenerator {
        group: Parametrizable<Group>,
        generator: Parametrizable<GroupExpression>,
    },

    /// An element normalizes a subgroup
    NormalizesSubgroup {
        element: Parametrizable<GroupExpression>,
        subgroup: Parametrizable<Group>,
        group: Parametrizable<Group>,
    },

    /// An element centralizes a subgroup
    CentralizesSubgroup {
        element: Parametrizable<GroupExpression>,
        subgroup: Parametrizable<Group>,
        group: Parametrizable<Group>,
    },

    /// A subgroup is characteristic
    IsCharacteristicSubgroupOf {
        subgroup: Parametrizable<Group>,
        group: Parametrizable<Group>,
    },

    /// The order of one group divides the order of another
    OrderDivides {
        group1: Parametrizable<Group>,
        group2: Parametrizable<Group>,
    },

    /// An element has a unique inverse in a group
    HasUniqueInverse {
        element: Parametrizable<GroupExpression>,
        group: Parametrizable<Group>,
    },

    /// Sylow p-subgroup properties
    SylowSubgroupProperties {
        prime: Parametrizable<GroupExpression>, // Assuming prime expression can be variable
        group: Parametrizable<Group>,
    },

    /// One element is the inverse of another
    IsInverseOf {
        element: Parametrizable<GroupExpression>,
        inverse: Parametrizable<GroupExpression>,
        group: Parametrizable<Group>,
    },

    /// A homomorphism between groups
    IsHomomorphism {
        homomorphism: Parametrizable<GroupExpression>,
        domain: Parametrizable<Group>,
        codomain: Parametrizable<Group>,
    },

    /// An isomorphic embedding of one group into another
    IsomorphicEmbedding {
        source: Parametrizable<Group>,
        target: Parametrizable<Group>,
    },

    /// Asserts a basic group property on a Group.
    HasBasicProperty {
        target: Parametrizable<Group>,
        property: GroupProperty,
    },

    HasTopologicalProperty {
        target: Parametrizable<TopologicalGroup>,
        property: TopologicalGroupProperty,
    },

    HasLieProperty {
        target: Parametrizable<LieGroup>,
        property: LieGroupProperty,
    },

    /// Asserts a property on a Group Action.
    HasActionProperty {
        target: Parametrizable<GroupAction>,
        property: GroupActionProperty,
    },

    /// Asserts a property on a Product Group.
    HasProductProperty {
        target: Parametrizable<ProductGroup>,
        property: ProductProperty,
    },

    /// Asserts a property on a Modular Additive Group.
    HasModularAdditiveProperty {
        target: Parametrizable<ModularAdditiveGroup>,
        property: ModularProperty,
    },

    /// Asserts a property on a Modular Multiplicative Group.
    HasModularMultiplicativeProperty {
        target: Parametrizable<ModularMultiplicativeGroup>,
        property: ModularProperty,
    },

    /// Asserts a Matrix property on a General Linear Group.
    HasGeneralLinearMatrixProperty {
        target: Parametrizable<GeneralLinearGroup>,
        property: MatrixProperty,
    },

    /// Asserts a Linear property on a General Linear Group.
    HasGeneralLinearLinearProperty {
        target: Parametrizable<GeneralLinearGroup>,
        property: LinearProperty,
    },

    /// Asserts a property on a Special Linear Group.
    HasSpecialLinearProperty {
        target: Parametrizable<SpecialLinearGroup>,
        property: SpecialLinearProperty,
    },

    /// Asserts a Matrix property on an Orthogonal Group.
    HasOrthogonalMatrixProperty {
        target: Parametrizable<OrthogonalGroup>,
        property: MatrixProperty,
    },

    /// Asserts a property on a Special Orthogonal Group.
    HasSpecialOrthogonalProperty {
        target: Parametrizable<SpecialOrthogonalGroup>,
        property: SpecialOrthogonalProperty,
    },

    /// Asserts a Matrix property on a Unitary Group.
    HasUnitaryMatrixProperty {
        target: Parametrizable<UnitaryGroup>,
        property: MatrixProperty,
    },

    /// Asserts a property on a Special Unitary Group.
    HasSpecialUnitaryProperty {
        target: Parametrizable<SpecialUnitaryGroup>,
        property: SpecialUnitaryProperty,
    },

    /// Asserts a Permutation property on an Alternating Group.
    HasAlternatingPermutationProperty {
        target: Parametrizable<AlternatingGroup>,
        property: PermutationProperty,
    },

    /// Asserts a property on a Free Group.
    HasFreeProperty {
        target: Parametrizable<FreeGroup>,
        property: FreeProperty,
    },

    /// Asserts a property on a Quotient Group.
    HasQuotientProperty {
        target: Parametrizable<QuotientGroup>,
        property: QuotientProperty,
    },

    /// Asserts a property on a Group Operation.
    HasOperationProperty {
        target: Parametrizable<GroupOperation>,
        property: GroupOperationProperty,
    },
}

// Helper methods for backward compatibility
impl GroupRelation {
    /// Create a new IsSubgroupOf relation with concrete groups
    pub fn is_subgroup_of(subgroup: &Group, group: &Group) -> Self {
        GroupRelation::IsSubgroupOf {
            subgroup: Parametrizable::Concrete(subgroup.clone()),
            group: Parametrizable::Concrete(group.clone()),
        }
    }

    /// Create a new IsNormalSubgroupOf relation with concrete groups
    pub fn is_normal_subgroup_of(subgroup: &Group, group: &Group) -> Self {
        GroupRelation::IsNormalSubgroupOf {
            subgroup: Parametrizable::Concrete(subgroup.clone()),
            group: Parametrizable::Concrete(group.clone()),
        }
    }

    /// Create a new IsIsomorphicTo relation with concrete groups
    pub fn is_isomorphic_to(first: &Group, second: &Group) -> Self {
        GroupRelation::IsIsomorphicTo {
            first: Parametrizable::Concrete(first.clone()),
            second: Parametrizable::Concrete(second.clone()),
        }
    }

    /// Create a new HasOrder relation with concrete group and order
    pub fn has_order(group: &Group, order: usize) -> Self {
        GroupRelation::HasOrder {
            group: Parametrizable::Concrete(group.clone()),
            order: Parametrizable::Concrete(order), // Keep usize unboxed unless needed
        }
    }

    /// Create a new OrderDivides relation with concrete groups
    pub fn order_divides(group1: &Group, group2: &Group) -> Self {
        GroupRelation::OrderDivides {
            group1: Parametrizable::Concrete(group1.clone()),
            group2: Parametrizable::Concrete(group2.clone()),
        }
    }
}

// Modify the GroupExpression enum to include a ProductOperation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GroupExpression {
    /// A concrete element in a group
    Element {
        group: Parametrizable<Group>,          // Group can be variable
        element: Parametrizable<GroupElement>, // Element can be variable
    },
    /// The identity element of a group
    Identity(Parametrizable<Group>), // Group can be variable
    /// A group operation between two element expressions
    Operation {
        group: Parametrizable<Group>, // Group can be variable
        left: Box<Parametrizable<GroupExpression>>,
        right: Box<Parametrizable<GroupExpression>>,
    },
    /// The inverse of an expression
    Inverse {
        group: Parametrizable<Group>, // Group can be variable
        element: Box<Parametrizable<GroupExpression>>,
    },
    /// A commutator of two elements
    Commutator {
        group: Parametrizable<Group>, // Group can be variable
        a: Box<Parametrizable<GroupExpression>>,
        b: Box<Parametrizable<GroupExpression>>,
    },
    /// A coset of a subgroup
    Coset {
        group: Parametrizable<Group>, // Group can be variable
        element: Box<Parametrizable<GroupExpression>>,
        subgroup: Parametrizable<Group>, // Subgroup can be variable
        is_left: bool,
    },
    /// A group action applied to an element
    ActionOnElement {
        action: Parametrizable<GroupAction>, // Action can be variable
        element: Box<Parametrizable<GroupExpression>>,
    },
    /// Represents a power (exponentiation) of an element
    Power {
        group: Parametrizable<Group>, // Group can be variable
        base: Box<Parametrizable<GroupExpression>>,
        exponent: Parametrizable<i32>, // Exponent can be variable
    },
    /// The order of a group: |G|
    GroupOrder {
        group: Parametrizable<Group>, // Group can be variable
    },
    /// The order of an element: |g|
    ElementOrder {
        element: Box<Parametrizable<GroupExpression>>,
        group: Parametrizable<Group>, // Group can be variable
    },
    /// A homomorphism between groups: φ : G → H
    Homomorphism(Parametrizable<GroupHomomorphism>), // Homomorphism itself can be variable
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupHomomorphism {
    /// The domain group
    pub domain: Parametrizable<Group>,
    /// The codomain group
    pub codomain: Parametrizable<Group>,
}

/// Different types of element values depending on the group structure
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum GroupElement {
    /// A numeric element (useful for Z/nZ, etc.)
    Integer(i64),
    /// A permutation (for symmetric groups)
    Permutation(Vec<usize>),
    /// A matrix (for matrix groups)
    Matrix(Vec<Vec<i64>>),
    /// A symbolic element (for abstract elements)
    Symbol(String),
}

/// Error type for group expression evaluation
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum GroupExpressionError {
    /// Element is not in the group
    InvalidElement(String),
    /// Operation not defined for these elements
    InvalidOperation(String),
    /// Other errors
    Other(String),
}

// Add GroupError type
#[derive(Debug, thiserror::Error)]
pub enum GroupError {
    #[error("Invalid element: {0}")]
    InvalidElement(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Other error: {0}")]
    Other(String),
}

// After the GroupAction enum definition, add the implementation
impl GroupAction {
    /// Create a new set action
    pub fn set_action(
        group: Group,
        space: Set,
        properties: VariantSet<GroupActionProperty>,
    ) -> Self {
        GroupAction::SetAction {
            group,
            space,
            point: None,
            properties,
        }
    }

    /// Create a new set action with a specific point
    pub fn set_action_with_point(
        group: Group,
        space: Set,
        point: GroupExpression,
        properties: VariantSet<GroupActionProperty>,
    ) -> Self {
        GroupAction::SetAction {
            group,
            space,
            point: Some(Box::new(point)),
            properties,
        }
    }

    /// Create a new vector space action
    pub fn vector_space_action(
        group: Group,
        space: impl Into<String>,
        properties: VariantSet<GroupActionProperty>,
    ) -> Self {
        GroupAction::VectorSpaceAction {
            group,
            space: space.into(),
            vector: None,
            properties,
        }
    }

    /// Create a new vector space action with a specific vector
    pub fn vector_space_action_with_vector(
        group: Group,
        space: impl Into<String>,
        vector: Vec<f64>,
        properties: VariantSet<GroupActionProperty>,
    ) -> Self {
        GroupAction::VectorSpaceAction {
            group,
            space: space.into(),
            vector: Some(vector),
            properties,
        }
    }

    /// Create a new topological space action
    pub fn topological_space_action(
        group: Group,
        space: impl Into<String>,
        properties: VariantSet<GroupActionProperty>,
    ) -> Self {
        GroupAction::TopologicalSpaceAction {
            group,
            space: space.into(),
            point: None,
            properties,
        }
    }

    /// Create a new topological space action with a specific point
    pub fn topological_space_action_with_point(
        group: Group,
        space: impl Into<String>,
        point: impl Into<String>,
        properties: VariantSet<GroupActionProperty>,
    ) -> Self {
        GroupAction::TopologicalSpaceAction {
            group,
            space: space.into(),
            point: Some(point.into()),
            properties,
        }
    }

    /// Get the group of this action
    pub fn get_group(&self) -> &Group {
        match self {
            GroupAction::SetAction { group, .. } => group,
            GroupAction::VectorSpaceAction { group, .. } => group,
            GroupAction::TopologicalSpaceAction { group, .. } => group,
        }
    }

    /// Get the properties of this action
    pub fn get_properties(&self) -> &VariantSet<GroupActionProperty> {
        match self {
            GroupAction::SetAction { properties, .. } => properties,
            GroupAction::VectorSpaceAction { properties, .. } => properties,
            GroupAction::TopologicalSpaceAction { properties, .. } => properties,
        }
    }
}

// Restore the GroupRelation helper methods that were removed
impl GroupRelation {
    /// Create a relation for element has unique inverse (simplified, concrete inputs)
    pub fn has_unique_inverse(element: &GroupExpression, group: &Group) -> Self {
        GroupRelation::HasUniqueInverse {
            element: Parametrizable::Concrete(element.clone()), // Keep Expr unboxed for now
            group: Parametrizable::Concrete(group.clone()),
        }
    }

    /// Create a relation for Sylow p-subgroup properties (simplified, concrete inputs)
    pub fn sylow_subgroup_properties(prime: &GroupExpression, group: &Group) -> Self {
        GroupRelation::SylowSubgroupProperties {
            prime: Parametrizable::Concrete(prime.clone()), // Keep Expr unboxed for now
            group: Parametrizable::Concrete(group.clone()),
        }
    }

    /// Create a relation for "p divides |G|" using general integer division
    // This returns a generic MathRelation rather than a GroupRelation
    // Needs update to handle Parametrizable group/prime potentially
    pub fn divides_order_of(prime: &GroupExpression, group: &Group) -> MathRelation {
        // TODO: Update this function carefully. How to get order of a potentially
        //       Parametrizable::Variable group? How to handle a Parametrizable prime expr?
        //       For now, assuming concrete inputs for demonstration.
        let concrete_prime = MathExpression::Expression(TheoryExpression::Group(prime.clone()));
        let concrete_group_boxed = Box::new(MathObject::Group(group.clone()));

        // Or ideally, use a representation that works with Parametrizable
        let group_order_expr =
            MathExpression::Expression(TheoryExpression::Group(GroupExpression::GroupOrder {
                group: Parametrizable::Concrete(group.clone()),
            }));

        let num_entity = super::super::number_theory::definitions::NumberTheoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };

        MathRelation::NumberTheory(
            super::super::number_theory::definitions::NumberTheoryRelation::Divides {
                entity: num_entity,
                divisor: concrete_prime,
                dividend: group_order_expr,
            },
        )
    }

    /// Create a relation for one element is the inverse of another (simplified, concrete)
    pub fn is_inverse_of(
        element: &GroupExpression,
        inverse: &GroupExpression,
        group: &Group,
    ) -> Self {
        GroupRelation::IsInverseOf {
            element: Parametrizable::Concrete(element.clone()),
            inverse: Parametrizable::Concrete(inverse.clone()),
            group: Parametrizable::Concrete(group.clone()),
        }
    }

    /// Create a relation for a homomorphism between groups (simplified, concrete)
    pub fn is_homomorphism(
        homomorphism: &GroupExpression,
        domain: &Group,
        codomain: &Group,
    ) -> Self {
        GroupRelation::IsHomomorphism {
            homomorphism: Parametrizable::Concrete(homomorphism.clone()),
            domain: Parametrizable::Concrete(domain.clone()),
            codomain: Parametrizable::Concrete(codomain.clone()),
        }
    }

    // Restore the isomorphic_embedding method (concrete)
    /// Create a relation for an isomorphic embedding (simplified, concrete)
    pub fn isomorphic_embedding(source: &Group, target: &Group) -> Self {
        GroupRelation::IsomorphicEmbedding {
            source: Parametrizable::Concrete(source.clone()),
            target: Parametrizable::Concrete(target.clone()),
        }
    }
}

/// A group with topological structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopologicalGroup {
    /// The core algebraic group structure
    pub core: GroupBasic,
    /// The topology on the group
    pub topology: TopologicalSpace,
    /// Properties specific to the topological structure
    pub props: VariantSet<TopologicalGroupProperty>,
}

/// A Lie group with smooth structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LieGroup {
    /// The core algebraic group structure
    pub core: GroupBasic,
    /// The topology on the group
    pub topology: TopologicalSpace,
    /// Smooth manifold structure (represented with charts)
    pub charts: Vec<String>, // Simplified; would be a real Charts type in production
    /// Properties specific to the Lie structure
    pub props: VariantSet<LieGroupProperty>,
}

/// A cyclic group generated by a single element
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CyclicGroup {
    /// The core algebraic group structure
    pub core: GroupBasic,
    /// The generator element
    pub generator: GroupElement,
    /// The order of the group (can be infinite)
    pub order: Option<usize>, // None means infinite
}

/// A symmetric group (permutation group) on n elements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SymmetricGroup {
    /// The core algebraic group structure
    pub core: GroupBasic,
    /// The number of elements being permuted
    pub degree: usize,
}

/// A dihedral group representing the symmetries of a regular polygon
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DihedralGroup {
    /// The core algebraic group structure
    pub core: GroupBasic,
    /// The order of the group (twice the number of sides of the polygon)
    pub order: usize,
}

/// Properties specific to matrix groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MatrixProperty {
    /// Determinant property
    Determinant(DeterminantPropertyVariant),
    /// Inner product preservation property
    InnerProductPreservation(InnerProductPreservationVariant),
    /// Orientation preservation property
    OrientationPreservation(OrientationPreservationVariant),
    /// Hermitian form preservation property
    HermitianFormPreservation(HermitianFormPreservationVariant),
    /// Dimension property
    Dimension(u32),
}

/// Property variants for determinant property
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DeterminantPropertyVariant {
    /// Determinant is always one
    AlwaysOne,
    /// Determinant is non-zero
    NonZero,
    /// Determinant has a specific value
    SpecificValue(String),
}

/// Property variants for inner product preservation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum InnerProductPreservationVariant {
    /// Preserves inner product
    Preserves,
    /// Scales inner product
    Scales,
    /// Does not preserve inner product
    DoesNotPreserve,
}

/// Property variants for orientation preservation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrientationPreservationVariant {
    /// Preserves orientation
    Preserves,
    /// Reverses orientation
    Reverses,
    /// May preserve or reverse (mixed)
    Mixed,
}

/// Property variants for Hermitian form preservation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum HermitianFormPreservationVariant {
    /// Preserves Hermitian form
    Preserves,
    /// Does not preserve Hermitian form
    DoesNotPreserve,
}

/// Properties specific to linear groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LinearProperty {
    /// Volume preservation property
    VolumePreservation(VolumePreservationVariant),
    /// Unimodularity property
    Unimodularity(UnimodularityVariant),
}

/// Property variants for volume preservation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VolumePreservationVariant {
    /// Preserves volume
    Preserves,
    /// Scales volume
    Scales,
    /// Does not preserve volume
    DoesNotPreserve,
}

/// Property variants for unimodularity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UnimodularityVariant {
    /// Is unimodular
    Unimodular,
    /// Is not unimodular
    NonUnimodular,
}

/// Properties specific to modular groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ModularProperty {
    /// Representatives style property
    Representatives(RepresentativesVariant),
    /// The modulus value
    Modulus(u32),
    /// Full multiplicative group property
    FullMultiplicative(FullMultiplicativeVariant),
    /// Coprime to modulus property
    CoprimeToModulus(CoprimeToModulusVariant),
}

/// Property variants for representatives
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RepresentativesVariant {
    /// Uses standard representatives
    Standard,
    /// Uses canonical representatives
    Canonical,
    /// Uses minimal representatives
    Minimal,
}

/// Property variants for full multiplicative status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FullMultiplicativeVariant {
    /// Is the full multiplicative group
    Full,
    /// Is a subgroup of the full multiplicative group
    Subgroup,
}

/// Property variants for coprime to modulus
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CoprimeToModulusVariant {
    /// All elements are coprime to modulus
    All,
    /// Some elements are coprime to modulus
    Some,
    /// No elements are coprime to modulus
    None,
}

/// Properties specific to product groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProductProperty {
    /// Projection maps property
    Projections(ProjectionsVariant),
    /// Embeddings property
    Embeddings(EmbeddingsVariant),
    /// Product type property
    ProductType(ProductTypeVariant),
    /// Action property (for semidirect products)
    Action(ActionVariant),
}

/// Property variants for projections
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProjectionsVariant {
    /// Has projection maps
    HasProjections,
    /// Has no projection maps
    HasNoProjections,
}

/// Property variants for embeddings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EmbeddingsVariant {
    /// Has canonical embeddings
    HasCanonical,
    /// Has non-canonical embeddings
    HasNonCanonical,
    /// Has no embeddings
    HasNoEmbeddings,
}

/// Property variants for product type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProductTypeVariant {
    /// Is a direct product
    Direct,
    /// Is a semidirect product
    Semidirect,
    /// Is a free product
    Free,
    /// Is a fibered product
    Fibered,
}

/// Property variants for action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ActionVariant {
    /// Has trivial action
    Trivial,
    /// Has non-trivial action
    NonTrivial,
    /// Has faithful action
    Faithful,
}

/// Properties specific to free groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FreeProperty {
    /// Rank property (number of generators)
    Rank(u32),
    /// Generation property
    Generation(GenerationVariant),
    /// Generators property
    Generators(GeneratorsVariant),
}

/// Property variants for generation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GenerationVariant {
    /// Is freely generated
    Freely,
    /// Is not freely generated
    NonFreely,
}

/// Property variants for generators
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GeneratorsVariant {
    /// Has standard generators
    Standard,
    /// Has non-standard generators
    NonStandard,
}

/// Properties specific to permutation groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PermutationProperty {
    /// Parity property
    Parity(ParityVariant),
    /// Degree property (size of the set being permuted)
    Degree(u32),
    /// Symmetric property
    Symmetric(SymmetricVariant),
    /// Primitivity property
    Primitivity(PrimitivityVariant),
}

/// Property variants for parity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ParityVariant {
    /// Contains only even permutations
    Even,
    /// Contains only odd permutations
    Odd,
    /// Contains both even and odd permutations
    Mixed,
}

/// Property variants for symmetric status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SymmetricVariant {
    /// Is the full symmetric group
    Full,
    /// Is a proper subgroup of the symmetric group
    ProperSubgroup,
}

/// Property variants for primitivity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PrimitivityVariant {
    /// Is primitive
    Primitive,
    /// Is imprimitive
    Imprimitive,
}

/// Properties specific to quotient groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QuotientProperty {
    /// Projection property
    Projection(ProjectionVariant),
    /// Simplicity property
    Simplicity(SimplicityVariant),
}

/// Property variants for projection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProjectionVariant {
    /// Has projection homomorphism
    HasProjection,
    /// Has no projection homomorphism
    HasNoProjection,
}

/// Property variants for simplicity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SimplicityVariant {
    /// Is a simple group
    Simple,
    /// Is not a simple group
    NonSimple,
}

/// Properties specific to special linear groups (SL)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpecialLinearProperty {
    /// Determinant property
    Determinant(SpecialLinearDeterminantVariant),
    /// Volume preservation property
    VolumePreservation(SpecialLinearVolumeVariant),
    /// Connectedness property
    Connectedness(SpecialLinearConnectednessVariant),
    /// Commutator subgroup property
    CommutatorSubgroup(SpecialLinearCommutatorVariant),
}

/// Property variants for special linear determinant
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpecialLinearDeterminantVariant {
    /// Determinant is always 1
    AlwaysOne,
}

/// Property variants for special linear volume preservation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpecialLinearVolumeVariant {
    /// Preserves volume forms
    Preserves,
}

/// Property variants for special linear connectedness
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpecialLinearConnectednessVariant {
    /// Is connected
    Connected,
    /// Is not connected
    Disconnected,
}

/// Property variants for special linear commutator subgroup
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpecialLinearCommutatorVariant {
    /// Is the commutator subgroup of GL(n,F)
    IsCommutator,
    /// Is not the commutator subgroup of GL(n,F)
    IsNotCommutator,
}

/// Properties specific to special orthogonal groups (SO)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpecialOrthogonalProperty {
    /// Orientation preservation property
    OrientationPreservation(SpecialOrthogonalOrientationVariant),
    /// Spin cover property
    SpinCover(SpecialOrthogonalSpinVariant),
    /// Connected component property
    ConnectedComponent(SpecialOrthogonalComponentVariant),
    /// Determinant property
    Determinant(SpecialOrthogonalDeterminantVariant),
}

/// Property variants for special orthogonal orientation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpecialOrthogonalOrientationVariant {
    /// Preserves orientation
    Preserves,
}

/// Property variants for special orthogonal spin cover
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpecialOrthogonalSpinVariant {
    /// Has a double cover (spin group)
    HasSpinCover,
    /// Has no spin cover
    NoSpinCover,
}

/// Property variants for special orthogonal component
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpecialOrthogonalComponentVariant {
    /// Is the connected component of O(n)
    IsConnectedComponent,
    /// Is not the connected component of O(n)
    IsNotConnectedComponent,
}

/// Property variants for special orthogonal determinant
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpecialOrthogonalDeterminantVariant {
    /// Determinant is always 1
    AlwaysOne,
}

/// Properties specific to special unitary groups (SU)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpecialUnitaryProperty {
    /// Determinant property
    Determinant(SpecialUnitaryDeterminantVariant),
    /// Compactness property
    Compactness(SpecialUnitaryCompactnessVariant),
    /// Connectedness property
    Connectedness(SpecialUnitaryConnectednessVariant),
    /// Volume preservation property
    VolumePreservation(SpecialUnitaryVolumeVariant),
}

/// Property variants for special unitary determinant
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpecialUnitaryDeterminantVariant {
    /// Determinant is always 1
    AlwaysOne,
}

/// Property variants for special unitary compactness
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpecialUnitaryCompactnessVariant {
    /// Always compact
    AlwaysCompact,
}

/// Property variants for special unitary connectedness
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpecialUnitaryConnectednessVariant {
    /// Simply connected
    SimplyConnected,
    /// Not simply connected
    NotSimplyConnected,
}

/// Property variants for special unitary volume preservation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpecialUnitaryVolumeVariant {
    /// Preserves complex volume form
    PreservesComplexVolume,
    /// Does not preserve complex volume form
    DoesNotPreserveComplexVolume,
}

/// General linear group GL(n,F)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneralLinearGroup {
    /// The core algebraic group structure
    pub core: GroupBasic,
    /// The dimension
    pub dimension: u32,
    /// The field over which the group is defined
    pub field: Field,
    /// Matrix specific properties
    pub matrix_props: VariantSet<MatrixProperty>,
    /// Linear specific properties
    pub linear_props: VariantSet<LinearProperty>,
}

/// Special linear group SL(n,F)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpecialLinearGroup {
    /// The underlying general linear group
    pub general_linear: GeneralLinearGroup,
    /// Properties specific to special linear groups
    pub special_linear_props: VariantSet<SpecialLinearProperty>,
}

/// Orthogonal group O(n)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrthogonalGroup {
    /// The core algebraic group structure
    pub core: GroupBasic,
    /// The dimension
    pub dimension: u32,
    /// Matrix specific properties
    pub matrix_props: VariantSet<MatrixProperty>,
}

/// Special orthogonal group SO(n)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpecialOrthogonalGroup {
    /// The underlying orthogonal group
    pub orthogonal: OrthogonalGroup,
    /// Properties specific to special orthogonal groups
    pub special_orthogonal_props: VariantSet<SpecialOrthogonalProperty>,
}

/// Unitary group U(n)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnitaryGroup {
    /// The core algebraic group structure
    pub core: GroupBasic,
    /// The dimension
    pub dimension: u32,
    /// Matrix specific properties
    pub matrix_props: VariantSet<MatrixProperty>,
}

/// Special unitary group SU(n)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpecialUnitaryGroup {
    /// The underlying unitary group
    pub unitary: UnitaryGroup,
    /// Properties specific to special unitary groups
    pub special_unitary_props: VariantSet<SpecialUnitaryProperty>,
}

/// Alternating group A_n
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlternatingGroup {
    /// The core algebraic group structure
    pub core: GroupBasic,
    /// The degree (n in A_n)
    pub degree: u32,
    /// Permutation specific properties
    pub perm_props: VariantSet<PermutationProperty>,
}

/// Modular additive group Z/nZ
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModularAdditiveGroup {
    /// The core algebraic group structure
    pub core: GroupBasic,
    /// The modulus
    pub modulus: u32,
    /// Modular specific properties
    pub modular_props: VariantSet<ModularProperty>,
}

/// Multiplicative group of integers modulo n
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModularMultiplicativeGroup {
    /// The core algebraic group structure
    pub core: GroupBasic,
    /// The modulus
    pub modulus: u32,
    /// Modular specific properties
    pub modular_props: VariantSet<ModularProperty>,
}

/// Free group F_n
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FreeGroup {
    /// The core algebraic group structure
    pub core: GroupBasic,
    /// The rank (number of generators)
    pub rank: u32,
    /// Free group specific properties
    pub free_props: VariantSet<FreeProperty>,
}

/// Quotient group G/N
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuotientGroup {
    /// The core algebraic group structure
    pub core: GroupBasic,
    /// The group
    pub group: Box<Group>,
    /// The normal subgroup
    pub normal_subgroup: Box<Group>,
    /// Quotient specific properties
    pub quotient_props: VariantSet<QuotientProperty>,
}

impl QuotientGroup {
    /// Creates a new quotient group from a group and a normal subgroup
    pub fn new(group: Group, normal_subgroup: Group, is_maximal: bool) -> Self {
        QuotientGroup {
            core: GroupBasic::default(),
            group: Box::new(group),
            normal_subgroup: Box::new(normal_subgroup),
            quotient_props: VariantSet::new(),
        }
    }
}

/// The trivial group
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrivialGroup {
    /// The core algebraic group structure
    pub core: GroupBasic,
}

// --- Structs for Flattened Group Constructions ---

/// A group defined as the kernel of a homomorphism
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KernelGroup {
    pub core: GroupBasic,
    pub defining_homomorphism: Box<GroupHomomorphism>,
    // Potentially add domain_group: Box<Group> if needed for context
}

/// A group defined as the image of a homomorphism
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageGroup {
    pub core: GroupBasic,
    pub defining_homomorphism: Box<GroupHomomorphism>,
    // Potentially add codomain_group: Box<Group> if needed for context
}

/// A group defined as the center of another group: Z(G)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CenterGroup {
    pub core: GroupBasic,
    pub parent_group: Box<Group>,
}

/// A group defined as a subgroup generated by a set of elements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneratedSubgroup {
    pub core: GroupBasic,
    pub parent_group: Box<Group>,
    pub generators: Vec<GroupElement>, // Or GroupExpression?
}

/// A group defined as the normalizer of a subgroup: N_G(H)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NormalizerGroup {
    pub core: GroupBasic,
    pub parent_group: Box<Group>,
    pub subgroup_normalized: Box<Group>,
}

/// A group defined as the centralizer of an element: C_G(x)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CentralizerGroup {
    pub core: GroupBasic,
    pub parent_group: Box<Group>,
    pub element_centralized: GroupElement, // Or GroupExpression?
}

/// A group defined as the commutator subgroup: [G,G]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommutatorSubgroup {
    pub core: GroupBasic,
    pub parent_group: Box<Group>,
}

/// A group defined as a Sylow p-subgroup: Syl_p(G)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SylowSubgroup {
    pub core: GroupBasic,
    pub parent_group: Box<Group>,
    pub prime: u64, // Assuming prime is a number
}

/// A group defined as a wreath product
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WreathProductGroup {
    pub core: GroupBasic,
    pub base_group: Box<Group>,
    pub acting_group: Box<Group>,
    // Add action details if needed
}

/// A group defined as a central product
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CentralProductGroup {
    pub core: GroupBasic,
    pub component_groups: Vec<Box<Group>>,
    pub center_identification_map: String, // Details on how centers are identified
}

/// A group defined as a pullback (fibered product)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PullbackGroup {
    pub core: GroupBasic,
    pub source_groups: Vec<Box<Group>>, // Groups being mapped from
    pub target_group: Box<Group>,       // Group being mapped to
    pub defining_homomorphisms: Vec<GroupHomomorphism>,
}

/// A group constructed by restricting to a specific subset satisfying group properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RestrictionGroup {
    pub core: GroupBasic,
    pub parent_group: Box<Group>,
    pub restriction_description: String, // How the restriction is defined
}

impl Group {
    /// Gets the properties of the group
    pub fn get_properties(&self) -> &VariantSet<GroupProperty> {
        match self {
            Group::Basic(g) => &g.props,
            Group::Topological(g) => &g.core.props,
            Group::Lie(g) => &g.core.props,
            Group::Cyclic(g) => &g.core.props,
            Group::Symmetric(g) => &g.core.props,
            Group::Dihedral(g) => &g.core.props,
            Group::GeneralLinear(g) => &g.core.props,
            Group::SpecialLinear(g) => &g.general_linear.core.props,
            Group::Orthogonal(g) => &g.core.props,
            Group::SpecialOrthogonal(g) => &g.orthogonal.core.props,
            Group::Unitary(g) => &g.core.props,
            Group::SpecialUnitary(g) => &g.unitary.core.props,
            Group::Alternating(g) => &g.core.props,
            Group::ModularAdditive(g) => &g.core.props,
            Group::ModularMultiplicative(g) => &g.core.props,
            Group::Free(g) => &g.core.props,
            Group::Trivial(g) => &g.core.props,
            Group::Product(g) => &g.core.props,
            Group::Quotient(g) => &g.core.props,
            Group::Kernel(g) => &g.core.props,
            Group::Image(g) => &g.core.props,
            Group::Center(g) => &g.core.props,
            Group::GeneratedSubgroup(g) => &g.core.props,
            Group::Normalizer(g) => &g.core.props,
            Group::Centralizer(g) => &g.core.props,
            Group::CommutatorSubgroup(g) => &g.core.props,
            Group::SylowSubgroup(g) => &g.core.props,
            Group::WreathProduct(g) => &g.core.props,
            Group::CentralProduct(g) => &g.core.props,
            Group::Pullback(g) => &g.core.props,
            Group::Restriction(g) => &g.core.props,
        }
    }

    /// Sets the properties of the group
    pub fn set_properties(&mut self, props: VariantSet<GroupProperty>) {
        match self {
            Group::Basic(g) => g.props = props,
            Group::Topological(g) => g.core.props = props,
            Group::Lie(g) => g.core.props = props,
            Group::Cyclic(g) => g.core.props = props,
            Group::Symmetric(g) => g.core.props = props,
            Group::Dihedral(g) => g.core.props = props,
            Group::GeneralLinear(g) => g.core.props = props,
            Group::SpecialLinear(g) => g.general_linear.core.props = props,
            Group::Orthogonal(g) => g.core.props = props,
            Group::SpecialOrthogonal(g) => g.orthogonal.core.props = props,
            Group::Unitary(g) => g.core.props = props,
            Group::SpecialUnitary(g) => g.unitary.core.props = props,
            Group::Alternating(g) => g.core.props = props,
            Group::ModularAdditive(g) => g.core.props = props,
            Group::ModularMultiplicative(g) => g.core.props = props,
            Group::Free(g) => g.core.props = props,
            Group::Trivial(g) => g.core.props = props,
            Group::Product(g) => g.core.props = props,
            Group::Quotient(g) => g.core.props = props,
            Group::Kernel(g) => g.core.props = props,
            Group::Image(g) => g.core.props = props,
            Group::Center(g) => g.core.props = props,
            Group::GeneratedSubgroup(g) => g.core.props = props,
            Group::Normalizer(g) => g.core.props = props,
            Group::Centralizer(g) => g.core.props = props,
            Group::CommutatorSubgroup(g) => g.core.props = props,
            Group::SylowSubgroup(g) => g.core.props = props,
            Group::WreathProduct(g) => g.core.props = props,
            Group::CentralProduct(g) => g.core.props = props,
            Group::Pullback(g) => g.core.props = props,
            Group::Restriction(g) => g.core.props = props,
        }
    }

    /// Gets a reference to the core of the group
    pub fn get_core(&self) -> &GroupBasic {
        match self {
            Group::Basic(g) => &g,
            Group::Topological(g) => &g.core,
            Group::Lie(g) => &g.core,
            Group::Cyclic(g) => &g.core,
            Group::Symmetric(g) => &g.core,
            Group::Dihedral(g) => &g.core,
            Group::GeneralLinear(g) => &g.core,
            Group::SpecialLinear(g) => &g.general_linear.core,
            Group::Orthogonal(g) => &g.core,
            Group::SpecialOrthogonal(g) => &g.orthogonal.core,
            Group::Unitary(g) => &g.core,
            Group::SpecialUnitary(g) => &g.unitary.core,
            Group::Alternating(g) => &g.core,
            Group::ModularAdditive(g) => &g.core,
            Group::ModularMultiplicative(g) => &g.core,
            Group::Free(g) => &g.core,
            Group::Trivial(g) => &g.core,
            Group::Product(g) => &g.core,
            Group::Quotient(g) => &g.core,
            Group::Kernel(g) => &g.core,
            Group::Image(g) => &g.core,
            Group::Center(g) => &g.core,
            Group::GeneratedSubgroup(g) => &g.core,
            Group::Normalizer(g) => &g.core,
            Group::Centralizer(g) => &g.core,
            Group::CommutatorSubgroup(g) => &g.core,
            Group::SylowSubgroup(g) => &g.core,
            Group::WreathProduct(g) => &g.core,
            Group::CentralProduct(g) => &g.core,
            Group::Pullback(g) => &g.core,
            Group::Restriction(g) => &g.core,
        }
    }
}

impl GroupExpression {
    pub fn matches_pattern_group_expr(&self, pattern: &GroupExpression) -> bool {
        match (self, pattern) {
            (
                GroupExpression::Element {
                    group: g1,
                    element: e1,
                },
                GroupExpression::Element {
                    group: g2,
                    element: e2,
                },
            ) => g1.matches_pattern_param(g2) && e1.matches_pattern_param(e2),
            (GroupExpression::Identity(g1), GroupExpression::Identity(g2)) => {
                g1.matches_pattern_param(g2)
            }
            (
                GroupExpression::Operation {
                    group: g1,
                    left: l1,
                    right: r1,
                },
                GroupExpression::Operation {
                    group: g2,
                    left: l2,
                    right: r2,
                },
            ) => {
                g1.matches_pattern_param(g2)
                    && l1.matches_pattern_param(l2)
                    && r1.matches_pattern_param(r2)
            }
            (
                GroupExpression::Inverse {
                    group: g1,
                    element: e1,
                },
                GroupExpression::Inverse {
                    group: g2,
                    element: e2,
                },
            ) => g1.matches_pattern_param(g2) && e1.matches_pattern_param(e2),
            // Add other GroupExpression variants
            _ => false,
        }
    }
}

impl GroupRelation {
    pub fn matches_pattern_group_relation(&self, pattern: &GroupRelation) -> bool {
        match (self, pattern) {
            (
                GroupRelation::IsSubgroupOf {
                    subgroup: sg1,
                    group: g1,
                },
                GroupRelation::IsSubgroupOf {
                    subgroup: sg2,
                    group: g2,
                },
            ) => sg1.matches_pattern_param(sg2) && g1.matches_pattern_param(g2),
            (
                GroupRelation::HasOrder {
                    group: g1,
                    order: o1,
                },
                GroupRelation::HasOrder {
                    group: g2,
                    order: o2,
                },
            ) => g1.matches_pattern_param(g2) && o1.matches_pattern_param(o2),
            // Add more GroupRelation variants here
            _ => false, // Default to no match for unhandled or different variants
        }
    }
}
