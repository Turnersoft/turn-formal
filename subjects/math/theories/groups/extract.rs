use super::definitions::{Group, GroupExpression};
use crate::subjects::math::formalism::expressions::MathExpression;
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::proof::ContextEntry;
use std::any::Any;

impl Extractable for Group {
    fn try_extract<T: 'static + Clone>(&self) -> Option<T> {
        macro_rules! extract_variant {
            ($val:expr) => {
                ($val as &dyn Any).downcast_ref::<T>().cloned()
            };
        }
        match self {
            Group::Generic(group) => extract_variant!(group),
            Group::Topological(topological_group) => extract_variant!(topological_group),
            Group::Lie(lie_group) => extract_variant!(lie_group),
            Group::Cyclic(cyclic_group) => extract_variant!(cyclic_group),
            Group::Symmetric(symmetric_group) => extract_variant!(symmetric_group),
            Group::Dihedral(dihedral_group) => extract_variant!(dihedral_group),
            Group::GeneralLinear(general_linear_group) => extract_variant!(general_linear_group),
            Group::SpecialLinear(special_linear_group) => extract_variant!(special_linear_group),
            Group::Orthogonal(orthogonal_group) => extract_variant!(orthogonal_group),
            Group::SpecialOrthogonal(special_orthogonal_group) => {
                extract_variant!(special_orthogonal_group)
            }
            Group::Unitary(unitary_group) => extract_variant!(unitary_group),
            Group::SpecialUnitary(special_unitary_group) => extract_variant!(special_unitary_group),
            Group::ModularMultiplicative(modular_multiplicative_group) => {
                extract_variant!(modular_multiplicative_group)
            }
            Group::Free(free_group) => extract_variant!(free_group),
            Group::Trivial(trivial_group) => extract_variant!(trivial_group),
            Group::Product(product_group) => extract_variant!(product_group),
            Group::Quotient(quotient_group) => extract_variant!(quotient_group),
            Group::Kernel(kernel_group) => extract_variant!(kernel_group),
            Group::Image(image_group) => extract_variant!(image_group),
            Group::Center(center_group) => extract_variant!(center_group),
            Group::GeneratedSubgroup(generated_subgroup) => extract_variant!(generated_subgroup),
            Group::Normalizer(normalizer_group) => extract_variant!(normalizer_group),
            Group::Centralizer(centralizer_group) => extract_variant!(centralizer_group),
            Group::CommutatorSubgroup(commutator_subgroup) => extract_variant!(commutator_subgroup),
            Group::SylowSubgroup(sylow_subgroup) => extract_variant!(sylow_subgroup),
            Group::WreathProduct(wreath_product_group) => extract_variant!(wreath_product_group),
            Group::CentralProduct(central_product_group) => extract_variant!(central_product_group),
            Group::Pullback(pullback_group) => extract_variant!(pullback_group),
            Group::Restriction(restriction_group) => extract_variant!(restriction_group),
            Group::Alternating(alternating_group) => extract_variant!(alternating_group),
            Group::ModularAdditive(modular_additive_group) => {
                extract_variant!(modular_additive_group)
            }
        }
    }
}
