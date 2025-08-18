use super::super::proof::{ProofForest, ProofNode};
use crate::{
    subjects::math::formalism::proof::{NodeRole, SubgoalCombination, tactics::Tactic},
    turn_render::second_order_math_node::{
        ContextType, ContextVariableDisplay, ExpressionPosition, GoalDisplay, GoalVisualStyle,
        HandlerType, InstantiationDirection, InstantiationMap, InteractionHandler,
        InteractiveElement, InteractiveElementType, InteractiveExpression, InteractiveProofDisplay,
        PatternMatch, ProofColorScheme, ProofExpressionInteractionType, ProofForestDisplay,
        ProofInteractionConfig, ProofLayoutType, ProofNodeDisplay, ProofNodeVisualState,
        ProofTransformationData, ProofVisualConfig, ProofVisualStyle, TacticDisplay,
        TacticVisualStyle, ToInteractiveProofDisplay, ToProofForestDisplay, ToProofNodeDisplay,
        TransformationAnimationConfig, TransformationDisplay, TransformationWorkflowStage,
        VisualConnection,
    },
    turn_render::{
        BranchingContainer, BranchingNode, ContainerLayout, ContainerType, LayoutAlignment,
        LayoutDirection, LayoutType, MathNode, NodeState, NodeType, RichText, RichTextSegment,
        SecondOrderMathNode, Section, SectionContentNode, ToRichText, ToSectionNode, ToTurnMath,
        TransformationFlow,
    },
};

impl ProofForest {
    // build_proof_tree method removed since proof types are not exported from section_node

    /// Convert ProofForest to BranchingContainer for export
    pub fn to_branching_container(&self, id_prefix: &str) -> BranchingContainer {
        let mut nodes = Vec::new();
        let mut visited = std::collections::HashSet::new();

        // Perform depth-first traversal starting from each root
        for root_id in &self.roots {
            self.traverse_and_collect_nodes(root_id, &mut nodes, &mut visited, id_prefix);
        }

        BranchingContainer {
            container_id: format!("{}-proof-forest", id_prefix),
            container_type: ContainerType::ProofForest,
            nodes,
            layout_config: Some(ContainerLayout {
                layout_type: LayoutType::Tree,
                direction: LayoutDirection::TopDown,
                spacing: Some("20px".to_string()),
                alignment: Some(LayoutAlignment::Center),
                max_depth: Some(5),
                collapse_branches: Some(true),
            }),
            container_metadata: vec![
                ("type".to_string(), "proof_forest".to_string()),
                (
                    "initial_goal".to_string(),
                    format!("{:?}", self.initial_goal),
                ),
            ],
        }
    }

    /// Recursively traverse the proof tree and collect nodes in order
    fn traverse_and_collect_nodes(
        &self,
        node_id: &str,
        nodes: &mut Vec<BranchingNode>,
        visited: &mut std::collections::HashSet<String>,
        id_prefix: &str,
    ) {
        if visited.contains(node_id) {
            return; // Avoid cycles
        }

        visited.insert(node_id.to_string());

        if let Some(node) = self.get_node(node_id) {
            // Add current node
            let branching_node = node.to_branching_node(id_prefix);
            nodes.push(branching_node);

            // Recursively traverse children in order
            for child_id in &node.children {
                self.traverse_and_collect_nodes(child_id, nodes, visited, id_prefix);
            }
        }
    }

    /// Check if a branch starting at a given node is complete (for rendering)
    fn is_branch_complete_for_display(&self, node_id: &str) -> bool {
        if let Some(node) = self.get_node(node_id) {
            match &node.role {
                NodeRole::Completed => true,
                NodeRole::Goal(_) => {
                    if node.children.is_empty() {
                        false // Leaf goal node that's not marked complete
                    } else {
                        // Check if all children are complete
                        node.children
                            .iter()
                            .all(|child_id| self.is_branch_complete_for_display(child_id))
                    }
                }
                NodeRole::SubgoalManager {
                    subgoal_ids,
                    combination_type,
                } => {
                    let result = match combination_type {
                        SubgoalCombination::And => {
                            // All subgoals must be complete
                            subgoal_ids
                                .iter()
                                .all(|subgoal_id| self.is_branch_complete_for_display(subgoal_id))
                        }
                        SubgoalCombination::Or => {
                            // At least one subgoal must be complete
                            subgoal_ids
                                .iter()
                                .any(|subgoal_id| self.is_branch_complete_for_display(subgoal_id))
                        }
                        _ => false,
                    };
                    result
                }
                NodeRole::Disproved(_) => false,
                NodeRole::AutomatedTacticStep {
                    description,
                    justification,
                    best_node_id,
                } => {
                    let best_node = self.get_node(best_node_id);
                    best_node.is_some() && self.is_branch_complete_for_display(best_node_id)
                }
                NodeRole::RewriteStep {
                    goal,
                    rewritten_from_id,
                    rewritten_to_id,
                } => todo!(),
            }
        } else {
            false
        }
    }
}

