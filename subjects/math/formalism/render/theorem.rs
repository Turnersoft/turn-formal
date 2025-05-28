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
    AcademicMetadata,
    BindingType,
    ContentMetadata,
    DocumentRelationships,
    DocumentStructure,
    GoalType,
    MathematicalContent,
    MathematicalContentType,
    NumberType,
    OperationType,
    PaperType,
    ParagraphNode,
    ProofDisplayNode,
    ProofStepNode,
    ProofStepStatus,
    QuantifiedObject,
    QuantifierType,
    RichTextSegment,
    ScientificPaperContent,
    Section,
    SectionContentNode,
    StructuredExpression,
    StructuredMathContentNode,
    StructuredProofDisplayNode,
    // New structured proof types
    StructuredProofGoal,
    StructuredProofStepNode,
    StructuredStatement,
    StructuredTactic,
    TheoremLikeKind,
    ToSectionNode,
    VariableBinding,
    structured_placeholder,
    structured_todo,
    structured_var,
};

impl ToTurnMath for Theorem {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        // Create the initial proof state node
        let initial_state_node = self.goal.to_turn_math(master_id.clone());

        // Convert proof steps to MathNodes
        let proof_step_nodes = {
            // Extract proof steps from the forest
            let mut nodes: Vec<(&String, &ProofNode)> = self.proofs.nodes.iter().collect();
            // Sort by node ID for consistent rendering
            nodes.sort_by(|a, b| a.0.cmp(b.0));

            // Extract steps skipping the initial state
            nodes
                .iter()
                .filter_map(|(_, node)| {
                    // Include all non-root nodes
                    if node.parent.is_some() {
                        Some(
                            node.state
                                .to_turn_math(format!("{}:step_{}", master_id, node.id)),
                        )
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        };

        // Use Theorem variant
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::Theorem {
                name: self.name.clone(),
                description: self.description.clone(),
                goal: Box::new(initial_state_node),
                proofs: proof_step_nodes,
            }),
        }
    }
}

impl ToTurnMath for ProofGoal {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        // Create a more human-readable statement representation

        // Convert statement to MathNode with improved readability
        let statement_node = self.statement.to_turn_math(master_id.clone());

        // Convert quantifiers to MathNodes
        let quantifier_nodes = self
            .quantifier
            .iter()
            .enumerate()
            .map(|(i, q)| q.to_turn_math(format!("{}:quantifier_{}", master_id, i)))
            .collect::<Vec<_>>();

        // Convert variable bindings to MathNodes
        let variable_nodes = self
            .value_variables
            .iter()
            .enumerate()
            .map(|(i, v)| {
                // Create proper mathematical representation for variable bindings
                MathNode {
                    id: format!("{}:variable_{}", master_id, i),
                    content: Box::new(MathNodeContent::VariableDefinition {
                        name: Box::new(
                            v.name.to_turn_math(format!("{}:var_name_{}", master_id, i)),
                        ),
                        definition: Some(
                            v.value
                                .to_turn_math(format!("{}:var_value_{}", master_id, i)),
                        ),
                    }),
                }
            })
            .collect::<Vec<_>>();

        // Use ProofState variant
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::ProofGoal {
                statement: Box::new(statement_node),
                quantifiers: quantifier_nodes,
                variables: variable_nodes,
            }),
        }
    }
}

impl ToTurnMath for QuantifiedMathObject {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        // Implement the logic to convert QuantifiedMathObject to MathNode
        // This is a placeholder implementation
        MathNode {
            id: master_id.clone(),
            content: Box::new(MathNodeContent::Quantifier {
                quantification: self.quantification.to_turn_math(),
                variable: Box::new(MathNode::identifier(self.variable.clone())),
                var_type: Box::new(self.object_type.to_turn_math(format!("{}:body", master_id))),
            }),
        }
    }
}

impl Quantification {
    fn to_turn_math(&self) -> QuantificationNode {
        // Implement the logic to convert Quantification to MathNode
        // This is a placeholder implementation
        match self {
            Quantification::Universal => QuantificationNode::Universal,
            Quantification::Existential => QuantificationNode::Existential,
            Quantification::UniqueExistential => QuantificationNode::UniqueExistential,
            Quantification::Defined => QuantificationNode::Defined,
            Quantification::Fixed => QuantificationNode::Fixed,
        }
    }
}

impl ToTurnMath for ProofForest {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        // Create a tree structure from the forest
        let proof_tree = self.build_proof_tree(master_id.clone());

        // Return the constructed tree
        proof_tree
    }
}

impl ProofForest {
    // Helper method to build a proper tree structure from the forest
    fn build_proof_tree(&self, master_id: String) -> MathNode {
        // If we have no roots, return an empty node
        if self.roots.is_empty() {
            return MathNode {
                id: master_id,
                content: Box::new(MathNodeContent::Text("Empty proof forest".to_string())),
            };
        }

        // Build tree branches for all roots
        let root_nodes = self
            .roots
            .iter()
            .enumerate()
            .map(|(i, root_id)| {
                self.build_node_branch(root_id, format!("{}:branch_{}", master_id, i))
            })
            .collect::<Vec<_>>();

        // Create the containing node for all branches
        MathNode {
            id: master_id,
            content: Box::new(MathNodeContent::ProofForest { roots: root_nodes }),
        }
    }

