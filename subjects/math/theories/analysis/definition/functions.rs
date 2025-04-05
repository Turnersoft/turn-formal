use crate::subjects::math::theories::topology::definitions::CompactProperty;
use crate::subjects::math::theories::zfc::set::Set;
use crate::subjects::math::theories::{
    common::spaces::Space, linear_algebra::definitions::VectorSpace, VariantSet,
};
use serde::{Deserialize, Serialize};

/// Function between spaces with properties
/// A mapping F: X → Y between two spaces X and Y that assigns to each element
/// x ∈ X exactly one element y ∈ Y. The function is characterized by its:
/// - Domain (X): The space of input values
/// - Codomain (Y): The space of possible output values
/// - Properties: Mathematical characteristics of the mapping
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Function {
    /// The domain X of the function, with its topological/metric structure
    pub domain: Space,
    /// The codomain Y of the function, with its topological/metric structure
    pub codomain: Space,
    /// Mathematical properties of the function
    pub properties: VariantSet<FunctionPropertyVariant>,
    /// Optional concrete representation
    pub concrete: Option<ConcreteFunction>,
}

/// Properties that can be possessed by functions
/// Each variant represents a distinct mathematical characteristic
/// that a function may or may not have
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FunctionPropertyVariant {
    /// Continuity: preservation of closeness between points
    /// f is continuous at x₀ if ∀ε>0 ∃δ>0: |x-x₀|<δ ⟹ |f(x)-f(x₀)|<ε
    Continuity(ContinuityPropertyVariant),

    /// Boundedness: existence of upper/lower bounds
    /// ∃M>0: |f(x)| ≤ M for all x in the domain
    Boundedness(BoundednessPropertyVariant),

    /// Open mapping: maps open sets to open sets
    /// U open in X ⟹ f(U) open in Y
    Open(OpenPropertyVariant),

    /// Closed mapping: maps closed sets to closed sets
    /// F closed in X ⟹ f(F) closed in Y
    Closed(ClosedPropertyVariant),

    /// Proper mapping: inverse images of compact sets are compact
    /// K compact in Y ⟹ f⁻¹(K) compact in X
    Proper(ProperPropertyVariant),

    /// Lipschitz continuity: bounded rate of change
    /// ∃L>0: |f(x)-f(y)| ≤ L|x-y| for all x,y
    Lipschitz(LipschitzPropertyVariant),

    /// Hölder continuity: generalized Lipschitz condition
    /// ∃C>0,α∈(0,1]: |f(x)-f(y)| ≤ C|x-y|ᵅ for all x,y
    Holder(HolderPropertyVariant),

    /// Analyticity: locally representable as power series
    /// f(x) = Σₙaₙ(x-x₀)ⁿ in some neighborhood of each point
    Analytic(AnalyticPropertyVariant),

    /// Integrability: existence of integral
    /// ∫|f| exists (in some specified sense)
    Integrability(IntegrabilityPropertyVariant),

    /// Differentiability: existence of derivatives
    /// limₕ→₀[f(x+h)-f(x)]/h exists
    Differentiability(DifferentiabilityPropertyVariant),

    /// Measurability: inverse images of measurable sets are measurable
    /// E measurable ⟹ f⁻¹(E) measurable
    Measurability(MeasurabilityPropertyVariant),

    /// Smoothness: degree of differentiability
    /// Existence and continuity of derivatives up to some order
    Smoothness(SmoothnessPropertyVariant),

    /// Monotonicity: order preservation
    /// x₁ ≤ x₂ ⟹ f(x₁) ≤ f(x₂) (or ≥)
    Monotonicity(MonotonicityPropertyVariant),

    /// Convexity: midpoint property
    /// f(tx+(1-t)y) ≤ tf(x)+(1-t)f(y) for t∈[0,1]
    Convexity(ConvexityPropertyVariant),

    /// Periodicity: repetition pattern
    /// ∃p≠0: f(x+p) = f(x) for all x
    Periodicity(PeriodicityPropertyVariant),

    /// Density: approximation property
    /// Closure of image contains specified set
    Density(DensityPropertyVariant),

    /// Compactness: for operators between function spaces
    /// Maps bounded sets to relatively compact sets
    Compactness(CompactnessPropertyVariant),
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

/// Density property variants
/// Describes how a subset approximates its ambient space:
/// - Dense: closure equals whole space
/// - Various topologies: strong, weak, pointwise
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DensityPropertyVariant {
    /// Set is dense in specified topology
    /// Closure equals whole space
    Dense(VariantSet<DensityProperty>),

    /// Set is not dense
    /// Closure is proper subset
    NonDense,
}

/// Properties modifying dense sets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DensityProperty {
    /// Where density holds
    Locality(LocalityPropertyVariant),

    /// Extent of density
    Coverage(CoveragePropertyVariant),

    /// Topology for density
    Topology(DensityTopologyVariant),
}