impl ProofNode {
    /// Render tactic information as RichText
    pub fn render_tactic_info(&self, id_prefix: &str) -> RichText {
        use crate::turn_render::{RichTextSegment, TextStyle};

        let (tactic_name, interactive_elements) = match &self.tactic {
            Tactic::AssumeImplicationAntecedent { with_name } => {
                ("Assume Implication Antecedent".to_string(), vec![with_name.body.clone()])
            }
            Tactic::SplitGoalConjunction => {
                ("Split Goal Conjunction".to_string(), vec![])
            }
            Tactic::SplitGoalDisjunction { disjunct_index } => {
                ("Split Goal Disjunction".to_string(), vec![format!("disjunct {}", disjunct_index)])
            }
            Tactic::CaseAnalysis { on_variable, cases } => {
                ("Case Analysis".to_string(), vec![on_variable.body.clone(), format!("{} cases", cases.len())])
            }
            Tactic::Induction { variable_name, hypothesis_name } => {
                ("Induction".to_string(), vec![variable_name.body.clone(), hypothesis_name.body.clone()])
            }
            Tactic::ProvideWitness { target_quantifier, witness: _ } => {
                ("Provide Witness".to_string(), vec![target_quantifier.body.clone()])
            }
            Tactic::SplitAssumptionConjunction { target_hypothesis, with_names } => {
                let names: Vec<String> = with_names.iter().map(|id| id.body.clone()).collect();
                ("Split Assumption Conjunction".to_string(), vec![target_hypothesis.body.clone()])
            }
            Tactic::SplitAssumptionDisjunction { target_hypothesis, with_names } => {
                ("Split Assumption Disjunction".to_string(), vec![target_hypothesis.body.clone(), format!("{} cases", with_names.len())])
            }
            Tactic::ByRelation(relation_source) => {
                match relation_source {
                    crate::subjects::math::formalism::proof::tactics::RelationSource::LocalAssumption(id) => {
                        ("By Relation".to_string(), vec![id.body.clone()])
                    }
                    crate::subjects::math::formalism::proof::tactics::RelationSource::Theorem(theorem_id, _) => {
                        ("By Theorem".to_string(), vec![theorem_id.clone()])
                    }
                }
            }
            Tactic::ByReflexivity => {
                ("By Reflexivity".to_string(), vec![])
            }
            Tactic::ByContradiction { hypothesis1, hypothesis2 } => {
                ("By Contradiction".to_string(), vec![hypothesis1.body.clone(), hypothesis2.body.clone()])
            }
            Tactic::ByGoalContradiction { conflicting_hypothesis } => {
                ("By Goal Contradiction".to_string(), vec![conflicting_hypothesis.body.clone()])
            }
            Tactic::Rewrite { using_rule, target, direction, instantiations } => {
                let direction_str = match direction {
                    crate::subjects::math::formalism::proof::tactics::RewriteDirection::Forward => "forward",
                    crate::subjects::math::formalism::proof::tactics::RewriteDirection::Backward => "backward",
                };
                let rule_name = match using_rule {
                    crate::subjects::math::formalism::proof::tactics::RelationSource::LocalAssumption(id) => id.body.clone(),
                    crate::subjects::math::formalism::proof::tactics::RelationSource::Theorem(theorem_id, _) => theorem_id.clone(),
                };
                ("Rewrite".to_string(), vec![rule_name, direction_str.to_string(), format!("{} instantiations", instantiations.len())])
            }
            Tactic::UnfoldDefinition { definition_to_unfold, target: _ } => {
                ("Unfold Definition".to_string(), vec![definition_to_unfold.body.clone()])
            }
            Tactic::IntroduceLetBinding { target_expression: _, with_name } => {
                ("Introduce Let Binding".to_string(), vec![with_name.body.clone()])
            }
            Tactic::RenameBoundVariable { target: _, from_name, to_name } => {
                ("Rename Bound Variable".to_string(), vec![from_name.body.clone(), to_name.body.clone()])
            }
            Tactic::Revert { hypothesis_to_revert } => {
                ("Revert".to_string(), vec![hypothesis_to_revert.body.clone()])
            }
            Tactic::SearchAssumptions => {
                ("Search Assumptions".to_string(), vec![])
            }
            Tactic::SearchTheoremLibrary => {
                ("Search Theorem Library".to_string(), vec![])
            }
            Tactic::Search => {
                ("Search".to_string(), vec![])
            }
            Tactic::Simplify { target: _ } => {
                ("Simplify".to_string(), vec![])
            }
            Tactic::Auto { depth, with_tactics } => {
                let depth_str = depth.map(|d| d.to_string()).unwrap_or_else(|| "unlimited".to_string());
                ("Auto".to_string(), vec![depth_str, format!("{} tactics", with_tactics.len())])
            }
            Tactic::DisproveByTheorem { theorem_id } => {
                ("Disprove By Theorem".to_string(), vec![theorem_id.clone()])
            }
            Tactic::RefineVariable { variable, theorem_id } => {
                (format!("Refine {}", variable.body), vec![theorem_id.clone()])
            }
            
        };

        let mut segments = vec![
            RichTextSegment::StyledText {
                text: "Tactic: ".to_string(),
                styles: vec![TextStyle::Bold],
            },
            RichTextSegment::Text(tactic_name),
        ];

        // Add interactive elements if any
        if !interactive_elements.is_empty() {
            segments.push(RichTextSegment::StyledText {
                text: " (".to_string(),
                styles: vec![],
            });

            for (i, element) in interactive_elements.iter().enumerate() {
                if i > 0 {
                    segments.push(RichTextSegment::StyledText {
                        text: ", ".to_string(),
                        styles: vec![],
                    });
                }
                segments.push(RichTextSegment::StyledText {
                    text: element.clone(),
                    styles: vec![TextStyle::Italic],
                });
            }

            segments.push(RichTextSegment::StyledText {
                text: ")".to_string(),
                styles: vec![],
            });
        }

        RichText {
            segments,
            alignment: None,
        }
    }

    /// Get the display name of the tactic (human-readable)
    pub fn get_tactic_display_name(&self) -> String {
        match &self.tactic {
            Tactic::AssumeImplicationAntecedent { .. } => {
                "Assume Implication Antecedent".to_string()
            }
            Tactic::SplitGoalConjunction => "Split Goal Conjunction".to_string(),
            Tactic::SplitGoalDisjunction { .. } => "Split Goal Disjunction".to_string(),
            Tactic::CaseAnalysis { .. } => "Case Analysis".to_string(),
            Tactic::Induction { .. } => "Induction".to_string(),
            Tactic::ProvideWitness { .. } => "Provide Witness".to_string(),
            Tactic::SplitAssumptionConjunction { .. } => "Split Assumption Conjunction".to_string(),
            Tactic::SplitAssumptionDisjunction { .. } => "Split Assumption Disjunction".to_string(),
            Tactic::ByRelation(..) => "By Relation".to_string(),
            Tactic::ByReflexivity => "By Reflexivity".to_string(),
            Tactic::ByContradiction { .. } => "By Contradiction".to_string(),
            Tactic::ByGoalContradiction { .. } => "By Goal Contradiction".to_string(),
            Tactic::Rewrite { .. } => "Rewrite".to_string(),
            Tactic::UnfoldDefinition { .. } => "Unfold Definition".to_string(),
            Tactic::IntroduceLetBinding { .. } => "Introduce Let Binding".to_string(),
            Tactic::RenameBoundVariable { .. } => "Rename Bound Variable".to_string(),
            Tactic::Revert { .. } => "Revert".to_string(),
            Tactic::SearchAssumptions => "Search Assumptions".to_string(),
            Tactic::SearchTheoremLibrary => "Search Theorem Library".to_string(),
            Tactic::Search => "Search".to_string(),
            Tactic::Simplify { .. } => "Simplify".to_string(),
            Tactic::Auto { .. } => "Auto".to_string(),
            Tactic::DisproveByTheorem { .. } => "Disprove By Theorem".to_string(),
            Tactic::RefineVariable { .. } => "Refine Variable".to_string(),
        }
    }

