use crate::subjects::math::formalism::location::Located;
use crate::subjects::math::formalism::{complexity::Complexity, objects::MathObject};
use crate::subjects::math::theories::zfc::definitions::GenericSet;
use crate::turn_render::Identifier;
use crate::variant_set;

use super::super::super::formalism::expressions::{MathExpression, TheoryExpression};
use super::super::super::formalism::relations::MathRelation;
use super::super::VariantSet;
use super::super::fields::definitions::Field;
use super::super::topology::definitions::TopologicalSpace;
use super::super::zfc::definitions::{Set, SetProperty};

use crate::subjects::math::theories::number_theory::{self, NumberTheoryRelation};
use crate::turn_render::math_node::MathNode;
use crate::turn_render::section_node::{RichText, RichTextSegment, Section, SectionContentNode};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fmt::{self, Display};
use std::hash::Hash;
use std::hash::Hasher;
use std::sync::Arc;
use thiserror::Error;

use crate::subjects::math::formalism::abstraction_level::GetAbstractionLevel;

//==== GROUP-SPECIFIC OPERATION TYPES ====//

/// Types of operations specific to group theory
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroupOperationVariant {
    /// Generic operation (used for abstract groups)
    Generic,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroupNotation {
    /// Infix notation: a * b
    Infix(GroupSymbol),
    /// Function notation: f(a, b)
    Function(String),
    /// Juxtaposition: ab (for multiplication)
    Juxtaposition,
}

/// Common symbols used in group theory
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroupIdentity {
    /// Generic identity (used for abstract groups)
    Generic,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroupInverse {
    /// Generic inverse (used for abstract groups)
    Generic,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroupInverseApplication {
    /// Generic inverse application (used for abstract groups)
    Generic,
    /// Left inverse: b*a = e
    Left,
    /// Right inverse: a*b = e
    Right,
    /// Two-sided inverse: a*b = b*a = e (standard for groups)
    TwoSided,
}

/// Complete binary operation structure specific to group theory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GroupOperation {
    /// this struct is problematic, all fields should be tied together, we will need serious runtime check for them
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
}

impl Default for GroupOperation {
    fn default() -> Self {
        GroupOperation {
            operation_type: GroupOperationVariant::Generic,
            notation: GroupNotation::Infix(GroupSymbol::Asterisk),
            identity: GroupIdentity::One,
            inverse: GroupInverse::MultiplicativeInverse,
            inverse_application: GroupInverseApplication::TwoSided,
        }
    }
}

/// Information about product operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

/// Core algebraic structure of a group, containing the minimal data needed to satisfy group axioms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GenericGroup {
    /// The underlying set
    pub base_set: Set,
    /// The binary operation with its properties
    pub operation: GroupOperation,
    /// Properties specific to the group structure
    pub props: VariantSet<GroupProperty>,
}

impl Default for GenericGroup {
    fn default() -> Self {
        GenericGroup {
            base_set: Set::Parametric {
                parameters: std::collections::HashMap::new(),
                description: "Abstract group set".to_string(),
                membership_condition: "x ∈ G".to_string(),
                properties: crate::subjects::math::theories::VariantSet::new(),
            },
            operation: GroupOperation::default(),
            props: crate::subjects::math::theories::VariantSet::new(),
        }
    }
}

/// Type of product operation used to form a product group
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProductOperation {
    /// Direct product (×): Cartesian product with componentwise operation
    Direct,

    /// Semidirect product (⋊): Normal subgroup with an action
    Semidirect {
        /// The action defining the semidirect product
        action: Arc<GroupAction>,
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
        homomorphism: Arc<GroupHomomorphism>,
    },
}

/// A product group combining two or more groups with a specific operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProductGroup {
    /// The core algebraic group structure
    pub core: GenericGroup,

    /// The type of product operation used
    pub operation: ProductOperation,

    /// The component groups
    pub components: Vec<Arc<Group>>,

    /// For semidirect products, identifies which component is normal
    pub normal_component: Option<usize>,

    /// Product specific properties
    pub product_props: VariantSet<ProductProperty>,
}

