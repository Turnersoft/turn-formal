use super::super::definitions::{
    AlternatingGroup, CenterGroup, CentralProductGroup, CentralizerGroup, CommutatorSubgroup,
    CyclicGroup, DihedralGroup, FreeGroup, GeneralLinearGroup, GeneratedSubgroup, GenericGroup,
    Group, GroupElement, GroupExpression, GroupHomomorphism, GroupOperation, GroupOperationVariant,
    ImageGroup, KernelGroup, LieGroup, ModularAdditiveGroup, ModularMultiplicativeGroup,
    NormalizerGroup, OrthogonalGroup, ProductGroup, PullbackGroup, QuotientGroup, RestrictionGroup,
    SpecialLinearGroup, SpecialOrthogonalGroup, SpecialUnitaryGroup, SylowSubgroup, SymmetricGroup,
    TopologicalGroup, TrivialGroup, UnitaryGroup, WreathProductGroup,
};
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::traits::abstraction_level::{
    AbstractionLevel, GetAbstractionLevel,
};
use crate::subjects::math::theories::topology::definitions::TopologicalSpace;
use crate::subjects::math::theories::zfc::definitions::Set;

/// Implementation of abstraction levels for Group Theory objects

impl GetAbstractionLevel for Group {
    fn level(&self) -> AbstractionLevel {
        match self {
            // Basic group definition with the axioms is Level 1 or 2 depending on specifications
            &Group::Generic(ref basic) => {
                // Delegate to the basic group implementation
                basic.level()
            }

            // More specialized group types are Level 2 (partially specified)
            &Group::Topological(ref topological) => topological.level(),
            &Group::Lie(ref lie) => lie.core.level(),
            &Group::Cyclic(ref cyclic) => cyclic.core.level(),
            &Group::Symmetric(ref symmetric) => symmetric.level(),
            &Group::Dihedral(ref dihedral) => dihedral.level(),
            &Group::GeneralLinear(ref general_linear) => general_linear.level(),
            &Group::SpecialLinear(ref special_linear) => special_linear.level(),
            &Group::Orthogonal(ref orthogonal) => orthogonal.level(),
            &Group::SpecialOrthogonal(ref special_orthogonal) => special_orthogonal.level(),
            &Group::Unitary(ref unitary) => unitary.level(),
            &Group::SpecialUnitary(ref special_unitary) => special_unitary.level(),
            &Group::Alternating(ref alternating) => alternating.level(),
            &Group::ModularAdditive(ref modular_additive) => modular_additive.level(),
            &Group::ModularMultiplicative(ref modular_multiplicative) => {
                modular_multiplicative.level()
            }
            &Group::Free(ref free) => free.level(),
            &Group::Trivial(ref trivial) => trivial.level(),

            // L3 Constructor Variants (these structs now have their own GetAbstractionLevel)
            &Group::Product(ref product) => product.level(),
            &Group::Quotient(ref quotient) => quotient.level(),
            &Group::Kernel(ref kernel) => kernel.level(),
            &Group::Image(ref image) => image.level(),
            &Group::Center(ref center) => center.level(),
            &Group::GeneratedSubgroup(ref generated_subgroup) => generated_subgroup.level(),
            &Group::Normalizer(ref normalizer) => normalizer.level(),
            &Group::Centralizer(ref centralizer) => centralizer.level(),
            &Group::CommutatorSubgroup(ref commutator_subgroup) => commutator_subgroup.level(),
            &Group::SylowSubgroup(ref sylow_subgroup) => sylow_subgroup.level(),
            &Group::WreathProduct(ref wreath_product) => wreath_product.level(),
            &Group::CentralProduct(ref central_product) => central_product.level(),
            &Group::Pullback(ref pullback) => pullback.level(),
            &Group::Restriction(ref restriction) => restriction.level(),
            &Group::Interception(ref interception) => interception.core.level(),
            &Group::SubGroup(ref subgroup) => subgroup.core.level(),
        }
    }
}