    /// Get the interactive elements that the tactic operates on
    pub fn get_interactive_elements(&self) -> Vec<String> {
        match &self.tactic {
            Tactic::AssumeImplicationAntecedent { with_name } => {
                vec![with_name.body.clone()]
            }
            Tactic::SplitGoalConjunction => {
                vec![]
            }
            Tactic::SplitGoalDisjunction { disjunct_index } => {
                vec![format!("disjunct {}", disjunct_index)]
            }
            Tactic::CaseAnalysis { on_variable, cases } => {
                let mut elements = vec![on_variable.body.clone()];
                elements.push(format!("{} cases", cases.len()));
                elements
            }
            Tactic::Induction { variable_name, hypothesis_name } => {
                vec![variable_name.body.clone(), hypothesis_name.body.clone()]
            }
            Tactic::ProvideWitness { target_quantifier, witness: _ } => {
                vec![target_quantifier.body.clone()]
            }
            Tactic::SplitAssumptionConjunction { target_hypothesis, with_names } => {
                let mut elements = vec![target_hypothesis.body.clone()];
                elements.extend(with_names.iter().map(|id| id.body.clone()));
                elements
            }
            Tactic::SplitAssumptionDisjunction { target_hypothesis, with_names } => {
                let mut elements = vec![target_hypothesis.body.clone()];
                elements.push(format!("{} cases", with_names.len()));
                elements
            }
            Tactic::ByRelation(relation_source) => {
                match relation_source {
                    crate::subjects::math::formalism::proof::tactics::RelationSource::LocalAssumption(id) => {
                        vec![id.body.clone()]
                    }
                    crate::subjects::math::formalism::proof::tactics::RelationSource::Theorem(theorem_id, _) => {
                        vec![theorem_id.clone()]
                    }
                }
            }
            Tactic::ByReflexivity => {
                vec![]
            }
            Tactic::ByContradiction { hypothesis1, hypothesis2 } => {
                vec![hypothesis1.body.clone(), hypothesis2.body.clone()]
            }
            Tactic::ByGoalContradiction { conflicting_hypothesis } => {
                vec![conflicting_hypothesis.body.clone()]
            }
            Tactic::Rewrite { using_rule, target: _, direction, instantiations } => {
                let direction_str = match direction {
                    crate::subjects::math::formalism::proof::tactics::RewriteDirection::Forward => "forward",
                    crate::subjects::math::formalism::proof::tactics::RewriteDirection::Backward => "backward",
                };
                let rule_name = match using_rule {
                    crate::subjects::math::formalism::proof::tactics::RelationSource::LocalAssumption(id) => id.body.clone(),
                    crate::subjects::math::formalism::proof::tactics::RelationSource::Theorem(theorem_id, _) => theorem_id.clone(),
                };
                vec![rule_name, direction_str.to_string(), format!("{} instantiations", instantiations.len())]
            }
            Tactic::UnfoldDefinition { definition_to_unfold, target: _ } => {
                vec![definition_to_unfold.body.clone()]
            }
            Tactic::IntroduceLetBinding { target_expression: _, with_name } => {
                vec![with_name.body.clone()]
            }
            Tactic::RenameBoundVariable { target: _, from_name, to_name } => {
                vec![from_name.body.clone(), to_name.body.clone()]
            }
            Tactic::Revert { hypothesis_to_revert } => {
                vec![hypothesis_to_revert.body.clone()]
            }
            Tactic::SearchAssumptions => {
                vec![]
            }
            Tactic::SearchTheoremLibrary => {
                vec![]
            }
            Tactic::Search => {
                vec![]
            }
            Tactic::Simplify { target: _ } => {
                vec![]
            }
            Tactic::Auto { depth, with_tactics } => {
                let depth_str = depth.map(|d| d.to_string()).unwrap_or_else(|| "unlimited".to_string());
                vec![depth_str, format!("{} tactics", with_tactics.len())]
            }
            Tactic::DisproveByTheorem { theorem_id } => {
                vec![theorem_id.clone()]
            }
            Tactic::RefineVariable { variable, theorem_id } => {
                vec![variable.body.clone(), theorem_id.clone()]
            }
            
        }
    }

    /// Get the complete transformation data for interactive visualization
    pub fn get_transformation_data(&self, id_prefix: &str) -> ProofTransformationData {
        // Extract source expressions from the previous proof node's goal
        let source_expressions = self.extract_source_expressions(id_prefix);

        // Extract target expressions from the current proof node's goal
        let target_expressions = self.extract_target_expressions(id_prefix);

        // Generate pattern matches based on the tactic
        let pattern_matches = self.generate_pattern_matches(id_prefix);

        // Generate instantiation maps based on the tactic
        let instantiations = self.generate_instantiations(id_prefix);

        // Generate visual connections
        let visual_connections = self.get_transformation_flow().visual_connections;

        // Generate interactive elements
        let interactive_elements = self.generate_interactive_elements(id_prefix);

        ProofTransformationData {
            tactic_name: self.get_tactic_display_name(),
            workflow_stage: self.get_workflow_stage(),
            source_expressions,
            target_expressions,
            pattern_matches,
            instantiations,
            visual_connections,
            interactive_elements,
        }
    }