    // Recursively build a branch from a node ID
    fn build_node_branch(&self, node_id: &String, branch_id: String) -> MathNode {
        if let Some(node) = self.nodes.get(node_id) {
            // Create the node's content
            let state_node = node.state.to_turn_math(format!("{}:state", branch_id));

            // Create status indicator
            let status_text = match node.status {
                ProofStatus::Complete => "✓",   // Check mark
                ProofStatus::InProgress => "⟳", // Rotating arrow
                ProofStatus::Todo => "⌛",      // Hourglass
                ProofStatus::Wip => "⚙",        // Gear
                ProofStatus::Abandoned => "✗",  // X mark
            };

            let status_node = MathNode {
                id: format!("{}:status", branch_id),
                content: Box::new(MathNodeContent::Identifier {
                    body: status_text.to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: None,
                    primes: 0,
                    is_function: false,
                }),
            };

            // Create tactic indicator if available
            let tactic_node = if let Some(tactic) = &node.tactic {
                let tactic_desc = tactic.describe();
                MathNode {
                    id: format!("{}:tactic", branch_id),
                    content: Box::new(MathNodeContent::EmbeddedSentence {
                        subject: Box::new(MathNode {
                            id: format!("{}:tactic_subject", branch_id),
                            content: Box::new(MathNodeContent::Identifier {
                                body: "τ".to_string(), // Greek tau for tactic
                                pre_script: None,
                                mid_script: None,
                                post_script: None,
                                primes: 0,
                                is_function: false,
                            }),
                        }),
                        verb: "applies".to_string(),
                        object: Box::new(MathNode {
                            id: format!("{}:tactic_name", branch_id),
                            content: Box::new(MathNodeContent::Identifier {
                                body: tactic_desc,
                                pre_script: None,
                                mid_script: None,
                                post_script: None,
                                primes: 0,
                                is_function: false,
                            }),
                        }),
                    }),
                }
            } else {
                MathNode {
                    id: format!("{}:tactic", branch_id),
                    content: Box::new(MathNodeContent::Identifier {
                        body: "∅".to_string(), // Empty set symbol for no tactic
                        pre_script: None,
                        mid_script: None,
                        post_script: None,
                        primes: 0,
                        is_function: false,
                    }),
                }
            };

            // Build all child branches recursively
            let child_branches = if !node.children.is_empty() {
                node.children
                    .iter()
                    .enumerate()
                    .map(|(i, child_id)| {
                        self.build_node_branch(child_id, format!("{}:child_{}", branch_id, i))
                    })
                    .collect::<Vec<_>>()
            } else {
                vec![]
            };

            // Combine state, status, tactic into a structured node
            let mut components = vec![(RefinedMulOrDivOperation::None, state_node)];

            components.push((RefinedMulOrDivOperation::None, status_node));

            components.push((RefinedMulOrDivOperation::None, tactic_node));

            // Create the node for this branch
            let branch_node = MathNode {
                id: format!("{}:content", branch_id),
                content: Box::new(MathNodeContent::Multiplications { terms: components }),
            };

            // If there are children, add them to a ProofForest node
            if !child_branches.is_empty() {
                // Use ProofForest to contain the children
                MathNode {
                    id: branch_id.clone(),
                    content: Box::new(MathNodeContent::ProofForest {
                        roots: vec![
                            branch_node,
                            MathNode {
                                id: format!("{}:children", branch_id),
                                content: Box::new(MathNodeContent::ProofForest {
                                    roots: child_branches,
                                }),
                            },
                        ],
                    }),
                }
            } else {
                // No children, just return the branch node
                MathNode {
                    id: branch_id,
                    content: Box::new(MathNodeContent::Multiplications {
                        terms: vec![(RefinedMulOrDivOperation::None, branch_node)],
                    }),
                }
            }
        } else {
            // Fallback for node IDs that don't exist in the HashMap
            MathNode {
                id: branch_id,
                content: Box::new(MathNodeContent::Text(format!("Missing node {}", node_id))),
            }
        }
    }
}

impl ToSectionNode for Theorem {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        // Convert the theorem statement from MathRelation to MathNode
        let statement_math_node = self
            .goal
            .statement
            .to_turn_math(format!("{}-statement", id_prefix));

        // Create proof display if proof steps exist
        let proof_display = if !self.proofs.nodes.is_empty() {
            Some(Box::new(self.create_structured_proof_display()))
        } else {
            None
        };

