use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::subjects::math::formalism::traits::abstraction_level::{
    AbstractionLevel, GetAbstractionLevel,
};
use crate::subjects::math::theories::groups::definitions::LieGroup;
use crate::turn_render::math_node::{MathNode, MathNodeContent, ToTurnMath};
use crate::turn_render::*;

impl ToTurnMath for LieGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        // Create a representative notation for Lie Group
        let base_set_str = format!("{:?}", self.core.base_set);

        MathNode {
            id: master_id,
            content: Arc::new(MathNodeContent::Text(format!("Lie({}, τ)", base_set_str))),
        }
    }
}

impl ToSectionNode for LieGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();

        // **ABSTRACT MATHEMATICAL NOTATION** - Use proper mathematical notation
        let title = "(G, τ)".to_string(); // G for group, τ for topology

        // **DETAILED MATHEMATICAL EXPLANATION**
        let mut content_nodes = vec![
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "A Lie group is a group that is also a differentiable manifold, with the property that \
                     the group operations (multiplication and inversion) are smooth (infinitely differentiable) functions.".to_string(),
                )],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "Definition: A Lie group is a set G equipped with both a group structure and a smooth manifold structure such that the group operations μ: G × G → G (multiplication) and ι: G → G (inversion) are smooth maps.".to_string(),
                )],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "Key Properties: Every Lie group is locally Euclidean, and near the identity element, the group structure is determined by its Lie algebra.".to_string(),
                )],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "Classical Examples: GL(n, ℝ) (general linear group), SL(n, ℝ) (special linear group), O(n) (orthogonal group), SO(n) (special orthogonal group), U(n) (unitary group), SU(n) (special unitary group).".to_string(),
                )],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "Applications: Lie groups are fundamental in differential geometry, physics (symmetries of physical systems), representation theory, and the study of differential equations.".to_string(),
                )],
                alignment: None,
            }),
        ];

        // Link to group basic information instead of embedding it directly
        content_nodes.push(SectionContentNode::RichText(RichText {
            segments: vec![
                RichTextSegment::Text("For the underlying group structure, see ".to_string()),
                RichTextSegment::Link {
                    content: vec![RichTextSegment::Text("Group Theory".to_string())],
                    target: LinkTarget::DefinitionId {
                        term_id: format!("{}-groupbasic-section", id_prefix),
                        theory_context: Some("GroupTheory".to_string()),
                    },
                    tooltip: Some(format!(
                        "View definition of {}-groupbasic-section",
                        id_prefix
                    )),
                },
                RichTextSegment::Text(".".to_string()),
            ],
            alignment: None,
        }));

        content_nodes.push(SectionContentNode::RichText(RichText {
            segments: vec![RichTextSegment::Text(format!(
                "Manifold Topology: {:?}",
                self.topology.base_set
            ))],
            alignment: None,
        }));

        // Add charts information if available
        if !self.charts.is_empty() {
            content_nodes.push(SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!(
                    "Charts: {}",
                    self.charts.join(", ")
                ))],
                alignment: None,
            }));
        }

        // Add abstraction level specific content
        match formalism_obj_level {
            AbstractionLevel::Level1 => {
                content_nodes.push(SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "This is L1: A general schema for any Lie group.".to_string(),
                    )],
                    alignment: None,
                }));
            }
            AbstractionLevel::Level2 => {
                content_nodes.push(SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "This is L2: A specific type of Lie group with defined properties."
                            .to_string(),
                    )],
                    alignment: None,
                }));

                content_nodes.push(SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "Examples of Lie groups include the classical groups: GL(n, ℝ), SL(n, ℝ), O(n), SO(n), \
                         U(n), SU(n), and Sp(n).".to_string(),
                    )],
                    alignment: None,
                }));
            }
            AbstractionLevel::Level3 => {
                content_nodes.push(SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "This is L3: A constructor for building a Lie group from a group and a compatible manifold structure.".to_string(),
                    )],
                    alignment: None,
                }));
            }
            AbstractionLevel::Level4 => {
                content_nodes.push(SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(
                        "This is L4: A concrete Lie group with fully specified group structure and manifold structure.".to_string(),
                    )],
                    alignment: None,
                }));
            }
        };

        // Create selectable properties
        let mut selectable_props = vec![];

        // Add properties from core group if any
        if !self.core.props.inner.is_empty() {
            for prop in self.core.props.inner.iter() {
                selectable_props.push(SelectableProperty {
                    name: format!("{:?}", prop.0),
                    current_variant: format!("{:?}", prop.0),
                    all_variants: vec![format!("{:?}", prop.0)],
                    description: Some("Group property".to_string()),
                    variant_descriptions: None,
                    property_type_def_id: None,
                });
            }
        }

        // Add Lie group specific properties
        if !self.props.inner.is_empty() {
            for prop in self.props.inner.iter() {
                selectable_props.push(SelectableProperty {
                    name: format!("{:?}", prop.0),
                    current_variant: format!("{:?}", prop.0),
                    all_variants: vec![format!("{:?}", prop.0)],
                    description: Some("Lie group property".to_string()),
                    variant_descriptions: None,
                    property_type_def_id: None,
                });
            }
        }

        // Add standard Lie group properties
        selectable_props.push(SelectableProperty {
            name: "Connected".to_string(),
            current_variant: "Connected".to_string(),
            all_variants: vec!["Connected".to_string(), "Disconnected".to_string()],
            description: Some("Connectedness property".to_string()),
            variant_descriptions: None,
            property_type_def_id: None,
        });

        Section {
            id: format!("{}-liegroup-section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: SectionContentNode::SubSection(vec![
                Section {
                    id: format!("{}-definition-text", id_prefix),
                    title: None,
                    content: SectionContentNode::RichText(RichText {
                        segments: vec![RichTextSegment::StyledText {
                            text: format!("Definition: {}", title),
                            styles: vec![TextStyle::Bold],
                        }],
                        alignment: None,
                    }),
                    metadata: vec![],
                    display_options: None,
                },
                Section {
                    id: format!("{}-formal-term", id_prefix),
                    title: None,
                    content: SectionContentNode::Math(
                        self.to_turn_math(format!("{}-formalTerm", id_prefix)),
                    ),
                    metadata: vec![],
                    display_options: None,
                },
                Section {
                    id: format!("{}-collapsible-definition", id_prefix),
                    title: None,
                    content: SectionContentNode::CollapsibleBlock(CollapsibleBlockNode {
                        summary: vec![RichTextSegment::Text(format!("Definition ({})", title))],
                        details: content_nodes,
                        initially_collapsed: Some(false),
                    }),
                    metadata: vec![],
                    display_options: None,
                },
            ]),
            metadata: vec![("type".to_string(), "LieGroupDefinition".to_string())],
            display_options: None,
        }
    }
}

impl ToMathDocument for LieGroup {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let main_section = self.to_section_node(&format!("{}-main", id_prefix));
        let title = main_section.title.as_ref().map_or_else(
            || "Lie Group Document".to_string(),
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
        );

        MathDocument {
            id: format!("{}-doc", id_prefix),
            content_type: MathDocumentType::ScientificPaper(ScientificPaperContent {
                title,
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
                    keywords: vec![],
                },
                structure: DocumentStructure {
                    abstract_content: None,
                    table_of_contents: None,
                    body: vec![main_section],
                    footnotes: vec![],
                    glossary: vec![],
                    bibliography: vec![],
                },
                relationships: DocumentRelationships {
                    parent_documents: vec![],
                    child_documents: vec![],
                    related_concepts: vec![],
                    cross_references: vec![],
                    dependency_graph: None,
                },
            }),
        }
    }
}
