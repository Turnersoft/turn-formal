use crate::subjects::math::formalism::traits::abstraction_level::{
    AbstractionLevel, GetAbstractionLevel,
};
use crate::subjects::math::theories::probability::definitions::GenericProbabilitySpace;
use crate::turn_render::*;

impl ToSectionNode for GenericProbabilitySpace {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        Section {
            id: format!("{}.main", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text("Generic Probability Space".to_string())],
                alignment: None,
            }),
            content: SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "A probability space (Ω, ℱ, P) consists of a sample space Ω, a σ-algebra ℱ of events, and a probability measure P.".to_string()
                )],
                alignment: None,
            }),
            metadata: vec![],
            display_options: None,
        }
    }
}

impl ToMathDocument for GenericProbabilitySpace {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let main_section = self.to_section_node(id_prefix);

        MathDocument {
            id: format!("{}.doc", id_prefix),
            content_type: MathDocumentType::ScientificPaper(ScientificPaperContent {
                title: "Generic Probability Space".to_string(),
                paper_type: PaperType::Research,
                venue: Some("Mathematical Probability".to_string()),
                peer_reviewed: true,
                content_metadata: ContentMetadata {
                    language: Some("en-US".to_string()),
                    version: Some("1.0".to_string()),
                    created_at: None,
                    last_modified: None,
                    content_hash: None,
                },
                academic_metadata: AcademicMetadata {
                    authors: vec!["Turn-Formal System".to_string()],
                    date_published: None,
                    date_modified: None,
                    venue: Some("Mathematical Probability".to_string()),
                    doi: None,
                    keywords: vec!["probability space".to_string()],
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
                    dependency_graph: None,
                    cross_references: vec![],
                },
            }),
        }
    }
}

impl GenericProbabilitySpace {
    pub fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        vec![RichTextSegment::Text(
            "Generic Probability Space (Ω, ℱ, P)".to_string(),
        )]
    }

    pub fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        vec![RichTextSegment::Text("(Ω, ℱ, P)".to_string())]
    }

    pub fn render_as_l1_schema_document(&self, id_prefix: &str) -> MathDocument {
        let main_section = Section {
            id: format!("{}.schema", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text("Probability Space Schema".to_string())],
                alignment: None,
            }),
            content: SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "Abstract schema for probability spaces with sample space, σ-algebra, and probability measure.".to_string()
                )],
                alignment: None,
            }),
            metadata: vec![],
            display_options: None,
        };

        MathDocument {
            id: format!("{}.schema.doc", id_prefix),
            content_type: MathDocumentType::ScientificPaper(ScientificPaperContent {
                title: "Probability Space Schema".to_string(),
                paper_type: PaperType::Research,
                venue: Some("Mathematical Schemas".to_string()),
                peer_reviewed: true,
                content_metadata: ContentMetadata {
                    language: Some("en-US".to_string()),
                    version: Some("1.0".to_string()),
                    created_at: None,
                    last_modified: None,
                    content_hash: None,
                },
                academic_metadata: AcademicMetadata {
                    authors: vec!["Turn-Formal System".to_string()],
                    date_published: None,
                    date_modified: None,
                    venue: Some("Mathematical Schemas".to_string()),
                    doi: None,
                    keywords: vec!["probability".to_string(), "schema".to_string()],
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
                    dependency_graph: None,
                    cross_references: vec![],
                },
            }),
        }
    }
}