impl GetAbstractionLevel for GenericGroup {
    fn level(&self) -> AbstractionLevel {
        let base_set_level = self.base_set.level();
        let operation_level = self.operation.level(); // L3 for the operation *type*

        // L1: Abstract set, abstract operation schema, no specific group properties.
        // (An L1 op would mean GroupOperation itself has an "AbstractOp" variant or similar)
        // Since GroupOperation is L3 (a type of op), an L1 group requires an L1 base_set.
        if base_set_level == AbstractionLevel::Level1 && self.props.inner.is_empty() {
            AbstractionLevel::Level1
        }
        // L4: Concrete base set, concrete operation (which GroupOperation doesn't fully capture, only its type).
        // If base_set is L4, and operation type is defined, it implies a specific group structure.
        // This GroupBasic would be L4 if we consider its operation to be fully defined by context with L4 base_set.
        else if base_set_level == AbstractionLevel::Level4 {
            // If self.props are also concrete L4-compatible, then L4.
            // If self.props add L2 abstract typing (e.g. "this GL2(R) is an L2 Lie Group Type"), then L2.
            if self.props.inner.is_empty() {
                // No further abstract typing by props
                AbstractionLevel::Level4
            } else {
                AbstractionLevel::Level2 // L4 components + L2 typing properties = L2 overall type
            }
        }
        // L2: Has specific group properties, or components are L2/mixed.
        else if !self.props.inner.is_empty() {
            AbstractionLevel::Level2
        }
        // L3: If base_set is an L3 constructor (e.g., Set::BinaryUnion).
        else if base_set_level == AbstractionLevel::Level3 {
            AbstractionLevel::Level3
        }
        // Default for other mixed cases (e.g., L2 base_set, L3 operation type).
        else {
            AbstractionLevel::Level2
        }
    }
}

impl GetAbstractionLevel for TopologicalGroup {
    fn level(&self) -> AbstractionLevel {
        let core_level = self.core.level();
        let topology_level = self.topology.level();
        if !self.props.inner.is_empty() {
            return AbstractionLevel::Level2;
        }
        // The level is the most abstract of its two main defining L1/L2/L4 components.
        // If one is L3 (e.g. core group is from an L3 Set constructor), then TG is L3.
        if core_level == AbstractionLevel::Level3 || topology_level == AbstractionLevel::Level3 {
            AbstractionLevel::Level3
        } else if core_level == AbstractionLevel::Level1
            || topology_level == AbstractionLevel::Level1
        {
            AbstractionLevel::Level1
        } else if core_level == AbstractionLevel::Level4
            && topology_level == AbstractionLevel::Level4
        {
            AbstractionLevel::Level4
        } else {
            AbstractionLevel::Level2
        }
    }
}

impl GetAbstractionLevel for CyclicGroup {
    fn level(&self) -> AbstractionLevel {
        let core_level = self.core.level();
        let generator_level = self.generator.level(); // L4 for a concrete element
        let order_defined = self.order.is_some();

        // If core is L1 and order is None (abstract), stay at L1
        if core_level == AbstractionLevel::Level1 && !order_defined {
            AbstractionLevel::Level1
        } else if core_level == AbstractionLevel::Level4
            && generator_level == AbstractionLevel::Level4
            && order_defined
        {
            AbstractionLevel::Level4 // e.g., Z_5 generated by concrete 1, with order 5 specified
        } else if order_defined || generator_level == AbstractionLevel::Level4 {
            // Specific order or specific generator implies a specific type of cyclic group (L2)
            // or an L3 constructor if the core is itself an L3 blueprint.
            if core_level == AbstractionLevel::Level3 {
                AbstractionLevel::Level3
            } else {
                AbstractionLevel::Level2
            }
        } else {
            // If order is not defined and generator is abstract (not possible with current GroupElement def),
            // level is driven by core_level.
            core_level
        }
    }
}

