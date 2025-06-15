use std::num::NonZeroI16;

use super::super::theorem::MathObject;
use super::super::{
    proof::{ProofForest, ProofNode},
    theorem::Theorem,
};
use crate::subjects::math::formalism::proof::ProofGoal;
use crate::turn_render::{
    BracketSize, BracketStyle, MathNode, MathNodeContent, MulSymbol, QuantificationNode,
    RefinedMulOrDivOperation, RelationOperatorNode, ToTurnMath,
};
// Importing ProofStatus
use super::super::proof::ProofStatus;

// Direct imports for MathRelation and MathExpression
use crate::subjects::math::formalism::expressions::MathExpression;
use crate::subjects::math::formalism::relations::{MathRelation, Quantification};

// Import the conversion trait
use crate::subjects::math::formalism::render::expressions::ToStructuredFormat;

use crate::turn_render::*;

use crate::subjects::math::formalism::proof::tactics::{
    CaseAnalysisBuilder, CaseResult, DecompositionMethod, InductionType, RewriteDirection, Tactic,
};
// use crate::subjects::math::theories::groups::theorems::{
//     prove_abelian_squared_criterion, prove_inverse_product_rule,
// };

impl ToSectionNode for ProofGoal {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        /// Create a comprehensive initial statement using generic methods
        let mut content = vec![];

        // Add variable explanations if available
        if !self.quantifiers.is_empty() {
            content.push(SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::StyledText {
                    text: "Variables: ".to_string(),
                    styles: vec![TextStyle::Bold],
                }],
                alignment: None,
            }));

            // list out quantifier
            for q in &self.quantifiers {
                let quantifier_word = match q.quantification {
                    Quantification::Universal => "For any",
                    Quantification::Existential => "There exists",
                    Quantification::UniqueExistential => "There exists a unique",
                };

                let var_description = if let Some(desc) = &q.description {
                    format!("• {} {} ({})", quantifier_word, q.variable, desc)
                } else {
                    format!("• {} {}", quantifier_word, q.variable)
                };

                content.push(SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Text(var_description)],
                    alignment: None,
                }));
            }
        }

        // list out value-variables
        if !self.value_variables.is_empty() {
            content.push(SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::StyledText {
                    text: "Value Variables: ".to_string(),
                    styles: vec![TextStyle::Bold],
                }],
                alignment: None,
            }));

            for v in &self.value_variables {
                content.push(SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::Math(MathNode {
                        id: format!("{}-value-variable", id_prefix),
                        content: Box::new(MathNodeContent::Relationship {
                            lhs: Box::new(v.name.to_turn_math("".to_string())),
                            rhs: Box::new(v.value.to_turn_math("".to_string())),
                            operator: RelationOperatorNode::IsEqual,
                        }),
                    })],
                    alignment: None,
                }));
            }
        }
        // Add theorem statement using structured types
        content.push(SectionContentNode::RichText(RichText {
            segments: vec![
                RichTextSegment::StyledText {
                    text: "Formal Statement: ".to_string(),
                    styles: vec![TextStyle::Bold],
                },
                RichTextSegment::Math(self.statement.to_turn_math("".to_string())),
            ],
            alignment: None,
        }));

        // // Add informal description
        // let informal_desc = self.create_generic_informal_description();
        // if !informal_desc.is_empty() {
        //     content.push(SectionContentNode::RichText(RichText {
        //         segments: vec![RichTextSegment::StyledText {
        //             text: format!("Informal Description: {}", informal_desc),
        //             styles: vec![TextStyle::Italic],
        //         }],
        //         alignment: None,
        //     }));
        // }

        Section {
            id: format!("{}-statement", id_prefix),
            title: None,
            content,
            metadata: vec![],
            display_options: None,
        }
    }
}

impl ToMathDocument for ProofGoal {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let main_section = self.to_section_node(&format!("{}-main", id_prefix));

        MathDocument {
            id: format!("{}-doc", id_prefix),
            content_type: MathDocumentType::ScientificPaper(ScientificPaperContent {
                title: "Proof Goal".to_string(),
                paper_type: PaperType::Research,
                venue: Some("Mathematical Proofs".to_string()),
                peer_reviewed: false,
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
                    venue: Some("Mathematical Proofs".to_string()),
                    doi: None,
                    keywords: vec!["proof-goal".to_string()],
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

impl ToSectionNode for Theorem {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        Section {
            id: format!("{}-main", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(self.name.clone())],
                alignment: None,
            }),
            content: {
                let mut content = vec![
                    // Theorem as structured mathematical content
                    SectionContentNode::StructuredMath(StructuredMathNode::TheoremLike {
                        kind: TheoremLikeKind::Theorem,
                        label: Some(self.id.clone()),
                        statement: TheoremStatement::Content(vec![SectionContentNode::RichText(
                            RichText {
                                segments: vec![RichTextSegment::Text(self.description.clone())],
                                alignment: None,
                            },
                        )]),
                        proof: Some(self.proofs.to_proof_display()),
                        abstraction_meta: None,
                    }),
                ];

                content
            },
            metadata: vec![
                ("type".to_string(), "theorem".to_string()),
                ("theorem_id".to_string(), self.id.clone()),
            ],
            display_options: None,
        }
    }
}

impl ToMathDocument for Theorem {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument {
        let main_section = self.to_section_node(&format!("{}-main", id_prefix));

        MathDocument {
            id: format!("{}-doc", id_prefix),
            content_type: MathDocumentType::ScientificPaper(ScientificPaperContent {
                title: self.name.clone(),
                paper_type: PaperType::Research,
                venue: Some("Mathematical Theorems".to_string()),
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
                    venue: Some("Mathematical Theorems".to_string()),
                    doi: None,
                    keywords: vec!["theorem".to_string()],
                },
                structure: DocumentStructure {
                    abstract_content: Some(Section {
                        id: format!("{}-abstract", id_prefix),
                        title: None,
                        content: vec![SectionContentNode::RichText(RichText {
                            segments: vec![RichTextSegment::Text(self.description.clone())],
                            alignment: None,
                        })],
                        metadata: vec![],
                        display_options: None,
                    }),
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
