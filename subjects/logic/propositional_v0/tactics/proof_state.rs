use super::{GenericProof, TacticResult, context::TacticalContext};
use crate::{
    formalize_v2::{
        foundational_theories::type_theory_v2::calculi::simply_typed::terms::Term,
        subjects::logic::propositional::{Foundation, Proposition},
    },
    parse::{Parse, entities::Identifier},
};

/// Status of a proof node's verification
#[derive(Debug, Clone)]
pub enum ProofVerification {
    /// The node has been verified successfully
    Success,
    /// The node failed verification with a specific error
    Failed(String),
    /// The node has not been verified yet
    Pending,
}

/// A node in the proof tree
#[derive(Debug, Clone)]
pub struct ProofNode {
    /// The tactical context at this node
    context: TacticalContext,
    /// The term being constructed at this node
    term: TacticProofTerm,
    /// The goal proposition at this node
    pub goal: Proposition,
    /// Child nodes in the proof tree
    children: Vec<ProofNode>,
    /// Verification status of this node
    verification: ProofVerification,
    /// Description of what this node is trying to prove
    description: String,
}

impl ProofNode {
    /// Create a new proof node
    pub fn new(context: TacticalContext, term: TacticProofTerm, goal: Proposition) -> Self {
        let description = format!("Proving: {}", goal);
        ProofNode {
            context,
            term,
            goal,
            children: Vec::new(),
            verification: ProofVerification::Pending,
            description,
        }
    }

    /// Add a child node
    pub fn add_child(&mut self, child: ProofNode) {
        self.children.push(child);
    }

    /// Get the context
    pub fn context(&self) -> &TacticalContext {
        &self.context
    }

    /// Get a mutable reference to the context
    pub fn context_mut(&mut self) -> &mut TacticalContext {
        &mut self.context
    }

    /// Get the term
    pub fn term(&self) -> &TacticProofTerm {
        &self.term
    }

    /// Get the goal
    pub fn goal(&self) -> &Proposition {
        &self.goal
    }

    /// Get all verification failures in this subtree
    pub fn get_failures(&self) -> Vec<(String, String)> {
        let mut failures = Vec::new();

        // Add this node's failure if any
        if let ProofVerification::Failed(error) = &self.verification {
            failures.push((self.description.clone(), error.clone()));
        }

        // Add failures from children
        for child in &self.children {
            failures.extend(child.get_failures());
        }

        failures
    }

    /// Set the verification status of this node
    pub fn set_verification(&mut self, status: ProofVerification) {
        self.verification = status;
    }

    /// Set a description for this proof step
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Checks that the tactic application completed the proof branch, and panics if not.
    pub fn should_complete(self) {
        match self.role {
            NodeRole::Completed => (),
            NodeRole::Goal(g) => {
                panic!(
                    "Tactic did not complete the proof. New goal generated: {:?}",
                    g
                )
            }
            other => panic!(
                "Tactic did not complete the proof. Unexpected node role: {:?}",
                other
            ),
        }
    }
}

/// The state of a proof in progress
#[derive(Debug, Clone)]
pub struct ProofState {
    /// The foundation being used
    pub foundation: Foundation,
    /// The root node of the proof tree
    root: ProofNode,
    /// The current node being worked on
    current: ProofNode,
    /// The final proof once completed
    pub conclusion: TacticProofTerm,
}

impl ProofState {
    /// Create a new proof state with an empty root node
    pub fn new(foundation: Foundation, goal: Proposition) -> Self {
        let context = TacticalContext::new();
        // Start with an incomplete proof that needs to be improved through tactics
        let term = TacticProofTerm::Incomplete;
        let root = ProofNode::new(context.clone(), term.clone(), goal);
        let current = root.clone();

        ProofState {
            foundation,
            root,
            current,
            conclusion: term,
        }
    }

    /// Get the current node's context
    pub fn current_context(&self) -> &TacticalContext {
        &self.current.context
    }

    /// Get a mutable reference to the current node's context
    pub fn current_context_mut(&mut self) -> &mut TacticalContext {
        &mut self.current.context
    }

    /// Get the current goal
    pub fn current_goal(&self) -> &Proposition {
        &self.current.goal
    }

    /// Get the current node
    pub fn current_node(&self) -> &ProofNode {
        &self.current
    }

    /// Get a mutable reference to the current node
    pub fn current_node_mut(&mut self) -> &mut ProofNode {
        &mut self.current
    }

    /// Add a child to the current node and update verification status
    pub fn add_child(&mut self, mut child: ProofNode) -> TacticResult<GenericProof> {
        // Verify the child's proof
        let verification = match child.term() {
            TacticProofTerm::Complete(term) => {
                let expected_type = self.foundation.convert_proposition_to_type(&child.goal);
                if self.foundation.verify_proof(
                    child.context().stlc_context(),
                    term,
                    &expected_type,
                ) {
                    ProofVerification::Success
                } else {
                    ProofVerification::Failed(format!("Failed to verify proof term: {:?}", term))
                }
            }
            TacticProofTerm::Incomplete => {
                ProofVerification::Failed("Proof is incomplete".to_string())
            }
        };

        child.set_verification(verification);
        self.current.add_child(child);
        Ok(GenericProof::default()) // TODO: Return actual proof
    }

    /// Set the current proof
    pub fn set_proof(&mut self, proof: GenericProof) {
        self.conclusion = TacticProofTerm::Complete(proof.term.clone());
    }

    /// Set the goal for the current node
    pub fn set_goal(&mut self, new_goal: Proposition) {
        self.current.goal = new_goal;
    }

    /// Get all verification failures in the proof tree
    pub fn get_all_failures(&self) -> Vec<(String, String)> {
        self.root.get_failures()
    }

    /// Set the term of the current node
    pub fn set_current_term(&mut self, term: TacticProofTerm) {
        self.current.term = term;
    }
}

#[derive(Debug, Clone)]
pub enum TacticProofTerm {
    /// No proof constructed yet
    Incomplete,
    /// A valid proof term
    Complete(Term),
}