impl GetAbstractionLevel for LieGroup {
    fn level(&self) -> AbstractionLevel {
        let core_level = self.core.level();
        let topology_level = self.topology.level();
        let charts_level = if self.charts.is_empty() {
            // No charts specified, doesn't constrain beyond topology and core group.
            // Could be L1 if core/topology are L1. This depends on if "Lie Group" implies charts must exist conceptually.
            // Let's assume if core/topology are L1 and no charts, it's still an L1 Lie Group schema.
            AbstractionLevel::Level1 // Or, could be core_level.min(topology_level)
        } else {
            // Charts specified (even as strings) makes it at least an L2 type of Lie Group.
            AbstractionLevel::Level2
        };

        if !self.props.inner.is_empty() {
            return AbstractionLevel::Level2;
        } // Overriding L2 properties

        // Overall level is the most abstract of these three defining aspects.
        // (L1 < L2 < L3 < L4)
        let mut min_level = core_level.clone();
        if topology_level < min_level {
            min_level = topology_level.clone();
        }
        if charts_level < min_level {
            min_level = charts_level;
        }

        // If any part is L3 (e.g. core from L3 set), then LieGroup is L3.
        if core_level == AbstractionLevel::Level3 || topology_level == AbstractionLevel::Level3 {
            // charts_level being L3 is less common for this field
            return AbstractionLevel::Level3;
        }
        min_level
    }
}

impl GetAbstractionLevel for GroupElement {
    fn level(&self) -> AbstractionLevel {
        match self {
            GroupElement::Integer(_)
            | GroupElement::Permutation(_)
            | GroupElement::Matrix(_)
            | GroupElement::Symbol(_) => AbstractionLevel::Level4,
        }
    }
}

impl GetAbstractionLevel for GroupOperation {
    fn level(&self) -> AbstractionLevel {
        // This describes the *type* of operation. Considered L3 as a constructor/template for an operation.
        // A fully concrete operation instance (mapping specific elements to specific results)
        // would be L4, but GroupOperation itself is the definition.
        AbstractionLevel::Level3
    }
}

impl GetAbstractionLevel for GroupExpression {
    fn level(&self) -> AbstractionLevel {
        match self {
            GroupExpression::Element { group, element } => {
                let gl = group.level();
                let el = match element {
                    Some(param_element) => param_element.level(),
                    None => AbstractionLevel::Level1, // Unknown element treated as abstract
                };
                if gl == AbstractionLevel::Level4 && el == AbstractionLevel::Level4 {
                    AbstractionLevel::Level4
                } else if gl == AbstractionLevel::Level1 || el == AbstractionLevel::Level1 {
                    AbstractionLevel::Level1
                }
                // If one is L3 (e.g. group is Set::Union), expression involves L3 concept.
                else if gl == AbstractionLevel::Level3 || el == AbstractionLevel::Level3 {
                    AbstractionLevel::Level3
                } else {
                    AbstractionLevel::Level2
                }
            }
            GroupExpression::Identity(g) => g.level(),
            GroupExpression::Operation { .. }
            | GroupExpression::Inverse { .. }
            | GroupExpression::Commutator { .. }
            | GroupExpression::Coset { .. }
            | GroupExpression::ActionOnElement { .. }
            | GroupExpression::Power { .. } => AbstractionLevel::Level3, // These are constructors for new group elements/expressions

            GroupExpression::GroupOrder { group } => {
                let gl = group.level();
                if gl == AbstractionLevel::Level4 {
                    AbstractionLevel::Level4
                }
                // Order of a concrete group is a concrete number (L4 value)
                else {
                    AbstractionLevel::Level2
                } // Concept of "order of group G" where G is L1/L2/L3
            }
            GroupExpression::ElementOrder { element, group } => {
                // Similar to GroupOrder
                let el = element.level();
                let gl = group.level();
                if el == AbstractionLevel::Level4 && gl == AbstractionLevel::Level4 {
                    AbstractionLevel::Level4
                } else {
                    AbstractionLevel::Level2
                }
            }
            GroupExpression::Homomorphism(h_param) => h_param.level(),
        }
    }
}

// Helper for standard group types that wrap a core and add structural identity.
// For L1 abstract definitions, these should remain L1 if they use abstract parameters
fn level_for_standard_group_wrapper(core_level: AbstractionLevel) -> AbstractionLevel {
    match core_level {
        AbstractionLevel::Level4 => AbstractionLevel::Level4,
        AbstractionLevel::Level3 => AbstractionLevel::Level3, // e.g. SymmetricGroup on an L3 Set
        AbstractionLevel::Level1 => AbstractionLevel::Level1, // L1 core with abstract parameters stays L1
        _ => AbstractionLevel::Level2, // L2 core results in an L2 specific group type
    }
}

