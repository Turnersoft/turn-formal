// A test comment to see if the file is writable
// Module: src/formalize_v2/subjects/math/theorem/proof.rs
// Implements a rich proof structure for mathematical theorems with branching support

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::env::consts::DLL_EXTENSION;
use std::hash::{Hash, Hasher};
use std::mem::{Discriminant, discriminant};
use std::rc::Rc;
use std::sync::{Arc, Mutex, OnceLock};
#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};
use tactics::{TacticApplicationResult, Target};
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use super::super::theories::zfc::definitions::SetRelation;
use super::expressions::{MathExpression, TheoryExpression};
use super::interpretation::TypeViewOperator;
use super::location::Located;
use super::relations::{MathRelation, Quantification};
use super::{objects::MathObject, theorem::Theorem};

use self::tactics::Tactic;
use crate::subjects::math::formalism::extract::Parametrizable;

use crate::subjects::math::theories::groups::definitions::GroupExpression;
use crate::subjects::math::theories::rings::definitions::{FieldExpression, RingExpression};
use crate::turn_render::{
    Identifier, MathNode, MathNodeContent, ProofDisplayNode, ProofStepNode, RichText,
    RichTextSegment, Section, SectionContentNode, ToProofDisplay, ToProofStep, ToTurnMath,
};

pub mod helpers;
pub mod tactics;

/// Describes how the value of a context entry is defined.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DefinitionState {
    /// The name is just a placeholder with no definition.
    Abstract,
    /// The name is defined by a separate mathematical expression.
    Separate(Located<MathExpression>),
    /// The name is defined inline within a larger structure (e.g., local let binding).
    Inlined,
    /// the value is part of the type
    ContainedInType,
}

/// Represents a single named entry in a proof context. This can be a variable
/// declaration, a hypothesis, or a local definition (abbreviation).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContextEntry {
    pub name: Identifier,
    pub ty: Located<MathExpression>,
    pub definition: DefinitionState,
    pub description: Option<RichText>,
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
    pub statement: Located<MathRelation>,
}

