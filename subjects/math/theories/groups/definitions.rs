use super::super::super::super::math::formalism::expressions::MathExpression;
use super::super::super::super::math::formalism::relations::MathRelation;
use super::super::super::super::math::theories::VariantSet;
use super::super::super::super::math::theories::topology::definitions::TopologicalSpace;
use super::super::super::super::math::theories::zfc::set::{Set, SetProperty};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

//==== GROUP-SPECIFIC OPERATION TYPES ====//

/// Types of operations specific to group theory
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    /// Left inverse: b*a = e
    Left,
    /// Right inverse: a*b = e
    Right,
    /// Two-sided inverse: a*b = b*a = e (standard for groups)
    TwoSided,
}

/// Properties specific to group operations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroupOperationProperty {
    /// Whether the operation is associative (required for groups)
    Associative,
    /// Whether the operation is commutative
    Commutative(bool),
    /// Whether the operation is closed (required for groups)
    Closed,
}

/// Complete binary operation structure specific to group theory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
        }
    }
}

/// A group (G,·) is a set G with a binary operation · satisfying:
/// 1. Associativity: (a·b)·c = a·(b·c)
/// 2. Identity: ∃e ∈ G: e·a = a·e = a
/// 3. Inverses: ∀a ∈ G, ∃b ∈ G: a·b = b·a = e
///
/// Key concepts:
/// - Subgroups: Subsets closed under operation
/// - Cosets: Translations of subgroups
/// - Normal subgroups: Invariant under conjugation
/// - Quotient groups: G/N for normal N
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Group {
    /// The underlying set
    pub base_set: Set,
    /// The binary operation with its properties
    pub operation: GroupOperation,
    /// Properties specific to the group structure
    pub properties: Vec<GroupProperty>,
}

impl Default for Group {
    fn default() -> Self {
        Group {
            base_set: Set::empty(),
            operation: GroupOperation::default(),
            properties: Vec::new(),
        }
    }
}

/// A topological group is a group that is also a topological space,
/// where the group operations are continuous.
///
/// Key concepts:
/// - Continuous multiplication: G × G → G
/// - Continuous inversion: G → G
/// - Local structure: Neighborhoods of identity
/// - Haar measure: Invariant measure
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TopologicalGroup {
    /// The underlying group
    pub group: Group,
    /// Properties specific to the topological structure
    pub properties: Vec<TopologicalGroupProperty>,
}

/// A Lie group is a smooth manifold that is also a group,
/// where the group operations are smooth maps.
///
/// Key concepts:
/// - Lie algebra: Tangent space at identity
/// - Exponential map: Lie algebra → Lie group
/// - One-parameter subgroups
/// - Adjoint representation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LieGroup {
    /// The underlying topological group
    pub topological_group: TopologicalGroup,
    /// Properties specific to the Lie structure
    pub properties: Vec<LieGroupProperty>,
}

/// Properties specific to groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GroupProperty {
    /// Commutativity properties
    Abelian(AbelianPropertyVariant),

    /// Finiteness properties
    Finite(FinitePropertyVariant),

    /// Simple finite group
    FiniteGroup(bool),

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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GroupAction {
    /// Action on a set
    SetAction {
        /// The acting group
        group: Group,
        /// The space being acted on
        space: Set,
        /// The specific point in the space (if any)
        point: Option<Box<MathExpression>>,
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

// Fix the GroupAction Hash implementation to not use Set::Named
impl std::hash::Hash for GroupAction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash just the discriminant and minimal data to avoid recursion issues
        match self {
            GroupAction::SetAction { space, .. } => {
                std::mem::discriminant(self).hash(state);
                // Hash just the description/name of the space if available
                match space {
                    Set::Parametric { description, .. } => description.hash(state),
                    _ => "unknown_set".hash(state),
                }
            }
            GroupAction::VectorSpaceAction { space, .. } => {
                std::mem::discriminant(self).hash(state);
                space.hash(state);
            }
            GroupAction::TopologicalSpaceAction { space, .. } => {
                std::mem::discriminant(self).hash(state);
                space.hash(state);
            }
        }
    }
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

/// Entity information for group relation operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupRelationEntity {
    /// Optional ID for referencing this relation
    pub id: Option<String>,

    /// Optional description explaining this relation instance
    pub description: Option<String>,

    /// Optional key-value pairs for additional context
    pub tags: Vec<(String, String)>,
}

