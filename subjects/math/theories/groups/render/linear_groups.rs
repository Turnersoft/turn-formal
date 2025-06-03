use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use crate::turn_render::math_node::{
    BracketSize, BracketStyle, MathNode, MathNodeContent, ToTurnMath,
};
use crate::turn_render::section_node::{
    AbstractionMetadata, AcademicMetadata, ContentMetadata, DocumentRelationships,
    DocumentStructure, LinkTarget, MathDocument, MathematicalContent, MathematicalContentType,
    PaperType, ParagraphNode, RichTextSegment, ScientificPaperContent, Section, SectionContentNode,
    StructuredMathNode, ToSectionNode,
};

use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};
use crate::subjects::math::theories::groups::definitions::{
    GeneralLinearGroup, GenericGroup, OrthogonalGroup, SpecialLinearGroup, SpecialOrthogonalGroup,
    SpecialUnitaryGroup, UnitaryGroup,
};

impl ToSectionNode for GeneralLinearGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();

        let title = format!(
            "GL({}, {})",
            self.dimension,
            "𝔽" // Simple field placeholder instead of calling content_as_text
        );

        let content_nodes = vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(format!(
                    "Dimension: {}",
                    self.dimension
                ))],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(format!(
                    "Field: {}",
                    "𝔽" // Simple field placeholder instead of calling content_as_text
                ))],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "General linear group of invertible matrices over the given field.".to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-generallinear-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathNode::Definition {
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
                "GeneralLinearGroupDefinition".to_string(),
            )],
            display_options: None,
        }
    }

    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!(
            "GL({}, {})",
            self.dimension,
            "𝔽" // Simple field placeholder instead of calling content_as_text
        );
        vec![RichTextSegment::Text(name)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!(
            "GL({}, {})",
            self.dimension,
            "𝔽" // Simple field placeholder instead of calling content_as_text
        );
        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name)],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}-generallinear-section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!(
                "View definition of {}-generallinear-section",
                id_prefix
            )),
        }]
    }

    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let level = self.level();

        // Use abstract notation for L1 groups
        let title = if level == AbstractionLevel::Level1 {
            vec![
                RichTextSegment::Text("General Linear Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("GL(n, F)".to_string())),
                }),
            ]
        } else {
            vec![RichTextSegment::Text(format!(
                "GL({}, {})",
                self.dimension, "𝔽"
            ))]
        };

        let title_text = title
            .iter()
            .map(|seg| match seg {
                RichTextSegment::Text(t) => t.clone(),
                RichTextSegment::Math(_) => "GL(n,F)".to_string(),
                _ => "".to_string(),
            })
            .collect::<String>();

        let main_section = self.to_section_node(id_prefix);

        MathDocument {
            id: format!("{}-doc", id_prefix),
            content_type: MathematicalContentType::ScientificPaper(ScientificPaperContent {
                title: title_text,
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

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = format!(
            "GL({}, {})",
            self.dimension,
            "𝔽" // Simple field placeholder instead of calling content_as_text
        );

        Section {
            id: format!("{}-main-generallinear-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::Paragraph(ParagraphNode {
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

impl ToTurnMath for GeneralLinearGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let level = self.level();

        // Use abstract notation for L1 groups
        if level == AbstractionLevel::Level1 {
            MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("GL(n, F)".to_string())),
            }
        } else {
            MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text(format!(
                    "GL({}, {})",
                    self.dimension, "𝔽"
                ))),
            }
        }
    }
}

impl ToSectionNode for SpecialLinearGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();

        let title = format!(
            "SL({}, {})",
            self.general_linear.dimension,
            "𝔽" // Simple field placeholder instead of calling content_as_text
        );

        let content_nodes = vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(format!(
                    "Dimension: {}",
                    self.general_linear.dimension
                ))],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(format!(
                    "Field: {}",
                    "𝔽" // Simple field placeholder instead of calling content_as_text
                ))],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "Special linear group of matrices with determinant 1 over the given field."
                        .to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-speciallinear-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathNode::Definition {
                    term_display: vec![RichTextSegment::Text(title.clone())],
                    formal_term: Some(
                        self.general_linear
                            .core
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

    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!(
            "SL({}, {})",
            self.general_linear.dimension,
            "𝔽" // Simple field placeholder instead of calling content_as_text
        );
        vec![RichTextSegment::Text(name)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!(
            "SL({}, {})",
            self.general_linear.dimension,
            "𝔽" // Simple field placeholder instead of calling content_as_text
        );
        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name)],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}-speciallinear-section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!(
                "View definition of {}-speciallinear-section",
                id_prefix
            )),
        }]
    }

    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let level = self.level();

        // Use abstract notation for L1 groups
        let title = if level == AbstractionLevel::Level1 {
            vec![
                RichTextSegment::Text("Special Linear Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("SL(n, F)".to_string())),
                }),
            ]
        } else {
            vec![RichTextSegment::Text(format!(
                "SL({}, {})",
                self.general_linear.dimension, "𝔽"
            ))]
        };

        let title_text = title
            .iter()
            .map(|seg| match seg {
                RichTextSegment::Text(t) => t.clone(),
                RichTextSegment::Math(_) => "SL(n,F)".to_string(),
                _ => "".to_string(),
            })
            .collect::<String>();

        let main_section = self.to_section_node(id_prefix);

        MathDocument {
            id: format!("{}-doc", id_prefix),
            content_type: MathematicalContentType::ScientificPaper(ScientificPaperContent {
                title: title_text,
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

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = format!(
            "SL({}, {})",
            self.general_linear.dimension,
            "𝔽" // Simple field placeholder instead of calling content_as_text
        );

        Section {
            id: format!("{}-main-speciallinear-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(format!(
                    "The special linear group SL({}, {}) consists of all {}×{} matrices with determinant 1 over the field {}.",
                    self.general_linear.dimension,
                    "𝔽", // Simple field placeholder instead of calling content_as_text
                    self.general_linear.dimension,
                    self.general_linear.dimension,
                    "𝔽" // Simple field placeholder instead of calling content_as_text
                ))],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }
}

