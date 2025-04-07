// Module: src/formalize_v2/subjects/math/theorem/proof.rs
// Implements a rich proof structure for mathematical theorems with branching support

use js_sys;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use super::core::{MathObjectType, ProofState, Theorem, ValueBindedVariable};
use super::expressions::{MathExpression, Variable};
use super::relations::MathRelation;

/// Node ID type
pub type NodeId = u64;

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

/// A tactic that can be applied to a proof state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Tactic {
    /// Introduce a new variable (string-based legacy version)
    Intro(String, usize),

    /// Introduce a new variable (syntax tree-based version)
    IntroExpr {
        /// Name of the variable
        name: String,
        /// Type of the variable
        var_type: MathObjectType,
        /// Expression associated with the variable
        expression: MathExpression,
        /// Sequence number for ordering
        sequence: usize,
    },

    /// Substitute an expression (string-based legacy version)
    Substitution(String, usize),

    /// Substitute an expression (syntax tree-based version)
    SubstitutionExpr {
        /// The pattern to match in the current expression
        pattern: MathExpression,
        /// The replacement expression
        replacement: MathExpression,
        /// Location/path to apply the substitution (if specific)
        location: Option<Vec<usize>>,
        /// Sequence number for ordering
        sequence: usize,
    },

    /// Apply a theorem (string-based legacy version)
    TheoremApplication(String, HashMap<String, MathExpression>),

    /// Apply a theorem (syntax tree-based version)
    TheoremApplicationExpr {
        /// ID of the theorem to apply
        theorem_id: String,
        /// Mapping of theorem variables to expressions
        instantiation: HashMap<String, MathExpression>,
        /// The specific expression to which the theorem is applied
        target_expr: Option<MathExpression>,
    },

    /// Case analysis
    CaseAnalysis {
        /// The target of case analysis
        target: String,
        /// The cases to consider
        cases: Vec<String>,
    },

    /// Case analysis (syntax tree-based version)
    CaseAnalysisExpr {
        /// The expression to analyze by cases
        target_expr: MathExpression,
        /// The expressions representing each case
        case_exprs: Vec<MathExpression>,
        /// Names for the cases (for display)
        case_names: Vec<String>,
    },

    /// Rewrite an expression using an equation
    Rewrite {
        /// The target expression (as string - legacy)
        target: String,
        /// The equation to use (as string - legacy)
        equation: String,
        /// Direction of the rewrite
        direction: RewriteDirection,
    },

    /// Rewrite an expression using an equation (syntax tree-based version)
    RewriteExpr {
        /// The target expression to rewrite
        target_expr: MathExpression,
        /// The equation to use for rewriting
        equation_expr: MathExpression,
        /// Direction of the rewrite
        direction: RewriteDirection,
        /// Specific location to apply the rewrite (if any)
        location: Option<Vec<usize>>,
    },

    /// Simplify an expression
    Simplify(String),

    /// Decompose a complex expression
    Decompose {
        /// The expression to decompose
        target: String,
        /// The method of decomposition
        method: DecompositionMethod,
    },

    /// Apply induction on a variable
    Induction {
        /// The variable to apply induction on
        variable: String,
        /// The type of induction to apply
        induction_type: InductionType,
    },

    /// Custom tactic for specialized domains
    Custom {
        /// Name of the custom tactic
        name: String,
        /// Arguments to the tactic
        args: Vec<String>,
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

/// Registry of theorems that can be applied during proofs
#[derive(Debug, Clone)]
pub struct TheoremRegistry {
    /// Map of theorem IDs to Theorem objects
    theorems: HashMap<String, Theorem>,
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
        self.theorems.insert(theorem.id.clone(), theorem);
    }

    /// Get a theorem by ID
    pub fn get_theorem(&self, id: &str) -> Option<&Theorem> {
        self.theorems.get(id)
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

impl ProofState {
    /// Apply a transformation to the current statement
    pub fn transform_statement(
        &self,
        transformer: impl Fn(&MathRelation) -> MathRelation,
    ) -> ProofState {
        let mut new_state = self.clone();
        new_state.statement = transformer(&self.statement);

        // Also update the path if present
        if let Some(path) = &new_state.path {
            let new_path = if path.contains('_') {
                // Increment the last part of the path
                let parts: Vec<&str> = path.rsplitn(2, '_').collect();
                if parts.len() == 2 {
                    if let Ok(num) = parts[0].parse::<usize>() {
                        format!("{}_{}", parts[1], num + 1)
                    } else {
                        format!("{}_1", path)
                    }
                } else {
                    format!("{}_1", path)
                }
            } else {
                format!("{}_1", path)
            };
            new_state.path = Some(new_path);
        }

        new_state
    }

    /// Add a variable to the state
    pub fn add_variable(
        &self,
        var_name: &str,
        var_type: MathObjectType,
        expr: MathExpression,
    ) -> ProofState {
        let mut new_state = self.clone();

        // Add the variable to the environment
        // In a real implementation, we would update a proper environment
        // For now, we just add a new ValueBindedVariable
        let var_binding = ValueBindedVariable {
            variable: var_name.to_string(),
            value: expr,
            math_type: var_type.clone(),
        };
        new_state.value_variables.push(var_binding);

        // Add a justification
        new_state.justification = Some(format!(
            "Introduced variable '{}' of type {:?}",
            var_name, var_type
        ));

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
        MathExpression::Operation(_) => {
            // In a real implementation, we would iterate through operands
            // For demonstration, just return None
            None
        }
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
    if path.is_empty() {
        return replacement.clone();
    }

    match expr {
        MathExpression::Operation(_) => {
            // This is a simplified version for demonstration
            // In a real implementation, we'd need to handle the operation's structure correctly
            // Just return the original expression for now
            expr.clone()
        }
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
    /// Apply this tactic to a proof state, yielding a new state
    pub fn apply(&self, state: &ProofState) -> ProofState {
        match self {
            Tactic::Intro(var, _) => {
                // Legacy string-based intro - simplified
                let mut new_state = state.clone();

                // Create a ValueBindedVariable
                let var_binding = ValueBindedVariable {
                    variable: var.clone(),
                    value: MathExpression::Var(Variable::E(100)), // Placeholder value
                    math_type: MathObjectType::Real,              // Default type
                };

                new_state.value_variables.push(var_binding);
                new_state.justification = Some(format!("Introduced variable '{}'", var));
                new_state
            }

            Tactic::IntroExpr {
                name,
                var_type,
                expression,
                sequence: _,
            } => {
                // Syntax tree-based intro
                let mut new_state = state.clone();

                // Create a ValueBindedVariable
                let var_binding = ValueBindedVariable {
                    variable: name.clone(),
                    value: expression.clone(),
                    math_type: var_type.clone(),
                };

                new_state.value_variables.push(var_binding);
                new_state.justification = Some(format!(
                    "Introduced variable '{}' of type {:?}",
                    name, var_type
                ));
                new_state
            }

            Tactic::Substitution(expr, _) => {
                // Legacy string-based substitution - simplified
                let mut new_state = state.clone();
                new_state.justification = Some(format!("Substituted with expression '{}'", expr));
                new_state
            }

            Tactic::SubstitutionExpr {
                pattern,
                replacement,
                location,
                sequence: _,
            } => {
                // Syntax tree-based substitution
                if let Some((expr_to_replace, path)) =
                    state.find_subexpression(pattern, location.clone())
                {
                    // Found the pattern, replace it
                    state.transform_statement(|rel| {
                        replace_subexpr_in_relation(rel, &expr_to_replace, &path, replacement)
                    })
                } else {
                    // Pattern not found, return the state unchanged
                    let mut new_state = state.clone();
                    new_state.justification =
                        Some(format!("Substitution pattern not found: {:?}", pattern));
                    new_state
                }
            }

            Tactic::TheoremApplication(theorem_id, instantiation) => {
                // Legacy string-based theorem application - simplified
                let mut new_state = state.clone();

                // Convert string instantiations to expressions for demonstration
                // In a real implementation, we would parse the strings properly
                let expr_instantiation: HashMap<String, MathExpression> = instantiation
                    .iter()
                    .map(|(k, v)| {
                        // Just use the provided expression directly since it's already a MathExpression
                        (k.clone(), v.clone())
                    })
                    .collect();

                new_state.justification = Some(format!("Applied theorem: {}", theorem_id));

                // Create registry and apply theorem
                let registry = TheoremRegistry::new();
                if let Some(new_relation) =
                    registry.apply_theorem(theorem_id, &state.statement, &expr_instantiation, None)
                {
                    new_state.statement = new_relation;
                }

                new_state
            }

            Tactic::TheoremApplicationExpr {
                theorem_id,
                instantiation,
                target_expr,
            } => {
                // Create a theorem registry
                let registry = TheoremRegistry::new();

                // Try to apply the theorem
                if let Some(target) = target_expr {
                    if let Some(new_relation) = registry.apply_theorem(
                        theorem_id,
                        &state.statement,
                        instantiation,
                        Some(target.clone()),
                    ) {
                        // Successfully applied the theorem
                        let mut new_state = state.clone();
                        new_state.statement = new_relation;
                        new_state.justification = Some(format!(
                            "Applied theorem '{}' with {} instantiations",
                            theorem_id,
                            instantiation.len()
                        ));
                        new_state
                    } else {
                        // Failed to apply the theorem
                        let mut new_state = state.clone();
                        new_state.justification = Some(format!(
                            "Could not apply theorem '{}' to the target expression",
                            theorem_id
                        ));
                        new_state
                    }
                } else {
                    // No target expression specified, apply globally
                    if let Some(new_relation) =
                        registry.apply_theorem(theorem_id, &state.statement, instantiation, None)
                    {
                        // Successfully applied the theorem
                        let mut new_state = state.clone();
                        new_state.statement = new_relation;
                        new_state.justification = Some(format!(
                            "Applied theorem '{}' with {} instantiations",
                            theorem_id,
                            instantiation.len()
                        ));
                        new_state
                    } else {
                        // Failed to apply the theorem
                        let mut new_state = state.clone();
                        new_state.justification =
                            Some(format!("Could not apply theorem '{}'", theorem_id));
                        new_state
                    }
                }
            }

            Tactic::Rewrite {
                target,
                equation,
                direction,
            } => {
                // Legacy string-based rewrite - simplified
                let mut new_state = state.clone();

                let dir_str = match direction {
                    RewriteDirection::LeftToRight => "left-to-right",
                    RewriteDirection::RightToLeft => "right-to-left",
                };

                new_state.justification = Some(format!(
                    "Rewrote '{}' using '{}' ({})",
                    target, equation, dir_str
                ));

                // For legacy rewrite, we create a string to expression conversion
                let target_expr = MathExpression::string_expr(target);
                let equation_expr = MathExpression::string_expr(equation);

                // Apply the actual rewrite if we can find the target
                if let Some((expr_to_rewrite, path)) = state.find_subexpression(&target_expr, None)
                {
                    return state.transform_statement(|rel| {
                        apply_rewrite(rel, &expr_to_rewrite, &path, &equation_expr, direction)
                    });
                }

                new_state
            }

            Tactic::RewriteExpr {
                target_expr,
                equation_expr,
                direction,
                location,
            } => {
                // Syntax tree-based rewrite
                if let Some((expr_to_rewrite, path)) =
                    state.find_subexpression(target_expr, location.clone())
                {
                    // Found the target, rewrite it using the equation
                    state.transform_statement(|rel| {
                        apply_rewrite(rel, &expr_to_rewrite, &path, equation_expr, direction)
                    })
                } else {
                    // Target not found
                    let mut new_state = state.clone();
                    new_state.justification =
                        Some(format!("Rewrite target not found: {:?}", target_expr));
                    new_state
                }
            }

            Tactic::CaseAnalysis { target, cases } => {
                // Legacy string-based case analysis - simplified
                let mut new_state = state.clone();
                new_state.justification = Some(format!(
                    "Case analysis on '{}' with {} cases",
                    target,
                    cases.len()
                ));
                new_state
            }

            Tactic::CaseAnalysisExpr {
                target_expr,
                case_exprs,
                case_names,
            } => {
                // Syntax tree-based case analysis
                // For case analysis, we don't transform the state directly -
                // the CaseAnalysisBuilder creates separate branches

                let mut new_state = state.clone();
                new_state.justification = Some(format!(
                    "Case analysis on expression {:?} with {} cases",
                    target_expr,
                    case_exprs.len()
                ));
                new_state
            }

            /// Simplify an expression
            Tactic::Simplify(expr) => {
                // Simplify an expression
                let mut new_state = state.clone();

                // Convert string to expression
                let target_expr = MathExpression::string_expr(expr);

                // Try to find the expression to simplify
                if let Some((expr_to_simplify, path)) = state.find_subexpression(&target_expr, None)
                {
                    // Apply simplification
                    return state.transform_statement(|rel| {
                        let simplified = simplify_expression(&expr_to_simplify);
                        replace_subexpr_in_relation(rel, &expr_to_simplify, &path, &simplified)
                    });
                }

                new_state.justification = Some(format!("Simplified expression: {}", expr));
                new_state
            }

            /// Decompose a complex expression
            Tactic::Decompose { target, method } => {
                // Decompose a complex expression
                let mut new_state = state.clone();

                // Convert string to expression
                let target_expr = MathExpression::string_expr(target);

                // Try to find the expression to decompose
                if let Some((expr_to_decompose, path)) =
                    state.find_subexpression(&target_expr, None)
                {
                    // Apply decomposition
                    return state.transform_statement(|rel| {
                        let decomposed = decompose_expression(&expr_to_decompose, method);
                        replace_subexpr_in_relation(rel, &expr_to_decompose, &path, &decomposed)
                    });
                }

                let method_str = match method {
                    DecompositionMethod::Components => "components",
                    DecompositionMethod::Factor => "factoring",
                    DecompositionMethod::Expand => "expansion",
                    DecompositionMethod::Other(s) => s,
                };

                new_state.justification = Some(format!(
                    "Decomposed expression '{}' using {} method",
                    target, method_str
                ));
                new_state
            }

            /// Apply induction on a variable
            Tactic::Induction {
                variable,
                induction_type,
            } => {
                // Apply induction
                let mut new_state = state.clone();

                let type_str = match induction_type {
                    InductionType::Natural => "mathematical",
                    InductionType::Structural => "structural",
                    InductionType::Transfinite => "transfinite",
                    InductionType::WellFounded => "well-founded",
                    InductionType::Other(s) => s,
                };

                new_state.justification = Some(format!(
                    "Applied {} induction on variable '{}'",
                    type_str, variable
                ));

                // In a full implementation, we would:
                // 1. Set up the base case
                // 2. Set up the inductive step
                // 3. Create branches for each

                new_state
            }

            /// Custom tactic for specialized domains
            Tactic::Custom { name, args } => {
                // Custom tactic
                let mut new_state = state.clone();
                new_state.justification = Some(format!(
                    "Applied custom tactic '{}' with arguments: {:?}",
                    name, args
                ));
                new_state
            }
        }
    }

    /// Describe the tactic in plain text
    pub fn describe(&self) -> String {
        match self {
            Tactic::Intro(var, seq) => format!("Intro(\"{}\", {})", var, seq),

            Tactic::IntroExpr {
                name,
                var_type: _,
                expression: _,
                sequence,
            } => {
                format!("IntroExpr(\"{}\", {}) with syntax tree", name, sequence)
            }

            Tactic::Substitution(expr, seq) => format!("Substitution(\"{}\", {})", expr, seq),

            Tactic::SubstitutionExpr {
                pattern: _,
                replacement: _,
                location: _,
                sequence,
            } => {
                format!("SubstitutionExpr({}) with syntax tree", sequence)
            }

            Tactic::TheoremApplication(theorem_id, _) => {
                format!("TheoremApplication(\"{}\")", theorem_id)
            }

            Tactic::TheoremApplicationExpr {
                theorem_id,
                instantiation: _,
                target_expr: _,
            } => {
                format!(
                    "TheoremApplicationExpr(\"{}\") with syntax tree",
                    theorem_id
                )
            }

            Tactic::Rewrite {
                target,
                equation,
                direction,
            } => {
                let dir_str = match direction {
                    RewriteDirection::LeftToRight => "→",
                    RewriteDirection::RightToLeft => "←",
                };
                format!(
                    "Rewrite {{ target: \"{}\", equation: \"{}\", direction: {} }}",
                    target, equation, dir_str
                )
            }

            Tactic::RewriteExpr {
                target_expr: _,
                equation_expr: _,
                direction,
                location: _,
            } => {
                let dir_str = match direction {
                    RewriteDirection::LeftToRight => "→",
                    RewriteDirection::RightToLeft => "←",
                };
                format!("RewriteExpr {{ direction: {} }} with syntax tree", dir_str)
            }

            Tactic::CaseAnalysis { target, cases } => {
                format!(
                    "CaseAnalysis {{ target: \"{}\", cases: {} }}",
                    target,
                    cases.len()
                )
            }

            Tactic::CaseAnalysisExpr {
                target_expr: _,
                case_exprs,
                case_names: _,
            } => {
                format!(
                    "CaseAnalysisExpr {{ cases: {} }} with syntax tree",
                    case_exprs.len()
                )
            }

            // Handle other variants
            _ => format!("{:?}", self),
        }
    }
}

/// Helper function to apply a rewrite using an equation
fn apply_rewrite(
    relation: &MathRelation,
    expr_to_rewrite: &MathExpression,
    path: &[usize],
    equation_expr: &MathExpression,
    direction: &RewriteDirection,
) -> MathRelation {
    // Actual implementation would extract left and right sides from equation_expr
    // For now, we'll just replace with equation_expr itself as placeholder
    let replacement = equation_expr.clone();
    replace_subexpr_in_relation(relation, expr_to_rewrite, path, &replacement)
}

/// Helper function to simplify an expression
fn simplify_expression(expr: &MathExpression) -> MathExpression {
    // In a real implementation, this would apply algebraic simplifications,
    // numeric evaluations, etc.

    // For now, just return the expression unchanged
    expr.clone()
}

/// Helper function to decompose an expression based on method
fn decompose_expression(expr: &MathExpression, method: &DecompositionMethod) -> MathExpression {
    // In a real implementation, this would apply different decomposition techniques
    // based on the method parameter

    match method {
        DecompositionMethod::Components => {
            // Break into component parts
            expr.clone()
        }
        DecompositionMethod::Factor => {
            // Factor expression
            expr.clone()
        }
        DecompositionMethod::Expand => {
            // Expand expression
            expr.clone()
        }
        DecompositionMethod::Other(_) => {
            // Custom method
            expr.clone()
        }
    }
}

/// A step in a proof tree
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofStep {
    /// Unique identifier for this step
    pub id: usize,

    /// The resulting state after applying the tactic
    pub state: ProofState,

    /// The tactic applied to reach this state
    pub tactic: Option<Tactic>,

    /// Parent step id
    pub parent_id: Option<usize>,

    /// Child branch steps
    pub branches: Vec<usize>,

    /// Status of this proof step
    pub status: ProofStatus,

    /// Human-readable description of this step
    pub description: Option<String>,

    /// Timestamp when this step was created (milliseconds since epoch)
    pub created_at: u64,

    /// Tags for organization
    pub tags: Vec<String>,
}

/// A node in the proof exploration tree
#[derive(Debug, Clone, PartialEq)]
pub struct ProofNode {
    /// Unique identifier for this node
    pub id: NodeId,
    /// Parent node ID, if any
    pub parent: Option<NodeId>,
    /// Child node IDs
    pub children: Vec<NodeId>,
    /// The proof state at this node
    pub state: ProofState,
    /// The tactic applied to reach this state
    pub tactic: Option<Tactic>,
    /// User notes about this step
    pub note: String,
    /// Status of this proof branch
    pub status: ProofStatus,
    /// When this node was created
    pub timestamp: u64,
}

/// A forest of proof exploration nodes
#[derive(Debug)]
pub struct ProofForest {
    /// All nodes in the forest
    pub nodes: HashMap<NodeId, ProofNode>,
    /// Root node ID for each tree in the forest
    pub roots: Vec<NodeId>,
    /// Bookmarks for important states
    pub bookmarks: HashMap<String, NodeId>,
    /// Next available node ID
    next_id: NodeId,
}

impl ProofForest {
    /// Create a new proof forest
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            roots: Vec::new(),
            bookmarks: HashMap::new(),
            next_id: 0,
        }
    }

    /// Add a new node to the forest
    pub fn add_node(
        &mut self,
        parent: Option<NodeId>,
        state: ProofState,
        tactic: Option<Tactic>,
        note: String,
        status: ProofStatus,
    ) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;

        let node = ProofNode {
            id,
            parent,
            children: Vec::new(),
            state,
            tactic,
            note,
            status,
            timestamp: get_timestamp(),
        };

        // Update parent's children list if this node has a parent
        if let Some(parent_id) = parent {
            if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
                parent_node.children.push(id);
            }
        } else {
            // If there's no parent, this is a root node
            self.roots.push(id);
        }

        // Add the node to the forest
        self.nodes.insert(id, node);
        id
    }

    /// Add a bookmark to a node
    pub fn add_bookmark(&mut self, name: &str, node_id: NodeId) {
        self.bookmarks.insert(name.to_string(), node_id);
    }

    /// Get a bookmark by name
    pub fn get_bookmark(&self, name: &str) -> Option<&NodeId> {
        self.bookmarks.get(name)
    }

    /// Get a node by ID
    pub fn get_node(&self, id: NodeId) -> Option<&ProofNode> {
        self.nodes.get(&id)
    }

    /// Get a mutable reference to a node
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut ProofNode> {
        self.nodes.get_mut(&id)
    }

    /// Get the root node state
    pub fn get_root_state(&self) -> Option<&ProofState> {
        if let Some(root_id) = self.roots.first() {
            if let Some(root_node) = self.nodes.get(root_id) {
                return Some(&root_node.state);
            }
        }
        None
    }

    /// Generate a text representation of the proof forest
    pub fn visualize(&self) -> String {
        let mut result = String::new();
        result.push_str("Proof Forest:\n");

        for &root_id in &self.roots {
            self.visualize_tree(&mut result, root_id, 0);
        }

        // Add bookmarks
        if !self.bookmarks.is_empty() {
            result.push_str("\nBookmarks:\n");
            for (name, &node_id) in &self.bookmarks {
                result.push_str(&format!("  {} -> Node {}\n", name, node_id));
            }
        }

        result
    }

    /// Helper for visualizing a tree
    fn visualize_tree(&self, result: &mut String, node_id: NodeId, depth: usize) {
        if let Some(node) = self.nodes.get(&node_id) {
            let indent = "  ".repeat(depth);
            let status_str = match node.status {
                ProofStatus::Complete => "✓",
                ProofStatus::InProgress => "→",
                ProofStatus::Todo => "□",
                ProofStatus::Wip => "●",
                ProofStatus::Abandoned => "✗",
            };

            let tactic_str = match &node.tactic {
                Some(tactic) => format!(" [{:?}]", tactic),
                None => "".to_string(),
            };

            // Get path ID from the node's state
            let path_str = match &node.state.path {
                Some(path) => format!(" (path: {})", path),
                None => "".to_string(),
            };

            result.push_str(&format!(
                "{}{} Node {}{}{} - {}\n",
                indent, status_str, node_id, tactic_str, path_str, node.note
            ));

            for &child_id in &node.children {
                self.visualize_tree(result, child_id, depth + 1);
            }
        }
    }

    /// Get the path from root to a node
    pub fn get_path(&self, node_id: NodeId) -> Vec<NodeId> {
        let mut path = Vec::new();
        let mut current_id = node_id;

        while let Some(node) = self.nodes.get(&current_id) {
            path.push(current_id);
            if let Some(parent_id) = node.parent {
                current_id = parent_id;
            } else {
                break;
            }
        }

        path.reverse();
        path
    }
}

