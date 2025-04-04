// Module: src/formalize_v2/subjects/math/visualization/models.rs
// Model data structures for theorem visualization

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default::Default;

use crate::formalize_v2::subjects::math::theorem::core::{MathContext, ProofState, Theorem};
use crate::formalize_v2::subjects::math::theorem::proof::ProofBranch;
use crate::formalize_v2::subjects::math::theorem::relations::MathRelation;

/// Represents a theory and its theorems for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheoryVisualization {
    /// Name of the theory
    pub name: String,

    /// Context of the theory
    pub context: MathContext,

    /// Description of the theory
    pub description: String,

    /// Theorems in this theory
    pub theorems: Vec<Theorem>,
}

/// Represents a theorem and its proof steps for visualization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TheoremVisualization {
    /// The theorem being visualized
    pub theorem: Theorem,

    /// Proof steps organized by branch
    pub proof_branches: Vec<ProofBranchVisualization>,

    /// Dependencies between branches
    pub branch_dependencies: HashMap<String, Vec<String>>,
}

/// Represents a proof branch for visualization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofBranchVisualization {
    /// Identifier for this branch
    pub id: String,

    /// Name or description of this branch
    pub name: String,

    /// Proof steps in this branch
    pub steps: Vec<ProofStepVisualization>,
}

/// Represents a proof step for visualization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofStepVisualization {
    /// Step identifier (e.g., "p1", "p2.1", etc.)
    pub id: String,

    /// The proof state at this step
    pub state: ProofState,

    /// The tactic used to reach this state
    pub tactic: String,

    /// Whether this step completes the proof
    pub is_complete: bool,
}

/// Represents a loaded math theory library
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathLibrary {
    /// All loaded theories
    pub theories: HashMap<String, TheoryVisualization>,

    /// All loaded theorems organized by theory
    pub theorems_by_theory: HashMap<String, Vec<String>>,
}

impl TheoremVisualization {
    /// Create a new theorem visualization from a theorem
    pub fn new(theorem: Theorem) -> Self {
        // Initialize with empty proof structure
        Self {
            theorem,
            proof_branches: Vec::new(),
            branch_dependencies: HashMap::new(),
        }
    }
}

impl ProofStepVisualization {
    /// Create a new proof step visualization from a proof state
    pub fn new(id: String, state: ProofState, tactic: String, is_complete: bool) -> Self {
        Self {
            id,
            state,
            tactic,
            is_complete,
        }
    }
}

impl Default for TheoremVisualization {
    fn default() -> Self {
        // Create a very minimal default instance
        TheoremVisualization {
            theorem: Theorem {
                id: "default".to_string(),
                name: "Default Theorem".to_string(),
                description: "Default theorem (placeholder)".to_string(),
                initial_proof_state: ProofState {
                    statement: MathRelation::custom("default".to_string(), Vec::new()),
                    path: None,
                    justification: None,
                    value_variables: Vec::new(),
                    quantifier: Vec::new(),
                },
            },
            proof_branches: Vec::new(),
            branch_dependencies: HashMap::new(),
        }
    }
}

impl Default for TheoryVisualization {
    fn default() -> Self {
        // Create a very minimal default instance
        TheoryVisualization {
            name: "Default Theory".to_string(),
            context: MathContext::GroupTheory, // Default to group theory
            description: "Default theory (placeholder)".to_string(),
            theorems: Vec::new(),
        }
    }
}
