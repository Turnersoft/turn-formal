use std::collections::HashMap;

use crate::subjects::math::formalism::expressions::{Identifier, MathExpression};
use crate::subjects::math::formalism::proof::DecompositionMethod;
use crate::subjects::math::formalism::proof::{find_subexpr_in_expr, replace_subexpr_in_relation};
use crate::subjects::math::formalism::relations::MathRelation;
use crate::subjects::math::formalism::theorem::ProofGoal;

use super::RewriteDirection;
use super::{InductionType, utils::*};
use super::{Tactic, get_theorem_registry};

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
                let var = crate::subjects::math::formalism::theorem::ValueBindedVariable {
                    name: name.clone(),
                    value: expression.clone(),
                };

                new_state.value_variables.push(var);
                Some(new_state)
            }
            Tactic::Apply {
                theorem_id,
                instantiation,
                target_expr,
            } => {
                // Apply a theorem or hypothesis from the context
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
                let statement = &state.statement;

                // If location is provided, use it directly
                if let Some(path) = location {
                    let new_statement =
                        local_replace_in_relation(statement, &path, target, replacement);

                    Some(ProofGoal {
                        quantifier: state.quantifier.clone(),
                        value_variables: state.value_variables.clone(),
                        statement: new_statement,
                    })
                } else {
                    // Search for occurrences of the target expression
                    let mut paths = Vec::new();
                    find_all_occurrences_in_relation(statement, target, &mut paths, vec![]);

                    if !paths.is_empty() {
                        // If found, use the first occurrence
                        let path = &paths[0];
                        let new_statement =
                            local_replace_in_relation(statement, path, target, replacement);

                        Some(ProofGoal {
                            quantifier: state.quantifier.clone(),
                            value_variables: state.value_variables.clone(),
                            statement: new_statement,
                        })
                    } else {
                        // Try more specialized matching for direct expression comparisons
                        match statement {
                            MathRelation::Equal { meta, left, right } => {
                                if left == target {
                                    return Some(ProofGoal {
                                        quantifier: state.quantifier.clone(),
                                        value_variables: state.value_variables.clone(),
                                        statement: MathRelation::Equal {
                                            meta: meta.clone(),
                                            left: replacement.clone(),
                                            right: right.clone(),
                                        },
                                    });
                                } else if right == target {
                                    return Some(ProofGoal {
                                        quantifier: state.quantifier.clone(),
                                        value_variables: state.value_variables.clone(),
                                        statement: MathRelation::Equal {
                                            meta: meta.clone(),
                                            left: left.clone(),
                                            right: replacement.clone(),
                                        },
                                    });
                                }
                            }
                            // Add other relation types as needed
                            _ => {}
                        }

                        // Not found anywhere
                        None
                    }
                }
            }
            Tactic::ChangeView { object: _, view: _ } => {
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
                // Convert Identifier keys to String keys
                let string_instantiation: HashMap<String, MathExpression> = instantiation
                    .iter()
                    .filter_map(|(id, expr)| {
                        if let Identifier::Name(name, _) = id {
                            Some((name.clone(), expr.clone()))
                        } else {
                            None // Cannot convert non-name Identifiers to String keys here
                        }
                    })
                    .collect();

                let registry = get_theorem_registry().lock().unwrap();
                if let Some(result) = registry.apply_theorem(
                    theorem_id,
                    &state.statement,
                    &string_instantiation, // Use the converted map
                    target_expr.clone(),
                ) {
                    let mut new_state = state.clone();
                    new_state.statement = result;
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

                        // Find target in statement using our local implementation
                        if let Some((found_expr, path)) = local_find_subexpression(
                            &state.statement,
                            target_expr,
                            location.clone(),
                        ) {
                            // Apply the replacement
                            new_state.statement = local_replace_in_relation(
                                &state.statement,
                                &path,
                                &found_expr,
                                replacement,
                            );

                            Some(new_state)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Tactic::CaseAnalysis {
                target_expr: _,
                case_exprs: _,
                case_names: _,
            } => {
                let new_state = state.clone();
                Some(new_state)
            }
            // Handle other tactics
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
                instantiation: _,
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
                instantiation: _,
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
                format!("Branched: {}", description)
            }
            Tactic::Case {
                parent_id,
                case_expr,
                case_name,
            } => {
                format!("Case: {} - {}", case_name, expression_summary(case_expr))
            }
        }
    }
}

pub fn legacy_apply(tactic: &Tactic, state: &ProofGoal) -> Option<ProofGoal> {
    // Legacy fallback for unimplemented tactics
    match tactic {
        Tactic::Branch { .. } => Some(state.clone()),
        Tactic::Case { .. } => Some(state.clone()),
        Tactic::Decompose { .. } => Some(state.clone()),
        Tactic::Induction { .. } => Some(state.clone()),
        Tactic::Simplify { .. } => Some(state.clone()),
        Tactic::Custom { .. } => Some(state.clone()),
        _ => None,
    }
}