/// Builder for creating theorems with proofs
#[derive(Debug)]
pub struct TheoremBuilder {
    /// Name of the theorem being built
    pub name: String,
    /// Main relationship that the theorem establishes
    pub statement: MathRelation,
    /// Assumptions required for the theorem
    pub assumptions: Vec<MathRelation>,
    /// The proof forest containing all exploration paths
    pub proof_forest: Rc<RefCell<ProofForest>>,
}

impl TheoremBuilder {
    /// Create a new theorem builder
    pub fn new(name: &str, statement: MathRelation, assumptions: Vec<MathRelation>) -> Self {
        let proof_forest = Rc::new(RefCell::new(ProofForest::new()));

        // Create the initial proof state
        let initial_state = ProofState {
            quantifier: vec![],
            value_variables: vec![],
            statement: statement.clone(),
            path: Some("p0".to_string()),
            justification: None,
        };

        // Add the initial node to the forest
        proof_forest.borrow_mut().add_node(
            None,
            initial_state,
            None,
            format!("Initial state for theorem: {}", name),
            ProofStatus::InProgress,
        );

        TheoremBuilder {
            name: name.to_string(),
            statement,
            assumptions,
            proof_forest,
        }
    }

    /// Get the initial branch for starting a proof
    pub fn initial_branch(&self) -> ProofBranch {
        // Get the root node ID from the forest
        let forest = self.proof_forest.borrow();
        let root_id = match forest.roots.first() {
            Some(&id) => id,
            None => panic!("Proof forest has no root node"),
        };

        ProofBranch {
            node_id: root_id,
            forest: self.proof_forest.clone(),
            path_id: "p0".to_string(), // Initialize with p0
        }
    }