impl ToTurnMath for SpecialLinearGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let level = self.level();

        // Use abstract notation for L1 groups
        if level == AbstractionLevel::Level1 {
            MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("SL(n, F)".to_string())),
            }
        } else {
            MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text(format!(
                    "SL({}, {})",
                    self.general_linear.dimension, "𝔽"
                ))),
            }
        }
    }
}

impl ToSectionNode for OrthogonalGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        let title = format!("O({})", self.dimension);

        let content_nodes = vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(format!(
                    "Dimension: {}",
                    self.dimension
                ))],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "Orthogonal group preserving the standard inner product.".to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-orthogonal-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathNode::Definition {
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
            metadata: vec![("type".to_string(), "OrthogonalGroupDefinition".to_string())],
            display_options: None,
        }
    }

    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!("O({})", self.dimension);
        vec![RichTextSegment::Text(name)]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!("O({})", self.dimension);
        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name)],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}-orthogonal-section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!(
                "View definition of {}-orthogonal-section",
                id_prefix
            )),
        }]
    }

    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let level = self.level();

        // Use abstract notation for L1 groups
        let title = if level == AbstractionLevel::Level1 {
            vec![
                RichTextSegment::Text("Orthogonal Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("O(n)".to_string())),
                }),
            ]
        } else {
            vec![RichTextSegment::Text(format!("O({})", self.dimension))]
        };

        let title_text = title
            .iter()
            .map(|seg| match seg {
                RichTextSegment::Text(t) => t.clone(),
                RichTextSegment::Math(_) => "O(n)".to_string(),
                _ => "".to_string(),
            })
            .collect::<String>();

        let main_section = self.to_section_node(id_prefix);

        MathDocument {
            id: format!("{}-doc", id_prefix),
            content_type: MathematicalContentType::ScientificPaper(ScientificPaperContent {
                title: title_text,
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

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = format!("O({})", self.dimension);

        Section {
            id: format!("{}-main-orthogonal-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(format!(
                    "The orthogonal group O({}) consists of all {}×{} orthogonal matrices that preserve the standard inner product.",
                    self.dimension, self.dimension, self.dimension
                ))],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }
}

impl ToTurnMath for OrthogonalGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let level = self.level();

        // Use abstract notation for L1 groups
        if level == AbstractionLevel::Level1 {
            MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("O(n)".to_string())),
            }
        } else {
            MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text(format!("O({})", self.dimension))),
            }
        }
    }
}

impl ToSectionNode for SpecialOrthogonalGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        let title = format!("SO({})", self.orthogonal.dimension);

        let content_nodes = vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(format!(
                    "Dimension: {}",
                    self.orthogonal.dimension
                ))],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "Special orthogonal group of orthogonal matrices with determinant 1."
                        .to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-specialorthogonal-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathNode::Definition {
                    term_display: vec![RichTextSegment::Text(title.clone())],
                    formal_term: Some(
                        self.orthogonal
                            .core
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

    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let level = self.level();

        // Use abstract notation for L1 groups
        let title = if level == AbstractionLevel::Level1 {
            vec![
                RichTextSegment::Text("Special Orthogonal Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("SO(n)".to_string())),
                }),
            ]
        } else {
            vec![RichTextSegment::Text(format!(
                "SO({})",
                self.orthogonal.dimension
            ))]
        };

        let title_text = title
            .iter()
            .map(|seg| match seg {
                RichTextSegment::Text(t) => t.clone(),
                RichTextSegment::Math(_) => "SO(n)".to_string(),
                _ => "".to_string(),
            })
            .collect::<String>();

        let main_section = self.to_section_node(id_prefix);

        MathDocument {
            id: format!("{}-doc", id_prefix),
            content_type: MathematicalContentType::ScientificPaper(ScientificPaperContent {
                title: title_text,
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

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = format!("SO({})", self.orthogonal.dimension);

        Section {
            id: format!("{}-main-specialorthogonal-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(format!(
                    "The special orthogonal group SO({}) consists of all {}×{} orthogonal matrices with determinant 1.",
                    self.orthogonal.dimension, self.orthogonal.dimension, self.orthogonal.dimension
                ))],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }
}

impl ToTurnMath for SpecialOrthogonalGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let level = self.level();

        // Use abstract notation for L1 groups
        if level == AbstractionLevel::Level1 {
            MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("SO(n)".to_string())),
            }
        } else {
            MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text(format!(
                    "SO({})",
                    self.orthogonal.dimension
                ))),
            }
        }
    }
}

