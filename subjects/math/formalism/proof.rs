// Module: src/formalize_v2/subjects/math/theorem/proof.rs
// Implements a rich proof structure for mathematical theorems with branching support

use js_sys;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use super::super::theories::zfc::relations::SetTheoryRelation;
use super::core::{MathObject, ProofGoal, Theorem, ValueBindedVariable};
use super::expressions::{Identifier, MathExpression, TheoryExpression};
use super::interpretation::TypeViewOperator;
use super::relations::{MathRelation, RelationDetail};

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

/// A tactic that can be applied to a proof state to transform it
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Tactic {
    /// Introduce a new variable or hypothesis
    Intro {
        /// Name to give the introduced variable
        name: Identifier,
        /// Expression for the variable
        expression: MathExpression,
        /// Optional view operator
        view: Option<TypeViewOperator>,
    },
    /// Apply a hypothesis or theorem to the goal
    Apply {
        /// ID of the theorem or hypothesis to apply
        theorem_id: String,
        /// Variable instantiations
        instantiation: HashMap<String, MathExpression>,
        /// Target expression to apply to (if any)
        target_expr: Option<MathExpression>,
    },
    /// Substitute an expression for another in a proof
    Substitution {
        /// The expression to substitute
        target: MathExpression,
        /// What to replace it with
        replacement: MathExpression,
        /// Location within the expression
        location: Option<Vec<usize>>,
    },
    /// Change the view of a mathematical object
    ChangeView {
        /// The object to change view of
        object: MathExpression,
        /// The new view to use
        view: TypeViewOperator,
    },
    /// Application of a previously proven theorem
    TheoremApplication {
        /// ID of the theorem to apply
        theorem_id: String,
        /// Variable instantiations for the theorem parameters
        instantiation: HashMap<String, MathExpression>,
        /// Target expression within the goal to focus the application on
        target_expr: Option<MathExpression>,
    },
    /// Decompose an expression into its components
    Decompose {
        /// The target expression to decompose
        target: MathExpression,
        /// Method of decomposition
        method: DecompositionMethod,
    },
    /// Simplify an expression
    Simplify {
        /// The target expression to simplify
        target: MathExpression,
        /// Optional hints for simplification
        hints: Option<Vec<String>>,
    },
    /// Apply mathematical induction
    Induction {
        /// Variable to induct on
        name: Identifier,
        /// Type of induction to apply
        induction_type: InductionType,
        /// Optional induction schema
        schema: Option<MathExpression>,
    },
    /// Custom tactic defined by the user
    Custom {
        /// Name of the custom tactic
        name: String,
        /// Arguments for the custom tactic
        args: Vec<String>,
    },
    /// Case analysis on an expression
    CaseAnalysis {
        /// Target expression for case analysis
        target_expr: MathExpression,
        /// Expressions for each case
        case_exprs: Vec<MathExpression>,
        /// Names for each case
        case_names: Vec<String>,
    },
    /// Rewrite using an equation
    Rewrite {
        /// Target expression to rewrite
        target_expr: MathExpression,
        /// Equation to use for rewriting
        equation_expr: MathExpression,
        /// Direction of rewriting
        direction: RewriteDirection,
        /// Location within the expression
        location: Option<Vec<usize>>,
    },
    /// Branch to create a new proof path
    Branch {
        /// Description of the new proof path
        description: String,
    },
    /// Case analysis case
    Case {
        /// Parent case node ID
        parent_id: String,
        /// Case expression
        case_expr: MathExpression,
        /// Case name
        case_name: String,
    },
}

/// Direction for rewriting expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RewriteDirection {
    /// Left to right (replace left side with right side)
    LeftToRight,
    /// Right to left (replace right side with left side)
    RightToLeft,
}

/// Methods for decomposing expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DecompositionMethod {
    /// Break into components
    Components,
    /// Factor out common terms
    Factor,
    /// Expand into sum of products
    Expand,
    /// Other domain-specific method
    Other(String),
}

/// Types of induction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InductionType {
    /// Mathematical induction on natural numbers
    Natural,
    /// Structural induction
    Structural,
    /// Transfinite induction
    Transfinite,
    /// Well-founded induction
    WellFounded,
    /// Other induction type
    Other(String),
}

