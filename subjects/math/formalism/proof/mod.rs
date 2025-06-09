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
use super::super::theories::zfc::definitions::SetRelation;
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

// Remove the invalid re-exports from super
// These are defined in this file, no need to import them from super
// pub use super::ProofForest;
// pub use super::ProofNode;
// pub use super::TheoremRegistry;
// pub use super::get_theorem_registry;

use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::theories::groups::definitions::GroupExpression;
use crate::subjects::math::theories::rings::definitions::{FieldExpression, RingExpression};

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
            quantifier: self.quantifier.clone(),
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
            quantifier: self.quantifier.clone(),
            value_variables: self.value_variables.clone(),
            statement: new_statement,
        }
    }

    /// Apply variable substitutions using SearchReplace
    pub fn substitute_variables(
        &self,
        substitutions: &HashMap<Identifier, MathExpression>,
    ) -> ProofGoal {
        use tactics::search_replace::SearchReplace;

        let new_statement =
            SearchReplace::substitute_variables_in_relation(&self.statement, substitutions);

        ProofGoal {
            quantifier: self.quantifier.clone(),
            value_variables: self.value_variables.clone(),
            statement: new_statement,
        }
    }

    /// Find all occurrences of a pattern in the statement
    pub fn find_all_patterns(&self, pattern: &MathExpression) -> Vec<(MathExpression, Vec<usize>)> {
        use tactics::search_replace::SearchReplace;

        SearchReplace::find_all_in_relation(&self.statement, pattern)
            .into_iter()
            .map(|result| (result.expression, result.path))
            .collect()
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
        MathExpression::Expression(theory_expr) => match theory_expr {
            TheoryExpression::Group(group_expr) => find_subexpr_in_group_expr(group_expr, pattern),
            TheoryExpression::Ring(ring_expr) => find_subexpr_in_ring_expr(ring_expr, pattern),
            TheoryExpression::Field(field_expr) => find_subexpr_in_field_expr(field_expr, pattern),
        },
        MathExpression::Relation(relation) => {
            // Check if the pattern is within this relation
            find_subexpr_in_relation_expr(relation, pattern)
        }
        // These expression types don't contain subexpressions
        MathExpression::Var(_) | MathExpression::Number(_) | MathExpression::Object(_) => None,
    }
}

/// Helper function to find a subexpression within a group expression
fn find_subexpr_in_group_expr(
    expr: &GroupExpression,
    pattern: &MathExpression,
) -> Option<Vec<usize>> {
    use crate::subjects::math::theories::groups::definitions::GroupExpression;

    match expr {
        GroupExpression::Operation { left, right, .. } => {
            // Check left operand
            if let Some(mut path) = find_subexpr_in_parametrizable(left, pattern) {
                path.insert(0, 0); // Add index for left
                return Some(path);
            }

            // Check right operand
            if let Some(mut path) = find_subexpr_in_parametrizable(right, pattern) {
                path.insert(0, 1); // Add index for right
                return Some(path);
            }

            None
        }
        GroupExpression::Inverse { element, .. } => {
            if let Some(mut path) = find_subexpr_in_parametrizable(element, pattern) {
                path.insert(0, 0);
                return Some(path);
            }
            None
        }
        GroupExpression::Power { base, .. } => {
            if let Some(mut path) = find_subexpr_in_parametrizable(base, pattern) {
                path.insert(0, 0);
                return Some(path);
            }
            None
        }
        GroupExpression::Commutator { a, b, .. } => {
            // Check the 'a' operand
            if let Some(mut path) = find_subexpr_in_parametrizable(a, pattern) {
                path.insert(0, 0);
                return Some(path);
            }

            // Check the 'b' operand
            if let Some(mut path) = find_subexpr_in_parametrizable(b, pattern) {
                path.insert(0, 1);
                return Some(path);
            }

            None
        }
        // Other group expression types may need similar handling
        // depending on their structure
        _ => None,
    }
}

/// Helper function to find a subexpression within a parametrizable item
fn find_subexpr_in_parametrizable<T>(
    param: &Parametrizable<T>,
    pattern: &MathExpression,
) -> Option<Vec<usize>>
where
    T: Clone + PartialEq,
{
    use crate::subjects::math::formalism::extract::Parametrizable;

    match param {
        Parametrizable::Concrete(_) => {
            // Convert to MathExpression and compare directly
            // This is a simplified approach - a complete implementation would
            // need proper conversion from concrete type to MathExpression
            None
        }
        Parametrizable::Variable(id) => {
            // Check if the variable matches our pattern
            if &MathExpression::Var(id.clone()) == pattern {
                return Some(vec![]);
            }
            None
        }
    }
}