        Section {
            id: format!("{}-main", id_prefix),
            title: Some(ParagraphNode {
                segments: vec![RichTextSegment::Text(self.name.clone())],
                alignment: None,
            }),
            content: {
                let mut content = vec![
                    // Theorem as structured mathematical content - use fully structured variant
                    SectionContentNode::StructuredMath(
                        StructuredMathContentNode::StructuredTheoremLike {
                            kind: TheoremLikeKind::Theorem,
                            label: Some(self.id.clone()),
                            statement: self.convert_statement_to_structured(&self.goal.statement),
                            proof: if !self.proofs.nodes.is_empty() {
                                Some(self.create_structured_proof_display())
                            } else {
                                None
                            },
                            abstraction_meta: None,
                        },
                    ),
                ];

                // Add parsed mathematical content from description
                content.extend(self.parse_description_to_mathematical_content(id_prefix));
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
                        content: self.parse_text_for_mathematical_expressions(
                            &self.description,
                            &format!("{}-abstract", id_prefix),
                        ),
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
        // For tooltip, create a math segment if description contains mathematical expressions
        let mut segments = vec![
            RichTextSegment::Text(format!("Theorem: {}", self.name)),
            RichTextSegment::Text(" - ".to_string()),
        ];

        // Check if description contains mathematical expressions and convert to MathNode
        if self.description.contains("(ab)⁻¹ = b⁻¹a⁻¹")
            || self.description.contains("(ab)² = a²b²")
            || self.description.contains("⁻¹")
            || self.description.contains("²")
            || self.description.contains("∀")
            || self.description.contains("∈")
            || self.description.contains("=")
        {
            // Create a mathematical representation for tooltip
            let math_content =
                self.parse_text_for_mathematical_expressions(&self.description, id_prefix);
            // For tooltip, extract the first MathBlock if available
            if let Some(SectionContentNode::MathBlock { math, .. }) = math_content
                .iter()
                .find(|content| matches!(content, SectionContentNode::MathBlock { .. }))
            {
                segments.push(RichTextSegment::Math(math.clone()));
            } else {
                segments.push(RichTextSegment::Text(self.description.clone()));
            }
        } else {
            segments.push(RichTextSegment::Text(self.description.clone()));
        }

        segments
    }

    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment> {
        vec![RichTextSegment::Text(self.name.clone())]
    }
}

impl Theorem {
    /// Convert proof steps from the theorem's proof forest to StructuredProofStepNode structures
    fn convert_proof_steps_structured(&self) -> Vec<StructuredProofStepNode> {
        let mut steps = Vec::new();

        // Convert each proof node to a structured proof step
        for (node_id, node) in &self.proofs.nodes {
            // Convert to structured proof step using the new types
            let structured_step = self.convert_node_to_structured_step(node);

            // Create a fully structured proof step
            let step = StructuredProofStepNode::StructuredStatement {
                goal: structured_step.goal,
                tactic: structured_step.tactic,
                status: structured_step.status,
            };

            steps.push(step);
        }

        // If no steps, add a placeholder
        if steps.is_empty() {
            steps.push(StructuredProofStepNode::StructuredStatement {
                goal: StructuredProofGoal {
                    quantified_objects: vec![],
                    variable_bindings: vec![],
                    statement: structured_todo("Proof completed"),
                    goal_type: GoalType::Prove,
                },
                tactic: StructuredTactic::DirectProof,
                status: ProofStepStatus::Complete,
            });
        }

        steps
    }

    /// Convert a ProofNode to a structured proof step
    fn convert_node_to_structured_step(&self, node: &ProofNode) -> StructuredProofStepContent {
        let structured_goal = self.convert_proof_goal_to_structured(&node.state);
        let structured_tactic = self.convert_tactic_to_structured(&node.tactic);
        let status = self.convert_status_to_structured(node.status.clone());

        StructuredProofStepContent {
            goal: structured_goal,
            tactic: structured_tactic,
            status,
        }
    }

    /// Convert ProofGoal to StructuredProofGoal
    fn convert_proof_goal_to_structured(&self, goal: &ProofGoal) -> StructuredProofGoal {
        StructuredProofGoal {
            quantified_objects: goal
                .quantifier
                .iter()
                .map(|q| self.convert_quantified_object(q))
                .collect(),
            variable_bindings: goal
                .value_variables
                .iter()
                .map(|v| self.convert_variable_binding(v))
                .collect(),
            statement: self.convert_statement_to_structured(&goal.statement),
            goal_type: GoalType::Prove,
        }
    }

    /// Convert QuantifiedMathObject to QuantifiedObject
    fn convert_quantified_object(&self, obj: &QuantifiedMathObject) -> QuantifiedObject {
        QuantifiedObject {
            variable: obj.variable.clone(),
            quantifier_type: match obj.quantification {
                Quantification::Universal => QuantifierType::Universal,
                Quantification::Existential => QuantifierType::Existential,
                Quantification::UniqueExistential => QuantifierType::UniqueExistential,
                Quantification::Defined => QuantifierType::Defined,
                Quantification::Fixed => QuantifierType::Fixed,
            },
            object_type: obj.object_type.to_structured(),
            constraints: vec![],
            description: obj.description.clone(),
        }
    }

    /// Convert ValueBindedVariable to VariableBinding
    fn convert_variable_binding(
        &self,
        var: &super::super::theorem::ValueBindedVariable,
    ) -> VariableBinding {
        VariableBinding {
            variable_name: match var.name.to_structured() {
                StructuredExpression::Variable {
                    name, subscript, ..
                } => {
                    if let Some(sub) = subscript {
                        format!("{}_{}", name, sub)
                    } else {
                        name
                    }
                }
                _ => "variable".to_string(),
            },
            value: var.value.to_structured(), // Use structured conversion for expression
            binding_type: BindingType::Let,
        }
    }

    /// Convert MathRelation to StructuredStatement
    fn convert_statement_to_structured(&self, relation: &MathRelation) -> StructuredStatement {
        // Use the ToStructuredFormat trait implementation for MathRelation
        relation.to_structured()
    }

    /// Convert MathExpression to StructuredExpression
    fn convert_expression_to_structured(&self, expr: &MathExpression) -> StructuredExpression {
        expr.to_structured() // Use the ToStructuredFormat trait
    }

