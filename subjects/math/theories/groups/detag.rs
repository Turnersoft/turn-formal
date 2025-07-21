use super::definitions::*;
use crate::subjects::math::{formalism::detag::TryDetag, theories::VariantSet};
use crate::{impl_try_get_for_terminal_type, try_detag_as};
use std::{any::Any, fmt::Debug};

// Implement TryGet for all terminal group types (excluding Group and GroupExpression which are already implemented in the main getter.rs)
impl_try_get_for_terminal_type!(GenericGroup);
impl_try_get_for_terminal_type!(TrivialGroup);
impl_try_get_for_terminal_type!(CyclicGroup);
impl_try_get_for_terminal_type!(DihedralGroup);
impl_try_get_for_terminal_type!(FreeGroup);
impl_try_get_for_terminal_type!(SymmetricGroup);
impl_try_get_for_terminal_type!(AlternatingGroup);
impl_try_get_for_terminal_type!(GeneralLinearGroup);
impl_try_get_for_terminal_type!(SpecialLinearGroup);
impl_try_get_for_terminal_type!(OrthogonalGroup);
impl_try_get_for_terminal_type!(SpecialOrthogonalGroup);
impl_try_get_for_terminal_type!(UnitaryGroup);
impl_try_get_for_terminal_type!(SpecialUnitaryGroup);
impl_try_get_for_terminal_type!(TopologicalGroup);
impl_try_get_for_terminal_type!(LieGroup);
impl_try_get_for_terminal_type!(ModularAdditiveGroup);
impl_try_get_for_terminal_type!(ModularMultiplicativeGroup);
impl_try_get_for_terminal_type!(ProductGroup);
impl_try_get_for_terminal_type!(QuotientGroup);
impl_try_get_for_terminal_type!(KernelGroup);
impl_try_get_for_terminal_type!(ImageGroup);
impl_try_get_for_terminal_type!(CenterGroup);
impl_try_get_for_terminal_type!(GeneratedSubgroup);
impl_try_get_for_terminal_type!(NormalizerGroup);
impl_try_get_for_terminal_type!(CentralizerGroup);
impl_try_get_for_terminal_type!(CommutatorSubgroup);
impl_try_get_for_terminal_type!(SylowSubgroup);
impl_try_get_for_terminal_type!(WreathProductGroup);
impl_try_get_for_terminal_type!(CentralProductGroup);
impl_try_get_for_terminal_type!(PullbackGroup);
impl_try_get_for_terminal_type!(RestrictionGroup);
impl_try_get_for_terminal_type!(GroupAction);

// --- Implementations for Enums (only those not already implemented in main getter.rs) ---

impl<T: 'static + Debug> TryDetag<T> for Group {
    fn try_detag(&self) -> Result<&T, String> {
        if let Ok(res) = try_detag_as!(self, T) {
            return Ok(res);
        }
        match self {
            Group::Generic(generic_group) => generic_group.try_detag(),
            Group::Trivial(trivial_group) => trivial_group.try_detag(),
            Group::Cyclic(cyclic_group) => cyclic_group.try_detag(),
            Group::Dihedral(dihedral_group) => dihedral_group.try_detag(),
            Group::Free(free_group) => free_group.try_detag(),
            Group::Symmetric(symmetric_group) => symmetric_group.try_detag(),
            Group::Alternating(alternating_group) => alternating_group.try_detag(),
            Group::GeneralLinear(general_linear_group) => general_linear_group.try_detag(),
            Group::SpecialLinear(special_linear_group) => special_linear_group.try_detag(),
            Group::Orthogonal(orthogonal_group) => orthogonal_group.try_detag(),
            Group::SpecialOrthogonal(special_orthogonal_group) => {
                special_orthogonal_group.try_detag()
            }
            Group::Unitary(unitary_group) => unitary_group.try_detag(),
            Group::SpecialUnitary(special_unitary_group) => special_unitary_group.try_detag(),
            Group::Topological(topological_group) => topological_group.try_detag(),
            Group::Lie(lie_group) => lie_group.try_detag(),
            Group::ModularAdditive(modular_additive_group) => modular_additive_group.try_detag(),
            Group::ModularMultiplicative(modular_multiplicative_group) => {
                modular_multiplicative_group.try_detag()
            }
            Group::Product(product_group) => product_group.try_detag(),
            Group::Quotient(quotient_group) => quotient_group.try_detag(),
            Group::Kernel(kernel_group) => kernel_group.try_detag(),
            Group::Image(image_group) => image_group.try_detag(),
            Group::Center(center_group) => center_group.try_detag(),
            Group::GeneratedSubgroup(generated_subgroup) => generated_subgroup.try_detag(),
            Group::Normalizer(normalizer_group) => normalizer_group.try_detag(),
            Group::Centralizer(centralizer_group) => centralizer_group.try_detag(),
            Group::CommutatorSubgroup(commutator_subgroup) => commutator_subgroup.try_detag(),
            Group::SylowSubgroup(sylow_subgroup) => sylow_subgroup.try_detag(),
            Group::WreathProduct(wreath_product_group) => wreath_product_group.try_detag(),
            Group::CentralProduct(central_product_group) => central_product_group.try_detag(),
            Group::Pullback(pullback_group) => pullback_group.try_detag(),
            Group::Restriction(restriction_group) => restriction_group.try_detag(),
        }
    }
}

