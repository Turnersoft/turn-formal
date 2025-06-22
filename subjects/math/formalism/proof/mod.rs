// Module: src/formalize_v2/subjects/math/theorem/proof.rs
// Implements a rich proof structure for mathematical theorems with branching support

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::mem::{Discriminant, discriminant};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};
use tactics::{TacticApplicationResult, TheoremApplicationError};
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Removed unused imports from the refactored traversal module
// use self::traversal::{PotentialTheoremTarget, TargetCollector};
use super::super::theories::zfc::definitions::SetRelation;
use super::expressions::{MathExpression, TheoryExpression};
use super::interpretation::TypeViewOperator;
use super::relations::{MathRelation, RelationDetail};
use super::{objects::MathObject, theorem::Theorem};
// Import the new traversal trait if needed, or rely on inherent methods
use self::collect::CollectSubExpressions;

use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::theories::groups::definitions::GroupExpression;
use crate::subjects::math::theories::rings::definitions::{FieldExpression, RingExpression};
use crate::turn_render::{
    Identifier, MathNode, MathNodeContent, ProofDisplayNode, ProofStepNode, RichText,
    RichTextSegment, Section, SectionContentNode, ToProofDisplay, ToProofStep, ToTurnMath,
};

pub mod collect;
pub mod helpers;
pub mod path_index;
pub mod tactics;

// Re-export the tactics types for backward compatibility
pub use tactics::{
    AutomatedTactic, CaseAnalysisBuilder, CaseCondition, CaseResult, DecompositionMethod,
    InductionType, RewriteDirection, Tactic, TargetRelationLocation,
};

// Re-export relations types for external use
pub use super::relations::Quantification;

// Re-export helpers for ergonomic subgoal extraction
pub use helpers::*;

// Add the missing TheoremRegistry and get_theorem_registry
lazy_static! {
    static ref THEOREM_REGISTRY: Arc<Mutex<TheoremRegistry>> =
        Arc::new(Mutex::new(TheoremRegistry::new()));
}

pub fn get_theorem_registry() -> Arc<Mutex<TheoremRegistry>> {
    THEOREM_REGISTRY.clone()
}

#[derive(Debug, Clone)]
pub struct TheoremRegistry {
    theorems: HashMap<String, Theorem>,
}

impl TheoremRegistry {
    pub fn new() -> Self {
        Self {
            theorems: HashMap::new(),
        }
    }

    pub fn register(&mut self, id: String, theorem: Theorem) {
        self.theorems.insert(id, theorem);
    }

    pub fn get(&self, id: &str) -> Option<&Theorem> {
        self.theorems.get(id)
    }

    pub fn list_ids(&self) -> Vec<String> {
        self.theorems.keys().cloned().collect()
    }

    pub fn register_globally(theorem: Theorem) {
        let registry = get_theorem_registry();
        let mut registry = registry.lock().unwrap();
        registry.register(theorem.id.clone(), theorem);
    }
}

/// Represents a single named entry in a proof context. This can be a variable
/// declaration, a hypothesis, or a local definition (abbreviation).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContextEntry {
    pub name: Identifier,

    /// The "type" of the entry.
    /// - For a variable `g` of type `Group`, `ty` holds the `Group` type object.
    /// - For a hypothesis `h: g > 0`, `ty` holds the proposition `g > 0`.
    pub ty: MathExpression,

    /// An optional definition for local abbreviations (`let y = x + 1`).
    /// If `None`, this is a declared variable or hypothesis.
    /// If `Some`, this is a definition that can be unfolded (delta-reduction).
    pub definition: Option<MathExpression>,

    pub description: Option<String>,
}

/// Represents a quantified variable in the main statement's prenex form.
/// The full definition of the variable is found by looking up its name in the context.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Quantifier {
    pub variable_name: Identifier,
    pub quantification: Quantification, // Universal or Existential
}

