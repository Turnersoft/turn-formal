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
                target,
                transformed_expr,
                ..
            } => {
                let new_statement =
                    Self::replace_expression_in_relation(&goal.statement, target, transformed_expr);
                if new_statement == goal.statement {
                    TacticApplicationResult::Error(
                        "Target expression for manipulation not found in goal statement."
                            .to_string(),
                    )
                } else {
                    let mut new_goal = goal.clone();
                    new_goal.statement = new_statement;
                    TacticApplicationResult::SingleGoal(new_goal)
                }
            }
            Tactic::ApplyTheorem { .. } => {
                TacticApplicationResult::NoChange // Placeholder
            }
            Tactic::Rewrite {
                target,
                rewrite_rule,
                ..
            } => {
                if let MathRelation::Equal { right, .. } = rewrite_rule {
                    let new_statement =
                        Self::replace_expression_in_relation(&goal.statement, target, right);
                    if new_statement == goal.statement {
                        TacticApplicationResult::Error(
                            "Target expression for rewrite not found in goal statement."
                                .to_string(),
                        )
                    } else {
                        let mut new_goal = goal.clone();
                        new_goal.statement = new_statement;
                        TacticApplicationResult::SingleGoal(new_goal)
                    }
                } else {
                    TacticApplicationResult::Error("Rewrite rule must be an equality.".to_string())
                }
            }
            Tactic::SplitConjunction { target, index } => {
                if let MathRelation::And(mut conjuncts) = *target.clone() {
                    if *index >= conjuncts.len() {
                        return TacticApplicationResult::Error(
                            "Index out of bounds for conjunction".to_string(),
                        );
                    }

                    let goal1_relation = conjuncts.remove(*index);
                    let goal2_relation = if conjuncts.len() > 1 {
                        MathRelation::And(conjuncts)
                    } else if conjuncts.is_empty() {
                        return TacticApplicationResult::Error(
                            "Cannot split a conjunction into an empty goal.".to_string(),
                        );
                    } else {
                        conjuncts.remove(0)
                    };

                    let statement1 = Self::replace_relation_in_relation(
                        &goal.statement,
                        target,
                        &goal1_relation,
                    );
                    let statement2 = Self::replace_relation_in_relation(
                        &goal.statement,
                        target,
                        &goal2_relation,
                    );

                    let mut subgoal1 = goal.clone();
                    subgoal1.statement = statement1;

                    let mut subgoal2 = goal.clone();
                    subgoal2.statement = statement2;

                    TacticApplicationResult::MultipleGoals(vec![subgoal1, subgoal2])
                } else {
                    TacticApplicationResult::Error(
                        "Target for SplitConjunction must be an And relation".to_string(),
                    )
                }
            }
            Tactic::SplitDisjunction { target, index } => {
                if let MathRelation::Or(mut disjuncts) = *target.clone() {
                    if *index >= disjuncts.len() {
                        return TacticApplicationResult::Error(
                            "Index out of bounds for disjunction".to_string(),
                        );
                    }
                    let new_relation = disjuncts.remove(*index);
                    let new_statement =
                        Self::replace_relation_in_relation(&goal.statement, target, &new_relation);

                    let mut new_goal = goal.clone();
                    new_goal.statement = new_statement;
                    TacticApplicationResult::SingleGoal(new_goal)
                } else {
                    TacticApplicationResult::Error(
                        "Target for SplitDisjunction must be an Or relation".to_string(),
                    )
                }
            }
            Tactic::StatementCaseAnalysis { target: _, cases } => {
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

            Tactic::IntroduceValueVariable { .. } => {
                MathExpression::Var(Identifier::Name("_no_pattern".to_string(), 0))
            }
            Tactic::SubstituteValueVariable {
                target_variable, ..
            } => MathExpression::Var(target_variable.clone()),
            Tactic::RemoveValueVariable { target_variable } => {
                MathExpression::Var(target_variable.clone())
            }

            Tactic::AlgebraicManipulation { target, .. } => target.clone(),
            Tactic::ApplyTheorem { target, .. } => target.clone(),
            Tactic::Rewrite { target, .. } => target.clone(),
            Tactic::SplitConjunction { target, .. } => MathExpression::Relation(target.clone()),
            Tactic::SplitDisjunction { target, .. } => MathExpression::Relation(target.clone()),
            Tactic::StatementCaseAnalysis { target, .. } => target.clone(),

            Tactic::CompleteProof { .. } => {
                MathExpression::Var(Identifier::Name("_complete".to_string(), 0))
            }
            Tactic::Induction {
                induction_variable, ..
            } => MathExpression::Var(induction_variable.clone()),
        }
    }
}