impl GetAbstractionLevel for SymmetricGroup {
    fn level(&self) -> AbstractionLevel {
        let core_level = self.core.level();
        // If degree is 0, it represents abstract S_n - stay at L1
        // If degree is concrete, it becomes L2 (specific symmetric group type)
        if core_level == AbstractionLevel::Level1 && self.degree == 0 {
            AbstractionLevel::Level1
        } else {
            level_for_standard_group_wrapper(core_level)
        }
    }
}
impl GetAbstractionLevel for DihedralGroup {
    fn level(&self) -> AbstractionLevel {
        let core_level = self.core.level();
        // If order is 0, it represents abstract D_n - stay at L1
        if core_level == AbstractionLevel::Level1 && self.order == 0 {
            AbstractionLevel::Level1
        } else {
            level_for_standard_group_wrapper(core_level)
        }
    }
}
impl GetAbstractionLevel for GeneralLinearGroup {
    fn level(&self) -> AbstractionLevel {
        let core_level = self.core.level();
        // If dimension is 0, it represents abstract GL(n,F) - stay at L1
        if core_level == AbstractionLevel::Level1 && self.dimension == 0 {
            AbstractionLevel::Level1
        } else {
            level_for_standard_group_wrapper(core_level)
        }
    }
}
impl GetAbstractionLevel for SpecialLinearGroup {
    fn level(&self) -> AbstractionLevel {
        self.general_linear.level()
    }
} // Defers
impl GetAbstractionLevel for OrthogonalGroup {
    fn level(&self) -> AbstractionLevel {
        let core_level = self.core.level();
        // If dimension is 0, it represents abstract O(n) - stay at L1
        if core_level == AbstractionLevel::Level1 && self.dimension == 0 {
            AbstractionLevel::Level1
        } else {
            level_for_standard_group_wrapper(core_level)
        }
    }
}
impl GetAbstractionLevel for SpecialOrthogonalGroup {
    fn level(&self) -> AbstractionLevel {
        self.orthogonal.level()
    }
} // Defers
impl GetAbstractionLevel for UnitaryGroup {
    fn level(&self) -> AbstractionLevel {
        let core_level = self.core.level();
        // If dimension is 0, it represents abstract U(n) - stay at L1
        if core_level == AbstractionLevel::Level1 && self.dimension == 0 {
            AbstractionLevel::Level1
        } else {
            level_for_standard_group_wrapper(core_level)
        }
    }
}
impl GetAbstractionLevel for SpecialUnitaryGroup {
    fn level(&self) -> AbstractionLevel {
        self.unitary.level()
    }
} // Defers
impl GetAbstractionLevel for AlternatingGroup {
    fn level(&self) -> AbstractionLevel {
        let core_level = self.core.level();
        // If degree is 0, it represents abstract A_n - stay at L1
        if core_level == AbstractionLevel::Level1 && self.degree == 0 {
            AbstractionLevel::Level1
        } else {
            level_for_standard_group_wrapper(core_level)
        }
    }
}
impl GetAbstractionLevel for ModularAdditiveGroup {
    fn level(&self) -> AbstractionLevel {
        let core_level = self.core.level();
        // If modulus is 0, it represents abstract ℤ/nℤ - stay at L1
        if core_level == AbstractionLevel::Level1 && self.modulus == 0 {
            AbstractionLevel::Level1
        } else {
            level_for_standard_group_wrapper(core_level)
        }
    }
}
impl GetAbstractionLevel for ModularMultiplicativeGroup {
    fn level(&self) -> AbstractionLevel {
        let core_level = self.core.level();
        // If modulus is 0, it represents abstract (ℤ/nℤ)* - stay at L1
        if core_level == AbstractionLevel::Level1 && self.modulus == 0 {
            AbstractionLevel::Level1
        } else {
            level_for_standard_group_wrapper(core_level)
        }
    }
}
impl GetAbstractionLevel for FreeGroup {
    fn level(&self) -> AbstractionLevel {
        let core_level = self.core.level();
        // If rank is 0, it represents abstract F_n - stay at L1
        if core_level == AbstractionLevel::Level1 && self.rank == 0 {
            AbstractionLevel::Level1
        } else {
            level_for_standard_group_wrapper(core_level)
        }
    }
}
impl GetAbstractionLevel for TrivialGroup {
    fn level(&self) -> AbstractionLevel {
        AbstractionLevel::Level4
    }
}

