use std::{num::NonZeroI16, sync::Arc};

use super::super::objects::MathObject;
use super::super::{
    proof::{ProofForest, ProofNode},
    theorem::Theorem,
};
use crate::subjects::math::formalism::proof::ProofGoal;
use crate::turn_render::{
    BracketSize, BracketStyle, MathNode, MathNodeContent, MulSymbol, QuantificationNode,
    RefinedMulOrDivOperation, RelationOperatorNode, ToTurnMath,
};

use crate::subjects::math::formalism::{
    expressions::MathExpression,
    proof::{ContextEntry, DefinitionState},
    relations::{MathRelation, Quantification},
};

// Import the conversion trait

use crate::turn_render::*;

use crate::subjects::math::formalism::automation::registry::TheoremRegistry;

// use crate::subjects::math::theories::groups::theorems::{
//     prove_abelian_squared_criterion, prove_inverse_product_rule,
// };

impl ToSectionNode for ProofGoal {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let mut content = vec![];

        // RENDER CONTEXT
        if !self.context.is_empty() {
            content.push(SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::StyledText {
                    text: "Context:".to_string(),
                    styles: vec![TextStyle::Bold],
                }],
                alignment: None,
            }));

            for entry in &self.context {
                let mut segments = vec![RichTextSegment::Text("• ".to_string())];

                // Render name
                segments.push(RichTextSegment::StyledText {
                    text: format!("{:?}: ", entry.name),
                    styles: vec![TextStyle::Italic],
                });

                // Render type
                segments.push(RichTextSegment::Math(
                    entry
                        .ty
                        .to_turn_math(format!("{}-type-{:?}", id_prefix, entry.name)),
                ));

                // Render definition if it exists
                match &entry.definition {
                    DefinitionState::Separate(def) => {
                        segments.push(RichTextSegment::Text(" := ".to_string()));
                        segments.push(RichTextSegment::Math(
                            def.to_turn_math(format!("{}-def-{:?}", id_prefix, entry.name)),
                        ));
                    }
                    DefinitionState::Inlined => {
                        segments.push(RichTextSegment::Text(" (defined inline)".to_string()));
                    }
                    DefinitionState::ContainedInType => {
                        segments.push(RichTextSegment::Text(
                            " (value is self-contained)".to_string(),
                        ));
                    }
                    DefinitionState::Abstract => { /* Do nothing for abstract entries */ }
                }

                content.push(SectionContentNode::RichText(RichText {
                    segments,
                    alignment: None,
                }));
            }
        }

        // RENDER QUANTIFIERS
        if !self.quantifiers.is_empty() {
            // Create formal mathematical representation using MathNode
            let quantifier_nodes: Vec<MathNode> = self
                .quantifiers
                .iter()
                .map(|q| {
                    MathNode {
                        id: format!("{}-quantifier-{:?}", id_prefix, q.variable_name),
                        content: Arc::new(MathNodeContent::QuantifiedExpression {
                            quantifier: match q.quantification {
                                Quantification::Universal => QuantificationNode::Universal,
                                Quantification::Existential => QuantificationNode::Existential,
                                Quantification::UniqueExistential => {
                                    QuantificationNode::UniqueExistential
                                }
                            },
                            variables: vec![MathNode {
                                id: format!("{}-var-{:?}", id_prefix, q.variable_name),
                                content: Arc::new(MathNodeContent::Identifier(
                                    q.variable_name.clone(),
                                )),
                            }],
                            domain: None, // Could be enhanced to show domain if available
                            predicate: None, // Could be enhanced to show constraints if available
                        }),
                    }
                })
                .collect();

            // Add formal mathematical quantifier representation
            // Since there's no Group variant, we'll add each quantifier separately
            for quantifier_node in quantifier_nodes {
                content.push(SectionContentNode::RichText(RichText {
                    segments: vec![
                        RichTextSegment::StyledText {
                            text: "Quantifier: ".to_string(),
                            styles: vec![TextStyle::Bold],
                        },
                        RichTextSegment::Math(quantifier_node),
                    ],
                    alignment: None,
                }));
            }

            // Also provide informal text representation for accessibility
            let quantifier_text = self
                .quantifiers
                .iter()
                .map(|q| format!("{:?} {:?}", q.quantification, q.variable_name))
                .collect::<Vec<_>>()
                .join(", ");

            content.push(SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::StyledText {
                    text: format!("(In words: {})", quantifier_text),
                    styles: vec![TextStyle::Italic],
                }],
                alignment: None,
            }));
        }

        // RENDER STATEMENT
        content.push(SectionContentNode::RichText(RichText {
            segments: vec![
                RichTextSegment::StyledText {
                    text: "⊢ ".to_string(),
                    styles: vec![],
                }, // Turnstile symbol for "proves"
                RichTextSegment::Math(
                    self.statement
                        .data
                        .to_turn_math(format!("{}-stmt", id_prefix)),
                ),
            ],
            alignment: None,
        }));

        Section {
            id: format!("{}-statement", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text("Proof Goal".to_string())],
                alignment: None,
            }),
            content,
            metadata: vec![],
            display_options: None,
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
                        statement: TheoremStatement::Content(vec![
                            // Show the formal statement
                            SectionContentNode::RichText(RichText {
                                segments: vec![
                                    RichTextSegment::StyledText {
                                        text: "Statement: ".to_string(),
                                        styles: vec![TextStyle::Bold],
                                    },
                                    RichTextSegment::Math(
                                        self.proofs
                                            .initial_goal
                                            .statement
                                            .data
                                            .to_turn_math("theorem-statement".to_string()),
                                    ),
                                ],
                                alignment: None,
                            }),
                            // Show quantifiers if any
                            if !self.proofs.initial_goal.quantifiers.is_empty() {
                                SectionContentNode::RichText(RichText {
                                    segments: vec![RichTextSegment::StyledText {
                                        text: "Quantifiers: ".to_string(),
                                        styles: vec![TextStyle::Bold],
                                    }],
                                    alignment: None,
                                })
                            } else {
                                SectionContentNode::RichText(RichText {
                                    segments: vec![],
                                    alignment: None,
                                })
                            },
                            // Show value variables if any
                            {
                                let variable_assumptions: Vec<_> = self
                                    .proofs
                                    .initial_goal
                                    .context
                                    .iter()
                                    .filter(|entry| {
                                        matches!(entry.definition, DefinitionState::Abstract)
                                    })
                                    .collect();

                                if !variable_assumptions.is_empty() {
                                    SectionContentNode::RichText(RichText {
                                        segments: vec![RichTextSegment::StyledText {
                                            text: "Variables: ".to_string(),
                                            styles: vec![TextStyle::Bold],
                                        }],
                                        alignment: None,
                                    })
                                } else {
                                    SectionContentNode::RichText(RichText {
                                        segments: vec![],
                                        alignment: None,
                                    })
                                }
                            },
                            // Show the description as commentary
                            SectionContentNode::RichText(RichText {
                                segments: vec![
                                    RichTextSegment::StyledText {
                                        text: "Description: ".to_string(),
                                        styles: vec![TextStyle::Italic],
                                    },
                                    RichTextSegment::Text(self.description.clone()),
                                ],
                                alignment: None,
                            }),
                        ]),
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

#[cfg(test)]
mod tests {
    use crate::subjects::math::formalism::automation::registry::get_theorem_registry;
    use crate::subjects::math::theories::groups::theorems::prove_inverse_uniqueness;

    use super::*;

    #[test]
    fn test_theorem_to_math_document() {
        // The call to get_theorem_registry() is enough to ensure axioms are registered.
        let _ = get_theorem_registry();
        let theorem = prove_inverse_uniqueness();
        let math_document = theorem.to_math_document("test_id");
        assert_eq!(math_document.id, "test_id_doc");
    }
}