/// Represents a complete state in a proof, structured with a unified, ordered context.
/// It cleanly separates the context (declarations, hypotheses, definitions) from the
/// core statement to be proven.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofGoal {
    /// A single, **ordered list** representing the proof context.
    /// Order is crucial: an entry can only refer to names defined in previous entries.
    /// This unified context holds all variables, hypotheses, and definitions.
    pub context: Vec<ContextEntry>,

    /// The ordered list of quantifiers for the main statement's prenex form.
    /// Each `variable_name` here MUST have a corresponding entry in the `context`.
    pub quantifiers: Vec<Quantifier>,

    /// The core logical statement to be proven (the part after the quantifiers).
    /// All free variables in this statement MUST be declared in the `context`.
    pub statement: MathRelation,
}

impl ProofGoal {
    /// Create a new, empty proof goal.
    pub fn new_empty() -> Self {
        Self {
            context: Vec::new(),
            quantifiers: Vec::new(),
            statement: MathRelation::False, // Default to False, must be set later
        }
    }

    /// Add a variable declaration to the context (e.g., `g: Group`).
    /// This is for fundamental variables that will be quantified over or used in hypotheses.
    pub fn with_variable(
        mut self,
        name: &str,
        ty: MathExpression,
        description: Option<String>,
    ) -> (Self, Identifier) {
        // In a full implementation, we would verify `ty` is a valid type expression
        // and that it is well-formed with respect to the existing context.
        let variable_name = Identifier::new_simple(name.to_string());
        let entry = ContextEntry {
            name: variable_name.clone(),
            ty,
            definition: None,
            description,
        };
        self.context.push(entry);
        (self, variable_name)
    }

    /// Add a named hypothesis to the context (e.g., `h: g > 0`).
    /// The proposition must be well-formed with respect to the existing context.
    pub fn with_hypothesis(
        mut self,
        name: &str,
        proposition: MathRelation,
        description: Option<String>,
    ) -> (Self, Identifier) {
        // To avoid circular references between MathExpression::Relation and MathRelation,
        // we'll use a simple variable type for the hypothesis and store the actual proposition
        // information in the description field.
        let hypothesis_name = Identifier::new_simple(name.to_string());

        // Create a safe description that includes the proposition information
        let safe_description = match description {
            Some(desc) => Some(format!(
                "{} (Proposition: {})",
                desc,
                format_relation_safely(&proposition)
            )),
            None => Some(format!(
                "Proposition: {}",
                format_relation_safely(&proposition)
            )),
        };

        let entry = ContextEntry {
            name: hypothesis_name.clone(),
            // Use a simple variable type instead of MathExpression::Relation to avoid circular reference
            ty: MathExpression::Var(Identifier::new_simple("Proposition".to_string())),
            definition: None,
            description: safe_description,
        };
        self.context.push(entry);
        (self, hypothesis_name)
    }

    /// Add a local definition to the context (e.g., `let y = x + 1`).
    pub fn with_definition(
        mut self,
        name: &str,
        definition: MathExpression,
        description: Option<String>,
    ) -> (Self, Identifier) {
        // In a full implementation, we would run type inference on `definition`
        // to get its type, and verify well-formedness.
        let inferred_type = definition.infer_type_or_placeholder();

        let variable_name = Identifier::new_simple(name.to_string());
        let entry = ContextEntry {
            name: variable_name.clone(),
            ty: inferred_type,
            definition: Some(definition),
            description,
        };
        self.context.push(entry);
        (self, variable_name)
    }

    /// Add a quantifier for a variable that is **already in the context**.
    pub fn with_quantifier(
        mut self,
        variable_name: &Identifier,
        quantification: Quantification,
    ) -> Self {
        // Check that the variable exists in the context.
        if !self
            .context
            .iter()
            .any(|entry| &entry.name == variable_name)
        {
            panic!(
                "Cannot add quantifier for variable '{:?}' not in context.",
                variable_name
            );
        }
        let quantifier = Quantifier {
            variable_name: variable_name.clone(),
            quantification,
        };
        self.quantifiers.push(quantifier);
        self
    }

    /// Set the final statement after the context and quantifiers are in place.
    pub fn with_statement(mut self, statement: MathRelation) -> Self {
        self.statement = statement;
        self
    }