    /// Extract source expressions from the previous proof node
    fn extract_source_expressions(&self, id_prefix: &str) -> Vec<InteractiveExpression> {
        // For now, extract from the current node's goal context
        // In a real implementation, this would come from the previous node
        let mut expressions = Vec::new();

        if let NodeRole::Goal(proof_goal) = &self.role {
            // Add context variables as source expressions
            for (i, entry) in proof_goal.context.iter().enumerate() {
                expressions.push(InteractiveExpression {
                    id: format!("{}-context-{}", id_prefix, i),
                    expression: entry
                        .name
                        .to_turn_math(format!("{}-context-name-{}", id_prefix, entry.name)),
                    position: ExpressionPosition {
                        node_id: self.id.clone(),
                        context_type: ContextType::Hypothesis,
                        index: Some(i),
                        path: vec!["context".to_string(), i.to_string()],
                    },
                    interaction_type: ProofExpressionInteractionType::Highlightable,
                    metadata: std::collections::HashMap::new(),
                });
            }

            // Add the goal statement as a source expression
            expressions.push(InteractiveExpression {
                id: format!("{}-goal-statement", id_prefix),
                expression: proof_goal
                    .statement
                    .to_turn_math(format!("{}-goal-statement", id_prefix)),
                position: ExpressionPosition {
                    node_id: self.id.clone(),
                    context_type: ContextType::Goal,
                    index: None,
                    path: vec!["statement".to_string()],
                },
                interaction_type: ProofExpressionInteractionType::Transformable,
                metadata: std::collections::HashMap::new(),
            });
        }

        expressions
    }

    /// Extract target expressions from the current proof node
    fn extract_target_expressions(&self, id_prefix: &str) -> Vec<InteractiveExpression> {
        // Similar to source expressions but for the result
        let mut expressions = Vec::new();

        if let NodeRole::Goal(proof_goal) = &self.role {
            // Add the goal statement as a target expression
            expressions.push(InteractiveExpression {
                id: format!("{}-target-statement", id_prefix),
                expression: proof_goal
                    .statement
                    .to_turn_math(format!("{}-target-statement", id_prefix)),
                position: ExpressionPosition {
                    node_id: self.id.clone(),
                    context_type: ContextType::Goal,
                    index: None,
                    path: vec!["statement".to_string()],
                },
                interaction_type: ProofExpressionInteractionType::Highlightable,
                metadata: std::collections::HashMap::new(),
            });
        }

        expressions
    }

    /// Generate pattern matches based on the tactic
    fn generate_pattern_matches(&self, id_prefix: &str) -> Vec<PatternMatch> {
        let mut matches = Vec::new();

        match &self.tactic {
            Tactic::Rewrite {
                using_rule, target, ..
            } => {
                // Create a pattern match for the rewrite
                let rule_name = match using_rule {
                    crate::subjects::math::formalism::proof::tactics::RelationSource::LocalAssumption(id) => id.body.clone(),
                    crate::subjects::math::formalism::proof::tactics::RelationSource::Theorem(theorem_id, _) => theorem_id.clone(),
                };

                matches.push(PatternMatch {
                    pattern_id: format!("{}-rewrite-pattern", id_prefix),
                    source_expression: rule_name,
                    matched_expression: format!("{:?}", target),
                    confidence: 1.0,
                    substitution_map: std::collections::HashMap::new(),
                });
            }
            Tactic::AssumeImplicationAntecedent { with_name } => {
                // Create a pattern match for the implication assumption
                matches.push(PatternMatch {
                    pattern_id: format!("{}-implication-pattern", id_prefix),
                    source_expression: with_name.body.clone(),
                    matched_expression: "consequent".to_string(),
                    confidence: 1.0,
                    substitution_map: std::collections::HashMap::new(),
                });
            }
            _ => {
                // Default pattern match for other tactics
                matches.push(PatternMatch {
                    pattern_id: format!("{}-default-pattern", id_prefix),
                    source_expression: "input".to_string(),
                    matched_expression: "result".to_string(),
                    confidence: 0.8,
                    substitution_map: std::collections::HashMap::new(),
                });
            }
        }

        matches
    }

    /// Generate instantiation maps based on the tactic
    fn generate_instantiations(&self, id_prefix: &str) -> Vec<InstantiationMap> {
        let mut instantiations = Vec::new();

        match &self.tactic {
            Tactic::Rewrite {
                instantiations: tactic_instantiations,
                ..
            } => {
                for (i, instantiation) in tactic_instantiations.iter().enumerate() {
                    instantiations.push(InstantiationMap {
                        variable_name: format!("var_{}", i),
                        instantiated_value: MathNode::text("instantiated_value".to_string()),
                        source_expression: "pattern".to_string(),
                        target_expression: "target".to_string(),
                        direction: InstantiationDirection::Forward,
                    });
                }
            }
            _ => {
                // Default instantiation for other tactics
                instantiations.push(InstantiationMap {
                    variable_name: "default_var".to_string(),
                    instantiated_value: MathNode::text("default_value".to_string()),
                    source_expression: "source".to_string(),
                    target_expression: "target".to_string(),
                    direction: InstantiationDirection::Forward,
                });
            }
        }

        instantiations
    }

    /// Generate interactive elements based on the tactic
    fn generate_interactive_elements(&self, id_prefix: &str) -> Vec<InteractiveElement> {
        let mut elements = Vec::new();

        // Add the tactic itself as an interactive element
        elements.push(InteractiveElement {
            id: format!("{}-tactic", id_prefix),
            element_type: InteractiveElementType::Tactic,
            expression: None,
            text: Some(self.get_tactic_display_name()),
            position: ExpressionPosition {
                node_id: self.id.clone(),
                context_type: ContextType::Goal,
                index: None,
                path: vec!["tactic".to_string()],
            },
            interaction_handlers: vec![InteractionHandler {
                handler_type: HandlerType::Click,
                action: "show_details".to_string(),
                parameters: std::collections::HashMap::new(),
            }],
        });

        // Add interactive elements based on the tactic
        for element_name in self.get_interactive_elements() {
            let element_id = format!("{}-element-{}", id_prefix, element_name);
            let element_path = vec!["interactive".to_string(), element_name.clone()];

            elements.push(InteractiveElement {
                id: element_id,
                element_type: InteractiveElementType::Expression,
                expression: None,
                text: Some(element_name),
                position: ExpressionPosition {
                    node_id: self.id.clone(),
                    context_type: ContextType::Goal,
                    index: None,
                    path: element_path,
                },
                interaction_handlers: vec![InteractionHandler {
                    handler_type: HandlerType::Highlight,
                    action: "highlight_element".to_string(),
                    parameters: std::collections::HashMap::new(),
                }],
            });
        }

        elements
    }

