use crate::subjects::math::formalism::proof::{ProofForest, ProofNode};
use crate::turn_render::section_node::{
    ProofDisplayNode, ProofStepNode, RichText, RichTextSegment, ToProofDisplay,
};

impl ToProofDisplay for ProofForest {
    fn to_proof_display(&self) -> ProofDisplayNode {
        let steps: Vec<ProofStepNode> = self
            .node_values()
            .filter(|node| node.tactic.is_some())
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
        match &self.tactic {
            Some(tactic) => {
                let tactic_display = tactic.to_display_node();
                ProofStepNode::TacticApplication(tactic_display)
            }
            None => {
                // For nodes without tactics, create a simple goal statement
                ProofStepNode::Goal(RichText {
                    segments: vec![RichTextSegment::Text("Initial goal".to_string())],
                    alignment: None,
                })
            }
        }
    }
}

impl ToProofDisplay for ProofNode {
    fn to_proof_display(&self) -> ProofDisplayNode {
        // For a single ProofNode, create a simplified display
        let step = match &self.tactic {
            Some(tactic) => {
                let tactic_display = tactic.to_display_node();
                ProofStepNode::TacticApplication(tactic_display)
            }
            None => {
                return ProofDisplayNode {
                    title: Some(RichText {
                        segments: vec![RichTextSegment::Text("Initial State".to_string())],
                        alignment: None,
                    }),
                    strategy: vec![],
                    steps: vec![],
                    qed_symbol: None,
                };
            }
        };

        ProofDisplayNode {
            title: None,
            strategy: vec![],
            steps: vec![step],
            qed_symbol: None,
        }
    }
}