    /// Create a branch at a bookmarked point
    pub fn branch_at(&self, node_id: NodeId) -> ProofBranch {
        // For branching at bookmarks, we should examine the existing path structure
        let forest = self.proof_forest.borrow();
        let bookmark_path = format!("p{}", node_id); // Simple fallback

        ProofBranch {
            node_id,
            forest: self.proof_forest.clone(),
            path_id: bookmark_path,
        }
    }

    /// Build the final theorem
    pub fn build(&self) -> Theorem {
        // Find completed proofs in the forest
        let forest = self.proof_forest.borrow();
        let completed_nodes: Vec<_> = forest
            .nodes
            .iter()
            .filter(|(_id, node)| node.status == ProofStatus::Complete)
            .collect();

        // Create the theorem
        Theorem {
            id: format!("thm_{}", self.name.to_lowercase().replace(' ', "_")),
            name: self.name.clone(),
            description: format!("Theorem: {}", self.name),
            initial_proof_state: ProofState {
                quantifier: vec![],
                value_variables: vec![],
                statement: self.statement.clone(),
                path: Some("p0".to_string()),
                justification: None,
            },
        }
    }
}

/// A branch in a proof exploration
#[derive(Debug, Clone)]
pub struct ProofBranch {
    /// The ID of the current node in the forest
    pub node_id: NodeId,
    /// Reference to the proof forest
    pub forest: Rc<RefCell<ProofForest>>,
    /// Path identifier for structured naming (e.g., "p0_1_2")
    pub path_id: String,
}