impl<T: 'static + Debug> TryDetag<T> for GroupRelation {
    fn try_detag(&self) -> Result<&T, String> {
        if let Ok(res) = try_detag_as!(self, T) {
            return Ok(res);
        }
        match self {
            // Handle any other variants that might exist
            _ => Err(format!(
                "TryGet not implemented for this GroupRelation variant to find {}",
                std::any::type_name::<T>()
            )),
        }
    }
}

impl<T: 'static + Debug> TryDetag<T> for GroupExpression {
    fn try_detag(&self) -> Result<&T, String> {
        if let Ok(res) = try_detag_as!(self, T) {
            return Ok(res);
        }
        match self {
            _ => Err(format!(
                "TryGet not implemented for this GroupExpression variant to find {}",
                std::any::type_name::<T>()
            )),
        }
    }
}

// Note: Group and GroupExpression implementations are already provided in subjects/math/formalism/getter.rs

impl Group {
    /// Gets a reference to the core `GenericGroup` contained within any `Group` variant.
    pub fn get_core(&self) -> &GenericGroup {
        match self {
            Group::Generic(g) => g,
            Group::Trivial(g) => &g.core,
            Group::Cyclic(g) => &g.core,
            Group::Dihedral(g) => &g.core,
            Group::Free(g) => &g.core,
            Group::Symmetric(g) => &g.core,
            Group::Alternating(g) => &g.core,
            Group::GeneralLinear(g) => &g.core,
            Group::SpecialLinear(g) => &g.general_linear.core,
            Group::Orthogonal(g) => &g.core,
            Group::SpecialOrthogonal(g) => &g.orthogonal.core,
            Group::Unitary(g) => &g.core,
            Group::SpecialUnitary(g) => &g.unitary.core,
            Group::Topological(g) => &g.core,
            Group::Lie(g) => &g.core,
            Group::ModularAdditive(g) => &g.core,
            Group::ModularMultiplicative(g) => &g.core,
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

    /// Gets a mutable reference to the `props` of the core `GenericGroup`
    /// contained within any `Group` variant.
    /// This is infallible as all Group variants have core properties.
    pub fn get_mut_core_properties(&mut self) -> &mut VariantSet<GroupProperty> {
        match self {
            Group::Generic(g) => &mut g.props,
            Group::Trivial(g) => &mut g.core.props,
            Group::Cyclic(g) => &mut g.core.props,
            Group::Dihedral(g) => &mut g.core.props,
            Group::Free(g) => &mut g.core.props,
            Group::Symmetric(g) => &mut g.core.props,
            Group::Alternating(g) => &mut g.core.props,
            Group::GeneralLinear(g) => &mut g.core.props,
            Group::SpecialLinear(g) => &mut g.general_linear.core.props,
            Group::Orthogonal(g) => &mut g.core.props,
            Group::SpecialOrthogonal(g) => &mut g.orthogonal.core.props,
            Group::Unitary(g) => &mut g.core.props,
            Group::SpecialUnitary(g) => &mut g.unitary.core.props,
            Group::Topological(g) => &mut g.core.props,
            Group::Lie(g) => &mut g.core.props,
            Group::ModularAdditive(g) => &mut g.core.props,
            Group::ModularMultiplicative(g) => &mut g.core.props,
            Group::Product(g) => &mut g.core.props,
            Group::Quotient(g) => &mut g.core.props,
            Group::Kernel(g) => &mut g.core.props,
            Group::Image(g) => &mut g.core.props,
            Group::Center(g) => &mut g.core.props,
            Group::GeneratedSubgroup(g) => &mut g.core.props,
            Group::Normalizer(g) => &mut g.core.props,
            Group::Centralizer(g) => &mut g.core.props,
            Group::CommutatorSubgroup(g) => &mut g.core.props,
            Group::SylowSubgroup(g) => &mut g.core.props,
            Group::WreathProduct(g) => &mut g.core.props,
            Group::CentralProduct(g) => &mut g.core.props,
            Group::Pullback(g) => &mut g.core.props,
            Group::Restriction(g) => &mut g.core.props,
        }
    }
}
