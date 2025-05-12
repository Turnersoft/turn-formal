// Module: src/formalize_v2/subjects/math/theorem/proof.rs
// Implements a rich proof structure for mathematical theorems with branching support

use js_sys;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::Discriminant;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};
use tactics::TheoremApplicationError;
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Removed unused imports from the refactored traversal module
// use self::traversal::{PotentialTheoremTarget, TargetCollector};
use super::super::theories::zfc::relations::SetTheoryRelation;
use super::expressions::{Identifier, MathExpression, TheoryExpression};
use super::interpretation::TypeViewOperator;
use super::relations::{MathRelation, RelationDetail};
use super::theorem::{MathObject, ProofGoal, Theorem, ValueBindedVariable};
// Import the new traversal trait if needed, or rely on inherent methods
use self::collect::CollectSubExpressions;

pub mod collect;
pub mod path_index;
pub mod tactics;

// Re-export the tactics types for backward compatibility
pub use tactics::{
    CaseAnalysisBuilder, CaseResult, DecompositionMethod, InductionType, RewriteDirection, Tactic,
};

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

        let discriminant = std::mem::discriminant(&theorem.goal.statement);
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
            name: Identifier::Name(var_name.to_string(), 0),
            value: expr,
        };

        // Add to value_variables
        new_state.value_variables.push(var);

        new_state
    }

    /// Find a subexpression in the state's statement
    pub fn find_subexpression(
        &self,
        pattern: &MathExpression,
        location: Option<Vec<usize>>,
    ) -> Option<(MathExpression, Vec<usize>)> {
        // For the test implementation, we'll just check for direct equality with the pattern
        // and return a simple path if it matches

        match &self.statement {
            MathRelation::Equal { left, right, .. } => {
                // Check if left or right side directly matches the pattern
                if left == pattern {
                    return Some((left.clone(), vec![0]));
                }

                if right == pattern {
                    return Some((right.clone(), vec![1]));
                }
            }
            // Handle other relation types with similar simplified approach
            _ => {}
        }

        // Not found
        None
    }
}

/// Navigate to a specific location in an expression
fn navigate_expr<'a>(expr: &'a MathExpression, path: &[usize]) -> Option<&'a MathExpression> {
    if path.is_empty() {
        return Some(expr);
    }

    // For simplicity in the test implementation, we'll just return None
    // In a real implementation, we would handle various MathExpression types
    None
}

/// Replace a subexpression in a math relation
fn replace_subexpr_in_relation(
    relation: &MathRelation,
    expr_to_replace: &MathExpression,
    path: &[usize],
    replacement: &MathExpression,
) -> MathRelation {
    if path.is_empty() {
        return relation.clone();
    }

    match relation {
        MathRelation::Equal { meta, left, right } => {
            if path[0] == 0 {
                // Replace in left side
                MathRelation::Equal {
                    meta: meta.clone(),
                    left: replace_subexpr_in_expr(left, &path[1..], replacement),
                    right: right.clone(),
                }
            } else if path[0] == 1 {
                // Replace in right side
                MathRelation::Equal {
                    meta: meta.clone(),
                    left: left.clone(),
                    right: replace_subexpr_in_expr(right, &path[1..], replacement),
                }
            } else {
                relation.clone()
            }
        }
        MathRelation::And(relations) => {
            if path[0] < relations.len() {
                let mut new_relations = relations.clone();
                new_relations[path[0]] = replace_subexpr_in_relation(
                    &relations[path[0]],
                    expr_to_replace,
                    &path[1..],
                    replacement,
                );
                MathRelation::And(new_relations)
            } else {
                relation.clone()
            }
        }
        MathRelation::Or(relations) => {
            if path[0] < relations.len() {
                let mut new_relations = relations.clone();
                new_relations[path[0]] = replace_subexpr_in_relation(
                    &relations[path[0]],
                    expr_to_replace,
                    &path[1..],
                    replacement,
                );
                MathRelation::Or(new_relations)
            } else {
                relation.clone()
            }
        }
        MathRelation::Implies(ante, cons) => {
            if path[0] == 0 {
                // Replace in antecedent
                MathRelation::Implies(
                    Box::new(replace_subexpr_in_relation(
                        ante,
                        expr_to_replace,
                        &path[1..],
                        replacement,
                    )),
                    cons.clone(),
                )
            } else if path[0] == 1 {
                // Replace in consequent
                MathRelation::Implies(
                    ante.clone(),
                    Box::new(replace_subexpr_in_relation(
                        cons,
                        expr_to_replace,
                        &path[1..],
                        replacement,
                    )),
                )
            } else {
                relation.clone()
            }
        }
        MathRelation::Equivalent(left, right) => {
            if path[0] == 0 {
                // Replace in left side
                MathRelation::Equivalent(
                    Box::new(replace_subexpr_in_relation(
                        left,
                        expr_to_replace,
                        &path[1..],
                        replacement,
                    )),
                    right.clone(),
                )
            } else if path[0] == 1 {
                // Replace in right side
                MathRelation::Equivalent(
                    left.clone(),
                    Box::new(replace_subexpr_in_relation(
                        right,
                        expr_to_replace,
                        &path[1..],
                        replacement,
                    )),
                )
            } else {
                relation.clone()
            }
        }
        // Add more cases for other relation types
        _ => relation.clone(),
    }
}