    /// Verifies that the proof goal is well-formed.
    pub fn verify(&self) -> Result<(), String> {
        if matches!(self.statement, MathRelation::False) {
            return Err("Statement has not been set.".to_string());
        }

        let context_names: HashSet<_> = self.context.iter().map(|e| &e.name).collect();

        // Check 1: Every quantifier corresponds to a defined variable.
        for q in &self.quantifiers {
            if !context_names.contains(&q.variable_name) {
                return Err(format!(
                    "Quantifier '{:?}' has no definition in the context.",
                    q.variable_name
                ));
            }
        }

        // Check 2: No duplicate names in the context.
        if context_names.len() != self.context.len() {
            return Err("Duplicate names found in the context.".to_string());
        }

        // A more advanced check would verify that all free variables in the statement
        // and in each context entry's type/definition are valid with respect to the
        // ordered context.

        Ok(())
    }
}

/// Represents what role a ProofNode plays in the proof structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeRole {
    /// A normal goal that needs to be proven
    Goal(ProofGoal),
    /// A manager node that coordinates multiple sub-goals (e.g., from conjunction splits)
    SubgoalManager {
        /// IDs of the sub-goals this node manages
        subgoal_ids: Vec<String>,
        /// How the sub-goals should be combined (And/Or)
        combination_type: SubgoalCombination,
    },
    // the role of the node is to represent the result of a
    Completed,
    Disproved,
}

/// How sub-goals should be combined in a SubgoalManager
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubgoalCombination {
    /// All sub-goals must be proven (conjunction)
    And,
    /// At least one sub-goal must be proven (disjunction)
    Or,
    /// Custom combination logic
    Custom(String),
}

/// Result returned by apply_tactic that includes the manager node and all sub-nodes created
#[derive(Debug, Clone)]
pub struct TacticOutcome {
    /// The manager node that was created/modified
    pub manager: ProofNode,
    /// All sub-nodes that were created by the tactic
    pub sub_nodes: Vec<ProofNode>,
}

impl TacticOutcome {
    /// Get the primary proof node for chaining tactics.
    /// For single-goal tactics, returns the manager.
    /// For multi-goal tactics, returns the manager (coordination node).
    pub fn primary_node(self) -> ProofNode {
        self.manager
    }

    /// Get the first goal node if this tactic created multiple goals,
    /// otherwise returns the manager node.
    /// Useful when you want to continue proving from the first sub-goal.
    pub fn first_goal_node(self) -> ProofNode {
        if !self.sub_nodes.is_empty() {
            self.sub_nodes.into_iter().next().unwrap()
        } else {
            self.manager
        }
    }

    /// Get all goal nodes that need to be proven.
    /// For single-goal tactics, returns vec![manager].
    /// For multi-goal tactics, returns the sub_nodes.
    pub fn goal_nodes(self) -> Vec<ProofNode> {
        if self.sub_nodes.is_empty() {
            vec![self.manager]
        } else {
            self.sub_nodes
        }
    }

    /// Check if this tactic created multiple goals that need separate handling
    pub fn is_multi_goal(&self) -> bool {
        !self.sub_nodes.is_empty()
    }

    /// Extract a specific sub-goal by index (0-based)
    /// Returns None if index is out of bounds or if this is a single-goal tactic
    pub fn sub_goal(mut self, index: usize) -> Option<ProofNode> {
        if index < self.sub_nodes.len() {
            Some(self.sub_nodes.remove(index))
        } else {
            None
        }
    }

    /// Handle subgoals immediately in the current scope (Lean-style).
    /// This is the recommended pattern for multi-goal tactics.
    /// The closure receives the subgoals and should return proof nodes for each.
    /// Returns the manager node after all subgoals are handled.
    pub fn handle_subgoals<F>(mut self, handler: F) -> ProofNode
    where
        F: FnOnce(Vec<ProofNode>) -> Vec<ProofNode>,
    {
        if !self.sub_nodes.is_empty() {
            // Handle the subgoals immediately
            let completed_subgoals = handler(self.sub_nodes);

            // Update the manager's children to point to the completed subgoals
            // (In a full implementation, we'd update the ProofForest structure)

            // For now, we return the manager as it represents the coordination of subgoals
            self.manager
        } else {
            // Single goal case - just return the manager
            self.manager
        }
    }