/// Result of a case analysis containing all case branches
#[derive(Debug, Clone)]
pub struct CaseResult {
    /// The parent node that contains all cases
    pub parent_branch: ProofBranch,
    /// The individual case branches
    pub cases: Vec<ProofBranch>,
    /// The path identifier of the parent
    pub parent_path: String,
}

impl CaseResult {
    /// Get a specific case by index
    pub fn case(&self, index: usize) -> Option<&ProofBranch> {
        self.cases.get(index)
    }

    /// Get the number of cases
    pub fn case_count(&self) -> usize {
        self.cases.len()
    }

    /// Complete all cases
    pub fn complete_all_cases(&self) -> Self {
        let completed_cases: Vec<ProofBranch> = self
            .cases
            .iter()
            .map(|case| case.should_complete())
            .collect();

        CaseResult {
            parent_branch: self.parent_branch.clone(),
            cases: completed_cases,
            parent_path: self.parent_path.clone(),
        }
    }

    /// Mark the parent as complete when all cases are handled
    pub fn should_complete(&self) -> ProofBranch {
        self.parent_branch.should_complete()
    }
}

/// Builder for case analysis with ad hoc scoping
pub struct CaseAnalysisBuilder {
    /// The parent branch where this case analysis starts
    parent_branch: ProofBranch,
    /// The target variable or expression being analyzed
    target: Option<String>,
    /// The cases with their descriptions and branches
    cases: Vec<(String, ProofBranch)>,
}

