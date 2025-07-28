use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use crate::turn_render::math_node::{
    BracketSize, BracketStyle, MathNode, MathNodeContent, ToTurnMath,
};
use crate::turn_render::*;

use crate::subjects::math::formalism::traits::abstraction_level::{
    AbstractionLevel, GetAbstractionLevel,
};
use crate::subjects::math::theories::groups::definitions::{
    GenericGroup, ModularAdditiveGroup, ModularMultiplicativeGroup,
};

impl ToSectionNode for ModularAdditiveGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        let title = format!("ℤ/{}", self.modulus);

        let content_nodes = vec![
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!("Modulus: {}", self.modulus))],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "Additive group of integers modulo n under addition.".to_string(),
                )],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!(
                    "This is a cyclic group of order {}.",
                    self.modulus
                ))],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-modularadditive-section", id_prefix),
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
                "ModularAdditiveGroupDefinition".to_string(),
            )],
            display_options: None,
        }
    }
}

impl ToMathDocument for ModularAdditiveGroup {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let level = self.level();

        // Use abstract notation for L1 groups
        let title = if level == AbstractionLevel::Level1 {
            "ℤ/nℤ".to_string()
        } else {
            format!("ℤ/{}", self.modulus)
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

impl ModularAdditiveGroup {
    pub fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!("ℤ/{}", self.modulus);
        vec![RichTextSegment::Text(name)]
    }

    pub fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!("ℤ/{}", self.modulus);
        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name)],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}-modularadditive-section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!(
                "View definition of {}-modularadditive-section",
                id_prefix
            )),
        }]
    }

    pub fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = format!("ℤ/{}", self.modulus);

        Section {
            id: format!("{}-main-modularadditive-section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!(
                    "The additive group ℤ/{} of integers modulo {} under addition. This is a cyclic group of order {}.",
                    self.modulus, self.modulus, self.modulus
                ))],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }
}

impl ToSectionNode for ModularMultiplicativeGroup {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let formalism_obj_level: AbstractionLevel = self.level();
        let title = format!("(ℤ/{})×", self.modulus);

        let content_nodes = vec![
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!("Modulus: {}", self.modulus))],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "Multiplicative group of units modulo n under multiplication.".to_string(),
                )],
                alignment: None,
            }),
            SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(
                    "This group consists of integers coprime to the modulus.".to_string(),
                )],
                alignment: None,
            }),
        ];

        Section {
            id: format!("{}-modularmultiplicative-section", id_prefix),
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
                "ModularMultiplicativeGroupDefinition".to_string(),
            )],
            display_options: None,
        }
    }
}

impl ToMathDocument for ModularMultiplicativeGroup {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let level = self.level();

        // Use abstract notation for L1 groups
        let title = if level == AbstractionLevel::Level1 {
            "(ℤ/nℤ)*".to_string()
        } else {
            format!("(ℤ/{})×", self.modulus)
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

impl ModularMultiplicativeGroup {
    pub fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!("(ℤ/{})×", self.modulus);
        vec![RichTextSegment::Text(name)]
    }

    pub fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        let name = format!("(ℤ/{})×", self.modulus);
        vec![RichTextSegment::Link {
            content: vec![RichTextSegment::Text(name)],
            target: LinkTarget::DefinitionId {
                term_id: format!("{}-modularmultiplicative-section", id_prefix),
                theory_context: Some("GroupTheory".to_string()),
            },
            tooltip: Some(format!(
                "View definition of {}-modularmultiplicative-section",
                id_prefix
            )),
        }]
    }

    pub fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        let title = format!("(ℤ/{})×", self.modulus);

        Section {
            id: format!("{}-main-modularmultiplicative-section", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(title.clone())],
                alignment: None,
            }),
            content: vec![SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!(
                    "The multiplicative group (ℤ/{})× of units modulo {} under multiplication. This group consists of integers coprime to the modulus.",
                    self.modulus, self.modulus
                ))],
                alignment: None,
            })],
            metadata: vec![("schema_level".to_string(), "1".to_string())],
            display_options: None,
        }
    }
}

impl ToTurnMath for ModularAdditiveGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        // Use proper mathematical notation ℤ/nℤ
        MathNode {
            id: master_id.clone(),
            content: Arc::new(MathNodeContent::Text(format!("ℤ/{}ℤ", self.modulus))),
        }
    }
}

impl ToTurnMath for ModularMultiplicativeGroup {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        // Use proper mathematical notation (ℤ/nℤ)×
        MathNode {
            id: master_id.clone(),
            content: Arc::new(MathNodeContent::Text(format!("(ℤ/{}ℤ)×", self.modulus))),
        }
    }
}
