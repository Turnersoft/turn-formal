// Module: src/formalize_v2/subjects/math/theorem/proof.rs
// Implements a rich proof structure for mathematical theorems with branching support

use js_sys;
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
use super::relations::{MathRelation, Quantification, RelationDetail};
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
pub mod path_index;
pub mod tactics;

// Re-export the tactics types for backward compatibility
pub use tactics::{
    CaseAnalysisBuilder, CaseResult, DecompositionMethod, InductionType, RewriteDirection, Tactic,
};

// Remove the invalid re-exports from super
// These are defined in this file, no need to import them from super
// pub use super::ProofForest;
// pub use super::ProofNode;
// pub use super::TheoremRegistry;
// pub use super::get_theorem_registry;

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

/// A registry of theorems that can be applied during proofs
#[derive(Debug, Clone)]
pub struct TheoremRegistry {
    /// Map of theorem IDs to Theorem objects
    theorems: HashMap<String, Theorem>,
    relation_index: HashMap<Discriminant<MathRelation>, Vec<String>>,
}

// Create a global registry that will be initialized on first access
lazy_static! {
    static ref GLOBAL_THEOREM_REGISTRY: Mutex<TheoremRegistry> = Mutex::new(TheoremRegistry::new());
}

impl TheoremRegistry {
    /// Create a new empty theorem registry
    pub fn new() -> Self {
        Self {
            theorems: HashMap::new(),
            relation_index: HashMap::new(),
        }
    }

    /// Register a theorem
    pub fn register(&mut self, theorem: Theorem) {
        let theorem_id = theorem.id.clone();
        println!("Registering theorem: {}", theorem.name);
        self.theorems.insert(theorem_id.clone(), theorem.clone());

        let discriminant = std::mem::discriminant(&theorem.proofs.initial_goal.statement);
        self.relation_index
            .entry(discriminant)
            .or_default()
            .push(theorem_id);
    }

    /// Get a theorem by ID
    pub fn get_theorem(&self, id: &str) -> Option<&Theorem> {
        self.theorems.get(id)
    }

    /// List all registered theorems
    pub fn list_all_theorems(&self) -> Vec<String> {
        self.theorems.keys().cloned().collect()
    }

    /// Get or create a global registry
    pub fn global() -> &'static Mutex<TheoremRegistry> {
        &GLOBAL_THEOREM_REGISTRY
    }

    /// Register a theorem in the global registry
    pub fn register_globally(theorem: Theorem) {
        let mut registry = GLOBAL_THEOREM_REGISTRY.lock().unwrap();
        registry.register(theorem);
    }

    /// Get theorem IDs relevant to a specific kind of MathRelation
    pub fn get_theorems_by_relation_kind(
        &self,
        relation_discriminant: &Discriminant<MathRelation>,
    ) -> Option<&Vec<String>> {
        self.relation_index.get(relation_discriminant)
    }

    /// Apply a theorem to a statement
    /// This uses the theorem as a rewrite rule
    pub fn apply_theorem(
        &self,
        theorem_id: &str,
        statement: &MathRelation,
        instantiation: &HashMap<String, MathExpression>,
        target_expr: Option<MathExpression>,
    ) -> Option<MathRelation> {
        // For the initial implementation, we'll simulate theorem application
        // with a simple pattern matching approach.

        // In a real implementation, we would:
        // 1. Look up the theorem
        // 2. Instantiate it with the provided variable mappings
        // 3. Check that it applies to the target
        // 4. Apply the rewrite

        // For now, we'll make a simple substitution to show the concept

        if let Some(target) = target_expr {
            // If we have a specific target, we'll search for it in the statement
            transform_with_target(statement, &target, theorem_id, instantiation)
        } else {
            // Apply globally
            Some(statement.clone()) // Placeholder - would actually transform
        }
    }
}

