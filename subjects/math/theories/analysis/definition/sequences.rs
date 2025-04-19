use super::super::super::super::super::math::theories::zfc::set::Set;
use super::super::super::super::super::math::theories::{
    VariantSet, common::spaces::Space, linear_algebra::definitions::VectorSpace,
};
use serde::{Deserialize, Serialize};

use super::functions::{
    CoveragePropertyVariant, LocalityPropertyVariant, MonotonicityProperty,
    PeriodicityPropertyVariant, StrengthPropertyVariant,
};

/// Sequence in a space with properties
/// An ordered list of elements (xₙ)ₙ∈ℕ from a space X, where each element
/// is indexed by a natural number. Can be viewed as a function ℕ → X.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sequence {
    /// The space X containing the sequence elements, with its structure
    pub space: Space,
    /// Mathematical properties of the sequence
    pub properties: VariantSet<SequenceProperty>,
}

/// Properties that can be possessed by sequences
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SequenceProperty {
    /// Convergence: existence of a limit
    /// ∃L: ∀ε>0 ∃N: n>N ⟹ |xₙ-L|<ε
    Convergence(ConvergencePropertyVariant),

    /// Boundedness: existence of bounds
    /// ∃M>0: |xₙ| ≤ M for all n
    Boundedness(SequenceBoundednessPropertyVariant),

    /// Monotonicity: order relation between terms
    /// xₙ ≤ xₙ₊₁ (or ≥) for all n
    Monotonic(MonotonicPropertyVariant),

    /// Cauchy property: self-convergence
    /// ∀ε>0 ∃N: m,n>N ⟹ |xₘ-xₙ|<ε
    Cauchy(CauchyPropertyVariant),

    /// Continuity: existence of a limit
    /// limₙ→∞ xₙ exists and is finite
    Continuity(SequenceContinuityPropertyVariant),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CauchyPropertyVariant {
    /// Self-convergent sequence
    /// ∀ε>0 ∃N: m,n>N ⟹ |xₘ-xₙ|<ε
    Cauchy,
    /// Not Cauchy
    NonCauchy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MonotonicPropertyVariant {
    /// Preserves or reverses order
    /// x₁ ≤ x₂ ⟹ f(x₁) ≤ f(x₂) (or ≥)
    Monotonic(VariantSet<MonotonicityProperty>),

    /// Function neither preserves nor reverses order
    /// ∃x₁<x₂: f(x₁)>f(x₂) and ∃y₁<y₂: f(y₁)<f(y₂)
    NonMonotonic,
}

/// Variants of boundedness for sequences
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SequenceBoundednessPropertyVariant {
    /// Sequence has upper/lower bounds
    /// ∃M>0: |xₙ| ≤ M for all n
    Bounded(VariantSet<SequenceBoundedProperty>),

    /// Sequence is unbounded
    /// ∀M>0 ∃n: |xₙ| > M
    Unbounded,
}

/// Properties modifying bounded sequences
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SequenceBoundedProperty {
    /// Where the boundedness holds
    Locality(SequenceLocalityPropertyVariant),

    /// Extent of boundedness
    Coverage(SequenceCoveragePropertyVariant),

    /// Type of boundedness
    BoundednessType(SequenceBoundednessTypeVariant),
}

/// Types of boundedness for sequences
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SequenceBoundednessTypeVariant {
    /// Bounded in norm sense
    NormBounded,
    /// Bounded in measure sense
    MeasureBounded,
    /// Bounded in probability
    ProbabilityBounded,
}

/// Variants of continuity for sequences
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SequenceContinuityPropertyVariant {
    /// Sequence preserves limits
    /// limₙ→∞ xₙ exists and is finite
    Continuous(VariantSet<SequenceContinuousProperty>),

    /// Sequence has discontinuities
    /// limₙ→∞ xₙ does not exist or is infinite
    Discontinuous,
}

/// Properties modifying continuous sequences
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SequenceContinuousProperty {
    /// Where the continuity holds
    Locality(SequenceLocalityPropertyVariant),

    /// Extent of continuity
    Coverage(SequenceCoveragePropertyVariant),

    /// Type of continuity
    ContinuityType(SequenceContinuityTypeVariant),
}

/// Types of continuity for sequences
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SequenceContinuityTypeVariant {
    /// Convergent sequence
    Convergent,
    /// Cauchy sequence
    Cauchy,
    /// Fundamental sequence
    Fundamental,
}

/// Locality variants for sequence properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SequenceLocalityPropertyVariant {
    /// Property holds for all terms
    Global,
    /// Property holds for terms after some N
    Eventually,
    /// Property holds for infinitely many terms
    Frequently,
    /// Property holds for all but finitely many terms
    CoFinitely,
}

/// Coverage variants for sequence properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SequenceCoveragePropertyVariant {
    /// Property holds for all terms
    Complete,
    /// Property holds for a subsequence
    Subsequence,
    /// Property holds for terms in arithmetic progression
    ArithmeticProgression,
    /// Property holds for terms in geometric progression
    GeometricProgression,
}

/// Convergence property variants with precise definitions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConvergencePropertyVariant {
    /// Sequence/series has a limit
    /// ∃L: ∀ε>0 ∃N: n>N ⟹ |xₙ-L|<ε
    Convergent(VariantSet<ConvergentProperty>),

    /// Sequence/series grows without bound
    /// ∀M>0 ∃N: n>N ⟹ |xₙ|>M
    Divergent(VariantSet<DivergentProperty>),

    /// Sequence/series neither converges nor diverges
    /// No limit exists but sequence is bounded
    Oscillating(VariantSet<OscillatingProperty>),
}

/// Properties modifying convergent sequences/series
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConvergentProperty {
    /// Where the convergence holds
    Locality(LocalityPropertyVariant),

    /// Extent of convergence
    Coverage(CoveragePropertyVariant),

    /// Type of convergence (pointwise, uniform, etc.)
    Mode(ConvergenceModePropertyVariant),

    /// Strength of convergence
    Strength(StrengthPropertyVariant),

    /// Rate of convergence
    Rate(StrengthPropertyVariant),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConvergenceModePropertyVariant {
    /// Convergence at each point: ∀x limₙfₙ(x)=f(x)
    Pointwise,
    /// Uniform convergence: supₓ|fₙ(x)-f(x)|→0
    Uniform,
    /// Weak* topology convergence: ⟨fₙ,φ⟩→⟨f,φ⟩ ∀φ
    WeakStar,
    /// Strong topology convergence: ‖fₙ-f‖→0
    Strong,
    /// Convergence in measure: μ({x:|fₙ(x)-f(x)|>ε})→0
    InMeasure,
    /// Convergence in probability: P(|Xₙ-X|>ε)→0
    InProbability,
}

/// Properties modifying divergent sequences/series
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DivergentProperty {
    /// Rate of divergence to infinity
    Rate(StrengthPropertyVariant),

    /// Direction of divergence
    Direction(DivergenceDirectionVariant),
}

/// Direction of divergence for sequences
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DivergenceDirectionVariant {
    /// Diverges to positive infinity
    PositiveInfinity,
    /// Diverges to negative infinity
    NegativeInfinity,
    /// Diverges in oscillating manner
    Oscillating,
}

/// Properties modifying oscillating sequences/series
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OscillatingProperty {
    /// Strength of oscillation
    Strength(StrengthPropertyVariant),

    /// Pattern of oscillation
    Periodicity(PeriodicityPropertyVariant),
}