/// Helper function to find a subexpression within a ring expression
fn find_subexpr_in_ring_expr(
    expr: &RingExpression,
    pattern: &MathExpression,
) -> Option<Vec<usize>> {
    use crate::subjects::math::theories::rings::definitions::RingExpression;

    match expr {
        RingExpression::Addition { left, right, .. } => {
            // Check left operand
            if let Some(mut path) =
                find_subexpr_in_expr(&convert_ring_expr_to_math_expr(left), pattern)
            {
                path.insert(0, 0);
                return Some(path);
            }

            // Check right operand
            if let Some(mut path) =
                find_subexpr_in_expr(&convert_ring_expr_to_math_expr(right), pattern)
            {
                path.insert(0, 1);
                return Some(path);
            }

            None
        }
        RingExpression::Multiplication { left, right, .. } => {
            // Check left operand
            if let Some(mut path) =
                find_subexpr_in_expr(&convert_ring_expr_to_math_expr(left), pattern)
            {
                path.insert(0, 0);
                return Some(path);
            }

            // Check right operand
            if let Some(mut path) =
                find_subexpr_in_expr(&convert_ring_expr_to_math_expr(right), pattern)
            {
                path.insert(0, 1);
                return Some(path);
            }

            None
        }
        RingExpression::AdditiveInverse { element, .. } => {
            if let Some(mut path) =
                find_subexpr_in_expr(&convert_ring_expr_to_math_expr(element), pattern)
            {
                path.insert(0, 0);
                return Some(path);
            }
            None
        }
        // Handle other ring expression types as needed
        _ => None,
    }
}

/// Helper function to find a subexpression within a field expression
fn find_subexpr_in_field_expr(
    expr: &FieldExpression,
    pattern: &MathExpression,
) -> Option<Vec<usize>> {
    use crate::subjects::math::theories::rings::definitions::FieldExpression;

    match expr {
        FieldExpression::Addition { left, right, .. } => {
            // Check left operand
            if let Some(mut path) =
                find_subexpr_in_expr(&convert_field_expr_to_math_expr(left), pattern)
            {
                path.insert(0, 0);
                return Some(path);
            }

            // Check right operand
            if let Some(mut path) =
                find_subexpr_in_expr(&convert_field_expr_to_math_expr(right), pattern)
            {
                path.insert(0, 1);
                return Some(path);
            }

            None
        }
        FieldExpression::Multiplication { left, right, .. } => {
            // Check left operand
            if let Some(mut path) =
                find_subexpr_in_expr(&convert_field_expr_to_math_expr(left), pattern)
            {
                path.insert(0, 0);
                return Some(path);
            }

            // Check right operand
            if let Some(mut path) =
                find_subexpr_in_expr(&convert_field_expr_to_math_expr(right), pattern)
            {
                path.insert(0, 1);
                return Some(path);
            }

            None
        }
        FieldExpression::Division {
            numerator,
            denominator,
            ..
        } => {
            // Check numerator
            if let Some(mut path) =
                find_subexpr_in_expr(&convert_field_expr_to_math_expr(numerator), pattern)
            {
                path.insert(0, 0);
                return Some(path);
            }

            // Check denominator
            if let Some(mut path) =
                find_subexpr_in_expr(&convert_field_expr_to_math_expr(denominator), pattern)
            {
                path.insert(0, 1);
                return Some(path);
            }

            None
        }
        // Handle other field expression types as needed
        _ => None,
    }
}

/// Helper function to find a subexpression within a relation
fn find_subexpr_in_relation_expr(
    relation: &MathRelation,
    pattern: &MathExpression,
) -> Option<Vec<usize>> {
    use crate::subjects::math::formalism::relations::MathRelation;

    match relation {
        MathRelation::Equal { left, right, .. } => {
            // Check left side
            if let Some(mut path) = find_subexpr_in_expr(left, pattern) {
                path.insert(0, 0);
                return Some(path);
            }

            // Check right side
            if let Some(mut path) = find_subexpr_in_expr(right, pattern) {
                path.insert(0, 1);
                return Some(path);
            }

            None
        }
        MathRelation::Implies(ante, cons) => {
            // Check antecedent (first box)
            if let Some(mut path) = find_subexpr_in_relation_expr(ante, pattern) {
                path.insert(0, 0);
                return Some(path);
            }

            // Check consequent (second box)
            if let Some(mut path) = find_subexpr_in_relation_expr(cons, pattern) {
                path.insert(0, 1);
                return Some(path);
            }

            None
        }
        // Handle other relation types
        _ => None,
    }
}

