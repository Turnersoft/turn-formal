use std::collections::HashMap;

use crate::subjects::math::{
    formalism::{
        expressions::{Identifier, MathExpression},
        proof::{ProofGoal, ValueBindedVariable},
        relations::MathRelation,
        search_replace::{SearchReplace, SearchResult},
        theorem::{MathObject, Quantification, QuantifiedMathObject},
    },
    theories::primitive_definitions::PrimitiveDef,
};

use super::{CaseCondition, Tactic};

/// The result of applying a tactic to a proof goal
pub enum TacticApplicationResult {
    /// A single transformed goal is produced
    SingleGoal(ProofGoal),
    /// Multiple goals are produced (e.g., for case analysis)
    MultipleGoals(Vec<ProofGoal>),
    /// The tactic made no change
    NoChange,
    /// An error occurred during tactic application
    Error(String),
}

/// A trait for tactics that can match patterns in a proof goal
pub trait TacticMatcher {
    /// Find all applicable targets for this tactic in a goal
    fn find_targets(&self, goal: &ProofGoal) -> Vec<SearchResult>;

    /// Check if this tactic can be applied to the goal
    fn is_applicable(&self, goal: &ProofGoal) -> bool {
        !self.find_targets(goal).is_empty()
    }

    /// Get the primary search pattern for this tactic
    fn get_search_pattern(&self) -> MathExpression;
}

/// A trait for tactics that can be applied to a proof goal
pub trait TacticApplier {
    /// Apply the tactic to a goal, transforming it
    fn apply_to_goal(&self, goal: &ProofGoal) -> TacticApplicationResult;
}