    /// Get the workflow stage for this transformation
    fn get_workflow_stage(&self) -> TransformationWorkflowStage {
        match &self.tactic {
            Tactic::Rewrite { .. } => TransformationWorkflowStage::Replace,
            Tactic::AssumeImplicationAntecedent { .. } => TransformationWorkflowStage::Prescribe,
            Tactic::SplitAssumptionConjunction { .. } => TransformationWorkflowStage::Search,
            Tactic::ByReflexivity => TransformationWorkflowStage::Verify,
            _ => TransformationWorkflowStage::Search,
        }
    }

    /// Get the transformation flow information for interactive visualization
    pub fn get_transformation_flow(&self) -> TransformationFlow {
        match &self.tactic {
            Tactic::AssumeImplicationAntecedent { with_name } => TransformationFlow {
                tactic_type: "introduction".to_string(),
                direction: "forward".to_string(),
                source_elements: vec![with_name.body.clone()],
                target_elements: vec!["consequent".to_string()],
                transformation_type: "implication_assumption".to_string(),
                visual_connections: vec![VisualConnection {
                    from: with_name.body.clone(),
                    to: "consequent".to_string(),
                    connection_type: "data_flow".to_string(),
                    style: "green_line".to_string(),
                }],
            },
            Tactic::Rewrite {
                using_rule,
                target,
                direction,
                instantiations,
            } => {
                let direction_str = match direction {
                    crate::subjects::math::formalism::proof::tactics::RewriteDirection::Forward => "forward",
                    crate::subjects::math::formalism::proof::tactics::RewriteDirection::Backward => "backward",
                };
                let rule_name = match using_rule {
                    crate::subjects::math::formalism::proof::tactics::RelationSource::LocalAssumption(id) => id.body.clone(),
                    crate::subjects::math::formalism::proof::tactics::RelationSource::Theorem(theorem_id, _) => theorem_id.clone(),
                };

                TransformationFlow {
                    tactic_type: "structural".to_string(),
                    direction: direction_str.to_string(),
                    source_elements: vec![rule_name.clone()],
                    target_elements: vec![format!("{:?}", target)],
                    transformation_type: "rewrite".to_string(),
                    visual_connections: vec![VisualConnection {
                        from: rule_name,
                        to: format!("{:?}", target),
                        connection_type: "rewrite_arrow".to_string(),
                        style: if direction_str == "forward" {
                            "blue_forward".to_string()
                        } else {
                            "blue_backward".to_string()
                        },
                    }],
                }
            }
            Tactic::SplitAssumptionConjunction {
                target_hypothesis,
                with_names,
            } => TransformationFlow {
                tactic_type: "elimination".to_string(),
                direction: "forward".to_string(),
                source_elements: vec![target_hypothesis.body.clone()],
                target_elements: with_names.iter().map(|id| id.body.clone()).collect(),
                transformation_type: "conjunction_split".to_string(),
                visual_connections: with_names
                    .iter()
                    .map(|id| VisualConnection {
                        from: target_hypothesis.body.clone(),
                        to: id.body.clone(),
                        connection_type: "split_arrow".to_string(),
                        style: "green_line".to_string(),
                    })
                    .collect(),
            },
            Tactic::ByReflexivity => TransformationFlow {
                tactic_type: "completion".to_string(),
                direction: "forward".to_string(),
                source_elements: vec![],
                target_elements: vec!["reflexive_equality".to_string()],
                transformation_type: "reflexivity".to_string(),
                visual_connections: vec![VisualConnection {
                    from: "equality".to_string(),
                    to: "reflexive_equality".to_string(),
                    connection_type: "completion_arrow".to_string(),
                    style: "green_circle".to_string(),
                }],
            },
            _ => {
                // Default transformation flow for other tactics
                TransformationFlow {
                    tactic_type: self.get_tactic_category(),
                    direction: "forward".to_string(),
                    source_elements: self.get_interactive_elements(),
                    target_elements: vec!["result".to_string()],
                    transformation_type: "general".to_string(),
                    visual_connections: vec![VisualConnection {
                        from: "input".to_string(),
                        to: "result".to_string(),
                        connection_type: "general_arrow".to_string(),
                        style: "blue_line".to_string(),
                    }],
                }
            }
        }
    }

    /// Get the category of the tactic (e.g., "introduction", "elimination", "completion")
    pub fn get_tactic_category(&self) -> String {
        match &self.tactic {
            Tactic::AssumeImplicationAntecedent { .. }
            | Tactic::SplitGoalConjunction
            | Tactic::SplitGoalDisjunction { .. }
            | Tactic::CaseAnalysis { .. }
            | Tactic::Induction { .. }
            | Tactic::ProvideWitness { .. } => "introduction".to_string(),
            Tactic::SplitAssumptionConjunction { .. }
            | Tactic::SplitAssumptionDisjunction { .. } => "elimination".to_string(),
            Tactic::ByRelation(..)
            | Tactic::ByReflexivity
            | Tactic::ByContradiction { .. }
            | Tactic::ByGoalContradiction { .. } => "completion".to_string(),
            Tactic::Rewrite { .. }
            | Tactic::UnfoldDefinition { .. }
            | Tactic::IntroduceLetBinding { .. }
            | Tactic::RenameBoundVariable { .. }
            | Tactic::Revert { .. } => "structural".to_string(),
            Tactic::SearchAssumptions
            | Tactic::SearchTheoremLibrary
            | Tactic::Search
            | Tactic::Simplify { .. }
            | Tactic::Auto { .. } => "automated".to_string(),
            Tactic::DisproveByTheorem { .. } => "meta-logical".to_string(),
            Tactic::RefineVariable { .. } => "type-roles".to_string(),
            
        }
    }

