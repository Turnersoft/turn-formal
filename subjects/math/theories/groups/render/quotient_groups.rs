use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use crate::turn_render::math_node::{
    BracketSize, BracketStyle, MathNode, MathNodeContent, ToTurnMath,
};
use crate::turn_render::{
    AbstractionMetadata, LinkTarget, RichText, RichTextSegment, Section, SectionContentNode,
    StructuredMathNode, ToSectionNode,
};

use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::theories::groups::definitions::{
    FreeGroup, GenericGroup, QuotientGroup, TrivialGroup,
};
use crate::turn_render::{
    AcademicMetadata, ContentMetadata, DocumentRelationships, DocumentStructure, MathDocument,
    MathDocumentType, PaperType, ScientificPaperContent, ToMathDocument,
};

impl ToSectionNode for QuotientGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();

        let group_name = "G";
        let normal_name = "N";
        let title = format!("{}/{}", group_name, normal_name);

        let content_nodes = vec![
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!(
                    "Parent Group: {}",
                    group_name
                ))],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!(
                    "Normal Subgroup: {}",
                    normal_name
                ))],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "Quotient group formed by the cosets of the normal subgroup.".to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-quotient-section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathNode::Definition {
                    term_display: RichText {
                        segments: vec![RichTextSegment::Text(title.clone())],
                        alignment: None,
                    },
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
            metadata: vec![("type".to_string(), "QuotientGroupDefinition".to_string())],
            display_options: None,
        }
    }
}

impl ToMathDocument for QuotientGroup {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let main_section = self.to_section_node(id_prefix);
        let group_name = "G";
        let normal_name = "N";
        let title = format!("{}/{}", group_name, normal_name);

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

impl QuotientGroup {
    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let group_name = "G";
        let normal_name = "N";
        let name = format!("{}/{}", group_name, normal_name);
        vec![RichTextSegment::Text(name)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let group_name = "G";
        let normal_name = "N";
        let name = format!("{}/{}", group_name, normal_name);
        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name)],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}-quotient-section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!("View definition of {}-quotient-section", id_prefix)),
        }]
    }

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let group_name = "G";
        let normal_name = "N";
        let title = format!("{}/{}", group_name, normal_name);

        Section {
            id: format!("{}-main-quotient-section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!(
                    "The quotient group {}/{} formed by the cosets of the normal subgroup {} in {}.",
                    group_name, normal_name, normal_name, group_name
                ))],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }
}

impl ToSectionNode for FreeGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        let title = format!("F_{}", self.rank);

        let content_nodes = vec![
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!("Rank: {}", self.rank))],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text("Free group on the given number of generators.".to_string())],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "The most general group with the given generators, subject only to the group axioms.".to_string()
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-free-section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathNode::Definition {
                    term_display: RichText {
                        segments: vec![RichTextSegment::Text(title.clone())],
                        alignment: None,
                    },
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
            metadata: vec![("type".to_string(), "FreeGroupDefinition".to_string())],
            display_options: None,
        }
    }
}

impl ToMathDocument for FreeGroup {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let main_section = self.to_section_node(id_prefix);
        let title = format!("F_{}", self.rank);

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

impl FreeGroup {
    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!("F_{}", self.rank);
        vec![RichTextSegment::Text(name)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!("F_{}", self.rank);
        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name)],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}-free-section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!("View definition of {}-free-section", id_prefix)),
        }]
    }

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = format!("F_{}", self.rank);

        Section {
            id: format!("{}-main-free-section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!(
                    "The free group F_{} on {} generators. This is the most general group with {} generators, subject only to the group axioms.",
                    self.rank, self.rank, self.rank
                ))],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }
}

impl ToSectionNode for TrivialGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        let title = "1".to_string();

        let content_nodes = vec![
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "The trivial group, containing only the identity element.".to_string(),
                )],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "This is the unique group of order 1.".to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-trivial-section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathNode::Definition {
                    term_display: RichText {
                        segments: vec![RichTextSegment::Text(title.clone())],
                        alignment: None,
                    },
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
            metadata: vec![("type".to_string(), "TrivialGroupDefinition".to_string())],
            display_options: None,
        }
    }
}

impl ToMathDocument for TrivialGroup {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let main_section = self.to_section_node(id_prefix);
        let title = "1".to_string();

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

impl TrivialGroup {
    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = "1".to_string();
        vec![RichTextSegment::Text(name)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = "1".to_string();
        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name)],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}-trivial-section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!("View definition of {}-trivial-section", id_prefix)),
        }]
    }

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = "1".to_string();

        Section {
            id: format!("{}-main-trivial-section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "The trivial group {1}, containing only the identity element.".to_string(),
                )],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }
}

impl ToTurnMath for QuotientGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let group_name = "G";
        let normal_name = "N";
        MathNode {
            id: master_id.clone(),
            content: Arc::new(MathNodeContent::Text(format!(
                "{}/{}",
                group_name, normal_name
            ))),
        }
    }
}

impl ToTurnMath for FreeGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id.clone(),
            content: Arc::new(MathNodeContent::Text(format!("F_{}", self.rank))),
        }
    }
}

impl ToTurnMath for TrivialGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id.clone(),
            content: Arc::new(MathNodeContent::Text("1".to_string())),
        }
    }
}