impl TacticApplier for Tactic {
    fn apply_to_goal(&self, goal: &ProofGoal) -> TacticApplicationResult {
        match self {
            // ========== QUANTIFIER TACTICS ==========
            Tactic::IntroduceQuantifier { object, position } => {
                let mut new_goal = goal.clone();
                if let Some(pos) = position {
                    if *pos <= new_goal.quantifier.len() {
                        new_goal.quantifier.insert(*pos, object.clone());
                    } else {
                        return TacticApplicationResult::Error(format!(
                            "Position {} out of bounds for quantifier array",
                            pos
                        ));
                    }
                } else {
                    new_goal.quantifier.push(object.clone());
                }
                TacticApplicationResult::SingleGoal(new_goal)
            }

            Tactic::IntroduceFreshVariable {
                target_quantifier,
                fresh_variable_name,
            } => {
                let mut new_goal = goal.clone();
                if let Some(pos) = new_goal
                    .quantifier
                    .iter()
                    .position(|q| q.variable == *target_quantifier)
                {
                    let quantified_obj = &new_goal.quantifier[pos];

                    if quantified_obj.quantification != Quantification::Universal {
                        return TacticApplicationResult::Error(
                            "IntroduceFreshVariable only applies to universal (∀) quantifiers."
                                .to_string(),
                        );
                    }

                    new_goal.quantifier.remove(pos);
                    new_goal.value_variables.push(ValueBindedVariable {
                        name: fresh_variable_name.clone(),
                        value: MathExpression::Var(Identifier::Name(
                            "arbitrary_constant".to_string(),
                            0,
                        )),
                    });

                    new_goal.statement = Self::substitute_in_relation(
                        &new_goal.statement,
                        target_quantifier,
                        &MathExpression::Var(fresh_variable_name.clone()),
                    );

                    TacticApplicationResult::SingleGoal(new_goal)
                } else {
                    TacticApplicationResult::Error(format!(
                        "Quantifier {:?} not found",
                        target_quantifier
                    ))
                }
            }

            Tactic::ProvideWitness {
                target_quantifier,
                witness,
            } => {
                let mut new_goal = goal.clone();
                if let Some(pos) = new_goal
                    .quantifier
                    .iter()
                    .position(|q| q.variable == *target_quantifier)
                {
                    let quantified_obj = &new_goal.quantifier[pos];
                    match quantified_obj.quantification {
                        Quantification::Existential | Quantification::UniqueExistential => {
                            new_goal.quantifier.remove(pos);
                            new_goal.value_variables.push(ValueBindedVariable {
                                name: target_quantifier.clone(),
                                value: witness.clone(),
                            });
                            new_goal.statement = Self::substitute_in_relation(
                                &goal.statement,
                                target_quantifier,
                                witness,
                            );
                            TacticApplicationResult::SingleGoal(new_goal)
                        }
                        _ => TacticApplicationResult::Error(
                            "ProvideWitness only applies to existential (∃, ∃!) quantifiers."
                                .to_string(),
                        ),
                    }
                } else {
                    TacticApplicationResult::Error(format!(
                        "Quantifier {:?} not found",
                        target_quantifier
                    ))
                }
            }

            Tactic::ReorderQuantifiers { new_order } => {
                // Warning: This is a simplified implementation. A real one must check for legality.
                let mut new_goal = goal.clone();
                let mut reordered = Vec::new();
                for id in new_order {
                    if let Some(pos) = new_goal.quantifier.iter().position(|q| q.variable == *id) {
                        reordered.push(new_goal.quantifier[pos].clone());
                    } else {
                        return TacticApplicationResult::Error(format!(
                            "Quantifier {:?} not found for reordering",
                            id
                        ));
                    }
                }
                if reordered.len() != new_goal.quantifier.len() {
                    return TacticApplicationResult::Error(
                        "New order doesn't include all quantifiers".to_string(),
                    );
                }
                new_goal.quantifier = reordered;
                TacticApplicationResult::SingleGoal(new_goal)
            }

            Tactic::UniversalCaseAnalysis {
                target_quantifier,
                cases,
            } => {
                if let Some(pos) = goal
                    .quantifier
                    .iter()
                    .position(|q| q.variable == *target_quantifier)
                {
                    let quantified_obj = goal.quantifier[pos].clone();
                    if quantified_obj.quantification != Quantification::Universal {
                        return TacticApplicationResult::Error(
                            "Case analysis only applies to universal quantifiers".to_string(),
                        );
                    }

                    let subgoals: Vec<ProofGoal> = cases
                        .iter()
                        .map(|case| {
                            let mut case_goal = goal.clone();
                            case_goal.quantifier.remove(pos);
                            case_goal.value_variables.push(ValueBindedVariable {
                                name: Identifier::Name(format!("case_{}", case.name), 0),
                                value: MathExpression::Relation(Box::new(case.condition.clone())),
                            });
                            for value in &case.values {
                                case_goal.statement = Self::substitute_in_relation(
                                    &case_goal.statement,
                                    target_quantifier,
                                    value,
                                );
                            }
                            case_goal
                        })
                        .collect();
                    TacticApplicationResult::MultipleGoals(subgoals)
                } else {
                    TacticApplicationResult::Error(format!(
                        "Quantifier {:?} not found",
                        target_quantifier
                    ))
                }
            }

            // ========== VALUE VARIABLE TACTICS ==========
            Tactic::IntroduceValueVariable { binding, position } => {
                let mut new_goal = goal.clone();
                if let Some(pos) = position {
                    if *pos <= new_goal.value_variables.len() {
                        new_goal.value_variables.insert(*pos, binding.clone());
                    } else {
                        return TacticApplicationResult::Error(format!(
                            "Position {} out of bounds for value_variables array",
                            pos
                        ));
                    }
                } else {
                    new_goal.value_variables.push(binding.clone());
                }
                TacticApplicationResult::SingleGoal(new_goal)
            }

            Tactic::SubstituteValueVariable { target_variable } => {
                let mut new_goal = goal.clone();
                if let Some(var_binding) = new_goal
                    .value_variables
                    .iter()
                    .find(|v| v.name == *target_variable)
                {
                    let replacement = var_binding.value.clone();
                    new_goal.statement = Self::substitute_in_relation(
                        &new_goal.statement,
                        target_variable,
                        &replacement,
                    );
                    for other_var in &mut new_goal.value_variables {
                        if &other_var.name != target_variable {
                            other_var.value = Self::substitute_in_expression(
                                &other_var.value,
                                target_variable,
                                &replacement,
                            );
                        }
                    }
                    TacticApplicationResult::SingleGoal(new_goal)
                } else {
                    TacticApplicationResult::Error(format!(
                        "Value variable {:?} not found for substitution",
                        target_variable
                    ))
                }
            }

            Tactic::RemoveValueVariable { target_variable } => {
                let mut new_goal = goal.clone();
                if new_goal
                    .value_variables
                    .iter()
                    .any(|v| v.name == *target_variable)
                {
                    new_goal
                        .value_variables
                        .retain(|v| &v.name != target_variable);
                    TacticApplicationResult::SingleGoal(new_goal)
                } else {
                    TacticApplicationResult::Error(format!(
                        "Value variable {:?} not found for removal",
                        target_variable
                    ))
                }
            }

            // ========== STATEMENT TACTICS ==========
            Tactic::AlgebraicManipulation {
                target_path,
                original_expr,
                transformed_expr,
                justification: _,
            } => {
                if let Some(found_expr) = Self::get_expression_at_path(&goal.statement, target_path)
                {
                    if found_expr == *original_expr {
                        let mut new_goal = goal.clone();
                        new_goal.statement = Self::replace_at_path_in_relation(
                            &goal.statement,
                            target_path,
                            transformed_expr,
                        );
                        TacticApplicationResult::SingleGoal(new_goal)
                    } else {
                        TacticApplicationResult::Error(
                            "Original expression doesn't match expression at target path"
                                .to_string(),
                        )
                    }
                } else {
                    TacticApplicationResult::Error("No expression found at target path".to_string())
                }
            }
            Tactic::ApplyTheorem {
                theorem_id,
                target_path,
                instantiation,
            } => {
                TacticApplicationResult::NoChange // Placeholder
            }
            Tactic::Rewrite {
                target_path,
                rewrite_rule,
                left_to_right,
            } => {
                TacticApplicationResult::NoChange // Placeholder
            }
            Tactic::SplitConjunction { target_path } => {
                if let Some(expr) = Self::get_expression_at_path(&goal.statement, target_path) {
                    if let MathExpression::Relation(rel) = expr {
                        if let MathRelation::And(conjuncts) = rel.as_ref() {
                            let goals: Vec<ProofGoal> = conjuncts
                                .iter()
                                .map(|conjunct| {
                                    let mut new_goal = goal.clone();
                                    new_goal.statement = Self::replace_at_path_in_relation(
                                        &goal.statement,
                                        target_path,
                                        &MathExpression::Relation(Box::new(conjunct.clone())),
                                    );
                                    new_goal
                                })
                                .collect();
                            TacticApplicationResult::MultipleGoals(goals)
                        } else {
                            TacticApplicationResult::Error(
                                "Target is not a conjunction".to_string(),
                            )
                        }
                    } else {
                        TacticApplicationResult::Error(
                            "Target path doesn't point to a relation".to_string(),
                        )
                    }
                } else {
                    TacticApplicationResult::Error("No expression found at target path".to_string())
                }
            }
            Tactic::SplitDisjunction { target_path } => {
                TacticApplicationResult::NoChange // Placeholder
            }
            Tactic::StatementCaseAnalysis {
                target_path,
                target_expr: _,
                cases,
            } => {
                let goals: Vec<ProofGoal> = cases
                    .iter()
                    .map(|(case_name, case_expr)| {
                        let mut new_goal = goal.clone();
                        new_goal.value_variables.push(ValueBindedVariable {
                            name: Identifier::Name(case_name.clone(), 0),
                            value: case_expr.clone(),
                        });
                        new_goal
                    })
                    .collect();
                TacticApplicationResult::MultipleGoals(goals)
            }

            // ========== META TACTICS ==========
            Tactic::CompleteProof { .. } => TacticApplicationResult::NoChange,
            Tactic::Induction { .. } => TacticApplicationResult::NoChange,
        }
    }
}

