use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use crate::subjects::math::export::unified_exporter::TheoryExporter;
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::formalism::location::Located;
use crate::subjects::math::theories::groups::definitions::{
    CenterGroup, CommutatorSubgroup, GroupHomomorphism, SylowSubgroup,
};
//--- Imports from crate::turn_render ---
use crate::turn_render::math_node::{
    Identifier, IntegralType, MathNode, MathNodeContent, MulSymbol, RefinedMulOrDivOperation,
    RelationOperatorNode, ToTurnMath, UnaryRelationOperatorNode,
};
use crate::turn_render::*;

//--- Imports from this crate (subjects) ---
use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::theories::VariantSet;
use crate::subjects::math::theories::fields::definitions::Field;
use crate::subjects::math::theories::fields::{FieldBasic, render::*};
use crate::subjects::math::theories::topology::definitions::{TopologicalSpace, Topology};
use crate::subjects::math::theories::zfc::definitions::Set;

//--- Imports from groups definitions ---
use crate::subjects::math::theories::groups::definitions::{
    AlternatingGroup, CentralProductGroup, CentralizerGroup, CyclicGroup, DihedralGroup, FreeGroup,
    GeneralLinearGroup, GeneratedSubgroup, GenericGroup, Group, GroupElement, GroupExpression,
    GroupOperation, GroupProperty, GroupRelation, ImageGroup, KernelGroup, LieGroup,
    ModularAdditiveGroup, ModularMultiplicativeGroup, NormalizerGroup, OrthogonalGroup,
    ProductGroup, ProductOperation, PullbackGroup, QuotientGroup, RestrictionGroup,
    SpecialLinearGroup, SpecialOrthogonalGroup, SpecialUnitaryGroup, SymmetricGroup,
    TopologicalGroup, TopologicalGroupProperty, TrivialGroup, UnitaryGroup, WreathProductGroup,
};

use super::theorems::prove_inverse_uniqueness;

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
            Group::Trivial(g) => g.core.to_turn_math(master_id),
            Group::Symmetric(g) => g.to_turn_math(master_id),
            Group::Alternating(g) => g.to_turn_math(master_id),
            Group::Cyclic(g) => g.to_turn_math(master_id),
            Group::Dihedral(g) => g.to_turn_math(master_id),
            Group::GeneralLinear(g) => g.to_turn_math(master_id),
            Group::SpecialLinear(g) => g.to_turn_math(master_id),
            Group::Orthogonal(g) => g.to_turn_math(master_id),
            Group::SpecialOrthogonal(g) => g.to_turn_math(master_id),
            Group::Unitary(g) => g.to_turn_math(master_id),
            Group::SpecialUnitary(g) => g.to_turn_math(master_id),
            Group::Topological(g) => g.to_turn_math(master_id),
            Group::Lie(g) => g.to_turn_math(master_id),
            Group::Product(g) => g.to_turn_math(master_id),
            Group::ModularAdditive(g) => g.to_turn_math(master_id),
            Group::ModularMultiplicative(g) => g.to_turn_math(master_id),
            Group::Free(g) => g.core.to_turn_math(master_id),
            Group::Quotient(g) => g.core.to_turn_math(master_id),
            Group::Kernel(g) => g.core.to_turn_math(master_id),
            Group::Image(g) => g.core.to_turn_math(master_id),
            Group::Center(g) => g.core.to_turn_math(master_id),
            Group::GeneratedSubgroup(g) => g.core.to_turn_math(master_id),
            Group::Normalizer(g) => g.core.to_turn_math(master_id),
            Group::Centralizer(g) => g.core.to_turn_math(master_id),
            Group::CommutatorSubgroup(g) => g.core.to_turn_math(master_id),
            Group::SylowSubgroup(g) => g.core.to_turn_math(master_id),
            Group::CentralProduct(g) => g.core.to_turn_math(master_id),
            Group::WreathProduct(g) => g.core.to_turn_math(master_id),
            Group::Pullback(g) => g.core.to_turn_math(master_id),
            Group::Restriction(g) => g.core.to_turn_math(master_id),
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
                content: Box::new(MathNodeContent::Quantity {
                    number: n.to_string(),
                    scientific_notation: None,
                    unit: None,
                }),
            },
            GroupElement::Symbol(s) => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Identifier(Identifier {
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
                content: Box::new(MathNodeContent::Identifier(Identifier {
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
                content: Box::new(MathNodeContent::Identifier(Identifier {
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
            GroupExpression::Operation { left, right, .. } => {
                todo!()
            }
            GroupExpression::Element { element, .. } => match element {
                Some(param_element) => param_element.to_turn_math(master_id),
                None => MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Identifier(Identifier {
                        body: "?".to_string(),
                        pre_script: None,
                        mid_script: None,
                        post_script: None,
                        primes: 0,
                        is_function: false,
                    })),
                },
            },
            GroupExpression::Identity(_) => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Identifier(Identifier {
                    body: "e".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                })),
            },
            GroupExpression::Inverse { element, .. } => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Power {
                    base: Box::new(element.to_turn_math(format!("{}-base", master_id))),
                    exponent: Box::new(MathNode {
                        id: format!("{}-exp", master_id),
                        content: Box::new(MathNodeContent::Quantity {
                            number: "-1".to_string(),
                            scientific_notation: None,
                            unit: None,
                        }),
                    }),
                }),
            },
            _ => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("⟨expr⟩".to_string())),
            },
        }
    }
}