    /// Get a human-readable description of what the tactic transformation does
    pub fn get_transformation_description(&self) -> String {
        match &self.tactic {
            Tactic::AssumeImplicationAntecedent { with_name } => {
                format!(
                    "Adds {} to context and changes goal to consequent",
                    with_name
                )
            }
            Tactic::SplitGoalConjunction => "Splits goal into two sub-goals (A and B)".to_string(),
            Tactic::SplitGoalDisjunction { disjunct_index } => {
                format!("Focuses on disjunct {} of the goal", disjunct_index)
            }
            Tactic::CaseAnalysis { on_variable, cases } => {
                format!(
                    "Performs case analysis on {} with {} cases",
                    on_variable,
                    cases.len()
                )
            }
            Tactic::Induction {
                variable_name,
                hypothesis_name,
            } => {
                format!(
                    "Sets up induction on {} with hypothesis {}",
                    variable_name, hypothesis_name
                )
            }
            Tactic::ProvideWitness {
                target_quantifier, ..
            } => {
                format!(
                    "Provides concrete witness for existential quantifier {}",
                    target_quantifier
                )
            }
            Tactic::SplitAssumptionConjunction {
                target_hypothesis,
                with_names,
            } => {
                format!(
                    "Splits assumption {} into {} parts",
                    target_hypothesis,
                    with_names.len()
                )
            }
            Tactic::SplitAssumptionDisjunction {
                target_hypothesis,
                with_names,
            } => {
                format!(
                    "Splits assumption {} into {} cases",
                    target_hypothesis,
                    with_names.len()
                )
            }
            Tactic::ByRelation(relation_source) => {
                format!("Applies relation: {:?}", relation_source)
            }
            Tactic::ByReflexivity => "Solves equality t = t by reflexivity".to_string(),
            Tactic::ByContradiction {
                hypothesis1,
                hypothesis2,
            } => {
                format!(
                    "Solves goal by contradiction between {} and {}",
                    hypothesis1, hypothesis2
                )
            }
            Tactic::ByGoalContradiction {
                conflicting_hypothesis,
            } => {
                format!(
                    "Solves goal by contradiction with hypothesis {}",
                    conflicting_hypothesis
                )
            }
            Tactic::Rewrite {
                using_rule,
                target,
                direction,
                instantiations,
            } => {
                let direction_str = match direction {
                    crate::subjects::math::formalism::proof::tactics::RewriteDirection::Forward => "forward",
                    crate::subjects::math::formalism::proof::tactics::RewriteDirection::Backward => "backward",
                };
                format!(
                    "Rewrites {:?} using {:?} in {} direction with {} instantiations",
                    target,
                    using_rule,
                    direction_str,
                    instantiations.len()
                )
            }
            Tactic::UnfoldDefinition {
                definition_to_unfold,
                target,
            } => {
                format!(
                    "Unfolds definition {} at target {:?}",
                    definition_to_unfold, target
                )
            }
            Tactic::IntroduceLetBinding {
                target_expression,
                with_name,
            } => {
                format!(
                    "Introduces let binding {} for expression {:?}",
                    with_name, target_expression
                )
            }
            Tactic::RenameBoundVariable {
                target,
                from_name,
                to_name,
            } => {
                format!(
                    "Renames bound variable from {} to {} at target {:?}",
                    from_name, to_name, target
                )
            }
            Tactic::Revert {
                hypothesis_to_revert,
            } => {
                format!(
                    "Moves hypothesis {} back into goal as implication",
                    hypothesis_to_revert
                )
            }
            Tactic::SearchAssumptions => "Searches context for matching hypothesis".to_string(),
            Tactic::SearchTheoremLibrary => {
                "Searches theorem library for applicable theorem".to_string()
            }
            Tactic::Search => "Searches assumptions and theorem library".to_string(),
            Tactic::Simplify { target } => {
                format!("Simplifies expression at target {:?}", target)
            }
            Tactic::Auto {
                depth,
                with_tactics,
            } => {
                let depth_str = depth
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "unlimited".to_string());
                format!(
                    "Automated proof search with depth {} and {} tactics",
                    depth_str,
                    with_tactics.len()
                )
            }
            Tactic::DisproveByTheorem { theorem_id } => {
                format!("Disproves goal using theorem {}", theorem_id)
            }
            Tactic::RefineVariable { variable, theorem_id } => {
                format!("Refines {} using equality {}", variable.body, theorem_id)
            }
        }
    }

    /// Convert ProofNode to BranchingNode for export
    pub fn to_branching_node(&self, id_prefix: &str) -> BranchingNode {
        let node_type = match &self.role {
            NodeRole::Goal(_) => NodeType::ProofGoal,
            NodeRole::SubgoalManager { .. } => NodeType::ProofManager,
            NodeRole::Completed => NodeType::ProofCompleted,
            NodeRole::Disproved(_) => NodeType::ProofDisproved,
            NodeRole::AutomatedTacticStep { .. } => NodeType::ProofStep,
            NodeRole::RewriteStep { .. } => NodeType::ProofStep,
        };

        let node_state = match &self.role {
            NodeRole::Completed => NodeState::Completed,
            NodeRole::Disproved(_) => NodeState::Disproved,
            NodeRole::Goal(_) => NodeState::Active,
            _ => NodeState::Active,
        };

        let mut content = Vec::new();

        // Add tactic information with detailed description
        let tactic_info = self.render_tactic_info(id_prefix);
        content.push(SectionContentNode::RichText(tactic_info));

        // Add node-specific content based on role
        match &self.role {
            NodeRole::Goal(proof_goal) => {
                // Render the proof goal as a Judgement
                content.push(SectionContentNode::SecondOrderMath(
                    SecondOrderMathNode::Judgement(
                        proof_goal.to_judgement(&format!("{}-goal", id_prefix)),
                    ),
                ));
            }
            NodeRole::SubgoalManager {
                subgoal_ids,
                combination_type,
            } => {
                content.push(SectionContentNode::RichText(RichText {
                    segments: vec![
                        RichTextSegment::StyledText {
                            text: "Managing ".to_string(),
                            styles: vec![crate::turn_render::TextStyle::Bold],
                        },
                        RichTextSegment::Text(format!("{} sub-goals", subgoal_ids.len())),
                        RichTextSegment::StyledText {
                            text: format!(" ({:?})", combination_type),
                            styles: vec![crate::turn_render::TextStyle::Italic],
                        },
                    ],
                    alignment: None,
                }));
            }
            NodeRole::AutomatedTacticStep {
                description,
                justification,
                best_node_id,
            } => {
                content.push(SectionContentNode::RichText(description.clone()));
                content.push(SectionContentNode::RichText(RichText {
                    segments: vec![
                        RichTextSegment::StyledText {
                            text: "Best outcome: ".to_string(),
                            styles: vec![crate::turn_render::TextStyle::Bold],
                        },
                        RichTextSegment::Text(format!("Node {}", best_node_id)),
                    ],
                    alignment: None,
                }));
            }
            NodeRole::Disproved(theorem_id) => {
                content.push(SectionContentNode::RichText(RichText {
                    segments: vec![
                        RichTextSegment::StyledText {
                            text: "Disproved by theorem: ".to_string(),
                            styles: vec![crate::turn_render::TextStyle::Bold],
                        },
                        RichTextSegment::Text(theorem_id.clone()),
                    ],
                    alignment: None,
                }));
            }
            NodeRole::RewriteStep {
                goal,
                rewritten_from_id,
                rewritten_to_id,
            } => {
                content.push(SectionContentNode::RichText(RichText {
                    segments: vec![
                        RichTextSegment::StyledText {
                            text: "Rewrite: ".to_string(),
                            styles: vec![crate::turn_render::TextStyle::Bold],
                        },
                        RichTextSegment::Text(format!(
                            "{:?} → {:?}",
                            rewritten_from_id, rewritten_to_id
                        )),
                    ],
                    alignment: None,
                }));
                // Add the resulting goal
                content.push(SectionContentNode::SecondOrderMath(
                    SecondOrderMathNode::Judgement(
                        goal.to_judgement(&format!("{}-rewrite-goal", id_prefix)),
                    ),
                ));
            }
            NodeRole::Completed => {
                content.push(SectionContentNode::RichText(RichText {
                    segments: vec![RichTextSegment::StyledText {
                        text: "✓ Completed".to_string(),
                        styles: vec![crate::turn_render::TextStyle::Bold],
                    }],
                    alignment: None,
                }));
            }
        }

        BranchingNode {
            node_id: self.id.clone(),
            parent_id: self.parent.clone(),
            node_type,
            content,
            node_metadata: vec![
                ("tactic".to_string(), self.get_tactic_display_name()),
                ("tactic_type".to_string(), self.get_tactic_category()),
                (
                    "transformation".to_string(),
                    self.get_transformation_description(),
                ),
                (
                    "interactive_elements".to_string(),
                    self.get_interactive_elements().join(", "),
                ),
                (
                    "transformation_flow".to_string(),
                    serde_json::to_string(&self.get_transformation_flow()).unwrap_or_default(),
                ),
                (
                    "transformation_data".to_string(),
                    serde_json::to_string(&self.get_transformation_data(id_prefix))
                        .unwrap_or_default(),
                ),
                (
                    "children_count".to_string(),
                    self.children.len().to_string(),
                ),
                ("node_role".to_string(), format!("{:?}", self.role)),
            ],
            children: self.children.clone(),
            node_state,
        }
    }
}

