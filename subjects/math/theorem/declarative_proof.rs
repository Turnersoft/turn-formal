// Module: src/formalize_v2/subjects/math/theorem/declarative_proof.rs
// Implements a declarative structure for proof creation where the entire proof
// tree is defined upfront, and success/failure is determined by execution.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use super::core::{ProofState, Theorem};
use super::expressions::MathExpression;
use super::proof::{NodeId, ProofForest, ProofNode, ProofStatus, Tactic};
use super::relations::MathRelation;

/// Status of a step in the proof process
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepStatus {
    /// Successfully completed
    Complete,
    /// In progress but making headway
    InProgress,
    /// Todo item for later
    Todo,
    /// Work in progress (active development)
    Wip,
    /// Abandoned (won't pursue further)
    Abandoned,
    /// Failed due to an error
    Failed(String),
    /// Not executed yet
    Pending,
}

/// A single step in a proof
#[derive(Debug, Clone)]
pub struct Step {
    /// The tactic to apply
    pub tactic: Tactic,
    /// Branches from this step (case analysis, alternative approaches)
    pub branches: Vec<Branch>,
    /// Notes about this step
    pub description: Option<String>,
    /// Status of this step (determined during execution)
    pub status: StepStatus,
    /// Unique identifier for this step (assigned during execution)
    pub node_id: Option<NodeId>,
    /// Path identifier for structured naming (assigned during execution)
    pub path_id: Option<String>,
}

/// A branch in a proof, containing a sequence of steps
#[derive(Debug, Clone)]
pub struct Branch {
    /// Steps in this branch
    pub steps: Vec<Step>,
    /// Branch description
    pub description: Option<String>,
    /// Status of this branch (determined during execution)
    pub status: StepStatus,
}

/// A collection of cases that share a common parent
#[derive(Debug, Clone)]
pub struct CaseAnalysis {
    /// The main tactic for the case analysis
    pub tactic: Tactic,
    /// Individual cases
    pub cases: Vec<Branch>,
    /// Description of the case analysis
    pub description: Option<String>,
}

/// A tree of proof steps
#[derive(Debug, Clone)]
pub struct ProofTree {
    /// The main branch of the proof
    pub main_branch: Branch,
    /// Status of the overall proof (determined during execution)
    pub status: StepStatus,
}

impl Step {
    /// Create a new step with a tactic
    pub fn new(tactic: Tactic) -> Self {
        Self {
            tactic,
            branches: Vec::new(),
            description: None,
            status: StepStatus::Pending,
            node_id: None,
            path_id: None,
        }
    }

    /// Add a description to this step
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Add a branch to this step
    pub fn with_branch(mut self, branch: Branch) -> Self {
        self.branches.push(branch);
        self
    }

    /// Add multiple branches to this step
    pub fn with_branches(mut self, branches: Vec<Branch>) -> Self {
        self.branches.extend(branches);
        self
    }
}

impl Branch {
    /// Create a new branch with a sequence of steps
    pub fn new(steps: Vec<Step>) -> Self {
        Self {
            steps,
            description: None,
            status: StepStatus::Pending,
        }
    }

    /// Add a description to this branch
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
}

impl ProofTree {
    /// Create a new proof tree with a main branch
    pub fn new(main_branch: Branch) -> Self {
        Self {
            main_branch,
            status: StepStatus::Pending,
        }
    }
}

/// A builder for constructing declarative proofs
pub struct DeclarativeProofBuilder {
    /// Name of the theorem
    pub name: String,
    /// Statement of the theorem
    pub statement: MathRelation,
    /// Assumptions required for the theorem
    pub assumptions: Vec<MathRelation>,
    /// The proof tree
    pub proof: ProofTree,
}

impl DeclarativeProofBuilder {
    /// Create a new declarative proof builder
    pub fn new(name: &str, statement: MathRelation, assumptions: Vec<MathRelation>) -> Self {
        Self {
            name: name.to_string(),
            statement,
            assumptions,
            proof: ProofTree::new(Branch::new(Vec::new())),
        }
    }

    /// Set the proof tree
    pub fn with_proof(mut self, proof: ProofTree) -> Self {
        self.proof = proof;
        self
    }