/// A registry of theorems that can be applied during proofs
#[derive(Debug, Clone)]
pub struct TheoremRegistry {
    /// Map of theorem IDs to Theorem objects
    theorems: HashMap<String, Theorem>,
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
        }
    }

    /// Register a theorem
    pub fn register(&mut self, theorem: Theorem) {
        println!("Registering theorem: {}", theorem.name);
        self.theorems.insert(theorem.id.clone(), theorem);
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

impl Tactic {
    /// Apply a tactic to a proof state
    pub fn apply(&self, state: &ProofGoal) -> Option<ProofGoal> {
        match self {
            Tactic::Intro {
                name,
                expression,
                view: _,
            } => {
                let mut new_state = state.clone();

                // Create a new variable binding
                let var = ValueBindedVariable {
                    name: name.clone(),
                    value: expression.clone(),
                };

                new_state.value_variables.push(var);
                // No path or justification to set

                Some(new_state)
            }
            Tactic::Apply {
                theorem_id,
                instantiation,
                target_expr,
            } => {
                // Apply a theorem or hypothesis from the context
                // This is a complex operation that would normally require careful verification
                // For this prototype, we'll simulate a simplified application
                let registry = get_theorem_registry().lock().unwrap();
                if let Some(result) = registry.apply_theorem(
                    theorem_id,
                    &state.statement,
                    instantiation,
                    target_expr.clone(),
                ) {
                    let mut new_state = state.clone();
                    new_state.statement = result;
                    Some(new_state)
                } else {
                    None
                }
            }
            Tactic::Substitution {
                target,
                replacement,
                location,
            } => {
                // Try to find the target in the relation
                if let Some((_, path)) = state.find_subexpression(target, location.clone()) {
                    let mut new_state = state.clone();
                    new_state.statement =
                        replace_subexpr_in_relation(&state.statement, target, &path, replacement);
                    // No path or justification to set

                    Some(new_state)
                } else {
                    None
                }
            }
            Tactic::ChangeView { object, view } => {
                // Change view of a mathematical object
                // This is more complex and would require proper implementation
                Some(state.clone())
            }
            Tactic::TheoremApplication {
                theorem_id,
                instantiation,
                target_expr,
            } => {
                // Apply a theorem from the registry
                let registry = get_theorem_registry().lock().unwrap();
                if let Some(result) = registry.apply_theorem(
                    theorem_id,
                    &state.statement,
                    instantiation,
                    target_expr.clone(),
                ) {
                    let mut new_state = state.clone();
                    new_state.statement = result;
                    // No path or justification to set

                    Some(new_state)
                } else {
                    None
                }
            }
            Tactic::Rewrite {
                target_expr,
                equation_expr,
                direction,
                location,
            } => {
                let mut new_state = state.clone();
                // No path to set

                // Check if the equation_expr is a relation
                if let MathExpression::Relation(equation_box) = equation_expr {
                    let equation = &**equation_box;

                    // Process the rewrite
                    if let MathRelation::Equal {
                        meta: _,
                        left,
                        right,
                    } = equation
                    {
                        // Determine which side to replace based on direction
                        let (to_replace, replacement) = match direction {
                            RewriteDirection::LeftToRight => (left, right),
                            RewriteDirection::RightToLeft => (right, left),
                        };

                        // Find target in statement
                        if let Some((_, path)) =
                            state.find_subexpression(target_expr, location.clone())
                        {
                            // Perform the replacement
                            new_state.statement = replace_subexpr_in_relation(
                                &state.statement,
                                target_expr,
                                &path,
                                replacement,
                            );

                            // Success - no justification needed

                            Some(new_state)
                        } else {
                            // Target not found - no justification needed

                            None
                        }
                    } else {
                        // Not an equality relation - no justification needed

                        None
                    }
                } else {
                    // Not a relation - no justification needed

                    None
                }
            }
            Tactic::CaseAnalysis {
                target_expr,
                case_exprs: _,
                case_names,
            } => {
                let mut new_state = state.clone();

                // No path or justification to set

                Some(new_state)
            }
            // Handle other tactics - remove path/justification references
            _ => legacy_apply(self, state),
        }
    }

    /// Generate a human-readable description of this tactic
    pub fn describe(&self) -> String {
        match self {
            Tactic::Intro {
                name, expression, ..
            } => {
                format!(
                    "Introduce '{}' as expression {}",
                    name_to_string(name),
                    expression_summary(expression)
                )
            }
            Tactic::Apply {
                theorem_id,
                instantiation,
                target_expr,
            } => {
                let target_str = match target_expr {
                    Some(target) => format!(" to {}", expression_summary(target)),
                    None => "".to_string(),
                };
                format!("Apply theorem '{}'", theorem_id)
            }
            Tactic::Substitution {
                target,
                replacement,
                location: _,
            } => {
                format!(
                    "Substitute {} with {}",
                    expression_summary(target),
                    expression_summary(replacement)
                )
            }
            Tactic::ChangeView { object, view } => {
                format!(
                    "Change view of {} as {:?}",
                    expression_summary(object),
                    view
                )
            }
            Tactic::TheoremApplication {
                theorem_id,
                instantiation,
                target_expr,
            } => {
                let target_str = match target_expr {
                    Some(target) => format!(" to {}", expression_summary(target)),
                    None => "".to_string(),
                };
                format!("Apply theorem '{}'", theorem_id)
            }
            Tactic::Decompose { target, method } => {
                let method_str = match method {
                    DecompositionMethod::Components => "components",
                    DecompositionMethod::Factor => "factoring",
                    DecompositionMethod::Expand => "expansion",
                    DecompositionMethod::Other(s) => s,
                };
                format!(
                    "Decomposed {} by {}",
                    expression_summary(target),
                    method_str
                )
            }
            Tactic::Simplify { target, hints } => {
                let hints_str = match hints {
                    Some(h) if !h.is_empty() => format!(" with hints: {}", h.join(", ")),
                    _ => "".to_string(),
                };
                format!("Simplified {}{}", expression_summary(target), hints_str)
            }
            Tactic::Induction {
                name,
                induction_type,
                schema,
            } => {
                let schema_str = match schema {
                    Some(s) => format!(" with schema {}", expression_summary(s)),
                    None => "".to_string(),
                };

                match induction_type {
                    InductionType::Natural => format!(
                        "Applied mathematical induction on {}{}",
                        name_to_string(name),
                        schema_str
                    ),
                    InductionType::Structural => format!(
                        "Applied structural induction on {}{}",
                        name_to_string(name),
                        schema_str
                    ),
                    InductionType::Transfinite => format!(
                        "Applied transfinite induction on {}{}",
                        name_to_string(name),
                        schema_str
                    ),
                    InductionType::WellFounded => format!(
                        "Applied well-founded induction on {}{}",
                        name_to_string(name),
                        schema_str
                    ),
                    InductionType::Other(s) => format!(
                        "Applied {} induction on {}{}",
                        s,
                        name_to_string(name),
                        schema_str
                    ),
                }
            }
            Tactic::Custom { name, args } => {
                let args_str = if args.is_empty() {
                    "".to_string()
                } else {
                    format!(" with {} arguments", args.len())
                };
                format!("Applied custom tactic: {}{}", name, args_str)
            }
            Tactic::CaseAnalysis {
                target_expr,
                case_names,
                case_exprs: _,
            } => {
                // Include the case names in the justification
                let cases_str = case_names.join(", ");
                format!(
                    "Case analysis on {} with {} cases: {}",
                    expression_summary(target_expr),
                    case_names.len(),
                    cases_str
                )
            }
            Tactic::Rewrite {
                target_expr,
                equation_expr,
                direction,
                location: _,
            } => {
                let dir_str = match direction {
                    RewriteDirection::LeftToRight => "left to right",
                    RewriteDirection::RightToLeft => "right to left",
                };

                format!(
                    "Rewrote {} using {} ({})",
                    expression_summary(target_expr),
                    expression_summary(equation_expr),
                    dir_str
                )
            }
            Tactic::Branch { description } => {
                format!("Branch: {}", description)
            }
            Tactic::Case {
                parent_id,
                case_expr,
                case_name,
            } => {
                format!(
                    "Case analysis case: {} with case expression {} and name {}",
                    parent_id,
                    expression_summary(case_expr),
                    case_name
                )
            }
        }
    }
}

/// Legacy implementation for other tactic variants
fn legacy_apply(tactic: &Tactic, state: &ProofGoal) -> Option<ProofGoal> {
    let mut new_state = state.clone();

    // Set appropriate handling based on tactic type (without path/justification)
    match tactic {
        Tactic::Branch { description } => {
            // Just return the state as-is for branching
            Some(new_state)
        }
        _ => {
            // For unimplemented tactics, just return the state as-is
            Some(new_state)
        }
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
    pub fn apply_tactic(&self, tactic: Tactic, forest: &mut ProofForest) -> ProofNode {
        // Apply the tactic to get a new state
        if let Some(new_state) = tactic.apply(&self.state) {
            // Create a new node ID (next available)

            // Create the new node
            let new_node = ProofNode {
                id: Uuid::new_v4().to_string(),
                parent: Some(self.id.clone()),
                children: vec![],
                state: new_state,
                tactic: Some(tactic),
                status: ProofStatus::InProgress,
            };

            // Add the new node to the forest
            forest.add_node(new_node.clone());

            // Update parent's children list
            if let Some(parent) = forest.nodes.get_mut(&self.id) {
                parent.children.push(new_node.id.clone());
            }

            new_node
        } else {
            // If tactic application fails, return self (unchanged)
            self.clone()
        }
    }

    /// Convenience method for introducing an expression
    pub fn tactics_intro_expr(
        &self,
        description: &str,
        expression: MathExpression,
        forest: &mut ProofForest,
    ) -> ProofNode {
        let tactic = Tactic::Intro {
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
        let tactic = Tactic::Substitution {
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
        instantiation: HashMap<String, MathExpression>,
        target_expr: Option<MathExpression>,
        forest: &mut ProofForest,
    ) -> ProofNode {
        let tactic = Tactic::TheoremApplication {
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
    pub fn case_analysis<'a>(&self, forest: &'a mut ProofForest) -> CaseAnalysisBuilder<'a> {
        CaseAnalysisBuilder::new(self.clone(), forest)
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

        let root_node = ProofNode {
            id: Uuid::new_v4().to_string(),
            parent: None,
            children: vec![],
            state: theorem.goal.clone(),
            tactic: None,
            status: ProofStatus::InProgress,
        };

        forest.add_node(root_node.clone());
        forest.roots.push(root_node.id.clone());

        root_node
    }
}

/// Updated CaseAnalysisBuilder to use mutable forest
pub struct CaseAnalysisBuilder<'a> {
    parent_branch: ProofNode,
    target: Option<String>,
    case_node_id: Option<String>, // Single node ID for all cases
    cases: Vec<(String, ProofNode)>,
    forest: &'a mut ProofForest,
}

impl<'a> CaseAnalysisBuilder<'a> {
    pub fn new(parent: ProofNode, forest: &'a mut ProofForest) -> Self {
        Self {
            parent_branch: parent,
            target: None,
            case_node_id: None,
            cases: Vec::new(),
            forest,
        }
    }

    pub fn on_expression(&mut self, target: impl Into<String>) -> &mut Self {
        self.target = Some(target.into());
        self
    }

    // Create or get the case analysis parent node
    fn ensure_case_parent(&mut self) -> String {
        // If we already have a case node, return its ID
        if let Some(id) = &self.case_node_id {
            return id.clone();
        }

        // Get a meaningful target expression from the target field or use a default
        let target_name = self
            .target
            .clone()
            .unwrap_or_else(|| "case_analysis".to_string());

        // Otherwise, create a case analysis parent node
        let tactic = Tactic::CaseAnalysis {
            target_expr: MathExpression::Var(Identifier::Name(target_name, 0)),
            case_exprs: vec![],
            case_names: vec![],
        };

        // Create the case analysis node
        let case_node = ProofNode {
            id: Uuid::new_v4().to_string(),
            parent: Some(self.parent_branch.id.clone()),
            children: vec![],
            state: self.parent_branch.state.clone(),
            tactic: Some(tactic),
            status: ProofStatus::InProgress,
        };

        // Add to forest
        let node_id = case_node.id.clone();
        self.forest.add_node(case_node);

        // Add to parent's children
        if let Some(parent) = self.forest.nodes.get_mut(&self.parent_branch.id) {
            parent.children.push(node_id.clone());
        }

        // Save and return the ID
        self.case_node_id = Some(node_id.clone());
        node_id
    }

    pub fn case<F>(&mut self, description: &str, case_fn: F) -> &mut Self
    where
        F: FnOnce(ProofNode, &mut ProofForest) -> ProofNode,
    {
        // Get or create the case analysis parent node
        let case_parent_id = self.ensure_case_parent();

        // Create a specialized ProofGoal for this case - without path/justification
        let case_goal = if let Some(parent_node) = self.forest.nodes.get(&case_parent_id) {
            // Just use the parent's goal - we don't have path/justification anymore
            parent_node.state.clone()
        } else {
            // Fallback: just use parent's state if case parent isn't found
            self.parent_branch.state.clone()
        };

        // Create a case node for this specific case using CaseAnalysis tactic
        // This makes it part of an exhaustive case analysis rather than a separate branch
        let case_tactic = Tactic::Case {
            parent_id: case_parent_id.clone(),
            case_expr: MathExpression::Var(Identifier::Name(
                format!("case_{}", self.cases.len() + 1),
                0,
            )),
            case_name: description.to_string(),
        };

        let case_branch = ProofNode {
            id: Uuid::new_v4().to_string(),
            parent: Some(case_parent_id.clone()),
            children: vec![],
            state: case_goal, // Use the case-specific goal
            tactic: Some(case_tactic),
            status: ProofStatus::InProgress,
        };

        // Add to forest - IMPORTANT: Register the initial case branch
        let case_branch_id = case_branch.id.clone();
        self.forest.add_node(case_branch.clone());

        // Add as child of the case parent node
        if let Some(parent) = self.forest.nodes.get_mut(&case_parent_id) {
            if !parent.children.contains(&case_branch_id) {
                parent.children.push(case_branch_id.clone());
            }
        }

        // Apply the case function to build this branch
        let result_branch = case_fn(case_branch, self.forest);

        // Store the case description and resulting node
        self.cases.push((description.to_string(), result_branch));

        self
    }

    pub fn build(&mut self) -> CaseResult {
        // If no cases were added, return an empty result
        if self.cases.is_empty() {
            return CaseResult {
                parent: self.parent_branch.clone(),
                target: self.target.clone(),
                case_nodes: vec![],
            };
        }

        // Ensure all case nodes are properly linked to the case parent
        if let Some(case_parent_id) = &self.case_node_id {
            // First, check if all cases are complete
            let all_cases_complete = {
                let mut complete = true;
                for (_, branch) in &self.cases {
                    if let Some(case_node) = self.forest.nodes.get(&branch.id) {
                        if case_node.status != ProofStatus::Complete {
                            complete = false;
                            break;
                        }
                    } else {
                        complete = false;
                        break;
                    }
                }
                complete
            };

            // Now update the case parent node
            if let Some(parent_node) = self.forest.nodes.get_mut(case_parent_id) {
                if let Some(Tactic::CaseAnalysis {
                    case_exprs,
                    case_names,
                    target_expr: _,
                }) = &mut parent_node.tactic
                {
                    // Clear and rebuild the case names
                    case_names.clear();
                    for (desc, _) in &self.cases {
                        case_names.push(desc.clone());
                    }
                }

                // Verify all cases are in the parent's children list
                let mut missing_children = Vec::new();
                for (_, branch) in &self.cases {
                    if !parent_node.children.contains(&branch.id) {
                        missing_children.push(branch.id.clone());
                    }
                }

                // Add any missing children
                for child_id in missing_children {
                    parent_node.children.push(child_id);
                }

                // Update status if all cases are complete
                if all_cases_complete {
                    parent_node.status = ProofStatus::Complete;
                }
            }
        }

        CaseResult {
            parent: self.parent_branch.clone(),
            target: self.target.clone(),
            case_nodes: self
                .cases
                .iter()
                .map(|(_, branch)| branch.clone())
                .collect(),
        }
    }

    pub fn add_case(
        &mut self,
        case_expr: MathExpression,
        case_name: String,
    ) -> Result<ProofNode, String> {
        // Get parent case node ID, creating it if necessary
        let case_parent_id = self.ensure_case_parent();

        // Get the parent case node
        let case_parent = self.forest.nodes.get_mut(&case_parent_id).unwrap();

        // Update the case analysis tactic with the new case
        if let Some(Tactic::CaseAnalysis {
            case_exprs,
            case_names,
            target_expr: _,
        }) = &mut case_parent.tactic
        {
            case_exprs.push(case_expr.clone());
            case_names.push(case_name.clone());
        }

        // Create a new branch for this specific case with the same state
        let new_node = ProofNode {
            id: Uuid::new_v4().to_string(),
            parent: Some(case_parent_id.clone()),
            children: vec![],
            state: case_parent.state.clone(),
            tactic: Some(Tactic::Case {
                parent_id: case_parent_id.clone(),
                case_expr: case_expr,
                case_name: case_name,
            }),
            status: ProofStatus::InProgress,
        };

        // Store the node ID
        let node_id = new_node.id.clone();

        // Add to parent's children
        case_parent.children.push(node_id.clone());

        // Add to forest
        self.forest.add_node(new_node.clone());

        Ok(new_node)
    }
}

/// Result of a case analysis
#[derive(Debug, Clone)]
pub struct CaseResult {
    /// The parent branch
    pub parent: ProofNode,
    /// The target expression
    pub target: Option<String>,
    /// The top level nodes for each case
    pub case_nodes: Vec<ProofNode>,
}
