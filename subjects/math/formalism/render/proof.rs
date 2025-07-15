use super::super::proof::{ProofForest, ProofNode};
use crate::{
    subjects::math::formalism::proof::{NodeRole, SubgoalCombination},
    turn_render::section_node::{
        ProofDisplayNode, ProofStepNode, RichText, RichTextSegment, ToProofDisplay,
    },
};

impl ProofForest {
    /// Build a tree of ProofDisplayNodes from a single root node
    fn build_proof_tree(&self, node: &ProofNode) -> ProofDisplayNode {
        let mut steps = Vec::new();

        // Add this node as a step
        steps.push(node.to_tactic_display_node(self));

        // For each child, either add it as a step or create a sub-tree
        for child_id in &node.children {
            if let Some(child_node) = self.get_node(child_id) {
                if child_node.children.is_empty() {
                    // Leaf node - add as a step
                    steps.push(child_node.to_tactic_display_node(self));
                } else {
                    // Non-leaf node - recursively build its subtree
                    let child_tree = self.build_proof_tree(child_node);
                    // Add the child tree's steps to our steps
                    steps.extend(child_tree.steps);
                }
            }
        }

        // Determine if this branch is complete
        let is_complete = self.is_branch_complete_for_display(&node.id);

        ProofDisplayNode {
            title: None,
            strategy: vec![],
            steps,
            qed_symbol: if is_complete {
                Some("∎".to_string())
            } else {
                None
            },
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

impl ToProofDisplay for ProofForest {
    fn to_proof_display(&self) -> ProofDisplayNode {
        // If there's only one root, return its tree directly
        if self.roots.len() == 1 {
            if let Some(root_node) = self.get_node(&self.roots[0]) {
                let mut tree = self.build_proof_tree(root_node);
                tree.title = Some(RichText {
                    segments: vec![RichTextSegment::Text("Proof".to_string())],
                    alignment: None,
                });
                return tree;
            }
        }

        // Multiple roots - create a master node containing all trees
        let mut all_steps = Vec::new();
        for (i, root_id) in self.roots.iter().enumerate() {
            if let Some(root_node) = self.get_node(root_id) {
                let tree = self.build_proof_tree(root_node);

                // Add a branch header if multiple roots
                if self.roots.len() > 1 {
                    all_steps.push(ProofStepNode::TacticApplication(
                        crate::turn_render::section_node::TacticDisplayNode::Auto {
                            automated_tactic_type:
                                crate::turn_render::section_node::AutomatedTacticDisplay::Auto {
                                    search_tree: None,
                                    successful_tactics: vec![],
                                    failed_attempts: vec![],
                                },
                            search_depth: Some(0),
                            tactics_attempted: vec![],
                            successful_path: None,
                            execution_summary: RichText {
                                segments: vec![RichTextSegment::Text(format!("Branch {}", i + 1))],
                                alignment: None,
                            },
                        },
                    ));
                }

                all_steps.extend(tree.steps);
            }
        }

        // Check if the entire proof is complete
        let is_fully_complete = self
            .roots
            .iter()
            .all(|root_id| self.is_branch_complete_for_display(root_id));

        ProofDisplayNode {
            title: Some(RichText {
                segments: vec![RichTextSegment::Text("Proof".to_string())],
                alignment: None,
            }),
            strategy: vec![],
            steps: all_steps,
            qed_symbol: if is_fully_complete {
                Some("∎".to_string())
            } else {
                None
            },
        }
    }

    fn to_proof_display_vec(&self) -> Vec<ProofDisplayNode> {
        // Return each root as a separate proof tree
        let mut trees = Vec::new();

        for (i, root_id) in self.roots.iter().enumerate() {
            if let Some(root_node) = self.get_node(root_id) {
                let mut tree = self.build_proof_tree(root_node);
                tree.title = Some(RichText {
                    segments: vec![RichTextSegment::Text(if self.roots.len() > 1 {
                        format!("Proof Branch {}", i + 1)
                    } else {
                        "Proof".to_string()
                    })],
                    alignment: None,
                });
                trees.push(tree);
            }
        }

        if trees.is_empty() {
            // Fallback: return a single empty proof
            vec![ProofDisplayNode {
                title: Some(RichText {
                    segments: vec![RichTextSegment::Text("Proof".to_string())],
                    alignment: None,
                }),
                strategy: vec![],
                steps: vec![],
                qed_symbol: None,
            }]
        } else {
            trees
        }
    }
}

impl ProofNode {
    fn to_tactic_display_node(&self, _forest: &ProofForest) -> ProofStepNode {
        let tactic_display = self.tactic.to_display_node();
        ProofStepNode::TacticApplication(tactic_display)
    }
}

impl ToProofDisplay for ProofNode {
    fn to_proof_display(&self) -> ProofDisplayNode {
        // For a single ProofNode, create a simplified display
        let tactic_display = self.tactic.to_display_node();
        let step = ProofStepNode::TacticApplication(tactic_display);

        ProofDisplayNode {
            title: None,
            strategy: vec![],
            steps: vec![step],
            qed_symbol: None,
        }
    }
}