    /// Execute the proof and build the theorem
    pub fn build(self) -> Theorem {
        // Create a proof forest to execute the proof
        let mut forest = ProofForest::new();

        // Create the initial proof state
        let initial_state = ProofState {
            quantifier: vec![],
            value_variables: vec![],
            statement: self.statement.clone(),
            path: Some("p0".to_string()),
            justification: None,
        };

        // Add the initial node to the forest
        let root_id = forest.add_node(
            None,
            initial_state.clone(),
            None,
            format!("Initial state for theorem: {}", self.name),
            ProofStatus::InProgress,
        );

        // Execute the proof tree, starting with the main branch
        let _executed_proof = self.execute_branch(
            &mut forest,
            root_id,
            &self.proof.main_branch,
            "p0".to_string(),
        );

        // Create the theorem
        let name = self.name.clone();
        Theorem {
            id: format!("thm_{}", name.to_lowercase().replace(' ', "_")),
            name,
            description: format!("Theorem: {}", self.name),
            initial_proof_state: initial_state,
        }
    }

    /// Execute a branch in the proof tree
    fn execute_branch(
        &self,
        forest: &mut ProofForest,
        parent_id: NodeId,
        branch: &Branch,
        path_prefix: String,
    ) -> StepStatus {
        let mut current_node_id = parent_id;
        let mut current_path = path_prefix;
        let mut branch_status = StepStatus::Pending;

        for (i, step) in branch.steps.iter().enumerate() {
            // Apply the tactic to the current state
            let parent_node = forest.get_node(current_node_id).unwrap();
            let new_state = step.tactic.apply(&parent_node.state);

            // Create a new step path
            let step_path = format!("{}_{}", current_path, i + 1);

            // Add the step to the forest
            let step_node_id = forest.add_node(
                Some(current_node_id),
                new_state,
                Some(step.tactic.clone()),
                step.description
                    .clone()
                    .unwrap_or_else(|| step.tactic.describe()),
                ProofStatus::InProgress,
            );

            // Update the current node and path
            current_node_id = step_node_id;
            current_path = step_path.clone();

            // Execute branch branches
            let mut branch_statuses = Vec::new();

            for (j, branch) in step.branches.iter().enumerate() {
                let branch_path = format!("{}_b{}", step_path, j + 1);
                let branch_status =
                    self.execute_branch(forest, current_node_id, branch, branch_path);
                branch_statuses.push(branch_status);
            }

            // Determine step status based on branch statuses
            let step_status = if branch_statuses.is_empty() {
                StepStatus::Complete // No branches, assume step is complete
            } else if branch_statuses.iter().any(|s| *s == StepStatus::Complete) {
                StepStatus::Complete // At least one branch is complete
            } else if branch_statuses
                .iter()
                .all(|s| matches!(*s, StepStatus::Failed(_)))
            {
                StepStatus::Failed("All branches failed".to_string())
            } else {
                StepStatus::InProgress // Mix of statuses, consider in progress
            };

            // Update the step's node status
            let proof_status = match step_status {
                StepStatus::Complete => ProofStatus::Complete,
                StepStatus::InProgress => ProofStatus::InProgress,
                StepStatus::Todo => ProofStatus::Todo,
                StepStatus::Wip => ProofStatus::Wip,
                StepStatus::Abandoned => ProofStatus::Abandoned,
                StepStatus::Failed(_) => ProofStatus::Abandoned,
                StepStatus::Pending => ProofStatus::Todo,
            };

            if let Some(node) = forest.get_node_mut(step_node_id) {
                node.status = proof_status;
            }

            // If this step fails, stop execution of this branch
            if let StepStatus::Failed(_) = step_status {
                branch_status = step_status;
                break;
            }

            // Update branch status based on the last step
            if i == branch.steps.len() - 1 {
                branch_status = step_status;
            }
        }

        // Update the branch status based on the steps executed
        branch_status
    }
}

/// Convenience functions for creating proof components
pub mod tactics {
    use super::super::proof::{DecompositionMethod, InductionType, RewriteDirection};
    use super::*;

    /// Create an 'Intro' tactic
    pub fn intro(var_name: &str, sequence: u32) -> Tactic {
        Tactic::Intro(var_name.to_string(), sequence as usize)
    }