/// A unified wrapper for all group-like structures.
/// The variants are ordered and commented to visually represent their conceptual hierarchy.
/// The formal "is-a" relationship is defined in the `GROUP_HIERARCHY` map.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Group {
    // --- Foundational & Abstract Groups ---
    Generic(GenericGroup),
    Trivial(TrivialGroup),
    Cyclic(CyclicGroup),
    Dihedral(DihedralGroup),
    Free(FreeGroup),

    // --- Permutation Groups ---
    Symmetric(SymmetricGroup),
    // A more specific type of SymmetricGroup
    Alternating(AlternatingGroup),

    // --- Matrix/Linear Groups ---
    GeneralLinear(GeneralLinearGroup),
    //  A more specific type of GeneralLinearGroup
    SpecialLinear(SpecialLinearGroup),
    //  A more specific type of GeneralLinearGroup
    Orthogonal(OrthogonalGroup),
    //      A more specific type of OrthogonalGroup
    SpecialOrthogonal(SpecialOrthogonalGroup),
    //  A more specific type of GeneralLinearGroup
    Unitary(UnitaryGroup),
    //      A more specific type of UnitaryGroup
    SpecialUnitary(SpecialUnitaryGroup),

    // --- Groups with Additional Structure ---
    Topological(TopologicalGroup),
    Lie(LieGroup),

    // --- Modular Groups ---
    ModularAdditive(ModularAdditiveGroup),
    ModularMultiplicative(ModularMultiplicativeGroup),

    // --- Groups Defined by Operations on Other Groups ---
    Product(ProductGroup),
    Quotient(QuotientGroup),

    // --- Groups Defined by Other Explicit Constructions ---
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TopologicalGroupProperty {
    /// Compactness properties
    Compact(CompactPropertyVariant),

    /// Connectedness properties
    Connected(ConnectedPropertyVariant),

    /// Metrizability properties
    Metrizable(MetrizablePropertyVariant),
}

/// Properties specific to Lie groups
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LieGroupProperty {
    /// Semisimplicity properties
    Semisimple(SemisimplePropertyVariant),

    /// Reductivity properties
    Reductive(ReductivePropertyVariant),
}

/// Types of abelian groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AbelianPropertyVariant {
    /// Commutative
    Abelian,

    /// Non-commutative
    NonAbelian,
}

/// Types of finite groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FinitePropertyVariant {
    /// Finite order
    Finite(u32),

    /// Infinite order
    Infinite,

    /// Locally finite (every finitely generated subgroup is finite)
    LocallyFinite,
}

/// Types of simple groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SimplePropertyVariant {
    /// No proper normal subgroups
    Simple,

    /// Not simple
    NonSimple,

    /// Quasi-simple
    QuasiSimple,
}

/// Types of solvable groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SolvablePropertyVariant {
    /// Has solvable series
    Solvable,

    /// Not solvable
    NonSolvable,

    /// Polysolvable
    Polysolvable,
}

/// Types of nilpotent groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NilpotentPropertyVariant {
    /// Has nilpotent series
    Nilpotent(u32),

    /// Not nilpotent
    NonNilpotent,
}

/// Types of compact groups
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompactPropertyVariant {
    /// Compact
    Compact,

    /// Non-compact
    NonCompact,

    /// Locally compact
    LocallyCompact,
}

/// Types of connected groups
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MetrizablePropertyVariant {
    /// Admits compatible metric
    Metrizable,

    /// Not metrizable
    NonMetrizable,
}

/// Types of semisimple Lie groups
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SemisimplePropertyVariant {
    /// No abelian ideals
    Semisimple,

    /// Not semisimple
    NonSemisimple,

    /// Split semisimple
    Split,
}

/// Types of reductive Lie groups
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReductivePropertyVariant {
    /// Reductive
    Reductive,

    /// Not reductive
    NonReductive,
}

/// A group action of G on X is a homomorphism:
/// φ: G → Aut(X)
/// This combines both the action definition and target information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum GroupAction {
    /// Action on a set
    SetAction {
        /// The acting group
        group: Group,
        /// The space being acted on
        space: Set,
        /// The specific point in the space (if any)
        point: Option<Arc<GroupExpression>>,
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
        vector: Option<Vec<i64>>,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransitivityPropertyVariant {
    /// Single orbit
    Transitive,
    /// Finitely many orbits
    FinitelyTransitive,
    /// Infinitely many orbits
    NonTransitive,
}

/// Properties for properness of group actions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PropernessPropertyVariant {
    /// Proper action
    Proper,

    /// Non-proper
    NonProper,

    /// Locally proper
    LocallyProper,
}

