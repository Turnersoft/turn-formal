use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use crate::subjects::math::export::unified_exporter::TheoryExporter;
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::location::Located;
use crate::subjects::math::formalism::theorem::Theorem;
use crate::subjects::math::theories::groups::definitions::{
    AbelianPropertyVariant, AlternatingGroup, CenterGroup, CentralProductGroup, CentralizerGroup, CommutatorSubgroup,
    CompactPropertyVariant, ConnectedPropertyVariant, CyclicGroup, DihedralGroup, FinitePropertyVariant, FreeGroup, GeneralLinearGroup, GeneratedSubgroup, GenericGroup,
    Group, GroupElement, GroupExpression, GroupHomomorphism, GroupOperation, GroupProperty,
    GroupRelation, ImageGroup, KernelGroup, LieGroup, MetrizablePropertyVariant, ModularAdditiveGroup,
    ModularMultiplicativeGroup, NilpotentPropertyVariant, NormalizerGroup, OrthogonalGroup, ProductGroup, ProductOperation,
    PullbackGroup, QuotientGroup, RestrictionGroup, SimplePropertyVariant, SolvablePropertyVariant, SpecialLinearGroup, SpecialOrthogonalGroup,
    SpecialUnitaryGroup, SubGroup, SylowSubgroup, SymmetricGroup, TopologicalGroup,
    TopologicalGroupProperty, TrivialGroup, UnitaryGroup, WreathProductGroup,
};
//--- Imports from crate::turn_render ---
use crate::turn_render::math_node::{
    Identifier, IntegralType, MathNode, MathNodeContent, MathTextSegment, MulSymbol, RefinedMulOrDivOperation,
    RelationOperatorNode, ToTurnMath, UnaryRelationOperatorNode,
};
use crate::turn_render::{RichText, RichTextSegment, TextStyle, ToRichText, *};

//--- Imports from this crate (subjects) ---
use crate::subjects::math::formalism::traits::abstraction_level::{
    AbstractionLevel, GetAbstractionLevel,
};
use crate::subjects::math::theories::VariantSet;
use crate::subjects::math::theories::fields::definitions::Field;
use crate::subjects::math::theories::fields::{FieldBasic, render::*};
use crate::subjects::math::theories::topology::definitions::{TopologicalSpace, Topology};
use crate::subjects::math::theories::zfc::definitions::Set;

use super::theorems::{
    group_inverse_uniqueness, subgroup_intersection_is_subgroup, first_isomorphism_theorem,
    element_order_divides_group_order, lagrange_theorem, cayley_theorem, sylow_first_theorem,
    fundamental_theorem_finite_abelian, normal_subgroup_test, cauchy_theorem, center_is_normal_subgroup,
};

// use super::theorems::{
//     prove_abelian_squared_criterion, prove_deduction_using_identity_uniqueness,
//     prove_example_chaining_theorems, prove_inverse_product_rule, prove_inverse_uniqueness,
//     prove_lagrange_theorem, prove_theorem_extraction_example,
// };

// Include all the render modules
pub mod alternating_group;
pub mod cyclic_group;
pub mod dihedral_group;
pub mod group_basic;
pub mod lie_group;
pub mod product_group;
pub mod symmetric_group;
pub mod tests;
pub mod topological_group;

pub mod advanced_constructions;
pub mod linear_groups;
pub mod modular_groups;
pub mod quotient_groups;
pub mod subgroup_constructions;

// use crate::subjects::math::formalism::location::Located;
// use crate::subjects::math::formalism::relations::MathRelation;
// use crate::subjects::math::theories::groups::definitions::{
//     Group, GroupExpression, GroupRelation,
// };
// use crate::turn_render::{
//     self, BracketSize, BracketStyle, MathNode, MathNodeContent, RelationOperatorNode, ToTurnMath,
// };

// === TOMATH IMPLEMENTATIONS ===

