use crate::turn_render::{
    BracketSize, BracketStyle, MathNode, MathNodeContent, MulSymbol, RefinedMulOrDivOperation,
    RelationOperatorNode, ToTurnMath, UnaryRelationOperatorNode,
};
use std::collections::HashMap;

use crate::subjects::math::formalism::extract::Parametrizable;

use super::definitions::{
    AbelianPropertyVariant, FinitePropertyVariant, Group, GroupElement, GroupIdentity,
    GroupInverse, GroupOperationProperty, GroupOperationVariant, GroupProperty,
    NilpotentPropertyVariant, SimplePropertyVariant, SolvablePropertyVariant,
};
use super::{GroupAction, GroupBasic, GroupExpression, GroupRelation};

impl ToTurnMath for Group {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            Group::Basic(group) => {
                let core = group;
                let set_name = match &core.base_set {
                    super::super::super::theories::zfc::Set::Parametric { description, .. } => {
                        description.clone()
                    }
                    _ => format!("{:?}", core.base_set),
                };

                let op_symbol = match core.operation.operation_type {
                    GroupOperationVariant::Addition => "+",
                    GroupOperationVariant::Multiplication => "·",
                    GroupOperationVariant::MatrixMultiplication => "×",
                    GroupOperationVariant::Composition => "∘",
                    GroupOperationVariant::DirectProduct => "×",
                    GroupOperationVariant::SemidirectProduct => "⋊",
                    GroupOperationVariant::FreeProduct => "*",
                };

                let name = if set_name.contains("Z_") || set_name.contains("ℤ_") {
                    format!("Abstract Cyclic group {}", set_name)
                } else if set_name.contains("S_") || set_name.contains("Sym") {
                    format!("Abstract Symmetric group {}", set_name)
                } else if set_name.contains("GL") || set_name.contains("SL") {
                    format!("Abstract Matrix group {}", set_name)
                } else {
                    format!("Abstract Group {}", set_name)
                };

                let mut description = format!("{} with operation {}", name, op_symbol);

                let identity_str = match core.operation.identity {
                    GroupIdentity::One => "1",
                    GroupIdentity::Zero => "0",
                    GroupIdentity::IdentityMatrix => "I",
                    GroupIdentity::IdentityPermutation => "id",
                    GroupIdentity::IdentityFunction => "idₑ",
                };

                let inverse_str = match core.operation.inverse {
                    GroupInverse::MultiplicativeInverse => "x⁻¹",
                    GroupInverse::AdditiveInverse => "-x",
                    GroupInverse::MatrixInverse => "A⁻¹",
                    GroupInverse::PermutationInverse => "σ⁻¹",
                    GroupInverse::FunctionInverse => "f⁻¹",
                };

                let mut props_vec = Vec::<String>::new();

                let is_abelian = core
                    .props
                    .iter()
                    .any(|p| matches!(p, GroupProperty::Abelian(AbelianPropertyVariant::Abelian)));

                let order = core.props.iter().find_map(|p| {
                    if let GroupProperty::Finite(FinitePropertyVariant::Finite(n)) = p {
                        Some(*n)
                    } else {
                        None
                    }
                });

                if is_abelian {
                    props_vec.push("abelian".to_string());
                }

                let is_simple = core
                    .props
                    .iter()
                    .any(|p| matches!(p, GroupProperty::Simple(SimplePropertyVariant::Simple)));

                if is_simple {
                    props_vec.push("simple".to_string());
                }

                if !props_vec.is_empty() {
                    description.push_str(&format!(" ({})", props_vec.join(", ")));
                }

                description.push_str(&format!("\n\nIdentity element: {}", identity_str));
                description.push_str(&format!("\nInverse operation: {}", inverse_str));

                let mut op_props = Vec::<String>::new();

                if core
                    .operation
                    .properties
                    .iter()
                    .any(|p| matches!(p, GroupOperationProperty::Associative))
                {
                    op_props.push("associative".to_string());
                }

                if core
                    .operation
                    .properties
                    .iter()
                    .any(|p| matches!(p, GroupOperationProperty::Commutative(true)))
                {
                    op_props.push("commutative".to_string());
                }