impl TacticMatcher for Tactic {
    fn find_targets(&self, goal: &ProofGoal) -> Vec<SearchResult> {
        let pattern = self.get_search_pattern();
        SearchReplace::find_all_in_relation(&goal.statement, &pattern)
    }

    fn get_search_pattern(&self) -> MathExpression {
        match self {
            // Quantifier tactics don't search within the statement expression
            Tactic::IntroduceQuantifier { .. }
            | Tactic::ReorderQuantifiers { .. }
            | Tactic::UniversalCaseAnalysis { .. } => {
                MathExpression::Var(Identifier::Name("_no_pattern".to_string(), 0))
            }

            Tactic::IntroduceFreshVariable {
                target_quantifier, ..
            } => MathExpression::Var(target_quantifier.clone()),
            Tactic::ProvideWitness {
                target_quantifier, ..
            } => MathExpression::Var(target_quantifier.clone()),

            // Value variable tactics
            Tactic::IntroduceValueVariable { .. } => {
                MathExpression::Var(Identifier::Name("_no_pattern".to_string(), 0))
            }
            Tactic::SubstituteValueVariable {
                target_variable, ..
            } => MathExpression::Var(target_variable.clone()),
            Tactic::RemoveValueVariable {
                target_variable, ..
            } => MathExpression::Var(target_variable.clone()),

            // Statement tactics
            Tactic::AlgebraicManipulation { original_expr, .. } => original_expr.clone(),
            Tactic::ApplyTheorem { .. } => {
                MathExpression::Var(Identifier::Name("_apply_theorem".to_string(), 0))
            }
            Tactic::Rewrite { .. } => {
                MathExpression::Var(Identifier::Name("_rewrite".to_string(), 0))
            }
            Tactic::SplitConjunction { .. } => {
                MathExpression::Var(Identifier::Name("_conjunction".to_string(), 0))
            }
            Tactic::SplitDisjunction { .. } => {
                MathExpression::Var(Identifier::Name("_disjunction".to_string(), 0))
            }
            Tactic::StatementCaseAnalysis { target_expr, .. } => target_expr.clone(),

            // Meta tactics
            Tactic::CompleteProof { .. } => {
                MathExpression::Var(Identifier::Name("_complete".to_string(), 0))
            }
            Tactic::Induction {
                induction_variable, ..
            } => MathExpression::Var(induction_variable.clone()),
        }
    }
}

