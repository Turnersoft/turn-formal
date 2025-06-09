use super::super::theorem::{MathObject, Quantification, QuantifiedMathObject};
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

use crate::subjects::math::formalism::proof::tactics::{
    CaseAnalysisBuilder, CaseResult, DecompositionMethod, InductionType, RewriteDirection, Tactic,
};
// use crate::subjects::math::theories::groups::theorems::{
//     prove_abelian_squared_criterion, prove_inverse_product_rule,
// };

// Helper function to create placeholder MathNode for todo items
fn create_todo_math_node(description: &str, id: &str) -> MathNode {
    MathNode {
        id: id.to_string(),
        content: Box::new(MathNodeContent::Text(format!("TODO: {}", description))),
    }
}

impl ToSectionNode for Theorem {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        // Create a simple formal statement without the broken ToTurnMath
        let statement_text = self.format_relation(&self.goal.statement);
        let statement_math_node = MathNode {
            id: format!("{}-statement", id_prefix),
            content: Box::new(MathNodeContent::Text(statement_text)),
        };

        Section {
            id: format!("{}-main", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(self.name.clone())],
                alignment: None,
            }),
            content: {
                let mut content = vec![
                    // Theorem as structured mathematical content
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

        // Add comprehensive initial statement first
        steps.push(self.create_comprehensive_initial_statement());

        // Start from root nodes and traverse in order
        for root_id in &self.proofs.roots {
            self.collect_proof_steps_from_node(root_id, &mut steps);
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

    /// Recursively collect proof steps from a node and its children
    fn collect_proof_steps_from_node(&self, node_id: &str, steps: &mut Vec<ProofStepNode>) {
        if let Some(node) = self.proofs.nodes.get(node_id) {
            // Create meaningful proof step based on the tactic used
            let step = match &node.tactic {
                Some(tactic) => {
                    use crate::subjects::math::formalism::proof::tactics::Tactic as ProofTactic;
                    match tactic {
                        ProofTactic::Intro {
                            name,
                            expression,
                            view,
                        } => {
                            let view_text = if view.is_some() { " (with view)" } else { "" };
                            let claim_text = match expression {
                                MathExpression::Relation(relation) => {
                                    match relation.as_ref() {
                                        MathRelation::Equal { left, right, .. } => {
                                            // Generic equality premise
                                            format!(
                                                "Introduce premise: {} = {}{}",
                                                self.format_expression(left),
                                                self.format_expression(right),
                                                view_text
                                            )
                                        }
                                        _ => {
                                            format!(
                                                "Introduce premise: {}{}",
                                                self.format_relation(relation),
                                                view_text
                                            )
                                        }
                                    }
                                }
                                _ => {
                                    // For other expressions, try to give meaningful context
                                    match expression {
                                        MathExpression::Var(id) => {
                                            match id {
                                                crate::subjects::math::formalism::expressions::Identifier::E(_) => {
                                                    format!("Introduce assumptions as defined in theorem statement{}", view_text)
                                                }
                                                _ => {
                                                    format!("Introduce assumptions: {}{}", self.format_identifier(id), view_text)
                                                }
                                            }
                                        }
                                        _ => {
                                            format!("Introduce assumptions: {}{}", self.format_expression(expression), view_text)
                                        }
                                    }
                                }
                            };

                            ProofStepNode::Statement {
                                claim: vec![RichTextSegment::Text(claim_text)],
                                justification: vec![RichTextSegment::Text(
                                    "Introduction rule".to_string(),
                                )],
                            }
                        }
                        ProofTactic::TheoremApplication {
                            theorem_id,
                            instantiation,
                            ..
                        } => {
                            // Generic theorem application description
                            let instantiation_text = if instantiation.is_empty() {
                                String::new()
                            } else {
                                let vars: Vec<String> = instantiation
                                    .iter()
                                    .map(|(id, expr)| {
                                        format!(
                                            "{} := {}",
                                            self.format_identifier(id),
                                            self.format_expression(expr)
                                        )
                                    })
                                    .collect();
                                format!(" with {}", vars.join(", "))
                            };
                            let description =
                                format!("Apply theorem '{}'{}", theorem_id, instantiation_text);

                            ProofStepNode::Statement {
                                claim: vec![RichTextSegment::Text(description)],
                                justification: vec![RichTextSegment::Text(
                                    "By previously established theorem".to_string(),
                                )],
                            }
                        }
                        ProofTactic::Substitution {
                            target,
                            replacement,
                            ..
                        } => {
                            let target_desc = self.format_expression(target);
                            let replacement_desc = self.format_expression(replacement);

                            // Generic substitution description
                            let description = format!(
                                "Substitute {} with {} (by established equality)",
                                target_desc, replacement_desc
                            );

                            ProofStepNode::Statement {
                                claim: vec![RichTextSegment::Text(description)],
                                justification: vec![RichTextSegment::Text(
                                    "Using established equality".to_string(),
                                )],
                            }
                        }

                        _ => {
                            // Fallback for other tactics
                            ProofStepNode::Statement {
                                claim: vec![RichTextSegment::Text(format!("Apply {:?}", tactic))],
                                justification: vec![RichTextSegment::Text(
                                    "Tactic application".to_string(),
                                )],
                            }
                        }
                    }
                }
                None => {
                    // No tactic - show meaningful goal state information
                    let goal_text = self.format_relation(&node.state.statement);
                    ProofStepNode::Statement {
                        claim: vec![RichTextSegment::Text(format!(
                            "Initial goal: {}",
                            goal_text
                        ))],
                        justification: vec![RichTextSegment::Text("Starting state".to_string())],
                    }
                }
            };

            // If there's a tactic, modify the step to include the resulting goal
            let final_step = if node.tactic.is_some() {
                match step {
                    ProofStepNode::Statement {
                        claim,
                        justification,
                    } => {
                        // Generic step with tactic description - no domain-specific hardcoding
                        let tactic_text = claim
                            .iter()
                            .filter_map(|seg| match seg {
                                RichTextSegment::Text(t) => Some(t.clone()),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join("");

                        let mut enhanced_content =
                            vec![SectionContentNode::Paragraph(ParagraphNode {
                                segments: vec![
                                    RichTextSegment::StyledText {
                                        text: "Step: ".to_string(),
                                        styles: vec![TextStyle::Bold],
                                    },
                                    RichTextSegment::Text(tactic_text.clone()),
                                ],
                                alignment: None,
                            })];

                        // Add generic transformation details based purely on tactic structure
                        if let Some(tactic) = &node.tactic {
                            use crate::subjects::math::formalism::proof::tactics::Tactic as ProofTactic;
                            match tactic {
                                ProofTactic::Substitution {
                                    target,
                                    replacement,
                                    ..
                                } => {
                                    enhanced_content.push(SectionContentNode::Paragraph(
                                        ParagraphNode {
                                            segments: vec![
                                                RichTextSegment::StyledText {
                                                    text: "Result: ".to_string(),
                                                    styles: vec![TextStyle::Bold],
                                                },
                                                RichTextSegment::Text(format!(
                                                    "{} → {}",
                                                    self.format_expression(target),
                                                    self.format_expression(replacement)
                                                )),
                                            ],
                                            alignment: None,
                                        },
                                    ));
                                }
                                ProofTactic::TheoremApplication { theorem_id, .. } => {
                                    enhanced_content.push(SectionContentNode::Paragraph(
                                        ParagraphNode {
                                            segments: vec![
                                                RichTextSegment::StyledText {
                                                    text: "Applied: ".to_string(),
                                                    styles: vec![TextStyle::Bold],
                                                },
                                                RichTextSegment::Text(theorem_id.clone()),
                                            ],
                                            alignment: None,
                                        },
                                    ));
                                }
                                _ => {
                                    // For other tactics, just show the resulting goal generically
                                    enhanced_content.push(SectionContentNode::Paragraph(
                                        ParagraphNode {
                                            segments: vec![
                                                RichTextSegment::StyledText {
                                                    text: "Result: ".to_string(),
                                                    styles: vec![TextStyle::Bold],
                                                },
                                                RichTextSegment::Text(
                                                    self.format_relation(&node.state.statement),
                                                ),
                                            ],
                                            alignment: None,
                                        },
                                    ));
                                }
                            }
                        }

                        ProofStepNode::Elaboration(enhanced_content)
                    }
                    _ => step,
                }
            } else {
                step
            };

            // Only add the step if it's not a duplicate of the previous step
            let should_add_step = if let Some(last_step) = steps.last() {
                !self.steps_are_equivalent(&final_step, last_step)
            } else {
                true
            };

            if should_add_step {
                steps.push(final_step);
            }

            // Recursively process children
            for child_id in &node.children {
                self.collect_proof_steps_from_node(child_id, steps);
            }
        }
    }

    /// Check if two proof steps are essentially equivalent (to avoid duplicates)
    fn steps_are_equivalent(&self, step1: &ProofStepNode, step2: &ProofStepNode) -> bool {
        match (step1, step2) {
            (ProofStepNode::Elaboration(content1), ProofStepNode::Elaboration(content2)) => {
                // Compare the last paragraph (which contains the goal)
                if let (Some(last1), Some(last2)) = (content1.last(), content2.last()) {
                    if let (SectionContentNode::Paragraph(p1), SectionContentNode::Paragraph(p2)) =
                        (last1, last2)
                    {
                        let text1 = p1
                            .segments
                            .iter()
                            .filter_map(|seg| match seg {
                                RichTextSegment::Text(t) => Some(t.clone()),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join("");
                        let text2 = p2
                            .segments
                            .iter()
                            .filter_map(|seg| match seg {
                                RichTextSegment::Text(t) => Some(t.clone()),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join("");

                        // Check if the goal states are the same
                        text1.contains("→ Given") && text2.contains("→ Given") && text1 == text2
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            (
                ProofStepNode::Statement { claim: claim1, .. },
                ProofStepNode::Statement { claim: claim2, .. },
            ) => {
                let text1 = claim1
                    .iter()
                    .filter_map(|seg| match seg {
                        RichTextSegment::Text(t) => Some(t.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("");
                let text2 = claim2
                    .iter()
                    .filter_map(|seg| match seg {
                        RichTextSegment::Text(t) => Some(t.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("");
                text1 == text2
            }
            _ => false,
        }
    }

    /// Helper function to format an identifier
    fn format_identifier(
        &self,
        id: &crate::subjects::math::formalism::expressions::Identifier,
    ) -> String {
        use crate::subjects::math::formalism::expressions::Identifier;
        match id {
            Identifier::Name(name, _) => name.clone(),
            Identifier::O(n) => format!("O{}", n),
            Identifier::M(n) => format!("M{}", n),
            Identifier::E(n) => format!("E{}", n),
            Identifier::N(n) => format!("N{}", n),
        }
    }

    /// Helper function to format an expression (simplified)
    fn format_expression(&self, expr: &MathExpression) -> String {
        match expr {
            MathExpression::Var(id) => self.format_identifier(id),
            MathExpression::Relation(rel) => format!("({})", self.format_relation(rel)),
            MathExpression::Expression(theory_expr) => {
                use crate::subjects::math::formalism::expressions::TheoryExpression;
                match theory_expr {
                    TheoryExpression::Group(group_expr) => self.format_group_expression(group_expr),
                    _ => "⟨expr⟩".to_string(),
                }
            }
            MathExpression::Number(_) => "⟨num⟩".to_string(),
            MathExpression::Object(_) => "⟨object⟩".to_string(),
            MathExpression::ViewAs {
                expression,
                view: _,
            } => {
                format!("⟨{} as view⟩", self.format_expression(expression))
            }
        }
    }

    /// Helper function to format group elements
    fn format_group_element(
        &self,
        elem: &crate::subjects::math::theories::groups::definitions::GroupElement,
    ) -> String {
        use crate::subjects::math::theories::groups::definitions::GroupElement;
        match elem {
            GroupElement::Integer(n) => n.to_string(),
            GroupElement::Permutation(perm) => format!("perm{:?}", perm),
            GroupElement::Matrix(matrix) => format!("matrix{:?}", matrix),
            GroupElement::Symbol(s) => s.clone(),
        }
    }

    /// Helper function to format group expressions
    fn format_group_expression(
        &self,
        expr: &crate::subjects::math::theories::groups::definitions::GroupExpression,
    ) -> String {
        use crate::subjects::math::formalism::extract::Parametrizable;
        use crate::subjects::math::theories::groups::definitions::GroupExpression;

        match expr {
            GroupExpression::Identity(_) => "e".to_string(),
            GroupExpression::Element { element, .. } => match element {
                Parametrizable::Variable(id) => self.format_identifier(id),
                Parametrizable::Concrete(elem) => self.format_group_element(elem),
            },
            GroupExpression::Operation { left, right, .. } => {
                let left_str = match left.as_ref() {
                    Parametrizable::Variable(id) => self.format_identifier(id),
                    Parametrizable::Concrete(expr) => self.format_group_expression(expr),
                };
                let right_str = match right.as_ref() {
                    Parametrizable::Variable(id) => self.format_identifier(id),
                    Parametrizable::Concrete(expr) => self.format_group_expression(expr),
                };
                format!("({} * {})", left_str, right_str)
            }
            GroupExpression::Inverse { element, .. } => {
                let elem_str = match element.as_ref() {
                    Parametrizable::Variable(id) => self.format_identifier(id),
                    Parametrizable::Concrete(expr) => self.format_group_expression(expr),
                };
                format!("{}⁻¹", elem_str)
            }
            _ => "⟨group_expr⟩".to_string(),
        }
    }

    /// Create a comprehensive initial statement using generic methods
    fn create_comprehensive_initial_statement(&self) -> ProofStepNode {
        let mut content = vec![];

        // Add variable explanations if available
        if !self.goal.quantifier.is_empty() {
            content.push(SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::StyledText {
                    text: "Variables: ".to_string(),
                    styles: vec![TextStyle::Bold],
                }],
                alignment: None,
            }));

            for q in &self.goal.quantifier {
                let quantifier_word = match q.quantification {
                    crate::subjects::math::formalism::theorem::Quantification::Universal => "For any",
                    crate::subjects::math::formalism::theorem::Quantification::Existential => "There exists",
                    crate::subjects::math::formalism::theorem::Quantification::UniqueExistential => "There exists a unique",
                    crate::subjects::math::formalism::theorem::Quantification::Defined => "Let",
                    crate::subjects::math::formalism::theorem::Quantification::Fixed => "Fix",
                };

                let var_description = if let Some(desc) = &q.description {
                    format!("• {} {} ({})", quantifier_word, q.variable, desc)
                } else {
                    format!("• {} {}", quantifier_word, q.variable)
                };

                content.push(SectionContentNode::Paragraph(ParagraphNode {
                    segments: vec![RichTextSegment::Text(var_description)],
                    alignment: None,
                }));
            }
        }

        // Add theorem statement using structured types
        content.push(SectionContentNode::Paragraph(ParagraphNode {
            segments: vec![
                RichTextSegment::StyledText {
                    text: "Formal Statement: ".to_string(),
                    styles: vec![TextStyle::Bold],
                },
                RichTextSegment::Text(self.format_relation(&self.goal.statement)),
            ],
            alignment: None,
        }));

        // Add informal description
        let informal_desc = self.create_generic_informal_description();
        if !informal_desc.is_empty() {
            content.push(SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::StyledText {
                    text: format!("Informal Description: {}", informal_desc),
                    styles: vec![TextStyle::Italic],
                }],
                alignment: None,
            }));
        }

        ProofStepNode::Elaboration(content)
    }

    /// Create a generic informal description from the theorem structure
    fn create_generic_informal_description(&self) -> String {
        // Focus only on the logical structure, not the variables (since they're explained separately)
        match &self.goal.statement {
            MathRelation::Implies(premise, conclusion) => {
                format!(
                    "If {} then {}.",
                    self.format_relation_informal(premise),
                    self.format_relation_informal(conclusion)
                )
            }
            MathRelation::Equal { left, right, .. } => {
                format!(
                    "{} equals {}.",
                    self.format_expression(left),
                    self.format_expression(right)
                )
            }
            _ => {
                format!("{}.", self.format_relation_informal(&self.goal.statement))
            }
        }
    }

    /// Helper function to format a relation (simplified)
    fn format_relation(&self, rel: &MathRelation) -> String {
        match rel {
            MathRelation::Equal { left, right, .. } => {
                format!(
                    "{} = {}",
                    self.format_expression(left),
                    self.format_expression(right)
                )
            }
            MathRelation::Implies(premise, conclusion) => {
                format!(
                    "{} ⟹ {}",
                    self.format_relation(premise),
                    self.format_relation(conclusion)
                )
            }
            MathRelation::And(relations) => {
                let parts: Vec<String> =
                    relations.iter().map(|r| self.format_relation(r)).collect();
                format!("({})", parts.join(" ∧ "))
            }
            MathRelation::Todo { name, .. } => format!("TODO: {}", name),
            _ => "⟨relation⟩".to_string(),
        }
    }

    /// Helper function to format a relation for informal descriptions (using words instead of symbols)
    fn format_relation_informal(&self, rel: &MathRelation) -> String {
        match rel {
            MathRelation::Equal { left, right, .. } => {
                format!(
                    "{} = {}",
                    self.format_expression(left),
                    self.format_expression(right)
                )
            }
            MathRelation::Implies(premise, conclusion) => {
                format!(
                    "{} implies {}",
                    self.format_relation_informal(premise),
                    self.format_relation_informal(conclusion)
                )
            }
            MathRelation::And(relations) => {
                let parts: Vec<String> = relations
                    .iter()
                    .map(|r| self.format_relation_informal(r))
                    .collect();
                format!("({})", parts.join(" and "))
            }
            MathRelation::Todo { name, .. } => format!("TODO: {}", name),
            _ => "⟨relation⟩".to_string(),
        }
    }
}

mod tests {
    use serde_json::to_value;

    use crate::subjects::math::theories::theorems::prove_inverse_product_rule;

    use super::*;

    #[test]
    fn test_theorem_render() {
        let theorem = prove_inverse_product_rule();
        let section = theorem.to_section_node("theorem_id");
        println!("{:#?}", to_value(&section));
    }
}