/// Types of topologies for density
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DensityTopologyVariant {
    /// Dense in norm topology
    /// ∀x ∃(xₙ): ‖xₙ-x‖→0
    Strong,

    /// Dense in weak topology
    /// ∀x ∃(xₙ): ⟨xₙ-x,φ⟩→0 ∀φ
    Weak,

    /// Dense in weak* topology
    /// ∀x* ∃(xₙ*): ⟨xₙ*-x*,x⟩→0 ∀x
    WeakStar,

    /// Dense pointwise
    /// ∀x ∃(xₙ): xₙ(t)→x(t) ∀t
    Pointwise,
}

/// Coverage variants describing the extent of a property
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CoveragePropertyVariant {
    /// Property holds without exception on X
    Everywhere,

    /// Property holds except on a null set N (μ(N)=0)
    Almost,

    /// Property holds on a dense subset D ⊆ X
    OnDense,

    /// Property holds on an open set U ⊆ X
    OnOpen,

    /// Property holds on a closed set F ⊆ X
    OnClosed,
}

/// Locality variants describing where a property holds
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LocalityPropertyVariant {
    /// Property holds on the entire space X
    Everywhere,

    /// Property holds in some open neighborhood U of each point
    OnNeighborhood,

    /// Property holds at a specific point x₀
    AtPoint,

    /// Property holds on a dense subset D ⊆ X
    OnDense,

    /// Property holds on compact subsets K ⊆ X
    OnCompact,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PeriodicityPropertyVariant {
    /// Strictly periodic: f(x+p)=f(x) for some p≠0
    Periodic,
    /// Almost periodic: f(x+pₙ)→f(x) for some sequence pₙ
    QuasiPeriodic,
    /// Not periodic in any sense
    Aperiodic,
}

/// Convexity property variants
/// A function f is convex if:
/// - f(tx + (1-t)y) ≤ tf(x) + (1-t)f(y) for t∈[0,1]
/// Concave functions satisfy the reverse inequality
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConvexityPropertyVariant {
    /// Function is convex
    /// f(tx + (1-t)y) ≤ tf(x) + (1-t)f(y)
    Convex(VariantSet<ConvexityProperty>),

    /// Function is concave
    /// f(tx + (1-t)y) ≥ tf(x) + (1-t)f(y)
    Concave(VariantSet<ConvexityProperty>),

    /// Neither convex nor concave
    Neither,
}

/// Properties modifying convex/concave functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConvexityProperty {
    /// Where convexity holds
    Locality(LocalityPropertyVariant),

    /// Extent of convexity
    Coverage(CoveragePropertyVariant),

    /// Strength of convexity (strict, uniform)
    Strength(StrengthPropertyVariant),
}

/// Strength variants describing how strongly a property holds
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StrengthPropertyVariant {
    /// Property holds in the strongest possible sense
    Strong,

    /// Property holds in a weaker topology/sense
    Weak,

    /// Property holds uniformly (same parameters throughout)
    Uniform,

    /// Property holds up to equivalence class
    Essential,
}

