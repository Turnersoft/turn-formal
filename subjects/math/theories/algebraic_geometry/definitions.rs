use crate::subjects::math::theories::VariantSet;
use crate::subjects::math::theories::{
    analysis::definition::functions::SmoothnessPropertyVariant, zfc::Set,
};
use serde::{Deserialize, Serialize};

/// A scheme is a locally ringed space (X,O_X) where:
/// - X is a topological space
/// - O_X is a sheaf of rings on X
/// - Each stalk O_X,p is a local ring
///
/// Key concepts:
/// - Affine schemes: Spec of a ring
/// - Structure sheaf: Local functions
/// - Morphisms: Compatible with ring structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Scheme {
    /// The underlying topological space
    pub base_space: Set,
    /// Properties of the scheme
    pub properties: VariantSet<SchemeProperty>,
}

/// Properties specific to schemes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SchemeProperty {
    /// Separated: Diagonal morphism is closed
    Separated(SeparatedPropertyVariant),
    /// Proper: Separated + universally closed
    Proper(ProperPropertyVariant),
    /// Smooth: Regular in characteristic 0
    Smooth(SmoothnessPropertyVariant),
    /// Projective: Admits closed embedding into P^n
    Projective(ProjectivityPropertyVariant),
    /// Noetherian: Ascending chain condition
    Noetherian(NoetherianPropertyVariant),
    /// Regular: Local rings are regular
    Regular(RegularityPropertyVariant),
    /// Normal: Local rings are integrally closed
    Normal(NormalityPropertyVariant),
}

/// Properties for separatedness of schemes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SeparatedPropertyVariant {
    /// Separated scheme
    Separated,
    /// Locally separated
    LocallySeparated,
    /// Non-separated
    NonSeparated,
}

/// Properties for properness of schemes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProperPropertyVariant {
    /// Proper scheme
    Proper,
    /// Locally proper
    LocallyProper,
    /// Non-proper
    NonProper,
}

/// Properties for projectivity of schemes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectivityPropertyVariant {
    /// Projective scheme
    Projective,
    /// Quasi-projective
    QuasiProjective,
    /// Non-projective
    NonProjective,
}

/// Properties for Noetherian schemes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NoetherianPropertyVariant {
    /// Noetherian scheme
    Noetherian,
    /// Locally Noetherian
    LocallyNoetherian,
    /// Non-Noetherian
    NonNoetherian,
}

/// Properties for regularity of schemes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RegularityPropertyVariant {
    /// Regular scheme
    Regular,
    /// Locally regular
    LocallyRegular,
    /// Singular
    Singular,
}

/// Properties for normality of schemes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NormalityPropertyVariant {
    /// Normal scheme
    Normal,
    /// Locally normal
    LocallyNormal,
    /// Non-normal
    NonNormal,
}

/// A variety is a reduced, separated scheme of finite type over a field k
/// Classical objects of algebraic geometry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Variety {
    /// The underlying scheme
    pub scheme: Scheme,
    /// The base field
    pub base_field: BaseField,
    /// Properties of the variety
    pub properties: VariantSet<VarietyProperty>,
}

/// Properties specific to varieties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VarietyProperty {
    /// Affine: Spec of finitely generated k-algebra
    Affine(AffinePropertyVariant),
    /// Projective: Closed subscheme of P^n
    Projective(ProjectivePropertyVariant),
    /// Quasi-projective: Open in projective variety
    QuasiProjective(QuasiProjectivePropertyVariant),
    /// Complete: Proper over base field
    Complete(CompletenessPropertyVariant),
    /// Rational: Birational to projective space
    Rational(RationalityPropertyVariant),
    /// Unirational: Dominated by projective space
    Unirational(UnirationialityPropertyVariant),
}

/// Properties for affine varieties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AffinePropertyVariant {
    /// Affine variety
    Affine,
    /// Locally affine
    LocallyAffine,
    /// Non-affine
    NonAffine,
}

/// Properties for projective varieties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectivePropertyVariant {
    /// Projective variety
    Projective,
    /// Locally projective
    LocallyProjective,
    /// Non-projective
    NonProjective,
}

/// Properties for quasi-projective varieties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuasiProjectivePropertyVariant {
    /// Quasi-projective variety
    QuasiProjective,
    /// Locally quasi-projective
    LocallyQuasiProjective,
    /// Not quasi-projective
    NonQuasiProjective,
}

/// Properties for completeness of varieties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompletenessPropertyVariant {
    /// Complete variety
    Complete,
    /// Locally complete
    LocallyComplete,
    /// Non-complete
    NonComplete,
}

/// Properties for rationality of varieties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RationalityPropertyVariant {
    /// Rational variety
    Rational,
    /// Unirational
    Unirational,
    /// Non-rational
    NonRational,
}

/// Properties for unirationality of varieties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UnirationialityPropertyVariant {
    /// Unirational variety
    Unirational,
    /// Locally unirational
    LocallyUnirational,
    /// Non-unirational
    NonUnirational,
}

/// A coherent sheaf F on a scheme X is a quasi-coherent O_X-module that:
/// - Is locally finitely presented
/// - Has coherent restriction to affine open sets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoherentSheaf {
    /// The underlying scheme
    pub base_scheme: Scheme,
    /// Properties of the sheaf
    pub properties: VariantSet<CoherentSheafProperty>,
}

/// Properties specific to coherent sheaves
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CoherentSheafProperty {
    /// Locally free: Locally isomorphic to O_X^n
    LocallyFree(LocallyFreePropertyVariant),
    /// Torsion free: No local torsion elements
    TorsionFree(TorsionFreePropertyVariant),
    /// Reflexive: Double dual isomorphism
    Reflexive(ReflexivityPropertyVariant),
    /// Normal: Pushforward under normal maps
    Normal(NormalityPropertyVariant),
}

/// Properties for local freeness of coherent sheaves
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LocallyFreePropertyVariant {
    /// Locally free sheaf
    LocallyFree(u32), // rank
    /// Not locally free
    NonLocallyFree,
}

/// Properties for torsion freeness of coherent sheaves
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TorsionFreePropertyVariant {
    /// Torsion free sheaf
    TorsionFree,
    /// Has torsion
    HasTorsion,
}

/// Properties for reflexivity of coherent sheaves
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReflexivityPropertyVariant {
    /// Reflexive sheaf
    Reflexive,
    /// Not reflexive
    NonReflexive,
}

/// Base field for varieties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BaseField {
    /// Complex numbers ℂ
    Complex,
    /// Real numbers ℝ
    Real,
    /// Finite field F_q
    Finite(u32),
    /// Algebraically closed field
    AlgebraicallyClosed,
    /// Perfect field
    Perfect,
}

// ... more definitions with detailed documentation