// L3/L4 Constructor Structs
fn level_for_constructor_struct(
    core_level: AbstractionLevel,
    component_levels: Vec<AbstractionLevel>,
) -> AbstractionLevel {
    let mut overall_level = core_level;
    for comp_level in component_levels {
        if comp_level < overall_level {
            overall_level = comp_level;
        }
    }
    // **FIXED LOGIC**: If all components (including core) are L1, this is an L1 abstract schema
    // If all components are L4, this is an L4 concrete construction
    // Otherwise, it's an L3 constructor with mixed abstraction levels
    if overall_level == AbstractionLevel::Level1 {
        AbstractionLevel::Level1 // L1 abstract schemas stay L1
    } else if overall_level == AbstractionLevel::Level4 {
        AbstractionLevel::Level4 // L4 concrete constructions stay L4
    } else {
        AbstractionLevel::Level3 // Mixed or L2/L3 components become L3 constructors
    }
}

impl GetAbstractionLevel for ProductGroup {
    fn level(&self) -> AbstractionLevel {
        level_for_constructor_struct(
            self.core.level(),
            self.components.iter().map(|c| c.level()).collect(),
        )
    }
}
impl GetAbstractionLevel for QuotientGroup {
    fn level(&self) -> AbstractionLevel {
        level_for_constructor_struct(
            self.core.level(),
            vec![self.group.level(), self.normal_subgroup.level()],
        )
    }
}
impl GetAbstractionLevel for KernelGroup {
    fn level(&self) -> AbstractionLevel {
        level_for_constructor_struct(self.core.level(), vec![self.defining_homomorphism.level()])
    }
}
impl GetAbstractionLevel for ImageGroup {
    fn level(&self) -> AbstractionLevel {
        level_for_constructor_struct(self.core.level(), vec![self.defining_homomorphism.level()])
    }
}
impl GetAbstractionLevel for CenterGroup {
    fn level(&self) -> AbstractionLevel {
        level_for_constructor_struct(self.core.level(), vec![self.parent_group.level()])
    }
}
// For these, the structure itself implies L3 construction. L4 if all inputs are L4 and result is concrete.
impl GetAbstractionLevel for GeneratedSubgroup {
    fn level(&self) -> AbstractionLevel {
        level_for_constructor_struct(self.core.level(), vec![self.parent_group.level()])
    }
}
impl GetAbstractionLevel for NormalizerGroup {
    fn level(&self) -> AbstractionLevel {
        level_for_constructor_struct(
            self.core.level(),
            vec![self.parent_group.level(), self.subgroup_normalized.level()],
        )
    }
}
impl GetAbstractionLevel for CentralizerGroup {
    fn level(&self) -> AbstractionLevel {
        level_for_constructor_struct(
            self.core.level(),
            vec![self.parent_group.level(), self.element_centralized.level()],
        )
    }
}
impl GetAbstractionLevel for CommutatorSubgroup {
    fn level(&self) -> AbstractionLevel {
        level_for_constructor_struct(self.core.level(), vec![self.parent_group.level()])
    }
}
impl GetAbstractionLevel for SylowSubgroup {
    fn level(&self) -> AbstractionLevel {
        level_for_constructor_struct(self.core.level(), vec![self.parent_group.level()])
    }
} // prime field also a factor
impl GetAbstractionLevel for WreathProductGroup {
    fn level(&self) -> AbstractionLevel {
        level_for_constructor_struct(
            self.core.level(),
            vec![self.base_group.level(), self.acting_group.level()],
        )
    }
}
impl GetAbstractionLevel for CentralProductGroup {
    fn level(&self) -> AbstractionLevel {
        level_for_constructor_struct(
            self.core.level(),
            self.component_groups.iter().map(|c| c.level()).collect(),
        )
    }
}
impl GetAbstractionLevel for PullbackGroup {
    fn level(&self) -> AbstractionLevel {
        let mut comp_levels = vec![self.target_group.level()];
        comp_levels.extend(self.source_groups.iter().map(|sg| sg.level()));
        comp_levels.extend(self.defining_homomorphisms.iter().map(|h| h.level()));
        level_for_constructor_struct(self.core.level(), comp_levels)
    }
}
impl GetAbstractionLevel for RestrictionGroup {
    fn level(&self) -> AbstractionLevel {
        level_for_constructor_struct(self.core.level(), vec![self.parent_group.level()])
    }
}