// ToProofDisplay trait implementation removed since proof types are not exported from section_node

impl ToSectionNode for ProofForest {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        // Convert ProofForest to BranchingContainer
        let branching_container = self.to_branching_container(id_prefix);

        Section {
            id: format!("{}-proof-forest", id_prefix),
            title: None,
            content: SectionContentNode::BranchingContainer(branching_container),
            metadata: vec![],
            display_options: None,
        }
    }
}

impl ToSectionNode for ProofNode {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        // For now, just create a simple section with the node info
        Section {
            id: format!("{}-proof-node", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text(format!("Proof Node {}", self.id))],
                alignment: None,
            }),
            content: SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!("Tactic: {:?}", self.tactic))],
                alignment: None,
            }),
            metadata: vec![],
            display_options: None,
        }
    }
}

impl ToProofNodeDisplay for ProofNode {
    fn to_proof_node_display(&self, id_prefix: &str, step_number: usize) -> ProofNodeDisplay {
        // Create tactic display
        let tactic_display = TacticDisplay {
            tactic_name: self.get_tactic_display_name(),
            tactic_type: self.get_tactic_category(),
            description: self.get_transformation_description(),
            interactive_elements: self.get_interactive_elements(),
            workflow_stage: self.get_workflow_stage(),
            visual_style: match self.get_tactic_category().as_str() {
                "introduction" => TacticVisualStyle::Introduction,
                "elimination" => TacticVisualStyle::Elimination,
                "structural" => TacticVisualStyle::Structural,
                "completion" => TacticVisualStyle::Completion,
                "automated" => TacticVisualStyle::Automated,
                _ => TacticVisualStyle::Default,
            },
        };

        // Create goal display
        let goal_display = if let NodeRole::Goal(proof_goal) = &self.role {
            let context_variables = proof_goal
                .context
                .iter()
                .enumerate()
                .map(|(i, entry)| ContextVariableDisplay {
                    variable_name: entry
                        .name
                        .to_turn_math(format!("{}-context-name-{}", id_prefix, entry.name)),
                    variable_type: match &entry.ty.data {
                        crate::subjects::math::formalism::extract::Parametrizable::Concrete(
                            arc_expr,
                        ) => arc_expr.to_rich_text(),
                        crate::subjects::math::formalism::extract::Parametrizable::Variable(id) => {
                            RichText {
                                segments: vec![RichTextSegment::Text(id.to_string())],
                                alignment: None,
                            }
                        }
                    },
                    is_highlighted: false,
                    interaction_handlers: vec![InteractionHandler {
                        handler_type: HandlerType::Highlight,
                        action: "highlight_variable".to_string(),
                        parameters: std::collections::HashMap::new(),
                    }],
                })
                .collect();

            GoalDisplay {
                context_variables,
                goal_statement: proof_goal
                    .statement
                    .to_turn_math(format!("{}-goal-statement", id_prefix)),
                visual_style: GoalVisualStyle::Standard,
            }
        } else {
            GoalDisplay {
                context_variables: vec![],
                goal_statement: MathNode::text("No goal".to_string()),
                visual_style: GoalVisualStyle::Standard,
            }
        };

        // Create transformation display
        let transformation_display = Some(TransformationDisplay {
            source_expressions: self.extract_source_expressions(id_prefix),
            target_expressions: self.extract_target_expressions(id_prefix),
            pattern_matches: self.generate_pattern_matches(id_prefix),
            instantiations: self.generate_instantiations(id_prefix),
            visual_connections: self.get_transformation_flow().visual_connections,
            interactive_elements: self.generate_interactive_elements(id_prefix),
            animation_config: Some(TransformationAnimationConfig {
                duration_ms: 500,
                easing: "ease-in-out".to_string(),
                show_progress: true,
                highlight_source: true,
                highlight_target: true,
                show_connections: true,
            }),
        });

        // Determine visual state
        let visual_state = match &self.role {
            NodeRole::Completed => ProofNodeVisualState::Completed,
            NodeRole::Goal(_) => ProofNodeVisualState::Active,
            _ => ProofNodeVisualState::Normal,
        };

        ProofNodeDisplay {
            node_id: self.id.clone(),
            step_number,
            tactic_display,
            goal_display,
            transformation_display,
            children: vec![], // Will be populated by parent
            visual_state,
        }
    }
}