impl ToTurnMath for Group {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            Group::Generic(g) => g.to_turn_math(master_id),
            Group::Trivial(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Trivial group ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::Symmetric(g) => g.to_turn_math(master_id),
            Group::Alternating(g) => g.to_turn_math(master_id),
            Group::Cyclic(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Cyclic group ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::Dihedral(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Dihedral group ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::GeneralLinear(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("General linear group ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::SpecialLinear(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Special linear group ".to_string()),
                        MathTextSegment::Math(g.general_linear.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::Orthogonal(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Orthogonal group ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::SpecialOrthogonal(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Special orthogonal group ".to_string()),
                        MathTextSegment::Math(g.orthogonal.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::Unitary(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Unitary group ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::SpecialUnitary(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Special unitary group ".to_string()),
                        MathTextSegment::Math(g.unitary.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::Topological(g) => {
                let id = master_id.clone();
                // Extract the core group properties and format them
                let core_props = g.core.to_turn_math(format!("{}-core", id.clone()));
                
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Topological Group ".to_string()),
                        MathTextSegment::Math(core_props),
                        MathTextSegment::Text(" [".to_string()),
                        MathTextSegment::Text("]".to_string()),
                    ])),
                }
            },
            Group::Lie(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Lie group ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::Product(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Product group ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::ModularAdditive(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Modular additive group ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::ModularMultiplicative(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Modular multiplicative group ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::Free(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Free group ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::Quotient(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::BinaryOperation {
                        operation_type: BinaryOperationType::GroupQuotient,
                        terms: vec![
                            (BinaryOperator::Slash, g.group.to_turn_math(format!("{}-group", id.clone()))),
                            (BinaryOperator::Slash, g.normal_subgroup.to_turn_math(format!("{}-normal", id))),
                        ],
                    }),
                }
            },
            Group::Kernel(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::FunctionCall {
                        name: Arc::new(MathNode {
                            id: format!("{}-ker-name", id.clone()),
                            content: Arc::new(MathNodeContent::Identifier(Identifier {
                                body: "Ker".to_string(),
                                pre_script: None,
                                mid_script: None,
                                post_script: None,
                                primes: 0,
                                is_function: true,
                            })),
                        }),
                        parameters: vec![g.defining_homomorphism.to_turn_math("".to_string())],
                    }),
                }
            },
            Group::Image(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::FunctionCall {
                        name: Arc::new(MathNode {
                            id: format!("{}-im-name", id.clone()),
                            content: Arc::new(MathNodeContent::Identifier(Identifier {
                                body: "Im".to_string(),
                                pre_script: None,
                                mid_script: None,
                                post_script: None,
                                primes: 0,
                                is_function: true,
                            })),
                        }),
                        parameters: vec![g.defining_homomorphism.to_turn_math("".to_string())],
                    }),
                }
            },
            Group::Center(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Center group ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::GeneratedSubgroup(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Generated subgroup ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::Normalizer(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Normalizer group ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::Centralizer(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Centralizer group ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::CommutatorSubgroup(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Commutator subgroup ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::SylowSubgroup(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Sylow subgroup ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::CentralProduct(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Central product group ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::WreathProduct(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Wreath product group ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::Pullback(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Pullback group ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::Restriction(g) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Restriction group ".to_string()),
                        MathTextSegment::Math(g.core.to_turn_math(format!("{}-core", id))),
                    ])),
                }
            },
            Group::Interception(g) => {
                // Render as "H ∩ K" using the actual subgroup fields
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::BinaryOperation {
                        operation_type: BinaryOperationType::SetIntersection,
                        terms: vec![
                            (BinaryOperator::Intersection, g.first_subgroup.to_turn_math(format!("{}-first", id.clone()))),
                            (BinaryOperator::Intersection, g.second_subgroup.to_turn_math(format!("{}-second", id))),
                        ],
                    }),
                }
            },
            Group::SubGroup(g) => {
                // Render as "Subgroup of G" where G is the parent group
                MathNode {
                    id: master_id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Subgroup of ".to_string()),
                        MathTextSegment::Math(g.parent_group.value().to_turn_math(format!("{}-parent", master_id))),
                    ])),
                }
            },
        }
    }
}

impl ToSectionNode for Group {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        match self {
            Group::Generic(g) => g.to_section_node(id_prefix),
            Group::Trivial(g) => g.core.to_section_node(id_prefix),
            Group::Symmetric(g) => g.to_section_node(id_prefix),
            Group::Alternating(g) => g.to_section_node(id_prefix),
            Group::Cyclic(g) => g.to_section_node(id_prefix),
            Group::Dihedral(g) => g.to_section_node(id_prefix),
            Group::GeneralLinear(g) => g.to_section_node(id_prefix),
            Group::SpecialLinear(g) => g.to_section_node(id_prefix),
            Group::Orthogonal(g) => g.to_section_node(id_prefix),
            Group::SpecialOrthogonal(g) => g.to_section_node(id_prefix),
            Group::Unitary(g) => g.to_section_node(id_prefix),
            Group::SpecialUnitary(g) => g.to_section_node(id_prefix),
            Group::Topological(g) => g.to_section_node(id_prefix),
            Group::Lie(g) => g.to_section_node(id_prefix),
            Group::Product(g) => g.to_section_node(id_prefix),
            Group::ModularAdditive(g) => g.to_section_node(id_prefix),
            Group::ModularMultiplicative(g) => g.to_section_node(id_prefix),
            Group::Free(g) => g.core.to_section_node(id_prefix),
            Group::Quotient(g) => g.core.to_section_node(id_prefix),
            Group::Kernel(g) => g.core.to_section_node(id_prefix),
            Group::Image(g) => g.core.to_section_node(id_prefix),
            Group::Center(g) => g.core.to_section_node(id_prefix),
            Group::GeneratedSubgroup(g) => g.core.to_section_node(id_prefix),
            Group::Normalizer(g) => g.core.to_section_node(id_prefix),
            Group::Centralizer(g) => g.core.to_section_node(id_prefix),
            Group::CommutatorSubgroup(g) => g.core.to_section_node(id_prefix),
            Group::SylowSubgroup(g) => g.core.to_section_node(id_prefix),
            Group::CentralProduct(g) => g.core.to_section_node(id_prefix),
            Group::WreathProduct(g) => g.core.to_section_node(id_prefix),
            Group::Pullback(g) => g.core.to_section_node(id_prefix),
            Group::Restriction(g) => g.core.to_section_node(id_prefix),
            Group::Interception(g) => g.core.to_section_node(id_prefix),
            Group::SubGroup(g) => g.core.to_section_node(id_prefix),
        }
    }
}

impl ToMathDocument for Group {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        // For specialized groups, use their own rendering even at L1 level
        // Only use the generic L1 schema for truly generic groups
        match self {
            Group::Generic(g) => {
                let level = g.level();
                if level == AbstractionLevel::Level1 {
                    g.render_as_l1_schema_document(&format!("{}.generic", id_prefix))
                } else {
                    g.to_math_document(&format!("{}.generic", id_prefix))
                }
            }
            // All specialized groups use their own rendering methods even at L1
            Group::Trivial(g) => g.to_math_document(&format!("{}.trivial", id_prefix)),
            Group::Symmetric(g) => g.to_math_document(&format!("{}.symmetric", id_prefix)),
            Group::Alternating(g) => g.to_math_document(&format!("{}.alternating", id_prefix)),
            Group::Cyclic(g) => g.to_math_document(&format!("{}.cyclic", id_prefix)),
            Group::Dihedral(g) => g.to_math_document(&format!("{}.dihedral", id_prefix)),
            Group::GeneralLinear(g) => g.to_math_document(&format!("{}.general_linear", id_prefix)),
            Group::SpecialLinear(g) => g.to_math_document(&format!("{}.special_linear", id_prefix)),
            Group::Orthogonal(g) => g.to_math_document(&format!("{}.orthogonal", id_prefix)),
            Group::SpecialOrthogonal(g) => {
                g.to_math_document(&format!("{}.special_orthogonal", id_prefix))
            }
            Group::Unitary(g) => g.to_math_document(&format!("{}.unitary", id_prefix)),
            Group::SpecialUnitary(g) => {
                g.to_math_document(&format!("{}.special_unitary", id_prefix))
            }
            Group::Topological(g) => g.to_math_document(&format!("{}.topological", id_prefix)),
            Group::Lie(g) => g.to_math_document(&format!("{}.lie", id_prefix)),
            Group::Product(g) => g.to_math_document(&format!("{}.product", id_prefix)),
            Group::ModularAdditive(g) => {
                g.to_math_document(&format!("{}.modular_additive", id_prefix))
            }
            Group::ModularMultiplicative(g) => {
                g.to_math_document(&format!("{}.modular_multiplicative", id_prefix))
            }
            Group::Free(g) => g.to_math_document(&format!("{}.free", id_prefix)),
            Group::Quotient(g) => g.to_math_document(&format!("{}.quotient", id_prefix)),
            Group::Kernel(g) => g.to_math_document(&format!("{}.kernel", id_prefix)),
            Group::Image(g) => g.to_math_document(&format!("{}.image", id_prefix)),
            Group::Center(g) => g.to_math_document(&format!("{}.center", id_prefix)),
            Group::GeneratedSubgroup(g) => {
                g.to_math_document(&format!("{}.generated_subgroup", id_prefix))
            }
            Group::Normalizer(g) => g.to_math_document(&format!("{}.normalizer", id_prefix)),
            Group::Centralizer(g) => g.to_math_document(&format!("{}.centralizer", id_prefix)),
            Group::CommutatorSubgroup(g) => {
                g.to_math_document(&format!("{}.commutator_subgroup", id_prefix))
            }
            Group::SylowSubgroup(g) => g.to_math_document(&format!("{}.sylow_subgroup", id_prefix)),
            Group::CentralProduct(g) => {
                g.to_math_document(&format!("{}.central_product", id_prefix))
            }
            Group::WreathProduct(g) => g.to_math_document(&format!("{}.wreath_product", id_prefix)),
            Group::Pullback(g) => g.to_math_document(&format!("{}.pullback", id_prefix)),
            Group::Restriction(g) => g.to_math_document(&format!("{}.restriction", id_prefix)),
            Group::Interception(g) => {
                // For now, use the core group rendering but with a custom ID
                // TODO: Implement proper intersection rendering as "H ∩ K"
                g.core.to_math_document(&format!("{}.interception", id_prefix))
            },
            Group::SubGroup(g) => {
                // For now, use the core group rendering but with a custom ID
                // TODO: Implement proper subgroup rendering as "SubGroup(G)"
                g.core.to_math_document(&format!("{}.subgroup", id_prefix))
            },
        }
    }
}

// === IMPLEMENTATIONS FOR BOX<GROUP> ===

impl ToTurnMath for Box<Group> {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        (**self).to_turn_math(master_id)
    }
}

// === GROUPELEMENT IMPLEMENTATIONS ===

impl ToTurnMath for GroupElement {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            GroupElement::Integer(n) => MathNode {
                id: master_id,
                content: Arc::new(MathNodeContent::Quantity {
                    number: n.to_string(),
                    scientific_notation: None,
                    unit: None,
                }),
            },
            GroupElement::Symbol(s) => MathNode {
                id: master_id,
                content: Arc::new(MathNodeContent::Identifier(Identifier {
                    body: s.clone(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                })),
            },
            GroupElement::Permutation(_) => MathNode {
                id: master_id,
                content: Arc::new(MathNodeContent::Identifier(Identifier {
                    body: "σ".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                })),
            },
            GroupElement::Matrix(_) => MathNode {
                id: master_id,
                content: Arc::new(MathNodeContent::Identifier(Identifier {
                    body: "M".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                })),
            },
        }
    }
}