impl GetAbstractionLevel for GroupHomomorphism {
    fn level(&self) -> AbstractionLevel {
        let dom_level = self.domain.level();
        let cod_level = self.codomain.level();
        // Definition of the map itself also matters. If it's a variable/placeholder for "a homomorphism", it's L1.
        // If it's a concrete map like x -> x^2, then its level depends on domain/codomain.
        // Assuming this struct represents a defined homomorphism, not just "any possible".
        if dom_level == AbstractionLevel::Level4 && cod_level == AbstractionLevel::Level4 {
            AbstractionLevel::Level4 // Concrete map between concrete groups
        } else if dom_level == AbstractionLevel::Level1 || cod_level == AbstractionLevel::Level1 {
            AbstractionLevel::Level1 // Homomorphism involving a schematic group
        } else {
            AbstractionLevel::Level2 // Homomorphism between L2/L3/L4 groups (generic type of map)
        }
    }
}

// Need GetAbstractionLevel for Parametrizable<T>
// This is crucial and was missing from direct thought process for groups.
// It should live in formalism::extract or formalism::abstraction_level.
// For now, I will define a generic one here for compilation, assuming T: GetAbstractionLevel.

impl<T: GetAbstractionLevel + Clone> GetAbstractionLevel for Parametrizable<T> {
    fn level(&self) -> AbstractionLevel {
        match self {
            Parametrizable::Concrete(c) => c.level(),
            Parametrizable::Variable(_) => AbstractionLevel::Level1,
        }
    }
}

// Placeholder for TopologicalSpace GetAbstractionLevel - should be in its own module
// This is added here to allow groups/abstraction_level.rs to compile.
// TODO: Move this to a proper topology/abstraction_level.rs file.
impl GetAbstractionLevel for TopologicalSpace {
    fn level(&self) -> AbstractionLevel {
        let base_set_level = self.base_set.level();
        // Consider the topology definition itself. If self.topology.properties are very specific,
        // or if it implies a concrete collection of open sets, it leans more concrete.
        // For simplicity, let's check if the topology has any specific properties.
        let topology_definition_level = if self.topology.properties.inner.is_empty() {
            AbstractionLevel::Level1 // "an abstract topology"
        } else {
            AbstractionLevel::Level2 // "a topology with certain properties (compact, etc.)"
        };

        // Check properties on the TopologicalSpace itself
        if !self.properties.is_empty() {
            // If TopologicalSpace has properties like T0, Hausdorff, Metrizable, it's an L2 type.
            return AbstractionLevel::Level2;
        }

        // If no overriding properties on TopologicalSpace itself, level is determined by components.
        // L1 < L2 < L3 < L4 (requires Ord on AbstractionLevel)
        let mut overall_level = base_set_level.clone();
        if topology_definition_level < overall_level {
            overall_level = topology_definition_level.clone();
        }

        // If base_set or topology definition is L3 (e.g. base_set = Set::Union), then TS is L3.
        if base_set_level == AbstractionLevel::Level3
            || topology_definition_level == AbstractionLevel::Level3
        {
            return AbstractionLevel::Level3;
        }

        // If both base_set and topology_definition are L4, then TS is L4.
        // (This simple check assumes topology_definition_level L4 implies concrete open sets)
        if base_set_level == AbstractionLevel::Level4
            && topology_definition_level == AbstractionLevel::Level4
        {
            return AbstractionLevel::Level4;
        }

        overall_level // L1 if both base_set & topology_def are L1, else L2 by default for mixed non-L3/L4.
    }
}