/// Helper function to find a subexpression within an expression
/// Returns the path to the subexpression if found
fn find_subexpr_in_expr(expr: &MathExpression, pattern: &MathExpression) -> Option<Vec<usize>> {
    // Base case: if the current expression matches the pattern
    if expr == pattern {
        return Some(vec![]);
    }

    // Recursive case: check within subexpressions
    match expr {
        MathExpression::ViewAs { expression, .. } => {
            if let Some(mut path) = find_subexpr_in_expr(expression, pattern) {
                path.insert(0, 0);
                return Some(path);
            }
            None
        }
        MathExpression::Expression(theory_expr) => {
            // Instantiate theory-specific expressions
            match theory_expr {
                // Add cases for theory expressions
                // For now, just return None
                _ => None,
            }
        }
        // Handle other expression types
        _ => None,
    }
}

/// Helper function to replace a subexpression at a specific path
fn replace_subexpr_in_expr(
    expr: &MathExpression,
    path: &[usize],
    replacement: &MathExpression,
) -> MathExpression {
    // If the path is empty, this is the exact expression to replace
    if path.is_empty() {
        return replacement.clone();
    }

    match expr {
        MathExpression::ViewAs { expression, view } => {
            if path[0] == 0 {
                MathExpression::ViewAs {
                    expression: Box::new(replace_subexpr_in_expr(
                        expression,
                        &path[1..],
                        replacement,
                    )),
                    view: view.clone(),
                }
            } else {
                expr.clone()
            }
        }
        // Handle other expression types
        _ => expr.clone(),
    }
}

/// Helper function to create the next path in a proof
fn create_next_path(base_path: Option<String>) -> String {
    if let Some(path) = base_path {
        if path.contains('_') {
            // Increment the last part of the path
            let parts: Vec<&str> = path.rsplitn(2, '_').collect();
            if parts.len() == 2 {
                if let Ok(num) = parts[0].parse::<usize>() {
                    return format!("{}_{}", parts[1], num + 1);
                }
            }
        }

        // If no underscore or parsing fails, add _1
        format!("{}_1", path)
    } else {
        "p1".to_string()
    }
}

/// Helper function to convert an Identifier to a human-readable string
fn name_to_string(id: &Identifier) -> String {
    match id {
        Identifier::Name(name, _) => name.clone(),
        Identifier::O(n) => format!("O{}", n),
        Identifier::M(n) => format!("M{}", n),
        Identifier::E(n) => format!("E{}", n),
        Identifier::N(n) => format!("N{}", n),
    }
}

/// Helper function to get a summary of an expression for display
fn expression_summary(expr: &MathExpression) -> String {
    match expr {
        MathExpression::Var(id) => name_to_string(id),
        MathExpression::Relation(rel) => format!("relation:{:?}", rel),
        MathExpression::Number(n) => format!("{:?}", n),
        MathExpression::Object(_) => "object".to_string(),
        MathExpression::Expression(theory_expr) => match theory_expr {
            TheoryExpression::Group(_) => "group_expression".to_string(),
            TheoryExpression::Ring(_) => "ring_expression".to_string(),
            TheoryExpression::Field(_) => "field_expression".to_string(),
        },
        MathExpression::ViewAs { expression, view } => {
            format!("{} as {:?}", expression_summary(expression), view)
        }
    }
}

