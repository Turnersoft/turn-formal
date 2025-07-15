use super::super::super::subjects::math::formalism::expressions::MathExpression;
use super::super::super::subjects::math::theories::VariantSet;
use serde::{Deserialize, Serialize};

/// Module containing all property variants for category theory
pub mod properties {
    use super::*;

    /// Properties for smallness of categories
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum SmallnessPropertyVariant {
        /// Small category (objects form a set)
        Small,
        /// Locally small (hom-sets are sets)
        LocallySmall,
        /// Large category
        Large,
    }

    /// Properties for completeness of categories
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum CompletenessPropertyVariant {
        /// Has all small limits
        Complete,
        /// Has all finite limits
        FinitelyComplete,
        /// Not complete
        NonComplete,
    }

    /// Properties for cocompleteness of categories
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum CocompletenessPropertyVariant {
        /// Has all small colimits
        Cocomplete,
        /// Has all finite colimits
        FinitelyCocomplete,
        /// Not cocomplete
        NonCocomplete,
    }

    /// Properties for abelian categories
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum AbelianPropertyVariant {
        /// Abelian category
        Abelian,
        /// Pre-abelian
        PreAbelian,
        /// Quasi-abelian
        QuasiAbelian,
    }

    /// Properties for cartesian closed categories
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum CartesianClosednessPropertyVariant {
        /// Cartesian closed
        CartesianClosed,
        /// Locally cartesian closed
        LocallyCartesianClosed,
        /// Not cartesian closed
        NonCartesianClosed,
    }

    /// Properties for monoidal categories
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum MonoidalPropertyVariant {
        /// Strict monoidal
        Strict,
        /// Symmetric monoidal
        Symmetric,
        /// Braided monoidal
        Braided,
    }

    /// Properties for faithfulness of functors
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum FaithfulnessPropertyVariant {
        /// Faithful functor
        Faithful,
        /// Not faithful
        NonFaithful,
    }

    /// Properties for fullness of functors
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum FullnessPropertyVariant {
        /// Full functor
        Full,
        /// Not full
        NonFull,
    }

    /// Properties for essential surjectivity of functors
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum EssentialSurjectivityPropertyVariant {
        /// Essentially surjective
        EssentiallySurjective,
        /// Not essentially surjective
        NonEssentiallySurjective,
    }

    /// Properties for left adjointness of functors
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum LeftAdjointnessPropertyVariant {
        /// Left adjoint
        LeftAdjoint,
        /// Not left adjoint
        NonLeftAdjoint,
    }

    /// Properties for right adjointness of functors
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum RightAdjointnessPropertyVariant {
        /// Right adjoint
        RightAdjoint,
        /// Not right adjoint
        NonRightAdjoint,
    }

    /// Properties for natural isomorphisms
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum NaturalIsomorphismPropertyVariant {
        /// Natural isomorphism
        NaturalIso,
        /// Not natural isomorphism
        NonNaturalIso,
    }

    /// Properties for component-wise properties
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum ComponentPropertyVariant {
        /// Component-wise isomorphism
        ComponentIso,
        /// Component-wise monomorphism
        ComponentMono,
        /// Component-wise epimorphism
        ComponentEpi,
    }

    /// Properties for monadicity of adjunctions
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum MonadicityPropertyVariant {
        /// Monadic adjunction
        Monadic,
        /// Not monadic
        NonMonadic,
    }

    /// Properties for comonadicity of adjunctions
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum ComonadicityPropertyVariant {
        /// Comonadic adjunction
        Comonadic,
        /// Not comonadic
        NonComonadic,
    }

    /// Properties for idempotency
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum IdempotencyPropertyVariant {
        /// Idempotent
        Idempotent,
        /// Not idempotent
        NonIdempotent,
    }

    /// Properties for strength of monads
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum StrongPropertyVariant {
        /// Strong monad
        Strong,
        /// Not strong
        NonStrong,
    }

    /// Properties for commutativity of monads
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum CommutativityPropertyVariant {
        /// Commutative monad
        Commutative,
        /// Not commutative
        NonCommutative,
    }
}

// Re-export all property variants for easier access
pub use self::properties::*;

