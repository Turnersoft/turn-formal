use serde::{Deserialize, Serialize};

use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::theories::groups::definitions::AlternatingGroup;
use crate::turn_render::math_node::{MathNode, MathNodeContent, ToTurnMath};
use crate::turn_render::section_node::{
    AbstractionMetadata, AcademicMetadata, ContentMetadata, DocumentRelationships,
    DocumentStructure, LinkTarget, MathDocument, MathematicalContentType, PaperType, ParagraphNode,
    RichTextSegment, ScientificPaperContent, Section, SectionContentNode, SelectableProperty,
    StructuredMathNode, ToSectionNode,
};
use crate::turn_render::{IdentifierNode, ScriptNode};

impl ToTurnMath for AlternatingGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        // Use proper mathematical notation A_n with subscript
        MathNode {
            id: master_id.clone(),
            content: Box::new(MathNodeContent::Identifier(IdentifierNode {
                body: "A".to_string(),
                pre_script: None,
                mid_script: None,
                post_script: Some(ScriptNode {
                    subscripts: vec![MathNode {
                        id: format!("{}_subscript", master_id),
                        content: Box::new(MathNodeContent::String(self.degree.to_string())),
                    }],
                    superscripts: vec![],
                }),
                primes: 0,
                is_function: false,
            })),
        }
    }
}

impl ToSectionNode for AlternatingGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();

        // Create title with abstract notation
        let title_segments = vec![
            RichTextSegment::Text("Alternating Group ".to_string()),
            RichTextSegment::Math(MathNode {
                id: format!("{}-title-math", id_prefix),
                content: Box::new(MathNodeContent::Identifier(IdentifierNode {
                    body: "A".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: Some(ScriptNode {
                        subscripts: vec![MathNode {
                            id: format!("{}-title-math-subscript", id_prefix),
                            content: Box::new(MathNodeContent::String("n".to_string())),
                        }],
                        superscripts: vec![],
                    }),
                    primes: 0,
                    is_function: false,
                })),
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
        let mut content_nodes = vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(format!("Degree: {}", self.degree))],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(format!(
                    "Order: {}",
                    factorial(self.degree as usize) / 2
                ))],
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

        content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
            segments: vec![RichTextSegment::Text(
                "The alternating group consists of all even permutations on n elements."
                    .to_string(),
            )],
            alignment: None,
        }));

        // Add abstraction level specific content
        match formalism_obj_level {
            AbstractionLevel::Level1 => {
                content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                    segments: vec![RichTextSegment::Text(
                        "This is L1: A general schema for any alternating group.".to_string(),
                    )],
                    alignment: None,
                }));
            }
            AbstractionLevel::Level2 => {
                content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                    segments: vec![RichTextSegment::Text(
                        "This is L2: A specific type of alternating group with defined properties."
                            .to_string(),
                    )],
                    alignment: None,
                }));

                content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                    segments: vec![RichTextSegment::Text(
                        "The alternating group A_n is the subgroup of the symmetric group S_n consisting of all \
                         even permutations. It has index 2 in S_n.".to_string()
                    )],
                    alignment: None,
                }));
            }
            AbstractionLevel::Level3 => {
                content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                    segments: vec![RichTextSegment::Text(
                        "This is L3: A constructor for building an alternating group from a degree.".to_string(),
                    )],
                    alignment: None,
                }));
            }
            AbstractionLevel::Level4 => {
                content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                    segments: vec![RichTextSegment::Text(
                        "This is L4: A concrete alternating group with fully specified degree and elements.".to_string(),
                    )],
                    alignment: None,
                }));

                // For small degree, list the elements
                if self.degree <= 4 {
                    content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                        segments: vec![RichTextSegment::Text("Elements: ".to_string())],
                        alignment: None,
                    }));

                    if self.degree == 1 || self.degree == 2 {
                        content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                            segments: vec![RichTextSegment::Text("e (identity only)".to_string())],
                            alignment: None,
                        }));
                    } else if self.degree == 3 {
                        content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                            segments: vec![RichTextSegment::Text(
                                "e (identity), (1 2 3), (1 3 2)".to_string(),
                            )],
                            alignment: None,
                        }));
                    } else if self.degree == 4 {
                        content_nodes.push(SectionContentNode::Paragraph(ParagraphNode {
                            segments: vec![RichTextSegment::Text(
                                "e, (1 2)(3 4), (1 3)(2 4), (1 4)(2 3), (1 2 3), (1 3 2), (1 2 4), (1 4 2), (1 3 4), (1 4 3), (2 3 4), (2 4 3)".to_string()
                            )],
                            alignment: None,
                        }));
                    }
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

        // Add abelian property (abelian only for n <= 3)
        if self.degree <= 3 {
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
            current_variant: format!("Finite({})", factorial(self.degree as usize) / 2),
            all_variants: vec!["Finite(n)".to_string(), "Infinite".to_string()],
            description: Some("Order of the group (number of elements)".to_string()),
            variant_descriptions: None,
            property_type_def_id: None,
        });

        // Simple property (A_n is simple for n >= 5)
        if self.degree >= 5 {
            selectable_props.push(SelectableProperty {
                name: "Simple".to_string(),
                current_variant: "Simple".to_string(),
                all_variants: vec!["Simple".to_string(), "NonSimple".to_string()],
                description: Some("Simplicity property (no normal subgroups)".to_string()),
                variant_descriptions: None,
                property_type_def_id: None,
            });
        } else {
            selectable_props.push(SelectableProperty {
                name: "Simple".to_string(),
                current_variant: "NonSimple".to_string(),
                all_variants: vec!["Simple".to_string(), "NonSimple".to_string()],
                description: Some("Simplicity property (no normal subgroups)".to_string()),
                variant_descriptions: None,
                property_type_def_id: None,
            });
        }

        Section {
            id: format!("{}-alternatinggroup-section", id_prefix),
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
            metadata: vec![("type".to_string(), "AlternatingGroupDefinition".to_string())],
            display_options: None,
        }
    }

    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        use crate::turn_render::section_node::{
            AcademicMetadata, ContentMetadata, DocumentRelationships, DocumentStructure,
            MathematicalContentType, ScientificPaperContent,
        };

        let main_section = self.to_section_node(&format!("{}-main", id_prefix));
        let title = main_section.title.as_ref().map_or_else(
            || "Alternating Group Document".to_string(),
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
        let tooltip_text = format!(
            "Alternating Group A_{} (order {})",
            self.degree,
            factorial(self.degree as usize) / 2
        );

        vec![RichTextSegment::Text(tooltip_text)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!("Alternating Group A_{}", self.degree);

        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name.clone())],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}-alternatinggroup-section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!(
                "View definition of {}-alternatinggroup-section",
                id_prefix
            )),
        }]
    }
}

// Helper function to calculate factorial for small numbers
fn factorial(n: usize) -> u64 {
    if n == 0 {
        return 1;
    }

    let mut result: u64 = 1;
    for i in 1..=n {
        result *= i as u64;
    }

    result
}