                if !op_props.is_empty() {
                    description.push_str(&format!("\nOperation is {}", op_props.join(", ")));
                }

                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text(description)),
                }
            }
            Group::Topological(group) => {
                let core = &group.core;
                let mut description = format!("Topological Group on {:?}", core.base_set);

                description.push_str("\n\nTopological Properties:");
                for prop in group.props.iter() {
                    description.push_str(&format!("\n- {:?}", prop));
                }

                description.push_str("\n\nGroup Properties:");
                for prop in core.props.iter() {
                    description.push_str(&format!("\n- {:?}", prop));
                }

                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text(description)),
                }
            }
            Group::Lie(group) => {
                let core = &group.core;
                let mut description = format!("Lie Group on {:?}", core.base_set);

                description.push_str("\n\nLie Properties:");
                for prop in group.props.iter() {
                    description.push_str(&format!("\n- {:?}", prop));
                }

                if !group.charts.is_empty() {
                    description.push_str("\n\nCharts:");
                    for chart in &group.charts {
                        description.push_str(&format!("\n- {}", chart));
                    }
                }

                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text(description)),
                }
            }
            Group::Cyclic(group) => {
                let core = &group.core;
                let mut description = format!("Cyclic Group <{:?}>", group.generator);

                if let Some(order) = group.order {
                    description.push_str(&format!("\nOrder: {}", order));
                } else {
                    description.push_str("\nOrder: infinite");
                }

                let identity_str = match core.operation.identity {
                    GroupIdentity::One => "1",
                    GroupIdentity::Zero => "0",
                    GroupIdentity::IdentityMatrix => "I",
                    GroupIdentity::IdentityPermutation => "id",
                    GroupIdentity::IdentityFunction => "idₑ",
                };

                description.push_str(&format!("\nIdentity element: {}", identity_str));

                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text(description)),
                }
            }
            Group::Symmetric(group) => {
                let description = format!("Symmetric Group S_{}", group.degree);

                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text(description)),
                }
            }
            Group::Dihedral(group) => {
                let description = format!("Dihedral Group D_{}", group.order / 2);

                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text(description)),
                }
            }
            Group::Product(group) => {
                let core = &group.core;
                let components_str = group
                    .components
                    .iter()
                    .map(|c| format!("{:?}", c))
                    .collect::<Vec<_>>()
                    .join(" × ");
                let description =
                    format!("Product Group: {} ({:?})", components_str, group.operation);
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text(description)),
                }
            }
            Group::Quotient(group) => {
                let group_str = format!("{:?}", group.group);
                let normal_subgroup_str = format!("{:?}", group.normal_subgroup);
                let description = format!("Quotient Group {} / {}", group_str, normal_subgroup_str);
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text(description)),
                }
            }
            Group::Kernel(k_group) => {
                let description = format!("Kernel Group Ker({:?})", k_group.defining_homomorphism);
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text(description)),
                }
            }
            Group::Image(i_group) => {
                let description = format!("Image Group Im({:?})", i_group.defining_homomorphism);
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text(description)),
                }
            }
            Group::Center(c_group) => {
                let description = format!("Center Z({:?})", c_group.parent_group);
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text(description)),
                }
            }
            Group::GeneratedSubgroup(g_group) => {
                let gens_str = g_group
                    .generators
                    .iter()
                    .map(|g| format!("{:?}", g))
                    .collect::<Vec<_>>()
                    .join(", ");
                let description = format!(
                    "Generated Subgroup <{}> of {:?}",
                    gens_str, g_group.parent_group
                );
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text(description)),
                }
            }
            _ => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text(format!(
                    "Group (Unrendered Type): {:?}",
                    self
                ))),
            },
        }
    }
}

fn factorial(n: usize) -> usize {
    if n == 0 || n == 1 {
        1
    } else {
        (2..=n).fold(1, |acc, x| acc * x)
    }
}

impl ToTurnMath for GroupBasic {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        todo!()
    }
}

