use crate::subjects::math::formalism::proof::{ProofForest, ProofNode};
use crate::turn_render::section_node::{
    ProofDisplayNode, ProofStepNode, RichText, RichTextSegment, ToProofDisplay,
};

impl ToProofDisplay for ProofForest {
    fn to_proof_display(&self) -> ProofDisplayNode {
        let steps: Vec<ProofStepNode> = self
            .node_values()
            .map(|node| node.to_tactic_display_node(self))
            .collect();

        ProofDisplayNode {
            title: Some(RichText {
                segments: vec![RichTextSegment::Text("Proof".to_string())],
                alignment: None,
            }),
            strategy: vec![],
            steps,
            qed_symbol: Some("∎".to_string()),
        }
    }

    fn to_proof_display_vec(&self) -> Vec<ProofDisplayNode> {
        vec![self.to_proof_display()]
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