impl CaseAnalysisBuilder {
    /// Create a new case analysis builder
    pub fn new(parent: ProofBranch) -> Self {
        Self {
            parent_branch: parent,
            target: None,
            cases: Vec::new(),
        }
    }

    /// Specify what variable/expression we're analyzing
    pub fn on_variable(mut self, var: &str) -> Self {
        self.target = Some(var.to_string());
        self
    }

    /// Specify what variable/expression we're analyzing (alias for on_variable)
    pub fn on_expression(mut self, expr: &str) -> Self {
        self.target = Some(expr.to_string());
        self
    }

    /// Add a case with its own scope provided by closure
    pub fn case<F>(mut self, description: &str, case_fn: F) -> Self
    where
        F: FnOnce(ProofBranch) -> ProofBranch,
    {
        // Create a node for the case in the proof forest
        let parent_branch = self.parent_branch.clone();
        let forest_clone = parent_branch.forest.clone();

        // Create case description
        let case_description = if let Some(target) = &self.target {
            format!("Case: {} where {}", target, description)
        } else {
            format!("Case: {}", description)
        };

        // We'll create all the state we need first, then handle the forest in a separate scope
        let (case_id, case_path) = {
            let mut forest_borrow = forest_clone.borrow_mut();
            let parent_node = forest_borrow
                .get_node(parent_branch.node_id)
                .unwrap()
                .clone();

            // Add case node to forest
            let case_id = forest_borrow.add_node(
                Some(parent_branch.node_id),
                parent_node.state.clone(),
                Some(Tactic::Intro(
                    case_description.clone(),
                    self.cases.len() + 1,
                )),
                case_description,
                ProofStatus::InProgress,
            );

            // Create case path
            let case_path = format!("{}_c{}", parent_branch.path_id, self.cases.len() + 1);

            (case_id, case_path)
        };

        // Create case branch outside of the borrow scope
        let case_branch = ProofBranch {
            node_id: case_id,
            forest: forest_clone.clone(),
            path_id: case_path,
        };

        // Execute the case function with its own scope - with no active borrows
        let completed_case = case_fn(case_branch);

        // Store the completed case
        self.cases.push((description.to_string(), completed_case));

        self
    }

