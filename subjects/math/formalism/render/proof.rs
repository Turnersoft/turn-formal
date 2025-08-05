use super::super::proof::{ProofForest, ProofNode};
use crate::{
    subjects::math::formalism::proof::{NodeRole, SubgoalCombination},
    turn_render::{
        BranchingContainer, BranchingNode, ContainerLayout, ContainerType, LayoutAlignment,
        LayoutDirection, LayoutType, NodeState, NodeType, RichText, RichTextSegment, Section,
        SectionContentNode, ToSectionNode,
    },
};

impl ProofForest {
    // build_proof_tree method removed since proof types are not exported from section_node

    /// Convert ProofForest to BranchingContainer for export
    pub fn to_branching_container(&self, id_prefix: &str) -> BranchingContainer {
        let mut nodes = Vec::new();

        // Convert all ProofNodes to BranchingNodes using public methods
        for node in self.node_values() {
            let branching_node = node.to_branching_node(id_prefix);
            nodes.push(branching_node);
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

        // Add tactic description
        if let Some(description) = &self.description {
            content.push(SectionContentNode::RichText(description.clone()));
        }

        // Add goal content if it's a goal node
        if let NodeRole::Goal(goal) = &self.role {
            content.push(SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::StyledText {
                    text: "Goal: ".to_string(),
                    styles: vec![crate::turn_render::TextStyle::Bold],
                }],
                alignment: None,
            }));
            // Note: Full goal rendering would require converting ProofGoal to SectionContentNode
            // For now, we'll add a placeholder
            content.push(SectionContentNode::RichText(RichText {
                segments: vec![RichTextSegment::Text(format!(
                    "{:?} variables, {:?} quantifiers",
                    goal.context.len(),
                    goal.quantifiers.len()
                ))],
                alignment: None,
            }));
        }

        // Add tactic information
        content.push(SectionContentNode::RichText(RichText {
            segments: vec![
                RichTextSegment::StyledText {
                    text: "Tactic: ".to_string(),
                    styles: vec![crate::turn_render::TextStyle::Bold],
                },
                RichTextSegment::Text(format!("{:?}", self.tactic)),
            ],
            alignment: None,
        }));

        BranchingNode {
            node_id: self.id.clone(),
            parent_id: self.parent.clone(),
            node_type,
            content,
            node_metadata: vec![
                ("tactic".to_string(), format!("{:?}", self.tactic)),
                (
                    "children_count".to_string(),
                    self.children.len().to_string(),
                ),
            ],
            children: self.children.clone(),
            node_state,
        }
    }
}

// ToProofDisplay trait implementation removed since proof types are not exported from section_node

impl ToSectionNode for ProofForest {
    fn to_section_node(&self, id_prefix: &str) -> Section {
        let mut content = vec![];

        for node in self.node_values() {
            content.push(node.to_section_node(&format!("{}-node", id_prefix)));
        }
        Section {
            id: format!("{}-proof-forest", id_prefix),
            title: Some(RichText {
                segments: vec![RichTextSegment::Text("Proof Forest".to_string())],
                alignment: None,
            }),
            content: SectionContentNode::SubSection(content),
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