impl ToTurnMath for GroupRelation {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let content = match self {
            GroupRelation::IsSubgroupOf { subgroup, group } => MathNodeContent::Relationship {
                lhs: Box::new(
                    subgroup
                        .unwrap()
                        .to_turn_math(format!("{}:subgroup", master_id)),
                ),
                rhs: Box::new(group.to_turn_math(format!("{}:group", master_id))),
                operator: RelationOperatorNode::IsSubgroupOf,
            },
            GroupRelation::IsNormalSubgroupOf { subgroup, group } => {
                MathNodeContent::Relationship {
                    lhs: Box::new(
                        subgroup
                            .unwrap()
                            .to_turn_math(format!("{}:subgroup", master_id)),
                    ),
                    rhs: Box::new(group.to_turn_math(format!("{}:group", master_id))),
                    operator: RelationOperatorNode::IsNormalSubgroupOf,
                }
            }
            GroupRelation::IsIsomorphicTo { first, second } => MathNodeContent::Relationship {
                lhs: Box::new(first.to_turn_math(format!("{}:first", master_id))),
                rhs: Box::new(
                    second
                        .unwrap()
                        .to_turn_math(format!("{}:second", master_id)),
                ),
                operator: RelationOperatorNode::IsIsomorphicTo,
            },
            GroupRelation::IsQuotientOf {
                quotient,
                group,
                normal_subgroup,
            } => {
                let quotient_node = quotient
                    .unwrap()
                    .to_turn_math(format!("{}:quotient", master_id));
                let group_node = group.to_turn_math(format!("{}:group", master_id));
                let normal_subgroup_node = normal_subgroup
                    .unwrap()
                    .to_turn_math(format!("{}:n_subgroup", master_id));

                let rhs = MathNode {
                    id: format!("{}:quotient_expr", master_id),
                    content: Box::new(MathNodeContent::Fraction {
                        numerator: Box::new(group_node),
                        denominator: Box::new(normal_subgroup_node),
                    }),
                };

                MathNodeContent::Relationship {
                    lhs: Box::new(quotient_node),
                    rhs: Box::new(rhs),
                    operator: RelationOperatorNode::IsEqual,
                }
            }
            GroupRelation::IsInCenterOf { element, group } => MathNodeContent::Relationship {
                lhs: Box::new(
                    element
                        .unwrap()
                        .to_turn_math(format!("{}:element", master_id)),
                ),
                rhs: Box::new(group.to_turn_math(format!("{}:group", master_id))),
                operator: RelationOperatorNode::IsInCenterOf,
            },
            GroupRelation::AreConjugateIn {
                element1,
                element2,
                group,
            } => {
                let left_expr = element1
                    .unwrap()
                    .to_turn_math(format!("{}:element1", master_id));
                let right_expr = element2
                    .unwrap()
                    .to_turn_math(format!("{}:element2", master_id));
                let group_expr = group.to_turn_math(format!("{}:group", master_id));

                let base_relation = MathNode {
                    id: format!("{}:conj_base", master_id),
                    content: Box::new(MathNodeContent::Relationship {
                        lhs: Box::new(left_expr),
                        rhs: Box::new(right_expr),
                        operator: RelationOperatorNode::AreConjugateIn,
                    }),
                };

                let group_info = MathNode {
                    id: format!("{}:in_group", master_id),
                    content: Box::new(MathNodeContent::Text(format!(" in {:?}", group.unwrap()))),
                };

                MathNodeContent::Multiplications {
                    terms: vec![
                        (RefinedMulOrDivOperation::None, base_relation),
                        (RefinedMulOrDivOperation::None, group_info),
                    ],
                }
            }
            GroupRelation::HasOrderInGroup {
                element,
                group,
                order,
            } => {
                let element_expr = element
                    .unwrap()
                    .to_turn_math(format!("{}:element", master_id));

                let order_expr = MathNode {
                    id: format!("{}:order_expr", master_id),
                    content: Box::new(MathNodeContent::Text(format!(
                        "|{:?}| = {}",
                        element.unwrap(),
                        order.unwrap()
                    ))),
                };

                let group_info = MathNode {
                    id: format!("{}:in_group", master_id),
                    content: Box::new(MathNodeContent::Text(format!(" in {:?}", group.unwrap()))),
                };

                MathNodeContent::Multiplications {
                    terms: vec![
                        (RefinedMulOrDivOperation::None, order_expr),
                        (RefinedMulOrDivOperation::None, group_info),
                    ],
                }
            }
            GroupRelation::HasIndexInGroup {
                subgroup,
                group,
                index,
            } => {
                let index_text = format!(
                    "[{:?}:{:?}] = {}",
                    group.unwrap(),
                    subgroup.unwrap(),
                    index.unwrap()
                )
                .to_string();

                MathNodeContent::Text(index_text)
            }
            GroupRelation::HasOrder { group, order } => {
                MathNodeContent::Text(format!("|{:?}| = {}", group.unwrap(), order.unwrap()))
            }
            GroupRelation::IsCyclicWithGenerator { group, generator } => {
                let base_relation = MathNode {
                    id: format!("{}:cyclic_base", master_id),
                    content: Box::new(MathNodeContent::Relationship {
                        lhs: Box::new(group.to_turn_math(format!("{}:group", master_id))),
                        rhs: Box::new(MathNode {
                            id: format!("{}:generator_expr", master_id),
                            content: Box::new(MathNodeContent::Text(format!(
                                "<{:?}>",
                                generator.unwrap()
                            ))),
                        }),
                        operator: RelationOperatorNode::IsEqual,
                    }),
                };

                MathNodeContent::Multiplications {
                    terms: vec![(RefinedMulOrDivOperation::None, base_relation)],
                }
            }
            GroupRelation::NormalizesSubgroup {
                element,
                subgroup,
                group,
            } => {
                let relation_text = format!(
                    "{:?} normalizes {:?} in {:?}",
                    element.unwrap(),
                    subgroup.unwrap(),
                    group.unwrap()
                )
                .to_string();

                MathNodeContent::Text(relation_text)
            }
            GroupRelation::CentralizesSubgroup {
                element,
                subgroup,
                group,
            } => {
                let relation_text = format!(
                    "{:?} centralizes {:?} in {:?}",
                    element.unwrap(),
                    subgroup.unwrap(),
                    group.unwrap()
                )
                .to_string();

                MathNodeContent::Text(relation_text)
            }
            GroupRelation::IsCharacteristicSubgroupOf { subgroup, group } => {
                MathNodeContent::Text(format!("{:?} char {:?}", subgroup.unwrap(), group.unwrap()))
            }
            GroupRelation::OrderDivides { group1, group2 } => MathNodeContent::Relationship {
                lhs: Box::new(MathNode {
                    id: format!("{}:order_g1", master_id),
                    content: Box::new(MathNodeContent::Text(format!("|{:?}|", group1.unwrap()))),
                }),
                rhs: Box::new(MathNode {
                    id: format!("{}:order_g2", master_id),
                    content: Box::new(MathNodeContent::Text(format!("|{:?}|", group2.unwrap()))),
                }),
                operator: RelationOperatorNode::Divides,
            },
            GroupRelation::HasUniqueInverse { element, group } => {
                MathNodeContent::UnaryRelationship {
                    subject: Box::new(
                        element
                            .unwrap()
                            .to_turn_math(format!("{}:element", master_id)),
                    ),
                    predicate: UnaryRelationOperatorNode::HasUniqueInverse,
                }
            }
            GroupRelation::SylowSubgroupProperties { prime, group } => {
                let relation_text = format!(
                    "Sylow {:?}-subgroup properties of {:?}",
                    prime,
                    group.unwrap()
                )
                .to_string();

                MathNodeContent::Text(relation_text)
            }
            GroupRelation::IsInverseOf {
                element,
                inverse,
                group,
            } => {
                let base_relation = MathNode {
                    id: format!("{}:inverse_base", master_id),
                    content: Box::new(MathNodeContent::Relationship {
                        lhs: Box::new(MathNode {
                            id: format!("{}:element_inv", master_id),
                            content: Box::new(MathNodeContent::Text(format!(
                                "({:?})^(-1)",
                                element.unwrap()
                            ))),
                        }),
                        rhs: Box::new(
                            inverse
                                .unwrap()
                                .to_turn_math(format!("{}:inverse", master_id)),
                        ),
                        operator: RelationOperatorNode::IsEqual,
                    }),
                };

                let group_info = MathNode {
                    id: format!("{}:in_group", master_id),
                    content: Box::new(MathNodeContent::Text(format!(" in {:?}", group.unwrap()))),
                };

                MathNodeContent::Multiplications {
                    terms: vec![
                        (RefinedMulOrDivOperation::None, base_relation),
                        (RefinedMulOrDivOperation::None, group_info),
                    ],
                }
            }
            GroupRelation::IsHomomorphism {
                homomorphism,
                domain,
                codomain,
            } => MathNodeContent::Relationship {
                lhs: Box::new(
                    homomorphism
                        .unwrap()
                        .to_turn_math(format!("{}:homomorphism", master_id)),
                ),
                rhs: Box::new(MathNode {
                    id: format!("{}:domain_codomain", master_id),
                    content: Box::new(MathNodeContent::Text(format!(
                        "{:?} → {:?}",
                        domain.unwrap(),
                        codomain.unwrap()
                    ))),
                }),
                operator: RelationOperatorNode::IsHomomorphicTo,
            },
            GroupRelation::IsomorphicEmbedding { source, target } => {
                let relation_text = format!(
                    "{:?} embeds isomorphically into {:?}",
                    source.unwrap(),
                    target.unwrap()
                )
                .to_string();

                MathNodeContent::Text(relation_text)
            }
            GroupRelation::HasBasicProperty { target, property } => MathNodeContent::Text(format!(
                "{:?} is {:?}",
                target.to_turn_math(master_id.clone()),
                property.to_turn_math(master_id.clone())
            )),
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
            GroupRelation::HasOperationProperty { target, property } => todo!(),
        };
        MathNode {
            id: master_id,
            content: Box::new(content),
        }
    }
}