    /// Build the case analysis result
    pub fn build(self) -> CaseResult {
        // Create a parent node for all cases
        let parent_branch = self.parent_branch.clone();
        let forest_clone = parent_branch.forest.clone();
        let case_descriptions: Vec<String> =
            self.cases.iter().map(|(desc, _)| desc.clone()).collect();
        let target = self.target.unwrap_or_else(|| "expression".to_string());
        let parent_path = parent_branch.path_id.clone();

        // Another scope to ensure forest_borrow is dropped before returning
        let parent_id = {
            let mut forest_borrow = forest_clone.borrow_mut();
            let parent_node = forest_borrow
                .get_node(parent_branch.node_id)
                .unwrap()
                .clone();

            let case_tactic = Tactic::CaseAnalysis {
                target: target.clone(),
                cases: case_descriptions.clone(),
            };

            forest_borrow.add_node(
                Some(parent_branch.node_id),
                parent_node.state.clone(),
                Some(case_tactic),
                format!("Case analysis with {} cases", self.cases.len()),
                ProofStatus::InProgress,
            )
        };

        // Create parent branch
        let parent_branch = ProofBranch {
            node_id: parent_id,
            forest: forest_clone,
            path_id: format!("{}_cases", parent_path),
        };

        // Extract case branches
        let case_branches: Vec<ProofBranch> =
            self.cases.into_iter().map(|(_, branch)| branch).collect();

        CaseResult {
            parent_branch,
            cases: case_branches,
            parent_path,
        }
    }
}

impl ProofBranch {
    /// Apply a tactic to this branch, creating a new state
    pub fn apply_tactic(&self, tactic: Tactic, note: String) -> Self {
        // Use a single mutable borrow for the entire operation
        let mut forest = self.forest.borrow_mut();

        // Get the current node and create the new state
        let current_node = forest.get_node(self.node_id).unwrap();
        let new_state = tactic.apply(&current_node.state);

        // Add the new node
        let new_id = forest.add_node(
            Some(self.node_id),
            new_state,
            Some(tactic),
            note,
            ProofStatus::InProgress,
        );

        Self {
            node_id: new_id,
            forest: self.forest.clone(),
            path_id: self.next_path_id(1), // Increment the last path segment
        }
    }

    /// Create a new branch from this point with a new branch index
    pub fn branch(&self) -> Self {
        let branch_count = {
            let forest = self.forest.borrow();
            let node = forest.get_node(self.node_id).unwrap();
            node.children.len()
        };

        Self {
            node_id: self.node_id,
            forest: self.forest.clone(),
            path_id: self.branch_path_id(branch_count), // Create a new branch path
        }
    }

    /// Create a new branch with an explicit branching identifier
    pub fn branch_with_id(&self, branch_id: usize) -> Self {
        Self {
            node_id: self.node_id,
            forest: self.forest.clone(),
            path_id: self.branch_path_id(branch_id),
        }
    }

    /// Helper to get the next path ID by incrementing the last segment
    fn next_path_id(&self, increment: usize) -> String {
        let mut segments: Vec<String> = self.path_id.split('_').map(|s| s.to_string()).collect();

        if let Some(last) = segments.last_mut() {
            if let Ok(num) = last.parse::<usize>() {
                *last = (num + increment).to_string();
                return segments.join("_");
            }
        }

        // Fallback if parsing fails
        format!("{}_1", self.path_id)
    }

    /// Helper to create a path ID for a new branch
    fn branch_path_id(&self, branch_id: usize) -> String {
        format!("{}_{}", self.path_id, branch_id)
    }

    /// Mark this branch as complete
    pub fn mark_complete(&self) -> Self {
        let mut forest = self.forest.borrow_mut();
        if let Some(node) = forest.get_node_mut(self.node_id) {
            node.status = ProofStatus::Complete;
        }
        self.clone()
    }

    /// Alternative to mark_complete - returns self for method chaining
    pub fn should_complete(&self) -> Self {
        self.mark_complete()
    }

    /// Mark this branch as a work in progress
    pub fn mark_wip(&self) -> Self {
        let mut forest = self.forest.borrow_mut();
        if let Some(node) = forest.get_node_mut(self.node_id) {
            node.status = ProofStatus::Wip;
        }
        self.clone()
    }

    /// Mark this branch as a todo item
    pub fn mark_todo(&self) -> Self {
        let mut forest = self.forest.borrow_mut();
        if let Some(node) = forest.get_node_mut(self.node_id) {
            node.status = ProofStatus::Todo;
        }
        self.clone()
    }

    /// Mark this branch as abandoned
    pub fn mark_abandoned(&self) -> Self {
        let mut forest = self.forest.borrow_mut();
        if let Some(node) = forest.get_node_mut(self.node_id) {
            node.status = ProofStatus::Abandoned;
        }
        self.clone()
    }

    /// Create a bookmark at this point
    pub fn bookmark(&self, name: &str) -> Self {
        let mut forest = self.forest.borrow_mut();
        forest.add_bookmark(name, self.node_id);
        self.clone()
    }

    /// Get a summary of this branch
    pub fn summary(&self) -> String {
        let forest = self.forest.borrow();
        let path = forest.get_path(self.node_id);

        let mut result = String::new();
        result.push_str("Proof Branch Summary:\n");

        for (i, &id) in path.iter().enumerate() {
            if let Some(node) = forest.get_node(id) {
                let indent = "  ".repeat(i);
                let status_str = match node.status {
                    ProofStatus::Complete => "✓",
                    ProofStatus::InProgress => "→",
                    ProofStatus::Todo => "□",
                    ProofStatus::Wip => "●",
                    ProofStatus::Abandoned => "✗",
                };

                let tactic_str = match &node.tactic {
                    Some(tactic) => format!(" [{:?}]", tactic),
                    None => "".to_string(),
                };

                result.push_str(&format!(
                    "{}{} Node {}{} - {}\n",
                    indent, status_str, id, tactic_str, node.note
                ));
            }
        }

        result
    }

    /// Visualize the entire proof forest
    pub fn visualize_forest(&self) -> String {
        let forest = self.forest.borrow();
        forest.visualize()
    }

