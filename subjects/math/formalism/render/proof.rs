use crate::{
    subjects::math::formalism::proof::{ProofForest, ProofNode},
    turn_render::section_node::{
        ProofDisplayNode, ProofStepNode, RichText, RichTextSegment, Section, SectionContentNode,
        SectionDisplayOptions, ToProofDisplay,
    },
};

impl ToProofDisplay for ProofForest {
    fn to_proof_display(&self) -> ProofDisplayNode {
        // Convert the forest to a single proof display
        // Since the parent-child relationships aren't properly maintained,
        // we'll collect all nodes that have tactics (excluding the initial root)
        let mut proof_steps: Vec<ProofStepNode> = self
            .node_values()
            .filter(|node| node.tactic.is_some()) // Only nodes with tactics applied
            .map(|node| node.to_proof_step_node(self))
            .collect();

        // Sort by creation order (this is a heuristic since we don't have explicit ordering)
        // In a proper implementation, we'd maintain the proof chain order

        ProofDisplayNode {
            title: Some(RichText {
                segments: vec![RichTextSegment::Text("Proof".to_string())],
                alignment: None,
            }),
            strategy: vec![],
            steps: proof_steps,
            qed_symbol: Some("∎".to_string()),
        }
    }

    fn to_proof_display_vec(&self) -> Vec<ProofDisplayNode> {
        vec![self.to_proof_display()]
    }
}

impl ProofNode {
    fn to_proof_step_node(&self, forest: &ProofForest) -> ProofStepNode {
        let tactic_name = self
            .tactic
            .as_ref()
            .map_or("Unknown".to_string(), |t| t.to_string());

        // For now, convert to a simple statement
        ProofStepNode::Statement {
            claim: vec![RichTextSegment::Text(format!(
                "Apply tactic: {}",
                tactic_name
            ))],
            justification: vec![RichTextSegment::Text("Tactic application".to_string())],
        }
    }
}

impl ToProofDisplay for ProofNode {
    fn to_proof_display(&self) -> ProofDisplayNode {
        // For a single ProofNode, we can't access the forest context
        // So we create a simplified display
        let tactic_name = self
            .tactic
            .as_ref()
            .map_or("Unknown".to_string(), |t| t.to_string());

        ProofDisplayNode {
            title: None,
            strategy: vec![],
            steps: vec![ProofStepNode::Statement {
                claim: vec![RichTextSegment::Text(format!(
                    "Apply tactic: {}",
                    tactic_name
                ))],
                justification: vec![RichTextSegment::Text("Tactic application".to_string())],
            }],
            qed_symbol: None,
        }
    }
}