    /// Extract subgoals for immediate handling using the helper functions.
    /// This provides access to the sub_nodes for use with the extract_subgoals! macro
    /// and helper functions from proof::helpers.
    pub fn extract_subgoals(self) -> (ProofNode, Vec<ProofNode>) {
        (self.manager, self.sub_nodes)
    }
}

/// Status of a proof branch
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProofStatus {
    /// Successfully completed proof
    Complete,
    /// In progress but making headway
    InProgress,
    /// Todo item for later
    Todo,
    /// Work in progress (active development)
    Wip,
    /// Abandoned (won't pursue further)
    Abandoned,
}

/// A single node in a proof tree, representing the application of a tactic
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProofNode {
    /// Unique identifier for this node
    pub id: String,
    /// Parent node ID, only root proofnode has none.
    pub parent: Option<String>,
    /// Child node IDs, each child is a branch
    pub children: Vec<String>,
    /// What role this node plays in the proof structure
    pub role: NodeRole,
    /// The tactic applied to reach this state - ALWAYS required
    pub tactic: Tactic,
    /// Status of this proof branch
    pub status: ProofStatus,
    /// Structured description of this proof step
    pub description: Option<RichText>,
}

impl ProofNode {
    /// Create a manager node for coordinating sub-goals
    pub fn new_manager(
        id: String,
        subgoal_ids: Vec<String>,
        combination_type: SubgoalCombination,
        tactic: Tactic,
    ) -> Self {
        Self {
            id,
            parent: None,
            children: Vec::new(),
            role: NodeRole::SubgoalManager {
                subgoal_ids,
                combination_type,
            },
            tactic,
            status: ProofStatus::InProgress,
            description: None,
        }
    }

    /// Get the goal from this node, panics if this is not a Goal node
    pub fn get_goal(&self) -> &ProofGoal {
        match &self.role {
            NodeRole::Goal(goal) => goal,
            _ => panic!("Node does not have a goal"),
        }
    }

    /// Get the goal mutably from this node, panics if this is not a Goal node
    pub fn get_goal_mut(&mut self) -> &mut ProofGoal {
        match &mut self.role {
            NodeRole::Goal(goal) => goal,
            _ => panic!("Node does not have a goal"),
        }
    }

    /// Check if this node has a goal
    pub fn has_goal(&self) -> bool {
        matches!(self.role, NodeRole::Goal(_))
    }

