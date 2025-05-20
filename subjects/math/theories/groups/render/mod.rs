use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

//--- Imports from crate::turn_render ---
use crate::turn_render::math_node::{
    BracketSize, BracketStyle, IntegralType, MathNode, MathNodeContent, MulSymbol,
    RefinedMulOrDivOperation, RelationOperatorNode, ToTurnMath, UnaryRelationOperatorNode,
};
use crate::turn_render::section_node::{
    AbstractionMetadata, LinkTarget, MathDocument, ParagraphNode, RichTextSegment, Section,
    SectionContentNode, SelectableProperty, StructuredMathContentNode, TheoremLikeKind,
    ToSectionNode, link_to_definition, p_text,
};

//--- Imports from this crate (subjects) ---
use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::theories::zfc::set::Set as ZFCSet;

// Group definitions from the current theory
use super::definitions::{
    AbelianPropertyVariant, AlternatingGroup, CenterGroup, CentralProductGroup, CentralizerGroup,
    CommutatorSubgroup, CyclicGroup, DihedralGroup, FreeGroup, GeneralLinearGroup,
    GeneratedSubgroup, Group, GroupAction, GroupElement, GroupExpression, GroupIdentity,
    GroupInverse, GroupInverseApplication, GroupNotation, GroupOperation, GroupOperationProperty,
    GroupOperationVariant, GroupProperty, GroupRelation, GroupSymbol, ImageGroup, KernelGroup,
    LieGroup, ModularAdditiveGroup, ModularMultiplicativeGroup, NilpotentPropertyVariant,
    NormalizerGroup, OrthogonalGroup, ProductGroup, PullbackGroup, QuotientGroup, RestrictionGroup,
    SimplePropertyVariant, SolvablePropertyVariant, SpecialLinearGroup, SpecialOrthogonalGroup,
    SpecialUnitaryGroup, SylowSubgroup, SymmetricGroup, TopologicalGroup, TrivialGroup,
    UnitaryGroup, WreathProductGroup,
};

// Import the new dedicated files for specific group variants
mod cyclic_group;
mod group_basic;
mod symmetric_group;
mod tests;
mod topological_group;

// Re-export the traits from the sub-modules
pub use cyclic_group::*;
pub use group_basic::*;
pub use symmetric_group::*;
pub use topological_group::*;

// Add these imports at the top if needed
// use crate::subjects::math::theories::groups::render::group_basic::{output_as_l1_section, output_as_l1_document};

// Add the necessary import for GroupBasic
use super::definitions::GroupBasic;

impl ToTurnMath for Group {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let name_str = match self {
            Group::Basic(gb) => format!("Group on {:?}", gb.base_set),
            Group::Cyclic(cg) => format!("Cyclic Group <{:?}>", cg.generator),
            Group::Symmetric(sg) => format!("S_{}", sg.degree),
            Group::Dihedral(dg) => format!("D_{}", dg.order / 2),
            Group::Alternating(ag) => format!("A_{}", ag.degree),
            Group::Product(pg) => format!("Product Group ({} comps)", pg.components.len()),
            Group::GeneralLinear(glg) => format!("GL({}, {:?})", glg.dimension, glg.field),
            _ => {
                format!("Group: {:?}", self)
                    .chars()
                    .take(30)
                    .collect::<String>()
                    + "..."
            }
        };
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::Text(name_str)),
        }
    }
}

impl ToTurnMath for GroupProperty {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::Text(format!("{:?}", self))),
        }
    }
}

impl ToTurnMath for GroupElement {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let content = match self {
            GroupElement::Integer(n) => MathNodeContent::Text(n.to_string()),
            GroupElement::Permutation(p) => MathNodeContent::Text(format!("{:?}", p)),
            GroupElement::Matrix(m) => MathNodeContent::Text(format!("{:?}", m)),
            GroupElement::Symbol(s) => MathNodeContent::Text(s.clone()),
        };
        MathNode {
            id: master_id,
            content: Box::new(content),
        }
    }
}

impl ToTurnMath for GroupOperation {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::Text(format!(
                "Op:{:?}",
                self.operation_type
            ))),
        }
    }
}