/// Monotonicity property variants
/// Describes order-preserving or order-reversing behavior:
/// - Increasing: x₁ < x₂ ⟹ f(x₁) ≤ f(x₂)
/// - Strictly increasing: x₁ < x₂ ⟹ f(x₁) < f(x₂)
/// - Decreasing: analogous with reversed inequalities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MonotonicityPropertyVariant {
    /// Function preserves or reverses order
    /// x₁ ≤ x₂ ⟹ f(x₁) ≤ f(x₂) (or ≥)
    Monotonic(VariantSet<MonotonicityProperty>),

    /// Function neither preserves nor reverses order
    /// ∃x₁<x₂: f(x₁)>f(x₂) and ∃y₁<y₂: f(y₁)<f(y₂)
    NonMonotonic,
}

/// Properties modifying monotonic functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MonotonicityProperty {
    /// Where monotonicity holds
    Locality(LocalityPropertyVariant),

    /// Extent of monotonicity
    Coverage(CoveragePropertyVariant),

    /// Direction of monotonicity
    Direction(MonotonicityDirectionVariant),

    /// Strength of monotonicity
    Strength(StrengthPropertyVariant),
}

/// Directions of monotonicity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MonotonicityDirectionVariant {
    /// Non-decreasing: x₁ ≤ x₂ ⟹ f(x₁) ≤ f(x₂)
    Increasing,

    /// Non-increasing: x₁ ≤ x₂ ⟹ f(x₁) ≥ f(x₂)
    Decreasing,

    /// Strictly increasing: x₁ < x₂ ⟹ f(x₁) < f(x₂)
    StrictlyIncreasing,

    /// Strictly decreasing: x₁ < x₂ ⟹ f(x₁) > f(x₂)
    StrictlyDecreasing,
}

/// Smoothness property variants
/// Describes the regularity/differentiability class of a function:
/// - C^k: k times continuously differentiable
/// - C^∞: infinitely differentiable
/// - Analytic: locally representable as power series
/// - Gevrey: intermediate between C^∞ and analytic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SmoothnessPropertyVariant {
    /// Function has specified smoothness class
    /// All derivatives up to specified order exist and are continuous
    Smooth(VariantSet<SmoothProperty>),

    /// Function lacks required smoothness
    /// Some derivative fails to exist or be continuous
    NonSmooth,
}

/// Properties modifying smooth functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SmoothProperty {
    /// Where smoothness holds
    Locality(LocalityPropertyVariant),

    /// Extent of smoothness
    Coverage(CoveragePropertyVariant),

    /// Smoothness class (C^k, C^∞, etc.)
    Class(SmoothnessClassVariant),
}

/// Classes of smoothness in differential geometry/analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SmoothnessClassVariant {
    /// k times continuously differentiable
    /// f⁽ⁿ⁾ exists and is continuous for n ≤ k
    Ck(u32),

    /// Infinitely differentiable
    /// f⁽ⁿ⁾ exists and is continuous for all n
    CInfinity,

    /// Real analytic function
    /// Locally equals its Taylor series
    RealAnalytic,

    /// Gevrey class function
    /// Derivatives bounded by |f⁽ⁿ⁾| ≤ C·M^n·(n!)^s
    Gevrey,
}

/// Variants of measurability for sets and functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MeasurabilityPropertyVariant {
    /// Set/function is measurable
    /// For functions: f⁻¹(E) measurable for all measurable E
    Measurable(VariantSet<MeasurableProperty>),

    /// Set/function is not measurable
    /// ∃ measurable E: f⁻¹(E) not measurable
    NonMeasurable,
}

/// Measurability property for sets and functions
/// For sets: belonging to a σ-algebra
/// For functions: inverse images of measurable sets are measurable
/// Key aspects:
/// - Borel measurability: inverse images of open sets are measurable
/// - Lebesgue measurability: inverse images of null sets are measurable
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MeasurableProperty {
    /// Extent of measurability
    Coverage(CoveragePropertyVariant),

    /// Strength of measurability
    Strength(StrengthPropertyVariant),

    /// Type of measure (Borel, Lebesgue)
    MeasureType(MeasureTypePropertyVariant),
}

