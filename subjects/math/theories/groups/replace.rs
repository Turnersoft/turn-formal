use std::collections::HashMap;

use crate::{
    subjects::math::{
        formalism::{
            expressions::MathExpression,
            proof::{ContextEntry, tactics::Target},
            replace::{Replace, Substitutable},
        },
        theories::groups::definitions::{GenericGroup, Group, GroupExpression, GroupOperation},
    },
    turn_render::Identifier,
};

use super::definitions::GroupRelation;

impl Replace for Group {
    fn replace(
        &self,
        current_id: &str,
        target: &Target,
        target_context: &Vec<ContextEntry>,
        pattern: &MathExpression,
        replacement: &MathExpression,
        pattern_and_replacement_context: &Vec<ContextEntry>,
    ) -> Self {
        match self {
            Group::Generic(g) => Group::Generic(g.replace(
                current_id,
                target,
                target_context,
                pattern,
                replacement,
                pattern_and_replacement_context,
            )),
            Group::Trivial(trivial_group) => todo!(),
            Group::Cyclic(cyclic_group) => todo!(),
            Group::Dihedral(dihedral_group) => todo!(),
            Group::Free(free_group) => todo!(),
            Group::Symmetric(symmetric_group) => todo!(),
            Group::Alternating(alternating_group) => todo!(),
            Group::GeneralLinear(general_linear_group) => todo!(),
            Group::SpecialLinear(special_linear_group) => todo!(),
            Group::Orthogonal(orthogonal_group) => todo!(),
            Group::SpecialOrthogonal(special_orthogonal_group) => todo!(),
            Group::Unitary(unitary_group) => todo!(),
            Group::SpecialUnitary(special_unitary_group) => todo!(),
            Group::Topological(topological_group) => todo!(),
            Group::Lie(lie_group) => todo!(),
            Group::ModularAdditive(modular_additive_group) => todo!(),
            Group::ModularMultiplicative(modular_multiplicative_group) => todo!(),
            Group::Product(product_group) => todo!(),
            Group::Quotient(quotient_group) => todo!(),
            Group::Kernel(kernel_group) => todo!(),
            Group::Image(image_group) => todo!(),
            Group::Center(center_group) => todo!(),
            Group::GeneratedSubgroup(generated_subgroup) => todo!(),
            Group::Normalizer(normalizer_group) => todo!(),
            Group::Centralizer(centralizer_group) => todo!(),
            Group::CommutatorSubgroup(commutator_subgroup) => todo!(),
            Group::SylowSubgroup(sylow_subgroup) => todo!(),
            Group::WreathProduct(wreath_product_group) => todo!(),
            Group::CentralProduct(central_product_group) => todo!(),
            Group::Pullback(pullback_group) => todo!(),
            Group::Restriction(restriction_group) => todo!(),
        }
    }
}

impl Replace for GenericGroup {
    fn replace(
        &self,
        current_id: &str,
        target: &Target,
        target_context: &Vec<ContextEntry>,
        pattern: &MathExpression,
        replacement: &MathExpression,
        pattern_and_replacement_context: &Vec<ContextEntry>,
    ) -> Self {
        todo!()
    }
}

impl Substitutable for Group {
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, MathExpression>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        match self {
            Group::Generic(g) => Group::Generic(g.substitute(instantiations, context)),
            Group::Trivial(trivial_group) => todo!(),
            Group::Cyclic(cyclic_group) => todo!(),
            Group::Dihedral(dihedral_group) => todo!(),
            Group::Free(free_group) => todo!(),
            Group::Symmetric(symmetric_group) => todo!(),
            Group::Alternating(alternating_group) => todo!(),
            Group::GeneralLinear(general_linear_group) => todo!(),
            Group::SpecialLinear(special_linear_group) => todo!(),
            Group::Orthogonal(orthogonal_group) => todo!(),
            Group::SpecialOrthogonal(special_orthogonal_group) => todo!(),
            Group::Unitary(unitary_group) => todo!(),
            Group::SpecialUnitary(special_unitary_group) => todo!(),
            Group::Topological(topological_group) => todo!(),
            Group::Lie(lie_group) => todo!(),
            Group::ModularAdditive(modular_additive_group) => todo!(),
            Group::ModularMultiplicative(modular_multiplicative_group) => todo!(),
            Group::Product(product_group) => todo!(),
            Group::Quotient(quotient_group) => todo!(),
            Group::Kernel(kernel_group) => todo!(),
            Group::Image(image_group) => todo!(),
            Group::Center(center_group) => todo!(),
            Group::GeneratedSubgroup(generated_subgroup) => todo!(),
            Group::Normalizer(normalizer_group) => todo!(),
            Group::Centralizer(centralizer_group) => todo!(),
            Group::CommutatorSubgroup(commutator_subgroup) => todo!(),
            Group::SylowSubgroup(sylow_subgroup) => todo!(),
            Group::WreathProduct(wreath_product_group) => todo!(),
            Group::CentralProduct(central_product_group) => todo!(),
            Group::Pullback(pullback_group) => todo!(),
            Group::Restriction(restriction_group) => todo!(),
        }
    }
}