/// Properties for faithfulness of group actions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FaithfulnessPropertyVariant {
    /// Trivial kernel
    Faithful,

    /// Non-faithful
    NonFaithful,

    /// Locally faithful (finite kernel)
    LocallyFaithful,
}

/// Properties for freeness of group actions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GroupRelation {
    /// One group is a subgroup of another
    IsSubgroupOf {
        subgroup: Located<Group>,
        group: Located<Group>,
    },

    /// One group is a normal subgroup of another
    IsNormalSubgroupOf {
        subgroup: Located<Group>,
        group: Located<Group>,
    },

    /// Two groups are isomorphic
    IsIsomorphicTo {
        first: Located<Group>,
        second: Located<Group>,
    },

    /// One group is a quotient of another
    IsQuotientOf {
        quotient: Located<Group>,
        group: Located<Group>,
        normal_subgroup: Located<Group>,
    },

    /// Element is in the center of a group
    IsInCenterOf {
        element: Located<GroupExpression>,
        group: Located<Group>,
    },

    /// Two elements are conjugate in a group
    AreConjugateIn {
        element1: Located<GroupElement>,
        element2: Located<GroupElement>,
        group: Located<Group>,
    },

    /// An element has a specified order in a group
    HasOrderInGroup {
        element: Located<GroupExpression>,
        group: Located<Group>,
        order: Located<usize>,
    },

    /// A subgroup is of a specific index in a group
    HasIndexInGroup {
        subgroup: Located<Group>,
        group: Located<Group>,
        index: Located<usize>,
    },

    /// A group has a specific number of elements
    HasOrder {
        group: Located<Group>,
        order: Located<usize>,
    },

    /// A group is cyclic with a specific generator
    IsCyclicWithGenerator {
        group: Located<Group>,
        generator: Located<GroupExpression>,
    },

    /// An element normalizes a subgroup
    NormalizesSubgroup {
        element: Located<GroupExpression>,
        subgroup: Located<Group>,
        group: Located<Group>,
    },

    /// An element centralizes a subgroup
    CentralizesSubgroup {
        element: Located<GroupExpression>,
        subgroup: Located<Group>,
        group: Located<Group>,
    },

    /// A subgroup is characteristic
    IsCharacteristicSubgroupOf {
        subgroup: Located<Group>,
        group: Located<Group>,
    },

    /// The order of one group divides the order of another
    OrderDivides {
        group1: Located<Group>,
        group2: Located<Group>,
    },

    /// An element has a unique inverse in a group
    HasUniqueInverse {
        element: Located<GroupExpression>,
        group: Located<Group>,
    },

    /// Sylow p-subgroup properties
    SylowSubgroupProperties {
        prime: Located<GroupExpression>, // Assuming prime expression can be variable
        group: Located<Group>,
    },

    /// One element is the inverse of another
    IsInverseOf {
        element: Located<GroupExpression>,
        inverse: Located<GroupExpression>,
        group: Located<Group>,
    },

    /// A homomorphism between groups
    IsHomomorphism {
        homomorphism: Located<GroupExpression>,
        domain: Located<Group>,
        codomain: Located<Group>,
    },

    /// An isomorphic embedding of one group into another
    IsomorphicEmbedding {
        source: Located<Group>,
        target: Located<Group>,
    },

    /// Asserts a basic group property on a Group.
    HasBasicProperty {
        target: Located<Group>,
        property: GroupProperty,
    },

    HasTopologicalProperty {
        target: Located<TopologicalGroup>,
        property: TopologicalGroupProperty,
    },

    HasLieProperty {
        target: Located<LieGroup>,
        property: LieGroupProperty,
    },

    /// Asserts a property on a Group Action.
    HasActionProperty {
        target: Located<GroupAction>,
        property: GroupActionProperty,
    },

    /// Asserts a property on a Product Group.
    HasProductProperty {
        target: Located<ProductGroup>,
        property: ProductProperty,
    },

    /// Asserts a property on a Modular Additive Group.
    HasModularAdditiveProperty {
        target: Located<ModularAdditiveGroup>,
        property: ModularProperty,
    },

    /// Asserts a property on a Modular Multiplicative Group.
    HasModularMultiplicativeProperty {
        target: Located<ModularMultiplicativeGroup>,
        property: ModularProperty,
    },

    /// Asserts a Matrix property on a General Linear Group.
    HasGeneralLinearMatrixProperty {
        target: Located<GeneralLinearGroup>,
        property: MatrixProperty,
    },

    /// Asserts a Linear property on a General Linear Group.
    HasGeneralLinearLinearProperty {
        target: Located<GeneralLinearGroup>,
        property: LinearProperty,
    },

    /// Asserts a property on a Special Linear Group.
    HasSpecialLinearProperty {
        target: Located<SpecialLinearGroup>,
        property: SpecialLinearProperty,
    },

    /// Asserts a Matrix property on an Orthogonal Group.
    HasOrthogonalMatrixProperty {
        target: Located<OrthogonalGroup>,
        property: MatrixProperty,
    },

    /// Asserts a property on a Special Orthogonal Group.
    HasSpecialOrthogonalProperty {
        target: Located<SpecialOrthogonalGroup>,
        property: SpecialOrthogonalProperty,
    },

    /// Asserts a Matrix property on a Unitary Group.
    HasUnitaryMatrixProperty {
        target: Located<UnitaryGroup>,
        property: MatrixProperty,
    },

    /// Asserts a property on a Special Unitary Group.
    HasSpecialUnitaryProperty {
        target: Located<SpecialUnitaryGroup>,
        property: SpecialUnitaryProperty,
    },

    /// Asserts a Permutation property on an Alternating Group.
    HasAlternatingPermutationProperty {
        target: Located<AlternatingGroup>,
        property: PermutationProperty,
    },

    /// Asserts a property on a Free Group.
    HasFreeProperty {
        target: Located<FreeGroup>,
        property: FreeProperty,
    },

    /// Asserts a property on a Quotient Group.
    HasQuotientProperty {
        target: Located<QuotientGroup>,
        property: QuotientProperty,
    },
}