// Helper implementations for substitution and path-based replacement
impl Tactic {
    /// Helper method to substitute an identifier in a MathRelation
    fn substitute_in_relation(
        relation: &MathRelation,
        target: &Identifier,
        replacement: &MathExpression,
    ) -> MathRelation {
        match relation {
            MathRelation::Equal { meta, left, right } => MathRelation::Equal {
                meta: meta.clone(),
                left: Self::substitute_in_expression(left, target, replacement),
                right: Self::substitute_in_expression(right, target, replacement),
            },
            MathRelation::And(relations) => MathRelation::And(
                relations
                    .iter()
                    .map(|r| Self::substitute_in_relation(r, target, replacement))
                    .collect(),
            ),
            MathRelation::Or(relations) => MathRelation::Or(
                relations
                    .iter()
                    .map(|r| Self::substitute_in_relation(r, target, replacement))
                    .collect(),
            ),
            MathRelation::Not(relation) => MathRelation::Not(Box::new(
                Self::substitute_in_relation(relation, target, replacement),
            )),
            MathRelation::Implies(p, c) => MathRelation::Implies(
                Box::new(Self::substitute_in_relation(p, target, replacement)),
                Box::new(Self::substitute_in_relation(c, target, replacement)),
            ),
            MathRelation::Equivalent(l, r) => MathRelation::Equivalent(
                Box::new(Self::substitute_in_relation(l, target, replacement)),
                Box::new(Self::substitute_in_relation(r, target, replacement)),
            ),
            _ => relation.clone(), // Placeholder for other relation types
        }
    }

    /// Helper method to substitute an identifier in a MathExpression
    fn substitute_in_expression(
        expr: &MathExpression,
        target: &Identifier,
        replacement: &MathExpression,
    ) -> MathExpression {
        match expr {
            MathExpression::Var(id) if id == target => replacement.clone(),
            MathExpression::Relation(rel) => MathExpression::Relation(Box::new(
                Self::substitute_in_relation(rel, target, replacement),
            )),
            MathExpression::ViewAs { expression, view } => MathExpression::ViewAs {
                expression: Box::new(Self::substitute_in_expression(
                    expression,
                    target,
                    replacement,
                )),
                view: view.clone(),
            },
            // Other expression types are terminals for this substitution or need their own logic
            _ => expr.clone(),
        }
    }

    /// Helper method to get an expression at a specific path
    fn get_expression_at_path(relation: &MathRelation, path: &[usize]) -> Option<MathExpression> {
        // Implementation omitted for brevity
        None
    }

    /// Helper method to replace an expression at a specific path
    fn replace_at_path_in_relation(
        relation: &MathRelation,
        path: &[usize],
        replacement: &MathExpression,
    ) -> MathRelation {
        // Implementation omitted for brevity
        relation.clone()
    }
}