    /// Apply the 'Intro' tactic to this branch
    pub fn tactics_intro(&self, variable_name: &str, sequence: u32) -> Self {
        self.apply_tactic(
            Tactic::Intro(variable_name.to_string(), sequence as usize),
            format!("Introduce variable '{}'", variable_name),
        )
    }

    /// Apply the 'Substitution' tactic to this branch
    pub fn tactics_subs(&self, expression: &str, sequence: u32) -> Self {
        self.apply_tactic(
            Tactic::Substitution(expression.to_string(), sequence as usize),
            format!("Substitute with '{}'", expression),
        )
    }

    /// Apply a theorem to this branch
    pub fn tactics_theorem_app(
        &self,
        theorem_id: &str,
        instantiation: HashMap<String, String>,
    ) -> Self {
        // In a real implementation, this would convert the string map to MathExpression
        // For now, we simplify by treating them as empty
        let math_instantiation = HashMap::new();

        self.apply_tactic(
            Tactic::TheoremApplication(theorem_id.to_string(), math_instantiation),
            format!("Apply theorem '{}'", theorem_id),
        )
    }

    /// Apply a rewrite tactic
    pub fn tactics_rewrite(
        &self,
        target: &str,
        equation: &str,
        direction: RewriteDirection,
    ) -> Self {
        self.apply_tactic(
            Tactic::Rewrite {
                target: target.to_string(),
                equation: equation.to_string(),
                direction,
            },
            format!("Rewrite '{}' using '{}'", target, equation),
        )
    }

    /// Apply a simplify tactic
    pub fn tactics_simplify(&self, expression: &str) -> Self {
        self.apply_tactic(
            Tactic::Simplify(expression.to_string()),
            format!("Simplify '{}'", expression),
        )
    }

    /// Get the path representation of this branch
    pub fn get_path_name(&self) -> String {
        self.path_id.clone()
    }

    /// Create multiple cases within a single tactic application
    /// This keeps all cases together as part of the same logical step
    pub fn cases(&self, case_descriptions: Vec<String>) -> CaseResult {
        let parent = self.clone();
        let parent_path = self.path_id.clone();

        // Create the parent node that will contain all cases
        let mut forest = self.forest.borrow_mut();
        let parent_node = forest.get_node(self.node_id).unwrap().clone();
        let case_tactic = Tactic::CaseAnalysis {
            target: "proposition".to_string(), // This should be customizable in a real implementation
            cases: case_descriptions.clone(),
        };

        let case_parent_id = forest.add_node(
            Some(self.node_id),
            parent_node.state.clone(),
            Some(case_tactic),
            format!("Case analysis with {} cases", case_descriptions.len()),
            ProofStatus::InProgress,
        );

        // Create individual cases
        let mut case_branches = Vec::new();
        for (i, description) in case_descriptions.iter().enumerate() {
            let case_path = format!("{}_c{}", self.path_id, i + 1); // Use c prefix for cases
            let case_state = parent_node.state.transform(
                |s| s.clone(),
                case_path.clone(),
                format!("Case {}: {}", i + 1, description),
            );

            let case_id = forest.add_node(
                Some(case_parent_id),
                case_state,
                None,
                description.clone(),
                ProofStatus::InProgress,
            );

            case_branches.push(ProofBranch {
                node_id: case_id,
                forest: self.forest.clone(),
                path_id: case_path,
            });
        }

        // Create the parent branch that contains all cases
        let case_parent_branch = ProofBranch {
            node_id: case_parent_id,
            forest: self.forest.clone(),
            path_id: format!("{}_cases", self.path_id),
        };

        CaseResult {
            parent_branch: case_parent_branch,
            cases: case_branches,
            parent_path: parent_path,
        }
    }

    /// Start a case analysis with builder pattern
    pub fn case_analysis(&self) -> CaseAnalysisBuilder {
        CaseAnalysisBuilder::new(self.clone())
    }

    /// Apply an 'IntroExpr' tactic that works directly with MathExpression
    pub fn tactics_intro_expr(
        &self,
        name: &str,
        var_type: MathObjectType,
        expression: MathExpression,
        sequence: u32,
    ) -> Self {
        self.apply_tactic(
            Tactic::IntroExpr {
                name: name.to_string(),
                var_type,
                expression,
                sequence: sequence as usize,
            },
            format!("Introduce variable '{}' with syntax tree expression", name),
        )
    }

    /// Apply a 'SubstitutionExpr' tactic that works directly with MathExpression
    pub fn tactics_subs_expr(
        &self,
        pattern: MathExpression,
        replacement: MathExpression,
        location: Option<Vec<usize>>,
        sequence: u32,
    ) -> Self {
        self.apply_tactic(
            Tactic::SubstitutionExpr {
                pattern,
                replacement,
                location,
                sequence: sequence as usize,
            },
            format!("Substitute with syntax tree expression"),
        )
    }

    /// Apply a theorem to this branch using MathExpression for instantiations
    pub fn tactics_theorem_app_expr(
        &self,
        theorem_id: &str,
        instantiation: HashMap<String, MathExpression>,
        target_expr: Option<MathExpression>,
    ) -> Self {
        self.apply_tactic(
            Tactic::TheoremApplicationExpr {
                theorem_id: theorem_id.to_string(),
                instantiation,
                target_expr,
            },
            format!(
                "Apply theorem '{}' with syntax tree expressions",
                theorem_id
            ),
        )
    }

    /// Apply a rewrite tactic with syntax tree expressions
    pub fn tactics_rewrite_expr(
        &self,
        target_expr: MathExpression,
        equation_expr: MathExpression,
        direction: RewriteDirection,
        location: Option<Vec<usize>>,
    ) -> Self {
        self.apply_tactic(
            Tactic::RewriteExpr {
                target_expr,
                equation_expr,
                direction,
                location,
            },
            format!("Rewrite using syntax tree expressions"),
        )
    }

    /// Apply a case analysis with syntax tree expressions
    pub fn tactics_case_analysis_expr(
        &self,
        target_expr: MathExpression,
        case_exprs: Vec<MathExpression>,
        case_names: Vec<String>,
    ) -> Self {
        self.apply_tactic(
            Tactic::CaseAnalysisExpr {
                target_expr,
                case_exprs,
                case_names,
            },
            format!("Case analysis with syntax tree expressions"),
        )
    }
}