/// Types of measures in measure theory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MeasureTypePropertyVariant {
    /// Generated by open sets
    /// σ-algebra generated by topology
    Borel,

    /// Complete measure
    /// E⊆N, μ(N)=0 ⟹ E is measurable
    Lebesgue,

    /// Both Borel and Lebesgue measurable
    Both,

    /// Neither Borel nor Lebesgue measurable
    Neither,
}

/// Differentiability property variants
/// Describes existence and properties of derivatives:
/// - Classical derivatives: limits of difference quotients
/// - Weak derivatives: integration by parts formula
/// - Distributional derivatives: action on test functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DifferentiabilityPropertyVariant {
    /// Function is differentiable
    /// limₕ→₀[f(x+h)-f(x)]/h exists
    Differentiable(VariantSet<DifferentiableProperty>),

    /// Function is not differentiable
    /// Limit of difference quotient doesn't exist
    NonDifferentiable,
}
/// Properties modifying differentiable functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DifferentiableProperty {
    /// Where differentiability holds
    Locality(LocalityPropertyVariant),

    /// Extent of differentiability
    Coverage(CoveragePropertyVariant),

    /// Order of derivative
    Order(DifferentiabilityOrderVariant),

    /// Strength of differentiability
    Strength(StrengthPropertyVariant),
}

/// Orders of differentiability
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DifferentiabilityOrderVariant {
    /// First derivative exists
    /// f' exists
    FirstOrder,

    /// Second derivative exists
    /// f'' exists
    SecondOrder,

    /// n-th derivative exists
    /// f⁽ⁿ⁾ exists
    NthOrder(u32),

    /// All derivatives exist
    /// f⁽ⁿ⁾ exists for all n
    Infinite,

    /// Distributional derivative
    /// ∫f·φ' = -∫f'·φ for test functions φ
    Distribution,
}
/// Variants of integrability for functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IntegrabilityPropertyVariant {
    /// Function is integrable in some sense
    /// ∫|f| exists (in specified sense)
    Integrable(VariantSet<IntegrableProperty>),

    /// Function is not integrable
    /// ∫|f| does not exist or is infinite
    NonIntegrable,
}

