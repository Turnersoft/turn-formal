use super::definitions::{Group, GroupExpression};
use crate::extract_as;
use crate::subjects::math::formalism::extract::Extractable;

impl Extractable for Group {
    fn extract<T: 'static + Clone>(&self) -> Option<T> {
        match self {
            Group::Generic(group) => extract_as!(group, T),
            Group::Topological(topological_group) => extract_as!(topological_group, T),
            Group::Lie(lie_group) => extract_as!(lie_group, T),
            Group::Cyclic(cyclic_group) => extract_as!(cyclic_group, T),
            Group::Symmetric(symmetric_group) => extract_as!(symmetric_group, T),
            Group::Dihedral(dihedral_group) => extract_as!(dihedral_group, T),
            Group::GeneralLinear(general_linear_group) => extract_as!(general_linear_group, T),
            Group::SpecialLinear(special_linear_group) => extract_as!(special_linear_group, T),
            Group::Orthogonal(orthogonal_group) => extract_as!(orthogonal_group, T),
            Group::SpecialOrthogonal(special_orthogonal_group) => {
                extract_as!(special_orthogonal_group, T)
            }
            Group::Unitary(unitary_group) => extract_as!(unitary_group, T),
            Group::SpecialUnitary(special_unitary_group) => extract_as!(special_unitary_group, T),
            Group::Alternating(alternating_group) => extract_as!(alternating_group, T),
            Group::ModularAdditive(modular_additive_group) => {
                extract_as!(modular_additive_group, T)
            }
            Group::ModularMultiplicative(modular_multiplicative_group) => {
                extract_as!(modular_multiplicative_group, T)
            }
            Group::Free(free_group) => extract_as!(free_group, T),
            Group::Trivial(trivial_group) => extract_as!(trivial_group, T),
            Group::Product(product_group) => extract_as!(product_group, T),
            Group::Quotient(quotient_group) => extract_as!(quotient_group, T),
            Group::Kernel(kernel_group) => extract_as!(kernel_group, T),
            Group::Image(image_group) => extract_as!(image_group, T),
            Group::Center(center_group) => extract_as!(center_group, T),
            Group::GeneratedSubgroup(generated_subgroup) => extract_as!(generated_subgroup, T),
            Group::Normalizer(normalizer_group) => extract_as!(normalizer_group, T),
            Group::Centralizer(centralizer_group) => extract_as!(centralizer_group, T),
            Group::CommutatorSubgroup(commutator_subgroup) => extract_as!(commutator_subgroup, T),
            Group::SylowSubgroup(sylow_subgroup) => extract_as!(sylow_subgroup, T),
            Group::WreathProduct(wreath_product_group) => extract_as!(wreath_product_group, T),
            Group::CentralProduct(central_product_group) => extract_as!(central_product_group, T),
            Group::Pullback(pullback_group) => extract_as!(pullback_group, T),
            Group::Restriction(restriction_group) => extract_as!(restriction_group, T),
        }
    }
}