/// Helper utility to convert a ring expression to a math expression
fn convert_ring_expr_to_math_expr(expr: &RingExpression) -> MathExpression {
    use crate::subjects::math::formalism::expressions::TheoryExpression;
    MathExpression::Expression(TheoryExpression::Ring(expr.clone()))
}

/// Helper utility to convert a field expression to a math expression
fn convert_field_expr_to_math_expr(expr: &FieldExpression) -> MathExpression {
    use crate::subjects::math::formalism::expressions::TheoryExpression;
    MathExpression::Expression(TheoryExpression::Field(expr.clone()))
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
        MathExpression::Expression(theory_expr) => {
            let new_theory_expr = match theory_expr {
                TheoryExpression::Group(group_expr) => TheoryExpression::Group(
                    replace_subexpr_in_group_expr(group_expr, &path[1..], replacement),
                ),
                TheoryExpression::Ring(ring_expr) => TheoryExpression::Ring(
                    replace_subexpr_in_ring_expr(ring_expr, &path[1..], replacement),
                ),
                TheoryExpression::Field(field_expr) => TheoryExpression::Field(
                    replace_subexpr_in_field_expr(field_expr, &path[1..], replacement),
                ),
            };
            MathExpression::Expression(new_theory_expr)
        }
        MathExpression::Relation(relation) => MathExpression::Relation(Box::new(
            replace_subexpr_in_relation(relation, expr, path, replacement),
        )),
        // For these types, no replacement is possible
        _ => expr.clone(),
    }
}

/// Helper function to replace a subexpression in a group expression
fn replace_subexpr_in_group_expr(
    expr: &GroupExpression,
    path: &[usize],
    replacement: &MathExpression,
) -> GroupExpression {
    use crate::subjects::math::theories::groups::definitions::GroupExpression;

    if path.is_empty() {
        // Try to convert replacement to GroupExpression
        // This is a simplified approach - a complete implementation would
        // need proper conversion from MathExpression to GroupExpression
        return expr.clone();
    }

    match expr {
        GroupExpression::Operation { group, left, right } => {
            if path[0] == 0 {
                // Replace in left operand
                GroupExpression::Operation {
                    group: group.clone(),
                    left: Box::new(replace_subexpr_in_parametrizable(
                        left,
                        &path[1..],
                        replacement,
                    )),
                    right: right.clone(),
                }
            } else if path[0] == 1 {
                // Replace in right operand
                GroupExpression::Operation {
                    group: group.clone(),
                    left: left.clone(),
                    right: Box::new(replace_subexpr_in_parametrizable(
                        right,
                        &path[1..],
                        replacement,
                    )),
                }
            } else {
                expr.clone()
            }
        }
        // Handle other group expression types
        _ => expr.clone(),
    }
}

/// Helper function to replace a subexpression in a parametrizable item
fn replace_subexpr_in_parametrizable<T>(
    param: &Parametrizable<T>,
    path: &[usize],
    replacement: &MathExpression,
) -> Parametrizable<T>
where
    T: Clone + PartialEq,
{
    use crate::subjects::math::formalism::extract::Parametrizable;

    if path.is_empty() {
        // Try to convert replacement to Parametrizable<T>
        // This is a simplified approach - a complete implementation would
        // need proper conversion from MathExpression to Parametrizable<T>
        return param.clone();
    }

    match param {
        // For concrete values and variables, no replacement is possible in subexpressions
        _ => param.clone(),
    }
}

/// Helper function to replace a subexpression in a ring expression
fn replace_subexpr_in_ring_expr(
    expr: &RingExpression,
    path: &[usize],
    replacement: &MathExpression,
) -> RingExpression {
    use crate::subjects::math::theories::rings::definitions::RingExpression;

    if path.is_empty() {
        // Try to convert replacement to RingExpression
        // This is a simplified approach - a complete implementation would
        // need proper conversion from MathExpression to RingExpression
        return expr.clone();
    }

    // Handle ring expression types as needed
    expr.clone()
}