/// Integrability property for functions
/// Describes how a function can be integrated:
/// - Riemann: limits of sums over partitions
/// - Lebesgue: limits of simple function approximations
/// - Improper: limits of proper integrals
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IntegrableProperty {
    /// Extent of integrability
    Coverage(CoveragePropertyVariant),

    /// Type of integral (Riemann, Lebesgue, Improper)
    Mode(IntegrableTypePropertyVariant),

    /// Strength of integrability
    Strength(StrengthPropertyVariant),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IntegrableTypePropertyVariant {
    /// Lebesgue integrable: ∫|f|dμ < ∞
    Lebesgue,
    /// Riemann integrable: upper/lower sums converge
    Riemann,
    /// Improper integral exists: limₐ→∞∫₀ᵃf exists
    Improper,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnalyticPropertyVariant {
    /// Locally representable as power series
    /// f(x) = Σₙaₙ(x-x₀)ⁿ in some neighborhood
    Analytic,
    /// Not analytic
    NonAnalytic,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HolderPropertyVariant {
    /// Satisfies Hölder condition
    /// ∃C>0,α∈(0,1]: |f(x)-f(y)| ≤ C|x-y|ᵅ
    Holder,
    /// Does not satisfy Hölder condition
    NonHolder,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LipschitzPropertyVariant {
    /// Satisfies Lipschitz condition
    /// ∃L>0: |f(x)-f(y)| ≤ L|x-y| for all x,y
    Lipschitz,
    /// Does not satisfy Lipschitz condition
    NonLipschitz,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProperPropertyVariant {
    /// Inverse images of compact sets are compact
    /// K compact in Y ⟹ f⁻¹(K) compact in X
    Proper,
    /// Not proper
    NonProper,
}

/// Basic property variants for functions and spaces
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OpenPropertyVariant {
    /// Maps open sets to open sets
    /// U open in X ⟹ f(U) open in Y
    Open,
    /// Does not preserve openness
    NonOpen,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClosedPropertyVariant {
    /// Maps closed sets to closed sets
    /// F closed in X ⟹ f(F) closed in Y
    Closed,
    /// Does not preserve closedness
    NonClosed,
}

/// Variants of boundedness properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BoundednessPropertyVariant {
    /// Bounded metric space
    Bounded,
    /// Locally bounded
    LocallyBounded,
    /// Unbounded
    Unbounded,
}

/// Continuity property variants with precise mathematical definitions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContinuityPropertyVariant {
    /// Function preserves closeness between points
    /// ∀ε>0 ∃δ>0: |x-x₀|<δ ⟹ |f(x)-f(x₀)|<ε
    Continuous(VariantSet<ContinuousProperty>),

    /// Function has points where limit doesn't exist or match value
    /// ∃x₀: limₓ→ₓ₀ f(x) ≠ f(x₀) or doesn't exist
    Discontinuous(VariantSet<DiscontinuousProperty>),
}

/// Properties modifying continuous functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContinuousProperty {
    /// Where the continuity holds (globally, locally, etc.)
    Locality(LocalityPropertyVariant),

    /// Extent of continuity (everywhere, almost everywhere, etc.)
    Coverage(CoveragePropertyVariant),

    /// Strength of continuity (uniform, weak, etc.)
    Strength(StrengthPropertyVariant),
}

/// Properties modifying discontinuous functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiscontinuousProperty {
    /// Extent of discontinuity
    Coverage(CoveragePropertyVariant),

    /// Type of discontinuity (removable, jump, essential)
    Essentiality(EssentialityPropertyVariant),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EssentialityPropertyVariant {
    /// Cannot be made continuous by redefining at point
    Essential,
    /// Can be made continuous by redefining at point
    Removable,
    /// Left and right limits exist but differ
    Jump,
}

/// Represents the domain type for functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DomainType {
    /// Real numbers
    Real,
    /// Complex numbers
    Complex,
    /// Vector space over a field
    VectorSpace(Box<DomainType>), // field type
    /// Matrix space
    MatrixSpace {
        rows: usize,
        cols: usize,
        field: Box<DomainType>,
    },
    /// Function space
    FunctionSpace {
        domain: Box<DomainType>,
        codomain: Box<DomainType>,
    },
}

/// Represents well-defined mathematical functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConcreteFunction {
    /// Elementary transcendental functions
    Exp {
        domain: DomainType,
        branch: Option<i32>, // For complex domain
    },
    Log {
        domain: DomainType,
        branch: Option<i32>, // Principal branch by default
    },
    Sin {
        domain: DomainType,
    },
    Cos {
        domain: DomainType,
    },
    Tan {
        domain: DomainType,
    },

    /// Polynomial and rational functions
    Polynomial {
        coefficients: Vec<f64>, // In ascending order of degree
        domain: DomainType,
    },
    RationalFunction {
        numerator: Vec<f64>,   // Coefficients of numerator polynomial
        denominator: Vec<f64>, // Coefficients of denominator polynomial
        domain: DomainType,
    },

    /// Special functions with well-defined properties
    Gamma {
        domain: DomainType,
    },
    Zeta {
        domain: DomainType,
    },
    BesselJ {
        order: f64,
        domain: DomainType,
    },
    AiryAi {
        domain: DomainType,
    },
    ErrorFunction {
        domain: DomainType,
    },

    /// Basic operations preserving well-defined properties
    Sum {
        terms: Vec<Box<ConcreteFunction>>,
        domain: DomainType,
    },
    Product {
        factors: Vec<Box<ConcreteFunction>>,
        domain: DomainType,
    },
    Composition {
        outer: Box<ConcreteFunction>,
        inner: Box<ConcreteFunction>,
    },

    /// Classical test functions
    Bump {
        // C^∞ function with compact support
        center: f64,
        radius: f64,
    },
    Schwartz {
        // Rapidly decreasing C^∞ function
        kind: SchwartzKind,
    },
}

/// Types of Schwartz functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SchwartzKind {
    Gaussian,         // exp(-x^2)
    Lorentzian,       // 1/(1 + x^2)
    SuperExponential, // exp(-x^4)
}