impl ToTurnMath for GroupExpression {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let content = match self {
            GroupExpression::Element { group: _, element } => {
                return element.to_turn_math(master_id);
            }
            GroupExpression::Identity(_group) => MathNodeContent::Text("e".to_string()),
            GroupExpression::Operation {
                group: _,
                left,
                right,
            } => {
                let left_node = left.as_ref().to_turn_math(format!("{}-leftOp", master_id));
                let right_node = right
                    .as_ref()
                    .to_turn_math(format!("{}-rightOp", master_id));
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
            GroupExpression::Inverse { group: _, element } => {
                let elem_node = element
                    .as_ref()
                    .to_turn_math(format!("{}-invBase", master_id));
                MathNodeContent::Power {
                    base: Box::new(elem_node),
                    exponent: Box::new(MathNode {
                        id: format!("{}-invExp", master_id),
                        content: Box::new(MathNodeContent::Text("-1".to_string())),
                    }),
                }
            }
            GroupExpression::Commutator { group: _, a, b } => {
                let a_node = a.as_ref().to_turn_math(format!("{}-commA", master_id));
                let b_node = b.as_ref().to_turn_math(format!("{}-commB", master_id));
                MathNodeContent::Bracketed {
                    inner: Box::new(MathNode {
                        id: format!("{}-commInner", master_id),
                        content: Box::new(MathNodeContent::Multiplications {
                            terms: vec![
                                (RefinedMulOrDivOperation::None, a_node),
                                (
                                    RefinedMulOrDivOperation::Multiplication(MulSymbol::Dot),
                                    b_node,
                                ),
                            ],
                        }),
                    }),
                    style: BracketStyle::Square,
                    size: BracketSize::Auto,
                }
            }
            _ => MathNodeContent::Text(
                format!("Expr{:?}", self)
                    .chars()
                    .take(20)
                    .collect::<String>()
                    + "...",
            ),
        };
        MathNode {
            id: master_id,
            content: Box::new(content),
        }
    }
}

impl ToTurnMath for GroupRelation {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let content = match self {
            GroupRelation::IsSubgroupOf { subgroup, group } => MathNodeContent::Relationship {
                lhs: Box::new(subgroup.to_turn_math(format!("{}-lhsSub", master_id))),
                rhs: Box::new(group.to_turn_math(format!("{}-rhsGrp", master_id))),
                operator: RelationOperatorNode::IsSubgroupOf,
            },
            GroupRelation::IsNormalSubgroupOf { subgroup, group } => {
                MathNodeContent::Relationship {
                    lhs: Box::new(subgroup.to_turn_math(format!("{}-lhsNormSub", master_id))),
                    rhs: Box::new(group.to_turn_math(format!("{}-rhsGrpNorm", master_id))),
                    operator: RelationOperatorNode::IsNormalSubgroupOf,
                }
            }
            GroupRelation::IsIsomorphicTo { first, second } => MathNodeContent::Relationship {
                lhs: Box::new(first.to_turn_math(format!("{}-isoLhs", master_id))),
                rhs: Box::new(second.to_turn_math(format!("{}-isoRhs", master_id))),
                operator: RelationOperatorNode::IsIsomorphicTo,
            },
            _ => MathNodeContent::Text(
                format!("Rel: {:?}", self)
                    .chars()
                    .take(20)
                    .collect::<String>()
                    + "...",
            ),
        };
        MathNode {
            id: master_id,
            content: Box::new(content),
        }
    }
}

macro_rules! stub_toturnmath_group_variant {
    ($($t:ty),*) => {
        $(impl ToTurnMath for $t {
            fn to_turn_math(&self, master_id: String) -> MathNode {
                MathNode {
                    id: master_id,
                    content: Box::new(MathNodeContent::Text(format!("{:?} (Group Variant)", self).chars().take(30).collect::<String>() + "..."))
                }
            }
        })*
    };
}

stub_toturnmath_group_variant! {
    LieGroup, DihedralGroup, ProductGroup, QuotientGroup,
    KernelGroup, ImageGroup, CenterGroup, GeneratedSubgroup, GeneralLinearGroup, SpecialLinearGroup,
    OrthogonalGroup, SpecialOrthogonalGroup, UnitaryGroup, SpecialUnitaryGroup, AlternatingGroup,
    ModularAdditiveGroup, ModularMultiplicativeGroup, FreeGroup, TrivialGroup, WreathProductGroup,
    CentralProductGroup, PullbackGroup, RestrictionGroup, NormalizerGroup, CentralizerGroup,
    CommutatorSubgroup, SylowSubgroup
}