/// Helper function to transform a statement by applying a theorem to a specific target
fn transform_with_target(
    statement: &MathRelation,
    target: &MathExpression,
    theorem_id: &str,
    instantiation: &HashMap<String, MathExpression>,
) -> Option<MathRelation> {
    // Find the target in the statement and apply the transformation

    // In a real implementation, this would:
    // 1. Find the target expression within the statement
    // 2. Apply the theorem to transform the expression
    // 3. Return the transformed statement

    // For demonstration, we'll just return a clone of the statement
    Some(statement.clone())
}

impl ProofGoal {
    /// Apply a transformation to the current statement
    pub fn transform_statement(
        &self,
        transformer: impl Fn(&MathRelation) -> MathRelation,
    ) -> ProofGoal {
        let mut new_state = self.clone();
        new_state.statement = transformer(&self.statement);
        new_state
    }

    /// Add a variable to the state
    pub fn add_variable(
        &self,
        var_name: &str,
        var_type: MathObject,
        expr: MathExpression,
    ) -> ProofGoal {
        let mut new_state = self.clone();

        // Create a variable binding
        let var = ValueBindedVariable {
            name: Identifier::new_simple(var_name.to_string()),
            value: expr,
        };

        // Add to value_variables
        new_state.value_variables.push(var);

        new_state
    }

    /// Find a subexpression in the state's statement using SearchReplace
    pub fn find_subexpression(
        &self,
        pattern: &MathExpression,
        location: Option<Vec<usize>>,
    ) -> Option<(MathExpression, Vec<usize>)> {
        use tactics::search_replace::SearchReplace;

        if let Some(search_result) = SearchReplace::find_first_in_relation(&self.statement, pattern)
        {
            Some((search_result.expression, search_result.path))
        } else {
            None
        }
    }

    /// Replace a pattern in the statement with a replacement using SearchReplace
    pub fn replace_pattern(
        &self,
        pattern: &MathExpression,
        replacement: &MathExpression,
    ) -> ProofGoal {
        use tactics::search_replace::SearchReplace;

        let new_statement =
            SearchReplace::replace_all_in_relation(&self.statement, pattern, replacement);

        ProofGoal {
            quantifiers: self.quantifiers.clone(),
            value_variables: self.value_variables.clone(),
            statement: new_statement,
        }
    }

    /// Replace at a specific path in the statement using SearchReplace
    pub fn replace_at_path(&self, path: &[usize], replacement: &MathExpression) -> ProofGoal {
        use tactics::search_replace::SearchReplace;

        let new_statement =
            SearchReplace::replace_at_path_in_relation(&self.statement, path, replacement);

        ProofGoal {
            quantifiers: self.quantifiers.clone(),
            value_variables: self.value_variables.clone(),
            statement: new_statement,
        }
    }

    /// Substitute variables in the statement
    pub fn substitute_variables(
        &self,
        substitutions: &HashMap<Identifier, MathExpression>,
    ) -> ProofGoal {
        use tactics::search_replace::SearchReplace;
        let new_statement =
            SearchReplace::substitute_variables_in_relation(&self.statement, substitutions);
        ProofGoal {
            quantifiers: self.quantifiers.clone(),
            value_variables: self.value_variables.clone(),
            statement: new_statement,
        }
    }

    /// Find all matching patterns in the statement
    pub fn find_all_patterns(&self, pattern: &MathExpression) -> Vec<(MathExpression, Vec<usize>)> {
        use tactics::search_replace::SearchReplace;
        SearchReplace::find_all_in_relation(&self.statement, pattern)
            .into_iter()
            .map(|sr| (sr.expression, sr.path))
            .collect()
    }
}

/// Get the global theorem registry
pub fn get_theorem_registry() -> &'static Mutex<TheoremRegistry> {
    // Return a reference to the global registry
    TheoremRegistry::global()
}