/// Relations specific to group theory
/// these are the verbs in the language of group theory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GroupRelation {
    /// One group is a subgroup of another
    IsSubgroupOf {
        entity: GroupRelationEntity,
        subgroup: MathExpression,
        group: MathExpression,
    },

    /// One group is a normal subgroup of another
    IsNormalSubgroupOf {
        entity: GroupRelationEntity,
        subgroup: MathExpression,
        group: MathExpression,
    },

    /// Two groups are isomorphic
    IsIsomorphicTo {
        entity: GroupRelationEntity,
        first: MathExpression,
        second: MathExpression,
    },

    /// One group is a quotient of another
    IsQuotientOf {
        entity: GroupRelationEntity,
        quotient: MathExpression,
        group: MathExpression,
        normal_subgroup: MathExpression,
    },

    /// Element is in the center of a group
    IsInCenterOf {
        entity: GroupRelationEntity,
        element: MathExpression,
        group: MathExpression,
    },

    /// Two elements are conjugate in a group
    AreConjugateIn {
        entity: GroupRelationEntity,
        element1: MathExpression,
        element2: MathExpression,
        group: MathExpression,
    },

    /// An element has a specified order in a group
    HasOrderInGroup {
        entity: GroupRelationEntity,
        element: MathExpression,
        group: MathExpression,
        order: usize,
    },

    /// A subgroup is of a specific index in a group
    HasIndexInGroup {
        entity: GroupRelationEntity,
        subgroup: MathExpression,
        group: MathExpression,
        index: usize,
    },

    /// A group has a specific number of elements
    HasOrder {
        entity: GroupRelationEntity,
        group: MathExpression,
        order: usize,
    },

    /// A group is cyclic with a specific generator
    IsCyclicWithGenerator {
        entity: GroupRelationEntity,
        group: MathExpression,
        generator: MathExpression,
    },

    /// An element normalizes a subgroup
    NormalizesSubgroup {
        entity: GroupRelationEntity,
        element: MathExpression,
        subgroup: MathExpression,
        group: MathExpression,
    },

    /// An element centralizes a subgroup
    CentralizesSubgroup {
        entity: GroupRelationEntity,
        element: MathExpression,
        subgroup: MathExpression,
        group: MathExpression,
    },

    /// A subgroup is characteristic
    IsCharacteristicSubgroupOf {
        entity: GroupRelationEntity,
        subgroup: MathExpression,
        group: MathExpression,
    },

    /// The order of one group divides the order of another
    OrderDivides {
        entity: GroupRelationEntity,
        group1: MathExpression,
        group2: MathExpression,
    },

    /// An element has a unique inverse in a group
    HasUniqueInverse {
        entity: GroupRelationEntity,
        element: MathExpression,
        group: MathExpression,
    },

    /// Sylow p-subgroup properties
    SylowSubgroupProperties {
        entity: GroupRelationEntity,
        prime: MathExpression,
        group: MathExpression,
    },

    /// One element is the inverse of another
    IsInverseOf {
        entity: GroupRelationEntity,
        element: MathExpression,
        inverse: MathExpression,
        group: MathExpression,
    },

    /// A homomorphism between groups
    IsHomomorphism {
        entity: GroupRelationEntity,
        homomorphism: MathExpression,
        domain: MathExpression,
        codomain: MathExpression,
    },

    /// An isomorphic embedding of one group into another
    IsomorphicEmbedding {
        entity: GroupRelationEntity,
        source: MathExpression,
        target: MathExpression,
    },
}

// Helper methods for backward compatibility
impl GroupRelation {
    /// Create a new IsSubgroupOf relation
    pub fn is_subgroup_of(subgroup: &MathExpression, group: &MathExpression) -> Self {
        let entity = GroupRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        GroupRelation::IsSubgroupOf {
            entity,
            subgroup: subgroup.clone(),
            group: group.clone(),
        }
    }

    /// Create a new IsNormalSubgroupOf relation
    pub fn is_normal_subgroup_of(subgroup: &MathExpression, group: &MathExpression) -> Self {
        let entity = GroupRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        GroupRelation::IsNormalSubgroupOf {
            entity,
            subgroup: subgroup.clone(),
            group: group.clone(),
        }
    }

    /// Create a new IsIsomorphicTo relation
    pub fn is_isomorphic_to(first: &MathExpression, second: &MathExpression) -> Self {
        let entity = GroupRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        GroupRelation::IsIsomorphicTo {
            entity,
            first: first.clone(),
            second: second.clone(),
        }
    }

    /// Create a new HasOrder relation
    pub fn has_order(group: &MathExpression, order: usize) -> Self {
        let entity = GroupRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        GroupRelation::HasOrder {
            entity,
            group: group.clone(),
            order,
        }
    }

    /// Create a new OrderDivides relation
    pub fn order_divides(group1: &MathExpression, group2: &MathExpression) -> Self {
        let entity = GroupRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        GroupRelation::OrderDivides {
            entity,
            group1: group1.clone(),
            group2: group2.clone(),
        }
    }
}