// === GROUPEXPRESSION IMPLEMENTATIONS ===

impl ToTurnMath for GroupExpression {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            GroupExpression::Operation { left, right, .. } => MathNode {
                id: master_id,
                content: Arc::new(MathNodeContent::Multiplications {
                    terms: vec![
                        (
                            RefinedMulOrDivOperation::None,
                            left.data.to_turn_math(left.id.clone()),
                        ),
                        (
                            RefinedMulOrDivOperation::Multiplication(MulSymbol::Dot),
                            right.data.to_turn_math(right.id.clone()),
                        ),
                    ],
                }),
            },
            GroupExpression::Element { group, element } => match element {
                Some(param_element) => param_element.value().to_turn_math(master_id),
                None => {
                    // For type rendering, we want to show the group name, not the element name
                    // Check if this is being used as a type (no specific element)
                    if master_id.contains("-type-") {
                        // Render as the group name (e.g., "G")
                        group.value().to_turn_math(master_id)
                    } else {
                                // Just render as a generic element g
                                let id = master_id.clone();
                        MathNode {
                                    id: id.clone(),
                                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                                        MathTextSegment::Text("Element of ".to_string()),
                                        MathTextSegment::Math(group.value().to_turn_math(format!("{}-group", id))),
                                    ])),
                        }
                    }
                }
            },
            GroupExpression::Identity(group) => {
                // For type rendering, we want to show the group name, not the identity name
                // Check if this is being used as a type
                if master_id.contains("-type-") {
                    // Render as the group name (e.g., "G")
                    group.value().to_turn_math(master_id)
                } else {
                            // Just render as the identity element e
                            let id = master_id.clone();
                    MathNode {
                                id: id.clone(),
                                content: Arc::new(MathNodeContent::RichTextContent(vec![
                                    MathTextSegment::Text("Identity of ".to_string()),
                                    MathTextSegment::Math(group.value().to_turn_math(format!("{}-group", id))),
                                ])),
                            }
                        }
                    },
            GroupExpression::Inverse { element, .. } => {
                        // Just render as the inverse power g⁻¹
                        let id = master_id.clone();
                        MathNode {
                            id: id.clone(),
                content: Arc::new(MathNodeContent::Power {
                                base: Arc::new(element.value().to_turn_math(format!("{}-base", id.clone()))),
                    exponent: Arc::new(MathNode {
                                    id: format!("{}-exp", id.clone()),
                        content: Arc::new(MathNodeContent::Quantity {
                            number: "-1".to_string(),
                            scientific_notation: None,
                            unit: None,
                        }),
                    }),
                }),
                        }
                    },
            GroupExpression::Commutator { group, a, b } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Commutator in ".to_string()),
                        MathTextSegment::Math(group.value().to_turn_math(format!("{}-group", id))),
                    ])),
                }
            },
            GroupExpression::Coset { group, element, subgroup, is_left } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text(if *is_left { "Left coset in " } else { "Right coset in " }.to_string()),
                        MathTextSegment::Math(group.value().to_turn_math(format!("{}-group", id))),
                    ])),
                }
            },
            GroupExpression::ActionOnElement { action, element } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Action on element".to_string()),
                    ])),
                }
            },
            GroupExpression::Power { group, base, exponent } => {
                let id = master_id.clone();
                let exp_value = match &exponent.data {
                    Parametrizable::Concrete(val) => val.to_string(),
                    Parametrizable::Variable(_) => "n".to_string(), // Use generic variable name
                };
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::Power {
                        base: Arc::new(base.value().to_turn_math(format!("{}-base", id.clone()))),
                        exponent: Arc::new(MathNode {
                            id: format!("{}-exp", id),
                            content: Arc::new(MathNodeContent::Quantity {
                                number: exp_value,
                                scientific_notation: None,
                                unit: None,
                            }),
                        }),
                    }),
                }
            },
            GroupExpression::GroupOrder { group } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Order of ".to_string()),
                        MathTextSegment::Math(group.value().to_turn_math(format!("{}-group", id))),
                    ])),
                }
            },
            GroupExpression::ElementOrder { element, group } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Order of element in ".to_string()),
                        MathTextSegment::Math(group.value().to_turn_math(format!("{}-group", id))),
                    ])),
                }
            },
            GroupExpression::Homomorphism(located) => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Homomorphism".to_string()),
                    ])),
                }
            },
                    }
    }
}

impl ToRichText for GroupExpression {
    fn to_rich_text(&self) -> RichText {
        match self {
            GroupExpression::Element { group, .. } => {
                // For group elements, show "element of G" where G is the group name
                let group_node = match &group.data {
                    Parametrizable::Concrete(_) => group.value().to_turn_math("group-name".to_string()),
                    Parametrizable::Variable(id) => MathNode {
                        id: "group-name".to_string(),
                        content: Arc::new(MathNodeContent::Identifier(Identifier {
                            body: id.to_string(),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: false,
                        })),
                    },
                };
                RichText {
                    segments: vec![
                        RichTextSegment::Text("element of ".to_string()),
                        RichTextSegment::Math(group_node),
                    ],
                    alignment: None,
                }
            }
            GroupExpression::Identity(group) => {
                // For identity elements, show "identity of G" where G is the group name
                let group_node = match &group.data {
                    Parametrizable::Concrete(_) => group.value().to_turn_math("group-name".to_string()),
                    Parametrizable::Variable(id) => MathNode {
                        id: "group-name".to_string(),
                        content: Arc::new(MathNodeContent::Identifier(Identifier {
                            body: id.to_string(),
                            pre_script: None,
                            mid_script: None,
                            post_script: None,
                            primes: 0,
                            is_function: false,
                        })),
                    },
                };
                RichText {
                    segments: vec![
                        RichTextSegment::Text("identity of ".to_string()),
                        RichTextSegment::Math(group_node),
                    ],
                    alignment: None,
                }
            }
            GroupExpression::Inverse { .. } => {
                // For inverse elements, show "inverse of element"
                RichText {
                    segments: vec![RichTextSegment::Text("inverse of element".to_string())],
                    alignment: None,
                }
            }
            GroupExpression::Operation { .. } => {
                RichText {
                    segments: vec![RichTextSegment::Text("group operation".to_string())],
                    alignment: None,
                }
            }
            GroupExpression::Commutator { group, a, b } => {
                RichText {
                    segments: vec![
                        RichTextSegment::Text("Commutator in ".to_string()),
                        RichTextSegment::Math(group.value().to_turn_math("commutator-group".to_string())),
                    ],
                alignment: None,
                }
            },
            GroupExpression::Coset { group, element, subgroup, is_left } => {
                RichText {
                    segments: vec![
                        RichTextSegment::Text(if *is_left { "Left coset in " } else { "Right coset in " }.to_string()),
                        RichTextSegment::Math(group.value().to_turn_math("coset-group".to_string())),
                    ],
                    alignment: None,
                }
            },
            GroupExpression::ActionOnElement { action, element } => {
                RichText {
                    segments: vec![
                        RichTextSegment::Text("Action on element".to_string()),
                    ],
                    alignment: None,
                }
            },
            GroupExpression::Power { group, base, exponent } => {
                RichText {
                    segments: vec![
                        RichTextSegment::Text("Power in ".to_string()),
                        RichTextSegment::Math(group.value().to_turn_math("power-group".to_string())),
                    ],
                    alignment: None,
                }
            },
            GroupExpression::GroupOrder { group } => {
                RichText {
                    segments: vec![
                        RichTextSegment::Text("Order of ".to_string()),
                        RichTextSegment::Math(group.value().to_turn_math("order-group".to_string())),
                    ],
                    alignment: None,
                }
            },
            GroupExpression::ElementOrder { element, group } => {
                RichText {
                    segments: vec![
                        RichTextSegment::Text("Order of element in ".to_string()),
                        RichTextSegment::Math(group.value().to_turn_math("element-order-group".to_string())),
                    ],
                    alignment: None,
                }
            },
            GroupExpression::Homomorphism(located) => {
                located.to_rich_text()
            },
                    }
    }
}