// === GROUPRELATION IMPLEMENTATIONS ===

impl ToTurnMath for GroupRelation {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            GroupRelation::IsSubgroupOf { subgroup, group } => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Relationship {
                    lhs: Box::new(MathNode {
                        id: format!("{}-sub", master_id),
                        content: Box::new(MathNodeContent::Text("H".to_string())),
                    }),
                    operator: RelationOperatorNode::SubsetOf,
                    rhs: Box::new(MathNode {
                        id: format!("{}-group", master_id),
                        content: Box::new(MathNodeContent::Text("G".to_string())),
                    }),
                }),
            },
            GroupRelation::IsIsomorphicTo { first, second } => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Relationship {
                    lhs: Box::new(MathNode {
                        id: format!("{}-first", master_id),
                        content: Box::new(MathNodeContent::Text("G".to_string())),
                    }),
                    operator: RelationOperatorNode::Equal,
                    rhs: Box::new(MathNode {
                        id: format!("{}-second", master_id),
                        content: Box::new(MathNodeContent::Text("H".to_string())),
                    }),
                }),
            },
            GroupRelation::HasOrder { group, order } => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Relationship {
                    lhs: Box::new(MathNode {
                        id: format!("{}-order", master_id),
                        content: Box::new(MathNodeContent::Text("|G|".to_string())),
                    }),
                    operator: RelationOperatorNode::Equal,
                    rhs: Box::new(MathNode {
                        id: format!("{}-value", master_id),
                        content: Box::new(MathNodeContent::Text("n".to_string())),
                    }),
                }),
            },
            _ => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("relation".to_string())),
            },
        }
    }
}