/// Helper function to create a MathExpression from a string
fn create_expr(s: &str) -> MathExpression {
    // Check if the string represents an equation (contains "=")
    if s.contains("=") {
        let parts: Vec<&str> = s.split("=").collect();
        if parts.len() == 2 {
            let left_part = parts[0].trim();
            let right_part = parts[1].trim();

            // Create variables from the left and right sides
            let left_expr = MathExpression::var(left_part);
            let right_expr = MathExpression::var(right_part);

            // Create an equality relation
            let relation = MathRelation::equal(left_expr, right_expr);

            // Return the relation wrapped in an expression
            return MathExpression::Relation(Box::new(relation));
        }
    }

    // Default to just creating a variable
    MathExpression::var(s)
}

/// Get the global theorem registry
pub fn get_theorem_registry() -> &'static Mutex<TheoremRegistry> {
    // Return a reference to the global registry
    TheoremRegistry::global()
}

// Helper for creating proof branches with shared context

/// A node in the proof exploration tree
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
}

impl ProofNode {
    /// Apply a tactic and return a new node
    pub fn apply_tactic(&self, tactic: tactics::Tactic, forest: &mut ProofForest) -> ProofNode {
        match tactic {
            tactics::Tactic::TheoremApplication {
                theorem_id,
                instantiation,
                target_expr,
            } => {
                // Store the original tactic details for potential use in an error node
                let original_tactic_for_error_node = tactics::Tactic::TheoremApplication {
                    theorem_id: theorem_id.clone(),
                    instantiation: instantiation.clone(),
                    target_expr: target_expr.clone(),
                };

                // Defer to the method that uses TheoremApplier and handles errors properly
                match self.apply_theorem_with_pattern_matching(
                    &theorem_id,   // Pass theorem_id as &str
                    instantiation, // Pass instantiation by value (moves)
                    target_expr,   // Pass target_expr by value (moves)
                    forest,
                ) {
                    Ok(new_node) => {
                        // explore_theorem_applications is now called inside apply_theorem_with_pattern_matching
                        new_node
                    }
                    Err(e) => {
                        // Create an error node if theorem application fails
                        // let error_message = format!("Error applying theorem: {}", e);
                        let todo_relation = MathRelation::Todo {
                            name: format!("FailedApplication:{}", theorem_id),
                            expressions: vec![],
                        };
                        let error_node_id = Uuid::new_v4().to_string();
                        let error_node = ProofNode {
                            id: error_node_id.clone(),
                            parent: Some(self.id.clone()),
                            children: vec![],
                            state: ProofGoal {
                                quantifier: self.state.quantifier.clone(),
                                value_variables: self.state.value_variables.clone(),
                                statement: todo_relation,
                            },
                            tactic: Some(original_tactic_for_error_node), // Store the original attempted tactic
                            status: ProofStatus::Abandoned,
                        };
                        forest.add_node(error_node.clone());
                        if let Some(parent_node) = forest.nodes.get_mut(&self.id) {
                            parent_node.children.push(error_node.id.clone());
                        }
                        error_node
                    }
                }
            }
            _ => {
                // For all other tactics
                // Apply the tactic to get a new state
                if let Some(new_state) = tactic.apply(&self.state) {
                    // tactic.apply is from tactics/mod.rs
                    let new_node_id = Uuid::new_v4().to_string();
                    // Create the new node
                    let new_node = ProofNode {
                        id: new_node_id.clone(),
                        parent: Some(self.id.clone()),
                        children: vec![],
                        state: new_state,
                        tactic: Some(tactic.clone()), // Clone the tactic for storage
                        status: ProofStatus::InProgress,
                    };

                    // Add the new node to the forest
                    forest.add_node(new_node.clone());

                    // Update parent's children list
                    if let Some(parent) = forest.nodes.get_mut(&self.id) {
                        parent.children.push(new_node.id.clone());
                    }

                    // For non-theorem application tactics, exploration is not automatically triggered here.
                    // If desired in the future, a call to explore_theorem_applications could be added.

                    new_node
                } else {
                    // If tactic application fails, return self (unchanged)
                    self.clone()
                }
            }
        }
    }

    /// Convenience method for introducing an expression
    pub fn tactics_intro_expr(
        &self,
        description: &str,
        expression: MathExpression,
        forest: &mut ProofForest,
    ) -> ProofNode {
        let tactic = tactics::Tactic::Intro {
            name: Identifier::Name(format!("var_{}", self.id), 0),
            expression,
            view: None,
        };

        self.apply_tactic(tactic, forest)
    }