// Add constructors for proper mathematical expressions
impl GroupRelation {
    // Create a structured representation of a kernel
    pub fn kernel(homomorphism: &MathExpression) -> GroupExpression {
        // Create a variable representing the kernel operation
        let variable_name = "kernel_func";

        // Create a proper Set instead of using Set::default()
        let base_set = super::super::super::super::math::theories::zfc::Set::Parametric {
            parameters: std::collections::HashMap::new(),
            description: "Kernel set".to_string(),
            membership_condition: "x ∈ ker(ϕ)".to_string(),
            properties: super::super::super::super::math::theories::VariantSet::new(),
        };

        let group = Group {
            base_set,
            operation: GroupOperation {
                operation_type: GroupOperationVariant::Multiplication,
                notation: GroupNotation::Infix(GroupSymbol::Times),
                identity: GroupIdentity::One,
                inverse: GroupInverse::MultiplicativeInverse,
                inverse_application: GroupInverseApplication::TwoSided,
                properties: vec![GroupOperationProperty::Associative],
            },
            properties: Vec::new(),
        };

        // Use variable to represent the kernel function
        GroupExpression::variable(group, variable_name)
    }

    // Create a structured representation of an image
    pub fn image(homomorphism: &MathExpression) -> GroupExpression {
        // Create a variable representing the image operation
        let variable_name = "image_func";

        // Create a proper Set instead of using Set::default()
        let base_set = super::super::super::super::math::theories::zfc::Set::Parametric {
            parameters: std::collections::HashMap::new(),
            description: "Image set".to_string(),
            membership_condition: "x ∈ Im(ϕ)".to_string(),
            properties: super::super::super::super::math::theories::VariantSet::new(),
        };

        let group = Group {
            base_set,
            operation: GroupOperation {
                operation_type: GroupOperationVariant::Multiplication,
                notation: GroupNotation::Infix(GroupSymbol::Times),
                identity: GroupIdentity::One,
                inverse: GroupInverse::MultiplicativeInverse,
                inverse_application: GroupInverseApplication::TwoSided,
                properties: vec![GroupOperationProperty::Associative],
            },
            properties: Vec::new(),
        };

        // Use variable to represent the image function
        GroupExpression::variable(group, variable_name)
    }

    // Create a structured representation of a quotient group
    pub fn quotient_group(
        group: &MathExpression,
        normal_subgroup: &MathExpression,
    ) -> GroupExpression {
        // Create a placeholder group for the quotient with a proper Set
        let base_set = super::super::super::super::math::theories::zfc::Set::Parametric {
            parameters: std::collections::HashMap::new(),
            description: "Quotient group set".to_string(),
            membership_condition: "x ∈ G/N".to_string(),
            properties: super::super::super::super::math::theories::VariantSet::new(),
        };

        let quotient_group = Group {
            base_set,
            operation: GroupOperation {
                operation_type: GroupOperationVariant::Multiplication,
                notation: GroupNotation::Infix(GroupSymbol::Times),
                identity: GroupIdentity::One,
                inverse: GroupInverse::MultiplicativeInverse,
                inverse_application: GroupInverseApplication::TwoSided,
                properties: vec![GroupOperationProperty::Associative],
            },
            properties: Vec::new(),
        };

        // Use a variable to represent the quotient group
        GroupExpression::variable(quotient_group, "quotient_group")
    }

    // Create a structured representation of a symmetric group on a set
    pub fn symmetric_group(base_set: &MathExpression) -> GroupExpression {
        // Create a placeholder group for the symmetric group
        let sym_set = super::super::super::super::math::theories::zfc::Set::Parametric {
            parameters: std::collections::HashMap::new(),
            description: "Symmetric group set".to_string(),
            membership_condition: "x is a permutation".to_string(),
            properties: super::super::super::super::math::theories::VariantSet::new(),
        };

        let sym_group = Group {
            base_set: sym_set,
            operation: GroupOperation {
                operation_type: GroupOperationVariant::Composition,
                notation: GroupNotation::Infix(GroupSymbol::Circle),
                identity: GroupIdentity::IdentityPermutation,
                inverse: GroupInverse::PermutationInverse,
                inverse_application: GroupInverseApplication::TwoSided,
                properties: vec![GroupOperationProperty::Associative],
            },
            properties: Vec::new(),
        };

        // Use a variable to represent the symmetric group
        GroupExpression::variable(sym_group, "symmetric_group")
    }