impl ToTurnMath for GroupProperty {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let content = match self {
            GroupProperty::Finite(property) => MathNodeContent::Text(format!(
                "{:?}",
                match property {
                    FinitePropertyVariant::Finite(n) => format!("finite of order {}", n),
                    FinitePropertyVariant::Infinite => "infinite".to_string(),
                    FinitePropertyVariant::LocallyFinite => "locally finite".to_string(),
                }
            )),
            GroupProperty::Abelian(abelian_property_variant) => MathNodeContent::Text(format!(
                "{:?}",
                match abelian_property_variant {
                    AbelianPropertyVariant::Abelian => "abelian",
                    AbelianPropertyVariant::NonAbelian => "non-abelian",
                }
            )),
            GroupProperty::Simple(simple_property_variant) => MathNodeContent::Text(format!(
                "{:?}",
                match simple_property_variant {
                    SimplePropertyVariant::Simple => "simple",
                    SimplePropertyVariant::NonSimple => "non-simple",
                    SimplePropertyVariant::QuasiSimple => "quasi-simple",
                }
            )),
            GroupProperty::Solvable(solvable_property_variant) => MathNodeContent::Text(format!(
                "{:?}",
                match solvable_property_variant {
                    SolvablePropertyVariant::Solvable => "solvable",
                    SolvablePropertyVariant::NonSolvable => "non-solvable",
                    SolvablePropertyVariant::Polysolvable => "poly-solvable",
                }
            )),
            GroupProperty::Nilpotent(nilpotent_property_variant) => MathNodeContent::Text(format!(
                "{:?}",
                match nilpotent_property_variant {
                    NilpotentPropertyVariant::Nilpotent(n) =>
                        format!("nilpotent of nilpotency {}", n),
                    NilpotentPropertyVariant::NonNilpotent => "non-nilpotent".to_string(),
                }
            )),
        };
        MathNode {
            id: master_id,
            content: Box::new(content),
        }
    }
}

