use serde::{Deserialize, Serialize};

use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::theories::groups::definitions::ProductGroup;
use crate::turn_render::RefinedMulOrDivOperation;
use crate::turn_render::math_node::{MathNode, MathNodeContent, ToTurnMath};
use crate::turn_render::{
    AbstractionMetadata, AcademicMetadata, ContentMetadata, DocumentRelationships,
    DocumentStructure, LinkTarget, MathDocument, MathDocumentType, PaperType, ParagraphNode,
    RichTextSegment, ScientificPaperContent, Section, SectionContentNode, SelectableProperty,
    StructuredMathNode, ToSectionNode,
};

impl ToTurnMath for ProductGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        if self.components.is_empty() {
            return MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Text("∅".to_string())),
            };
        }

        if self.components.len() == 1 {
            return MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Text("G_1".to_string())),
            };
        }

        // For multiple groups, create a product notation like G₁ × G₂ × ...
        let group_names: Vec<String> = (0..self.components.len())
            .map(|i| format!("G_{}", i + 1))
            .collect();

        MathNode {
            id: master_id.clone(),
            content: Box::new(MathNodeContent::Text(group_names.join(" × "))),
        }
    }
}

impl ToSectionNode for ProductGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();

        // Create title with abstract notation
        let title_segments = vec![
            RichTextSegment::Text("Product Group ".to_string()),
            RichTextSegment::Math(MathNode {
                id: format!("{}-title-math", id_prefix),
                content: Box::new(MathNodeContent::Text(
                    "G_1 \\times G_2 \\times \\cdots \\times G_n".to_string(),
                )),
            }),
        ];

        // Helper function to convert title_segments to a simple string for labels
        let title_text = title_segments
            .iter()
            .map(|seg| match seg {
                RichTextSegment::Text(t) => t.clone(),
                RichTextSegment::Math(_) => "[Math]".to_string(),
                _ => "".to_string(),
            })
            .collect::<String>();

        // Create content nodes
        let mut content_nodes = vec![SectionContentNode::Paragraph(ParagraphNode {
            segments: vec![RichTextSegment::Text(format!(
                "Number of components: {}",
                self.components.len()
            ))],
            alignment: None,
        })];

        // Link to group basic information instead of embedding it directly
        content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
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

        // Add abstraction level specific content
        match formalism_obj_level {
            AbstractionLevel::Level1 => {
                content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                    segments: vec![RichTextSegment::Text(
                        "This is L1: A general schema for any product group.".to_string(),
                    )],
                    alignment: None,
                }));
            }
            AbstractionLevel::Level2 => {
                content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                    segments: vec![RichTextSegment::Text(
                        "This is L2: A specific type of product group with defined properties."
                            .to_string(),
                    )],
                    alignment: None,
                }));
            }
            AbstractionLevel::Level3 => {
                content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                    segments: vec![RichTextSegment::Text(
                        "This is L3: A constructor for building a product group from component groups.".to_string(),
                    )],
                    alignment: None,
                }));
            }
            AbstractionLevel::Level4 => {
                content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                    segments: vec![RichTextSegment::Text(
                        "This is L4: A concrete product group with fully specified components."
                            .to_string(),
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

        // Add standard properties for product groups

        // Abelian if all components are abelian
        selectable_props.push(SelectableProperty {
            name: "Abelian".to_string(),
            current_variant: "Depends on components".to_string(),
            all_variants: vec!["Abelian".to_string(), "NonAbelian".to_string()],
            description: Some(
                "Abelian if and only if all component groups are abelian".to_string(),
            ),
            variant_descriptions: None,
            property_type_def_id: None,
        });

        // Order is product of component orders
        selectable_props.push(SelectableProperty {
            name: "Order".to_string(),
            current_variant: "Product of component orders".to_string(),
            all_variants: vec!["Finite".to_string(), "Infinite".to_string()],
            description: Some("Order is the product of orders of component groups".to_string()),
            variant_descriptions: None,
            property_type_def_id: None,
        });

        Section {
            id: format!("{}-productgroup-section", id_prefix),
            title: Some(ParagraphNode {
                segments: title_segments,
                alignment: None,
            }),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathNode::Definition {
                    term_display: vec![RichTextSegment::Text(title_text.clone())],
                    formal_term: Some(self.to_turn_math(format!("{}-formalTerm", id_prefix))),
                    label: Some(format!("Definition ({})", title_text)),
                    body: content_nodes,
                    abstraction_meta: Some(AbstractionMetadata {
                        level: Some(formalism_obj_level as u8),
                        source_template_id: None,
                        specified_parameters: vec![],
                        universally_quantified_properties: vec![],
                    }),
                    selectable_properties: if selectable_props.is_empty() {
                        vec![]
                    } else {
                        selectable_props
                    },
                },
            )],
            metadata: vec![("type".to_string(), "ProductGroupDefinition".to_string())],
            display_options: None,
        }
    }

    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let main_section = self.to_section_node(&format!("{}-main", id_prefix));
        let title = main_section.title.as_ref().map_or_else(
            || "Product Group Document".to_string(),
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

    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let components_count = self.components.len();
        let tooltip_text = format!("Product Group with {} components", components_count);

        vec![RichTextSegment::Text(tooltip_text)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let components_count = self.components.len();
        let name = format!("Product Group ({})", components_count);

        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name)],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}-productgroup-section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!(
                "View definition of {}-productgroup-section",
                id_prefix
            )),
        }]
    }
}