/// Helper function to replace a subexpression in a field expression
fn replace_subexpr_in_field_expr(
    expr: &FieldExpression,
    path: &[usize],
    replacement: &MathExpression,
) -> FieldExpression {
    use crate::subjects::math::theories::rings::definitions::FieldExpression;

    if path.is_empty() {
        // Try to convert replacement to FieldExpression
        // This is a simplified approach - a complete implementation would
        // need proper conversion from MathExpression to FieldExpression
        return expr.clone();
    }

    // Handle field expression types as needed
    expr.clone()
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

/// Simple variable substitution in a relation - no recursion
fn apply_simple_substitution(
    relation: &MathRelation,
    substitutions: &HashMap<Identifier, MathExpression>,
) -> MathRelation {
    match relation {
        MathRelation::Equal { meta, left, right } => {
            let new_left = substitute_in_expression(left, substitutions);
            let new_right = substitute_in_expression(right, substitutions);
            MathRelation::Equal {
                meta: meta.clone(),
                left: new_left,
                right: new_right,
            }
        }
        MathRelation::Implies(premise, conclusion) => {
            let new_premise = apply_simple_substitution(premise, substitutions);
            let new_conclusion = apply_simple_substitution(conclusion, substitutions);
            MathRelation::Implies(Box::new(new_premise), Box::new(new_conclusion))
        }
        MathRelation::And(relations) => {
            let new_relations = relations
                .iter()
                .map(|r| apply_simple_substitution(r, substitutions))
                .collect();
            MathRelation::And(new_relations)
        }
        // For other relation types, just return them unchanged for now
        _ => relation.clone(),
    }
}

/// Simple variable substitution in an expression - no recursion
fn substitute_in_expression(
    expr: &MathExpression,
    substitutions: &HashMap<Identifier, MathExpression>,
) -> MathExpression {
    match expr {
        MathExpression::Var(id) => {
            // Check if this variable should be substituted
            substitutions
                .get(id)
                .cloned()
                .unwrap_or_else(|| expr.clone())
        }
        // For other expression types, return unchanged for now
        _ => expr.clone(),
    }
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
    /// Calculate the depth of this node from the root of the proof tree
    fn calculate_depth(&self, forest: &ProofForest) -> usize {
        let mut depth = 0;
        let mut current_node_id = &self.id;
        let mut visited = std::collections::HashSet::new();

        // Traverse up the tree to count depth, with cycle detection
        while let Some(node) = forest.nodes.get(current_node_id) {
            // Prevent cycles
            if !visited.insert(current_node_id.clone()) {
                break;
            }

            if let Some(parent_id) = &node.parent {
                depth += 1;
                current_node_id = parent_id;

                // Safety limit to prevent excessive traversal
                if depth > 20 {
                    break;
                }
            } else {
                break; // Reached root
            }
        }

        depth
    }

    /// Apply a tactic and return a new node (only way to apply tactics)
    pub fn apply_tactic(&self, tactic: Tactic, forest: &mut ProofForest) -> ProofNode {
        use tactics::{TacticApplicationResult, TacticApplier};

        // Use the clean trait-based approach
        let result = tactic.apply_to_goal(&self.state);

        match result {
            TacticApplicationResult::SingleGoal(new_state) => {
                let new_node_id = Uuid::new_v4().to_string();
                let new_node = ProofNode {
                    id: new_node_id.clone(),
                    parent: Some(self.id.clone()),
                    children: vec![],
                    state: new_state,
                    tactic: Some(tactic.clone()),
                    status: ProofStatus::InProgress,
                };
                forest.add_node(new_node.clone());
                if let Some(parent) = forest.nodes.get_mut(&self.id) {
                    parent.children.push(new_node_id);
                }
                new_node
            }
            TacticApplicationResult::MultipleGoals(goals) => {
                // Create multiple child nodes for tactics like CaseAnalysis, Decompose, Induction
                let child_nodes: Vec<ProofNode> = goals
                    .into_iter()
                    .map(|goal_state| {
                        let child_id = Uuid::new_v4().to_string();
                        ProofNode {
                            id: child_id,
                            parent: Some(self.id.clone()),
                            children: vec![],
                            state: goal_state,
                            tactic: Some(tactic.clone()),
                            status: ProofStatus::InProgress,
                        }
                    })
                    .collect();

                // Add all child nodes to forest and link them
                let child_ids: Vec<String> = child_nodes.iter().map(|n| n.id.clone()).collect();
                for child in child_nodes {
                    forest.add_node(child);
                }

                // Update parent to reference children
                if let Some(parent) = forest.nodes.get_mut(&self.id) {
                    parent.children.extend(child_ids.clone());
                }

                // Return the first child node (convention for multi-goal tactics)
                forest.nodes.get(&child_ids[0]).unwrap().clone()
            }
            TacticApplicationResult::NoChange => {
                // Tactic wasn't applicable, return original node
                self.clone()
            }
            TacticApplicationResult::Error(error_msg) => {
                // Create an error node
                let error_node_id = Uuid::new_v4().to_string();
                let error_node = ProofNode {
                    id: error_node_id.clone(),
                    parent: Some(self.id.clone()),
                    children: vec![],
                    state: ProofGoal {
                        quantifier: self.state.quantifier.clone(),
                        value_variables: self.state.value_variables.clone(),
                        statement: MathRelation::Todo {
                            name: format!("Error: {}", error_msg),
                            expressions: vec![],
                        },
                    },
                    tactic: Some(tactic),
                    status: ProofStatus::Abandoned,
                };
                forest.add_node(error_node.clone());
                if let Some(parent) = forest.nodes.get_mut(&self.id) {
                    parent.children.push(error_node_id);
                }
                error_node
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

    /// Mark this proof branch as complete is it completes
    pub fn should_complete(self, forest: &mut ProofForest) -> Self {
        // TODO: assert the proof node is complete
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

    /// Helper method to extract a reusable expression from a theorem and introduce it
    pub fn tactics_intro_theorem_result(
        &self,
        description: &str,
        theorem_id: &str,
        instantiation: HashMap<Identifier, MathExpression>,
        forest: &mut ProofForest,
    ) -> ProofNode {
        // Simple, direct theorem application without recursion
        let theorem_result_expr = {
            let registry = GLOBAL_THEOREM_REGISTRY.lock().unwrap();
            if let Some(theorem) = registry.get_theorem(theorem_id) {
                // Extract the conclusion from the theorem statement
                let conclusion = match &theorem.goal.statement {
                    MathRelation::Implies(_, conclusion) => conclusion.as_ref().clone(),
                    other_relation => other_relation.clone(),
                };

                // Apply simple variable substitution to the conclusion
                let instantiated_conclusion =
                    apply_simple_substitution(&conclusion, &instantiation);
                MathExpression::Relation(Box::new(instantiated_conclusion))
            } else {
                // Theorem not found - create an error expression
                let error_relation = MathRelation::Todo {
                    name: format!("TheoremNotFound:{}", theorem_id),
                    expressions: vec![],
                };
                MathExpression::Relation(Box::new(error_relation))
            }
        };

        // Create new proof goal statement from the theorem result
        let new_proof_goal_statement = match &theorem_result_expr {
            MathExpression::Relation(rel_box) => rel_box.as_ref().clone(),
            _ => MathRelation::Todo {
                name: "InvalidTheoremResultType".to_string(),
                expressions: vec![theorem_result_expr.clone()],
            },
        };

        // Create the new node with the theorem application result
        let new_node = ProofNode {
            id: Uuid::new_v4().to_string(),
            parent: Some(self.id.clone()),
            children: Vec::new(),
            state: ProofGoal {
                quantifier: self.state.quantifier.clone(),
                value_variables: self.state.value_variables.clone(),
                statement: new_proof_goal_statement,
            },
            tactic: Some(Tactic::TheoremApplication {
                theorem_id: theorem_id.to_string(),
                instantiation,
                target_expr: None,
            }),
            status: ProofStatus::InProgress,
        };

        // Add to forest and update parent-child relationships
        forest.add_node(new_node.clone());
        if let Some(current_node_mut) = forest.nodes.get_mut(&self.id) {
            current_node_mut.children.push(new_node.id.clone());
        }

        new_node
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
        let init_tactic = tactics::Tactic::Intro {
            name: Identifier::Name("init".to_string(), 0),
            expression: MathExpression::Var(Identifier::Name(format!("theorem_{}", theorem.id), 0)),
            view: None,
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