    /// Apply a tactic to this node - handles both single and multi-goal results
    pub fn apply_tactic(&self, tactic: Tactic, forest: &mut ProofForest) -> TacticOutcome {
        let current_goal = self.get_goal();
        let result = tactic.apply_to_goal(current_goal);

        match result {
            TacticApplicationResult::SingleGoal(new_goal) => {
                let new_node = ProofNode {
                    id: Uuid::new_v4().to_string(),
                    parent: Some(self.id.clone()),
                    children: vec![],
                    role: NodeRole::Goal(new_goal),
                    tactic,
                    status: ProofStatus::InProgress,
                    description: None,
                };

                // Add the new node to the forest and update parent's children
                forest.add_node(new_node.clone());
                if let Some(parent_node) = forest.get_node_mut(&self.id) {
                    parent_node.children.push(new_node.id.clone());
                }

                TacticOutcome {
                    manager: new_node,
                    sub_nodes: vec![],
                }
            }
            TacticApplicationResult::MultiGoal(goals) => {
                // Create sub-nodes for each goal
                let sub_nodes: Vec<ProofNode> = goals
                    .into_iter()
                    .map(|goal| {
                        let node = ProofNode {
                            id: Uuid::new_v4().to_string(),
                            parent: Some(self.id.clone()),
                            children: vec![],
                            role: NodeRole::Goal(goal),
                            tactic: tactic.clone(),
                            status: ProofStatus::InProgress,
                            description: None,
                        };
                        forest.add_node(node.clone());
                        node
                    })
                    .collect();

                // Create a manager node to coordinate the sub-goals
                let subgoal_ids: Vec<String> = sub_nodes.iter().map(|n| n.id.clone()).collect();
                let mut manager = ProofNode::new_manager(
                    Uuid::new_v4().to_string(),
                    subgoal_ids.clone(),
                    SubgoalCombination::And,
                    tactic,
                );
                manager.parent = Some(self.id.clone());

                forest.add_node(manager.clone());

                // Update parent's children to include manager and all sub-nodes
                if let Some(parent_node) = forest.get_node_mut(&self.id) {
                    parent_node.children.push(manager.id.clone());
                    parent_node.children.extend(subgoal_ids);
                }

                TacticOutcome { manager, sub_nodes }
            }
            TacticApplicationResult::ProofComplete => {
                let completed_node = ProofNode {
                    id: Uuid::new_v4().to_string(),
                    parent: Some(self.id.clone()),
                    children: vec![],
                    role: NodeRole::Completed,
                    tactic,
                    status: ProofStatus::Complete,
                    description: None,
                };

                forest.add_node(completed_node.clone());
                if let Some(parent_node) = forest.get_node_mut(&self.id) {
                    parent_node.children.push(completed_node.id.clone());
                }

                TacticOutcome {
                    manager: completed_node,
                    sub_nodes: vec![],
                }
            }
            TacticApplicationResult::NoChange => {
                let unchanged_node = ProofNode {
                    id: Uuid::new_v4().to_string(),
                    parent: Some(self.id.clone()),
                    children: vec![],
                    role: NodeRole::Goal(current_goal.clone()),
                    tactic,
                    status: ProofStatus::InProgress,
                    description: None,
                };

                forest.add_node(unchanged_node.clone());
                if let Some(parent_node) = forest.get_node_mut(&self.id) {
                    parent_node.children.push(unchanged_node.id.clone());
                }

                TacticOutcome {
                    manager: unchanged_node,
                    sub_nodes: vec![],
                }
            }
            TacticApplicationResult::Error(msg) => {
                panic!("Tactic application failed: {}", msg);
            }
        }
    }

    /// Convenience method to create a new forest from sub-goals
    pub fn from_subproofs(subforests: Vec<ProofForest>) -> ProofNode {
        // This would combine multiple completed sub-proofs into a single proof node
        // For now, we'll create a simple manager node
        let subgoal_ids: Vec<String> = subforests
            .iter()
            .map(|f| f.get_root().unwrap().id.clone())
            .collect();

        // Use a placeholder tactic for combining subproofs
        let combine_tactic = Tactic::Auto(AutomatedTactic::ByAssumption);

        ProofNode::new_manager(
            Uuid::new_v4().to_string(),
            subgoal_ids,
            SubgoalCombination::And,
            combine_tactic,
        )
    }

    /// Marks a proof branch as complete.
    pub fn should_complete(self, forest: &mut ProofForest) -> Self {
        let mut node = self;
        node.role = NodeRole::Completed;
        // forest.nodes.insert(node.id.clone(), node.clone());
        node
    }

    /// Create a case analysis
    pub fn case_analysis<'a>(
        &self,
        forest: &'a mut ProofForest,
    ) -> tactics::CaseAnalysisBuilder<'a> {
        tactics::CaseAnalysisBuilder::new(self.clone(), forest)
    }

    pub fn to_proof_display(&self, forest: &ProofForest) -> ProofDisplayNode {
        ProofDisplayNode {
            title: None,
            strategy: vec![],
            steps: vec![self.to_proof_step()],
            qed_symbol: None,
        }
    }

    /// Simple test function to verify tactic application works
    #[cfg(test)]
    pub fn test_tactic_application_directly() -> bool {
        use crate::subjects::math::formalism::expressions::MathExpression;
        use crate::subjects::math::formalism::proof::tactics::{Tactic, TacticApplicationResult};
        use crate::subjects::math::formalism::relations::MathRelation;
        use crate::subjects::math::theories::number_theory::definitions::Number;
        use crate::turn_render::Identifier;

        let antecedent = MathRelation::equal(
            MathExpression::Number(Number {}),
            MathExpression::Number(Number {}),
        );
        let consequent = MathRelation::equal(
            MathExpression::Number(Number {}),
            MathExpression::Number(Number {}),
        );

        let goal = ProofGoal {
            context: vec![],
            quantifiers: vec![],
            statement: MathRelation::Implies(
                Box::new(antecedent.clone()),
                Box::new(consequent.clone()),
            ),
        };

        let tactic = Tactic::AssumeImplicationAntecedent {
            hypothesis_name: Identifier::new_simple("H".to_string()),
        };

        match tactic.apply_to_goal(&goal) {
            TacticApplicationResult::SingleGoal(new_goal) => {
                // Verify the transformation worked
                new_goal.statement == consequent && new_goal.context.len() == 1
            }
            _ => false,
        }
    }
}

