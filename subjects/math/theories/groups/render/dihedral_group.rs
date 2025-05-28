use serde::{Deserialize, Serialize};

use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::theories::groups::definitions::DihedralGroup;
use crate::turn_render::math_node::{MathNode, MathNodeContent, ToTurnMath};
use crate::turn_render::section_node::{
    AbstractionMetadata, AcademicMetadata, ContentMetadata, DocumentRelationships,
    DocumentStructure, LinkTarget, MathDocument, MathematicalContentType, PaperType, ParagraphNode,
    RichTextSegment, ScientificPaperContent, Section, SectionContentNode, SelectableProperty,
    StructuredMathContentNode, ToSectionNode,
};

impl ToTurnMath for DihedralGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        // Use proper mathematical notation D_n with subscript
        let n = self.order / 2; // Dihedral group D_n has order 2n
        MathNode {
            id: master_id.clone(),
            content: Box::new(MathNodeContent::Identifier {
                body: "D".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: Some(Box::new(MathNode {
                    id: format!("{}_subscript", master_id),
                    content: Box::new(MathNodeContent::Quantity {
                        number: n.to_string(),
                        unit: None,
                    }),
                })),
                primes: 0,
                is_function: false,
            }),
        }
    }
}

impl ToSectionNode for DihedralGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();

        // Create title
        let n = self.order / 2; // Dihedral group D_n has order 2n
        let title = format!("Dihedral Group D_{}", n);

        // Create content nodes
        let mut content_nodes = vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(format!("Regular n-gon: n = {}", n))],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(format!("Order: {}", self.order))],
                alignment: None,
            }),
        ];

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
                        "This is L1: A general schema for any dihedral group.".to_string(),
                    )],
                    alignment: None,
                }));
            }
            AbstractionLevel::Level2 => {
                content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                    segments: vec![RichTextSegment::Text(
                        "This is L2: A specific type of dihedral group with defined properties."
                            .to_string(),
                    )],
                    alignment: None,
                }));
            }
            AbstractionLevel::Level3 => {
                content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                    segments: vec![RichTextSegment::Text(
                        "This is L3: A constructor for building a dihedral group from a regular n-gon.".to_string(),
                    )],
                    alignment: None,
                }));
            }
            AbstractionLevel::Level4 => {
                content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                    segments: vec![RichTextSegment::Text(
                        "This is L4: A concrete dihedral group with fully specified elements."
                            .to_string(),
                    )],
                    alignment: None,
                }));

                // For small n, list the elements
                if n <= 4 {
                    content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                        segments: vec![RichTextSegment::Text(format!(
                            "Elements: {} rotations and {} reflections",
                            n, n
                        ))],
                        alignment: None,
                    }));
                }
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

        // Abelian only for n=1 and n=2
        if n <= 2 {
            selectable_props.push(SelectableProperty {
                name: "Abelian".to_string(),
                current_variant: "Abelian".to_string(),
                all_variants: vec!["Abelian".to_string(), "NonAbelian".to_string()],
                description: Some("Commutativity property".to_string()),
                variant_descriptions: None,
                property_type_def_id: None,
            });
        } else {
            selectable_props.push(SelectableProperty {
                name: "Abelian".to_string(),
                current_variant: "NonAbelian".to_string(),
                all_variants: vec!["Abelian".to_string(), "NonAbelian".to_string()],
                description: Some("Commutativity property".to_string()),
                variant_descriptions: None,
                property_type_def_id: None,
            });
        }

        // Always finite
        selectable_props.push(SelectableProperty {
            name: "Order".to_string(),
            current_variant: format!("Finite({})", self.order),
            all_variants: vec!["Finite(n)".to_string(), "Infinite".to_string()],
            description: Some("Order of the group (number of elements)".to_string()),
            variant_descriptions: None,
            property_type_def_id: None,
        });

        Section {
            id: format!("{}-dihedralgroup-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathContentNode::Definition {
                    term_display: vec![RichTextSegment::Text(title.clone())],
                    formal_term: Some(self.to_turn_math(format!("{}-formalTerm", id_prefix))),
                    label: Some(format!("Definition ({})", title)),
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
            metadata: vec![("type".to_string(), "DihedralGroupDefinition".to_string())],
            display_options: None,
        }
    }

    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let main_section = self.to_section_node(&format!("{}-main", id_prefix));
        let title = main_section.title.as_ref().map_or_else(
            || "Dihedral Group Document".to_string(),
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
            content_type: MathematicalContentType::ScientificPaper(ScientificPaperContent {
                title,
                paper_type: crate::turn_render::section_node::PaperType::Research,
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
        let n = self.order / 2;
        let tooltip_text = format!("Dihedral Group D_{} (order {})", n, self.order);

        vec![RichTextSegment::Text(tooltip_text)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let n = self.order / 2;
        let name = format!("Dihedral Group D_{}", n);

        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name)],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}-dihedralgroup-section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!(
                "View definition of {}-dihedralgroup-section",
                id_prefix
            )),
        }]
    }
}
