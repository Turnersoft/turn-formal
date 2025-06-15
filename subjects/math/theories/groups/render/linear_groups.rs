use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use crate::turn_render::ToMathDocument;
use crate::turn_render::*;

use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::theories::groups::definitions::{
    GeneralLinearGroup, GenericGroup, OrthogonalGroup, SpecialLinearGroup, SpecialOrthogonalGroup,
    SpecialUnitaryGroup, UnitaryGroup,
};

impl ToTurnMath for GeneralLinearGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id.clone(),
            content: Box::new(MathNodeContent::Text(format!("GL({}, 𝔽)", self.dimension))),
        }
    }
}

impl ToSectionNode for GeneralLinearGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();

        let title = format!(
            "GL({}, {})",
            self.dimension,
            "𝔽" // Simple field placeholder instead of calling content_as_text
        );

        let content_nodes = vec![
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!(
                    "Dimension: {}",
                    self.dimension
                ))],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!(
                    "Field: {}",
                    "𝔽" // Simple field placeholder instead of calling content_as_text
                ))],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "General linear group of invertible matrices over the given field.".to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-generallinear-section", id_prefix),
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
            metadata: vec![(
                "type".to_string(),
                "GeneralLinearGroupDefinition".to_string(),
            )],
            display_options: None,
        }
    }

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = format!(
            "GL({}, {})",
            self.dimension,
            "𝔽" // Simple field placeholder instead of calling content_as_text
        );

        Section {
            id: format!("{}-main-generallinear-section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!(
                    "The general linear group GL({}, {}) is the group of all invertible {}×{} matrices over the field {}. It forms a group under matrix multiplication.",
                    self.dimension,
                    "𝔽", // Simple field placeholder instead of calling content_as_text
                    self.dimension,
                    self.dimension,
                    "𝔽" // Simple field placeholder instead of calling content_as_text
                ))],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }
}

impl ToMathDocument for GeneralLinearGroup {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let level = self.level();

        // Use abstract notation for L1 groups
        let title = if level == AbstractionLevel::Level1 {
            "GL(n, F)".to_string()
        } else {
            format!("GL({}, {})", self.dimension, "𝔽")
        };

        let main_section = self.to_section_node(id_prefix);

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

impl ToSectionNode for SpecialLinearGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        let title = format!("SL({})", self.general_linear.dimension);

        let content_nodes = vec![
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!(
                    "Dimension: {}",
                    self.general_linear.dimension
                ))],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "Special linear group of matrices with determinant 1.".to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-speciallinear-section", id_prefix),
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
                    formal_term: Some(
                        self.general_linear
                            .to_turn_math(format!("{}-formalTerm", id_prefix)),
                    ),
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
                "SpecialLinearGroupDefinition".to_string(),
            )],
            display_options: None,
        }
    }

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = "SL(n,F)".to_string();

        Section {
            id: format!("{}-main-speciallinear-section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "The special linear group SL(n,F) is the group of all n×n matrices over the field F with determinant 1. It forms a group under matrix multiplication.".to_string(),
                )],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }
}

impl ToMathDocument for SpecialLinearGroup {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let level = self.level();

        // Use abstract notation for L1 groups
        let title = if level == AbstractionLevel::Level1 {
            "SL(n, F)".to_string()
        } else {
            format!("SL({})", self.general_linear.dimension)
        };

        let main_section = self.to_section_node(id_prefix);

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

impl ToSectionNode for OrthogonalGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        let title = format!("O({})", self.dimension);

        let content_nodes = vec![
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!(
                    "Dimension: {}",
                    self.dimension
                ))],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "Orthogonal group of orthogonal matrices.".to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-orthogonal-section", id_prefix),
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
            metadata: vec![("type".to_string(), "OrthogonalGroupDefinition".to_string())],
            display_options: None,
        }
    }

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = "O(n)".to_string();

        Section {
            id: format!("{}-main-orthogonal-section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "The orthogonal group O(n) is the group of all n×n orthogonal matrices over the real numbers. It forms a group under matrix multiplication.".to_string(),
                )],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }
}

impl ToMathDocument for OrthogonalGroup {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let level = self.level();

        // Use abstract notation for L1 groups
        let title = if level == AbstractionLevel::Level1 {
            "O(n)".to_string()
        } else {
            format!("O({})", self.dimension)
        };

        let main_section = self.to_section_node(id_prefix);

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

impl ToSectionNode for SpecialOrthogonalGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        let title = format!("SO({})", self.orthogonal.dimension);

        let content_nodes = vec![
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!(
                    "Dimension: {}",
                    self.orthogonal.dimension
                ))],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "Special orthogonal group of orthogonal matrices with determinant 1."
                        .to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-specialorthogonal-section", id_prefix),
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
                    formal_term: Some(
                        self.orthogonal
                            .to_turn_math(format!("{}-formalTerm", id_prefix)),
                    ),
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
                "SpecialOrthogonalGroupDefinition".to_string(),
            )],
            display_options: None,
        }
    }

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = "SO(n)".to_string();

        Section {
            id: format!("{}-main-specialorthogonal-section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "The special orthogonal group SO(n) is the group of all n×n orthogonal matrices over the real numbers with determinant 1. It forms a group under matrix multiplication.".to_string(),
                )],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }
}