impl ToProofStep for ProofNode {
    fn to_proof_step(&self) -> ProofStepNode {
        let tactic_name = self.tactic.to_string();

        ProofStepNode::Statement {
            claim: vec![RichTextSegment::Text(format!(
                "Apply tactic: {}",
                tactic_name
            ))],
            justification: vec![RichTextSegment::Text("Tactic application".to_string())],
        }
    }
}

/// A forest of proof exploration nodes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofForest {
    pub initial_goal: ProofGoal,
    /// All nodes in the forest
    nodes: HashMap<String, ProofNode>,
    /// Root node ID for each tree in the forest
    pub roots: Vec<String>,
}

impl ProofForest {
    pub fn new_from_goal(goal: ProofGoal) -> Self {
        Self {
            initial_goal: goal,
            nodes: HashMap::new(),
            roots: Vec::new(),
        }
    }

    /// Create a new ProofForest from a goal (for use in subgoal extraction)
    pub fn from_goal(goal: ProofGoal) -> Self {
        Self::new_from_goal(goal)
    }

    pub fn add_node(&mut self, node: ProofNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn get_node(&self, node_id: &str) -> Option<&ProofNode> {
        self.nodes.get(node_id)
    }

    pub fn get_node_mut(&mut self, node_id: &str) -> Option<&mut ProofNode> {
        self.nodes.get_mut(node_id)
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn node_values(&self) -> impl Iterator<Item = &ProofNode> {
        self.nodes.values()
    }

    /// Apply a tactic to the initial goal
    pub fn apply_initial_tactic(&mut self, tactic: Tactic) -> &ProofNode {
        let initial_state = self.initial_goal.clone();

        // Apply the tactic to get the new state
        let result = tactic.apply_to_goal(&initial_state);

        let root_node = match result {
            TacticApplicationResult::SingleGoal(new_goal) => ProofNode {
                id: Uuid::new_v4().to_string(),
                parent: None,
                children: vec![],
                role: NodeRole::Goal(new_goal),
                tactic,
                status: ProofStatus::InProgress,
                description: None,
            },
            TacticApplicationResult::ProofComplete => ProofNode {
                id: Uuid::new_v4().to_string(),
                parent: None,
                children: vec![],
                role: NodeRole::Goal(initial_state),
                tactic,
                status: ProofStatus::Complete,
                description: None,
            },
            TacticApplicationResult::NoChange => ProofNode {
                id: Uuid::new_v4().to_string(),
                parent: None,
                children: vec![],
                role: NodeRole::Goal(initial_state),
                tactic,
                status: ProofStatus::InProgress,
                description: None,
            },
            TacticApplicationResult::MultiGoal(_) => {
                panic!("Multi-goal tactics should use apply_tactic_with_subgoals method instead.");
            }
            TacticApplicationResult::Error(msg) => {
                panic!("Initial tactic application failed: {}", msg);
            }
        };

        let root_id = root_node.id.clone();
        self.add_node(root_node);
        self.roots.push(root_id.clone());

        self.nodes.get(&root_id).unwrap()
    }

    pub fn get_root(&self) -> Option<&ProofNode> {
        self.roots.first().and_then(|id| self.nodes.get(id))
    }

    pub fn get_root_mut(&mut self) -> Option<&mut ProofNode> {
        if self.roots.is_empty() {
            None
        } else {
            self.nodes.get_mut(&self.roots[0])
        }
    }

    /// Check if all branches of the proof are complete
    pub fn is_fully_proven(&self) -> bool {
        for root_id in &self.roots {
            if !self.is_branch_complete(root_id) {
                return false;
            }
        }
        true
    }

    /// Recursively check if a branch starting at a given node is complete
    fn is_branch_complete(&self, node_id: &str) -> bool {
        if let Some(node) = self.nodes.get(node_id) {
            if node.children.is_empty() {
                return node.status == ProofStatus::Complete;
            }
            for child_id in &node.children {
                if !self.is_branch_complete(child_id) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_tactic_application() {
        assert!(ProofNode::test_tactic_application_directly());
    }

    #[test]
    fn test_assume_implication_transforms_goal() {
        use crate::subjects::math::formalism::expressions::MathExpression;
        use crate::subjects::math::formalism::proof::tactics::{Tactic, TacticApplicationResult};
        use crate::subjects::math::formalism::relations::MathRelation;
        use crate::subjects::math::theories::number_theory::definitions::Number;
        use crate::turn_render::Identifier;

        let antecedent = MathRelation::equal(
            MathExpression::Number(Number {}),
            MathExpression::Number(Number {}),
        );
        let consequent = MathRelation::equal(
            MathExpression::Number(Number {}),
            MathExpression::Number(Number {}),
        );

        let goal = ProofGoal {
            context: vec![],
            quantifiers: vec![],
            statement: MathRelation::Implies(
                Box::new(antecedent.clone()),
                Box::new(consequent.clone()),
            ),
        };

        let tactic = Tactic::AssumeImplicationAntecedent {
            hypothesis_name: Identifier::new_simple("H1".to_string()),
        };

        match tactic.apply_to_goal(&goal) {
            TacticApplicationResult::SingleGoal(new_goal) => {
                // The statement should now be just the consequent
                assert_eq!(new_goal.statement, consequent);

                // There should be one hypothesis in the context
                assert_eq!(new_goal.context.len(), 1);
                // Check that the context entry is a hypothesis with the antecedent as its type
                assert_eq!(
                    new_goal.context[0].ty,
                    MathExpression::Relation(Box::new(antecedent))
                );
            }
            other => panic!("Expected SingleGoal, got {:?}", other),
        }
    }

    #[test]
    fn test_introduce_value_variable_tactic() {
        use crate::subjects::math::formalism::expressions::MathExpression;
        use crate::subjects::math::formalism::proof::tactics::{Tactic, TacticApplicationResult};
        use crate::subjects::math::formalism::relations::MathRelation;
        use crate::subjects::math::theories::number_theory::definitions::Number;
        use crate::turn_render::Identifier;

        let goal = ProofGoal {
            context: vec![],
            quantifiers: vec![],
            statement: MathRelation::equal(
                MathExpression::Number(Number {}),
                MathExpression::Number(Number {}),
            ),
        };

        let new_entry = ContextEntry {
            name: Identifier::new_simple("temp".to_string()),
            ty: MathExpression::Number(Number {}),
            definition: Some(MathExpression::Number(Number {})),
            description: None,
        };

        let tactic = Tactic::Introduce {
            entry: new_entry.clone(),
            position: None,
        };

        match tactic.apply_to_goal(&goal) {
            TacticApplicationResult::SingleGoal(new_goal) => {
                // The statement should remain the same
                assert_eq!(new_goal.statement, goal.statement);

                // There should now be one entry in the context
                assert_eq!(new_goal.context.len(), 1);
                assert_eq!(new_goal.context[0], new_entry);
            }
            other => panic!("Expected SingleGoal, got {:?}", other),
        }
    }

    #[test]
    fn test_split_conjunction_tactic() {
        use crate::subjects::math::formalism::expressions::MathExpression;
        use crate::subjects::math::formalism::proof::tactics::{Tactic, TacticApplicationResult};
        use crate::subjects::math::formalism::relations::MathRelation;
        use crate::subjects::math::theories::number_theory::definitions::Number;

        let part1 = MathRelation::equal(
            MathExpression::Number(Number {}),
            MathExpression::Number(Number {}),
        );
        let part2 = MathRelation::equal(
            MathExpression::Number(Number {}),
            MathExpression::Number(Number {}),
        );
        let part3 = MathRelation::equal(
            MathExpression::Number(Number {}),
            MathExpression::Number(Number {}),
        );

        let conjunction = MathRelation::And(vec![part1.clone(), part2.clone(), part3.clone()]);

        let goal = ProofGoal {
            context: vec![],
            quantifiers: vec![],
            statement: conjunction.clone(),
        };

        let tactic = Tactic::SplitConjunction;

        match tactic.apply_to_goal(&goal) {
            TacticApplicationResult::MultiGoal(goals) => {
                assert_eq!(goals.len(), 3);

                // Each goal should be one of the conjuncts
                assert_eq!(goals[0].statement, part1);
                assert_eq!(goals[1].statement, part2);
                assert_eq!(goals[2].statement, part3);
            }
            other => panic!("Expected MultiGoal, got {:?}", other),
        }
    }
}

// Backward compatibility types for existing code
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValueBindedVariable {
    pub name: Identifier,
    pub value: MathExpression,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuantifiedMathObject {
    pub quantification: Quantification,
    pub variable: Identifier,
    pub object_type: MathObject,
    pub description: Option<String>,
}

/// Safely format a MathRelation to a string without causing circular references
pub fn format_relation_safely(relation: &MathRelation) -> String {
    match relation {
        MathRelation::Equal { left, right, .. } => {
            format!(
                "{} = {}",
                format_expression_safely(left),
                format_expression_safely(right)
            )
        }
        MathRelation::And(relations) => {
            let formatted: Vec<String> = relations.iter().map(format_relation_safely).collect();
            format!("({})", formatted.join(" ∧ "))
        }
        MathRelation::Or(relations) => {
            let formatted: Vec<String> = relations.iter().map(format_relation_safely).collect();
            format!("({})", formatted.join(" ∨ "))
        }
        MathRelation::Not(relation) => {
            format!("¬({})", format_relation_safely(relation))
        }
        MathRelation::Implies(premise, conclusion) => {
            format!(
                "({} → {})",
                format_relation_safely(premise),
                format_relation_safely(conclusion)
            )
        }
        MathRelation::Equivalent(left, right) => {
            format!(
                "({} ↔ {})",
                format_relation_safely(left),
                format_relation_safely(right)
            )
        }
        MathRelation::True => "True".to_string(),
        MathRelation::False => "False".to_string(),
        MathRelation::NumberTheory(_) => "[NumberTheoryRelation]".to_string(),
        MathRelation::SetTheory(_) => "[SetTheoryRelation]".to_string(),
        MathRelation::GroupTheory(_) => "[GroupTheoryRelation]".to_string(),
        MathRelation::RingTheory(_) => "[RingTheoryRelation]".to_string(),
        MathRelation::TopologyTheory(_) => "[TopologyTheoryRelation]".to_string(),
        MathRelation::CategoryTheory(_) => "[CategoryTheoryRelation]".to_string(),
        MathRelation::ProbabilityTheory(_) => "[ProbabilityTheoryRelation]".to_string(),
    }
}

/// Safely format a MathExpression to a string without causing circular references
fn format_expression_safely(expr: &MathExpression) -> String {
    match expr {
        MathExpression::Var(id) => id.to_string(),
        MathExpression::Relation(_) => "[Relation]".to_string(), // Avoid circular reference
        MathExpression::Object(obj) => format!(
            "[Object: {}]",
            match obj.as_ref() {
                MathObject::Group(_) => "Group",
                MathObject::Ring(_) => "Ring",
                MathObject::Field(_) => "Field",
                MathObject::Module(_) => "Module",
                MathObject::Algebra(_) => "Algebra",
                MathObject::TopologicalSpace(_) => "TopologicalSpace",
                MathObject::VectorSpace(_) => "VectorSpace",
                MathObject::Set(_) => "Set",
                MathObject::Function(_) => "Function",
            }
        ),
        MathExpression::Expression(theory_expr) => "[TheoryExpression]".to_string(),
        MathExpression::Number(num) => format!("[Number: {:?}]", num),
        MathExpression::ViewAs { expression, view } => {
            format!(
                "[ViewAs: {} as {:?}]",
                format_expression_safely(expression),
                view
            )
        }
    }
}
