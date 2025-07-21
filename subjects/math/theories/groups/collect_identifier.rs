use std::collections::HashSet;

use crate::{
    subjects::math::formalism::collect_identifier::CollectIdentifier, turn_render::Identifier,
};

use super::definitions::{Group, GroupElement, GroupExpression, GroupHomomorphism, GroupRelation};

// ===== GROUP THEORY IMPLEMENTATIONS (Child nodes of MathExpression) =====

impl CollectIdentifier for Group {
    fn collect_identifier(&self) -> HashSet<Identifier> {
        // Groups can be variables, so we need to check if they're parametrizable
        // For now, we don't traverse into group internal structure since
        // groups in expressions are typically used as atomic identifiers
        HashSet::new()
    }
}

impl CollectIdentifier for GroupRelation {
    fn collect_identifier(&self) -> HashSet<Identifier> {
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

impl CollectIdentifier for GroupExpression {
    fn collect_identifier(&self) -> HashSet<Identifier> {
        match self {
            GroupExpression::Element { group, element } => {
                let mut identifiers = HashSet::new();
                identifiers.extend(group.collect_identifier());
                identifiers.extend(element.collect_identifier());
                identifiers
            }
            GroupExpression::Identity(group) => group.collect_identifier(),
            GroupExpression::Operation { group, left, right } => {
                let mut identifiers = HashSet::new();
                identifiers.extend(group.collect_identifier());
                identifiers.extend(left.collect_identifier());
                identifiers.extend(right.collect_identifier());
                identifiers
            }
            GroupExpression::Inverse { group, element } => {
                let mut identifiers = HashSet::new();
                identifiers.extend(group.collect_identifier());
                identifiers.extend(element.collect_identifier());
                identifiers
            }
            GroupExpression::Commutator { group, a, b } => {
                let mut identifiers = HashSet::new();
                identifiers.extend(group.collect_identifier());
                identifiers.extend(a.collect_identifier());
                identifiers.extend(b.collect_identifier());
                identifiers
            }
            GroupExpression::Coset {
                group,
                element,
                subgroup,
                ..
            } => {
                let mut identifiers = HashSet::new();
                identifiers.extend(group.collect_identifier());
                identifiers.extend(element.collect_identifier());
                identifiers.extend(subgroup.collect_identifier());
                identifiers
            }
            GroupExpression::ActionOnElement { action: _, element } => {
                // Don't traverse into action since it's complex structure
                element.collect_identifier()
            }
            GroupExpression::Power {
                group,
                base,
                exponent,
            } => {
                let mut identifiers = HashSet::new();
                identifiers.extend(group.collect_identifier());
                identifiers.extend(base.collect_identifier());
                identifiers.extend(exponent.collect_identifier());
                identifiers
            }
            GroupExpression::GroupOrder { group } => group.collect_identifier(),
            GroupExpression::ElementOrder { element, group } => {
                let mut identifiers = HashSet::new();
                identifiers.extend(element.collect_identifier());
                identifiers.extend(group.collect_identifier());
                identifiers
            }
            GroupExpression::Homomorphism(hom) => hom.collect_identifier(),
        }
    }
}

impl CollectIdentifier for GroupElement {
    fn collect_identifier(&self) -> HashSet<Identifier> {
        // GroupElement variants are concrete values, no identifiers
        HashSet::new()
    }
}

impl CollectIdentifier for GroupHomomorphism {
    fn collect_identifier(&self) -> HashSet<Identifier> {
        let mut identifiers = HashSet::new();
        identifiers.extend(self.domain.collect_identifier());
        identifiers.extend(self.codomain.collect_identifier());
        identifiers
    }
}