// Helper methods for backward compatibility
impl GroupRelation {
    /// Create a new IsSubgroupOf relation with concrete groups
    pub fn is_subgroup_of(subgroup: &Group, group: &Group) -> Self {
        GroupRelation::IsSubgroupOf {
            subgroup: Located::new_concrete(subgroup.clone()),
            group: Located::new_concrete(group.clone()),
        }
    }

    /// Create a new IsNormalSubgroupOf relation with concrete groups
    pub fn is_normal_subgroup_of(subgroup: &Group, group: &Group) -> Self {
        GroupRelation::IsNormalSubgroupOf {
            subgroup: Located::new_concrete(subgroup.clone()),
            group: Located::new_concrete(group.clone()),
        }
    }

    /// Create a new IsIsomorphicTo relation with concrete groups
    pub fn is_isomorphic_to(first: &Group, second: &Group) -> Self {
        GroupRelation::IsIsomorphicTo {
            first: Located::new_concrete(first.clone()),
            second: Located::new_concrete(second.clone()),
        }
    }

    /// Create a new HasOrder relation with concrete group and order
    pub fn has_order(group: &Group, order: usize) -> Self {
        GroupRelation::HasOrder {
            group: Located::new_concrete(group.clone()),
            order: Located::new_concrete(order), // Keep usize unboxed unless needed
        }
    }

    /// Create a new OrderDivides relation with concrete groups
    pub fn order_divides(group1: &Group, group2: &Group) -> Self {
        GroupRelation::OrderDivides {
            group1: Located::new_concrete(group1.clone()),
            group2: Located::new_concrete(group2.clone()),
        }
    }
}

