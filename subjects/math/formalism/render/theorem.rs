use super::super::theorem::{Quantification, QuantifiedMathObject};
use super::super::{
    proof::{ProofForest, ProofNode},
    theorem::{ProofGoal, Theorem},
};
use crate::turn_render::{
    MathNode, MathNodeContent, MulSymbol, QuantificationNode, RefinedMulOrDivOperation, ToTurnMath,
};
// Importing ProofStatus
use super::super::proof::ProofStatus;

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
            .map(|(i, v)| MathNode {
                id: format!("{}:variable_{}", master_id, i),
                content: Box::new(MathNodeContent::Text(format!("{:?}", v))),
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
                ProofStatus::Complete => "✓ Complete",
                ProofStatus::InProgress => "⟳ In Progress",
                ProofStatus::Todo => "⌛ Todo",
                ProofStatus::Wip => "⚙ Work in Progress",
                ProofStatus::Abandoned => "✗ Abandoned",
            };

            let status_node = MathNode {
                id: format!("{}:status", branch_id),
                content: Box::new(MathNodeContent::Text(status_text.to_string())),
            };

            // Create tactic indicator if available
            let tactic_node = if let Some(tactic) = &node.tactic {
                let tactic_desc = tactic.describe();
                MathNode {
                    id: format!("{}:tactic", branch_id),
                    content: Box::new(MathNodeContent::Text(format!("Via: {}", tactic_desc))),
                }
            } else {
                MathNode {
                    id: format!("{}:tactic", branch_id),
                    content: Box::new(MathNodeContent::Text("Initial state".to_string())),
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
