use super::super::theorem::{Quantification, QuantifiedMathObject};
use super::super::{
    proof::{ProofForest, ProofNode},
    theorem::{ProofGoal, Theorem},
};
use crate::turn_render::{
    BracketSize, BracketStyle, MathNode, MathNodeContent, MulSymbol, QuantificationNode,
    RefinedMulOrDivOperation, RelationOperatorNode, ToTurnMath,
};
// Importing ProofStatus
use super::super::proof::ProofStatus;

// Direct imports for MathRelation and MathExpression
use crate::subjects::math::formalism::expressions::MathExpression;
use crate::subjects::math::formalism::relations::MathRelation;

// Import the conversion trait
use crate::subjects::math::formalism::render::expressions::ToStructuredFormat;

use crate::turn_render::*;

// Helper function to create placeholder MathNode for todo items
fn create_todo_math_node(description: &str, id: &str) -> MathNode {
    MathNode {
        id: id.to_string(),
        content: Box::new(MathNodeContent::Text(format!("TODO: {}", description))),
    }
}

impl ToSectionNode for Theorem {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        // Convert the theorem statement from MathRelation to MathNode using ToTurnMath
        let statement_math_node = self
            .goal
            .statement
            .to_turn_math(format!("{}-statement", id_prefix));

        Section {
            id: format!("{}-main", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(self.name.clone())],
                alignment: None,
            }),
            content: {
                let mut content = vec![
                    // Theorem as structured mathematical content - use the updated TheoremLike variant
                    SectionContentNode::StructuredMath(StructuredMathNode::TheoremLike {
                        kind: TheoremLikeKind::Theorem,
                        label: Some(self.id.clone()),
                        statement: TheoremStatement::Mathematical(statement_math_node),
                        proof: if !self.proofs.nodes.is_empty() {
                            Some(self.create_structured_proof_display())
                        } else {
                            None
                        },
                        abstraction_meta: None,
                    }),
                ];

                // Add simple description paragraph
                if !self.description.is_empty() {
                    content.push(SectionContentNode::Paragraph(ParagraphNode {
                        segments: vec![RichTextSegment::Text(self.description.clone())],
                        alignment: None,
                    }));
                }

                content
            },
            metadata: vec![
                ("type".to_string(), "theorem".to_string()),
                ("theorem_id".to_string(), self.id.clone()),
            ],
            display_options: None,
        }
    }

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
                        content: vec![SectionContentNode::Paragraph(ParagraphNode {
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

    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        vec![
            RichTextSegment::Text(format!("Theorem: {}", self.name)),
            RichTextSegment::Text(" - ".to_string()),
            RichTextSegment::Text(self.description.clone()),
        ]
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        vec![RichTextSegment::Text(self.name.clone())]
    }
}

impl Theorem {
    /// Create a ProofDisplayNode for the theorem
    fn create_structured_proof_display(&self) -> ProofDisplayNode {
        ProofDisplayNode {
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text("Proof.".to_string())],
                alignment: None,
            }),
            strategy: vec![], // Could add proof strategy here
            steps: self.convert_proof_steps_structured(),
            qed_symbol: Some("∎".to_string()),
        }
    }

    /// Convert proof steps from the theorem's proof forest to ProofStepNode structures
    fn convert_proof_steps_structured(&self) -> Vec<ProofStepNode> {
        let mut steps = Vec::new();

        // Convert each proof node to a structured proof step
        for (node_id, node) in &self.proofs.nodes {
            // Create a simple proof step using the original ProofStepNode structure
            let step = ProofStepNode::Statement {
                claim: vec![RichTextSegment::Text(format!(
                    "Goal: {}",
                    node_id // Simplified representation
                ))],
                justification: vec![RichTextSegment::Text("Direct proof".to_string())],
            };

            steps.push(step);
        }

        // If no steps, add a placeholder
        if steps.is_empty() {
            steps.push(ProofStepNode::Statement {
                claim: vec![RichTextSegment::Text("Proof completed.".to_string())],
                justification: vec![RichTextSegment::Text("by construction".to_string())],
            });
        }

        steps
    }
}

mod tests {
    use serde_json::to_value;

    use crate::subjects::math::theories::theorems::{
        prove_abelian_squared_criterion, prove_inverse_product_rule,
    };

    use super::*;

    #[test]
    fn test_theorem_render() {
        let theorem = prove_inverse_product_rule();
        let section = theorem.to_section_node("theorem_id");
        println!("{:#?}", to_value(&section));
    }
}
