use std::collections::HashMap;

use super::super::{ProofGoal, ValueBindedVariable, get_theorem_registry};
use super::{RewriteDirection, Tactic};
use crate::subjects::math::formalism::expressions::{Identifier, MathExpression};
use crate::subjects::math::formalism::proof::TacticApplicationResult;
use crate::subjects::math::formalism::proof::tactics::search_replace::SearchReplace;
use crate::subjects::math::formalism::relations::MathRelation;

impl Tactic {
    pub fn apply_to_goal(&self, goal: &ProofGoal) -> TacticApplicationResult {
        match self {
            Tactic::AssumeImplicationAntecedent { hypothesis_name } => {
                if let MathRelation::Implies(antecedent, consequent) = &goal.statement {
                    let mut new_goal = goal.clone();
                    new_goal.statement = *consequent.clone();
                    new_goal.value_variables.push(ValueBindedVariable {
                        name: hypothesis_name.clone(),
                        value: MathExpression::Relation(antecedent.clone()),
                    });
                    TacticApplicationResult::SingleGoal(new_goal)
                } else {
                    TacticApplicationResult::NoChange
                }
            }

            Tactic::IntroduceValueVariable { binding, position } => {
                let mut new_goal = goal.clone();
                match position {
                    Some(index) => {
                        new_goal.value_variables.insert(*index, binding.clone());
                    }
                    None => {
                        new_goal.value_variables.push(binding.clone());
                    }
                }
                TacticApplicationResult::SingleGoal(new_goal)
            }

            Tactic::Rewrite {
                target,
                theorem_id,
                instantiation,
                direction,
            } => {
                let registry = get_theorem_registry().lock().unwrap();
                if let Some(theorem) = registry.get_theorem(theorem_id) {
                    if let MathRelation::Equal { left, right, .. } =
                        &theorem.proofs.initial_goal.statement
                    {
                        let mut ident_instantiation = HashMap::new();
                        for (k, v) in instantiation.iter() {
                            ident_instantiation.insert(Identifier::Name(k.clone(), 0), v.clone());
                        }

                        let inst_left =
                            SearchReplace::substitute_variables(&left, &ident_instantiation);
                        let inst_right =
                            SearchReplace::substitute_variables(&right, &ident_instantiation);

                        let (to_replace, replacement) = match direction {
                            RewriteDirection::LeftToRight => (inst_left, inst_right),
                            RewriteDirection::RightToLeft => (inst_right, inst_left),
                        };

                        if target != &to_replace {
                            return TacticApplicationResult::Error(format!(
                                "Rewrite target does not match the instantiated theorem side.\nTarget: {:?}\nTheorem Side: {:?}",
                                target, to_replace
                            ));
                        }

                        let new_statement = SearchReplace::replace_all_in_relation(
                            &goal.statement,
                            &to_replace,
                            &replacement,
                        );

                        let mut new_goal = goal.clone();
                        new_goal.statement = new_statement;
                        TacticApplicationResult::SingleGoal(new_goal)
                    } else {
                        TacticApplicationResult::Error(
                            "Rewrite theorem must be an equality.".to_string(),
                        )
                    }
                } else {
                    TacticApplicationResult::Error(format!(
                        "Theorem '{}' not found in registry.",
                        theorem_id
                    ))
                }
            }

            Tactic::ExactWith {
                theorem_id,
                instantiation,
            } => {
                let registry = get_theorem_registry().lock().unwrap();
                if let Some(theorem) = registry.get_theorem(theorem_id) {
                    let mut ident_instantiation = HashMap::new();
                    for (k, v) in instantiation.iter() {
                        ident_instantiation.insert(Identifier::Name(k.clone(), 0), v.clone());
                    }

                    let instantiated_theorem_statement =
                        SearchReplace::substitute_variables_in_relation(
                            &theorem.proofs.initial_goal.statement,
                            &ident_instantiation,
                        );

                    if instantiated_theorem_statement == goal.statement {
                        let mut new_goal = goal.clone();
                        new_goal.statement = MathRelation::True;
                        TacticApplicationResult::SingleGoal(new_goal)
                    } else {
                        TacticApplicationResult::Error(format!(
                            "Goal does not exactly match theorem {}. Got:\n{:#?}\nExpected:\n{:#?}",
                            theorem_id, goal.statement, instantiated_theorem_statement
                        ))
                    }
                } else {
                    TacticApplicationResult::Error(format!(
                        "Theorem '{}' not found in registry.",
                        theorem_id
                    ))
                }
            }

            Tactic::SplitConjunction { target, index } => {
                if let MathRelation::And(mut relations) = goal.statement.clone() {
                    if *index < relations.len() {
                        let target_conjunct = relations.remove(*index);
                        let remaining_conjuncts = if relations.len() == 1 {
                            relations.remove(0)
                        } else {
                            MathRelation::And(relations)
                        };

                        let goal1 = ProofGoal {
                            statement: target_conjunct,
                            ..goal.clone()
                        };
                        let goal2 = ProofGoal {
                            statement: remaining_conjuncts,
                            ..goal.clone()
                        };
                        TacticApplicationResult::MultiGoal(vec![goal1, goal2])
                    } else {
                        TacticApplicationResult::NoChange
                    }
                } else {
                    TacticApplicationResult::NoChange
                }
            }
            Tactic::IntroduceQuantifier { object, position } => {
                let mut new_goal = goal.clone();
                match position {
                    Some(index) => {
                        new_goal.quantifiers.insert(*index, object.clone());
                    }
                    None => {
                        new_goal.quantifiers.push(object.clone());
                    }
                }
                TacticApplicationResult::SingleGoal(new_goal)
            }
            Tactic::IntroduceFreshVariable {
                target_quantifier,
                fresh_variable_name,
            } => {
                let mut new_goal = goal.clone();
                if let Some(pos) = new_goal
                    .quantifiers
                    .iter()
                    .position(|q| q.variable == *target_quantifier)
                {
                    let quantifier = new_goal.quantifiers.remove(pos);
                    let fresh_var_expr = MathExpression::Var(fresh_variable_name.clone());

                    new_goal.statement = SearchReplace::replace_all_in_relation(
                        &new_goal.statement,
                        &MathExpression::Var(quantifier.variable),
                        &fresh_var_expr,
                    );
                    TacticApplicationResult::SingleGoal(new_goal)
                } else {
                    TacticApplicationResult::Error(format!(
                        "Quantifier '{}' not found.",
                        target_quantifier.to_string()
                    ))
                }
            }
            Tactic::ProvideWitness {
                target_quantifier,
                witness,
            } => {
                let mut new_goal = goal.clone();
                if let Some(pos) = new_goal
                    .quantifiers
                    .iter()
                    .position(|q| q.variable == *target_quantifier)
                {
                    let quantifier = new_goal.quantifiers.remove(pos);

                    new_goal.statement = SearchReplace::replace_all_in_relation(
                        &new_goal.statement,
                        &MathExpression::Var(quantifier.variable),
                        witness,
                    );
                    TacticApplicationResult::SingleGoal(new_goal)
                } else {
                    TacticApplicationResult::Error(format!(
                        "Quantifier '{}' not found.",
                        target_quantifier.to_string()
                    ))
                }
            }
            Tactic::ReorderQuantifiers { new_order } => {
                let mut new_goal = goal.clone();
                if new_order.len() != new_goal.quantifiers.len() {
                    return TacticApplicationResult::Error(
                        "New order must have the same number of quantifiers.".to_string(),
                    );
                }

                let mut current_quantifiers: HashMap<_, _> = new_goal
                    .quantifiers
                    .into_iter()
                    .map(|q| (q.variable.clone(), q))
                    .collect();

                if new_order.len() != current_quantifiers.len() {
                    return TacticApplicationResult::Error(
                        "New order must not contain duplicate identifiers.".to_string(),
                    );
                }

                let mut reordered_quantifiers = Vec::with_capacity(new_order.len());
                for ident in new_order {
                    if let Some(quantifier) = current_quantifiers.remove(ident) {
                        reordered_quantifiers.push(quantifier);
                    } else {
                        return TacticApplicationResult::Error(format!(
                            "Identifier '{}' in new_order not found in goal quantifiers.",
                            ident
                        ));
                    }
                }

                new_goal.quantifiers = reordered_quantifiers;
                TacticApplicationResult::SingleGoal(new_goal)
            }
            Tactic::UniversalCaseAnalysis {
                target_quantifier,
                cases,
            } => {
                let mut base_goal = goal.clone();
                if let Some(pos) = base_goal
                    .quantifiers
                    .iter()
                    .position(|q| q.variable == *target_quantifier)
                {
                    base_goal.quantifiers.remove(pos);

                    let new_goals: Vec<ProofGoal> = cases
                        .iter()
                        .map(|case| {
                            let mut case_goal = base_goal.clone();
                            let hypothesis = ValueBindedVariable {
                                name: Identifier::Name(format!("case_{}", case.name), 0),
                                value: MathExpression::Relation(Box::new(case.condition.clone())),
                            };
                            case_goal.value_variables.push(hypothesis);

                            // The goal is to prove the original statement, given the case
                            let mut new_statement = base_goal.statement.clone();
                            for value in &case.values {
                                new_statement = SearchReplace::replace_all_in_relation(
                                    &new_statement,
                                    &MathExpression::Var(target_quantifier.clone()),
                                    value,
                                );
                            }
                            case_goal.statement = new_statement;
                            case_goal
                        })
                        .collect();

                    if new_goals.is_empty() {
                        TacticApplicationResult::NoChange
                    } else {
                        TacticApplicationResult::MultiGoal(new_goals)
                    }
                } else {
                    TacticApplicationResult::Error(format!(
                        "Quantifier '{}' not found.",
                        target_quantifier.to_string()
                    ))
                }
            }
            Tactic::SubstituteValueVariable { target_variable } => {
                let mut new_goal = goal.clone();
                if let Some(binding) = new_goal
                    .value_variables
                    .iter()
                    .find(|v| v.name == *target_variable)
                {
                    let var_expr = MathExpression::Var(target_variable.clone());
                    new_goal.statement = SearchReplace::replace_all_in_relation(
                        &new_goal.statement,
                        &var_expr,
                        &binding.value,
                    );
                    TacticApplicationResult::SingleGoal(new_goal)
                } else {
                    TacticApplicationResult::Error(format!(
                        "Variable '{}' not found in context.",
                        target_variable.to_string()
                    ))
                }
            }
            Tactic::RewriteInValueBinding {
                target_variable,
                target_sub_expression,
                replacement,
                justification,
            } => {
                let mut new_goal = goal.clone();
                if let Some(binding) = new_goal
                    .value_variables
                    .iter_mut()
                    .find(|v| v.name == *target_variable)
                {
                    binding.value = SearchReplace::replace_all_in_expression(
                        &binding.value,
                        target_sub_expression,
                        replacement,
                    );
                    TacticApplicationResult::SingleGoal(new_goal)
                } else {
                    TacticApplicationResult::Error(format!(
                        "Variable '{}' not found in context.",
                        target_variable.to_string()
                    ))
                }
            }
            Tactic::RemoveValueVariable { target_variable } => {
                let mut new_goal = goal.clone();
                new_goal
                    .value_variables
                    .retain(|v| &v.name != target_variable);
                TacticApplicationResult::SingleGoal(new_goal)
            }
            Tactic::SplitDisjunction { target, index } => {
                if let MathRelation::Or(mut relations) = goal.statement.clone() {
                    if *index < relations.len() {
                        let target_disjunct = relations.remove(*index);
                        let new_goal = ProofGoal {
                            statement: target_disjunct,
                            ..goal.clone()
                        };
                        TacticApplicationResult::SingleGoal(new_goal)
                    } else {
                        TacticApplicationResult::NoChange
                    }
                } else {
                    TacticApplicationResult::NoChange
                }
            }
            Tactic::StatementCaseAnalysis { target, cases } => {
                let new_goals: Vec<ProofGoal> = cases
                    .iter()
                    .map(|(name, case_expr)| {
                        let mut new_goal = goal.clone();
                        let hypothesis = ValueBindedVariable {
                            name: Identifier::Name(format!("case_{}", name), 0),
                            value: case_expr.clone(),
                        };
                        new_goal.value_variables.push(hypothesis);
                        new_goal
                    })
                    .collect();

                if new_goals.is_empty() {
                    TacticApplicationResult::NoChange
                } else {
                    TacticApplicationResult::MultiGoal(new_goals)
                }
            }
            Tactic::Simplify {
                target_path,
                original_expr,
                simplified_expr,
            } => {
                let new_statement = SearchReplace::replace_all_in_relation(
                    &goal.statement,
                    original_expr,
                    simplified_expr,
                );

                let mut new_goal = goal.clone();
                new_goal.statement = new_statement;
                TacticApplicationResult::SingleGoal(new_goal)
            }
            Tactic::Auto(automated_tactic) => {
                if goal.statement == MathRelation::True {
                    let mut new_goal = goal.clone();
                    new_goal.statement = MathRelation::True; // Explicitly mark as proven
                    TacticApplicationResult::SingleGoal(new_goal)
                } else {
                    TacticApplicationResult::NoChange
                }
            }
            Tactic::Induction {
                target_relation_path,
                base_case_value,
                induction_variable_name,
                induction_hypothesis_name,
            } => {
                // For simplicity, we assume the target_relation_path points to the whole statement for now.
                let induction_relation = goal.statement.clone();

                // 1. Base Case Goal
                let base_case_statement = SearchReplace::replace_all_in_relation(
                    &induction_relation,
                    &MathExpression::Var(induction_variable_name.clone()),
                    base_case_value,
                );
                let base_case_goal = ProofGoal {
                    statement: base_case_statement,
                    ..goal.clone()
                };

                // 2. Inductive Step Goal
                let induction_hypothesis = ValueBindedVariable {
                    name: induction_hypothesis_name.clone(),
                    value: MathExpression::Relation(Box::new(induction_relation)),
                };

                // This is a simplification. A real implementation would need to define a 'successor' function.
                // For now, we'll just create a placeholder variable for the successor.
                let successor_var_name =
                    Identifier::Name(format!("succ_{}", induction_variable_name.to_string()), 0);
                let successor_expr = MathExpression::Var(successor_var_name);

                let inductive_step_statement = SearchReplace::replace_all_in_relation(
                    &goal.statement,
                    &MathExpression::Var(induction_variable_name.clone()),
                    &successor_expr,
                );

                let mut inductive_step_goal = ProofGoal {
                    statement: inductive_step_statement,
                    ..goal.clone()
                };
                inductive_step_goal
                    .value_variables
                    .push(induction_hypothesis);

                TacticApplicationResult::MultiGoal(vec![base_case_goal, inductive_step_goal])
            }
        }
    }
}