impl ToRichText for GroupHomomorphism {
    fn to_rich_text(&self) -> RichText {
        RichText {
            segments: vec![
                RichTextSegment::Text("group homomorphism from ".to_string()),
                RichTextSegment::Math(self.domain.to_turn_math("domain".to_string())),
                RichTextSegment::Text(" to ".to_string()),
                RichTextSegment::Math(self.codomain.to_turn_math("codomain".to_string())),
            ],
            alignment: None,
        }
    }
}

impl ToTurnMath for GroupHomomorphism {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id,
            content: Arc::new(MathNodeContent::RichTextContent(vec![
                MathTextSegment::Text("Group homomorphism from ".to_string()),
                MathTextSegment::Math(self.domain.to_turn_math("domain".to_string())),
                MathTextSegment::Text(" to ".to_string()),
                MathTextSegment::Math(self.codomain.to_turn_math("codomain".to_string())),
            ])),
        }
    }
}

// === GROUPRELATION IMPLEMENTATIONS ===

impl ToTurnMath for GroupRelation {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            GroupRelation::IsSubgroupOf { subgroup, group } => {
                        // Use proper Relationship for H ⊆ G
                        let id = master_id.clone();
                        MathNode {
                            id: id.clone(),
                content: Arc::new(MathNodeContent::Relationship {
                    lhs: Arc::new(MathNode {
                                    id: format!("{}-sub", id.clone()),
                                    content: Arc::new(MathNodeContent::Identifier(Identifier {
                                        body: "H".to_string(),
                                        pre_script: None,
                                        mid_script: None,
                                        post_script: None,
                                        primes: 0,
                                        is_function: false,
                                    })),
                    }),
                    operator: RelationOperatorNode::SubsetOf,
                    rhs: Arc::new(MathNode {
                                    id: format!("{}-group", id.clone()),
                                    content: Arc::new(MathNodeContent::Identifier(Identifier {
                                        body: "G".to_string(),
                                        pre_script: None,
                                        mid_script: None,
                                        post_script: None,
                                        primes: 0,
                                        is_function: false,
                                    })),
                    }),
                }),
                        }
                    },
            GroupRelation::IsIsomorphicTo { first, second } => {
                // Use proper Relationship for G ≅ H
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                content: Arc::new(MathNodeContent::Relationship {
                        lhs: Arc::new(first.to_turn_math(format!("{}-first", id.clone()))),
                        operator: RelationOperatorNode::IsIsomorphicTo,
                        rhs: Arc::new(second.to_turn_math(format!("{}-second", id.clone()))),
                    }),
                }
            },
            GroupRelation::HasOrder { group, order } => {
                        // Use proper Relationship for |G| = n
                        let id = master_id.clone();
                        MathNode {
                            id: id.clone(),
                content: Arc::new(MathNodeContent::Relationship {
                    lhs: Arc::new(MathNode {
                                    id: format!("{}-order", id.clone()),
                        content: Arc::new(MathNodeContent::Text("|G|".to_string())),
                    }),
                    operator: RelationOperatorNode::Equal,
                    rhs: Arc::new(MathNode {
                                    id: format!("{}-value", id.clone()),
                                    content: Arc::new(MathNodeContent::Identifier(Identifier {
                                        body: "n".to_string(),
                                        pre_script: None,
                                        mid_script: None,
                                        post_script: None,
                                        primes: 0,
                                        is_function: false,
                                    })),
                    }),
                }),
                        }
                    },
            GroupRelation::IsNormalSubgroupOf { subgroup, group } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Normal subgroup of ".to_string()),
                        MathTextSegment::Math(group.value().to_turn_math(format!("{}-group", id))),
                    ])),
                }
            },
            GroupRelation::IsQuotientOf { quotient, group, normal_subgroup } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Quotient of ".to_string()),
                        MathTextSegment::Math(group.value().to_turn_math(format!("{}-group", id))),
                    ])),
                }
            },
            GroupRelation::IsInCenterOf { element, group } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Element in center of ".to_string()),
                        MathTextSegment::Math(group.value().to_turn_math(format!("{}-group", id))),
                    ])),
                }
            },
            GroupRelation::AreConjugateIn { element1, element2, group } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Conjugate elements in ".to_string()),
                        MathTextSegment::Math(group.value().to_turn_math(format!("{}-group", id))),
                    ])),
                }
            },
            GroupRelation::HasOrderInGroup { element, group, order } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Element order in ".to_string()),
                        MathTextSegment::Math(group.value().to_turn_math(format!("{}-group", id))),
                    ])),
                }
            },
            GroupRelation::HasIndexInGroup { subgroup, group, index } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Subgroup index in ".to_string()),
                        MathTextSegment::Math(group.value().to_turn_math(format!("{}-group", id))),
                    ])),
                }
            },
            GroupRelation::IsCyclicWithGenerator { group, generator } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Cyclic group with generator ".to_string()),
                        MathTextSegment::Math(generator.value().to_turn_math(format!("{}-gen", id))),
                    ])),
                }
            },
            GroupRelation::NormalizesSubgroup { element, subgroup, group } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Element normalizes subgroup in ".to_string()),
                        MathTextSegment::Math(group.value().to_turn_math(format!("{}-group", id))),
                    ])),
                }
            },
            GroupRelation::CentralizesSubgroup { element, subgroup, group } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Element centralizes subgroup in ".to_string()),
                        MathTextSegment::Math(group.value().to_turn_math(format!("{}-group", id))),
                    ])),
                }
            },
            GroupRelation::IsCharacteristicSubgroupOf { subgroup, group } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Characteristic subgroup of ".to_string()),
                        MathTextSegment::Math(group.value().to_turn_math(format!("{}-group", id))),
                    ])),
                }
            },
            GroupRelation::OrderDivides { group1, group2 } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Order of ".to_string()),
                        MathTextSegment::Math(group1.value().to_turn_math(format!("{}-group1", id.clone()))),
                        MathTextSegment::Text(" divides order of ".to_string()),
                        MathTextSegment::Math(group2.value().to_turn_math(format!("{}-group2", id.clone()))),
                    ])),
                }
            },
            GroupRelation::HasUniqueInverse { element, group } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Element has unique inverse in ".to_string()),
                        MathTextSegment::Math(group.value().to_turn_math(format!("{}-group", id))),
                    ])),
                }
            },
            GroupRelation::SylowSubgroupProperties { prime, group } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Sylow subgroup properties in ".to_string()),
                        MathTextSegment::Math(group.value().to_turn_math(format!("{}-group", id))),
                    ])),
                }
            },
            GroupRelation::IsInverseOf { element, inverse, group } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Inverse relationship in ".to_string()),
                        MathTextSegment::Math(group.value().to_turn_math(format!("{}-group", id))),
                    ])),
                }
            },
            GroupRelation::IsHomomorphism { homomorphism, domain, codomain } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Homomorphism from ".to_string()),
                        MathTextSegment::Math(domain.value().to_turn_math(format!("{}-domain", id.clone()))),
                        MathTextSegment::Text(" to ".to_string()),
                        MathTextSegment::Math(codomain.value().to_turn_math(format!("{}-codomain", id))),
                    ])),
                }
            },
            GroupRelation::IsomorphicEmbedding { source, target } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Isomorphic embedding from ".to_string()),
                        MathTextSegment::Math(source.value().to_turn_math(format!("{}-source", id.clone()))),
                        MathTextSegment::Text(" to ".to_string()),
                        MathTextSegment::Math(target.value().to_turn_math(format!("{}-target", id))),
                    ])),
                }
            },
            GroupRelation::HasBasicProperty { target, property } => {
                let id = master_id.clone();
                let property_text = match property {
                    GroupProperty::Abelian(AbelianPropertyVariant::Abelian) => "abelian",
                    GroupProperty::Abelian(AbelianPropertyVariant::NonAbelian) => "non-abelian",
                    GroupProperty::Finite(FinitePropertyVariant::Finite(n)) => &format!("finite of order {}", n),
                    GroupProperty::Finite(FinitePropertyVariant::Infinite) => "infinite",
                    GroupProperty::Finite(FinitePropertyVariant::LocallyFinite) => "locally finite",
                    GroupProperty::Simple(SimplePropertyVariant::Simple) => "simple",
                    GroupProperty::Simple(SimplePropertyVariant::NonSimple) => "non-simple",
                    GroupProperty::Simple(SimplePropertyVariant::QuasiSimple) => "quasi-simple",
                    GroupProperty::Solvable(SolvablePropertyVariant::Solvable) => "solvable",
                    GroupProperty::Solvable(SolvablePropertyVariant::NonSolvable) => "non-solvable",
                    GroupProperty::Solvable(SolvablePropertyVariant::Polysolvable) => "polysolvable",
                    GroupProperty::Nilpotent(NilpotentPropertyVariant::Nilpotent(n)) => &format!("nilpotent of class {}", n),
                    GroupProperty::Nilpotent(NilpotentPropertyVariant::NonNilpotent) => "non-nilpotent",
                };
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Has ".to_string()),
                        MathTextSegment::Text(property_text.to_string()),
                        MathTextSegment::Text(" property in ".to_string()),
                        MathTextSegment::Math(target.value().to_turn_math(format!("{}-target", id))),
                    ])),
                }
            },
            GroupRelation::HasTopologicalProperty { target, property } => {
                let id = master_id.clone();
                let property_text = match property {
                    TopologicalGroupProperty::Compact(CompactPropertyVariant::Compact) => "compact",
                    TopologicalGroupProperty::Compact(CompactPropertyVariant::NonCompact) => "non-compact",
                    TopologicalGroupProperty::Compact(CompactPropertyVariant::LocallyCompact) => "locally compact",
                    TopologicalGroupProperty::Connected(ConnectedPropertyVariant::Connected) => "connected",
                    TopologicalGroupProperty::Connected(ConnectedPropertyVariant::SimplyConnected) => "simply connected",
                    TopologicalGroupProperty::Connected(ConnectedPropertyVariant::TotallyDisconnected) => "totally disconnected",
                    TopologicalGroupProperty::Connected(ConnectedPropertyVariant::LocallyConnected) => "locally connected",
                    TopologicalGroupProperty::Connected(ConnectedPropertyVariant::LocallySimplyConnected) => "locally simply connected",
                    TopologicalGroupProperty::Metrizable(MetrizablePropertyVariant::Metrizable) => "metrizable",
                    TopologicalGroupProperty::Metrizable(MetrizablePropertyVariant::NonMetrizable) => "non-metrizable",
                };
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Has ".to_string()),
                        MathTextSegment::Text(property_text.to_string()),
                        MathTextSegment::Text(" topological property in ".to_string()),
                        MathTextSegment::Math(target.value().to_turn_math(format!("{}-target", id))),
                    ])),
                }
            },
            GroupRelation::HasLieProperty { target, property } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Has Lie property".to_string()),
                    ])),
                }
            },
            GroupRelation::HasActionProperty { target, property } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Has action property".to_string()),
                    ])),
                }
            },
            GroupRelation::HasProductProperty { target, property } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Has product property".to_string()),
                    ])),
                }
            },
            GroupRelation::HasModularAdditiveProperty { target, property } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Has modular additive property".to_string()),
                    ])),
                }
            },
            GroupRelation::HasModularMultiplicativeProperty { target, property } => {
                let id = master_id.clone();
                MathNode {
                    id: master_id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Has modular multiplicative property".to_string()),
                    ])),
                }
            },
            GroupRelation::HasGeneralLinearMatrixProperty { target, property } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Has general linear matrix property".to_string()),
                    ])),
                }
            },
            GroupRelation::HasGeneralLinearLinearProperty { target, property } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Has general linear property".to_string()),
                    ])),
                }
            },
            GroupRelation::HasSpecialLinearProperty { target, property } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Has special linear property".to_string()),
                    ])),
                }
            },
            GroupRelation::HasOrthogonalMatrixProperty { target, property } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Has orthogonal matrix property".to_string()),
                    ])),
                }
            },
            GroupRelation::HasSpecialOrthogonalProperty { target, property } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Has special orthogonal property".to_string()),
                    ])),
                }
            },
            GroupRelation::HasUnitaryMatrixProperty { target, property } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Has unitary matrix property".to_string()),
                    ])),
                }
            },
            GroupRelation::HasSpecialUnitaryProperty { target, property } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Has special unitary property".to_string()),
                    ])),
                }
            },
            GroupRelation::HasAlternatingPermutationProperty { target, property } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Has alternating permutation property".to_string()),
                    ])),
                }
            },
            GroupRelation::HasFreeProperty { target, property } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Has free property".to_string()),
                    ])),
                }
            },
            GroupRelation::HasQuotientProperty { target, property } => {
                let id = master_id.clone();
                MathNode {
                    id: id.clone(),
                    content: Arc::new(MathNodeContent::RichTextContent(vec![
                        MathTextSegment::Text("Has quotient property".to_string()),
                    ])),
                }
            },
                    }
    }
}