/// A category C consists of:
/// - A collection of objects Ob(C)
/// - For each pair of objects A,B ∈ Ob(C), a collection of morphisms Hom(A,B)
/// - For each triple of objects A,B,C, a composition operation
///   ∘: Hom(B,C) × Hom(A,B) → Hom(A,C)
/// - For each object A, an identity morphism id_A ∈ Hom(A,A)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Category {
    /// Name of the category
    pub name: String,
    /// Properties of the category
    pub properties: Vec<CategoryProperty>,
}

/// Properties of a category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CategoryProperty {
    /// Smallness properties
    Smallness(SmallnessPropertyVariant),
    /// Completeness properties
    Completeness(CompletenessPropertyVariant),
    /// Cocompleteness properties
    Cocompleteness(CocompletenessPropertyVariant),
    /// Abelian properties
    Abelian(AbelianPropertyVariant),
    /// Cartesian closedness properties
    CartesianClosedness(CartesianClosednessPropertyVariant),
    /// Monoidal properties
    Monoidal(MonoidalPropertyVariant),
}

/// A functor F: C → D consists of:
/// - For each object A ∈ C, an object F(A) ∈ D
/// - For each morphism f: A → B in C, a morphism F(f): F(A) → F(B) in D
/// - Preserves composition and identities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Functor {
    /// Name of the functor
    pub name: String,
    /// Source category
    pub source: Category,
    /// Target category
    pub target: Category,
    /// Properties of the functor
    pub properties: Vec<FunctorProperty>,
}

/// Properties of a functor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FunctorProperty {
    /// Faithfulness properties
    Faithfulness(FaithfulnessPropertyVariant),
    /// Fullness properties
    Fullness(FullnessPropertyVariant),
    /// Essential surjectivity properties
    EssentialSurjectivity(EssentialSurjectivityPropertyVariant),
    /// Left adjointness properties
    LeftAdjointness(LeftAdjointnessPropertyVariant),
    /// Right adjointness properties
    RightAdjointness(RightAdjointnessPropertyVariant),
}

/// A natural transformation α: F ⇒ G consists of:
/// - For each object A ∈ C, a morphism α_A: F(A) → G(A) in D
/// - For each morphism f: A → B in C, the naturality square commutes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NaturalTransformation {
    /// Name of the natural transformation
    pub name: String,
    /// Source functor
    pub source: Functor,
    /// Target functor
    pub target: Functor,
    /// Properties of the natural transformation
    pub properties: Vec<NaturalTransformationProperty>,
}

/// Properties of a natural transformation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NaturalTransformationProperty {
    /// Natural isomorphism properties
    NaturalIsomorphism(NaturalIsomorphismPropertyVariant),
    /// Component-wise properties
    ComponentWise(ComponentPropertyVariant),
}

/// An adjunction F ⊣ G consists of:
/// - A functor F: C → D (left adjoint)
/// - A functor G: D → C (right adjoint)
/// - Natural isomorphism Hom_D(F(A),B) ≅ Hom_C(A,G(B))
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Adjunction {
    /// Name of the adjunction
    pub name: String,
    /// Left adjoint functor
    pub left_adjoint: Functor,
    /// Right adjoint functor
    pub right_adjoint: Functor,
    /// Properties of the adjunction
    pub properties: Vec<AdjunctionProperty>,
}

/// Properties of an adjunction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AdjunctionProperty {
    /// Monadicity properties
    Monadicity(MonadicityPropertyVariant),
    /// Comonadicity properties
    Comonadicity(ComonadicityPropertyVariant),
    /// Idempotency properties
    Idempotency(IdempotencyPropertyVariant),
}

/// A monad T on a category C consists of:
/// - An endofunctor T: C → C
/// - A natural transformation η: 1_C ⇒ T (unit)
/// - A natural transformation μ: T∘T ⇒ T (multiplication)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Monad {
    /// Name of the monad
    pub name: String,
    /// Underlying endofunctor
    pub functor: Functor,
    /// Properties of the monad
    pub properties: Vec<MonadProperty>,
}

/// Properties of a monad
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MonadProperty {
    /// Strength properties
    Strength(StrongPropertyVariant),
    /// Commutativity properties
    Commutativity(CommutativityPropertyVariant),
    /// Idempotency properties
    Idempotency(IdempotencyPropertyVariant),
}

// Category theory relations

/// Entity information for category theory relation operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CategoryRelationEntity {
    /// Optional ID for referencing this relation
    pub id: Option<String>,

    /// Optional description explaining this relation instance  
    pub description: Option<String>,

    /// Optional key-value pairs for additional context
    pub tags: Vec<(String, String)>,
}