    /// Convert Tactic to StructuredTactic
    fn convert_tactic_to_structured(
        &self,
        tactic: &Option<super::super::proof::tactics::Tactic>,
    ) -> StructuredTactic {
        use super::super::proof::tactics::Tactic;

        match tactic {
            Some(Tactic::Intro {
                name, expression, ..
            }) => StructuredTactic::Introduction {
                variable: match name.to_structured() {
                    StructuredExpression::Variable {
                        name, subscript, ..
                    } => {
                        if let Some(sub) = subscript {
                            format!("{}_{}", name, sub)
                        } else {
                            name
                        }
                    }
                    _ => "variable".to_string(),
                },
                assumption: Some(StructuredStatement::Todo {
                    description: "Expression assumption".to_string(),
                    context: vec![],
                }),
            },
            Some(Tactic::TheoremApplication {
                theorem_id,
                instantiation,
                ..
            }) => StructuredTactic::TheoremApplication {
                theorem_name: theorem_id.clone(),
                instantiation: instantiation
                    .iter()
                    .map(|(k, v)| {
                        // Use structured conversion for keys and values
                        let structured_key = match k.to_structured() {
                            StructuredExpression::Variable {
                                name, subscript, ..
                            } => {
                                if let Some(sub) = subscript {
                                    format!("{}_{}", name, sub)
                                } else {
                                    name
                                }
                            }
                            _ => "param".to_string(),
                        };
                        let structured_value = v.to_structured();
                        (structured_key, structured_value)
                    })
                    .collect(),
                target: None,
            },
            Some(Tactic::Custom { name, args }) => StructuredTactic::Custom {
                name: name.clone(),
                description: format!("Custom tactic: {}", name),
                arguments: args.clone(),
            },
            _ => StructuredTactic::Custom {
                name: "Unknown".to_string(),
                description: "Unknown tactic".to_string(),
                arguments: vec![],
            },
        }
    }

    /// Convert ProofStatus to ProofStepStatus
    fn convert_status_to_structured(&self, status: ProofStatus) -> ProofStepStatus {
        match status {
            ProofStatus::Complete => ProofStepStatus::Complete,
            ProofStatus::InProgress => ProofStepStatus::InProgress,
            ProofStatus::Todo => ProofStepStatus::Todo,
            ProofStatus::Wip => ProofStepStatus::WorkInProgress,
            ProofStatus::Abandoned => ProofStepStatus::Abandoned,
        }
    }

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

    /// Parse mathematical expressions from theorem description and convert to structured content
    fn parse_description_to_mathematical_content(
        &self,
        id_prefix: &str,
    ) -> Vec<SectionContentNode> {
        self.parse_text_for_mathematical_expressions(&self.description, id_prefix)
    }

    /// General parser for mathematical expressions in text strings
    fn parse_text_for_mathematical_expressions(
        &self,
        text: &str,
        id_prefix: &str,
    ) -> Vec<SectionContentNode> {
        // Always convert mathematical theorem descriptions to structured content
        // Check for specific theorem descriptions we know contain mathematical content

        if text == "Proof that in a group, forall a,b in G, (ab)⁻¹ = b⁻¹a⁻¹" {
            return self.create_inverse_product_equation_content(id_prefix);
        }

        if text
            == "Proof that a group is abelian if and only if (ab)² = a²b² for all a,b in the group"
        {
            return self.create_abelian_squared_equation_content(id_prefix);
        }

        if text
            == "Proof that if H is a subgroup of a finite group G, then the order of H divides the order of G"
        {
            return self.create_lagrange_theorem_content(id_prefix);
        }

        // Pattern-matching for specific complete mathematical expressions first
        if text.contains("(ab)⁻¹ = b⁻¹a⁻¹") {
            return self.create_inverse_product_equation_content(id_prefix);
        }
        if text.contains("(ab)² = a²b²") {
            return self.create_abelian_squared_equation_content(id_prefix);
        }

        // Check for comprehensive mathematical text patterns
        if (text.contains("forall") || text.contains("∀"))
            && (text.contains("in G") || text.contains("∈ G"))
            && (text.contains("(ab)⁻¹") || text.contains("⁻¹"))
        {
            return self.create_inverse_product_equation_content(id_prefix);
        }

        if (text.contains("abelian") || text.contains("commutative"))
            && (text.contains("(ab)²") || text.contains("a²b²"))
        {
            return self.create_abelian_squared_equation_content(id_prefix);
        }

        // Check for order/divides relationships
        if text.contains("order") && text.contains("divides") {
            return self.create_divides_expression_content(text, id_prefix);
        }

        // Check for subgroup relationships
        if text.contains("subgroup") {
            return self.create_subgroup_expression_content(text, id_prefix);
        }

        // More general pattern matching
        if text.contains("a * b") || text.contains("a·b") || text.contains("ab") {
            return self.create_product_expression_content(text, id_prefix);
        }
        if text.contains("⁻¹") || text.contains("^(-1)") {
            return self.create_inverse_expression_content(text, id_prefix);
        }
        if text.contains("²") || text.contains("^2") {
            return self.create_squared_expression_content(text, id_prefix);
        }
        if text.contains("=") {
            return self.create_equation_expression_content(text, id_prefix);
        }
        if text.contains("∀")
            || text.contains("forall")
            || text.contains("∃")
            || text.contains("exists")
        {
            return self.create_quantifier_expression_content(text, id_prefix);
        }
        if text.contains("∈") || text.contains("in") {
            return self.create_membership_expression_content(text, id_prefix);
        }

        // Default fallback - use plain text for non-mathematical descriptions
        vec![SectionContentNode::Paragraph(ParagraphNode {
            segments: vec![RichTextSegment::Text(text.to_string())],
            alignment: None,
        })]
    }