    /// Create a 'Substitution' tactic
    pub fn subs(expression: &str, sequence: u32) -> Tactic {
        Tactic::Substitution(expression.to_string(), sequence as usize)
    }

    /// Create a 'TheoremApplication' tactic
    pub fn theorem_app(theorem_id: &str) -> Tactic {
        Tactic::TheoremApplication(theorem_id.to_string(), HashMap::new())
    }

    /// Create a 'Rewrite' tactic
    pub fn rewrite(target: &str, equation: &str, direction: RewriteDirection) -> Tactic {
        Tactic::Rewrite {
            target: target.to_string(),
            equation: equation.to_string(),
            direction,
        }
    }

    /// Create a 'Simplify' tactic
    pub fn simplify(expression: &str) -> Tactic {
        Tactic::Simplify(expression.to_string())
    }

    /// Create a 'Decompose' tactic
    pub fn decompose(target: &str, method: DecompositionMethod) -> Tactic {
        Tactic::Decompose {
            target: target.to_string(),
            method,
        }
    }

    /// Create a 'CaseAnalysis' tactic
    pub fn case_analysis(target: &str, cases: Vec<String>) -> Tactic {
        Tactic::CaseAnalysis {
            target: target.to_string(),
            cases,
        }
    }

    /// Create an 'Induction' tactic
    pub fn induction(variable: &str, induction_type: InductionType) -> Tactic {
        Tactic::Induction {
            variable: variable.to_string(),
            induction_type,
        }
    }
}

/// Convenience methods for creating a declarative theorem proof
pub mod proof_builder {
    use super::*;

    /// Create a step with an intro tactic
    pub fn intro(var_name: &str, sequence: u32) -> Step {
        Step::new(super::tactics::intro(var_name, sequence))
    }

    /// Create a step with a substitution tactic
    pub fn subs(expression: &str, sequence: u32) -> Step {
        Step::new(super::tactics::subs(expression, sequence))
    }

    /// Create a step with a theorem application tactic
    pub fn theorem_app(theorem_id: &str) -> Step {
        Step::new(super::tactics::theorem_app(theorem_id))
    }

    /// Create a step with a rewrite tactic
    pub fn rewrite(
        target: &str,
        equation: &str,
        direction: super::super::proof::RewriteDirection,
    ) -> Step {
        Step::new(super::tactics::rewrite(target, equation, direction))
    }

    /// Create a step with a simplify tactic
    pub fn simplify(expression: &str) -> Step {
        Step::new(super::tactics::simplify(expression))
    }

    /// Create a step with a decompose tactic
    pub fn decompose(target: &str, method: super::super::proof::DecompositionMethod) -> Step {
        Step::new(super::tactics::decompose(target, method))
    }

    /// Create a step with a case analysis tactic
    pub fn case_analysis(target: &str, cases: Vec<String>) -> Step {
        Step::new(super::tactics::case_analysis(target, cases))
    }

    /// Create a step with an induction tactic
    pub fn induction(variable: &str, induction_type: super::super::proof::InductionType) -> Step {
        Step::new(super::tactics::induction(variable, induction_type))
    }

    /// Create a branch with a sequence of steps
    pub fn branch(steps: Vec<Step>) -> Branch {
        Branch::new(steps)
    }

    /// Create a proof tree with a main branch
    pub fn proof_tree(main_branch: Branch) -> ProofTree {
        ProofTree::new(main_branch)
    }
}

// Implementation of Display for better debugging
impl fmt::Display for StepStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StepStatus::Complete => write!(f, "Complete"),
            StepStatus::InProgress => write!(f, "InProgress"),
            StepStatus::Todo => write!(f, "Todo"),
            StepStatus::Wip => write!(f, "Wip"),
            StepStatus::Abandoned => write!(f, "Abandoned"),
            StepStatus::Failed(reason) => write!(f, "Failed({})", reason),
            StepStatus::Pending => write!(f, "Pending"),
        }
    }
}

// Implementation of Display for ProofTree
impl fmt::Display for ProofTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ProofTree({} steps, status: {})",
            self.main_branch.steps.len(),
            self.status
        )
    }
}