/// Group Theory Exporter Implementation
pub struct GroupTheoryExporter;

impl TheoryExporter<Group, GroupExpression, GroupRelation> for GroupTheoryExporter {
    fn theory_id(&self) -> &str {
        "group_theory"
    }

    fn theory_name(&self) -> &str {
        "Group Theory"
    }

    fn export_theory_overview(&self) -> MathDocument {
        // **DRY PRINCIPLE**: Get ALL actual exported content to maximize references
        let all_definitions = self.export_object_definitions(self.generate_object_definitions());
        let all_expressions =
            self.export_expression_definitions(self.generate_expression_definitions());
        let all_relations = self.export_relation_definitions(self.generate_relation_definitions());
        let all_theorems = self.export_theorems();

        // Extract real IDs from ALL exported content
        let definition_links: Vec<String> = all_definitions
            .iter()
            .map(|content| content.id.clone())
            .collect();

        let expression_links: Vec<String> = all_expressions
            .iter()
            .map(|content| content.id.clone())
            .collect();

        let relation_links: Vec<String> = all_relations
            .iter()
            .map(|content| content.id.clone())
            .collect();

        let theorem_links: Vec<String> = all_theorems
            .iter()
            .map(|content| content.id.clone())
            .collect();

        // Organize definitions by category using actual IDs
        let fundamental_links: Vec<String> = definition_links
            .iter()
            .filter(|id| id.contains("generic") || id.contains("trivial"))
            .cloned()
            .collect();

        let enriched_links: Vec<String> = definition_links
            .iter()
            .filter(|id| id.contains("topological") || id.contains("lie"))
            .cloned()
            .collect();

        let concrete_links: Vec<String> = definition_links
            .iter()
            .filter(|id| {
                id.contains("cyclic")
                    || id.contains("symmetric")
                    || id.contains("dihedral")
                    || id.contains("alternating")
            })
            .cloned()
            .collect();

        let matrix_links: Vec<String> = definition_links
            .iter()
            .filter(|id| {
                id.contains("general_linear")
                    || id.contains("special_linear")
                    || id.contains("orthogonal")
                    || id.contains("unitary")
            })
            .cloned()
            .collect();

        let modular_links: Vec<String> = definition_links
            .iter()
            .filter(|id| id.contains("modular_additive") || id.contains("modular_multiplicative"))
            .cloned()
            .collect();

        let combinatorial_links: Vec<String> = definition_links
            .iter()
            .filter(|id| id.contains("free"))
            .cloned()
            .collect();

        let operations_links: Vec<String> = definition_links
            .iter()
            .filter(|id| {
                id.contains("product")
                    || id.contains("quotient")
                    || id.contains("kernel")
                    || id.contains("image")
            })
            .cloned()
            .collect();

        let subgroup_links: Vec<String> = definition_links
            .iter()
            .filter(|id| {
                id.contains("center")
                    || id.contains("generated_subgroup")
                    || id.contains("normalizer")
                    || id.contains("centralizer")
                    || id.contains("commutator_subgroup")
                    || id.contains("sylow_subgroup")
            })
            .cloned()
            .collect();

        let advanced_links: Vec<String> = definition_links
            .iter()
            .filter(|id| {
                id.contains("wreath_product")
                    || id.contains("central_product")
                    || id.contains("pullback")
                    || id.contains("restriction")
            })
            .cloned()
            .collect();

        // ALL content links for comprehensive navigation
        let all_content_links: Vec<String> = [
            definition_links.clone(),
            expression_links.clone(),
            relation_links.clone(),
            theorem_links.clone(),
        ]
        .concat();

        // Create comprehensive theory overview page as main entrance
        MathDocument {
            id: "group_theory.theory_overview.main".to_string(),
            content_type: MathDocumentType::ScientificPaper(ScientificPaperContent {
                title: "Group Theory: Mathematical Framework Overview".to_string(),
                paper_type: PaperType::Research,
                venue: None,
                peer_reviewed: false,
                content_metadata: ContentMetadata {
                    language: Some("en-US".to_string()),
                    version: Some("1.0".to_string()),
                    created_at: None,
                    last_modified: None,
                    content_hash: None,
                },
                academic_metadata: AcademicMetadata {
            authors: vec![],
            date_published: None,
            date_modified: None,
                    venue: None,
                    doi: None,
                    keywords: vec!["group theory".to_string(), "abstract algebra".to_string(), "symmetry".to_string()],
                },
                structure: DocumentStructure {
                    abstract_content: Some(Section {
                        id: "group_theory.theory_overview.abstract".to_string(),
                        title: None,
                        content: SectionContentNode::RichText(RichText {
                            segments: vec![
                                RichTextSegment::Text(format!("Welcome to the comprehensive Group Theory framework. This theory encompasses {} distinct group objects, {} expressions, {} relations, and {} fundamental theorems. Navigate through the organized sections below to explore the complete mathematical landscape of group theory, from basic algebraic structures to advanced constructions.", all_definitions.len(), all_expressions.len(), all_relations.len(), all_theorems.len())),
                            ],
                            alignment: None,
                        }),
                        metadata: vec![],
                        display_options: None,
                    }),
            table_of_contents: None,
                    body: vec![
                        Section {
                            id: "group_theory.navigation.definitions".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("📚 Group Definitions".to_string())],
                                alignment: None,
                            }),
                            content: 
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("Explore {} group definitions organized by mathematical category:", definition_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.fundamental".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("• Fundamental Groups".to_string())],
                                alignment: None,
                            }),
                            content: 
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("{} basic group structures that establish foundational algebraic properties.", fundamental_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.enriched".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("• Groups with Additional Structure".to_string())],
                                alignment: None,
                            }),
                            content: 
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("{} groups enhanced with topological or geometric structure.", enriched_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.concrete".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("• Concrete Group Constructions".to_string())],
                                alignment: None,
                            }),
                            content: 
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("{} groups arising from symmetries and permutations.", concrete_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.matrix".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("• Matrix Groups".to_string())],
                                alignment: None,
                            }),
                            content: 
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("{} groups defined through linear transformations and matrix properties.", matrix_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.modular".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("• Modular Arithmetic Groups".to_string())],
                                alignment: None,
                            }),
                            content: 
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("{} groups based on modular arithmetic and number theory.", modular_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.operations".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("• Group Operations & Constructions".to_string())],
                                alignment: None,
                            }),
                            content: 
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("{} constructions that build new groups from existing ones.", operations_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.subgroups".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("• Subgroup Constructions".to_string())],
                                alignment: None,
                            }),
                            content: 
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("{} subgroup constructions that reveal internal structure.", subgroup_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.advanced".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("• Advanced Constructions".to_string())],
                                alignment: None,
                            }),
                            content: 
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("{} sophisticated group constructions for advanced mathematical analysis.", advanced_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.theorems".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("🔬 Fundamental Theorems".to_string())],
                                alignment: None,
                            }),
                            content: 
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("Discover {} fundamental theorems that establish the theoretical foundation of group theory, providing essential insights into group structure and behavior.", theorem_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.mathematical_framework".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("🌐 Complete Mathematical Framework".to_string())],
                                alignment: None,
                            }),
                                content: SectionContentNode::SubSection(vec![
                                Section {
                                    id: "group_theory.framework-description".to_string(),
                                    title: None,
                                    content: SectionContentNode::RichText(RichText {
                                        segments: vec![
                                            RichTextSegment::Text("This comprehensive framework demonstrates the interconnected nature of group theory, where each mathematical object contributes to a unified understanding of algebraic structure and symmetry.".to_string()),
                                        ],
                                        alignment: None,
                                    }),
                                    metadata: vec![],
                                    display_options: None,
                                },
                                Section {
                                    id: "group_theory.navigation-instruction".to_string(),
                                    title: None,
                                    content: SectionContentNode::RichText(RichText {
                                        segments: vec![
                                            RichTextSegment::Text("Use the organized navigation above to explore specific areas of interest, or dive into the complete collection to experience the full scope of group-theoretic mathematics.".to_string()),
                                        ],
                                        alignment: None,
                                    }),
                                    metadata: vec![],
                                    display_options: None,
                                },
                            ]),
                            metadata: vec![],
                            display_options: None,
                        },
                    ],
            footnotes: vec![],
            glossary: vec![],
            bibliography: vec![],
                },
                relationships: DocumentRelationships {
                    parent_documents: vec![],
                    child_documents: all_content_links,
                    related_concepts: vec![],
                    cross_references: vec![],
                    dependency_graph: None,
                },
            }),
        }
    }

    fn export_definitions(&self) -> Vec<MathDocument> {
        let mut content = Vec::new();

        // **OBJECT DEFINITIONS ONLY** - Theory overview is exported separately
        content.extend(self.export_object_definitions(self.generate_object_definitions()));
        content.extend(self.export_expression_definitions(self.generate_expression_definitions()));
        content.extend(self.export_relation_definitions(self.generate_relation_definitions()));

        content
    }

    fn export_theorems(&self) -> Vec<MathDocument> {
        // Initializing the registry is now handled automatically by the OnceLock
        // in get_theorem_registry(). We can call it here to be explicit.

        let theorems: Vec<(String, Theorem)> = vec![
            (
                "group_inverse_uniqueness".to_string(),
                group_inverse_uniqueness(),
            ),
            (
                "subgroup_intersection_is_subgroup".to_string(),
                subgroup_intersection_is_subgroup(),
            ),
            (
                "first_isomorphism_theorem".to_string(),
                first_isomorphism_theorem(),
            ),
            (
                "element_order_divides_group_order".to_string(),
                element_order_divides_group_order(),
            ),
            (
                "lagrange_theorem".to_string(),
                lagrange_theorem(),
            ),
            (
                "cayley_theorem".to_string(),
                cayley_theorem(),
            ),
            (
                "sylow_first_theorem".to_string(),
                sylow_first_theorem(),
            ),
            (
                "fundamental_theorem_finite_abelian".to_string(),
                fundamental_theorem_finite_abelian(),
            ),
            (
                "normal_subgroup_test".to_string(),
                normal_subgroup_test(),
            ),
            (
                "cauchy_theorem".to_string(),
                cauchy_theorem(),
            ),
            (
                "center_is_normal_subgroup".to_string(),
                center_is_normal_subgroup(),
            ),
        ];

        theorems
            .into_iter()
            .map(|(id, theorem)| theorem.to_math_document(&format!("group_theory.thm.{}", id)))
            .collect()
    }

    fn export_object_definitions(&self, objects: Vec<Group>) -> Vec<MathDocument> {
        let mut content = Vec::new();

        // Export each variant instance to MathematicalContent
        // Use new clear ID pattern: theory.content_type.group_type
        for group in objects.iter() {
            let group_type = match group {
                Group::Generic(_) => "generic_group",
                Group::Trivial(_) => "trivial_group",
                Group::Symmetric(_) => "symmetric_group",
                Group::Alternating(_) => "alternating_group",
                Group::Cyclic(_) => "cyclic_group",
                Group::Dihedral(_) => "dihedral_group",
                Group::GeneralLinear(_) => "general_linear_group",
                Group::SpecialLinear(_) => "special_linear_group",
                Group::Orthogonal(_) => "orthogonal_group",
                Group::SpecialOrthogonal(_) => "special_orthogonal_group",
                Group::Unitary(_) => "unitary_group",
                Group::SpecialUnitary(_) => "special_unitary_group",
                Group::Topological(_) => "topological_group",
                Group::Lie(_) => "lie_group",
                Group::Product(_) => "product_group",
                Group::ModularAdditive(_) => "modular_additive_group",
                Group::ModularMultiplicative(_) => "modular_multiplicative_group",
                Group::Free(_) => "free_group",
                Group::Quotient(_) => "quotient_group",
                Group::Kernel(_) => "kernel_group",
                Group::Image(_) => "image_group",
                Group::Center(_) => "center_group",
                Group::GeneratedSubgroup(_) => "generated_subgroup",
                Group::Normalizer(_) => "normalizer_group",
                Group::Centralizer(_) => "centralizer_group",
                Group::CommutatorSubgroup(_) => "commutator_subgroup",
                Group::SylowSubgroup(_) => "sylow_subgroup",
                Group::CentralProduct(_) => "central_product_group",
                Group::WreathProduct(_) => "wreath_product_group",
                Group::Pullback(_) => "pullback_group",
                Group::Restriction(_) => "restriction_group",
                Group::Interception(_) => "interception_group",
                Group::SubGroup(_) => "subgroup",
            };

            // Use new clear ID pattern: group_theory.def.{group_type}
            let document_id = format!("group_theory.def.{}", group_type);
            let math_doc = group.to_math_document(&document_id);
            content.push(math_doc);
        }

        // **DEFINITIONS ONLY** - No overview document here since it's exported separately
        content
    }

    fn generate_object_definitions(&self) -> Vec<Group> {
        // **L1 ABSTRACT DEFINITIONS**: Create abstract mathematical schemas, not concrete instances
        vec![
            // ===== BASIC GROUP TYPES =====
            Group::Generic(GenericGroup::default()),
            // ===== GROUPS WITH ADDITIONAL STRUCTURE =====
            Group::Topological(TopologicalGroup {
                core: GenericGroup::default(),
                topology: TopologicalSpace {
                    base_set: Set::Parametric {
                        parameters: HashMap::new(),
                        description: "Abstract topological space".to_string(),
                        membership_condition: "x ∈ G".to_string(),
                        properties: VariantSet::new(),
                    },
                    topology: Topology {
                        properties: VariantSet::new(),
                    },
                    properties: vec![],
                },
                props: VariantSet::new(),
            }),
            Group::Lie(LieGroup {
                core: GenericGroup::default(),
                topology: TopologicalSpace {
                    base_set: Set::Parametric {
                        parameters: HashMap::new(),
                        description: "Abstract Lie group manifold".to_string(),
                        membership_condition: "x ∈ G".to_string(),
                        properties: VariantSet::new(),
                    },
                    topology: Topology {
                        properties: VariantSet::new(),
                    },
                    properties: vec![],
                },
                charts: vec![], // Abstract - no specific charts
                props: VariantSet::new(),
            }),
            // ===== CONCRETE GROUP CONSTRUCTIONS =====
            Group::Cyclic(CyclicGroup {
                core: GenericGroup::default(),
                generator: GroupElement::Symbol("g".to_string()), // Abstract generator
                order: None, // Abstract - could be finite or infinite
            }),
            Group::Symmetric(SymmetricGroup {
                core: GenericGroup::default(),
                degree: 0, // Abstract - represents S_n for any n
            }),
            Group::Dihedral(DihedralGroup {
                core: GenericGroup::default(),
                order: 0, // Abstract - represents D_n for any n
            }),
            // ===== MATRIX GROUPS =====
            Group::GeneralLinear(GeneralLinearGroup {
                core: GenericGroup::default(),
                dimension: 0, // Abstract - represents GL(n,F) for any n
                field: Field::Basic(FieldBasic::default()),
                matrix_props: VariantSet::new(),
                linear_props: VariantSet::new(),
            }),
            Group::SpecialLinear(SpecialLinearGroup {
                general_linear: GeneralLinearGroup {
                    core: GenericGroup::default(),
                    dimension: 0, // Abstract - represents SL(n,F) for any n
                    field: Field::Basic(FieldBasic::default()),
                    matrix_props: VariantSet::new(),
                    linear_props: VariantSet::new(),
                },
                special_linear_props: VariantSet::new(),
            }),
            Group::Orthogonal(OrthogonalGroup {
                core: GenericGroup::default(),
                dimension: 0, // Abstract - represents O(n) for any n
                matrix_props: VariantSet::new(),
            }),
            Group::SpecialOrthogonal(SpecialOrthogonalGroup {
                orthogonal: OrthogonalGroup {
                    core: GenericGroup::default(),
                    dimension: 0, // Abstract - represents SO(n) for any n
                    matrix_props: VariantSet::new(),
                },
                special_orthogonal_props: VariantSet::new(),
            }),
            Group::Unitary(UnitaryGroup {
                core: GenericGroup::default(),
                dimension: 0, // Abstract - represents U(n) for any n
                matrix_props: VariantSet::new(),
            }),
            Group::SpecialUnitary(SpecialUnitaryGroup {
                unitary: UnitaryGroup {
                    core: GenericGroup::default(),
                    dimension: 0, // Abstract - represents SU(n) for any n
                    matrix_props: VariantSet::new(),
                },
                special_unitary_props: VariantSet::new(),
            }),
            // ===== PERMUTATION GROUPS =====
            Group::Alternating(AlternatingGroup {
                core: GenericGroup::default(),
                degree: 0, // Abstract - represents A_n for any n
                perm_props: VariantSet::new(),
            }),
            // ===== MODULAR GROUPS =====
            Group::ModularAdditive(ModularAdditiveGroup {
                core: GenericGroup::default(),
                modulus: 0, // Abstract - represents ℤ/nℤ for any n
                modular_props: VariantSet::new(),
            }),
            Group::ModularMultiplicative(ModularMultiplicativeGroup {
                core: GenericGroup::default(),
                modulus: 0, // Abstract - represents (ℤ/nℤ)* for any n
                modular_props: VariantSet::new(),
            }),
            // ===== COMBINATORIAL GROUPS =====
            Group::Free(FreeGroup {
                core: GenericGroup::default(),
                rank: 0, // Abstract - represents F_n for any n
                free_props: VariantSet::new(),
            }),
            Group::Trivial(TrivialGroup {
                core: GenericGroup::default(),
            }),
            // ===== GROUP OPERATIONS =====
            Group::Product(ProductGroup {
                core: GenericGroup::default(),
                operation: ProductOperation::Direct,
                components: vec![], // Abstract - no specific components
                normal_component: None,
                product_props: VariantSet::new(),
            }),
            Group::Quotient(QuotientGroup {
                core: GenericGroup::default(),
                group: Located::new_variable(Identifier::new_simple("G".to_string())), // Abstract group
                normal_subgroup: Located::new_variable(Identifier::new_simple("H".to_string())), // Abstract normal subgroup
                quotient_props: VariantSet::new(),
            }),
            // ===== HOMOMORPHISM-BASED CONSTRUCTIONS =====
            Group::Kernel(KernelGroup {
                core: GenericGroup::default(),
                defining_homomorphism: Located::new_concrete(GroupHomomorphism {
                    domain: Located::new_variable(Identifier::new_simple("G".to_string())),
                    codomain: Located::new_variable(Identifier::new_simple("H".to_string())),
                }),
            }),
            Group::Image(ImageGroup {
                core: GenericGroup::default(),
                defining_homomorphism: Located::new_concrete(GroupHomomorphism {
                    domain: Located::new_variable(Identifier::new_simple("G".to_string())),
                    codomain: Located::new_variable(Identifier::new_simple("H".to_string())),
                }),
            }),
            // ===== SUBGROUP CONSTRUCTIONS =====
            Group::Center(CenterGroup {
                core: GenericGroup::default(),
                parent_group: Located::new_variable(Identifier::new_simple("G".to_string())), // Abstract parent
            }),
            Group::GeneratedSubgroup(GeneratedSubgroup {
                core: GenericGroup::default(),
                parent_group: Located::new_variable(Identifier::new_simple("G".to_string())), // Abstract parent
                generators: vec![GroupElement::Symbol("g".to_string())], // Abstract generators
            }),
            Group::Normalizer(NormalizerGroup {
                core: GenericGroup::default(),
                parent_group: Located::new_variable(Identifier::new_simple("G".to_string())), // Abstract parent
                subgroup_normalized: Located::new_variable(Identifier::new_simple("H".to_string())), // Abstract subgroup
            }),
            Group::Centralizer(CentralizerGroup {
                core: GenericGroup::default(),
                parent_group: Located::new_variable(Identifier::new_simple("G".to_string())), // Abstract parent
                element_centralized: GroupElement::Symbol("x".to_string()),      // Abstract element
            }),
            Group::CommutatorSubgroup(CommutatorSubgroup {
                core: GenericGroup::default(),
                parent_group: Located::new_variable(Identifier::new_simple("G".to_string())), // Abstract parent
            }),
            Group::SylowSubgroup(SylowSubgroup {
                core: GenericGroup::default(),
                parent_group: Located::new_variable(Identifier::new_simple("G".to_string())), // Abstract parent
                prime: 0, // Abstract - represents p-Sylow for any prime p
            }),
            // ===== ADVANCED CONSTRUCTIONS =====
            Group::WreathProduct(WreathProductGroup {
                core: GenericGroup::default(),
                base_group: Located::new_variable(Identifier::new_simple("G".to_string())), // Abstract base
                acting_group: Located::new_variable(Identifier::new_simple("H".to_string())), // Abstract acting group
            }),
            Group::CentralProduct(CentralProductGroup {
                core: GenericGroup::default(),
                component_groups: vec![], // Abstract - no specific components
                center_identification_map: "central_identification".to_string(),
            }),
            Group::Pullback(PullbackGroup {
                core: GenericGroup::default(),
                source_groups: vec![], // Abstract - no specific sources
                target_group: Located::new_variable(Identifier::new_simple("G".to_string())), // Abstract target
                defining_homomorphisms: vec![], // Abstract - no specific homomorphisms
            }),
            Group::Restriction(RestrictionGroup {
                core: GenericGroup::default(),
                parent_group: Located::new_variable(Identifier::new_simple("G".to_string())), // Abstract parent
                restriction_description: "subset_restriction".to_string(),
            }),
            Group::SubGroup(SubGroup {
                core: GenericGroup::default(),
                parent_group: Located::new_concrete(Group::Generic(GenericGroup::default())), // Abstract parent
                subgroup_props: VariantSet::new(),
            }),
        ]
    }

    fn generate_expression_definitions(&self) -> Vec<GroupExpression> {
        // For now, expressions are embedded in object definitions
        // We could later extract standalone expression documentation
        vec![]
    }

    fn generate_relation_definitions(&self) -> Vec<GroupRelation> {
        // Could generate example relations for each group type
        // For now, relations are handled within object definitions
        vec![]
    }

    fn export_expression_definitions(
        &self,
        expressions: Vec<GroupExpression>,
    ) -> Vec<MathDocument> {
        // For now, expressions are embedded in object definitions
        // We could later extract standalone expression documentation
        vec![]
    }

    fn export_relation_definitions(&self, relations: Vec<GroupRelation>) -> Vec<MathDocument> {
        vec![]
    }
}