impl Substitutable for GenericGroup {
    fn substitute(
        &self,
        instantiations: &HashMap<Identifier, MathExpression>,
        context: &Vec<ContextEntry>,
    ) -> Self {
        GenericGroup {
            base_set: self.base_set.substitute(instantiations, context),
            operation: self.operation.substitute(instantiations, context),
            props: self.props.clone(),
        }
    }
}

impl Substitutable for GroupOperation {
    fn substitute(
        &self,
        _instantiations: &HashMap<Identifier, MathExpression>,
        _context: &Vec<ContextEntry>,
    ) -> Self {
        self.clone()
    }
}

impl Substitutable for GroupExpression {
    fn substitute(
        &self,
        _instantiations: &HashMap<Identifier, MathExpression>,
        _context: &Vec<ContextEntry>,
    ) -> Self {
        //TODO: implement this properly
        self.clone()
    }
}

impl Replace for GroupRelation {
    fn replace(
        &self,
        current_id: &str,
        target: &Target,
        target_context: &Vec<ContextEntry>,
        pattern: &MathExpression,
        replacement: &MathExpression,
        pattern_and_replacement_context: &Vec<ContextEntry>,
    ) -> Self {
        match self {
            GroupRelation::IsSubgroupOf { subgroup, group } => todo!(),
            GroupRelation::IsNormalSubgroupOf { subgroup, group } => todo!(),
            GroupRelation::IsIsomorphicTo { first, second } => todo!(),
            GroupRelation::IsQuotientOf {
                quotient,
                group,
                normal_subgroup,
            } => todo!(),
            GroupRelation::IsInCenterOf { element, group } => todo!(),
            GroupRelation::AreConjugateIn {
                element1,
                element2,
                group,
            } => todo!(),
            GroupRelation::HasOrderInGroup {
                element,
                group,
                order,
            } => todo!(),
            GroupRelation::HasIndexInGroup {
                subgroup,
                group,
                index,
            } => todo!(),
            GroupRelation::HasOrder { group, order } => todo!(),
            GroupRelation::IsCyclicWithGenerator { group, generator } => todo!(),
            GroupRelation::NormalizesSubgroup {
                element,
                subgroup,
                group,
            } => todo!(),
            GroupRelation::CentralizesSubgroup {
                element,
                subgroup,
                group,
            } => todo!(),
            GroupRelation::IsCharacteristicSubgroupOf { subgroup, group } => todo!(),
            GroupRelation::OrderDivides { group1, group2 } => todo!(),
            GroupRelation::HasUniqueInverse { element, group } => todo!(),
            GroupRelation::SylowSubgroupProperties { prime, group } => todo!(),
            GroupRelation::IsInverseOf {
                element,
                inverse,
                group,
            } => todo!(),
            GroupRelation::IsHomomorphism {
                homomorphism,
                domain,
                codomain,
            } => todo!(),
            GroupRelation::IsomorphicEmbedding { source, target } => todo!(),
            GroupRelation::HasBasicProperty { target, property } => todo!(),
            GroupRelation::HasTopologicalProperty { target, property } => todo!(),
            GroupRelation::HasLieProperty { target, property } => todo!(),
            GroupRelation::HasActionProperty { target, property } => todo!(),
            GroupRelation::HasProductProperty { target, property } => todo!(),
            GroupRelation::HasModularAdditiveProperty { target, property } => todo!(),
            GroupRelation::HasModularMultiplicativeProperty { target, property } => todo!(),
            GroupRelation::HasGeneralLinearMatrixProperty { target, property } => todo!(),
            GroupRelation::HasGeneralLinearLinearProperty { target, property } => todo!(),
            GroupRelation::HasSpecialLinearProperty { target, property } => todo!(),
            GroupRelation::HasOrthogonalMatrixProperty { target, property } => todo!(),
            GroupRelation::HasSpecialOrthogonalProperty { target, property } => todo!(),
            GroupRelation::HasUnitaryMatrixProperty { target, property } => todo!(),
            GroupRelation::HasSpecialUnitaryProperty { target, property } => todo!(),
            GroupRelation::HasAlternatingPermutationProperty { target, property } => todo!(),
            GroupRelation::HasFreeProperty { target, property } => todo!(),
            GroupRelation::HasQuotientProperty { target, property } => todo!(),
        }
    }
}