// Modify the GroupExpression enum to include a ProductOperation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GroupExpression {
    /// A concrete element in a group, this is different to GroupRelation::ElementOf,
    /// becuase it is checked by default, not something to be proven
    Element {
        group: Located<Group>,                  // Group can be variable
        element: Option<Located<GroupElement>>, // Element can be variable
    },
    /// The identity element of a group
    Identity(Located<Group>), // Group can be variable
    /// A group operation between two element expressions
    Operation {
        group: Located<Group>, // Group can be variable
        left: Located<GroupExpression>,
        right: Located<GroupExpression>,
    },
    /// The inverse of an expression
    Inverse {
        group: Located<Group>, // Group can be variable
        element: Located<GroupExpression>,
    },
    /// A commutator of two elements
    Commutator {
        group: Located<Group>, // Group can be variable
        a: Located<GroupExpression>,
        b: Located<GroupExpression>,
    },
    /// A coset of a subgroup
    Coset {
        group: Located<Group>, // Group can be variable
        element: Located<GroupExpression>,
        subgroup: Located<Group>, // Subgroup can be variable
        is_left: bool,
    },
    /// A group action applied to an element
    ActionOnElement {
        action: Located<GroupAction>, // Action can be variable
        element: Located<GroupExpression>,
    },
    /// Represents a power (exponentiation) of an element
    Power {
        group: Located<Group>, // Group can be variable
        base: Located<GroupExpression>,
        exponent: Located<i32>, // Exponent can be variable
    },
    /// The order of a group: |G|
    GroupOrder {
        group: Located<Group>, // Group can be variable
    },
    /// The order of an element: |g|
    ElementOrder {
        element: Located<GroupExpression>,
        group: Located<Group>, // Group can be variable
    },
    /// A homomorphism between groups: φ : G → H
    Homomorphism(Located<GroupHomomorphism>), // Homomorphism itself can be variable
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupHomomorphism {
    /// The domain group
    pub domain: Located<Group>,
    /// The codomain group
    pub codomain: Located<Group>,
}

/// Different types of element values depending on the group structure
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl Display for GroupElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GroupElement::Integer(i) => write!(f, "{}", i),
            GroupElement::Permutation(p) => write!(f, "{:?}", p),
            GroupElement::Matrix(m) => write!(f, "{:?}", m),
            GroupElement::Symbol(s) => write!(f, "{}", s),
        }
    }
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
            point: Some(Arc::new(point)),
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
        vector: Vec<i64>,
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
            element: Located::new_concrete(element.clone()),
            group: Located::new_concrete(group.clone()),
        }
    }

    /// Create a relation for Sylow p-subgroup properties (simplified, concrete inputs)
    pub fn sylow_subgroup_properties(prime: &GroupExpression, group: &Group) -> Self {
        GroupRelation::SylowSubgroupProperties {
            prime: Located::new_concrete(prime.clone()), // Keep Expr unboxed for now
            group: Located::new_concrete(group.clone()),
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
                group: Located::new_concrete(group.clone()),
            }));

        MathRelation::NumberTheory(NumberTheoryRelation::Divides {
            divisor: Located::new_concrete(concrete_prime),
            dividend: Located::new_concrete(group_order_expr),
        })
    }

    /// Create a relation for one element is the inverse of another (simplified, concrete)
    pub fn is_inverse_of(
        element: &GroupExpression,
        inverse: &GroupExpression,
        group: &Group,
    ) -> Self {
        GroupRelation::IsInverseOf {
            element: Located::new_concrete(element.clone()),
            inverse: Located::new_concrete(inverse.clone()),
            group: Located::new_concrete(group.clone()),
        }
    }

    /// Create a relation for a homomorphism between groups (simplified, concrete)
    pub fn is_homomorphism(
        homomorphism: &GroupExpression,
        domain: &Group,
        codomain: &Group,
    ) -> Self {
        GroupRelation::IsHomomorphism {
            homomorphism: Located::new_concrete(homomorphism.clone()),
            domain: Located::new_concrete(domain.clone()),
            codomain: Located::new_concrete(codomain.clone()),
        }
    }

    // Restore the isomorphic_embedding method (concrete)
    /// Create a relation for an isomorphic embedding (simplified, concrete)
    pub fn isomorphic_embedding(source: &Group, target: &Group) -> Self {
        GroupRelation::IsomorphicEmbedding {
            source: Located::new_concrete(source.clone()),
            target: Located::new_concrete(target.clone()),
        }
    }
}

/// A group with topological structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TopologicalGroup {
    /// The core algebraic group structure
    pub core: GenericGroup,
    /// The topology on the group
    pub topology: TopologicalSpace,
    /// Properties specific to the topological structure
    pub props: VariantSet<TopologicalGroupProperty>,
}

/// A Lie group with smooth structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LieGroup {
    /// The core algebraic group structure
    pub core: GenericGroup,
    /// The topology on the group
    pub topology: TopologicalSpace,
    /// Smooth manifold structure (represented with charts)
    pub charts: Vec<String>, // Simplified; would be a real Charts type in production
    /// Properties specific to the Lie structure
    pub props: VariantSet<LieGroupProperty>,
}

/// A cyclic group generated by a single element
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CyclicGroup {
    /// The core algebraic group structure
    pub core: GenericGroup,
    /// The generator element
    pub generator: GroupElement,
    /// The order of the group (can be infinite)
    pub order: Option<usize>, // None means infinite
}