    // Create an "element of" expression for group membership
    pub fn element_of_expr(element: &MathExpression, group: &MathExpression) -> MathRelation {
        // Create a SetTheoryRelation::ElementOf and wrap it in MathRelation::SetTheory
        let set_entity =
            super::super::super::super::math::theories::zfc::relations::SetTheoryRelationEntity {
                id: None,
                description: None,
                tags: Vec::new(),
            };

        MathRelation::SetTheory(
            super::super::super::super::math::theories::zfc::relations::SetTheoryRelation::ElementOf {
                entity: set_entity,
                element: element.clone(),
                set: group.clone(),
            },
        )
    }

    // Define a structured representation for "p divides n" where p and n are integers
    pub fn integer_divides(divisor: &MathExpression, dividend: &MathExpression) -> MathRelation {
        // Create a NumberTheoryRelation::Divides and wrap it in MathRelation::NumberTheory
        let num_entity = super::super::super::super::math::theories::number_theory::definitions::NumberTheoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };

        MathRelation::NumberTheory(
            super::super::super::super::math::theories::number_theory::definitions::NumberTheoryRelation::Divides {
                entity: num_entity,
                divisor: divisor.clone(),
                dividend: dividend.clone(),
            }
        )
    }

    // Add a structured representation for Sylow p-subgroups
    pub fn sylow_p_subgroup(prime: &MathExpression, group: &MathExpression) -> GroupExpression {
        // Create a placeholder group for the Sylow subgroup
        let sylow_set = super::super::super::super::math::theories::zfc::Set::Parametric {
            parameters: std::collections::HashMap::new(),
            description: "Sylow p-subgroup set".to_string(),
            membership_condition: "x ∈ Syl_p(G)".to_string(),
            properties: super::super::super::super::math::theories::VariantSet::new(),
        };

        let sylow_group = Group {
            base_set: sylow_set,
            operation: GroupOperation {
                operation_type: GroupOperationVariant::Multiplication,
                notation: GroupNotation::Infix(GroupSymbol::Times),
                identity: GroupIdentity::One,
                inverse: GroupInverse::MultiplicativeInverse,
                inverse_application: GroupInverseApplication::TwoSided,
                properties: vec![GroupOperationProperty::Associative],
            },
            properties: Vec::new(),
        };

        // Use a variable to represent the Sylow subgroup
        GroupExpression::variable(sylow_group, "sylow_p_subgroup")
    }

    // Add a structured representation for conjugate elements
    pub fn are_conjugate(
        x: &MathExpression,
        y: &MathExpression,
        group: &MathExpression,
    ) -> GroupRelation {
        let entity = GroupRelationEntity {
            id: None,
            description: Some("Elements are conjugate in the group".to_string()),
            tags: Vec::new(),
        };

        GroupRelation::AreConjugateIn {
            entity,
            element1: x.clone(),
            element2: y.clone(),
            group: group.clone(),
        }
    }

    // Create a structured representation for "is an inverse of" relation using MathRelation
    pub fn is_inverse_relation(x: &MathExpression, y: &MathExpression) -> MathRelation {
        // This should create a representation that x is the inverse of y in the group
        // First create the identity element
        let identity = MathExpression::Var(
            super::super::super::super::math::formalism::expressions::Identifier::O(111),
        );

        // Since BinaryOp is removed, we need to use GroupExpression and convert
        // Create a placeholder group
        let base_set = super::super::super::super::math::theories::zfc::Set::Parametric {
            parameters: std::collections::HashMap::new(),
            description: "Group set".to_string(),
            membership_condition: "x ∈ G".to_string(),
            properties: super::super::super::super::math::theories::VariantSet::new(),
        };

        let group = Group {
            base_set,
            operation: GroupOperation {
                operation_type: GroupOperationVariant::Multiplication,
                notation: GroupNotation::Infix(GroupSymbol::Times),
                identity: GroupIdentity::One,
                inverse: GroupInverse::MultiplicativeInverse,
                inverse_application: GroupInverseApplication::TwoSided,
                properties: vec![GroupOperationProperty::Associative],
            },
            properties: Vec::new(),
        };

        // Create x * y as GroupExpression
        let x_expr = GroupExpression::from_math_expression(x, &group)
            .unwrap_or_else(|_| GroupExpression::variable(group.clone(), "x"));
        let y_expr = GroupExpression::from_math_expression(y, &group)
            .unwrap_or_else(|_| GroupExpression::variable(group.clone(), "y"));

        let product = GroupExpression::operation(group, x_expr, y_expr);
        let product_math = product.to_math_expression();

        MathRelation::equal(product_math, identity)
    }

    // Add a structured representation for "is in the center of" relation
    pub fn is_in_center_of(element: &MathExpression, group: &MathExpression) -> GroupRelation {
        let entity = GroupRelationEntity {
            id: None,
            description: Some("Element is in the center of the group".to_string()),
            tags: Vec::new(),
        };

        GroupRelation::IsInCenterOf {
            entity,
            element: element.clone(),
            group: group.clone(),
        }
    }
}