impl ConcreteFunction {
    /// Validates if the domain type is valid for this function
    pub fn validate_domain(&self) -> bool {
        match self {
            // Transcendental functions only accept Real or Complex domains
            ConcreteFunction::Exp { domain, .. }
            | ConcreteFunction::Log { domain, .. }
            | ConcreteFunction::Sin { domain }
            | ConcreteFunction::Cos { domain }
            | ConcreteFunction::Tan { domain } => {
                matches!(domain, DomainType::Real | DomainType::Complex)
            }

            // Polynomial and rational functions accept Real, Complex, or VectorSpace domains
            ConcreteFunction::Polynomial { domain, .. }
            | ConcreteFunction::RationalFunction { domain, .. } => {
                matches!(
                    domain,
                    DomainType::Real | DomainType::Complex | DomainType::VectorSpace(_)
                )
            }

            // Special functions typically only accept Real or Complex domains
            ConcreteFunction::Gamma { domain }
            | ConcreteFunction::Zeta { domain }
            | ConcreteFunction::BesselJ { domain, .. }
            | ConcreteFunction::AiryAi { domain }
            | ConcreteFunction::ErrorFunction { domain } => {
                matches!(domain, DomainType::Real | DomainType::Complex)
            }

            // Operations inherit domain restrictions from their components
            ConcreteFunction::Sum { terms, .. } => terms.iter().all(|f| f.validate_domain()),
            ConcreteFunction::Product { factors, .. } => {
                factors.iter().all(|f| f.validate_domain())
            }

            ConcreteFunction::Composition { outer, inner } => {
                outer.validate_domain() && inner.validate_domain()
            }

            // Test functions are always defined on Real domain
            ConcreteFunction::Bump { .. } | ConcreteFunction::Schwartz { .. } => true,
        }
    }

    pub fn properties(&self) -> VariantSet<FunctionPropertyVariant> {
        // Create a new VariantSet
        let mut props = VariantSet::new();

        // match self {
        //     ConcreteFunction::Exp { domain, .. } => {
        //         props.insert(FunctionPropertyVariant::Differentiability(
        //             DifferentiabilityPropertyVariant::Differentiable(VariantSet::from_iter(vec![
        //                 DifferentiableProperty::Order(DifferentiabilityOrderVariant::Infinite),
        //             ])),
        //         ));
        //         props.insert(FunctionPropertyVariant::Analytic(
        //             AnalyticPropertyVariant::Analytic,
        //         ));

        //         match domain {
        //             DomainType::Real => {
        //                 props.insert(FunctionPropertyVariant::Monotonicity(
        //                     MonotonicityPropertyVariant::Monotonic(VariantSet::from_iter(vec![
        //                         MonotonicityProperty::Direction(
        //                             MonotonicityDirectionVariant::StrictlyIncreasing,
        //                         ),
        //                     ])),
        //                 ));
        //             }
        //             DomainType::Complex => {
        //                 props.insert(FunctionPropertyVariant::Periodicity(
        //                     PeriodicityPropertyVariant::Periodic,
        //                 ));
        //             }
        //             _ => {}
        //         }
        //     }
        //     ConcreteFunction::Polynomial { coefficients, .. } => {
        //         let degree = coefficients.len().saturating_sub(1);
        //         props.insert(FunctionPropertyVariant::Differentiability(
        //             DifferentiabilityPropertyVariant::Differentiable(VariantSet::from_iter(vec![
        //                 DifferentiableProperty::Order(DifferentiabilityOrderVariant::Infinite),
        //             ])),
        //         ));
        //         props.insert(FunctionPropertyVariant::Analytic(
        //             AnalyticPropertyVariant::Analytic,
        //         ));
        //         props.insert(FunctionPropertyVariant::AlgebraicDegree(degree as u32));
        //     }
        //     ConcreteFunction::Bump { .. } => {
        //         props.insert(FunctionPropertyVariant::Differentiability(
        //             DifferentiabilityPropertyVariant::Differentiable(VariantSet::from_iter(vec![
        //                 DifferentiableProperty::Order(DifferentiabilityOrderVariant::Infinite),
        //             ])),
        //         ));
        //         props.insert(FunctionPropertyVariant::Support(
        //             SupportPropertyVariant::Compact,
        //         ));
        //     }
        //     _ => {}
        // }
        props
    }