// Implementation of Display for Branch
impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = self.description.as_deref().unwrap_or("Unnamed branch");
        write!(
            f,
            "Branch({}, {} steps, status: {})",
            desc,
            self.steps.len(),
            self.status
        )
    }
}

// Implementation of Display for Step
impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = self.description.as_deref().unwrap_or("Unnamed step");
        let path = self.path_id.as_deref().unwrap_or("No path");
        write!(
            f,
            "Step({}, tactic: {:?}, path: {}, status: {})",
            desc, self.tactic, path, self.status
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::proof::RewriteDirection;
    use super::tactics::*;
    use super::*;

    #[test]
    fn test_declarative_proof() {
        // Create a theorem statement
        let left = MathExpression::string_expr("a * (b * c)");
        let right = MathExpression::string_expr("(a * b) * c");

        // Build a complex proof tree
        let main_proof = Branch::new(vec![
            // Step 1: Introduce variable
            Step::new(intro("a", 1))
                .with_description("Introduce variable 'a'")
                .with_branch(
                    Branch::new(vec![
                        // Step 1.1: Deep nesting example
                        Step::new(intro("x", 3)).with_branch(Branch::new(vec![
                            // Step 1.1.1: Even deeper
                            Step::new(subs("y", 4)).with_branch(Branch::new(vec![
                                // Step 1.1.1.1: Deepest level
                                Step::new(intro("deeper", 5)).with_branch(Branch::new(vec![
                                    Step::new(theorem_app("identity")),
                                ])),
                            ])),
                        ])),
                    ])
                    .with_description("Deep nesting branch"),
                ),
            // Step 2: Apply substitution
            Step::new(subs("b+c", 2))
                .with_description("Substitute with 'b+c'")
                .with_branch(Branch::new(vec![
                    Step::new(intro("alternative", 7)).with_description("Alternative approach"),
                ])),
            // Step 3: Apply a theorem to complete the proof
            Step::new(theorem_app("associativity")).with_description("Apply associativity theorem"),
        ]);

        // Create the proof tree
        let proof_tree = ProofTree::new(main_proof);

        // Build the theorem
        let builder = DeclarativeProofBuilder::new(
            "Group Associativity",
            MathRelation::equal(left, right),
            vec![],
        )
        .with_proof(proof_tree);

        let theorem = builder.build();

        // Verify the theorem was created
        assert_eq!(theorem.name, "Group Associativity");
    }

    #[test]
    fn test_case_analysis() {
        // Example of how to use case analysis
        let absolute_value = MathExpression::string_expr("|x|");
        let zero = MathExpression::string_expr("0");

        // Create case analysis branches
        let cases_branches = vec![
            // Case 1: x ≥ 0
            Branch::new(vec![
                Step::new(intro("x ≥ 0 implies |x| = x", 1)),
                Step::new(subs("|x| = x", 2)),
                Step::new(theorem_app("x_geq_0_implies_x_geq_0")),
            ])
            .with_description("Case x ≥ 0"),
            // Case 2: x < 0
            Branch::new(vec![
                Step::new(intro("x < 0 implies |x| = -x", 1)),
                Step::new(subs("|x| = -x", 2)),
                Step::new(intro("For x < 0, -x > 0", 3)),
                Step::new(theorem_app("x_lt_0_implies_neg_x_gt_0")),
            ])
            .with_description("Case x < 0"),
        ];

        // Create the main proof branch
        let main_branch = Branch::new(vec![
            // Step 1: Set up the proof
            Step::new(intro("x as a real number", 1)).with_description("Set up the proof"),
            // Step 2: Case analysis
            Step::new(case_analysis(
                "x",
                vec!["x ≥ 0".to_string(), "x < 0".to_string()],
            ))
            .with_branches(cases_branches)
            .with_description("Analyze cases based on sign of x"),
        ]);

        // Create the proof tree
        let proof_tree = ProofTree::new(main_branch);

        // Build the theorem
        let builder = DeclarativeProofBuilder::new(
            "Absolute Value Non-Negativity",
            MathRelation::greater_than_or_equal(absolute_value, zero),
            vec![],
        )
        .with_proof(proof_tree);

        let theorem = builder.build();

        // Verify the theorem was created
        assert_eq!(theorem.name, "Absolute Value Non-Negativity");
    }
}
