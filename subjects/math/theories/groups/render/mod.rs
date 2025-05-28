use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

//--- Imports from crate::turn_render ---
use crate::turn_render::math_node::{
    BracketSize, BracketStyle, IntegralType, MathNode, MathNodeContent, MulSymbol,
    RefinedMulOrDivOperation, RelationOperatorNode, ToTurnMath, UnaryRelationOperatorNode,
};
use crate::turn_render::section_node::{
    AbstractionMetadata, AcademicMetadata, ContentMetadata, DocumentRelationships,
    DocumentStructure, LinkTarget, MathDocument, MathematicalContent, MathematicalContentType,
    PaperType, ParagraphNode, RichTextSegment, ScientificPaperContent, Section, SectionContentNode,
    SelectableProperty, StructuredMathContentNode, TheoremLikeKind, ToSectionNode,
};

//--- Imports from this crate (subjects) ---
use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::theories::VariantSet;
use crate::subjects::math::theories::fields::definitions::Field;
use crate::subjects::math::theories::fields::render::*;
use crate::subjects::math::theories::topology::definitions::{TopologicalSpace, Topology};
use crate::subjects::math::theories::zfc::set::Set;

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
            Group::GeneralLinear(g) => g.core.to_turn_math(master_id),
            Group::SpecialLinear(g) => g.general_linear.core.to_turn_math(master_id),
            Group::Orthogonal(g) => g.core.to_turn_math(master_id),
            Group::SpecialOrthogonal(g) => g.orthogonal.core.to_turn_math(master_id),
            Group::Unitary(g) => g.core.to_turn_math(master_id),
            Group::SpecialUnitary(g) => g.unitary.core.to_turn_math(master_id),
            Group::Topological(g) => g.to_turn_math(master_id),
            Group::Lie(g) => g.to_turn_math(master_id),
            Group::Product(g) => g.to_turn_math(master_id),
            Group::ModularAdditive(g) => g.core.to_turn_math(master_id),
            Group::ModularMultiplicative(g) => g.core.to_turn_math(master_id),
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
            Group::GeneralLinear(g) => g.core.to_section_node(id_prefix),
            Group::SpecialLinear(g) => g.general_linear.core.to_section_node(id_prefix),
            Group::Orthogonal(g) => g.core.to_section_node(id_prefix),
            Group::SpecialOrthogonal(g) => g.orthogonal.core.to_section_node(id_prefix),
            Group::Unitary(g) => g.core.to_section_node(id_prefix),
            Group::SpecialUnitary(g) => g.unitary.core.to_section_node(id_prefix),
            Group::Topological(g) => g.to_section_node(id_prefix),
            Group::Lie(g) => g.to_section_node(id_prefix),
            Group::Product(g) => g.to_section_node(id_prefix),
            Group::ModularAdditive(g) => g.core.to_section_node(id_prefix),
            Group::ModularMultiplicative(g) => g.core.to_section_node(id_prefix),
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

    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        match self {
            Group::Generic(g) => g.to_math_document(id_prefix),
            Group::Trivial(g) => g.core.to_math_document(id_prefix),
            Group::Symmetric(g) => g.to_math_document(id_prefix),
            Group::Alternating(g) => g.to_math_document(id_prefix),
            Group::Cyclic(g) => g.to_math_document(id_prefix),
            Group::Dihedral(g) => g.to_math_document(id_prefix),
            Group::GeneralLinear(g) => g.core.to_math_document(id_prefix),
            Group::SpecialLinear(g) => g.general_linear.core.to_math_document(id_prefix),
            Group::Orthogonal(g) => g.core.to_math_document(id_prefix),
            Group::SpecialOrthogonal(g) => g.orthogonal.core.to_math_document(id_prefix),
            Group::Unitary(g) => g.core.to_math_document(id_prefix),
            Group::SpecialUnitary(g) => g.unitary.core.to_math_document(id_prefix),
            Group::Topological(g) => g.to_math_document(id_prefix),
            Group::Lie(g) => g.to_math_document(id_prefix),
            Group::Product(g) => g.to_math_document(id_prefix),
            Group::ModularAdditive(g) => g.core.to_math_document(id_prefix),
            Group::ModularMultiplicative(g) => g.core.to_math_document(id_prefix),
            Group::Free(g) => g.core.to_math_document(id_prefix),
            Group::Quotient(g) => g.core.to_math_document(id_prefix),
            Group::Kernel(g) => g.core.to_math_document(id_prefix),
            Group::Image(g) => g.core.to_math_document(id_prefix),
            Group::Center(g) => g.core.to_math_document(id_prefix),
            Group::GeneratedSubgroup(g) => g.core.to_math_document(id_prefix),
            Group::Normalizer(g) => g.core.to_math_document(id_prefix),
            Group::Centralizer(g) => g.core.to_math_document(id_prefix),
            Group::CommutatorSubgroup(g) => g.core.to_math_document(id_prefix),
            Group::SylowSubgroup(g) => g.core.to_math_document(id_prefix),
            Group::CentralProduct(g) => g.core.to_math_document(id_prefix),
            Group::WreathProduct(g) => g.core.to_math_document(id_prefix),
            Group::Pullback(g) => g.core.to_math_document(id_prefix),
            Group::Restriction(g) => g.core.to_math_document(id_prefix),
        }
    }

    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        match self {
            Group::Generic(_) => vec![RichTextSegment::Text("Generic Group".to_string())],
            Group::Cyclic(_) => vec![RichTextSegment::Text("Cyclic Group".to_string())],
            Group::Topological(_) => vec![RichTextSegment::Text("Topological Group".to_string())],
            Group::Symmetric(_) => vec![RichTextSegment::Text("Symmetric Group".to_string())],
            Group::Alternating(_) => vec![RichTextSegment::Text("Alternating Group".to_string())],
            _ => vec![RichTextSegment::Text("Group".to_string())],
        }
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        match self {
            Group::Generic(_) => vec![RichTextSegment::Text("G".to_string())],
            Group::Cyclic(_) => vec![RichTextSegment::Text("⟨g⟩".to_string())],
            Group::Topological(_) => vec![RichTextSegment::Text("(G, τ)".to_string())],
            Group::Symmetric(_) => vec![RichTextSegment::Text("Sₙ".to_string())],
            Group::Alternating(_) => vec![RichTextSegment::Text("Aₙ".to_string())],
            _ => vec![RichTextSegment::Text("G".to_string())],
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
                content: Box::new(MathNodeContent::Text(n.to_string())),
            },
            GroupElement::Symbol(s) => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text(s.clone())),
            },
            GroupElement::Permutation(_) => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("σ".to_string())),
            },
            GroupElement::Matrix(_) => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("M".to_string())),
            },
        }
    }
}

// === GROUPEXPRESSION IMPLEMENTATIONS ===

impl ToTurnMath for GroupExpression {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            GroupExpression::Element { group, element } => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("g".to_string())),
            },
            GroupExpression::Identity(group) => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("e".to_string())),
            },
            GroupExpression::Operation { group, left, right } => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Relationship {
                    lhs: Box::new(MathNode {
                        id: format!("{}-left", master_id),
                        content: Box::new(MathNodeContent::Text("a".to_string())),
                    }),
                    operator: RelationOperatorNode::Equal,
                    rhs: Box::new(MathNode {
                        id: format!("{}-right", master_id),
                        content: Box::new(MathNodeContent::Text("b".to_string())),
                    }),
                }),
            },
            GroupExpression::Inverse { group, element } => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::UnaryPrefix {
                    parameter: Box::new(MathNode {
                        id: format!("{}-operand", master_id),
                        content: Box::new(MathNodeContent::Text("g".to_string())),
                    }),
                    operator: "^{-1}".to_string(),
                }),
            },
            _ => MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("expr".to_string())),
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