    /// Convenience method for substituting expressions
    pub fn tactics_subs_expr(
        &self,
        target: MathExpression,
        replacement: MathExpression,
        location: Option<Vec<usize>>,
        forest: &mut ProofForest,
    ) -> ProofNode {
        let tactic = tactics::Tactic::Substitution {
            target,
            replacement,
            location,
        };

        self.apply_tactic(tactic, forest)
    }

    /// Convenience method for applying theorems
    pub fn tactics_theorem_app_expr(
        &self,
        theorem_id: &str,
        instantiation: HashMap<Identifier, MathExpression>,
        target_expr: Option<MathExpression>,
        forest: &mut ProofForest,
    ) -> ProofNode {
        let tactic = tactics::Tactic::TheoremApplication {
            theorem_id: theorem_id.to_string(),
            instantiation,
            target_expr,
        };

        self.apply_tactic(tactic, forest)
    }

    /// Mark this proof branch as complete
    pub fn should_complete(self, forest: &mut ProofForest) -> Self {
        if let Some(node) = forest.nodes.get_mut(&self.id) {
            node.status = ProofStatus::Complete;
        }
        self
    }

    /// Create a case analysis
    pub fn case_analysis<'a>(
        &self,
        forest: &'a mut ProofForest,
    ) -> tactics::CaseAnalysisBuilder<'a> {
        tactics::CaseAnalysisBuilder::new(self.clone(), forest)
    }

    /// Apply a theorem with pattern matching and proper error handling
    pub fn apply_theorem_with_pattern_matching(
        &self,
        theorem_id: &str,
        initial_instantiation: HashMap<Identifier, MathExpression>,
        target_expr: Option<MathExpression>,
        forest: &mut ProofForest,
    ) -> Result<ProofNode, TheoremApplicationError> {
        let registry_guard = get_theorem_registry().lock().unwrap();
        let applier = tactics::TheoremApplier::new(&registry_guard);

        let target_expr_ref = target_expr.as_ref();
        let target_path: Option<&[usize]> = None; // TODO: Determine path properly

        let result = applier.apply_theorem(
            theorem_id,
            &initial_instantiation,
            target_expr_ref,
            target_path,
            &self.state,
        )?;

        let applied_tactic = tactics::Tactic::TheoremApplication {
            theorem_id: theorem_id.to_string(),
            instantiation: result.instantiations,
            target_expr,
        };

        let new_node_id = Uuid::new_v4().to_string();
        let new_node = ProofNode {
            id: new_node_id.clone(),
            parent: Some(self.id.clone()),
            children: vec![],
            state: result.new_goal,
            tactic: Some(applied_tactic),
            status: ProofStatus::InProgress,
        };

        forest.add_node(new_node.clone());
        if let Some(parent_node) = forest.nodes.get_mut(&self.id) {
            parent_node.children.push(new_node.id.clone());
        }

        new_node.explore_theorem_applications(forest, &registry_guard);

        Ok(new_node)
    }

    /// Apply a theorem with pattern matching and handling error as a string message
    pub fn tactics_theorem_apply_with_pattern_matching(
        &self,
        theorem_id: &str,
        instantiation: HashMap<Identifier, MathExpression>,
        target_expr: Option<MathExpression>,
        forest: &mut ProofForest,
    ) -> ProofNode {
        let initial_instantiation_clone = instantiation.clone();
        let target_expr_clone = target_expr.clone();

        match self.apply_theorem_with_pattern_matching(
            theorem_id,
            instantiation,
            target_expr,
            forest,
        ) {
            Ok(node) => node,
            Err(e) => {
                let _error_message = format!("Error applying theorem: {}", e);
                let todo_relation = MathRelation::Todo {
                    name: format!("FailedApplication:{}", theorem_id),
                    expressions: vec![],
                };

                let failed_tactic_instantiation: HashMap<Identifier, MathExpression> =
                    initial_instantiation_clone
                        .into_iter()
                        .map(|(name, expr)| (name, expr))
                        .collect();

                let failed_tactic = tactics::Tactic::TheoremApplication {
                    theorem_id: theorem_id.to_string(),
                    instantiation: failed_tactic_instantiation,
                    target_expr: target_expr_clone,
                };

                let new_node = ProofNode {
                    id: Uuid::new_v4().to_string(),
                    parent: Some(self.id.clone()),
                    children: vec![],
                    state: ProofGoal {
                        quantifier: self.state.quantifier.clone(),
                        value_variables: self.state.value_variables.clone(),
                        statement: todo_relation,
                    },
                    tactic: Some(failed_tactic),
                    status: ProofStatus::Abandoned,
                };

                forest.add_node(new_node.clone());
                if let Some(parent_node) = forest.nodes.get_mut(&self.id) {
                    parent_node.children.push(new_node.id.clone());
                }
                new_node
            }
        }
    }

    /// Create an expression to be reused in mathematical proofs from a theorem's statement
    pub fn create_expression_from_theorem(
        &self,
        theorem_id: &str,
        instantiation: HashMap<String, MathExpression>,
        forest: &mut ProofForest,
    ) -> Result<MathExpression, TheoremApplicationError> {
        // Get theorem registry
        let registry = get_theorem_registry().lock().unwrap();

        // Get theorem
        let theorem = registry
            .get_theorem(theorem_id)
            .ok_or_else(|| TheoremApplicationError::TheoremNotFound(theorem_id.to_string()))?;

        // Simply wrap the statement in a MathExpression
        let statement_expr = MathExpression::Relation(Box::new(theorem.goal.statement.clone()));

        // A complete implementation would instantiate variables, but this is simplified
        Ok(statement_expr)
    }

    /// Helper method to extract a reusable expression from a theorem and introduce it
    pub fn tactics_intro_theorem_result(
        &self,
        description: &str,
        theorem_id: &str,
        instantiation: HashMap<String, MathExpression>,
        forest: &mut ProofForest,
    ) -> ProofNode {
        // Try to create expression from theorem
        match self.create_expression_from_theorem(theorem_id, instantiation.clone(), forest) {
            Ok(expr) => {
                // Introduce the expression
                self.tactics_intro_expr(description, expr, forest)
            }
            Err(e) => {
                // Create error node
                let error_message = format!("Error extracting theorem result: {}", e);
                let todo_relation = MathRelation::Todo {
                    name: format!("FailedExtraction:{}", theorem_id),
                    expressions: vec![],
                };

                // Create intro tactic that was attempted but failed
                let failed_tactic = tactics::Tactic::Intro {
                    name: Identifier::Name(description.to_string(), 0),
                    expression: MathExpression::Relation(Box::new(MathRelation::Todo {
                        name: format!("TheoremResult:{}", theorem_id),
                        expressions: vec![],
                    })),
                    view: None,
                };

                let new_node = ProofNode {
                    id: Uuid::new_v4().to_string(),
                    parent: Some(self.id.clone()),
                    children: vec![],
                    state: ProofGoal {
                        quantifier: self.state.quantifier.clone(),
                        value_variables: self.state.value_variables.clone(),
                        statement: todo_relation,
                    },
                    tactic: Some(failed_tactic), // Store the failed tactic
                    status: ProofStatus::Abandoned,
                };

                forest.add_node(new_node.clone());

                if let Some(parent_node) = forest.nodes.get_mut(&self.id) {
                    parent_node.children.push(new_node.id.clone());
                }

                new_node
            }
        }
    }

    /// Explores all possible theorem applications to sub-expressions in the current node's goal,
    /// creating new proof branches in the forest for each successful application.
    pub fn explore_theorem_applications(
        &self,
        forest: &mut ProofForest,
        registry: &TheoremRegistry,
    ) -> Vec<String> {
        let mut collected_targets: Vec<(Vec<usize>, MathExpression)> = Vec::new();
        let base_path = Vec::new(); // Start with an empty path for the root statement
        let applier = tactics::TheoremApplier::new(registry);
        let mut new_branch_ids = Vec::new();

        // Step 1: Collect all sub-expressions from the current node's statement using the new method
        let statement_as_expr = MathExpression::Relation(Box::new(self.state.statement.clone()));
        // Use the CollectSubExpressions trait method directly on MathExpression
        statement_as_expr.collect_sub_expressions_with_paths(base_path, &mut collected_targets, 0);

        // Step 2: Iterate through collected targets and try to apply theorems
        for (target_path, target_sub_expression) in collected_targets {
            let mut theorems_to_try: Vec<String> = Vec::new();

            // Determine theorems to try based on the kind of the target sub-expression
            // This logic remains largely the same, focusing on MathExpression::Relation targets
            if let MathExpression::Relation(inner_relation_box) = &target_sub_expression {
                let discriminant = std::mem::discriminant(inner_relation_box.as_ref());
                if let Some(indexed_ids) = registry.get_theorems_by_relation_kind(&discriminant) {
                    theorems_to_try.extend(indexed_ids.clone());
                }
            }
            // Optional: Add logic here to try theorems based on other target_sub_expression types if needed
            // else if let MathExpression::Expression(TheoryExpression::Group(_)) = target_sub_expression { ... }

            // Consider adding a general fallback if specific indexing yields no theorems:
            // if theorems_to_try.is_empty() {
            //     theorems_to_try = registry.list_all_theorems();
            // }

            for theorem_id_str in theorems_to_try {
                let theorem_id = theorem_id_str.as_str();
                let instantiations = HashMap::new();

                // --- APPLICATION LOGIC ---
                // Pass target_sub_expression by reference and the collected target_path

                match applier.apply_theorem(
                    theorem_id,
                    &instantiations,
                    Some(&target_sub_expression), // Pass as reference
                    Some(&target_path),           // Pass the actual path
                    &self.state,
                ) {
                    Ok(application_result) => {
                        let tactic_used = tactics::Tactic::TheoremApplication {
                            theorem_id: theorem_id.to_string(),
                            instantiation: application_result.instantiations.clone(),
                            target_expr: Some(target_sub_expression.clone()), // Clone for tactic storage
                        };
                        let new_node_id_str = Uuid::new_v4().to_string();
                        let new_node = ProofNode {
                            id: new_node_id_str.clone(),
                            parent: Some(self.id.clone()),
                            children: Vec::new(),
                            state: application_result.new_goal,
                            tactic: Some(tactic_used),
                            status: ProofStatus::InProgress,
                        };
                        forest.add_node(new_node);
                        if let Some(parent_node_in_forest) = forest.nodes.get_mut(&self.id) {
                            parent_node_in_forest.children.push(new_node_id_str.clone());
                        }
                        new_branch_ids.push(new_node_id_str);
                    }
                    Err(_error) => {}
                }
            }
        }
        new_branch_ids
    }
}