impl ToSectionNode for UnitaryGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        let title = format!("U({})", self.dimension);

        let content_nodes = vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(format!(
                    "Dimension: {}",
                    self.dimension
                ))],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "Unitary group of unitary matrices over the complex numbers.".to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-unitary-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathNode::Definition {
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
            metadata: vec![("type".to_string(), "UnitaryGroupDefinition".to_string())],
            display_options: None,
        }
    }

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

    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let level = self.level();

        // Use abstract notation for L1 groups
        let title = if level == AbstractionLevel::Level1 {
            vec![
                RichTextSegment::Text("Unitary Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("U(n)".to_string())),
                }),
            ]
        } else {
            vec![RichTextSegment::Text(format!("U({})", self.dimension))]
        };

        let title_text = title
            .iter()
            .map(|seg| match seg {
                RichTextSegment::Text(t) => t.clone(),
                RichTextSegment::Math(_) => "U(n)".to_string(),
                _ => "".to_string(),
            })
            .collect::<String>();

        let main_section = self.to_section_node(id_prefix);

        MathDocument {
            id: format!("{}-doc", id_prefix),
            content_type: MathematicalContentType::ScientificPaper(ScientificPaperContent {
                title: title_text,
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

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = format!("U({})", self.dimension);

        Section {
            id: format!("{}-main-unitary-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(format!(
                    "The unitary group U({}) consists of all {}×{} unitary matrices. These are complex matrices whose conjugate transpose equals their inverse.",
                    self.dimension, self.dimension, self.dimension
                ))],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }
}

impl ToTurnMath for UnitaryGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let level = self.level();

        // Use abstract notation for L1 groups
        if level == AbstractionLevel::Level1 {
            MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("U(n)".to_string())),
            }
        } else {
            MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text(format!("U({})", self.dimension))),
            }
        }
    }
}

impl ToSectionNode for SpecialUnitaryGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        let title = format!("SU({})", self.unitary.dimension);

        let content_nodes = vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(format!(
                    "Dimension: {}",
                    self.unitary.dimension
                ))],
                alignment: None,
            }),
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "Special unitary group of unitary matrices with determinant 1.".to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-specialunitary-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::StructuredMath(
                StructuredMathNode::Definition {
                    term_display: vec![RichTextSegment::Text(title.clone())],
                    formal_term: Some(
                        self.unitary
                            .core
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

    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let level = self.level();

        // Use abstract notation for L1 groups
        let title = if level == AbstractionLevel::Level1 {
            vec![
                RichTextSegment::Text("Special Unitary Group ".to_string()),
                RichTextSegment::Math(MathNode {
                    id: format!("{}-title-math", id_prefix),
                    content: Box::new(MathNodeContent::Text("SU(n)".to_string())),
                }),
            ]
        } else {
            vec![RichTextSegment::Text(format!(
                "SU({})",
                self.unitary.dimension
            ))]
        };

        let title_text = title
            .iter()
            .map(|seg| match seg {
                RichTextSegment::Text(t) => t.clone(),
                RichTextSegment::Math(_) => "SU(n)".to_string(),
                _ => "".to_string(),
            })
            .collect::<String>();

        let main_section = self.to_section_node(id_prefix);

        MathDocument {
            id: format!("{}-doc", id_prefix),
            content_type: MathematicalContentType::ScientificPaper(ScientificPaperContent {
                title: title_text,
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

    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = format!("SU({})", self.unitary.dimension);

        Section {
            id: format!("{}-main-specialunitary-section", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(format!(
                    "The special unitary group SU({}) consists of all {}×{} unitary matrices with determinant 1.",
                    self.unitary.dimension, self.unitary.dimension, self.unitary.dimension
                ))],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }
}

impl ToTurnMath for SpecialUnitaryGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        let level = self.level();

        // Use abstract notation for L1 groups
        if level == AbstractionLevel::Level1 {
            MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("SU(n)".to_string())),
            }
        } else {
            MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text(format!(
                    "SU({})",
                    self.unitary.dimension
                ))),
            }
        }
    }
}