    pub fn domain_type(&self) -> DomainType {
        match self {
            ConcreteFunction::Exp { domain, .. } => domain.clone(),
            ConcreteFunction::Log { domain, .. } => domain.clone(),
            ConcreteFunction::Sin { domain } => domain.clone(),
            ConcreteFunction::Cos { domain } => domain.clone(),
            ConcreteFunction::Polynomial { domain, .. } => domain.clone(),
            ConcreteFunction::RationalFunction { domain, .. } => domain.clone(),
            ConcreteFunction::Bump { .. } => DomainType::Real,
            ConcreteFunction::Schwartz { .. } => DomainType::Real,
            _ => DomainType::Real, // Default case
        }
    }
}

/// Variants of boundedness for analytic functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnalyticBoundednessPropertyVariant {
    /// Function has upper/lower bounds
    /// ∃M>0: |f(x)| ≤ M for all x
    Bounded(VariantSet<AnalyticBoundedProperty>),

    /// Function is unbounded
    /// ∀M>0 ∃x: |f(x)| > M
    Unbounded,
}

/// Properties modifying bounded analytic functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnalyticBoundedProperty {
    /// Where the boundedness holds
    Locality(AnalyticLocalityPropertyVariant),

    /// Extent of boundedness
    Coverage(AnalyticCoveragePropertyVariant),

    /// Type of boundedness
    BoundednessType(AnalyticBoundednessTypeVariant),
}

/// Types of boundedness in analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnalyticBoundednessTypeVariant {
    /// Bounded in norm sense
    NormBounded,
    /// Bounded in measure sense
    MeasureBounded,
    /// Bounded in essential sense
    EssentiallyBounded,
}

/// Variants of continuity for analytic functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnalyticContinuityPropertyVariant {
    /// Function preserves limits
    /// ∀ε>0 ∃δ>0: |x-x₀|<δ ⟹ |f(x)-f(x₀)|<ε
    Continuous(VariantSet<AnalyticContinuousProperty>),

    /// Function has discontinuities
    /// ∃x₀: limₓ→ₓ₀ f(x) ≠ f(x₀) or doesn't exist
    Discontinuous,
}

/// Properties modifying continuous analytic functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnalyticContinuousProperty {
    /// Where the continuity holds
    Locality(AnalyticLocalityPropertyVariant),

    /// Extent of continuity
    Coverage(AnalyticCoveragePropertyVariant),

    /// Type of continuity
    ContinuityType(AnalyticContinuityTypeVariant),
}

/// Types of continuity in analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnalyticContinuityTypeVariant {
    /// Continuous in norm topology
    NormContinuous,
    /// Continuous in weak topology
    WeakContinuous,
    /// Continuous in weak* topology
    WeakStarContinuous,
}

/// Locality variants for analytic properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnalyticLocalityPropertyVariant {
    /// Property holds on entire domain
    Global,
    /// Property holds on open sets
    Local,
    /// Property holds at points
    PointWise,
    /// Property holds on compact sets
    OnCompact,
}

/// Coverage variants for analytic properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnalyticCoveragePropertyVariant {
    /// Property holds everywhere
    Complete,
    /// Property holds almost everywhere
    AlmostEverywhere,
    /// Property holds on a set of positive measure
    PositiveMeasure,
    /// Property holds on a residual set
    Residual,
}