/// A forest of proof exploration nodes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofForest {
    /// All nodes in the forest
    pub nodes: HashMap<String, ProofNode>,
    /// Root node ID for each tree in the forest
    pub roots: Vec<String>,
}

impl ProofForest {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            roots: vec![],
        }
    }

    pub fn add_node(&mut self, node: ProofNode) -> () {
        self.nodes.insert(node.id.clone(), node);
    }

    /// Initialize a proof forest for a theorem
    pub fn initialize_branch(theorem: &Theorem) -> ProofNode {
        let mut forest = Self::new();

        // Create an "initialization" tactic to represent the starting point
        let init_tactic = tactics::Tactic::Custom {
            name: "init".to_string(),
            args: vec![format!("theorem:{}", theorem.id)],
        };

        let root_node = ProofNode {
            id: Uuid::new_v4().to_string(),
            parent: None,
            children: vec![],
            state: theorem.goal.clone(),
            tactic: Some(init_tactic),
            status: ProofStatus::InProgress,
        };

        forest.add_node(root_node.clone());
        forest.roots.push(root_node.id.clone());

        root_node
    }
}

/// Find a subexpression in a relation
fn find_subexpr_in_relation(
    relation: &MathRelation,
    target: &MathExpression,
) -> Option<(MathExpression, Vec<usize>)> {
    match relation {
        MathRelation::Equal { left, right, .. } => {
            if left == target {
                return Some((left.clone(), vec![0]));
            }
            if right == target {
                return Some((right.clone(), vec![1]));
            }

            // Search within left and right - fix the pattern matching to use Option<Vec<usize>>
            if let Some(path) = find_subexpr_in_expr(left, target) {
                let mut full_path = vec![0];
                full_path.extend(path);
                return Some((left.clone(), full_path));
            }

            if let Some(path) = find_subexpr_in_expr(right, target) {
                let mut full_path = vec![1];
                full_path.extend(path);
                return Some((right.clone(), full_path));
            }

            None
        }
        // Add cases for other relation types
        _ => None,
    }
}

/// Find a relation containing an expression
fn find_relation_containing_expr(
    relation: &MathRelation,
    target: &MathExpression,
) -> Option<(MathRelation, Vec<usize>)> {
    match relation {
        MathRelation::Equal { left, right, .. } => {
            // Fix the pattern matching to use Option<Vec<usize>>
            if let Some(_path) = find_subexpr_in_expr(left, target) {
                return Some((relation.clone(), vec![0]));
            }
            if let Some(_path) = find_subexpr_in_expr(right, target) {
                return Some((relation.clone(), vec![1]));
            }
            None
        }
        // Add cases for other relation types
        _ => None,
    }
}