/// Important abstract mathematical objects in group theory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GroupObject {
    /// The kernel of a homomorphism: Ker(φ)
    Kernel {
        /// Entity information
        entity: GroupRelationEntity,
        /// The homomorphism
        homomorphism: Box<MathExpression>,
    },

    /// The image of a homomorphism: Im(φ)
    Image {
        /// Entity information
        entity: GroupRelationEntity,
        /// The homomorphism
        homomorphism: Box<MathExpression>,
    },

    /// A quotient group: G/N
    QuotientGroup {
        /// Entity information
        entity: GroupRelationEntity,
        /// The group
        group: Box<MathExpression>,
        /// The normal subgroup
        normal_subgroup: Box<MathExpression>,
    },

    /// A symmetric group: Sym(G)
    SymmetricGroup {
        /// Entity information
        entity: GroupRelationEntity,
        /// The base set
        base_set: Box<MathExpression>,
    },

    /// A Sylow p-subgroup: Syl_p(G)
    SylowSubgroup {
        /// Entity information
        entity: GroupRelationEntity,
        /// The prime
        prime: Box<MathExpression>,
        /// The group
        group: Box<MathExpression>,
    },

    /// The center of a group: Z(G)
    Center {
        /// Entity information
        entity: GroupRelationEntity,
        /// The group
        group: Box<MathExpression>,
    },

    /// A centralizer: C_G(x)
    Centralizer {
        /// Entity information
        entity: GroupRelationEntity,
        /// The element being centralized
        element: Box<MathExpression>,
        /// The group
        group: Box<MathExpression>,
    },

    /// A normalizer: N_G(H)
    Normalizer {
        /// Entity information
        entity: GroupRelationEntity,
        /// The subgroup being normalized
        subgroup: Box<MathExpression>,
        /// The group
        group: Box<MathExpression>,
    },

    /// The commutator subgroup: [G,G]
    CommutatorSubgroup {
        /// Entity information
        entity: GroupRelationEntity,
        /// The group
        group: Box<MathExpression>,
    },

    /// The order of a group: |G|
    GroupOrder {
        /// Entity information
        entity: GroupRelationEntity,
        /// The group
        group: Box<MathExpression>,
    },

    /// The order of an element: |g|
    ElementOrder {
        /// Entity information
        entity: GroupRelationEntity,
        /// The element
        element: Box<MathExpression>,
        /// The group
        group: Box<MathExpression>,
    },
}

// Implementation for GroupObject
impl GroupObject {
    /// Create a kernel object
    pub fn kernel(homomorphism: &MathExpression) -> Self {
        let entity = GroupRelationEntity {
            id: None,
            description: Some("Kernel of a homomorphism".to_string()),
            tags: Vec::new(),
        };

        GroupObject::Kernel {
            entity,
            homomorphism: Box::new(homomorphism.clone()),
        }
    }

    /// Create an image object
    pub fn image(homomorphism: &MathExpression) -> Self {
        let entity = GroupRelationEntity {
            id: None,
            description: Some("Image of a homomorphism".to_string()),
            tags: Vec::new(),
        };

        GroupObject::Image {
            entity,
            homomorphism: Box::new(homomorphism.clone()),
        }
    }

    /// Create a quotient group object
    pub fn quotient_group(group: &MathExpression, normal_subgroup: &MathExpression) -> Self {
        let entity = GroupRelationEntity {
            id: None,
            description: Some("Quotient group".to_string()),
            tags: Vec::new(),
        };

        GroupObject::QuotientGroup {
            entity,
            group: Box::new(group.clone()),
            normal_subgroup: Box::new(normal_subgroup.clone()),
        }
    }

    /// Create a symmetric group object
    pub fn symmetric_group(base_set: &MathExpression) -> Self {
        let entity = GroupRelationEntity {
            id: None,
            description: Some("Symmetric group".to_string()),
            tags: Vec::new(),
        };

        GroupObject::SymmetricGroup {
            entity,
            base_set: Box::new(base_set.clone()),
        }
    }

    /// Create a Sylow p-subgroup object
    pub fn sylow_subgroup(prime: &MathExpression, group: &MathExpression) -> Self {
        let entity = GroupRelationEntity {
            id: None,
            description: Some("Sylow p-subgroup".to_string()),
            tags: Vec::new(),
        };

        GroupObject::SylowSubgroup {
            entity,
            prime: Box::new(prime.clone()),
            group: Box::new(group.clone()),
        }
    }