impl ToTurnMath for GroupElement {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            GroupElement::Integer(n) => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text(format!("{}", n))),
            },
            GroupElement::Permutation(perm) => {
                let perm_str = format!("{:?}", perm);
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text(perm_str)),
                }
            }
            GroupElement::Matrix(matrix) => {
                let matrix_str = format!("{:?}", matrix);
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text(matrix_str)),
                }
            }
            GroupElement::Symbol(s) => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text(s.clone())),
            },
        }
    }
}

impl ToTurnMath for GroupExpression {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let content = match self {
            GroupExpression::Element { group, element } => return element.to_turn_math(master_id),
            GroupExpression::Identity(group) => return group.to_turn_math(master_id.clone()),
            GroupExpression::Operation { group, left, right } => {
                let left_node = left.as_ref().to_turn_math(format!("{}:left", master_id));
                let right_node = right.as_ref().to_turn_math(format!("{}:right", master_id));

                MathNodeContent::Multiplications {
                    terms: vec![
                        (RefinedMulOrDivOperation::None, left_node),
                        (
                            RefinedMulOrDivOperation::Multiplication(MulSymbol::Dot),
                            right_node,
                        ),
                    ],
                }
            }
            GroupExpression::Inverse { group, element } => {
                let element_node = element
                    .as_ref()
                    .to_turn_math(format!("{}:element", master_id));

                match group.unwrap().get_core().operation.inverse {
                    GroupInverse::MultiplicativeInverse => MathNodeContent::Power {
                        base: Box::new(element_node),
                        exponent: Box::new(MathNode {
                            id: format!("{}:exp", master_id),
                            content: Box::new(MathNodeContent::Text("-1".to_string())),
                        }),
                    },
                    GroupInverse::AdditiveInverse => MathNodeContent::Multiplications {
                        terms: vec![
                            (
                                RefinedMulOrDivOperation::None,
                                MathNode {
                                    id: format!("{}:neg", master_id),
                                    content: Box::new(MathNodeContent::Text("-".to_string())),
                                },
                            ),
                            (RefinedMulOrDivOperation::None, element_node),
                        ],
                    },
                    _ => MathNodeContent::Power {
                        base: Box::new(element_node),
                        exponent: Box::new(MathNode {
                            id: format!("{}:exp_fallback", master_id),
                            content: Box::new(MathNodeContent::Text("-1".to_string())),
                        }),
                    },
                }
            }
            GroupExpression::Commutator { group, a, b } => {
                let a_node = a.as_ref().to_turn_math(format!("{}:a", master_id));
                let b_node = b.as_ref().to_turn_math(format!("{}:b", master_id));

                MathNodeContent::Bracketed {
                    style: BracketStyle::Square,
                    size: BracketSize::Auto,
                    inner: Box::new(MathNode {
                        id: format!("{}:commutator_inner", master_id),
                        content: Box::new(MathNodeContent::Multiplications {
                            terms: vec![
                                (RefinedMulOrDivOperation::None, a_node),
                                (RefinedMulOrDivOperation::None, b_node),
                            ],
                        }),
                    }),
                }
            }
            GroupExpression::Coset {
                group,
                element,
                subgroup,
                is_left,
            } => {
                let element_node = element
                    .as_ref()
                    .to_turn_math(format!("{}:element", master_id));
                let subgroup_node = subgroup.to_turn_math(format!("{}:subgroup", master_id));

                let terms = if *is_left {
                    vec![
                        (RefinedMulOrDivOperation::None, element_node),
                        (RefinedMulOrDivOperation::None, subgroup_node),
                    ]
                } else {
                    vec![
                        (RefinedMulOrDivOperation::None, subgroup_node),
                        (RefinedMulOrDivOperation::None, element_node),
                    ]
                };
                MathNodeContent::Multiplications { terms }
            }
            GroupExpression::ActionOnElement { action, element } => {
                let action_node = action.to_turn_math(format!("{}:action", master_id));
                let element_node = element.to_turn_math(format!("{}:element", master_id));

                MathNodeContent::Multiplications {
                    terms: vec![
                        (RefinedMulOrDivOperation::None, action_node),
                        (RefinedMulOrDivOperation::None, element_node),
                    ],
                }
            }
            GroupExpression::Power {
                group,
                base,
                exponent,
            } => {
                let base_node = base
                    .as_ref()
                    .unwrap()
                    .to_turn_math(format!("{}:base", master_id));

                MathNodeContent::Power {
                    base: Box::new(base_node),
                    exponent: Box::new(MathNode {
                        id: format!("{}:exponent", master_id),
                        content: Box::new(MathNodeContent::Text(format!("{}", exponent.unwrap()))),
                    }),
                }
            }
            _ => MathNodeContent::Text(format!("(Expr: {:?})", self)),
        };
        MathNode {
            id: master_id,
            content: Box::new(content),
        }
    }
}