/// A single node in a proof tree, representing the application of a tactic
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProofNode {
    /// Unique identifier for this node
    pub id: String,
    /// Parent node ID, if any
    pub parent: Option<String>,
    /// Child node IDs, each child is a branch
    pub children: Vec<String>,
    /// The proof state at this node
    pub state: ProofGoal,
    /// The tactic applied to reach this state
    pub tactic: Option<Tactic>,
    /// Status of this proof branch
    pub status: ProofStatus,
    /// Structured description of this proof step
    pub description: Option<RichText>,
}

impl ProofNode {
    /// The single, unified method for progressing a proof.
    /// It applies the given tactic to the current node's state,
    /// creates a new child node with the result, and adds it to the forest.
    pub fn apply_tactic(&self, tactic: Tactic, forest: &mut ProofForest) -> ProofNode {
        // In a full implementation, the TacticApplier would handle the logic.
        // let result = TacticApplier::apply(&tactic, &self.state);
        // let next_state = match result { ... };

        // For now, we'll assume the tactic doesn't change the state for this example.
        let next_state = self.state.clone();

        let new_node = ProofNode {
            id: Uuid::new_v4().to_string(),
            parent: Some(self.id.clone()),
            children: vec![],
            state: next_state,
            tactic: Some(tactic),
            status: ProofStatus::InProgress,
            description: None, // Descriptions should be part of the tactic application
        };

        forest.add_node(new_node.clone());

        // This is not ideal and will be fixed. We need to get a mutable reference
        // to the parent to add the child, which is complex with Rust's borrow checker.
        // A better design might use IDs and have the forest manage connections.
        // forest.get_node_mut(&self.id).unwrap().children.push(new_node.id.clone());

        new_node
    }

    /// Marks a proof branch as complete.
    pub fn should_complete(self, forest: &mut ProofForest) -> Self {
        let mut node = self;
        node.status = ProofStatus::Complete;
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
}

impl ToProofStep for ProofNode {
    fn to_proof_step(&self) -> ProofStepNode {
        let tactic_name = self
            .tactic
            .as_ref()
            .map_or("Unknown".to_string(), |t| t.to_string());

        ProofStepNode::Statement {
            claim: vec![RichTextSegment::Text(format!(
                "Apply tactic: {}",
                tactic_name
            ))],
            justification: vec![RichTextSegment::Text("Tactic application".to_string())],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofGoal {
    /// Quantified objects in this state
    pub quantifiers: Vec<QuantifiedMathObject>,
    /// Variables with assigned values
    pub value_variables: Vec<ValueBindedVariable>,
    /// The main mathematical relation being proven
    pub statement: MathRelation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValueBindedVariable {
    pub name: Identifier,
    pub value: MathExpression,
}

/// A mathematical object with quantification information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuantifiedMathObject {
    /// The variable representing this object
    pub variable: Identifier,

    /// The type of object
    pub object_type: MathObject, // this is not a higher order logic, unless we change it to MathExpresssion

    /// How this object is quantified
    pub quantification: Quantification,

    /// Human-readable description of the object
    pub description: Option<String>,
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
        let mut forest = Self {
            initial_goal: goal.clone(),
            nodes: HashMap::new(),
            roots: Vec::new(),
        };

        let root_node = ProofNode {
            id: Uuid::new_v4().to_string(),
            parent: None,
            children: Vec::new(),
            state: goal,
            tactic: None,
            status: ProofStatus::Todo,
            description: None,
        };

        forest.roots.push(root_node.id.clone());
        forest.nodes.insert(root_node.id.clone(), root_node);

        forest
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
        let root_id = self.roots[0].clone();
        let initial_state = self.initial_goal.clone();

        // The tactic is applied to get the new state of the root node
        let result = tactic.apply_to_goal(&initial_state);
        let next_state = match result {
            TacticApplicationResult::SingleGoal(goal) => goal,
            _ => panic!("Initial tactic must result in a single goal"),
        };

        let root_node = ProofNode {
            id: Uuid::new_v4().to_string(),
            parent: None,
            children: vec![],
            state: next_state,
            tactic: Some(tactic),
            status: ProofStatus::InProgress,
            description: None,
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
