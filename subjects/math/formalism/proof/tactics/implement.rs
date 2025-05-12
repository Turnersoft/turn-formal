use std::collections::HashMap;

use crate::subjects::math::formalism::expressions::{Identifier, MathExpression};
use crate::subjects::math::formalism::proof::DecompositionMethod;
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
                // Try to find the target in the relation
                if let Some((_, path)) = state.find_subexpression(target, location.clone()) {
                    let mut new_state = state.clone();

                    // Since replace_subexpr_in_relation is not available, we'll just
                    // return a clone of the state for now
                    // new_state.statement = replace_subexpr_in_relation(&state.statement, target, &path, replacement);

                    Some(new_state)
                } else {
                    None
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

                        // Find target in statement
                        if let Some((_, path)) =
                            state.find_subexpression(target_expr, location.clone())
                        {
                            // For now just return the cloned state since we don't have
                            // replace_subexpr_in_relation
                            /*
                            new_state.statement = replace_subexpr_in_relation(
                                &state.statement,
                                target_expr,
                                &path,
                                replacement,
                            );
                            */

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
