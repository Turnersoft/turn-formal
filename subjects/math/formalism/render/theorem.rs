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

// Add imports for section_node types including the new structured proof types
use crate::turn_render::section_node::{
    AcademicMetadata, BindingType, ContentMetadata, DocumentRelationships, DocumentStructure, Goal,
    GoalType, MathematicalContent, MathematicalContentType, NumberType, OperationType, PaperType,
    ParagraphNode, ProofCase, ProofDisplayNode, ProofStepNode, ProofStepStatus, QuantifiedObject,
    QuantifierType, RichTextSegment, ScientificPaperContent, Section, SectionContentNode, Step,
    StructuredMathNode, StructuredProofDisplayNode, Tactic, TheoremLikeKind, TheoremStatement,
    ToSectionNode, VariableBinding,
};

use crate::turn_render::math_node::IdentifierNode;

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

    fn to_math_document(&self, id_prefix: &str) -> MathematicalContent {
        let main_section = self.to_section_node(&format!("{}-main", id_prefix));

        MathematicalContent {
            id: format!("{}-doc", id_prefix),
            content_type: MathematicalContentType::ScientificPaper(ScientificPaperContent {
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
    /// Create a StructuredProofDisplayNode for the theorem
    fn create_structured_proof_display(&self) -> StructuredProofDisplayNode {
        StructuredProofDisplayNode {
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text("Proof.".to_string())],
                alignment: None,
            }),
            strategy: vec![], // Could add proof strategy here
            steps: self.convert_proof_steps_structured(),
            qed_symbol: Some("∎".to_string()),
        }
    }

    /// Convert proof steps from the theorem's proof forest to Step structures
    fn convert_proof_steps_structured(&self) -> Vec<Step> {
        let mut steps = Vec::new();

        // Convert each proof node to a structured proof step
        for (node_id, node) in &self.proofs.nodes {
            // Create a simple structured proof step
            let step = Step::Statement {
                goal: Goal {
                    quantified_objects: vec![],
                    variable_bindings: vec![],
                    statement: node
                        .state
                        .statement
                        .to_turn_math(format!("{}-goal", node_id)),
                    goal_type: GoalType::Prove,
                },
                tactic: Tactic::DirectProof,
                status: match node.status {
                    ProofStatus::Complete => ProofStepStatus::Complete,
                    ProofStatus::InProgress => ProofStepStatus::InProgress,
                    ProofStatus::Todo => ProofStepStatus::Todo,
                    ProofStatus::Wip => ProofStepStatus::WorkInProgress,
                    ProofStatus::Abandoned => ProofStepStatus::Abandoned,
                },
            };

            steps.push(step);
        }

        // If no steps, add a placeholder
        if steps.is_empty() {
            steps.push(Step::Statement {
                goal: Goal {
                    quantified_objects: vec![],
                    variable_bindings: vec![],
                    statement: create_todo_math_node("Proof completed", "proof-todo"),
                    goal_type: GoalType::Prove,
                },
                tactic: Tactic::DirectProof,
                status: ProofStepStatus::Complete,
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