/// Relations specific to category theory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CategoryRelation {
    /// Relation indicating an object belongs to a category
    ObjectInCategory {
        entity: CategoryRelationEntity,
        object: MathExpression,
        category: MathExpression,
    },

    /// Relation indicating a morphism between objects in a category
    MorphismBetween {
        entity: CategoryRelationEntity,
        morphism: MathExpression,
        source: MathExpression,
        target: MathExpression,
        category: MathExpression,
    },

    /// Relation indicating composition of morphisms
    CompositionOf {
        entity: CategoryRelationEntity,
        result: MathExpression,
        first: MathExpression,
        second: MathExpression,
    },

    /// Relation indicating a morphism is an identity morphism
    IsIdentityMorphism {
        entity: CategoryRelationEntity,
        morphism: MathExpression,
        object: MathExpression,
    },

    /// Relation indicating a functor mapping between categories
    FunctorBetween {
        entity: CategoryRelationEntity,
        functor: MathExpression,
        source: MathExpression,
        target: MathExpression,
    },

    /// Relation indicating a natural transformation between functors
    NaturalTransformationBetween {
        entity: CategoryRelationEntity,
        transformation: MathExpression,
        source_functor: MathExpression,
        target_functor: MathExpression,
    },

    /// Relation indicating an adjunction between functors
    AdjunctionBetween {
        entity: CategoryRelationEntity,
        left_adjoint: MathExpression,
        right_adjoint: MathExpression,
    },

    /// Relation indicating a morphism is an isomorphism
    IsIsomorphism {
        entity: CategoryRelationEntity,
        morphism: MathExpression,
    },

    /// Relation indicating a morphism is a monomorphism (left-cancellative)
    IsMonomorphism {
        entity: CategoryRelationEntity,
        morphism: MathExpression,
    },

    /// Relation indicating a morphism is an epimorphism (right-cancellative)
    IsEpimorphism {
        entity: CategoryRelationEntity,
        morphism: MathExpression,
    },

    /// Relation for a limit in a category
    IsLimit {
        entity: CategoryRelationEntity,
        limit: MathExpression,
        diagram: MathExpression,
    },

    /// Relation for a colimit in a category
    IsColimit {
        entity: CategoryRelationEntity,
        colimit: MathExpression,
        diagram: MathExpression,
    },

    /// Custom category theory relation
    Custom {
        entity: CategoryRelationEntity,
        name: String,
        parameters: Vec<MathExpression>,
    },
}

// Helper methods for constructor functions
impl CategoryRelation {
    /// Create a new ObjectInCategory relation
    pub fn object_in_category(object: &MathExpression, category: &MathExpression) -> Self {
        let entity = CategoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        CategoryRelation::ObjectInCategory {
            entity,
            object: object.clone(),
            category: category.clone(),
        }
    }

    /// Create a new MorphismBetween relation
    pub fn morphism_between(
        morphism: &MathExpression,
        source: &MathExpression,
        target: &MathExpression,
        category: &MathExpression,
    ) -> Self {
        let entity = CategoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        CategoryRelation::MorphismBetween {
            entity,
            morphism: morphism.clone(),
            source: source.clone(),
            target: target.clone(),
            category: category.clone(),
        }
    }

    /// Create a new IsIsomorphism relation
    pub fn is_isomorphism(morphism: &MathExpression) -> Self {
        let entity = CategoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        CategoryRelation::IsIsomorphism {
            entity,
            morphism: morphism.clone(),
        }
    }

    /// Create a new FunctorBetween relation
    pub fn functor_between(
        functor: &MathExpression,
        source: &MathExpression,
        target: &MathExpression,
    ) -> Self {
        let entity = CategoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        CategoryRelation::FunctorBetween {
            entity,
            functor: functor.clone(),
            source: source.clone(),
            target: target.clone(),
        }
    }

    /// Create a custom relation
    pub fn custom(name: &str, parameters: Vec<MathExpression>) -> Self {
        let entity = CategoryRelationEntity {
            id: None,
            description: None,
            tags: Vec::new(),
        };
        CategoryRelation::Custom {
            entity,
            name: name.to_string(),
            parameters,
        }
    }
}

// ... more definitions with detailed documentation