    /// Create a group order object |G|
    pub fn group_order(group: &MathExpression) -> Self {
        let entity = GroupRelationEntity {
            id: None,
            description: Some("Order of a group".to_string()),
            tags: Vec::new(),
        };

        GroupObject::GroupOrder {
            entity,
            group: Box::new(group.clone()),
        }
    }

    /// Create a element order object |g|
    pub fn element_order(element: &MathExpression, group: &MathExpression) -> Self {
        let entity = GroupRelationEntity {
            id: None,
            description: Some("Order of an element".to_string()),
            tags: Vec::new(),
        };

        GroupObject::ElementOrder {
            entity,
            element: Box::new(element.clone()),
            group: Box::new(group.clone()),
        }
    }

    /// Convert a GroupObject to a MathExpression
    pub fn to_expression(&self) -> MathExpression {
        use super::super::super::super::math::formalism::expressions::Identifier;

        // Create a simplified expression for each group object type
        match self {
            GroupObject::Kernel { homomorphism, .. } => {
                // Using Variable instead of Apply
                MathExpression::Var(Identifier::O(100))
            }
            GroupObject::Image { homomorphism, .. } => {
                // Using Variable instead of Apply
                MathExpression::Var(Identifier::O(101))
            }
            GroupObject::QuotientGroup {
                group,
                normal_subgroup,
                ..
            } => {
                // Using Variable instead of BinaryOp
                MathExpression::Var(Identifier::O(102))
            }
            GroupObject::SymmetricGroup { base_set, .. } => {
                // Using Variable instead of Apply
                MathExpression::Var(Identifier::O(103))
            }
            GroupObject::SylowSubgroup { prime, group, .. } => {
                // Using Variable instead of Apply
                MathExpression::Var(Identifier::O(104))
            }
            GroupObject::GroupOrder { group, .. } => {
                // Using Variable instead of Apply
                MathExpression::Var(Identifier::O(105))
            }
            GroupObject::ElementOrder { element, group, .. } => {
                // Using Variable instead of Apply
                MathExpression::Var(Identifier::O(106))
            }
            GroupObject::Center { group, .. } => {
                // Using Variable instead of Apply
                MathExpression::Var(Identifier::O(107))
            }
            GroupObject::Centralizer { element, group, .. } => {
                // Using Variable instead of Apply
                MathExpression::Var(Identifier::O(108))
            }
            GroupObject::Normalizer {
                subgroup, group, ..
            } => {
                // Using Variable instead of Apply
                MathExpression::Var(Identifier::O(109))
            }
            GroupObject::CommutatorSubgroup { group, .. } => {
                // Using Variable instead of Apply
                MathExpression::Var(Identifier::O(110))
            }
        }
    }
}

/// A representation of a group element for type-safe operations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupElement {
    /// The group this element belongs to
    pub group: Box<Group>,
    /// The underlying representation of the element (depends on the group's structure)
    pub value: ElementValue,
}

/// Different types of element values depending on the group structure
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementValue {
    /// A numeric element (useful for Z/nZ, etc.)
    Integer(i64),
    /// A permutation (for symmetric groups)
    Permutation(Vec<usize>),
    /// A matrix (for matrix groups)
    Matrix(Vec<Vec<i64>>),
    /// A symbolic element (for abstract elements)
    Symbol(String),
}

/// Represents operations in a group theory context
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GroupExpression {
    /// An element in a group
    Element(GroupElement),
    /// The identity element of a group
    Identity(Box<Group>),
    /// A group operation between two expressions
    Operation {
        /// The group this operation belongs to
        group: Box<Group>,
        /// The left operand
        left: Box<GroupExpression>,
        /// The right operand
        right: Box<GroupExpression>,
    },
    /// The inverse of an expression
    Inverse {
        /// The group this inverse belongs to
        group: Box<Group>,
        /// The element to invert
        element: Box<GroupExpression>,
    },
    /// A commutator of two elements
    Commutator {
        /// The group this commutator belongs to
        group: Box<Group>,
        /// The first element
        a: Box<GroupExpression>,
        /// The second element
        b: Box<GroupExpression>,
    },
    /// A coset of a subgroup
    Coset {
        /// The group this coset belongs to
        group: Box<Group>,
        /// The element for the coset
        element: Box<GroupExpression>,
        /// The subgroup
        subgroup: Box<Group>,
        /// Whether this is a left or right coset
        is_left: bool,
    },
    /// A variable referencing a group element
    Variable {
        /// The group this variable belongs to
        group: Box<Group>,
        /// The name of the variable
        name: String,
    },
    /// A group action
    Action {
        /// The element being acted on
        element: Box<GroupExpression>,
        /// The action being applied
        action: Box<GroupAction>,
    },
    /// Represents a power (exponentiation)
    Power {
        /// The base group
        group: Box<Group>,
        /// The base expression
        base: Box<GroupExpression>,
        /// The exponent
        exponent: i32,
    },
}