impl ToSectionNode for Group {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();

        // For Level1, we should now be using render_as_l1_schema instead
        if formalism_obj_level == AbstractionLevel::Level1 {
            // Return a warning section to indicate that render_as_l1_schema should be used
            let warning_message = "WARNING: For Level 1 schema, please use render_as_l1_schema() instead of to_section_node().";
            return Section {
                id: format!("{}-warning-section", id_prefix),
                title: Some(p_text("Warning")),
                content: vec![SectionContentNode::Paragraph(p_text(warning_message))],
                sub_sections: vec![],
                metadata: Some(vec![(
                    "warning".to_string(),
                    "For L1 schema, use render_as_l1_schema() instead".to_string(),
                )]),
                display_options: None,
            };
        }

        // For other levels, continue with the existing logic
        let (title_text, content_nodes, selectable_props) = match self {
            Group::Basic(g) => {
                return g.to_section_node(id_prefix);
            }
            Group::Topological(g) => {
                return g.to_section_node(id_prefix);
            }
            Group::Lie(g) => {
                let _obj_lvl = g.level();
                let title = format!("Lie Group");
                let mut c_nodes = vec![SectionContentNode::Paragraph(p_text(&format!(
                    "Core Group: {:?}",
                    g.core.base_set
                )))];
                c_nodes.push(SectionContentNode::Paragraph(p_text(&format!(
                    "Manifold Topology: {:?}",
                    g.topology.base_set
                ))));
                if !g.charts.is_empty() {
                    c_nodes.push(SectionContentNode::Paragraph(p_text(&format!(
                        "Charts: {}",
                        g.charts.join(", ")
                    ))));
                }
                (title, c_nodes, vec![])
            }
            Group::Cyclic(g) => {
                return g.to_section_node(id_prefix);
            }
            Group::Symmetric(g) => {
                return g.to_section_node(id_prefix);
            }
            _ => {
                let title = self
                    .to_turn_math(format!("{}-title", id_prefix))
                    .content_as_text()
                    .unwrap_or_else(|| format!("Group Variant: {}", id_prefix));
                (
                    title,
                    vec![SectionContentNode::Paragraph(p_text(&format!(
                        "Details for Group variant {:?} (display as {:?}) not fully implemented.",
                        self, formalism_obj_level
                    )))],
                    vec![],
                )
            }
        };