impl ProofGoal {
    /// Create a new, empty proof goal.
    pub fn new_empty() -> Self {
        Self {
            context: Vec::new(),
            quantifiers: Vec::new(),
            statement: Located::new(MathRelation::False),
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
            ty: Located::new(ty),
            definition: DefinitionState::Abstract,
            description: description.map(|s| RichText {
                segments: vec![RichTextSegment::Text(s)],
                alignment: None,
            }),
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

        let entry = ContextEntry {
            name: hypothesis_name.clone(),
            // Use a simple variable type instead of MathExpression::Relation to avoid circular reference
            ty: Located::new(MathExpression::Relation(Box::new(proposition))),
            definition: DefinitionState::Abstract,
            description: description.map(|s| RichText {
                segments: vec![RichTextSegment::Text(s)],
                alignment: None,
            }),
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
        // // In a full implementation, we would run type inference on `definition`
        // // to get its type, and verify well-formedness.
        // let inferred_type = MathExpression::Object(Box::new(MathObject::Set(Set::empty())));

        // let variable_name = Identifier::new_simple(name.to_string());
        // let entry = ContextEntry {
        //     name: variable_name.clone(),
        //     ty: Located::new(inferred_type),
        //     definition: DefinitionState::Separate(Located::new(definition)),
        //     description: description.map(|s| RichText {
        //         segments: vec![RichTextSegment::Text(s)],
        //         alignment: None,
        //     }),
        // };
        // self.context.push(entry);
        // (self, variable_name)
        todo!()
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
        self.statement = Located::new(statement);
        self
    }

    /// Verifies that the proof goal is well-formed.
    pub fn verify(&self) -> Result<(), String> {
        if matches!(self.statement.value(), MathRelation::False) {
            return Err("Statement has not been set.".to_string());
        }

        let context_map: HashMap<_, _> = self.context.iter().map(|e| (&e.name, e)).collect();

        // Check 1: Every quantifier corresponds to an abstract variable.
        for q in &self.quantifiers {
            match context_map.get(&q.variable_name) {
                None => {
                    return Err(format!(
                        "Quantifier '{:?}' has no definition in the context.",
                        q.variable_name
                    ));
                }
                Some(entry) => {
                    if !matches!(entry.definition, DefinitionState::Abstract) {
                        return Err(format!(
                            "Quantified variable '{:?}' must be abstract, but it has a concrete definition.",
                            q.variable_name
                        ));
                    }
                }
            }
        }

        // Check 2: No duplicate names in the context.
        if context_map.len() != self.context.len() {
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

    /// Represents a single, user-visible step that was performed by a
    /// high-level, automated tactic (like 'simplify' or 'auto').
    ///
    /// This node encapsulates a complex internal proof that justifies the
    /// transformation from its parent's goal to a new, transformed state.
    AutomatedTacticStep {
        /// A human-readable description of the automated tactic that was run
        /// (e.g., "Simplified the goal using 5 rewrites").
        description: RichText,

        /// The detailed, internal proof showing all attempts made by the
        /// automated tactic. This is a self-contained "scratchpad" forest.
        justification: ProofForest,

        /// The ID of the node within the `justification` forest that represents
        /// the successful outcome of this automated step. The state of this
        /// "best node" becomes the new current state for the main proof tree.
        best_node_id: String,
    },

    /// Represents a goal that has been proven false by citing a
    /// counter-theorem. This is a terminal failure state for a proof branch.
    Disproved(String), // The ID of the theorem that proves the negation of this node's goal.

    /// A goal that was transformed by a rewrite tactic.
    RewriteStep {
        /// The goal state after the rewrite.
        goal: ProofGoal,
        /// The ID of the sub-expression that was replaced.
        rewritten_from_id: Target,
        /// The ID of the new sub-expression that was substituted in.
        rewritten_to_id: Target,
    },

    Completed,
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
        let combine_tactic = Tactic::SearchAssumptions;

        ProofNode::new_manager(
            Uuid::new_v4().to_string(),
            subgoal_ids,
            SubgoalCombination::And,
            combine_tactic,
        )
    }

    /// Validates that this node represents a completed proof.
    /// This method should be called on the result of applying a completing tactic.
    /// It will panic if the node doesn't actually represent a completed proof.
    pub fn should_complete(self) -> Self {
        // Check that this node actually represents a completed proof
        match &self.role {
            NodeRole::Completed => self, // Good! The proof is completed
            NodeRole::Goal(_) => {
                panic!(
                    "Expected the node to represent a completed proof, but it still has a goal. \
                     The proof step does not actually close this goal."
                );
            }
            other => {
                panic!(
                    "Expected the node to represent a completed proof (Completed role), but got: {:?}. \
                     The proof step does not actually close this goal.",
                    other
                );
            }
        }
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
            statement: Located::new(MathRelation::Implies(
                Box::new(Located::new(Parametrizable::Concrete(antecedent.clone()))),
                Box::new(Located::new(Parametrizable::Concrete(consequent.clone()))),
            )),
        };

        let tactic = Tactic::AssumeImplicationAntecedent {
            with_name: Identifier::new_simple("H".to_string()),
        };

        match tactic.apply_to_goal(&goal) {
            TacticApplicationResult::SingleGoal(new_goal) => {
                // Verify the transformation worked
                new_goal.statement.data == consequent && new_goal.context.len() == 1
            }
            _ => false,
        }
    }
}

impl ToProofStep for ProofNode {
    fn to_proof_step(&self) -> ProofStepNode {
        let tactic_name = format!("{:?}", self.tactic);

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
                description: None,
            },
            TacticApplicationResult::ProofComplete => ProofNode {
                id: Uuid::new_v4().to_string(),
                parent: None,
                children: vec![],
                role: NodeRole::Completed,
                tactic,
                description: None,
            },
            TacticApplicationResult::NoChange => ProofNode {
                id: Uuid::new_v4().to_string(),
                parent: None,
                children: vec![],
                role: NodeRole::Goal(initial_state),
                tactic,
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
                return matches!(node.role, NodeRole::Completed);
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
            statement: Located::new(MathRelation::Implies(
                Box::new(Located::new(Parametrizable::Concrete(antecedent.clone()))),
                Box::new(Located::new(Parametrizable::Concrete(consequent.clone()))),
            )),
        };

        let tactic = Tactic::AssumeImplicationAntecedent {
            with_name: Identifier::new_simple("H1".to_string()),
        };

        match tactic.apply_to_goal(&goal) {
            TacticApplicationResult::SingleGoal(new_goal) => {
                // The statement should now be just the consequent
                assert_eq!(new_goal.statement.data, consequent);

                // There should be one hypothesis in the context
                assert_eq!(new_goal.context.len(), 1);
                // Check that the context entry is a hypothesis with a Relation type
                assert!(matches!(
                    new_goal.context[0].ty.data,
                    MathExpression::Relation(_)
                ));
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

        let conjunction = MathRelation::And(vec![
            Located::new(Parametrizable::Concrete(part1.clone())),
            Located::new(Parametrizable::Concrete(part2.clone())),
            Located::new(Parametrizable::Concrete(part3.clone())),
        ]);

        let goal = ProofGoal {
            context: vec![],
            quantifiers: vec![],
            statement: Located::new(conjunction.clone()),
        };

        let tactic = Tactic::SplitGoalConjunction;

        match tactic.apply_to_goal(&goal) {
            TacticApplicationResult::MultiGoal(goals) => {
                assert_eq!(goals.len(), 3);

                // Each goal should be one of the conjuncts
                assert_eq!(goals[0].statement.data, part1);
                assert_eq!(goals[1].statement.data, part2);
                assert_eq!(goals[2].statement.data, part3);
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

/// Safely format a Parametrizable<MathExpression> to a string
fn format_parametrizable_expression_safely(pexpr: &Parametrizable<MathExpression>) -> String {
    match pexpr {
        Parametrizable::Concrete(expr) => format_expression_safely(expr),
        Parametrizable::Variable(id) => id.to_string(),
    }
}

/// Safely format a MathExpression to a string without causing circular references
fn format_expression_safely(expr: &MathExpression) -> String {
    match expr {
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
        MathExpression::Expression(_theory_expr) => "[TheoryExpression]".to_string(),
        MathExpression::Number(num) => format!("[Number: {:?}]", num),
        MathExpression::ViewAs { expression, view } => {
            format!(
                "[ViewAs: {} as {:?}]",
                format_parametrizable_expression_safely(&expression.data),
                view
            )
        }
    }
}

/// Safely format a Parametrizable<MathRelation> to a string
fn format_parametrizable_relation_safely(prel: &Parametrizable<MathRelation>) -> String {
    match prel {
        Parametrizable::Concrete(rel) => format_relation_safely(rel),
        Parametrizable::Variable(id) => id.to_string(),
    }
}

/// Safely format a MathRelation to a string without causing circular references
fn format_relation_safely(rel: &MathRelation) -> String {
    match rel {
        MathRelation::Equal { left, right, .. } => {
            format!(
                "{} = {}",
                format_parametrizable_expression_safely(&left.data),
                format_parametrizable_expression_safely(&right.data)
            )
        }
        MathRelation::Implies(antecedent, consequent) => {
            format!(
                "{} → {}",
                format_parametrizable_relation_safely(&antecedent.data),
                format_parametrizable_relation_safely(&consequent.data)
            )
        }
        MathRelation::And(relations) => {
            let parts: Vec<String> = relations
                .iter()
                .map(|r| format_parametrizable_relation_safely(&r.data))
                .collect();
            format!("({})", parts.join(" ∧ "))
        }
        MathRelation::Or(relations) => {
            let parts: Vec<String> = relations
                .iter()
                .map(|r| format_parametrizable_relation_safely(&r.data))
                .collect();
            format!("({})", parts.join(" ∨ "))
        }
        MathRelation::Not(rel) => {
            format!("¬({})", format_parametrizable_relation_safely(&rel.data))
        }
        MathRelation::True => "⊤".to_string(),
        MathRelation::False => "⊥".to_string(),
        _ => "[Relation]".to_string(),
    }
}