// GroupExpression methods
impl GroupExpression {
    /// Create a new element expression
    pub fn element(group: Group, value: ElementValue) -> Self {
        GroupExpression::Element(GroupElement::new(group, value))
    }

    /// Create a group operation
    pub fn operation(group: Group, left: GroupExpression, right: GroupExpression) -> Self {
        GroupExpression::Operation {
            group: Box::new(group),
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create an inverse expression
    pub fn inverse(group: Group, element: GroupExpression) -> Self {
        GroupExpression::Inverse {
            group: Box::new(group),
            element: Box::new(element),
        }
    }

    /// Create an identity element
    pub fn identity(group: Group) -> Self {
        GroupExpression::Identity(Box::new(group))
    }

    /// Create a commutator expression
    pub fn commutator(group: Group, a: GroupExpression, b: GroupExpression) -> Self {
        GroupExpression::Commutator {
            group: Box::new(group),
            a: Box::new(a),
            b: Box::new(b),
        }
    }

    /// Create a coset expression
    pub fn coset(group: Group, element: GroupExpression, subgroup: Group, is_left: bool) -> Self {
        GroupExpression::Coset {
            group: Box::new(group),
            element: Box::new(element),
            subgroup: Box::new(subgroup),
            is_left,
        }
    }

    /// Create a variable expression
    pub fn variable(group: Group, name: impl Into<String>) -> Self {
        GroupExpression::Variable {
            group: Box::new(group),
            name: name.into(),
        }
    }

    /// Create a group action expression
    pub fn action(element: GroupExpression, action: GroupAction) -> Self {
        GroupExpression::Action {
            element: Box::new(element),
            action: Box::new(action),
        }
    }

    /// Create a power expression
    pub fn power(group: Group, base: GroupExpression, exponent: i32) -> Self {
        GroupExpression::Power {
            group: Box::new(group),
            base: Box::new(base),
            exponent,
        }
    }

    /// Convert GroupExpression to MathExpression
    pub fn to_math_expression(&self) -> MathExpression {
        use super::super::super::super::math::formalism::expressions::{
            MathExpression, TheoryExpression,
        };
        MathExpression::Expression(TheoryExpression::Group(self.clone()))
    }

    /// Convert MathExpression to GroupExpression
    pub fn from_math_expression(expr: &MathExpression, group: &Group) -> Result<Self, String> {
        use super::super::super::super::math::formalism::expressions::{
            Identifier, TheoryExpression,
        };

        match expr {
            MathExpression::Expression(TheoryExpression::Group(group_expr)) => {
                // Direct conversion from group expression
                Ok(group_expr.clone())
            }
            MathExpression::Var(var) => {
                // Handle variables directly
                match var {
                    Identifier::O(id) => {
                        // Object variable with special handling
                        if *id == 200 {
                            // This is our special identity element
                            Ok(GroupExpression::Identity(Box::new(group.clone())))
                        } else {
                            // Other object variables become group variables
                            Ok(GroupExpression::Variable {
                                group: Box::new(group.clone()),
                                name: format!("var_{}", id),
                            })
                        }
                    }
                    _ => Err(format!("Unsupported variable type: {:?}", var)),
                }
            }
            // Handle other expression types as needed...
            _ => {
                // Default case: treat as an element directly
                Ok(GroupExpression::Element(GroupElement::new(
                    group.clone(),
                    ElementValue::Symbol("unknown".to_string()),
                )))
            }
        }
    }
}

/// Implement Eq trait for GroupExpression
impl Eq for GroupExpression {}

/// Implement Hash trait for GroupExpression
impl std::hash::Hash for GroupExpression {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            GroupExpression::Element(elem) => {
                // Hash the element's value
                std::mem::discriminant(self).hash(state);
                elem.value.hash(state);
            }
            GroupExpression::Operation { group, left, right } => {
                // Hash operation components
                std::mem::discriminant(self).hash(state);
                left.hash(state);
                right.hash(state);
                // We don't hash the group to avoid deep recursion
            }
            GroupExpression::Inverse { group, element } => {
                // Hash inverse components
                std::mem::discriminant(self).hash(state);
                element.hash(state);
                // We don't hash the group to avoid deep recursion
            }
            GroupExpression::Identity(group) => {
                // Hash identity discriminant
                std::mem::discriminant(self).hash(state);
                // We don't hash the group to avoid deep recursion
            }
            GroupExpression::Commutator { group, a, b } => {
                // Hash commutator components
                std::mem::discriminant(self).hash(state);
                a.hash(state);
                b.hash(state);
                // We don't hash the group to avoid deep recursion
            }
            GroupExpression::Coset {
                group,
                subgroup,
                element,
                is_left,
            } => {
                // Hash coset components
                std::mem::discriminant(self).hash(state);
                element.hash(state);
                is_left.hash(state);
                // We don't hash the groups to avoid deep recursion
            }
            GroupExpression::Action { element, action } => {
                // Hash action components
                std::mem::discriminant(self).hash(state);
                element.hash(state);
                // We don't hash the action to avoid deep recursion
            }
            GroupExpression::Power {
                group,
                base,
                exponent,
            } => {
                // Hash power components
                std::mem::discriminant(self).hash(state);
                base.hash(state);
                exponent.hash(state);
                // We don't hash the group to avoid deep recursion
            }
            GroupExpression::Variable { group, name } => {
                // Hash variable components
                std::mem::discriminant(self).hash(state);
                name.hash(state);
                // We don't hash the group to avoid deep recursion
            }
        }
    }
}