        Section {
            id: format!("{}-group-section", id_prefix),
            title: Some(p_text(&title_text)),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathContentNode::Definition {
                    term_display: vec![RichTextSegment::Text(title_text.clone())],
                    formal_term: Some(self.to_turn_math(format!("{}-formalTerm", id_prefix))),
                    label: Some(format!("Definition ({})", title_text)),
                    body: content_nodes,
                    abstraction_meta: Some(AbstractionMetadata {
                        level: Some(formalism_obj_level as u8),
                        source_template_id: None,
                        specified_parameters: None,
                        universally_quantified_properties: None,
                    }),
                    selectable_properties: if selectable_props.is_empty() {
                        None
                    } else {
                        Some(selectable_props)
                    },
                },
            )],
            sub_sections: vec![],
            metadata: Some(vec![("type".to_string(), "GroupDefinition".to_string())]),
            display_options: None,
        }
    }

    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let inherent_formalism_level = self.level();

        // For Level1, we should now be using render_as_l1_schema_document instead
        if inherent_formalism_level == AbstractionLevel::Level1 {
            // Return a warning document to indicate that render_as_l1_schema_document should be used
            let warning_message = "WARNING: For Level 1 schema document, please use render_as_l1_schema_document() instead of to_math_document().";
            let warning_section = Section {
                id: format!("{}-warning-section", id_prefix),
                title: Some(p_text("Warning")),
                content: vec![SectionContentNode::Paragraph(p_text(warning_message))],
                sub_sections: vec![],
                metadata: None,
                display_options: None,
            };

            return MathDocument {
                id: format!("{}-warning-doc", id_prefix),
                title: "Warning: Incorrect Method Used".to_string(),
                language: Some("en-US".to_string()),
                version: Some("1.0".to_string()),
                authors: None,
                date_published: None,
                date_modified: None,
                abstract_content: Some(vec![SectionContentNode::Paragraph(p_text(
                    warning_message,
                ))]),
                table_of_contents: None,
                body: vec![warning_section],
                footnotes: None,
                glossary: None,
                bibliography: None,
                document_metadata: Some(vec![(
                    "warning".to_string(),
                    "For L1 schema, use render_as_l1_schema_document() instead".to_string(),
                )]),
            };
        }

        // For other levels, continue with the existing logic
        let main_section = self.to_section_node(&format!("{}-main", id_prefix));
        MathDocument {
            id: format!("{}-doc", id_prefix),
            title: main_section.title.as_ref().map_or_else(
                || "Group Document".to_string(),
                |p| {
                    p.segments
                        .iter()
                        .map(|s| match s {
                            RichTextSegment::Text(t) => t.clone(),
                            RichTextSegment::StyledText { text, .. } => text.clone(),
                            _ => "".to_string(),
                        })
                        .collect::<String>()
                },
            ),
            language: Some("en-US".to_string()),
            version: Some("1.0".to_string()),
            authors: None,
            date_published: None,
            date_modified: None,
            abstract_content: None,
            table_of_contents: None,
            body: vec![main_section],
            footnotes: None,
            glossary: None,
            bibliography: None,
            document_metadata: None,
        }
    }

    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        match self {
            Group::Basic(g) => g.to_tooltip_node(id_prefix),
            Group::Cyclic(g) => g.to_tooltip_node(id_prefix),
            Group::Topological(g) => g.to_tooltip_node(id_prefix),
            Group::Symmetric(g) => g.to_tooltip_node(id_prefix),
            _ => {
                let name = format!("Group: {}", id_prefix);
                vec![RichTextSegment::Text(name)]
            }
        }
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        match self {
            Group::Basic(g) => g.to_reference_node(id_prefix),
            Group::Cyclic(g) => g.to_reference_node(id_prefix),
            Group::Topological(g) => g.to_reference_node(id_prefix),
            Group::Symmetric(g) => g.to_reference_node(id_prefix),
            _ => {
                let name = format!("Group ({})", id_prefix);
                vec![link_to_definition(
                    &name,
                    &format!("{}-group-section", id_prefix),
                    Some("GroupTheory"),
                )]
            }
        }
    }

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        // For Group enum, delegate to GroupBasic for L1 schema
        match self {
            Group::Basic(g) => g.render_as_l1_schema(id_prefix),
            // For other variants, we can still use the GroupBasic schema as a base representation
            _ => {
                let dummy = GroupBasic::default();
                let mut section = dummy.render_as_l1_schema(id_prefix);

                // Add metadata indicating this is using a default representation
                if let Some(metadata) = &mut section.metadata {
                    metadata.push((
                        "note".to_string(),
                        format!("L1 schema for {:?} variant using GroupBasic as base", self),
                    ));
                }

                section
            }
        }
    }

    fn render_as_l1_schema_document(&self, id_prefix: &str) -> MathDocument {
        // For Group enum, delegate to GroupBasic for L1 schema document
        match self {
            Group::Basic(g) => g.render_as_l1_schema_document(id_prefix),
            // For other variants, we can still use the GroupBasic schema as a base representation
            _ => {
                let dummy = GroupBasic::default();
                let mut doc = dummy.render_as_l1_schema_document(id_prefix);

                // Add metadata indicating this is using a default representation
                if let Some(metadata) = &mut doc.document_metadata {
                    metadata.push((
                        "note".to_string(),
                        format!("L1 schema for {:?} variant using GroupBasic as base", self),
                    ));
                }

                doc
            }
        }
    }
}

impl MathNode {
    pub fn content_as_text(&self) -> Option<String> {
        match self.content.as_ref() {
            MathNodeContent::Text(s) => Some(s.clone()),
            _ => None,
        }
    }
}