/// Helper function to find all occurrences of a target expression in a relation
fn find_all_occurrences_in_relation(
    relation: &MathRelation,
    target: &MathExpression,
    results: &mut Vec<Vec<usize>>,
    current_path: Vec<usize>,
) {
    use crate::subjects::math::formalism::relations::MathRelation;

    match relation {
        MathRelation::Equal { left, right, .. } => {
            // Helper function to check if a pattern is in an expression and add result
            fn check_and_add(
                expr: &MathExpression,
                pattern: &MathExpression,
                index: usize,
                current_path: &[usize],
                results: &mut Vec<Vec<usize>>,
            ) {
                // Simple exact match check
                if expr == pattern {
                    let mut path = current_path.to_vec();
                    path.push(index);
                    results.push(path);
                }

                // We could do deeper matching, but for now just check for equality
            }

            // Check left and right sides
            check_and_add(left, target, 0, &current_path, results);
            check_and_add(right, target, 1, &current_path, results);
        }
        MathRelation::Implies(ante, cons) => {
            // For boxed MathRelation values, we recursively check inside them
            let mut ante_path = current_path.clone();
            ante_path.push(0);
            find_all_occurrences_in_relation(ante, target, results, ante_path);

            let mut cons_path = current_path.clone();
            cons_path.push(1);
            find_all_occurrences_in_relation(cons, target, results, cons_path);
        }
        MathRelation::Equivalent(left, right) => {
            // For Equivalent with boxed MathRelation values
            let mut left_path = current_path.clone();
            left_path.push(0);
            find_all_occurrences_in_relation(left, target, results, left_path);

            let mut right_path = current_path.clone();
            right_path.push(1);
            find_all_occurrences_in_relation(right, target, results, right_path);
        }
        // Handle other relation types as needed
        _ => {}
    }
}

/// Helper function to find an expression in a MathRelation
fn find_expression_in_relation(
    relation: &MathRelation,
    target: &MathExpression,
) -> Option<Vec<usize>> {
    // Directly extract expressions from the relation if possible
    match relation {
        MathRelation::Equal { left, right, .. } => {
            if let Some(mut path) = find_subexpr_in_expr(left, target) {
                path.insert(0, 0);
                return Some(path);
            }
            if let Some(mut path) = find_subexpr_in_expr(right, target) {
                path.insert(0, 1);
                return Some(path);
            }
        }
        // Handle other relation types that might contain MathExpressions
        _ => {}
    }

    // If no direct expression is found, recursively check for expressions in the structure
    None
}

// Add a helper function to replace a subexpression in a relation
fn local_replace_in_relation(
    relation: &MathRelation,
    path: &[usize],
    target: &MathExpression,
    replacement: &MathExpression,
) -> MathRelation {
    if path.is_empty() {
        return relation.clone();
    }

    match relation {
        MathRelation::Equal { meta, left, right } => {
            if path[0] == 0 && left == target {
                // Replace in left side
                MathRelation::Equal {
                    meta: meta.clone(),
                    left: replacement.clone(),
                    right: right.clone(),
                }
            } else if path[0] == 1 && right == target {
                // Replace in right side
                MathRelation::Equal {
                    meta: meta.clone(),
                    left: left.clone(),
                    right: replacement.clone(),
                }
            } else {
                // No match found, return unchanged
                relation.clone()
            }
        }
        MathRelation::Implies(ante, cons) => {
            if path[0] == 0 {
                // Replace in antecedent
                MathRelation::Implies(
                    Box::new(local_replace_in_relation(
                        ante,
                        &path[1..],
                        target,
                        replacement,
                    )),
                    cons.clone(),
                )
            } else if path[0] == 1 {
                // Replace in consequent
                MathRelation::Implies(
                    ante.clone(),
                    Box::new(local_replace_in_relation(
                        cons,
                        &path[1..],
                        target,
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
                    Box::new(local_replace_in_relation(
                        left,
                        &path[1..],
                        target,
                        replacement,
                    )),
                    right.clone(),
                )
            } else if path[0] == 1 {
                // Replace in right side
                MathRelation::Equivalent(
                    left.clone(),
                    Box::new(local_replace_in_relation(
                        right,
                        &path[1..],
                        target,
                        replacement,
                    )),
                )
            } else {
                relation.clone()
            }
        }
        // For other relation types, just return the relation unchanged
        _ => relation.clone(),
    }
}

// Add a local implementation of find_subexpression to find expressions in MathRelation
fn local_find_subexpression(
    statement: &MathRelation,
    pattern: &MathExpression,
    location: Option<Vec<usize>>,
) -> Option<(MathExpression, Vec<usize>)> {
    // If location is provided, use it to navigate to that position
    if let Some(path) = location {
        // We don't actually use current_path in this case, so we can remove it

        // Navigate according to the path
        match statement {
            MathRelation::Equal { left, right, .. } => {
                if path[0] == 0 && left == pattern {
                    return Some((left.clone(), vec![0]));
                } else if path[0] == 1 && right == pattern {
                    return Some((right.clone(), vec![1]));
                }
            }
            // Add other relation types as needed
            _ => {}
        }

        // Not found at the specified location
        return None;
    }

    // Otherwise, search for occurrences of the pattern
    let mut paths = Vec::new();
    find_all_occurrences_in_relation(statement, pattern, &mut paths, vec![]);

    if !paths.is_empty() {
        // Get the first occurrence path
        let path = paths[0].clone();

        // Extract the expression at that path
        match statement {
            MathRelation::Equal { left, right, .. } => {
                if path[0] == 0 {
                    return Some((left.clone(), path));
                } else if path[0] == 1 {
                    return Some((right.clone(), path));
                }
            }
            // Add other relation types as needed
            _ => {}
        }
    }

    // Not found
    None
}
