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
    extract::Parametrizable,
    proof::{ContextEntry, DefinitionState},
    relations::{MathRelation, Quantification},
};

// Import the conversion trait

use crate::turn_render::*;

use crate::subjects::math::formalism::automation::registry::TheoremRegistry;

use crate::subjects::math::theories::groups::definitions::GroupExpression;

// use crate::subjects::math::theories::groups::theorems::{
//     prove_abelian_squared_criterion, prove_inverse_product_rule,
// };

impl ToSectionNode for ProofGoal {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        // Show the formal statement as Judgement
        let content = SectionContentNode::SecondOrderMath(SecondOrderMathNode::Judgement({
            // Process context once to create both non_quantifiers and quantifiers
            let mut non_quantifiers = Vec::new();
            let mut quantifiers = Vec::new();
            let mut current_universal_group = Vec::new();

            // Process each context entry
            for entry in &self.context {
                let variable_declaration = VariableDeclaration {
                    name: entry
                        .name
                        .to_turn_math(format!("context-name-{}", entry.name)),
                    type_info: match &entry.ty.data {
                        Parametrizable::Concrete(arc_expr) => arc_expr.to_rich_text(),
                        Parametrizable::Variable(id) => RichText {
                            segments: vec![RichTextSegment::Text(id.to_string())],
                            alignment: None,
                        },
                    },
                };

                // Check if this variable is quantified
                if let Some(quantifier) = self
                    .quantifiers
                    .iter()
                    .find(|q| q.variable_name == entry.name)
                {
                    // This is a quantified variable
                    match quantifier.quantification {
                        Quantification::Universal => {
                            // Add to current universal group
                            current_universal_group.push(variable_declaration);
                        }
                        Quantification::Existential => {
                            // First, finalize any pending universal group
                            if !current_universal_group.is_empty() {
                                quantifiers.push(QuantifiedVariableDeclarationGroup::ForAll(
                                    current_universal_group.clone(),
                                ));
                                current_universal_group.clear();
                            }
                            // Create single existential quantifier group
                            quantifiers.push(QuantifiedVariableDeclarationGroup::Exists(
                                variable_declaration,
                            ));
                        }
                        Quantification::UniqueExistential => {
                            // First, finalize any pending universal group
                            if !current_universal_group.is_empty() {
                                quantifiers.push(QuantifiedVariableDeclarationGroup::ForAll(
                                    current_universal_group.clone(),
                                ));
                                current_universal_group.clear();
                            }
                            // Create single unique existential quantifier group
                            quantifiers.push(QuantifiedVariableDeclarationGroup::UniqueExists(
                                variable_declaration,
                            ));
                        }
                    }
                } else {
                    // This is a non-quantified variable
                    non_quantifiers.push(variable_declaration);
                }
            }

            // Finalize any remaining universal group
            if !current_universal_group.is_empty() {
                quantifiers.push(QuantifiedVariableDeclarationGroup::ForAll(
                    current_universal_group,
                ));
            }

            Judgement {
                non_quantifiers,
                quantifiers,
                statement: self.statement.to_logical_node(),
            }
        }));

        Section {
            id: format!("{}-proof-goal", id_prefix),
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
        let mut content = vec![
            // Show the formal statement as Judgement
            self.proofs
                .initial_goal
                .to_section_node(&format!("{}-proof-goal", id_prefix)),
            // Show the description as commentary
            Section {
                id: format!("{}-description", id_prefix),
                title: Some(RichText {
                    segments: vec![RichTextSegment::Text("Description".to_string())],
                    alignment: None,
                }),
                content: SectionContentNode::RichText(RichText {
                    segments: vec![
                        RichTextSegment::StyledText {
                            text: "Description: ".to_string(),
                            styles: vec![TextStyle::Italic],
                        },
                        RichTextSegment::Text(self.description.clone()),
                    ],
                    alignment: None,
                }),
                metadata: vec![],
                display_options: None,
            },
        ];

        // Add proof structure if available
        if self.proofs.node_values().next().is_some() {
            content.push(self.proofs.to_section_node(&format!("{}-proof", id_prefix)));
        }

        Section {
            id: format!("{}-main", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(self.name.clone())],
                alignment: None,
            }),
            content: SectionContentNode::SubSection(content),
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
                        content: SectionContentNode::RichText(RichText {
                            segments: vec![RichTextSegment::Text(self.description.clone())],
                            alignment: None,
                        }),
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
    use crate::subjects::math::theories::groups::theorems::group_inverse_uniqueness;

    use super::*;

    #[test]
    fn test_simple_theorem_creation() {
        // Test just creating the theorem without rendering
        let _ = get_theorem_registry();
        let theorem = group_inverse_uniqueness();
        // Just test that we can create the theorem
        assert_eq!(theorem.id, "inverse_uniqueness");
    }

    #[test]
    fn test_theorem_to_section_node() {
        // Test just the to_section_node method
        let _ = get_theorem_registry();
        let theorem = group_inverse_uniqueness();
        let section = theorem.to_section_node("test_id");
        assert_eq!(section.id, "test_id-main");
    }

    #[test]
    fn test_proof_goal_to_section_node() {
        // Test just the ProofGoal::to_section_node method
        let _ = get_theorem_registry();
        let theorem = group_inverse_uniqueness();
        let proof_goal = &theorem.proofs.initial_goal;
        let section = proof_goal.to_section_node("test_id");
        assert_eq!(section.id, "test_id-proof-goal");
    }

    #[test]
    fn test_theorem_to_math_document() {
        // The call to get_theorem_registry() is enough to ensure axioms are registered.
        let _ = get_theorem_registry();
        let theorem = group_inverse_uniqueness();
        let math_document = theorem.to_math_document("test_id");
        assert_eq!(math_document.id, "test_id-doc");
    }
}
