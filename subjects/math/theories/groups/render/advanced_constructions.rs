use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use crate::turn_render::math_node::{
    BracketSize, BracketStyle, MathNode, MathNodeContent, ToTurnMath,
};
use crate::turn_render::section_node::{
    AbstractionMetadata, LinkTarget, ParagraphNode, RichTextSegment, Section, SectionContentNode,
    StructuredMathContentNode, ToSectionNode,
};

use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::theories::groups::definitions::{
    CentralProductGroup, GenericGroup, PullbackGroup, RestrictionGroup, WreathProductGroup,
};

impl ToSectionNode for WreathProductGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        let title = "G ≀ H".to_string();

        let content_nodes = vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "The wreath product G ≀ H of groups G and H.".to_string(),
                )],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "This construction combines the direct product with a group action."
                        .to_string(),
                )],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "Elements are pairs (f, h) where f: H → G is a function and h ∈ H.".to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-wreathproduct-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathContentNode::Definition {
                    term_display: vec![RichTextSegment::Text(title.clone())],
                    formal_term: Some(self.core.to_turn_math(format!("{}-formalTerm", id_prefix))),
                    label: Some(format!("Definition ({})", title)),
                    body: content_nodes,
                    abstraction_meta: Some(AbstractionMetadata {
                        level: Some(formalism_obj_level as u8),
                        source_template_id: None,
                        specified_parameters: vec![],
                        universally_quantified_properties: vec![],
                    }),
                    selectable_properties: vec![],
                },
            )],
            metadata: vec![(
                "type".to_string(),
                "WreathProductGroupDefinition".to_string(),
            )],
            display_options: None,
        }
    }

    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = "G ≀ H".to_string();
        vec![RichTextSegment::Text(name)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = "G ≀ H".to_string();
        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name)],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}-wreathproduct-section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!(
                "View definition of {}-wreathproduct-section",
                id_prefix
            )),
        }]
    }

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = "G ≀ H".to_string();

        Section {
            id: format!("{}-main-wreathproduct-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "The wreath product G ≀ H of groups G and H. This construction combines the direct product with a group action.".to_string(),
                )],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }

    fn to_math_document(
        &self,
        id_prefix: &str,
    ) -> crate::turn_render::section_node::MathematicalContent {
        let main_section = self.to_section_node(id_prefix);
        let title = "G ≀ H".to_string();

        use crate::turn_render::section_node::*;

        MathematicalContent {
            id: format!("{}-doc", id_prefix),
            content_type: MathematicalContentType::ScientificPaper(ScientificPaperContent {
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

impl ToSectionNode for CentralProductGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        let title = "G ∘ H".to_string();

        let content_nodes = vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text("A central product group G ∘ H.".to_string())],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "This is formed by identifying the centers of component groups.".to_string(),
                )],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "The construction allows for combining groups while maintaining certain structural properties.".to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-centralproduct-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathContentNode::Definition {
                    term_display: vec![RichTextSegment::Text(title.clone())],
                    formal_term: Some(self.core.to_turn_math(format!("{}-formalTerm", id_prefix))),
                    label: Some(format!("Definition ({})", title)),
                    body: content_nodes,
                    abstraction_meta: Some(AbstractionMetadata {
                        level: Some(formalism_obj_level as u8),
                        source_template_id: None,
                        specified_parameters: vec![],
                        universally_quantified_properties: vec![],
                    }),
                    selectable_properties: vec![],
                },
            )],
            metadata: vec![(
                "type".to_string(),
                "CentralProductGroupDefinition".to_string(),
            )],
            display_options: None,
        }
    }

    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = "G ∘ H".to_string();
        vec![RichTextSegment::Text(name)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = "G ∘ H".to_string();
        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name)],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}-centralproduct-section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!(
                "View definition of {}-centralproduct-section",
                id_prefix
            )),
        }]
    }

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = "G ∘ H".to_string();

        Section {
            id: format!("{}-main-centralproduct-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "A central product group formed by identifying the centers of component groups. This construction allows for combining groups while maintaining certain structural properties.".to_string(),
                )],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }

    fn to_math_document(
        &self,
        id_prefix: &str,
    ) -> crate::turn_render::section_node::MathematicalContent {
        let main_section = self.to_section_node(id_prefix);
        let title = "G ∘ H".to_string();

        use crate::turn_render::section_node::*;

        MathematicalContent {
            id: format!("{}-doc", id_prefix),
            content_type: MathematicalContentType::ScientificPaper(ScientificPaperContent {
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

impl ToSectionNode for PullbackGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        let title = "G ×_H K".to_string();

        let content_nodes = vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "The pullback (fibered product) G ×_H K of groups over a common target H."
                        .to_string(),
                )],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "This is a universal construction in the category of groups.".to_string(),
                )],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "Elements are pairs (g, k) where g ∈ G and k ∈ K such that φ(g) = ψ(k)."
                        .to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-pullback-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathContentNode::Definition {
                    term_display: vec![RichTextSegment::Text(title.clone())],
                    formal_term: Some(self.core.to_turn_math(format!("{}-formalTerm", id_prefix))),
                    label: Some(format!("Definition ({})", title)),
                    body: content_nodes,
                    abstraction_meta: Some(AbstractionMetadata {
                        level: Some(formalism_obj_level as u8),
                        source_template_id: None,
                        specified_parameters: vec![],
                        universally_quantified_properties: vec![],
                    }),
                    selectable_properties: vec![],
                },
            )],
            metadata: vec![("type".to_string(), "PullbackGroupDefinition".to_string())],
            display_options: None,
        }
    }

    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = "G ×_H K".to_string();
        vec![RichTextSegment::Text(name)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = "G ×_H K".to_string();
        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name)],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}-pullback-section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!("View definition of {}-pullback-section", id_prefix)),
        }]
    }

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = "G ×_H K".to_string();

        Section {
            id: format!("{}-main-pullback-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "The pullback (fibered product) G ×_H K of groups over a common target H."
                        .to_string(),
                )],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }

    fn to_math_document(
        &self,
        id_prefix: &str,
    ) -> crate::turn_render::section_node::MathematicalContent {
        let main_section = self.to_section_node(id_prefix);
        let title = "G ×_H K".to_string();

        use crate::turn_render::section_node::*;

        MathematicalContent {
            id: format!("{}-doc", id_prefix),
            content_type: MathematicalContentType::ScientificPaper(ScientificPaperContent {
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

impl ToSectionNode for RestrictionGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        let title = "G|_S".to_string();

        let content_nodes = vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "The restriction G|_S of a group G to a subset S.".to_string(),
                )],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "This construction restricts the group operation to a specific subset."
                        .to_string(),
                )],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "The subset S must satisfy specific conditions to form a valid subgroup."
                        .to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-restriction-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathContentNode::Definition {
                    term_display: vec![RichTextSegment::Text(title.clone())],
                    formal_term: Some(self.core.to_turn_math(format!("{}-formalTerm", id_prefix))),
                    label: Some(format!("Definition ({})", title)),
                    body: content_nodes,
                    abstraction_meta: Some(AbstractionMetadata {
                        level: Some(formalism_obj_level as u8),
                        source_template_id: None,
                        specified_parameters: vec![],
                        universally_quantified_properties: vec![],
                    }),
                    selectable_properties: vec![],
                },
            )],
            metadata: vec![("type".to_string(), "RestrictionGroupDefinition".to_string())],
            display_options: None,
        }
    }

    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = "G|_S".to_string();
        vec![RichTextSegment::Text(name)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = "G|_S".to_string();
        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name)],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}-restriction-section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!(
                "View definition of {}-restriction-section",
                id_prefix
            )),
        }]
    }

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = "G|_S".to_string();

        Section {
            id: format!("{}-main-restriction-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "The restriction G|_S of a group G to a subset S satisfying specific conditions.".to_string(),
                )],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }

    fn to_math_document(
        &self,
        id_prefix: &str,
    ) -> crate::turn_render::section_node::MathematicalContent {
        let main_section = self.to_section_node(id_prefix);
        let title = "G|_S".to_string();

        use crate::turn_render::section_node::*;

        MathematicalContent {
            id: format!("{}-doc", id_prefix),
            content_type: MathematicalContentType::ScientificPaper(ScientificPaperContent {
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