/// Helper to get the current timestamp in milliseconds
#[cfg(not(target_arch = "wasm32"))]
fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Helper to get the current timestamp in milliseconds using JavaScript's Date.now() for WASM
#[cfg(target_arch = "wasm32")]
fn get_timestamp() -> u64 {
    // Use js-sys to get the current timestamp in a WASM-compatible way
    js_sys::Date::now() as u64
}

/// Example usage of the proof builder
#[cfg(test)]
mod tests {
    use super::*;
    use crate::subjects::math::theorem::core::MathObjectType;
    use crate::subjects::math::theorem::expressions::{MathExpression, Variable};
    use crate::subjects::math::theorem::relations::MathRelation;
    use std::collections::HashMap;

    // Helper functions to create test expressions
    fn create_var(name: &str) -> MathExpression {
        MathExpression::Var(Variable::E(200))
    }

    fn create_relation() -> MathRelation {
        // Create a simple equality relation for testing
        let a = create_var("a");
        let b = create_var("b");
        MathRelation::equal(a, b)
    }

    #[test]
    fn test_intro_expr_tactic() {
        // Create a proof state with a simple relation
        let state = ProofState {
            quantifier: vec![],
            value_variables: vec![],
            statement: create_relation(),
            path: Some("p0".to_string()),
            justification: None,
        };

        // Create a simple variable introduction tactic
        let var_name = "x";
        let var_type = MathObjectType::Real;
        let var_expr = create_var(var_name);

        let tactic = Tactic::IntroExpr {
            name: var_name.to_string(),
            var_type: var_type.clone(),
            expression: var_expr.clone(),
            sequence: 1,
        };

        // Apply the tactic and check the result
        let new_state = tactic.apply(&state);

        // Check that we have a justification
        assert!(new_state.justification.is_some());
        let justification = new_state.justification.unwrap();
        assert!(justification.contains(var_name));
    }

    #[test]
    fn test_substitution_expr_tactic() {
        // Create a proof state with a simple relation
        let state = ProofState {
            quantifier: vec![],
            value_variables: vec![],
            statement: create_relation(),
            path: Some("p0".to_string()),
            justification: None,
        };

        // Create a simple substitution tactic
        let pattern = create_var("a");
        let replacement = create_var("c");

        let tactic = Tactic::SubstitutionExpr {
            pattern: pattern.clone(),
            replacement: replacement.clone(),
            location: None,
            sequence: 1,
        };

        // Apply the tactic
        let new_state = tactic.apply(&state);

        // Check that we have a justification
        assert!(new_state.justification.is_some());
    }

    #[test]
    fn test_theorem_application_expr_tactic() {
        // Create a proof state with a simple relation
        let state = ProofState {
            quantifier: vec![],
            value_variables: vec![],
            statement: create_relation(),
            path: Some("p0".to_string()),
            justification: None,
        };

        // Create a theorem application tactic
        let tactic = Tactic::TheoremApplicationExpr {
            theorem_id: "commutativity".to_string(),
            instantiation: HashMap::new(),
            target_expr: Some(create_var("a")),
        };

        // Apply the tactic
        let new_state = tactic.apply(&state);

        // Check that we have a justification
        assert!(new_state.justification.is_some());
        let justification = new_state.justification.unwrap();
        assert!(justification.contains("commutativity"));
    }

    #[test]
    fn test_case_analysis_expr_tactic() {
        // Create a proof state with a simple relation
        let state = ProofState {
            quantifier: vec![],
            value_variables: vec![],
            statement: create_relation(),
            path: Some("p0".to_string()),
            justification: None,
        };

        // Create a case analysis tactic
        let tactic = Tactic::CaseAnalysisExpr {
            target_expr: create_var("a"),
            case_exprs: vec![create_var("case1"), create_var("case2")],
            case_names: vec!["Case 1".to_string(), "Case 2".to_string()],
        };

        // Apply the tactic
        let new_state = tactic.apply(&state);

        // Check that we have a justification
        assert!(new_state.justification.is_some());
        let justification = new_state.justification.unwrap();
        assert!(justification.contains("Case analysis"));
        assert!(justification.contains("2 cases"));
    }

    #[test]
    fn test_rewrite_expr_tactic() {
        // Create a proof state with a simple relation
        let state = ProofState {
            quantifier: vec![],
            value_variables: vec![],
            statement: create_relation(),
            path: Some("p0".to_string()),
            justification: None,
        };

        // Create a rewrite tactic
        let tactic = Tactic::RewriteExpr {
            target_expr: create_var("a"),
            equation_expr: create_var("a = b"),
            direction: RewriteDirection::LeftToRight,
            location: None,
        };

        // Apply the tactic
        let new_state = tactic.apply(&state);

        // Check that we have a justification
        assert!(new_state.justification.is_some());
    }

    #[test]
    fn test_proof_branch() {
        // Create a theorem builder
        let builder = TheoremBuilder::new("Test Theorem", create_relation(), vec![]);

        // Get the initial branch
        let p0 = builder.initial_branch();

        // Add some proof steps
        let p1 = p0.tactics_intro_expr("a", MathObjectType::Real, create_var("a"), 1);

        let p2 = p1.tactics_intro_expr("b", MathObjectType::Real, create_var("b"), 2);

        // Mark as complete
        let p3 = p2.should_complete();

        // Build the theorem
        let theorem = builder.build();
        assert_eq!(theorem.name, "Test Theorem");
    }

    #[test]
    fn test_multiple_tactics_chain() {
        // Create a theorem builder
        let builder = TheoremBuilder::new("Test Theorem", create_relation(), vec![]);

        // Get the initial branch
        let p0 = builder.initial_branch();

        // Apply multiple tactics in sequence
        let p1 = p0.tactics_intro_expr("a", MathObjectType::Real, create_var("a"), 1);

        let p2 = p1.tactics_subs_expr(create_var("a"), create_var("b"), None, 2);

        let p3 = p2.tactics_theorem_app_expr("commutativity", HashMap::new(), None);

        // Mark as complete
        let p4 = p3.should_complete();

        // Check that the proof tree has the right structure
        let forest = builder.proof_forest.borrow();
        assert_eq!(forest.nodes.len(), 5); // Initial node + 4 steps
    }
}
