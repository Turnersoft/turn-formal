use crate::subjects::math::{
    formalism::objects::MathObject,
    theories::{VariantSet, groups::definitions::TopologicalGroup},
};
use std::fmt;

use super::definitions::*;

impl Group {
    // --- Getters for Foundational & Abstract Groups ---

    pub fn get_generic(&self) -> Result<&GenericGroup, String> {
        if let Group::Generic(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected GenericGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_generic(&mut self) -> Result<&mut GenericGroup, String> {
        if let Group::Generic(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected GenericGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_trivial(&self) -> Result<&TrivialGroup, String> {
        if let Group::Trivial(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected TrivialGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_trivial(&mut self) -> Result<&mut TrivialGroup, String> {
        if let Group::Trivial(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected TrivialGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_cyclic(&self) -> Result<&CyclicGroup, String> {
        if let Group::Cyclic(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected CyclicGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_cyclic(&mut self) -> Result<&mut CyclicGroup, String> {
        if let Group::Cyclic(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected CyclicGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_dihedral(&self) -> Result<&DihedralGroup, String> {
        if let Group::Dihedral(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected DihedralGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_dihedral(&mut self) -> Result<&mut DihedralGroup, String> {
        if let Group::Dihedral(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected DihedralGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_free(&self) -> Result<&FreeGroup, String> {
        if let Group::Free(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected FreeGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_free(&mut self) -> Result<&mut FreeGroup, String> {
        if let Group::Free(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected FreeGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    // --- Getters for Permutation Groups ---

    pub fn get_symmetric(&self) -> Result<&SymmetricGroup, String> {
        if let Group::Symmetric(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected SymmetricGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_symmetric(&mut self) -> Result<&mut SymmetricGroup, String> {
        if let Group::Symmetric(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected SymmetricGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_alternating(&self) -> Result<&AlternatingGroup, String> {
        if let Group::Alternating(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected AlternatingGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_alternating(&mut self) -> Result<&mut AlternatingGroup, String> {
        if let Group::Alternating(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected AlternatingGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    // --- Getters for Matrix/Linear Groups ---

    pub fn get_general_linear(&self) -> Result<&GeneralLinearGroup, String> {
        if let Group::GeneralLinear(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected GeneralLinearGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_general_linear(&mut self) -> Result<&mut GeneralLinearGroup, String> {
        if let Group::GeneralLinear(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected GeneralLinearGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_special_linear(&self) -> Result<&SpecialLinearGroup, String> {
        if let Group::SpecialLinear(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected SpecialLinearGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_special_linear(&mut self) -> Result<&mut SpecialLinearGroup, String> {
        if let Group::SpecialLinear(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected SpecialLinearGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_orthogonal(&self) -> Result<&OrthogonalGroup, String> {
        if let Group::Orthogonal(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected OrthogonalGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_orthogonal(&mut self) -> Result<&mut OrthogonalGroup, String> {
        if let Group::Orthogonal(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected OrthogonalGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_special_orthogonal(&self) -> Result<&SpecialOrthogonalGroup, String> {
        if let Group::SpecialOrthogonal(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected SpecialOrthogonalGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_special_orthogonal(&mut self) -> Result<&mut SpecialOrthogonalGroup, String> {
        if let Group::SpecialOrthogonal(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected SpecialOrthogonalGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_unitary(&self) -> Result<&UnitaryGroup, String> {
        if let Group::Unitary(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected UnitaryGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_unitary(&mut self) -> Result<&mut UnitaryGroup, String> {
        if let Group::Unitary(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected UnitaryGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_special_unitary(&self) -> Result<&SpecialUnitaryGroup, String> {
        if let Group::SpecialUnitary(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected SpecialUnitaryGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_special_unitary(&mut self) -> Result<&mut SpecialUnitaryGroup, String> {
        if let Group::SpecialUnitary(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected SpecialUnitaryGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    // --- Getters for Groups with Additional Structure ---

    /// Returns a reference to the inner `TopologicalGroup`,
    /// or an error if the variant is not `Group::Topological`.
    pub fn get_topological(&self) -> Result<&TopologicalGroup, String> {
        if let Group::Topological(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected TopologicalGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_topological(&mut self) -> Result<&mut TopologicalGroup, String> {
        if let Group::Topological(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected TopologicalGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_lie(&self) -> Result<&LieGroup, String> {
        if let Group::Lie(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected LieGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_lie(&mut self) -> Result<&mut LieGroup, String> {
        if let Group::Lie(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected LieGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    // --- Getters for Modular Groups ---

    pub fn get_modular_additive(&self) -> Result<&ModularAdditiveGroup, String> {
        if let Group::ModularAdditive(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected ModularAdditiveGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_modular_additive(&mut self) -> Result<&mut ModularAdditiveGroup, String> {
        if let Group::ModularAdditive(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected ModularAdditiveGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_modular_multiplicative(&self) -> Result<&ModularMultiplicativeGroup, String> {
        if let Group::ModularMultiplicative(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected ModularMultiplicativeGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_modular_multiplicative(
        &mut self,
    ) -> Result<&mut ModularMultiplicativeGroup, String> {
        if let Group::ModularMultiplicative(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected ModularMultiplicativeGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    // --- Getters for Groups Defined by Operations on Other Groups ---

    pub fn get_product(&self) -> Result<&ProductGroup, String> {
        if let Group::Product(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected ProductGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_product(&mut self) -> Result<&mut ProductGroup, String> {
        if let Group::Product(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected ProductGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_quotient(&self) -> Result<&QuotientGroup, String> {
        if let Group::Quotient(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected QuotientGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_quotient(&mut self) -> Result<&mut QuotientGroup, String> {
        if let Group::Quotient(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected QuotientGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    // --- Getters for Groups Defined by Other Explicit Constructions ---

    pub fn get_kernel(&self) -> Result<&KernelGroup, String> {
        if let Group::Kernel(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected KernelGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_kernel(&mut self) -> Result<&mut KernelGroup, String> {
        if let Group::Kernel(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected KernelGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_image(&self) -> Result<&ImageGroup, String> {
        if let Group::Image(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected ImageGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_image(&mut self) -> Result<&mut ImageGroup, String> {
        if let Group::Image(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected ImageGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_center(&self) -> Result<&CenterGroup, String> {
        if let Group::Center(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected CenterGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_center(&mut self) -> Result<&mut CenterGroup, String> {
        if let Group::Center(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected CenterGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_generated_subgroup(&self) -> Result<&GeneratedSubgroup, String> {
        if let Group::GeneratedSubgroup(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected GeneratedSubgroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_generated_subgroup(&mut self) -> Result<&mut GeneratedSubgroup, String> {
        if let Group::GeneratedSubgroup(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected GeneratedSubgroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_normalizer(&self) -> Result<&NormalizerGroup, String> {
        if let Group::Normalizer(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected NormalizerGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_normalizer(&mut self) -> Result<&mut NormalizerGroup, String> {
        if let Group::Normalizer(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected NormalizerGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_centralizer(&self) -> Result<&CentralizerGroup, String> {
        if let Group::Centralizer(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected CentralizerGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_centralizer(&mut self) -> Result<&mut CentralizerGroup, String> {
        if let Group::Centralizer(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected CentralizerGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_commutator_subgroup(&self) -> Result<&CommutatorSubgroup, String> {
        if let Group::CommutatorSubgroup(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected CommutatorSubgroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_commutator_subgroup(&mut self) -> Result<&mut CommutatorSubgroup, String> {
        if let Group::CommutatorSubgroup(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected CommutatorSubgroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_sylow_subgroup(&self) -> Result<&SylowSubgroup, String> {
        if let Group::SylowSubgroup(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected SylowSubgroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_sylow_subgroup(&mut self) -> Result<&mut SylowSubgroup, String> {
        if let Group::SylowSubgroup(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected SylowSubgroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_wreath_product(&self) -> Result<&WreathProductGroup, String> {
        if let Group::WreathProduct(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected WreathProductGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_wreath_product(&mut self) -> Result<&mut WreathProductGroup, String> {
        if let Group::WreathProduct(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected WreathProductGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_central_product(&self) -> Result<&CentralProductGroup, String> {
        if let Group::CentralProduct(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected CentralProductGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_central_product(&mut self) -> Result<&mut CentralProductGroup, String> {
        if let Group::CentralProduct(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected CentralProductGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_pullback(&self) -> Result<&PullbackGroup, String> {
        if let Group::Pullback(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected PullbackGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_pullback(&mut self) -> Result<&mut PullbackGroup, String> {
        if let Group::Pullback(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected PullbackGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    pub fn get_restriction(&self) -> Result<&RestrictionGroup, String> {
        if let Group::Restriction(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected RestrictionGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }
    pub fn get_mut_restriction(&mut self) -> Result<&mut RestrictionGroup, String> {
        if let Group::Restriction(g) = self {
            Ok(g)
        } else {
            Err(format!(
                "Expected RestrictionGroup, but found {}",
                self.get_variant_name()
            ))
        }
    }

    // --- Core Property Getters ---

    /// Gets a reference to the core `GenericGroup` contained within any `Group` variant.
    pub fn get_core(&self) -> &GenericGroup {
        match self {
            Group::Generic(g) => g,
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