// === IMPLEMENTATIONS FOR PARAMETRIZABLE ===
// Note: ToTurnMath implementation for Parametrizable<T> is already defined elsewhere
// to avoid conflicting implementations

/// Render a context variable with proper group theory type information
pub fn render_context_variable(
    name: &str,
    _ty: &crate::subjects::math::formalism::proof::ContextEntry,
) -> MathNode {
    // For now, use basic formatting. Complex type matching can be added later
    MathNode {
        id: format!("context-{}", name),
        content: Arc::new(MathNodeContent::Text(format!("{} : group element", name))),
    }
}

#[cfg(test)]
mod export_tests {
    use super::*;

    #[test]
    fn test_export_theorems() {
        let exporter = GroupTheoryExporter;
        let theorems = exporter.export_theorems();
        
        // Should export 11 theorems
        assert_eq!(theorems.len(), 11);
        
        println!("Successfully exported {} theorems:", theorems.len());
        for theorem in &theorems {
            println!("  - {}", theorem.id);
        }
        
        // Check that each theorem has the expected ID format
        for theorem in &theorems {
            assert!(theorem.id.starts_with("group_theory.thm."));
        }
        
        // Check that we have the expected theorem IDs
        let theorem_ids: Vec<&str> = theorems.iter().map(|t| t.id.as_str()).collect();
        assert!(theorem_ids.contains(&"group_theory.thm.group_inverse_uniqueness-doc"));
        assert!(theorem_ids.contains(&"group_theory.thm.subgroup_intersection_is_subgroup-doc"));
        assert!(theorem_ids.contains(&"group_theory.thm.first_isomorphism_theorem-doc"));
        assert!(theorem_ids.contains(&"group_theory.thm.element_order_divides_group_order-doc"));
        assert!(theorem_ids.contains(&"group_theory.thm.lagrange_theorem-doc"));
        assert!(theorem_ids.contains(&"group_theory.thm.cayley_theorem-doc"));
        assert!(theorem_ids.contains(&"group_theory.thm.sylow_first_theorem-doc"));
        assert!(theorem_ids.contains(&"group_theory.thm.fundamental_theorem_finite_abelian-doc"));
        assert!(theorem_ids.contains(&"group_theory.thm.normal_subgroup_test-doc"));
        assert!(theorem_ids.contains(&"group_theory.thm.cauchy_theorem-doc"));
        assert!(theorem_ids.contains(&"group_theory.thm.center_is_normal_subgroup-doc"));
    }
}