impl ToProofForestDisplay for ProofForest {
    fn to_proof_forest_display(&self, id_prefix: &str) -> ProofForestDisplay {
        let mut root_nodes = Vec::new();
        let mut step_counter = 1;

        // Convert root nodes to display format
        for root_id in &self.roots {
            if let Some(node) = self.get_node(root_id) {
                let mut display_node = node.to_proof_node_display(id_prefix, step_counter);
                step_counter += 1;

                // Recursively convert children
                display_node.children =
                    self.convert_children_to_display(root_id, id_prefix, &mut step_counter);

                root_nodes.push(display_node);
            }
        }

        ProofForestDisplay {
            forest_id: format!("{}-proof-forest", id_prefix),
            root_nodes,
            layout_type: ProofLayoutType::Tree,
            visual_style: ProofVisualStyle::Interactive,
        }
    }

    fn convert_children_to_display(
        &self,
        parent_id: &str,
        id_prefix: &str,
        step_counter: &mut usize,
    ) -> Vec<ProofNodeDisplay> {
        let mut children = Vec::new();

        if let Some(parent_node) = self.get_node(parent_id) {
            for child_id in &parent_node.children {
                if let Some(child_node) = self.get_node(child_id) {
                    let mut display_child =
                        child_node.to_proof_node_display(id_prefix, *step_counter);
                    *step_counter += 1;

                    // Recursively convert grandchildren
                    display_child.children =
                        self.convert_children_to_display(child_id, id_prefix, step_counter);

                    children.push(display_child);
                }
            }
        }

        children
    }
}

impl ToInteractiveProofDisplay for ProofForest {
    fn to_interactive_proof_display(&self, id_prefix: &str) -> InteractiveProofDisplay {
        let proof_forest = self.to_proof_forest_display(id_prefix);

        // Collect all transformation data from all nodes
        let mut transformation_data = Vec::new();
        self.collect_transformation_data(id_prefix, &mut transformation_data);

        InteractiveProofDisplay {
            id: format!("{}-interactive-proof", id_prefix),
            title: "Interactive Proof Visualization".to_string(),
            proof_forest,
            transformation_data,
            visual_config: ProofVisualConfig {
                layout_type: ProofLayoutType::Tree,
                visual_style: ProofVisualStyle::Interactive,
                animation_enabled: true,
                show_connections: true,
                show_interactive_elements: true,
                color_scheme: ProofColorScheme {
                    primary_color: "#2563eb".to_string(),
                    secondary_color: "#64748b".to_string(),
                    accent_color: "#f59e0b".to_string(),
                    success_color: "#10b981".to_string(),
                    error_color: "#ef4444".to_string(),
                    warning_color: "#f59e0b".to_string(),
                    info_color: "#3b82f6".to_string(),
                },
            },
            interaction_config: ProofInteractionConfig {
                allow_click_interactions: true,
                allow_hover_interactions: true,
                allow_drag_interactions: false,
                allow_selection: true,
                allow_highlighting: true,
                interaction_handlers: vec![InteractionHandler {
                    handler_type: HandlerType::Click,
                    action: "show_transformation_details".to_string(),
                    parameters: std::collections::HashMap::new(),
                }],
            },
        }
    }

    fn collect_transformation_data(
        &self,
        id_prefix: &str,
        data: &mut Vec<ProofTransformationData>,
    ) {
        for node in self.node_values() {
            data.push(node.get_transformation_data(id_prefix));
        }
    }
}

// Add a simple export function for proof forests
impl ProofForest {
    /// Export proof forest as a SecondOrderMathNode for rendering
    pub fn to_second_order_math_node(
        &self,
        id_prefix: &str,
    ) -> crate::turn_render::SecondOrderMathNode {
        let interactive_display = self.to_interactive_proof_display(id_prefix);
        crate::turn_render::SecondOrderMathNode::InteractiveProof(interactive_display)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subjects::math::formalism::expressions::MathExpression;
    use crate::subjects::math::formalism::location::Located;
    use crate::subjects::math::formalism::proof::tactics::Tactic;
    use crate::subjects::math::formalism::proof::{NodeRole, ProofGoal, ProofNode};
    use crate::subjects::math::formalism::relations::MathRelation;

    #[test]
    fn test_proof_forest_export() {
        // Create a simple proof forest for testing
        let initial_goal = ProofGoal {
            context: vec![],
            quantifiers: vec![],
            statement: Located::new_concrete(MathRelation::True),
        };

        let mut forest = ProofForest::new_from_goal(initial_goal);

        // Add a simple proof node
        let proof_goal = ProofGoal {
            context: vec![],
            quantifiers: vec![],
            statement: Located::new_concrete(MathRelation::True),
        };

        let proof_node = ProofNode {
            id: "test-node".to_string(),
            parent: None,
            children: vec![],
            role: NodeRole::Goal(proof_goal),
            tactic: Tactic::ByReflexivity,
            description: None,
        };

        forest.add_node(proof_node);
        forest.roots.push("test-node".to_string());

        // Test the export function
        let exported = forest.to_second_order_math_node("test");

        // Verify it's the right type
        match exported {
            crate::turn_render::SecondOrderMathNode::InteractiveProof(_) => {
                // Success - the export worked
            }
            _ => panic!("Expected InteractiveProof variant"),
        }
    }
}