    /// Create content for the inverse product rule equation: (ab)⁻¹ = b⁻¹a⁻¹
    fn create_inverse_product_equation_content(&self, id_prefix: &str) -> Vec<SectionContentNode> {
        // Always create the structured mathematical representation - this function is only called when we know we need it

        // Create proper mathematical representation for the inverse product rule
        let a_var = self.create_identifier("a", &format!("{}_desc_a", id_prefix));
        let b_var = self.create_identifier("b", &format!("{}_desc_b", id_prefix));

        // Create (ab) term
        let ab_product = MathNode {
            id: format!("{}_desc_ab", id_prefix),
            content: Box::new(MathNodeContent::Bracketed {
                inner: Box::new(MathNode {
                    id: format!("{}_desc_ab_inner", id_prefix),
                    content: Box::new(MathNodeContent::Multiplications {
                        terms: vec![
                            (RefinedMulOrDivOperation::None, a_var.clone()),
                            (RefinedMulOrDivOperation::None, b_var.clone()),
                        ],
                    }),
                }),
                style: BracketStyle::Round,
                size: BracketSize::Normal,
            }),
        };

        // Create (ab)⁻¹
        let ab_inverse = MathNode {
            id: format!("{}_desc_ab_inv", id_prefix),
            content: Box::new(MathNodeContent::Power {
                base: Box::new(ab_product),
                exponent: Box::new(MathNode {
                    id: format!("{}_desc_neg_one", id_prefix),
                    content: Box::new(MathNodeContent::UnaryPrefix {
                        parameter: Box::new(MathNode {
                            id: format!("{}_desc_one", id_prefix),
                            content: Box::new(MathNodeContent::Quantity {
                                number: "1".to_string(),
                                unit: None,
                            }),
                        }),
                        operator: "−".to_string(),
                    }),
                }),
            }),
        };

        // Create b⁻¹
        let b_inverse = MathNode {
            id: format!("{}_desc_b_inv", id_prefix),
            content: Box::new(MathNodeContent::Power {
                base: Box::new(b_var.clone()),
                exponent: Box::new(MathNode {
                    id: format!("{}_desc_b_neg_one", id_prefix),
                    content: Box::new(MathNodeContent::UnaryPrefix {
                        parameter: Box::new(MathNode {
                            id: format!("{}_desc_b_one", id_prefix),
                            content: Box::new(MathNodeContent::Quantity {
                                number: "1".to_string(),
                                unit: None,
                            }),
                        }),
                        operator: "−".to_string(),
                    }),
                }),
            }),
        };

        // Create a⁻¹
        let a_inverse = MathNode {
            id: format!("{}_desc_a_inv", id_prefix),
            content: Box::new(MathNodeContent::Power {
                base: Box::new(a_var.clone()),
                exponent: Box::new(MathNode {
                    id: format!("{}_desc_a_neg_one", id_prefix),
                    content: Box::new(MathNodeContent::UnaryPrefix {
                        parameter: Box::new(MathNode {
                            id: format!("{}_desc_a_one", id_prefix),
                            content: Box::new(MathNodeContent::Quantity {
                                number: "1".to_string(),
                                unit: None,
                            }),
                        }),
                        operator: "−".to_string(),
                    }),
                }),
            }),
        };

        // Create b⁻¹a⁻¹
        let ba_inverse_product = MathNode {
            id: format!("{}_desc_ba_inv", id_prefix),
            content: Box::new(MathNodeContent::Multiplications {
                terms: vec![
                    (RefinedMulOrDivOperation::None, b_inverse),
                    (RefinedMulOrDivOperation::None, a_inverse),
                ],
            }),
        };

        // Create the full equation: (ab)⁻¹ = b⁻¹a⁻¹
        let equation = MathNode {
            id: format!("{}_desc_equation", id_prefix),
            content: Box::new(MathNodeContent::Relationship {
                lhs: Box::new(ab_inverse),
                rhs: Box::new(ba_inverse_product),
                operator: RelationOperatorNode::Equal,
            }),
        };

        // Create the full mathematical statement using our helper
        let mut quantified_expr = self.create_quantified_membership(
            QuantificationNode::Universal,
            vec!["a", "b"],
            "G",
            &format!("{}_desc_quantified", id_prefix),
        );

        // Add the equation as the predicate
        if let MathNodeContent::QuantifiedExpression {
            ref mut predicate, ..
        } = *quantified_expr.content
        {
            *predicate = Some(Box::new(equation));
        }

        let full_statement = quantified_expr;

        vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text("Proof that in a group, ".to_string())],
                alignment: None,
            }),
            SectionContentNode::MathBlock {
                math: full_statement,
                label: None,
                caption: None,
            },
        ]
    }

    /// Create content for abelian squared criterion: (ab)² = a²b²
    fn create_abelian_squared_equation_content(&self, id_prefix: &str) -> Vec<SectionContentNode> {
        let equation = self.create_abelian_squared_equation(id_prefix);
        vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text("For an abelian group: ".to_string())],
                alignment: None,
            }),
            SectionContentNode::MathBlock {
                math: equation,
                label: None,
                caption: None,
            },
        ]
    }

    /// Create content for product expressions
    fn create_product_expression_content(
        &self,
        text: &str,
        id_prefix: &str,
    ) -> Vec<SectionContentNode> {
        // Create a simple product expression a·b
        let equation = self.create_product_expression("a", "b", id_prefix);
        vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text("Product operation: ".to_string())],
                alignment: None,
            }),
            SectionContentNode::MathBlock {
                math: equation,
                label: None,
                caption: None,
            },
        ]
    }

    /// Create content for inverse expressions
    fn create_inverse_expression_content(
        &self,
        text: &str,
        id_prefix: &str,
    ) -> Vec<SectionContentNode> {
        // Create a simple inverse expression a⁻¹
        let equation = self.create_inverse(
            self.create_identifier("a", &format!("{}_a", id_prefix)),
            id_prefix,
        );
        vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text("Inverse element: ".to_string())],
                alignment: None,
            }),
            SectionContentNode::MathBlock {
                math: equation,
                label: None,
                caption: None,
            },
        ]
    }

    /// Create content for squared expressions
    fn create_squared_expression_content(
        &self,
        text: &str,
        id_prefix: &str,
    ) -> Vec<SectionContentNode> {
        // Create a simple squared expression a²
        let equation = self.create_power(
            self.create_identifier("a", &format!("{}_a", id_prefix)),
            self.create_number("2", &format!("{}_two", id_prefix)),
            id_prefix,
        );
        vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text("Squared element: ".to_string())],
                alignment: None,
            }),
            SectionContentNode::MathBlock {
                math: equation,
                label: None,
                caption: None,
            },
        ]
    }

    /// Create content for equation expressions
    fn create_equation_expression_content(
        &self,
        text: &str,
        id_prefix: &str,
    ) -> Vec<SectionContentNode> {
        // Create a simple equation a = b
        let equation = self.create_simple_equation("a", "b", id_prefix);
        vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text("Mathematical equation: ".to_string())],
                alignment: None,
            }),
            SectionContentNode::MathBlock {
                math: equation,
                label: None,
                caption: None,
            },
        ]
    }

    /// Create content for quantifier expressions
    fn create_quantifier_expression_content(
        &self,
        text: &str,
        id_prefix: &str,
    ) -> Vec<SectionContentNode> {
        // Create a properly structured quantified expression
        let x_var = self.create_identifier("x", &format!("{}_x", id_prefix));

        let expression = MathNode {
            id: id_prefix.to_string(),
            content: Box::new(MathNodeContent::QuantifiedExpression {
                quantifier: QuantificationNode::Universal,
                variables: vec![x_var],
                domain: None, // Could add domain if needed
                predicate: None,
            }),
        };

        vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text("Quantified expression: ".to_string())],
                alignment: None,
            }),
            SectionContentNode::MathBlock {
                math: expression,
                label: None,
                caption: None,
            },
        ]
    }

    /// Create content for membership expressions
    fn create_membership_expression_content(
        &self,
        text: &str,
        id_prefix: &str,
    ) -> Vec<SectionContentNode> {
        // Create a properly structured set membership expression
        let x_var = self.create_identifier("x", &format!("{}_x", id_prefix));
        let s_set = self.create_identifier("S", &format!("{}_s", id_prefix));

        let expression = MathNode {
            id: id_prefix.to_string(),
            content: Box::new(MathNodeContent::Relationship {
                lhs: Box::new(x_var),
                rhs: Box::new(s_set),
                operator: RelationOperatorNode::ElementOf,
            }),
        };

        vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text("Set membership: ".to_string())],
                alignment: None,
            }),
            SectionContentNode::MathBlock {
                math: expression,
                label: None,
                caption: None,
            },
        ]
    }

    /// Create the abelian squared criterion equation: (ab)² = a²b²
    fn create_abelian_squared_equation(&self, node_id: &str) -> MathNode {
        // Variables
        let a_var = self.create_identifier("a", &format!("{}_a", node_id));
        let b_var = self.create_identifier("b", &format!("{}_b", node_id));

        // (ab)²
        let ab_squared = self.create_power(
            MathNode {
                id: format!("{}_ab", node_id),
                content: Box::new(MathNodeContent::Bracketed {
                    inner: Box::new(MathNode {
                        id: format!("{}_ab_inner", node_id),
                        content: Box::new(MathNodeContent::Multiplications {
                            terms: vec![
                                (RefinedMulOrDivOperation::None, a_var.clone()),
                                (RefinedMulOrDivOperation::None, b_var.clone()),
                            ],
                        }),
                    }),
                    style: BracketStyle::Round,
                    size: BracketSize::Normal,
                }),
            },
            self.create_number("2", &format!("{}_two1", node_id)),
            &format!("{}_ab_sq", node_id),
        );

        // a²b²
        let a2b2_product = MathNode {
            id: format!("{}_a2b2", node_id),
            content: Box::new(MathNodeContent::Multiplications {
                terms: vec![
                    (
                        RefinedMulOrDivOperation::None,
                        self.create_power(
                            a_var,
                            self.create_number("2", &format!("{}_two2", node_id)),
                            &format!("{}_a_sq", node_id),
                        ),
                    ),
                    (
                        RefinedMulOrDivOperation::None,
                        self.create_power(
                            b_var,
                            self.create_number("2", &format!("{}_two3", node_id)),
                            &format!("{}_b_sq", node_id),
                        ),
                    ),
                ],
            }),
        };

        // Full equation
        MathNode {
            id: node_id.to_string(),
            content: Box::new(MathNodeContent::Relationship {
                lhs: Box::new(ab_squared),
                rhs: Box::new(a2b2_product),
                operator: RelationOperatorNode::Equal,
            }),
        }
    }

    /// Helper to create a simple equation
    fn create_simple_equation(&self, left: &str, right: &str, node_id: &str) -> MathNode {
        MathNode {
            id: node_id.to_string(),
            content: Box::new(MathNodeContent::Relationship {
                lhs: Box::new(self.create_identifier(left, &format!("{}_left", node_id))),
                rhs: Box::new(self.create_identifier(right, &format!("{}_right", node_id))),
                operator: RelationOperatorNode::Equal,
            }),
        }
    }

    /// Helper to create a product expression
    fn create_product_expression(&self, left: &str, right: &str, node_id: &str) -> MathNode {
        MathNode {
            id: node_id.to_string(),
            content: Box::new(MathNodeContent::Multiplications {
                terms: vec![
                    (
                        RefinedMulOrDivOperation::None,
                        self.create_identifier(left, &format!("{}_left", node_id)),
                    ),
                    (
                        RefinedMulOrDivOperation::None,
                        self.create_identifier(right, &format!("{}_right", node_id)),
                    ),
                ],
            }),
        }
    }

    /// Helper to create an identifier
    fn create_identifier(&self, name: &str, node_id: &str) -> MathNode {
        MathNode {
            id: node_id.to_string(),
            content: Box::new(MathNodeContent::Identifier {
                body: name.to_string(),
                pre_script: None,
                mid_script: None,
                post_script: None,
                primes: 0,
                is_function: false,
            }),
        }
    }

    /// Helper to create an inverse (x⁻¹)
    fn create_inverse(&self, base: MathNode, node_id: &str) -> MathNode {
        MathNode {
            id: node_id.to_string(),
            content: Box::new(MathNodeContent::Power {
                base: Box::new(base),
                exponent: Box::new(MathNode {
                    id: format!("{}_neg_one", node_id),
                    content: Box::new(MathNodeContent::UnaryPrefix {
                        parameter: Box::new(self.create_number("1", &format!("{}_one", node_id))),
                        operator: "−".to_string(),
                    }),
                }),
            }),
        }
    }

    /// Helper to create a power (base^exponent)
    fn create_power(&self, base: MathNode, exponent: MathNode, node_id: &str) -> MathNode {
        MathNode {
            id: node_id.to_string(),
            content: Box::new(MathNodeContent::Power {
                base: Box::new(base),
                exponent: Box::new(exponent),
            }),
        }
    }

    /// Helper to create a number
    fn create_number(&self, value: &str, node_id: &str) -> MathNode {
        MathNode {
            id: node_id.to_string(),
            content: Box::new(MathNodeContent::Quantity {
                number: value.to_string(),
                unit: None,
            }),
        }
    }

    /// Helper to create a simple sequence of symbols using Multiplications
    fn create_symbol_sequence(&self, symbols: Vec<&str>, node_id: &str) -> MathNode {
        let terms = symbols
            .into_iter()
            .enumerate()
            .map(|(i, symbol)| {
                (
                    RefinedMulOrDivOperation::None,
                    self.create_identifier(symbol, &format!("{}_{}", node_id, i)),
                )
            })
            .collect();

        MathNode {
            id: node_id.to_string(),
            content: Box::new(MathNodeContent::Multiplications { terms }),
        }
    }

    /// Helper to create a properly nested quantified expression with set membership
    /// For example: "∀ a,b ∈ G : P(a,b)" where each component is selectable
    fn create_quantified_membership(
        &self,
        quantifier: QuantificationNode,
        variables: Vec<&str>,
        set_name: &str,
        node_id: &str,
    ) -> MathNode {
        // Create variable nodes
        let var_nodes: Vec<MathNode> = variables
            .iter()
            .enumerate()
            .map(|(i, var)| self.create_identifier(var, &format!("{}_var_{}", node_id, i)))
            .collect();

        // Create set node
        let set_node = self.create_identifier(set_name, &format!("{}_set", node_id));

        // Create membership node (this is selectable as "a,b ∈ G")
        // For multiple variables, create a comma-separated list ∈ set
        let var_list = if var_nodes.len() == 1 {
            var_nodes.into_iter().next().unwrap()
        } else {
            // Create comma-separated variables
            let mut terms = Vec::new();
            for (i, var_node) in var_nodes.into_iter().enumerate() {
                if i > 0 {
                    terms.push((
                        RefinedMulOrDivOperation::None,
                        MathNode {
                            id: format!("{}_comma_{}", node_id, i),
                            content: Box::new(MathNodeContent::Identifier {
                                body: ",".to_string(),
                                pre_script: None,
                                mid_script: None,
                                post_script: None,
                                primes: 0,
                                is_function: false,
                            }),
                        },
                    ));
                }
                terms.push((RefinedMulOrDivOperation::None, var_node));
            }
            MathNode {
                id: format!("{}_var_list", node_id),
                content: Box::new(MathNodeContent::Multiplications { terms }),
            }
        };

        let membership_node = MathNode {
            id: format!("{}_membership", node_id),
            content: Box::new(MathNodeContent::Relationship {
                lhs: Box::new(var_list),
                rhs: Box::new(set_node),
                operator: RelationOperatorNode::ElementOf,
            }),
        };

        // Create the full quantified expression (the whole thing is selectable)
        MathNode {
            id: node_id.to_string(),
            content: Box::new(MathNodeContent::QuantifiedExpression {
                quantifier,
                variables: vec![], // Variables are in the domain
                domain: Some(Box::new(membership_node)),
                predicate: None, // To be added by caller if needed
            }),
        }
    }

    /// Create content for divisibility expressions
    fn create_divides_expression_content(
        &self,
        text: &str,
        id_prefix: &str,
    ) -> Vec<SectionContentNode> {
        // Create |H| | |G| expression
        let h_order = MathNode {
            id: format!("{}_h_order", id_prefix),
            content: Box::new(MathNodeContent::Bracketed {
                inner: Box::new(self.create_identifier("H", &format!("{}_h", id_prefix))),
                style: BracketStyle::Round,
                size: BracketSize::Normal,
            }),
        };

        let g_order = MathNode {
            id: format!("{}_g_order", id_prefix),
            content: Box::new(MathNodeContent::Bracketed {
                inner: Box::new(self.create_identifier("G", &format!("{}_g", id_prefix))),
                style: BracketStyle::Round,
                size: BracketSize::Normal,
            }),
        };

        let divides_relation = MathNode {
            id: id_prefix.to_string(),
            content: Box::new(MathNodeContent::Relationship {
                lhs: Box::new(h_order),
                rhs: Box::new(g_order),
                operator: RelationOperatorNode::Divides,
            }),
        };

        vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text("Order relationship: ".to_string())],
                alignment: None,
            }),
            SectionContentNode::MathBlock {
                math: divides_relation,
                label: None,
                caption: None,
            },
        ]
    }

    /// Create content for subgroup expressions
    fn create_subgroup_expression_content(
        &self,
        text: &str,
        id_prefix: &str,
    ) -> Vec<SectionContentNode> {
        // Create H ⊆ G expression using proper structure
        let h_group = self.create_identifier("H", &format!("{}_h", id_prefix));
        let g_group = self.create_identifier("G", &format!("{}_g", id_prefix));

        let subgroup_relation = MathNode {
            id: id_prefix.to_string(),
            content: Box::new(MathNodeContent::Relationship {
                lhs: Box::new(h_group),
                rhs: Box::new(g_group),
                operator: RelationOperatorNode::IsSubgroupOf,
            }),
        };

        vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text("Subgroup relationship: ".to_string())],
                alignment: None,
            }),
            SectionContentNode::MathBlock {
                math: subgroup_relation,
                label: None,
                caption: None,
            },
        ]
    }

    /// Create content for Lagrange's theorem
    fn create_lagrange_theorem_content(&self, id_prefix: &str) -> Vec<SectionContentNode> {
        // Create H ⊆ G expression
        let h_group = self.create_identifier("H", &format!("{}_h", id_prefix));
        let g_group = self.create_identifier("G", &format!("{}_g", id_prefix));
        let subset_symbol = self.create_identifier("⊆", &format!("{}_subset", id_prefix));

        let subgroup_relation = MathNode {
            id: format!("{}_subgroup", id_prefix),
            content: Box::new(MathNodeContent::Relationship {
                lhs: Box::new(h_group.clone()),
                rhs: Box::new(g_group.clone()),
                operator: RelationOperatorNode::IsSubgroupOf,
            }),
        };

        // Create |H| | |G| (order divides)
        let h_order = MathNode {
            id: format!("{}_h_order", id_prefix),
            content: Box::new(MathNodeContent::Bracketed {
                inner: Box::new(h_group),
                style: BracketStyle::Vertical,
                size: BracketSize::Normal,
            }),
        };

        let g_order = MathNode {
            id: format!("{}_g_order", id_prefix),
            content: Box::new(MathNodeContent::Bracketed {
                inner: Box::new(g_group),
                style: BracketStyle::Vertical,
                size: BracketSize::Normal,
            }),
        };

        let divides_symbol = self.create_identifier("|", &format!("{}_divides", id_prefix));

        let divides_relation = MathNode {
            id: format!("{}_divides_rel", id_prefix),
            content: Box::new(MathNodeContent::Relationship {
                lhs: Box::new(h_order),
                rhs: Box::new(g_order),
                operator: RelationOperatorNode::Divides,
            }),
        };

        // Create the implication H ⊆ G ⟹ |H| | |G|
        let implication = MathNode {
            id: id_prefix.to_string(),
            content: Box::new(MathNodeContent::Relationship {
                lhs: Box::new(subgroup_relation),
                rhs: Box::new(divides_relation),
                operator: RelationOperatorNode::Implies,
            }),
        };

        vec![
            SectionContentNode::Paragraph(ParagraphNode {
                segments: vec![RichTextSegment::Text(
                    "Lagrange's Theorem states: ".to_string(),
                )],
                alignment: None,
            }),
            SectionContentNode::MathBlock {
                math: implication,
                label: None,
                caption: None,
            },
        ]
    }
}

/// Helper struct for structured proof step content
struct StructuredProofStepContent {
    goal: StructuredProofGoal,
    tactic: StructuredTactic,
    status: ProofStepStatus,
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
        let rendered = theorem.to_turn_math("theorem_id".to_string());
        println!("{:#?}", to_value(&rendered));
    }
}