impl SpecialOrthogonalGroup {
    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!("SO({})", self.orthogonal.dimension);
        vec![RichTextSegment::Text(name)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!("SO({})", self.orthogonal.dimension);
        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name)],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}-specialorthogonal-section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!(
                "View definition of {}-specialorthogonal-section",
                id_prefix
            )),
        }]
    }
}

impl ToMathDocument for SpecialOrthogonalGroup {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let level = self.level();

        // Use abstract notation for L1 groups
        let title = if level == AbstractionLevel::Level1 {
            "SO(n)".to_string()
        } else {
            format!("SO({})", self.orthogonal.dimension)
        };

        let main_section = self.to_section_node(id_prefix);

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

impl ToSectionNode for UnitaryGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        let title = format!("U({})", self.dimension);

        let content_nodes = vec![
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!(
                    "Dimension: {}",
                    self.dimension
                ))],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "Unitary group of unitary matrices over the complex numbers.".to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-unitary-section", id_prefix),
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
            metadata: vec![("type".to_string(), "UnitaryGroupDefinition".to_string())],
            display_options: None,
        }
    }

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = "U(n)".to_string();

        Section {
            id: format!("{}-main-unitary-section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "The unitary group U(n) is the group of all n×n unitary matrices over the complex numbers. It forms a group under matrix multiplication.".to_string(),
                )],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }
}

impl UnitaryGroup {
    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!("U({})", self.dimension);
        vec![RichTextSegment::Text(name)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!("U({})", self.dimension);
        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name)],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}-unitary-section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!("View definition of {}-unitary-section", id_prefix)),
        }]
    }
}

impl ToMathDocument for UnitaryGroup {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let level = self.level();

        // Use abstract notation for L1 groups
        let title = if level == AbstractionLevel::Level1 {
            "U(n)".to_string()
        } else {
            format!("U({})", self.dimension)
        };

        let main_section = self.to_section_node(id_prefix);

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

impl ToSectionNode for SpecialUnitaryGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        let title = format!("SU({})", self.unitary.dimension);

        let content_nodes = vec![
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!(
                    "Dimension: {}",
                    self.unitary.dimension
                ))],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "Special unitary group of unitary matrices with determinant 1.".to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-specialunitary-section", id_prefix),
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
                    formal_term: Some(
                        self.unitary
                            .to_turn_math(format!("{}-formalTerm", id_prefix)),
                    ),
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
                "SpecialUnitaryGroupDefinition".to_string(),
            )],
            display_options: None,
        }
    }

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = "SU(n)".to_string();

        Section {
            id: format!("{}-main-specialunitary-section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "The special unitary group SU(n) is the group of all n×n unitary matrices over the complex numbers with determinant 1. It forms a group under matrix multiplication.".to_string(),
                )],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }
}

impl SpecialUnitaryGroup {
    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!("SU({})", self.unitary.dimension);
        vec![RichTextSegment::Text(name)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!("SU({})", self.unitary.dimension);
        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name)],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}-specialunitary-section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!(
                "View definition of {}-specialunitary-section",
                id_prefix
            )),
        }]
    }
}

impl ToMathDocument for SpecialUnitaryGroup {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let level = self.level();

        // Use abstract notation for L1 groups
        let title = if level == AbstractionLevel::Level1 {
            "SU(n)".to_string()
        } else {
            format!("SU({})", self.unitary.dimension)
        };

        let main_section = self.to_section_node(id_prefix);

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

impl ToTurnMath for SpecialLinearGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id.clone(),
            content: Box::new(MathNodeContent::Text(format!(
                "SL({}, 𝔽)",
                self.general_linear.dimension
            ))),
        }
    }
}

impl ToTurnMath for OrthogonalGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id.clone(),
            content: Box::new(MathNodeContent::Text(format!("O({}, 𝔽)", self.dimension))),
        }
    }
}

impl ToTurnMath for SpecialOrthogonalGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id.clone(),
            content: Box::new(MathNodeContent::Text(format!(
                "SO({}, 𝔽)",
                self.orthogonal.dimension
            ))),
        }
    }
}

impl ToTurnMath for UnitaryGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id.clone(),
            content: Box::new(MathNodeContent::Text(format!("U({}, ℂ)", self.dimension))),
        }
    }
}

impl ToTurnMath for SpecialUnitaryGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        MathNode {
            id: master_id.clone(),
            content: Box::new(MathNodeContent::Text(format!(
                "SU({}, ℂ)",
                self.unitary.dimension
            ))),
        }
    }
}