impl ToTurnMath for GroupAction {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let content = match self {
            GroupAction::SetAction {
                group,
                space,
                point,
                properties,
            } => {
                let group_node = group.to_turn_math(format!("{}:group", master_id));
                todo!()
                // let space_node = space.to_turn_math(format!("{}:space", master_id));
                // let point_node = point.to_turn_math(format!("{}:point", master_id));
                // MathNodeContent::Text(format!("Set action on {:?}", point))
            }
            GroupAction::VectorSpaceAction {
                group,
                space,
                vector,
                properties,
            } => todo!(),
            GroupAction::TopologicalSpaceAction {
                group,
                space,
                point,
                properties,
            } => todo!(),
        };
        MathNode {
            id: master_id,
            content: Box::new(content),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::super::super::VariantSet;
    use super::super::super::zfc::Set;
    use super::super::definitions::{
        AbelianPropertyVariant, FinitePropertyVariant, Group, GroupBasic, GroupIdentity,
        GroupInverse, GroupInverseApplication, GroupNotation, GroupOperation,
        GroupOperationProperty, GroupOperationVariant, GroupProperty, GroupSymbol,
    };
    use crate::turn_render::{MathNodeContent, ToTurnMath};

    #[test]
    fn test_group_to_turn_math() {
        let mut props = VariantSet::new();
        props.insert(GroupProperty::Finite(FinitePropertyVariant::Finite(5)));

        let group_core_1 = GroupBasic {
            base_set: Set::Parametric {
                parameters: HashMap::new(),
                description: "Z_5".to_string(),
                membership_condition: "x mod 5 = 0".to_string(),
                properties: VariantSet::new(),
            },
            operation: GroupOperation::default(),
            props,
        };

        let group_1 = Group::Basic(group_core_1);
        let group_math_node = group_1.to_turn_math("test_id".to_string());

        println!("Cyclic group Z_5 output: {:?}", group_math_node);

        if let MathNodeContent::Text(text) = *group_math_node.content {
            assert!(
                text.contains("Abstract Group"),
                "Group text should identify it as a group"
            );
            assert!(
                text.contains("finite"),
                "Group should be identified as finite"
            );
        } else {
            panic!("Expected Text content, got something else");
        }

        let mut props = VariantSet::new();
        props.insert(GroupProperty::Abelian(AbelianPropertyVariant::Abelian));
        props.insert(GroupProperty::Finite(FinitePropertyVariant::Finite(6)));

        let mut operation = GroupOperation::default();
        operation.operation_type = GroupOperationVariant::Multiplication;
        operation
            .properties
            .push(GroupOperationProperty::Commutative(true));

        let group_core = GroupBasic {
            base_set: Set::empty(),
            operation,
            props,
        };

        let custom_group = Group::Basic(group_core);

        let math_node = custom_group.to_turn_math("test_id_2".to_string());

        println!("Custom group output: {:?}", math_node);

        if let MathNodeContent::Text(text) = *math_node.content {
            assert!(
                text.contains("·"),
                "Multiplicative group should use · symbol"
            );
            assert!(text.contains("abelian"), "Should identify as abelian");

            let order_string = "finite (order 6)";
            assert!(text.contains(order_string), "Should show order 6");

            assert!(
                text.contains("Identity element: 1"),
                "Should show identity element"
            );
            assert!(
                text.contains("Operation is associative, commutative"),
                "Should identify operation properties"
            );
        } else {
            panic!("Expected Text content, got something else");
        }
    }
}