/// A symmetric group (permutation group) on n elements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SymmetricGroup {
    /// The core algebraic group structure
    pub core: GenericGroup,
    /// The number of elements being permuted
    pub degree: usize,
}

/// A dihedral group representing the symmetries of a regular polygon
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DihedralGroup {
    /// The core algebraic group structure
    pub core: GenericGroup,
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
}

/// Property variants for projection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProjectionVariant {
    /// Has projection homomorphism
    HasProjection,
    /// Has no projection homomorphism
    HasNoProjection,
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GeneralLinearGroup {
    /// The core algebraic group structure
    pub core: GenericGroup,
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SpecialLinearGroup {
    /// The underlying general linear group
    pub general_linear: GeneralLinearGroup,
    /// Properties specific to special linear groups
    pub special_linear_props: VariantSet<SpecialLinearProperty>,
}

/// Orthogonal group O(n)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OrthogonalGroup {
    /// The core algebraic group structure
    pub core: GenericGroup,
    /// The dimension
    pub dimension: u32, // todo: make it natural number
    /// Matrix specific properties
    pub matrix_props: VariantSet<MatrixProperty>,
}

/// Special orthogonal group SO(n)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SpecialOrthogonalGroup {
    /// The underlying orthogonal group
    pub orthogonal: OrthogonalGroup,
    /// Properties specific to special orthogonal groups
    pub special_orthogonal_props: VariantSet<SpecialOrthogonalProperty>,
}

/// Unitary group U(n)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UnitaryGroup {
    /// The core algebraic group structure
    pub core: GenericGroup,
    /// The dimension
    pub dimension: u32,
    /// Matrix specific properties
    pub matrix_props: VariantSet<MatrixProperty>,
}

/// Special unitary group SU(n)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SpecialUnitaryGroup {
    /// The underlying unitary group
    pub unitary: UnitaryGroup,
    /// Properties specific to special unitary groups
    pub special_unitary_props: VariantSet<SpecialUnitaryProperty>,
}

/// Alternating group A_n
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AlternatingGroup {
    /// The core algebraic group structure
    pub core: GenericGroup,
    /// The degree (n in A_n)
    pub degree: u32,
    /// Permutation specific properties
    pub perm_props: VariantSet<PermutationProperty>,
}

/// Modular additive group Z/nZ
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ModularAdditiveGroup {
    /// The core algebraic group structure
    pub core: GenericGroup,
    /// The modulus
    pub modulus: u32,
    /// Modular specific properties
    pub modular_props: VariantSet<ModularProperty>,
}

/// Multiplicative group of integers modulo n
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ModularMultiplicativeGroup {
    /// The core algebraic group structure
    pub core: GenericGroup,
    /// The modulus
    pub modulus: u32,
    /// Modular specific properties
    pub modular_props: VariantSet<ModularProperty>,
}

/// Free group F_n
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FreeGroup {
    /// The core algebraic group structure
    pub core: GenericGroup,
    /// The rank (number of generators)
    pub rank: u32,
    /// Free group specific properties
    pub free_props: VariantSet<FreeProperty>,
}

/// Quotient group G/N
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QuotientGroup {
    /// The core algebraic group structure
    pub core: GenericGroup,
    /// The group
    pub group: Arc<Group>,
    /// The normal subgroup
    pub normal_subgroup: Arc<Group>,
    /// Quotient specific properties
    pub quotient_props: VariantSet<QuotientProperty>,
}

impl QuotientGroup {
    /// Creates a new quotient group from a group and a normal subgroup
    pub fn new(group: Group, normal_subgroup: Group, is_maximal: bool) -> Self {
        QuotientGroup {
            core: GenericGroup::default(),
            group: Arc::new(group),
            normal_subgroup: Arc::new(normal_subgroup),
            quotient_props: VariantSet::new(),
        }
    }
}

/// The trivial group
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TrivialGroup {
    /// The core algebraic group structure
    pub core: GenericGroup,
}

// --- Structs for Flattened Group Constructions ---

/// A group defined as the kernel of a homomorphism
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct KernelGroup {
    pub core: GenericGroup,
    pub defining_homomorphism: Arc<GroupHomomorphism>,
    // Potentially add domain_group: Arc<Group> if needed for context
}