impl Tactic {
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
            _ => relation.clone(),
        }
    }

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
            _ => expr.clone(),
        }
    }

    fn replace_relation_in_relation(
        haystack: &MathRelation,
        needle: &MathRelation,
        replacement: &MathRelation,
    ) -> MathRelation {
        if haystack == needle {
            return replacement.clone();
        }
        match haystack {
            MathRelation::And(relations) => MathRelation::And(
                relations
                    .iter()
                    .map(|r| Self::replace_relation_in_relation(r, needle, replacement))
                    .collect(),
            ),
            MathRelation::Or(relations) => MathRelation::Or(
                relations
                    .iter()
                    .map(|r| Self::replace_relation_in_relation(r, needle, replacement))
                    .collect(),
            ),
            MathRelation::Not(relation) => MathRelation::Not(Box::new(
                Self::replace_relation_in_relation(relation, needle, replacement),
            )),
            MathRelation::Implies(p, c) => MathRelation::Implies(
                Box::new(Self::replace_relation_in_relation(p, needle, replacement)),
                Box::new(Self::replace_relation_in_relation(c, needle, replacement)),
            ),
            MathRelation::Equivalent(l, r) => MathRelation::Equivalent(
                Box::new(Self::replace_relation_in_relation(l, needle, replacement)),
                Box::new(Self::replace_relation_in_relation(r, needle, replacement)),
            ),
            MathRelation::Equal { meta, left, right } => MathRelation::Equal {
                meta: meta.clone(),
                left: Self::replace_expression_in_expression_inside_relation(
                    left,
                    needle,
                    replacement,
                ),
                right: Self::replace_expression_in_expression_inside_relation(
                    right,
                    needle,
                    replacement,
                ),
            },
            _ => haystack.clone(),
        }
    }

    fn replace_expression_in_relation(
        haystack: &MathRelation,
        needle: &MathExpression,
        replacement: &MathExpression,
    ) -> MathRelation {
        match haystack {
            MathRelation::Equal { meta, left, right } => MathRelation::Equal {
                meta: meta.clone(),
                left: Self::replace_expression_in_expression(left, needle, replacement),
                right: Self::replace_expression_in_expression(right, needle, replacement),
            },
            MathRelation::And(relations) => MathRelation::And(
                relations
                    .iter()
                    .map(|r| Self::replace_expression_in_relation(r, needle, replacement))
                    .collect(),
            ),
            MathRelation::Or(relations) => MathRelation::Or(
                relations
                    .iter()
                    .map(|r| Self::replace_expression_in_relation(r, needle, replacement))
                    .collect(),
            ),
            MathRelation::Not(relation) => MathRelation::Not(Box::new(
                Self::replace_expression_in_relation(relation, needle, replacement),
            )),
            MathRelation::Implies(p, c) => MathRelation::Implies(
                Box::new(Self::replace_expression_in_relation(p, needle, replacement)),
                Box::new(Self::replace_expression_in_relation(c, needle, replacement)),
            ),
            MathRelation::Equivalent(l, r) => MathRelation::Equivalent(
                Box::new(Self::replace_expression_in_relation(l, needle, replacement)),
                Box::new(Self::replace_expression_in_relation(r, needle, replacement)),
            ),
            _ => haystack.clone(),
        }
    }

    fn replace_expression_in_expression(
        haystack: &MathExpression,
        needle: &MathExpression,
        replacement: &MathExpression,
    ) -> MathExpression {
        if haystack == needle {
            return replacement.clone();
        }
        match haystack {
            MathExpression::Relation(rel) => {
                let new_rel = Self::replace_expression_in_relation(rel, needle, replacement);
                MathExpression::Relation(Box::new(new_rel))
            }
            MathExpression::ViewAs { expression, view } => MathExpression::ViewAs {
                expression: Box::new(Self::replace_expression_in_expression(
                    expression,
                    needle,
                    replacement,
                )),
                view: view.clone(),
            },
            _ => haystack.clone(),
        }
    }

    fn replace_expression_in_expression_inside_relation(
        haystack: &MathExpression,
        needle: &MathRelation,
        replacement: &MathRelation,
    ) -> MathExpression {
        if let MathExpression::Relation(rel) = haystack {
            if rel.as_ref() == needle {
                return MathExpression::Relation(Box::new(replacement.clone()));
            }
        }
        match haystack {
            MathExpression::Relation(rel) => {
                let new_rel = Self::replace_relation_in_relation(rel, needle, replacement);
                MathExpression::Relation(Box::new(new_rel))
            }
            MathExpression::ViewAs { expression, view } => MathExpression::ViewAs {
                expression: Box::new(Self::replace_expression_in_expression_inside_relation(
                    expression,
                    needle,
                    replacement,
                )),
                view: view.clone(),
            },
            _ => haystack.clone(),
        }
    }
}