impl GroupRelation {
    pub fn to_structured_group_relation(&self) -> String {
        match self {
            GroupRelation::IsSubgroupOf { subgroup, group } => {
                "H ⊆ G (subgroup relation)".to_string()
            }
            GroupRelation::IsIsomorphicTo { first, second } => "G ≅ H (isomorphism)".to_string(),
            GroupRelation::HasOrder { group, order } => "|G| = n (order relation)".to_string(),
            _ => format!("{:?}", self),
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
                        content: vec![SectionContentNode::RichText(RichText {
                            segments: vec![
                                RichTextSegment::Text(format!("Welcome to the comprehensive Group Theory framework. This theory encompasses {} distinct group objects, {} expressions, {} relations, and {} fundamental theorems. Navigate through the organized sections below to explore the complete mathematical landscape of group theory, from basic algebraic structures to advanced constructions.", all_definitions.len(), all_expressions.len(), all_relations.len(), all_theorems.len())),
                            ],
                            alignment: None,
                        })],
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
                            content: vec![
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("Explore {} group definitions organized by mathematical category:", definition_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            ],
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.fundamental".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("• Fundamental Groups".to_string())],
                                alignment: None,
                            }),
                            content: vec![
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("{} basic group structures that establish foundational algebraic properties.", fundamental_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            ],
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.enriched".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("• Groups with Additional Structure".to_string())],
                                alignment: None,
                            }),
                            content: vec![
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("{} groups enhanced with topological or geometric structure.", enriched_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            ],
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.concrete".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("• Concrete Group Constructions".to_string())],
                                alignment: None,
                            }),
                            content: vec![
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("{} groups arising from symmetries and permutations.", concrete_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            ],
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.matrix".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("• Matrix Groups".to_string())],
                                alignment: None,
                            }),
                            content: vec![
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("{} groups defined through linear transformations and matrix properties.", matrix_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            ],
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.modular".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("• Modular Arithmetic Groups".to_string())],
                                alignment: None,
                            }),
                            content: vec![
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("{} groups based on modular arithmetic and number theory.", modular_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            ],
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.operations".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("• Group Operations & Constructions".to_string())],
                                alignment: None,
                            }),
                            content: vec![
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("{} constructions that build new groups from existing ones.", operations_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            ],
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.subgroups".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("• Subgroup Constructions".to_string())],
                                alignment: None,
                            }),
                            content: vec![
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("{} subgroup constructions that reveal internal structure.", subgroup_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            ],
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.advanced".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("• Advanced Constructions".to_string())],
                                alignment: None,
                            }),
                            content: vec![
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("{} sophisticated group constructions for advanced mathematical analysis.", advanced_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            ],
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.theorems".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("🔬 Fundamental Theorems".to_string())],
                                alignment: None,
                            }),
                            content: vec![
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text(format!("Discover {} fundamental theorems that establish the theoretical foundation of group theory, providing essential insights into group structure and behavior.", theorem_links.len())),
                                    ],
                                    alignment: None,
                                }),
                            ],
                            metadata: vec![],
                            display_options: None,
                        },
                        Section {
                            id: "group_theory.navigation.mathematical_framework".to_string(),
                            title: Some(RichText {
                                segments: vec![RichTextSegment::Text("🌐 Complete Mathematical Framework".to_string())],
                                alignment: None,
                            }),
                            content: vec![
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text("This comprehensive framework demonstrates the interconnected nature of group theory, where each mathematical object contributes to a unified understanding of algebraic structure and symmetry.".to_string()),
                                    ],
                                    alignment: None,
                                }),
                                SectionContentNode::RichText(RichText {
                                    segments: vec![
                                        RichTextSegment::Text("Use the organized navigation above to explore specific areas of interest, or dive into the complete collection to experience the full scope of group-theoretic mathematics.".to_string()),
                                    ],
                                    alignment: None,
                                }),
                            ],
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

        let theorems = vec![(
            "inverse_uniqueness".to_string(),
            super::theorems::prove_inverse_uniqueness(),
        )];

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
                group: Box::new(Group::Generic(GenericGroup::default())), // Abstract group
                normal_subgroup: Box::new(Group::Generic(GenericGroup::default())), // Abstract normal subgroup
                quotient_props: VariantSet::new(),
            }),
            // ===== HOMOMORPHISM-BASED CONSTRUCTIONS =====
            Group::Kernel(KernelGroup {
                core: GenericGroup::default(),
                defining_homomorphism: Box::new(GroupHomomorphism {
                    domain: Parametrizable::Variable(Identifier::new_simple("G".to_string())),
                    codomain: Parametrizable::Variable(Identifier::new_simple("H".to_string())),
                }),
            }),
            Group::Image(ImageGroup {
                core: GenericGroup::default(),
                defining_homomorphism: Box::new(GroupHomomorphism {
                    domain: Parametrizable::Variable(Identifier::new_simple("G".to_string())),
                    codomain: Parametrizable::Variable(Identifier::new_simple("H".to_string())),
                }),
            }),
            // ===== SUBGROUP CONSTRUCTIONS =====
            Group::Center(CenterGroup {
                core: GenericGroup::default(),
                parent_group: Box::new(Group::Generic(GenericGroup::default())), // Abstract parent
            }),
            Group::GeneratedSubgroup(GeneratedSubgroup {
                core: GenericGroup::default(),
                parent_group: Box::new(Group::Generic(GenericGroup::default())), // Abstract parent
                generators: vec![GroupElement::Symbol("g".to_string())], // Abstract generators
            }),
            Group::Normalizer(NormalizerGroup {
                core: GenericGroup::default(),
                parent_group: Box::new(Group::Generic(GenericGroup::default())), // Abstract parent
                subgroup_normalized: Box::new(Group::Generic(GenericGroup::default())), // Abstract subgroup
            }),
            Group::Centralizer(CentralizerGroup {
                core: GenericGroup::default(),
                parent_group: Box::new(Group::Generic(GenericGroup::default())), // Abstract parent
                element_centralized: GroupElement::Symbol("x".to_string()),      // Abstract element
            }),
            Group::CommutatorSubgroup(CommutatorSubgroup {
                core: GenericGroup::default(),
                parent_group: Box::new(Group::Generic(GenericGroup::default())), // Abstract parent
            }),
            Group::SylowSubgroup(SylowSubgroup {
                core: GenericGroup::default(),
                parent_group: Box::new(Group::Generic(GenericGroup::default())), // Abstract parent
                prime: 0, // Abstract - represents p-Sylow for any prime p
            }),
            // ===== ADVANCED CONSTRUCTIONS =====
            Group::WreathProduct(WreathProductGroup {
                core: GenericGroup::default(),
                base_group: Box::new(Group::Generic(GenericGroup::default())), // Abstract base
                acting_group: Box::new(Group::Generic(GenericGroup::default())), // Abstract acting group
            }),
            Group::CentralProduct(CentralProductGroup {
                core: GenericGroup::default(),
                component_groups: vec![], // Abstract - no specific components
                center_identification_map: "central_identification".to_string(),
            }),
            Group::Pullback(PullbackGroup {
                core: GenericGroup::default(),
                source_groups: vec![], // Abstract - no specific sources
                target_group: Box::new(Group::Generic(GenericGroup::default())), // Abstract target
                defining_homomorphisms: vec![], // Abstract - no specific homomorphisms
            }),
            Group::Restriction(RestrictionGroup {
                core: GenericGroup::default(),
                parent_group: Box::new(Group::Generic(GenericGroup::default())), // Abstract parent
                restriction_description: "subset_restriction".to_string(),
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

impl<T> ToTurnMath for Parametrizable<T>
where
    T: ToTurnMath + Clone,
{
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            Parametrizable::Concrete(c) => c.to_turn_math(master_id),
            Parametrizable::Variable(id) => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Identifier(id.clone())),
            },
        }
    }
}