/// A group defined as the image of a homomorphism
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ImageGroup {
    pub core: GenericGroup,
    pub defining_homomorphism: Arc<GroupHomomorphism>,
    // Potentially add codomain_group: Arc<Group> if needed for context
}

/// A group defined as the center of another group: Z(G)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CenterGroup {
    pub core: GenericGroup,
    pub parent_group: Arc<Group>,
}

/// A group defined as a subgroup generated by a set of elements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GeneratedSubgroup {
    pub core: GenericGroup,
    pub parent_group: Arc<Group>,
    pub generators: Vec<GroupElement>, // Or GroupExpression?
}

/// A group defined as the normalizer of a subgroup: N_G(H)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NormalizerGroup {
    pub core: GenericGroup,
    pub parent_group: Arc<Group>,
    pub subgroup_normalized: Arc<Group>,
}

/// A group defined as the centralizer of an element: C_G(x)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CentralizerGroup {
    pub core: GenericGroup,
    pub parent_group: Arc<Group>,
    pub element_centralized: GroupElement, // Or GroupExpression?
}

/// A group defined as the commutator subgroup: [G,G]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CommutatorSubgroup {
    pub core: GenericGroup,
    pub parent_group: Arc<Group>,
}

/// A group defined as a Sylow p-subgroup: Syl_p(G)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SylowSubgroup {
    pub core: GenericGroup,
    pub parent_group: Arc<Group>,
    pub prime: u64, // Assuming prime is a number
}

/// A group defined as a wreath product
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WreathProductGroup {
    pub core: GenericGroup,
    pub base_group: Arc<Group>,
    pub acting_group: Arc<Group>,
    // Add action details if needed
}

/// A group defined as a central product
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CentralProductGroup {
    pub core: GenericGroup,
    pub component_groups: Vec<Arc<Group>>,
    pub center_identification_map: String, // Details on how centers are identified
}

/// A group defined as a pullback (fibered product)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PullbackGroup {
    pub core: GenericGroup,
    pub source_groups: Vec<Arc<Group>>, // Groups being mapped from
    pub target_group: Arc<Group>,       // Group being mapped to
    pub defining_homomorphisms: Vec<GroupHomomorphism>,
}

/// A group constructed by restricting to a specific subset satisfying group properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RestrictionGroup {
    pub core: GenericGroup,
    pub parent_group: Arc<Group>,
    pub restriction_description: String, // How the restriction is defined
}

impl Group {
    pub fn get_variant_name(&self) -> &'static str {
        match self {
            Group::Generic(_) => "Generic",
            Group::Trivial(_) => "Trivial",
            Group::Cyclic(_) => "Cyclic",
            Group::Dihedral(_) => "Dihedral",
            Group::Free(_) => "Free",
            Group::Symmetric(_) => "Symmetric",
            Group::Alternating(_) => "Alternating",
            Group::GeneralLinear(_) => "GeneralLinear",
            Group::SpecialLinear(_) => "SpecialLinear",
            Group::Orthogonal(_) => "Orthogonal",
            Group::SpecialOrthogonal(_) => "SpecialOrthogonal",
            Group::Unitary(_) => "Unitary",
            Group::SpecialUnitary(_) => "SpecialUnitary",
            Group::Topological(_) => "Topological",
            Group::Lie(_) => "Lie",
            Group::ModularAdditive(_) => "ModularAdditive",
            Group::ModularMultiplicative(_) => "ModularMultiplicative",
            Group::Product(_) => "Product",
            Group::Quotient(_) => "Quotient",
            Group::Kernel(_) => "Kernel",
            Group::Image(_) => "Image",
            Group::Center(_) => "Center",
            Group::GeneratedSubgroup(_) => "GeneratedSubgroup",
            Group::Normalizer(_) => "Normalizer",
            Group::Centralizer(_) => "Centralizer",
            Group::CommutatorSubgroup(_) => "CommutatorSubgroup",
            Group::SylowSubgroup(_) => "SylowSubgroup",
            Group::WreathProduct(_) => "WreathProduct",
            Group::CentralProduct(_) => "CentralProduct",
            Group::Pullback(_) => "Pullback",
            Group::Restriction(_) => "Restriction",
        }
    }

    pub fn new_generic() -> Self {
        Group::Generic(GenericGroup {
            base_set: Set::Generic(GenericSet::new()),
            operation: GroupOperation::default(),
            props: VariantSet::new(),
        })
    }
}