/// Error type for group expression evaluation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        point: MathExpression,
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
    /// Create a relation for element has unique inverse
    pub fn has_unique_inverse(element: &MathExpression, group: &MathExpression) -> Self {
        let entity = GroupRelationEntity {
            id: None,
            description: Some("Element has a unique inverse".to_string()),
            tags: Vec::new(),
        };

        GroupRelation::HasUniqueInverse {
            entity,
            element: element.clone(),
            group: group.clone(),
        }
    }

    /// Create a relation for Sylow p-subgroup properties
    pub fn sylow_subgroup_properties(prime: &MathExpression, group: &MathExpression) -> Self {
        let entity = GroupRelationEntity {
            id: None,
            description: Some("Sylow p-subgroup properties".to_string()),
            tags: Vec::new(),
        };

        GroupRelation::SylowSubgroupProperties {
            entity,
            prime: prime.clone(),
            group: group.clone(),
        }
    }

    /// Create a relation for "p divides |G|" using general integer division
    /// This returns a generic MathRelation rather than a GroupRelation
    pub fn divides_order_of(prime: &MathExpression, group: &MathExpression) -> MathRelation {
        // First get the group order as a MathExpression
        let group_order_obj = GroupObject::group_order(group);
        let group_order_expr = group_order_obj.to_expression();

        // Now use the general integer_divides relation from number theory
        Self::integer_divides(prime, &group_order_expr)
    }

    /// Create a relation for one element is the inverse of another
    pub fn is_inverse_of(
        element: &MathExpression,
        inverse: &MathExpression,
        group: &MathExpression,
    ) -> Self {
        let entity = GroupRelationEntity {
            id: None,
            description: Some("Element is the inverse of another".to_string()),
            tags: Vec::new(),
        };

        GroupRelation::IsInverseOf {
            entity,
            element: element.clone(),
            inverse: inverse.clone(),
            group: group.clone(),
        }
    }

    /// Create a relation for a homomorphism between groups
    pub fn is_homomorphism(
        homomorphism: &MathExpression,
        domain: &MathExpression,
        codomain: &MathExpression,
    ) -> Self {
        let entity = GroupRelationEntity {
            id: None,
            description: Some("Homomorphism between groups".to_string()),
            tags: Vec::new(),
        };

        GroupRelation::IsHomomorphism {
            entity,
            homomorphism: homomorphism.clone(),
            domain: domain.clone(),
            codomain: codomain.clone(),
        }
    }

    /// Create a relation for an isomorphic embedding
    pub fn isomorphic_embedding(source: &MathExpression, target: &MathExpression) -> Self {
        let entity = GroupRelationEntity {
            id: None,
            description: Some("Isomorphic embedding".to_string()),
            tags: Vec::new(),
        };

        GroupRelation::IsomorphicEmbedding {
            entity,
            source: source.clone(),
            target: target.clone(),
        }
    }
}

impl GroupElement {
    /// Create a new group element
    pub fn new(group: Group, value: ElementValue) -> Self {
        GroupElement {
            group: Box::new(group),
            value,
        }
    }

    /// Check if this element is the identity
    pub fn is_identity(&self) -> bool {
        match &self.value {
            ElementValue::Integer(n) => {
                // For Z/nZ-like groups
                if self.group.operation.operation_type == GroupOperationVariant::Addition {
                    *n == 0
                } else {
                    *n == 1
                }
            }
            ElementValue::Permutation(p) => {
                // For permutation groups, identity is [1,2,3,...,n]
                p.iter().enumerate().all(|(i, &val)| val == i + 1)
            }
            ElementValue::Symbol(s) => {
                // For symbolic groups
                s == "e" || s == "1" || s == "id"
            }
            _ => false, // More complex cases would need group-specific logic
        }
    }
}
